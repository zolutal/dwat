pub mod parse;
pub mod format;

pub use parse::*;

pub mod prelude {
    pub use super::parse::NamedType;
    pub use super::parse::InnerType;
    pub use super::parse::HasMembers;
}

#[derive(thiserror::Error, Debug)]
pub enum DwatError {
    #[error("failed to load dwarf info from file")]
    DwarfLoadError(String),

    #[error("object failed to parse file")]
    ObjectError(#[from] object::Error),

    #[error("failed when attempting to get some CU")]
    CUError(String),

    #[error("failed when attempting to get some DIE")]
    DIEError(String),
}
