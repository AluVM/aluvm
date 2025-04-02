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

//! Flags used by operation codes

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::borrow::ToOwned;
#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::string::String;
use core::fmt::{self, Display, Formatter, Write};
use core::str::FromStr;

use amplify::num::apfloat::Round;
use amplify::num::{u1, u2, u3};

/// Marker trait for flag types
pub trait Flag: FromStr<Err = ParseFlagError> + Default {}

/// Errors for parsing string representation for a flag values
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display)]
#[cfg_attr(feature = "std", derive(Error))]
#[display(doc_comments)]
pub enum ParseFlagError {
    /// unknown `{0}` flag `{1}`
    UnknownFlag(/** Flag description */ &'static str, /** Unrecognized flag */ char),

    /// unknown `{0}` flags `{1}`
    UnknownFlags(/** Flag description */ &'static str, /** Unrecognized flags */ String),

    /// only one of mutually exclusive flags must be specified for {0} (only `{1}` or `{2}`)
    MutuallyExclusiveFlags(
        /** Flag description */ &'static str,
        /** Flag 1 */ char,
        /** Flag 2 */ char,
    ),

    /// required flag for {0} is absent
    RequiredFlagAbsent(/** Flag description */ &'static str),

    /// duplicated flags `{1}` are specified for {0}
    DuplicatedFlags(/** Flag description */ &'static str, /** List of duplicated flags */ String),
}

/// Integer encoding flag
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display)]
pub enum SignFlag {
    /// Unsigned integer
    #[display("u")]
    Unsigned = 0,

    /// Signed integer
    #[display("s")]
    Signed = 1,
}

impl Flag for SignFlag {}

impl Default for SignFlag {
    #[inline]
    fn default() -> Self { Self::Unsigned }
}

impl FromStr for SignFlag {
    type Err = ParseFlagError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseFlagError::RequiredFlagAbsent("integer sign"));
        }
        let filtered = s.replace(&['u', 's'][..], "");
        if !filtered.is_empty() {
            return Err(ParseFlagError::UnknownFlags("integer sign", filtered));
        }
        match (s.contains('u'), s.contains('s')) {
            (true, false) => Ok(SignFlag::Unsigned),
            (false, true) => Ok(SignFlag::Signed),
            (true, true) => Err(ParseFlagError::MutuallyExclusiveFlags("integer sign", 'u', 's')),
            (false, false) => Err(ParseFlagError::RequiredFlagAbsent("integer sign")),
        }
    }
}

impl SignFlag {
    /// Constructs integer sign flag from `u1` value (used in bytecode serialization)
    pub fn from_u1(val: u1) -> SignFlag {
        match val.into_u8() {
            v if v == SignFlag::Unsigned as u8 => SignFlag::Unsigned,
            v if v == SignFlag::Signed as u8 => SignFlag::Signed,
            _ => unreachable!(),
        }
    }

    /// Returns `u1` representation of integer sign flag (used in bytecode serialization).
    pub fn as_u1(self) -> u1 { u1::with(self as u8) }
}

impl From<u1> for SignFlag {
    fn from(val: u1) -> SignFlag { SignFlag::from_u1(val) }
}

impl From<&SignFlag> for u1 {
    fn from(flag: &SignFlag) -> u1 { flag.as_u1() }
}

impl From<SignFlag> for u1 {
    fn from(flag: SignFlag) -> u1 { flag.as_u1() }
}

impl From<SignFlag> for bool {
    fn from(flag: SignFlag) -> Self { flag == SignFlag::Signed }
}

impl From<&SignFlag> for bool {
    fn from(flag: &SignFlag) -> Self { *flag == SignFlag::Signed }
}

/// Non-equality flag
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display)]
pub enum NoneEqFlag {
    /// Two `None` register values are considered equal
    #[display("e")]
    Equal = 1,

    /// Two `None` register values are considered non-equal
    #[display("n")]
    NonEqual = 0,
}

impl Flag for NoneEqFlag {}

impl Default for NoneEqFlag {
    #[inline]
    fn default() -> Self { Self::Equal }
}

