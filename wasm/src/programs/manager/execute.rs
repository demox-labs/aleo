// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the Aleo SDK library.

// The Aleo SDK library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Aleo SDK library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Aleo SDK library. If not, see <https://www.gnu.org/licenses/>.

use super::*;
use core::ops::Add;

use crate::{
    execute_fee,
    execute_program,
    log,
    process_inputs,
    PrivateKey,
    RecordPlaintext,
    Transaction, verifying_key,
};

use crate::types::native::{
    AuthorizationNative,
    ExecutionNative,
    IdentifierNative,
    ProcessNative,
    ProgramNative,
    RecordPlaintextNative,
    TransactionNative,
    PrivateKeyNative
};
use snarkvm_circuit_network::Aleo;
use js_sys::{Array, Object};
use rand::{rngs::StdRng, SeedableRng};
use serde::Serialize;
use std::str::FromStr;
use snarkvm_console::prelude::ToBytes;

#[derive(Serialize)]
pub struct AuthorizationResponse {
    pub authorization: String,
    pub fee_authorization: String,
    pub program: String
}

#[wasm_bindgen]
impl ProgramManager {
    #[wasm_bindgen]
    pub fn synthesize(
        network: &str,
        program_string: &str,
        function: &str,
        imports: Option<Object>,
    ) -> Result<KeyPair, String> {
      match dispatch_network_aleo!(network, execute_synthesize_impl, program_string, function, imports) {
        Ok(result) => Ok(result),
        Err(e) => return Err(e),
      }
    }

    #[wasm_bindgen]
    #[allow(clippy::too_many_arguments)]
    pub async fn execute_transaction(
        private_key: &PrivateKey,
        program: &str,
        function: &str,
        inputs: Array,
        fee_credits: f64,
        fee_record: Option<RecordPlaintext>,
        url: &str,
        imports: Option<Object>,
        proving_key: Option<ProvingKey>,
        verifying_key: Option<VerifyingKey>,
        fee_proving_key: Option<ProvingKey>,
        fee_verifying_key: Option<VerifyingKey>,
        inclusion_key: ProvingKey,
    ) -> Result<Transaction, String> {
      match dispatch_network_aleo_async!(
        private_key.network.as_str(),
        execute_transaction_impl,
        private_key,
        program,
        function,
        inputs,
        fee_credits,
        fee_record,
        url,
        imports,
        proving_key,
        verifying_key,
        fee_proving_key,
        fee_verifying_key,
        inclusion_key
      ) {
        Ok(result) => Ok(result),
        Err(e) => return Err(e),
      }
    }
  
    #[wasm_bindgen]
    #[allow(clippy::too_many_arguments)]
    pub async fn authorize_transaction(
        private_key: &PrivateKey,
        program: &str,
        function: &str,
        inputs: Array,
        fee_credits: f64,
        fee_record: Option<RecordPlaintext>,
        imports: Option<Object>,
    ) -> Result<String, String> {
      match dispatch_network_aleo_async!(
        private_key.network.as_str(),
        authorize_transaction_impl,
        private_key,
        program,
        function,
        inputs,
        fee_credits,
        fee_record,
        imports
      ) {
        Ok(result) => Ok(result),
        Err(e) => return Err(e),
      }
    }

    #[wasm_bindgen]
    #[allow(clippy::too_many_arguments)]
    pub async fn execute_authorization(
        authorization: &str,
        fee_authorization: Option<String>,
        program: &str,
        function: &str,
        url: &str,
        imports: Option<Object>,
        proving_key: Option<ProvingKey>,
        verifying_key: Option<VerifyingKey>,
        fee_proving_key: Option<ProvingKey>,
        fee_verifying_key: Option<VerifyingKey>,
        inclusion_key: ProvingKey,
    ) -> Result<Transaction, String> {
      match dispatch_network_aleo_async!(
        inclusion_key.network.as_str(),
        execute_authorization_impl,
        authorization,
        fee_authorization,
        program,
        function,
        url,
        imports,
        proving_key,
        verifying_key,
        fee_proving_key,
        fee_verifying_key,
        inclusion_key
      ) {
        Ok(result) => Ok(result),
        Err(e) => return Err(e),
      }
    }

