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

use crate::{execute_fee, log, PrivateKey, RecordPlaintext, Transaction, Network};

use crate::types::native::{
  PrivateKeyNative,
  DeploymentNative,
  ProcessNative,
  ProgramIDNative,
  ProgramNative,
  ProgramOwnerNative,
  RecordPlaintextNative,
  TransactionNative
};
use snarkvm_circuit_network::Aleo;
use js_sys::Object;
use rand::{rngs::StdRng, SeedableRng};
use serde::Serialize;
use std::str::FromStr;

#[derive(Serialize)]
pub struct DeployAuthorizationResponse {
    pub deployment: String,
    pub fee_authorization: String,
    pub owner: String
}

#[wasm_bindgen]
impl ProgramManager {
    /// Estimate the component of the deployment cost which comes from the fee for the program name.
    /// Note that this cost does not represent the entire cost of deployment. It is additional to
    /// the cost of the size (in bytes) of the deployment.
    ///
    /// Disclaimer: Fee estimation is experimental and may not represent a correct estimate on any current or future network
    ///
    /// @param name The name of the program to be deployed
    /// @returns {u64 | Error}
    #[wasm_bindgen(js_name = estimateProgramNameCost)]
    pub fn program_name_cost(name: &str) -> Result<u64, String> {
        log(
            "Disclaimer: Fee estimation is experimental and may not represent a correct estimate on any current or future network",
        );
        let num_characters = name.chars().count() as u32;
        let namespace_cost = 10u64
            .checked_pow(10u32.saturating_sub(num_characters))
            .ok_or("The namespace cost computation overflowed for a deployment")?
            .saturating_mul(1_000_000); // 1 microcredit = 1e-6 credits.
        Ok(namespace_cost)
    }

    #[wasm_bindgen]
    #[allow(clippy::too_many_arguments)]
    pub async fn deploy_transaction(
        private_key: PrivateKey,
        program: String,
        imports: Option<Object>,
        fee_credits: f64,
        fee_record: Option<RecordPlaintext>,
        url: String,
        fee_proving_key: Option<ProvingKey>,
        fee_verifying_key: Option<VerifyingKey>,
        inclusion_key: ProvingKey,
    ) -> Result<Transaction, String> {
        match dispatch_network_aleo!(
          private_key.network.as_str(),
          deploy_deploy_transaction_impl,
          private_key,
          program,
          imports,
          fee_credits,
          fee_record,
          url,
          fee_proving_key,
          fee_verifying_key,
          inclusion_key
        ) {
            Ok(result) => Ok(result),
            Err(e) => return Err(e),
        }
    }

    #[wasm_bindgen]
    pub async fn authorize_deploy(
        private_key: &PrivateKey,
        deployment: &str,
        fee_credits: f64,
        fee_record: Option<RecordPlaintext>
    ) -> Result<String, String> {
      match dispatch_network_aleo!(
        private_key.network.as_str(),
        deploy_authorize_deploy_impl,
        private_key,
        deployment,
        fee_credits,
        fee_record
      ) {
          Ok(result) => Ok(result),
          Err(e) => return Err(e),
      }
    }
}

pub async fn deploy_authorize_deploy_impl<N: Network, A: Aleo<Network = N>>(
  private_key: &PrivateKey,
  deployment: &str,
  fee_credits: f64,
  fee_record: Option<RecordPlaintext>
) -> Result<String, String> {
      log("Creating deployment transaction");
        // Convert fee to microcredits and check that the fee record has enough credits to pay it
        let fee_microcredits = match &fee_record {
            Some(fee_record) => ProgramManager::validate_amount(fee_credits, fee_record, true)?,
            None => (fee_credits * 1_000_000.0) as u64,
        };

        log("Create and validate deployment");
        let deployment = DeploymentNative::<N>::from_str(deployment).map_err(|e| e.to_string())?;
        let deployment_id = deployment.to_deployment_id().map_err(|e| e.to_string())?;

        let mut process_native = ProcessNative::<N>::load_web().map_err(|err| err.to_string())?;
        let process = &mut process_native;
        let pk_native = PrivateKeyNative::<N>::from_str(&**private_key).unwrap();

        let stack = process.get_stack("credits.aleo").map_err(|e| e.to_string())?;
        let fee_identifier = if fee_record.is_some() {
            IdentifierNative::<N>::from_str("fee_private").unwrap()
        } else {
            IdentifierNative::<N>::from_str("fee_public").unwrap()
        };

        log("Build fee execution");
        let fee_authorization = match fee_record {
            Some(fee_record) => {
                process.authorize_fee_private::<A, _>(
                    &pk_native,
                    fee_record.into(),
                    fee_microcredits,
                    0u64,
                    deployment_id,
                    &mut StdRng::from_entropy()
                ).map_err(|e| e.to_string())?
            }
            None => {
                process.authorize_fee_public::<A, _>(&pk_native, fee_microcredits, 0u64, deployment_id, &mut StdRng::from_entropy()).map_err(|e| e.to_string())?
            }
        };

        log("Create the program owner");
        let owner = ProgramOwnerNative::<N>::new(&pk_native, deployment_id, &mut StdRng::from_entropy())
            .map_err(|err| err.to_string())?;

        let authorization_response = DeployAuthorizationResponse {
            deployment: deployment.to_string(),
            fee_authorization: fee_authorization.to_string(),
            owner: owner.to_string()
        };

        let authorization_response = serde_json::to_string(&authorization_response)
            .map_err(|_| "Could not serialize authorization response".to_string())?;

        Ok(authorization_response)
}

