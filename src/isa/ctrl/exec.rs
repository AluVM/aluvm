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

use super::CtrlInstr;
use crate::core::{Core, NoExt, NoRegs, Site, SiteId, Status};
use crate::isa::{ExecStep, GotoTarget, Instr, Instruction, ReservedInstr};

impl<Id: SiteId> Instruction<Id> for Instr<Id> {
    const ISA_EXT: &'static [&'static str] = &[];

    type Core = NoExt;
    type Context<'ctx> = ();

    fn is_goto_target(&self) -> bool {
        match self {
            Instr::Ctrl(instr) => instr.is_goto_target(),
            Instr::Reserved(instr) => Instruction::<Id>::is_goto_target(instr),
        }
    }

    fn local_goto_pos(&mut self) -> GotoTarget {
        match self {
            Instr::Ctrl(instr) => instr.local_goto_pos(),
            Instr::Reserved(instr) => Instruction::<Id>::local_goto_pos(instr),
        }
    }

    fn remote_goto_pos(&mut self) -> Option<&mut Site<Id>> {
        match self {
            Instr::Ctrl(instr) => instr.remote_goto_pos(),
            Instr::Reserved(instr) => Instruction::<Id>::remote_goto_pos(instr),
        }
    }

    fn src_regs(&self) -> BTreeSet<NoRegs> {
        match self {
            Instr::Ctrl(instr) => instr.src_regs(),
            Instr::Reserved(instr) => Instruction::<Id>::src_regs(instr),
        }
    }

    fn dst_regs(&self) -> BTreeSet<NoRegs> {
        match self {
            Instr::Ctrl(instr) => instr.dst_regs(),
            Instr::Reserved(instr) => Instruction::<Id>::dst_regs(instr),
        }
    }

    fn op_data_bytes(&self) -> u16 {
        match self {
            Instr::Ctrl(instr) => instr.op_data_bytes(),
            Instr::Reserved(instr) => Instruction::<Id>::op_data_bytes(instr),
        }
    }

    fn ext_data_bytes(&self) -> u16 {
        match self {
            Instr::Ctrl(instr) => instr.ext_data_bytes(),
            Instr::Reserved(instr) => Instruction::<Id>::ext_data_bytes(instr),
        }
    }

    fn complexity(&self) -> u64 {
        match self {
            Instr::Ctrl(instr) => instr.complexity(),
            Instr::Reserved(instr) => Instruction::<Id>::complexity(instr),
        }
    }

    fn exec(
        &self,
        site: Site<Id>,
        core: &mut Core<Id, Self::Core>,
        _: &Self::Context<'_>,
    ) -> ExecStep<Site<Id>> {
        match self {
            Instr::Ctrl(instr) => instr.exec(site, core, &()),
            Instr::Reserved(instr) => instr.exec(site, core, &()),
        }
    }
}

impl<Id: SiteId> Instruction<Id> for ReservedInstr {
    const ISA_EXT: &'static [&'static str] = &[];

    type Core = NoExt;
    type Context<'ctx> = ();

    fn is_goto_target(&self) -> bool { false }

    fn local_goto_pos(&mut self) -> GotoTarget { GotoTarget::None }

    fn remote_goto_pos(&mut self) -> Option<&mut Site<Id>> { None }

    fn src_regs(&self) -> BTreeSet<NoRegs> { none!() }

    fn dst_regs(&self) -> BTreeSet<NoRegs> { none!() }

    fn op_data_bytes(&self) -> u16 { none!() }

    fn ext_data_bytes(&self) -> u16 { none!() }

    fn complexity(&self) -> u64 { u64::MAX }

    fn exec(
        &self,
        _: Site<Id>,
        _: &mut Core<Id, Self::Core>,
        _: &Self::Context<'_>,
    ) -> ExecStep<Site<Id>> {
        ExecStep::Fail
    }
}

