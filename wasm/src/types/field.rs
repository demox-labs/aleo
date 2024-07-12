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

use crate::types::native::FieldNative;
use crate::native::Network;
use snarkvm_console::prelude::ToBits;

use wasm_bindgen::prelude::wasm_bindgen;

use std::str::FromStr;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Field{
  network: String,
  as_string: String
}

#[wasm_bindgen]
impl Field {
    #[wasm_bindgen(js_name = "toString")]
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
      self.as_string.clone()
    }

    #[wasm_bindgen(js_name = "fromString")]
    pub fn from_string(network: &str, field: &str) -> Result<Field, String> {
      match dispatch_network!(network, field_from_string_impl, field) {
        Ok(result) => Ok(Self{ network: network.to_string(), as_string: result}),
        Err(e) => return Err(e)
      }
    }

    pub fn bhp256_hash_to_field(network: &str, hash: &[u8]) -> Result<Field, String> {
      match dispatch_network!(network, field_bhp256_hash_to_field_impl, hash) {
        Ok(result) => Ok(Self{ network: network.to_string(), as_string: result}),
        Err(e) => return Err(e)
      }
    }
}

pub fn field_bhp256_hash_to_field_impl<N: Network>(hash: &[u8]) -> Result<String, String> {
  let bits = hash.to_vec().to_bits_le();
  let field_string = N::hash_bhp256(&bits).map_err(|e| e.to_string())?.to_string();
  Ok(field_string)
}

pub fn field_from_string_impl<N: Network>(field: &str) -> Result<String, String> {
  let field_string = FieldNative::<N>::from_str(field).map_err(|e| e.to_string())?.to_string();
  Ok(field_string)
}

impl<N: Network> From<FieldNative<N>> for Field {
    fn from(native: FieldNative<N>) -> Self {
      let network = network_string_id!(N::ID).unwrap().to_string();
      Self { network, as_string: native.to_string() }
    }
}

impl<N: Network> From<Field> for FieldNative<N> {
    fn from(field: Field) -> Self {
      FieldNative::<N>::from_str(&field.as_string).unwrap()
    }
}
