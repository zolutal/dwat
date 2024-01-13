use crate::dwarf::DwarfLookups;

use pyo3::exceptions::PyValueError;
use pyo3::wrap_pyfunction;
use pyo3::prelude::*;

#[cfg(target_family = "unix")]
use std::os::unix::io::FromRawFd;
use libc::dup;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::fs::File;
use memmap2::Mmap;

mod pytypes;
use pytypes::NamedTypes;
use pytypes::*;

impl std::convert::From<crate::Error> for PyErr {
    fn from(err: crate::Error) -> PyErr {
        PyValueError::new_err(err.to_string())
    }
}

/// Represents a loaded DWARF file
#[pyclass]
#[derive(Clone)]
struct Dwarf {
    pub(crate) inner: Arc<crate::dwarf::OwnedDwarf>
}

#[pymethods]
impl Dwarf {
    /// Lookup a type corresponding to some NamedType and `name`.
    pub fn lookup_type(&mut self, py: Python<'_>, named_type: &NamedTypes,
                       name: String) -> PyResult<Option<PyObject>> {
        let obj = match named_type {
            NamedTypes::Struct => {
                let found = self.inner.lookup_type::<crate::Struct>(name)?;
                if let Some(found) = found {
                    Some(Struct {
                            inner: found,
                            dwarf: self.clone()
                    }.into_py(py))
                } else {
                    None
                }
            },
            NamedTypes::Enum => {
                let found = self.inner.lookup_type::<crate::Enum>(name)?;
                if let Some(found) = found {
                    Some(Enum {
                        inner: found,
                        dwarf: self.clone()
                    }.into_py(py))
                } else {
                    None
                }
            },
            NamedTypes::Typedef => {
                let found = self.inner.lookup_type::<crate::Typedef>(name)?;
                if let Some(found) = found {
                    Some(Typedef {
                        inner: found,
                        dwarf: self.clone()
                    }.into_py(py))
                } else {
                    None
                }
            },
            NamedTypes::Union => {
                let found = self.inner.lookup_type::<crate::Union>(name)?;
                if let Some(found) = found {
                    Some(Union {
                        inner: found,
                        dwarf: self.clone()
                    }.into_py(py))
                } else {
                    None
                }
            },
            NamedTypes::Base => {
                let found = self.inner.lookup_type::<crate::Base>(name)?;
                if let Some(found) = found {
                    Some(Base {
                        inner: found,
                        dwarf: self.clone()
                    }.into_py(py))
                } else {
                    None
                }
            }
        };
        Ok(obj)
    }

