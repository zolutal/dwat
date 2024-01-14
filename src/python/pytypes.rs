use pyo3::prelude::*;

use crate::prelude::*;
use crate::Error;
use super::Dwarf;

#[pyclass]
pub(super) enum Types {
    Struct,
    Array,
    Enum,
    Pointer,
    Subroutine,
    Typedef,
    Union,
    Base,
    Const,
    Volatile,
    Restrict,
}

/// Types that have names, used by Dwarf's lookup/get_named* methods
#[pyclass(name = "NamedType")]
pub(super) enum NamedTypes {
    Struct,
    Enum,
    Typedef,
    Union,
    Base,
}

#[pyclass]
pub(super) struct Struct {
    pub(super) inner: crate::Struct,
    pub(super) dwarf: Dwarf
}

#[pyclass]
pub(super) struct Array {
    pub(super) inner: crate::Array,
    pub(super) dwarf: Dwarf
}

#[pyclass]
pub(super) struct Enum {
    pub(super) inner: crate::Enum,
    pub(super) dwarf: Dwarf
}

#[pyclass]
pub(super) struct Pointer {
    pub(super) inner: crate::Pointer,
    pub(super) dwarf: Dwarf
}

#[pyclass]
pub(super) struct Subroutine {
    pub(super) inner: crate::Subroutine,
    pub(super) dwarf: Dwarf
}

#[pyclass]
pub(super) struct Typedef {
    pub(super) inner: crate::Typedef,
    pub(super) dwarf: Dwarf
}

#[pyclass]
pub(super) struct Union {
    pub(super) inner: crate::Union,
    pub(super) dwarf: Dwarf
}

#[pyclass]
pub(super) struct Base {
    pub(super) inner: crate::Base,
    pub(super) dwarf: Dwarf
}

#[pyclass]
pub(super) struct Const {
    pub(super) inner: crate::Const,
    pub(super) dwarf: Dwarf
}

#[pyclass]
pub(super) struct Volatile {
    pub(super) inner: crate::Volatile,
    pub(super) dwarf: Dwarf
}

#[pyclass]
pub(super) struct Restrict {
    pub(super) inner: crate::Restrict,
    pub(super) dwarf: Dwarf
}

#[pyclass]
pub(super) struct Member {
    pub(super) inner: crate::Member,
    pub(super) dwarf: Dwarf
}

#[pyclass]
pub(super) struct Parameter {
    pub(super) inner: crate::FormalParameter,
    pub(super) dwarf: Dwarf
}

