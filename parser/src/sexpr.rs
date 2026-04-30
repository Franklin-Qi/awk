use std::fmt::{Debug, Formatter, Result};

use crate::ast::{Atom, Body, Identifier, Statement, Variable};

const PRETTY_PRINT_INDENT: usize = 2;

fn fmt_vars(f: &mut Formatter<'_>) -> (bool, usize, String) {
    let ni = f.width().unwrap_or(0) + PRETTY_PRINT_INDENT;
    (f.alternate(), ni, " ".repeat(ni))
}

impl Debug for Statement<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let (alt, ni, pad) = fmt_vars(f);

        match self {
            Statement::Expression(expr) => {
                if alt {
                    write!(f, "{expr:#ni$?}")
                } else {
                    write!(f, "{expr:?}")
                }
            }
            Self::Command {
                name,
                args,
                redirection,
            } => {
                if let Some(rx) = redirection {
                    if alt {
                        write!(
                            f,
                            "(redir {rx:?}\n{pad}({name:?}{:#width$?}))",
                            ListLispFmt(args),
                            width = ni
                        )
                    } else {
                        write!(f, "(redir {rx:?} ({name:?}{:?}))", ListLispFmt(args))
                    }
                } else {
                    if alt {
                        write!(f, "({name:?}{:#width$?})", ListLispFmt(args), width = ni)
                    } else {
                        write!(f, "({name:?}{:?}", ListLispFmt(args))
                    }
                }
            }
            Self::If {
                condition,
                then_body,
                else_body,
            } => {
                if alt {
                    write!(f, "(if {condition:?}\n{pad}")?;
                    write!(f, "{then_body:#ni$?}")?;
                    if let Some(else_body) = else_body {
                        write!(f, "\n{pad}Some({else_body:#ni$?})")
                    } else {
                        write!(f, "\n{pad}None)")
                    }
                } else if let Some(else_body) = else_body {
                    write!(f, "(if {condition:?} {then_body:?} Some({else_body:?})")
                } else {
                    write!(f, "(if {condition:?} {then_body:?} None)")
                }
            }
            Self::While {
                condition,
                then_body,
            } => {
                if alt {
                    write!(f, "(while {condition:?}\n{pad}{then_body:#ni$?})")
                } else {
                    write!(f, "(while {condition:?} {then_body:?})")
                }
            }
            Self::DoWhile {
                then_body,
                condition,
            } => {
                if alt {
                    write!(f, "(do-while\n{pad}{then_body:#ni$?}\n{pad}{condition:?})")
                } else {
                    write!(f, "(do-while {then_body:?} {condition:?})")
                }
            }
            Self::For {
                init,
                condition,
                update,
                body,
            } => {
                if alt {
                    write!(
                        f,
                        "(for\n{pad}{init:?}\n{pad}{condition:?}\n{pad}{update:?}\n{pad}{body:#ni$?})"
                    )
                } else {
                    write!(f, "(for {init:?} {condition:?} {update:?} {body:?})")
                }
            }
            Self::ForEach { place, array, body } => {
                if alt {
                    write!(f, "(for-each {place:?} {array:?}\n{pad}{body:#ni$?})")
                } else {
                    write!(f, "(for-each {place:?} {array:?} {body:?})")
                }
            }
            Self::Switch {
                scrutinee,
                branches,
                default,
            } => {
                if alt {
                    if let Some((dx, i)) = default {
                        write!(
                            f,
                            "(switch {scrutinee:?}\n{pad}(cases{:#width$?})\n{pad}Some({dx:?}) {i})",
                            ListLispCasesFmt(branches.as_slice()),
                            width = ni
                        )
                    } else {
                        write!(
                            f,
                            "(switch {scrutinee:?}\n{pad}(cases{:#width$?}))",
                            ListLispCasesFmt(branches),
                            width = ni
                        )
                    }
                } else {
                    if let Some((dx, i)) = default {
                        write!(
                            f,
                            "(switch {scrutinee:?} (cases{:?}) Some({dx:?}) {i})",
                            ListLispCasesFmt(branches.as_slice())
                        )
                    } else {
                        write!(
                            f,
                            "(switch {scrutinee:?} (cases{:?}))",
                            ListLispCasesFmt(branches)
                        )
                    }
                }
            }
            Self::Continue => write!(f, "(continue)"),
            Self::Break => write!(f, "(break)"),
            Self::Return(expr) => {
                if alt {
                    write!(f, "(return {expr:#ni$?})")
                } else {
                    write!(f, "(return {expr:?})")
                }
            }
        }
    }
}

struct ListLispFmt<'a, T: Debug>(&'a [T]);
impl<T: Debug> Debug for ListLispFmt<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let (alt, ni, pad) = fmt_vars(f);
        for e in self.0 {
            if alt {
                write!(f, "\n{pad}{e:#ni$?}")?;
            } else {
                write!(f, " {e:?}")?;
            }
        }
        Ok(())
    }
}

struct ListLispCasesFmt<'a, T: Debug>(&'a [(T, Body<'a>)]);
impl<T: Debug> Debug for ListLispCasesFmt<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let (alt, ni, pad) = fmt_vars(f);
        for (i, e) in self.0 {
            if alt {
                write!(
                    f,
                    "\n{pad}(case {i:?}\n{pad}  {:#width$?})",
                    e,
                    width = ni + PRETTY_PRINT_INDENT
                )?;
            } else {
                write!(f, " (case {i:?} {e:?}")?;
            }
        }
        Ok(())
    }
}

impl Debug for Identifier<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}::{}", self.namespace, self.literal)
    }
}

impl Debug for Body<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let (alt, ni, pad) = fmt_vars(f);
        write!(f, "(body")?;
        for e in &self.0 {
            if alt {
                write!(f, "\n{pad}{e:#ni$?}")?;
            } else {
                write!(f, " {e:?}")?;
            }
        }
        write!(f, ")")
    }
}

impl Debug for Atom<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Variable(var) => write!(f, "{var:?}"),
            Self::String(str) => write!(f, "{str:?}"),
            Self::Number(num) => write!(f, "{num}"),
            Self::Regex(rgx) => write!(f, "/{rgx}/"),
            Self::BigInt() | Self::BigFloat() => unimplemented!(),
        }
    }
}

impl Debug for Variable<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::User(ident) => ident.fmt(f),
            Self::Nr => write!(f, "NR"),
            Self::Nf => write!(f, "NF"),
            Self::Fs => write!(f, "FS"),
            Self::Rs => write!(f, "RS"),
            Self::Ofs => write!(f, "OFS"),
            Self::Ors => write!(f, "ORS"),
            Self::Filename => write!(f, "FILENAME"),
            Self::Argc => write!(f, "ARGC"),
            Self::Argv => write!(f, "ARGV"),
            Self::Subsep => write!(f, "SUBSEP"),
            Self::Fnr => write!(f, "FNR"),
            Self::Argind => write!(f, "ARGIND"),
            Self::Ofmt => write!(f, "OFMT"),
            Self::Rstart => write!(f, "RSTART"),
            Self::Rlength => write!(f, "RLENGTH"),
            Self::Environ => write!(f, "ENVIRON"),
        }
    }
}
