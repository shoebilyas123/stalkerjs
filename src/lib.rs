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
    }
    impl Default for PackageManager {
        fn default() -> Self {
            PackageManager::NPM
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Config {
        pub scripts: Map<String, Value>,

        #[serde(default = "default_pkg_manager")]
        pub pkg_manager: PackageManager,
    }

    fn default_pkg_manager() -> PackageManager {
        PackageManager::NPM // or whatever default you want
    }

    impl Config {
        pub fn new(project_dir: &str) -> Config {
            // TODO: Make this a global constant variable;
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
                            pkg_manager = PackageManager::NPM;
                            break;
                        }
                    },
                    _ => {
                        continue;
                    }
                }
            }

            let mut new_config = Config {
                scripts: Map::new(),
                pkg_manager,
            };
            let file_result = File::open(project_dir);

            match file_result {
                Ok(mut file) => {
                    let mut pkg_json_file = String::new();
                    match file.read_to_string(&mut pkg_json_file) {
                        Ok(_) => {
                            let conf: Config = serde_json::from_str(&pkg_json_file).unwrap();
                            new_config.scripts = conf.scripts;
                            return new_config;
                        }
                        Err(err) => {
                            panic!("Error reading from package.json = {:?}", err);
                        }
                    }
                }
                Err(err) => match err.kind() {
                    ErrorKind::NotFound => {
                        panic!("Missing file: package.json");
                    }
                    ErrorKind::PermissionDenied => {
                        panic!("The user does not have the permission to access package.json");
                    }
                    _ => {
                        panic!("Error opening package.json:{:?}", err);
                    }
                },
            };
        }
        pub fn get_command(&self, cmd: &str) -> String {
            let command: String = match self.scripts[cmd].as_str() {
                Some(cmd_str) => String::from(cmd_str),
                None => {
                    let mut cmd_str = match self.pkg_manager {
                        PackageManager::NPM => String::from("npm run"),
                        PackageManager::PNPM => String::from("pnpm run"),
                        PackageManager::YARN => String::from("yarn"),
                    };

                    cmd_str.push_str(cmd);
                    cmd_str
                }
            };

            return command;
        }
    }
}

pub mod processor {
    use std::process::{Child, Command};

    pub fn run_command(cmd: &str) -> Option<Child> {
        let cmd_v: Vec<String> = cmd.split(" ").map(|str| str.trim().to_string()).collect();

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
}

#[cfg(test)]
mod tests {
    use crate::types::{Config, PackageManager};
    use std::fs;

    #[test]
    fn test_read_pkg_json() {
        let _: Config = Config::new("./test_data/package.json");
        assert!(true);
    }

    #[test]
    fn test_read_pkg_json_scripts() {
        let conf: Config = Config::new("./test_data/package.json");
        assert_eq!(conf.scripts["build"].as_str(), Some("npm run build"));
        assert_eq!(conf.scripts["dev"].as_str(), Some("npm run dev"));
        assert_eq!(conf.scripts["start"].as_str(), Some("npm run start"));
    }

    #[test]
    fn test_pkg_managers() {
        let conf: Config = Config::new("./test_data/package.json");
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
        let conf: Config = Config::new("./test_data/package.json");

        let cmd = conf.get_command("dev");

        assert_eq!(cmd, String::from("npm run dev"))
    }
}
