// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the Aleo library.

// The Aleo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Aleo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Aleo library. If not, see <https://www.gnu.org/licenses/>.

use crate::{
    Program,
    ProvingKey,
    VerifyingKey,
    IntermediateTransaction,
    account::PrivateKey,
    types::{
        Aleo,
        ExecutionNative,
        InclusionNative,
        ProcessNative,
        ProgramNative,
        ProvingKeyNative,
        VerifyingKeyNative,
        StatePathNative,
        StateRootNative,
        TransactionNative,
        FeeNative,
        IdentifierNative
    }, RecordPlaintext
};
use std::collections::HashMap;

use std::str::FromStr;
use snarkvm_console::program::Record;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct TransactionBuilder {}

#[wasm_bindgen]
impl TransactionBuilder {
    pub fn build_transition(
        program: Program,
        function_name: &str,
        inputs: &str, // https://github.com/rustwasm/wasm-bindgen/issues/111
        private_key: PrivateKey,
        proving_key: ProvingKey
    ) -> Result<String, String> {
        console_error_panic_hook::set_once();
        let inputs: Vec<String> = serde_json::from_str(inputs)
            .map_err(|_| "Could not deserialize inputs".to_string())?;

        // Get the function identifier
        let program_native = ProgramNative::from(program);
        let function_identifier = IdentifierNative::from_str(function_name)
            .map_err(|_| "Could not get function identifer".to_string())?;
        if !program_native.contains_function(&function_identifier) {
            return Err(format!("Function: {:?} not found", function_identifier))
        }

        // Create the process with only the credits program by default
        let mut process = ProcessNative::load_wasm().map_err(|_| "Could not initialize wasm".to_string())?;

        // Check if the process contains the program, if not, add it
        if !process.contains_program(program_native.id()) {
            process.add_program(&program_native).map_err(|_| "Could not add program".to_string())?;
        }

        // Insert the proving key
        process.insert_proving_key(program_native.id(), &function_identifier, ProvingKeyNative::from(proving_key))
            .map_err(|_| "Could not insert proving key".to_string())?;

        // Generate the authorization
        let rng = &mut rand::thread_rng();
        let authorization =
            process.authorize::<Aleo, _>(&private_key, program_native.id(), function_name, inputs.iter(), rng)
            .map_err(|thing|  {
                println!("{:?}", thing);
                return "Could not generate authorization".to_string();
            })?;

        // Get the input_ids, necessary to reconstruct the inclusion proof
        let next = authorization.peek_next().unwrap();
        let input_ids = next.input_ids().to_vec();

        // Generate the execution
        let (_, execution, _, _) = process.execute::<Aleo, _>(authorization, rng)
            .map_err(|_| "Could not complete program execution".to_string())?;
        
        // Get the transition from the execution
        let mut transitions = execution.transitions();
        let transition = transitions.next().unwrap().to_owned();
        let intermediate_transaction = IntermediateTransaction {
            transition,
            input_ids
        };
        let intermediate_transaction = serde_json::to_string(&intermediate_transaction)
            .map_err(|_| "Could not serialize intermediate transaction".to_string())?;
        
        Ok(intermediate_transaction)

    }

    pub fn build_transaction(
        inclusion_key: ProvingKey,
        intermediate_transactions: &str,
        state_root: &str,
        commitment_map: &str
    ) -> Result<String, String> {
        console_error_panic_hook::set_once();

        // Create Inclusion and Execution from all IntermediateTransactions
        let mut inclusion = InclusionNative::new();
        let mut execution = ExecutionNative::new();

        let intermediate_transactions: Vec<IntermediateTransaction> = serde_json::from_str(intermediate_transactions)
            .map_err(|_| "Could not deserialize intermediate transactions".to_string())?;

        for IntermediateTransaction { transition, input_ids } in &intermediate_transactions {
            let transition = transition.to_owned();
            inclusion.insert_transition(&input_ids, &transition).unwrap();
            execution.push(transition);
        }

        // Parse state root and state paths
        let global_state_root = StateRootNative::from_str(state_root).unwrap();
        let default_map: HashMap<String, StatePathNative> = HashMap::new();
        let commitment_to_state_path: HashMap<String, StatePathNative> = serde_json::from_str(commitment_map).unwrap_or(default_map);

        // Prepare the assignments.
        let (assignments, _) = inclusion
            .prepare_execution_stateless(&execution, global_state_root, commitment_to_state_path).unwrap();

        // Compute the inclusion proof and update the execution.
        let rng = &mut rand::thread_rng();
        let execution = inclusion
            .prove_execution_stateless::<Aleo, _>(inclusion_key.into(), execution, &assignments, global_state_root, rng).unwrap();

        let tx = TransactionNative::from_execution(execution, None).unwrap();
        Ok(tx.to_string())
    }

