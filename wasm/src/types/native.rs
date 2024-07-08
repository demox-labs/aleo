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

pub use snarkvm_circuit_network::{AleoTestnetV0, AleoV0, AleoCanaryV0};
pub use snarkvm_console::{
    account::{Address, PrivateKey, Signature, ViewKey},
    network::{Network, TestnetV0, MainnetV0, CanaryV0},
    program::{
        Ciphertext,
        Entry,
        EntryType,
        Identifier,
        Literal,
        Plaintext,
        PlaintextType,
        ProgramID,
        ProgramOwner,
        Record,
        Response,
        ValueType,
    },
    types::{Field, Group},
};
pub use snarkvm_ledger_block::{Deployment, Execution, Transaction, Transition};
pub use snarkvm_ledger_query::Query;
pub use snarkvm_ledger_store::helpers::memory::BlockMemory;
use snarkvm_synthesizer::Authorization;
pub use snarkvm_synthesizer::{
    snark::{ProvingKey, VerifyingKey},
    Process,
    Program,
    VM,
    Trace
};
pub use snarkvm_wasm::{
    console::network::Environment,
    fields::PrimeField,
    utilities::{FromBytes, ToBytes, Uniform},
};

// Account types
pub type AddressNative<N> = Address<N>;
pub type PrivateKeyNative<N> = PrivateKey<N>;
pub type SignatureNative<N> = Signature<N>;
pub type ViewKeyNative<N> = ViewKey<N>;

// Algebraic types
pub type FieldNative<N> = Field<N>;
pub type GroupNative<N> = Group<N>;

// Record types
pub type CiphertextNative<N> = Ciphertext<N>;
pub type PlaintextNative<N> = Plaintext<N>;
pub type RecordCiphertextNative<N> = Record<N, CiphertextNative<N>>;
pub type RecordPlaintextNative<N> = Record<N, PlaintextNative<N>>;

// Program types
type CurrentBlockMemory<N> = BlockMemory<N>;
pub type AuthorizationNative<N> = Authorization<N>;
pub type ExecutionNative<N> = Execution<N>;
pub type DeploymentNative<N> = Deployment<N>;
pub type IdentifierNative<N> = Identifier<N>;
pub type LiteralNative<N> = Literal<N>;
pub type ProcessNative<N> = Process<N>;
pub type ProgramIDNative<N> = ProgramID<N>;
pub type ProgramNative<N> = Program<N>;
pub type ProgramOwnerNative<N> = ProgramOwner<N>;
pub type ProvingKeyNative<N> = ProvingKey<N>;
pub type QueryNative<N> = Query<N, CurrentBlockMemory<N>>;
pub type ResponseNative<N> = Response<N>;
pub type TransactionNative<N> = Transaction<N>;
pub type TransitionNative<N> = Transition<N>;
pub type VerifyingKeyNative<N> = VerifyingKey<N>;
