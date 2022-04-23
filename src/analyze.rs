use crate::keyboards;
use crate::Config;
use crate::Data;
use crate::util;
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
        
        for m in names {
            let t = metrics.get(m).unwrap();
            println!("{}: {}", m, util::display_metric_total(t, util::total(k, td)));
        }
    } else {
        println!("Couldn't analyze this layout.");
    }
}
