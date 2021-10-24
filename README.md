# DIPSE (Directory Independent Project Script Executor)

`dipse <cmd1> <cmd2> <cmd3> ...`

Use your same "aliases" for all your projects

## Installation

- Clone the repository: `git clone https://github.com/DevHyperCoder/dipse.git`
- Change Directory into dipse: `cd dipse/`

- Run the code: `cargo run -- <your options>`
- Build the code: `cargo build --release`
- Install it: `cargo install --path .`

> Eventually, dipse will be available on the AUR and crates.io
## Features

- Simple to use TOML config file
- Works if you are inside a child directory
- Global config file

## Configuration

Each project can have its own `.d.toml` file.

```toml
["/rust/project"]
f = "cargo fmt"
r = "cargo run"
b = "cargo build"


["/node/project"]
f = "npm run format"
r = "npm run dev"
b = "npm run build"
```

See how in the above example, the aliases work for both `/rust/project` and `/node/project`

> NOTE: Each path and command needs to be inside ""

`dipse` will traverse up the directory structure to find a `.d.toml` file. If none is found, it will create a config file in `$XDG_CONFIG_HOME/dipse/`

## Contributions

Pull Requests and Issues are accepted.

## LICENSE

`dipse` is licensed under the GNU General Public License 3. Our copy of
GPL-3 can be found [here](./LICENSE)
