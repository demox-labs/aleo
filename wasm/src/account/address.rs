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

use crate::account::{PrivateKey, ViewKey};

use crate::types::native::{AddressNative, ViewKeyNative, PrivateKeyNative};
use core::{convert::TryFrom, fmt, ops::Deref, str::FromStr};
use wasm_bindgen::prelude::*;
use crate::native::Network;

/// Public address of an Aleo account
#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Address {
  #[wasm_bindgen(skip)]
  pub network: String,
  as_string: String
}

#[wasm_bindgen]
impl Address {
    /// Derive an Aleo address from a private key
    ///
    /// @param {PrivateKey} private_key The private key to derive the address from
    /// @returns {Address} Address corresponding to the private key
    pub fn from_private_key(private_key: &PrivateKey) -> Result<Address, String> {
        match dispatch_network!(private_key.network.as_str(), address_from_private_key_impl, private_key) {
            Ok(result) => Ok(result),
            Err(e) => return Err(e.to_string()),
        }
    }

    /// Derive an Aleo address from a view key
    ///
    /// @param {ViewKey} view_key The view key to derive the address from
    /// @returns {Address} Address corresponding to the view key
    pub fn from_view_key(view_key: &ViewKey) -> Result<Address, String> {
        match dispatch_network!(view_key.network.as_str(), address_from_view_key_impl, view_key) {
            Ok(result) => Ok(result),
            Err(e) => return Err(e.to_string()),
        }
    }

    /// Create an aleo address object from a string representation of an address
    ///
    /// @param {string} address String representation of an addressm
    /// @returns {Address} Address
    pub fn from_string(network: &str, address: &str) -> Result<Address, String> {
        match dispatch_network!(network, address_from_string_impl, address) {
            Ok(result) => Ok(result),
            Err(e) => return Err(e),
        }
    }

    /// Get a string representation of an Aleo address object
    ///
    /// @param {Address} Address
    /// @returns {string} String representation of the address
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        self.as_string.clone()
    }

    pub fn to_x_coordinate(&self) -> Result<String, String> {
      match dispatch_network!(self.network.as_str(), address_to_x_coordinate_impl, &self.as_string) {
        Ok(result) => Ok(result),
        Err(e) => return Err(e)
      }
    }

    // /// Verify a signature for a message signed by the address
    // ///
    // /// @param {Uint8Array} Byte array representing a message signed by the address
    // /// @returns {boolean} Boolean representing whether or not the signature is valid
    // pub fn verify(&self, message: &[u8], signature: &Signature) -> bool {
    //     signature.verify(self, message)
    // }
}

pub fn address_to_x_coordinate_impl<N: Network>(address: &str) -> Result<String, String> {
    let address = AddressNative::<N>::from_str(address).unwrap();
    Ok(address.to_x_coordinate().to_string())
}

pub fn address_from_private_key_impl<N: Network>(private_key: &PrivateKey) -> Result<Address, String> {
  let network = network_string_id!(N::ID).unwrap().to_string();
  let pk_native = PrivateKeyNative::<N>::from_str(&**private_key).unwrap();
  let address = AddressNative::<N>::try_from(pk_native).map_err(|e| e.to_string())?;
  Ok(Address { network, as_string: address.to_string() })
}

pub fn address_from_view_key_impl<N: Network>(view_key: &ViewKey) -> Result<Address, String> {
  let network = network_string_id!(N::ID).unwrap().to_string();
  let vk_native = ViewKeyNative::<N>::from_str(&**view_key).unwrap();
  let address = AddressNative::<N>::try_from(vk_native).map_err(|e| e.to_string())?;
  Ok(Address { network, as_string: address.to_string() })
}

pub fn address_from_string_impl<N: Network>(address: &str) -> Result<Address, String> {
  let network = network_string_id!(N::ID).unwrap().to_string();
  let address_str = AddressNative::<N>::from_str(address).map_err(|e| e.to_string())?.to_string();
  Ok(Address { network, as_string: address_str })
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_string)
    }
}

impl Deref for Address {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.as_string
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::account::PrivateKey;

    use wasm_bindgen_test::*;

    const ITERATIONS: u64 = 1_000;

    #[wasm_bindgen_test]
    pub fn test_from_private_key() {
        for _ in 0..ITERATIONS {
            // Sample a new private key.
            let private_key = PrivateKey::new();
            let expected = Address::from_private_key(&private_key);

            // Check the address derived from the view key.
            let view_key = private_key.to_view_key();
            assert_eq!(expected, Address::from_view_key(&view_key));
        }
    }
}
