# YJ - YAML to JSON

Simple command line tool to convert a YAML input file into a JSON output file.

## How to, etc.

### Built in help

```bash
$ yj --help
yj 0.3.0
Bruce Adams <bruce.adams@acm.org>
Read YAML, write JSON

USAGE:
    yj [FLAGS] [OPTIONS] [input]

FLAGS:
    -c, --compact    Use compact formatting for the JSON output.
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --output <output>    Output file name for the JSON. Defaults to stdout.

ARGS:
    <input>    Input YAML file name. Defaults to stdin.
```

### Releases

Grab binary builds from https://github.ibm.com/ba/yj/releases

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

Build it your self with Rust 2018, which currently requires the `beta` channel.
First, get Rust installed from https://rustup.rs/, then switch to `beta` for this project.

```bash
rustup override set beta
cargo build
```
