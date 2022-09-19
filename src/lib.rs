#![warn(rustdoc::missing_doc_code_examples)]
#![warn(
    missing_docs,
    rustdoc::missing_crate_level_docs,
    missing_debug_implementations,
    rust_2018_idioms,
    unreachable_pub,
    future_incompatible,
)]

//! a crate for creating a tiny vmm
//! ```rust
//! let x86_code = [0x0F, 0xA2 /*cpuid*/, 0xf4 /* hlt */];
//! ```

mod memory;
mod cpu;

use kvm_ioctls::Kvm;
use kvm_bindings::KVM_MAX_CPUID_ENTRIES;

use anyhow::Result;

const MEMORY_SIZE: usize = 1024 * 1024;

/// create a tiny vm and run a few instructions.
pub fn create_and_run_vm() -> Result<()>{
    let kvm = Kvm::new()?;

    let kvm_cpuid = kvm.get_supported_cpuid(KVM_MAX_CPUID_ENTRIES)?;

    // create a virtual machine
    let vm = kvm.create_vm()?;

    // setup memory and instructions
    memory::setup_memory(&vm, MEMORY_SIZE)?;

    // create one vcpu
    let vcpu_fd = vm.create_vcpu(0)?;

    // setup cpuid
    vcpu_fd.set_cpuid2(&kvm_cpuid)?;

    // setup and run vcpu
    cpu::setup_and_run(&vcpu_fd)?;

    Ok(())
}
