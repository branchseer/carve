/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
#![doc = include_str!("../README.md")]
#![doc(hidden)]

use std::{
    fs::{self, OpenOptions},
    io::{self, Write as _},
    path::{Path, PathBuf},
};

use editres::injector::{inject as base_inject, list as base_list};

pub fn list<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<(String, bool)>> {
    let content = fs::read(path)?;
    base_list(&content)
}

pub fn write_executable(path: &mut PathBuf, content: &[u8]) -> io::Result<()> {
    let mut open_options = OpenOptions::new();
    open_options.create(true).write(true);
    #[cfg(unix)]
    {
        std::os::unix::fs::OpenOptionsExt::mode(&mut open_options, 0o755);
    }
    #[cfg(windows)]
    {
        path.set_extension("exe");
    }
    let mut file = open_options.open(path)?;
    file.write_all(content)?;
    file.sync_all()?;
    Ok(())
}

pub fn inject(
    path: impl AsRef<Path>,
    resource_name: &str,
    resource_data: &[u8],
    out_path: &mut PathBuf,
) -> anyhow::Result<()> {
    let mut executable_content = fs::read(path)?;
    base_inject(&mut executable_content, resource_name, resource_data)?;
    write_executable(out_path, &executable_content)?;
    Ok(())
}
