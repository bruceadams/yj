# YJ - YAML to JSON
[![Build Status](https://travis-ci.com/bruceadams/yj.svg?branch=master)](https://travis-ci.com/bruceadams/yj)

Simple command line tool to convert a YAML input file into a JSON output file.

## How to, etc.

### Built in help

```bash
$ yj --help
yj 0.4.0
Bruce Adams <bruce.adams@acm.org>
Read YAML, write JSON

USAGE:
    yj [FLAGS] [OPTIONS] [input]

FLAGS:
    -c, --compact    Use compact formatting for the JSON output.
    -h, --help       Prints help information
    -j, --json       Parse the input as JSON. For more use cases,
                     this option makes no difference. Valid JSON is
                     valid YAML, so JSON input will (should?) parse
                     correctly even when being handled with the
                     YAML parser. Use this option when you want
                     failure (instead of weird results) when the
                     input is invalid JSON.
    -V, --version    Prints version information
    -y, --yaml       Format the output as YAML instead of JSON.

OPTIONS:
    -o, --output <output>
            Output file name for the JSON. Defaults to stdout.


ARGS:
    <input>    Input YAML file name. Defaults to stdin.
```

### Releases

Grab binary builds from https://github.com/bruceadams/yj/releases

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
