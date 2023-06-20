# Dump Rust dylib metadata

## Preerequisites

rustup override set nightly-2022-12-31
rustup component add rustc-dev rust-src llvm-tools-preview

## Usage

Example:

```console
$ cargo run <path>
Crate info:
name macros
hash dfdb2a2d1e486db2 stable_crate_id StableCrateId(17924924324798206262)
proc_macro true
=External Dependencies=
```console