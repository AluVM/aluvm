// Reference rust implementation of AluVM (arithmetic logic unit virtual machine).
// To find more on AluVM please check <https://aluvm.org>
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2021-2024 by
//     Dr Maxim Orlovsky <orlovsky@ubideco.org>
//
// Copyright (C) 2021-2022 LNP/BP Standards Association. All rights reserved.
// Copyright (C) 2023-2024 UBIDECO Labs,
//     Institute for Distributed and Cognitive Computing, Switzerland.
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

use core::convert::TryFrom;

use amplify::num::{u1, u3, u4};

use crate::data as number;
use crate::reg::Register;

/// Common set of methods handled by different sets and families of VM registers
pub trait NumericRegister: Register {
    /// Register bit dimension
    #[inline]
    fn bits(&self) -> u16 { self.bytes() * 8 }

    /// Size of the register value in bytes
    fn bytes(&self) -> u16;

    /// Returns register layout
    fn layout(&self) -> number::Layout;
}

/// Enumeration of integer arithmetic registers (`A`-registers)
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display)]
#[repr(u8)]
#[derive(Default)]
pub enum RegA {
    /// 8-bit arithmetics register
    #[display("a8")]
    A8 = 0,

    /// 16-bit arithmetics register
    #[display("a16")]
    A16 = 1,

    /// 32-bit arithmetics register
    #[display("a32")]
    A32 = 2,

    /// 64-bit arithmetics register
    #[display("a64")]
    #[default]
    A64 = 3,

    /// 128-bit arithmetics register
    #[display("a128")]
    A128 = 4,

    /// 256-bit arithmetics register
    #[display("a256")]
    A256 = 5,

    /// 512-bit arithmetics register
    #[display("a512")]
    A512 = 6,

    /// 1024-bit arithmetics register
    #[display("a1024")]
    A1024 = 7,
}

impl Register for RegA {
    #[inline]
    fn description() -> &'static str { "A register" }
}

impl NumericRegister for RegA {
    #[inline]
    fn bytes(&self) -> u16 {
        match self {
            RegA::A8 => 1,
            RegA::A16 => 2,
            RegA::A32 => 4,
            RegA::A64 => 8,
            RegA::A128 => 16,
            RegA::A256 => 32,
            RegA::A512 => 64,
            RegA::A1024 => 128,
        }
    }

    #[inline]
    fn layout(&self) -> number::Layout { number::Layout::unsigned(self.bytes()) }
}

impl RegA {
    /// Set of all A registers
    pub const ALL: [RegA; 8] = [
        RegA::A8,
        RegA::A16,
        RegA::A32,
        RegA::A64,
        RegA::A128,
        RegA::A256,
        RegA::A512,
        RegA::A1024,
    ];

    /// Constructs [`RegA`] object for a provided requirement for register bit size
    pub fn with(bits: u16) -> Option<Self> {
        Some(match bits {
            8 => RegA::A8,
            16 => RegA::A16,
            32 => RegA::A32,
            64 => RegA::A64,
            128 => RegA::A128,
            256 => RegA::A256,
            512 => RegA::A512,
            1024 => RegA::A1024,
            _ => return None,
        })
    }

    /// Returns integer layout [`number::IntLayout`] specific for this register
    #[inline]
    pub fn int_layout(self) -> number::IntLayout { number::IntLayout::unsigned(self.bytes()) }
}

impl From<&RegA> for u3 {
    fn from(rega: &RegA) -> Self { u3::with(*rega as u8) }
}

impl From<RegA> for u3 {
    fn from(rega: RegA) -> Self { u3::with(rega as u8) }
}

impl From<u3> for RegA {
    fn from(val: u3) -> Self {
        match val {
            v if v == RegA::A8.into() => RegA::A8,
            v if v == RegA::A16.into() => RegA::A16,
            v if v == RegA::A32.into() => RegA::A32,
            v if v == RegA::A64.into() => RegA::A64,
            v if v == RegA::A128.into() => RegA::A128,
            v if v == RegA::A256.into() => RegA::A256,
            v if v == RegA::A512.into() => RegA::A512,
            v if v == RegA::A1024.into() => RegA::A1024,
            _ => unreachable!(),
        }
    }
}

impl From<RegA2> for RegA {
    #[inline]
    fn from(reg: RegA2) -> Self {
        match reg {
            RegA2::A8 => RegA::A8,
            RegA2::A16 => RegA::A16,
        }
    }
}

