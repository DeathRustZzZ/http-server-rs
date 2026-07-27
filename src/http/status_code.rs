//! Коды состояния HTTP-ответа.
//!
//! HTTP response status codes.

use std::fmt::{self, Display, Formatter};

/// Код состояния HTTP-ответа в допустимом диапазоне `100..=599`.
///
/// Тип поддерживает как известные коды из связанных констант, так и расширения,
/// созданные с помощью [`StatusCode::from_u16`].
///
/// An HTTP response status code in the valid `100..=599` range.
///
/// The type supports both known codes exposed as associated constants and
/// extension codes created with [`StatusCode::from_u16`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct StatusCode(u16);

/// Ошибка, возвращаемая для кода вне допустимого диапазона `100..=599`.
///
/// An error returned for a status code outside the valid `100..=599` range.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct InvalidStatusCode(u16);

impl StatusCode {
    pub const OK: Self = Self(200);
    pub const BAD_REQUEST: Self = Self(400);
    pub const NOT_FOUND: Self = Self(404);
    pub const INTERNAL_SERVER_ERROR: Self = Self(500);
    /// Создаёт код состояния из числового значения.
    /// Возвращает [`InvalidStatusCode`], если значение находится вне диапазона
    /// `100..=599`.
    ///
    /// Creates a status code from its numeric value.
    /// Returns [`InvalidStatusCode`] when the value is outside `100..=599`.
    pub const fn from_u16(code: u16) -> Result<Self, InvalidStatusCode> {
        if code >= 100 && code <= 599 {
            Ok(Self(code))
        } else {
            Err(InvalidStatusCode(code))
        }
    }
    /// Возвращает числовое значение кода состояния.
    ///
    /// Returns the numeric status code.
    pub const fn as_u16(self) -> u16 {
        self.0
    }

    /// Возвращает стандартную поясняющую фразу для известного кода.
    ///
    /// Для допустимых расширенных кодов, неизвестных этой реализации,
    /// возвращает [`None`].
    ///
    /// Returns the standard reason phrase for a known status code.
    ///
    /// Returns [`None`] for valid extension codes unknown to this
    /// implementation.
    pub const fn reason_phrase(self) -> Option<&'static str> {
        match self.0 {
            200 => Some("OK"),
            400 => Some("Bad Request"),
            404 => Some("Not Found"),
            500 => Some("Internal Server Error"),
            _ => None,
        }
    }
}

/// Выводит только трёхзначное числовое значение кода.
///
/// Formats only the three-digit numeric status code.
impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_status_codes_at_valid_range_boundaries() {
        assert_eq!(StatusCode::from_u16(100).unwrap().as_u16(), 100);
        assert_eq!(StatusCode::from_u16(599).unwrap().as_u16(), 599);
    }

    #[test]
    fn rejects_status_codes_outside_valid_range() {
        assert_eq!(StatusCode::from_u16(99), Err(InvalidStatusCode(99)));
        assert_eq!(StatusCode::from_u16(600), Err(InvalidStatusCode(600)));
    }

    #[test]
    fn returns_reason_phrase_for_known_code() {
        assert_eq!(StatusCode::OK.reason_phrase(), Some("OK"));
        assert_eq!(StatusCode::NOT_FOUND.reason_phrase(), Some("Not Found"));
    }

    #[test]
    fn returns_no_reason_phrase_for_unknown_valid_code() {
        let status = StatusCode::from_u16(471).unwrap();

        assert_eq!(status.reason_phrase(), None);
    }

    #[test]
    fn displays_numeric_status_code() {
        assert_eq!(StatusCode::OK.to_string(), "200");
    }
}
