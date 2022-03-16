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

#![warn(missing_docs)]
//! DIPSE (DIrectory Independent Project Script Executor)
//!
//! dipse is a tool to have the same "aliases" which map to different commands in different
//! projects. A `run` alias could point to `cargo run` on a rust project, while on a NodeJS
//! project, it might point to `npm run start`.
//!
//! ## Configuration
//!
//! TODO

/// StructOpt and argument parsing
pub mod args;
/// Find, create and read config files
pub mod config;
/// Errors
pub mod error;
/// Parse config files
pub mod parser;
/// Utility methods
pub mod utils;

use crate::{
    args::{Crud, Opt, SubOpt},
    config::get_config_path,
    parser::{parse_toml, Entry},
    utils::CommandParams,
};
use error::Error;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    process::Output,
    vec,
};
use structopt::StructOpt;
use utils::{exec_command, get_current_dir};

/// Get a tuple with config path and the config str
fn get_config_path_and_str(config_path: Option<PathBuf>) -> Result<(PathBuf, String), Error> {
    let config_path = match config_path {
        Some(c) => Some(c),
        None => get_config_path()?,
    };

    match config_path {
        Some(e) => {
            let config_str = match fs::read_to_string(&e) {
                Err(a) => return Err(Error::NoFile(e, a)),
                Ok(s) => s,
            };

            println!("C: {}", config_str);

            Ok((e, config_str))
        }
        None => Err(Error::NoConfigFile),
    }
}

/// Executor
pub fn run() -> Result<(), Error> {
    let opt = Opt::from_args();

    let config_path = opt.config_path;
    let debug = opt.debug;

    if let Some(sub_cmd) = opt.sub_cmd {
        match sub_cmd {
            SubOpt::Init => {
                match config_path {
                    Some(c) => return Err(Error::ConfigExist(c)),
                    None => {
                        let curr_dir = get_current_dir()?;

                        let new_p = curr_dir.join(".d.toml");
                        let mut f = match File::create(&new_p) {
                            Err(e) => return Err(Error::ConfigFileCreation(new_p, e)),
                            Ok(e) => e,
                        };

                        if let Err(e) = writeln!(f, "[\"{}\"]", new_p.display()) {
                            return Err(Error::ConfigFileWrite(new_p, e));
                        }

                        if let Err(e) = f.flush() {
                            return Err(Error::ConfigFileWrite(new_p, e));
                        }
                    }
                };
            }
            SubOpt::Edit => {
                let (config_path, _config_str) = get_config_path_and_str(config_path)?;
                start_editor(config_path)?;
            }
            SubOpt::Crud(crud) => {
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

                let (_path, _entry) = this_dir[0];

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
            SubOpt::Other(cmd) => {
                let (_config_path, config_str) = get_config_path_and_str(config_path)?;

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

                run_cmd(cmd, entry, debug, opt.no_op)?
            }
        }
    }

    Ok(())
}

/// Run the specified commands defined in entry
///
/// If a command requires arguments, then last command will get the arguments
///
/// dipse build run -- args
fn run_cmd(cmd_list: Vec<String>, entry: &Entry, debug: bool, no_op: bool) -> Result<(), Error> {
    let index_of_splitter = cmd_list.iter().position(|s| s == "--");

    let cmd_params = match index_of_splitter {
        Some(idx) => {
            // THese are cmds without args
            let cmd = &cmd_list[..idx - 1];

            let mut cmd_params = cmd
                .iter()
                .map(|c| CommandParams {
                    cmd_str: c.to_string(),
                    params: vec![],
                })
                .collect::<Vec<CommandParams>>();

            let arg_cmd = &cmd_list[idx - 1];
            let args = &cmd_list[idx + 1..];

            cmd_params.push(CommandParams {
                cmd_str: arg_cmd.to_string(),
                params: args.into(),
            });

            cmd_params
        }
        None => cmd_list
            .iter()
            .map(|c| CommandParams {
                cmd_str: c.to_string(),
                params: vec![],
            })
            .collect::<Vec<CommandParams>>(),
    };

    for mut cmd in cmd_params {
        let cmd_str = get_cmd_str(entry, &cmd.cmd_str)?;
        cmd.cmd_str = cmd_str.clone();
        if debug {
            println!("`{}`", cmd);
        }
        if no_op {
            continue;
        }
        exec_command(cmd)?;
    }
    Ok(())
}

fn start_editor(config_path: PathBuf) -> Result<Output, Error> {
    exec_command(CommandParams {
        cmd_str: format!("$EDITOR {}", config_path.display()),
        params: vec![],
    })
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
fn get_cmd_str(entry: &Entry, cmd: &str) -> Result<String, Error> {
    match entry.get(cmd) {
        Some(s) => Ok(s.to_owned()),
        None => Err(Error::NoCmdStringFound(get_current_dir()?, cmd.to_string())),
    }
}
