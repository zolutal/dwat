//! Formatting methods for type information.
use crate::dwarf::borrowable_dwarf::BorrowableDwarf;
use crate::unit_has_members::UnitHasMembers;
use crate::unit_inner_type::UnitInnerType;
use crate::unit_name_type::UnitNamedType;
use crate::{Member, Error, Type, CU};
use crate::dwarf::DwarfContext;

pub fn format_type<D>(dwarf: &D, unit: &CU, member_name: String, typ: Type,
                      level: usize, tablevel: usize, verbosity: u8,
                      base_offset: usize)
-> Result<String, Error>
where D: DwarfContext + BorrowableDwarf {
    let mut out = String::new();
    match typ {
        Type::Array(a) => {
            let inner = a.u_get_type(unit)?;
            let inner_fmt = format_type(dwarf, unit, "".to_string(), inner,
                                        level+1, tablevel, verbosity,
                                        base_offset)?;
            out.push_str(&inner_fmt);
            if !out.ends_with('*') {
                out.push(' ');
            }
            if level == 0 {
                out.push_str(&member_name);
            }

            let bound = a.u_get_bound(unit)?;
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
        Type::Typedef(t) => {
            let name = t.u_name(dwarf, unit)?;
            if level == 0 {
                out.push_str(
                    &format!("{name} {member_name}")
                );
                return Ok(out);
            }
            out.push_str(&name);
        },
        Type::Struct(t) => {
            let name = t.u_name(dwarf, unit);
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
                    for memb in t.u_members(unit)?.into_iter() {
                        out.push_str(
                            &format_member(dwarf, unit, memb, tablevel+1,
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
        Type::Enum(t) => {
            match t.u_name(dwarf, unit) {
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
        Type::Union(u) => {
            let name = u.u_name(dwarf, unit);
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
                    for memb in u.u_members(unit)?.into_iter() {
                        out.push_str(
                            &format_member(dwarf, unit, memb, tablevel+1,
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
        },
        Type::Base(t) => {
            let name = t.u_name(dwarf, unit)?;
            if level == 0 {
                out.push_str(&format!("{name} {member_name}"));
                return Ok(out);
            }
            out.push_str(&name);
            return Ok(out);
        },
        Type::Subroutine(t) => {
            // just return comma separated arg string
            let params = t.u_get_params(unit)?;
            for pidx in 0..params.len() {
                let param = params[pidx].u_get_type(unit)?;
                // recursively convert type to string
                out.push_str(&format_type(dwarf, unit, "".to_string(),
                                          param, level+1, tablevel, verbosity,
                                          base_offset)?);
                if pidx != params.len()-1 {
                    out.push_str(", ");
                }
            };
        },
        Type::Pointer(p) => {
            let inner = p.u_get_type(unit);

            // pointers to subroutines must be handled differently
            if let Ok(Type::Subroutine(subp)) = inner {

                let return_type = match subp.u_get_type(unit) {
                    Ok(rtype) => format_type(dwarf, unit, "".to_string(), rtype,
                                             level+1, tablevel, verbosity,
                                             base_offset)?,
                    Err(Error::TypeAttributeNotFound) => "void".to_string(),
                    Err(e) => return Err(e)
                };

                let argstr = {
                    format_type(dwarf, unit, "".to_string(),
                                Type::Subroutine(subp),
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
                    format_type(dwarf, unit, "".to_string(), inner,
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
        Type::Const(c) => {
            let inner = c.u_get_type(unit);
            match inner {
                Ok(inner) => {
                    let inner_fmt = format_type(dwarf, unit, "".to_string(),
                                                inner, level+1, tablevel,
                                                verbosity, base_offset)?;
                    out.push_str(&format!("const {inner_fmt}"));
                }
                Err(Error::TypeAttributeNotFound) => {
                    out.push_str("const void");
                }
                Err(e) => return Err(e)
            }
        },
        Type::Volatile(c) => {
            let inner = c.u_get_type(unit)?;
            let inner_fmt = format_type(dwarf, unit, "".to_string(), inner,
                                        level+1, tablevel, verbosity,
                                        base_offset)?;
            out.push_str(&format!("volatile {inner_fmt}"));
            return Ok(out);
        },
        Type::Restrict(c) => {
            let inner = c.u_get_type(unit)?;
            let inner_fmt = format_type(dwarf, unit, "".to_string(), inner,
                                        level+1, tablevel, verbosity,
                                        base_offset)?;
            out.push_str(&format!("{inner_fmt} restrict"));
            return Ok(out);
        }
    }
    Ok(out)
}

pub fn format_member<D>(dwarf: &D, unit: &CU, member: Member, tablevel: usize,
                        verbosity: u8, base_offset: usize)
-> Result<String, Error>
where D: DwarfContext + BorrowableDwarf {
    let mtype = member.u_get_type(unit)?;
    let name = match member.u_name(dwarf, unit) {
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

    let memb_offset = match member.u_offset(unit) {
        Ok(memb_offset) => memb_offset,
        Err(Error::MemberLocationAttributeNotFound) => 0,
        Err(e) => return Err(e)

    };
    let offset = base_offset + memb_offset;

    formatted.push_str(
        &format_type(dwarf, unit, name, mtype, 0, tablevel, verbosity, offset)?
    );

    match member.u_bit_size(unit) {
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

        let bytesz = member.u_byte_size(unit)?;
        formatted.push_str(&format!("\t/* {bytesz: >4} | \
                                          {offset: >4} */"));
    }

    formatted.push('\n');

    Ok(formatted)
}
