//! Interfaces representing DWARF type information

use gimli::{DebugStrOffset, DebugLineStrOffset, AttributeValue};

use crate::dwarf::{DwarfContext, GimliDIE, GimliCU, DwarfUnit};
use crate::dwarf::borrowable_dwarf::BorrowableDwarf;
use crate::types::unit_has_members::UnitHasMembers;
use crate::types::unit_inner_type::UnitInnerType;
use crate::types::unit_name_type::UnitNamedType;
use crate::format::{format_member, format_type};
use crate::Error;

/// Represents a struct type
#[derive(Clone, Copy, Debug)]
pub struct Struct {
    pub location: DwarfUnit,
}

/// Represents an array type
#[derive(Clone, Copy, Debug)]
pub struct Array {
    pub location: DwarfUnit,
}

/// Represents an enum type
#[derive(Clone, Copy, Debug)]
pub struct Enum {
    pub location: DwarfUnit,
}

/// Represents a pointer to a type
#[derive(Clone, Copy, Debug)]
pub struct Pointer {
    pub location: DwarfUnit,
}

/// Represents a type that is a function pointer prototype
#[derive(Clone, Copy, Debug)]
pub struct Subroutine {
    pub location: DwarfUnit,
}

/// Represents a typedef renaming of a type
#[derive(Clone, Copy, Debug)]
pub struct Typedef {
    pub location: DwarfUnit,
}

/// Represents a union type
#[derive(Clone, Copy, Debug)]
pub struct Union {
    pub location: DwarfUnit,
}

/// Represents a base type, e.g. int, long, etc...
#[derive(Clone, Copy, Debug)]
pub struct Base {
    pub location: DwarfUnit,
}

/// Represents the C const type-modifier
#[derive(Clone, Copy, Debug)]
pub struct Const {
    pub location: DwarfUnit,
}

/// Represents the C volatile type-modifier
#[derive(Clone, Copy, Debug)]
pub struct Volatile {
    pub location: DwarfUnit,
}

/// Represents the C restrict type-modifier
#[derive(Clone, Copy, Debug)]
pub struct Restrict {
    pub location: DwarfUnit,
}

/// Represents the arguments list of a Subprocedure
#[derive(Clone, Copy, Debug)]
pub struct FormalParameter {
    pub location: DwarfUnit,
}

/// Represents a variable declaration
#[derive(Clone, Copy, Debug)]
pub struct Variable {
    pub location: DwarfUnit,
}

/// Represents a value of an enum option
#[derive(Clone, Debug)]
pub struct Enumerator {
    pub name: String,
    pub value: u64,
}

/// Represents a field of a struct or union
#[derive(Clone, Copy, Debug)]
pub struct Member {
    pub location: DwarfUnit,
}

/// Represents a DWARF compile unit
#[derive(Clone, Copy, Debug)]
pub struct CompileUnit {
    pub location: DwarfUnit,
}

/// Represents a subprogram/function
#[derive(Clone, Copy, Debug)]
pub struct Subprogram {
    pub location: DwarfUnit,
}


/// This trait specifies that a type is associated with some DWARF tag
pub trait Tagged {
    fn new(location: DwarfUnit) -> Self;
    fn tag() -> gimli::DwTag;
}

macro_rules! impl_tagged {
    ($type:ty, $tag:expr) => {
        impl Tagged for $type {
            fn new(location: DwarfUnit) -> Self {
                Self { location }
            }

            fn tag() -> gimli::DwTag {
                $tag
            }
        }
    };
}

impl_tagged!(Struct, gimli::DW_TAG_structure_type);
impl_tagged!(Array, gimli::DW_TAG_array_type);
impl_tagged!(Enum, gimli::DW_TAG_enumeration_type);
impl_tagged!(Pointer, gimli::DW_TAG_pointer_type);
impl_tagged!(Subroutine, gimli::DW_TAG_subroutine_type);
impl_tagged!(Typedef, gimli::DW_TAG_typedef);
impl_tagged!(Union, gimli::DW_TAG_union_type);
impl_tagged!(Base, gimli::DW_TAG_base_type);
impl_tagged!(Const, gimli::DW_TAG_const_type);
impl_tagged!(Volatile, gimli::DW_TAG_volatile_type);
impl_tagged!(Restrict, gimli::DW_TAG_restrict_type);
impl_tagged!(Variable, gimli::DW_TAG_variable);
impl_tagged!(CompileUnit, gimli::DW_TAG_compile_unit);
impl_tagged!(Subprogram, gimli::DW_TAG_subprogram);


/// Enum of supported types which may be returned by get_type()
#[derive(Clone, Copy, Debug)]
pub enum Type {
    Struct(Struct),
    Array(Array),
    Enum(Enum),
    Pointer(Pointer),
    Subroutine(Subroutine),
    Typedef(Typedef),
    Union(Union),
    Base(Base),
    Const(Const),
    Volatile(Volatile),
    Restrict(Restrict),
    Variable(Variable),
}

impl Type {
    fn u_byte_size(&self, unit: &GimliCU) -> Result<usize, Error> {
        match self {
            Type::Struct(struc) => {
                struc.u_byte_size(unit)
            },
            Type::Array(arr) => {
                arr.u_byte_size(unit)
            }
            Type::Pointer(ptr) => {
                ptr.u_byte_size(unit)
            }
            Type::Base(base) => {
                base.u_byte_size(unit)
            }
            Type::Union(uni) => {
                uni.u_byte_size(unit)
            }
            Type::Enum(enu) => {
                enu.u_byte_size(unit)
            }
            Type::Typedef(typedef) => {
                typedef.u_byte_size(unit)
            }
            Type::Const(cons) => {
                cons.u_byte_size(unit)
            }
            Type::Volatile(vol) => {
                vol.u_byte_size(unit)
            }
            Type::Restrict(vol) => {
                vol.u_byte_size(unit)
            }
            Type::Variable(vol) => {
                vol.u_byte_size(unit)
            }
            // --- Unsized ---
            Type::Subroutine(_) => {
                Err(Error::ByteSizeAttributeNotFound)
            }
        }
    }

