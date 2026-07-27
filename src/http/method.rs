//! HTTP request methods and method parsing.

use std::{error::Error, fmt, str::FromStr};

/// A standard HTTP request method.
///
/// Method names are represented as idiomatic Rust enum variants. On the wire,
/// they are parsed from their case-sensitive uppercase representation, such as
/// `GET` or `POST`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    /// Requests a representation of the target resource.
    Get,
    /// Removes the target resource.
    Delete,
    /// Submits data to the target resource.
    Post,
    /// Creates or replaces the target resource with the request content.
    Put,
    /// Requests the response headers without a response body.
    Head,
    /// Establishes a tunnel to the target server.
    Connect,
    /// Requests the communication options available for the target resource.
    Options,
    /// Performs a diagnostic loopback of the request.
    Trace,
    /// Applies partial modifications to the target resource.
    Patch,
}

/// Parses a case-sensitive HTTP method token.
///
/// # Errors
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
/// An error returned when an HTTP method cannot be parsed.
///
/// The error retains the unsupported method token so it can be included in
/// diagnostics or mapped to an HTTP response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodError(String);

impl MethodError {
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
