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

#![cfg_attr(coverage_nightly, feature(coverage_attribute), coverage(off))]

use aluvm::stl;
use strict_types::typelib::parse_args;

fn main() {
    let (format, dir) = parse_args();

    stl::aluvm_stl()
        .serialize(
            format,
            dir,
            "0.1.0",
            Some(
                "
  Description: AluVM data type library
  Author: Dr Maxim Orlovsky <orlovsky@ubideco.org>
  Copyright (C) 2024-2025 Laboratories for Ubiquitous Deterministic Computing (UBIDECO),
                          Institute for Distributed and Cognitive Systems (InDCS), Switzerland.
  Copyright (C) 2021-2025 Dr Maxim Orlovsky.
  License: Apache-2.0",
            ),
        )
        .expect("unable to write to the file");
}
