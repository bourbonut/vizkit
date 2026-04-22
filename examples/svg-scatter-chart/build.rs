use std::{fs, path::Path};

fn main() {
    let url = "https://raw.githubusercontent.com/flother/rio2016/refs/heads/master/athletes.csv";

    let dest = Path::new("src/athletes.csv");
    if !dest.exists() {
        let content = reqwest::blocking::get(url)
            .expect("Failed to request 'athletes.json'")
            .text()
            .expect("Failed to read the body of the request");

        fs::write(dest, content).expect("Failed to write the content in 'src/athletes.json'");
    }

    println!("cargo:rerun-if-changed=build.rs");
}