impl<Id: SiteId> Instruction<Id> for CtrlInstr<Id> {
    const ISA_EXT: &'static [&'static str] = &[];

    type Core = NoExt;
    type Context<'ctx> = ();

    fn is_goto_target(&self) -> bool {
        match self {
            CtrlInstr::Nop => true,
            CtrlInstr::ChkCo
            | CtrlInstr::ChkCk
            | CtrlInstr::NotCo
            | CtrlInstr::FailCk
            | CtrlInstr::RsetCk => false,
            CtrlInstr::Jmp { .. } | CtrlInstr::JiOvfl { .. } | CtrlInstr::JiFail { .. } => false,
            CtrlInstr::Sh { .. } | CtrlInstr::ShOvfl { .. } | CtrlInstr::ShFail { .. } => false,
            CtrlInstr::Exec { .. } | CtrlInstr::Fn { .. } | CtrlInstr::Call { .. } => false,
            CtrlInstr::Ret | CtrlInstr::Stop => false,
        }
    }

    fn local_goto_pos(&mut self) -> GotoTarget {
        match self {
            CtrlInstr::Nop
            | CtrlInstr::ChkCo
            | CtrlInstr::ChkCk
            | CtrlInstr::NotCo
            | CtrlInstr::FailCk
            | CtrlInstr::RsetCk => GotoTarget::None,
            CtrlInstr::Jmp { pos }
            | CtrlInstr::JiOvfl { pos }
            | CtrlInstr::JiFail { pos }
            | CtrlInstr::Fn { pos } => GotoTarget::Absolute(pos),
            CtrlInstr::Sh { shift } | CtrlInstr::ShOvfl { shift } | CtrlInstr::ShFail { shift } => {
                GotoTarget::Relative(shift)
            }
            CtrlInstr::Exec { site: _ } | CtrlInstr::Call { site: _ } => GotoTarget::None,
            CtrlInstr::Ret | CtrlInstr::Stop => GotoTarget::None,
        }
    }

    fn remote_goto_pos(&mut self) -> Option<&mut Site<Id>> {
        match self {
            CtrlInstr::Nop
            | CtrlInstr::ChkCo
            | CtrlInstr::ChkCk
            | CtrlInstr::NotCo
            | CtrlInstr::FailCk
            | CtrlInstr::RsetCk => None,
            CtrlInstr::Jmp { pos: _ }
            | CtrlInstr::JiOvfl { pos: _ }
            | CtrlInstr::JiFail { pos: _ }
            | CtrlInstr::Fn { pos: _ } => None,
            CtrlInstr::Sh { shift: _ }
            | CtrlInstr::ShOvfl { shift: _ }
            | CtrlInstr::ShFail { shift: _ } => None,
            CtrlInstr::Exec { site } | CtrlInstr::Call { site } => Some(site),
            CtrlInstr::Ret | CtrlInstr::Stop => None,
        }
    }

    fn src_regs(&self) -> BTreeSet<NoRegs> { none!() }

    fn dst_regs(&self) -> BTreeSet<NoRegs> { none!() }

    fn op_data_bytes(&self) -> u16 {
        match self {
            CtrlInstr::Nop
            | CtrlInstr::ChkCo
            | CtrlInstr::ChkCk
            | CtrlInstr::NotCo
            | CtrlInstr::FailCk
            | CtrlInstr::RsetCk => 0,
            CtrlInstr::Jmp { .. } | CtrlInstr::JiOvfl { .. } | CtrlInstr::JiFail { .. } => 2,
            CtrlInstr::Sh { .. } | CtrlInstr::ShOvfl { .. } | CtrlInstr::ShFail { .. } => 1,
            CtrlInstr::Exec { .. } => 2,
            CtrlInstr::Fn { .. } => 2,
            CtrlInstr::Call { .. } => 2,
            CtrlInstr::Ret | CtrlInstr::Stop => 0,
        }
    }

    fn ext_data_bytes(&self) -> u16 {
        match self {
            CtrlInstr::Nop
            | CtrlInstr::ChkCo
            | CtrlInstr::ChkCk
            | CtrlInstr::NotCo
            | CtrlInstr::FailCk
            | CtrlInstr::RsetCk => 0,
            CtrlInstr::Jmp { .. } | CtrlInstr::JiOvfl { .. } | CtrlInstr::JiFail { .. } => 0,
            CtrlInstr::Sh { .. } | CtrlInstr::ShOvfl { .. } | CtrlInstr::ShFail { .. } => 0,
            CtrlInstr::Exec { .. } => 32,
            CtrlInstr::Fn { .. } => 0,
            CtrlInstr::Call { .. } => 32,
            CtrlInstr::Ret | CtrlInstr::Stop => 0,
        }
    }

    fn complexity(&self) -> u64 {
        let complexity = match self {
            CtrlInstr::Nop => 0,
            CtrlInstr::ChkCo
            | CtrlInstr::ChkCk
            | CtrlInstr::NotCo
            | CtrlInstr::FailCk
            | CtrlInstr::RsetCk => 2,
            CtrlInstr::Jmp { .. } | CtrlInstr::Sh { .. } => 10,
            CtrlInstr::JiOvfl { .. }
            | CtrlInstr::JiFail { .. }
            | CtrlInstr::ShOvfl { .. }
            | CtrlInstr::ShFail { .. } => 20,
            CtrlInstr::Exec { .. } => return self.base_complexity() + 20_000,
            CtrlInstr::Fn { .. } => 30,
            CtrlInstr::Call { .. } => return self.base_complexity() + 20_000,
            CtrlInstr::Ret => 20,
            CtrlInstr::Stop => 0,
        };
        complexity * 1000
    }

    fn exec(
        &self,
        cursor: Site<Id>,
        core: &mut Core<Id, Self::Core>,
        _: &Self::Context<'_>,
    ) -> ExecStep<Site<Id>> {
        let shift_jump = |shift: i8| {
            let Some(pos) = cursor.offset.checked_add_signed(shift as i16) else {
                return ExecStep::Fail;
            };
            ExecStep::Jump(pos)
        };

        match *self {
            CtrlInstr::Nop => {}
            CtrlInstr::ChkCo => {
                if !core.co().is_ok() {
                    return ExecStep::Fail;
                }
            }
            CtrlInstr::ChkCk => {
                if !core.ck().is_ok() {
                    return ExecStep::Stop;
                }
            }
            CtrlInstr::FailCk => {
                if core.fail_ck() {
                    return ExecStep::Stop;
                }
            }
            CtrlInstr::RsetCk => {
                core.set_co(core.ck());
                core.reset_ck()
            }
            CtrlInstr::NotCo => core.set_co(!core.co()),
            CtrlInstr::Jmp { pos } => return ExecStep::Jump(pos),
            CtrlInstr::JiOvfl { pos } => {
                if core.co() == Status::Fail {
                    return ExecStep::Jump(pos);
                }
            }
            CtrlInstr::JiFail { pos } => {
                if core.ck() == Status::Fail {
                    return ExecStep::Jump(pos);
                }
            }
            CtrlInstr::Sh { shift } => {
                return shift_jump(shift);
            }
            CtrlInstr::ShOvfl { shift } => {
                if core.co() == Status::Fail {
                    return shift_jump(shift);
                }
            }
            CtrlInstr::ShFail { shift } => {
                if core.ck() == Status::Fail {
                    return shift_jump(shift);
                }
            }
            CtrlInstr::Exec { site } => return ExecStep::Call(site),
            CtrlInstr::Fn { pos } => {
                return match core.push_cs(cursor) {
                    Some(_) => ExecStep::Jump(pos),
                    None => ExecStep::Fail,
                }
            }
            CtrlInstr::Call { site } => {
                return match core.push_cs(cursor) {
                    Some(_) => ExecStep::Call(site),
                    None => ExecStep::Fail,
                }
            }
            CtrlInstr::Ret => {
                return match core.pop_cs() {
                    Some(site) => ExecStep::Ret(site),
                    None => ExecStep::Stop,
                }
            }
            CtrlInstr::Stop => return ExecStep::Stop,
        }
        ExecStep::Next
    }
}

