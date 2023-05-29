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

use std::{ops::Deref};

use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct ProvingKey(ProvingKeyNative);

#[wasm_bindgen]
impl ProvingKey {
    #[wasm_bindgen]
    pub fn from_bytes(bytes: &[u8]) -> ProvingKey {
        // console_error_panic_hook::set_once();
        ProvingKeyNative::from_bytes_le(bytes).map(|proving_key| Self(proving_key))
            .expect("Failed to deserialize prover key")
    }

    #[wasm_bindgen]
    pub fn to_randomness_bytes(&self) -> Result<Vec<u8>, String> {
        console_error_panic_hook::set_once();
        self.0.circuit_commitment_randomness.to_bytes_le().map_err(|_| "Failed to serialize prover key".to_string())
    }

    #[wasm_bindgen]
    pub fn to_committer_bytes(&self) -> Result<Vec<u8>, String> {
        console_error_panic_hook::set_once();
        self.0.committer_key.to_bytes_le().map_err(|_| "Failed to serialize prover key".to_string())
    }

    #[wasm_bindgen]
    pub fn to_verifying_bytes(&self) -> Result<Vec<u8>, String> {
        console_error_panic_hook::set_once();
        self.0.circuit_verifying_key.to_bytes_le().map_err(|_| "Failed to serialize prover key".to_string())
    }

    #[wasm_bindgen]
    pub fn to_bytes(&self) -> Result<Vec<u8>, String> {
        console_error_panic_hook::set_once();
        self.0.to_bytes_le().map_err(|_| "Failed to serialize prover key".to_string())
    }
}

impl Deref for ProvingKey {
    type Target = ProvingKeyNative;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<ProvingKey> for ProvingKeyNative {
    fn from(proving_key: ProvingKey) -> ProvingKeyNative {
        proving_key.0
    }
}
