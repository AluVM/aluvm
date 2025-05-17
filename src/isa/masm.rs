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

/// Macro compiler for AluVM assembler.
///
/// # Example
///
/// ```
/// ##![cfg_attr(coverage_nightly, feature(coverage_attribute), coverage(off))]
/// use aluvm::isa::Instr;
/// use aluvm::regs::Status;
/// use aluvm::{aluasm, Lib, LibId, LibSite, Vm};
///
/// const START: u16 = 0;
///
/// let code = aluasm! {
///    routine  START:
///     not     CO;
///     fail    CK;
///     mov     CO, CK;
///     chk     CO;
///     jif     CO, +2;
///     jif     CK, -2;
///     jmp     +2;
///     call    START;
///     stop;
/// };
///
/// let lib = Lib::assemble::<Instr<LibId>>(&code).unwrap();
/// let mut vm = Vm::<Instr<LibId>>::new();
/// match vm.exec(LibSite::new(lib.lib_id(), 0), &(), |_| Some(&lib)) {
///     Status::Ok => println!("success"),
///     Status::Fail => println!("failure"),
/// }
/// ```
#[macro_export]
macro_rules! aluasm {
    ($( $tt:tt )+) => {{
        use $crate::instr;
        #[cfg(not(feature = "std"))]
        use alloc::vec::Vec;

        let mut code: Vec<$crate::isa::Instr<$crate::LibId>> = Default::default();
        #[allow(unreachable_code)] {
            $crate::aluasm_inner! { code => $( $tt )+ }
        }
        code
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! aluasm_inner {
    // end of program
    { $code:ident => } => { };
    // skipped annotations
    { $code:ident => offset $_:literal : $($tt:tt)* } => {
        $crate::aluasm_inner! { $code => $( $tt )* }
    };
    { $code:ident => site $lib:ident @ $_:literal : $($tt:tt)* } => {
        $crate::aluasm_inner! { $code => $( $tt )* }
    };
    // macro instruction
    { $code:ident => $masm:ident $label:ident : $($tt:tt)* } => {
        $code.push(instr!{ $masm $label : });
        $crate::aluasm_inner! { $code => $( $tt )* }
    };
    // no operands
    { $code:ident => $op:ident ; $($tt:tt)* } => {
        $code.push(instr!{ $op });
        $crate::aluasm_inner! { $code => $( $tt )* }
    };
    // operands are all literals
    { $code:ident => $op:ident $( $arg:literal ),+ ; $($tt:tt)* } => {
        $code.push(instr!{ $op $( $arg ),+ });
        $crate::aluasm_inner! { $code => $( $tt )* }
    };
    // operands are all idents
    { $code:ident => $op:ident $( $arg:ident ),+ ; $($tt:tt)* } => {
        $code.push(instr!{ $op $( $arg ),+ });
        $crate::aluasm_inner! { $code => $( $tt )* }
    };
    // operand is a positive shift
    { $code:ident => $op:ident + $pos:literal ; $($tt:tt)* } => {
        $code.push(instr!{ $op + $pos });
        $crate::aluasm_inner! { $code => $( $tt )* }
    };
    { $code:ident => $op:ident $arg:ident, + $pos:literal ; $($tt:tt)* } => {
        $code.push(instr!{ $op $arg, + $pos });
        $crate::aluasm_inner! { $code => $( $tt )* }
    };
    // operand is a negative shift
    { $code:ident => $op:ident - $pos:literal ; $($tt:tt)* } => {
        $code.push(instr!{ $op - $pos });
        $crate::aluasm_inner! { $code => $( $tt )* }
    };
    { $code:ident => $op:ident $arg:ident, - $pos:literal ; $($tt:tt)* } => {
        $code.push(instr!{ $op $arg, - $pos });
        $crate::aluasm_inner! { $code => $( $tt )* }
    };
    // operands are indent followed by a literal
    { $code:ident => $op:ident $arg:ident, $val:literal ; $($tt:tt)* } => {
        $code.push(instr!{ $op $arg, $val });
        $crate::aluasm_inner! { $code => $( $tt )* }
    };
    // suffixes
    { $code:ident => $op:ident $val:literal . $ty:ident ; $($tt:tt)* } => {
        $code.push(instr!{ $op $val.$ty });
        $crate::aluasm_inner! { $code => $( $tt )* }
    };
    { $code:ident => $op:ident $reg:ident, $val:literal . $ty:ident ; $($tt:tt)* } => {
        $code.push(instr!{ $op $reg, $val.$ty });
        $crate::aluasm_inner! { $code => $( $tt )* }
    };
    // external constants and variables
    { $code:ident => $op:ident & $val:ident ; $($tt:tt)* } => {
        $code.push(instr!{ $op & $val });
        $crate::aluasm_inner! { $code => $( $tt )* }
    };
    { $code:ident => $op:ident $reg:ident, & $val:ident ; $($tt:tt)* } => {
        $code.push(instr!{ $op $reg, & $val });
        $crate::aluasm_inner! { $code => $( $tt )* }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! instr {
    (routine $_:ident :) => {
        $crate::isa::CtrlInstr::Nop.into()
    };
    (proc $_:ident :) => {
        $crate::isa::CtrlInstr::Nop.into()
    };
    (label $_:ident :) => {
        $crate::isa::CtrlInstr::Nop.into()
    };
    (loop $_:ident :) => {
        $crate::isa::CtrlInstr::Nop.into()
    };

    (nop) => {
        $crate::isa::CtrlInstr::Nop.into()
    };
    (chk CO) => {
        $crate::isa::CtrlInstr::ChkCo.into()
    };
    (chk CK) => {
        $crate::isa::CtrlInstr::ChkCk.into()
    };
    (not CO) => {
        $crate::isa::CtrlInstr::NotCo.into()
    };
    (fail CK) => {
        $crate::isa::CtrlInstr::FailCk.into()
    };
    (mov CO,CK) => {
        $crate::isa::CtrlInstr::RsetCk.into()
    };
    (ret) => {
        $crate::isa::CtrlInstr::Ret.into()
    };
    (stop) => {
        $crate::isa::CtrlInstr::Stop.into()
    };

    // Jumps
    (jmp $pos:literal) => {
        $crate::isa::CtrlInstr::Jmp { pos: $pos }.into()
    };
    (jmp $pos:ident) => {
        $crate::isa::CtrlInstr::Jmp { pos: $pos }.into()
    };

    (jif CO, + $shift:literal) => {
        $crate::isa::CtrlInstr::ShOvfl { shift: $shift }.into()
    };
    (jif CO, - $shift:literal) => {
        $crate::isa::CtrlInstr::ShOvfl { shift: -$shift }.into()
    };
    (jif CK, + $shift:literal) => {
        $crate::isa::CtrlInstr::ShFail { shift: $shift }.into()
    };
    (jif CK, - $shift:literal) => {
        $crate::isa::CtrlInstr::ShFail { shift: -$shift }.into()
    };
    (jif CO, $pos:literal) => {
        $crate::isa::CtrlInstr::JiOvfl { pos: $pos }.into()
    };
    (jif CK, $pos:literal) => {
        $crate::isa::CtrlInstr::JiFail { pos: $pos }.into()
    };
    (jif CO, $pos:ident) => {
        $crate::isa::CtrlInstr::JiOvfl { pos: $pos }.into()
    };
    (jif CK, $pos:ident) => {
        $crate::isa::CtrlInstr::JiFail { pos: $pos }.into()
    };
    (jmp + $shift:literal) => {
        $crate::isa::CtrlInstr::Sh { shift: $shift }.into()
    };
    (jmp - $shift:literal) => {
        $crate::isa::CtrlInstr::Sh { shift: -$shift }.into()
    };

    // Calls
    (jmp $lib:ident, $pos:literal) => {
        $crate::isa::CtrlInstr::Exec { site: $crate::Site::new($lib, $pos).into() }.into()
    };
    (jmp $lib:ident, $pos:ident) => {
        $crate::isa::CtrlInstr::Exec { site: $crate::Site::new($lib, $pos).into() }.into()
    };
    (call $lib:ident, $pos:literal) => {
        $crate::isa::CtrlInstr::Call { site: $crate::Site::new($lib, $pos).into() }.into()
    };
    (call $lib:ident, $pos:ident) => {
        $crate::isa::CtrlInstr::Call { site: $crate::Site::new($lib, $pos).into() }.into()
    };
    (call $pos:literal) => {
        $crate::isa::CtrlInstr::Fn { pos: $pos }.into()
    };
    (call $pos:ident) => {
        $crate::isa::CtrlInstr::Fn { pos: $pos }.into()
    };

    // Halt
    (halt) => {
        $crate::isa::ReservedInstr::default().into()
    };
}
