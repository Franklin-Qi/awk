// This file is part of the uutils awk package.
//
// For the full copyright and license information, please view the LICENSE
// files that was distributed with this source code.

/// Returns whether `name` is a reserved keyword that cannot appear in a
/// qualified identifier (`ns::name`), matching gawk.
pub fn is_reserved_keyword(name: &str) -> bool {
    matches!(
        name,
        "BEGIN"
            | "END"
            | "if"
            | "else"
            | "switch"
            | "case"
            | "default"
            | "do"
            | "while"
            | "for"
            | "in"
            | "print"
            | "printf"
            | "getline"
            | "next"
            | "nextfile"
            | "exit"
            | "break"
            | "continue"
            | "return"
            | "delete"
            | "function"
            | "func"
    )
}
