//! `dwat` is a library for accessing [DWARF](https://dwarfstd.org/)
//! (v4/v5) debuging information.
//!
//! Currently, functionality is focused on making type information from DWARF
//! more accessible by providing [pahole](https://github.com/acmel/dwarves)'s
//! functionality in a library form.

pub mod format;
pub mod parse;
pub mod dwarf;

pub use dwarf::Dwarf;
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
    // Fatal
    #[error("failed to load dwarf info from file")]
    DwarfLoadError(String),

    #[error("object failed to parse file")]
    ObjectError(#[from] object::Error),

    #[error("failed when attempting to get some CU")]
    CUError(String),

    #[error("failed when attempting to get some DIE")]
    DIEError(String),

    #[error("failed due to unimplemented functionality")]
    UnimplementedError(String),

    // Non-Fatal
    #[error("failure when attempting to find a Name Attribute")]
    NameAttributeNotFound,

    #[error("failure when attempting to find a Type Attribute")]
    TypeAttributeNotFound,

    #[error("failure when attempting to find a ByteSize Attribute")]
    ByteSizeAttributeNotFound,

    #[error("failure when attempting to find a BitSize Attribute")]
    BitSizeAttributeNotFound,

    #[error("failure when attempting to find a MemberLocation Attribute")]
    MemberLocationAttributeNotFound,

    #[error("failure when attempting to find an Alignment Attribute")]
    AlignmentAttributeNotFound,
}
