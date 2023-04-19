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

use super::{Address, PrivateKey};
use crate::{record::RecordCiphertext, types::ViewKeyNative};

use core::{convert::TryFrom, fmt, ops::Deref, str::FromStr};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ViewKey(ViewKeyNative);

#[wasm_bindgen]
impl ViewKey {
    pub fn from_private_key(private_key: &PrivateKey) -> Self {
        Self(ViewKeyNative::try_from(**private_key).unwrap())
    }

    pub fn from_string(view_key: &str) -> Self {
        Self::from_str(view_key).unwrap()
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    pub fn to_address(&self) -> Address {
        Address::from_view_key(self)
    }

    pub fn decrypt(&self, ciphertext: &str) -> Result<String, String> {
        let ciphertext = RecordCiphertext::from_str(ciphertext).map_err(|error| error.to_string())?;
        match ciphertext.decrypt(self) {
            Ok(plaintext) => Ok(plaintext.to_string()),
            Err(error) => Err(error),
        }
    }
}

impl FromStr for ViewKey {
    type Err = anyhow::Error;

    fn from_str(view_key: &str) -> Result<Self, Self::Err> {
        Ok(Self(ViewKeyNative::from_str(view_key)?))
    }
}

impl fmt::Display for ViewKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for ViewKey {
    type Target = ViewKeyNative;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use wasm_bindgen_test::*;

    const OWNER_PLAINTEXT: &str = "{\n  owner: aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px.private,\n  microcredits: 374999900000000u64.private,\n  _nonce: 2740969152411805314773998757266754905498340517856884701781803565459552168415group.public\n}";
    const OWNER_CIPHERTEXT: &str = "record1qyqspsy0qghu8wqmf8wq2w4ccqqg8zsgxc3ge2znf4uklh8tutq2swgqqyxx66trwfhkxun9v35hguerqqpqzq9c6u30j7srax79wdvdqt2ytpne4vyvae6z9fq85rs09nj2f72uqm0kn9tx5t3znnj7hrqffzdcquacgyrqdfuuum2km7wvxcmy258svvkjzsh";
    const OWNER_VIEW_KEY: &str = "AViewKey1mSnpFFC8Mj4fXbK5YiWgZ3mjiV8CxA79bYNa8ymUpTrw";
    const NON_OWNER_VIEW_KEY: &str = "AViewKey1e2WyreaH5H4RBcioLL2GnxvHk5Ud46EtwycnhTdXLmXp";

    #[wasm_bindgen_test]
    pub fn test_from_private_key() {
        let given_private_key = "APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH";
        let given_view_key = "AViewKey1mSnpFFC8Mj4fXbK5YiWgZ3mjiV8CxA79bYNa8ymUpTrw";
        let private_key = PrivateKey::from_string(given_private_key).unwrap();
        let view_key = ViewKey::from_private_key(&private_key);
        assert_eq!(given_view_key, view_key.to_string());
    }

    #[wasm_bindgen_test]
    pub fn test_decrypt_success() {
        let view_key = ViewKey::from_string(OWNER_VIEW_KEY);
        let plaintext = view_key.decrypt(OWNER_CIPHERTEXT);
        assert!(plaintext.is_ok());
        assert_eq!(OWNER_PLAINTEXT, plaintext.unwrap())
    }

    #[wasm_bindgen_test]
    pub fn test_decrypt_fails() {
        let ciphertext = RecordCiphertext::from_str(OWNER_CIPHERTEXT).map_err(|error| error.to_string()).unwrap();
        let incorrect_view_key = ViewKey::from_string(NON_OWNER_VIEW_KEY);
        let plaintext = ciphertext.decrypt(&incorrect_view_key);
        assert!(plaintext.is_err());
    }
}
