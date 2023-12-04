use std::{collections::BTreeMap, borrow::Cow};
use object::{Object, ObjectSection, ReadRef};
use fallible_iterator::FallibleIterator;
use gimli::{RunTimeEndian, DebugStrOffset};
use gimli::AttributeValue;

pub mod format;

pub mod prelude {
    pub use super::NamedType;
    pub use super::TypeModifier;
}

// Abbreviations for some lengthy gimli types
type R<'a> = gimli::EndianSlice<'a, RunTimeEndian>;
type DIE<'a> = gimli::DebuggingInformationEntry<'a, 'a, R<'a>, usize>;
type CU<'a> = gimli::Unit<R<'a>, usize>;
type GimliDwarf<'a> = gimli::Dwarf<R<'a>>;

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

type Error = DwatError;


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Location {
    pub header: usize,
    pub offset: gimli::UnitOffset,
}

#[derive(Clone, Copy, Debug)]
pub struct Struct {
    pub location: Location,
}

#[derive(Clone, Copy, Debug)]
pub struct Array {
    pub location: Location,
}

#[derive(Clone, Copy, Debug)]
pub struct Enum {
    pub location: Location,
}

#[derive(Clone, Copy, Debug)]
pub struct Pointer {
    pub location: Location,
}

#[derive(Clone, Copy, Debug)]
pub struct Subroutine {
    pub location: Location,
}

#[derive(Clone, Copy, Debug)]
pub struct Typedef {
    pub location: Location,
}

#[derive(Clone, Copy, Debug)]
pub struct Union {
    pub location: Location,
}

#[derive(Clone, Copy, Debug)]
pub struct Base {
    pub location: Location,
}

#[derive(Clone, Copy, Debug)]
pub struct Const {
    pub location: Location,
}

#[derive(Clone, Copy, Debug)]
pub struct Volatile {
    pub location: Location,
}

#[derive(Clone, Copy, Debug)]
pub struct Subrange {
    pub location: Location,
}

#[derive(Clone, Copy, Debug)]
pub struct FormalParameter {
    pub location: Location,
}

#[derive(Clone, Copy, Debug)]
pub struct Variable {
    pub location: Location,
}

/// A member is a field of a struct that has an unknown type
/// members are found via a DW_TAG_member, they represent an intermediate state
/// where we can know the location of the member type but haven't parsed it yet
/// TODO: Maybe this should be standardized, e.g.: don't hold type_loc
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
    Subrange(Subrange),
}

/// Try to retrieve a string from the debug_str section for a given offset
fn from_dbg_str_ref(dwarf: &Dwarf, str_ref: DebugStrOffset<usize>)
-> Option<String> {
    let dwarf = dwarf.borrow_dwarf();
    if let Ok(str_ref) = dwarf.debug_str.get_str(str_ref) {
        let str_ref = str_ref.to_string_lossy();
        return Some(str_ref.to_string());
    }
    None
}

/// Try to retrieve the name attribute as a string for a DIE if one exists
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
    fn name(&self, dwarf: &Dwarf) -> Option<String> {
        dwarf.entry_context(&self.location(), |entry| {
            let name = get_entry_name(dwarf, entry);
            name
        }).unwrap()
    }
}

impl NamedType for Struct {
    fn location(&self) -> Location {
        self.location
    }
}

impl NamedType for Array {
    fn location(&self) -> Location {
        self.location
    }
}

impl NamedType for Enum {
    fn location(&self) -> Location {
        self.location
    }
}

impl NamedType for Pointer {
    fn location(&self) -> Location {
        self.location
    }
}

impl NamedType for Subroutine {
    fn location(&self) -> Location {
        self.location
    }
}

impl NamedType for Typedef {
    fn location(&self) -> Location {
        self.location
    }
}

impl NamedType for Union {
    fn location(&self) -> Location {
        self.location
    }
}

impl NamedType for Base {
    fn location(&self) -> Location {
        self.location
    }
}

impl NamedType for Const {
    fn location(&self) -> Location {
        self.location
    }
}

impl NamedType for Volatile {
    fn location(&self) -> Location {
        self.location
    }
}

