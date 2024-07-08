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
    ProgramIDNative,
    CiphertextNative,
    IdentifierNative,
    ViewKeyNative
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
    network: &str,
    view_key: &ViewKey,
    tpk_str: &str,
    tcm_str: &str,
  ) -> Result<bool, String> {
    console_error_panic_hook::set_once();
    match dispatch_network!(network, owns_transition_impl, view_key, tpk_str, tcm_str) {
      Ok(owns_transition) => Ok(owns_transition),
      Err(e) => Err(e),
    }
  }

  pub fn decrypt_ciphertext(
    network: &str,
    view_key: &ViewKey,
    ciphertext_str: &str,
    tpk_str: &str,
    program_id: &str,
    function_name_str: &str,
    index: usize,
  ) -> Result<String, String> {
    console_error_panic_hook::set_once();

    match dispatch_network!(network, decrypt_ciphertext_impl, view_key, ciphertext_str, tpk_str, program_id, function_name_str, index) {
      Ok(plaintext) => Ok(plaintext),
      Err(e) => Err(e),
    }
  }

  pub fn decrypt_transition(
    network: &str,
    view_key: &ViewKey,
    transition_str: &str
  ) -> Result<String, String> {
    console_error_panic_hook::set_once();
    match dispatch_network!(network, decrypt_transition_impl, view_key, transition_str) {
      Ok(transition) => Ok(transition),
      Err(e) => Err(e),
    }
  }
}

pub fn owns_transition_impl<N: Network>(
  view_key: &ViewKey,
  tpk_str: &str,
  tcm_str: &str
) -> Result<bool, String> {
  let tpk = Group::<N>::from_str(tpk_str)
    .map_err(|_| "Could not deserialize transition public key".to_string())?;

  let tcm = Field::<N>::from_str(tcm_str)
    .map_err(|_| "Could not deserialize transition commitment".to_string())?;

  let vk_native = ViewKeyNative::<N>::from_str(&*view_key)
    .map_err(|_| "Could not deserialize view key".to_string())?;
  let scalar = *vk_native;
  let tvk = (tpk * scalar).to_x_coordinate();

  let tcm_derived = N::hash_psd2(&[tvk])
    .map_err(|_| "Could not deserialize transition".to_string())?;

  return Ok(tcm == tcm_derived);
}

pub fn decrypt_ciphertext_impl<N: Network>(
  view_key: &ViewKey,
  ciphertext_str: &str,
  tpk_str: &str,
  program_id: &str,
  function_name_str: &str,
  index: usize
) -> Result<String, String> {
  let tpk = Group::<N>::from_str(tpk_str)
    .map_err(|_| "Could not deserialize transition public key".to_string())?;

  let program_id = ProgramIDNative::<N>::from_str(program_id)
    .map_err(|_| "Could not deserialize program id".to_string())?;

  let function_name = IdentifierNative::<N>::from_str(function_name_str)
    .map_err(|_| "Could not deserialize function name".to_string())?;

  let vk_native = ViewKeyNative::<N>::from_str(&*view_key)
    .map_err(|_| "Could not deserialize view key".to_string())?;
  let scalar = *vk_native;
  let tvk = (tpk * scalar).to_x_coordinate();

  let function_id = N::hash_bhp1024(
    &(U16::<N>::new(N::ID),
    program_id.name().size_in_bits(),
    program_id.name(),
    program_id.network().size_in_bits(),
    program_id.network(),
    function_name.size_in_bits(),
    function_name
  ).to_bits_le(),
  ).map_err(|_| "Could not create function id".to_string())?;

  let index_field = Field::<N>::from_u16(u16::try_from(index).unwrap());
  let ciphertext_view_key = N::hash_psd4(&[function_id, tvk, index_field])
    .map_err(|_| "Could not create ciphertext view key".to_string())?;

  let ciphertext = CiphertextNative::<N>::from_str(ciphertext_str)
    .map_err(|_| "Could not deserialize ciphertext".to_string())?;

  let plaintext = ciphertext.decrypt_symmetric(ciphertext_view_key)
    .map_err(|e| e.to_string())?;

  Ok(plaintext.to_string())
}

