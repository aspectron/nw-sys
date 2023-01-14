//!
//! [`Result`] type used by [`nw_sys`](super) crate encapsulating
//! internal [`Error`](super::error::Error) enum.
//!

pub type Result<T> = std::result::Result<T, crate::error::Error>;
