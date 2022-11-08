use clap::Parser;
use serde::Deserialize;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {}

#[derive(Debug, Deserialize)]
struct CargoToml {
    package: Package,
}

#[derive(Debug, Deserialize)]
struct Package {
    version: String,
}

fn main() {
    // let output = Command::new("git")
    //     .arg("tag")
    //     .output()
    //     .unwrap();

    // let output = String::from_utf8_lossy(&output.stdout);
    // let versions: Vec<&str> = output.split_whitespace().collect();

    // println!("{:?}", versions);

    // let output = Command::new("git")
    //     .args(vec!["rev-parse", "v0.4.0"])
    //     .output()
    //     .unwrap();

    // if !output.stderr.is_empty() {
    //     panic!("{}", String::from_utf8_lossy(&output.stderr));
    // }

    // let output = String::from_utf8_lossy(&output.stdout);
    // println!("output {}", output);

    let output = Command::new("git")
        .args(["log", "--format=%H", "HEAD", "Cargo.toml"])
        .output()
        .unwrap();

    let output = String::from_utf8_lossy(&output.stdout);

    let hashes: Vec<&str> = output.split_whitespace().collect();

    let mut last_hash = "";
    let mut v = String::from("0.0.0");
    for hash in hashes {
        let output = Command::new("git")
            .args(["show", &format!("{}:Cargo.toml", hash)])
            .output()
            .unwrap();

        let output = String::from_utf8_lossy(&output.stdout);
        let toml: CargoToml = toml::from_str(&output).unwrap();

        if v == toml.package.version {
            last_hash = hash;
            continue;
        }
        
        if v == "0.0.0" {
            v = toml.package.version.clone();
            last_hash = hash;
        }

        tag(&v, last_hash);

        last_hash = hash;
        v = toml.package.version;
    }
}

fn tag(v: &str, hash: &str) {
    let tag = format!("v{}", v);
    let output = Command::new("git")
        .args(["rev-parse", &format!("{}^{{}}", tag)])
        .output()
        .unwrap();

    let output = String::from_utf8_lossy(&output.stdout);
    let existing = output.split_whitespace().next().unwrap();
    if existing == hash {
        println!("{} {} (already tagged)", hash, tag);
        return;
    }

    let status = Command::new("git")
        .args(["tag", "-a", "-m", &format!("Release {}", tag), &tag, &hash])
        .status()
        .unwrap();

    if !status.success() {
        panic!("git tag failed");
    }

    println!("{} {}", hash, tag);
}
