use serde::Deserialize;
use std::process::Command;

#[derive(Debug, Deserialize)]
struct CargoToml {
    package: Package,
}

#[derive(Debug, Deserialize)]
struct Package {
    version: String,
}

fn main() {
    let mut last_hash = String::new();
    let mut v = String::from("0.0.0");

    walk(&mut last_hash, &mut v);

    tag(&v, &last_hash);
}

fn walk(last_hash: &mut String, v: &mut String) {
    let output = Command::new("git")
    .args(["log", "--format=%H", "HEAD", "Cargo.toml"])
    .output()
    .unwrap();

    let output = String::from_utf8_lossy(&output.stdout);

    let hashes: Vec<&str> = output.split_whitespace().collect();

    for hash in hashes {
        let output = Command::new("git")
            .args(["show", &format!("{}:Cargo.toml", hash)])
            .output()
            .unwrap();

        let output = String::from_utf8_lossy(&output.stdout);
        let toml: CargoToml = toml::from_str(&output).unwrap();

        if *v == toml.package.version {
            *last_hash = hash.to_owned();
            continue;
        }
        
        if v == "0.0.0" {
            *v = toml.package.version.clone();
            *last_hash = hash.to_owned();
            continue;
        }

        tag(&v, last_hash);
        *last_hash = hash.to_owned();
        *v = toml.package.version;
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
