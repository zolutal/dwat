//! Formatting methods for type information.

use crate::prelude::*;
use crate::MemberType;
use crate::Member;
use crate::Dwarf;
use crate::Error;

pub fn format_type(dwarf: &Dwarf, member_name: String, typ: MemberType,
                   level: usize, tablevel: usize, verbosity: u8,
                   base_offset: usize)
-> Result<String, Error> {
    let mut out = String::new();
    match typ {
        MemberType::Array(a) => {
            let inner = a.get_type(dwarf)?;
            let inner_fmt = format_type(dwarf, "".to_string(), inner,
                                        level+1, tablevel, verbosity,
                                        base_offset)?;
            out.push_str(&inner_fmt);
            if !out.ends_with('*') {
                out.push(' ');
            }
            if level == 0 {
                out.push_str(&member_name);
            }

            let bound = a.get_bound(dwarf)?;
            let bound_str = {
                if bound == 0 {
                    String::from("[]")
                } else {
                    format!("[{bound}]")
                }
            };
            out.push_str(&bound_str);
            return Ok(out);
        }
        MemberType::Typedef(t) => {
            let name = t.name(dwarf)?;
            if level == 0 {
                out.push_str(
                    &format!("{name} {member_name}")
                );
                return Ok(out);
            }
            out.push_str(&name);
        },
        MemberType::Struct(t) => {
            let name = t.name(dwarf);
            match name {
                Ok(name) => {
                    if level == 0 {
                        out.push_str(
                            &format!("struct {name} {member_name}")
                        );
                        return Ok(out);
                    }
                    out.push_str(&format!("struct {name}"));
                    return Ok(out);
                }
                Err(Error::NameAttributeNotFound) => {
                    // reaching here means we hit a nested struct type
                    out.push_str("struct {\n");
                    for memb in t.members(dwarf)?.into_iter() {
                        out.push_str(
                            &format_member(dwarf, memb, tablevel+1,
                                           verbosity, base_offset)?
                        );
                    }

                    for _ in 0..=tablevel {
                        out.push_str("    ");
                    }
                    out.push('}');
                    return Ok(out);
                }
                Err(e) => return Err(e)
            }
        },
        MemberType::Enum(t) => {
            match t.name(dwarf) {
                Ok(name) => {
                    if level == 0 {
                        out.push_str(
                            &format!("enum {name} {member_name}")
                        );
                        return Ok(out)
                    }
                    // TODO: print enum members
                    out.push_str(&format!("enum {name}"));
                }
                Err(Error::NameAttributeNotFound) => {
                    if level == 0 {
                        out.push_str(&format!("enum {member_name}"));
                        return Ok(out)
                    }
                    // TODO: print enum members
                    out.push_str("enum");
                }
                Err(e) => return Err(e)
            }
        },
        MemberType::Union(u) => {
            let name = u.name(dwarf);
            match name {
                Ok(name) => {
                    if level == 0 {
                        out.push_str(
                            &format!("union {name} {member_name}")
                        );
                        return Ok(out);
                    }
                    out.push_str(&format!("union {name}"));
                    return Ok(out);
                }
                Err(Error::NameAttributeNotFound) => {
                    out.push_str("union {\n");
                    for memb in u.members(dwarf)?.into_iter() {
                        out.push_str(
                            &format_member(dwarf, memb, tablevel+1,
                                           verbosity, base_offset)?);
                    }

                    for _ in 0..=tablevel {
                        out.push_str("    ");
                    }
                    out.push('}');
                    return Ok(out);
                }
                Err(e) => return Err(e)
            }
            // reaching here means we hit a nested union type
        },
        MemberType::Base(t) => {
            let name = t.name(dwarf)?;
            if level == 0 {
                out.push_str(&format!("{name} {member_name}"));
                return Ok(out);
            }
            out.push_str(&name);
            return Ok(out);
        },
        MemberType::Subroutine(t) => {
            // just return comma separated arg string
            let params = t.get_params(dwarf)?;
            for pidx in 0..params.len() {
                let param = params[pidx].get_type(dwarf)?;
                // recursively convert type to string
                out.push_str(&format_type(dwarf, "".to_string(),
                             param, level+1, tablevel, verbosity,
                             base_offset)?);
                if pidx != params.len()-1 {
                    out.push_str(", ");
                }
            };
        },
        MemberType::Pointer(p) => {
            let inner = p.deref(dwarf);

            // pointers to subroutines must be handled differently
            if let Ok(MemberType::Subroutine(subp)) = inner {

                // FIXME: get the actual return type
                let return_type = "void";

                let argstr = {
                    format_type(dwarf, "".to_string(),
                                MemberType::Subroutine(subp),
                                level+1, tablevel, verbosity,
                                base_offset)?
                };

                out.push_str(
                    &format!("{return_type} (*{member_name})({argstr})")
                );
                return Ok(out);
            }

            // FORMAT: {type} *{member_name}

            let ptr_type = match inner {
                Ok(inner) => {
                    format_type(dwarf, "".to_string(), inner,
                                level+1, tablevel, verbosity,
                                base_offset)?
                },
                Err(Error::TypeAttributeNotFound) => {
                    "void".to_string()
                },
                Err(e) => return Err(e)
            };
            out.push_str(&ptr_type);

            if ptr_type.ends_with('*'){
                out.push('*');
            } else {
                out.push_str(" *");
            }

            if level == 0 {
                out.push_str(&member_name);
                return Ok(out);
            }
            return Ok(out);
        },
        MemberType::Const(c) => {
            let inner = c.get_type(dwarf);
            match inner {
                Ok(inner) => {
                    let inner_fmt = format_type(dwarf, "".to_string(), inner,
                                                level+1, tablevel, verbosity,
                                                base_offset)?;
                    out.push_str(&format!("const {inner_fmt}"));
                }
                Err(Error::TypeAttributeNotFound) => {
                    out.push_str("const void");
                }
                Err(e) => return Err(e)
            }
        },
        MemberType::Volatile(c) => {
            let inner = c.get_type(dwarf)?;
            let inner_fmt = format_type(dwarf, "".to_string(), inner,
                                             level+1, tablevel, verbosity,
                                             base_offset)?;
            out.push_str(&format!("volatile {inner_fmt}"));
            return Ok(out);
        },
        MemberType::Restrict(c) => {
            let inner = c.get_type(dwarf)?;
            let inner_fmt = format_type(dwarf, "".to_string(), inner,
                                             level+1, tablevel, verbosity,
                                             base_offset)?;
            out.push_str(&format!("{inner_fmt} restrict"));
            return Ok(out);
        },
        _ => {
            eprintln!("Unhandled type could not be formatted {typ:?}");
        }
    }
    Ok(out)
}

