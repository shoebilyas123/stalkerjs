use std::{path::Path, process::Command, sync::mpsc};

use notify::{Event, Result, Watcher};
use stalkerjs::{processor::run_command, types::Config};

fn should_ignore_path(path: &String) -> bool {
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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let confg = Config::new("./package.json");

    if args.len() <= 1 {
        panic!("Missing: Command name");
    }

    let cmd = confg.get_command(&args[1]);

    let (tx, rx) = mpsc::channel::<Result<Event>>();

    let mut watcher = notify::recommended_watcher(tx).unwrap();
    let _ = watcher.watch(Path::new("."), notify::RecursiveMode::Recursive);

    let mut pid: u32 = 0;

    let child = run_command(&cmd);
    pid = child.unwrap().id();

    for res in rx {
        match res {
            Ok(event) => {
                let mut ignore_path = false;

                for path in event.paths {
                    let p = path.to_str().unwrap().to_string();
                    ignore_path = should_ignore_path(&p);
                }

                if ignore_path {
                    continue;
                }

                if pid != 0 {
                    let output = Command::new("kill")
                        .arg("-9")
                        .arg(pid.to_string())
                        .output()
                        .expect("failed to execute kill command");

                    if output.status.success() {
                        println!("Restarting development server");
                    } else {
                        eprintln!(
                            "Failed to kill process: {}",
                            String::from_utf8_lossy(&output.stderr)
                        );
                    }
                }
                let child = run_command(&cmd);
                pid = child.unwrap().id();
            }
            Err(_) => {}
        }
    }
}
