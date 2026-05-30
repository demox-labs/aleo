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

use snarkvm_console::prelude::{ConsensusVersion, Network, ToBits};

use indexmap::IndexMap;
use snarkvm_algorithms::snark::varuna::VarunaVersion;
use snarkvm_ledger_block::Execution;
use snarkvm_synthesizer::{Process, process::InclusionVersion};

pub fn to_bits<T: ToBits>(value: T) -> Vec<bool> {
    value.to_bits_le()
}

/// Verifies an execution against the given protocol versions.
///
/// In snarkVM v4.7.x `Process::verify_execution` became an associated function that takes the
/// execution's program stacks explicitly (rather than reading them from `&self`). This helper
/// reconstructs that `execution_stacks` map from the process and delegates to it, preserving the
/// call shape the wasm bindings relied on previously.
pub fn verify_execution_with_versions<N: Network>(
    process: &Process<N>,
    consensus_version: ConsensusVersion,
    varuna_version: VarunaVersion,
    inclusion_version: InclusionVersion,
    execution: &Execution<N>,
) -> anyhow::Result<()> {
    let mut execution_stacks = IndexMap::new();
    for transition in execution.transitions() {
        execution_stacks.insert(*transition.program_id(), process.get_stack(transition.program_id())?);
    }
    Process::<N>::verify_execution(consensus_version, varuna_version, inclusion_version, execution, &execution_stacks)
}

/// Verifies an execution using the latest protocol versions. Used by call sites that historically
/// did not thread a consensus version through (they predate the version parameters).
pub fn verify_execution_latest<N: Network>(process: &Process<N>, execution: &Execution<N>) -> anyhow::Result<()> {
    verify_execution_with_versions(
        process,
        consensus_version_from_u8(u8::MAX),
        VarunaVersion::V2,
        InclusionVersion::V1,
        execution,
    )
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
        9 => ConsensusVersion::V9,
        10 => ConsensusVersion::V10,
        11 => ConsensusVersion::V11,
        12 => ConsensusVersion::V12,
        _ => ConsensusVersion::V12
    }
}
