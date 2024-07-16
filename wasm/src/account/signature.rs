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

use crate::account::{Address, PrivateKey};

use crate::types::native::{AddressNative, PrivateKeyNative, SignatureNative};
use core::{fmt, ops::Deref, str::FromStr};
use rand::{rngs::StdRng, SeedableRng};
use wasm_bindgen::prelude::*;
use crate::Network;

/// Cryptographic signature of a message signed by an Aleo account
#[wasm_bindgen]
pub struct Signature {
  #[wasm_bindgen(skip)]
  pub network: String,
  as_string: String
}

#[wasm_bindgen]
impl Signature {
    /// Sign a message with a private key
    ///
    /// @param {PrivateKey} private_key The private key to sign the message with
    /// @param {Uint8Array} message Byte representation of the message to sign
    /// @returns {Signature} Signature of the message
    pub fn sign(private_key: &PrivateKey, message: &[u8]) -> Result<Signature, String> {
      match dispatch_network!(private_key.network.as_str(), signature_sign_impl, private_key, message) {
        Ok(result) => Ok(result),
        Err(e) => return Err(e),
      }
    }

    /// Verify a signature of a message with an address
    ///
    /// @param {Address} address The address to verify the signature with
    /// @param {Uint8Array} message Byte representation of the message to verify
    /// @returns {boolean} True if the signature is valid, false otherwise
    pub fn verify(&self, address: &Address, message: &[u8]) -> bool {
      match dispatch_network!(self.network.as_str(), signature_verify_impl, self, address, message) {
        Ok(result) => result,
        Err(_) => false,
      }
    }

    /// Get a signature from a string representation of a signature
    ///
    /// @param {string} signature String representation of a signature
    /// @returns {Signature} Signature
    pub fn from_string(network: &str, signature: &str) -> Result<Signature, String> {
      match dispatch_network!(network, signature_from_string_impl, signature) {
        Ok(result) => Ok(result),
        Err(e) => return Err(e.to_string()),
      }
    }

    /// Get a string representation of a signature
    ///
    /// @returns {string} String representation of a signature
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        self.as_string.clone()
    }
}

pub fn signature_sign_impl<N: Network>(private_key: &PrivateKey, message: &[u8]) -> Result<Signature, String> {
  let pk_native = PrivateKeyNative::<N>::from_str(&**private_key).unwrap();
  let signature = SignatureNative::<N>::sign_bytes(&pk_native, message, &mut StdRng::from_entropy()).unwrap();
  let network = network_string_id!(N::ID).unwrap().to_string();
  Ok(Signature { network, as_string: signature.to_string() })
}

pub fn signature_verify_impl<N: Network>(signature: &Signature, address: &Address, message: &[u8]) -> Result<bool, String> {
  let s_native = SignatureNative::<N>::from_str(&signature.as_string).unwrap();
  let address_native = AddressNative::<N>::from_str(&address.to_string()).unwrap();
  Ok(s_native.verify_bytes(&address_native, message))
}

pub fn signature_from_string_impl<N: Network>(signature: &str) -> Result<Signature, String> {
  let s_native = SignatureNative::<N>::from_str(signature).unwrap();
  let network = network_string_id!(N::ID).unwrap().to_string();
  Ok(Signature { network, as_string: s_native.to_string()})
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_string)
    }
}

impl Deref for Signature {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.as_string
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rand::{rngs::StdRng, Rng, SeedableRng};
    use wasm_bindgen_test::*;

    const ITERATIONS: u64 = 1_000;

    // #[wasm_bindgen_test]
    // pub fn test_sign_and_verify() {
    //     for _ in 0..ITERATIONS {
    //         // Sample a new private key and message.
    //         let private_key = PrivateKey::new();
    //         let message: [u8; 32] = StdRng::from_entropy().gen();

    //         // Sign the message.
    //         let signature = Signature::sign(&private_key, &message);
    //         // Check the signature is valid.
    //         assert!(signature.verify(&private_key.to_address(), &message));

    //         // Sample a different message.
    //         let bad_message: [u8; 32] = StdRng::from_entropy().gen();
    //         // Check the signature is invalid.
    //         assert!(!signature.verify(&private_key.to_address(), &bad_message));
    //     }
    // }
}
