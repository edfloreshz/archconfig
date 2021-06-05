use crate::models::config::UserConfig;
use std::fs::File;

#[derive(Debug)]
pub struct OnlineOptions {
    pub data: UserConfig,
}

#[derive(Debug)]
pub struct FileOptions {
    pub file: Option<File>,
}
