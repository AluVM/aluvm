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

extern crate alloc;

use aluvm::isa::{CtrlInstr, Instr};
use aluvm::regs::Status;
use aluvm::{aluasm, CompiledLib, CoreConfig, LibId, LibSite, Vm};

fn code() -> Vec<Instr<LibId>> {
    const MAIN: u16 = 0;
    const SUB: u16 = 1;
    const END: u16 = 2;

    aluasm! {
       routine MAIN:
        chk     CO;
        chk     CK;

        jif     CO, MAIN;
        jif     CO, -1;

        jif     CK, MAIN;
        jif     CK, -1;

        fail    CK;
        mov     CO, CK;
        chk     CK;
        not     CO;
        chk     CO;

        jmp     +5;
        jmp     MAIN; // this is skipped

        call    SUB;
        stop;

       routine  SUB:
        jmp     END;
       label    END:
        ret;
    }
}

#[test]
fn run() {
    let code = code();

    let lib = CompiledLib::compile(code.clone(), &[]).unwrap().into_lib();
    let mut disasm = lib.disassemble::<Instr<_>>().unwrap();
    assert_eq!(disasm[14], CtrlInstr::Fn { pos: 27 }.into());
    assert_eq!(disasm[17], CtrlInstr::Jmp { pos: 31 }.into());
    disasm[14] = CtrlInstr::Fn { pos: 1 }.into();
    disasm[17] = CtrlInstr::Jmp { pos: 2 }.into();
    assert_eq!(disasm, code);

    let mut vm_main =
        Vm::<Instr<LibId>>::with(CoreConfig { halt: false, complexity_lim: None }, ());
    let resolver = |_: LibId| Some(&lib);
    let status = vm_main.exec(LibSite::new(lib.lib_id(), 0), &(), resolver);
    assert_eq!(status, Status::Ok);
}

#[test]
fn print_disassemble() {
    let lib = CompiledLib::compile(code(), &[]).unwrap().into_lib();
    let mut buf = Vec::new();
    lib.print_disassemble::<Instr<_>>(&mut buf).unwrap();
    assert_eq!(
        String::from_utf8(buf).unwrap(),
        "offset 000000: nop
offset 000001: chk     CO
offset 000002: chk     CK
offset 000003: jif     CO, 0
offset 000006: jif     CO, -1
offset 000008: jif     CK, 0
offset 000011: jif     CK, -1
offset 000013: fail    CK
offset 000014: mov     CO, CK
offset 000015: chk     CK
offset 000016: not     CO
offset 000017: chk     CO
offset 000018: jmp     +5
offset 000020: jmp     0
offset 000023: call    27
offset 000026: stop
offset 000027: nop
offset 000028: jmp     31
offset 000031: nop
offset 000032: ret
"
    );
}
