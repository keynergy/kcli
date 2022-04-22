use crate::keyboards;
use crate::Config;
use crate::Data;
use keynergy::Analyzer;
use keynergy::Keys;

pub fn rank(data: &Data, cfg: &Config, metric_query: String) {
    if !data.metrics.contains_key(metric_query) {
	println!("Unknown metric `{}`", metric_query);
    }
    let td = data.corpus_list.get(&cfg.default_corpus).unwrap();
    let mut analyze = Analyzer::with(
	toml::from_str(
	    &std::fs::read_to_string(cfg.data_dir.join("metrics").join("metric_list.toml"))
		.unwrap(),
	)
	    .unwrap(),
	td,
    );
    analyze.keyboard_stats = data.kb_stats.clone();
    let r = analyzer.analyze_keys(keyboards::matrix(), k.clone());
    let mut layouts: Vec<[Keys, u64]> = Vec::with_capacity(data.layouts.len());
    data.layouts.map(|l| layouts.push)
	if let Some(amounts) = r {
	    let mut total: u64 = 
	}
}
