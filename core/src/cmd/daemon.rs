use dotsy_daemon::daemon;

pub struct DaemonOptions {
    pub show: bool,
}

pub fn start() -> Result<(), std::io::Error> {
    daemon::construct()
}

pub fn show() -> Result<(), std::io::Error> {
    daemon::show()
}
