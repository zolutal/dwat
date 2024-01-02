//! Load and Parse DWARF debugging information
use std::{collections::HashMap, borrow::Cow};
use object::{Object, ObjectSection, ReadRef};
use fallible_iterator::FallibleIterator;
use gimli::{RunTimeEndian, DebugStrOffset};
use gimli::AttributeValue;

use crate::format::format_member;
use crate::Error;

// Abbreviations for some lengthy gimli types
type R<'a> = gimli::EndianSlice<'a, RunTimeEndian>;
type DIE<'a> = gimli::DebuggingInformationEntry<'a, 'a, R<'a>, usize>;
type CU<'a> = gimli::Unit<R<'a>, usize>;
type GimliDwarf<'a> = gimli::Dwarf<R<'a>>;

/// Represents a location of some type/tag in the DWARF information
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Location {
    pub header: usize,
    pub offset: gimli::UnitOffset,
}

/// Represents a struct type
#[derive(Clone, Copy, Debug)]
pub struct Struct {
    pub location: Location,
}

/// Represents an array type
#[derive(Clone, Copy, Debug)]
pub struct Array {
    pub location: Location,
}

/// Represents an enum type
#[derive(Clone, Copy, Debug)]
pub struct Enum {
    pub location: Location,
}

/// Represents a pointer to a type
#[derive(Clone, Copy, Debug)]
pub struct Pointer {
    pub location: Location,
}

/// Represents a type that is a function pointer prototype
#[derive(Clone, Copy, Debug)]
pub struct Subroutine {
    pub location: Location,
}

/// Represents a typedef renaming of a type
#[derive(Clone, Copy, Debug)]
pub struct Typedef {
    pub location: Location,
}

/// Represents a union type
#[derive(Clone, Copy, Debug)]
pub struct Union {
    pub location: Location,
}

/// Represents a base type, e.g. int, long, etc...
#[derive(Clone, Copy, Debug)]
pub struct Base {
    pub location: Location,
}

/// Represents the C const type-modifier
#[derive(Clone, Copy, Debug)]
pub struct Const {
    pub location: Location,
}

/// Represents the C volatile type-modifier
#[derive(Clone, Copy, Debug)]
pub struct Volatile {
    pub location: Location,
}

/// Represents the C restrict type-modifier
#[derive(Clone, Copy, Debug)]
pub struct Restrict {
    pub location: Location,
}

/// Represents the bounds of an array
#[derive(Clone, Copy, Debug)]
pub struct Subrange {
    pub location: Location,
}

/// Represents the arguments list of a Subprocedure
#[derive(Clone, Copy, Debug)]
pub struct FormalParameter {
    pub location: Location,
}

/// Represents a variable declaration
#[derive(Clone, Copy, Debug)]
pub struct Variable {
    pub location: Location,
}

/// Represents a field of a struct or union
// TODO: Maybe this should be standardized, e.g.: don't hold type_loc?
#[derive(Clone, Copy, Debug)]
pub struct Member {
    pub memb_loc: Location,
    pub type_loc: Location
}

#[derive(Clone, Copy, Debug)]
pub enum MemberType {
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
    Subrange(Subrange),
}

impl MemberType {
    pub fn byte_size(&self, dwarf: &Dwarf) -> Result<Option<usize>, Error> {
        match self {
            MemberType::Struct(struc) => {
                struc.byte_size(dwarf)
            },
            MemberType::Array(arr) => {
                arr.byte_size(dwarf)
            }
            MemberType::Pointer(ptr) => {
                ptr.byte_size(dwarf)
            }
            MemberType::Base(base) => {
                base.byte_size(dwarf)
            }
            MemberType::Union(uni) => {
                uni.byte_size(dwarf)
            }
            MemberType::Subrange(sub) => {
                sub.byte_size(dwarf)
            }
            MemberType::Enum(enu) => {
                enu.byte_size(dwarf)
            }
            MemberType::Typedef(typedef) => {
                typedef.byte_size(dwarf)
            }
            MemberType::Const(cons) => {
                cons.byte_size(dwarf)
            }
            MemberType::Volatile(vol) => {
                vol.byte_size(dwarf)
            }
            MemberType::Restrict(vol) => {
                vol.byte_size(dwarf)
            }
            // --- Unsized ---
            MemberType::Subroutine(_) => {
                Ok(None)
            }
        }
    }
}