#[cfg(test)]
mod test {
    #![cfg_attr(coverage_nightly, coverage(off))]

    use core::str::FromStr;

    use super::*;
    use crate::core::Site;
    use crate::LibId;

    const LIB_ID: &str = "5iMb1eHJ-bN5BOe6-9RvBjYL-jF1ELjj-VV7c8Bm-WvFen1Q";

    #[test]
    fn nop() {
        let mut instr = Instr::<LibId>::Ctrl(CtrlInstr::Nop);
        assert_eq!(instr.is_goto_target(), true);
        assert_eq!(instr.local_goto_pos(), GotoTarget::None);
        assert_eq!(instr.remote_goto_pos(), None);
        assert_eq!(instr.regs(), none!());
        assert_eq!(instr.src_regs(), none!());
        assert_eq!(instr.dst_regs(), none!());
        assert_eq!(instr.src_reg_bytes(), 0);
        assert_eq!(instr.dst_reg_bytes(), 0);
        assert_eq!(instr.op_data_bytes(), 0);
        assert_eq!(instr.ext_data_bytes(), 0);
        assert_eq!(instr.base_complexity(), 0);
        assert_eq!(instr.complexity(), instr.base_complexity());
    }

    #[test]
    fn chk() {
        let mut instr = Instr::<LibId>::Ctrl(CtrlInstr::ChkCk);
        assert_eq!(instr.is_goto_target(), false);
        assert_eq!(instr.local_goto_pos(), GotoTarget::None);
        assert_eq!(instr.remote_goto_pos(), None);
        assert_eq!(instr.regs(), none!());
        assert_eq!(instr.src_regs(), none!());
        assert_eq!(instr.dst_regs(), none!());
        assert_eq!(instr.src_reg_bytes(), 0);
        assert_eq!(instr.dst_reg_bytes(), 0);
        assert_eq!(instr.op_data_bytes(), 0);
        assert_eq!(instr.ext_data_bytes(), 0);
        assert_eq!(instr.complexity(), 2000);
    }

