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

// Returns .d.toml if it exists in current directory or any parent dir
// Or the XDG spec config/dipse/d.toml
fn traverse_upwards_for_config(path: &mut PathBuf) -> Option<PathBuf> {
    let current_dir_config = path.join(".d.toml");

    if current_dir_config.exists() {
        return Some(current_dir_config);
    }

    let i = path.pop();

    if !i {
        return None;
    }

    traverse_upwards_for_config(path)
}

/// Get a config file path.
/// First traverse upwards for config
pub fn get_config_path() -> Result<Option<PathBuf>, Error> {
    let mut curr_path = match PathBuf::from(".").canonicalize() {
        Err(e) => return Err(Error::ConfigPath(e)),
        Ok(c) => c,
    };
    Ok(traverse_upwards_for_config(&mut curr_path))
}
