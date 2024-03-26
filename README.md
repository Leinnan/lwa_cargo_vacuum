# lwa_cargo_vacuum [![build](https://github.com/Leinnan/lwa_cargo_vacuum/actions/workflows/rust.yml/badge.svg)](https://github.com/Leinnan/lwa_cargo_vacuum/actions/workflows/rust.yml)
 [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://img.shields.io/crates/v/lwa_cargo_vacuum.svg)](https://crates.io/crates/lwa_cargo_vacuum)
[![crates.io](https://img.shields.io/crates/d/lwa_cargo_vacuum.svg)](https://crates.io/crates/lwa_cargo_vacuum)

Simple CLI tool for cleaning up old target folders.

```bash
Simple CLI tool for cleaning up old target folders

Usage: lwa_cargo_vacuum.exe [OPTIONS] [PATH]

Arguments:
  [PATH]
          Relative path to operate on

Options:
  -d, --depth <DEPTH>
          directories search depth

      --minimal-size <MINIMAL_SIZE>
          Minimal size in MB

      --time-since-edit <TIME_SINCE_EDIT>
          Minimal time since last edit in hours

  -r, --remove
          removes target dirs matching requirements

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Install

It can be installed using Rust Cargo:

```sh
cargo install lwa_cargo_vacuum
```