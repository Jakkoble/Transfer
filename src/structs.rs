use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, about, version)]
pub struct Cli {
    /// Copy the file instead of moving it
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub copy: bool,
    /// Force action even if the destination file already exists
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub force: bool,
    /// Use Downloads directory as destination
    #[arg(short = 'd', long = "download", action = clap::ArgAction::SetTrue)]
    pub use_downloads: bool,
    /// Origin path
    #[arg(value_name = "ORIGIN_PATH")]
    pub origin: PathBuf,
    /// Optional: New file/dir name, if not specified the original name will be used
    #[arg(value_name = "NEW_NAME")]
    pub name: Option<String>,
}
