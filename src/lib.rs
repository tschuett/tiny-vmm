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

mod memory;
mod cpu;

use kvm_ioctls::Kvm;
use kvm_ioctls::VmFd;
use kvm_bindings::KVM_MAX_CPUID_ENTRIES;

const MEMORY_SIZE: usize = 1024 *1024;


/// create a tiny vm and run a few instructions.
pub fn create_and_run_vm() {
    let kvm = Kvm::new().unwrap();

    let kvm_cpuid = kvm.get_supported_cpuid(KVM_MAX_CPUID_ENTRIES).unwrap();

    let vm: VmFd = kvm.create_vm().unwrap();

    memory::setup_memory(&vm, MEMORY_SIZE);

    let vcpu_fd = vm.create_vcpu(0).unwrap();

    // setup cpuid
    vcpu_fd.set_cpuid2(&kvm_cpuid).unwrap();

    cpu::setup_and_run(&vcpu_fd);

}
