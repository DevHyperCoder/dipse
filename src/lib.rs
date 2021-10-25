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

mod args;
mod config;
mod error;
mod parser;

use crate::{
    args::{Opt, SubOpt},
    config::get_config_path,
    parser::{parse_toml, Entry},
};
use error::Error;
use std::{
    env::current_dir,
    fs,
    path::PathBuf,
    process::{Command, Output, Stdio},
};
use structopt::StructOpt;
use toml::Value;

/// Executor
pub fn run() -> Result<(), Error> {
    let opt = Opt::from_args();

    let config_path = match opt.config_path {
        Some(c) => c,
        None => get_config_path()?,
    };

    let config_str = match fs::read_to_string(&config_path) {
        Err(a) => return Err(Error::NoFile(config_path, a)),
        Ok(s) => s,
    };

    let entry = get_entry(&config_str)?;

    if let Some(sub_cmd) = opt.sub_cmd {
        println!("{:?}", sub_cmd);
        match sub_cmd {
            SubOpt::List => list_entries(entry)?,
        }
        return Ok(());
    }

    run_cmd(opt.cmd, entry)
}

/// Run the specified commands defined in entry
fn run_cmd(cmd_list: Vec<String>, entry: Entry) -> Result<(), Error> {
    for cmd in cmd_list {
        let cmd_str = get_cmd_str(&entry, &cmd)?;

        println!("`{}`", cmd_str);
        exec_command(cmd_str)?;
    }
    Ok(())
}

/// List everything in the entry
/// Maybe can be moved to a Display trait
fn list_entries(entry: Entry) -> Result<(), Error> {
    match &entry.entry_table {
        Value::Table(table) => {
            for key in table.keys() {
                // Unwrap here is safe
                println!("\"{}\": {}", key, table.get(key).unwrap())
            }
            Ok(())
        }
        _ => Err(Error::NoTable(Some(entry.path.clone()))),
    }
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
