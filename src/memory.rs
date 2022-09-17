use kvm_bindings::bindings::kvm_userspace_memory_region;
use kvm_ioctls::VmFd;
use vm_memory::Address;
use vm_memory::GuestAddress;
use vm_memory::GuestMemory;
use vm_memory::GuestMemoryMmap;
use vm_memory::GuestMemoryRegion;

const MEM_32BIT_GAP_SIZE: u64 = 768 << 20;
/// The start of the memory area reserved for MMIO devices.
const MMIO_MEM_START: u64 = FIRST_ADDR_PAST_32BITS - MEM_32BIT_GAP_SIZE;
const FIRST_ADDR_PAST_32BITS: u64 = 1 << 32;

fn arch_memory_regions(size: usize) -> Vec<(GuestAddress, usize)> {
    // It's safe to cast MMIO_MEM_START to usize because it fits in a u32 variable
    // (It points to an address in the 32 bit space).
    match size.checked_sub(MMIO_MEM_START as usize) {
        // case1: guest memory fits before the gap
        None | Some(0) => vec![(GuestAddress(0), size)],
        // case2: guest memory extends beyond the gap
        Some(remaining) => vec![
            (GuestAddress(0), MMIO_MEM_START as usize),
            (GuestAddress(FIRST_ADDR_PAST_32BITS), remaining),
        ],
    }
}

fn allocate(size: usize) -> GuestMemoryMmap {
    GuestMemoryMmap::from_ranges(&arch_memory_regions(size)).unwrap()
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

        unsafe { vm.set_user_memory_region(user_region).unwrap() };
    }

    // Dummy x86 code that just calls cpuid and halt.
    let x86_code = [0x0F, 0xA2 /*cpuid*/, 0xf4 /* hlt */];

    if let Ok(code_slice) = guest_mem.get_slice(GuestAddress::new(0), x86_code.len()) {
        code_slice.copy_from(&x86_code[..]);
    } else {
        // FIXME
    }
}
