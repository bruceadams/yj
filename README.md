# YJ - YAML to JSON
[![Build Status](https://api.cirrus-ci.com/github/bruceadams/yj.svg)](https://cirrus-ci.com/github/bruceadams/yj)

Simple command line tool to convert a YAML input file into a JSON output file.

## How to, etc.

### Built in help

```bash
$ yj --help
yj 1.1.34

Command line tool that converts YAML to JSON

USAGE:
    yj [FLAGS] [OPTIONS] [INPUT]

ARGS:
    <INPUT>    Input YAML file name. Defaults to stdin

FLAGS:
    -c, --compact    Use compact formatting for the JSON output
    -h, --help       Print help information
    -j, --json       Parse the input as JSON. For most use cases, this option
                     makes no difference. Valid JSON is valid YAML, so JSON
                     input will (should?) parse correctly even when being
                     handled with the YAML parser. Use this option when you
                     want failure (instead of weird results) when the input is
                     invalid JSON
    -V, --version    Print version information
    -y, --yaml       Format the output as YAML instead of JSON

OPTIONS:
    -o, --output <OUTPUT>    Output file name for the JSON. Defaults to stdout
```

### Installing

Local build and install with `cargo`:

```bash
$ cargo install yj
```

Prebuilt binaries are available on
[Github releases](https://github.com/bruceadams/yj/releases)
for some common platforms.

On macOS, the prebuilt binary can be installed using
[Homebrew](https://brew.sh). Unfortunately, Homebrew picked up a
different utility with the name `yj` after I chose that name here.
So, a simple `brew install yj` gets that tool, not this one 😞.

```bash
$ brew tap bruceadams/utilities
$ brew install bruceadams/utilities/yj
```

Minimal Docker images are available on
[Docker Hub](https://cloud.docker.com/repository/docker/bruceadams/yj):

```bash
$ docker pull bruceadams/yj
```

### Example runs

```bash
$ cat .travis.yml
language: rust
os:
  - linux
  - osx
  - windows
rust:
  - stable
  - beta
  - nightly
matrix:
  allow_failures:
    - rust: nightly
  fast_finish: true
$ yj .travis.yml
{
  "language": "rust",
  "os": [
    "linux",
    "osx",
    "windows"
  ],
  "rust": [
    "stable",
    "beta",
    "nightly"
  ],
  "matrix": {
    "allow_failures": [
      {
        "rust": "nightly"
      }
    ],
    "fast_finish": true
  }
}
$ echo pi: 3.1415926 | yj
{
  "pi": 3.1415926
}
$ echo pi: 3.1415926 | yj -c
{"pi":3.1415926}$
```

## Build

Build it your self with Rust 2018, which needs a recent installation of Rust.
Get Rust installed from https://rustup.rs/.

```bash
cargo build
```
