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

//! Alu virtual machine

use core::marker::PhantomData;

use crate::core::{Core, CoreConfig, CoreExt, Status};
use crate::isa::{Instr, Instruction};
use crate::library::{Jump, Lib, LibId, LibSite};

/// Alu virtual machine providing single-core execution environment
#[derive(Clone, Debug, Default)]
pub struct Vm<Isa = Instr<LibId>>
where Isa: Instruction<LibId>
{
    /// A set of registers
    pub core: Core<LibId, Isa::Core>,

    phantom: PhantomData<Isa>,
}

/// Runtime for program execution.
impl<Isa> Vm<Isa>
where Isa: Instruction<LibId>
{
    /// Constructs new virtual machine instance with default core configuration.
    pub fn new() -> Self { Self { core: Core::new(), phantom: Default::default() } }

    /// Constructs new virtual machine instance with default core configuration.
    pub fn with(config: CoreConfig, cx_config: <Isa::Core as CoreExt>::Config) -> Self {
        Self {
            core: Core::with(config, cx_config),
            phantom: Default::default(),
        }
    }

    /// Resets all registers of the VM except those which were set up with the config object.
    pub fn reset(&mut self) { self.core.reset(); }

    /// Executes the program starting from the provided entry point.
    ///
    /// # Returns
    ///
    /// Value of the `CK` register at the end of the program execution.
    pub fn exec<L: AsRef<Lib>>(
        &mut self,
        entry_point: LibSite,
        context: &Isa::Context<'_>,
        lib_resolver: impl Fn(LibId) -> Option<L>,
    ) -> Status {
        let mut site = entry_point;
        let mut skip = false;
        loop {
            if let Some(lib) = lib_resolver(site.lib_id) {
                let jump = lib
                    .as_ref()
                    .exec::<Isa>(site.offset, skip, &mut self.core, context);
                match jump {
                    Jump::Halt => {
                        #[cfg(feature = "log")]
                        {
                            let core = &self.core;
                            let z = "\x1B[0m";
                            let y = "\x1B[0;33m";
                            let c = if core.ck().is_ok() { "\x1B[0;32m" } else { "\x1B[0;31m" };
                            eprintln!();
                            eprintln!(
                                ">; execution stopped: {y}CK{z} {c}{}{z}, {y}CO{z} {c}{}{z}",
                                core.ck(),
                                core.co()
                            );
                        }
                        break;
                    }
                    Jump::Instr(new_site) => {
                        skip = false;
                        site = new_site.into();
                    }
                    Jump::Next(new_site) => {
                        skip = true;
                        site = new_site.into();
                    }
                }
            } else {
                let fail = self.core.fail_ck();
                // We stop execution if the failure flag is set
                if fail {
                    break;
                } else if let Some(pos) = site.offset.checked_add(1) {
                    // Otherwise we just proceed
                    site.offset = pos;
                } else {
                    // or we still stop if we reached the end of the code
                    break;
                }
            };
        }
        self.core.ck()
    }
}
