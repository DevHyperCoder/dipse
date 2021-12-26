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

use std::{fmt, io, path::PathBuf};

/// Error enum with all possible error cases
pub enum Error {
    /// No file found at specified path
    NoFile(PathBuf, io::Error),
    /// TOML Parsing error
    UnableToParse(toml::de::Error),
    /// TOML Serialization error
    UnableToSerialize(toml::ser::Error),
    /// Unable to find the command string in Entry map
    NoCmdStringFound(PathBuf, String),
    /// New command to be inserted already exists
    CmdStringExists(PathBuf, String),
    /// No configuration file found, even after traversing upwards
    NoConfigForPath(PathBuf),
    /// Error running a shell command
    Command(io::Error),
    /// Unable to get CWD
    CurrentDir,
    /// Unable to get the configuration directory
    ConfigDir,
    /// Unable to access configuration path
    ConfigPath(io::Error),
    /// Error while file creation
    ConfigFileCreation(PathBuf, io::Error),
    /// Error while writing to file
    ConfigFileWrite(PathBuf, io::Error),
    /// Error while configuration directory creation
    ConfigDirCreation(PathBuf, io::Error),
    /// Created a new configuration, time for the user to update it.
    NewConfig(PathBuf),
}

/// User readable error messages
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err = match self {
            Error::NoFile(file_loc, e) => {
                format!("Could not read file: {}\n{}", file_loc.display(), e)
            }
            Error::ConfigFileWrite(path, e) => {
                format!("Could not write to config file: {}\n{}", path.display(), e)
            }
            Error::UnableToParse(e) => {
                format!("{}", e)
            }
            Error::UnableToSerialize(e) => {
                format!("{}", e)
            }
            Error::CurrentDir => "Unable to access the current working directory.".to_string(),
            Error::NoConfigForPath(path) => {
                format!("No entries found for path: {}", path.display())
            }
            Error::NoCmdStringFound(path, cmd) => {
                format!("No command {} found for path: {}", cmd, path.display())
            }
            Error::CmdStringExists(path, cmd) => {
                format!(
                    "Command {} already exists for path: {}",
                    cmd,
                    path.display()
                )
            }
            Error::Command(e) => {
                format!("{}", e)
            }
            Error::ConfigDirCreation(path, e) => {
                format!(
                    "Could not create configuration directory: {}\n{}",
                    path.display(),
                    e
                )
            }
            Error::ConfigPath(e) => {
                format!("{}", e)
            }
            Error::ConfigFileCreation(path, e) => {
                format!(
                    "Could not create configuration file: {}\n{}",
                    path.display(),
                    e
                )
            }
            Error::NewConfig(path) => {
                format!("Empty configuration file. Please edit {}", path.display())
            }
            Error::ConfigDir => "Could not access config directory".into(),
        };
        write!(f, "{}", err)
    }
}
