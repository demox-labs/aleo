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
  types::{VerifyingKeyNative, ToBytes, FromBytes},
};

use std::{ops::Deref};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct VerifyingKey(VerifyingKeyNative);

#[wasm_bindgen]
impl VerifyingKey {
    #[wasm_bindgen]
    pub fn from_bytes(bytes: Vec<u8>) -> VerifyingKey {
        console_error_panic_hook::set_once();
        let verifying_key = VerifyingKeyNative::from_bytes_le(&bytes);
        Self(verifying_key.unwrap())
    }

    #[wasm_bindgen]
    pub fn to_bytes(&self) -> Result<Vec<u8>, String> {
        console_error_panic_hook::set_once();
        self.0.to_bytes_le().map_err(|_| "Failed to serialize prover key".to_string())
    }
}

impl Deref for VerifyingKey {
    type Target = VerifyingKeyNative;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<VerifyingKey> for VerifyingKeyNative {
    fn from(verifying_key: VerifyingKey) -> VerifyingKeyNative {
        verifying_key.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_deserialize_serialize_key_wasm() {
        let transfer_bytes = include_bytes!(concat!(env!("HOME"), "/.aleo/resources/transfer.verifier.db46e4c")).to_vec();
        let verifying_key = VerifyingKey::from_bytes(transfer_bytes.clone());
        let bytes = verifying_key.to_bytes().unwrap();
        assert!(transfer_bytes.eq(&bytes));
    }
}