use std::{fs, path::Path};

fn main() {
    let url = "https://raw.githubusercontent.com/BrookingsInstitution/MetroRecoveryIndex/refs/heads/master/BLS%20-%20Unemployment%20Rate.csv";

    let dest = Path::new("src/bls-unemployment-rate.csv");
    if !dest.exists() {
        let content = reqwest::blocking::get(url)
            .expect("Failed to request 'bls-unemployment-rate.csv'")
            .text()
            .expect("Failed to read the body of the request");

        fs::write(dest, content)
            .expect("Failed to write the content in 'src/bls-unemployment-rate.csv'");
    }

    println!("cargo:rerun-if-changed=build.rs");
}
