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
    NoFile(PathBuf, io::Error),
    UnableToParse(toml::de::Error),
    NoCmdStringFound(PathBuf, String),
    NoConfigForPath(PathBuf),
    Command(io::Error),
    CurrentDir,
    ConfigDir,
    ConfigPath(io::Error),
    ConfigFileCreation(PathBuf, io::Error),
    ConfigDirCreation(PathBuf, io::Error),
    NewConfig(PathBuf),
}

/// User readable error messages
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err = match self {
            Error::NoFile(file_loc, e) => {
                format!("Could not read file: {}\n{}", file_loc.display(), e)
            }
            Error::UnableToParse(e) => {
                format!("{}", e)
            }
            Error::CurrentDir => "Unable to access the current working directory.".to_string(),
            Error::NoConfigForPath(path) => {
                format!("No entries found for path: {}", path.display())
            }
            Error::NoCmdStringFound(path, cmd) => {
                format!("No command {} found for path: {}", cmd, path.display())
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
