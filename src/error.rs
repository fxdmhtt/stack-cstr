use thiserror::Error;

#[derive(Error, Debug)]
pub enum CStrError {
    #[error("format failed")]
    FormatError(#[from] core::fmt::Error),
    #[error("buffer overflow")]
    OverflowError(#[from] arrayvec::CapacityError<char>),
}
