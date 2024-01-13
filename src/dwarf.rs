//! Loading of DWARF information
use std::{collections::HashMap, borrow::Cow};
use object::{Object, ObjectSection, ReadRef};
use gimli::RunTimeEndian;

use crate::dwarf::borrowable_dwarf::BorrowableDwarf;
use crate::unit_has_members::UnitHasMembers;
use crate::unit_name_type::UnitNamedType;
use crate::{DIE, CU, GimliDwarf};
// use crate::owned_get_entry_name;
use crate::get_entry_name;
use crate::Location;
use crate::Tagged;
use crate::Struct;
use crate::Error;

/// A struct to hold the HashMap key for `get_named_structs_map`
#[derive(Eq, Hash, PartialEq)]
pub struct StructHashKey {
    /// The name of the struct
    pub name: String,

    /// The size of the struct in bytes
    pub byte_size: usize,

    /// A tuple of: member name, member offset
    pub members: Vec<(String, usize)>
}

fn for_each_die<T: Tagged, F>(dwarf: &GimliDwarf, mut f: F)
-> Result<(), Error>
where F: FnMut(&CU, &DIE, Location) -> Result<bool, Error> {
    let mut unit_headers = dwarf.debug_info.units();
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

            let header_offset =
                match header.offset().as_debug_info_offset() {
                    Some(offset) => offset,
                    // should be unreachable
                    None => return Err(Error::HeaderOffsetError)
            };

            let location = Location {
                header: header_offset,
                offset: entry.offset(),
            };

            // return if function returns true
            if f(&unit, entry, location)? {
                return Ok(())
            }
        }
    }
    Ok(())
}

/// Represents DWARF data
pub struct Dwarf<'a> {
    dwarf_cow: gimli::Dwarf<Cow<'a, [u8]>>,
    endianness: RunTimeEndian
}

impl<'a> Dwarf<'a> {
    pub fn load(data: impl ReadRef<'a>) -> Result<Self, Error> {
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

        // Load all of the sections
        let dwarf_cow = gimli::Dwarf::load(&load_section).unwrap();

        Ok(Self{dwarf_cow, endianness})
    }
}

pub(crate) mod borrowable_dwarf {
    use crate::GimliDwarf;

    pub trait BorrowableDwarf {
        fn borrow_dwarf<F,R>(&self, f: F) -> R
        where F: FnOnce(&GimliDwarf) -> R;
    }
}


pub trait DwarfLookups : borrowable_dwarf::BorrowableDwarf
where Self: Sized + DwarfContext {
    /// Get the first occurrence of debug info of some type with the specified
    /// name
    fn lookup_type<T: Tagged>(&self, name: String)
    -> Result<Option<T>, Error> {
        let mut item: Option<T> = None;
        self.borrow_dwarf(|dwarf| {
            let _ = for_each_die::<T, _>(dwarf, |_, entry, loc| {
                if let Some(entry_name) = get_entry_name(self, entry) {
                    if name == entry_name {
                        item = Some(T::new(loc));
                        return Ok(true);
                    }
                }
                Ok(false)
            });
        });
        Ok(item)
    }

    /// Get a HashMap of all debug info of some type hashed by name
    fn get_named_types_map<T: Tagged>(&self)
    -> Result<HashMap<String, T>, Error> {
        let mut item_locations: HashMap<String, T> = HashMap::new();
        self.borrow_dwarf(|dwarf| {
            let _ = for_each_die::<T, _>(dwarf, |_unit, entry, loc| {
                 if let Some(name) = get_entry_name(self, entry) {
                    let typ = T::new(loc);
                    item_locations.insert(name, typ);
                 }
                Ok(false)
            });
        });
        Ok(item_locations)
    }

    /// Similar to get_named_entries_map but with a more fine grained key for
    /// the hash, this should catch most cases where a struct with the same name
    /// is defined in multiple places
    fn get_fg_named_structs_map(&self)
    -> Result<HashMap<StructHashKey, Struct>, Error> {
        let mut struct_locations: HashMap<StructHashKey, Struct> = {
            HashMap::new()
        };
        self.borrow_dwarf(|dwarf| {
            let _ = for_each_die::<Struct, _>(dwarf, |unit, entry, loc| {
                if let Some(name) = get_entry_name(self, entry) {
                    let typ = Struct::new(loc);
                    let byte_size = typ.u_byte_size(unit)?;
                    let members: Vec<(String,usize)> = {
                        typ.u_members(unit)?
                        .iter().map(|m| -> Result<(String,usize), Error> {
                            Ok((m.u_name(self, unit).unwrap_or("".to_string()),
                                m.u_offset(unit)?))
                        }).collect::<Result<Vec<_>, _>>()?
                    };

                    let key = StructHashKey {name, byte_size, members};
                    struct_locations.insert(key, typ);
                }
                Ok(false)
            });
        });
        Ok(struct_locations)
    }

    /// Get a vector of all debug info of some type by name
    fn get_named_types<T: Tagged>(&self)
    -> Result<Vec<(String, T)>, Error> {
        let mut items: Vec<(String, T)> = Vec::new();
        self.borrow_dwarf(|dwarf| {
            let _ = for_each_die::<T, _>(dwarf, |_, entry, loc| {
                if let Some(name) = get_entry_name(self, entry) {
                    let typ = T::new(loc);
                    items.push((name, typ));
                }
                Ok(false)
            });
        });
        Ok(items)
    }
}

