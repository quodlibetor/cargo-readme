extern crate assert_cli;

use assert_cli::Assert;

const EXPECTED: &str = "Error: Multiple binaries found, choose one: [src/entry1.rs, src/entry2.rs]";

#[test]
fn test() {
    let args = ["readme", "--project-root", "tests/multiple-bin-fail"];

    Assert::main_binary()
        .with_args(&args)
        .fails()
        .prints_error(EXPECTED)
        .unwrap();
}
