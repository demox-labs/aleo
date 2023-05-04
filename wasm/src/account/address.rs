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
    account::{PrivateKey, Signature, ViewKey},
    types::{AddressNative, CurrentNetwork},
};

use core::{convert::TryFrom, fmt, ops::Deref, str::FromStr};
use aleo_rust::{Network, Field};
use js_sys::Array;
use snarkvm_wasm::{FromBytes, program::{ProjectiveCurve, Double, Inverse, Pow, AffineCurve}, types::{Group, Scalar}, SquareRootField, PrimeField, FftField, Fp256, Fp256Parameters};
use wasm_bindgen::prelude::*;
use snarkvm_algorithms::{msm::standard::msm, fft::EvaluationDomain};

/// Public address of an Aleo account
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Address(AddressNative);

#[wasm_bindgen]
impl Address {
    /// Derive an Aleo address from a private key
    ///
    /// @param {PrivateKey} private_key The private key to derive the address from
    /// @returns {Address} Address corresponding to the private key
    pub fn from_private_key(private_key: &PrivateKey) -> Self {
        Self(AddressNative::try_from(**private_key).unwrap())
    }

    /// Derive an Aleo address from a view key
    ///
    /// @param {ViewKey} view_key The view key to derive the address from
    /// @returns {Address} Address corresponding to the view key
    pub fn from_view_key(view_key: &ViewKey) -> Self {
        Self(AddressNative::try_from(**view_key).unwrap())
    }

    /// Create an aleo address object from a string representation of an address
    ///
    /// @param {string} address String representation of an addressm
    /// @returns {Address} Address
    pub fn from_string(address: &str) -> Self {
        Self::from_str(address).unwrap()
    }

    /// Get a string representation of an Aleo address object
    ///
    /// @param {Address} Address
    /// @returns {string} String representation of the address
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    /// Verify a signature for a message signed by the address
    ///
    /// @param {Uint8Array} Byte array representing a message signed by the address
    /// @returns {boolean} Boolean representing whether or not the signature is valid
    pub fn verify(&self, message: &[u8], signature: &Signature) -> bool {
        signature.verify(self, message)
    }

    pub fn to_x_coordinate(&self) -> String {
        self.0.to_x_coordinate().to_string()
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self(AddressNative::read_le(&bytes[..]).unwrap())
    }

    pub fn to_affine(&self) -> String {
        self.0.to_affine().to_string()
    }

    pub fn to_projective(&self) -> String {
        self.0.to_string()
    }

    pub fn to_group(&self) -> String {
        self.0.to_string()
    }

    pub fn add_fields(field1: &str, field2: &str) -> String {
        let field1 = Field::<CurrentNetwork>::from_str(field1).unwrap();
        let field2 = Field::<CurrentNetwork>::from_str(field2).unwrap();
        let result = field1 + field2;
        result.to_string()
    }

    pub fn sub_fields(field1: &str, field2: &str) -> String {
        let field1 = Field::<CurrentNetwork>::from_str(field1).unwrap();
        let field2 = Field::<CurrentNetwork>::from_str(field2).unwrap();
        let result = field1 - field2;
        result.to_string()
    }

    pub fn invert_field(field: &str) -> String {
        let field = Field::<CurrentNetwork>::from_str(field).unwrap();
        let result = field.inverse().unwrap();
        result.to_string()
    }

    pub fn double_field(field: &str) -> String {
        let field = Field::<CurrentNetwork>::from_str(field).unwrap();
        let result = field.double();
        result.to_string()
    }

    pub fn mul_fields(field1: &str, field2: &str) -> String {
        let field1 = Field::<CurrentNetwork>::from_str(field1).unwrap();
        let field2 = Field::<CurrentNetwork>::from_str(field2).unwrap();
        let result = field1 * field2;
        result.to_string()
    }

    pub fn pow_field(field1: &str, field2: &str) -> String {
        let field1 = Field::<CurrentNetwork>::from_str(field1).unwrap();
        let field2 = Field::<CurrentNetwork>::from_str(field2).unwrap();
        let result = field1.pow(&field2);
        result.to_string()
    }

    pub fn poseidon_hash(field: &str) -> String {
        let field = Field::<CurrentNetwork>::from_str(field).unwrap();
        let result = CurrentNetwork::hash_many_psd8(&[CurrentNetwork::encryption_domain(), field], 1);
        return result[0].to_string();
    }

    pub fn sqrt(field: &str) -> String {
        let field = Field::<CurrentNetwork>::from_str(field).unwrap();
        let result = field.sqrt().unwrap();
        Field::<CurrentNetwork>::new(result).to_string()
        // result.to_string()
    }

    pub fn add_points(group1: &str, group2: &str) -> String {
        let group1 = Group::<CurrentNetwork>::from_str(group1).unwrap();
        let group2 = Group::<CurrentNetwork>::from_str(group2).unwrap();
        let result = group1 + group2;
        result.to_string()
    }

    pub fn group_scalar_mul(group: &str, scalar: &str) -> String {
        let group = Group::<CurrentNetwork>::from_str(group).unwrap();
        let scalar = Scalar::<CurrentNetwork>::from_str(scalar).unwrap();
        let result = group * scalar;
        result.to_string()
    }

    pub fn msm(groups: Array, scalars: Array) -> String {
        let mut groups_vec = Vec::new();
        let mut scalars_vec = Vec::new();

        // convert groups array to groups_vec
        for i in 0..groups.length() {
            let group = Group::<CurrentNetwork>::from_str(&groups.get(i).as_string().unwrap()).unwrap();
            let affine_group = group.to_affine();
            groups_vec.push(affine_group);
        }
        // convert scalars array to scalars_vec
        for i in 0..scalars.length() {
            let scalar = Scalar::<CurrentNetwork>::from_str(&scalars.get(i).as_string().unwrap()).unwrap();
            let bigint_scalar = scalar.to_bigint();
            scalars_vec.push(bigint_scalar);
        }

        let result = msm(&groups_vec, &scalars_vec);
        let affine_result = result.to_affine();
        let group_result = Group::<CurrentNetwork>::new(affine_result);
        group_result.to_string()
    }

    // pub fn ntt(coeffs: Array) -> String {
    //     let mut coeffs_vec = Vec::new();

    //     // convert coeffs array to coeffs_vec
    //     for i in 0..coeffs.length() {
    //         let coeff = Field::<CurrentNetwork>::from_str(&coeffs.get(i).as_string().unwrap()).unwrap();
    //         coeffs_vec.push(coeff);
    //     }

    //     let domain = EvaluationDomain::<bls12_377::fr>::new(coeffs_vec.len()).unwrap();
    //     let result = domain.fft(&coeffs_vec);
    //     let mut result_vec = Vec::new();
    //     for i in 0..result.len() {
    //         result_vec.push(result[i].to_string());
    //     }
    //     JsValue::from_serde(&result_vec).unwrap().as_string().unwrap()
    // }
}

impl FromStr for Address {
    type Err = anyhow::Error;

    fn from_str(address: &str) -> Result<Self, Self::Err> {
        Ok(Self(AddressNative::from_str(address)?))
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Address {
    type Target = AddressNative;

    fn deref(&self) -> &Self::Target {
        &self.0
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
