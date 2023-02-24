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

use crate::types::{StateRootNative};

use core::{str::FromStr};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct StateRoot(StateRootNative);

#[wasm_bindgen]
impl StateRoot {
    pub fn from_string(state_root: &str) -> Result<StateRoot, String> {
        Self::from_str(state_root).map_err(|_| "Invalid state root".to_string())
    }
}

impl FromStr for StateRoot {
    type Err = anyhow::Error;

    fn from_str(state_root: &str) -> Result<Self, Self::Err> {
        Ok(Self(StateRootNative::from_str(state_root)?))
    }
}

// impl From<StateRoot> for StateRootNative {
//     fn from(state_root: StateRoot) -> StateRootNative {
//         state_root.0
//     }
// }