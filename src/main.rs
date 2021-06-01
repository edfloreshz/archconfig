use core::panic;
use daemonize::Daemonize;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::fs::{self, File};
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() {
    let home = dirs::data_dir().unwrap().join("dotsy");
    if let Err(e) = fs::create_dir_all(&home.join("logs")) {
        panic!("{} at {:?}", e, home);
    }
    let stdout = File::create(home.join("logs/daemon.out")).unwrap();
    let stderr = File::create(home.join("logs/daemon.err")).unwrap();

    let daemonize = Daemonize::new()
        .working_directory(&home) // for default behaviour.
        .stdout(stdout) // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr) // Redirect stderr to `/tmp/daemon.err`.
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => {
            if let Err(e) = watch() {
                println!("error: {:?}", e)
            }
        }
        Err(e) => eprintln!("Error, {}", e),
    }
}

fn watch() -> notify::Result<()> {
    let home = dirs::home_dir().unwrap();
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1))?;
    watcher.watch(&home.join(".zshrc"), RecursiveMode::Recursive)?;
    watcher.watch(&home.join("Desktop"), RecursiveMode::Recursive)?;
    loop {
        match rx.recv() {
            Ok(event) => println!("{:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
