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

pub use aleo_rust::{
    Address,
    Ciphertext,
    Encryptor,
    Identifier,
    Plaintext,
    PrivateKey,
    ProgramID,
    Record,
    Signature,
    Testnet3,
    ViewKey,
};
use snarkvm_circuit_network::AleoV0;
use snarkvm_synthesizer::{Fee, Inclusion, Execution, Process, Program, Transaction, Transition, helpers::memory::BlockMemory};
use snarkvm_wasm::program::{ProgramOwner, TransactionLeaf};
use snarkvm_console::program::InputID;
pub use snarkvm_wasm::{
    network::Environment,
    program::{Response, TRANSACTION_DEPTH},
    FromBytes,
    PrimeField,
    ToBytes,
};

// Account types
pub type AddressNative = Address<CurrentNetwork>;
pub type PrivateKeyNative = PrivateKey<CurrentNetwork>;
pub type SignatureNative = Signature<CurrentNetwork>;
pub type ViewKeyNative = ViewKey<CurrentNetwork>;

// Network types
pub type CurrentNetwork = Testnet3;
pub type CurrentAleo = AleoV0;

// Record types
pub type CiphertextNative = Ciphertext<CurrentNetwork>;
pub type PlaintextNative = Plaintext<CurrentNetwork>;
pub type RecordCiphertextNative = Record<CurrentNetwork, CiphertextNative>;
pub type RecordPlaintextNative = Record<CurrentNetwork, PlaintextNative>;

// Program types
pub type CurrentBlockMemory = BlockMemory<CurrentNetwork>;
pub type FeeNative = Fee<CurrentNetwork>;
pub type IdentifierNative = Identifier<CurrentNetwork>;
pub type ProcessNative = Process<CurrentNetwork>;
pub type ProgramNative = Program<CurrentNetwork>;
pub type ProgramIDNative = ProgramID<CurrentNetwork>;
pub type ProgramOwnerNative = ProgramOwner<CurrentNetwork>;
pub type ResponseNative = Response<CurrentNetwork>;
pub type TransactionLeafNative = TransactionLeaf<CurrentNetwork>;
pub type TransactionNative = Transaction<CurrentNetwork>;
pub type TransitionNative = Transition<CurrentNetwork>;
pub type InputIDNative = InputID<CurrentNetwork>;
pub type InclusionNative = Inclusion<CurrentNetwork>;
pub type ExecutionNative = Execution<CurrentNetwork>;
