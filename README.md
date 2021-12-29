# DIPSE (Directory Independent Project Script Executor)

`dipse <cmd1> <cmd2> <cmd3> ...`

Use your same "aliases" for all your projects

## Installation

### crates.io

- `cargo install dipse`

### Manual

- Clone the repository: `git clone https://github.com/DevHyperCoder/dipse.git`
- Change Directory into dipse: `cd dipse/`

- Run the code: `cargo run -- <your options>`
- Build the code: `cargo build --release`
- Install it: `cargo install --path .`

> Eventually, dipse will be available on the AUR
## Features

- Simple to use TOML config file
- Works if you are inside a child directory
- Global config file
- Specify multiple commands at once to execute.
- Add paramaters to aliases (See example below)

## Options

- `-f`: Specify which config file to use
- `-d`: Debug flag. Print out the command to execute
- `-n`: Do not execute the command. Use in combination with `-d`

## Subcommands

- `add`: Add a new alias
- `update`: Update a alias
- `delete`: Delete a alias
- `list`: Lists all the aliases for current dir. Optionally specify a name to see the command of that alias
- `edit`: Edit the config file for current dir. If `-f` is provided, it will edit that instead. Uses your `$EDITOR` variable, please set it before you run this command

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

## Example

### CRUD operation

```sh

# Add
dipse add "alias name" "command to execute"

# List specific
dipse list "alias name"

# List all
dipse list 

dipse update "alias name" "command to execute"

dipse delete "alias name"
```

### Running multiple commands

You can execute multiple aliases at once like this:

`dipse alias1 alias2 alias3 alias4`

Each alias will be executed **after** the previous one is finished.

### Arguments / Parameters

Seperate the arguments from the alias name like this:

`dipse alias_name -- --option-you-want "param1"`

In case you wish to run multiple aliases at the same time, `dipse` will apply the arguments to the **last** alias only.

`dipse alias1 alias2 -- "a"` In this case, `"a"` is passed only to `alias2`, `alias1` is executed without any changes.

## Contributions

Pull Requests and Issues are accepted.

## LICENSE

`dipse` is licensed under the GNU General Public License 3. Our copy of
GPL-3 can be found [here](./LICENSE)
