//! Generate README.md from doc comments.
//!
//! Cargo subcommand that extract documentation from your crate's doc comments that you can use to
//! populate your README.md.
//!
//! # Installation
//!
//!     cargo install cargo-readme
//!
//! # Motivation
//!
//! As you write documentation, you often have to show examples of how to use your software. But
//! how do you make sure your examples are all working properly? That we didn't forget to update
//! them after a braking change and left our (possibly new) users with errors they will have to
//! figure out by themselves?
//!
//! With `cargo-readme`, you just write the rustdoc, run the tests, and then run:
//!
//!     cargo readme > README.md
//!
//! And that's it! Your `README.md` is populated with the contents of the doc comments from your
//! `lib.rs` (or `main.rs`).
//!
//! # Usage
//!
//! Let's take the following rust doc:
//!
//!     //! This is my awesome crate
//!     //!
//!     //! Here goes some other description of what it is and what is does
//!     //!
//!     //! # Examples
//!     //! ```
//!     //! fn sum2(n1: i32, n2: i32) -> i32 {
//!     //!   n1 + n2
//!     //! }
//!     //! # assert_eq!(4, sum2(2, 2));
//!     //! ```
//!
//! Running `cargo readme` will output the following:
//!
//!     # my_crate
//!
//!     This is my awesome crate
//!
//!     Here goes some other description of what it is and what is does
//!
//!     ## Examples
//!     ```rust
//!     fn sum2(n1: i32, n2: i32) -> i32 {
//!       n1 + n2
//!     }
//!     ```
//!
//!     License: MY_LICENSE
//!
//! Let's see what's happened:
//! - the crate name ("my-crate") was added at the top
//! - "# Examples" heading became "## Examples"
//! - code block became "```rust"
//! - hidden line `# assert_eq!(4, sum2(2, 2));` was removed
//!
//! `cargo-readme` also supports multiline doc comments `/*! */` (but you cannot mix styles):
//!
//!     /*!
//!     This is my awesome crate
//!
//!     Here goes some other description of what it is and what is does
//!
//!     # Examples
//!     ```
//!     fn sum2(n1: i32, n2: i32) -> i32 {
//!       n1 + n2
//!     }
//!     # assert_eq!(4, sum2(2, 2));
//!     ```
//!     */
//!
//! If you have additional information that does not fit in doc comments, you can use a template.
//! Just create a file called `README.tpl` in the same directory as `Cargo.toml` with the following
//! content:
//!
//!     Badges here
//!
//!     # {{crate}}
//!
//!     {{readme}}
//!
//!     Some additional info here
//!
//!     License: {{license}}
//!
//! The output will look like this
//!
//!     Badges here
//!
//!     # my_crate
//!
//!     This is my awesome crate
//!
//!     Here goes some other description of what it is and what is does
//!
//!     ## Examples
//!     ```rust
//!     fn sum2(n1: i32, n2: i32) -> i32 {
//!       n1 + n2
//!     }
//!     ```
//!
//!     Some additional info here
//!
//!     License: MY_LICENSE
//!
//! By default, `README.tpl` will be used as the template, but you can override it using the
//! `--template` to choose a different template or `--no-template` to disable it.

#[macro_use] extern crate clap;

extern crate cargo_readme;

use std::io::{self, Write};

use clap::{Arg, ArgMatches, App, AppSettings, SubCommand};

use cargo_readme::cargo_info;

mod helper;

