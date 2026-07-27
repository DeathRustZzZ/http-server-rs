//! HTTP-ответ и его запись в поток.
//!
//! HTTP response and its serialization to a stream.

use std::io::{Result as IoResult, Write};

use super::status_code::StatusCode;

/// HTTP-ответ, отправляемый клиенту.
///
/// Ответ использует HTTP/1.1, автоматически добавляет заголовок
/// `Content-Length` и поддерживает необязательное текстовое UTF-8-тело.
///
/// An HTTP response sent to a client.
///
/// The response uses HTTP/1.1, automatically adds the `Content-Length` header,
/// and supports an optional UTF-8 text body.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    /// Создаёт HTTP-ответ с указанным статусом и необязательным телом.
    ///
    /// Creates an HTTP response with the given status and optional body.
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self { status_code, body }
    }

    /// Записывает HTTP-ответ в указанный поток.
    ///
    /// Writes the HTTP response to the provided stream.
    ///
    /// # Ошибки
    ///
    /// Возвращает ошибку ввода-вывода, если ответ не удалось полностью записать
    /// в поток.
    ///
    /// # Errors
    ///
    /// Returns an I/O error if the complete response could not be written to
    /// the stream.
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = self.body.as_deref().unwrap_or("");
        let reason_phrase = self.status_code.reason_phrase().unwrap_or("");

        write!(
            stream,
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status_code,
            reason_phrase,
            body.len(),
            body
        )
    }
}

#[cfg(test)]
mod tests {
    use std::io::{self, Write};

    use super::*;

    #[test]
    fn sends_response_with_body() {
        let response = Response::new(StatusCode::OK, Some("Hello".to_owned()));
        let mut output = Vec::new();

        response.send(&mut output).unwrap();

        assert_eq!(output, b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nHello");
    }

    #[test]
    fn sends_response_without_body() {
        let response = Response::new(StatusCode::NOT_FOUND, None);
        let mut output = Vec::new();

        response.send(&mut output).unwrap();

        assert_eq!(
            output,
            b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n"
        );
    }

    #[test]
    fn uses_utf8_byte_length_for_content_length() {
        let response = Response::new(StatusCode::OK, Some("Привет".to_owned()));
        let mut output = Vec::new();

        response.send(&mut output).unwrap();

        assert!(output.starts_with(b"HTTP/1.1 200 OK\r\nContent-Length: 12\r\n\r\n"));
        assert!(output.ends_with("Привет".as_bytes()));
    }

    #[test]
    fn propagates_writer_error() {
        struct FailingWriter;

        impl Write for FailingWriter {
            fn write(&mut self, _buffer: &[u8]) -> io::Result<usize> {
                Err(io::Error::other("write failed"))
            }

            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }

        let response = Response::new(StatusCode::OK, None);

        assert!(response.send(&mut FailingWriter).is_err());
    }
}
