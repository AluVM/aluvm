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

use crate::core::SiteId;
use crate::Site;

/// Control flow instructions.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Display)]
#[display(inner)]
pub enum CtrlInstr<Id: SiteId> {
    /// Not an operation.
    #[display("nop")]
    Nop,

    /// Test `CO` value, terminates if set to true.
    #[display("chk     CO")]
    ChkCo,

    /// Test `CK` value, terminates if in a failed state.
    #[display("chk     CK")]
    ChkCk,

    /// Invert `CO` register.
    #[display("not     CO")]
    NotCo,

    /// Set `CK` register to a failed state.
    #[display("fail    CK")]
    FailCk,

    ///  Assigns `CK` value to `CO` resister and sets `CK` to a non-failed state.
    #[display("mov     CO, CK")]
    RsetCk,

    /// Jump to location (unconditionally).
    #[display("jmp     {pos}")]
    Jmp {
        /** Target position to jump to */
        pos: u16,
    },

    /// Jump to location if `CO` is in a failed state.
    #[display("jif     CO, {pos}")]
    JiOvfl {
        /** Target position to jump to */
        pos: u16,
    },

    /// Jump to location if `CK` is in a failed state.
    #[display("jif     CK, {pos}")]
    JiFail {
        /** Target position to jump to */
        pos: u16,
    },

    /// Relative jump.
    #[display("jmp     {shift:+}")]
    Sh {
        /** Number of bytes for the relative shift */
        shift: i8,
    },

    /// Relative jump if `CO` is in a failed state.
    #[display("jif     CO, {shift:+}")]
    ShOvfl {
        /** Number of bytes for the relative shift */
        shift: i8,
    },

    /// Relative jump if `CK` is in a failed state.
    #[display("jif     CK, {shift:+}")]
    ShFail {
        /** Number of bytes for the relative shift */
        shift: i8,
    },

    /// External jump.
    #[display("jmp     {site}")]
    Exec {
        /** Target site to jump to */
        site: Site<Id>,
    },

    /// Subroutine call.
    #[display("call    {pos}")]
    Fn {
        /** Target position for the function jump */
        pos: u16,
    },

    /// External subroutine call.
    #[display("call    {site}")]
    Call {
        /** Target site */
        site: Site<Id>,
    },

    /// Return from a subroutine or finish the program.
    #[display("ret")]
    Ret,

    /// Stop the program.
    #[display("stop")]
    Stop,
}
