#![cfg_attr(any(target_arch = "riscv32", target_arch = "riscv64"), no_std)]
#![cfg_attr(any(target_arch = "riscv32", target_arch = "riscv64"), no_main)]

extern crate alloc;

use alloc::{string::ToString, vec, vec::Vec};
use jam_pvm_common::{
    accumulate::*,
    refine::{export_slice, import},
    *,
};
use jam_types::*;

#[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
fn main() {}

#[allow(dead_code)]
struct Service;
jam_pvm_common::declare_service!(Service);

impl jam_pvm_common::Service for Service {
    fn refine(
        id: ServiceId,
        _payload: WorkPayload,
        _package_hash: WorkPackageHash,
        _context: RefineContext,
        _auth_code_hash: CodeHash,
    ) -> WorkOutput {
        info!(target = "bf", "Brainfuck Service Refine, {id:x}h");
        vec![].into()
    }

    fn accumulate(now: Slot, id: ServiceId, results: Vec<AccumulateItem>) -> Option<Hash> {
        info!(
            target = "bf",
            "Brainfuck Service Accumulate, {id:x}h @{now} ${}",
            my_info().balance
        );
        None
    }

    fn on_transfer(slot: Slot, id: ServiceId, transfers: Vec<TransferRecord>) {
        info!(target = "bf", "Brainfuck received on_transfer");
    }
}
