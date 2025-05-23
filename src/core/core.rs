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

use core::fmt::{self, Debug, Formatter};

use amplify::confinement::ConfinedVec;

use super::{Site, SiteId, Status};
use crate::{Register, LIB_NAME_ALUVM};

/// Maximal size of the call stack.
///
/// Equals to 0xFFFF (i.e., maximum limited by `cy` and `cp` bit size).
pub const CALL_STACK_SIZE_MAX: u16 = 0xFF;

/// Extension to the AluVM core provided by an ISA.
pub trait CoreExt: Clone + Debug {
    /// A type of registers provided by the ISA.
    type Reg: Register;
    /// A configuration used in initializing the core extension.
    type Config: Default;

    /// Constructs the core extensions to be added to AluVM core.
    fn with(config: Self::Config) -> Self;

    /// Read the value of a register.
    fn get(&self, reg: Self::Reg) -> Option<<Self::Reg as Register>::Value>;

    /// Clear the register by setting it to `None`.
    fn clr(&mut self, reg: Self::Reg);

    /// Set the register to the provided value.
    fn set(&mut self, reg: Self::Reg, val: <Self::Reg as Register>::Value) {
        self.put(reg, Some(val))
    }

    /// Put either a value or None to the register.
    fn put(&mut self, reg: Self::Reg, val: Option<<Self::Reg as Register>::Value>);

    /// Reset the core extension by setting all the registers to `None`.
    fn reset(&mut self);
}

/// A trait for the external part of AluVM core which can operate with core ISA extensions.
pub trait Supercore<Subcore> {
    /// An ISA extension subcore.
    fn subcore(&self) -> Subcore;

    /// Merge the values generated in the subcore ISA extension with the main core.
    fn merge_subcore(&mut self, subcore: Subcore);
}

/// Registers of a single CPU/VM core.
#[derive(Clone)]
pub struct Core<
    Id: SiteId,
    Cx: CoreExt,
    const CALL_STACK_SIZE: usize = { CALL_STACK_SIZE_MAX as usize },
> {
    /// Halt register. If set to `true`, halts program when `CK` is set to [`Status::Failed`] for
    /// the first time.
    ///
    /// # See also
    ///
    /// - [`Core::ck`] register
    /// - [`Core::cf`] register
    pub(super) ch: bool,

    /// Check register, which is set on any failure (accessing register in `None` state, zero
    /// division, etc.). Can be reset.
    ///
    /// # See also
    ///
    /// - [`Core::ch`] register
    /// - [`Core::cf`] register
    pub(super) ck: Status,

    /// Failure register, which counts how many times `CK` was set, and can't be reset.
    ///
    /// # See also
    ///
    /// - [`Core::ch`] register
    /// - [`Core::ck`] register
    pub(super) cf: u64,

    /// Test register, which acts as a boolean test result (also a carry flag).
    pub(super) co: Status,

    /// Counts number of jumps (possible cycles). The number of jumps is limited by 2^16 per
    /// script.
    pub(super) cy: u16,

    /// Complexity accumulator / counter.
    ///
    /// Each instruction has an associated computational complexity level. This register sums
    /// the complexity of executed instructions.
    ///
    /// # See also
    ///
    /// - [`Core::cy`] register
    /// - [`Core::cl`] register
    pub(super) ca: u64,

    /// Complexity limit.
    ///
    /// If this register has a value set, once [`Core::ca`] reaches this value, the VM will stop
    /// program execution setting `CK` to a failure.
    pub(super) cl: Option<u64>,

    /// Call stack.
    ///
    /// # See also
    ///
    /// - [`CALL_STACK_SIZE_MAX`] constant
    /// - [`Core::cp`] register
    pub(super) cs: ConfinedVec<Site<Id>, 0, CALL_STACK_SIZE>,

    /// Core extension module.
    pub cx: Cx,
}

/// Configuration for [`Core`] initialization.
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Debug)]
#[derive(StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_ALUVM)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CoreConfig {
    /// Initial value for the `CH` register.
    pub halt: bool,
    /// Initial value for the `CL` register.
    pub complexity_lim: Option<u64>,
}

impl Default for CoreConfig {
    /// Sets
    /// - [`CoreConfig::halt`] to `true`,
    /// - [`CoreConfig::complexity_lim`] to `None`
    ///
    /// # See also
    ///
    /// - [`CoreConfig::halt`]
    /// - [`CoreConfig::complexity_lim`]
    fn default() -> Self { CoreConfig { halt: true, complexity_lim: None } }
}

