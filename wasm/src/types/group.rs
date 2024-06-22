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

use crate::types::native::GroupNative;
use crate::native::Network;

use wasm_bindgen::prelude::wasm_bindgen;

use std::str::FromStr;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Group(String);

#[wasm_bindgen]
impl Group {
    #[wasm_bindgen(js_name = "toString")]
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        self.0.clone()
    }

    #[wasm_bindgen(js_name = "fromString")]
    pub fn from_string(network: &str, group: &str) -> Result<Group, String> {
      match dispatch_network!(network, from_string_impl, group) {
        Ok(result) => Ok(Self(result)),
        Err(e) => return Err(e)
      }
    }
}

pub fn from_string_impl<N: Network>(group: &str) -> Result<String, String> {
  Ok(GroupNative::<N>::from_str(group).map_err(|e| e.to_string())?.to_string())
}

impl<N: Network> From<GroupNative<N>> for Group {
    fn from(native: GroupNative<N>) -> Self {
        Self(native.to_string())
    }
}

impl<N: Network> From<Group> for GroupNative<N> {
    fn from(group: Group) -> Self {
      GroupNative::<N>::from_str(&group.0).unwrap()
    }
}
