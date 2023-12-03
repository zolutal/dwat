use crate::prelude::*;
use crate::MemberType;
use crate::Struct;
use crate::Member;
use crate::Dwarf;

type Error = crate::DwatError;

pub fn format_type(dwarf: &Dwarf, member_name: String, typ: MemberType,
                   level: usize, tablevel: usize) -> Result<String, Error> {
    let mut out = String::new();
    match typ {
        MemberType::Array(a) => {

            if let Some(inner) = a.get_type(dwarf)? {
                let inner_fmt = format_type(dwarf, "".to_string(), inner,
                                            level+1, tablevel)?;
                out.push_str(&format!("{inner_fmt}"));
            }
            if level == 0 {
                out.push_str(&format!(" {member_name}"));
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
            if let Some(name) = t.name(dwarf) {
                if level == 0 {
                    out.push_str(
                        &format!("{name} {member_name}")
                    );
                    return Ok(out);
                }
                out.push_str(&format!("{name}"));
            }
        },
        MemberType::Struct(t) => {
            if let Some(name) = t.name(dwarf) {
                if level == 0 {
                    out.push_str(
                        &format!("struct {name} {member_name}")
                    );
                    return Ok(out);
                }
                out.push_str(&format!("struct {name}"));
                return Ok(out);
            }
            // reaching here means we hit a nested struct type
            out.push_str("struct {\n");
            for memb in t.members(dwarf).into_iter() {
                out.push_str(
                    &format_member(dwarf, memb, tablevel+1)?
                );
            }

            let mut tabs = String::new();
            for _ in 0..=tablevel {
                tabs.push('\t');
            }
            out.push_str(&format!("{tabs}}}"));
            return Ok(out);
        },
        MemberType::Enum(t) => {
            if let Some(name) = t.name(dwarf) {
                if level == 0 {
                    out.push_str(
                        &format!("enum {name} {member_name}")
                    );
                    return Ok(out);
                }
                out.push_str(&format!("enum {name}"));
            }
            return Ok(out);
        },
        MemberType::Union(u) => {
            if let Some(name) = u.name(dwarf) {
                if level == 0 {
                    out.push_str(
                        &format!("union {name} {member_name}")
                    );
                    return Ok(out);
                }
                out.push_str(&format!("union {name}"));
                return Ok(out);
            }
            // reaching here means we hit a nested union type
            out.push_str("union {\n");
            for memb in u.members(dwarf).into_iter() {
                out.push_str(
                    &format_member(dwarf, memb, tablevel+1)?);
            }

            let mut tabs = String::new();
            for _ in 0..=tablevel {
                tabs.push('\t');
            }
            out.push_str(&format!("{tabs}}}"));
            return Ok(out);
        },
        MemberType::Base(t) => {
            if let Some(name) = t.name(dwarf) {
                if level == 0 {
                    out.push_str(&format!("{name} {member_name}"));
                    return Ok(out);
                }
                out.push_str(&format!("{name}"));
            }
            return Ok(out);
        },
        MemberType::Subroutine(t) => {
            // just return comma separated arg string
            let params = t.get_params(dwarf)?;
            for pidx in 0..params.len() {
                let param = params[pidx].get_type(dwarf)?;
                // recursively convert type to string
                if let Some(param) = param {
                    out.push_str(&format_type(dwarf, "".to_string(),
                                 param, level+1, tablevel)?);
                    if pidx != params.len()-1 {
                        out.push_str(", ");
                    }
                }
            };
            return Ok(out);
        },
        MemberType::Pointer(p) => {
            let inner = p.deref(dwarf)?;

            // pointers to subroutines must be handled differently
            if let Some(MemberType::Subroutine(subp)) = inner {

                // FIXME: get the actual return type
                let return_type = "void";

                let argstr = {
                    format_type(dwarf, "".to_string(),
                                MemberType::Subroutine(subp),
                                level+1, tablevel)?
                };

                out.push_str(
                    &format!("{return_type} (*{member_name})({argstr})")
                );
                return Ok(out);
            }

            // FORMAT: {type} *{member_name}

            let ptr_type = match inner {
                Some(inner) => {
                    format_type(dwarf, "".to_string(), inner,
                                level+1, tablevel)?
                },
                None => {
                    "void".to_string()
                }
            };
            out.push_str(&ptr_type);

            if ptr_type.ends_with('*'){
                out.push_str("*");
            } else {
                out.push_str(" *");
            }

            if level == 0 {
                out.push_str(&format!("{member_name}"));
                return Ok(out);
            }
            return Ok(out);
        },
        MemberType::Const(c) => {
            if let Some(inner) = c.get_type(dwarf)? {
                let inner_fmt = format_type(dwarf, "".to_string(), inner,
                                            level+1, tablevel)?;
                out.push_str(&format!("const {inner_fmt}"));
                return Ok(out);
            }
            out.push_str("const void");
            return Ok(out);
        },
        MemberType::Volatile(c) => {
            if let Some(inner) = c.get_type(dwarf)? {
                let inner_fmt = format_type(dwarf, "".to_string(), inner,
                                                 level+1, tablevel)?;
                out.push_str(&format!("volatile {inner_fmt}"));
            }
            return Ok(out);
        },
        _ => {
            eprintln!("Unhandled type could not be formatted {typ:?}");
        }
    }
    Ok(out)
}

pub fn format_member(dwarf: &Dwarf, member: Member, tablevel: usize)
-> Result<String, Error> {
    let mut name = String::new();
    if let Some(n) = member.name(&dwarf)? {
        name = n;
    };

    let mtype = member.get_type(&dwarf)?;
    let mut formatted = format_type(dwarf, name, mtype, 0, tablevel)?;

    let mut tabs = String::new();
    for _ in 0..=tablevel {
        tabs.push('\t');
    }
    formatted = format!("{tabs}{formatted}");
    let bitsz = member.get_bit_size(dwarf)?;
    if let Some(bitsz) = bitsz {
        formatted.push_str(&format!(":{bitsz}"));
    }
    formatted.push_str(&format!(";\n"));
    Ok(formatted)
}

pub fn print_struct(dwarf: &Dwarf, struc: Struct)
-> Result<(), Error> {
    println!("struct {} {{", struc.name(&dwarf).unwrap());
    let members = struc.members(&dwarf);
    for member in members.into_iter() {
        print!("{}", format_member(dwarf, member, 0)?);
    }
    println!("}};");
    Ok(())
}
