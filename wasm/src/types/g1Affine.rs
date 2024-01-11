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

use crate::types::native::G1AffineNative;

use wasm_bindgen::prelude::wasm_bindgen;

use std::str::FromStr;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G1Affine(G1AffineNative);

#[wasm_bindgen]
impl G1Affine {
    #[wasm_bindgen(js_name = "toString")]
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    #[wasm_bindgen(js_name = "fromString")]
    pub fn from_string(x: &str, y: &str) -> Result<G1Affine, String> {
        let xField = G1BaseFieldNative::from_str(x).map_err(|e| e.to_string())?;
        let yField = G1BaseFieldNative::from_str(y).map_err(|e| e.to_string())?;
        Ok(Self(G1AffineNative::new(xField, yField, false).map_err(|e| e.to_string())?))
    }
}

impl From<G1AffineNative> for Group {
    fn from(native: G1AffineNative) -> Self {
        Self(native)
    }
}

impl From<G1Affine> for G1AffineNative {
    fn from(group: G1Affine) -> Self {
        group.0
    }
}
