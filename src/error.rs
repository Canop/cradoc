#[derive(thiserror::Error, Debug)]
pub enum CradError {
    #[error("IO Error: {0}")]
    IO(#[from] std::io::Error),
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
}

pub type CradResult<T> = Result<T, CradError>;
