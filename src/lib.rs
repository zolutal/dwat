//! `dwat` is a library for accessing [DWARF](https://dwarfstd.org/)
//! (v4/v5) debuging information.
//!
//! Currently, functionality is focused on making type information from DWARF
//! more accessible by providing [pahole](https://github.com/acmel/dwarves)'s
//! functionality in a library form.

pub mod format;
pub mod parse;

pub use parse::*;

pub mod prelude {
    //! Re-exports commonly needed traits
    pub use super::parse::NamedType;
    pub use super::parse::InnerType;
    pub use super::parse::HasMembers;
}

/// Error type for parsing/loading DWARF information
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to load dwarf info from file")]
    DwarfLoadError(String),

    #[error("object failed to parse file")]
    ObjectError(#[from] object::Error),

    #[error("failed when attempting to get some CU")]
    CUError(String),

    #[error("failed when attempting to get some DIE")]
    DIEError(String),

    // Non Fatal
    #[error("failed when attempting to get a Name Attribute")]
    NameAttributeError,

    #[error("failed when attempting to get a Type Attribute")]
    TypeAttributeError,

    #[error("failed when attempting to get a Type Attribute")]
    ByteSizeAttributeError,
}
