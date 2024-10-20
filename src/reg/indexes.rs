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

use amplify::num::error::OverflowError;
use amplify::num::{u3, u4, u5};

use crate::reg::Register;

/// All possible register indexes for `a` and `r` register sets
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display)]
#[repr(u8)]
#[derive(Default)]
pub enum Reg32 {
    /// Register with index `[0]`
    #[display("[0]")]
    #[default]
    Reg0 = 0,

    /// Register with index `[1]`
    #[display("[1]")]
    Reg1 = 1,

    /// Register with index `[2]`
    #[display("[2]")]
    Reg2 = 2,

    /// Register with index `[3]`
    #[display("[3]")]
    Reg3 = 3,

    /// Register with index `[4]`
    #[display("[4]")]
    Reg4 = 4,

    /// Register with index `[5]`
    #[display("[5]")]
    Reg5 = 5,

    /// Register with index `[6]`
    #[display("[6]")]
    Reg6 = 6,

    /// Register with index `[7]`
    #[display("[7]")]
    Reg7 = 7,

    /// Register with index `[8]`
    #[display("[8]")]
    Reg8 = 8,

    /// Register with index `[9]`
    #[display("[9]")]
    Reg9 = 9,

    /// Register with index `[10]`
    #[display("[10]")]
    Reg10 = 10,

    /// Register with index `[11]`
    #[display("[11]")]
    Reg11 = 11,

    /// Register with index `[12]`
    #[display("[12]")]
    Reg12 = 12,

    /// Register with index `[13]`
    #[display("[13]")]
    Reg13 = 13,

    /// Register with index `[14]`
    #[display("[14]")]
    Reg14 = 14,

    /// Register with index `[15]`
    #[display("[15]")]
    Reg15 = 15,

    /// Register with index `[16]`
    #[display("[16]")]
    Reg16 = 16,

    /// Register with index `[17]`
    #[display("[17]")]
    Reg17 = 17,

    /// Register with index `[18]`
    #[display("[18]")]
    Reg18 = 18,

    /// Register with index `[19]`
    #[display("[19]")]
    Reg19 = 19,

    /// Register with index `[20]`
    #[display("[20]")]
    Reg20 = 20,

    /// Register with index `[21]`
    #[display("[21]")]
    Reg21 = 21,

    /// Register with index `[22]`
    #[display("[22]")]
    Reg22 = 22,

    /// Register with index `[23]`
    #[display("[23]")]
    Reg23 = 23,

    /// Register with index `[24]`
    #[display("[24]")]
    Reg24 = 24,

    /// Register with index `[25]`
    #[display("[25]")]
    Reg25 = 25,

    /// Register with index `[26]`
    #[display("[26]")]
    Reg26 = 26,

    /// Register with index `[27]`
    #[display("[27]")]
    Reg27 = 27,

    /// Register with index `[28]`
    #[display("[28]")]
    Reg28 = 28,

    /// Register with index `[29]`
    #[display("[29]")]
    Reg29 = 29,

    /// Register with index `[30]`
    #[display("[30]")]
    Reg30 = 30,

    /// Register with index `[31]`
    #[display("[31]")]
    Reg31 = 31,
}

impl Reg32 {
    /// Constant enumerating all register indexes.
    pub const ALL: [Reg32; 32] = [
        Reg32::Reg0,
        Reg32::Reg1,
        Reg32::Reg2,
        Reg32::Reg3,
        Reg32::Reg4,
        Reg32::Reg5,
        Reg32::Reg6,
        Reg32::Reg7,
        Reg32::Reg8,
        Reg32::Reg9,
        Reg32::Reg10,
        Reg32::Reg11,
        Reg32::Reg12,
        Reg32::Reg13,
        Reg32::Reg14,
        Reg32::Reg15,
        Reg32::Reg16,
        Reg32::Reg17,
        Reg32::Reg18,
        Reg32::Reg19,
        Reg32::Reg20,
        Reg32::Reg21,
        Reg32::Reg22,
        Reg32::Reg23,
        Reg32::Reg24,
        Reg32::Reg25,
        Reg32::Reg26,
        Reg32::Reg27,
        Reg32::Reg28,
        Reg32::Reg29,
        Reg32::Reg30,
        Reg32::Reg31,
    ];

    /// Returns `usize` representation of the register index
    #[inline]
    pub fn to_usize(self) -> usize { self as u8 as usize }
}

