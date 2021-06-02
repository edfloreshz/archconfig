pub enum Error {
    MissingArg,
    MissingSubCmd,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::MissingArg => write!(f, "Argument missing"),
            Error::MissingSubCmd => write!(f, "Subcommand not found"),
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::MissingArg => write!(
                f,
                "{{ message: argument missing, file: {}, line: {} }}",
                file!(),
                line!()
            ),
            Error::MissingSubCmd => write!(
                f,
                "{{ message: subcommand not found, file: {}, line: {} }}",
                file!(),
                line!()
            ),
        }
    }
}
