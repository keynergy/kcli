use crate::keyboards;
use crate::Config;
use crate::Data;
use keynergy::analysis::ErrorType;
use keynergy::Analyzer;

pub fn refresh<'a>(data: &'a mut Data, cfg: &'a Config) {
    let td = match data.corpus_list.get(&cfg.default_corpus) {
        Some(c) => c,
        None => {
            println!("No corpora available.");
            std::process::exit(1);
        }
    };
    data.metrics = Data::get_metrics(cfg);
    data.layouts = Data::get_layouts(cfg);
    let mut analyzer = Analyzer::with(data.metrics.clone(), td);
    let r = analyzer.run_ket_code(
        std::fs::read_to_string(cfg.data_dir.join("metrics").join("metrics.ket")).unwrap(),
    );
    analyzer.trace(r).unwrap();

    for k in &[keyboards::ansi(), keyboards::matrix()] {
        let result = analyzer.calculate_metrics(k);
        if let Err(e) = result {
            if let ErrorType::Ketos(ke) = e.error {
                analyzer.trace_err(ke);
            }
        }
    }
    data.kb_stats = analyzer.keyboard_stats;
    data.save(cfg);
}
