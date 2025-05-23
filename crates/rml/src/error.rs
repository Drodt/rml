//! Error handling.
use rustc_session::{EarlyDiagCtxt, Session};
use rustc_span::{Span, DUMMY_SP};

/// Collects all errors encountered by RML.
#[derive(Debug)]
pub struct Error {
    span: Span,
    msg: String,
}

impl Error {
    /// Create error with a `span` and the message `msg`.
    pub(crate) fn new(span: Span, msg: impl Into<String>) -> Self {
        Error {
            span,
            msg: msg.into(),
        }
    }

    /// Emit the error through `rustc`.
    pub(crate) fn emit(self, diag: &EarlyDiagCtxt) -> ! {
        todo!("error reporting")
    }
}

/// Error generated by RML.
#[derive(Debug, Clone)]
pub struct RmlErr;

impl From<RmlErr> for Error {
    fn from(_: RmlErr) -> Error {
        Error::new(DUMMY_SP, "internal error")
    }
}

impl std::fmt::Display for RmlErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "encountered errors during validation")
    }
}
impl std::error::Error for RmlErr {}
