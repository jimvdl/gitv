mod version;
mod hash;

use serde::Deserialize;
use std::{process::Command, borrow::Cow, io};
use version::Version;
use hash::Hash;

#[derive(Debug, Deserialize)]
struct CargoToml {
    package: Package,
}

#[derive(Debug, Deserialize)]
struct Package {
    version: String,
}

fn main() -> io::Result<()> {
    let (version, hash) = walk_git_log()?;
    tag(&version, &hash)
}

fn walk_git_log<'a>() -> io::Result<(Version, Hash<'a>)> {
    let output = Command::new("git")
        .args(["log", "--format=%H", "HEAD", "Cargo.toml"])
        .output()?;

    let output = String::from_utf8_lossy(&output.stdout);
    let hashes: Vec<&str> = output.split_whitespace().collect();

    let mut last_hash = Cow::Borrowed("");
    let mut v = Version::default();
    for hash in hashes {
        let output = Command::new("git")
            .args(["show", &format!("{}:Cargo.toml", hash)])
            .output()?;

        let output = String::from_utf8_lossy(&output.stdout);
        let toml: CargoToml = toml::from_str(&output).unwrap();
        let crate_version = Version::from_string(toml.package.version);

        if v == crate_version {
            last_hash = Cow::Borrowed(hash);
            continue;
        }
        
        if v.is_unset() {
            v = crate_version;
            last_hash = Cow::Borrowed(hash);
            continue;
        }

        tag(&v, &Hash::from_borrow(last_hash))?;
        last_hash = Cow::Borrowed(hash);
        v = crate_version;
    }

    Ok((v, Hash::from_string(last_hash.into_owned())))
    // the below somehow doesn't compile, can't reproduce in a mre
    // (v, last_hash.into_owned())
    //
    // pub fn into_owned(self) -> Self {
    //     Self(Cow::Owned(self.0.into_owned()))
    // }
}

fn tag(tag: &Version, hash: &Hash) -> io::Result<()> {
    let output = Command::new("git")
        .args(["rev-parse", &format!("{}^{{}}", tag)])
        .output()?;

    let output = String::from_utf8_lossy(&output.stdout);
    if let Some(existing) = output.split_whitespace().next() {
        if existing == hash {
            println!("{} {} (already tagged)", hash, tag);
            return Ok(());
        }
    }

    let status = Command::new("git")
        .args([
            "tag", 
            "-a", 
            "-m", 
            &format!("Release {}", tag), 
            tag.as_ref(), 
            hash.as_ref()
        ])
        .status()?;

    if !status.success() {
        panic!("git tag failed");
    }

    println!("{} {}", hash, tag);
    Ok(())
}