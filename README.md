# gitv

Walks back the git log of Cargo.toml by first executing `git log --format=%H
HEAD Cargo.toml` to get all git commit hashes where Cargo.toml was modified
(`%H` outputs only hashes). It then iterates the commit hashes while running
`git show $hash:Cargo.toml` for every hash, where `$hash` is the current
[`Hash`](src/hash.rs). It then tries to parse the version field of the toml file
and wraps it in a [`Version`](src/version.rs). Some versioning checks determine
if the current commit needs to be tagged and if it finds that commit, tag it. 

Output: prints every commit has, the tag and if it has been previously tagged. 

`gitv` tags as an example:

| Commit Hash                              | Tag                     |
| -----------------------------------------|-------------------------|
| 9d0fae2bcbb9621c17b711716f7bbc9adcff31a1 | v0.4.1                  |
| f36d7da9da4a2dfb1a841eccbbc237cc63e9a5b5 | v0.4.0 (already tagged) |
| 47f8945c7eaa250f7c00712e0aee0153b2d4f8eb | v0.3.3 (already tagged) |
| 687bd92d1abd491c2c3f113faefa782391d147fe | v0.3.2 (already tagged) |
| 9b7e86ff1f94c282f4cc861a15130f824d9e2cdb | v0.3.1 (already tagged) |
| d2b91b5d5107f8382b06a02754ddca35ea69a1d9 | v0.3.0 (already tagged) |
| 1cc4d2dc1227a7a1e109a9d4f3478d8432d34973 | v0.2.2 (already tagged) |
| 1758f90132b99223fd77cb9129f9e9e438a3ee5e | v0.2.1 (already tagged) |
| bcca18f8540a8594cd34224ec84d30ae68afc9c4 | v0.2.0 (already tagged) |
| 2c58087a74babd0d8b9fbe34cd702ffc115a95f8 | v0.1.0 (already tagged) |

Rust port of [jonhoo](https://github.com/jonhoo)'s
[`tag-from-cargo-toml.sh`](https://github.com/jonhoo/configs/blob/master/bins/bin/tag-from-cargo-toml.sh)
script.

# Usage

Run the following command in the root directory of any crate with git history.
```
cargo r --release
```

# Disclaimer

`gitv` is for personal use and not intended for wide-spread adoption. Executing
this program likely results in undefined behaviour if used improperly, as it has
not yet been rigorously tested. Use at your own risk!