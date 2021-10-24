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

mod config;
mod error;
mod parser;

use crate::{
    config::get_config_path,
    parser::{parse_toml, Entry},
};
use error::Error;
use std::{
    env::{args, current_dir},
    fs,
    path::PathBuf,
    process::{Command, Output, Stdio},
};
use toml::Value;

/// Executor
pub fn run() -> Result<(), Error> {
    let config_path = get_config_path()?;
    let mut cmd_list = args().collect::<Vec<String>>();

    // Removing the program name from the list of cmd to execute.
    cmd_list.remove(0);

    let config_str = match fs::read_to_string(&config_path) {
        Err(a) => return Err(Error::NoFile(config_path, a)),
        Ok(s) => s,
    };

    for cmd in cmd_list {
        let entry = get_entry(&config_str)?;

        let cmd_str = get_cmd_str(&entry, &cmd)?;

        println!("`{}`", cmd_str);
        exec_command(cmd_str)?;
    }
    Ok(())
}

/// Get entry for current dir from config file
fn get_entry(config_str: &str) -> Result<Entry, Error> {
    let current_dir = get_current_dir()?;

    let all_dirs = parse_toml(config_str)?;
    for dir in all_dirs {
        if current_dir.clone().starts_with(&dir.path) {
            return Ok(dir);
        }
    }
    Err(Error::InsuffcientEntries(current_dir))
}

/// Get current directory ($PWD)
fn get_current_dir() -> Result<PathBuf, Error> {
    match current_dir() {
        Ok(d) => Ok(d),
        Err(_) => Err(Error::CurrentDir),
    }
}

/// Get command string for alias from a entry
fn get_cmd_str(entry: &Entry, cmd: &str) -> Result<String, Error> {
    match &entry.entry_table {
        Value::Table(table) => match &table.get(cmd) {
            Some(Value::String(s)) => Ok(s.to_owned()),

            Some(_) | None => Err(Error::NoCmdStringFound(entry.path.clone(), cmd.to_string())),
        },
        _ => Err(Error::NoTable(Some(entry.path.clone()))),
    }
}

/// Execute command with io inherited
fn exec_command(cmd_str: String) -> Result<Output, Error> {
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
