extern crate assert_cli;

use assert_cli::Assert;

#[test]
fn main() {
    let args = [
        "readme",
        "--project-root",
        "tests/entrypoint-resolution/main",
        "--no-title",
        "--no-license"
    ];

    Assert::main_binary()
        .with_args(&args)
        .succeeds()
        .prints_exactly("main")
        .unwrap();
}

#[test]
fn lib() {
    let args = [
        "readme",
        "--project-root",
        "tests/entrypoint-resolution/lib",
        "--no-title",
        "--no-license"
    ];

    Assert::main_binary()
        .with_args(&args)
        .succeeds()
        .prints_exactly("lib")
        .unwrap();
}

#[test]
fn cargo_lib() {
    let args = [
        "readme",
        "--project-root",
        "tests/entrypoint-resolution/cargo-lib",
        "--no-title",
        "--no-license"
    ];

    Assert::main_binary()
        .with_args(&args)
        .succeeds()
        .prints_exactly("cargo lib")
        .unwrap();
}

#[test]
fn cargo_bin() {
    let args = [
        "readme",
        "--project-root",
        "tests/entrypoint-resolution/cargo-bin",
        "--no-title",
        "--no-license"
    ];

    Assert::main_binary()
        .with_args(&args)
        .succeeds()
        .prints_exactly("cargo bin")
        .unwrap();
}
