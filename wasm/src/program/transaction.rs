use crate::{
  account::{Address, PrivateKey},
  record::RecordPlaintext,
  program::ProvingKey,
};

use crate::{Aleo, Process, Program, Transaction};
use wasm_bindgen::prelude::*;
use js_sys::Array;

#[wasm_bindgen]
pub struct TransactionBuilder {}

#[wasm_bindgen]
impl TransactionBuilder {
    /// Creates an execute transaction from a full proof of execution
    pub fn build_transfer_full(
        private_key: PrivateKey,
        proving_key: ProvingKey,
        address: Address,
        amount: u64,
        record: RecordPlaintext,
    ) -> String {
        console_error_panic_hook::set_once();

        let process = Process::load_wasm().unwrap();
        let credits_program = Program::credits().unwrap();

        process.insert_transfer_proving_key(proving_key.into()).unwrap();

        let mut amount_str = amount.to_string();
        amount_str.push_str("u64");
        let inputs = [record.to_string(), address.to_string(), amount_str];
        let rng = &mut rand::thread_rng();

        let authorization =
            process.authorize::<Aleo, _>(&private_key, credits_program.id(), "transfer", inputs.iter(), rng).unwrap();
        let (_, execution, _, _) = process.execute::<Aleo, _>(authorization, rng).unwrap();
        
        // TODO: Figure out how to get proper inclusion proofs
        let tx = Transaction::from_execution(execution, None).unwrap();
        let tx_string = tx.to_string();
        tx_string
    }
}