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

use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(setting = structopt::clap::AppSettings::TrailingVarArg)]
pub struct Opt {
    #[structopt(subcommand)]
    pub sub_cmd: Option<SubOpt>,

    /// Optional configuration path
    #[structopt(short, long)]
    pub config_path: Option<PathBuf>,

    /// List of commands to execute
    pub cmd: Vec<String>,
}

#[derive(Debug, StructOpt)]
pub enum SubOpt {
    /// List all entries for current dir
    List,
}
