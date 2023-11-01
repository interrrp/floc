use clap::Parser;

/// Count the lines in your codebase.
#[derive(clap::Parser)]
pub struct Args {
    /// The file extension to check. Example: "rs" or "py".
    #[clap(short, long, default_value = "rs")]
    pub extension: String,

    /// The directory to check. Example: "src" or "tests".
    #[clap(short, long, default_value = "src")]
    pub directory: String,
}

pub fn parse() -> Args {
    Args::parse()
}
