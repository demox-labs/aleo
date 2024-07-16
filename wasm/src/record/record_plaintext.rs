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

use crate::{
    account::PrivateKey,
    types::Field,
    Credits,
    // RecordCiphertext,
};

use crate::types::native::{IdentifierNative, ProgramIDNative, RecordPlaintextNative};
use crate::native::Network;
use std::{ops::Deref, str::FromStr};
use wasm_bindgen::prelude::*;

/// Plaintext representation of an Aleo record
#[wasm_bindgen]
#[derive(Clone)]
pub struct RecordPlaintext{
  network: String,
  as_string: String
}

#[wasm_bindgen]
impl RecordPlaintext {
    #[wasm_bindgen]
    pub fn commitment(&self, program_id: &str, record_name: &str) -> Result<Field, String> {
      match dispatch_network!(self.network.as_str(), record_plaintext_commitment_impl, &self.as_string, program_id, record_name) {
        Ok(result) => Ok(result),
        Err(e) => return Err(e)
      }
    }

    /// Return a record plaintext from a string.
    ///
    /// @param {string} record String representation of a plaintext representation of an Aleo record
    /// @returns {RecordPlaintext | Error} Record plaintext
    #[wasm_bindgen(js_name = fromString)]
    pub fn from_string(network: &str, record: &str) -> Result<RecordPlaintext, String> {
      match dispatch_network!(network, record_plaintext_from_string_impl, record) {
        Ok(result) => Ok(result),
        Err(e) => return Err(e)
      }
    }

    /// Returns the record plaintext string
    ///
    /// @returns {string} String representation of the record plaintext
    #[allow(clippy::inherent_to_string)]
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.as_string.clone()
    }

    /// Returns the amount of microcredits in the record
    ///
    /// @returns {u64} Amount of microcredits in the record
    pub fn microcredits(&self) -> Result<u64, String> {
      match dispatch_network!(self.network.as_str(), record_plaintext_microcredits_impl, &self.as_string) {
        Ok(result) => Ok(result),
        Err(e) => return Err(e)
      }
    }

    /// Returns the nonce of the record. This can be used to uniquely identify a record.
    ///
    /// @returns {string} Nonce of the record
    #[wasm_bindgen(js_name = nonce)]
    pub fn nonce(&self) -> Result<String, String> {
      match dispatch_network!(self.network.as_str(), record_plaintext_nonce_impl, &self.as_string) {
        Ok(result) => Ok(result),
        Err(e) => return Err(e)
      }
    }

    // /// Decrypt the record ciphertext into plaintext using the view key. The record will only
    // /// decrypt if the record was encrypted by the account corresponding to the view key
    // ///
    // /// @param {ViewKey} view_key View key used to decrypt the ciphertext
    // /// @returns {RecordPlaintext | Error} Record plaintext object
    // pub fn encrypt(&self, view_key: &ViewKey) -> Result<RecordCiphertext, String> {
    //     Ok(RecordCiphertext::from(self.0.encrypt(***view_key).map_err(|_| "Encryption failed - view key did not match record".to_string())?
    //     ))
    // }

    /// Attempt to get the serial number of a record to determine whether or not is has been spent
    ///
    /// @param {PrivateKey} private_key Private key of the account that owns the record
    /// @param {string} program_id Program ID of the program that the record is associated with
    /// @param {string} record_name Name of the record
    /// @returns {string | Error} Serial number of the record
    #[wasm_bindgen(js_name = serialNumberString)]
    pub fn serial_number_string(
        &self,
        private_key: &PrivateKey,
        program_id: &str,
        record_name: &str,
    ) -> Result<String, String> {
      match dispatch_network!(self.network.as_str(), record_plaintext_serial_number_string_impl, &self.as_string, private_key, program_id, record_name) {
        Ok(result) => Ok(result),
        Err(e) => return Err(e)
      }
    }
}

pub fn record_plaintext_commitment_impl<N: Network>(record: &str, program_id: &str, record_name: &str) -> Result<Field, String> {
    Ok(Field::from(
        RecordPlaintextNative::<N>::from_str(record)
            .map_err(|_| "Invalid record".to_string())?
            .to_commitment(
                &ProgramIDNative::from_str(program_id)
                    .map_err(|_| format!("{program_id} is an invalid program name"))?,
                &IdentifierNative::from_str(record_name)
                    .map_err(|_| format!("{record_name} is an invalid identifier"))?,
            )
            .map_err(|e| e.to_string())?,
    ))
}

pub fn record_plaintext_serial_number_string_impl<N: Network>(
  record_string: &str,
  private_key: &PrivateKey,
  program_id: &str,
  record_name: &str,
) -> Result<String, String> {
    let commitment = record_plaintext_commitment_impl::<N>(record_string, program_id, record_name).unwrap();

    let serial_number = RecordPlaintextNative::<N>::serial_number(private_key.into(), commitment.into())
        .map_err(|_| "Serial number derivation failed".to_string())?;
    Ok(serial_number.to_string())
}

pub fn record_plaintext_nonce_impl<N: Network>(record: &str) -> Result<String, String> {
    Ok(RecordPlaintextNative::<N>::from_str(record).unwrap().nonce().to_string())
}

