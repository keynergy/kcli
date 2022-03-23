use crate::keyboards;
use crate::Config;
use dirs;
use keynergy::Analyzer;

fn load_metrics(cfg: Config) {
    let cache_dir = dirs::cache_dir().unwrap();

    for k in [keyboards::ansi(), keyboards::matrix()] {
        let analyzer = Analyzer::with(&k, td);
    }
}
  