use anyhow::Result;

fn main() -> Result<()> {
    tiny_vmm::create_and_run_vm()?;

    Ok(())
}
