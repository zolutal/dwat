import typing
from dwat import *

def load_dwarf(
    file: typing.IO,
) -> Dwarf: ...

def load_dwarf_path(
    path: str,
) -> Dwarf: ...

class Dwarf:
    def lookup_type(self, named_type: NamedType, name: str) -> typing.Union[
        Struct | Array | Enum | Pointer | Subroutine | Typedef | Union | Base |
        Const | Volatile | Restrict
    ]: ...

    def get_named_types_dict(self, named_type: NamedType) -> typing.Dict[
        str,
        typing.Union[
            Struct | Array | Enum | Pointer | Subroutine | Typedef | Union |
            Base | Const | Volatile | Restrict
        ]
    ]: ...

    def get_named_types(self, named_type: NamedType) -> typing.List[
        typing.Tuple[
            str,
            Struct | Array | Enum | Pointer | Subroutine | Typedef | Union |
            Base | Const | Volatile | Restrict
        ]
    ]: ...

class Struct:
    def members(self) -> typing.List[dwat.Member]: ...
    byte_size: typing.Optional[int]
    name: typing.Optional[str]

class Array:
    def type(self) -> typing.Union[
        Struct | Array | Enum | Pointer | Subroutine | Typedef | Union |
        Base | Const | Volatile | Restrict
    ]: ...
    byte_size: typing.Optional[int]
    bounds: int

class Enum:
    def type(self) -> typing.Union[
        Struct | Array | Enum | Pointer | Subroutine | Typedef | Union |
        Base | Const | Volatile | Restrict
    ]: ...
    byte_size: typing.Optional[int]
    name: typing.Optional[str]

class Pointer:
    def type(self) -> typing.Union[
        Struct | Array | Enum | Pointer | Subroutine | Typedef | Union |
        Base | Const | Volatile | Restrict
    ]: ...
    def deref(self) -> typing.Union[
        Struct | Array | Enum | Pointer | Subroutine | Typedef | Union |
        Base | Const | Volatile | Restrict
    ]: ...
    byte_size: typing.Optional[int]

class Subroutine:
    def return_type(self) -> typing.Union[
        Struct | Array | Enum | Pointer | Subroutine | Typedef | Union |
        Base | Const | Volatile | Restrict
    ]: ...
    def params(self) -> typing.List[Parameter]: ...

class Typedef:
    def type(self) -> typing.Union[
        Struct | Array | Enum | Pointer | Subroutine | Typedef | Union |
        Base | Const | Volatile | Restrict
    ]: ...
    byte_size: typing.Optional[int]
    name: typing.Optional[str]

class Union:
    def members(self) -> typing.List[dwat.Member]: ...
    byte_size: typing.Optional[int]
    name: typing.Optional[str]

class Base:
    byte_size: typing.Optional[int]
    name: typing.Optional[str]

class Const:
    def type(self) -> typing.Union[
        Struct | Array | Enum | Pointer | Subroutine | Typedef | Union |
        Base | Const | Volatile | Restrict
    ]: ...
    byte_size: typing.Optional[int]
    name: typing.Optional[str]

class Volatile:
    def type(self) -> typing.Union[
        Struct | Array | Enum | Pointer | Subroutine | Typedef | Union |
        Base | Const | Volatile | Restrict
    ]: ...
    byte_size: typing.Optional[int]
    name: typing.Optional[str]

class Restrict:
    def type(self) -> typing.Union[
        Struct | Array | Enum | Pointer | Subroutine | Typedef | Union |
        Base | Const | Volatile | Restrict
    ]: ...
    byte_size: typing.Optional[int]
    name: typing.Optional[str]

class Member:
    def type(self) -> typing.Union[
        Struct | Array | Enum | Pointer | Subroutine | Typedef | Union |
        Base | Const | Volatile | Restrict
    ]: ...
    byte_size: typing.Optional[int]
    bit_size: typing.Optional[int]
    offset: typing.Optional[str]
    name: typing.Optional[str]

class Parameter:
    def type(self) -> typing.Union[
        Struct | Array | Enum | Pointer | Subroutine | Typedef | Union |
        Base | Const | Volatile | Restrict
    ]: ...
