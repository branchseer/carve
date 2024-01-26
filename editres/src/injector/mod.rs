//! APIs for injecting or listing resources in executables

mod inject;
mod list;

pub use inject::inject;
pub use list::list;
