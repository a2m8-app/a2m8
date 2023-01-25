use std::path::PathBuf;

use clap::Parser;
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, env)]
    pub data_dir: Option<PathBuf>,
    #[clap(subcommand)]
    pub subcommand: Option<Command>,
}
#[derive(Parser, Debug)]
pub enum Command {
    /// Run a script from path
    Run { file: PathBuf },
    /// Open the tauri UI
    Open {},
    /// List all the scripts
    List {},
    /// Add a file to the list of scripts
    Add { file: PathBuf },
    /// Remove a script this requires the id you can view it in list sub
    Delete { id: Uuid },
    /// Start
    Start { id: Uuid },
    /// Inspect
    Inspect { id: Uuid },
}