pub fn decrypt_transition_impl<N: Network>(
  view_key: &ViewKey,
  transition_str: &str
) -> Result<String, String> {
  console_error_panic_hook::set_once();

  let transition: TransitionNative<N> = serde_json::from_str(transition_str)
    .map_err(|_| "Could not deserialize transition".to_string())?;

  let vk_native = ViewKeyNative::<N>::from_str(&*view_key)
    .map_err(|_| "Could not deserialize view key".to_string())?;
  let scalar = *vk_native;
  let tvk = (*transition.tpk() * scalar).to_x_coordinate();

  let function_id = N::hash_bhp1024(
    &(U16::<N>::new(N::ID),
    transition.program_id().name().size_in_bits(),
    transition.program_id().name(),
    transition.program_id().network().size_in_bits(),
    transition.program_id().network(),
    transition.function_name().size_in_bits(),
    transition.function_name()
  ).to_bits_le(),
  ).map_err(|_| "Could not create function id".to_string())?;

  let mut decrypted_inputs: Vec<Input<N>> = vec![]; 
  let mut decrypted_outputs: Vec<Output<N>> = vec![];

  for (index, input) in transition.inputs().iter().enumerate() {
    if let Input::Private(id, ciphertext_option) = input {
      if let Some(ciphertext) = ciphertext_option {
        let index_field = Field::from_u16(u16::try_from(index).unwrap());
        let input_view_key = N::hash_psd4(&[function_id, tvk, index_field])
          .map_err(|_| "Could not create input view key".to_string())?;
        let plaintext = ciphertext.decrypt_symmetric(input_view_key)
          .map_err(|e| e.to_string())?;
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
        let output_view_key = N::hash_psd4(&[function_id, tvk, index_field])
          .map_err(|_| "Could not create output view key".to_string())?;
        let plaintext = ciphertext.decrypt_symmetric(output_view_key)
          .map_err(|e| e.to_string())?;
        decrypted_outputs.push(Output::Public(*id, Some(plaintext)));
      } else {
        decrypted_outputs.push(output.clone());
      }
    } else {
        decrypted_outputs.push(output.clone());
    }
  }

  let decrypted_transition = TransitionNative::<N>::new(
    *transition.program_id(),
    *transition.function_name(),
    decrypted_inputs,
    decrypted_outputs,
    *transition.tpk(),
    *transition.tcm(),
    *transition.scm()
  ).unwrap();

  let transition_output = serde_json::to_string(&decrypted_transition)
      .map_err(|_| "Could not serialize decrypted transition".to_string())?;

  Ok(transition_output)
}

// Write a test to check that the decryption of a transition is correct
#[cfg(test)]
mod tests {
  use super::*;
  use crate::account::{PrivateKey, ViewKey};
  use crate::types::native::{TransitionNative, ProgramIDNative, CiphertextNative, IdentifierNative};
  use snarkvm_console::types::Field;
  use snarkvm_console::program::ToBits;
  use snarkvm_console::types::U16;

  #[test]
  fn test_decrypt_transition() {
    let view_key = ViewKey::from_str("AViewKey1d9sQKCg1fpur3UkYLytGPALZ1hsCdmz9tLtiQbqt7T21").unwrap();
    let transition_str = r#"{"id":"au1qlwdvgf22gq645v490c5aa6sspk90zgde2l5q3jrw9p2s4ggncfq4yq4tx","program":"credits.aleo","function":"transfer_public_to_private","inputs":[{"type":"private","id":"5109995434704492516289710482676608996989648767452888020485171839808760694698field","value":"ciphertext1qgq06j04znejshu5y9k5y2nfz25jhflwfk3lfq5drjqzcjakang77z5ts4lksftu0qpha6zrhh2yc7kpcg79dfjzf3qumprx95hc7vu7qyls0fvd"},{"type":"public","id":"3161157639370013645831138307879790339370582521051845617561995466932988115257field","value":"10000000u64"}],"outputs":[{"type":"record","id":"5895707112902843126424716675816600113782878383442992908152423335043259147564field","checksum":"7069102984147140073032970292930254002685215707355246114401158902641333007935field","value":"record1qyqsq2fqczrvtt78wsf02q3jttw6fuqfsdmax2yux9ejeurcttasj9qqqyxx66trwfhkxun9v35hguerqqpqzq9g3aegwx3jadl9np9adkn5csnhxzalltentg3dnpes3khep2l3qa0q6w50w6uhn5dlzjsveq408m5fu7pkpz8n7t0zxhdtxxdy076syjyzepn"},{"type":"future","id":"267626828617319262179665487216822819947560552670678105765385893570851929436field","value":"{\n  program_id: credits.aleo,\n  function_name: transfer_public_to_private,\n  arguments: [\n    aleo1f8pj85z7w6zf59dk2wnf88df2ttnqymqjd3sunj9q82vj7uvp5xs9vvx6t,\n    10000000u64\n  ]\n}"}],"tpk":"3705090958747679190852824318127602306143559325439955270434448404596070617183group","tcm":"1782726546453804967758306961543927542155659117379878730166707720938913681448field","scm":"7985738558187645438997556582845879845493256926977338712413747237095526715414field"}"#;
    let result = DecryptTransition::decrypt_transition(view_key, transition_str);
    assert!(result.is_ok());
  }

  #[test]
  fn test_decrypt_ciphertext() {
    let view_key = ViewKey::from_str("AViewKey1d9sQKCg1fpur3UkYLytGPALZ1hsCdmz9tLtiQbqt7T21").unwrap();
    let ciphertext_str = r#"ciphertext1qgq06j04znejshu5y9k5y2nfz25jhflwfk3lfq5drjqzcjakang77z5ts4lksftu0qpha6zrhh2yc7kpcg79dfjzf3qumprx95hc7vu7qyls0fvd"#;
    let tpk_str = "3705090958747679190852824318127602306143559325439955270434448404596070617183group";
    let program_id = "credits.aleo";
    let function_name_str = "transfer_public_to_private";
    let index = 0;
    let result = DecryptTransition::decrypt_ciphertext(view_key, ciphertext_str, tpk_str, program_id, function_name_str, index);
    result.unwrap();
  }
}