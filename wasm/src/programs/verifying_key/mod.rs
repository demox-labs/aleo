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

use crate::types::native::{FromBytes, ToBytes, VerifyingKeyNative};
use crate::Network;

use sha2::Digest;
use wasm_bindgen::prelude::wasm_bindgen;

use std::{ops::Deref, str::FromStr};

/// Verifying key for a function within an Aleo program
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct VerifyingKey {
  network: String,
  as_string: String
}

#[wasm_bindgen]
impl VerifyingKey {
    /// Get the checksum of the verifying key
    ///
    /// @returns {string} Checksum of the verifying key
    pub fn checksum(&self) -> Result<String, String> {
      match dispatch_network!(self.network.as_str(), verifying_key_checksum_impl, &self) {
        Ok(checksum) => Ok(checksum),
        Err(e) => Err(e),
      }
    }

    /// Create a copy of the verifying key
    ///
    /// @returns {VerifyingKey} A copy of the verifying key
    #[wasm_bindgen]
    pub fn copy(&self) -> VerifyingKey {
      VerifyingKey { network: self.network.clone(), as_string: self.as_string.clone() }
    }

    /// Construct a new verifying key from a byte array
    ///
    /// @param {Uint8Array} bytes Byte representation of a verifying key
    /// @returns {VerifyingKey | Error}
    #[wasm_bindgen(js_name = "fromBytes")]
    pub fn from_bytes(network: &str, bytes: &[u8]) -> Result<VerifyingKey, String> {
      match dispatch_network!(network, verifying_key_from_bytes_impl, bytes) {
        Ok(verifying_key) => Ok(verifying_key),
        Err(e) => Err(e),
      }
    }

    /// Create a verifying key from string
    ///
    /// @param {String} string String representation of a verifying key
    /// @returns {VerifyingKey | Error}
    #[wasm_bindgen(js_name = "fromString")]
    pub fn from_string(network: &str, string: &str) -> Result<VerifyingKey, String> {
      match dispatch_network!(network, verifying_key_from_string_impl, string) {
        Ok(verifying_key) => Ok(verifying_key),
        Err(e) => Err(e),
      }
    }

    /// Create a byte array from a verifying key
    ///
    /// @returns {Uint8Array | Error} Byte representation of a verifying key
    #[wasm_bindgen(js_name = "toBytes")]
    pub fn to_bytes(&self) -> Result<Vec<u8>, String> {
      match dispatch_network!(self.network.as_str(), verifying_key_to_bytes_impl, &self) {
        Ok(bytes) => Ok(bytes),
        Err(e) => Err(e),
      }
    }

    /// Get a string representation of the verifying key
    ///
    /// @returns {String} String representation of the verifying key
    #[wasm_bindgen(js_name = "toString")]
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
      format!("{:?}", self.as_string)
    }
}

pub fn verifying_key_to_bytes_impl<N: Network>(verifying_key: &VerifyingKey) -> Result<Vec<u8>, String> {
  let vk_native = VerifyingKeyNative::<N>::from_str(&verifying_key.as_string).map_err(|e| e.to_string())?;
  vk_native.to_bytes_le().map_err(|_| "Failed to serialize verifying key".to_string())
}

pub fn verifying_key_from_string_impl<N: Network>(string: &str) -> Result<VerifyingKey, String> {
  let vk_native = VerifyingKeyNative::<N>::from_str(string).map_err(|e| e.to_string())?;
  let network = network_string_id!(N::ID).unwrap().to_string();
  Ok(VerifyingKey { network, as_string: vk_native.to_string() })
}

pub fn verifying_key_checksum_impl<N: Network>(verifying_key: &VerifyingKey) -> Result<String, String> {
    let vk_native = VerifyingKeyNative::<N>::from_str(&verifying_key.as_string).map_err(|e| e.to_string())?;
    Ok(hex::encode(sha2::Sha256::digest(vk_native.to_bytes_le().unwrap())))
}

pub fn verifying_key_from_bytes_impl<N: Network>(bytes: &[u8]) -> Result<VerifyingKey, String> {
    let vk_native = VerifyingKeyNative::<N>::from_bytes_le(bytes).map_err(|e| e.to_string())?;
    let network = network_string_id!(N::ID).unwrap().to_string();
    Ok(VerifyingKey { network, as_string: vk_native.to_string() })
}

impl Deref for VerifyingKey {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.as_string
    }
}

impl<N: Network> From<VerifyingKey> for VerifyingKeyNative<N> {
    fn from(verifying_key: VerifyingKey) -> VerifyingKeyNative<N> {
        VerifyingKeyNative::<N>::from_str(&verifying_key.as_string).unwrap()
    }
}