pub fn record_plaintext_microcredits_impl<N: Network>(record: &str) -> Result<u64, String> {
    Ok(RecordPlaintextNative::<N>::from_str(record).unwrap().microcredits().unwrap_or(0))
}

pub fn record_plaintext_from_string_impl<N: Network>(record: &str) -> Result<RecordPlaintext, String> {
  let network = network_string_id!(N::ID).unwrap().to_string();
  let record_string = RecordPlaintextNative::<N>::from_str(record).map_err(|_| "Invalid record".to_string())?.to_string();
  Ok(RecordPlaintext { network, as_string: record_string })
}

impl<N: Network> From<RecordPlaintextNative<N>> for RecordPlaintext {
    fn from(record: RecordPlaintextNative<N>) -> Self {
      let network = network_string_id!(N::ID).unwrap().to_string();
        Self { network, as_string: record.to_string() }
    }
}

impl<N: Network> From<RecordPlaintext> for RecordPlaintextNative<N> {
    fn from(record: RecordPlaintext) -> Self {
        RecordPlaintextNative::<N>::from_str(&record.as_string).unwrap()
    }
}

impl Deref for RecordPlaintext {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.as_string
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     use wasm_bindgen_test::wasm_bindgen_test;

//     const RECORD: &str = r"{
//   owner: aleo1j7qxyunfldj2lp8hsvy7mw5k8zaqgjfyr72x2gh3x4ewgae8v5gscf5jh3.private,
//   microcredits: 1500000000000000u64.private,
//   _nonce: 3077450429259593211617823051143573281856129402760267155982965992208217472983group.public
// }";

//     #[wasm_bindgen_test]
//     fn test_to_and_from_string() {
//         let record = RecordPlaintext::from_string("mainnet", RECORD).unwrap();
//         assert_eq!(record.to_string(), RECORD);
//     }

//     #[wasm_bindgen_test]
//     fn test_microcredits_from_string() {
//         let record = RecordPlaintext::from_string("mainnet", RECORD).unwrap();
//         assert_eq!(record.microcredits(), 1500000000000000);
//     }

//     #[wasm_bindgen_test]
//     fn test_serial_number() {
//         let pk = PrivateKey::from_string("mainnet", "APrivateKey1zkpDeRpuKmEtLNPdv57aFruPepeH1aGvTkEjBo8bqTzNUhE").unwrap();
//         let record = RecordPlaintext::from_string("mainnet", RECORD).unwrap();
//         let program_id = "credits.aleo";
//         let record_name = "credits";
//         let expected_sn = "8170619507075647151199239049653235187042661744691458644751012032123701508940field";
//         let result = record.serial_number_string(&pk, program_id, record_name);
//         assert_eq!(expected_sn, result.unwrap());
//     }

//     #[wasm_bindgen_test]
//     fn test_serial_number_can_run_twice_with_same_private_key() {
//         let pk = PrivateKey::from_string("mainnet", "APrivateKey1zkpDeRpuKmEtLNPdv57aFruPepeH1aGvTkEjBo8bqTzNUhE").unwrap();
//         let record = RecordPlaintext::from_string("mainnet", RECORD).unwrap();
//         let program_id = "credits.aleo";
//         let record_name = "credits";
//         let expected_sn = "8170619507075647151199239049653235187042661744691458644751012032123701508940field";
//         assert_eq!(expected_sn, record.serial_number_string(&pk, program_id, record_name).unwrap());
//         assert_eq!(expected_sn, record.serial_number_string(&pk, program_id, record_name).unwrap());
//     }

//     #[wasm_bindgen_test]
//     fn test_serial_number_invalid_program_id_returns_err_string() {
//         let pk = PrivateKey::from_string("mainnet", "APrivateKey1zkpDeRpuKmEtLNPdv57aFruPepeH1aGvTkEjBo8bqTzNUhE").unwrap();
//         let record = RecordPlaintext::from_string("mainnet", RECORD).unwrap();
//         let program_id = "not a real program id";
//         let record_name = "token";
//         assert!(record.serial_number_string(&pk, program_id, record_name).is_err());
//     }

//     #[wasm_bindgen_test]
//     fn test_serial_number_invalid_record_name_returns_err_string() {
//         let pk = PrivateKey::from_string("mainnet", "APrivateKey1zkpDeRpuKmEtLNPdv57aFruPepeH1aGvTkEjBo8bqTzNUhE").unwrap();
//         let record = RecordPlaintext::from_string("mainnet", RECORD).unwrap();
//         let program_id = "token.aleo";
//         let record_name = "not a real record name";
//         assert!(record.serial_number_string(&pk, program_id, record_name).is_err());
//     }

//     #[wasm_bindgen_test]
//     fn test_bad_inputs_to_from_string() {
//         let invalid_bech32 = "{ owner: aleo2d5hg2z3ma00382pngntdp68e74zv54jdxy249qhaujhks9c72yrs33ddah.private, microcredits: 99u64.public, _nonce: 0group.public }";
//         assert_eq!(
//             RecordPlaintext::from_string("mainnet", "string").err(),
//             Some("The record plaintext string provided was invalid".into())
//         );
//         assert!(RecordPlaintext::from_string("mainnet", invalid_bech32).is_err());
//     }
// }