    #[test]
    fn not_co() {
        let mut instr = Instr::<LibId>::Ctrl(CtrlInstr::NotCo);
        assert_eq!(instr.is_goto_target(), false);
        assert_eq!(instr.local_goto_pos(), GotoTarget::None);
        assert_eq!(instr.remote_goto_pos(), None);
        assert_eq!(instr.regs(), none!());
        assert_eq!(instr.src_regs(), none!());
        assert_eq!(instr.dst_regs(), none!());
        assert_eq!(instr.src_reg_bytes(), 0);
        assert_eq!(instr.dst_reg_bytes(), 0);
        assert_eq!(instr.op_data_bytes(), 0);
        assert_eq!(instr.ext_data_bytes(), 0);
        assert_eq!(instr.complexity(), 2000);
    }

    #[test]
    fn fail_ck() {
        let mut instr = Instr::<LibId>::Ctrl(CtrlInstr::FailCk);
        assert_eq!(instr.is_goto_target(), false);
        assert_eq!(instr.local_goto_pos(), GotoTarget::None);
        assert_eq!(instr.remote_goto_pos(), None);
        assert_eq!(instr.regs(), none!());
        assert_eq!(instr.src_regs(), none!());
        assert_eq!(instr.dst_regs(), none!());
        assert_eq!(instr.src_reg_bytes(), 0);
        assert_eq!(instr.dst_reg_bytes(), 0);
        assert_eq!(instr.op_data_bytes(), 0);
        assert_eq!(instr.ext_data_bytes(), 0);
        assert_eq!(instr.complexity(), 2000);
    }

    #[test]
    fn reset_ck() {
        let mut instr = Instr::<LibId>::Ctrl(CtrlInstr::RsetCk);
        assert_eq!(instr.is_goto_target(), false);
        assert_eq!(instr.local_goto_pos(), GotoTarget::None);
        assert_eq!(instr.remote_goto_pos(), None);
        assert_eq!(instr.regs(), none!());
        assert_eq!(instr.src_regs(), none!());
        assert_eq!(instr.dst_regs(), none!());
        assert_eq!(instr.src_reg_bytes(), 0);
        assert_eq!(instr.dst_reg_bytes(), 0);
        assert_eq!(instr.op_data_bytes(), 0);
        assert_eq!(instr.ext_data_bytes(), 0);
        assert_eq!(instr.complexity(), 2000);
    }

    #[test]
    fn jmp() {
        let mut instr = Instr::<LibId>::Ctrl(CtrlInstr::Jmp { pos: 0x75AE });
        assert_eq!(instr.is_goto_target(), false);
        assert_eq!(instr.local_goto_pos(), GotoTarget::Absolute(&mut 0x75AE));
        assert_eq!(instr.remote_goto_pos(), None);
        assert_eq!(instr.regs(), none!());
        assert_eq!(instr.src_regs(), none!());
        assert_eq!(instr.dst_regs(), none!());
        assert_eq!(instr.src_reg_bytes(), 0);
        assert_eq!(instr.dst_reg_bytes(), 0);
        assert_eq!(instr.op_data_bytes(), 2);
        assert_eq!(instr.ext_data_bytes(), 0);
        assert_eq!(instr.complexity(), 10_000);
    }