    pub fn byte_size<D>(&self, dwarf: &D) -> Result<usize, Error>
    where D: DwarfContext {
        match self {
            Type::Struct(struc) => {
                struc.byte_size(dwarf)
            },
            Type::Array(arr) => {
                arr.byte_size(dwarf)
            }
            Type::Pointer(ptr) => {
                ptr.byte_size(dwarf)
            }
            Type::Base(base) => {
                base.byte_size(dwarf)
            }
            Type::Union(uni) => {
                uni.byte_size(dwarf)
            }
            Type::Enum(enu) => {
                enu.byte_size(dwarf)
            }
            Type::Typedef(typedef) => {
                typedef.byte_size(dwarf)
            }
            Type::Const(cons) => {
                cons.byte_size(dwarf)
            }
            Type::Volatile(vol) => {
                vol.byte_size(dwarf)
            }
            Type::Restrict(vol) => {
                vol.byte_size(dwarf)
            }
            Type::Variable(vol) => {
                vol.byte_size(dwarf)
            }
            // --- Unsized ---
            Type::Subroutine(_) => {
                Err(Error::ByteSizeAttributeNotFound)
            }
        }
    }
}

// Try to retrieve a string from the debug_str section for a given offset
pub(crate) fn from_dbg_str_ref<D>(dwarf: &D, str_ref: DebugStrOffset<usize>)
-> Option<String>
where D: DwarfContext + BorrowableDwarf {
    dwarf.borrow_dwarf(|dwarf| {
        if let Ok(str_ref) = dwarf.debug_str.get_str(str_ref) {
            let str_ref = str_ref.to_string_lossy();
            return Some(str_ref.to_string());
        }
        None
    })
}

// Try to retrieve a string from the debug_str section for a given offset
pub(crate) fn from_dbg_line_str_ref<D>(dwarf: &D, str_ref: DebugLineStrOffset<usize>)
-> Option<String>
where D: DwarfContext + BorrowableDwarf {
    dwarf.borrow_dwarf(|dwarf| {
        if let Ok(str_ref) = dwarf.debug_line_str.get_str(str_ref) {
            let str_ref = str_ref.to_string_lossy();
            return Some(str_ref.to_string());
        }
        None
    })
}

// Try to retrieve the name attribute as a string for a DIE if one exists
pub(crate) fn get_entry_name<D>(dwarf: &D, entry: &GimliDIE) -> Result<String, Error>
where D: DwarfContext + BorrowableDwarf {
    if let Ok(attr_val) = entry.attr_value(gimli::DW_AT_name) {
        match attr_val {
            Some(AttributeValue::String(str)) => {
                if let Ok(str) = str.to_string() {
                    return Ok(str.to_string())
                }
            },
            Some(AttributeValue::DebugStrRef(strref)) => {
                if let Some(str) = from_dbg_str_ref(dwarf, strref) {
                    return Ok(str)
                }
            },
            Some(AttributeValue::DebugLineStrRef(strref)) => {
                if let Some(str) = from_dbg_line_str_ref(dwarf, strref) {
                    return Ok(str)
                }
            },
            None => {
                return Err(Error::NameAttributeNotFound)
            },
            _ => { }
        }
    }
    Err(Error::InvalidAttributeError)
}

/// force UnitNamedType trait to be private
pub(crate) mod unit_name_type {
    use crate::types::*;
    use crate::Error;

    /// Public crate trait backing NamedType
    pub trait UnitNamedType {
        fn location(&self) -> DwarfUnit;

        fn u_name<D>(&self, dwarf: &D, unit: &GimliCU) -> Result<String, Error>
        where D: DwarfContext + BorrowableDwarf {
            unit.entry_context(&self.location(), |entry| {
                get_entry_name(dwarf, entry)
            })?
        }
    }
}

pub trait NamedType : unit_name_type::UnitNamedType {
    fn name<D>(&self, dwarf: &D) -> Result<String, Error>
    where D: DwarfContext + BorrowableDwarf {
        dwarf.unit_context(&self.location(), |unit| {
            self.u_name(dwarf, unit)
        })?
    }
}

macro_rules! impl_named_type {
    ($type:ty) => {
        impl unit_name_type::UnitNamedType for $type {
            fn location(&self) -> DwarfUnit {
                self.location
            }
        }
        impl NamedType for $type { }
    };
}

impl_named_type!(Struct);
impl_named_type!(Array);
impl_named_type!(Enum);
impl_named_type!(Subroutine);
impl_named_type!(Typedef);
impl_named_type!(Union);
impl_named_type!(Base);
impl_named_type!(Const);
impl_named_type!(Volatile);
impl_named_type!(Restrict);
impl_named_type!(Variable);
impl_named_type!(Member);
impl_named_type!(CompileUnit);
impl_named_type!(Subprogram);


/// force UnitInnerType trait to be private
pub(crate) mod unit_inner_type {
    use crate::types::*;
    use crate::Error;

    pub trait UnitInnerType {
        fn location(&self) -> DwarfUnit;

