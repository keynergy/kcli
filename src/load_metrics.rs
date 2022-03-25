use crate::keyboards;
use crate::Config;
use crate::Data;
use dirs;
use keynergy::analysis::MetricList;
use keynergy::Analyzer;
use std::fs::File;
use std::io::Write;
use toml;

pub fn load_metrics<'a>(data: &'a Data, cfg: &'a Config) {
    let td = data.corpus_list.get(&cfg.default_corpus).unwrap();
    let metrics: MetricList = toml::from_str(
        &std::fs::read_to_string(cfg.data_dir.join("metrics").join("metric_list.toml")).unwrap(),
    )
	.unwrap();
    let mut analyzer = Analyzer::with(metrics, td);
    let r = analyzer.run_ket_code(
        std::fs::read_to_string(cfg.data_dir.join("metrics").join("metrics.ket")).unwrap(),
    );
    analyzer.trace(r).unwrap();

    for k in &[keyboards::ansi(), keyboards::matrix()] {
        analyzer.calculate_metrics(k).unwrap();
    }
    let map = analyzer.keyboard_stats;
    let mut f = File::create(cfg.data_dir.join(".stats")).unwrap();
    f.write(&bincode::serialize(&map).unwrap()).unwrap();
}
