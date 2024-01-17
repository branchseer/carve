use std::{
    fs,
    io::{stdin, stdout, Read, Write},
    path::{Path, PathBuf},
};

use anyhow::Ok;
use clap::Parser;
use editres_cli::{inject, list};
use std::borrow::Cow;

#[derive(Parser)]
#[command()]
enum Command {
    /// List declared resources and whether they have been injected
    List { executable_path: PathBuf },
    /// Inject resource data.
    Inject {
        /// Path of the executable
        executable_path: PathBuf,
        /// Name of the resource 
        resource_name: String,
        /// Resource data file
        #[arg(long = "data", short = 'd')]
        resource_data_path: Option<PathBuf>,
        /// Output path of the injected executable
        #[arg(long = "output", short = 'o')]
        output_executable_path: Option<PathBuf>,
    },
}

#[cfg(unix)]
fn as_bytes(path: &Path) -> Cow<[u8]> {
    use std::os::unix::ffi::OsStrExt;
    path.as_os_str().as_bytes().into()
}

#[cfg(windows)]
fn as_bytes(path: &Path) -> Cow<[u8]> {
    match path.to_string_lossy() {
        Cow::Borrowed(s) => s.as_bytes().into(),
        Cow::Owned(s) => s.into_bytes().into(),
    }
}

fn main() -> anyhow::Result<()> {
    let command = Command::parse();
    match command {
        Command::List { executable_path } => {
            let res_vec = list(executable_path)?;
            println!("Name\tInjected");
            for (name, injected) in res_vec {
                println!("{}\t{}", name, if injected { "yes" } else { "no" });
            }
        }
        Command::Inject {
            executable_path,
            resource_name,
            resource_data_path,
            output_executable_path,
        } => {
            let mut output_executable_path =
                output_executable_path.unwrap_or_else(|| executable_path.clone());
            let resource_data = if let Some(resource_data_path) = resource_data_path {
                fs::read(resource_data_path)?
            } else {
                let mut buf = Vec::<u8>::with_capacity(8192);
                stdin().lock().read_to_end(&mut buf)?;
                buf
            };
            inject(
                executable_path,
                &resource_name,
                &resource_data,
                &mut output_executable_path,
            )?;
            let mut stdout = stdout().lock();
            stdout.write_all(&as_bytes(&output_executable_path))?;
            writeln!(&mut stdout)?;
        }
    }
    Ok(())
}
