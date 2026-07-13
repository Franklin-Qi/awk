// This file is part of the uutils awk package.
//
// For the full copyright and license information, please view the LICENSE
// files that was distributed with this source code.

// static POSIX: bool = false;

mod cli;
mod utils;

use std::{env::args_os, fs};

use bumpalo::Bump;
use clap::Parser as _;
use color_eyre::Result;
use interpreter::{CodeGen, ExecMode, Interpreter};
use parser::Parser;

use crate::{
    cli::{Args, KeyValue},
    utils::{ensure_consistent_panic, exit_err},
};

fn main() {
    if let Err(e) = ensure_consistent_panic(uu_main) {
        exit_err(Some(e))
    }
}

#[tracing::instrument]
fn uu_main() -> Result<()> {
    let args = match Args::try_parse_from(args_os()) {
        Ok(args) => args,
        Err(msg) => {
            msg.print()?;
            exit_err(Option::<&str>::None)
        }
    };

    let rt_arena = Bump::with_capacity(4000); // 4KB minus metadata-ish
    let mut cg = {
        let ast_arena = Bump::with_capacity(4000);
        let code = args.code.unwrap(); // TODO: handle other forms of code input.
        let mut parser = Parser::new(&ast_arena, args.pretty_print.is_some());
        let ast = match parser.parse("CLI", code.as_encoded_bytes()) {
            Ok(ast) => ast,
            Err((report, source)) => {
                report.eprint(("CLI", source)).unwrap();
                return Ok(());
            }
        };

        if let Some(file) = args.pretty_print {
            fs::write(file, format!("{ast}"))?;
        }

        let mut cg = CodeGen::new(&rt_arena);
        cg.lower_ast(ast);
        cg
    };

    for KeyValue { .. } in args.assign {
        todo!()
    }

    let bc = cg.bytecode();
    let mut intrp = Interpreter::new(ExecMode::Uu, cg);

    intrp.run_chunk(bc.begin_code()).unwrap();

    Ok(())
}
