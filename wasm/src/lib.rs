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

pub mod account;
pub use account::*;

pub mod record;
pub use record::*;

pub mod program;
pub use program::*;

pub(crate) mod types;
pub(crate) use types::*;

pub use wasm_bindgen_rayon::init_thread_pool;

use wasm_bindgen::prelude::*;
use rayon::iter::IntoParallelIterator;
use rayon::iter::*;
use rayon::iter::ParallelIterator;

#[wasm_bindgen]
pub fn sum(numbers: &[i32]) -> i32 {
  numbers.into_par_iter().with_max_len(1).sum()
}