    #[test]
    fn jine() {
        let mut instr = Instr::<LibId>::Ctrl(CtrlInstr::JiOvfl { pos: 0x75AE });
        assert_eq!(instr.is_goto_target(), false);
        assert_eq!(instr.local_goto_pos(), GotoTarget::Absolute(&mut 0x75AE));
        assert_eq!(instr.remote_goto_pos(), None);
        assert_eq!(instr.regs(), none!());
        assert_eq!(instr.src_regs(), none!());
        assert_eq!(instr.dst_regs(), none!());
        assert_eq!(instr.src_reg_bytes(), 0);
        assert_eq!(instr.dst_reg_bytes(), 0);
        assert_eq!(instr.op_data_bytes(), 2);
        assert_eq!(instr.ext_data_bytes(), 0);
        assert_eq!(instr.complexity(), 20_000);
    }

    #[test]
    fn jifail() {
        let mut instr = Instr::<LibId>::Ctrl(CtrlInstr::JiFail { pos: 0x75AE });
        assert_eq!(instr.is_goto_target(), false);
        assert_eq!(instr.local_goto_pos(), GotoTarget::Absolute(&mut 0x75AE));
        assert_eq!(instr.remote_goto_pos(), None);
        assert_eq!(instr.regs(), none!());
        assert_eq!(instr.src_regs(), none!());
        assert_eq!(instr.dst_regs(), none!());
        assert_eq!(instr.src_reg_bytes(), 0);
        assert_eq!(instr.dst_reg_bytes(), 0);
        assert_eq!(instr.op_data_bytes(), 2);
        assert_eq!(instr.ext_data_bytes(), 0);
        assert_eq!(instr.complexity(), 20_000);
    }

    #[test]
    fn sh() {
        let mut instr = Instr::<LibId>::Ctrl(CtrlInstr::Sh { shift: -0x5 });
        assert_eq!(instr.is_goto_target(), false);
        assert_eq!(instr.local_goto_pos(), GotoTarget::Relative(&mut -0x5));
        assert_eq!(instr.remote_goto_pos(), None);
        assert_eq!(instr.regs(), none!());
        assert_eq!(instr.src_regs(), none!());
        assert_eq!(instr.dst_regs(), none!());
        assert_eq!(instr.src_reg_bytes(), 0);
        assert_eq!(instr.dst_reg_bytes(), 0);
        assert_eq!(instr.op_data_bytes(), 1);
        assert_eq!(instr.ext_data_bytes(), 0);
        assert_eq!(instr.complexity(), 10_000);
    }

    #[test]
    fn shne() {
        let mut instr = Instr::<LibId>::Ctrl(CtrlInstr::ShOvfl { shift: -0x5 });
        assert_eq!(instr.is_goto_target(), false);
        assert_eq!(instr.local_goto_pos(), GotoTarget::Relative(&mut -0x5));
        assert_eq!(instr.remote_goto_pos(), None);
        assert_eq!(instr.regs(), none!());
        assert_eq!(instr.src_regs(), none!());
        assert_eq!(instr.dst_regs(), none!());
        assert_eq!(instr.src_reg_bytes(), 0);
        assert_eq!(instr.dst_reg_bytes(), 0);
        assert_eq!(instr.op_data_bytes(), 1);
        assert_eq!(instr.ext_data_bytes(), 0);
        assert_eq!(instr.complexity(), 20_000);
    }

    #[test]
    fn shfail() {
        let mut instr = Instr::<LibId>::Ctrl(CtrlInstr::ShFail { shift: -0x5 });
        assert_eq!(instr.is_goto_target(), false);
        assert_eq!(instr.local_goto_pos(), GotoTarget::Relative(&mut -0x5));
        assert_eq!(instr.remote_goto_pos(), None);
        assert_eq!(instr.regs(), none!());
        assert_eq!(instr.src_regs(), none!());
        assert_eq!(instr.dst_regs(), none!());
        assert_eq!(instr.src_reg_bytes(), 0);
        assert_eq!(instr.dst_reg_bytes(), 0);
        assert_eq!(instr.op_data_bytes(), 1);
        assert_eq!(instr.ext_data_bytes(), 0);
        assert_eq!(instr.complexity(), 20_000);
    }

    #[test]
    fn exec() {
        let lib_id = LibId::from_str(LIB_ID).unwrap();
        let mut site = Site::new(lib_id, 0x69AB);
        let mut instr = Instr::<LibId>::Ctrl(CtrlInstr::Exec { site });
        assert_eq!(instr.is_goto_target(), false);
        assert_eq!(instr.local_goto_pos(), GotoTarget::None);
        assert_eq!(instr.remote_goto_pos(), Some(&mut site));
        assert_eq!(instr.regs(), none!());
        assert_eq!(instr.src_regs(), none!());
        assert_eq!(instr.dst_regs(), none!());
        assert_eq!(instr.src_reg_bytes(), 0);
        assert_eq!(instr.dst_reg_bytes(), 0);
        assert_eq!(instr.op_data_bytes(), 2);
        assert_eq!(instr.ext_data_bytes(), 32);
        assert_eq!(instr.complexity(), 548000);
    }

