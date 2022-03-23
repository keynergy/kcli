use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub data_dir: PathBuf,
}

/// `Config` implements `Default`
impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            data_dir: PathBuf::from(""),
        }
    }
}
