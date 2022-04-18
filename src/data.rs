use crate::corpus::CorpusList;
use crate::Config;
use bincode;
use keynergy::analysis::{MetricList, MetricMap};
use keynergy::Layout;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{
    fs::{self, File},
    io::Write,
};

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub layouts: HashMap<String, Layout>,
    pub metrics: MetricList,
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
        let layouts: HashMap<String, Layout> = match fs::read(cfg.data_dir.join(".layouts")) {
            Ok(b) => bincode::deserialize(&b).unwrap(),
            Err(_) => HashMap::new(),
        };
        let metrics: MetricList = match fs::read(cfg.data_dir.join(".metrics")) {
            Ok(b) => bincode::deserialize(&b).unwrap(),
            Err(_) => MetricList::new(),
        };

        Data {
            layouts,
            metrics,
            corpus_list,
            kb_stats,
        }
    }
    pub fn get_layouts(cfg: &Config) -> HashMap<String, Layout> {
        let mut layouts = HashMap::new();
        let dir = fs::read_dir(cfg.data_dir.join("layouts")).unwrap();
        for f in dir.flatten() {
            if let Some(path) = f.path().to_str() {
                if let Ok(l) = Layout::load(path) {
                    let name = l.name.to_ascii_lowercase();
                    layouts.insert(name.clone(), l);
                }
            }
        }
        layouts
    }
    pub fn get_metrics(cfg: &Config) -> MetricList {
        toml::from_str(
            &std::fs::read_to_string(cfg.data_dir.join("metrics").join("metric_list.toml"))
                .unwrap(),
        )
        .unwrap()
    }
    pub fn save(&self, cfg: &Config) {
        let mut f = File::create(cfg.data_dir.join(".corpora")).unwrap();
        f.write_all(&bincode::serialize(&self.corpus_list).unwrap())
            .unwrap();

        let mut f = File::create(cfg.data_dir.join(".stats")).unwrap();
        f.write_all(&bincode::serialize(&self.kb_stats).unwrap())
            .unwrap();

        let mut f = File::create(cfg.data_dir.join(".layouts")).unwrap();
        f.write_all(&bincode::serialize(&self.layouts).unwrap())
            .unwrap();

        let mut f = File::create(cfg.data_dir.join(".metrics")).unwrap();
        f.write_all(&bincode::serialize(&self.metrics).unwrap())
            .unwrap();
    }
}
