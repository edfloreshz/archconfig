use dotsy_daemon::daemon;

pub fn start() -> Result<(), std::io::Error> {
    daemon::construct()
}

pub fn show() -> Result<(), std::io::Error> {
    daemon::show()
}
