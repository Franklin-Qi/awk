use std::fmt::{self, Display};

use bumpalo::{Bump, collections::Vec};
use hashbrown::{DefaultHashBuilder, HashMap};
use indexmap::{IndexMap, IndexSet};
use parser::Identifier;

use crate::ir::{
    NonLocal, OpCode, Reg,
    lower::{Bytecode, Code},
};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Value(pub f64); // TODO: use NaN-boxing.

#[derive(Debug)]
pub enum ExecMode {
    Uu,
    Gnu,
    Posix,
}

#[derive(Debug)]
pub struct Interpreter<'a> {
    arena: &'a Bump,
    bc: Bytecode<'a>,
    program_counter: usize,
    registers: Registers<'a>,
    symbols: SymbolTable<'a>,
    consts: Consts,
    compat: ExecMode,
}

#[derive(Debug)]
pub struct Registers<'a>(Vec<'a, Value>);

#[derive(Debug)]
pub struct SymbolTable<'a> {
    user: IndexMap<Identifier<'a>, Value>,
    // separate table for cheap invalidation. It's an arena _visibly shrugs_.
    records: HashMap<usize, Value, DefaultHashBuilder, &'a Bump>,
    // etc
}

#[derive(Debug)]
pub struct Consts(pub IndexSet<Value>);

impl<'a> Interpreter<'a> {
    pub fn new(compat: ExecMode, code: Code<'a>) -> Self {
        Self {
            arena: code.arena,
            bc: code.bc,
            program_counter: 0,
            registers: Registers(bumpalo::vec![in code.arena; Value(0.); 8]),
            symbols: code.symbols,
            consts: code.consts,
            compat,
        }
    }
}

impl<'a> SymbolTable<'a> {
    pub fn new_in(arena: &'a Bump) -> Self {
        Self {
            user: IndexMap::new(),
            records: HashMap::new_in(arena),
        }
    }
    fn lookup_user_var(&self, var: NonLocal) -> &Value {
        self.user.get_index(var.0 as _).unwrap().1
    }

    fn write_user_val(&mut self, var: NonLocal, value: &Value) {
        *self.user.get_index_mut(var.0 as _).unwrap().1 = Value::clone(value);
    }

    pub fn register_user_var(&mut self, var: &Identifier, bump: &'a Bump) -> NonLocal {
        if let Some(index) = self.user.get_index_of(var) {
            NonLocal(index as _)
        } else {
            let ident = Identifier {
                namespace: bump.alloc_str(var.namespace),
                literal: bump.alloc_str(var.literal),
            };
            NonLocal(self.user.insert_full(ident, Value(0.)).0 as _)
        }
    }
}

impl Consts {
    pub fn new() -> Self {
        Self(IndexSet::with_capacity(4))
    }
}

impl Interpreter<'_> {
    pub fn run(&mut self) {
        while let Some(instr) = self.bc.code.get(self.program_counter) {
            match instr {
                // ix if let Some(&(dest, src)) = ix.get_unary() => {}
                ix if let Some(&(dest, lhs, rhs)) = ix.get_binary() => {
                    let lhs = self.registers.read(lhs);
                    let rhs = self.registers.read(rhs);
                    let val = match ix.opcode {
                        OpCode::Add => Value(lhs.0 + rhs.0),
                        OpCode::Subtract => Value(lhs.0 - rhs.0),
                        OpCode::Multiply => Value(lhs.0 * rhs.0),
                        OpCode::Divide => Value(lhs.0 / rhs.0),
                        _ => todo!(),
                    };
                    self.registers.write(dest, &val);
                }
                ix if let Some(&(dest, src)) = ix.get_load_store() => match ix.opcode {
                    OpCode::LoadConst => self
                        .registers
                        .write(dest, self.consts.0.get_index(src.0 as _).unwrap()),
                    OpCode::LoadUser => {
                        self.registers
                            .write(dest, self.symbols.lookup_user_var(src));
                    }
                    OpCode::StoreUser => {
                        self.symbols.write_user_val(src, self.registers.read(dest));
                    }
                    _ => todo!(),
                },
                ix if let Some((cond, true_to, false_to)) = ix.get_branch() => {
                    let label = if self.registers.read(*cond).0 == 0. {
                        false_to.0
                    } else {
                        true_to.0
                    };
                    self.program_counter = label as _;
                    continue;
                }
                ix if let Some(label) = ix.get_jump() => {
                    self.program_counter = label.0 as _;
                    continue;
                }
                ix => todo!("{ix:?}"),
            }
            self.program_counter += 1;
        }
    }
}

impl Registers<'_> {
    fn read(&self, src: Reg) -> &Value {
        &self.0[src.0 as usize]
    }
    fn write(&mut self, dest: Reg, src: &Value) {
        self.0[dest.0 as usize] = Value::clone(src);
    }
}

impl Display for Interpreter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}\n", self.bc)?;
        writeln!(f, "{}\n", self.registers)?;
        writeln!(f, "{}\n", self.symbols)?;
        write!(f, "{}", self.consts)
    }
}

impl Display for Code<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}\n", self.bc)?;
        writeln!(f, "{}\n", self.symbols)?;
        write!(f, "{}", self.consts)
    }
}

impl Display for Bytecode<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bytecode:")?;
        let n = self.code.len().checked_ilog10().unwrap_or(0) as usize + 1;
        fmt_list(f, self.code.iter(), |f, i, e| write!(f, "{i:0n$}: {e}"))
    }
}

impl Display for Registers<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Registers:")?;
        let n = self.0.len().checked_ilog10().unwrap_or(0) as usize + 1;
        fmt_list(f, self.0.iter(), |f, i, e| write!(f, "r{i:0n$} = {e:?}"))
    }
}

impl Display for SymbolTable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Symbols:")?;
        fmt_list(f, self.user.iter(), |f, i, (k, v)| {
            write!(f, "user[{i}] @ {k} = {v:?}")
        })
    }
}

impl Display for Consts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Consts:")?;
        fmt_list(f, self.0.iter(), |f, i, e| write!(f, "mem[{i}] = {e:?}"))
    }
}

fn fmt_list<'a, T: Copy>(
    f: &mut fmt::Formatter<'a>,
    iter: impl Iterator<Item = T>,
    cb: impl Fn(&mut fmt::Formatter<'a>, usize, T) -> fmt::Result,
) -> fmt::Result {
    for (i, e) in iter.enumerate() {
        write!(f, "\n  ")?;
        cb(f, i, e)?;
    }
    Ok(())
}