pub fn format_member(dwarf: &Dwarf, member: Member, tablevel: usize,
                     verbosity: u8, base_offset: usize)
-> Result<String, Error> {
    let mtype = member.get_type(dwarf)?;
    let name = match member.name(dwarf) {
        Ok(name) => name,
        Err(Error::NameAttributeNotFound) => {
            // members can be anon structs or unions
            // it would be nice to check for those cases and propogate the error
            // otherwise, but type modifiers would also need to be stripped...
            // just excluding the name on error is probably fine tbh
            "".to_string()
        },
        Err(e) => return Err(e)
    };

    let mut formatted = String::new();
    for _ in 0..=tablevel {
        formatted.push_str("    ");
    }

    let offset = base_offset + member.member_location(dwarf)?.unwrap_or(0);

    formatted.push_str(
        &format_type(dwarf, name, mtype, 0, tablevel, verbosity, offset)?
    );

    match member.bit_size(dwarf) {
        Ok(bitsz) => {
            formatted.push_str(&format!(":{bitsz}"));
        }
        Err(Error::BitSizeAttributeNotFound) => {},
        Err(e) => return Err(e)
    }

    formatted.push(';');

    if verbosity > 0 {
        // generic padding based on last newline in formatted string
        let last_newline = formatted.rfind('\n').map(|idx| idx+1).unwrap_or(0);

        // cast to signed to prevent underflow
        let last_line_len: isize = (formatted.len()-last_newline) as isize;
        for _ in 0..(48-last_line_len) {
            formatted.push(' ');
        }

        let bytesz = member.byte_size(dwarf)?;
        formatted.push_str(&format!("\t/* sz: {bytesz: >4} | \
                                          off: {offset: >4} */"));
    }

    formatted.push('\n');

    Ok(formatted)
}
