use {
    crate::*,
    rustdoc_types::Crate,
    std::{
        fs,
        path::Path,
        process::Command,
    },
};

/// Build the crate documentation in JSON format and read it into a `rustdoc_types::Crate` struct
pub fn build_and_read_json_doc(
    crate_path: &Path,
    crate_file_name: &str,
) -> CradResult<Crate> {
    let exe = "cargo";
    let args = [
        "+nightly",
        "rustdoc",
        "--lib",
        "--",
        "-Z",
        "unstable-options",
        "--output-format",
        "json",
    ];
    let output = Command::new(exe)
        .args(args)
        .current_dir(&crate_path)
        .output()?;
    //eprintln!("cargo rustdoc output: {:?}", output);
    // TODO check output.status
    let json_path = crate_path
        .join("target")
        .join("doc")
        .join(format!("{crate_file_name}.json"));
    let json = fs::read_to_string(&json_path)?;
    let rd_crate: Crate = serde_json::from_str(&json)?;
    Ok(rd_crate)
}
