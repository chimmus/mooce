# mooce 
  [![rust]][rustw]
  [![github]][repo]
  [![wbadge]][wiki]
  [![crate_download]][crate]


mooce is designed to be a list. It is very unintuitive to use. 💤

## Building
>Tested with `cargo 1.97.1` on Apple Silicon™.

Clone the git repo.
```bash
git clone https://github.com/chimmus/mooce.git ~/mooce
```
Next build the program with `cargo`:
```bash
cd ~/mooce
cargo build --release
cargo install --path .
```
You can also get it from [crates.io](https://crates.io/crates/mooce).
## Utilities
You can run `mooce -h`:
```
An unintuitive list

Usage: mooce [OPTIONS]

Options:
  -a, --add <topic>              Add topic
  -l, --list                     List
  -L, --list-file                List all lists
  -d, --delete <index>           Delete entry at the index
  -D, --delete-list <file name>  Delete entire list (no confirmation)
  -r, --read <index>             Read entry at index
  -t, --target <file name>       Which list file to target?
  -c, --create <file name>       Create a list
  -v, --verbose                  Verbose
  -h, --help                     Print help
  -V, --version                  Print version
```
The flags work as described above
`-t` is important when you want to use a different list than the default (ability to choose defaults is on its way).
### Here is an example:
```bash
$ mooce -t shopping-list -a chips
```
In the example above, we add `chips` to our `shopping-list`.
We can then print our `shopping-list` with `mooce -t shopping-list -l`.

More features will be added in the future.

## Use cases
There are many ways to use `mooce`.
Here are some ways:
- using it as a todo list, you run `mooce` every time you want to do something productive
- a "wheel" of names

## Config
You can create a config file named `~/.config/mooce/mooce.conf`.
There is currently 1 option:
```
default=<list name>
```
Currently, there are issues with lists with spaces in their names. The program will fallback to `~/.config/mooce/list` if there is no default list selected.
You can also set an environment variable to change the location `mooce` checks for lists. 
```bash
$ echo 'export MPATH="<dir path>"' >> ~/.bashrc 
```
Currently, the environment variable supports `~`.

<!---------------------------------[ Internal ]-------------------------------->
[repo]: https://github.com/chimmus/mooce
[wiki]: https://github.com/chimmus/mooce/wiki
[rustw]: https://rust-lang.org/
[crate]: https://crates.io/crates/mooce

<!----------------------------------[ Badges ]--------------------------------->
[rust]: https://img.shields.io/badge/made%20with-Rust-orange?logo=rust&style=flat-square
[github]: https://img.shields.io/badge/GitHub-Repo-green?logo=github&style=flat-square
[wbadge]: https://img.shields.io/badge/Wiki-+-blue?logo=mdbook&style=flat-square
[crate_download]: https://img.shields.io/crates/dv/mooce/0.1.8?style=for-the-badge&logo=rust
