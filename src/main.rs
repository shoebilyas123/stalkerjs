use std::{
    path::Path,
    process::Command,
    sync::{Arc, Mutex, atomic::AtomicBool, mpsc},
    time::{Duration, Instant},
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
    let last_event = Arc::new(Mutex::new(Instant::now()));
    let pending_restart = Arc::new(Mutex::new(AtomicBool::new(false)));
    let pending_restart_clone = pending_restart.clone();
    let last_event_watcher = last_event.clone();
    let config_clone = confg.clone();
    let restart_tx = tx.clone();

    let mut watcher = notify::recommended_watcher(move |event: Result<Event>| {
        let event = match event {
            Ok(ev) => ev,
            Err(_) => return,
        };

        let mut ignore_path = false;
        for path in &event.paths {
            let p = path.to_str().unwrap().to_string();
            ignore_path = should_ignore_path(&p, &config_clone);
        }
        if ignore_path {
            return;
        }

        *last_event_watcher.lock().unwrap() = Instant::now();
        pending_restart
            .lock()
            .unwrap()
            .store(true, std::sync::atomic::Ordering::SeqCst);
    })
    .unwrap();

    let _ = watcher.watch(Path::new("."), notify::RecursiveMode::Recursive);

    let mut pid: u32 = 0;
    let child = run_command(&confg);
    pid = child.unwrap().id();

    let confg_clone = confg.clone();
    let last_event_debounce = last_event.clone();

    std::thread::spawn(move || {
        loop {
            let flag = pending_restart_clone
                .lock()
                .unwrap()
                .load(std::sync::atomic::Ordering::SeqCst);

            if !flag {
                continue;
            }

            std::thread::sleep(Duration::from_millis(30));

            let elapsed = last_event_debounce.lock().unwrap().elapsed();
            if elapsed >= Duration::from_millis(confg_clone.delay) {
                println!("Restarting server after delay");
                let _ = restart_tx.send(Ok(Event::new(Default::default())));

                pending_restart_clone
                    .lock()
                    .unwrap()
                    .store(false, std::sync::atomic::Ordering::SeqCst);
            }
        }
    });

    for res in rx {
        match res {
            Ok(event) => {
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
