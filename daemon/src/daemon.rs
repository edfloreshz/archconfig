use colour::{self, green, white, yellow_ln};
use daemonize::Daemonize;
use dirs;
use psutil;
use std::{fs, path::PathBuf};

use crate::watch::watch;
use std::process::Command;

pub fn construct() -> Result<(), std::io::Error> {
    let home = dirs::data_dir().unwrap_or(PathBuf::new()).join("dotsy");
    let stdout = fs::File::create(home.join("logs/daemon.out"))?;
    let stderr = fs::File::create(home.join("logs/daemon.err"))?;
    let daemonize = Daemonize::new()
        .working_directory(&home) // for default behaviour.
        .stdout(stdout) // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr) // Redirect stderr to `/tmp/daemon.err`.
        .privileged_action(|| "Executed before drop privileges");

    white!("Run ");
    green!("dotsy daemon -s ");
    white!("to show the output.\n");
    match daemonize.start() {
        Ok(_) => {
            if let Err(e) = watch() {
                println!("error: {:?}", e)
            }
        }
        Err(e) => {
            eprintln!("Error, {}", e);
        }
    }
    Ok(())
}

pub fn show() -> Result<(), std::io::Error> {
    let processes = psutil::process::processes().unwrap_or_default();
    let mut count = 0;
    for process in processes {
        if process.unwrap().name().unwrap() == "dotsy" {
            count += 1;
        }
    }
    if count > 1 {
        let home = dirs::data_dir().unwrap_or(PathBuf::new()).join("dotsy");
        yellow_ln!("Waiting for changes...");
        Command::new("tail")
            .arg("-f")
            .arg(
                home.join("logs/daemon.out")
                    .to_str()
                    .expect("Failed to convert path to str"),
            )
            .status()
            .expect("Failed to tail.");
    } else {
        white!("First, run ");
        green!("dotsy daemon.\n");
    }
    Ok(())
}
