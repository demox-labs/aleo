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

use crate::types::StatePathNative;

use core::{str::FromStr};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct StatePath(StatePathNative);

#[wasm_bindgen]
impl StatePath {
    pub fn from_string(state_path: &str) -> Result<StatePath, String> {
        Self::from_str(state_path).map_err(|_| "Invalid state path".to_string())
    }
}

impl FromStr for StatePath {
    type Err = anyhow::Error;

    fn from_str(state_path: &str) -> Result<Self, Self::Err> {
        Ok(Self(StatePathNative::from_str(state_path)?))
    }
}

impl From<StatePath> for StatePathNative {
  fn from(state_path: StatePath) -> StatePathNative {
      state_path.0
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatePathMap {
    pub map: HashMap<String, StatePathNative>
}