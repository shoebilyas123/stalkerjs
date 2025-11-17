# Wachit

Wachit is a tool that helps develop applications by automatically restarting the server when file changes in the directory are detected.

Wachit does **not** require *any* additional changes to your code or development process. Wachit is a replacement wrapper for `node`. To use `wachit`, replace the word `node` on the command line when executing your script.


# Installation

Either through cloning with git or by using `cargo install` command

```bash
cargo install wachit
```

And wachit will be installed globally to your system path.
# Usage

## Automatic re-running

wachit was originally written to restart hanging processes such as web servers, but now supports apps that cleanly exit. If your script exits cleanly, wachit will continue to monitor the directory (or directories) and restart the script if there are any changes.


# Features In-Development
- Support for path in cli arguments
- Default script search
- Manual restarting with `rst` command
- Using wachit as a child process
- Using wachit as a module
- Running non-node scripts
- Manually configuring Wachit with `Wachit.json` config file
- Monitoring multiple directories
- Unix globbing
- Specifying extension watch list
- Ignoring files extended support

Wachit is not perfect, and CLI arguments has sprawled beyond where I'm completely happy, but perhaps it can be reduced a little one day.

# License

MIT [http://rem.mit-license.org](http://rem.mit-license.org)