impl FromStr for NoneEqFlag {
    type Err = ParseFlagError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseFlagError::RequiredFlagAbsent("none-equality"));
        }
        let filtered = s.replace(&['e', 'n'][..], "");
        if !filtered.is_empty() {
            return Err(ParseFlagError::UnknownFlags("none-equality", filtered));
        }
        match (s.contains('e'), s.contains('n')) {
            (true, false) => Ok(NoneEqFlag::Equal),
            (false, true) => Ok(NoneEqFlag::NonEqual),
            (true, true) => Err(ParseFlagError::MutuallyExclusiveFlags("none-equality", 'e', 'n')),
            (false, false) => Err(ParseFlagError::RequiredFlagAbsent("none-equality")),
        }
    }
}

impl NoneEqFlag {
    /// Constructs none-equality flag from `u1` value (used in bytecode serialization)
    pub fn from_u1(val: u1) -> NoneEqFlag {
        match val.into_u8() {
            v if v == NoneEqFlag::Equal as u8 => NoneEqFlag::Equal,
            v if v == NoneEqFlag::NonEqual as u8 => NoneEqFlag::NonEqual,
            _ => unreachable!(),
        }
    }

    /// Returns `u1` representation of none-equality flag (used in bytecode serialization).
    pub fn as_u1(self) -> u1 { u1::with(self as u8) }
}

impl From<u1> for NoneEqFlag {
    fn from(val: u1) -> NoneEqFlag { NoneEqFlag::from_u1(val) }
}

impl From<&NoneEqFlag> for u1 {
    fn from(flag: &NoneEqFlag) -> u1 { flag.as_u1() }
}

impl From<NoneEqFlag> for u1 {
    fn from(flag: NoneEqFlag) -> u1 { flag.as_u1() }
}

impl From<NoneEqFlag> for bool {
    fn from(flag: NoneEqFlag) -> Self { flag == NoneEqFlag::Equal }
}

impl From<&NoneEqFlag> for bool {
    fn from(flag: &NoneEqFlag) -> Self { *flag == NoneEqFlag::Equal }
}

/// Float equality flag
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display)]
pub enum FloatEqFlag {
    /// Use exact match, when nearest floats are always non-equal.
    ///
    /// NB: This still implies `+0` == `-0`.
    #[display("e")]
    Exact = 0,

    /// Use rounded matching, when floats which differ only on a single bit in significand are
    /// still treated as euqal.
    #[display("r")]
    Rounding = 1,
}

impl Flag for FloatEqFlag {}

impl Default for FloatEqFlag {
    #[inline]
    fn default() -> Self { Self::Exact }
}

impl FromStr for FloatEqFlag {
    type Err = ParseFlagError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseFlagError::RequiredFlagAbsent("float equality"));
        }
        let filtered = s.replace(&['e', 'r'][..], "");
        if !filtered.is_empty() {
            return Err(ParseFlagError::UnknownFlags("float equality", filtered));
        }
        match (s.contains('e'), s.contains('r')) {
            (true, false) => Ok(FloatEqFlag::Exact),
            (false, true) => Ok(FloatEqFlag::Rounding),
            (true, true) => Err(ParseFlagError::MutuallyExclusiveFlags("float equality", 'e', 'r')),
            (false, false) => Err(ParseFlagError::RequiredFlagAbsent("float equality")),
        }
    }
}

impl FloatEqFlag {
    /// Constructs float equality flag from `u1` value (used in bytecode serialization)
    pub fn from_u1(val: u1) -> FloatEqFlag {
        match val.into_u8() {
            v if v == FloatEqFlag::Exact as u8 => FloatEqFlag::Exact,
            v if v == FloatEqFlag::Rounding as u8 => FloatEqFlag::Rounding,
            _ => unreachable!(),
        }
    }

    /// Returns `u1` representation of float equality flag (used in bytecode serialization).
    pub fn as_u1(self) -> u1 { u1::with(self as u8) }
}

impl From<u1> for FloatEqFlag {
    fn from(val: u1) -> FloatEqFlag { FloatEqFlag::from_u1(val) }
}

impl From<&FloatEqFlag> for u1 {
    fn from(flag: &FloatEqFlag) -> u1 { flag.as_u1() }
}

impl From<FloatEqFlag> for u1 {
    fn from(flag: FloatEqFlag) -> u1 { flag.as_u1() }
}

