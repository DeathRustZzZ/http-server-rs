//! Core HTTP protocol types.
//!
//! This module contains request-method parsing and the errors produced while
//! processing HTTP data.

/// Errors produced while processing HTTP data.
pub mod error;

/// HTTP request methods and their parser.
pub mod method;
