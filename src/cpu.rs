use kvm_ioctls::{VcpuExit, VcpuFd};

pub(super) fn setup_and_run(vcpu_fd: &VcpuFd) {
    // FIXME: setup registers

    // general purpose registers
    let mut gp_regs = vcpu_fd.get_regs().unwrap();
    gp_regs.rip = 0; // base of memory and instructions
    vcpu_fd.set_regs(&gp_regs).unwrap();

    // special registers
    let mut sp_regs = vcpu_fd.get_sregs().unwrap();
    sp_regs.cs.base = 0;
    sp_regs.cs.selector = 0;
    vcpu_fd.set_sregs(&sp_regs).unwrap();

    let vcpu_exit = vcpu_fd.run().unwrap();
    match vcpu_exit {
        VcpuExit::Hlt => {
            println!("we are done");
        }
        _ => {
            println!("this should not happen");
        }
    }
}