/// Rounding flags for float numbers
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display)]
pub enum RoundingFlag {
    /// Round always toward zero, which means ceiling for negative numbers and flooring for
    /// positive numbers.
    #[display("z")]
    TowardsZero = 0,

    /// Round to the nearest neighbour, and if the number is exactly in the middle, ties round to
    /// the nearest even digit in the required position.
    #[display("n")]
    TowardsNearest = 1,

    /// Round down (flooring), ie toward -∞; negative results thus round away from zero.
    #[display("f")]
    Floor = 2,

    /// Round up (ceiling), ie toward +∞; negative results thus round toward zero.
    #[display("c")]
    Ceil = 3,
}

impl Flag for RoundingFlag {}

impl Default for RoundingFlag {
    #[inline]
    fn default() -> Self { Self::TowardsNearest }
}

impl FromStr for RoundingFlag {
    type Err = ParseFlagError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseFlagError::RequiredFlagAbsent("float rounding"));
        }

        let filtered = s.replace(&['n', 'z', 'c', 'f'][..], "");
        if !filtered.is_empty() {
            return Err(ParseFlagError::UnknownFlags("float rounding", filtered));
        }
        if s.len() > 1 {
            return Err(ParseFlagError::MutuallyExclusiveFlags(
                "float rounding",
                s.as_bytes()[0].into(),
                s.as_bytes()[1].into(),
            ));
        }

        if s.contains('n') {
            Ok(RoundingFlag::TowardsNearest)
        } else if s.contains('z') {
            Ok(RoundingFlag::TowardsZero)
        } else if s.contains('c') {
            Ok(RoundingFlag::Ceil)
        } else if s.contains('f') {
            Ok(RoundingFlag::Floor)
        } else {
            Err(ParseFlagError::UnknownFlag("float rounding", s.as_bytes()[0].into()))
        }
    }
}

impl RoundingFlag {
    /// Constructs float rounding flag from `u2` value (used in bytecode serialization)
    pub fn from_u2(val: u2) -> Self {
        match val.to_u8() {
            v if v == RoundingFlag::TowardsZero as u8 => RoundingFlag::TowardsZero,
            v if v == RoundingFlag::TowardsNearest as u8 => RoundingFlag::TowardsNearest,
            v if v == RoundingFlag::Ceil as u8 => RoundingFlag::Ceil,
            v if v == RoundingFlag::Floor as u8 => RoundingFlag::Floor,
            _ => unreachable!(),
        }
    }

    /// Returns `u2` representation of float rounding flag (used in bytecode serialization).
    pub fn as_u2(self) -> u2 { u2::with(self as u8) }
}

impl From<u2> for RoundingFlag {
    fn from(val: u2) -> RoundingFlag { RoundingFlag::from_u2(val) }
}

impl From<&RoundingFlag> for u2 {
    fn from(flag: &RoundingFlag) -> u2 { flag.as_u2() }
}

impl From<RoundingFlag> for u2 {
    fn from(flag: RoundingFlag) -> u2 { flag.as_u2() }
}

impl From<RoundingFlag> for Round {
    fn from(flag: RoundingFlag) -> Self {
        match flag {
            RoundingFlag::TowardsZero => Round::TowardZero,
            RoundingFlag::TowardsNearest => Round::NearestTiesToEven,
            RoundingFlag::Floor => Round::TowardNegative,
            RoundingFlag::Ceil => Round::TowardPositive,
        }
    }
}

/// Encoding and overflowing flags for integer numbers
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct IntFlags {
    /// Treat the integer as signed (`true`) or unsigned (`false`). Signed integers has a different
    /// behaviour on detecting overflows, since they use only 7 bits for significant digits and not
    /// 8.
    pub signed: bool,

    /// With addition / subtraction / multiplication, indicates whether overflow must result in
    /// modulo-based wrapping (`true`) or set the destination into `None` state (`false`).
    /// With division, `true` means that Euclidean division should be performed.
    pub wrap: bool,
}

impl Flag for IntFlags {}

impl Display for IntFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.signed {
            f.write_char('s')?;
        } else {
            f.write_char('u')?;
        }
        if self.wrap {
            f.write_char('w')
        } else {
            f.write_char('c')
        }
    }
}

