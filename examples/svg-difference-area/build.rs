use std::{fs, path::Path};

fn download(url: &str, filename: &str) {
    let path = format!("src/{filename}");
    let dest = Path::new(&path);
    let content = reqwest::blocking::get(url)
        .expect(&format!("Failed to request '{filename}'"))
        .text()
        .expect("Failed to read the body of the request");

    fs::write(dest, content).expect(&format!("Failed to write the content in '{:?}'", dest));
}

fn main() {
    download(
        "https://raw.githubusercontent.com/zonination/weather-us/refs/heads/master/nyc.csv",
        "nyc.csv",
    );
    download(
        "https://raw.githubusercontent.com/zonination/weather-us/refs/heads/master/sanfrancisco.csv",
        "sanfrancisco.csv",
    );

    println!("cargo:rerun-if-changed=build.rs");
}
