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

use core::cmp::Ordering;
use core::fmt::{self, Debug, Display, Formatter};
use core::ops::Not;
use core::str::FromStr;

use crate::core::CoreExt;

/// A trait for a set of registers provided by an ISA extension.
pub trait Register: Copy + Ord + Debug + Display {
    /// The value type contained in the registers.
    type Value: Copy + Debug + Display;

    /// The size of the value in the register, in bytes.
    fn bytes(self) -> u16;
}

/// Default [`Register`] implementation for ISA extensions providing no new registers.
#[derive(Debug)]
pub enum NoRegs {}
#[allow(clippy::non_canonical_clone_impl)]
impl Clone for NoRegs {
    fn clone(&self) -> Self { unreachable!() }
}
impl Copy for NoRegs {}
#[allow(clippy::non_canonical_clone_impl)]
impl PartialEq for NoRegs {
    fn eq(&self, _: &Self) -> bool { unreachable!() }
}
impl Eq for NoRegs {}
impl Ord for NoRegs {
    fn cmp(&self, _: &Self) -> Ordering { unreachable!() }
}
#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for NoRegs {
    fn partial_cmp(&self, _: &Self) -> Option<Ordering> { unreachable!() }
}
impl Display for NoRegs {
    fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result { unreachable!() }
}
impl Register for NoRegs {
    type Value = u8;
    fn bytes(self) -> u16 { unreachable!() }
}

/// Status for flag registers.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Display)]
#[repr(i8)]
pub enum Status {
    /// Flag is not set, indicating absence of failures.
    #[display("ok")]
    Ok = 0,

    /// Flag is set, indicating a failure.
    #[display("fail")]
    Fail = -1,
}

impl Status {
    /// Checks if the flag is not set, and no failure has happened.
    pub fn is_ok(self) -> bool { self == Status::Ok }
}

impl Not for Status {
    type Output = Status;

    fn not(self) -> Self::Output {
        match self {
            Status::Ok => Status::Fail,
            Status::Fail => Status::Ok,
        }
    }
}

/// Trait for program identifiers.
///
/// This type is required in addition to [`crate::LibId`] in order to achieve proper abstraction,
/// layering, and separation of concerns: the core must know nothing about library structure.
pub trait SiteId: Copy + Ord + Debug + Display + FromStr {}

/// Location inside the instruction sequence which can be executed by the core.
///
/// This type is required in addition to [`crate::LibSite`] in order to achieve proper abstraction,
/// layering, and separation of concerns: the core must know nothing about library structure.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Site<Id: SiteId> {
    /// Identifier of the program.
    pub prog_id: Id,
    /// Offset in the code segment within the program.
    pub offset: u16,
}

impl<Id: SiteId> Site<Id> {
    /// Construct a new code site out of program identifier and code offset.
    #[inline]
    pub fn new(prog_id: Id, offset: u16) -> Self { Self { prog_id, offset } }
}

impl<Id: SiteId> Display for Site<Id> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{:04}", self.prog_id, self.offset)
    }
}

/// Helper data structure for base core which has no ISA extensions.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct NoExt;

impl CoreExt for NoExt {
    type Reg = NoRegs;
    type Config = ();

    fn with(_config: Self::Config) -> Self { NoExt }

    fn get(&self, _reg: Self::Reg) -> Option<u8> { unreachable!() }

    fn clr(&mut self, _reg: Self::Reg) { unreachable!() }

    fn put(&mut self, _reg: Self::Reg, _val: Option<u8>) { unreachable!() }

    fn reset(&mut self) {}
}