impl FromStr for IntFlags {
    type Err = ParseFlagError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let signed = match (s.contains('s'), s.contains('u')) {
            (true, false) => true,
            (false, true) => false,
            (true, true) => {
                return Err(ParseFlagError::MutuallyExclusiveFlags(
                    "integer serialization",
                    's',
                    'u',
                ));
            }
            (false, false) => {
                return Err(ParseFlagError::RequiredFlagAbsent("integer serialization"));
            }
        };
        let wrap = match (s.contains('w'), s.contains('c')) {
            (true, false) => true,
            (false, true) => false,
            (true, true) => {
                return Err(ParseFlagError::MutuallyExclusiveFlags("overflow", 'w', 'c'));
            }
            (false, false) => return Err(ParseFlagError::RequiredFlagAbsent("overflow")),
        };
        if s.len() > 2 {
            return Err(ParseFlagError::UnknownFlags(
                "integer serialization",
                s.replace(&['s', 'u', 'c', 'w'][..], ""),
            ));
        }

        Ok(IntFlags { signed, wrap })
    }
}

impl IntFlags {
    /// Constructs integer arithmetic flags from `u2` value (used in bytecode serialization)
    pub fn from_u2(val: u2) -> Self {
        let val = val.to_u8();
        IntFlags {
            signed: val & 0x01 == 1,
            wrap: val & (0x02 >> 1) == 1,
        }
    }

    /// Returns `u2` representation of integer arithmetic flags (used in bytecode serialization).
    pub fn as_u2(self) -> u2 { u2::with(self.signed as u8 | ((self.wrap as u8) << 1)) }

    /// Constructs variant for unsigned checked operation flags
    #[inline]
    pub fn unsigned_checked() -> Self {
        IntFlags {
            signed: false,
            wrap: false,
        }
    }

    /// Constructs variant for signed checked operation flags
    #[inline]
    pub fn signed_checked() -> Self {
        IntFlags {
            signed: true,
            wrap: false,
        }
    }

    /// Constructs variant for unsigned wrapped operation flags
    #[inline]
    pub fn unsigned_wrapped() -> Self {
        IntFlags {
            signed: false,
            wrap: true,
        }
    }

    /// Constructs variant for signed wrapped operation flags
    #[inline]
    pub fn signed_wrapped() -> Self {
        IntFlags {
            signed: true,
            wrap: true,
        }
    }
}

impl From<u2> for IntFlags {
    fn from(val: u2) -> IntFlags { IntFlags::from_u2(val) }
}

impl From<&IntFlags> for u2 {
    fn from(flag: &IntFlags) -> u2 { flag.as_u2() }
}

impl From<IntFlags> for u2 {
    fn from(flag: IntFlags) -> u2 { flag.as_u2() }
}

/// Merge flags for operations which need to add certain bit value to the register existing value
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display)]
pub enum MergeFlag {
    /// Assign the bit value to the register clearing its previous content
    #[display("s")]
    Set = 0,

    /// Add the bit value to the register value, treating existing register value as an unsigned
    /// value. If the addition leads to an overflow, set `st0` register to `false` and keep the
    /// register value at the maximum ("saturating" addition). Otherwise, do not modify `st0`
    /// value.
    #[display("a")]
    Add = 1,

    /// Bit-and the bit and the lowest bit value from the register.
    #[display("n")]
    And = 2,

    /// Bit-or the bit and the lowest bit value from the register.
    #[display("o")]
    Or = 3,
}

impl Flag for MergeFlag {}

impl Default for MergeFlag {
    #[inline]
    fn default() -> Self { Self::Set }
}

impl FromStr for MergeFlag {
    type Err = ParseFlagError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseFlagError::RequiredFlagAbsent("merge operation"));
        }

        let filtered = s.replace(&['s', 'a', 'n', 'o'][..], "");
        if !filtered.is_empty() {
            return Err(ParseFlagError::UnknownFlags("merge operation", filtered));
        }
        if s.len() > 1 {
            return Err(ParseFlagError::MutuallyExclusiveFlags(
                "merge",
                s.as_bytes()[0].into(),
                s.as_bytes()[1].into(),
            ));
        }

        if s.contains('s') {
            Ok(MergeFlag::Set)
        } else if s.contains('a') {
            Ok(MergeFlag::Add)
        } else if s.contains('n') {
            Ok(MergeFlag::And)
        } else if s.contains('o') {
            Ok(MergeFlag::Or)
        } else {
            Err(ParseFlagError::UnknownFlag("merge operation", s.as_bytes()[0].into()))
        }
    }
}

