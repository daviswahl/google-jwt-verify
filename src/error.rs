use serde_json;
use base64::DecodeError;
use algorithm::Algorithm;
use openssl;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidToken,
    RetrieveKeyFailure,
    UnsupportedAlgorithm(Algorithm),
    Expired
}

impl From<DecodeError> for Error {
    fn from(_: DecodeError) -> Self {
        Error::InvalidToken
    }
}

impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Self {
        Error::InvalidToken
    }
}

impl From<openssl::error::ErrorStack> for Error {
    fn from(_: openssl::error::ErrorStack) -> Self {
        Error::InvalidToken
    }
}