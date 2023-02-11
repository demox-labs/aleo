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
  types::{ProvingKeyNative, ToBytes, FromBytes},
};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvingKey(ProvingKeyNative);

#[wasm_bindgen]
impl ProvingKey {
    /// Generate a new private key
    #[wasm_bindgen]
    #[allow(clippy::new_without_default)]
    pub fn from_bytes(bytes: Vec<u8>) -> Vec<u8> {
        console_error_panic_hook::set_once();
        let key = ProvingKeyNative::from_bytes_le(&bytes[2..]).unwrap();
        let recovered_bytes = key.to_bytes_le().unwrap();
        let first_1000 = &recovered_bytes[0..1000];
        first_1000.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use wasm_bindgen_test::*;
    use web_sys::console;

    fn get_transfer_bytes() -> Vec<u8> {
        include_bytes!("/Users/evanmarshall/.aleo/resources/transfer.prover.837ad21").to_vec()
    }

    #[test]
    fn test_deserialize_key() {
        let transfer_bytes = get_transfer_bytes();
        let bytes = ProvingKey::from_bytes(transfer_bytes);
        println!("First 1000 bytes: {:?}", bytes);
    }

    #[wasm_bindgen_test]
    fn test_deserialize_key_wasm() {
        let transfer_bytes = get_transfer_bytes();
        let bytes = ProvingKey::from_bytes(transfer_bytes);
        let formatted_string = format!("First 1000 bytes: {:?}", bytes);
        console::log_1(&formatted_string.into());
    }
}