impl MergeFlag {
    /// Constructs merge operation flag from `u2` value (used in bytecode serialization)
    pub fn from_u2(val: u2) -> Self {
        match val.to_u8() {
            v if v == MergeFlag::Set as u8 => MergeFlag::Set,
            v if v == MergeFlag::Add as u8 => MergeFlag::Add,
            v if v == MergeFlag::And as u8 => MergeFlag::And,
            v if v == MergeFlag::Or as u8 => MergeFlag::Or,
            _ => unreachable!(),
        }
    }

    /// Returns `u2` representation of merge operation flag (used in bytecode serialization).
    pub fn as_u2(self) -> u2 { u2::with(self as u8) }
}

impl From<u2> for MergeFlag {
    fn from(val: u2) -> MergeFlag { MergeFlag::from_u2(val) }
}

impl From<&MergeFlag> for u2 {
    fn from(flag: &MergeFlag) -> u2 { flag.as_u2() }
}

impl From<MergeFlag> for u2 {
    fn from(flag: MergeFlag) -> u2 { flag.as_u2() }
}

/// Flag for bytestring operations indicating whether the string should be extended to a new length
/// or the operation should fail (for instance, see `fill` operation).
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display)]
pub enum ExtendFlag {
    /// Unsigned integer
    #[display("e")]
    Extend = 0,

    /// Signed integer
    #[display("f")]
    Fail = 1,
}

impl Flag for ExtendFlag {}

impl Default for ExtendFlag {
    #[inline]
    fn default() -> Self { Self::Extend }
}

impl FromStr for ExtendFlag {
    type Err = ParseFlagError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseFlagError::RequiredFlagAbsent("extension flag"));
        }
        let filtered = s.replace(&['e', 'f'][..], "");
        if !filtered.is_empty() {
            return Err(ParseFlagError::UnknownFlags("extension flag", filtered));
        }
        match (s.contains('e'), s.contains('f')) {
            (true, false) => Ok(ExtendFlag::Extend),
            (false, true) => Ok(ExtendFlag::Fail),
            (true, true) => Err(ParseFlagError::MutuallyExclusiveFlags("extension flag", 'e', 'f')),
            (false, false) => Err(ParseFlagError::RequiredFlagAbsent("extension flag")),
        }
    }
}

impl ExtendFlag {
    /// Constructs extension flag from `u1` value (used in bytecode serialization)
    pub fn from_u1(val: u1) -> ExtendFlag {
        match val.into_u8() {
            v if v == ExtendFlag::Extend as u8 => ExtendFlag::Extend,
            v if v == ExtendFlag::Fail as u8 => ExtendFlag::Fail,
            _ => unreachable!(),
        }
    }

    /// Returns `u1` representation of extension flag (used in bytecode serialization).
    pub fn as_u1(self) -> u1 { u1::with(self as u8) }
}

impl From<u1> for ExtendFlag {
    fn from(val: u1) -> ExtendFlag { ExtendFlag::from_u1(val) }
}

impl From<&ExtendFlag> for u1 {
    fn from(flag: &ExtendFlag) -> u1 { flag.as_u1() }
}

impl From<ExtendFlag> for u1 {
    fn from(flag: ExtendFlag) -> u1 { flag.as_u1() }
}

impl From<ExtendFlag> for bool {
    fn from(flag: ExtendFlag) -> Self { flag == ExtendFlag::Fail }
}

impl From<&ExtendFlag> for bool {
    fn from(flag: &ExtendFlag) -> Self { *flag == ExtendFlag::Fail }
}

/// Flags for bytestring split operation.
///
/// If offset exceeds the length of the string in the register, than the behaviour of
/// [`crate::isa::BytesOp::Splt`] op code is defined by this flag. Please check its description
/// for more details.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display)]
pub enum SplitFlag {
    /// If the offset is equal to zero, exceeds or equal to the length of the source string sets
    /// first and second destination register to `None`; `st0` to `false`.
    ///
    /// Matches case (1) in [`crate::isa::BytesOp::Splt`] description
    #[display("n")]
    NoneNone = 0,

