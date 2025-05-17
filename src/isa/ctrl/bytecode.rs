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

use core::ops::RangeInclusive;

use super::CtrlInstr;
use crate::core::SiteId;
use crate::isa::bytecode::CodeEofError;
use crate::isa::{Bytecode, BytecodeRead, BytecodeWrite, Instr, ReservedInstr};
use crate::Site;

impl<Id: SiteId> Bytecode<Id> for Instr<Id> {
    fn op_range() -> RangeInclusive<u8> { 0..=0xFF }

    fn opcode_byte(&self) -> u8 {
        match self {
            Instr::Ctrl(instr) => instr.opcode_byte(),
            Instr::Reserved(instr) => Bytecode::<Id>::opcode_byte(instr),
        }
    }

    fn code_byte_len(&self) -> u16 {
        match self {
            Instr::Ctrl(instr) => instr.code_byte_len(),
            Instr::Reserved(instr) => Bytecode::<Id>::code_byte_len(instr),
        }
    }

    fn external_ref(&self) -> Option<Id> {
        match self {
            Instr::Ctrl(instr) => instr.external_ref(),
            Instr::Reserved(instr) => Bytecode::<Id>::external_ref(instr),
        }
    }

    fn encode_operands<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where W: BytecodeWrite<Id> {
        match self {
            Instr::Ctrl(instr) => instr.encode_operands(writer),
            Instr::Reserved(instr) => instr.encode_operands(writer),
        }
    }

    fn decode_operands<R>(reader: &mut R, opcode: u8) -> Result<Self, CodeEofError>
    where
        Self: Sized,
        R: BytecodeRead<Id>,
    {
        match opcode {
            op if CtrlInstr::<Id>::op_range().contains(&op) => {
                CtrlInstr::<Id>::decode_operands(reader, op).map(Self::Ctrl)
            }
            _ => ReservedInstr::decode_operands(reader, opcode).map(Self::Reserved),
        }
    }
}

impl<Id: SiteId> Bytecode<Id> for ReservedInstr {
    fn op_range() -> RangeInclusive<u8> { 0..=0x7F }

    fn opcode_byte(&self) -> u8 { self.0 }

    fn code_byte_len(&self) -> u16 { 1 }

    fn external_ref(&self) -> Option<Id> { None }

    fn encode_operands<W>(&self, _writer: &mut W) -> Result<(), W::Error>
    where W: BytecodeWrite<Id> {
        Ok(())
    }

    fn decode_operands<R>(_reader: &mut R, opcode: u8) -> Result<Self, CodeEofError>
    where
        Self: Sized,
        R: BytecodeRead<Id>,
    {
        Ok(ReservedInstr(opcode))
    }
}

#[allow(missing_docs)]
impl<Id: SiteId> CtrlInstr<Id> {
    const START: u8 = 0;
    const END: u8 = Self::START + Self::STOP;

    pub const NOP: u8 = 0;
    pub const NOCO: u8 = 1;
    pub const CHCO: u8 = 2;
    pub const CHCK: u8 = 3;
    pub const FAIL: u8 = 4;
    pub const RSET: u8 = 5;
    pub const JMP: u8 = 6;
    pub const JINE: u8 = 7;
    pub const JIFAIL: u8 = 8;
    pub const SH: u8 = 9;
    pub const SHNE: u8 = 10;
    pub const SHFAIL: u8 = 11;
    pub const EXEC: u8 = 12;
    pub const FN: u8 = 13;
    pub const CALL: u8 = 14;
    pub const RET: u8 = 15;
    pub const STOP: u8 = 16;
}

impl<Id: SiteId> Bytecode<Id> for CtrlInstr<Id> {
    fn op_range() -> RangeInclusive<u8> { Self::START..=Self::END }

    fn opcode_byte(&self) -> u8 {
        match self {
            CtrlInstr::Nop => Self::NOP,
            CtrlInstr::ChkCo => Self::CHCO,
            CtrlInstr::ChkCk => Self::CHCK,
            CtrlInstr::NotCo => Self::NOCO,
            CtrlInstr::FailCk => Self::FAIL,
            CtrlInstr::RsetCk => Self::RSET,
            CtrlInstr::Jmp { .. } => Self::JMP,
            CtrlInstr::JiOvfl { .. } => Self::JINE,
            CtrlInstr::JiFail { .. } => Self::JIFAIL,
            CtrlInstr::Sh { .. } => Self::SH,
            CtrlInstr::ShOvfl { .. } => Self::SHNE,
            CtrlInstr::ShFail { .. } => Self::SHFAIL,
            CtrlInstr::Exec { .. } => Self::EXEC,
            CtrlInstr::Fn { .. } => Self::FN,
            CtrlInstr::Call { .. } => Self::CALL,
            CtrlInstr::Ret => Self::RET,
            CtrlInstr::Stop => Self::STOP,
        }
    }

