// This file is part of the uutils awk package.
//
// For the full copyright and license information, please view the LICENSE
// files that was distributed with this source code.

use crate::ucmd;

#[test]
fn empty_program_succeeds() {
    ucmd().arg("").succeeds();
}

// #[test]
// fn print_first_field() {
//     ucmd().args(&["{ print $1 }"]).succeeds();
// }

#[test]
fn no_args_fails_code_one() {
    ucmd().fails_with_code(1);
}

#[test]
fn switch_default_in_middle_fallthrough() {
    ucmd()
        .arg("BEGIN { x = 1; switch (x) { case 1: print 1; default: print 2; case 3: print 3 } }")
        .succeeds()
        .stdout_only("1\n2\n3\n");
    ucmd()
        .arg("BEGIN { x = 2; switch (x) { case 1: print 1; default: print 2; case 3: print 3 } }")
        .succeeds()
        .stdout_only("2\n3\n");
}

#[test]
fn switch_matches_integer_case_with_fallthrough() {
    ucmd()
        .arg("BEGIN { x = 1; switch (x) { case 1: print \"one\"; default: print \"def\" } }")
        .succeeds()
        .stdout_only("one\ndef\n");
}

#[test]
fn switch_falls_through_to_default() {
    ucmd()
        .arg("BEGIN { x = 9; switch (x) { case 1: print 1; default: print 2 } }")
        .succeeds()
        .stdout_only("2\n");
}

#[test]
fn switch_default_first_still_tests_later_cases() {
    ucmd()
        .arg("BEGIN { x = 3; switch (x) { default: print 2; case 3: print 3 } }")
        .succeeds()
        .stdout_only("3\n");
    ucmd()
        .arg("BEGIN { x = 4; switch (x) { default: print 2; case 3: print 3 } }")
        .succeeds()
        .stdout_only("2\n3\n");
}

#[test]
fn switch_string_case_match_with_fallthrough() {
    ucmd()
        .arg("BEGIN { x = \"a\"; switch (x) { case \"a\": print \"match\"; default: print \"no\" } }")
        .succeeds()
        .stdout_only("match\nno\n");
}

#[test]
fn switch_regex_case_match() {
    ucmd()
        .arg("BEGIN { x = \"abc\"; switch (x) { case /bc/: print \"match\" } }")
        .succeeds()
        .stdout_only("match\n");
}

#[test]
fn switch_no_match_without_default_continues() {
    ucmd()
        .arg("BEGIN { x = 2; switch (x) { case 1: print 1 }; print \"done\" }")
        .succeeds()
        .stdout_only("done\n");
}

#[test]
fn short_circuit_and_or_truth_values() {
    ucmd()
        .arg("BEGIN { print (0 && 1); print (1 && 2); print (1 || 0); print (0 || 5) }")
        .succeeds()
        .stdout_only("0\n1\n1\n1\n");
}

#[test]
fn short_circuit_and_in_if_condition() {
    ucmd()
        .arg("BEGIN { if (1 && 0) print 1; else print 0 }")
        .succeeds()
        .stdout_only("0\n");
}

#[test]
fn short_circuit_or_in_if_condition() {
    ucmd()
        .arg("BEGIN { if (1 || 0) print 1; else print 0 }")
        .succeeds()
        .stdout_only("1\n");
}

#[test]
fn short_circuit_chained_and() {
    ucmd()
        .arg("BEGIN { a=1; b=1; c=3; print (a && b && c == 3) }")
        .succeeds()
        .stdout_only("1\n");
    ucmd()
        .arg("BEGIN { a=1; b=0; c=3; print (a && b && c == 3) }")
        .succeeds()
        .stdout_only("0\n");
}

#[test]
fn short_circuit_and_skips_rhs_side_effects() {
    ucmd()
        .arg("BEGIN { i=0; print (0 && ++i); print i }")
        .succeeds()
        .stdout_only("0\n0\n");
}

#[test]
fn short_circuit_or_skips_rhs_side_effects() {
    ucmd()
        .arg("BEGIN { i=0; print (1 || ++i); print i }")
        .succeeds()
        .stdout_only("1\n0\n");
}

// Regression test for issue #5: writing to /dev/full must not panic.
#[cfg(target_os = "linux")]
#[test]
fn write_to_dev_full_does_not_panic() {
    use std::{
        fs::OpenOptions,
        process::{Command, Stdio},
    };

    let Ok(dev_full) = OpenOptions::new().write(true).open("/dev/full") else {
        return; // /dev/full not available; skip.
    };
    let output = Command::new(super::TESTS_BINARY)
        .arg("BEGIN { print 1 }")
        .stdout(Stdio::from(dev_full))
        .stderr(Stdio::piped())
        .output()
        .expect("failed to spawn awk");
    // Must not panic (panic exits with code 2).
    assert_ne!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("panicked"),
        "awk panicked on write to /dev/full: stderr={stderr}"
    );
}
