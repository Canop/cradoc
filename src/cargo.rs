use {
    crate::*,
    std::{
        path::Path,
        process::Command,
    },
};

/// Build the crate documentation in JSON format and read it into a `rustdoc_types::Crate` struct
pub fn run_cargo_command(
    args: &'static[&'static str],
    workdir: &Path,
) -> CradResult<()> {
    let exe = "cargo";
    let status = Command::new(exe)
        .args(args)
        .current_dir(workdir)
        .status()?;
    if !status.success() {
        return Err(CradError::CargoCommandFailure {
            args,
            status,
        });
    }
    Ok(())
}