    #[wasm_bindgen]
    #[allow(clippy::too_many_arguments)]
    pub async fn build_execution(
        private_key: &PrivateKey,
        program: &str,
        function: &str,
        inputs: Array,
        url: &str,
        imports: Option<Object>,
        proving_key: Option<ProvingKey>,
        verifying_key: Option<VerifyingKey>,
        inclusion_key: ProvingKey,
    ) -> Result<String, String> {
        match dispatch_network_aleo_async!(
            private_key.network.as_str(),
            build_execution_impl,
            private_key,
            program,
            function,
            inputs,
            url,
            imports,
            proving_key,
            verifying_key,
            inclusion_key
        ) {
            Ok(result) => Ok(result),
            Err(e) => return Err(e),
        }
    }
   }

   pub fn execute_synthesize_impl<N: Network, A: Aleo<Network = N>>(
    program_string: &str,
    function: &str,
    imports: Option<Object>,
   ) -> Result<KeyPair, String> {
    let mut process_native = ProcessNative::<N>::load_web().map_err(|err| err.to_string())?;
    let process = &mut process_native;

    let program =
        ProgramNative::<N>::from_str(program_string).map_err(|err| err.to_string())?;
    let function_name =
        IdentifierNative::<N>::from_str(function).map_err(|err| err.to_string())?;

    log("Check program imports are valid and add them to the process");
    program_manager_resolve_imports_impl::<N>(process, &program, imports)?;

    if program.id().to_string() != "credits.aleo" {
        log(&format!("Adding program: {}", program.id().to_string()));
        process.add_program(&program).map_err(|_| "Failed to add program".to_string())?;
    }

    process.synthesize_key::<A, _>(&program.id(), &function_name,  &mut StdRng::from_entropy())
        .map_err(|err| err.to_string())?;

    let proving_key = process.get_proving_key(program.id(), function_name).map_err(|e| e.to_string())?;
    let verifying_key = process.get_verifying_key(program.id(), function_name).map_err(|e| e.to_string())?;
    return Ok(KeyPair::from((proving_key, verifying_key)));
   }

   pub async fn execute_transaction_impl<N: Network, A: Aleo<Network = N>>(
    private_key: &PrivateKey,
    program: &str,
    function: &str,
    inputs: Array,
    fee_credits: f64,
    fee_record: Option<RecordPlaintext>,
    url: &str,
    imports: Option<Object>,
    proving_key: Option<ProvingKey>,
    verifying_key: Option<VerifyingKey>,
    fee_proving_key: Option<ProvingKey>,
    fee_verifying_key: Option<VerifyingKey>,
    inclusion_key: ProvingKey,
   ) -> Result<Transaction, String> {
    log(&format!("Executing function: {function} on-chain"));
    let fee_microcredits = match &fee_record {
        Some(fee_record) => ProgramManager::validate_amount(fee_credits, fee_record, true)?,
        None => (fee_credits * 1_000_000.0) as u64,
    };

    let mut process_native = ProcessNative::<N>::load_web().map_err(|err| err.to_string())?;
    let process = &mut process_native;
    let pk_native = PrivateKeyNative::<N>::from_str(&**private_key).unwrap();

    log("Check program imports are valid and add them to the process");
    let program_native = ProgramNative::<N>::from_str(&program).map_err(|e| e.to_string())?;
    program_manager_resolve_imports_impl::<N>(process, &program_native, imports)?;

    let stack = process.get_stack("credits.aleo").map_err(|e| e.to_string())?;
    let fee_identifier = if fee_record.is_some() {
        IdentifierNative::<N>::from_str("fee_private").unwrap()
    } else {
        IdentifierNative::<N>::from_str("fee_public").unwrap()
    };

    if !stack.contains_proving_key(&fee_identifier) && fee_proving_key.is_some() && fee_verifying_key.is_some() {
        let fee_proving_key = fee_proving_key.unwrap();
        let fee_verifying_key = fee_verifying_key.unwrap();
        stack
            .insert_proving_key(&fee_identifier, ProvingKeyNative::<N>::from(fee_proving_key))
            .map_err(|e| e.to_string())?;
        stack
            .insert_verifying_key(&fee_identifier, VerifyingKeyNative::<N>::from(fee_verifying_key))
            .map_err(|e| e.to_string())?;
    }

    log("Executing program");
    let rng = &mut StdRng::from_entropy();
    let (_, mut trace) =
        execute_program!(process, process_inputs!(inputs), &program, &function, &pk_native, proving_key, verifying_key, rng);

    log("Preparing inclusion proofs for execution");
    // Prepare the inclusion proofs for the fee & execution
    let query = QueryNative::<N>::from(url);
    trace.prepare_async(query).await.map_err(|err| err.to_string())?;

    log("Proving execution");
    // Prove the execution and fee
    let program = ProgramNative::<N>::from_str(&program).map_err(|err| err.to_string())?;
    let locator = program.id().to_string().add("/").add(&function);
    let execution = trace
        .prove_execution_web::<A, _>(&locator, inclusion_key.clone().into(), &mut StdRng::from_entropy())
        .map_err(|e| e.to_string())?;

    log("Created inclusion");
    let execution_id = execution.to_execution_id().map_err(|e| e.to_string())?;

    let fee_authorization = match fee_record {
        Some(fee_record) => {
            process.authorize_fee_private::<A, _>(
                &pk_native,
                fee_record.into(),
                fee_microcredits,
                0u64,
                execution_id,
                &mut StdRng::from_entropy()
            ).map_err(|e| e.to_string())?
        }
        None => {
            process.authorize_fee_public::<A, _>(&pk_native, fee_microcredits, 0u64, execution_id, &mut StdRng::from_entropy()).map_err(|e| e.to_string())?
        }
    };

    let rng = &mut StdRng::from_entropy();
    let (_, mut trace) = process
        .execute::<A, _>(
            fee_authorization,
            rng
        )
        .map_err(|err| err.to_string())?;

    log("Created fee");
    let query = QueryNative::<N>::from(url);
    trace.prepare_async(query).await.map_err(|err| err.to_string())?;
    log("Prepared fee");
    let fee = trace.prove_fee_web::<A, _>(inclusion_key.into(), &mut StdRng::from_entropy()).map_err(|e| e.to_string())?;

    log("Proved fee");

    // Verify the execution and fee
    process.verify_execution(&execution).map_err(|err| err.to_string())?;
    process.verify_fee(&fee, execution_id).map_err(|err| err.to_string())?;

    log("Creating execution transaction");
    let t_native = TransactionNative::<N>::from_execution(execution, Some(fee)).map_err(|err| err.to_string())?;
    let t_wasm: Transaction = t_native.into();
    Ok(t_wasm)
   }

   pub async fn build_execution_impl<N: Network, A: Aleo<Network = N>>(
    private_key: &PrivateKey,
    program: &str,
    function: &str,
    inputs: Array,
    url: &str,
    imports: Option<Object>,
    proving_key: Option<ProvingKey>,
    verifying_key: Option<VerifyingKey>,
    inclusion_key: ProvingKey,
   ) -> Result<String, String> {
    log(&format!("Executing function: {function} on-chain"));
    let mut process_native = ProcessNative::<N>::load_web().map_err(|err| err.to_string())?;
    let process = &mut process_native;
    let pk_native = PrivateKeyNative::<N>::from_str(&**private_key).unwrap();

    log("Check program imports are valid and add them to the process");
    let program_native = ProgramNative::<N>::from_str(&program).map_err(|e| e.to_string())?;
    program_manager_resolve_imports_impl::<N>(process, &program_native, imports)?;

    let rng = &mut StdRng::from_entropy();
    let (_, mut trace) =
        execute_program!(process, process_inputs!(inputs), &program, &function, &pk_native, proving_key, verifying_key, rng);

    log("Creating inclusion");
    // Prepare the inclusion proofs for the fee & execution
    let query = QueryNative::<N>::from(url);
    trace.prepare_async(query).await.map_err(|err| err.to_string())?;

    // Prove the execution and fee
    let locator = program_native.id().to_string().add("/").add(&function);
    let execution = trace
        .prove_execution_web::<A, _>(&locator, inclusion_key.clone().into(), &mut StdRng::from_entropy())
        .map_err(|e| e.to_string())?;

    log("Created inclusion");

    process.verify_execution(&execution).map_err(|err| err.to_string())?;

    let execution_string = serde_json::to_string(&execution)
        .map_err(|_| "Could not serialize execution".to_string())?;

    Ok(execution_string)
   }

