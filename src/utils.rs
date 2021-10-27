/*
 * DIPSE (Directory Independent Project Script Executor)
 * Copyright (C) 2021 DevHyperCoder
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::{env::current_dir, path::PathBuf};

use crate::error::Error;
use std::process::{Command, Output, Stdio};

/// Get current directory ($PWD)
pub fn get_current_dir() -> Result<PathBuf, Error> {
    match current_dir() {
        Ok(d) => Ok(d),
        Err(_) => Err(Error::CurrentDir),
    }
}

/// Execute command with io inherited
pub fn exec_command(cmd_str: String) -> Result<Output, Error> {
    match Command::new("sh")
        .arg("-c")
        .arg(cmd_str)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
    {
        Ok(e) => Ok(e),
        Err(e) => Err(Error::Command(e)),
    }
}
