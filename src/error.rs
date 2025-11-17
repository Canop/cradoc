#[derive(thiserror::Error, Debug)]
pub enum CradError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON Error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Invalid UTF-8: {0}")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("Crate name could not be determined from path")]
    CrateName,
    #[error("No item for crate found in rustdoc JSON")]
    NoCrateItem,
    #[error("No doc in item")]
    NoDocInItem,
    #[error("Cargo command {args:?} failed with status {status:?}")]
    CargoCommandFailure {
        args: &'static[&'static str],
        status: std::process::ExitStatus,
    },
    #[error("Failed to read {path:?}: {error}")]
    Read {
        path: std::path::PathBuf,
        error: std::io::Error,
    },
}

pub type CradResult<T> = Result<T, CradError>;
