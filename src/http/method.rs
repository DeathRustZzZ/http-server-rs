use std::{error::Error, fmt, str::FromStr};

/// Стандартный метод HTTP-запроса.
///
/// A standard HTTP request method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Get,
    Delete,
    Post,
    Put,
    Head,
    Connect,
    Options,
    Trace,
    Patch,
}

/// Возвращает [`MethodError`], если входная строка не является именем одного из
/// поддерживаемых стандартных методов.
///
/// Returns [`MethodError`] when the input is not one of the supported standard
/// method names.
impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::Get),
            "DELETE" => Ok(Self::Delete),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            "HEAD" => Ok(Self::Head),
            "CONNECT" => Ok(Self::Connect),
            "OPTIONS" => Ok(Self::Options),
            "TRACE" => Ok(Self::Trace),
            "PATCH" => Ok(Self::Patch),
            invalid => Err(MethodError(invalid.to_owned())),
        }
    }
}
/// Ошибка сохраняет неподдерживаемый токен метода, чтобы его можно было
/// включить в диагностическое сообщение или преобразовать в HTTP-ответ.
///
/// The error retains the unsupported method token so it can be included in
/// diagnostics or mapped to an HTTP response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodError(String);

impl MethodError {
    /// Возвращает неподдерживаемый токен метода, переданный парсеру.
    ///
    /// Returns the unsupported method token supplied to the parser.
    pub fn invalid_method(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for MethodError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unsupported HTTP method: {}", self.0)
    }
}

impl Error for MethodError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_get_method() {
        assert_eq!("GET".parse::<Method>(), Ok(Method::Get));
    }

    #[test]
    fn rejects_unknown_methods() {
        let error = "UNKNOWN".parse::<Method>().unwrap_err();
        assert_eq!(error.invalid_method(), "UNKNOWN")
    }

    #[test]
    fn rejects_lowercase_method() {
        let error = "get".parse::<Method>().unwrap_err();
        assert_eq!(error.invalid_method(), "get");
    }
}
