use dotsy_daemon;

pub fn start() -> Result<(), std::io::Error> {
    dotsy_daemon::daemon::construct()
}