    fn code_byte_len(&self) -> u16 {
        let arg_bytes = match self {
            CtrlInstr::Nop
            | CtrlInstr::ChkCo
            | CtrlInstr::ChkCk
            | CtrlInstr::NotCo
            | CtrlInstr::FailCk
            | CtrlInstr::RsetCk => 0,
            CtrlInstr::Jmp { pos: _ }
            | CtrlInstr::JiOvfl { pos: _ }
            | CtrlInstr::JiFail { pos: _ }
            | CtrlInstr::Fn { pos: _ } => 2,
            CtrlInstr::Sh { shift: _ }
            | CtrlInstr::ShOvfl { shift: _ }
            | CtrlInstr::ShFail { shift: _ } => 1,
            CtrlInstr::Exec { site: _ } | CtrlInstr::Call { site: _ } => 3,
            CtrlInstr::Ret | CtrlInstr::Stop => 0,
        };
        arg_bytes + 1
    }

    fn external_ref(&self) -> Option<Id> {
        match *self {
            CtrlInstr::Nop
            | CtrlInstr::ChkCo
            | CtrlInstr::ChkCk
            | CtrlInstr::FailCk
            | CtrlInstr::RsetCk
            | CtrlInstr::NotCo
            | CtrlInstr::Ret
            | CtrlInstr::Stop => None,

            CtrlInstr::Jmp { pos: _ }
            | CtrlInstr::JiOvfl { pos: _ }
            | CtrlInstr::JiFail { pos: _ }
            | CtrlInstr::Fn { pos: _ } => None,
            CtrlInstr::Sh { shift: _ }
            | CtrlInstr::ShOvfl { shift: _ }
            | CtrlInstr::ShFail { shift: _ } => None,
            CtrlInstr::Call { site } | CtrlInstr::Exec { site } => Some(site.prog_id),
        }
    }

    fn encode_operands<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where W: BytecodeWrite<Id> {
        match *self {
            CtrlInstr::Nop
            | CtrlInstr::ChkCo
            | CtrlInstr::ChkCk
            | CtrlInstr::FailCk
            | CtrlInstr::RsetCk
            | CtrlInstr::NotCo
            | CtrlInstr::Ret
            | CtrlInstr::Stop => {}

            CtrlInstr::Jmp { pos }
            | CtrlInstr::JiOvfl { pos }
            | CtrlInstr::JiFail { pos }
            | CtrlInstr::Fn { pos } => writer.write_word(pos)?,
            CtrlInstr::Sh { shift } | CtrlInstr::ShOvfl { shift } | CtrlInstr::ShFail { shift } => {
                writer.write_byte(shift.to_le_bytes()[0])?
            }
            CtrlInstr::Call { site } | CtrlInstr::Exec { site } => {
                let site = Site::new(site.prog_id, site.offset);
                writer.write_ref(site.prog_id)?;
                writer.write_word(site.offset)?;
            }
        }
        Ok(())
    }

    fn decode_operands<R>(reader: &mut R, opcode: u8) -> Result<Self, CodeEofError>
    where
        Self: Sized,
        R: BytecodeRead<Id>,
    {
        Ok(match opcode {
            Self::NOP => Self::Nop,
            Self::CHCO => Self::ChkCo,
            Self::CHCK => Self::ChkCk,
            Self::FAIL => Self::FailCk,
            Self::RSET => Self::RsetCk,
            Self::NOCO => Self::NotCo,
            Self::RET => Self::Ret,
            Self::STOP => Self::Stop,

            Self::JMP => CtrlInstr::Jmp { pos: reader.read_word()? },
            Self::JINE => CtrlInstr::JiOvfl { pos: reader.read_word()? },
            Self::JIFAIL => CtrlInstr::JiFail { pos: reader.read_word()? },
            Self::FN => CtrlInstr::Fn { pos: reader.read_word()? },

            Self::SH => CtrlInstr::Sh { shift: i8::from_le_bytes([reader.read_byte()?]) },
            Self::SHNE => CtrlInstr::ShOvfl { shift: i8::from_le_bytes([reader.read_byte()?]) },
            Self::SHFAIL => CtrlInstr::ShFail { shift: i8::from_le_bytes([reader.read_byte()?]) },

            Self::CALL => {
                let prog_id = reader.read_ref()?;
                let offset = reader.read_word()?;
                let site = Site::new(prog_id, offset);
                CtrlInstr::Call { site }
            }
            Self::EXEC => {
                let prog_id = reader.read_ref()?;
                let offset = reader.read_word()?;
                let site = Site::new(prog_id, offset);
                CtrlInstr::Exec { site }
            }

            _ => unreachable!(),
        })
    }
}

