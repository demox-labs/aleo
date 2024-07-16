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

use super::{Address, PrivateKey};
use crate::record::RecordCiphertext;

use crate::types::native::{ViewKeyNative, RecordCiphertextNative, FieldNative, GroupNative};
use core::{convert::TryFrom, fmt, ops::Deref, str::FromStr};
use crate::native::Network;
use crate::native::PrivateKeyNative;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ViewKey {
  #[wasm_bindgen(skip)]
  pub network: String,
  as_string: String
}

#[wasm_bindgen]
impl ViewKey {
    /// Create a new view key from a private key
    ///
    /// @param {PrivateKey} private_key Private key
    /// @returns {ViewKey} View key
    pub fn from_private_key(network: &str, private_key: &PrivateKey) -> Result<ViewKey, String> {
      match dispatch_network!(network, view_key_from_private_key_impl, private_key) {
        Ok(result) => Ok(result),
        Err(e) => return Err(e.to_string())
      }
    }

    #[wasm_bindgen(getter)]
    pub fn network(&self) -> String {
      self.network.clone()
    }

    /// Create a new view key from a string representation of a view key
    ///
    /// @param {string} view_key String representation of a view key
    /// @returns {ViewKey} View key
    pub fn from_string(network: &str, view_key: &str) -> Result<ViewKey, String> {
      match dispatch_network!(network, view_key_from_string_impl, view_key) {
        Ok(result) => Ok(result),
        Err(e) => return Err(e.to_string())
      }
    }

    pub fn is_owner(&self, address_x_coordinate: &str, record_nonce: &str, record_owner_x_coordinate: &str) -> Result<bool, String> {
      match dispatch_network!(self.network.as_str(), view_key_is_owner_impl, &self.as_string, address_x_coordinate, record_nonce, record_owner_x_coordinate) {
        Ok(result) => Ok(result),
        Err(e) => return Err(e)
      }
    }

    pub fn to_scalar(&self) -> Result<String, String> {
      match dispatch_network!(self.network.as_str(), view_key_to_scalar_impl, &self.as_string) {
        Ok(result) => Ok(result),
        Err(e) => return Err(e)
      }
    }

    /// Get a string representation of a view key
    ///
    /// @returns {string} String representation of a view key
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        self.as_string.clone()
    }

    /// Get the address corresponding to a view key
    ///
    /// @returns {Address} Address
    pub fn to_address(&self) -> Result<Address, String> {
      match dispatch_network!(self.network.as_str(), view_key_to_address_impl, &self.as_string) {
        Ok(result) => Ok(result),
        Err(e) => return Err(e)
      }
    }

    /// Decrypt a record ciphertext with a view key
    ///
    /// @param {string} ciphertext String representation of a record ciphertext
    /// @returns {string} String representation of a record plaintext
    pub fn decrypt(&self, ciphertext: &str) -> Result<String, String> {
      match dispatch_network!(self.network.as_str(), view_key_decrypt_impl, &self.as_string, ciphertext) {
        Ok(result) => Ok(result),
        Err(e) => return Err(e)
      }
    }
}

pub fn view_key_decrypt_impl<N: Network>(view_key: &str, ciphertext: &str) -> Result<String, String> {
  let network = network_string_id!(N::ID).unwrap().to_string();
  let view_key = ViewKey::from_string(&network, view_key).unwrap();
  let cypher_text = RecordCiphertext::from_string(&network, ciphertext).map_err(|e| e.to_string())?;
  match cypher_text.decrypt(&view_key) {
    Ok(plaintext) => Ok(plaintext.to_string()),
    Err(error) => Err(error),
  }
}

pub fn view_key_to_address_impl<N: Network>(view_key: &str) -> Result<Address, String> {
  let network = network_string_id!(N::ID).unwrap().to_string();
  let vk_string = ViewKeyNative::<N>::from_str(view_key).map_err(|e| e.to_string())?.to_string();
  crate::address::address_from_view_key_impl::<N>(&ViewKey { network, as_string: vk_string })
}

pub fn view_key_from_private_key_impl<N: Network>(private_key: &PrivateKey) -> Result<ViewKey, String> {
  let network = network_string_id!(N::ID).unwrap().to_string();
  let vk_string = ViewKeyNative::<N>::try_from(PrivateKeyNative::<N>::from_str(&**private_key).unwrap()).unwrap().to_string();
  Ok(ViewKey { network, as_string: vk_string })
}

