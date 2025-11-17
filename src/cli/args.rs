use {
    clap::{
        CommandFactory,
        Parser,
    },
    std::path::PathBuf,
};

static INTRO: &str = "
cradoc updates any markdown file with a crate's lib documentation.

See https://github.com/Canop/cradoc

";

/// Launch arguments
#[derive(Debug, Parser)]
#[command(
    author,
    about,
    version,
    disable_version_flag = true,
    disable_help_flag = true
)]
pub struct Args {
    /// Print help information
    #[arg(long)]
    pub help: bool,

    /// Print the version
    #[arg(long)]
    pub version: bool,

    pub path: Option<PathBuf>,
}

impl Args {
    pub fn print_help(&self) {
        let printer = clap_help::Printer::new(Args::command())
            .with("introduction", INTRO)
            .with("options", clap_help::TEMPLATE_OPTIONS_MERGED_VALUE)
            .without("author");
        printer.print_help();
    }
}
