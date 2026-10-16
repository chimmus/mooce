# mooce 
  [![rust]][rustw]
  [![github]][repo]
  [![wbadge]][wiki]


mooce is designed to be a list. It is very unintuitive to use. 💤

## Building
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

<!---------------------------------[ Internal ]-------------------------------->
[repo]: https://github.com/chimmus/mooce
[wiki]: https://github.com/chimmus/mooce/wiki
[rustw]: https://rust-lang.org/

<!----------------------------------[ Badges ]--------------------------------->
[rust]: https://img.shields.io/badge/made%20with-Rust-orange?logo=rust&style=flat-square
[github]: https://img.shields.io/badge/GitHub-Repo-green?logo=github&style=flat-square
[wbadge]: https://img.shields.io/badge/Wiki-+-blue?logo=mdbook&style=flat-square
