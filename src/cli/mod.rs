mod args;

pub use args::*;

use {
    crate::*,
    clap::Parser,
    std::path::Path,
};

pub fn run() -> CradResult<()> {
    let args: Args = Args::parse();
    info!("args: {:#?}", &args);

    if args.help {
        args.print_help();
        return Ok(());
    }

    if args.version {
        println!("cradoc {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let crate_path = args.path.as_deref().unwrap_or(Path::new("."));

    let context = Context::load(crate_path)?;

    context.update_all_md_files()?;

    Ok(())
}
