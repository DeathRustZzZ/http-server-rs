use super::method::MethodError;

#[derive(Debug)]
pub enum HttpError {
    InvalidMethod(MethodError),
}

/// Реализация From для преобразования MethodError в HttpError
///
/// Implementation of From for converting MethodError to HttpError
impl From<MethodError> for HttpError {
    fn from(error: MethodError) -> Self {
        Self::InvalidMethod(error)
    }
}
