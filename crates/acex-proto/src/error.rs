#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    BufferTooShort { expected: usize, actual: usize },
    InvalidProtocolVersion,
    PayloadLengthMismatch { expected: usize, actual: usize },
    InvalidPayloadType,
    InvalidPayloadLength,
}

impl From<Error> for acex_core::DiagError {
    fn from(e: Error) -> Self {
        match e {
            Error::BufferTooShort { expected, actual } => {
                acex_core::DiagError::LengthMismatch { expected, actual }
            }
            Error::PayloadLengthMismatch { expected, actual } => {
                acex_core::DiagError::LengthMismatch { expected, actual }
            }
            _ => acex_core::DiagError::InvalidFrame(acex_core::diag_err_str("proto parse error")),
        }
    }
}