pub async fn deploy_deploy_transaction_impl<N: Network, A: Aleo<Network = N>>(
  private_key: PrivateKey,
  program: String,
  imports: Option<Object>,
  fee_credits: f64,
  fee_record: Option<RecordPlaintext>,
  url: String,
  fee_proving_key: Option<ProvingKey>,
  fee_verifying_key: Option<VerifyingKey>,
  inclusion_key: ProvingKey,
) -> Result<Transaction, String> {
  log("Creating deployment transaction");
  // Convert fee to microcredits and check that the fee record has enough credits to pay it
  let fee_microcredits = match &fee_record {
      Some(fee_record) => ProgramManager::validate_amount(fee_credits, fee_record, true)?,
      None => (fee_credits * 1_000_000.0) as u64,
  };

  let mut process_native = ProcessNative::<N>::load_web().map_err(|err| err.to_string())?;
  let process = &mut process_native;
  let pk_native = PrivateKeyNative::<N>::from_str(&**private_key).unwrap();

  log("Check program has a valid name");
  let program = ProgramNative::<N>::from_str(&program).map_err(|err| err.to_string())?;

  log("Checking program imports are valid and add them to the process");
  program_manager_resolve_imports_impl::<N>(process, &program, imports)?;
  
  log("Create and validate deployment");
  let deployment =
      process.deploy::<A, _>(&program, &mut StdRng::from_entropy()).map_err(|err| err.to_string())?;
  if deployment.program().functions().is_empty() {
      return Err("Attempted to create an empty transaction deployment".to_string());
  }

  log("Verify the deployment and fees");
  process
      .verify_deployment::<A, _>(&deployment, &mut StdRng::from_entropy())
      .map_err(|err| err.to_string())?;

  let deployment_id = deployment.to_deployment_id().map_err(|e| e.to_string())?;

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

  let fee_authorization = match fee_record {
      Some(fee_record) => {
          process.authorize_fee_private::<A, _>(
              &pk_native,
              fee_record.into(),
              fee_microcredits,
              0u64,
              deployment_id,
              &mut StdRng::from_entropy()
          ).map_err(|e| e.to_string())?
      }
      None => {
          process.authorize_fee_public::<A, _>(&pk_native, fee_microcredits, 0u64, deployment_id, &mut StdRng::from_entropy()).map_err(|e| e.to_string())?
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
  let query = QueryNative::<N>::from(&url);
  trace.prepare_async(query).await.map_err(|err| err.to_string())?;
  log("Prepared fee");
  let fee = trace.prove_fee_web::<A, _>(inclusion_key.into(), &mut StdRng::from_entropy()).map_err(|e| e.to_string())?;

  log("Proved fee");

  log("Create the program owner");
  let owner = ProgramOwnerNative::<N>::new(&pk_native, deployment_id, &mut StdRng::from_entropy())
      .map_err(|err| err.to_string())?;

  log("Verify the deployment and fees");
  process
      .verify_deployment::<A, _>(&deployment, &mut StdRng::from_entropy())
      .map_err(|err| err.to_string())?;

  log("Creating deployment transaction");
  let t_native = TransactionNative::<N>::from_deployment(owner, deployment, fee).map_err(|err| err.to_string())?;
  let t_wasm: Transaction = t_native.into();
  Ok(t_wasm)
}