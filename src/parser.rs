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

use crate::error::Error;
use std::{collections::HashMap, path::PathBuf};

pub type Entries = HashMap<PathBuf, Entry>;
pub type Entry = HashMap<String, String>;

/// Parses toml file into a list of entries
/// Error:
/// - Could not parse TOML
pub fn parse_toml(c: &str) -> Result<Entries, Error> {
    match toml::from_str::<Entries>(c) {
        Ok(e) => Ok(e),
        Err(e) => Err(Error::UnableToParse(e)),
    }
}
