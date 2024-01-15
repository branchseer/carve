use std::path::PathBuf;

use clap::Parser;


#[derive(Parser)]
#[command(about)]
enum Command {
    /// List declared resources and whether they have been injected in an executable
    List { executable_path: PathBuf },
    /// Inject resource data into an executable.
    Inject {
        /// Path of the executable
        executable_path: PathBuf,
        resource_name: String,
        resource_data_path: PathBuf,
        output_executable_path: Option<PathBuf>,
    }
}

fn main() {
    let command = Command::parse();
}

