use daemonize::Daemonize;
use dirs;
use std::fs;

use crate::watch::watch;

pub fn construct() {
    let home = dirs::data_dir().unwrap().join("dotsy");
    let stdout = fs::File::open(home.join("logs/daemon.out")).unwrap();
    let stderr = fs::File::open(home.join("logs/daemon.err")).unwrap();
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