// Try to retrieve a string from the debug_str section for a given offset
fn from_dbg_str_ref(dwarf: &Dwarf, str_ref: DebugStrOffset<usize>)
-> Option<String> {
    let dwarf = dwarf.borrow_dwarf();
    if let Ok(str_ref) = dwarf.debug_str.get_str(str_ref) {
        let str_ref = str_ref.to_string_lossy();
        return Some(str_ref.to_string());
    }
    None
}

// Try to retrieve the name attribute as a string for a DIE if one exists
fn get_entry_name(dwarf: &Dwarf, entry: &DIE) -> Option<String> {
    let mut attrs = entry.attrs();
    while let Ok(Some(attr)) = &attrs.next() {
        if attr.name() == gimli::DW_AT_name {
            match attr.value() {
                gimli::AttributeValue::String(str) => {
                    if let Ok(str) = str.to_string() {
                        return Some(str.to_string())
                    }
                }
                gimli::AttributeValue::DebugStrRef(strref) => {
                    return from_dbg_str_ref(dwarf, strref)
                }
                _ => { }
            };
        }
    }
    None
}

pub trait NamedType {
    fn location(&self) -> Location;

    // it should be safe to call this on a type that doesn't have a name
    // just to check if it has a name
    // in that case: return Ok(None)
    // Ok(Err(..)) is only returned when something went wrong seeking the
    // member's location
    fn name(&self, dwarf: &Dwarf) -> Result<Option<String>, Error> {
        dwarf.entry_context(&self.location(), |entry| {
            get_entry_name(dwarf, entry)
        })
    }
}

