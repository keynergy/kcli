use crate::{Config, Data};
use console::Term;
use dialoguer::{Input, Select};
use indicatif::{ProgressBar, ProgressStyle};
use keynergy::TextData;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub type CorpusList = HashMap<String, TextData>;

pub fn list(data: &Data) {
    for (name, d) in &data.corpus_list {
        let total: u64 = d.chars.values().sum();
        println!("{}: {} words", *name, total / 5);
    }
    if &data.corpus_list.len() == &0 {
        println!("No corpora stored.");
    }
}

fn process(text: String) -> TextData {
    let pb = ProgressBar::new(text.len() as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
        )
        .unwrap()
        .with_key("eta", |state| format!("{:.0}s", state.eta().as_secs_f64()))
        .progress_chars("███"),
    );
    let mut chars: HashMap<char, u64> = HashMap::with_capacity(30);
    let mut bigrams: HashMap<[char; 2], u64> = HashMap::with_capacity(30 * 30);
    let mut trigrams: HashMap<[char; 3], u64> = HashMap::with_capacity(30 * 30 * 15);
    let mut skip_1_grams: HashMap<[char; 2], u64> = HashMap::with_capacity(30 * 30);
    for (i, v) in text
        .chars()
        .map(|x| x.to_ascii_lowercase())
        .collect::<Vec<char>>()
        .windows(3)
        .enumerate()
    {
        if i % 50000 == 0 {
            pb.inc(50000);
        }
        let ch = chars.entry(v[0]).or_insert(0);
        *ch += 1;
        if v.len() >= 2 {
            let bg = bigrams.entry([v[0], v[1]]).or_insert(0);
            *bg += 1;
        }
        if v.len() == 3 {
            let tg = trigrams.entry([v[0], v[1], v[2]]).or_insert(0);
            let sg = skip_1_grams.entry([v[0], v[1]]).or_insert(0);
            *tg += 1;
            *sg += 1;
        }
    }
    pb.finish_with_message("done!");
    TextData {
        chars,
        bigrams,
        trigrams,
        skip_1_grams,
    }
}

pub fn load(data: &mut Data, f: &PathBuf) {
    let text = match fs::read_to_string(f) {
        Ok(t) => t,
        Err(_) => {
            println!("Error reading file.");
            std::process::exit(1);
        }
    };
    let name = Input::new()
        .with_prompt("Enter a name for the corpus")
        .with_initial_text(f.file_name().unwrap().to_owned().into_string().unwrap())
        .interact()
        .unwrap();

    println!("Loading corpus \"{}\"...", &name);
    data.corpus_list.insert(name, process(text));
}

pub fn default(data: &Data, cfg: &mut Config) {
    let items: Vec<&String> = data.corpus_list.keys().collect();
    cfg.default_corpus = items[Select::new()
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .unwrap()
        .unwrap()]
    .to_string();
}

pub fn remove(data: &mut Data) {
    let items: Vec<String> = data
        .corpus_list
        .keys()
        .map(|s| s.clone())
        .collect::<Vec<String>>();
    data.corpus_list
        .remove(&items[Select::new().items(&items).default(0).interact().unwrap()]);
}