#[cfg(test)]
mod test {
    #![cfg_attr(coverage_nightly, coverage(off))]
    use core::str::FromStr;

    use amplify::confinement::SmallBlob;

    use super::*;
    use crate::library::{LibId, LibsSeg, Marshaller};

    const LIB_ID: &str = "5iMb1eHJ-bN5BOe6-9RvBjYL-jF1ELjj-VV7c8Bm-WvFen1Q";

    fn roundtrip(instr: impl Into<Instr<LibId>>, bytecode: impl AsRef<[u8]>) -> SmallBlob {
        let instr = instr.into();
        let mut libs = LibsSeg::new();
        libs.push(LibId::from_str(LIB_ID).unwrap()).unwrap();
        let mut marshaller = Marshaller::new(&libs);
        instr.encode_instr(&mut marshaller).unwrap();
        let (code, data) = marshaller.finish();
        assert_eq!(code.len(), instr.code_byte_len() as usize);
        assert_eq!(code.as_slice(), bytecode.as_ref());
        let mut marshaller = Marshaller::with(code, data, &libs);
        let decoded = Instr::<LibId>::decode_instr(&mut marshaller).unwrap();
        assert_eq!(decoded, instr);
        marshaller.into_code_data().1
    }

    #[test]
    fn nop() {
        let instr = Instr::<LibId>::Ctrl(CtrlInstr::Nop);
        roundtrip(instr, [CtrlInstr::<LibId>::NOP]);
        assert_eq!(instr.code_byte_len(), 1);
        assert_eq!(instr.opcode_byte(), CtrlInstr::<LibId>::NOP);
        assert_eq!(instr.external_ref(), None);
    }

    #[test]
    fn chk() {
        let instr = Instr::<LibId>::Ctrl(CtrlInstr::ChkCk);
        roundtrip(instr, [CtrlInstr::<LibId>::CHCK]);
        assert_eq!(instr.code_byte_len(), 1);
        assert_eq!(instr.opcode_byte(), CtrlInstr::<LibId>::CHCK);
        assert_eq!(instr.external_ref(), None);
    }

    #[test]
    fn not_co() {
        let instr = Instr::<LibId>::Ctrl(CtrlInstr::NotCo);
        roundtrip(instr, [CtrlInstr::<LibId>::NOCO]);
        assert_eq!(instr.code_byte_len(), 1);
        assert_eq!(instr.opcode_byte(), CtrlInstr::<LibId>::NOCO);
        assert_eq!(instr.external_ref(), None);
    }

    #[test]
    fn fail_ck() {
        let instr = Instr::<LibId>::Ctrl(CtrlInstr::FailCk);
        roundtrip(instr, [CtrlInstr::<LibId>::FAIL]);
        assert_eq!(instr.code_byte_len(), 1);
        assert_eq!(instr.opcode_byte(), CtrlInstr::<LibId>::FAIL);
        assert_eq!(instr.external_ref(), None);
    }

    #[test]
    fn reset_ck() {
        let instr = Instr::<LibId>::Ctrl(CtrlInstr::RsetCk);
        roundtrip(instr, [CtrlInstr::<LibId>::RSET]);
        assert_eq!(instr.code_byte_len(), 1);
        assert_eq!(instr.opcode_byte(), CtrlInstr::<LibId>::RSET);
        assert_eq!(instr.external_ref(), None);
    }

    #[test]
    fn jmp() {
        let instr = Instr::<LibId>::Ctrl(CtrlInstr::Jmp { pos: 0x75AE });
        roundtrip(instr, [CtrlInstr::<LibId>::JMP, 0xAE, 0x75]);
        assert_eq!(instr.code_byte_len(), 3);
        assert_eq!(instr.opcode_byte(), CtrlInstr::<LibId>::JMP);
        assert_eq!(instr.external_ref(), None);
    }

    #[test]
    fn jine() {
        let instr = Instr::<LibId>::Ctrl(CtrlInstr::JiOvfl { pos: 0x75AE });
        roundtrip(instr, [CtrlInstr::<LibId>::JINE, 0xAE, 0x75]);
        assert_eq!(instr.code_byte_len(), 3);
        assert_eq!(instr.opcode_byte(), CtrlInstr::<LibId>::JINE);
        assert_eq!(instr.external_ref(), None);
    }