        // DW_AT_type : reference
        fn u_get_type(&self, unit: &GimliCU) -> Result<Type, Error> {
            unit.entry_context(&self.location(), |entry| {
                if let Ok(attr_val) = entry.attr_value(gimli::DW_AT_type) {
                    if let Some(AttributeValue::UnitRef(entry_offset)) = attr_val {
                        let type_loc = DwarfUnit {
                            die_offset: self.location().die_offset,
                            entry_offset,
                        };
                        return unit.entry_context(&type_loc, |entry| {
                            entry_to_type(type_loc, entry)
                        })?
                    } else {
                        Err(Error::TypeAttributeNotFound)
                    }
                } else {
                    Err(Error::InvalidAttributeError)
                }
            })?
        }
    }
}

/// This trait specifies that a types contains another type (singular)
pub trait InnerType : unit_inner_type::UnitInnerType {
    fn get_type<D>(&self, dwarf: &D) -> Result<Type, Error>
    where D: DwarfContext + BorrowableDwarf {
        dwarf.unit_context(&self.location().clone(), |unit| {
            self.u_get_type(unit)
        })?
    }
}

macro_rules! impl_inner_type {
    ($type:ty) => {
        impl unit_inner_type::UnitInnerType for $type {
            fn location(&self) -> DwarfUnit {
                self.location
            }
        }
        impl InnerType for $type { }
    };
}

impl_inner_type!(Const);
impl_inner_type!(Volatile);
impl_inner_type!(Restrict);
impl_inner_type!(FormalParameter);
impl_inner_type!(Subroutine);
impl_inner_type!(Pointer);
impl_inner_type!(Variable);
impl_inner_type!(Typedef);
impl_inner_type!(Array);
impl_inner_type!(Enum);
impl_inner_type!(Member);


// DW_AT_byte_size : constant,exprloc,reference
fn get_entry_byte_size(entry: &GimliDIE) -> Result<usize, Error> {
    if let Ok(opt_attr) = entry.attr(gimli::DW_AT_byte_size) {
        if let Some(attr) = opt_attr {
            if let Some(attr_val) = attr.udata_value() {
                return Ok(attr_val as usize)
            }
            match attr.value() {
                AttributeValue::Exprloc(_) => {
                    return Err(Error::UnimplementedError("byte_size with exprloc value".into()))
                },
                AttributeValue::LocationListsRef(_) => {
                    return Err(Error::UnimplementedError("byte_size with loclist value".into()))
                },
                _ => { }
            }
        } else {
            return Err(Error::ByteSizeAttributeNotFound)
        }
    }
    Err(Error::InvalidAttributeError)
}

// Try to retrieve the alignment attribute if one exists, alignment was added
// in DWARF 5 but gcc will inlcude it even for -gdwarf-4
// DW_AT_alignment : constant
fn get_entry_alignment(entry: &GimliDIE) -> Result<usize, Error> {
    if let Ok(opt_attr) = entry.attr(gimli::DW_AT_alignment) {
        if let Some(attr) = opt_attr {
            if let Some(alignment) = attr.udata_value() {
                return Ok(alignment as usize)
            }
        } else {
            return Err(Error::AlignmentAttributeNotFound)
        }
    }
    Err(Error::InvalidAttributeError)
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Language {
    /// ISO C:1989
    C89,
    /// Non-standardized C, such as K&R
    C,
    /// ISO Ada:1983
    Ada83,
    // ISO C++98
    C_plus_plus,
    /// ISO COBOL:1974
    Cobol74,
    /// ISO COBOL:1985
    Cobol85,
    /// ISO FORTRAN:1977
    Fortran77,
    /// ISO Fortran:1990
    Fortran90,
    /// ISO Pascal:1983
    Pascal83,
    /// ISO Modula-2:1996
    Modula2,
    /// Java
    Java,
    /// ISO C:1999
    C99,
    /// ISO Ada:1995
    Ada95,
    /// ISO Fortran:1995
    Fortran95,
    /// ANSI PL/I:1976
    PLI,
    /// Objective C
    ObjC,
    /// Objective C++
    ObjC_plus_plus,
    /// UPC (Unified Parallel C)
    UPC,
    /// D
    D,
    /// Python
    Python,
    /// OpenCL
    OpenCL,
    /// Go
    Go,
    /// Modula-3
    Modula3,
    /// Haskell
    Haskell,
    /// ISO C++03
    C_plus_plus_03,
    /// ISO C++11
    C_plus_plus_11,
    /// OCaml
    OCaml,
    /// Rust
    Rust,
    /// ISO C:2011
    C11,
    /// Swift
    Swift,
    /// Julia
    Julia,
    /// Dylan
    Dylan,
    /// ISO C++14
    C_plus_plus_14,
    /// ISO Fortran:2004
    Fortran03,
    /// ISO Fortran:2010
    Fortran08,
    /// RenderScript Kernel Language
    RenderScript,
    /// BLISS
    BLISS,
    /// Vendor Extension
    Vendor(u16)
}

impl TryFrom<u16> for Language {
    type Error = ();

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Language::C89),
            2 => Ok(Language::C),
            3 => Ok(Language::Ada83),
            4 => Ok(Language::C_plus_plus),
            5 => Ok(Language::Cobol74),
            6 => Ok(Language::Cobol85),
            7 => Ok(Language::Fortran77),
            8 => Ok(Language::Fortran90),
            9 => Ok(Language::Pascal83),
            10 => Ok(Language::Modula2),
            11 => Ok(Language::Java),
            12 => Ok(Language::C99),
            13 => Ok(Language::Ada95),
            14 => Ok(Language::Fortran95),
            15 => Ok(Language::PLI),
            16 => Ok(Language::ObjC),
            17 => Ok(Language::ObjC_plus_plus),
            18 => Ok(Language::UPC),
            19 => Ok(Language::D),
            20 => Ok(Language::Python),
            21 => Ok(Language::OpenCL),
            22 => Ok(Language::Go),
            23 => Ok(Language::Modula3),
            24 => Ok(Language::Haskell),
            25 => Ok(Language::C_plus_plus_03),
            26 => Ok(Language::C_plus_plus_11),
            27 => Ok(Language::OCaml),
            28 => Ok(Language::Rust),
            29 => Ok(Language::C11),
            30 => Ok(Language::Swift),
            31 => Ok(Language::Julia),
            32 => Ok(Language::Dylan),
            33 => Ok(Language::C_plus_plus_14),
            34 => Ok(Language::Fortran03),
            35 => Ok(Language::Fortran08),
            36 => Ok(Language::RenderScript),
            37 => Ok(Language::BLISS),
            _ => Err(()),
        }
    }
}

