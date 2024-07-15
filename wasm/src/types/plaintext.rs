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

use crate::types::native::PlaintextNative;
use crate::native::Network;
use snarkvm_console::prelude::ToBits;

use snarkvm_wasm::utilities::ToBytes;
use wasm_bindgen::prelude::wasm_bindgen;

use std::str::FromStr;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Plaintext{
  network: String,
  as_string: String
}

#[wasm_bindgen]
impl Plaintext {
    #[wasm_bindgen(js_name = "toString")]
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
      self.as_string.clone()
    }

    #[wasm_bindgen(js_name = "fromString")]
    pub fn from_string(network: &str, plaintext: &str) -> Result<Plaintext, String> {
      match dispatch_network!(network, plaintext_from_string_impl, plaintext) {
        Ok(result) => Ok(Self{ network: network.to_string(), as_string: result}),
        Err(e) => return Err(e)
      }
    }

    #[wasm_bindgen(js_name = "toBytes")]
    pub fn to_bits(&self) -> Result<Vec<u8>, String> {
      self.as_string.to_bits_le().to_bytes_le().map_err(|err| err.to_string())
    }
}

pub fn plaintext_from_string_impl<N: Network>(plaintext: &str) -> Result<String, String> {
  let plaintext_string = PlaintextNative::<N>::from_str(plaintext).map_err(|e| e.to_string())?.to_string();
  Ok(plaintext_string)
}

impl<N: Network> From<PlaintextNative<N>> for Plaintext {
    fn from(native: PlaintextNative<N>) -> Self {
      let network = network_string_id!(N::ID).unwrap().to_string();
      Self { network, as_string: native.to_string() }
    }
}

impl<N: Network> From<Plaintext> for PlaintextNative<N> {
    fn from(plaintext: Plaintext) -> Self {
      PlaintextNative::<N>::from_str(&plaintext.as_string).unwrap()
    }
}
