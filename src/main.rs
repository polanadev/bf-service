//! JAM Bootstrap Service
//!
//! Use by concatenating one or more encoded `Instruction`s into a work item's payload.

#![cfg_attr(any(target_arch = "riscv32", target_arch = "riscv64"), no_std)]
#![cfg_attr(any(target_arch = "riscv32", target_arch = "riscv64"), no_main)]
#![allow(clippy::unwrap_used)]

extern crate alloc;

use alloc::{vec, vec::Vec};
use jam_pvm_common::{accumulate::*, *};
use scale::{Decode, Encode};

use jam_types::*;

#[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
fn main() {}

#[derive(Encode, Decode, Debug)]
pub enum Instruction {
    Store {
        value: u64,
    },
    Update {
        value: u64,
    },
    #[doc(hidden)]
    Stored {
        value: u64,
    },
    #[doc(hidden)]
    Updated {
        value: u64,
    },
}

#[allow(dead_code)]
struct Service;
jam_pvm_common::declare_service!(Service);

impl jam_pvm_common::Service for Service {
    fn refine(
        id: ServiceId,
        payload: WorkPayload,
        _package_hash: WorkPackageHash,
        _context: RefineContext,
        _auth_code_hash: CodeHash,
    ) -> WorkOutput {
        info!(target = "bf", "Brainfuck Refine, {id:x}h");
        let mut cursor = &payload[..];
        let mut out = vec![];
        while !cursor.is_empty() {
            match Instruction::decode(&mut cursor).unwrap() {
                Instruction::Store { value } => {
                    out.push(Instruction::Stored { value });
                }
                Instruction::Update { value } => {
                    out.push(Instruction::Update { value });
                }
                x => out.push(x),
            }
        }
        debug!(target = "bf", "Returning {:?} into accumulate", out);
        out.encode().into()
    }
    fn accumulate(now: Slot, id: ServiceId, results: Vec<AccumulateItem>) -> Option<Hash> {
        info!(
            target = "bf",
            "Brainfuck Accumulate, {id:x}h @{now} ${}",
            my_info().balance
        );

        for raw_instructions in results.into_iter().filter_map(|x| x.result.ok()) {
            for inst in Vec::<Instruction>::decode(&mut &raw_instructions[..]).unwrap() {
                debug!(target = "bf", "Decoded instruction: {:?}", inst);
                match inst {
                    Instruction::Stored { value } => {
                        set(b"stored", value).expect("balance?");
                    }
                    Instruction::Updated { value } => {
                        if let Some(v) = get::<u64>(b"stored") {
                            debug!(target = "bf", "Got value: {:?}", v);
                            if value != v {
                                set(b"stored", value).expect("balance?");
                                debug!(target = "bf", "Value updated: {:?}", value);
                            } else {
                                debug!(target = "bf", "Value remains: {:?}", value);
                            }
                        }
                    }
                    i => {
                        info!(target = "boot", "Instruction not handled: {:?}", i);
                    }
                }
            }
        }
        None
    }

    fn on_transfer(_slot: Slot, _id: ServiceId, items: Vec<TransferRecord>) {}
}
