use anyhow::Result;
use kvm_ioctls::{VcpuExit, VcpuFd};

pub(super) fn setup_and_run(vcpu_fd: &VcpuFd) -> Result<()> {
    // general purpose registers
    let mut gp_regs = vcpu_fd.get_regs()?;
    // instruction pointer
    gp_regs.rip = 0;
    // input for cpuid
    gp_regs.rax = 0;
    vcpu_fd.set_regs(&gp_regs)?;

    // special registers
    let mut sp_regs = vcpu_fd.get_sregs()?;
    // code segment
    sp_regs.cs.base = 0;
    sp_regs.cs.selector = 0;
    vcpu_fd.set_sregs(&sp_regs)?;

    let vcpu_exit = vcpu_fd.run()?;

    match vcpu_exit {
        VcpuExit::Hlt => {
            println!("we are done");
        }
        _ => {
            println!("this should not happen: {:?}", vcpu_exit);
        }
    }

    Ok(())
}
