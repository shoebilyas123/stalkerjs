pub mod constants {
    pub const CONFIG_FILE: &str = "wachit.json";
    pub const ERR_MESSAGE: &str = "wacht [wachit options] [target file]";
}

pub mod types {
    use std::{
        fmt,
        fs::{self, File},
        io::{ErrorKind, Read},
    };

    use serde::{Deserialize, Serialize};
    use serde_json::{Map, Value};

    use crate::constants::ERR_MESSAGE;
    use crate::processor;

    #[derive(Serialize, Deserialize, Debug)]
    pub enum ConfigValues {
        String,
    }
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    pub enum Executable {
        NODE,
    }
    impl fmt::Display for Executable {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Executable::NODE => write!(f, "node"),
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Config {
        #[serde(default = "set_default_executable")]
        pub executable: Executable,

        #[serde(default = "set_default_target")]
        pub target: String,

        #[serde(default = "set_default_inspect")]
        pub inspect: bool,
    }
    impl Config {
        pub fn create_default_config() -> Config {
            Config {
                inspect: set_default_inspect(),
                executable: set_default_executable(),
                target: set_default_target(),
            }
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
            let mut inspect = false;
            if args[1].eq("--inspect") {
                target_index = 2;
                inspect = true;
            }
            new_config.inspect = inspect;

            if target_index >= args.len() {
                panic!("{ERR_MESSAGE}");
            }

            let target = File::open(&args[target_index]);

            let target_file: String = match target {
                Ok(_) => String::from(&args[target_index]),
                Err(_) => {
                    panic!("Invalid arguments: Target file not specified");
                }
            };
            new_config.target = target_file;
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
}

pub mod processor {
    use crate::constants::CONFIG_FILE;
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

        println!("{:?}", config.get_command());

        match command.spawn() {
            Ok(child) => Some(child),
            Err(err) => {
                println!("{:?}", err);
                None
            }
        }
    }

    pub fn should_ignore_path(path: &String) -> bool {
        if path.contains("wachit.json")
            || path.contains("build")
            || path.contains("node_modules")
            || path.ends_with(".env")
            || path.ends_with(".gitignore")
            || path.ends_with(".next")
            || path.ends_with(".git")
            || path.contains("target")
            || path.contains(".vscode")
            || path.contains("dist")
            || (!(path.ends_with(".ts"))
                && !(path.ends_with(".js"))
                && !(path.ends_with(".tsx"))
                && !(path.ends_with(".jsx"))
                && !(path.ends_with(".json")))
        {
            return true;
        }

        return false;
    }
}
