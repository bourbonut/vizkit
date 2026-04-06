use std::{fs, path::Path};

fn main() {
    let url = "https://gist.githubusercontent.com/eidietrich/0047db2bfcfae1543ff37c70474587d3/raw/51bcb25225d5517c40fc8328645973183ed140e6/trimmed-for-vis.json";

    let dest = Path::new("src/trimmed-for-vis.json");
    if !dest.exists() {
        let content = reqwest::blocking::get(url)
            .expect("Failed to request 'trimmed-for-vis.json'")
            .text()
            .expect("Failed to read the body of the request");

        fs::write(dest, content)
            .expect("Failed to write the content in 'src/trimmed-for-vis.json'");
    }

    println!("cargo:rerun-if-changed=build.rs");
}
