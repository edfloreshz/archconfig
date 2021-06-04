use daemonize::Daemonize;
use dirs;
use std::fs;

use crate::watch::watch;
use std::process::Command;

pub fn construct() -> Result<(), std::io::Error> {
    let home = dirs::data_dir().unwrap().join("dotsy");
    let stdout = fs::File::create(home.join("logs/daemon.out"))?;
    let stderr = fs::File::create(home.join("logs/daemon.err"))?;
    let daemonize = Daemonize::new()
        .working_directory(&home) // for default behaviour.
        .stdout(stdout) // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr) // Redirect stderr to `/tmp/daemon.err`.
        .privileged_action(|| "Executed before drop privileges");

    println!("Waiting for changes...");
    let output = Command::new("sh")
        .arg("-c")
        .arg("tail")
        .arg("-f")
        .arg(
            home.join("logs/daemon.out")
                .to_str()
                .expect("Failed to convert path to str"),
        )
        .output()
        .expect("Failed to execute tail.");
    println!("{:?}", output.status);
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
