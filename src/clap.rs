//! clap setup.
//!
//! clap is a crate for command line argument parsing.
//! See https://docs.rs/clap/
//!
//! Clap has a variety of setup approachs:
//!
//!   * via typical functions, which favors advanced uses yet is verbose.
//!   * via usage strings, which looks more like writing documentation.
//1   * via macros, which is fast and less verbose, yet atypical to read.
//!   * via YAML file, which favors localization and text file readability.
//!
//! We prefer the typical functions, because they provide maximum capability,
//! and in our experience are the easiest for Rust IDEs to read and debug.
//!
//! We favor our convention of doing clap setup in a file named `clap.rs`,
//! rather than in `main.rs`, because we favor the separation of concerns.

use clap::{Arg, App};
use crate::args::Args;

/// Create a clap app.
pub fn app() -> App<'static> {
    App::new("checkline")
    .version("1.1.0")
    .author("Joel Parker Henderson <joel@joelparkerhenderson.com>")
    .about("Checkbox line picker for stdin line input")
    .arg(Arg::new("verbose")
        .short('v')
        .long("verbose")
        .multiple(true)
        .help("Set the verbosity level"))
}
/// Create an Args struct initiatied with the clap App settings.
pub fn args() -> Args {
    let matches = app().get_matches();
    Args {
        verbose: std::cmp::max(3, matches.occurrences_of("verbose") as u8),
    }
}