//! Interfaces representing DWARF type information

use gimli::{RunTimeEndian, DebugStrOffset};
use gimli::AttributeValue;

use crate::dwarf::borrowable_dwarf::BorrowableDwarf;
use crate::types::unit_has_members::UnitHasMembers;
use crate::types::unit_inner_type::UnitInnerType;
use crate::types::unit_name_type::UnitNamedType;
use crate::format::format_member;
use crate::dwarf::DwarfContext;
use crate::Error;

// Abbreviations for some lengthy gimli types
pub(crate) type R<'a> = gimli::EndianSlice<'a, RunTimeEndian>;
pub(crate) type DIE<'a> = gimli::DebuggingInformationEntry<'a,'a,R<'a>,usize>;
pub(crate) type CU<'a> = gimli::Unit<R<'a>, usize>;
pub(crate) type GimliDwarf<'a> = gimli::Dwarf<R<'a>>;

/// Represents a location of some type/tag in the DWARF information
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Location {
    pub header: gimli::DebugInfoOffset,
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
#[derive(Clone, Copy, Debug)]
pub struct Member {
    pub location: Location,
}

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
}

impl Type {
    fn u_byte_size(&self, unit: &CU) -> Result<usize, Error> {
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

// Try to retrieve the name attribute as a string for a DIE if one exists
pub(crate) fn get_entry_name<D>(dwarf: &D, entry: &DIE) -> Option<String>
where D: DwarfContext + BorrowableDwarf {
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

// // Try to retrieve a string from the debug_str section for a given offset
// pub(crate) fn owned_from_dbg_str_ref(dwarf: &OwnedDwarf, str_ref: DebugStrOffset<usize>)
// -> Option<String> {
//     dwarf.borrow_dwarf(|dwarf| {
//         if let Ok(str_ref) = dwarf.debug_str.get_str(str_ref) {
//             let str_ref = str_ref.to_string_lossy();
//             return Some(str_ref.to_string());
//         }
//         None
//     })
// }
//
// // Try to retrieve the name attribute as a string for a DIE if one exists
// pub(crate) fn owned_get_entry_name(dwarf: &OwnedDwarf, entry: &DIE) -> Option<String> {
//     let mut attrs = entry.attrs();
//     while let Ok(Some(attr)) = &attrs.next() {
//         if attr.name() == gimli::DW_AT_name {
//             match attr.value() {
//                 gimli::AttributeValue::String(str) => {
//                     if let Ok(str) = str.to_string() {
//                         return Some(str.to_string())
//                     }
//                 }
//                 gimli::AttributeValue::DebugStrRef(strref) => {
//                     return owned_from_dbg_str_ref(dwarf, strref)
//                 }
//                 _ => { }
//             };
//         }
//     }
//     None
// }

/// force UnitNamedType trait to be private
pub(crate) mod unit_name_type {
    use crate::types::*;
    use crate::Error;

    /// Public crate trait backing NamedType
    pub trait UnitNamedType {
        fn location(&self) -> Location;

        fn u_name<D>(&self, dwarf: &D, unit: &CU) -> Result<String, Error>
        where D: DwarfContext + BorrowableDwarf {
            if let Some(name) = unit.entry_context(&self.location(), |entry| {
                get_entry_name(dwarf, entry)
            })? {
                Ok(name)
            } else {
                Err(Error::NameAttributeNotFound)
            }
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
            fn location(&self) -> Location {
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
impl_tagged_type!(Variable, gimli::DW_TAG_variable);


/// force UnitInnerType trait to be private
pub(crate) mod unit_inner_type {
    use crate::types::*;
    use crate::Error;

    pub trait UnitInnerType {
        fn location(&self) -> Location;

        fn u_get_type(&self, unit: &CU) -> Result<Type, Error> {
            unit.entry_context(&self.location().clone(), |entry|
            -> Result<Type, Error> {
                let mut attrs = entry.attrs();
                while let Ok(Some(attr)) = attrs.next() {
                    if attr.name() == gimli::DW_AT_type {
                        if let AttributeValue::UnitRef(offset) = attr.value() {
                            let type_loc = Location {
                                header: self.location().header,
                                offset,
                            };
                            return unit.entry_context(&type_loc, |entry| {
                                entry_to_type(type_loc, entry)
                            })?
                        }
                    };
                };
                Err(Error::TypeAttributeNotFound)
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
            fn location(&self) -> Location {
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


fn get_entry_bit_size(entry: &DIE) -> Option<usize> {
    let mut attrs = entry.attrs();
    while let Ok(Some(attr)) = &attrs.next() {
        if attr.name() == gimli::DW_AT_bit_size {
            return attr.udata_value().map(|v| v as usize)
        }
    }
    None
}

fn get_entry_byte_size(entry: &DIE) -> Option<usize> {
    let mut attrs = entry.attrs();
    while let Ok(Some(attr)) = &attrs.next() {
        if attr.name() == gimli::DW_AT_byte_size {
            return attr.udata_value().map(|v| v as usize)
        }
    }
    None
}

// Try to retrieve the alignment attribute if one exists, alignment was added
// in DWARF 5 but gcc will inlcude it even for -gdwarf-4
fn get_entry_alignment(entry: &DIE) -> Option<usize> {
    let mut attrs = entry.attrs();
    while let Ok(Some(attr)) = &attrs.next() {
        if attr.name() == gimli::DW_AT_alignment {
            return attr.udata_value().map(|v| v as usize)
        }
    }
    None
}


impl Subroutine {
    fn location(&self) -> Location {
        self.location
    }

    pub(crate) fn u_get_params(&self, unit: &CU)
    -> Result<Vec<FormalParameter>, Error> {
        let mut params: Vec<FormalParameter> = vec![];
        let mut entries = {
            match unit.entries_at_offset(self.location.offset) {
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
            let location = Location {
                header: self.location.header,
                offset: entry.offset(),
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

fn entry_to_type(location: Location, entry: &DIE) -> Result<Type, Error> {
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
    pub(crate) fn u_bit_size(&self, unit: &CU) -> Result<usize, Error> {
        let bit_size = unit.entry_context(&self.location, |entry| {
            get_entry_bit_size(entry)
        })?;
        if let Some(bit_size) = bit_size {
            Ok(bit_size)
        } else {
            Err(Error::BitSizeAttributeNotFound)
        }
    }

    pub fn bit_size<D>(&self, dwarf: &D) -> Result<usize, Error>
    where D: DwarfContext {
        dwarf.unit_context(&self.location, |unit| {
            self.u_bit_size(unit)
        })?
    }

    pub(crate) fn u_byte_size(&self, unit: &CU) -> Result<usize, Error> {
        let inner = self.u_get_type(unit)?;
        inner.u_byte_size(unit)
    }

    pub fn byte_size<D>(&self, dwarf: &D) -> Result<usize, Error>
    where D: DwarfContext {
        dwarf.unit_context(&self.location, |unit| {
            self.u_byte_size(unit)
        })?
    }

    pub(crate) fn u_member_location(&self, unit: &CU) -> Result<usize, Error> {
        let member_location = unit.entry_context(&self.location, |entry| {
            let mut attrs = entry.attrs();
            while let Ok(Some(attr)) = &attrs.next() {
                if attr.name() == gimli::DW_AT_data_member_location {
                    if let gimli::AttributeValue::Udata(v) = attr.value() {
                        return Some(v as usize);
                    }
                }
            }
            None
        })?;

        if let Some(member_location) = member_location {
            Ok(member_location)
        } else {
            Err(Error::MemberLocationAttributeNotFound)
        }
    }

    /// The byte offset of the member from the start of the datatype
    pub fn member_location<D>(&self, dwarf: &D) -> Result<usize, Error>
    where D: DwarfContext {
        dwarf.unit_context(&self.location, |unit| {
            self.u_member_location(unit)
        })?
    }

    pub(crate) fn u_offset(&self, unit: &CU) -> Result<usize, Error> {
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
        fn location(&self) -> Location;

        fn u_members(&self, unit: &CU) -> Result<Vec<Member>, Error> {
            let mut members: Vec<Member> = Vec::new();
            let mut entries = {
                match unit.entries_at_offset(self.location().offset) {
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
                let location = Location {
                    header: self.location().header,
                    offset: entry.offset(),
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
    fn location(&self) -> Location { self.location }
}
impl unit_has_members::UnitHasMembers for Union {
    fn location(&self) -> Location { self.location }
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
    fn location(&self) -> Location {
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

    pub(crate) fn u_byte_size(&self, unit: &CU) -> Result<usize, Error> {
        let entry_size = unit.entry_context(&self.location(), |entry| {
            get_entry_byte_size(entry)
        })?;

        if let Some(entry_size) = entry_size {
            return Ok(entry_size)
        }

        // This should(?) be unreachable
        Err(Error::ByteSizeAttributeNotFound)
    }

    pub fn byte_size<D>(&self, dwarf: &D) -> Result<usize, Error>
    where D: DwarfContext {
        dwarf.unit_context(&self.location, |unit| {
            self.u_byte_size(unit)
        })?
    }

    pub(crate) fn u_alignment(&self, unit: &CU) -> Result<usize, Error> {
        let alignment = unit.entry_context(&self.location(), |entry| {
            get_entry_alignment(entry)
        })?;

        if let Some(alignment) = alignment {
            return Ok(alignment)
        }

        Err(Error::AlignmentAttributeNotFound)
    }

    pub fn alignment<D>(&self, dwarf: &D) -> Result<usize, Error>
    where D: DwarfContext {
        dwarf.unit_context(&self.location, |unit| {
            self.u_alignment(unit)
        })?
    }
}

impl Union {
    fn location(&self) -> Location {
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

    pub(crate) fn u_byte_size(&self, unit: &CU) -> Result<usize, Error> {
        let entry_size = unit.entry_context(&self.location(), |entry| {
            get_entry_byte_size(entry)
        })?;

        if let Some(entry_size) = entry_size {
            return Ok(entry_size);
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
    fn location(&self) -> Location {
        self.location
    }

    /// internal byte_size on CU
    pub(crate) fn u_byte_size(&self, unit: &CU) -> Result<usize, Error> {
        let entry_size = unit.entry_context(&self.location(), |entry| {
            get_entry_byte_size(entry)
        })?;

        if let Some(entry_size) = entry_size {
            return Ok(entry_size);
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
}

impl Pointer {
    /// alias for get_type()
    pub fn deref<D>(&self, dwarf: &D) -> Result<Type, Error>
    where D: DwarfContext + BorrowableDwarf {
        self.get_type(dwarf)
    }

    /// internal byte_size on CU
    pub(crate) fn u_byte_size(&self, unit: &CU) -> Result<usize, Error> {
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
    pub(crate) fn u_byte_size(&self, unit: &CU) -> Result<usize, Error> {
        let entry_size = unit.entry_context(&self.location(), |entry| {
            get_entry_byte_size(entry)
        })?;

        if let Some(entry_size) = entry_size {
            Ok(entry_size)
        } else {
            Err(Error::ByteSizeAttributeNotFound)
        }
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
    fn location(&self) -> Location {
        self.location
    }

    pub(crate) fn u_byte_size(&self, unit: &CU) -> Result<usize, Error> {
        let entry_size = unit.entry_context(&self.location(), |entry| {
            get_entry_byte_size(entry)
        })?;

        if let Some(entry_size) = entry_size {
            return Ok(entry_size);
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

impl Const {
    fn location(&self) -> Location {
        self.location
    }

    pub(crate) fn u_byte_size(&self, unit: &CU) -> Result<usize, Error> {
        let entry_size = unit.entry_context(&self.location(), |entry| {
            get_entry_byte_size(entry)
        })?;

        if let Some(entry_size) = entry_size {
            return Ok(entry_size);
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
    fn location(&self) -> Location {
        self.location
    }

    pub(crate) fn u_byte_size(&self, unit: &CU) -> Result<usize, Error> {
        let entry_size = unit.entry_context(&self.location(), |entry| {
            get_entry_byte_size(entry)
        })?;

        if let Some(entry_size) = entry_size {
            return Ok(entry_size);
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

impl Restrict {
    fn location(&self) -> Location {
        self.location
    }

    pub(crate) fn u_byte_size(&self, unit: &CU) -> Result<usize, Error> {
        let entry_size = unit.entry_context(&self.location(), |entry| {
            get_entry_byte_size(entry)
        })?;

        if let Some(entry_size) = entry_size {
            return Ok(entry_size);
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

impl Array {
    fn location(&self) -> Location {
        self.location
    }

    pub(crate) fn u_get_bound(&self, unit: &CU) -> Result<usize, Error> {
        let bound = 0;
        let mut entries = {
            match unit.entries_at_offset(self.location.offset) {
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
            let mut attrs = entry.attrs();
            while let Ok(Some(attr)) = attrs.next() {
                if attr.name() == gimli::DW_AT_upper_bound {
                    if let Some(val) = attr.udata_value() {
                        return Ok((val + 1) as usize);
                    }
                };
                if attr.name() == gimli::DW_AT_count {
                    if let Some(val) = attr.udata_value() {
                        return Ok(val as usize);
                    }
                };
            };
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

    pub(crate) fn u_entry_size(&self, unit: &CU) -> Result<usize, Error> {
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

    pub(crate) fn u_byte_size(&self, unit: &CU) -> Result<usize, Error> {
        let byte_size = unit.entry_context(&self.location(), |entry| {
            get_entry_byte_size(entry)
        })?;

        if let Some(byte_size) = byte_size {
            return Ok(byte_size);
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
