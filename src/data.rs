use crate::corpus::CorpusList;
use crate::Config;
use bincode;
use keynergy::analysis::MetricMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{
    fs::{self, File},
    io::Write,
};

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub corpus_list: CorpusList,
    pub kb_stats: HashMap<String, MetricMap>,
}

impl Data {
    pub fn load(cfg: &Config) -> Self {
        let corpus_list: CorpusList = match fs::read(cfg.data_dir.join(".corpora")) {
            Ok(b) => bincode::deserialize(&b).unwrap(),
            Err(_) => HashMap::new(),
        };
        let kb_stats: HashMap<String, MetricMap> = match fs::read(cfg.data_dir.join(".stats")) {
            Ok(b) => bincode::deserialize(&b).unwrap(),
            Err(_) => HashMap::new(),
        };
        Data {
            corpus_list,
            kb_stats,
        }
    }
    pub fn save(self, cfg: &Config) {
        let mut f = File::create(cfg.data_dir.join(".corpora")).unwrap();
        f.write(&bincode::serialize(&self.corpus_list).unwrap())
            .unwrap();

        let mut f = File::create(cfg.data_dir.join(".stats")).unwrap();
        f.write(&bincode::serialize(&self.kb_stats).unwrap())
            .unwrap();
    }
}
