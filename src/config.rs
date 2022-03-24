use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub data_dir: PathBuf,
    pub default_corpus: String,
}

/// `Config` implements `Default`
impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            data_dir: PathBuf::from(""),
            default_corpus: String::new(),
        }
    }
}
