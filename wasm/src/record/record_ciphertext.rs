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

use super::RecordPlaintext;
use crate::account::ViewKey;

use crate::types::native::{RecordCiphertextNative, ViewKeyNative, RecordPlaintextNative};
use std::{ops::Deref, str::FromStr};
use snarkvm_console::program::{Owner, ToFields};
use wasm_bindgen::prelude::*;
use crate::Network;

/// Encrypted Aleo record
#[wasm_bindgen]
#[derive(Clone)]
pub struct RecordCiphertext
{
  network: String,
  as_string: String
}

#[wasm_bindgen]
impl RecordCiphertext {
    /// Create a record ciphertext from a string
    ///
    /// @param {string} record String representation of a record ciphertext
    /// @returns {RecordCiphertext | Error} Record ciphertext
    #[wasm_bindgen(js_name = fromString)]
    pub fn from_string(network: &str, record: &str) -> Result<RecordCiphertext, String> {
      match dispatch_network!(network, record_ciphertext_from_string_impl, record) {
        Ok(result) => Ok(RecordCiphertext { network: network.to_string(), as_string: result }),
        Err(e) => return Err(e)
      }
    }

    /// Return the string reprensentation of the record ciphertext
    ///
    /// @returns {string} String representation of the record ciphertext
    #[allow(clippy::inherent_to_string)]
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.as_string.clone()
    }

    /// Decrypt the record ciphertext into plaintext using the view key. The record will only
    /// decrypt if the record was encrypted by the account corresponding to the view key
    ///
    /// @param {ViewKey} view_key View key used to decrypt the ciphertext
    /// @returns {RecordPlaintext | Error} Record plaintext object
    pub fn decrypt(&self, view_key: &ViewKey) -> Result<RecordPlaintext, String> {
      match dispatch_network!(self.network.as_str(), record_ciphertext_decrypt_impl, &self, view_key) {
        Ok(result) => Ok(result),
        Err(e) => return Err(e)
      }
    }

    /// Determines if the account corresponding to the view key is the owner of the record
    ///
    /// @param {ViewKey} view_key View key used to decrypt the ciphertext
    /// @returns {boolean}
    #[wasm_bindgen(js_name = isOwner)]
    pub fn is_owner(&self, view_key: &ViewKey) -> Result<bool, String> {
      match dispatch_network!(self.network.as_str(), record_ciphertext_is_owner_impl, &self, view_key) {
        Ok(result) => Ok(result),
        Err(e) => return Err(e)
      }
    }

    #[wasm_bindgen(js_name = getOwnerX)]
    pub fn get_owner_x(&self) -> Result<String, String> {
      match dispatch_network!(self.network.as_str(), record_ciphertext_get_owner_x_impl, &self) {
        Ok(result) => Ok(result),
        Err(e) => return Err(e)
      }
    }

    #[wasm_bindgen(js_name = getNonceX)]
    pub fn get_nonce_x(&self) -> Result<String, String> {
      match dispatch_network!(self.network.as_str(), record_ciphertext_get_nonce_x_impl, &self) {
        Ok(result) => Ok(result),
        Err(e) => return Err(e)
      }
    }

    #[wasm_bindgen(js_name = getNonceY)]
    pub fn get_nonce_y(&self) -> Result<String, String> {
      match dispatch_network!(self.network.as_str(), record_ciphertext_get_nonce_y_impl, &self) {
        Ok(result) => Ok(result),
        Err(e) => return Err(e)
      }
    }
}

pub fn record_ciphertext_get_nonce_y_impl<N: Network>(record: &RecordCiphertext) -> Result<String, String> {
  let rc_native = RecordCiphertextNative::<N>::from_str(&*record).unwrap();
  Ok(rc_native.nonce().to_y_coordinate().to_string())
}

pub fn record_ciphertext_get_nonce_x_impl<N: Network>(record: &RecordCiphertext) -> Result<String, String> {
  let rc_native = RecordCiphertextNative::<N>::from_str(&*record).unwrap();
  Ok(rc_native.nonce().to_x_coordinate().to_string())
}

pub fn record_ciphertext_get_owner_x_impl<N: Network>(record: &RecordCiphertext) -> Result<String, String> {
    let rc_native = RecordCiphertextNative::<N>::from_str(&*record).unwrap();
    let owner = rc_native.owner();
    match owner {
        Owner::Public(owner) => {
            Ok(owner.to_x_coordinate().to_string())
        }
        Owner::Private(owner) => {
            Ok(owner.to_fields().unwrap().first().unwrap().to_string())
        }
    }
}

pub fn record_ciphertext_is_owner_impl<N: Network>(record: &RecordCiphertext, view_key: &ViewKey) -> Result<bool, String> {
    let rc_native = RecordCiphertextNative::<N>::from_str(&*record).unwrap();
    let vk_native = ViewKeyNative::<N>::from_str(&*view_key).unwrap();
    Ok(rc_native.is_owner(&vk_native))
}

pub fn record_ciphertext_from_string_impl<N: Network>(record: &str) -> Result<String, String> {
    let rc_native = RecordCiphertextNative::<N>::from_str(record).map_err(|_| "The record ciphertext string provided was invalid".to_string());
    Ok(rc_native.unwrap().to_string())
}

pub fn record_ciphertext_decrypt_impl<N: Network>(record: &RecordCiphertext, view_key: &ViewKey) -> Result<RecordPlaintext, String> {
  let rc_native = RecordCiphertextNative::<N>::from_str(&*record).map_err(|_| "The record ciphertext string provided was invalid".to_string())?;
  let vk_native = ViewKeyNative::<N>::from_str(&*view_key).map_err(|_| "The view key string provided was invalid".to_string())?;
  let rp_native = RecordPlaintextNative::<N>::from(rc_native.decrypt(&vk_native).map_err(|_| "Decryption failed - view key did not match record".to_string())?);
  let network = network_string_id!(N::ID).unwrap().to_string();
  RecordPlaintext::from_string(
    &network,
    &rp_native.to_string()
  ).map_err(|_| "The record plaintext string provided was invalid".to_string())
}

