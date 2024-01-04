//! Loading of DWARF information
use std::{collections::HashMap, borrow::Cow};
use object::{Object, ObjectSection, ReadRef};
use fallible_iterator::FallibleIterator;
use gimli::RunTimeEndian;
use std::sync::RwLock;
use std::rc::Rc;

use crate::{DIE, CU, GimliDwarf};
use crate::get_entry_name;
use crate::Location;
use crate::Tagged;
use crate::Error;

type RwDwarfCow<'a> = Rc<RwLock<gimli::Dwarf<Cow<'a, [u8]>>>>;

/// Represents DWARF data
pub struct Dwarf<'a> {
    dwarf_cow: RwDwarfCow<'a>,
    endianness: RunTimeEndian
}

impl<'a> Clone for Dwarf<'a> {
    fn clone(&self) -> Dwarf<'a> {
        Dwarf {
            dwarf_cow: self.dwarf_cow.clone(),
            endianness: self.endianness
        }
    }
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

        // Load all of the sections
        let dwarf_cow = Rc::new(RwLock::new(
                gimli::Dwarf::load(&load_section).unwrap()
        ));

        Ok(Self{dwarf_cow, endianness})
    }

    pub(crate) fn borrow_dwarf<F,R>(&self, f: F) -> R
    where F: FnOnce(&GimliDwarf) -> R {
        let borrow_section: &dyn for<'b> Fn(&'b Cow<[u8]>)
        -> gimli::EndianSlice<'b, gimli::RunTimeEndian> =
        &|section| gimli::EndianSlice::new(section, self.endianness);

        let binding = self.dwarf_cow.read().unwrap();
        let dwarf = binding.borrow(borrow_section);
        f(&dwarf)
    }

    pub(crate) fn entry_context<F,R>(&self, loc: &Location, f: F) -> Result<R, Error>
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

    pub(crate) fn unit_context<F,R>(&self, loc: &Location, f: F) -> Result<R, Error>
    where F: FnOnce(&CU) -> R {
        self.borrow_dwarf(|dwarf| {
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
        })
    }

    fn for_each_item<T: Tagged, F>(&self, mut f: F)
    -> Result<(), Error>
    where F: FnMut(&DIE, Location) -> bool {
        self.borrow_dwarf(|dwarf| {
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

                    let location = Location {
                        header: header_idx,
                        offset: entry.offset(),
                    };

                    // return if function returns true
                    if f(entry, location) {
                        return Ok(())
                    }
                }
                header_idx += 1;
            }
            Ok(())
        })
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
