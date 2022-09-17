use kvm_ioctls::{VcpuExit, VcpuFd};

pub(super) fn setup_and_run(vcpu_fd: &VcpuFd) {
    // general purpose registers
    let mut gp_regs = vcpu_fd.get_regs().unwrap();
    // base of memory and instructions (instruction pointer)
    gp_regs.rip = 0;
    vcpu_fd.set_regs(&gp_regs).unwrap();

    // special registers
    let mut sp_regs = vcpu_fd.get_sregs().unwrap();
    // code segment
    sp_regs.cs.base = 0;
    sp_regs.cs.selector = 0;
    vcpu_fd.set_sregs(&sp_regs).unwrap();

    let vcpu_exit = vcpu_fd.run().unwrap();
    match vcpu_exit {
        VcpuExit::Hlt => {
            println!("we are done");
        }
        default => {
            println!("this should not happen: {:?}", default);
        }
    }
}