impl From<&RegA2> for RegA {
    #[inline]
    fn from(reg: &RegA2) -> Self {
        match reg {
            RegA2::A8 => RegA::A8,
            RegA2::A16 => RegA::A16,
        }
    }
}

impl TryFrom<RegAll> for RegA {
    type Error = ();

    #[inline]
    fn try_from(value: RegAll) -> Result<Self, Self::Error> { value.reg_a().ok_or(()) }
}

/// Enumeration of integer arithmetic registers suited for string addresses (`a8` and `a16`
/// registers)
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display)]
#[repr(u8)]
#[derive(Default)]
pub enum RegA2 {
    /// 8-bit arithmetics register
    #[display("a8")]
    #[default]
    A8 = 0,

    /// 16-bit arithmetics register
    #[display("a16")]
    A16 = 1,
}

impl Register for RegA2 {
    #[inline]
    fn description() -> &'static str { "A8 or A16 register" }
}

impl NumericRegister for RegA2 {
    #[inline]
    fn bytes(&self) -> u16 {
        match self {
            RegA2::A8 => 1,
            RegA2::A16 => 2,
        }
    }

    #[inline]
    fn layout(&self) -> number::Layout { number::Layout::unsigned(self.bytes()) }
}

impl RegA2 {
    /// Constructs [`RegA2`] object for a provided requirement for register bit size
    pub fn with(bits: u16) -> Option<Self> {
        Some(match bits {
            8 => RegA2::A8,
            16 => RegA2::A16,
            _ => return None,
        })
    }
}

impl From<&RegA2> for u1 {
    fn from(rega: &RegA2) -> Self { u1::with(*rega as u8) }
}

impl From<RegA2> for u1 {
    fn from(rega: RegA2) -> Self { u1::with(rega as u8) }
}

impl From<u1> for RegA2 {
    fn from(val: u1) -> Self {
        match val {
            v if v == RegA2::A8.into() => RegA2::A8,
            v if v == RegA2::A16.into() => RegA2::A16,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<RegAll> for RegA2 {
    type Error = ();

    #[inline]
    fn try_from(value: RegAll) -> Result<Self, Self::Error> {
        match value.reg_a() {
            Some(RegA::A8) => Ok(RegA2::A8),
            Some(RegA::A16) => Ok(RegA2::A16),
            _ => Err(()),
        }
    }
}

/// Enumeration of float arithmetic registers (`F`-registers)
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display)]
#[repr(u8)]
#[derive(Default)]
pub enum RegF {
    /// 16-bit bfloat16 format used in machine learning
    #[display("f16b")]
    F16B = 0,

    /// 16-bit IEEE-754 binary16 half-precision
    #[display("f16")]
    F16 = 1,

    /// 32-bit IEEE-754 binary32 single-precision
    #[display("f32")]
    F32 = 2,

    /// 64-bit IEEE-754 binary64 double-precision
    #[display("f64")]
    #[default]
    F64 = 3,

    /// 80-bit IEEE-754 extended precision
    #[display("f80")]
    F80 = 4,

    /// 128-bit IEEE-754 binary128 quadruple precision
    #[display("f128")]
    F128 = 5,

    /// 256-bit IEEE-754 binary256 octuple precision
    #[display("f256")]
    F256 = 6,

    /// 512-bit tapered floating point
    #[display("f512")]
    F512 = 7,
}

impl Register for RegF {
    #[inline]
    fn description() -> &'static str { "F register" }
}

impl NumericRegister for RegF {
    #[inline]
    fn bytes(&self) -> u16 {
        match self {
            RegF::F16B => 2,
            RegF::F16 => 2,
            RegF::F32 => 4,
            RegF::F64 => 8,
            RegF::F80 => 10,
            RegF::F128 => 16,
            RegF::F256 => 32,
            RegF::F512 => 64,
        }
    }

    #[inline]
    fn layout(&self) -> number::Layout {
        let fl = match self {
            RegF::F16B => number::FloatLayout::BFloat16,
            RegF::F16 => number::FloatLayout::IeeeHalf,
            RegF::F32 => number::FloatLayout::IeeeSingle,
            RegF::F64 => number::FloatLayout::IeeeDouble,
            RegF::F80 => number::FloatLayout::X87DoubleExt,
            RegF::F128 => number::FloatLayout::IeeeQuad,
            RegF::F256 => number::FloatLayout::IeeeOct,
            RegF::F512 => number::FloatLayout::FloatTapered,
        };
        number::Layout::float(fl)
    }
}

impl RegF {
    /// Set of all F registers
    pub const ALL: [RegF; 8] = [
        RegF::F16B,
        RegF::F16,
        RegF::F32,
        RegF::F64,
        RegF::F80,
        RegF::F128,
        RegF::F256,
        RegF::F512,
    ];

