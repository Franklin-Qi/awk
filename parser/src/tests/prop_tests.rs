// This file is part of the uutils awk package.
//
// For the full copyright and license information, please view the LICENSE
// files that was distributed with this source code.

use std::fmt::Write;

use bumpalo::Bump;
use proptest::prelude::*;

use crate::tests::{
    ast_gen::{self, GenAtom, GenBody, GenExpr, GenPattern, GenProgram, GenRule, GenStatement},
    utils::{roundtrip_ast, roundtrip_source},
};

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 128,
        .. ProptestConfig::default()
    })]

    #[test]
    fn test_parser_roundtrip_generated_program(program in ast_gen::gen_program()) {
        let arena = Bump::new();
        let ast = ast_gen::materialize(&program, &arena);
        roundtrip_ast(&ast).map_err(TestCaseError::fail)?;
    }

    #[test]
    fn test_parser_roundtrip_generated_source(program in ast_gen::gen_program()) {
        let arena = Bump::new();
        let ast = ast_gen::materialize(&program, &arena);
        let mut source = String::new();
        write!(source, "{ast}").map_err(|e| TestCaseError::fail(format!("display failed: {e}")))?;
        roundtrip_source(&source).map_err(TestCaseError::fail)?;
    }
}

#[test]
fn test_parser_roundtrip_smoke_cases() {
    let cases = [
        "{ print 1 + a }",
        "BEGIN { print 1 }",
        "{ if (a) print; else print 0 }",
        "{ while (a < 10) a++ }",
        "{ a = 1; b = a + 2 }",
        "/pat/ { print $1, $2 }",
        "BEGIN { print \"hi\" }",
    ];
    for source in cases {
        roundtrip_source(source)
            .unwrap_or_else(|e| panic!("round-trip failed for {source:?}: {e}"));
    }
}

#[test]
fn test_parser_roundtrip_handwritten_ast() {
    let program = GenProgram {
        rules: vec![
            GenRule {
                pattern: Some(GenPattern::Regex(0)),
                body: GenBody {
                    statements: vec![GenStatement::Print(vec![GenExpr::Atom(GenAtom::Var(0))])],
                },
            },
            GenRule {
                pattern: None,
                body: GenBody {
                    statements: vec![
                        GenStatement::If {
                            condition: GenExpr::Binary(
                                crate::ast::BinaryOperator::Lt,
                                Box::new(GenExpr::Atom(GenAtom::Var(0))),
                                Box::new(GenExpr::Atom(GenAtom::SmallInt(10))),
                            ),
                            then_body: GenBody {
                                statements: vec![GenStatement::Print(vec![GenExpr::Atom(
                                    GenAtom::SmallInt(1),
                                )])],
                            },
                            else_body: None,
                        },
                        GenStatement::Print(vec![GenExpr::Binary(
                            crate::ast::BinaryOperator::Add,
                            Box::new(GenExpr::Atom(GenAtom::SmallInt(1))),
                            Box::new(GenExpr::Atom(GenAtom::Var(0))),
                        )]),
                        GenStatement::Expr(GenExpr::Binary(
                            crate::ast::BinaryOperator::Eq,
                            Box::new(GenExpr::Atom(GenAtom::SmallInt(0))),
                            Box::new(GenExpr::Atom(GenAtom::SmallInt(1))),
                        )),
                    ],
                },
            },
        ],
        begin: Some(GenBody {
            statements: vec![GenStatement::Print(vec![GenExpr::Atom(GenAtom::SmallInt(
                0,
            ))])],
        }),
    };

    let arena = Bump::new();
    let ast = ast_gen::materialize(&program, &arena);
    roundtrip_ast(&ast).expect("hand-written AST should round-trip");
}
