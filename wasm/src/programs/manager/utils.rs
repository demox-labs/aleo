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

use snarkvm_console::prelude::{ConsensusVersion, ToBits};

pub fn to_bits<T: ToBits>(value: T) -> Vec<bool> {
    value.to_bits_le()
}

pub fn consensus_version_from_u8(value: u8) -> ConsensusVersion {
    match value {
        1 => ConsensusVersion::V1,
        2 => ConsensusVersion::V2,
        3 => ConsensusVersion::V3,
        4 => ConsensusVersion::V4,
        5 => ConsensusVersion::V5,
        6 => ConsensusVersion::V6,
        7 => ConsensusVersion::V7,
        8 => ConsensusVersion::V8,
        _ => ConsensusVersion::V8,
    }
}