    /// Constructs [`RegF`] object for a provided requirement for register bit size
    pub fn with(bits: u16, use_bfloat16: bool) -> Option<Self> {
        Some(match bits {
            16 => {
                if use_bfloat16 {
                    RegF::F16B
                } else {
                    RegF::F16
                }
            }
            32 => RegF::F32,
            64 => RegF::F64,
            80 => RegF::F80,
            128 => RegF::F128,
            256 => RegF::F256,
            512 => RegF::F512,
            _ => return None,
        })
    }
}

impl From<&RegF> for u3 {
    fn from(regf: &RegF) -> Self { u3::with(*regf as u8) }
}

impl From<RegF> for u3 {
    fn from(regf: RegF) -> Self { u3::with(regf as u8) }
}

impl From<u3> for RegF {
    fn from(val: u3) -> Self {
        match val {
            v if v == RegF::F16B.into() => RegF::F16B,
            v if v == RegF::F16.into() => RegF::F16,
            v if v == RegF::F32.into() => RegF::F32,
            v if v == RegF::F64.into() => RegF::F64,
            v if v == RegF::F80.into() => RegF::F80,
            v if v == RegF::F128.into() => RegF::F128,
            v if v == RegF::F256.into() => RegF::F256,
            v if v == RegF::F512.into() => RegF::F512,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<RegAll> for RegF {
    type Error = ();

    #[inline]
    fn try_from(value: RegAll) -> Result<Self, Self::Error> { value.reg_f().ok_or(()) }
}

/// Enumeration of the set of general registers (`R`-registers: non-arithmetic registers, mostly
/// used for cryptography)
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display)]
#[repr(u8)]
#[derive(Default)]
pub enum RegR {
    /// 128-bit non-arithmetics register
    #[display("r128")]
    R128 = 0,

    /// 160-bit non-arithmetics register
    #[display("r160")]
    R160 = 1,

    /// 256-bit non-arithmetics register
    #[display("r256")]
    #[default]
    R256 = 2,

    /// 512-bit non-arithmetics register
    #[display("r512")]
    R512 = 3,

    /// 1024-bit non-arithmetics register
    #[display("r1024")]
    R1024 = 4,

    /// 2048-bit non-arithmetics register
    #[display("r2048")]
    R2048 = 5,

    /// 4096-bit non-arithmetics register
    #[display("r4096")]
    R4096 = 6,

    /// 8192-bit non-arithmetics register
    #[display("r8192")]
    R8192 = 7,
}

impl Register for RegR {
    #[inline]
    fn description() -> &'static str { "R register" }
}

impl NumericRegister for RegR {
    #[inline]
    fn bytes(&self) -> u16 {
        match self {
            RegR::R128 => 16,
            RegR::R160 => 20,
            RegR::R256 => 32,
            RegR::R512 => 64,
            RegR::R1024 => 128,
            RegR::R2048 => 256,
            RegR::R4096 => 512,
            RegR::R8192 => 1024,
        }
    }

    #[inline]
    fn layout(&self) -> number::Layout { number::Layout::unsigned(self.bytes()) }
}

impl RegR {
    /// Set of all R registers
    pub const ALL: [RegR; 8] = [
        RegR::R128,
        RegR::R160,
        RegR::R256,
        RegR::R512,
        RegR::R1024,
        RegR::R2048,
        RegR::R4096,
        RegR::R8192,
    ];

