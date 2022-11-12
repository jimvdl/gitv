use serde::Deserialize;
use std::{process::Command, fmt::Display, borrow::Cow};

fn main() {
    let (version, hash) = walk_git_log();
    tag(&version, &hash);
}

fn walk_git_log<'a>() -> (Version, Hash<'a>) {
    let output = Command::new("git")
        .args(["log", "--format=%H", "HEAD", "Cargo.toml"])
        .output()
        .unwrap();

    let output = String::from_utf8_lossy(&output.stdout);
    let hashes: Vec<&str> = output.split_whitespace().collect();

    let mut last_hash = "";
    let mut v = Version::default();
    for hash in hashes {
        let output = Command::new("git")
            .args(["show", &format!("{}:Cargo.toml", hash)])
            .output()
            .unwrap();

        let output = String::from_utf8_lossy(&output.stdout);
        let toml: CargoToml = toml::from_str(&output).unwrap();
        let crate_version = Version(Some(toml.package.version));

        if v == crate_version {
            last_hash = hash;
            continue;
        }
        
        if v.is_unset() {
            v = crate_version;
            last_hash = hash;
            continue;
        }

        tag(&v, &Hash(Cow::Borrowed(last_hash)));
        last_hash = hash;
        v = crate_version;
    }

    (v, Hash(Cow::Owned(last_hash.to_owned())))
}

fn tag(tag: &Version, hash: &Hash) {
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
        .args(["tag", "-a", "-m", &format!("Release {}", tag), tag.as_ref(), hash.as_ref()])
        .status()
        .unwrap();

    if !status.success() {
        panic!("git tag failed");
    }

    println!("{} {}", hash, tag);
}

#[derive(Debug, Deserialize)]
struct CargoToml {
    package: Package,
}

#[derive(Debug, Deserialize)]
struct Package {
    version: String,
}

#[derive(Default, PartialEq)]
struct Version(Option<String>);

impl Version {
    pub fn is_unset(&self) -> bool {
        self.0.is_none()
    }
}

impl AsRef<str> for Version {
    fn as_ref(&self) -> &str {
        match &self.0 {
            Some(s) => &s,
            None => ""
        }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(v) => f.write_fmt(format_args!("v{}", v)),
            None => f.write_str("v0.0.0")
        }
    }
}

struct Hash<'a>(Cow<'a, str>);

impl<'a> AsRef<str> for Hash<'a> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<'a> PartialEq<&Hash<'a>> for &str {
    fn eq(&self, other: &&Hash) -> bool {
        **self == *other.0
    }
}

impl<'a> Display for Hash<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}