    /// If the offset is equal to zero, sets first destination register to `None`, second is set to
    /// `None` only if the string in the source register is empty; `st0` in both cases is set
    /// to `false`.
    ///
    /// Matches case (2) in [`crate::isa::BytesOp::Splt`] description
    #[display("nn")]
    NoneNoneOnEmpty = 1,

    /// If the offset is equal to zero, sets first destination register to `None`, second is set to
    /// an empty string if the string in the source register is empty; `st0` in both cases is
    /// set to `false`.
    ///
    /// Matches case (3) in [`crate::isa::BytesOp::Splt`] description
    #[display("nz")]
    NoneZeroOnEmpty = 2,

    /// If the offset is equal to zero, sets first destination register to empty string, second is
    /// set to an empty string if the string in the source register is empty; `st0` value
    /// remain unchanged.
    ///
    /// Matches case (4) in [`crate::isa::BytesOp::Splt`] description
    #[display("ee")]
    ZeroZeroOnEmpty = 3,

    /// If the offset exceeds the length of the source string sets the first destination register
    /// to the source string (<=offset in len) and second to `None`; `st0` value is set to
    /// `false`.
    ///
    /// Matches case (5) in [`crate::isa::BytesOp::Splt`] description
    #[display("cn")]
    CutNone = 4,

    /// If the offset exceeds the length of the source string sets the first destination register
    /// to the source string (<=offset in len) and second to zero-length string; `st0` value is
    /// set to `false`.
    ///
    /// Matches case (6) in [`crate::isa::BytesOp::Splt`] description
    #[display("cz")]
    CutZero = 5,

    /// If the offset exceeds the length of the source string sets the first destination register
    /// to zero-length string and second to `None`; `st0` value is set to `false`.
    ///
    /// Matches case (7) in [`crate::isa::BytesOp::Splt`] description
    #[display("zn")]
    ZeroNone = 6,

    /// If the offset exceeds the length of the source string sets both the first and second
    /// destination registers to zero-length string; `st0` value is set to `false`.
    ///
    /// Matches case (8) in [`crate::isa::BytesOp::Splt`] description
    #[display("zz")]
    ZeroZero = 7,
}

impl Flag for SplitFlag {}

impl Default for SplitFlag {
    #[inline]
    fn default() -> Self { Self::NoneNone }
}

impl FromStr for SplitFlag {
    type Err = ParseFlagError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseFlagError::RequiredFlagAbsent("split operation"));
        }

        Ok(match s {
            "n" => SplitFlag::NoneNone,
            "nn" => SplitFlag::NoneNoneOnEmpty,
            "nz" => SplitFlag::NoneZeroOnEmpty,
            "ee" => SplitFlag::ZeroZeroOnEmpty,
            "cn" => SplitFlag::CutNone,
            "cz" => SplitFlag::CutZero,
            "zn" => SplitFlag::ZeroNone,
            "zz" => SplitFlag::ZeroZero,
            _ => return Err(ParseFlagError::UnknownFlags("split operation", s.to_owned())),
        })
    }
}

impl SplitFlag {
    /// Constructs split operation flag from `u3` value (used in bytecode serialization)
    pub fn from_u3(val: u3) -> Self {
        match val.to_u8() {
            v if v == SplitFlag::NoneNone as u8 => SplitFlag::NoneNone,
            v if v == SplitFlag::NoneNoneOnEmpty as u8 => SplitFlag::NoneNoneOnEmpty,
            v if v == SplitFlag::NoneZeroOnEmpty as u8 => SplitFlag::NoneZeroOnEmpty,
            v if v == SplitFlag::ZeroZeroOnEmpty as u8 => SplitFlag::ZeroZeroOnEmpty,
            v if v == SplitFlag::CutNone as u8 => SplitFlag::CutNone,
            v if v == SplitFlag::CutZero as u8 => SplitFlag::CutZero,
            v if v == SplitFlag::ZeroNone as u8 => SplitFlag::ZeroNone,
            v if v == SplitFlag::ZeroZero as u8 => SplitFlag::ZeroZero,
            _ => unreachable!(),
        }
    }