    /// Constructs [`RegR`] object for a provided requirement for register bit size
    #[inline]
    pub fn with(bits: u16) -> Option<Self> {
        Some(match bits {
            128 => RegR::R128,
            160 => RegR::R160,
            256 => RegR::R256,
            512 => RegR::R512,
            1024 => RegR::R1024,
            2048 => RegR::R2048,
            4096 => RegR::R4096,
            8192 => RegR::R8192,
            _ => return None,
        })
    }
}

impl From<&RegR> for u3 {
    fn from(regr: &RegR) -> Self { u3::with(*regr as u8) }
}

impl From<RegR> for u3 {
    fn from(regr: RegR) -> Self { u3::with(regr as u8) }
}

impl From<u3> for RegR {
    fn from(val: u3) -> Self {
        match val {
            v if v == RegR::R128.into() => RegR::R128,
            v if v == RegR::R160.into() => RegR::R160,
            v if v == RegR::R256.into() => RegR::R256,
            v if v == RegR::R512.into() => RegR::R512,
            v if v == RegR::R1024.into() => RegR::R1024,
            v if v == RegR::R2048.into() => RegR::R2048,
            v if v == RegR::R4096.into() => RegR::R4096,
            v if v == RegR::R8192.into() => RegR::R8192,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<RegAll> for RegR {
    type Error = ();

    #[inline]
    fn try_from(value: RegAll) -> Result<Self, Self::Error> { value.reg_r().ok_or(()) }
}

/// Superset of all registers accessible via instructions. The superset includes `A`, `F`, `R` and
/// `S` families of registers.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display, From)]
#[display(inner)]
pub enum RegAll {
    /// Arithmetic integer registers (`A` registers)
    #[from]
    A(RegA),

    /// Arithmetic float registers (`F` registers)
    #[from]
    F(RegF),

    /// Non-arithmetic (general) registers (`R` registers)
    #[from]
    R(RegR),

    /// String registers (`S` registers)
    S,
}

impl Default for RegAll {
    fn default() -> Self { RegAll::A(Default::default()) }
}

impl Register for RegAll {
    #[inline]
    fn description() -> &'static str { "A, F, R or S register" }
}

impl RegAll {
    /// Returns inner A-register type, if any
    #[inline]
    pub fn reg_a(self) -> Option<RegA> {
        match self {
            RegAll::A(a) => Some(a),
            _ => None,
        }
    }

    /// Returns inner F-register type, if any
    #[inline]
    pub fn reg_f(self) -> Option<RegF> {
        match self {
            RegAll::F(f) => Some(f),
            _ => None,
        }
    }

    /// Returns inner R-register type, if any
    #[inline]
    pub fn reg_r(self) -> Option<RegR> {
        match self {
            RegAll::R(r) => Some(r),
            _ => None,
        }
    }

    /// Returns string describing the family of the register
    #[inline]
    pub fn family_name(self) -> &'static str {
        match self {
            RegAll::A(_) => RegA::description(),
            RegAll::F(_) => RegF::description(),
            RegAll::R(_) => RegR::description(),
            RegAll::S => "S register",
        }
    }
}

impl From<&RegA> for RegAll {
    #[inline]
    fn from(reg: &RegA) -> Self { Self::A(*reg) }
}

impl From<&RegF> for RegAll {
    #[inline]
    fn from(reg: &RegF) -> Self { Self::F(*reg) }
}

impl From<&RegR> for RegAll {
    #[inline]
    fn from(reg: &RegR) -> Self { Self::R(*reg) }
}

impl From<RegA2> for RegAll {
    #[inline]
    fn from(reg: RegA2) -> Self { Self::A(reg.into()) }
}

impl From<&RegA2> for RegAll {
    #[inline]
    fn from(reg: &RegA2) -> Self { Self::A(reg.into()) }
}

impl From<RegAF> for RegAll {
    #[inline]
    fn from(reg: RegAF) -> Self {
        match reg {
            RegAF::A(a) => Self::A(a),
            RegAF::F(f) => Self::F(f),
        }
    }
}

impl From<&RegAF> for RegAll {
    #[inline]
    fn from(reg: &RegAF) -> Self {
        match reg {
            RegAF::A(a) => Self::A(*a),
            RegAF::F(f) => Self::F(*f),
        }
    }
}

impl From<RegAR> for RegAll {
    #[inline]
    fn from(reg: RegAR) -> Self {
        match reg {
            RegAR::A(a) => Self::A(a),
            RegAR::R(r) => Self::R(r),
        }
    }
}

impl From<&RegAR> for RegAll {
    #[inline]
    fn from(reg: &RegAR) -> Self {
        match reg {
            RegAR::A(a) => Self::A(*a),
            RegAR::R(r) => Self::R(*r),
        }
    }
}

/// Superset of all registers which value can be represented by a
/// [`crate::data::Number`]/[`crate::data::MaybeNumber`]. The superset includes `A`, `F`, and
/// `R` families of registers.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display, From)]
#[display(inner)]
pub enum RegAFR {
    /// Arithmetic integer registers (`A` registers)
    #[from]
    A(RegA),

    /// Arithmetic float registers (`F` registers)
    #[from]
    F(RegF),

    /// Non-arithmetic (general) registers (`R` registers)
    #[from]
    R(RegR),
}

impl Default for RegAFR {
    fn default() -> Self { RegAFR::A(Default::default()) }
}

impl Register for RegAFR {
    #[inline]
    fn description() -> &'static str { "A, F or R register" }
}

impl NumericRegister for RegAFR {
    #[inline]
    fn bytes(&self) -> u16 {
        match self {
            RegAFR::A(a) => a.bytes(),
            RegAFR::F(f) => f.bytes(),
            RegAFR::R(r) => r.bytes(),
        }
    }

    #[inline]
    fn layout(&self) -> number::Layout {
        match self {
            RegAFR::A(a) => a.layout(),
            RegAFR::F(f) => f.layout(),
            RegAFR::R(r) => r.layout(),
        }
    }
}

impl RegAFR {
    /// Returns inner A-register type, if any
    #[inline]
    pub fn reg_a(self) -> Option<RegA> {
        match self {
            RegAFR::A(a) => Some(a),
            _ => None,
        }
    }

    /// Returns inner F-register type, if any
    #[inline]
    pub fn reg_f(self) -> Option<RegF> {
        match self {
            RegAFR::F(f) => Some(f),
            _ => None,
        }
    }

    /// Returns inner R-register type, if any
    #[inline]
    pub fn reg_r(self) -> Option<RegR> {
        match self {
            RegAFR::R(r) => Some(r),
            _ => None,
        }
    }
}

impl From<&RegA> for RegAFR {
    #[inline]
    fn from(reg: &RegA) -> Self { Self::A(*reg) }
}

impl From<&RegF> for RegAFR {
    #[inline]
    fn from(reg: &RegF) -> Self { Self::F(*reg) }
}

impl From<&RegR> for RegAFR {
    #[inline]
    fn from(reg: &RegR) -> Self { Self::R(*reg) }
}

impl From<RegA2> for RegAFR {
    #[inline]
    fn from(reg: RegA2) -> Self { Self::A(reg.into()) }
}

impl From<&RegA2> for RegAFR {
    #[inline]
    fn from(reg: &RegA2) -> Self { Self::A(reg.into()) }
}

impl From<RegAF> for RegAFR {
    #[inline]
    fn from(reg: RegAF) -> Self {
        match reg {
            RegAF::A(a) => Self::A(a),
            RegAF::F(f) => Self::F(f),
        }
    }
}

impl From<&RegAF> for RegAFR {
    #[inline]
    fn from(reg: &RegAF) -> Self {
        match reg {
            RegAF::A(a) => Self::A(*a),
            RegAF::F(f) => Self::F(*f),
        }
    }
}

impl From<RegAR> for RegAFR {
    #[inline]
    fn from(reg: RegAR) -> Self {
        match reg {
            RegAR::A(a) => Self::A(a),
            RegAR::R(r) => Self::R(r),
        }
    }
}

impl From<&RegAR> for RegAFR {
    #[inline]
    fn from(reg: &RegAR) -> Self {
        match reg {
            RegAR::A(a) => Self::A(*a),
            RegAR::R(r) => Self::R(*r),
        }
    }
}

impl TryFrom<RegAll> for RegAFR {
    type Error = ();

    #[inline]
    fn try_from(value: RegAll) -> Result<Self, Self::Error> {
        match value {
            RegAll::A(a) => Ok(RegAFR::A(a)),
            RegAll::F(f) => Ok(RegAFR::F(f)),
            RegAll::R(r) => Ok(RegAFR::R(r)),
            _ => Err(()),
        }
    }
}

/// Superset of `A` and `F` arithmetic registers
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display, From)]
#[display(inner)]
pub enum RegAF {
    /// Arithmetic integer registers (`A` registers)
    #[from]
    A(RegA),

