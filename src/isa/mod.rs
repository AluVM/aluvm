// Reference rust implementation of AluVM (arithmetic logic unit virtual machine).
// To find more on AluVM please check <https://github.com/internet2-org/aluvm-spec>
//
// Designed & written in 2021 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
// This work is donated to LNP/BP Standards Association by Pandora Core AG
//
// This software is licensed under the terms of MIT License.
// You should have received a copy of the MIT License along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

//! AluVM instruction set architecture

#[macro_use]
mod asm;
mod bytecode;
mod exec;
mod flags;
mod instr;
pub mod opcodes;

pub use bytecode::{Bytecode, DecodeError, EncodeError};
pub use exec::{ExecStep, InstructionSet};
pub use flags::{
    DeleteFlag, FloatEqFlag, InsertFlag, IntFlags, MergeFlag, ParseFlagError, RoundingFlag,
    SignFlag, SplitFlag,
};
pub use instr::{
    ArithmeticOp, BitwiseOp, BytesOp, CmpOp, ControlFlowOp, Curve25519Op, DigestOp, Instr, MoveOp,
    PutOp, ReservedOp, Secp256k1Op,
};