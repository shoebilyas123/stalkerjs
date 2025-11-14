pub mod types {
    use serde::{Deserialize, Serialize};
    use serde_json::{Map, Value};
    use std::{
        fs::{self, File},
        io::{ErrorKind, Read},
    };

    #[derive(Serialize, Deserialize, Debug)]
    pub enum ConfigValues {
        String,
    }
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    pub enum PackageManager {
        NPM,
        PNPM,
        YARN,
        NIL,
    }
    impl Default for PackageManager {
        fn default() -> Self {
            PackageManager::NPM
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Config {
        #[serde[default = "set_default_scripts"]]
        pub scripts: Map<String, Value>,

        #[serde(default = "set_default_pkg_manager")]
        pub pkg_manager: PackageManager,

        #[serde(default = "set_default_target_file")]
        pub target_file: String,

        #[serde(default = "set_default_args")]
        pub args: Vec<String>,

        #[serde(default = "set_default_inspect")]
        pub inspect: bool,

        #[serde(default = "set_default_target_index")]
        pub target_index: usize,
    }
    impl Config {
        pub fn new(project_dir: &str, args: Vec<String>) -> Config {
            let pkg_mngr_options = vec!["pnpm-lock.yaml", "package-lock.json", "yarn.lock"];
            let mut pkg_manager = PackageManager::NPM;

            for (idx, opt) in pkg_mngr_options.iter().enumerate() {
                match fs::exists(opt) {
                    Ok(true) => match idx {
                        0 => {
                            pkg_manager = PackageManager::PNPM;
                            break;
                        }
                        1 => {
                            pkg_manager = PackageManager::NPM;
                            break;
                        }
                        2 => {
                            pkg_manager = PackageManager::YARN;
                            break;
                        }
                        _ => {
                            pkg_manager = PackageManager::NIL;
                            break;
                        }
                    },
                    _ => {
                        continue;
                    }
                }
            }

            // Check if the first argument is a file
            // We will process our command based on this condition
            let mut target_index = 1;
            let mut inspect = false;
            if args[1].eq("--inspect") {
                target_index = 2;
                inspect = true;
            }

            let target = File::open(&args[target_index]);

            let target_file: String = match target {
                Ok(_) => String::from(&args[target_index]),
                Err(_) => String::new(),
            };

            let mut new_config = Config {
                scripts: Map::new(),
                pkg_manager,
                target_file,
                args,
                inspect,
                target_index,
            };
            let file_result = File::open(project_dir);

            match file_result {
                Ok(mut file) => {
                    let mut pkg_json_file = String::new();
                    match file.read_to_string(&mut pkg_json_file) {
                        Ok(_) => {
                            let conf: Config = serde_json::from_str(&pkg_json_file).unwrap();
                            new_config.scripts = conf.scripts;
                        }
                        Err(err) => {
                            panic!("Error reading from package.json = {:?}", err);
                        }
                    }
                }
                Err(err) => match err.kind() {
                    ErrorKind::NotFound => {
                        println!("Missing file: package.json");
                    }
                    ErrorKind::PermissionDenied => {
                        println!("The user does not have the permission to access package.json");
                    }
                    _ => {
                        println!("Error opening package.json:{:?}", err);
                    }
                },
            };

            new_config
        }
        pub fn get_command(&self) -> String {
            if self.target_file.len() <= 0 && self.inspect == true {
                panic!("Missing: Node inspect file path");
            } else if self.target_file.len() > 0 {
                let mut cmd = String::from("node ");
                if self.inspect == true {
                    cmd.push_str("--inspect ");
                }

                cmd.push_str(&self.target_file);
                cmd
            } else {
                let command: String = match self
                    .scripts
                    .get(&self.args[self.target_index])
                    .and_then(|v| v.as_str())
                {
                    Some(cmd) => String::from(cmd),
                    _ => {
                        let mut cmd_str = match self.pkg_manager {
                            PackageManager::NPM => String::from("npm run "),
                            PackageManager::PNPM => String::from("pnpm run "),
                            PackageManager::YARN => String::from("yarn "),
                            PackageManager::NIL => String::from("node "),
                        };

                        cmd_str.push_str(&self.args[self.target_index]);

                        let target = &self.args[self.target_index];

                        if target.ends_with(".js") || target.ends_with(".ts") {
                            cmd_str.push_str(" index.js");
                        }

                        cmd_str
                    }
                };

                return command;
            }
        }

        pub fn get_arguments(&self) -> &Vec<String> {
            &self.args
        }
    }

    fn set_default_pkg_manager() -> PackageManager {
        PackageManager::NPM
    }
    fn set_default_scripts() -> Map<String, Value> {
        Map::new()
    }
    fn set_default_target_file() -> String {
        String::new()
    }
    fn set_default_args() -> Vec<String> {
        Vec::new()
    }

    fn set_default_inspect() -> bool {
        false
    }

    fn set_default_target_index() -> usize {
        1
    }
}

pub mod processor {
    use std::{
        io::Stdin,
        process::{Child, Command, Stdio},
    };

    use crate::types::Config;

    pub fn run_command(config: &Config) -> Option<Child> {
        let cmd_v: Vec<String> = config
            .get_command()
            .split(" ")
            .map(|str| str.trim().to_string())
            .collect();

        println!("Starting stalkerjs");
        let base_cmd = &cmd_v[0];
        let args = &cmd_v[1..];
        let mut command = Command::new(base_cmd);
        command.args(args);
        command.args(&config.args[config.target_index + 1..]);
        // command.stdin(Stdio::piped()).stdout(Stdio::piped());

        match command.spawn() {
            Ok(child) => Some(child),
            Err(err) => {
                println!("{:?}", err);
                None
            }
        }
    }

    pub fn should_ignore_path(path: &String) -> bool {
        if path.contains("build")
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

#[cfg(test)]
mod tests {
    use crate::types::{Config, PackageManager};
    use std::fs;

    #[test]
    fn test_read_pkg_json() {
        let _: Config = Config::new("./test_data/package.json", vec![]);
        assert!(true);
    }

    #[test]
    fn test_read_pkg_json_scripts() {
        let conf: Config = Config::new("./test_data/package.json", vec![]);
        assert_eq!(conf.scripts["build"].as_str(), Some("npm run build"));
        assert_eq!(conf.scripts["dev"].as_str(), Some("npm run dev"));
        assert_eq!(conf.scripts["start"].as_str(), Some("npm run start"));
    }

    #[test]
    fn test_pkg_managers() {
        let conf: Config = Config::new("./test_data/package.json", vec![]);
        let pkg_mngr_options = vec!["pnpm-lock.yaml", "package-lock.json", "yarn.lock"];

        for (idx, opt) in pkg_mngr_options.iter().enumerate() {
            match fs::exists(opt) {
                Ok(true) => match idx {
                    0 => {
                        assert_eq!(conf.pkg_manager, PackageManager::PNPM);
                        break;
                    }
                    1 => {
                        assert_eq!(conf.pkg_manager, PackageManager::NPM);

                        break;
                    }
                    2 => {
                        assert_eq!(conf.pkg_manager, PackageManager::YARN);
                        break;
                    }
                    _ => {
                        assert_eq!(conf.pkg_manager, PackageManager::NPM);
                        break;
                    }
                },
                _ => {
                    continue;
                }
            }
        }
    }

    #[test]
    fn test_script_command() {
        let args = Vec::from([String::from(""), String::from("dev")]);
        let conf: Config = Config::new("./test_data/package.json", args);

        let cmd = conf.get_command();

        assert_eq!(cmd, String::from("npm run dev"))
    }
}