    /// Arithmetic float registers (`F` registers)
    #[from]
    F(RegF),
}

impl Default for RegAF {
    fn default() -> Self { RegAF::A(Default::default()) }
}

impl Register for RegAF {
    #[inline]
    fn description() -> &'static str { "A or F register" }
}

impl NumericRegister for RegAF {
    #[inline]
    fn bytes(&self) -> u16 {
        match self {
            RegAF::A(a) => a.bytes(),
            RegAF::F(f) => f.bytes(),
        }
    }

    #[inline]
    fn layout(&self) -> number::Layout {
        match self {
            RegAF::A(a) => a.layout(),
            RegAF::F(f) => f.layout(),
        }
    }
}

impl RegAF {
    /// Returns inner A-register type, if any
    #[inline]
    pub fn reg_a(self) -> Option<RegA> {
        match self {
            RegAF::A(a) => Some(a),
            RegAF::F(_) => None,
        }
    }

    /// Returns inner F-register type, if any
    #[inline]
    pub fn reg_f(self) -> Option<RegF> {
        match self {
            RegAF::A(_) => None,
            RegAF::F(f) => Some(f),
        }
    }
}

impl From<&RegAF> for u4 {
    fn from(reg: &RegAF) -> Self { u4::from(*reg) }
}

impl From<RegAF> for u4 {
    fn from(reg: RegAF) -> Self {
        match reg {
            RegAF::A(a) => u4::with(u3::from(a).to_u8()),
            RegAF::F(f) => u4::with(u3::from(f).to_u8() + 8),
        }
    }
}

impl From<u4> for RegAF {
    fn from(val: u4) -> Self {
        match val.to_u8() {
            0..=7 => RegAF::A(RegA::from(u3::with(val.to_u8()))),
            _ => RegAF::F(RegF::from(u3::with(val.to_u8() - 8))),
        }
    }
}

impl From<RegA2> for RegAF {
    #[inline]
    fn from(reg: RegA2) -> Self { Self::A(reg.into()) }
}

impl From<&RegA2> for RegAF {
    #[inline]
    fn from(reg: &RegA2) -> Self { Self::A(reg.into()) }
}

impl TryFrom<RegAll> for RegAF {
    type Error = ();