impl<Id: SiteId, Cx: CoreExt, const CALL_STACK_SIZE: usize> Default
    for Core<Id, Cx, CALL_STACK_SIZE>
{
    fn default() -> Self { Core::new() }
}

impl<Id: SiteId, Cx: CoreExt, const CALL_STACK_SIZE: usize> Core<Id, Cx, CALL_STACK_SIZE> {
    /// Initializes registers. Sets `CK` to `true`, counters to zero, call stack to empty and the
    /// rest of registers to `None` value.
    ///
    /// An alias for [`Core::with`]`(`[`CoreConfig::default()`]`, Cx::default())`.
    #[inline]
    pub fn new() -> Self {
        assert!(CALL_STACK_SIZE <= CALL_STACK_SIZE_MAX as usize, "Call stack size is too large");
        Core::with(default!(), default!())
    }

    /// Initializes registers using a configuration object [`CoreConfig`].
    pub fn with(config: CoreConfig, cx_config: Cx::Config) -> Self {
        assert!(CALL_STACK_SIZE <= CALL_STACK_SIZE_MAX as usize, "Call stack size is too large");
        Core {
            ch: config.halt,
            ck: Status::Ok,
            cf: 0,
            co: Status::Ok,
            cy: 0,
            ca: 0,
            cl: config.complexity_lim,
            cs: ConfinedVec::with_capacity(CALL_STACK_SIZE),
            cx: Cx::with(cx_config),
        }
    }

    /// Reset the core extension by setting all the registers to `None`.
    pub fn reset(&mut self) {
        let mut new = Self::new();
        new.ch = self.ch;
        new.cl = self.cl;
        new.cx.reset();
        *self = new;
    }
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl<Id: SiteId, Cx: CoreExt, const CALL_STACK_SIZE: usize> Debug
    for Core<Id, Cx, CALL_STACK_SIZE>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (sect, reg, val, reset) = if f.alternate() {
            ("\x1B[0;4;1m", "\x1B[0;1m", "\x1B[0;32m", "\x1B[0m")
        } else {
            ("", "", "", "")
        };

        writeln!(f, "{sect}C-regs:{reset}")?;
        write!(f, "{reg}CH{reset} {val}{}{reset}, ", self.ch)?;
        write!(f, "{reg}CK{reset} {val}{}{reset}, ", self.ck)?;
        write!(f, "{reg}CF{reset} {val}{}{reset}, ", self.cf)?;
        write!(f, "{reg}CO{reset} {val}{}{reset}, ", self.co)?;
        write!(f, "{reg}CY{reset} {val}{}{reset}, ", self.cy)?;
        write!(f, "{reg}CA{reset} {val}{}{reset}, ", self.ca)?;
        let cl = self
            .cl
            .map(|v| v.to_string())
            .unwrap_or_else(|| "~".to_string());
        write!(f, "{reg}CL{reset} {val}{cl}{reset}, ")?;
        write!(f, "{reg}CP{reset} {val}{}{reset}, ", self.cp())?;
        write!(f, "\n{reg}CS{reset} {val}{reset}")?;
        for item in &self.cs {
            write!(f, "{}   ", item)?;
        }
        writeln!(f)?;

        Debug::fmt(&self.cx, f)
    }
}

impl<Id: SiteId, Cx: CoreExt + Supercore<Cx2>, Cx2: CoreExt, const CALL_STACK_SIZE: usize>
    Supercore<Core<Id, Cx2, CALL_STACK_SIZE>> for Core<Id, Cx, CALL_STACK_SIZE>
{
    fn subcore(&self) -> Core<Id, Cx2, CALL_STACK_SIZE> {
        Core {
            ch: self.ch,
            ck: self.ck,
            cf: self.cf,
            co: self.co,
            cy: self.cy,
            ca: self.ca,
            cl: self.cl,
            cs: self.cs.clone(),
            cx: self.cx.subcore(),
        }
    }

    fn merge_subcore(&mut self, subcore: Core<Id, Cx2, CALL_STACK_SIZE>) {
        assert_eq!(self.ch, subcore.ch);
        self.ck = subcore.ck;
        self.co = subcore.co;
        self.cf = subcore.cf;
        self.cy = subcore.cy;
        self.ca = subcore.ca;
        assert_eq!(self.cl, subcore.cl);
        self.cs = subcore.cs;
        self.cx.merge_subcore(subcore.cx);
    }
}
