use crate::ucmd;

#[test]
fn empty_program_succeeds() {
    ucmd().arg("").succeeds();
}

#[test]
fn print_first_field() {
    ucmd().args(&["{ print $1 }"]).succeeds();
}

#[test]
fn no_args_fails_code_one() {
    ucmd().fails_with_code(1);
}