pub async fn authorize_transaction_impl<N: Network, A: Aleo<Network = N>>(
    private_key: &PrivateKey,
    program: &str,
    function: &str,
    inputs: Array,
    fee_credits: f64,
    fee_record: Option<RecordPlaintext>,
    imports: Option<Object>,
) -> Result<String, String> {
  log(&format!("Authorizing function: {function} on-chain"));
  let fee_microcredits = match &fee_record {
      Some(fee_record) => ProgramManager::validate_amount(fee_credits, fee_record, true)?,
      None => (fee_credits * 1_000_000.0) as u64,
  };

  let mut process_native = ProcessNative::<N>::load_web().map_err(|err| err.to_string())?;
  let process = &mut process_native;
  let pk_native = PrivateKeyNative::<N>::from_str(&**private_key).unwrap();

  log("Check program imports are valid and add them to the process");
  let program_native = ProgramNative::<N>::from_str(&program).map_err(|e| e.to_string())?;
  program_manager_resolve_imports_impl::<N>(process, &program_native, imports)?;
  let program_id = program_native.id();
  if program_id.to_string() != "credits.aleo" {
      process.add_program(&program_native).map_err(|e| e.to_string())?;
  }
  
  log("Creating authorization");
  let rng = &mut StdRng::from_entropy();
  let authorization = process
      .authorize::<A, _>(
          &pk_native,
          program_id,
          function,
          process_inputs!(inputs).iter(),
          rng,
      )
      .map_err(|err| err.to_string())?;

  let execution_id = authorization.to_execution_id().map_err(|e| e.to_string())?;

  let fee_authorization = match fee_record {
      Some(fee_record) => {
          process.authorize_fee_private::<A, _>(
              &pk_native,
              fee_record.into(),
              fee_microcredits,
              0u64,
              execution_id,
              &mut StdRng::from_entropy()
          ).map_err(|e| e.to_string())?
      }
      None => {
          process.authorize_fee_public::<A, _>(&pk_native, fee_microcredits, 0u64, execution_id, &mut StdRng::from_entropy()).map_err(|e| e.to_string())?
      }
  };

  let authorization_response = AuthorizationResponse {
      authorization: authorization.to_string(),
      fee_authorization: fee_authorization.to_string(),
      program: program_native.id().to_string(),
  };

  let authorization_response = serde_json::to_string(&authorization_response)
      .map_err(|_| "Could not serialize authorization response".to_string())?;

  Ok(authorization_response)
}