impl From<&Reg32> for u5 {
    #[inline]
    fn from(reg32: &Reg32) -> Self { u5::with(*reg32 as u8) }
}

impl From<Reg32> for u5 {
    #[inline]
    fn from(reg32: Reg32) -> Self { u5::with(reg32 as u8) }
}

impl From<&Reg32> for u8 {
    #[inline]
    fn from(reg32: &Reg32) -> Self { *reg32 as u8 }
}

impl From<Reg32> for u8 {
    #[inline]
    fn from(reg32: Reg32) -> Self { reg32 as u8 }
}

impl From<&Reg32> for Reg32 {
    #[inline]
    fn from(reg32: &Reg32) -> Self { *reg32 }
}

impl From<u5> for Reg32 {
    fn from(val: u5) -> Self {
        match val {
            v if v == Reg32::Reg0.into() => Reg32::Reg0,
            v if v == Reg32::Reg1.into() => Reg32::Reg1,
            v if v == Reg32::Reg2.into() => Reg32::Reg2,
            v if v == Reg32::Reg3.into() => Reg32::Reg3,
            v if v == Reg32::Reg4.into() => Reg32::Reg4,
            v if v == Reg32::Reg5.into() => Reg32::Reg5,
            v if v == Reg32::Reg6.into() => Reg32::Reg6,
            v if v == Reg32::Reg7.into() => Reg32::Reg7,
            v if v == Reg32::Reg8.into() => Reg32::Reg8,
            v if v == Reg32::Reg9.into() => Reg32::Reg9,
            v if v == Reg32::Reg10.into() => Reg32::Reg10,
            v if v == Reg32::Reg11.into() => Reg32::Reg11,
            v if v == Reg32::Reg12.into() => Reg32::Reg12,
            v if v == Reg32::Reg13.into() => Reg32::Reg13,
            v if v == Reg32::Reg14.into() => Reg32::Reg14,
            v if v == Reg32::Reg15.into() => Reg32::Reg15,
            v if v == Reg32::Reg16.into() => Reg32::Reg16,
            v if v == Reg32::Reg17.into() => Reg32::Reg17,
            v if v == Reg32::Reg18.into() => Reg32::Reg18,
            v if v == Reg32::Reg19.into() => Reg32::Reg19,
            v if v == Reg32::Reg20.into() => Reg32::Reg20,
            v if v == Reg32::Reg21.into() => Reg32::Reg21,
            v if v == Reg32::Reg22.into() => Reg32::Reg22,
            v if v == Reg32::Reg23.into() => Reg32::Reg23,
            v if v == Reg32::Reg24.into() => Reg32::Reg24,
            v if v == Reg32::Reg25.into() => Reg32::Reg25,
            v if v == Reg32::Reg26.into() => Reg32::Reg26,
            v if v == Reg32::Reg27.into() => Reg32::Reg27,
            v if v == Reg32::Reg28.into() => Reg32::Reg28,
            v if v == Reg32::Reg29.into() => Reg32::Reg29,
            v if v == Reg32::Reg30.into() => Reg32::Reg30,
            v if v == Reg32::Reg31.into() => Reg32::Reg31,
            _ => unreachable!(),
        }
    }
}

/// Shorter version of possible register indexes for `a` and `r` register sets
/// covering initial 16 registers
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display)]
#[repr(u8)]
#[derive(Default)]
pub enum Reg16 {
    /// Register with index `[0]`
    #[display("[0]")]
    #[default]
    Reg0 = 0,

    /// Register with index `[1]`
    #[display("[1]")]
    Reg1 = 1,

    /// Register with index `[2]`
    #[display("[2]")]
    Reg2 = 2,

    /// Register with index `[3]`
    #[display("[3]")]
    Reg3 = 3,

    /// Register with index `[4]`
    #[display("[4]")]
    Reg4 = 4,

    /// Register with index `[5]`
    #[display("[5]")]
    Reg5 = 5,

    /// Register with index `[6]`
    #[display("[6]")]
    Reg6 = 6,

    /// Register with index `[7]`
    #[display("[7]")]
    Reg7 = 7,

    /// Register with index `[8]`
    #[display("[8]")]
    Reg8 = 8,

    /// Register with index `[9]`
    #[display("[9]")]
    Reg9 = 9,

    /// Register with index `[10]`
    #[display("[10]")]
    Reg10 = 10,

    /// Register with index `[11]`
    #[display("[11]")]
    Reg11 = 11,

