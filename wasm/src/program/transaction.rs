use crate::{
  account::{Address, PrivateKey},
  record::RecordPlaintext, types::{StatePathNative, CurrentNetwork, TransitionNative}
};
use crate::{
    Aleo,
    Identifier,
    Process,
    Program,
    ProvingKey,
    ProvingKeyNative,
    StatePathMap,
    TransactionNative,
};
use crate::StateRootNative;
use std::collections::HashMap;

use std::str::FromStr;
use snarkvm_console::program::{InputID};
use snarkvm_synthesizer::{Execution, Inclusion};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct TransactionBuilder {}

#[wasm_bindgen]
impl TransactionBuilder {
    /// Creates an execute transaction from a full proof of execution
    pub fn build_transition(
        private_key: PrivateKey,
        proving_key: ProvingKey,
        address: Address,
        amount: u64,
        record: RecordPlaintext,
    ) -> Result<String, String> {
        console_error_panic_hook::set_once();
        let process = Process::load_wasm().map_err(|_| "Could not initialize wasm".to_string())?;
        let credits_program = Program::credits().map_err(|_| "Could not access credits program".to_string())?;
        let transfer_identifier = &Identifier::from_str("transfer").map_err(|_| "Could not create transfer identifier".to_string())?;

        process.insert_proving_key(credits_program.id(), transfer_identifier, ProvingKeyNative::from(proving_key))
            .map_err(|_| "Could not insert proving key".to_string())?;

        let mut amount_str = amount.to_string();
        amount_str.push_str("u64");
        let inputs = [record.to_string(), address.to_string(), amount_str];
        let rng = &mut rand::thread_rng();

        let authorization =
            process.authorize::<Aleo, _>(&private_key, credits_program.id(), "transfer", inputs.iter(), rng)
            .map_err(|_| "Could generate authorization".to_string())?;

        let next = authorization.peek_next().unwrap();
        let input_ids = next.input_ids().to_vec();

        let (_, execution, _, _) = process.execute::<Aleo, _>(authorization, rng)
            .map_err(|_| "Could not complete program execution 1".to_string())?;

        let mut transitions = execution.transitions();
        let transition = transitions.next().unwrap();

        let output_tuple = (transition, input_ids);
        let transition_string = serde_json::to_string(&output_tuple)
            .map_err(|_| "Could not complete program execution 10".to_string())?;

        Ok(transition_string)
    }

    pub fn build_transaction(
        inclusion_key: ProvingKey,
        transition_string: &str,
        input_ids_string: &str,
        state_root: &str,
        commitment_map: &str
    ) -> Result<String, String> {
        console_error_panic_hook::set_once();
        // Parse Transition & input ids
        let transition = TransitionNative::from_str(transition_string).unwrap();
        let input_ids: Vec<InputID::<CurrentNetwork>> = serde_json::from_str(input_ids_string).unwrap();

        // Recreate inclusion
        let mut inclusion = Inclusion::<CurrentNetwork>::new();
        inclusion.insert_transition(&input_ids, &transition).unwrap();

        // Recreate execution     
        let mut execution = Execution::<CurrentNetwork>::new();
        execution.push(transition);

        // Parse state root and state paths
        let global_state_root = StateRootNative::from_str(state_root).unwrap();
        let default_map: HashMap<String, StatePathNative> = HashMap::new();
        let commitment_to_state_path: StatePathMap = serde_json::from_str(commitment_map).unwrap_or(StatePathMap { map: default_map });

        // Prepare the assignments.
        let rng = &mut rand::thread_rng();
        let (assignments, _) = inclusion
            .prepare_execution_stateless(&execution, global_state_root, commitment_to_state_path.map).unwrap();

        // Compute the inclusion proof and update the execution.
        let execution = inclusion
            .prove_execution_stateless::<Aleo, _>(inclusion_key.into(), execution, &assignments, rng).unwrap();

        // Verify the inclusion proof
        Inclusion::<CurrentNetwork>::verify_execution(&execution).unwrap();

        let tx = TransactionNative::from_execution(execution, None).unwrap();
        let tx_string = tx.to_string();
        Ok(tx_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const OWNER_PLAINTEXT: &str = r"{
  owner: aleo184vuwr5u7u0ha5f5k44067dd2uaqewxx6pe5ltha5pv99wvhfqxqv339h4.private,
  gates: 1159017656332810u64.private,
  _nonce: 1635890755607797813652478911794003479783620859881520791852904112255813473142group.public
}";

    const ALEO_PRIVATE_KEY: &str = "APrivateKey1zkp3dQx4WASWYQVWKkq14v3RoQDfY2kbLssUj7iifi1VUQ6";

    #[test]
    // #[ignore]
    fn test_build_transaction() {
        let bytes = include_bytes!(concat!(env!("HOME"), "/.aleo/resources/transfer.prover.837ad21")).to_vec();
        let private_key = PrivateKey::from_string(ALEO_PRIVATE_KEY).unwrap();
        let proving_key =ProvingKey::from_bytes(bytes);
        let address = Address::from_private_key(&private_key);
        let amount = 100;
        let record = RecordPlaintext::from_string(OWNER_PLAINTEXT).unwrap();
        TransactionBuilder::build_transfer_full(private_key, proving_key, address, amount, record);
    }
}