impl CompileUnit {
    pub fn producer<D: DwarfContext + BorrowableDwarf>(&self, dwarf: &D)
    -> Result<String, Error> {
        dwarf.entry_context(&self.location, |entry| {
            if let Ok(attr_val) = entry.attr_value(gimli::DW_AT_producer) {
                match attr_val {
                    Some(AttributeValue::String(str)) => {
                        if let Ok(str) = str.to_string() {
                            return Ok(str.to_string())
                        }
                    },
                    Some(AttributeValue::DebugStrRef(strref)) => {
                        if let Some(str) = from_dbg_str_ref(dwarf, strref) {
                            return Ok(str)
                        }
                    },
                    Some(AttributeValue::DebugLineStrRef(strref)) => {
                        if let Some(str) = from_dbg_line_str_ref(dwarf, strref) {
                            return Ok(str)
                        }
                    },
                    None => {
                        return Err(Error::ProducerAttributeNotFound)
                    },
                    _ => { }
                }
            }
            Err(Error::InvalidAttributeError)
        })?
    }

    pub fn language<D: DwarfContext + BorrowableDwarf>(&self, dwarf: &D)
    -> Result<Language, Error> {
        dwarf.entry_context(&self.location, |entry| {
            if let Ok(attr_val) = entry.attr_value(gimli::DW_AT_language) {
                match attr_val {
                    Some(AttributeValue::Language(lang)) => {
                        if lang.0 >= 0x8000 {
                            return Ok(Language::Vendor(lang.0));
                        } else {
                            let lang = match lang.0.try_into() {
                                Ok(lang) => lang,
                                Err(_) => return Err(Error::InvalidAttributeError)
                            };
                            return Ok(lang)
                        }
                    },
                    None => {
                        return Err(Error::LanguageAttributeNotFound)
                    },
                    _ => { }
                }
            }
            Err(Error::InvalidAttributeError)
        })?
    }
}

impl Subroutine {
    fn location(&self) -> DwarfUnit {
        self.location
    }

    pub(crate) fn u_get_params(&self, unit: &GimliCU)
    -> Result<Vec<FormalParameter>, Error> {
        let mut params: Vec<FormalParameter> = vec![];
        let mut entries = {
            match unit.entries_at_offset(self.location.entry_offset) {
                Ok(entries) => entries,
                _ => return Err(Error::DIEError(
                   format!("Failed to seek to DIE at {:?}", self.location())
                ))
            }
        };
        if entries.next_dfs().is_err() {
            return Err(Error::DIEError(
               format!("Failed to find next DIE at {:?}", self.location())
            ))
        }
        while let Ok(Some((_, entry))) = entries.next_dfs() {
            if entry.tag() != gimli::DW_TAG_formal_parameter {
                break;
            }
            let location = DwarfUnit {
                die_offset: self.location.die_offset,
                entry_offset: entry.offset(),
            };
            params.push(FormalParameter { location });
        };
        Ok(params)
    }

    pub fn get_params<D: DwarfContext>(&self, dwarf: &D)
    -> Result<Vec<FormalParameter>, Error> {
        dwarf.unit_context(&self.location, |unit| {
            self.u_get_params(unit)
        })?
    }
}

fn entry_to_type(location: DwarfUnit, entry: &GimliDIE) -> Result<Type, Error> {
    let tag = match entry.tag() {
        gimli::DW_TAG_array_type => {
            Type::Array(Array{location})
        },
        gimli::DW_TAG_enumeration_type => {
            Type::Enum(Enum{location})
        },
        gimli::DW_TAG_pointer_type => {
            Type::Pointer(Pointer{location})
        },
        gimli::DW_TAG_structure_type => {
            Type::Struct(Struct{location})
        },
        gimli::DW_TAG_subroutine_type => {
            Type::Subroutine(Subroutine{location})
        },
        gimli::DW_TAG_typedef => {
            Type::Typedef(Typedef{location})
        },
        gimli::DW_TAG_union_type => {
            Type::Union(Union{location})
        },
        gimli::DW_TAG_base_type => {
            Type::Base(Base{location})
        },
        gimli::DW_TAG_const_type => {
            Type::Const(Const{location})
        },
        gimli::DW_TAG_volatile_type => {
            Type::Volatile(Volatile{location})
        },
        gimli::DW_TAG_restrict_type => {
            Type::Restrict(Restrict{location})
        },
        _ => {
            return Err(Error::UnimplementedError(
                "entry_to_type, unhandled dwarf type".to_string()
            ));
        }
    };
    Ok(tag)
}

