# Wachit
View live website here: [wachit.dev](https://wachit.dev)
Wachit is a tool that helps develop applications by automatically restarting the server when file changes in the directory are detected.

Wachit does **not** require *any* additional changes to your code or development process. Wachit is a replacement wrapper for `node`. To use `wachit`, replace the word `node` on the command line when executing your script.

# Installation

Make sure rust is installed on your system. If not you can refer to the rust Installation instructions [here](https://rust-lang.org/tools/install/): 
```bash
cargo install wachit
```

And wachit will be installed globally to your system path.
# Usage
As of the current version, we only support JavaScript runtime.

## Using CLI
Navigate to your project directory and run the command in the following format:
> `wachit [wachit options] [target file name]`.

### wachit options
You can pass multiple options to configure wachit runtime. As of the current version, we only support `inspect` option and we will introduce more options in the upcoming releases.

`inspect`: This command only runs with a provided file name. This will be serialied to `node --inspect [provided-file-name]`.

### Target File Name
You have to provide a file name, wachit will use your project's executable, which defaults to `node`, to run that file.

## Using Config file

If there is a wachit.json file present in the current working directory, wachit will prioritize it over any cli arguments. The following code defines the configuration keys you can define in the wachit.json file. Remember, using a config will render all cli arguments as null and void.

```ts
type Config = {
  executable: "NODE"; // As of current version, it only supports node executables
  target: string; // target filename with path
  inspect: Boolean; // Whether we want to inspect with node
}
```


# Features In-Development
- Manual restarting with `rst` command
- Running non-node scripts and extension for runtimes and environments beyond JavaScript.
- Monitoring multiple directories
- Specifying extension watch list
- Ignoring files extended support

# Contributing

Wachit is not perfect and that is why it needs talented developers like you to contribute.

If you have a feature suggestion or come across a bug please don't hesitate to open a ticket. I have too much free time on my hands to not go through your ideas and suggestions. If you get my approval on any feature or bugfix, you can contribute with the following steps:
1. Clone the git repository.
2. Create a branch, code something cool.
3. Create a pull-request.

That's all.

# License

MIT [http://rem.mit-license.org](http://rem.mit-license.org)