impl NamedType for Subrange {
    fn location(&self) -> Location {
        self.location
    }
}

impl NamedType for Variable {
    fn location(&self) -> Location {
        self.location
    }
}

pub trait Tagged {
    fn new(location: Location) -> Self;
    fn tag() -> gimli::DwTag;
}

impl Tagged for Struct {
    fn new(location: Location) -> Self {
        Self { location }
    }

    fn tag() -> gimli::DwTag {
        gimli::DW_TAG_structure_type
    }
}

impl Tagged for Array {
    fn new(location: Location) -> Self {
        Self { location }
    }

    fn tag() -> gimli::DwTag {
        gimli::DW_TAG_array_type
    }
}

impl Tagged for Enum {
    fn new(location: Location) -> Self {
        Self { location }
    }

    fn tag() -> gimli::DwTag {
        gimli::DW_TAG_enumeration_type
    }
}

impl Tagged for Pointer {
    fn new(location: Location) -> Self {
        Self { location }
    }

    fn tag() -> gimli::DwTag {
        gimli::DW_TAG_pointer_type
    }
}

impl Tagged for Subroutine {
    fn new(location: Location) -> Self {
        Self { location }
    }

    fn tag() -> gimli::DwTag {
        gimli::DW_TAG_subroutine_type
    }
}

impl Tagged for Typedef {
    fn new(location: Location) -> Self {
        Self { location }
    }

    fn tag() -> gimli::DwTag {
        gimli::DW_TAG_typedef
    }
}

impl Tagged for Union {
    fn new(location: Location) -> Self {
        Self { location }
    }

    fn tag() -> gimli::DwTag {
        gimli::DW_TAG_union_type
    }
}

impl Tagged for Base {
    fn new(location: Location) -> Self {
        Self { location }
    }

    fn tag() -> gimli::DwTag {
        gimli::DW_TAG_base_type
    }
}

impl Tagged for Const {
    fn new(location: Location) -> Self {
        Self { location }
    }

    fn tag() -> gimli::DwTag {
        gimli::DW_TAG_const_type
    }
}

impl Tagged for Volatile {
    fn new(location: Location) -> Self {
        Self { location }
    }

    fn tag() -> gimli::DwTag {
        gimli::DW_TAG_volatile_type
    }
}

impl Tagged for Subrange {
    fn new(location: Location) -> Self {
        Self { location }
    }

    fn tag() -> gimli::DwTag {
        gimli::DW_TAG_subrange_type
    }
}

impl Tagged for Variable {
    fn new(location: Location) -> Self {
        Self { location }
    }

    fn tag() -> gimli::DwTag {
        gimli::DW_TAG_variable
    }
}

// Types which contain another type
pub trait TypeModifier {
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
                            return Ok(Some(entry_to_type(type_loc, entry)));
                        })?
                    }
                };
            };
            Ok(None)
        })?
    }
}

impl TypeModifier for Const {
    fn location(&self) -> Location {
        self.location
    }
}

impl TypeModifier for Volatile {
    fn location(&self) -> Location {
        self.location
    }
}

impl TypeModifier for FormalParameter {
    fn location(&self) -> Location {
        self.location
    }
}

