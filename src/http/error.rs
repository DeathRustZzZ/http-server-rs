use super::method::MethodError;

/// Ошибка, возникшая при разборе или обработке HTTP-данных.
///
/// An error encountered while parsing or processing HTTP data.
#[derive(Debug)]
pub enum HttpError {
    /// Запрос содержит неподдерживаемый HTTP-метод.
    ///
    /// The request contains an unsupported HTTP method.
    InvalidMethod(MethodError),
}

/// Преобразует ошибку разбора метода в общую HTTP-ошибку.
///
/// Это преобразование позволяет передавать [`MethodError`] с помощью оператора
/// `?` из функций, возвращающих [`HttpError`].
///
/// Converts a method parsing error into a general HTTP error.
///
/// This conversion allows callers to propagate [`MethodError`] values with the
/// `?` operator from functions returning [`HttpError`].
impl From<MethodError> for HttpError {
    fn from(error: MethodError) -> Self {
        Self::InvalidMethod(error)
    }
}
