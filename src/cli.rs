// This file is part of the uutils awk package.
//
// For the full copyright and license information, please view the LICENSE
// files that was distributed with this source code.

use std::{ffi::OsString, path::PathBuf};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, name = "uutils AWK")]
#[clap(about = ::std::concat!("uutils awk ", ::std::env!("CARGO_PKG_VERSION")))]
pub struct Args {
    // POSIX
    pub code: OsString,
    #[arg(short = 'f', long)]
    pub file: Option<PathBuf>,
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
    #[arg(short = 'd', long)]
    pub dump_variables: Option<PathBuf>,
    #[arg(short = 'D', long)]
    pub debug: Option<PathBuf>,
    #[arg(short = 'e', long)]
    pub source: Vec<u8>,
    #[arg(short = 'E', long)]
    pub exec: Option<PathBuf>,
    #[arg(short = 'g', long)]
    pub gen_pot: bool,
    #[arg(short = 'i', long)]
    pub include: Option<PathBuf>,
    #[arg(short = 'I', long)]
    pub trace: bool,
    #[arg(short = 'l', long)]
    pub load: Vec<OsString>,
    #[arg(short = 'L', long)]
    pub lint: Vec<String>,
    #[arg(short = 'M', long)]
    pub bignum: bool,
    #[arg(short = 'n', long)]
    pub non_decimal_data: bool,
    #[arg(short = 'N', long)]
    pub use_lc_numeric: bool,
    #[arg(short = 'o', long, num_args = 0..=1, default_missing_value = "./awkprof.out")]
    pub pretty_print: Option<PathBuf>,
    #[arg(short = 'O', long, default_value_t = true)]
    pub optimize: bool,
    #[arg(short = 's', long = "no-optimize")]
    pub no_optimize: bool,
    #[arg(short = 'p', num_args = 0..=1, long)]
    pub profile: Option<PathBuf>,
    #[arg(short = 'P', long)]
    pub posix: bool,
    #[arg(short = 'r', long, default_value_t = true)]
    pub re_interval: bool,
    #[arg(short = 'S', long)]
    pub sandbox: bool,
    #[arg(short = 't', long)]
    pub lint_old: bool,
}

fn parse_kv(s: &str) -> Result<(String, String), String> {
    let (k, v) = s.split_once('=').ok_or("expected key=value")?;
    Ok((k.to_string(), v.trim_matches(['"', '\'']).to_string()))
}