impl Subroutine {
    pub fn get_params(&self, dwarf: &Dwarf)
    -> Result<Vec<FormalParameter>, Error> {
        dwarf.unit_context(&self.location.clone(), |unit| {
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

fn entry_to_type (location: Location, entry: &DIE) -> MemberType {
    match entry.tag() {
        gimli::DW_TAG_array_type => {
            return MemberType::Array(Array{location});
        },
        gimli::DW_TAG_enumeration_type => {
            return MemberType::Enum(Enum{location});
        },
        gimli::DW_TAG_pointer_type => {
            return MemberType::Pointer(Pointer{location});
        },
        gimli::DW_TAG_structure_type => {
            return MemberType::Struct(Struct{location});
        },
        gimli::DW_TAG_subroutine_type => {
            return MemberType::Subroutine(Subroutine{location});
        },
        gimli::DW_TAG_typedef => {
            return MemberType::Typedef(Typedef{location});
        },
        gimli::DW_TAG_union_type => {
            return MemberType::Union(Union{location});
        },
        gimli::DW_TAG_base_type => {
            return MemberType::Base(Base{location});
        },
        gimli::DW_TAG_const_type => {
            return MemberType::Const(Const{location});
        },
        gimli::DW_TAG_volatile_type => {
            return MemberType::Volatile(Volatile{location});
        },
        gimli::DW_TAG_subrange_type => {
            return MemberType::Subrange(Subrange{location});
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

    pub fn get_bit_size(&self, dwarf: &Dwarf)
    -> Result<Option<usize>, Error> {
        dwarf.entry_context(&self.memb_loc, |entry| {
            let mut attrs = entry.attrs();
            let mut bitsz: Option<usize> = None;
            while let Ok(Some(attr)) = &attrs.next() {
                if attr.name() == gimli::DW_AT_bit_size {
                    if let gimli::AttributeValue::Udata(v) = attr.value() {
                        bitsz = Some(v as usize);
                    }
                    break
                }
            }
            bitsz
        })
    }

    // retrieve the type belonging to a member
    pub fn get_type(&self, dwarf: &Dwarf) -> Result<MemberType, Error> {
        dwarf.entry_context(&self.type_loc, |entry| {
                return entry_to_type(self.type_loc, entry)
        })
    }
}

impl Struct {
    pub fn members(&self, dwarf: &Dwarf) -> Vec<Member> {
        let members = dwarf.unit_context(&self.location.clone(), |unit| {
            let mut members: Vec<Member> = vec![];
            let mut entries = {
                match unit.entries_at_offset(self.location.offset) {
                    Ok(entries) => entries,
                    _ => return members,
                }
            };
            if entries.next_dfs().is_err() {
                return members;
            }
            while let Ok(Some((_, entry))) = entries.next_dfs() {
                if entry.tag() != gimli::DW_TAG_member {
                    break;
                }
                let memb_loc = Location {
                    header: self.location.header,
                    offset: entry.offset(),
                };
                let mut attrs = entry.attrs();
                while let Ok(Some(attr)) = attrs.next() {
                    if attr.name() == gimli::DW_AT_type {
                        if let AttributeValue::UnitRef(offset) = attr.value() {
                            let type_loc = Location {
                                header: self.location.header,
                                offset,
                            };
                            members.push(Member { type_loc, memb_loc });
                            break;
                        }
                    };
                };
            };
            members
        });
        members.unwrap()
    }
}

impl Variable {
    pub fn get_type(&self, dwarf: &Dwarf)
    -> Result<Option<MemberType>, Error> {
        dwarf.entry_context(&self.location.clone(), |entry|
         -> Result<Option<MemberType>, Error> {
            let mut attrs = entry.attrs();
            while let Ok(Some(attr)) = attrs.next() {
                if attr.name() == gimli::DW_AT_type {
                    if let AttributeValue::UnitRef(offset) = attr.value() {
                        let type_loc = Location {
                            header: self.location.header,
                            offset,
                        };
                        return dwarf.entry_context(&type_loc, |entry| {
                            return Ok(Some(entry_to_type(type_loc, entry)));
                        })?
                    }
                };
            };
            Ok(None)
        })?
    }
}

impl Union {
    pub fn members(&self, dwarf: &Dwarf) -> Vec<Member> {
        let members = dwarf.unit_context(&self.location.clone(), |unit| {
            let mut members: Vec<Member> = vec![];
            let mut entries = {
                match unit.entries_at_offset(self.location.offset) {
                    Ok(entries) => entries,
                    _ => return members,
                }
            };
            if entries.next_dfs().is_err() {
                return members;
            }
            while let Ok(Some((_, entry))) = entries.next_dfs() {
                if entry.tag() != gimli::DW_TAG_member {
                    break;
                }
                let memb_loc = Location {
                    header: self.location.header,
                    offset: entry.offset(),
                };
                let mut attrs = entry.attrs();
                while let Ok(Some(attr)) = attrs.next() {
                    if attr.name() == gimli::DW_AT_type {
                        if let AttributeValue::UnitRef(offset) = attr.value() {
                            let type_loc = Location {
                                header: self.location.header,
                                offset,
                            };
                            members.push(Member { type_loc, memb_loc });
                            break;
                        }
                    };
                };
            };
            members
        });
        members.unwrap()
    }
}

impl Pointer {
    pub fn deref(&self, dwarf: &Dwarf)
    -> Result<Option<MemberType>, Error> {
        dwarf.entry_context(&self.location.clone(), |entry|
         -> Result<Option<MemberType>, Error> {
            let mut attrs = entry.attrs();
            while let Ok(Some(attr)) = attrs.next() {
                if attr.name() == gimli::DW_AT_type {
                    if let AttributeValue::UnitRef(offset) = attr.value() {
                        let type_loc = Location {
                            header: self.location.header,
                            offset,
                        };
                        return dwarf.entry_context(&type_loc, |entry| {
                            return Ok(Some(entry_to_type(type_loc, entry)));
                        })?
                    }
                };
            };
            Ok(None)
        })?
    }
}

impl Array {
    pub fn get_type(&self, dwarf: &Dwarf)
    -> Result<Option<MemberType>, Error> {
        dwarf.entry_context(&self.location.clone(), |entry|
         -> Result<Option<MemberType>, Error> {
            let mut attrs = entry.attrs();
            while let Ok(Some(attr)) = attrs.next() {
                if attr.name() == gimli::DW_AT_type {
                    if let AttributeValue::UnitRef(offset) = attr.value() {
                        let type_loc = Location {
                            header: self.location.header,
                            offset,
                        };
                        return dwarf.entry_context(&type_loc, |entry| {
                            return Ok(Some(entry_to_type(type_loc, entry)));
                        })?
                    }
                };
            };
            Ok(None)
        })?
    }

    pub fn get_bound(&self, dwarf: &Dwarf) -> Result<usize, Error> {
        dwarf.unit_context(&self.location.clone(), |unit| {
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
                        if let gimli::AttributeValue::Data1(v) = attr.value() {
                            return v as usize + 1;
                        }
                    };
                };
            };
            bound
        })
    }
}

pub struct Dwarf<'a> {
    dwarf_cow: gimli::Dwarf<Cow<'a, [u8]>>,
    endianness: RunTimeEndian,
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
        let res = self.unit_context(loc, |unit| -> Result<R, Error> {
            let entry = match unit.entry(loc.offset) {
                Ok(entry) => entry,
                Err(_) => {
                    return Err(
                        Error::CUError(
                            format!("Failed to find DIE at location: {loc:?}")
                        )
                    );
                }
            };
            Ok(f(&entry))
        })?;
        res
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

    pub fn lookup_item<T: Tagged>(&mut self, name: String)
    -> Result<Option<T>, Error> {
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
                }

                if let Some(entry_name) = get_entry_name(self, entry) {
                    if name != entry_name {
                        continue;
                    }
                    let location = Location {
                        header: header_idx,
                        offset: entry.offset(),
                    };
                    let typ = T::new(location);
                    return Ok(Some(typ));
                }
            }
            header_idx += 1;
        }
        Ok(None)
    }

    pub fn get_named_items_map<T: Tagged>(&self)
    -> Result<BTreeMap<String, T>, Error> {
        let dwarf = self.borrow_dwarf();
        let mut header_idx = 0;
        let mut struct_locations: BTreeMap<String, T> = BTreeMap::new();
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
                }

                if let Some(name) = get_entry_name(self, entry) {
                    let location = Location {
                        header: header_idx,
                        offset: entry.offset(),
                    };
                    let typ = T::new(location);
                    struct_locations.insert(name, typ);
                }
            }
            header_idx += 1;
        }
        Ok(struct_locations)
    }

    pub fn get_named_items<T: Tagged>(&self)
    -> Result<Vec<(String, T)>, Error> {
        let dwarf = self.borrow_dwarf();
        let mut header_idx = 0;
        let mut items = Vec::<(String, T)>::new();
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
                }

                if let Some(name) = get_entry_name(self, entry) {
                    let location = Location {
                        header: header_idx,
                        offset: entry.offset(),
                    };
                    let typ = T::new(location);
                    items.push((name, typ));
                }
            }
            header_idx += 1;
        }
        Ok(items)
    }
}
