use std::{
    io::{BufRead, BufReader, Error, Read, Write},
    path::Path,
    process::Command,
    sync::mpsc,
};

use notify::{Event, Result, Watcher};
use wachit::{
    processor::{run_command, should_ignore_path},
    types::Config,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let confg = Config::new(args);

    let (tx, rx) = mpsc::channel::<Result<Event>>();

    let mut watcher = notify::recommended_watcher(tx).unwrap();
    let _ = watcher.watch(Path::new("."), notify::RecursiveMode::Recursive);

    let mut pid: u32 = 0;

    let child = run_command(&confg);
    pid = child.unwrap().id();

    for res in rx {
        match res {
            Ok(event) => {
                let mut ignore_path = false;

                for path in event.paths {
                    let p = path.to_str().unwrap().to_string();
                    ignore_path = should_ignore_path(&p, &confg);
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
                let child = run_command(&confg).unwrap();
                pid = child.id();
            }
            Err(_) => {}
        }
    }
}