    /// Returns `u3` representation of split operation flag (used in bytecode serialization).
    pub fn as_u3(self) -> u3 { u3::with(self as u8) }
}

impl From<u3> for SplitFlag {
    fn from(val: u3) -> Self { Self::from_u3(val) }
}

impl From<&SplitFlag> for u3 {
    fn from(flag: &SplitFlag) -> u3 { flag.as_u3() }
}

impl From<SplitFlag> for u3 {
    fn from(flag: SplitFlag) -> u3 { flag.as_u3() }
}

/// Flags for bytestring insert operation. For the detailed description please read
/// [`crate::isa::BytesOp::Ins`].
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display)]
pub enum InsertFlag {
    /// Set destination to `None` if `offset < dst_len && src_len + dst_len > 2^16`.
    ///
    /// Matches case (6) in [`crate::isa::BytesOp::Ins`] description
    #[display("l")]
    FailOnLen = 0,

    /// Set destination to `None` if `offset > dst_len && src_len + dst_len + offset <= 2^16`.
    ///
    /// Matches case (1) in [`crate::isa::BytesOp::Ins`] description
    #[display("o")]
    FailOnOffset = 1,

    /// Set destination to `None` if `offset > dst_len && src_len + dst_len + offset > 2^16`.
    ///
    /// Matches case (4) in [`crate::isa::BytesOp::Ins`] description
    #[display("f")]
    FailOnOffsetLen = 2,

    /// Fill destination from `dst_let` to `offset` with zeros if
    /// `offset > dst_len && src_len + dst_len + offset <= 2^16`.
    ///
    /// Matches case (2) in [`crate::isa::BytesOp::Ins`] description
    #[display("e")]
    Extend = 3,

    /// Use `src_len` instead of `offset` if
    /// `offset > dst_len && src_len + dst_len + offset <= 2^16`.
    ///
    /// Matches case (3) in [`crate::isa::BytesOp::Ins`] description
    #[display("a")]
    Append = 4,

    /// Fill destination from `dst_let` to `offset` with zeros and cut source string part exceeding
    /// `2^16` if `offset > dst_len && src_len + dst_len + offset > 2^16`
    ///
    /// Matches case (5) in [`crate::isa::BytesOp::Ins`] description
    #[display("x")]
    ExtendCut = 5,

    /// Cut destination string part exceeding `2^16`
    ///
    /// Matches case (7) in [`crate::isa::BytesOp::Ins`] description
    #[display("c")]
    Cut = 6,

    /// Reduce `src_len` such that it will fit the destination
    ///
    /// Matches case (8) in [`crate::isa::BytesOp::Ins`] description
    #[display("s")]
    Shorten = 7,
}

impl Flag for InsertFlag {}

impl Default for InsertFlag {
    #[inline]
    fn default() -> Self { Self::FailOnLen }
}

impl FromStr for InsertFlag {
    type Err = ParseFlagError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseFlagError::RequiredFlagAbsent("insert operation"));
        }
        let filtered = s.replace(&['l', 'o', 'f', 'e', 'a', 'x', 'c', 's'][..], "");
        if !filtered.is_empty() {
            return Err(ParseFlagError::UnknownFlags("insert operation", filtered));
        }
        if filtered.len() > 1 {
            return Err(ParseFlagError::DuplicatedFlags("insert operation", filtered));
        }

        Ok(match filtered.as_bytes()[0].into() {
            'l' => InsertFlag::FailOnLen,
            'o' => InsertFlag::FailOnOffset,
            'f' => InsertFlag::FailOnOffsetLen,
            'e' => InsertFlag::Extend,
            'a' => InsertFlag::Append,
            'x' => InsertFlag::ExtendCut,
            'c' => InsertFlag::Cut,
            's' => InsertFlag::Shorten,
            _ => unreachable!(),
        })
    }
}

