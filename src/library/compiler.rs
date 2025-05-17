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

use alloc::vec::Vec;
use std::collections::BTreeMap;

use crate::isa::{GotoTarget, Instruction};
use crate::library::assembler::AssemblerError;
use crate::{Lib, LibId, LibSite};

/// Errors generated during the library compilation.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Display, Error, From)]
#[display(doc_comments)]
pub enum CompilerError<Isa: Instruction<LibId>> {
    /// Error in assembling the bytecode (see [`AssemblerError`] for the details).
    #[from]
    #[display(inner)]
    Assemble(AssemblerError),

    /// instruction number {1} `{0}` (offset {2:#x}) references goto target absent in the code. Use
    /// `nop` instruction to mark the goto target.
    ///
    /// The known goto target offsets are: {3:#x?}
    InvalidRef(Isa, usize, u16, Vec<u16>),

    /// instruction number {1} `{0}` (offset {2:#x}) references library which is not a dependency
    /// (lib id {3}).
    InvalidLib(Isa, usize, u16, LibId),
}

/// The compiled AluVM library containing information about the routines.
pub struct CompiledLib {
    id: LibId,
    lib: Lib,
    routines: Vec<u16>,
}

impl CompiledLib {
    /// Compiles a library from the provided instructions by resolving local call pointers first
    /// and then assembling it into a bytecode by calling [`Self::assemble`].
    pub fn compile<Isa>(
        mut code: impl AsMut<[Isa]>,
        deps: &[&CompiledLib],
    ) -> Result<Self, CompilerError<Isa>>
    where
        Isa: Instruction<LibId>,
    {
        let deps = deps
            .iter()
            .map(|lib| (lib.id, lib))
            .collect::<BTreeMap<_, _>>();
        let code = code.as_mut();
        let mut routines = vec![];
        let mut cursor = 0u16;
        for instr in &*code {
            if instr.is_goto_target() {
                routines.push(cursor);
            }
            cursor += instr.code_byte_len();
        }
        let mut cursor = 0u16;
        for (no, instr) in code.iter_mut().enumerate() {
            if let GotoTarget::Absolute(goto_pos) = instr.local_goto_pos() {
                let Some(pos) = routines.get(*goto_pos as usize) else {
                    return Err(CompilerError::InvalidRef(instr.clone(), no, cursor, routines));
                };
                *goto_pos = *pos;
            }
            let cloned_instr = instr.clone();
            if let Some(remote_pos) = instr.remote_goto_pos() {
                let Some(lib) = deps.get(&remote_pos.prog_id) else {
                    return Err(CompilerError::InvalidLib(
                        cloned_instr,
                        no,
                        cursor,
                        remote_pos.prog_id,
                    ));
                };
                remote_pos.offset = lib.routine(remote_pos.offset).offset;
            }
            cursor += instr.code_byte_len();
        }
        let lib = Lib::assemble(code)?;
        let id = lib.lib_id();
        Ok(Self { id, lib, routines })
    }

    /// Count the number of routines in the library.
    pub fn routines_count(&self) -> usize { self.routines.len() }

    /// Returns code offset for the entry point of a given routine.
    ///
    /// # Panics
    ///
    /// Panics if the routine with the given number is not defined
    pub fn routine(&self, no: u16) -> LibSite {
        let pos = self.routines[no as usize];
        LibSite::new(self.id, pos)
    }

    /// Get a reference to the underlying [`Lib`].
    pub fn as_lib(&self) -> &Lib { &self.lib }

    /// Convert into the underlying [`Lib`].
    pub fn into_lib(self) -> Lib { self.lib }
}