impl Member {
    // DW_AT_data_member_location : constant,exprloc,loclist
    pub(crate) fn u_bit_size(&self, unit: &GimliCU) -> Result<usize, Error> {
        unit.entry_context(&self.location, |entry| {
            if let Ok(opt_attr) = entry.attr(gimli::DW_AT_bit_size) {
                if let Some(attr) = opt_attr {
                    if let Some(attr_val) = attr.udata_value() {
                        return Ok(attr_val as usize)
                    }
                    match attr.value() {
                        AttributeValue::Exprloc(_) => {
                            return Err(Error::UnimplementedError("bit_size with exprloc value".into()))
                        },
                        AttributeValue::LocationListsRef(_) => {
                            return Err(Error::UnimplementedError("bit_size with loclist value".into()))
                        },
                        _ => { }
                    }
                } else {
                    return Err(Error::BitSizeAttributeNotFound)
                }
            }
            Err(Error::InvalidAttributeError)
        })?
    }

    /// The size of a member in bits
    pub fn bit_size<D>(&self, dwarf: &D) -> Result<usize, Error>
    where D: DwarfContext {
        dwarf.unit_context(&self.location, |unit| {
            self.u_bit_size(unit)
        })?
    }

    pub(crate) fn u_byte_size(&self, unit: &GimliCU) -> Result<usize, Error> {
        let inner = self.u_get_type(unit)?;
        inner.u_byte_size(unit)
    }

    pub fn byte_size<D>(&self, dwarf: &D) -> Result<usize, Error>
    where D: DwarfContext {
        dwarf.unit_context(&self.location, |unit| {
            self.u_byte_size(unit)
        })?
    }

    // DW_AT_data_member_location : constant,exprloc,loclist
    pub(crate) fn u_member_location(&self, unit: &GimliCU) -> Result<usize, Error> {
        unit.entry_context(&self.location, |entry| {
            if let Ok(opt_attr) = entry.attr(gimli::DW_AT_data_member_location) {
                if let Some(attr) = opt_attr {
                    if let Some(attr_val) = attr.udata_value() {
                        return Ok(attr_val as usize)
                    }
                    match attr.value() {
                        AttributeValue::Exprloc(_) => {
                            return Err(Error::UnimplementedError("member_location with exprloc value".into()))
                        },
                        AttributeValue::LocationListsRef(_) => {
                            return Err(Error::UnimplementedError("member_location with loclist value".into()))
                        },
                        _ => { }
                    }
                } else {
                    return Err(Error::MemberLocationAttributeNotFound)
                }
            }
            Err(Error::InvalidAttributeError)
        })?
    }

    /// The byte offset of the member from the start of the datatype
    pub fn member_location<D>(&self, dwarf: &D) -> Result<usize, Error>
    where D: DwarfContext {
        dwarf.unit_context(&self.location, |unit| {
            self.u_member_location(unit)
        })?
    }

    pub(crate) fn u_offset(&self, unit: &GimliCU) -> Result<usize, Error> {
        self.u_member_location(unit)
    }

    /// Alias for member_location
    pub fn offset<D>(&self, dwarf: &D) -> Result<usize, Error>
    where D: DwarfContext {
        self.member_location(dwarf)
    }
}

/// prevent UnitHasMembers trait from being usable outside of the library
pub(crate) mod unit_has_members {
    use crate::types::*;
    use crate::Error;

    pub trait UnitHasMembers {
        fn location(&self) -> DwarfUnit;

        fn u_members(&self, unit: &GimliCU) -> Result<Vec<Member>, Error> {
            let mut members: Vec<Member> = Vec::new();
            let mut entries = {
                match unit.entries_at_offset(self.location().entry_offset) {
                    Ok(entries) => entries,
                    _ => return Err(Error::DIEError(
                       format!("Failed to seek to DIE at {:?}", self.location())
                    ))
                }
            };
            if entries.next_dfs().is_err() {
                return Err(Error::DIEError(
                    format!("Failed to find next DIE at {:?}", self.location())
                ))
            }
            while let Ok(Some((_, entry))) = entries.next_dfs() {
                if entry.tag() != gimli::DW_TAG_member {
                    break;
                }
                let location = DwarfUnit {
                    die_offset: self.location().die_offset,
                    entry_offset: entry.offset(),
                };
                members.push(Member { location });
            };
            Ok(members)
        }
    }
}

pub trait HasMembers : unit_has_members::UnitHasMembers {
    /// Get the members/fields of this type
    fn members<D>(&self, dwarf: &D) -> Result<Vec<Member>, Error>
    where D: DwarfContext {
        dwarf.unit_context(&self.location(), |unit| {
            self.u_members(unit)
        })?
    }
}

impl unit_has_members::UnitHasMembers for Struct {
    fn location(&self) -> DwarfUnit { self.location }
}
impl unit_has_members::UnitHasMembers for Union {
    fn location(&self) -> DwarfUnit { self.location }
}

impl HasMembers for Struct { }
impl HasMembers for Union { }


/// A summary of alignment data for a Struct, used to determine packed and
/// aligned attributes
pub struct AlignmentStats {
    /// A count of gaps, 'holes', in the struct
    pub nr_holes: usize,

    /// A vector containing tuples of (index, hole size)
    pub hole_positions: Vec<(usize, usize)>,

    /// The sum of unused bytes from holes in the struct
    pub sum_holes: usize,

    /// The sum of the sizes of members in the struct
    pub sum_member_size: usize,

    /// The amount of trailing unused bytes
    pub padding: usize,

    /// The number of times a member was aligned with less than its natural
    /// alignment, e.g. an 32-bit int was not 4-byte aligned
    /// (this is currently innacurate, unsure how natural size should be
    /// determined for structs, potentially needs to be done recursively)
    pub nr_unnat_alignment: usize,
}

impl Struct {
    fn location(&self) -> DwarfUnit {
        self.location
    }

