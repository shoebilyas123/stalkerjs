# Wachit
View live website here: [wachit.dev](https://wachit.dev)

Wachit is a lightweight file watcher tool that helps develop applications by automatically restarting the server when file changes in the directory are detected.

Wachit does **not** require *any* additional changes to your code or development process. Wachit is a replacement wrapper for your runtime executable. To use `wachit`, replace your runtime command on the command line when executing your script.

# Installation
Make sure rust is installed on your system. If not you can refer to the rust Installation instructions [here](https://rust-lang.org/tools/install/): 
```bash
cargo install wachit
```
And wachit will be installed globally to your system path.

# Usage
Wachit supports multiple language runtimes including Node.js, Python, Go, and Rust (Cargo).

## Using CLI
Navigate to your project directory and run the command in the following format:
> `wachit [wachit options] [target file name]`

### wachit options
You can pass multiple options to configure wachit runtime:

- `--inspect`: Enables inspect mode (currently supports Node.js). This will be serialized to `node --inspect [provided-file-name]`.
- `--exec=<RUNTIME>`: Specify the runtime executable. Options: `NODE`, `PYTHON`, `GOLANG`, `CARGO`. Defaults to `NODE`.
- `--watch=<extensions>`: Comma-separated list of file extensions to watch (e.g., `--watch=.js,.ts,.json`).
- `--ignore=<extensions>`: Comma-separated list of file extensions to ignore (e.g., `--ignore=.test.js,.spec.ts`).

### Examples
```bash
# Run a Node.js application
wachit index.js

# Run a Python application
wachit --exec=PYTHON main.py

# Run with Node.js inspect mode
wachit --inspect server.js

# Watch specific file types
wachit --watch=.js,.json --exec=NODE app.js

# Ignore specific file types
wachit --ignore=.test.js,.spec.js index.js

# Run a Go application
wachit --exec=GOLANG main.go

# Run a Rust application
wachit --exec=CARGO
```

### Target File Name
You must provide a target file name. Wachit will use the specified executable (or default to `node`) to run that file.

## Using Config file
If a `wachit.json` file is present in the current working directory, wachit will prioritize it over any CLI arguments. The configuration file will override all CLI arguments.

```json
{
  "executable": "NODE",
  "target": "index.js",
  "inspect": false,
  "watch_list": [".js", ".ts"],
  "ignore_list": [".test.js"],
  "delay": 2000
}
```

### Configuration Options
```typescript
type Config = {
  executable: "NODE" | "PYTHON" | "GOLANG" | "CARGO"; // Runtime executable
  target: string; // Target filename with path
  inspect: boolean; // Whether to enable inspect mode (Node.js only)
  watch_list: string[]; // List of file extensions to watch
  ignore_list: string[]; // List of file extensions to ignore
  delay: number; // Restart delay in milliseconds (default: 2000)
}
```

## Default Behavior
Wachit automatically ignores common directories and files:
- `node_modules/`
- `build/`
- `dist/`
- `target/` (for Rust projects)
- `.next/`
- `.git/`
- `.vscode/`
- `.env`
- `.gitignore`
- `wachit.json`

## Supported File Extensions by Runtime
- **NODE**: `.js`, `.jsx`, `.ts`, `.tsx`
- **PYTHON**: `.py`
- **GOLANG**: `.go`
- **CARGO**: `.rs`

# Contributing
Wachit is not perfect and that is why it needs talented developers like you to contribute.

If you have a feature suggestion or come across a bug please don't hesitate to open a ticket. I have too much free time on my hands to not go through your ideas and suggestions. If you get my approval on any feature or bugfix, you can contribute with the following steps:

1. Clone the git repository.
2. Create a branch, code something cool.
3. Create a pull-request.

That's all.

# License
MIT [http://rem.mit-license.org](http://rem.mit-license.org)
