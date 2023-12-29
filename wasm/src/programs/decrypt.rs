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


pub use super::*;

use std::str::FromStr;

use crate::{
  account::ViewKey,
  types::native::{
    TransitionNative,
    CurrentNetwork,
    ProgramIDNative,
    CiphertextNative,
    IdentifierNative
  }
};
use snarkvm_console::{prelude::Network, types::{U16, Field, Group}, program::ToBits};
use snarkvm_ledger_block::{Input, Output};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct DecryptTransition {}

#[wasm_bindgen]
impl DecryptTransition {
  pub fn owns_transition(
    view_key: ViewKey,
    tpk_str: &str,
    tcm_str: &str,
  ) -> Result<bool, String> {
    console_error_panic_hook::set_once();

    let tpk = Group::<CurrentNetwork>::from_str(tpk_str)
      .map_err(|_| "Could not deserialize transition public key".to_string())?;

    let tcm = Field::<CurrentNetwork>::from_str(tcm_str)
      .map_err(|_| "Could not deserialize transition commitment".to_string())?;

    let scalar = *view_key;
    let tvk = (tpk * *scalar).to_x_coordinate();

    let tcm_derived = CurrentNetwork::hash_psd2(&[tvk])
      .map_err(|_| "Could not deserialize transition".to_string())?;

    return Ok(tcm == tcm_derived);
  }

  pub fn decrypt_ciphertext(
    view_key: ViewKey,
    ciphertext_str: &str,
    tpk_str: &str,
    program_id: &str,
    function_name_str: &str,
    index: usize,
  ) -> Result<String, String> {
    console_error_panic_hook::set_once();

    let tpk = Group::<CurrentNetwork>::from_str(tpk_str)
      .map_err(|_| "Could not deserialize transition public key".to_string())?;

    let program_id = ProgramIDNative::from_str(program_id)
      .map_err(|_| "Could not deserialize program id".to_string())?;

    let function_name = IdentifierNative::from_str(function_name_str)
      .map_err(|_| "Could not deserialize function name".to_string())?;

    let scalar = *view_key;
    let tvk = (tpk * *scalar).to_x_coordinate();

    let function_id = CurrentNetwork::hash_bhp1024(
      &(U16::<CurrentNetwork>::new(CurrentNetwork::ID), program_id.name(), program_id.network(), function_name).to_bits_le(),
    ).map_err(|_| "Could not create function id".to_string())?;

    let index_field = Field::from_u16(u16::try_from(index).unwrap());
    let ciphertext_view_key = CurrentNetwork::hash_psd4(&[function_id, tvk, index_field])
      .map_err(|_| "Could not create ciphertext view key".to_string())?;

    let ciphertext = CiphertextNative::from_str(ciphertext_str)
      .map_err(|_| "Could not deserialize ciphertext".to_string())?;

    let plaintext = ciphertext.decrypt_symmetric(ciphertext_view_key)
      .map_err(|_| "Could not decrypt ciphertext".to_string())?;

    Ok(plaintext.to_string())
  }

  pub fn decrypt_transition(
    view_key: ViewKey,
    transition_str: &str
  ) -> Result<String, String> {
    console_error_panic_hook::set_once();

    let transition: TransitionNative = serde_json::from_str(transition_str)
      .map_err(|_| "Could not deserialize transition".to_string())?;

    let scalar = *view_key;
    let tvk = (*transition.tpk() * *scalar).to_x_coordinate();

    let function_id = CurrentNetwork::hash_bhp1024(
      &(U16::<CurrentNetwork>::new(CurrentNetwork::ID), transition.program_id().name(), transition.program_id().network(), transition.function_name()).to_bits_le(),
    ).map_err(|_| "Could not create function id".to_string())?;

    let mut decrypted_inputs: Vec<Input<CurrentNetwork>> = vec![]; 
    let mut decrypted_outputs: Vec<Output<CurrentNetwork>> = vec![];

    for (index, input) in transition.inputs().iter().enumerate() {
      if let Input::Private(id, ciphertext_option) = input {
        if let Some(ciphertext) = ciphertext_option {
          let index_field = Field::from_u16(u16::try_from(index).unwrap());
          let input_view_key = CurrentNetwork::hash_psd4(&[function_id, tvk, index_field])
            .map_err(|_| "Could not create input view key".to_string())?;
          let plaintext = ciphertext.decrypt_symmetric(input_view_key).unwrap();
          decrypted_inputs.push(Input::Public(*id, Some(plaintext)));
        } else {
          decrypted_inputs.push(input.clone());
        }
      } else {
          decrypted_inputs.push(input.clone());
      }
    }

    let num_inputs = transition.inputs().len();
    for (index, output) in transition.outputs().iter().enumerate() {
      if let Output::Private(id, ciphertext_option) = output {
        if let Some(ciphertext) = ciphertext_option {
          let index_field = Field::from_u16(u16::try_from(num_inputs + index).unwrap());
          let output_view_key = CurrentNetwork::hash_psd4(&[function_id, tvk, index_field])
            .map_err(|_| "Could not create output view key".to_string())?;
          let plaintext = ciphertext.decrypt_symmetric(output_view_key).unwrap();
          decrypted_outputs.push(Output::Public(*id, Some(plaintext)));
        } else {
          decrypted_outputs.push(output.clone());
        }
      } else {
          decrypted_outputs.push(output.clone());
      }
    }

    let decrypted_transition = TransitionNative::new(
      *transition.program_id(),
      *transition.function_name(),
      decrypted_inputs,
      decrypted_outputs,
      *transition.tpk(),
      *transition.tcm()
    ).unwrap();

    let transition_output = serde_json::to_string(&decrypted_transition)
        .map_err(|_| "Could not serialize decrypted transition".to_string())?;

    Ok(transition_output)
  }
}