    pub fn alignment_stats<D>(&self, dwarf: &D)
    -> Result<AlignmentStats, Error>
    where D: DwarfContext + BorrowableDwarf {
        let mut nr_holes: usize = 0;
        let mut hole_positions: Vec<(usize, usize)> = Vec::new();
        let mut sum_holes: usize = 0;
        let mut sum_member_size: usize = 0;
        let mut nr_unnat_alignment: usize = 0;

        let mut prev_offset: usize = 0;
        let mut prev_size: usize = 0;
        for (idx, member) in self.members(dwarf)?.into_iter().enumerate() {
            let curr_offset = member.offset(dwarf)?;
            let curr_size = member.byte_size(dwarf)?;

            sum_member_size += curr_size;

            // nothing to do for the first member
            if prev_offset == 0 {
                prev_offset = curr_offset;
                prev_size = curr_size;
                continue
            }

            // array alignment is based on the entry type size
            let byte_size_single = match member.get_type(dwarf)? {
                Type::Array(arr) => arr.entry_size(dwarf)?,
                _ => curr_size
            };

            // size zero members don't matter
            if curr_size == 0 || byte_size_single == 0 {
                continue
            }

            // calc padding between end of prev type
            let hole_sz = curr_offset - (prev_size + prev_offset);
            sum_holes += hole_sz;

            if hole_sz > 0 {
                nr_holes += 1;
                hole_positions.push((idx, hole_sz));
            }

            // if the size is divisible byte the type size, it is naturally
            // aligned, otherwise some packing likely occurred
            if curr_offset % byte_size_single != 0 {
                nr_unnat_alignment += 1;
            }

            prev_offset = curr_offset;
            prev_size = curr_size;
        }

        let byte_size = self.byte_size(dwarf)?;

        // check the distance to the end of the struct for padding
        let padding = byte_size - (prev_size + prev_offset);

        Ok(AlignmentStats { nr_holes, sum_holes, hole_positions, padding,
                            sum_member_size, nr_unnat_alignment })
    }

    pub fn to_string_verbose<D>(&self, dwarf: &D, verbosity: u8)
    -> Result<String, Error>
    where D: BorrowableDwarf + DwarfContext {
        let mut repr = String::new();
        let _ = dwarf.unit_context(&self.location, |unit| {
            match self.u_name(dwarf, unit) {
                Ok(name) => repr.push_str(&format!("struct {} {{\n", name)),
                Err(Error::NameAttributeNotFound) => {
                    repr.push_str("struct {\n")
                },
                Err(e) => return Err(e)
            };
            let members = self.u_members(unit)?;
            for member in members.into_iter() {
                let tab_level = 0;
                let base_offset = 0;
                repr.push_str(&format_member(dwarf, unit, member, tab_level,
                                             verbosity, base_offset)?);
            }

            if verbosity > 0 {
                let bytesz = self.u_byte_size(unit)?;
                repr.push_str(&format!("\n    /* total size: {} */\n", bytesz));
            }
            repr.push('}');

            let alignment = match self.u_alignment(unit) {
                Ok(alignment) => Some(alignment),
                Err(Error::AlignmentAttributeNotFound) => None,
                Err(e) => return Err(e)
            };

            if let Some(alignment) = alignment {
                repr.push_str(
                    &format!(" __attribute((__aligned__({})))", alignment)
                )
            }

            repr.push(';');

            Ok(())
        });
        Ok(repr)
    }

    pub fn to_string<D>(&self, dwarf: &D) -> Result<String, Error>
    where D: BorrowableDwarf + DwarfContext {
        self.to_string_verbose(dwarf, 0)
    }

    pub(crate) fn u_byte_size(&self, unit: &GimliCU) -> Result<usize, Error> {
        unit.entry_context(&self.location(), |entry| {
            get_entry_byte_size(entry)
        })?
    }

    pub fn byte_size<D>(&self, dwarf: &D) -> Result<usize, Error>
    where D: DwarfContext {
        dwarf.unit_context(&self.location, |unit| {
            self.u_byte_size(unit)
        })?
    }

    pub(crate) fn u_alignment(&self, unit: &GimliCU) -> Result<usize, Error> {
        unit.entry_context(&self.location(), |entry| {
            get_entry_alignment(entry)
        })?
    }

    pub fn alignment<D>(&self, dwarf: &D) -> Result<usize, Error>
    where D: DwarfContext {
        dwarf.unit_context(&self.location, |unit| {
            self.u_alignment(unit)
        })?
    }
}

impl Union {
    fn location(&self) -> DwarfUnit {
        self.location
    }

    pub fn to_string_verbose<D>(&self, dwarf: &D, verbosity: u8)
    -> Result<String, Error>
    where D: DwarfContext + BorrowableDwarf {
        let mut repr = String::new();
        let _ = dwarf.unit_context(&self.location, |unit| {
            match self.u_name(dwarf, unit) {
                Ok(name) => repr.push_str(&format!("union {} {{\n", name)),
                Err(Error::NameAttributeNotFound) => repr.push_str("union {\n"),
                Err(e) => return Err(e)
            };
            let members = self.u_members(unit)?;
            for member in members.into_iter() {
                let tab_level = 0;
                let base_offset = 0;
                repr.push_str(&format_member(dwarf, unit, member, tab_level,
                                             verbosity, base_offset)?);
            }
            repr.push_str("};");
            Ok(())
        })?;
        Ok(repr)
    }

    pub fn to_string<D>(&self, dwarf: &D) -> Result<String, Error>
    where D: DwarfContext + BorrowableDwarf {
        self.to_string_verbose(dwarf, 0)
    }

