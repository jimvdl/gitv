//! Automatically sorts through the commit history of a crate and tags each
//! semver bump with that version.
//! 
//! # Usage
//! 
//! Download the latest release and place it somewhere where your `PATH`
//! variable can find it and simply run
//! ```shell
//! gitv
//! ```
//! 
//! Alternatively, clone this repository and run it in the root directory of any
//! crate with git history.
//! ```
//! cargo r --release
//! ```
//! 
//! ## How does it work?
//! 
//! Walks back the git log of Cargo.toml by first executing `git log --format=%H
//! HEAD Cargo.toml` to get all git commit hashes where Cargo.toml was modified
//! (`%H` outputs only hashes). It then iterates the commit hashes while running
//! `git show $hash:Cargo.toml` for every hash, where `$hash` is the current
//! [`struct@Hash`]. It then tries to parse the version field of the toml file
//! and wraps it in a [`Version`]. Some versioning checks determine if the
//! current commit needs to be tagged and if it finds that commit, tag it. 
//! 
//! Output: prints every commit has, the tag and if it has been previously
//! tagged. 
//! 
//! `gitv` tags as an example:
//! 
//! | Commit Hash                              | Tag                     |
//! | -----------------------------------------|-------------------------|
//! | 9d0fae2bcbb9621c17b711716f7bbc9adcff31a1 | v0.4.1                  |
//! | f36d7da9da4a2dfb1a841eccbbc237cc63e9a5b5 | v0.4.0 (already tagged) |
//! | 47f8945c7eaa250f7c00712e0aee0153b2d4f8eb | v0.3.3 (already tagged) |
//! | 687bd92d1abd491c2c3f113faefa782391d147fe | v0.3.2 (already tagged) |
//! | 9b7e86ff1f94c282f4cc861a15130f824d9e2cdb | v0.3.1 (already tagged) |
//! | d2b91b5d5107f8382b06a02754ddca35ea69a1d9 | v0.3.0 (already tagged) |
//! | 1cc4d2dc1227a7a1e109a9d4f3478d8432d34973 | v0.2.2 (already tagged) |
//! | 1758f90132b99223fd77cb9129f9e9e438a3ee5e | v0.2.1 (already tagged) |
//! | bcca18f8540a8594cd34224ec84d30ae68afc9c4 | v0.2.0 (already tagged) |
//! | 2c58087a74babd0d8b9fbe34cd702ffc115a95f8 | v0.1.0 (already tagged) |
//! 
//! Rust port of [jonhoo]'s [`tag-from-cargo-toml.sh`] script.
//! 
//! # Disclaimer
//! 
//! `gitv` is for personal use and not intended for wide-spread adoption.
//! Executing this program likely results in undefined behaviour if used
//! improperly, as it has not yet been rigorously tested. Use at your own risk!
//! 
//! [Hash]:(hash::Hash)
//! [Version]:(version::Version)
//! [jonhoo]:(https://github.com/jonhoo)
//! [`tag-from-cargo-toml.sh`]:(https://github.com/jonhoo/configs/blob/master/bins/bin/tag-from-cargo-toml.sh)
mod version;
mod hash;

use serde::Deserialize;
use std::{process::{Command, self}, borrow::Cow, io::{self, Write}};
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
    if !std::path::Path::new("./Cargo.toml").exists() {
        let _ = writeln!(io::stderr(), "directory is not a rust crate");
        process::exit(1);
    }

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
    // the below somehow doesn't compile, can't reproduce in a mre (v,
    // last_hash.into_owned())
    //
    // pub fn into_owned(self) -> Self { Self(Cow::Owned(self.0.into_owned())) }
}

fn tag(version: &Version, hash: &Hash) -> io::Result<()> {
    if version.is_unset() {
        let _ = writeln!(io::stderr(), "crate has no commit history to walk back");
        process::exit(1);
    }

    let output = Command::new("git")
        .args(["rev-parse", &format!("{}^{{}}", version)])
        .output()?;

    let output = String::from_utf8_lossy(&output.stdout);
    if let Some(existing) = output.split_whitespace().next() {
        if existing == hash {
            println!("{} {} (already tagged)", hash, version);
            return Ok(());
        }
    }

    let status = Command::new("git")
        .args([
            "tag", 
            "-a", 
            "-m", 
            &format!("Release {}", version), 
            version.as_ref(), 
            hash.as_ref()
        ])
        .status()?;

    if !status.success() {
        panic!("git tag failed");
    }

    println!("{} {}", hash, version);
    Ok(())
}