impl Deref for RecordCiphertext {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.as_string
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     use wasm_bindgen_test::wasm_bindgen_test;

//     const OWNER_PLAINTEXT: &str = r"{
//   owner: aleo1j7qxyunfldj2lp8hsvy7mw5k8zaqgjfyr72x2gh3x4ewgae8v5gscf5jh3.private,
//   microcredits: 1500000000000000u64.private,
//   _nonce: 3077450429259593211617823051143573281856129402760267155982965992208217472983group.public
// }";
//     const OWNER_CIPHERTEXT: &str = "record1qyqsqpe2szk2wwwq56akkwx586hkndl3r8vzdwve32lm7elvphh37rsyqyxx66trwfhkxun9v35hguerqqpqzqrtjzeu6vah9x2me2exkgege824sd8x2379scspmrmtvczs0d93qttl7y92ga0k0rsexu409hu3vlehe3yxjhmey3frh2z5pxm5cmxsv4un97q";
//     const OWNER_VIEW_KEY: &str = "AViewKey1ccEt8A2Ryva5rxnKcAbn7wgTaTsb79tzkKHFpeKsm9NX";
//     const NON_OWNER_VIEW_KEY: &str = "AViewKey1e2WyreaH5H4RBcioLL2GnxvHk5Ud46EtwycnhTdXLmXp";

//     // Related material for use in future tests
//     const _OWNER_PRIVATE_KEY: &str = "APrivateKey1zkpJkyYRGYtkeHDaFfwsKtUJzia7csiWhfBWPXWhXJzy9Ls";
//     const _OWNER_ADDRESS: &str = "aleo1j7qxyunfldj2lp8hsvy7mw5k8zaqgjfyr72x2gh3x4ewgae8v5gscf5jh3";

//     #[test]
//     fn test_from_string() {
//         let record_ciphertext_string = "record1qyqsqcznnug9rmrdnqc2sqp8ycss5szk6y356pguxpqf2vsxpa6lhrgfqgzhgmmtv4hyxqqzqgqz7w6vhxqdug0akcwlu8etvpg8htz2ghdrpwam3mafyxpgxlg32pm5xnp9zjyrelm7fj8v3lt48xvgs3ympvxdc7xy8httn2streepqvrxzmt0w4h8ggcqqgqsqfadmznhnwslhav2l3k9p6jvyejrr3segxpeear9hgla4zn8vzqxz9yu572mhc39ef942w60rgk35a5qu2xfwek8upg4ldv8rpcwwc9snk80zq";
//         let record = RecordCiphertext::from_string("mainnet", record_ciphertext_string).unwrap();
//         println!("{}", record.get_nonce_x());
//         println!("{}", record.get_nonce_y());
//         println!("{}", record.get_owner_x());
//     }

//     #[wasm_bindgen_test]
//     fn test_to_and_from_string() {
//         let record = RecordCiphertext::from_string("mainnet", OWNER_CIPHERTEXT).unwrap();
//         assert_eq!(record.to_string(), OWNER_CIPHERTEXT);
//     }

//     #[wasm_bindgen_test]
//     fn test_invalid_strings() {
//         let invalid_bech32 = "record2qqj3a67efazf0awe09grqqg44htnh9vaw7l729vl309c972x7ldquqq2k2cax8s7qsqqyqtpgvqqyqsq4seyrzvfa98fkggzccqr68af8e9m0q8rzeqh8a8aqql3a854v58sgrygdv4jn9s8ckwfd48vujrmv0rtfasqh8ygn88ch34ftck8szspvfpsqqszqzvxx9t8s9g66teeepgxmvnw5ymgapcwt2lpy9d5eus580k08wpq544jcl437wjv206u5pxst6few9ll4yhufwldgpx80rlwq8nhssqywmfsd85skg564vqhm3gxsp8q6r30udmqxrxmxx2v8xycdg8pn5ps3dhfvv";
//         assert_eq!(
//             RecordCiphertext::from_string("mainnet", "garbage").err(),
//             Some("The record ciphertext string provided was invalid".to_string())
//         );
//         assert!(RecordCiphertext::from_string("mainnet", invalid_bech32).is_err());
//     }

//     #[wasm_bindgen_test]
//     fn test_decrypt() {
//         let record = RecordCiphertext::from_string("mainnet", OWNER_CIPHERTEXT).unwrap();
//         let view_key = ViewKey::from_string("mainnet", OWNER_VIEW_KEY);
//         let plaintext = record.decrypt("mainnet", &view_key).unwrap();
//         assert_eq!(plaintext.to_string(), OWNER_PLAINTEXT);
//         let incorrect_view_key = ViewKey::from_string("mainnet", NON_OWNER_VIEW_KEY);
//         assert!(record.decrypt("mainnet", &incorrect_view_key).is_err());
//     }

//     #[wasm_bindgen_test]
//     fn test_is_owner() {
//         let record = RecordCiphertext::from_string("mainnet", OWNER_CIPHERTEXT).unwrap();
//         let view_key = ViewKey::from_string("mainnet", OWNER_VIEW_KEY);
//         assert!(record.is_owner("mainnet", &view_key));
//         let incorrect_view_key = ViewKey::from_string("mainnet", NON_OWNER_VIEW_KEY);
//         assert!(!record.is_owner("mainnet", &incorrect_view_key));
//     }
// }
