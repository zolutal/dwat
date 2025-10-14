use crate::dwarf::DwarfLookups;

use pyo3::exceptions::PyValueError;
use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;

use std::collections::HashMap;
use std::sync::Arc;

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
                       name: String) -> PyResult<Option<Py<PyAny>>> {
        let obj = match named_type {
            NamedTypes::Struct => {
                let found = self.inner.lookup_type::<crate::Struct>(name)?;
                if let Some(found) = found {
                    Some(Struct {
                            inner: found,
                            dwarf: self.clone()
                    }.into_py_any(py).unwrap())
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
                    }.into_py_any(py).unwrap())
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
                    }.into_py_any(py).unwrap())
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
                    }.into_py_any(py).unwrap())
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
                    }.into_py_any(py).unwrap())
                } else {
                    None
                }
            },
            NamedTypes::Variable => {
                let found = self.inner.lookup_type::<crate::Variable>(name)?;
                if let Some(found) = found {
                    Some(Variable {
                        inner: found,
                        dwarf: self.clone()
                    }.into_py_any(py).unwrap())
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
    -> PyResult<HashMap<String, Py<PyAny>>> {
        let mut type_map: HashMap<String, Py<PyAny>> = HashMap::new();
        match named_type {
            NamedTypes::Struct => {
                let inner = self.inner.clone();
                let found = inner.get_named_types_map::<crate::Struct>()?;
                for (k,v) in found.into_iter() {
                    type_map.insert(k, Struct {
                        inner: v,
                        dwarf: self.clone()
                    }.into_py_any(py).unwrap());
                }
            },
            NamedTypes::Enum => {
                let inner = self.inner.clone();
                let found = inner.get_named_types_map::<crate::Enum>()?;
                for (k,v) in found.into_iter() {
                    type_map.insert(k, Enum {
                        inner: v,
                        dwarf: self.clone()
                    }.into_py_any(py).unwrap());
                }
            },
            NamedTypes::Typedef => {
                let inner = self.inner.clone();
                let found = inner.get_named_types_map::<crate::Typedef>()?;
                for (k,v) in found.into_iter() {
                    type_map.insert(k, Typedef {
                        inner: v,
                        dwarf: self.clone()
                    }.into_py_any(py).unwrap());
                }
            },
            NamedTypes::Union => {
                let inner = self.inner.clone();
                let found = inner.get_named_types_map::<crate::Union>()?;
                for (k,v) in found.into_iter() {
                    type_map.insert(k, Union {
                        inner: v,
                        dwarf: self.clone()
                    }.into_py_any(py).unwrap());
                }
            },
            NamedTypes::Base => {
                let inner = self.inner.clone();
                let found = inner.get_named_types_map::<crate::Base>()?;
                for (k,v) in found.into_iter() {
                    type_map.insert(k, Base {
                        inner: v,
                        dwarf: self.clone()
                    }.into_py_any(py).unwrap());
                }
            }
            NamedTypes::Variable => {
                let inner = self.inner.clone();
                let found = inner.get_named_types_map::<crate::Variable>()?;
                for (k,v) in found.into_iter() {
                    type_map.insert(k, Variable {
                        inner: v,
                        dwarf: self.clone()
                    }.into_py_any(py).unwrap());
                }
            }
        };
        Ok(type_map)
    }

    /// Get a list of tuples of (name, type) corresponding to some NamedType.
    pub fn get_named_types(&self, py: Python<'_>, named_type: &NamedTypes)
    -> PyResult<Vec<(String, Py<PyAny>)>> {
        let mut types: Vec<(String, Py<PyAny>)> = Vec::new();
        match named_type {
            NamedTypes::Struct => {
                let found = self.inner.get_named_types::<crate::Struct>()?;
                for (k, v) in found {
                    types.push((k, Struct {
                        inner: v,
                        dwarf: self.clone()
                    }.into_py_any(py).unwrap()))
                }
            },
            NamedTypes::Enum => {
                let found = self.inner.get_named_types::<crate::Enum>()?;
                for (k, v) in found {
                    types.push((k, Enum {
                        inner: v,
                        dwarf: self.clone()
                    }.into_py_any(py).unwrap()))
                }
            },
            NamedTypes::Typedef => {
                let found = self.inner.get_named_types::<crate::Typedef>()?;
                for (k, v) in found {
                    types.push((k, Typedef {
                        inner: v,
                        dwarf: self.clone()
                    }.into_py_any(py).unwrap()))
                }
            },
            NamedTypes::Union => {
                let found = self.inner.get_named_types::<crate::Union>()?;
                for (k, v) in found {
                    types.push((k, Union {
                        inner: v,
                        dwarf: self.clone()
                    }.into_py_any(py).unwrap()))
                }
            },
            NamedTypes::Base => {
                let found = self.inner.get_named_types::<crate::Base>()?;
                for (k, v) in found {
                    types.push((k, Base {
                        inner: v,
                        dwarf: self.clone()
                    }.into_py_any(py).unwrap()))
                }
            },
            NamedTypes::Variable => {
                let found = self.inner.get_named_types::<crate::Variable>()?;
                for (k, v) in found {
                    types.push((k, Variable {
                        inner: v,
                        dwarf: self.clone()
                    }.into_py_any(py).unwrap()))
                }
            }
        };
        Ok(types)
    }
}

#[pymodule]
mod dwat {
    use pyo3::types::PyAnyMethods;
    use pyo3::prelude::*;
    use memmap2::Mmap;
    use libc::dup;
    use std::path::PathBuf;
    use std::fs::File;
    use std::sync::Arc;
    #[cfg(target_family = "unix")]
    use std::os::unix::io::FromRawFd;

    #[pymodule_export]
    use super::Dwarf;

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
    fn load_dwarf(file: &Bound<PyAny>) -> PyResult<Dwarf> {
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


    #[pymodule_export]
    use super::NamedTypes;

    #[pymodule_export]
    use super::Member;

    #[pymodule_export]
    use super::Parameter;

    // Types
    #[pymodule_export]
    use super::Struct;

    #[pymodule_export]
    use super::Array;

    #[pymodule_export]
    use super::Enum;

    #[pymodule_export]
    use super::Pointer;

    #[pymodule_export]
    use super::Subroutine;

    #[pymodule_export]
    use super::Typedef;

    #[pymodule_export]
    use super::Union;

    #[pymodule_export]
    use super::Base;

    #[pymodule_export]
    use super::Const;

    #[pymodule_export]
    use super::Volatile;

    #[pymodule_export]
    use super::Restrict;

    #[pymodule_export]
    use super::Variable;
}
