use kvm_ioctls::{VcpuExit, VcpuFd};

pub(super) fn setup_and_run(vcpu_fd: &VcpuFd) {
    // FIXME: setup registers
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