pub(crate) fn to_py_object(py: Python<'_>, typ: crate::Type, dwarf: &Dwarf)
-> Option<PyObject> {
    match typ {
        crate::Type::Struct(struc) => {
            Some(Struct {
                    inner: struc,
                    dwarf: dwarf.clone()
            }.into_py(py))
        },
        crate::Type::Array(arr) => {
            Some(Array {
                    inner: arr,
                    dwarf: dwarf.clone()
            }.into_py(py))
        },
        crate::Type::Enum(enu) => {
            Some(Enum {
                    inner: enu,
                    dwarf: dwarf.clone()
            }.into_py(py))
        },
        crate::Type::Pointer(ptr) => {
            Some(Pointer {
                    inner: ptr,
                    dwarf: dwarf.clone()
            }.into_py(py))
        },
        crate::Type::Subroutine(sub) => {
            Some(Subroutine {
                    inner: sub,
                    dwarf: dwarf.clone()
            }.into_py(py))
        },
        crate::Type::Typedef(typedef) => {
            Some(Typedef {
                    inner: typedef,
                    dwarf: dwarf.clone()
            }.into_py(py))
        },
        crate::Type::Union(union) => {
            Some(Union {
                    inner: union,
                    dwarf: dwarf.clone()
            }.into_py(py))
        },
        crate::Type::Base(base) => {
            Some(Base {
                    inner: base,
                    dwarf: dwarf.clone()
            }.into_py(py))
        },
        crate::Type::Const(cons) => {
            Some(Const {
                    inner: cons,
                    dwarf: dwarf.clone()
            }.into_py(py))
        },
        crate::Type::Volatile(vol) => {
            Some(Volatile {
                    inner: vol,
                    dwarf: dwarf.clone()
            }.into_py(py))
        },
        crate::Type::Restrict(res) => {
            Some(Restrict {
                    inner: res,
                    dwarf: dwarf.clone()
            }.into_py(py))
        }
    }
}

macro_rules! attr_getter {
    ($self:ident, $method:ident, $error:pat) => {
        match $self.inner.$method(&*$self.dwarf.inner) {
            Ok(value) => Ok(Some(value)),
            Err($error) => Ok(None),
            Err(e) => Err(e.into())
        }
    };
}

#[pymethods]
impl Struct {
    /// The name of the struct
    #[getter]
    pub fn name(&self) -> PyResult<Option<String>> {
        attr_getter!(self, name, Error::NameAttributeNotFound)
    }

    /// The size of this type in bytes
    #[getter]
    pub fn byte_size(&self) -> PyResult<Option<usize>> {
        attr_getter!(self, byte_size, Error::ByteSizeAttributeNotFound)
    }

    /// A list of members/fields of this struct
    pub fn members(&self) -> PyResult<Vec<Member>> {
        let dwarf = &*self.dwarf.inner;
        let members = self.inner.members(dwarf)?;

        let mut py_members: Vec<Member> = Vec::new();
        for member in members.iter() {
            let py_object =  Member {
                inner: *member,
                dwarf: self.dwarf.clone()
            };
            py_members.push(py_object);
        }

        Ok(py_members)
    }

    pub fn __str__(&self) -> PyResult<String> {
        Ok(self.inner.to_string(&*self.dwarf.inner)?)
    }

    pub fn __repr__(&self) -> PyResult<String> {
        if let Ok(Some(name)) = self.name() {
            Ok(format!("<Struct: {name}>"))
        } else {
            Ok("<Struct>".to_string())
        }
    }
}

#[pymethods]
impl Array {
    /// The size (footprint) of this type in bytes
    #[getter]
    pub fn byte_size(&self) -> PyResult<Option<usize>> {
        attr_getter!(self, byte_size, Error::ByteSizeAttributeNotFound)
    }

    /// Retrieves the backing type of the array
    pub fn r#type(&self, py: Python<'_>) -> PyResult<Option<PyObject>> {
        let dwarf = &*self.dwarf.inner;
        Ok(to_py_object(py, self.inner.get_type(dwarf)?, &self.dwarf))
    }

    /// Get the bounds (number of entries) of the Array
    #[getter]
    pub fn bounds(&self) -> PyResult<usize> {
        let dwarf = &*self.dwarf.inner;
        Ok(self.inner.get_bound(dwarf)?)
    }

    pub fn __repr__(&self) -> PyResult<String> {
        Ok("<Array>".to_string())
    }
}

#[pymethods]
impl Enum {
    /// The size of this type in bytes
    #[getter]
    pub fn byte_size(&self) -> PyResult<Option<usize>> {
        attr_getter!(self, byte_size, Error::ByteSizeAttributeNotFound)
    }

    /// The name of the enum
    #[getter]
    pub fn name(&self) -> PyResult<Option<String>> {
        attr_getter!(self, name, Error::NameAttributeNotFound)
    }

    /// Retrieves the backing type of the enum
    pub fn r#type(&self, py: Python<'_>) -> PyResult<Option<PyObject>> {
        let dwarf = &*self.dwarf.inner;
        Ok(to_py_object(py, self.inner.get_type(dwarf)?, &self.dwarf))
    }

    pub fn __repr__(&self) -> PyResult<String> {
        Ok("<Enum>".to_string())
    }
}

#[pymethods]
impl Pointer {
    /// The size of this type in bytes
    #[getter]
    pub fn byte_size(&self) -> PyResult<Option<usize>> {
        attr_getter!(self, byte_size, Error::ByteSizeAttributeNotFound)
    }

