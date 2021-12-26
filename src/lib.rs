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
mod utils;

use crate::{
    args::{Crud, Opt, SubOpt},
    config::get_config_path,
    parser::{parse_toml, Entry},
};
use error::Error;
use std::{fs, path::PathBuf, process::Output};
use structopt::StructOpt;
use utils::{exec_command, get_current_dir};

/// Get a tuple with config path and the config str
fn get_config_path_and_str(config_path: Option<PathBuf>) -> Result<(PathBuf, String), Error> {
    let config_path = match config_path {
        Some(c) => c,
        None => get_config_path()?,
    };

    let config_str = match fs::read_to_string(&config_path) {
        Err(a) => return Err(Error::NoFile(config_path, a)),
        Ok(s) => s,
    };

    Ok((config_path, config_str))
}

/// Executor
pub fn run() -> Result<(), Error> {
    let opt = Opt::from_args();

    let config_path = opt.config_path;
    let debug = opt.debug;

    let (config_path, config_str) = get_config_path_and_str(config_path)?;

    let pwd = get_current_dir()?;

    let entries = parse_toml(&config_str)?;
    let mut this_dir = vec![];
    for entry in &entries {
        // Check if the entry path is IN the pwd
        // This allows dipse to work when inside a nested system
        if pwd.starts_with(entry.0) {
            this_dir.push(entry)
        }
    }

    if this_dir.is_empty() {
        return Err(Error::NoConfigForPath(pwd));
    }

    this_dir.sort_by(|a, b| a.0.cmp(b.0));

    let (_path, entry) = this_dir[0];

    if let Some(sub_cmd) = opt.sub_cmd {
        match sub_cmd {
            SubOpt::Edit => {
                start_editor(config_path)?;
            }
            SubOpt::Crud(crud) => {
                // All entries in the file
                let mut new_entries = entries.clone();

                // Entry for the current path
                let entry = new_entries.get_mut(_path).unwrap();

                match crud {
                    Crud::List { name } => list_entries(entry, name)?,
                    Crud::Add { name, cmd } => {
                        if let Some(c) = entry.get(&name) {
                            return Err(Error::CmdStringExists(get_current_dir()?, c.to_string()));
                        }
                        entry.insert(name, cmd);
                    }
                    Crud::Delete { name } => {
                        if entry.get(&name).is_none() {
                            return Err(Error::NoCmdStringFound(get_current_dir()?, name));
                        }
                        entry.remove(&name);
                    }
                    Crud::Update { name, cmd } => {
                        if entry.get(&name).is_none() {
                            return Err(Error::NoCmdStringFound(get_current_dir()?, name));
                        }
                        entry.insert(name, cmd);
                    }
                }

                let new_config_str = match toml::to_string_pretty(&new_entries) {
                    Err(e) => return Err(Error::UnableToSerialize(e)),
                    Ok(s) => s,
                };
                if let Err(e) = fs::write(&config_path, new_config_str) {
                    return Err(Error::ConfigFileWrite(config_path, e));
                }
            }
            SubOpt::Other(cmd) => run_cmd(cmd, entry, debug)?,
        }
    }

    Ok(())
}

/// Run the specified commands defined in entry
fn run_cmd(cmd_list: Vec<String>, entry: &Entry, debug: bool) -> Result<(), Error> {
    for cmd in cmd_list {
        let cmd_str = get_cmd_str(entry, cmd)?;

        if debug {
            println!("`{}`", cmd_str);
        }
        exec_command(cmd_str)?;
    }
    Ok(())
}

fn start_editor(config_path: PathBuf) -> Result<Output, Error> {
    exec_command(format!("$EDITOR {}", config_path.display()))
}

/// List everything in the entry
/// Maybe can be moved to a Display trait
fn list_entries(entry: &Entry, name: Option<String>) -> Result<(), Error> {
    if let Some(name) = name {
        match entry.get(&name) {
            None => return Err(Error::NoCmdStringFound(get_current_dir()?, name)),
            Some(cmd) => {
                println!("{}: {}", name, cmd);
            }
        }
    } else {
        println!("{:#?}", entry);
    }
    Ok(())
}

/// Get command string for alias from a entry
fn get_cmd_str(entry: &Entry, cmd: String) -> Result<String, Error> {
    match entry.get(&cmd) {
        Some(s) => Ok(s.to_owned()),
        None => Err(Error::NoCmdStringFound(get_current_dir()?, cmd.to_string())),
    }
}
