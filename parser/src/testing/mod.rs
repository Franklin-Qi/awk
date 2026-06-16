// This file is part of the uutils awk package.
//
// For the full copyright and license information, please view the LICENSE
// files that was distributed with this source code.

pub mod ast_gen;

use std::fmt::Write;

use bumpalo::Bump;

use crate::{Ast, Lexer, Parser, Result};

/// Canonical `Debug` fingerprint used to compare ASTs across round-trips.
pub fn ast_signature(ast: &Ast<'_>) -> String {
    let mut out = String::new();
    for load in &ast.loads {
        let _ = writeln!(out, "load:{load:?}");
    }
    for body in &ast.begin {
        let _ = writeln!(out, "begin:{body:?}");
    }
    for body in &ast.begin_file {
        let _ = writeln!(out, "begin_file:{body:?}");
    }
    for rule in &ast.rules {
        let pattern = rule.pattern.as_ref().map(|p| format!("{p:?}"));
        let actions = rule.actions.as_ref().map(|a| format!("{a:?}"));
        let _ = writeln!(out, "rule:({pattern:?},{actions:?})");
    }
    for rule in &ast.concurrent {
        let pattern = rule.pattern.as_ref().map(|p| format!("{p:?}"));
        let actions = rule.actions.as_ref().map(|a| format!("{a:?}"));
        let _ = writeln!(out, "concurrent:({pattern:?},{actions:?})");
    }
    for body in &ast.end_file {
        let _ = writeln!(out, "end_file:{body:?}");
    }
    for body in &ast.end {
        let _ = writeln!(out, "end:{body:?}");
    }
    for (name, fun) in ast.functions.iter() {
        let _ = writeln!(out, "function:{name:?}:{fun:?}");
    }
    out
}

pub fn parse_top<'a>(source: &'a str, arena: &'a Bump) -> Result<&'a Ast<'a>> {
    let parser = arena.alloc(Parser::new(arena));
    parser.parse_top(&mut Lexer::new(source.as_bytes(), arena), true)
}

/// Pretty-print `ast`, parse the result, and require both signatures to match.
pub fn roundtrip_ast(ast: &Ast<'_>) -> Result<(), String> {
    let mut printed = String::new();
    write!(printed, "{ast}").map_err(|e| format!("display failed: {e}"))?;

    let arena = Bump::new();
    let reparsed = parse_top(&printed, &arena).map_err(|e| format!("re-parse failed: {e}"))?;

    let before = ast_signature(ast);
    let after = ast_signature(reparsed);
    if before != after {
        return Err(format!(
            "AST mismatch after round-trip\nprinted:\n{printed}\nbefore:\n{before}\nafter:\n{after}"
        ));
    }
    Ok(())
}

pub fn roundtrip_source(source: &str) -> Result<(), String> {
    let arena = Bump::new();
    let ast = parse_top(source, &arena).map_err(|e| format!("initial parse failed: {e}"))?;
    roundtrip_ast(ast)
}
