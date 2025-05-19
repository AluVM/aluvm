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

use core::fmt::Debug;

use strict_encoding::stl::AlphaCapsNum;
use strict_encoding::{RString, StrictDumb};

use super::CtrlInstr;
use crate::core::SiteId;
use crate::LIB_NAME_ALUVM;

/// Maximal length of the ISA identifier.
pub const ISA_ID_MAX_LEN: usize = 16;

/// Macro for constructing ISA identifiers
#[macro_export]
macro_rules! isa {
    ($id:literal) => {
        $crate::IsaId::from($id)
    };
    ($id:ident) => {
        $crate::IsaId::from($id)
    };
}

/// ISA identifier.
///
/// ISA identifier is a capitalized ASCII alphanumeric string, consisting of minimum one character
/// and with a maximal length up to [`ISA_ID_MAX_LEN`].
#[derive(Wrapper, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, From)]
#[wrapper(Deref, Display, FromStr)]
#[derive(StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_ALUVM)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
pub struct IsaId(RString<AlphaCapsNum, AlphaCapsNum, 1, ISA_ID_MAX_LEN>);

impl StrictDumb for IsaId {
    fn strict_dumb() -> Self { Self::from("DUMB") }
}

impl From<&'static str> for IsaId {
    fn from(id: &'static str) -> Self { Self(RString::from(id)) }
}

/// Reserved instruction, which equal to [`crate::ExecStep::Fail`].
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display)]
#[display("halt    {0:#02X}.h")]
pub struct ReservedInstr(/** Reserved instruction op code value */ pub(super) u8);

impl Default for ReservedInstr {
    fn default() -> Self { Self(0xFF) }
}

/// Complete AluVM ISA.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Display, From)]
#[display(inner)]
pub enum Instr<Id: SiteId> {
    /// Control flow instructions.
    #[from]
    Ctrl(CtrlInstr<Id>),

    // #[cfg(feature = "str")]
    // Str(array::instr::StrInstr),
    /// Reserved instruction for future use in core `ALU` ISAs.
    #[from]
    Reserved(ReservedInstr),
}
