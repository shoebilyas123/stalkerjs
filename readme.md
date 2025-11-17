# Iris

Iris is a tool that helps develop applications by automatically restarting the server when file changes in the directory are detected.

Iris does **not** require *any* additional changes to your code or development process. Iris is a replacement wrapper for `node`. To use `iris`, replace the word `node` on the command line when executing your script.


# Installation

Either through cloning with git or by using `cargo install` command

```bash
cargo install iris
```

And iris will be installed globally to your system path.
# Usage

## Automatic re-running

iris was originally written to restart hanging processes such as web servers, but now supports apps that cleanly exit. If your script exits cleanly, iris will continue to monitor the directory (or directories) and restart the script if there are any changes.


# Features In-Development
- Support for path in cli arguments
- Default script search
- Manual restarting with `rst` command
- Using iris as a child process
- Using iris as a module
- Running non-node scripts
- Manually configuring Iris with `Iris.json` config file
- Monitoring multiple directories
- Unix globbing
- Specifying extension watch list
- Ignoring files extended support

Iris is not perfect, and CLI arguments has sprawled beyond where I'm completely happy, but perhaps it can be reduced a little one day.

# License

MIT [http://rem.mit-license.org](http://rem.mit-license.org)
