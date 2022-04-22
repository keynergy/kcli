use crate::keyboards;
use crate::Config;
use crate::Data;
use keynergy::analysis::MetricTotal;
use keynergy::Analyzer;
use keynergy::Keys;

pub fn analyze(data: &Data, cfg: &Config, k: &Keys) {
    println!("{}", *k);
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
    let r = analyzer.analyze_keys(keyboards::matrix(), k.clone());
    if let Some(metrics) = r {
        let mut names: Vec<&String> = metrics.keys().collect();
        names.sort();
        let mut total: u64 = 0;
        for (c, f) in &td.chars {
	    if k.map.contains_key(c) {
                total += f;
	    }
        }
        for m in names {
	    let t = metrics.get(m).unwrap();
	    println!(
                "{}: {}",
                m,
                match *t {
		    MetricTotal::Count(c) => format!("{:.2}%", (100 * c) as f64 / total as f64),
		    MetricTotal::Scalar(s) => format!("{:.1}", s),
                }
	    );
        }
	} else {
        println!("Couldn't analyze this layout.");
    }
}