    /// Retrieves the backing type of the pointer
    pub fn r#type(&self, py: Python<'_>) -> PyResult<Option<PyObject>> {
        let dwarf = &*self.dwarf.inner;
        Ok(to_py_object(py, self.inner.get_type(dwarf)?, &self.dwarf))
    }

    /// Retrieves the backing type of the pointer
    pub fn deref(&self, py: Python<'_>) -> PyResult<Option<PyObject>> {
        let dwarf = &*self.dwarf.inner;
        Ok(to_py_object(py, self.inner.get_type(dwarf)?, &self.dwarf))
    }

    pub fn __repr__(&self) -> PyResult<String> {
        Ok("<Pointer>".to_string())
    }
}

#[pymethods]
impl Subroutine {
    /// Retrieves the return_type of the subroutine
    pub fn return_type(&self, py: Python<'_>) -> PyResult<Option<PyObject>> {
        let dwarf = &*self.dwarf.inner;
        Ok(to_py_object(py, self.inner.get_type(dwarf)?, &self.dwarf))
    }

    /// Retrieves the parameters/arguments of the subroutine
    pub fn params(&self)
    -> PyResult<Vec<Parameter>> {
        let dwarf = &*self.dwarf.inner;
        let members = self.inner.get_params(dwarf)?;

        let mut py_params: Vec<Parameter> = Vec::new();
        for member in members.iter() {
            let py_object = Parameter {
                inner: *member,
                dwarf: self.dwarf.clone()
            };
            py_params.push(py_object);
        }

        Ok(py_params)
    }

    pub fn __repr__(&self) -> PyResult<String> {
        Ok("<Subroutine>".to_string())
    }
}

#[pymethods]
impl Typedef {
    /// The name of the typedef
    #[getter]
    pub fn name(&self) -> PyResult<Option<String>> {
        attr_getter!(self, name, Error::NameAttributeNotFound)
    }

    /// The size of this type in bytes
    #[getter]
    pub fn byte_size(&self) -> PyResult<Option<usize>> {
        attr_getter!(self, byte_size, Error::ByteSizeAttributeNotFound)
    }

    /// Retrieves the backing type of the typedef
    pub fn r#type(&self, py: Python<'_>) -> PyResult<Option<PyObject>> {
        let dwarf = &*self.dwarf.inner;
        Ok(to_py_object(py, self.inner.get_type(dwarf)?, &self.dwarf))
    }

    pub fn __str__(&self) -> PyResult<Option<String>> {
        self.name()
    }

    pub fn __repr__(&self) -> PyResult<String> {
        if let Ok(Some(name)) = self.name() {
            Ok(format!("<Typedef: {name}>"))
        } else {
            Ok("<Typedef>".to_string())
        }
    }
}

#[pymethods]
impl Union {
    /// The name of the union
    #[getter]
    pub fn name(&self) -> PyResult<Option<String>> {
        attr_getter!(self, name, Error::NameAttributeNotFound)
    }

    /// The size of this type in bytes
    #[getter]
    pub fn byte_size(&self) -> PyResult<Option<usize>> {
        attr_getter!(self, byte_size, Error::ByteSizeAttributeNotFound)
    }

    /// A list of members of this union
    pub fn members(&self) -> PyResult<Vec<Member>> {
        let dwarf = &*self.dwarf.inner;
        let members = self.inner.members(dwarf)?;

        let mut py_members: Vec<Member> = Vec::new();
        for member in members.iter() {
            let py_object =  Member {
                inner: *member,
                dwarf: self.dwarf.clone()
            };
            py_members.push(py_object);
        }

        Ok(py_members)
    }

    pub fn __str__(&self) -> PyResult<String> {
        Ok(self.inner.to_string(&*self.dwarf.inner)?)
    }

    pub fn __repr__(&self) -> PyResult<String> {
        if let Ok(Some(name)) = self.name() {
            Ok(format!("<Union: {name}>"))
        } else {
            Ok("<Union>".to_string())
        }
    }
}

#[pymethods]
impl Base {
    /// The name of the base type
    #[getter]
    pub fn name(&self) -> PyResult<Option<String>> {
        attr_getter!(self, name, Error::NameAttributeNotFound)
    }

    /// The size of this type in bytes
    #[getter]
    pub fn byte_size(&self) -> PyResult<Option<usize>> {
        attr_getter!(self, byte_size, Error::ByteSizeAttributeNotFound)
    }

    pub fn __str__(&self) -> PyResult<Option<String>> {
        self.name()
    }

