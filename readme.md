# StalkerJS

stalkerjs is a tool that helps develop Node.js based applications by automatically restarting the node application when file changes in the directory are detected.

stalkerjs does **not** require *any* additional changes to your code or method of development. stalkerjs is a replacement wrapper for `node`. To use `stalkerjs`, replace the word `node` on the command line when executing your script.


# Installation

Either through cloning with git or by using `cargo install` command

```bash
cargo install stalkerjs
```

And stalkerjs will be installed globally to your system path.

With a local installation, stalkerjs will not be available in your system path or you can't use it directly from the command line. Instead, the local installation of stalkerjs can be run by calling it from within an npm script (such as `npm start`) or using `npx stalkerjs`.

# Usage

## Automatic re-running

stalkerjs was originally written to restart hanging processes such as web servers, but now supports apps that cleanly exit. If your script exits cleanly, stalkerjs will continue to monitor the directory (or directories) and restart the script if there are any changes.


# Features In-Development
- Support for path in cli arguments
- Default script search
- Manual restarting with `rst` command
- Using stalkerJS as a child process
- Using stalkerJS as a module
- Running non-node scripts
- Manually configuring StalkerJS with `StalkerJS.json` config file
- Monitoring multiple directories
- Unix globbing
- Specifying extension watch list
- Ignoring files extended support

StalkerJS is not perfect, and CLI arguments has sprawled beyond where I'm completely happy, but perhaps it can be reduced a little one day.

# License

MIT [http://rem.mit-license.org](http://rem.mit-license.org)
