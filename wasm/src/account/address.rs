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
    account::{PrivateKey, Signature, ViewKey},
    types::{AddressNative, CurrentNetwork},
};

use core::{convert::TryFrom, fmt, ops::Deref, str::FromStr};
use aleo_rust::{Network, Field};
use snarkvm_wasm::{FromBytes, program::{ProjectiveCurve, Environment}};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Address(AddressNative);

#[wasm_bindgen]
impl Address {
    pub fn from_private_key(private_key: &PrivateKey) -> Self {
        Self(AddressNative::try_from(**private_key).unwrap())
    }

    pub fn from_view_key(view_key: &ViewKey) -> Self {
        Self(AddressNative::try_from(**view_key).unwrap())
    }

    pub fn from_string(address: &str) -> Self {
        Self::from_str(address).unwrap()
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

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

    pub fn poseidon_hash(field: &str) -> String {
        let field = Field::<CurrentNetwork>::from_str(field).unwrap();
        let result = CurrentNetwork::hash_many_psd8(&[CurrentNetwork::encryption_domain(), field], 1);
        return result[0].to_string();
    }
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
