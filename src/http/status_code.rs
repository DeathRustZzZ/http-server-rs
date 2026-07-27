use std::fmt::{self, Display, Formatter};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct StatusCode(u16);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct InvalidStatusCode(u16);

impl StatusCode {
    pub const OK: Self = Self(200);
    pub const BAD_REQUEST: Self = Self(400);
    pub const NOT_FOUND: Self = Self(404);
    pub const INTERNAL_SERVER_ERROR: Self = Self(500);

    pub const fn from_u16(code: u16) -> Result<Self, InvalidStatusCode> {
        if code >= 100 && code <= 599 {
            Ok(Self(code))
        } else {
            Err(InvalidStatusCode(code))
        }
    }

    pub const fn as_u16(self) -> u16 {
        self.0
    }

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

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
