//! Errors produced while processing HTTP data.

use super::method::MethodError;

/// An error encountered while parsing or processing HTTP data.
#[derive(Debug)]
pub enum HttpError {
    /// The request contains an unsupported HTTP method.
    InvalidMethod(MethodError),
}

/// Converts a method parsing error into a general HTTP error.
///
/// This conversion allows callers to propagate [`MethodError`] values with
/// the `?` operator from functions returning [`HttpError`].
impl From<MethodError> for HttpError {
    fn from(error: MethodError) -> Self {
        Self::InvalidMethod(error)
    }
}