pub async fn execute_authorization_impl<N: Network, A: Aleo<Network = N>>(
    authorization: &str,
    fee_authorization: Option<String>,
    program: &str,
    function: &str,
    url: &str,
    imports: Option<Object>,
    proving_key: Option<ProvingKey>,
    verifying_key: Option<VerifyingKey>,
    fee_proving_key: Option<ProvingKey>,
    fee_verifying_key: Option<VerifyingKey>,
    inclusion_key: ProvingKey,
) -> Result<Transaction, String> {
  log(&format!("Authorizing function: {function} on-chain"));
  let authorization = AuthorizationNative::<N>::from_str(&authorization).map_err(|err| err.to_string())?;
  let fee_authorization = match fee_authorization {
      Some(fee_authorization) => Some(AuthorizationNative::<N>::from_str(&fee_authorization).map_err(|err| err.to_string())?),
      None => None,
  };
  
  let mut process_native = ProcessNative::<N>::load_web().map_err(|err| err.to_string())?;
  let process = &mut process_native;

  log("Check program imports are valid and add them to the process");
  let program_native = ProgramNative::<N>::from_str(&program).map_err(|e| e.to_string())?;
  program_manager_resolve_imports_impl::<N>(process, &program_native, imports)?;

  let stack = process.get_stack("credits.aleo").map_err(|e| e.to_string())?;

  let fee_auth_clone = fee_authorization.clone();
  let fee_identifier = if fee_auth_clone.is_some() && fee_auth_clone.unwrap().is_fee_private() {
      IdentifierNative::<N>::from_str("fee_private").unwrap()
  } else {
      IdentifierNative::<N>::from_str("fee_public").unwrap()
  };

  if !stack.contains_proving_key(&fee_identifier) && fee_proving_key.is_some() && fee_verifying_key.is_some() {
      let fee_proving_key = fee_proving_key.unwrap();
      let fee_verifying_key = fee_verifying_key.unwrap();
      stack
          .insert_proving_key(&fee_identifier, ProvingKeyNative::<N>::from(fee_proving_key))
          .map_err(|e| e.to_string())?;
      stack
          .insert_verifying_key(&fee_identifier, VerifyingKeyNative::<N>::from(fee_verifying_key))
          .map_err(|e| e.to_string())?;
  }

  let program_id = program_native.id();
  if program_id.to_string() != "credits.aleo" {
      process.add_program(&program_native).map_err(|e| e.to_string())?;
  }

  let function_name = IdentifierNative::<N>::from_str(function).map_err(|err| err.to_string())?;
  if let Some(proving_key) = proving_key {
      if contains_key::<N>(process, program_id, &function_name) {
          log(&format!("Proving & verifying keys were specified for {program_id} - {function_name:?} but a key already exists in the cache. Using cached keys"));
      } else {
          log(&format!("Inserting externally provided proving and verifying keys for {program_id} - {function_name:?}"));
          process
              .insert_proving_key(program_id, &function_name, ProvingKeyNative::<N>::from(proving_key))
              .map_err(|e| e.to_string())?;
          if let Some(verifying_key) = verifying_key {
              process.insert_verifying_key(program_id, &function_name, VerifyingKeyNative::<N>::from(verifying_key)).map_err(|e| e.to_string())?;
          }
      }
  };

  log("Executing program");
  let rng = &mut StdRng::from_entropy();
  let (_, mut trace) = process
      .execute::<A, _>(authorization, rng)
      .map_err(|err| err.to_string())?;

  log("Preparing inclusion proofs for execution");
  // Prepare the inclusion proofs for the fee & execution
  let query = QueryNative::<N>::from(url);
  trace.prepare_async(query).await.map_err(|err| err.to_string())?;

  log("Proving execution");
  // Prove the execution and fee
  let program = ProgramNative::<N>::from_str(&program).map_err(|err| err.to_string())?;
  let locator = program.id().to_string().add("/").add(&function);
  let execution = trace
      .prove_execution_web::<A, _>(&locator, inclusion_key.clone().into(), &mut StdRng::from_entropy())
      .map_err(|e| e.to_string())?;

  log("Created inclusion");
  let execution_id = execution.to_execution_id().map_err(|e| e.to_string())?;

  let fee = match fee_authorization {
      Some(fee_authorization) => {
          let rng = &mut StdRng::from_entropy();
          let (_, mut trace) = process
              .execute::<A, _>(fee_authorization, rng)
              .map_err(|err| err.to_string())?;
          log("Created fee");
          let query = QueryNative::<N>::from(url);
          trace.prepare_async(query).await.map_err(|err| err.to_string())?;
          log("Prepared fee");
          let fee = trace.prove_fee_web::<A, _>(inclusion_key.into(), &mut StdRng::from_entropy()).map_err(|e| e.to_string())?;

          log("Proved fee");
          process.verify_fee(&fee, execution_id).map_err(|err| err.to_string())?;

          Some(fee)
          
      }
      None => None,
  };

  // Verify the execution and fee
  process.verify_execution(&execution).map_err(|err| err.to_string())?;

  log("Creating execution transaction");
  let t_native = TransactionNative::<N>::from_execution(execution, fee).map_err(|err| err.to_string())?;
  let t_wasm: Transaction = t_native.into();
  Ok(t_wasm)
}