use crate::keyboards;
use crate::Config;
use crate::Data;
use crate::util;
use keynergy::Analyzer;
use keynergy::Keys;
use keynergy::analysis::MetricTotal;

pub fn rank(data: &Data, cfg: &Config, metric_query: String, least: bool, amount: usize) {
    if !data.metrics.bigrams.contains_key(&metric_query)
        && !data.metrics.trigrams.contains_key(&metric_query)
    {
        println!("Unknown metric `{}`", metric_query);
	return;
    }
    let td = data.corpus_list.get(&cfg.default_corpus).unwrap();
    let mut analyzer = Analyzer::with(
        toml::from_str(
            &std::fs::read_to_string(cfg.data_dir.join("metrics").join("metric_list.toml"))
                .unwrap(),
        )
        .unwrap(),
        td,
    );
    analyzer.keyboard_stats = data.kb_stats.clone();
    let mut results: Vec<(String, MetricTotal, u64)> = Vec::with_capacity(data.layouts.len());
    for (_, l) in &data.layouts {
	if let Some(keys) = &l.formats.standard {
	    if let Some(a) = analyzer.analyze_keys(keyboards::matrix(), keys.clone()) {
		results.push((l.name.clone(), a.get(&metric_query).unwrap().clone(), util::total(&keys, td)));
	    }
	}
    }
    results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    if least {
	results.reverse();
    }
    for (name, mt, total) in results.chunks(amount).next().unwrap() {
	println!("{}: {}", name, crate::util::display_metric_total(&mt, *total));
    }
}
