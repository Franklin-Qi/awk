// This file is part of the uutils awk package.
//
// For the full copyright and license information, please view the LICENSE
// files that was distributed with this source code.

use std::{convert::Infallible, ffi::OsString, path::PathBuf};

use clap::{ArgAction, Parser, ValueEnum};

#[derive(Parser, Debug)]
#[clap(version, name = "uutils AWK")]
#[clap(about = ::std::concat!("uutils awk ", ::std::env!("CARGO_PKG_VERSION")))]
pub struct Args {
    #[arg(required_unless_present_any = ["file", "source"])]
    pub code: Option<OsString>,
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, value_parser = parse_kv)]
    pub operands: Vec<(String, String)>,
    #[arg(short = 'f', long)]
    pub file: Vec<PathBuf>,
    #[arg(short = 'F', long)]
    pub field_separator: Option<OsString>,
    #[arg(short = 'v', long, value_parser = parse_kv)]
    pub assign: Vec<(String, String)>,
    #[arg(short = 'b', long)]
    pub characters_as_bytes: bool,
    #[arg(short = 'c', long)]
    pub traditional: bool,
    #[arg(short = 'C', long)]
    pub copyright: bool,
    #[arg(
        short = 'd',
        long,
        num_args = 0..=1,
        default_missing_value = "./awkvars.out",
        help = "[default value, if empty: `./awkvars.out`]"
    )]
    pub dump_variables: Option<PathBuf>,
    #[arg(short = 'D', long, num_args = 0..=1, default_missing_value = "", value_parser = parse_debug_mode)]
    pub debug: Option<DebugMode>,
    #[arg(short = 'e', long)]
    pub source: Vec<OsString>,
    #[arg(short = 'E', long)]
    pub exec: Option<PathBuf>,
    #[arg(short = 'g', long)]
    pub gen_pot: bool,
    #[arg(short = 'i', long)]
    pub include: Vec<PathBuf>,
    #[arg(short = 'I', long)]
    pub trace: bool,
    #[arg(short = 'l', long)]
    pub load: Vec<OsString>,
    #[arg(short = 'L', long, num_args = 0..=1, default_missing_value = "basic", value_enum)]
    pub lint: Vec<LintMode>,
    #[arg(short = 'M', long)]
    pub bignum: bool,
    #[arg(short = 'n', long)]
    pub non_decimal_data: bool,
    #[arg(short = 'N', long)]
    pub use_lc_numeric: bool,
    #[arg(short = 'o',
         long,
         num_args = 0..=1,
         default_missing_value = "./awkprof.out",
         help = "[default value, if empty: `./awkprof.out`]"
     )]
    pub pretty_print: Option<PathBuf>,
    #[arg(short = 'O', long, default_value_t = true, action = ArgAction::SetTrue)]
    pub optimize: bool,
    #[arg(short = 's', long = "no-optimize")]
    pub no_optimize: bool,
    #[arg(short = 'p',
        long,
        num_args = 0..=1,
        default_missing_value = "./awkprof.out",
        help = "[default value, if empty: `./awkprof.out`]"
    )]
    pub profile: Option<PathBuf>,
    #[arg(short = 'P', long)]
    pub posix: bool,
    #[arg(short = 'r', long, default_value_t = true, action = ArgAction::SetTrue)]
    pub re_interval: bool,
    #[arg(short = 'S', long)]
    pub sandbox: bool,
    #[arg(short = 't', long)]
    pub lint_old: bool,
    #[arg(short = 'k', long, conflicts_with = "posix")]
    pub csv: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum LintMode {
    Basic,
    Fatal,
    Invalid,
    #[value(name = "no-ext")]
    NoExt,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DebugMode {
    Interactive,
    File(PathBuf),
}

#[allow(clippy::unnecessary_wraps)]
fn parse_debug_mode(s: &str) -> Result<DebugMode, Infallible> {
    if s.is_empty() {
        Ok(DebugMode::Interactive)
    } else {
        Ok(DebugMode::File(s.into()))
    }
}

fn parse_kv(s: &str) -> Result<(String, String), String> {
    let (k, v) = s.split_once('=').ok_or("expected key=value")?;
    Ok((k.to_string(), v.to_string()))
}