    pub fn add_fee_to_transaction(
        transaction_string: &str,
        fee_string: &str
    ) -> Result<String, String> {
        console_error_panic_hook::set_once();
        // Parse Transaction
        let mut transaction = TransactionNative::from_str(transaction_string).unwrap();
        let mut fee = FeeNative::from_str(fee_string).unwrap();

        if let TransactionNative::Execute(id, execution, _) = transaction {
            let tx = TransactionNative::from_execution(execution, Some(fee)).unwrap();
            Ok(tx.to_string())
        } else {
            return Err("Transaction is not an execution".to_string());
        }
    }

    pub fn verify_transaction(
        transaction_string: &str,
        program: Program,
        function_name: &str,
        function_verifying_key: VerifyingKey,
        inclusion_key: VerifyingKey,
        verify_inclusion: bool,
        verify_execution: bool
    ) -> Result<String, String> {
        console_error_panic_hook::set_once();
        // Parse Transaction
        let transaction = TransactionNative::from_str(transaction_string).unwrap();

        // Get the function identifier
        let program_native = ProgramNative::from(program);
        let function_identifier = IdentifierNative::from_str(function_name)
            .map_err(|_| "Could not get function identifer".to_string())?;
        if !program_native.contains_function(&function_identifier) {
            return Err(format!("Function: {:?} not found", function_identifier))
        }

        // Create the process with only the credits program by default
        let mut process = ProcessNative::load_wasm().map_err(|_| "Could not initialize wasm".to_string())?;

        // Check if the process contains the program, if not, add it
        if !process.contains_program(program_native.id()) {
            process.add_program(&program_native).map_err(|_| "Could not add program".to_string())?;
        }

        // Insert the verifying key
        process.insert_verifying_key(program_native.id(), &function_identifier, VerifyingKeyNative::from(function_verifying_key))
            .map_err(|_| "Could not insert transfer verifying key".to_string())?;

        match transaction {
            TransactionNative::Deploy(_, _, _, _) => {
                panic!("Cannot verify Deploy Transactions");
            }
            TransactionNative::Execute(_, execution, _) => {
                if verify_execution {
                    process.verify_execution::<false>(&execution)
                        .map_err(|_| "Failed to verify execution".to_string())?;
                }
                if verify_inclusion {
                    InclusionNative::verify_execution_stateless(&execution, VerifyingKeyNative::from(inclusion_key))
                        .map_err(|_| "Failed to verify inclusion".to_string())?;
                }
            }
        }

        Ok("Transaction verified".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use web_sys::console;

    // const PRIVATE_KEY: &str = "APrivateKey1zkpDKDJELtg3GN29R2UPihgsucvYSkb7BzEYDRkJtZcwhj8";
    // const PRIVATE_KEY: &str = "APrivateKey1zkpEkwRohuPgJGxFS7TECc6nEzvNa1P7vP38mq7RkhS6RCG";
    // const PRIVATE_KEY: &str = "APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH";
    const PRIVATE_KEY: &str = "APrivateKey1zkp8UsqkCvrUoFaCn1Dw9WJG69hp8TLFgT1V99cuZWVoDKx";
    const OWNER_PLAINTEXT1: &str = r"{  owner: aleo146dx5e4nssf49t0aq9qljk474kqxk848tl05m8w84vc0jqa30spqf4me04.private,  microcredits: 101u64.private,  _nonce: 2036393439745953531101794606409563566245421800022328395850697982644162839463group.public}";
    // const OWNER_PLAINTEXT1: &str = r"{  owner: aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px.private,  microcredits: 375000000000000u64.private,  _nonce: 3929214896371155671696476761462680699003730187884776587690604310763551537209group.public}";
    // const OWNER_PLAINTEXT1: &str = r"{  owner: aleo1mc6h9k9g03hfkrh93wm6vlew6q2ehzwalqutzjx6hp5mdcv9rsyq2w3alz.private,  gates: 9973u64.private,  _nonce: 1157538218493794217455834413151916695596644156738232294925137030650794330428group.public}";
    const OWNER_PLAINTEXT1_COMMITMENT: &str = r"605538192287731854742364189642613370746132375889525872894501892977418029640field";
    const OWNER_PLAINTEXT1_PATH: &str = r"path1qqqd4vjjgfd05ekjmd02r74pxd7rqjsu2n3a9rnutkc6x5c3yhj6szklquqqqqqqqqqr4hq4tv6w8qp64680gfnq69tvdr0unup88k5sgum5ynru8f7zqpx6z4t3pxwm2zh2c9n8jhm9f9nat86kdx2yf5lfdl8kr0p9rh3fq347xs7al64hvdjuetejauufsfhvrvzz429kcmcavjs9229ctedssw8ucpfq88q4jkf083fwp64nwf6eq06wj0vlqa2s87c4vudemsqy3facnk8c6xwskfg23h6xctfca2r28echq935pzjt8n6upgv9ls9zm4ll46tsszsxkdg3hw0vl6ah8vmvrh7g2eejfjeju6c6dlt7gy9jsvfa60yjq5tft7zt6gnsz4flegx50dgsluqwwv6w7hwpct99z2y8ftxn4pupqlvupt4jtjkv2zxkye0333ktmjfgz03f99xy03aqjzx4zmzemg6kyxugy97m6p9mlfkmp5xwz5udre5rh4rk40ezsrsftcghmjqf793nuq482m5uyznnegaq8v6tu4zxr78pyh23hug8jvqmut52jefmeqd8dnzsugx65fwhr3qmtfw8nw9am5kwwtnjyk4jsy5yzk022z5wng8yez9mttc0s26mxfrlk4qe9znxxqxlcfapv326qjzpt849p28f5rjv3za44u8c9ddny3lm2svj3fnrqr0uy7skg4dqfpq4n6js4r56pexg3w667ruzkkejgla4gxfg5e3sph7z0gty2ksyss2eafg236dqunyghdd0p7pttvey0765ry52vccqmlp859j9tgzgg9v7559gaxswfjytkkhslq44kvj8ld2pj29xvvqdlsn6zez45pyyzk022z5wng8yez9mttc0s26mxfrlk4qe9znxxqxlcfapv326qjzpt849p28f5rjv3za44u8c9ddny3lm2svj3fnrqr0uy7skg4dqfpq4n6js4r56pexg3w667ruzkkejgla4gxfg5e3sph7z0gty2ksyss2eafg236dqunyghdd0p7pttvey0765ry52vccqmlp859j9tgzgg9v7559gaxswfjytkkhslq44kvj8ld2pj29xvvqdlsn6zez45pyyzk022z5wng8yez9mttc0s26mxfrlk4qe9znxxqxlcfapv326qjzpt849p28f5rjv3za44u8c9ddny3lm2svj3fnrqr0uy7skg4dqfpq4n6js4r56pexg3w667ruzkkejgla4gxfg5e3sph7z0gty2ksyss2eafg236dqunyghdd0p7pttvey0765ry52vccqmlp859j9tgzgg9v7559gaxswfjytkkhslq44kvj8ld2pj29xvvqdlsn6zez45pyyzk022z5wng8yez9mttc0s26mxfrlk4qe9znxxqxlcfapv326qjzpt849p28f5rjv3za44u8c9ddny3lm2svj3fnrqr0uy7skg4dqfpq4n6js4r56pexg3w667ruzkkejgla4gxfg5e3sph7z0gty2ksyss2eafg236dqunyghdd0p7pttvey0765ry52vccqmlp859j9tgzgg9v7559gaxswfjytkkhslq44kvj8ld2pj29xvvqdlsn6zez45pyyzk022z5wng8yez9mttc0s26mxfrlk4qe9znxxqxlcfapv326qns27zmwr8z5y604wad5pcrjyx0xw0jsev8zmp92hetyf8nvepgqlrrj4qwaueef7jukyclk7xy5djeg3jqan4cpdnv69aa7az6tulssu4qm9lrx3fzgdqlpd7zfct94fyw63a2gzyglj72cx7rlyfmrlgrszqqqqqqqqqqqkrg383d3ctrsv3dn365v0hg3wmuj7frqzmuvldsj9mjcgllaqyqx8ce6fm5jhhfup3as6eg7m95h7hhd38f2m2e8w582lwzj253rkpqplxpvafj73f7kuymxn2lnhajfymjp7tg4r8xc8ltq70zgdwtcpvqcmhna74rpc8h4yt4f6g9uxdls83qr6cpvyymespsaahtpn0c9spsqqqqqqqqqqqqqhcewf768vg6pldnetgxf448sm9ecxd9vlzcp72d4nxqxlsx2vr5yzk022z5wng8yez9mttc0s26mxfrlk4qe9znxxqxlcfapv326qjzpt849p28f5rjv3za44u8c9ddny3lm2svj3fnrqr0uy7skg4dqfpq4n6js4r56pexg3w667ruzkkejgla4gxfg5e3sph7z0gty2ksyss2eafg236dqunyghdd0p7pttvey0765ry52vccqmlp859j9tgzgg9v7559gaxswfjytkkhslq44kvj8ld2pj29xvvqdlsn6zez45pyyzk022z5wng8yez9mttc0s26mxfrlk4qe9znxxqxlcfapv326qjzpt849p28f5rjv3za44u8c9ddny3lm2svj3fnrqr0uy7skg4dqfpq4n6js4r56pexg3w667ruzkkejgla4gxfg5e3sph7z0gty2ksyss2eafg236dqunyghdd0p7pttvey0765ry52vccqmlp859j9tgzgg9v7559gaxswfjytkkhslq44kvj8ld2pj29xvvqdlsn6zez45pyyzk022z5wng8yez9mttc0s26mxfrlk4qe9znxxqxlcfapv326qjzpt849p28f5rjv3za44u8c9ddny3lm2svj3fnrqr0uy7skg4dqfpq4n6js4r56pexg3w667ruzkkejgla4gxfg5e3sph7z0gty2ksyss2eafg236dqunyghdd0p7pttvey0765ry52vccqmlp859j9tgzgg9v7559gaxswfjytkkhslq44kvj8ld2pj29xvvqdlsn6zez45pzumn3nfqsdylxcesax8zwvks7de3v0gw5ndttpd4l80ucku7mvp5qqqqqqqqqqqqyyzk022z5wng8yez9mttc0s26mxfrlk4qe9znxxqxlcfapv326qjzpt849p28f5rjv3za44u8c9ddny3lm2svj3fnrqr0uy7skg4dqfpq4n6js4r56pexg3w667ruzkkejgla4gxfg5e3sph7z0gty2ksyss2eafg236dqunyghdd0p7pttvey0765ry52vccqmlp859j9tgzqzqqqhgl59s8lcdezyqg4ylcvajkjwqkcv9dygxm23njmd9fvvmk30c9sxqqqqqqqqqqqe332uur0a63lncg7gw5nz5vqqkz3njajl9v4henfgjuuh3hjksr879j0kkkteaehcn9yfmmh5ny4j9vt8gqelnygdue682j97ka4yq099ryhtp3hk6f0mvcr5watm4ca7wg42sze6skdmsxt8jx8t72hq6zpt849p28f5rjv3za44u8c9ddny3lm2svj3fnrqr0uy7skg4dqgqqrqvqy3jnmdu3va9w68y6rcnlmvh26mhud86nqqs0v0p84nq7l7wu9vqg4sqmk7";
    
    const RECIPIENT: &str = "aleo1nhdttzqjjj68qqjge68m7ugpz6d9dq30qxxngyg2qjr9f8hx4cgs42t7wp";
    const AMOUNT: &str = "97u64";
    const GLOBAL_STATE_ROOT: &str = "ar13manm2epykqtk0zxzcqa7nn3zc5z064lap6r2ys0t0l5t3uy9crsufrdj2";

    // #[wasm_bindgen_test]
    // fn test_build_transition() {
    //     // Prepare inputs to build transition
    //     let program = Program::credits().unwrap();
    //     let function_name = "transfer";
    //     let inputs = serde_json::to_string(&[
    //         OWNER_PLAINTEXT1,
    //         RECIPIENT,
    //         AMOUNT
    //     ]).unwrap();
    //     let private_key = PrivateKey::from_str(PRIVATE_KEY).unwrap();
    //     let transfer_bytes = include_bytes!(concat!(env!("HOME"), "/.aleo/resources/transfer.prover.95214f1")).to_vec();
    //     let transfer_proving_key = ProvingKey::from_bytes(transfer_bytes);

    //     // Build transition
    //     let intermediate_transaction1 = TransactionBuilder::build_transition(
    //         program,
    //         function_name,
    //         &inputs,
    //         private_key,
    //         transfer_proving_key
    //     ).unwrap();

    //     console::log_1(&intermediate_transaction1.into());
    // }

    // #[wasm_bindgen_test]
    // fn test_build_transaction() {
    //     // Prepare inputs to build transaction
    //     let intermediate_transaction1 = r#"{"transition":{"id":"as100gs7rlucq4zu7u3dlwys9vnep80ftwws2u2ausvzcpf2fesrgzsfuf9jv","program":"credits.aleo","function":"transfer","inputs":[{"type":"record","id":"5014771957006340580317363981373591084465734774693302641777058302177738009042field","tag":"1592215853668322267062146193964960404834934970523747812466009574754114740452field"},{"type":"private","id":"322815084863476371220771878109567628405323161045197678148136586030747914889field","value":"ciphertext1qgq9ya29q50jnxwspkwgr6g4cgvf87sw5j2hvk6x2zmkarg9xxauxqhyj3fh8xs0gjqfezfl689krqxj5th76uz075465mfjdvfh72tgpc3tan76"},{"type":"private","id":"2145739655038782315766524255894847900603699299809703897648490882673572222470field","value":"ciphertext1qyqrh2twkdal03adakvv4cl4lzrwu5wljxyms84js9s5h6l62ftevqcj2rk79"}],"outputs":[{"type":"record","id":"4069439388021451078269049650605705726761983682392111434935933144215765309236field","checksum":"1406934742874233776553884855777823800086998498181550189152055420777952320965field","value":"record1qyqspyrgflcgucsr59gqzqzegxf395wxhw0eq3kv63zsfa7y9sqd4tqsqyqspuv6ewd4qhz6p9r2nfakxmn2l2zfyqgsh6vh6p2d5q2650vxtjq8qzajnka4q5u50vfwdavv3qqwe2wnpsuymfxvzyazmejnjnuqgurswsfx9eh"},{"type":"record","id":"5420469059357958525128014207249608754218509899266503418642860591860771984738field","checksum":"980276437940144926956762218081372377044893877273476995664467211172445965935field","value":"record1qyqsp0lyytf9eym2xccvpcm45nyuymmksnvmlyvd6furxqx7rv8wfkssqyqspevvj0qamqf3rsz4sf67k7qppslfj9e3fwfzrfhmxuwz6rw8myc2qzz8gdp55x534a0e2haqg9jd960yyuhkcn09jgqddlu0fyumxmzsvrj4due"}],"proof":"proof1qqqqzqqqqqqqqqqqkvuv2w8r0npu43mp8s7p9sh48cjssqq5q6ya4x9ryg4fu7dl0x6xsmqqdnjd7s9e3yu0n6yzvv5czgvaghn8vfurfcm6dlfgwzp3lk8un8wzshhleqpdxm8l04e86p3n8ux8kwn42vaz6kgz7ngnahvssq7er0szz4ku9q2xdz2pz58fhcuka5k4ff9lsmjax273maz9ph2tf0mvfzdh8zm0snh6v4xj7pls2qgptdaxeyqg8x57n7rehkywucu38mup6gnm6f700j7x8wak7evtsw8juqqxswxdh525ml6spr5smsncp03emkk85pt52jtdt8qqcmttg3rxj2uvjv2km4r7acxuyqca4f632fksax6ed287c4khjwucqvvjq8ks5k5atpdftwu82vxmu65pxsn494p9gsthhg0j4kpqrk30cd4pxkjnwrjewltuvp7jztfd4dppyqwss3yd26vecdad3rt6ez4gqhx9tn776m4grugc3ztfg2r2facr5mcw8uxwq963vc6pxngenzqp55q2j0y0j8rz62ry40ygxr72s6zk6hxdltfvmuxmlujuzkf5a09p8nre7slgxff4rwqelmfqd4jved5q4j542ddwlgp20qhsd8mnkdcckgvwrt6rv6a20rhutj72f5ns44tsxlztrdffj4lqycfy6qr7jaesr6zch4v9keullzuem4nz7rgs9rps8d3aywq8lu9pz7nprgkjjau6p3qh87gceqe6wtevss8wl0ppsx24nhppj8z4274wegs9te8eeekr9pl8ue2hy7war4smlksmdngsva9g5kjvm4h3046070enwemsduztlwwd08gxmaymwc8hrd485gqsp2nakpzjlaqdmv6q3w6f9usju4l760zgmzny7l9chfcfy8xekcgyysv4a3yvcnm84t0c7c8l4ef0afkp37409gctx6cg79z88h86wpce4tan5lecxaa4aqde8gay4wmdu4sywhdw32cnses8pdka9xhqp09uxrfndxj35ng0r0zj625t3ny2gp6990wwe5dsk2d8ng2j4ajsk097d5gjh0m3nzkng39sfrrrf6v3rjdc6mr2agn6clp09jmgzlqjy37y8p2rymn2nuw3aktznpy4kra58cdkfgfuvehpwg5hgq0g9c8syqqqqqqqqqqq6cac9vgjv8433sg7e522nz0dg379t955ft035ce8z7xe4h228dqajv9g6qlkwn8zjt6kgy0c7ragqq2matv09cpdyswjsknfd9g7srqm8dpfq5wjqxee6yavqlw08jc3qwwtahh9nnwav6cl4lhfstp3yezyu82m2068drjqqzkuy705mt8fwwdwdgpx2z2pljnal63zppm9vqqqqq2pftw6","tpk":"7232922488843750299609714637897998954390062332698850825353778535377930212519group","tcm":"2576677602366284087862363339085985775279049642524480921123773832178280700244field","fee":0},"input_ids":[{"type":"record","commitment":"605538192287731854742364189642613370746132375889525872894501892977418029640field","gamma":"5356502370233901355799069746156088185496903661954521847638241903524144982783group","serial_number":"5014771957006340580317363981373591084465734774693302641777058302177738009042field","tag":"1592215853668322267062146193964960404834934970523747812466009574754114740452field"},{"type":"private","id":"322815084863476371220771878109567628405323161045197678148136586030747914889field"},{"type":"private","id":"2145739655038782315766524255894847900603699299809703897648490882673572222470field"}]}"#;
    //     let intermediate_transaction1: IntermediateTransaction = serde_json::from_str(intermediate_transaction1).unwrap();
    //     let inclusion_bytes = include_bytes!(concat!(env!("HOME"), "/.aleo/resources/inclusion.prover.209da1d")).to_vec();
    //     let inclusion_proving_key = ProvingKey::from_bytes(inclusion_bytes);
    //     let intermediate_transactions = serde_json::to_string(&[
    //         intermediate_transaction1
    //     ]).unwrap();

    //     let mut commitment_map: HashMap<String, StatePathNative> = HashMap::new();
    //     commitment_map.insert(OWNER_PLAINTEXT1_COMMITMENT.to_string(), StatePathNative::from_str(&OWNER_PLAINTEXT1_PATH).unwrap());
    //     let commitment_map_serialized = serde_json::to_string(&commitment_map).unwrap();

    //     let transaction = TransactionBuilder::build_transaction(
    //         inclusion_proving_key,
    //         intermediate_transactions.as_str(),
    //         GLOBAL_STATE_ROOT,
    //         &commitment_map_serialized.as_str()
    //     ).unwrap();

    //     console::log_1(&transaction.into());
    // }
}