impl<N: Network> From<&VerifyingKey> for VerifyingKeyNative<N> {
    fn from(verifying_key: &VerifyingKey) -> VerifyingKeyNative<N> {
      VerifyingKeyNative::<N>::from_str(&verifying_key.as_string).unwrap()
    }
}

impl<N: Network> From<VerifyingKeyNative<N>> for VerifyingKey {
    fn from(verifying_key: VerifyingKeyNative<N>) -> VerifyingKey {
      let network = network_string_id!(N::ID).unwrap().to_string();
      VerifyingKey { network, as_string: verifying_key.to_string() }
    }
}

impl PartialEq for VerifyingKey {
    fn eq(&self, other: &Self) -> bool {
      *self.as_string == *other.as_string && *self.network == *other.network
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use wasm_bindgen_test::*;

//     const TRANSFER_PUBLIC_VERIFYING_KEY: &str = "verifier1qygqqqqqqqqqqqq79uqqqqqqqqqp2tcqqqqqqqqqwd4qqqqqqqqqp5ydqqqqqqqqqqvyqqqqqqqqqqqvqqqqqqqqqqqre7drur40rst43dq9at346py7hkmrhexarf59f2tjt4stlsdj5uwrgnrkjjej7jf3djk2w4njtxcq0mezac793craujm8mr7wutcqtu2aday5g03wl0cu2572fsrtpyjhdyqlh0447z7dshlkhksjsusgp4ezrvc0n64fwetfmml3kvfg7n03w2e602sl7et4cpw98hgpzxwzrmzu8r3x77v49njysy2lp55xsqh6t5qjvhyl5a7nzy3e73y7dzvvs9p450u0s8g84prqnrk6jeah89c6882uzqdvxgzcedfmsc43uq99n3ycrjh70ys8n02pyvdvzmu7z608desdd5yw9dc8v3ddrdddmrzz2pupe09yn9esy25cfzmd0wqcgjdxm4dvlt2t6k66lw8e9ccj49qj2ahpht62kh7p56xpvpekenq2arng2t55mwxe59mqpkp6a0yqlwt7tdf98rt3kqlr9tdtq6hua3wrka0mqzhva4nhucxn9u4w92mly69jy2c7cqm5ftnk3m0qxy9spaxwfz0xkqd947yvf2zh8h4y59fltxdpeu4utpv9zw0cr7ad9d462qxyc2f05lezw6dwhcmep942qqv38lp3x9efestt5pk8rplvmrk0zz9zel48l8h9ldfzyd8zyr7knze92cdyanez6k7q5fu6tnw9wqrywjnhevaujz20xn0h3n47g85zs6ejfh7z8jt9qjesqgmdymvcxlceudkdsl49t5r69c4mg7hfwyq88z7zn0efda8fdjmhz8aaq24q34g2ekdzr5w9em3cev2ktxtmupqwltu0nh3fjzm04cy3cgnqlnqq0chzq4rs2dmfjwryxrxxgjtdcsnn9fpwykkxwfuervtznu3lmvhhpdflgwgm0xklu6c0xsxt9dfcp29w2nz6zkjetz7cqremg68eqxq86rn082czp50ldw9qkq6w3p9xxg4hrg";

//     #[allow(dead_code)]
//     #[test]
//     fn verifying_key_strings() {
//         let bond_public_verifier_string =
//             VerifyingKey::from_bytes(&snarkvm_parameters::testnet::BondPublicVerifier::load_bytes().unwrap())
//                 .unwrap()
//                 .to_string();
//         let claim_unbond_public_verifier_string =
//             VerifyingKey::from_bytes(&snarkvm_parameters::testnet::ClaimUnbondPublicVerifier::load_bytes().unwrap())
//                 .unwrap()
//                 .to_string();
//         let fee_private_verifier_string =
//             VerifyingKey::from_bytes(&snarkvm_parameters::testnet::FeePrivateVerifier::load_bytes().unwrap())
//                 .unwrap()
//                 .to_string();
//         let fee_public_verifier_string =
//             VerifyingKey::from_bytes(&snarkvm_parameters::testnet::FeePublicVerifier::load_bytes().unwrap())
//                 .unwrap()
//                 .to_string();
//         let inclusion_verifier_string =
//             VerifyingKey::from_bytes(&snarkvm_parameters::testnet::InclusionVerifier::load_bytes().unwrap())
//                 .unwrap()
//                 .to_string();
//         let join_verifier_string =
//             VerifyingKey::from_bytes(&snarkvm_parameters::testnet::JoinVerifier::load_bytes().unwrap())
//                 .unwrap()
//                 .to_string();
//         let set_validator_state_verifier_string =
//             VerifyingKey::from_bytes(&snarkvm_parameters::testnet::SetValidatorStateVerifier::load_bytes().unwrap())
//                 .unwrap()
//                 .to_string();
//         let split_verifier_string =
//             VerifyingKey::from_bytes(&snarkvm_parameters::testnet::SplitVerifier::load_bytes().unwrap())
//                 .unwrap()
//                 .to_string();
//         let transfer_private_verifier_string =
//             VerifyingKey::from_bytes(&snarkvm_parameters::testnet::TransferPrivateVerifier::load_bytes().unwrap())
//                 .unwrap()
//                 .to_string();
//         let transfer_private_to_public_verifier_string = VerifyingKey::from_bytes(
//             &snarkvm_parameters::testnet::TransferPrivateToPublicVerifier::load_bytes().unwrap(),
//         )
//         .unwrap()
//         .to_string();
//         let transfer_public_verifier_string =
//             VerifyingKey::from_bytes(&snarkvm_parameters::testnet::TransferPublicVerifier::load_bytes().unwrap())
//                 .unwrap()
//                 .to_string();
//         let transfer_public_to_private_verifier_string = VerifyingKey::from_bytes(
//             &snarkvm_parameters::testnet::TransferPublicToPrivateVerifier::load_bytes().unwrap(),
//         )
//         .unwrap()
//         .to_string();
//         let unbond_delegator_as_validator_verifier_string = VerifyingKey::from_bytes(
//             &snarkvm_parameters::testnet::UnbondDelegatorAsValidatorVerifier::load_bytes().unwrap(),
//         )
//         .unwrap()
//         .to_string();
//         let unbond_public_verifier_string =
//             VerifyingKey::from_bytes(&snarkvm_parameters::testnet::UnbondPublicVerifier::load_bytes().unwrap())
//                 .unwrap()
//                 .to_string();
//         println!("bond_public_verifier:\nverifying_key: \"{}\"", bond_public_verifier_string);
//         println!("claim_unbond_public_verifier:\nverifying_key: \"{}\"", claim_unbond_public_verifier_string);
//         println!("fee_private_verifier:\nverifying_key: \"{}\"", fee_private_verifier_string);
//         println!("fee_public_verifier:\nverifying_key: \"{}\"", fee_public_verifier_string);
//         println!("inclusion_verifier:\nverifying_key: \"{}\"", inclusion_verifier_string);
//         println!("join_verifier:\nverifying_key: \"{}\"", join_verifier_string);
//         println!("set_validator_state_verifier:\nverifying_key: \"{}\"", set_validator_state_verifier_string);
//         println!("split_verifier:\nverifying_key: \"{}\"", split_verifier_string);
//         println!("transfer_private_verifier:\nverifying_key: \"{}\"", transfer_private_verifier_string);
//         println!(
//             "transfer_private_to_public_verifier:\nverifying_key: \"{}\"",
//             transfer_private_to_public_verifier_string
//         );
//         println!("transfer_public_verifier:\nverifying_key: \"{}\"", transfer_public_verifier_string);
//         println!(
//             "transfer_public_to_private_verifier:\nverifying_key: \"{}\"",
//             transfer_public_to_private_verifier_string
//         );
//         println!(
//             "unbond_delegator_as_validator_verifier:\nverifying_key: \"{}\"",
//             unbond_delegator_as_validator_verifier_string
//         );
//         println!("unbond_public_verifier:\nverifying_key: \"{}\"", unbond_public_verifier_string);
//     }

//     #[wasm_bindgen_test]
//     async fn test_verifying_key_roundtrip() {
//         let transfer_public_verifier_bytes =
//             snarkvm_parameters::testnet::TransferPublicVerifier::load_bytes().unwrap();
//         let transfer_public_verifier = VerifyingKey::from_bytes(&transfer_public_verifier_bytes).unwrap();
//         let transfer_public_verifying_key_string = transfer_public_verifier.to_string();
//         assert_eq!(transfer_public_verifying_key_string, TRANSFER_PUBLIC_VERIFYING_KEY);
//     }

//     #[wasm_bindgen_test]
//     async fn test_verifier_checksum() {
//         let transfer_public_verifier_bytes =
//             snarkvm_parameters::testnet::TransferPublicVerifier::load_bytes().unwrap();
//         let transfer_public_verifier = VerifyingKey::from_bytes(&transfer_public_verifier_bytes).unwrap();
//         let transfer_public_verifying_key_checksum = transfer_public_verifier.checksum();
//         assert_eq!(
//             transfer_public_verifying_key_checksum,
//             "a4c2906a95b2f8bdcc6f192a0c71fb0a1c1aa3830feb54454627cf552674932a"
//         );
//     }
// }
