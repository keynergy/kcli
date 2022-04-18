use crate::config::Config;
use dialoguer::Input;
use git2::Repository;
use std::{fs, path::Path};

pub fn setup(dir: &Option<String>) -> Config {
    let dir = match dir {
        Some(d) => d.clone(),
        None => Input::new()
            .with_prompt("Enter data directory location")
            .with_initial_text(
                dirs::home_dir()
                    .unwrap()
                    .join("Documents")
                    .join("keynergy")
                    .into_os_string()
                    .into_string()
                    .unwrap(),
            )
            .interact()
            .unwrap(),
    };
    let dir = Path::new(&dir);
    let layouts = dir.join("layouts");
    let metrics = dir.join("metrics");
    fs::create_dir_all(dir).unwrap();

    if !layouts.is_dir() {
        println!("[1/2] Downloading default layouts from git...");
        let repo = Repository::clone("https://github.com/keynergy/layouts", layouts)
            .expect("failed to clone repository");
        let refname = "origin/layouts";
        let (object, reference) = repo.revparse_ext(refname).expect("Object not found");
        repo.checkout_tree(&object, None)
            .expect("Failed to checkout");
        match reference {
            Some(gref) => repo.set_head(gref.name().unwrap()),
            None => repo.set_head_detached(object.id()),
        }
        .expect("Failed to set HEAD");
    }

    if !metrics.is_dir() {
        println!("[2/2] Downloading default metrics from git...");
        Repository::clone("https://github.com/keynergy/metrics", metrics)
            .expect("failed to clone repository");
    }

    println!("Saving config...");

    let config = Config {
        data_dir: dir.to_path_buf(),
        default_corpus: String::new(),
    };

    confy::store("keynergy", &config).unwrap();
    println!("Done setting up!");
    config
}