fn main() {
    let matches = App::new("cargo-readme")
        .version(&*format!("v{}", crate_version!()))
        // We have to lie about our binary name since this will be a third party
        // subcommand for cargo but we want usage strings to generated properly
        .bin_name("cargo")
        // Global version uses the version we supplied (Cargo.toml) for all subcommands as well
        .settings(&[AppSettings::GlobalVersion, AppSettings::SubcommandRequired])
        // We use a subcommand because everything parsed after `cargo` is sent to the third party
        // plugin which will then be interpreted as a subcommand/positional arg by clap
        .subcommand(SubCommand::with_name("readme")
            .author("Livio Ribeiro <livioribeiro@outlook.com>")
            .about("Generate README.md from doc comments")
            .arg(Arg::with_name("INPUT")
                .short("i")
                .long("input")
                .takes_value(true)
                .help("File to read from.{n}\
                       If not provided, will try to use `src/main.rs`, then `src/lib.rs`. If \
                       neither file could be found, will look into `Cargo.toml` for a `[lib]`, \
                       then for a single `[[bin]]`. If multiple binaries are found, you will be \
                       asked to choose one."))
            .arg(Arg::with_name("OUTPUT")
                .short("o")
                .long("output")
                .takes_value(true)
                .help("File to write to. If not provided, will output to stdout."))
            .arg(Arg::with_name("ROOT")
                .short("r")
                .long("project-root")
                .takes_value(true)
                .help("Directory to be set as project root (where `Cargo.toml` is){n}\
                       Defaults to the current directory."))
            .arg(Arg::with_name("TEMPLATE")
                .short("t")
                .long("template")
                .takes_value(true)
                .conflicts_with("NO_TEMPLATE")
                .help("Template used to render the output.{n}\
                       Default behavior is to use `README.tpl` if it exists."))
            .arg(Arg::with_name("NO_TITLE")
                .long("no-title")
                .help("Do not prepend title line.{n}\
                       By default, the title ('# crate-name') is prepended to the output. If a \
                       template is used and it contains the tag '{{crate}}', the template takes \
                       precedence and this option is ignored."))
            .arg(Arg::with_name("NO_LICENSE")
                .long("no-license")
                .help("Do not append license line. By default, the license, if defined in \
                       `Cargo.toml`, will be prepended to the output. If a template is used \
                       and it contains the tag '{{license}}', the template takes precedence and \
                       this option is ignored."))
            .arg(Arg::with_name("NO_TEMPLATE")
                .long("no-template")
                .help("Ignore template file when generating README.{n}\
                       Only useful to ignore default template `README.tpl`."))
            .arg(Arg::with_name("NO_INDENT_HEADINGS")
                .long("no-indent-headings")
                .help("Do not add an extra level to headings.{n}\
                       By default, '#' headings become '##', so the first '#' can be the crate \
                       name. Use this option to prevent this behavior.{n}")))
        .get_matches();

    if let Some(m) = matches.subcommand_matches("readme") {
        match execute(m) {
            Err(e) => {
                io::stderr()
                    .write_fmt(format_args!("Error: {}\n", e))
                    .expect("An error occurred while trying to show an error message");
                std::process::exit(1);
            }
            _ => {}
        }
    }
}

/// Takes the arguments matches from clap and outputs the result, either to stdout of a file
fn execute(m: &ArgMatches) -> Result<(), String> {
    // get inputs
    let input = m.value_of("INPUT");
    let output = m.value_of("OUTPUT");
    let template = m.value_of("TEMPLATE");
    let add_title = !m.is_present("NO_TITLE");
    let add_license = !m.is_present("NO_LICENSE");
    let no_template = m.is_present("NO_TEMPLATE");
    let indent_headings = !m.is_present("NO_INDENT_HEADINGS");

    // get project root
    let project_root = helper::get_project_root(m.value_of("ROOT"))?;

    // get source file
    let mut source = helper::get_source(&project_root, input)?;

    // get destination file
    let mut dest = helper::get_dest(&project_root, output)?;

    // get template file
    let mut template_file = if no_template {
        None
    } else {
        helper::get_template_file(&project_root, template)?
    };

    // generate output
    let readme = cargo_readme::generate_readme(
        &project_root,
        &mut source,
        template_file.as_mut(),
        add_title,
        add_license,
        indent_headings,
    )?;

    helper::write_output(&mut dest, readme)
}
