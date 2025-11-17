use {
    crate::*,
    rustdoc_types::Crate,
    std::{
        fs,
        path::Path,
    },
};

/// Build the crate documentation in JSON format and read it into a `rustdoc_types::Crate` struct
pub fn build_and_read_json_doc(
    crate_path: &Path,
    crate_file_name: &str,
) -> CradResult<Crate> {
    run_cargo_command(
        &[
            "+nightly",
            "rustdoc",
            "--lib",
            "--",
            "-Z",
            "unstable-options",
            "--output-format",
            "json",
        ],
        crate_path,
    )?;
    let json_path = crate_path
        .join("target")
        .join("doc")
        .join(format!("{crate_file_name}.json"));
    let json = fs::read_to_string(&json_path).map_err(|error| CradError::Read {
        path: json_path.to_path_buf(),
        error,
    })?;
    let rd_crate: Crate = serde_json::from_str(&json)?;
    Ok(rd_crate)
}