    /// Get a dictionary mapping names to types corresponding to some
    /// NamedType
    pub fn get_named_types_dict(&self, py: Python<'_>, named_type: &NamedTypes)
    -> PyResult<HashMap<String, PyObject>> {
        let mut type_map: HashMap<String, PyObject> = HashMap::new();
        match named_type {
            NamedTypes::Struct => {
                let inner = self.inner.clone();
                let found = inner.get_named_types_map::<crate::Struct>()?;
                for (k,v) in found.into_iter() {
                    type_map.insert(k, Struct {
                        inner: v,
                        dwarf: self.clone()
                    }.into_py(py));
                }
            },
            NamedTypes::Enum => {
                let inner = self.inner.clone();
                let found = inner.get_named_types_map::<crate::Enum>()?;
                for (k,v) in found.into_iter() {
                    type_map.insert(k, Enum {
                        inner: v,
                        dwarf: self.clone()
                    }.into_py(py));
                }
            },
            NamedTypes::Typedef => {
                let inner = self.inner.clone();
                let found = inner.get_named_types_map::<crate::Typedef>()?;
                for (k,v) in found.into_iter() {
                    type_map.insert(k, Typedef {
                        inner: v,
                        dwarf: self.clone()
                    }.into_py(py));
                }
            },
            NamedTypes::Union => {
                let inner = self.inner.clone();
                let found = inner.get_named_types_map::<crate::Union>()?;
                for (k,v) in found.into_iter() {
                    type_map.insert(k, Union {
                        inner: v,
                        dwarf: self.clone()
                    }.into_py(py));
                }
            },
            NamedTypes::Base => {
                let inner = self.inner.clone();
                let found = inner.get_named_types_map::<crate::Base>()?;
                for (k,v) in found.into_iter() {
                    type_map.insert(k, Base {
                        inner: v,
                        dwarf: self.clone()
                    }.into_py(py));
                }
            }
        };
        Ok(type_map)
    }

    /// Get a list of tuples of (name, type) corresponding to some NamedType.
    pub fn get_named_types(&self, py: Python<'_>, named_type: &NamedTypes)
    -> PyResult<Vec<(String, PyObject)>> {
        let mut types: Vec<(String, PyObject)> = Vec::new();
        match named_type {
            NamedTypes::Struct => {
                let found = self.inner.get_named_types::<crate::Struct>()?;
                for (k, v) in found {
                    types.push((k, Struct {
                        inner: v,
                        dwarf: self.clone()
                    }.into_py(py)))
                }
            },
            NamedTypes::Enum => {
                let found = self.inner.get_named_types::<crate::Enum>()?;
                for (k, v) in found {
                    types.push((k, Enum {
                        inner: v,
                        dwarf: self.clone()
                    }.into_py(py)))
                }
            },
            NamedTypes::Typedef => {
                let found = self.inner.get_named_types::<crate::Typedef>()?;
                for (k, v) in found {
                    types.push((k, Typedef {
                        inner: v,
                        dwarf: self.clone()
                    }.into_py(py)))
                }
            },
            NamedTypes::Union => {
                let found = self.inner.get_named_types::<crate::Union>()?;
                for (k, v) in found {
                    types.push((k, Union {
                        inner: v,
                        dwarf: self.clone()
                    }.into_py(py)))
                }
            },
            NamedTypes::Base => {
                let found = self.inner.get_named_types::<crate::Base>()?;
                for (k, v) in found {
                    types.push((k, Base {
                        inner: v,
                        dwarf: self.clone()
                    }.into_py(py)))
                }
            }
        };
        Ok(types)
    }
}

/// Load a DWARF file by path
#[pyfunction]
fn load_dwarf_path(path: PathBuf) -> PyResult<Dwarf> {
    let file = File::open(path)?;
    let mmap = unsafe { Mmap::map(&file) }?;
    let dwarf = crate::dwarf::OwnedDwarf::load(&*mmap)?;
    Ok(Dwarf { inner: Arc::new(dwarf) })
}

/// Load a DWARF file from a python File IO object (unix only)
#[pyfunction]
#[cfg(target_family = "unix")]
fn load_dwarf(file: &PyAny) -> PyResult<Dwarf> {
    let fd: i32 = file.call_method0("fileno")?.extract()?;

    // need to duplicate the file descriptor, otherwise rust takes ownership
    // of it when from_raw_fd is called and will close it once it goes out of
    // scope
    let dup_fd = unsafe { dup(fd) };
    if dup_fd == -1 {
        return Err(PyErr::new::<pyo3::exceptions::PyOSError, _>(
            "Failed to duplicate file descriptor"
        ));
    }

    let file = unsafe { std::fs::File::from_raw_fd(dup_fd as i32) };
    let mmap = unsafe { Mmap::map(&file) }?;
    let dwarf = crate::dwarf::OwnedDwarf::load(&*mmap)?;
    Ok(Dwarf { inner: Arc::new(dwarf) })
}

#[pymodule]
fn dwat(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Dwarf>()?;

    #[cfg(target_family = "unix")]
    m.add_function(wrap_pyfunction!(load_dwarf, m)?)?;

    m.add_function(wrap_pyfunction!(load_dwarf_path, m)?)?;

    m.add_class::<NamedTypes>()?;

    m.add_class::<Member>()?;
    m.add_class::<Parameter>()?;

    // Types
    m.add_class::<Struct>()?;
    m.add_class::<Array>()?;
    m.add_class::<Enum>()?;
    m.add_class::<Pointer>()?;
    m.add_class::<Subroutine>()?;
    m.add_class::<Typedef>()?;
    m.add_class::<Union>()?;
    m.add_class::<Base>()?;
    m.add_class::<Const>()?;
    m.add_class::<Volatile>()?;
    m.add_class::<Restrict>()?;

    Ok(())
}
