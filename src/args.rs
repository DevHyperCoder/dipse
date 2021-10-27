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
    #[structopt(global = true, short = "f", long)]
    pub config_path: Option<PathBuf>,
}

#[derive(Debug, StructOpt)]
pub enum SubOpt {
    #[structopt(flatten)]
    /// CURD Operations
    Crud(Crud),
    ///
    /// Opens $EDITOR so you can edit your config file for current directory
    Edit,

    #[structopt(external_subcommand)]
    Other(Vec<String>),
}

#[derive(Debug, StructOpt)]
pub enum Crud {
    /// Add a new alias
    Add {
        name: String,
        cmd: String,
    },

    /// List all entries for current dir
    List {
        name: Option<String>,
    },

    Update {
        name: String,
        cmd: String,
    },

    Delete {
        name: String,
    },
}