    pub(crate) fn u_byte_size(&self, unit: &GimliCU) -> Result<usize, Error> {
        let entry_size = unit.entry_context(&self.location(), |entry| {
            get_entry_byte_size(entry)
        })?;

        if entry_size.is_ok() {
            return entry_size
        }

        // if there was no byte_size attribute, need to loop over all the
        // children to find the size
        // do zero-member unions exist? maybe need to err here if bytesz is zero
        let mut bytesz = 0;
        for member in self.u_members(unit)? {
            let member_type = member.u_get_type(unit)?;
            let membytesz = member_type.u_byte_size(unit)?;

            if membytesz > bytesz {
                bytesz = membytesz;
            }
        }
        Ok(bytesz)
    }

    pub fn byte_size<D>(&self, dwarf: &D) -> Result<usize, Error>
    where D: DwarfContext {
        dwarf.unit_context(&self.location(), |unit| {
            self.u_byte_size(unit)
        })?
    }
}

impl Enum {
    fn location(&self) -> DwarfUnit {
        self.location
    }

    pub fn to_string_verbose<D>(&self, dwarf: &D, verbosity: u8)
    -> Result<String, Error>
    where D: DwarfContext + BorrowableDwarf {
        let mut repr = String::new();
        let _: Result<_, Error> = dwarf.unit_context(&self.location, |unit| {
            let level = 0;
            let tab_level = 0;
            let base_offset = 0;
            repr.push_str(
                &format_type(
                    dwarf,
                    unit,
                    "".to_string(),
                    Type::Enum(*self),
                    level,
                    tab_level,
                    verbosity,
                    base_offset
                )?
            );
            repr.push_str(";");
            Ok(())
        })?;
        Ok(repr)
    }

    pub fn to_string<D>(&self, dwarf: &D)
    -> Result<String, Error>
    where D: DwarfContext + BorrowableDwarf {
        self.to_string_verbose(dwarf, 0)
    }
    /// internal byte_size on CU
    pub(crate) fn u_byte_size(&self, unit: &GimliCU) -> Result<usize, Error> {
        let entry_size = unit.entry_context(&self.location(), |entry| {
            get_entry_byte_size(entry)
        })?;

        if entry_size.is_ok() {
            return entry_size
        }

        self.u_get_type(unit)?.u_byte_size(unit)
    }

    /// The memory footprint of the enum, generally the size of the largest
    /// variant
    pub fn byte_size<D>(&self, dwarf: &D) -> Result<usize, Error>
    where D: DwarfContext {
        dwarf.unit_context(&self.location(), |unit| {
            self.u_byte_size(unit)
        })?
    }

    pub fn enumerators<D>(&self, dwarf: &D) -> Result<Vec<Enumerator>, Error>
    where D: DwarfContext + BorrowableDwarf {
        let mut enumers: Vec<Enumerator> = Vec::new();
        dwarf.unit_context(&self.location(), |unit| {
            let mut entries = {
                match unit.entries_at_offset(self.location().entry_offset) {
                    Ok(entries) => entries,
                    _ => return Err(Error::DIEError(
                       format!("Failed to seek to DIE at {:?}", self.location())
                    ))
                }
            };
            if entries.next_dfs().is_err() {
                return Err(Error::DIEError(
                    format!("Failed to find next DIE at {:?}", self.location())
                ))
            }
            while let Ok(Some((_, entry))) = entries.next_dfs() {
                if entry.tag() != gimli::DW_TAG_enumerator {
                    break;
                }
                let name = get_entry_name(dwarf, entry)?;
                if let Ok(Some(at)) = entry.attr(gimli::DW_AT_const_value) {
                    if let Some(attr_val) = at.udata_value() {
                        enumers.push(Enumerator {name, value: attr_val})
                    }
                };
            };
            Ok(())
        })??;
        Ok(enumers)
    }
}

impl Pointer {
    /// alias for get_type()
    pub fn deref<D>(&self, dwarf: &D) -> Result<Type, Error>
    where D: DwarfContext + BorrowableDwarf {
        self.get_type(dwarf)
    }

    /// internal byte_size on CU
    pub(crate) fn u_byte_size(&self, unit: &GimliCU) -> Result<usize, Error> {
        let size = unit.header.encoding().address_size as usize;
        Ok(size)
    }

    /// byte_size of a pointer will be the address size
    pub fn byte_size<D>(&self, dwarf: &D) -> Result<usize, Error>
    where D: DwarfContext {
        dwarf.unit_context(&self.location, |unit| {
            self.u_byte_size(unit)
        })?
    }
}

impl Base {
    pub(crate) fn u_byte_size(&self, unit: &GimliCU) -> Result<usize, Error> {
        unit.entry_context(&self.location(), |entry| {
            get_entry_byte_size(entry)
        })?
    }

    // if a base type doesn't have a size something is horribly wrong
    // so don't recurse on them
    pub fn byte_size<D>(&self, dwarf: &D) -> Result<usize, Error>
    where D: DwarfContext {
        dwarf.unit_context(&self.location(), |unit| {
            self.u_byte_size(unit)
        })?
    }
}

impl Typedef {
    fn location(&self) -> DwarfUnit {
        self.location
    }

    pub(crate) fn u_byte_size(&self, unit: &GimliCU) -> Result<usize, Error> {
        let inner_type = self.u_get_type(unit)?;
        inner_type.u_byte_size(unit)
    }

    pub fn byte_size<D>(&self, dwarf: &D) -> Result<usize, Error>
    where D: DwarfContext {
        dwarf.unit_context(&self.location(), |unit| {
            self.u_byte_size(unit)
        })?
    }
}

impl Const {
    fn location(&self) -> DwarfUnit {
        self.location
    }

    pub(crate) fn u_byte_size(&self, unit: &GimliCU) -> Result<usize, Error> {
        let entry_size = unit.entry_context(&self.location(), |entry| {
            get_entry_byte_size(entry)
        })?;

        if entry_size.is_ok() {
            return entry_size
        }

        let inner_type = self.u_get_type(unit)?;
        inner_type.u_byte_size(unit)
    }