    #[inline]
    fn try_from(value: RegAll) -> Result<Self, Self::Error> {
        match value {
            RegAll::A(a) => Ok(RegAF::A(a)),
            RegAll::F(f) => Ok(RegAF::F(f)),
            _ => Err(()),
        }
    }
}

/// Superset of `A` and `R` registers
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display, From)]
#[display(inner)]
pub enum RegAR {
    /// Arithmetic integer registers (`A` registers)
    #[from]
    A(RegA),

    /// Non-arithmetic (general) registers (`R` registers)
    #[from]
    R(RegR),
}

impl Default for RegAR {
    fn default() -> Self { RegAR::A(Default::default()) }
}

impl Register for RegAR {
    #[inline]
    fn description() -> &'static str { "A or R register" }
}

impl NumericRegister for RegAR {
    #[inline]
    fn bytes(&self) -> u16 {
        match self {
            RegAR::A(a) => a.bytes(),
            RegAR::R(r) => r.bytes(),
        }
    }

    #[inline]
    fn layout(&self) -> number::Layout {
        match self {
            RegAR::A(a) => a.layout(),
            RegAR::R(r) => r.layout(),
        }
    }
}

impl RegAR {
    /// Constructs register superset from register block and family integer representation
    #[inline]
    pub fn from(block: u1, reg: u3) -> Self {
        match block.into_u8() {
            0 => RegAR::A(reg.into()),
            1 => RegAR::R(reg.into()),
            _ => unreachable!(),
        }
    }

    /// Returns inner A-register type, if any
    #[inline]
    pub fn reg_a(self) -> Option<RegA> {
        match self {
            RegAR::A(a) => Some(a),
            RegAR::R(_) => None,
        }
    }

    /// Returns inner R-register type, if any
    #[inline]
    pub fn reg_r(self) -> Option<RegR> {
        match self {
            RegAR::A(_) => None,
            RegAR::R(r) => Some(r),
        }
    }
}

impl From<&RegAR> for u4 {
    fn from(reg: &RegAR) -> Self { u4::from(*reg) }
}

impl From<RegAR> for u4 {
    fn from(reg: RegAR) -> Self {
        match reg {
            RegAR::A(a) => u4::with(u3::from(a).to_u8()),
            RegAR::R(r) => u4::with(u3::from(r).to_u8() + 8),
        }
    }
}

impl From<u4> for RegAR {
    fn from(val: u4) -> Self {
        match val.to_u8() {
            0..=7 => RegAR::A(RegA::from(u3::with(val.to_u8()))),
            _ => RegAR::R(RegR::from(u3::with(val.to_u8() - 8))),
        }
    }
}

