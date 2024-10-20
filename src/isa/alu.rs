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

/// Control flow instructions.
#[derive(Clone, PartialEq, Eq, Hash, Debug, Display)]
#[display(inner)]
pub enum CtrlInstr {
    Placeholder,
}

/// Register manipulation instructions.
#[derive(Clone, PartialEq, Eq, Hash, Debug, Display)]
#[display(inner)]
pub enum RegInstr {
    Placeholder,
}

/// Arithmetic instructions for natural numbers.
#[derive(Clone, PartialEq, Eq, Hash, Debug, Display)]
#[display(inner)]
pub enum ArithmInstr {
    Placeholder,
}

/// Sign-aware arithmetic instructions.
#[derive(Clone, PartialEq, Eq, Hash, Debug, Display)]
#[display(inner)]
pub enum SignedInstr {
    Placeholder,
}

/// Bit-manipulation and boolean arithmetic instructions.
#[derive(Clone, PartialEq, Eq, Hash, Debug, Display)]
#[display(inner)]
pub enum BitInstr {
    Placeholder,
}

#[cfg(feature = "float")]
/// Floating-point arithmetic instructions.
#[derive(Clone, PartialEq, Eq, Hash, Debug, Display)]
#[display(inner)]
pub enum FloatInstr {
    Placeholder,
}

#[cfg(feature = "array")]
/// Array register (`r`) instructions.
#[derive(Clone, PartialEq, Eq, Hash, Debug, Display)]
#[display(inner)]
pub enum ArrayInstr {
    Placeholder,
}

#[cfg(feature = "str")]
/// Bytestring register (`s`) instructions.
#[derive(Clone, PartialEq, Eq, Hash, Debug, Display)]
#[display(inner)]
pub enum StrInstr {
    Placeholder,
}
