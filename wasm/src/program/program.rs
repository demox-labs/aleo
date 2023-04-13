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

use crate::types::ProgramNative;

use core::{ops::Deref, str::FromStr};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Program(ProgramNative);

#[wasm_bindgen]
impl Program {
    /// Get the id of the program
    pub fn id(&self) -> String {
        self.0.id().to_string()
    }

    /// Create a program from a string representation
    ///
    /// This function will fail if the text is not a valid program
    pub fn from_string(program: &str) -> Result<Program, String> {
        Self::from_str(program).map_err(|_| "Invalid program".to_string())
    }

    /// Get a string representation of the program
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    /// Get the default credits program
    ///
    /// This function shouldn't fail as the credits program is always defined
    pub fn credits() -> Result<Program, String> {
        let credits_program = ProgramNative::credits().map_err(|_| "Could not load credits program".to_string())?;
        Ok(Self(credits_program))
    }
}

impl From<ProgramNative> for Program {
  fn from(program: ProgramNative) -> Self {
      Self(program)
  }
}

impl From<Program> for ProgramNative {
  fn from(program: Program) -> Self {
      program.0
  }
}

impl FromStr for Program {
  type Err = anyhow::Error;

  fn from_str(program: &str) -> Result<Self, Self::Err> {
      Ok(Self(ProgramNative::from_str(program)?))
  }
}

impl Deref for Program {
  type Target = ProgramNative;

  fn deref(&self) -> &Self::Target {
      &self.0
  }
}