macro_rules! impl_named_type {
    ($type:ty) => {
        impl NamedType for $type {
            fn location(&self) -> Location {
                self.location
            }
        }
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
impl_named_type!(Subrange);
impl_named_type!(Variable);


/// This trait specifies that a type is associated with some DWARF tag
pub trait Tagged {
    fn new(location: Location) -> Self;
    fn tag() -> gimli::DwTag;
}

macro_rules! impl_tagged_type {
    ($type:ty, $tag:expr) => {
        impl Tagged for $type {
            fn new(location: Location) -> Self {
                Self { location }
            }

            fn tag() -> gimli::DwTag {
                $tag
            }
        }
    };
}

impl_tagged_type!(Struct, gimli::DW_TAG_structure_type);
impl_tagged_type!(Array, gimli::DW_TAG_array_type);
impl_tagged_type!(Enum, gimli::DW_TAG_enumeration_type);
impl_tagged_type!(Pointer, gimli::DW_TAG_pointer_type);
impl_tagged_type!(Subroutine, gimli::DW_TAG_subroutine_type);
impl_tagged_type!(Typedef, gimli::DW_TAG_typedef);
impl_tagged_type!(Union, gimli::DW_TAG_union_type);
impl_tagged_type!(Base, gimli::DW_TAG_base_type);
impl_tagged_type!(Const, gimli::DW_TAG_const_type);
impl_tagged_type!(Volatile, gimli::DW_TAG_volatile_type);
impl_tagged_type!(Restrict, gimli::DW_TAG_restrict_type);
impl_tagged_type!(Subrange, gimli::DW_TAG_subrange_type);
impl_tagged_type!(Variable, gimli::DW_TAG_variable);


/// This trait specifies that a types contains another type (singular)
pub trait InnerType {
    fn location(&self) -> Location;

    fn get_type(&self, dwarf: &Dwarf)
    -> Result<Option<MemberType>, Error> {
        dwarf.entry_context(&self.location().clone(), |entry|
        -> Result<Option<MemberType>, Error> {
            let mut attrs = entry.attrs();
            while let Ok(Some(attr)) = attrs.next() {
                if attr.name() == gimli::DW_AT_type {
                    if let AttributeValue::UnitRef(offset) = attr.value() {
                        let type_loc = Location {
                            header: self.location().header,
                            offset,
                        };
                        return dwarf.entry_context(&type_loc, |entry| {
                            Ok(Some(entry_to_type(type_loc, entry)))
                        })?
                    }
                };
            };
            Ok(None)
        })?
    }
}

macro_rules! impl_inner_type {
    ($type:ty) => {
        impl InnerType for $type {
            fn location(&self) -> Location {
                self.location
            }
        }
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


fn get_entry_bit_size(entry: &DIE) -> Option<usize> {
    let mut attrs = entry.attrs();
    while let Ok(Some(attr)) = &attrs.next() {
        if attr.name() == gimli::DW_AT_bit_size {
            if let gimli::AttributeValue::Udata(v) = attr.value() {
                return Some(v as usize);
            }
        }
    }
    None
}

fn get_entry_byte_size(entry: &DIE) -> Option<usize> {
    let mut attrs = entry.attrs();
    while let Ok(Some(attr)) = &attrs.next() {
        if attr.name() == gimli::DW_AT_byte_size {
            if let gimli::AttributeValue::Udata(v) = attr.value() {
                return Some(v as usize);
            }
        }
    }
    None
}

impl Subroutine {
    pub fn get_params(&self, dwarf: &Dwarf)
    -> Result<Vec<FormalParameter>, Error> {
        dwarf.unit_context(&self.location, |unit| {
            let mut params: Vec<FormalParameter> = vec![];
            let mut entries = {
                match unit.entries_at_offset(self.location.offset) {
                    Ok(entries) => entries,
                    _ => return params,
                }
            };
            if entries.next_dfs().is_err() {
                return params;
            }
            while let Ok(Some((_, entry))) = entries.next_dfs() {
                if entry.tag() != gimli::DW_TAG_formal_parameter {
                    break;
                }
                let location = Location {
                    header: self.location.header,
                    offset: entry.offset(),
                };
                params.push(FormalParameter { location });
            };
            params
        })
    }
}

fn entry_to_type(location: Location, entry: &DIE) -> MemberType {
    match entry.tag() {
        gimli::DW_TAG_array_type => {
            MemberType::Array(Array{location})
        },
        gimli::DW_TAG_enumeration_type => {
            MemberType::Enum(Enum{location})
        },
        gimli::DW_TAG_pointer_type => {
            MemberType::Pointer(Pointer{location})
        },
        gimli::DW_TAG_structure_type => {
            MemberType::Struct(Struct{location})
        },
        gimli::DW_TAG_subroutine_type => {
            MemberType::Subroutine(Subroutine{location})
        },
        gimli::DW_TAG_typedef => {
            MemberType::Typedef(Typedef{location})
        },
        gimli::DW_TAG_union_type => {
            MemberType::Union(Union{location})
        },
        gimli::DW_TAG_base_type => {
            MemberType::Base(Base{location})
        },
        gimli::DW_TAG_const_type => {
            MemberType::Const(Const{location})
        },
        gimli::DW_TAG_volatile_type => {
            MemberType::Volatile(Volatile{location})
        },
        gimli::DW_TAG_restrict_type => {
            MemberType::Restrict(Restrict{location})
        },
        gimli::DW_TAG_subrange_type => {
            MemberType::Subrange(Subrange{location})
        },
        _ => {
            // TODO: return an error indicating unimplemented?
            dbg!(entry.tag());
            unimplemented!("entry_to_type, unhandled dwarf type")
        }
    }
}

impl Member {
    pub fn name(&self, dwarf: &Dwarf) -> Result<Option<String>, Error> {
        dwarf.entry_context(&self.memb_loc, |entry| -> Option<String> {
            let mut attrs = entry.attrs();
            while let Ok(Some(attr)) = &attrs.next() {
                if attr.name() == gimli::DW_AT_name {
                    match attr.value() {
                        gimli::AttributeValue::String(str) => {
                            if let Ok(str) = str.to_string() {
                                return Some(str.to_string())
                            }
                        }
                        gimli::AttributeValue::DebugStrRef(strref) => {
                            return from_dbg_str_ref(dwarf, strref)
                        }
                        _ => { }
                    };
                }
            }
            None
        })
    }

    pub fn bit_size(&self, dwarf: &Dwarf) -> Result<Option<usize>, Error> {
        dwarf.entry_context(&self.memb_loc, |entry| {
            get_entry_bit_size(entry)
        })
    }

    pub fn byte_size(&self, dwarf: &Dwarf) -> Result<Option<usize>, Error> {
        let entry_byte_size = dwarf.entry_context(&self.memb_loc, |entry| {
            get_entry_byte_size(entry)
        })?;

        if let Some(bytesz) = entry_byte_size {
            Ok(Some(bytesz))
        } else {
            self.get_type(dwarf)?.byte_size(dwarf)
        }
    }

    // retrieve the type belonging to a member
    pub fn get_type(&self, dwarf: &Dwarf) -> Result<MemberType, Error> {
        dwarf.entry_context(&self.type_loc, |entry| {
            entry_to_type(self.type_loc, entry)
        })
    }

    pub fn member_location(&self, dwarf: &Dwarf) -> Result<Option<usize>, Error> {
        dwarf.entry_context(&self.memb_loc, |entry| {
            let mut attrs = entry.attrs();
            while let Ok(Some(attr)) = &attrs.next() {
                if attr.name() == gimli::DW_AT_data_member_location {
                    if let gimli::AttributeValue::Udata(v) = attr.value() {
                        return Some(v as usize);
                    }
                }
            }
            None
        })
    }

}

pub trait HasMembers {
    fn location(&self) -> Location;

    fn members(&self, dwarf: &Dwarf) -> Result<Vec<Member>, Error> {
        let mut members: Vec<Member> = Vec::new();
        dwarf.unit_context(&self.location(), |unit| {
            let mut entries = {
                match unit.entries_at_offset(self.location().offset) {
                    Ok(entries) => entries,
                    _ => return
                }
            };
            if entries.next_dfs().is_err() {
                return;
            }
            while let Ok(Some((_, entry))) = entries.next_dfs() {
                if entry.tag() != gimli::DW_TAG_member {
                    break;
                }
                let memb_loc = Location {
                    header: self.location().header,
                    offset: entry.offset(),
                };
                let mut attrs = entry.attrs();
                while let Ok(Some(attr)) = attrs.next() {
                    if attr.name() == gimli::DW_AT_type {
                        if let AttributeValue::UnitRef(offset) = attr.value() {
                            let type_loc = Location {
                                header: self.location().header,
                                offset,
                            };
                            members.push(Member { type_loc, memb_loc });
                            break;
                        }
                    };
                };
            };
        })?;
        Ok(members)
    }
}

impl HasMembers for Struct {
    fn location(&self) -> Location {
        self.location
    }
}

impl HasMembers for Union {
    fn location(&self) -> Location {
        self.location
    }
}

impl Struct {
    fn location(&self) -> Location {
        self.location
    }

    pub fn to_string_verbose(&self, dwarf: &Dwarf, verbosity: u8) -> Result<String, Error> {
        let mut repr = String::new();
        if let Some(name) =  self.name(dwarf)? {
            repr.push_str(&format!("struct {} {{\n", name));
        } else {
            repr.push_str("struct {\n");
        }
        let members = self.members(dwarf)?;
        for member in members.into_iter() {
            let tab_level = 0;
            let base_offset = 0;
            repr.push_str(&format_member(dwarf, member, tab_level,
                                         verbosity, base_offset)?);
        }
        if verbosity > 0 {
            if let Some(bytesz) = self.byte_size(dwarf)? {
                repr.push_str(&format!("\n    /* total size: {} */\n", bytesz))
            } else {
                // TODO: maybe byte_size should really just return an Error
                // instead of None if it fails... this feels awkward
                repr.push_str("\n    /* total size: ? */\n");
            }
        }
        repr.push_str("};");
        Ok(repr)
    }

    pub fn to_string(&self, dwarf: &Dwarf) -> Result<String, Error> {
        self.to_string_verbose(dwarf, 0)
    }

    pub fn byte_size(&self, dwarf: &Dwarf) -> Result<Option<usize>, Error> {
        let entry_size = dwarf.entry_context(&self.location(), |entry| {
            get_entry_byte_size(entry)
        })?;

        if let Some(entry_size) = entry_size {
            return Ok(Some(entry_size))
        }

        // This should(?) be unreachable
        Ok(None)
    }

}

impl Union {
    fn location(&self) -> Location {
        self.location
    }

    pub fn to_string_verbose(&self, dwarf: &Dwarf, verbosity: u8)
    -> Result<String, Error> {
        let mut repr = String::new();
        if let Some(name) =  self.name(dwarf)? {
            repr.push_str(&format!("union {} {{\n", name));
        } else {
            repr.push_str("union {\n");
        }
        let members = self.members(dwarf)?;
        for member in members.into_iter() {
            let tab_level = 0;
            let base_offset = 0;
            repr.push_str(&format_member(dwarf, member, tab_level,
                                         verbosity, base_offset)?);
        }
        repr.push_str("};");
        Ok(repr)
    }

    pub fn to_string(&self, dwarf: &Dwarf) -> Result<String, Error> {
        self.to_string_verbose(dwarf, 0)
    }

    fn byte_size(&self, dwarf: &Dwarf) -> Result<Option<usize>, Error> {
        let entry_size = dwarf.entry_context(&self.location(), |entry| {
            get_entry_byte_size(entry)
        })?;

        if let Some(entry_size) = entry_size {
            return Ok(Some(entry_size));
        }

        // if there was no byte_size attribute, need to loop over all the
        // children to find the size
        // do zero-member unions exist? maybe need to err here if bytesz is zero
        let mut bytesz = 0;
        for member in self.members(dwarf)? {
            let member_type = member.get_type(dwarf)?;
            if let Some(membytesz) = member_type.byte_size(dwarf)? {
                if membytesz > bytesz {
                    bytesz = membytesz;
                }
            } else {
                // maybe we should err here instead of returning None...
                return Ok(None);
            }
        }
        Ok(Some(bytesz))
    }
}

impl Enum {
    fn location(&self) -> Location {
        self.location
    }

    fn byte_size(&self, dwarf: &Dwarf) -> Result<Option<usize>, Error> {
        let entry_size = dwarf.entry_context(&self.location(), |entry| {
            get_entry_byte_size(entry)
        })?;

        if let Some(entry_size) = entry_size {
            return Ok(Some(entry_size));
        }

        if let Some(inner_type) = self.get_type(dwarf)? {
            return inner_type.byte_size(dwarf);
        };
        Ok(None)
    }
}

impl Pointer {
    /// alias for get_type()
    pub fn deref(&self, dwarf: &Dwarf) -> Result<Option<MemberType>, Error> {
        self.get_type(dwarf)
    }

    // special case of byte_size, pointer is of size address_size
    pub fn byte_size(&self, dwarf: &Dwarf) -> Result<Option<usize>, Error> {
        let size = dwarf.unit_context(&self.location, |unit| {
            unit.header.encoding().address_size as usize
        })?;
        Ok(Some(size))
    }
}

impl Base {
    // if a base type doesn't have a size something is horribly wrong
    // so don't recurse on them
    pub fn byte_size(&self, dwarf: &Dwarf) -> Result<Option<usize>, Error> {
        dwarf.entry_context(&self.location(), |entry| {
            get_entry_byte_size(entry)
        })
    }
}

impl Typedef {
    fn location(&self) -> Location {
        self.location
    }

    pub fn byte_size(&self, dwarf: &Dwarf) -> Result<Option<usize>, Error> {
        let entry_size = dwarf.entry_context(&self.location(), |entry| {
            get_entry_byte_size(entry)
        })?;

        if let Some(entry_size) = entry_size {
            return Ok(Some(entry_size));
        }

        if let Some(inner_type) = self.get_type(dwarf)? {
            if let Some(inner_size) = inner_type.byte_size(dwarf)? {
                return Ok(Some(inner_size));
            }
        };

        Ok(None)
    }
}

impl Const {
    fn location(&self) -> Location {
        self.location
    }

    pub fn byte_size(&self, dwarf: &Dwarf) -> Result<Option<usize>, Error> {
        let entry_size = dwarf.entry_context(&self.location(), |entry| {
            get_entry_byte_size(entry)
        })?;

        if let Some(entry_size) = entry_size {
            return Ok(Some(entry_size));
        }

        if let Some(inner_type) = self.get_type(dwarf)? {
            if let Some(inner_size) = inner_type.byte_size(dwarf)? {
                return Ok(Some(inner_size));
            }
        };

        Ok(None)
    }
}

impl Volatile {
    fn location(&self) -> Location {
        self.location
    }

    pub fn byte_size(&self, dwarf: &Dwarf) -> Result<Option<usize>, Error> {
        let entry_size = dwarf.entry_context(&self.location(), |entry| {
            get_entry_byte_size(entry)
        })?;

        if let Some(entry_size) = entry_size {
            return Ok(Some(entry_size));
        }

        if let Some(inner_type) = self.get_type(dwarf)? {
            if let Some(inner_size) = inner_type.byte_size(dwarf)? {
                return Ok(Some(inner_size));
            }
        };

        Ok(None)
    }
}

impl Restrict {
    fn location(&self) -> Location {
        self.location
    }

    pub fn byte_size(&self, dwarf: &Dwarf) -> Result<Option<usize>, Error> {
        let entry_size = dwarf.entry_context(&self.location(), |entry| {
            get_entry_byte_size(entry)
        })?;

        if let Some(entry_size) = entry_size {
            return Ok(Some(entry_size));
        }

        if let Some(inner_type) = self.get_type(dwarf)? {
            if let Some(inner_size) = inner_type.byte_size(dwarf)? {
                return Ok(Some(inner_size));
            }
        };

        Ok(None)
    }
}

impl Subrange {
    fn location(&self) -> Location {
        self.location
    }

    pub fn byte_size(&self, dwarf: &Dwarf) -> Result<Option<usize>, Error> {
        let entry_size = dwarf.entry_context(&self.location(), |entry| {
            get_entry_byte_size(entry)
        })?;

        if let Some(entry_size) = entry_size {
            return Ok(Some(entry_size));
        }

        Ok(None)
    }
}

impl Array {
    fn location(&self) -> Location {
        self.location
    }

    pub fn get_bound(&self, dwarf: &Dwarf) -> Result<usize, Error> {
        dwarf.unit_context(&self.location, |unit| {
            let bound = 0;
            let mut entries = {
                match unit.entries_at_offset(self.location.offset) {
                    Ok(entries) => entries,
                    _ => return bound,
                }
            };
            if entries.next_dfs().is_err() {
                return bound;
            }
            while let Ok(Some((_, entry))) = entries.next_dfs() {
                // handle subrange_type
                if entry.tag() != gimli::DW_TAG_subrange_type {
                    break;
                }
                let mut attrs = entry.attrs();
                while let Ok(Some(attr)) = attrs.next() {
                    if attr.name() == gimli::DW_AT_upper_bound {
                        if let Some(val) = attr.udata_value() {
                            return (val + 1) as usize;
                        }
                    };
                    if attr.name() == gimli::DW_AT_count {
                        if let Some(val) = attr.udata_value() {
                            return val as usize;
                        }
                    };
                };
            };
            bound
        })
    }

    // another weird case of byte_size, we need to get the bound and multiply
    // the bound by the byte_size of the child type if there is no byte size
    // attribute
    pub fn byte_size(&self, dwarf: &Dwarf) -> Result<Option<usize>, Error> {
        let entry_size = dwarf.entry_context(&self.location(), |entry| {
            get_entry_byte_size(entry)
        })?;

        if let Some(entry_size) = entry_size {
            return Ok(Some(entry_size));
        }

        if let Some(inner_type) = self.get_type(dwarf)? {
            let bound = self.get_bound(dwarf)?;
            if let Some(inner_size) = inner_type.byte_size(dwarf)? {
                return Ok(Some(inner_size * bound));
            }
        };

        Ok(None)
    }
}

/// Represents DWARF data
pub struct Dwarf<'a> {
    dwarf_cow: gimli::Dwarf<Cow<'a, [u8]>>,
    endianness: RunTimeEndian
}

impl<'a> Dwarf<'a> {
    pub fn parse(data: impl ReadRef<'a>) -> Result<Self, Error> {
        let object = object::File::parse(data)?;

        let endianness = if object.is_little_endian() {
            gimli::RunTimeEndian::Little
        } else {
            gimli::RunTimeEndian::Big
        };

        let load_section = |id: gimli::SectionId|
        -> Result<Cow<[u8]>, gimli::Error> {
            match object.section_by_name(id.name()) {
                Some(ref section) => Ok(section
                    .uncompressed_data()
                    .unwrap_or(Cow::Borrowed(&[][..]))),
                None => Ok(Cow::Borrowed(&[][..])),
            }
        };

        // Load all of the sections.
        let dwarf_cow = gimli::Dwarf::load(&load_section).unwrap();

        Ok(Self{dwarf_cow, endianness})
    }

    fn borrow_dwarf(&self) -> GimliDwarf {
        let borrow_section: &dyn for<'b> Fn(&'b Cow<[u8]>,
        ) -> gimli::EndianSlice<'b, gimli::RunTimeEndian> =
        &|section| gimli::EndianSlice::new(section, self.endianness);

        self.dwarf_cow.borrow(borrow_section)
    }

    fn entry_context<F,R>(&self, loc: &Location, f: F) -> Result<R, Error>
    where F: FnOnce(&DIE) -> R {
        self.unit_context(loc, |unit| -> Result<R, Error> {
            let entry = match unit.entry(loc.offset) {
                Ok(entry) => entry,
                Err(_) => {
                    return Err(
                        Error::DIEError(
                            format!("Failed to find DIE at location: {loc:?}")
                        )
                    );
                }
            };
            Ok(f(&entry))
        })?
    }

    fn unit_context<F,R>(&self, loc: &Location, f: F) -> Result<R, Error>
    where F: FnOnce(&CU) -> R {
        let dwarf = self.borrow_dwarf();
        let mut unit_headers = dwarf.units();
        let unit = if let Ok(Some(header)) = unit_headers.nth(loc.header) {
            if let Ok(unit) = dwarf.unit(header) {
                unit
            } else {
                return Err(Error::CUError(
                    format!("Failed to find CU at location: {:?}", loc)
                ));
            }
        } else {
            return Err(Error::CUError(
                format!("Failed to find CU header at location: {:?}", loc)
            ));
        };
        Ok(f(&unit))
    }

    fn for_each_item<T: Tagged, F>(&self, mut f: F)
    -> Result<(), Error>
    where F: FnMut(&DIE, Location) -> bool {
        let dwarf = self.borrow_dwarf();
        let mut header_idx = 0;
        let mut unit_headers = dwarf.units();
        while let Ok(Some(header)) = unit_headers.next() {
            let unit = match dwarf.unit(header) {
                Ok(unit) => unit,
                Err(_) => continue
            };
            let mut entries = unit.entries();
            'entries:
            while let Ok(Some((_delta_depth, entry))) = entries.next_dfs() {
                if entry.tag() != T::tag() {
                    continue;
                }

                let mut attrs = entry.attrs();
                while let Ok(Some(attr)) = attrs.next() {
                    if attr.name() == gimli::DW_AT_declaration {
                        continue 'entries
                    }

                    let location = Location {
                        header: header_idx,
                        offset: entry.offset(),
                    };

                    // return if function returns true
                    if f(entry, location) {
                        return Ok(())
                    }
                }
            }
            header_idx += 1;
        }
        Ok(())
    }

    pub fn lookup_item<T: Tagged>(&mut self, name: String)
    -> Result<Option<T>, Error> {
        let mut item: Option<T> = None;
        self.for_each_item::<T, _>(|entry, loc| {
            if let Some(entry_name) = get_entry_name(self, entry) {
                if name == entry_name {
                    item = Some(T::new(loc));
                    return true;
                }
            }
            false
        })?;
        Ok(item)
    }

    pub fn get_named_items_map<T: Tagged>(&self)
    -> Result<HashMap<String, T>, Error> {
        let mut item_locations: HashMap<String, T> = HashMap::new();
        self.for_each_item::<T, _>(|entry, loc| {
            if let Some(name) = get_entry_name(self, entry) {
                let typ = T::new(loc);
                item_locations.insert(name, typ);
            }
            false
        })?;
        Ok(item_locations)
    }

    pub fn get_named_items<T: Tagged>(&self)
    -> Result<Vec<(String, T)>, Error> {
        let mut items: Vec<(String, T)> = Vec::new();
        self.for_each_item::<T, _>(|entry, loc| {
            if let Some(name) = get_entry_name(self, entry) {
                let typ = T::new(loc);
                items.push((name, typ));
            }
            false
        })?;
        Ok(items)
    }
}