    /// Register with index `[12]`
    #[display("[12]")]
    Reg12 = 12,

    /// Register with index `[13]`
    #[display("[13]")]
    Reg13 = 13,

    /// Register with index `[14]`
    #[display("[14]")]
    Reg14 = 14,

    /// Register with index `[15]`
    #[display("[15]")]
    Reg15 = 15,
}

impl Reg16 {
    /// Constant enumerating all register indexes.
    pub const ALL: [Reg16; 16] = [
        Reg16::Reg0,
        Reg16::Reg1,
        Reg16::Reg2,
        Reg16::Reg3,
        Reg16::Reg4,
        Reg16::Reg5,
        Reg16::Reg6,
        Reg16::Reg7,
        Reg16::Reg8,
        Reg16::Reg9,
        Reg16::Reg10,
        Reg16::Reg11,
        Reg16::Reg12,
        Reg16::Reg13,
        Reg16::Reg14,
        Reg16::Reg15,
    ];
}

impl From<&Reg16> for u4 {
    #[inline]
    fn from(reg16: &Reg16) -> Self { u4::with(*reg16 as u8) }
}

impl From<Reg16> for u4 {
    #[inline]
    fn from(reg16: Reg16) -> Self { u4::with(reg16 as u8) }
}

impl From<u4> for Reg16 {
    fn from(val: u4) -> Self {
        match val {
            v if v == Reg16::Reg0.into() => Reg16::Reg0,
            v if v == Reg16::Reg1.into() => Reg16::Reg1,
            v if v == Reg16::Reg2.into() => Reg16::Reg2,
            v if v == Reg16::Reg3.into() => Reg16::Reg3,
            v if v == Reg16::Reg4.into() => Reg16::Reg4,
            v if v == Reg16::Reg5.into() => Reg16::Reg5,
            v if v == Reg16::Reg6.into() => Reg16::Reg6,
            v if v == Reg16::Reg7.into() => Reg16::Reg7,
            v if v == Reg16::Reg8.into() => Reg16::Reg8,
            v if v == Reg16::Reg9.into() => Reg16::Reg9,
            v if v == Reg16::Reg10.into() => Reg16::Reg10,
            v if v == Reg16::Reg11.into() => Reg16::Reg11,
            v if v == Reg16::Reg12.into() => Reg16::Reg12,
            v if v == Reg16::Reg13.into() => Reg16::Reg13,
            v if v == Reg16::Reg14.into() => Reg16::Reg14,
            v if v == Reg16::Reg15.into() => Reg16::Reg15,
            _ => unreachable!(),
        }
    }
}

impl From<Reg16> for Reg32 {
    #[inline]
    fn from(reg16: Reg16) -> Self { u5::with(reg16 as u8).into() }
}

impl From<&Reg16> for Reg32 {
    #[inline]
    fn from(reg16: &Reg16) -> Self { u5::with(*reg16 as u8).into() }
}

impl TryFrom<Reg32> for Reg16 {
    type Error = OverflowError<u8>;

    fn try_from(value: Reg32) -> Result<Self, Self::Error> {
        u4::try_from(value as u8).map(Reg16::from)
    }
}

/// Short version of register indexes for `a` and `r` register sets covering
/// initial 8 registers only
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display)]
#[repr(u8)]
#[derive(Default)]
pub enum Reg8 {
    /// Register with index `[0]`
    #[display("[0]")]
    #[default]
    Reg0 = 0,

    /// Register with index `[1]`
    #[display("[1]")]
    Reg1 = 1,

    /// Register with index `[2]`
    #[display("[2]")]
    Reg2 = 2,

    /// Register with index `[3]`
    #[display("[3]")]
    Reg3 = 3,

    /// Register with index `[4]`
    #[display("[4]")]
    Reg4 = 4,

    /// Register with index `[5]`
    #[display("[5]")]
    Reg5 = 5,

    /// Register with index `[6]`
    #[display("[6]")]
    Reg6 = 6,

    /// Register with index `[7]`
    #[display("[7]")]
    Reg7 = 7,
}

impl Reg8 {
    /// Constant enumerating all register indexes.
    pub const ALL: [Reg8; 8] = [
        Reg8::Reg0,
        Reg8::Reg1,
        Reg8::Reg2,
        Reg8::Reg3,
        Reg8::Reg4,
        Reg8::Reg5,
        Reg8::Reg6,
        Reg8::Reg7,
    ];
}