impl DwarfLookups for Dwarf<'_> {}
impl DwarfLookups for OwnedDwarf {}

/// Represents owned DWARF data, intended to be used by python bindings
pub struct OwnedDwarf {
    dwarf_vec: gimli::Dwarf<Vec<u8>>,
    endianness: RunTimeEndian
}

impl<'a> OwnedDwarf {
    pub fn load(data: impl ReadRef<'a>) -> Result<Self, Error> {
        let object = object::File::parse(data)?;

        let endianness = if object.is_little_endian() {
            gimli::RunTimeEndian::Little
        } else {
            gimli::RunTimeEndian::Big
        };

        let load_section = |id: gimli::SectionId|
        -> Result<Vec<u8>, gimli::Error> {
            let data = match object.section_by_name(id.name()) {
                Some(ref section) => {
                    section.uncompressed_data()
                           .unwrap_or_else(|_| Cow::Borrowed(&[][..]))
                           .into_owned()
                },
                None =>Vec::new(),
            };
            Ok(data)
        };

        // Load all of the sections
        let dwarf_vec = gimli::Dwarf::load(&load_section).unwrap();

        Ok(Self{dwarf_vec, endianness})
    }
}


impl borrowable_dwarf::BorrowableDwarf for OwnedDwarf {
    fn borrow_dwarf<F,R>(&self, f: F) -> R
    where F: FnOnce(&GimliDwarf) -> R {
        let borrow_section: &dyn for<'b> Fn(&'b Vec<u8>)
        -> gimli::EndianSlice<'b, gimli::RunTimeEndian> =
        &|section| gimli::EndianSlice::new(section, self.endianness);

        let dwarf = self.dwarf_vec.borrow(borrow_section);
        f(&dwarf)
    }
}

impl borrowable_dwarf::BorrowableDwarf for Dwarf<'_> {
    fn borrow_dwarf<F,R>(&self, f: F) -> R
    where F: FnOnce(&GimliDwarf) -> R {
        let borrow_section: &dyn for<'b> Fn(&'b Cow<[u8]>)
        -> gimli::EndianSlice<'b, gimli::RunTimeEndian> =
        &|section| gimli::EndianSlice::new(section, self.endianness);

        let dwarf = self.dwarf_cow.borrow(borrow_section);
        f(&dwarf)
    }
}

/// General functions for getting a CU/DIE from either a Dwarf or CU object
/// if possible, since type information does not cross CUs its best for perf to
/// use Dwarf.unit_context to obtain a CU once and pass that CU to the 'u_'
// variants of the parsing methods as many times as necessary
pub trait DwarfContext {
    fn entry_context<F,R>(&self, loc: &Location, f: F) -> Result<R, Error>
    where F: FnOnce(&DIE) -> R;

    fn unit_context<F,R>(&self, loc: &Location, f: F) -> Result<R, Error>
    where F: FnOnce(&CU) -> R;
}

impl DwarfContext for Dwarf<'_> {
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
        self.borrow_dwarf(|dwarf| {
            let debug_info = dwarf.debug_info;
            let unit_header = match debug_info.header_from_offset(loc.header) {
                Ok(header) => header,
                Err(e) => return Err(
                    Error::CUError(
                        format!("Failed to seek to UnitHeader, error: {}", e)
                    ))
            };
            let unit = gimli::Unit::new(dwarf, unit_header).unwrap();
            Ok(f(&unit))
        })
    }
}

impl DwarfContext for OwnedDwarf {
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
        self.borrow_dwarf(|dwarf| {
            let debug_info = dwarf.debug_info;
            let unit_header = match debug_info.header_from_offset(loc.header) {
                Ok(header) => header,
                Err(e) => return Err(
                    Error::CUError(
                        format!("Failed to seek to UnitHeader, error: {}", e)
                    ))
            };
            let unit = gimli::Unit::new(dwarf, unit_header).unwrap();
            Ok(f(&unit))
        })
    }
}

impl DwarfContext for CU<'_> {
    fn entry_context<F,R>(&self, loc: &Location, f: F) -> Result<R, Error>
    where F: FnOnce(&DIE) -> R {
        let entry = match self.entry(loc.offset) {
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
    }

    fn unit_context<F,R>(&self, _loc: &Location, f: F) -> Result<R, Error>
    where F: FnOnce(&CU) -> R {
        Ok(f(self))
    }
}

