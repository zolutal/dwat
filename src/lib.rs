//! `dwat` is a library for accessing [DWARF](https://dwarfstd.org/)
//! (v4/v5) debuging information.
//!
//! The library is primarily focused on making type information from DWARF more
//! accessible, enabling quick lookups and enumeration of type related DWARF
//! information.
//!
//! `dwat` also implements formatting methods for pretty pretting structs/unions
//! similar to the [pahole](https://github.com/acmel/dwarves) utility and the
//! gdb `ptype` command.

pub mod format;
pub mod types;
pub mod dwarf;

pub use dwarf::Dwarf;
pub use types::*;

#[cfg(feature = "python")]
pub mod python;

pub mod prelude {
    //! Re-exports commonly needed traits
    pub use crate::types::NamedType;
    pub use crate::types::InnerType;
    pub use crate::types::HasMembers;
    pub use crate::dwarf::DwarfContext;
    pub use crate::dwarf::DwarfLookups;
}

/// Error type for parsing/loading DWARF information
#[derive(thiserror::Error, Debug)]
pub enum Error {
    // Fatal
    #[error("failed to load dwarf info from file")]
    DwarfLoadError(String),

    #[error("object failed to parse file")]
    ObjectError(#[from] object::Error),

    #[error("failed when attempting to get offset of a UnitHeader")]
    HeaderOffsetError,

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
