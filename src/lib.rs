pub mod constants {
    use std::collections::HashMap;

    pub const CONFIG_FILE: &str = "wachit.json";
    pub const ERR_MESSAGE: &str = "wacht [wachit options] [target file]";

    pub const PY_EXTS: &str = ".py";
    pub const NODE_EXTS: &str = ".js .jsx .ts .tsx";
    pub const GOLANG_EXTS: &str = ".go";
    pub const CARGO_EXTS: &str = ".rs";
}

pub mod types {
    use std::{
        fmt,
        fs::{self, File},
        io::{ErrorKind, Read},
        ops::Index,
    };

    use serde::{Deserialize, Serialize};
    use serde_json::{Map, Value};

    use crate::constants::{CARGO_EXTS, ERR_MESSAGE, GOLANG_EXTS, NODE_EXTS, PY_EXTS};
    use crate::processor;

    #[derive(Serialize, Deserialize, Debug)]
    pub enum ConfigValues {
        String,
    }
    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    pub enum Executable {
        NODE,
        GOLANG,
        CARGO,
        PYTHON,
    }

    impl fmt::Display for Executable {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Executable::NODE => write!(f, "node"),
                Executable::GOLANG => write!(f, "go run"),
                Executable::PYTHON => write!(f, "python3"),
                Executable::CARGO => write!(f, "cargo run"),
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Config {
        #[serde(default = "set_default_executable")]
        pub executable: Executable,

        #[serde(default = "set_default_target")]
        pub target: String,

        #[serde(default = "set_default_inspect")]
        pub inspect: bool,

        #[serde(default = "set_default_ignore_list")]
        pub ignore_list: Vec<String>,

        #[serde(default = "set_default_watch_list")]
        pub watch_list: Vec<String>,

        #[serde(default = "set_default_delay")]
        pub delay: u64,
    }
    impl Config {
        pub fn create_default_config() -> Config {
            Config {
                inspect: set_default_inspect(),
                executable: set_default_executable(),
                target: set_default_target(),
                watch_list: set_default_watch_list(),
                ignore_list: set_default_ignore_list(),
                delay: set_default_delay(),
            }
        }

        pub fn should_restart(&self, path: &str) -> bool {
            if self.ignore_list.len() > 0 {
                for wf in &self.ignore_list {
                    let mut fs = wf.clone();

                    if wf.starts_with(".") {
                        fs = wf[1..].to_string();
                    }

                    if path.ends_with(&fs) {
                        return false;
                    }
                }
            }

            if self.watch_list.len() > 0 {
                for wf in &self.watch_list {
                    let mut fs = wf.clone();

                    if wf.starts_with(".") {
                        fs = wf[1..].to_string();
                    }

                    if path.ends_with(&fs) {
                        return true;
                    }
                }
            }

            let extension_options: &str = match self.executable {
                Executable::NODE => NODE_EXTS,
                Executable::CARGO => CARGO_EXTS,
                Executable::GOLANG => GOLANG_EXTS,
                Executable::PYTHON => PY_EXTS,
            };

            for n_ext in extension_options.split(" ") {
                if path.ends_with(n_ext) {
                    return true;
                }
            }

            return false;
        }

        pub fn new(args: Vec<String>) -> Config {
            let file_conf_result = processor::load_file_config();
            let mut is_config_file: bool = false;
            let mut new_config: Config = match file_conf_result {
                Some(_conf) => {
                    is_config_file = true;
                    _conf
                }
                _ => Config::create_default_config(),
            };

            if is_config_file {
                return new_config;
            }

            if args.len() < 2 {
                panic!("Format: wachit [wachit options] [target or command name]")
            }

            let mut target_index = 1;

            loop {
                if target_index == args.len() - 1 {
                    break;
                }

                if target_index >= args.len() {
                    panic!("{ERR_MESSAGE} V");
                }

                if args[target_index].contains("--inspect") {
                    new_config.inspect = true;
                }

                if args[target_index].contains("--exec") {
                    let exec =
                        get_executable(args[target_index].split("=").collect::<Vec<&str>>()[1]);
                    new_config.executable = exec;
                }

                if args[target_index].contains("--ignore") {
                    let ls = args[target_index].split("=").collect::<Vec<&str>>()[1]
                        .split(",")
                        .map(|str| str.to_string())
                        .collect::<Vec<String>>();
                    new_config.ignore_list = ls;
                }

                if args[target_index].contains("--watch") {
                    let ls = args[target_index].split("=").collect::<Vec<&str>>()[1]
                        .split(",")
                        .map(|str| str.to_string())
                        .collect::<Vec<String>>();
                    new_config.watch_list = ls;
                }

                target_index += 1;
            }

            let target = File::open(&args[target_index]);

            let target_file: String = match target {
                Ok(_) => String::from(&args[target_index]),
                Err(_) => {
                    panic!("Invalid arguments: Target file not specified");
                }
            };

            new_config.target = target_file;
            println!("{:?}", new_config);
            new_config
        }
        pub fn get_command(&self) -> String {
            let mut cmd = String::from(self.executable.to_string());

            if self.inspect {
                cmd.push_str(" --inspect");
            };

            cmd.push_str(" ");
            cmd.push_str(&self.target);

            return cmd;
        }
    }

    fn set_default_executable() -> Executable {
        Executable::NODE
    }
    fn set_default_target() -> String {
        String::new()
    }
    fn set_default_inspect() -> bool {
        false
    }
    fn set_default_ignore_list() -> Vec<String> {
        vec![]
    }
    fn set_default_watch_list() -> Vec<String> {
        vec![]
    }
    fn set_default_delay() -> u64 {
        2000
    }

    fn get_executable(exec: &str) -> Executable {
        match exec {
            "NODE" => Executable::NODE,
            "GOLANG" => Executable::GOLANG,
            "PYTHON" => Executable::PYTHON,
            "CARGO" => Executable::CARGO,
            _ => Executable::NODE,
        }
    }
}

pub mod processor {
    use crate::{constants::CONFIG_FILE, types::Executable};
    use std::{
        fs::File,
        io::Read,
        process::{Child, Command, Stdio},
    };

    use crate::types::Config;

    pub fn load_file_config() -> Option<Config> {
        let conf_file = File::open(&CONFIG_FILE);

        match conf_file {
            Ok(mut config_file) => {
                let mut config_string: String = String::new();
                match config_file.read_to_string(&mut config_string) {
                    Ok(_) => {
                        let conf: Config = serde_json::from_str(&config_string).unwrap();
                        Some(conf)
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    pub fn run_command(config: &Config) -> Option<Child> {
        let cmd_v: Vec<String> = config
            .get_command()
            .split(" ")
            .map(|str| str.trim().to_string())
            .collect();

        println!("Starting wachit process...");
        let base_cmd = &cmd_v[0];
        let args = &cmd_v[1..];
        let mut command = Command::new(base_cmd);
        command.args(args);

        match command.spawn() {
            Ok(child) => Some(child),
            Err(err) => {
                println!("{:?}", err);
                None
            }
        }
    }

    pub fn should_ignore_path(path: &String, config: &Config) -> bool {
        if (path.contains("wachit.json")
            || path.contains("build")
            || path.contains("node_modules")
            || path.ends_with(".env")
            || path.ends_with(".gitignore")
            || path.ends_with(".next")
            || path.ends_with(".git")
            || (config.executable == Executable::CARGO && path.contains("target"))
            || path.contains(".vscode")
            || path.contains("dist"))
            || !config.should_restart(path)
        {
            return true;
        }

        return false;
    }
}
