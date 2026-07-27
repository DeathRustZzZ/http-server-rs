/// Ошибки, возникающие при обработке HTTP-данных.
///
/// Errors produced while processing HTTP data.
pub mod error;

/// Методы HTTP-запроса и их парсер.
///
/// HTTP request methods and their parser.
pub mod method;

/// Коды состояния HTTP-ответа.
///
/// HTTP response status codes.
pub mod status_code;

/// HTTP-ответ и его запись в поток.
///
/// HTTP response and its serialization to a stream.
pub mod response;
