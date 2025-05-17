// Reference rust implementation of AluVM (arithmetic logic unit virtual machine).
// To find more on AluVM please check <https://aluvm.org>
//
// SPDX-License-Identifier: Apache-2.0
//
// Designed in 2021-2025 by Dr Maxim Orlovsky <orlovsky@ubideco.org>
// Written in 2021-2025 by Dr Maxim Orlovsky <orlovsky@ubideco.org>
//
// Copyright (C) 2021-2024 LNP/BP Standards Association, Switzerland.
// Copyright (C) 2024-2025 Laboratories for Ubiquitous Deterministic Computing (UBIDECO),
//                         Institute for Distributed and Cognitive Systems (InDCS), Switzerland.
// Copyright (C) 2021-2025 Dr Maxim Orlovsky.
// All rights under the above copyrights are reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the License
// is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express
// or implied. See the License for the specific language governing permissions and limitations under
// the License.

use alloc::collections::BTreeSet;
use core::fmt::{Debug, Display};

use amplify::confinement::TinyOrdSet;

use crate::core::{Core, Register, Site, SiteId};
use crate::isa::Bytecode;
use crate::{CoreExt, IsaId};

/// Turing machine movement after instruction execution
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ExecStep<Site> {
    /// Stop program execution.
    Stop,

    /// Set `CK` to `Fail`. The program execution will halt if `CH` is set.
    Fail,

    /// Move to the next instruction.
    Next,

    /// Jump to the offset from the origin.
    Jump(u16),

    /// Jump to another code fragment.
    Call(Site),

    /// Return to the next instruction after the original caller position.
    Ret(Site),
}

/// A local goto position for the jump instructions.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum GotoTarget<'a> {
    /// The instruction does not perform a local jump.
    ///
    /// NB: It still may call a code from an external library, use [`Instruction::remote_goto_pos`]
    /// to check that.
    None,

    /// An absolute offset in the code segment of the library.
    Absolute(&'a mut u16),

    /// An offset relative to the current position.
    Relative(&'a mut i8),
}

/// Trait for instructions
pub trait Instruction<Id: SiteId>: Display + Debug + Bytecode<Id> + Clone + Eq {
    /// The names of the ISA extension set these instructions cover.
    const ISA_EXT: &'static [&'static str];

    /// Extensions to the AluVM core unit provided by this instruction set.
    type Core: CoreExt;
    /// Context: external data which are accessible to the ISA.
    type Context<'ctx>;

    /// Convert the set of ISA extensions from [`Self::ISA_EXT`] into a set of [`IsaId`].
    fn isa_ext() -> TinyOrdSet<IsaId> {
        let iter = Self::ISA_EXT.iter().copied().map(IsaId::from);
        TinyOrdSet::from_iter_checked(iter)
    }

    /// Whether the instruction can be used as a goto-target.
    fn is_goto_target(&self) -> bool;

    /// If an instruction is a jump operation inside the library, it should return its goto target
    /// position number.
    fn local_goto_pos(&mut self) -> GotoTarget;

    /// If an instruction is a jump operation into an external library, it should return its remote
    /// target.
    fn remote_goto_pos(&mut self) -> Option<&mut Site<Id>>;

    /// Lists all registers which are used by the instruction.
    fn regs(&self) -> BTreeSet<<Self::Core as CoreExt>::Reg> {
        let mut regs = self.src_regs();
        regs.extend(self.dst_regs());
        regs
    }

    /// List of registers which value is taken into account by the instruction.
    fn src_regs(&self) -> BTreeSet<<Self::Core as CoreExt>::Reg>;

    /// List of registers which value may be changed by the instruction.
    fn dst_regs(&self) -> BTreeSet<<Self::Core as CoreExt>::Reg>;

    /// The number of bytes in the source registers.
    fn src_reg_bytes(&self) -> u16 {
        self.src_regs()
            .into_iter()
            .map(<Self::Core as CoreExt>::Reg::bytes)
            .sum()
    }

    /// The number of bytes in the destination registers.
    fn dst_reg_bytes(&self) -> u16 {
        self.dst_regs()
            .into_iter()
            .map(<Self::Core as CoreExt>::Reg::bytes)
            .sum()
    }

    /// The size of the data coming as an instruction operand (i.e., except data coming from
    /// registers or read from outside the instruction operands).
    fn op_data_bytes(&self) -> u16;

    /// The size of the data read by the instruction from outside the registers (except data coming
    /// as a parameter).
    fn ext_data_bytes(&self) -> u16;

    /// Computes base (non-adjusted) complexity of the instruction.
    ///
    /// Called by the default [`Self::complexity`] implementation. See it for more details.
    fn base_complexity(&self) -> u64 {
        (self.op_data_bytes() as u64
            + self.src_reg_bytes() as u64
            + self.dst_reg_bytes() as u64
            + self.ext_data_bytes() as u64 * 2)
            * 8 // per bit
            * 1000 // by default use large unit
    }

    /// Returns computational complexity of the instruction.
    ///
    /// Computational complexity is the number of "CPU ticks" required to process the instruction.
    fn complexity(&self) -> u64 { self.base_complexity() }

    /// Executes the given instruction taking all registers as input and output.
    ///
    /// # Arguments
    ///
    /// The method is provided with the current code position which may be used by the instruction
    /// for constructing call stack.
    ///
    /// # Returns
    ///
    /// Returns whether further execution should be stopped.
    fn exec(
        &self,
        site: Site<Id>,
        core: &mut Core<Id, Self::Core>,
        context: &Self::Context<'_>,
    ) -> ExecStep<Site<Id>>;
}
