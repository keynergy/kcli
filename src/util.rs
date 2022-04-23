use keynergy::analysis::MetricTotal;
use keynergy::TextData;
use keynergy::Keys;

pub fn display_metric_total(mt: &MetricTotal, total: u64) -> String {
    format!(
        "{}",
        match *mt {
            MetricTotal::Count(c) => format!("{:.2}%", (100 * c) as f64 / total as f64),
            MetricTotal::Scalar(s) => format!("{:.1}", s),
        }
    )
}

pub fn total(k: &Keys, td: &TextData) -> u64 {
    let mut total: u64 = 0;
    for (c, f) in &td.chars {
        if k.map.contains_key(c) {
            total += f;
        }
    }
    total
}
