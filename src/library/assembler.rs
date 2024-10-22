// Reference rust implementation of AluVM (arithmetic logic unit virtual machine).
// To find more on AluVM please check <https://aluvm.org>
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2021-2024 by
//     Dr Maxim Orlovsky <orlovsky@ubideco.org>
//
// Copyright (C) 2021-2024 UBIDECO Labs,
//     Laboratories for Distributed and Cognitive Computing, Switzerland.
//     All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use amplify::confinement::{self, TinyOrdSet};

use super::{Lib, LibId, MarshallError, Marshaller};
use crate::isa::{Bytecode, BytecodeRead, CodeEofError, InstructionSet};

/// Errors while assembling lib-old from the instruction set.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Display, Error, From)]
#[display(inner)]
pub enum AssemblerError {
    /// Error assembling code and data segments.
    #[from]
    Bytecode(MarshallError),

    /// Error assembling library segment.
    #[from]
    LibSegOverflow(confinement::Error),
}

impl Lib {
    /// Assembles library from the provided instructions by encoding them into bytecode.
    pub fn assemble<Isa>(code: &[Isa::Instr]) -> Result<Lib, AssemblerError>
    where
        Isa: InstructionSet<LibId>,
        Isa::Instr: Bytecode<LibId>,
    {
        let call_sites = code.iter().filter_map(|instr| instr.external_ref());
        let libs_segment = TinyOrdSet::try_from_iter(call_sites)?;

        let mut writer = Marshaller::new(&libs_segment);
        for instr in code.iter() {
            instr.encode_instr(&mut writer)?;
        }
        let (code_segment, data_segment) = writer.finish();

        Ok(Lib {
            isa: Isa::isa_id(),
            isae: Isa::isa_ext(),
            libs: libs_segment,
            code: code_segment,
            data: data_segment,
        })
    }

    /// Disassembles library into a set of instructions.
    pub fn disassemble<Isa>(&self) -> Result<Vec<Isa::Instr>, CodeEofError>
    where
        Isa: InstructionSet<LibId>,
        Isa::Instr: Bytecode<LibId>,
    {
        let mut code = Vec::new();
        let mut reader = Marshaller::with(&self.code, &self.data, &self.libs);
        while !reader.is_eof() {
            code.push(Isa::Instr::decode_instr(&mut reader)?);
        }
        Ok(code)
    }

    /// Disassembles library into a set of instructions and offsets and prints it to the writer.
    pub fn print_disassemble<Isa>(&self, mut writer: impl std::io::Write) -> Result<(), std::io::Error>
    where
        Isa: InstructionSet<LibId>,
        Isa::Instr: Bytecode<LibId>,
    {
        let mut reader = Marshaller::with(&self.code, &self.data, &self.libs);
        while !reader.is_eof() {
            let pos = reader.offset().0 as usize;
            write!(writer, "@x{pos:06X}: ")?;
            match Isa::Instr::decode_instr(&mut reader) {
                Ok(instr) => writeln!(writer, "{instr}")?,
                Err(_) => writeln!(writer, "; <incomplete instruction>")?,
            }
        }
        Ok(())
    }
}