impl InsertFlag {
    /// Constructs insert operation flag from `u3` value (used in bytecode serialization)
    pub fn from_u3(val: u3) -> Self {
        match val.to_u8() {
            v if v == InsertFlag::FailOnLen as u8 => InsertFlag::FailOnLen,
            v if v == InsertFlag::FailOnOffset as u8 => InsertFlag::FailOnOffset,
            v if v == InsertFlag::FailOnOffsetLen as u8 => InsertFlag::FailOnOffsetLen,
            v if v == InsertFlag::Extend as u8 => InsertFlag::Extend,
            v if v == InsertFlag::Append as u8 => InsertFlag::Append,
            v if v == InsertFlag::ExtendCut as u8 => InsertFlag::ExtendCut,
            v if v == InsertFlag::Cut as u8 => InsertFlag::Cut,
            v if v == InsertFlag::Shorten as u8 => InsertFlag::Shorten,
            _ => unreachable!(),
        }
    }

    /// Returns `u3` representation of insert operation flag (used in bytecode serialization).
    pub fn as_u3(self) -> u3 { u3::with(self as u8) }
}

impl From<u3> for InsertFlag {
    fn from(val: u3) -> Self { Self::from_u3(val) }
}

impl From<&InsertFlag> for u3 {
    fn from(flag: &InsertFlag) -> u3 { flag.as_u3() }
}

impl From<InsertFlag> for u3 {
    fn from(flag: InsertFlag) -> u3 { flag.as_u3() }
}

/// Flags for bytestring delete operation. For the detailed description please read
/// [`crate::isa::BytesOp::Del`].
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display)]
pub enum DeleteFlag {
    /// Set destination to `None` on any failure.
    ///
    /// Matches case (1) in [`crate::isa::BytesOp::Del`] description
    #[display("n")]
    None = 0,

    /// Set destination to zero-length string if `offset_start > src_len`.
    ///
    /// Matches case (2) in [`crate::isa::BytesOp::Del`] description
    #[display("z")]
    Zero = 1,

    /// Set destination to the fragment of the string `offset_start..src_len` if
    /// `offset_end > src_len && offset_start <= src_len`.
    ///
    /// Matches case (3) in [`crate::isa::BytesOp::Del`] description
    #[display("c")]
    Cut = 2,

    /// Set destination to the fragment of the string `offset_start..src_len` and extend its length
    /// up to `offset_end - offset_start` with trailing zeros if
    /// `offset_end > src_len && offset_start <= src_len`.
    ///
    /// Matches case (4) in [`crate::isa::BytesOp::Del`] description
    #[display("e")]
    Extend = 3,
}

impl Flag for DeleteFlag {}

impl Default for DeleteFlag {
    #[inline]
    fn default() -> Self { Self::None }
}

impl FromStr for DeleteFlag {
    type Err = ParseFlagError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseFlagError::RequiredFlagAbsent("delete operation"));
        }
        let filtered = s.replace(&['n', 'z', 'c', 'e'][..], "");
        if !filtered.is_empty() {
            return Err(ParseFlagError::UnknownFlags("delete operation", filtered));
        }
        if filtered.len() > 1 {
            return Err(ParseFlagError::DuplicatedFlags("delete operation", filtered));
        }

        Ok(match filtered.as_bytes()[0].into() {
            'n' => DeleteFlag::None,
            'z' => DeleteFlag::Zero,
            'c' => DeleteFlag::Cut,
            'e' => DeleteFlag::Extend,
            _ => unreachable!(),
        })
    }
}

impl DeleteFlag {
    /// Constructs delete operation flag from `u2` value (used in bytecode serialization)
    pub fn from_u2(val: u2) -> Self {
        match val.to_u8() {
            v if v == DeleteFlag::None as u8 => DeleteFlag::None,
            v if v == DeleteFlag::Zero as u8 => DeleteFlag::Zero,
            v if v == DeleteFlag::Cut as u8 => DeleteFlag::Cut,
            v if v == DeleteFlag::Extend as u8 => DeleteFlag::Extend,
            _ => unreachable!(),
        }
    }

    /// Returns `u2` representation of delete operation flag (used in bytecode serialization).
    pub fn as_u2(self) -> u2 { u2::with(self as u8) }
}

impl From<u2> for DeleteFlag {
    fn from(val: u2) -> Self { Self::from_u2(val) }
}

impl From<&DeleteFlag> for u2 {
    fn from(flag: &DeleteFlag) -> u2 { flag.as_u2() }
}

impl From<DeleteFlag> for u2 {
    fn from(flag: DeleteFlag) -> u2 { flag.as_u2() }
}