impl From<RegA2> for RegAR {
    #[inline]
    fn from(reg: RegA2) -> Self { Self::A(reg.into()) }
}

impl From<&RegA2> for RegAR {
    #[inline]
    fn from(reg: &RegA2) -> Self { Self::A(reg.into()) }
}

impl TryFrom<RegAll> for RegAR {
    type Error = ();

    #[inline]
    fn try_from(value: RegAll) -> Result<Self, Self::Error> {
        match value {
            RegAll::A(a) => Ok(RegAR::A(a)),
            RegAll::R(r) => Ok(RegAR::R(r)),
            _ => Err(()),
        }
    }
}

/// Block of registers, either integer arithmetic or non-arithmetic (general) registers
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display)]
#[derive(Default)]
pub enum RegBlockAR {
    /// Arithmetic integer registers (`A` registers)
    #[display("a")]
    #[default]
    A,

    /// Non-arithmetic (generic) registers (`R` registers)
    #[display("r")]
    R,
}

impl Register for RegBlockAR {
    #[inline]
    fn description() -> &'static str { "A or R register block" }
}

impl RegBlockAR {
    /// Converts value into specific register matching the provided bit dimension. If the register
    /// with the given dimension does not exists, returns `None`.
    pub fn into_reg(self, bits: u16) -> Option<RegAR> {
        match self {
            RegBlockAR::A => RegA::with(bits).map(RegAR::A),
            RegBlockAR::R => RegR::with(bits).map(RegAR::R),
        }
    }
}

impl TryFrom<RegAll> for RegBlockAR {
    type Error = ();

    fn try_from(value: RegAll) -> Result<Self, Self::Error> {
        match value {
            RegAll::A(_) => Ok(RegBlockAR::A),
            RegAll::F(_) => Err(()),
            RegAll::R(_) => Ok(RegBlockAR::R),
            RegAll::S => Err(()),
        }
    }
}

/// Block of registers, either integer, float arithmetic or non-arithmetic (general) registers
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display)]
#[derive(Default)]
pub enum RegBlockAFR {
    /// Arithmetic integer registers (`A` registers)
    #[display("a")]
    #[default]
    A,

    /// Arithmetic float registers (`F` registers)
    #[display("f")]
    F,

    /// Non-arithmetic (generic) registers (`R` registers)
    #[display("r")]
    R,
}

impl Register for RegBlockAFR {
    #[inline]
    fn description() -> &'static str { "A, F or R register block" }
}

impl RegBlockAFR {
    /// Converts value into specific register matching the provided bit dimension. If the register
    /// with the given dimension does not exists, returns `None`.
    pub fn into_reg(self, bits: u16) -> Option<RegAFR> {
        match self {
            RegBlockAFR::A => RegA::with(bits).map(RegAFR::A),
            RegBlockAFR::F => RegF::with(bits, false).map(RegAFR::F),
            RegBlockAFR::R => RegR::with(bits).map(RegAFR::R),
        }
    }
}

impl TryFrom<RegAll> for RegBlockAFR {
    type Error = ();

    fn try_from(value: RegAll) -> Result<Self, Self::Error> {
        match value {
            RegAll::A(_) => Ok(RegBlockAFR::A),
            RegAll::F(_) => Ok(RegBlockAFR::F),
            RegAll::R(_) => Ok(RegBlockAFR::R),
            RegAll::S => Err(()),
        }
    }
}

/// Blocks of registers including all non-control register types
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display)]
#[derive(Default)]
pub enum RegBlock {
    /// Arithmetic integer registers (`A` registers)
    #[display("a")]
    #[default]
    A,

    /// Arithmetic float registers (`F` registers)
    #[display("f")]
    F,

    /// Non-arithmetic (generic) registers (`R` registers)
    #[display("r")]
    R,

    /// Byte-string registers (`S` registers)
    #[display("s")]
    S,
}

impl Register for RegBlock {
    #[inline]
    fn description() -> &'static str { "A, F, R or S register block" }
}

impl From<RegAll> for RegBlock {
    fn from(reg: RegAll) -> Self {
        match reg {
            RegAll::A(_) => RegBlock::A,
            RegAll::F(_) => RegBlock::F,
            RegAll::R(_) => RegBlock::R,
            RegAll::S => RegBlock::S,
        }
    }
}