    #[test]
    fn func() {
        let mut instr = Instr::<LibId>::Ctrl(CtrlInstr::Fn { pos: 0x75AE });
        assert_eq!(instr.is_goto_target(), false);
        assert_eq!(instr.local_goto_pos(), GotoTarget::Absolute(&mut 0x75AE));
        assert_eq!(instr.remote_goto_pos(), None);
        assert_eq!(instr.regs(), none!());
        assert_eq!(instr.src_regs(), none!());
        assert_eq!(instr.dst_regs(), none!());
        assert_eq!(instr.src_reg_bytes(), 0);
        assert_eq!(instr.dst_reg_bytes(), 0);
        assert_eq!(instr.op_data_bytes(), 2);
        assert_eq!(instr.ext_data_bytes(), 0);
        assert_eq!(instr.complexity(), 30_000);
    }

    #[test]
    fn call() {
        let lib_id = LibId::from_str(LIB_ID).unwrap();
        let mut site = Site::new(lib_id, 0x69AB);
        let mut instr = Instr::<LibId>::Ctrl(CtrlInstr::Call { site });
        assert_eq!(instr.is_goto_target(), false);
        assert_eq!(instr.local_goto_pos(), GotoTarget::None);
        assert_eq!(instr.remote_goto_pos(), Some(&mut site));
        assert_eq!(instr.regs(), none!());
        assert_eq!(instr.src_regs(), none!());
        assert_eq!(instr.dst_regs(), none!());
        assert_eq!(instr.src_reg_bytes(), 0);
        assert_eq!(instr.dst_reg_bytes(), 0);
        assert_eq!(instr.op_data_bytes(), 2);
        assert_eq!(instr.ext_data_bytes(), 32);
        assert_eq!(instr.complexity(), 548000);
    }

    #[test]
    fn ret() {
        let mut instr = Instr::<LibId>::Ctrl(CtrlInstr::Ret);
        assert_eq!(instr.is_goto_target(), false);
        assert_eq!(instr.local_goto_pos(), GotoTarget::None);
        assert_eq!(instr.remote_goto_pos(), None);
        assert_eq!(instr.regs(), none!());
        assert_eq!(instr.src_regs(), none!());
        assert_eq!(instr.dst_regs(), none!());
        assert_eq!(instr.src_reg_bytes(), 0);
        assert_eq!(instr.dst_reg_bytes(), 0);
        assert_eq!(instr.op_data_bytes(), 0);
        assert_eq!(instr.ext_data_bytes(), 0);
        assert_eq!(instr.complexity(), 20_000);
    }

    #[test]
    fn stop() {
        let mut instr = Instr::<LibId>::Ctrl(CtrlInstr::Stop);
        assert_eq!(instr.is_goto_target(), false);
        assert_eq!(instr.local_goto_pos(), GotoTarget::None);
        assert_eq!(instr.remote_goto_pos(), None);
        assert_eq!(instr.regs(), none!());
        assert_eq!(instr.src_regs(), none!());
        assert_eq!(instr.dst_regs(), none!());
        assert_eq!(instr.src_reg_bytes(), 0);
        assert_eq!(instr.dst_reg_bytes(), 0);
        assert_eq!(instr.op_data_bytes(), 0);
        assert_eq!(instr.ext_data_bytes(), 0);
        assert_eq!(instr.complexity(), 0);
    }

    #[test]
    fn reserved() {
        let mut instr = Instr::<LibId>::Reserved(default!());
        assert_eq!(instr.is_goto_target(), false);
        assert_eq!(instr.local_goto_pos(), GotoTarget::None);
        assert_eq!(instr.remote_goto_pos(), None);
        assert_eq!(instr.regs(), none!());
        assert_eq!(instr.src_regs(), none!());
        assert_eq!(instr.dst_regs(), none!());
        assert_eq!(instr.src_reg_bytes(), 0);
        assert_eq!(instr.dst_reg_bytes(), 0);
        assert_eq!(instr.op_data_bytes(), 0);
        assert_eq!(instr.ext_data_bytes(), 0);
        assert_eq!(instr.complexity(), u64::MAX);
    }
}
