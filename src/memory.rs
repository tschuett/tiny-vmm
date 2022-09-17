use kvm_bindings::bindings::kvm_userspace_memory_region;
use kvm_ioctls::VmFd;
use vm_memory::Address;
use vm_memory::GuestAddress;
use vm_memory::GuestMemory;
use vm_memory::GuestMemoryMmap;
use vm_memory::GuestMemoryRegion;

fn allocate(size: usize) -> GuestMemoryMmap {
    let range = vec![(GuestAddress(0), size)];
    GuestMemoryMmap::from_ranges(&range).unwrap()
}

pub(super) fn setup_memory(vm: &VmFd, memory_size: usize) {
    let guest_mem = allocate(memory_size);
    let flags = 0x0;

    for (index, region) in guest_mem.iter().enumerate() {
        let user_region = kvm_userspace_memory_region {
            slot: index as u32,
            flags,
            guest_phys_addr: region.start_addr().raw_value() as u64,
            memory_size: region.len() as u64,
            userspace_addr: guest_mem.get_host_address(region.start_addr()).unwrap() as u64,
        };

        // set memory region in VM
        unsafe { vm.set_user_memory_region(user_region).unwrap() };
    }

    // dummy x86 code that just calls cpuid and halt.
    let x86_code = [0x0F, 0xA2 /*cpuid*/, 0xf4 /* hlt */];

    // write dummy code into VM memory at address 0
    if let Ok(code_slice) = guest_mem.get_slice(GuestAddress::new(0), x86_code.len()) {
        code_slice.copy_from(&x86_code[..]);
    } else {
        // FIXME
    }
}
