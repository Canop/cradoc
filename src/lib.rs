//!
//! **cradoc** (a *mot-valise* for *crate* + *documentation*)
//! leverages Rust's documentation generation to automatically
//! maintain other documents up to date.
//!
//! Actually, in the current version, it does only one thing: ensuring
//! markdown files in your repo (eg the README.md) contain an up-to-date
//! version of the crate documentation.
//!
//! Insert `<!-- cradoc -->` in your markdown file where you want
//! the crate documentation to appear, run `cradoc` in your repo,
//! and the markdown file will be updated accordingly.
//!
//! There are several previous crates with the same exact feature, most
//! especially [cargo-readme](https://crates.io/crates/cargo-rdme).
//! It's very probable the crate you need is one of those previous ones.
//!
//! The main reason I made this alternative is that cargo-readme does not
//! currently properly handle intra-links (I guess this is being worked
//! on, don't discard this much more mature crate).
//!
//! To get an example of what cradoc produces, head up to
//! [the readme of lazy-regex](https://github.com/Canop/lazy-regex).
//! This README full of links is made from the crate's lib.rs which
//! is also used for the [crate documentation](https://docs.rs/lazy-regex).
//!
//! (other features should come later)

mod cli;
mod context;
mod error;
mod html;
mod json;

pub use {
    cli::*,
    context::*,
    error::*,
    html::*,
    json::*,
};

#[macro_use]
extern crate cli_log;
