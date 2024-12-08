use thiserror::Error;

#[derive(Debug, Error)]
pub(super) enum Error {
    #[error("Cannot read file: {msg}")]
    CannotReadFile {
        msg: String,
    },
    #[error("Cannot parse line")]
    CannotParseLine,
}
