//!
//! **cradoc** (a *mot-valise* for *crate* + *documentation*) is a tool
//! leveraging Rust's documentation generation to automatically
//! maintain other documents up to date.
//!
//! Actually, in the current version, it does only one thing: ensuring
//! markdown files in your repo (eg the README.md) contain an up-to-date
//! version of the crate documentation.
//!
//! Just insert `<!-- cradoc -->` in your markdown file where you want
//! the crate documentation to appear, and run `cradoc` in your repo,
//! and the markdown file will be updated accordingly.
//!
//! There are several previous crates with the same exact feature, most
//! especially `cargo-readme`. And it's very probable that the crate
//! you need is one of those existing ones.
//!
//! The reason I made this alternative is that cargo-readme does not
//! currently properly handle intra-links (but I guess this is being worked
//! on, don't discard this much more mature crate).
//!

mod context;
mod cli;
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
