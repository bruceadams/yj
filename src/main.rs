use exitfailure::ExitFailure;
use snafu::{ResultExt, Snafu};
use std::{
    fs::File,
    io::{stdin, stdout, Read, Write},
    path::PathBuf,
};
use structopt::{clap::AppSettings::ColoredHelp, StructOpt};

struct Input {
    handle: Box<dyn Read>,
    name: String,
}

struct Output {
    handle: Box<dyn Write>,
    name: String,
}

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Failed to parse JSON from {}: {}", name, source))]
    ReadJSON {
        name: String,
        source: serde_json::error::Error,
    },
    #[snafu(display("Failed to parse YAML from {}: {}", name, source))]
    ReadYAML {
        name: String,
        source: serde_yaml::Error,
    },
    #[snafu(display("Failed to format JSON from {}: {}", name, source))]
    WriteJSON {
        name: String,
        source: serde_json::error::Error,
    },
    #[snafu(display("Failed to format YAML from {}: {}", name, source))]
    WriteYAML {
        name: String,
        source: serde_yaml::Error,
    },
    #[snafu(display("Failed to open input file \"{}\": {}", filename.display(), source))]
    OpenInput {
        source: std::io::Error,
        filename: PathBuf,
    },
    #[snafu(display("Failed to create output file \"{}\": {}", filename.display(), source))]
    OpenOutput {
        source: std::io::Error,
        filename: PathBuf,
    },
}

/// Read YAML, write JSON
#[derive(Debug, StructOpt)]
#[structopt(global_settings(&[ColoredHelp]))]
struct MyArgs {
    /// Use compact formatting for the JSON output.
    #[structopt(long = "compact", short = "c")]
    compact: bool,

    /// Format the output as YAML instead of JSON.
    #[structopt(long = "yaml", short = "y")]
    yaml: bool,

    /// Parse the input as JSON.
    /// For most use cases, this option makes no difference.
    /// Valid JSON is valid YAML, so JSON input will (should?) parse
    /// correctly even when being handled with the YAML parser.
    /// Use this option when you want failure (instead of weird results)
    /// when the input is invalid JSON.
    #[structopt(long = "json", short = "j")]
    json: bool,

    /// Output file name for the JSON. Defaults to stdout.
    #[structopt(long = "output", parse(from_os_str), short = "o")]
    output: Option<PathBuf>,

    /// Input YAML file name. Defaults to stdin.
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,
}

fn from_json(input: Input, mut output: Output, args: &MyArgs) -> Result<(), Error> {
    let data: serde_json::Value =
        serde_json::from_reader(input.handle).context(ReadJSON { name: input.name })?;

    // Failure to format JSON output should never happen.
    if args.yaml {
        serde_yaml::to_writer(output.handle.as_mut(), &data)
            .context(WriteYAML { name: output.name })?;
        // We'd like to write out a final newline. Ignore any failure to do so.
        let _result = output.handle.write(b"\n");
    } else if args.compact {
        serde_json::to_writer(output.handle, &data).context(WriteJSON { name: output.name })?;
    } else {
        serde_json::to_writer_pretty(output.handle.as_mut(), &data)
            .context(WriteJSON { name: output.name })?;
        // We'd like to write out a final newline. Ignore any failure to do so.
        let _result = output.handle.write(b"\n");
    };
    Ok(())
}

fn from_yaml(input: Input, mut output: Output, args: &MyArgs) -> Result<(), Error> {
    let data: serde_yaml::Value =
        serde_yaml::from_reader(input.handle).context(ReadYAML { name: input.name })?;

    // Failure to format JSON output should never happen.
    if args.yaml {
        serde_yaml::to_writer(output.handle.as_mut(), &data)
            .context(WriteYAML { name: output.name })?;
        // We'd like to write out a final newline. Ignore any failure to do so.
        let _result = output.handle.write(b"\n");
    } else if args.compact {
        serde_json::to_writer(output.handle, &data).context(WriteJSON { name: output.name })?;
    } else {
        serde_json::to_writer_pretty(output.handle.as_mut(), &data)
            .context(WriteJSON { name: output.name })?;
        // We'd like to write out a final newline. Ignore any failure to do so.
        let _result = output.handle.write(b"\n");
    };
    Ok(())
}

fn dispatch(input: Input, output: Output, args: &MyArgs) -> Result<(), Error> {
    if args.json {
        from_json(input, output, &args)
    } else {
        from_yaml(input, output, &args)
    }
}

fn main() -> Result<(), ExitFailure> {
    let args = MyArgs::from_args();

    let input: Input = match &args.input {
        Some(filename) => Input {
            handle: Box::new(File::open(&filename).context(OpenInput { filename })?),
            name: filename.display().to_string(),
        },
        None => Input {
            handle: Box::new(stdin()),
            name: "<stdin>".to_string(),
        },
    };

    let output: Output = match &args.output {
        Some(filename) => Output {
            handle: Box::new(File::create(&filename).context(OpenOutput { filename })?),
            name: filename.display().to_string(),
        },
        None => Output {
            handle: Box::new(stdout()),
            name: "<stdout>".to_string(),
        },
    };

    dispatch(input, output, &args)?;

    Ok(())
}

// Copyright 2018 Bruce Adams
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
