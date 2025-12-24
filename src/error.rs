use thiserror::Error;

#[derive(Error, Copy, Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[error("Unexpected '\0' was found!")]
pub struct ContainsNulError;

#[derive(Error, Debug)]
pub enum CStrError {
    #[error(transparent)]
    FormatError(#[from] core::fmt::Error),
    #[error(transparent)]
    OverflowError(#[from] arrayvec::CapacityError<char>),
    #[error(transparent)]
    FromBytesWithNulError(#[from] std::ffi::FromBytesWithNulError),
    #[error(transparent)]
    NulError(#[from] std::ffi::NulError),
    #[error(transparent)]
    ContainsNulError(#[from] ContainsNulError),
}
