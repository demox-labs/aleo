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

// mod credits;
// pub use credits::*;

use crate::types::native::{FromBytes, ProvingKeyNative, ToBytes};
use crate::Network;

use sha2::Digest;
use wasm_bindgen::prelude::wasm_bindgen;

use std::{ops::Deref, str::FromStr};

/// Proving key for a function within an Aleo program
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct ProvingKey {
  #[wasm_bindgen(skip)]
  pub network: String,
  as_string: String
}

#[wasm_bindgen]
impl ProvingKey {
    /// Return the checksum of the proving key
    ///
    /// @returns {string} Checksum of the proving key
    pub fn checksum(&self) -> Result<String, String> {
      match dispatch_network!(self.network.as_str(), proving_key_checksum_impl, &self) {
        Ok(checksum) => Ok(checksum),
        Err(e) => Err(e),
      }
    }

    /// Create a copy of the proving key
    ///
    /// @returns {ProvingKey} A copy of the proving key
    #[wasm_bindgen]
    pub fn copy(&self) -> ProvingKey {
        ProvingKey { network: self.network.clone(), as_string: self.as_string.clone() }
    }

    /// Construct a new proving key from a byte array
    ///
    /// @param {Uint8Array} bytes Byte array representation of a proving key
    /// @returns {ProvingKey | Error}
    #[wasm_bindgen(js_name = "fromBytes")]
    pub fn from_bytes(network: &str, bytes: &[u8]) -> Result<ProvingKey, String> {
      match dispatch_network!(network, proving_key_from_bytes_impl, bytes) {
        Ok(proving_key) => Ok(proving_key),
        Err(e) => Err(e),
      }
    }

    /// Create a proving key from string
    ///
    /// @param {string | Error} String representation of the proving key
    #[wasm_bindgen(js_name = "fromString")]
    pub fn from_string(network: &str, string: &str) -> Result<ProvingKey, String> {
      match dispatch_network!(network, proving_key_from_string_impl, string) {
        Ok(proving_key) => Ok(proving_key),
        Err(e) => Err(e),
      }
    }

    /// Return the byte representation of a proving key
    ///
    /// @returns {Uint8Array | Error} Byte array representation of a proving key
    #[wasm_bindgen(js_name = "toBytes")]
    pub fn to_bytes(&self) -> Result<Vec<u8>, String> {
      match dispatch_network!(self.network.as_str(), proving_key_to_bytes_impl, &self) {
        Ok(bytes) => Ok(bytes),
        Err(e) => Err(e),
      }
    }

    /// Get a string representation of the proving key
    ///
    /// @returns {string} String representation of the proving key
    #[wasm_bindgen(js_name = "toString")]
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        format!("{:?}", self.as_string)
    }
}

pub fn proving_key_to_bytes_impl<N: Network>(proving_key: &ProvingKey) -> Result<Vec<u8>, String> {
  let pk_native = ProvingKeyNative::<N>::from_str(&proving_key.as_string).map_err(|e| e.to_string())?;
  pk_native.to_bytes_le().map_err(|_| "Failed to serialize proving key".to_string())
}

pub fn proving_key_from_string_impl<N: Network>(proving_key: &str) -> Result<ProvingKey, String> {
  let pk_native = ProvingKeyNative::<N>::from_str(proving_key).map_err(|e| e.to_string())?;
  let network = network_string_id!(N::ID).unwrap().to_string();
  Ok(ProvingKey { network, as_string: pk_native.to_string() })
}

pub fn proving_key_from_bytes_impl<N: Network>(bytes: &[u8]) -> Result<ProvingKey, String> {
  let pk_native = ProvingKeyNative::<N>::from_bytes_le(bytes).map_err(|e| e.to_string())?;
  let network = network_string_id!(N::ID).unwrap().to_string();
  Ok(ProvingKey { network, as_string: pk_native.to_string() })
}

pub fn proving_key_checksum_impl<N: Network>(proving_key: &ProvingKey) -> Result<String, String> {
  let pk_native = ProvingKeyNative::<N>::from_str(&proving_key.as_string).map_err(|e| e.to_string())?;
  Ok(hex::encode(sha2::Sha256::digest(pk_native.to_bytes_le().unwrap())))
}

impl Deref for ProvingKey {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.as_string
    }
}

impl<N: Network> From<ProvingKey> for ProvingKeyNative<N> {
    fn from(proving_key: ProvingKey) -> ProvingKeyNative<N> {
        ProvingKeyNative::<N>::from_str(&proving_key.as_string).unwrap()
    }
}

impl<N: Network> From<ProvingKeyNative<N>> for ProvingKey {
    fn from(proving_key: ProvingKeyNative<N>) -> ProvingKey {
      let network = network_string_id!(N::ID).unwrap().to_string();
      ProvingKey { network, as_string: proving_key.to_string() }
    }
}

impl PartialEq for ProvingKey {
    fn eq(&self, other: &Self) -> bool {
        *self.as_string == *other.as_string && *self.network == *other.network
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    const TRANSFER_PUBLIC_PROVER: &str = "https://testnet3.parameters.aleo.org/transfer_public.prover.a74565e";

    #[wasm_bindgen_test]
    async fn test_proving_key_roundtrip() {
        let fee_proving_key_bytes = reqwest::get(TRANSFER_PUBLIC_PROVER).await.unwrap().bytes().await.unwrap().to_vec();
        let fee_proving_key = ProvingKey::from_bytes(&fee_proving_key_bytes).unwrap();
        let bytes = fee_proving_key.to_bytes().unwrap();
        assert_eq!(bytes, fee_proving_key_bytes);
    }

    #[wasm_bindgen_test]
    async fn test_from_string() {
        let transfer_public_prover =
            reqwest::get(TRANSFER_PUBLIC_PROVER).await.unwrap().bytes().await.unwrap().to_vec();
        let transfer_public_proving_key = ProvingKey::from_bytes(&transfer_public_prover).unwrap();
        let transfer_public_proving_key_string = transfer_public_proving_key.to_string();
        let transfer_public_proving_key_from_string =
            ProvingKey::from_string(&transfer_public_proving_key_string).unwrap();
        assert_eq!(transfer_public_proving_key, transfer_public_proving_key_from_string);
    }

    #[wasm_bindgen_test]
    async fn test_prover_checksum() {
        let transfer_public_prover =
            reqwest::get(TRANSFER_PUBLIC_PROVER).await.unwrap().bytes().await.unwrap().to_vec();
        let transfer_public_proving_key = ProvingKey::from_bytes(&transfer_public_prover).unwrap();
        let transfer_public_proving_key_checksum = transfer_public_proving_key.checksum();
        assert_eq!(
            transfer_public_proving_key_checksum,
            "a74565e4fd408a90b2d04b0e6c0dea6bf0ab6a27926ef28049da62d18727f6c6"
        );
    }
}