    pub fn byte_size<D>(&self, dwarf: &D) -> Result<usize, Error>
    where D: DwarfContext {
        dwarf.unit_context(&self.location(), |unit| {
            self.u_byte_size(unit)
        })?
    }
}

impl Volatile {
    fn location(&self) -> DwarfUnit {
        self.location
    }

    pub(crate) fn u_byte_size(&self, unit: &GimliCU) -> Result<usize, Error> {
        let inner_type = self.u_get_type(unit)?;
        inner_type.u_byte_size(unit)
    }

    pub fn byte_size<D>(&self, dwarf: &D) -> Result<usize, Error>
    where D: DwarfContext {
        dwarf.unit_context(&self.location(), |unit| {
            self.u_byte_size(unit)
        })?
    }
}

impl Restrict {
    fn location(&self) -> DwarfUnit {
        self.location
    }

    pub(crate) fn u_byte_size(&self, unit: &GimliCU) -> Result<usize, Error> {
        let inner_type = self.u_get_type(unit)?;
        inner_type.u_byte_size(unit)
    }

    pub fn byte_size<D>(&self, dwarf: &D) -> Result<usize, Error>
    where D: DwarfContext {
        dwarf.unit_context(&self.location(), |unit| {
            self.u_byte_size(unit)
        })?
    }
}

impl Array {
    fn location(&self) -> DwarfUnit {
        self.location
    }

    pub(crate) fn u_get_bound(&self, unit: &GimliCU) -> Result<usize, Error> {
        let bound = 0;
        let mut entries = {
            match unit.entries_at_offset(self.location.entry_offset) {
                Ok(entries) => entries,
                _ => return Err(Error::DIEError(
                   format!("Failed to seek to DIE at {:?}", self.location())
                ))
            }
        };
        if entries.next_dfs().is_err() {
            return Err(Error::DIEError(
                format!("Failed to find next DIE at {:?}", self.location())
            ))
        }
        while let Ok(Some((_, entry))) = entries.next_dfs() {
            // handle subrange_type
            if entry.tag() != gimli::DW_TAG_subrange_type {
                break;
            }

            if let Ok(opt_attr) = entry.attr(gimli::DW_AT_upper_bound) {
                if let Some(attr) = opt_attr {
                    if let Some(attr_val) = attr.udata_value() {
                        return Ok((attr_val + 1) as usize)
                    }
                    match attr.value() {
                        AttributeValue::Exprloc(_) => {
                            return Err(Error::UnimplementedError("upper_bound with exprloc value".into()))
                        },
                        AttributeValue::LocationListsRef(_) => {
                            return Err(Error::UnimplementedError("upper_bound with loclist value".into()))
                        },
                        _ => {
                            return Err(Error::InvalidAttributeError)
                        }
                    }
                }
            } else {
                return Err(Error::InvalidAttributeError)
            }

            if let Ok(opt_attr) = entry.attr(gimli::DW_AT_count) {
                if let Some(attr) = opt_attr {
                    if let Some(attr_val) = attr.udata_value() {
                        return Ok(attr_val as usize)
                    }
                    match attr.value() {
                        AttributeValue::Exprloc(_) => {
                            return Err(Error::UnimplementedError("count with exprloc value".into()))
                        },
                        AttributeValue::LocationListsRef(_) => {
                            return Err(Error::UnimplementedError("count with loclist value".into()))
                        },
                        _ => {
                            return Err(Error::InvalidAttributeError)
                        }
                    }
                }
            } else {
                return Err(Error::InvalidAttributeError)
            }
        };
        Ok(bound)
    }

    /// The number of items in the array
    pub fn get_bound<D>(&self, dwarf: &D) -> Result<usize, Error>
    where D: DwarfContext {
        dwarf.unit_context(&self.location(), |unit| {
            self.u_get_bound(unit)
        })?
    }

    pub(crate) fn u_entry_size(&self, unit: &GimliCU) -> Result<usize, Error> {
        let inner_type = self.u_get_type(unit)?;
        inner_type.u_byte_size(unit)
    }

    /// The size of one array item
    pub fn entry_size<D>(&self, dwarf: &D) -> Result<usize, Error>
    where D: DwarfContext {
        dwarf.unit_context(&self.location, |unit| {
            self.u_entry_size(unit)
        })?
    }

    pub(crate) fn u_byte_size(&self, unit: &GimliCU) -> Result<usize, Error> {
        let byte_size = unit.entry_context(&self.location(), |entry| {
            get_entry_byte_size(entry)
        })?;

        if byte_size.is_ok() {
            return byte_size
        }

        let inner_size = self.u_entry_size(unit)?;
        let bound = self.u_get_bound(unit)?;
        Ok(inner_size * bound)
    }

    /// The memory footprint of the entire array
    pub fn byte_size<D>(&self, dwarf: &D) -> Result<usize, Error>
    where D: DwarfContext {
        dwarf.unit_context(&self.location(), |unit| {
            self.u_byte_size(unit)
        })?
    }
}

impl Variable {
    fn location(&self) -> DwarfUnit {
        self.location
    }

    pub(crate) fn u_byte_size(&self, unit: &GimliCU) -> Result<usize, Error> {
        let inner_type = self.u_get_type(unit)?;
        inner_type.u_byte_size(unit)
    }

    pub fn byte_size<D>(&self, dwarf: &D) -> Result<usize, Error>
    where D: DwarfContext {
        dwarf.unit_context(&self.location(), |unit| {
            self.u_byte_size(unit)
        })?
    }
}
