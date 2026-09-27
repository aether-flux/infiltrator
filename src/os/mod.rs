//! Central module to handle cross-platform modules and code.
//! Currently, only `linux` is supported.
//! Further modules like `windows` and `mac` may be supported in the future.

pub mod linux;
pub use linux::*;