pub fn view_key_from_string_impl<N: Network>(view_key: &str) -> Result<ViewKey, String> {
  let network = network_string_id!(N::ID).unwrap().to_string();
  let vk_string = ViewKeyNative::<N>::from_str(view_key).map_err(|e| e.to_string())?.to_string();
  Ok(ViewKey { network, as_string: vk_string })
}

pub fn view_key_is_owner_impl<N: Network>(view_key: &str, address_x_coordinate: &str, record_nonce: &str, record_owner_x_coordinate: &str) -> Result<bool, String> {
  let x_field = FieldNative::<N>::from_str(address_x_coordinate).unwrap();
  let nonce_group = GroupNative::<N>::from_str(record_nonce).unwrap();
  let owner_x_field = FieldNative::<N>::from_str(record_owner_x_coordinate).unwrap();
  let scalar = *ViewKeyNative::<N>::from_str(view_key).unwrap();
  Ok(RecordCiphertextNative::<N>::is_owner_direct(
      x_field,
      scalar,
      nonce_group,
      owner_x_field,
  ))
}

pub fn view_key_to_scalar_impl<N: Network>(view_key: &str) -> Result<String, String> {
  let scalar = *ViewKeyNative::<N>::from_str(view_key).unwrap();
  Ok(scalar.to_string())
}

impl fmt::Display for ViewKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_string)
    }
}

impl Deref for ViewKey {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.as_string
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use wasm_bindgen_test::*;

    const RECORD_PLAINTEXT: &str = r"{
  owner: aleo1j7qxyunfldj2lp8hsvy7mw5k8zaqgjfyr72x2gh3x4ewgae8v5gscf5jh3.private,
  microcredits: 1500000000000000u64.private,
  _nonce: 3077450429259593211617823051143573281856129402760267155982965992208217472983group.public
}";
    const OWNER_CIPHERTEXT: &str = "record1qyqsqpe2szk2wwwq56akkwx586hkndl3r8vzdwve32lm7elvphh37rsyqyxx66trwfhkxun9v35hguerqqpqzqrtjzeu6vah9x2me2exkgege824sd8x2379scspmrmtvczs0d93qttl7y92ga0k0rsexu409hu3vlehe3yxjhmey3frh2z5pxm5cmxsv4un97q";
    const OWNER_VIEW_KEY: &str = "AViewKey1ccEt8A2Ryva5rxnKcAbn7wgTaTsb79tzkKHFpeKsm9NX";
    const NON_OWNER_VIEW_KEY: &str = "AViewKey1e2WyreaH5H4RBcioLL2GnxvHk5Ud46EtwycnhTdXLmXp";

    // #[wasm_bindgen_test]
    // pub fn test_from_private_key() {
    //     let given_private_key = "APrivateKey1zkp4RyQ8Utj7aRcJgPQGEok8RMzWwUZzBhhgX6rhmBT8dcP";
    //     let given_view_key = "AViewKey1i3fn5SECcVBtQMCVtTPSvdApoMYmg3ToJfNDfgHJAuoD";
    //     let private_key = PrivateKey::from_string(given_private_key).unwrap();
    //     let view_key = ViewKey::from_private_key(&private_key);
    //     assert_eq!(given_view_key, view_key.to_string());
    // }

    // #[wasm_bindgen_test]
    // pub fn test_decrypt_success() {
    //     let view_key = ViewKey::from_string(OWNER_VIEW_KEY);
    //     let plaintext = view_key.decrypt(OWNER_CIPHERTEXT);
    //     plaintext.clone().unwrap();
    //     assert!(plaintext.is_ok());
    //     assert_eq!(RECORD_PLAINTEXT, plaintext.unwrap())
    // }

    // #[wasm_bindgen_test]
    // pub fn test_decrypt_fails() {
    //     let ciphertext = RecordCiphertext::from_str(OWNER_CIPHERTEXT).map_err(|error| error.to_string()).unwrap();
    //     let incorrect_view_key = ViewKey::from_string(NON_OWNER_VIEW_KEY);
    //     let plaintext = ciphertext.decrypt(&incorrect_view_key);
    //     assert!(plaintext.is_err());
    // }
}
