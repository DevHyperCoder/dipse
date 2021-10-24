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
use std::path::PathBuf;
use toml::Value;

/// Stores the path and the list of values for given path
pub struct Entry {
    pub path: PathBuf,
    pub entry_table: Value,
}

/// Parses toml file into a list of entries
/// Error:
/// - Could not parse TOML
/// - No Main Table was found
pub fn parse_toml(c: &str) -> Result<Vec<Entry>, Error> {
    let parsed_config = match c.parse::<Value>() {
        Err(e) => return Err(Error::UnableToParse(e)),
        Ok(c) => c,
    };
    match parsed_config {
        Value::Table(main_table) => {
            let mut all_keys = main_table
                .keys()
                .into_iter()
                .map(|key| Entry {
                    path: key.into(),
                    entry_table: main_table.get(key).unwrap().to_owned(),
                })
                .collect::<Vec<Entry>>();

            all_keys.sort_by(|a, b| b.path.cmp(&a.path));
            Ok(all_keys)
        }
        _ => Err(Error::NoTable(None)),
    }
}