impl From<&Reg8> for u3 {
    #[inline]
    fn from(reg8: &Reg8) -> Self { u3::with(*reg8 as u8) }
}

impl From<Reg8> for u3 {
    #[inline]
    fn from(reg8: Reg8) -> Self { u3::with(reg8 as u8) }
}

impl From<u3> for Reg8 {
    fn from(val: u3) -> Self {
        match val {
            v if v == Reg8::Reg0.into() => Reg8::Reg0,
            v if v == Reg8::Reg1.into() => Reg8::Reg1,
            v if v == Reg8::Reg2.into() => Reg8::Reg2,
            v if v == Reg8::Reg3.into() => Reg8::Reg3,
            v if v == Reg8::Reg4.into() => Reg8::Reg4,
            v if v == Reg8::Reg5.into() => Reg8::Reg5,
            v if v == Reg8::Reg6.into() => Reg8::Reg6,
            v if v == Reg8::Reg7.into() => Reg8::Reg7,
            _ => unreachable!(),
        }
    }
}

impl From<Reg8> for Reg32 {
    #[inline]
    fn from(reg8: Reg8) -> Self { u5::with(reg8 as u8).into() }
}

impl From<&Reg8> for Reg32 {
    #[inline]
    fn from(reg8: &Reg8) -> Self { u5::with(*reg8 as u8).into() }
}

impl TryFrom<Reg32> for Reg8 {
    type Error = OverflowError<u8>;

    fn try_from(value: Reg32) -> Result<Self, Self::Error> {
        u3::try_from(value as u8).map(Reg8::from)
    }
}

/// Possible index values for string registers (`S`-registers).
///
/// For `S`-registers it is possible to denote index as `u4` value, with the real index equal to
/// this value modulo 32. This is required because of the bit size parameters for the string
/// opcode arguments.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display, From)]
#[display("s16[{0}]")]
pub struct RegS(#[from] u4);

impl RegS {
    /// Returns `u8` value corresponding to the register number
    #[inline]
    pub fn as_u8(self) -> u8 { self.0.to_u8() }

    /// Returns `usize` value corresponding to the register number
    #[inline]
    pub fn as_usize(self) -> usize { self.0.to_u8() as usize }
}

impl Register for RegS {
    #[inline]
    fn description() -> &'static str { "4-bit S register index" }

    fn bytes(self) -> u16 { u16::MAX }
}

impl Default for RegS {
    #[inline]
    fn default() -> Self { RegS(u4::MIN) }
}

impl From<RegS> for u8 {
    #[inline]
    fn from(reg: RegS) -> Self { reg.0.to_u8() }
}

impl From<&RegS> for u8 {
    #[inline]
    fn from(reg: &RegS) -> Self { reg.0.to_u8() }
}

impl From<RegS> for usize {
    #[inline]
    fn from(reg: RegS) -> Self { reg.0.to_u8() as usize }
}

impl From<u8> for RegS {
    #[inline]
    fn from(val: u8) -> Self { RegS(u4::with(val % 16)) }
}

impl From<&u4> for RegS {
    #[inline]
    fn from(val: &u4) -> Self { RegS(*val) }
}

impl From<RegS> for u4 {
    #[inline]
    fn from(reg: RegS) -> Self { reg.0 }
}

impl From<&RegS> for u4 {
    #[inline]
    fn from(reg: &RegS) -> Self { reg.0 }
}

impl From<u5> for RegS {
    #[inline]
    fn from(val: u5) -> Self { RegS(u4::with(val.to_u8() % 16)) }
}

impl From<&u5> for RegS {
    #[inline]
    fn from(val: &u5) -> Self { RegS(u4::with(val.to_u8() % 16)) }
}

impl From<RegS> for u5 {
    #[inline]
    fn from(reg: RegS) -> Self { u5::with(reg.0.to_u8()) }
}

impl From<&RegS> for u5 {
    #[inline]
    fn from(reg: &RegS) -> Self { u5::with(reg.0.to_u8()) }
}

impl From<RegS> for Reg32 {
    fn from(reg: RegS) -> Self { u5::from(reg.0).into() }
}

impl TryFrom<Reg32> for RegS {
    type Error = OverflowError<u8>;

    fn try_from(value: Reg32) -> Result<Self, Self::Error> {
        u5::try_from(value as u8).map(RegS::from)
    }
}