    pub fn __repr__(&self) -> PyResult<String> {
        if let Ok(Some(name)) = self.name() {
            Ok(format!("<Base: {name}>"))
        } else {
            Ok("<Base>".to_string())
        }
    }
}

#[pymethods]
impl Const {
    /// The size of this type in bytes
    #[getter]
    pub fn byte_size(&self) -> PyResult<Option<usize>> {
        attr_getter!(self, byte_size, Error::ByteSizeAttributeNotFound)
    }

    /// Retrieves the backing type of the const modifier
    pub fn r#type(&self, py: Python<'_>) -> PyResult<Option<PyObject>> {
        let dwarf = &*self.dwarf.inner;
        Ok(to_py_object(py, self.inner.get_type(dwarf)?, &self.dwarf))
    }

    pub fn __repr__(&self) -> PyResult<String> {
        Ok("<Const>".to_string())
    }
}

#[pymethods]
impl Volatile {
    /// The size of this type in bytes
    #[getter]
    pub fn byte_size(&self) -> PyResult<Option<usize>> {
        attr_getter!(self, byte_size, Error::ByteSizeAttributeNotFound)
    }

    /// Retrieves the backing type of the volatile modifier
    pub fn r#type(&self, py: Python<'_>) -> PyResult<Option<PyObject>> {
        let dwarf = &*self.dwarf.inner;
        Ok(to_py_object(py, self.inner.get_type(dwarf)?, &self.dwarf))
    }

    pub fn __repr__(&self) -> PyResult<String> {
        Ok("<Volatile>".to_string())
    }
}

#[pymethods]
impl Restrict {
    /// The size of this type in bytes
    #[getter]
    pub fn byte_size(&self) -> PyResult<Option<usize>> {
        attr_getter!(self, byte_size, Error::ByteSizeAttributeNotFound)
    }

    /// Retrieves the backing type of the restrict modifier
    pub fn r#type(&self, py: Python<'_>) -> PyResult<Option<PyObject>> {
        let dwarf = &*self.dwarf.inner;
        Ok(to_py_object(py, self.inner.get_type(dwarf)?, &self.dwarf))
    }

    pub fn __repr__(&self) -> PyResult<String> {
        Ok("<Restrict>".to_string())
    }
}

#[pymethods]
impl Parameter {
    /// Retrieves the backing type of the parameter
    pub fn r#type(&self, py: Python<'_>) -> PyResult<Option<PyObject>> {
        let dwarf = &*self.dwarf.inner;
        Ok(to_py_object(py, self.inner.get_type(dwarf)?, &self.dwarf))
    }

    pub fn __repr__(&self) -> PyResult<String> {
        Ok("<Parameter>".to_string())
    }
}

#[pymethods]
impl Member {
    /// The name of the member
    #[getter]
    pub fn name(&self) -> PyResult<Option<String>> {
        attr_getter!(self, name, Error::NameAttributeNotFound)
    }

    /// The size of this member in bytes
    #[getter]
    pub fn byte_size(&self) -> PyResult<Option<usize>> {
        attr_getter!(self, byte_size, Error::ByteSizeAttributeNotFound)
    }

    /// The size of this member in bits (only present for bitfields)
    #[getter]
    pub fn bit_size(&self) -> PyResult<Option<usize>> {
        attr_getter!(self, bit_size, Error::BitSizeAttributeNotFound)
    }

    /// The offset of this member from the start of the data type
    #[getter]
    pub fn offset(&self) -> PyResult<Option<usize>> {
        attr_getter!(self, offset, Error::MemberLocationAttributeNotFound)
    }

    /// Retrieves the backing type of the member
    pub fn r#type(&self, py: Python<'_>) -> PyResult<Option<PyObject>> {
        let dwarf = &*self.dwarf.inner;
        Ok(to_py_object(py, self.inner.get_type(dwarf)?, &self.dwarf))
    }

    pub fn __str__(&self) -> PyResult<Option<String>> {
        self.name()
    }

    pub fn __repr__(&self) -> PyResult<String> {
        if let Ok(Some(name)) = self.name() {
            Ok(format!("<Member: {name}>"))
        } else {
            Ok("<Member>".to_string())
        }
    }
}