    #[test]
    fn jifail() {
        let instr = Instr::<LibId>::Ctrl(CtrlInstr::JiFail { pos: 0x75AE });
        roundtrip(instr, [CtrlInstr::<LibId>::JIFAIL, 0xAE, 0x75]);
        assert_eq!(instr.code_byte_len(), 3);
        assert_eq!(instr.opcode_byte(), CtrlInstr::<LibId>::JIFAIL);
        assert_eq!(instr.external_ref(), None);
    }

    #[test]
    fn sh() {
        let instr = Instr::<LibId>::Ctrl(CtrlInstr::Sh { shift: -0x5 });
        roundtrip(instr, [CtrlInstr::<LibId>::SH, 255 - 5 + 1]);
        assert_eq!(instr.code_byte_len(), 2);
        assert_eq!(instr.opcode_byte(), CtrlInstr::<LibId>::SH);
        assert_eq!(instr.external_ref(), None);
    }

    #[test]
    fn shne() {
        let instr = Instr::<LibId>::Ctrl(CtrlInstr::ShOvfl { shift: -0x5 });
        roundtrip(instr, [CtrlInstr::<LibId>::SHNE, 255 - 5 + 1]);
        assert_eq!(instr.code_byte_len(), 2);
        assert_eq!(instr.opcode_byte(), CtrlInstr::<LibId>::SHNE);
        assert_eq!(instr.external_ref(), None);
    }

    #[test]
    fn shfail() {
        let instr = Instr::<LibId>::Ctrl(CtrlInstr::ShFail { shift: -0x5 });
        roundtrip(instr, [CtrlInstr::<LibId>::SHFAIL, 255 - 5 + 1]);
        assert_eq!(instr.code_byte_len(), 2);
        assert_eq!(instr.opcode_byte(), CtrlInstr::<LibId>::SHFAIL);
        assert_eq!(instr.external_ref(), None);
    }

    #[test]
    fn exec() {
        let lib_id = LibId::from_str(LIB_ID).unwrap();
        let instr = Instr::<LibId>::Ctrl(CtrlInstr::Exec { site: Site::new(lib_id, 0x69AB) });
        roundtrip(instr, [CtrlInstr::<LibId>::EXEC, 0x00, 0xAB, 0x69]);
        assert_eq!(instr.code_byte_len(), 4);
        assert_eq!(instr.opcode_byte(), CtrlInstr::<LibId>::EXEC);
        assert_eq!(instr.external_ref(), Some(lib_id));
    }

    #[test]
    fn func() {
        let instr = Instr::<LibId>::Ctrl(CtrlInstr::Fn { pos: 0x75AE });
        roundtrip(instr, [CtrlInstr::<LibId>::FN, 0xAE, 0x75]);
        assert_eq!(instr.code_byte_len(), 3);
        assert_eq!(instr.opcode_byte(), CtrlInstr::<LibId>::FN);
        assert_eq!(instr.external_ref(), None);
    }

    #[test]
    fn call() {
        let lib_id = LibId::from_str(LIB_ID).unwrap();
        let instr = Instr::<LibId>::Ctrl(CtrlInstr::Call { site: Site::new(lib_id, 0x69AB) });
        roundtrip(instr, [CtrlInstr::<LibId>::CALL, 0x00, 0xAB, 0x69]);
        assert_eq!(instr.code_byte_len(), 4);
        assert_eq!(instr.opcode_byte(), CtrlInstr::<LibId>::CALL);
        assert_eq!(instr.external_ref(), Some(lib_id));
    }

    #[test]
    fn ret() {
        let instr = Instr::<LibId>::Ctrl(CtrlInstr::Ret);
        roundtrip(instr, [CtrlInstr::<LibId>::RET]);
        assert_eq!(instr.code_byte_len(), 1);
        assert_eq!(instr.opcode_byte(), CtrlInstr::<LibId>::RET);
        assert_eq!(instr.external_ref(), None);
    }

    #[test]
    fn stop() {
        let instr = Instr::<LibId>::Ctrl(CtrlInstr::Stop);
        roundtrip(instr, [CtrlInstr::<LibId>::STOP]);
        assert_eq!(instr.code_byte_len(), 1);
        assert_eq!(instr.opcode_byte(), CtrlInstr::<LibId>::STOP);
        assert_eq!(instr.external_ref(), None);
    }

    #[test]
    fn reserved() {
        let instr = Instr::<LibId>::Reserved(default!());
        roundtrip(instr, [0xFF]);

        assert_eq!(instr.code_byte_len(), 1);
        assert_eq!(instr.opcode_byte(), 0xFF);
        assert_eq!(instr.external_ref(), None);
    }
}
