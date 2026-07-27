use std::{error::Error, fmt, str::FromStr};

/// Добавлены все методы HTTP
///
/// Added all HTTP methods
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
/// Реализация FromStr для метода HTTP
///
/// Implementation of FromStr for HTTP method
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
/// Ошибка при разборе метода HTTP
///
/// Error when parsing HTTP method
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodError(String);

impl MethodError {
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
