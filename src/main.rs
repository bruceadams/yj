use failure::{Error, ResultExt};
use std::{
    fs::File,
    io::{stdin, stdout, Read, Write},
    path::PathBuf,
};
use structopt::{clap::AppSettings::ColoredHelp, StructOpt};

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

fn from_json(input: Box<dyn Read>, mut output: Box<dyn Write>, args: &MyArgs) -> Result<(), Error> {
    let data: serde_json::Value =
        serde_json::from_reader(input).context("Failed to parse input JSON file")?;

    // Failure to format JSON output should never happen.
    if args.yaml {
        serde_yaml::to_writer(output.as_mut(), &data).context("Failed to format output as YAML")?;
        // We'd like to write out a final newline. Ignore any failure to do so.
        let _result = output.write(b"\n");
    } else if args.compact {
        serde_json::to_writer(output, &data).context("Failed to format output as JSON")?;
    } else {
        serde_json::to_writer_pretty(output.as_mut(), &data)
            .context("Failed to format output as JSON")?;
        // We'd like to write out a final newline. Ignore any failure to do so.
        let _result = output.write(b"\n");
    };
    Ok(())
}

fn from_yaml(input: Box<dyn Read>, mut output: Box<dyn Write>, args: &MyArgs) -> Result<(), Error> {
    let data: serde_yaml::Value =
        serde_yaml::from_reader(input).context("Failed to parse input YAML file")?;

    // Failure to format JSON output should never happen.
    if args.yaml {
        serde_yaml::to_writer(output.as_mut(), &data).context("Failed to format output as YAML")?;
        // We'd like to write out a final newline. Ignore any failure to do so.
        let _result = output.write(b"\n");
    } else if args.compact {
        serde_json::to_writer(output, &data).context("Failed to format output as JSON")?;
    } else {
        serde_json::to_writer_pretty(output.as_mut(), &data)
            .context("Failed to format output as JSON")?;
        // We'd like to write out a final newline. Ignore any failure to do so.
        let _result = output.write(b"\n");
    };
    Ok(())
}

fn main() -> Result<(), Error> {
    let args = MyArgs::from_args();

    let input: Box<dyn Read> = match &args.input {
        Some(filename) => Box::new(
            File::open(&filename).context(format!("Failed to open input file {:?}", filename))?,
        ),
        None => Box::new(stdin()),
    };

    let output: Box<dyn Write> = match &args.output {
        Some(filename) => Box::new(
            File::create(&filename)
                .context(format!("Failed to create output file {:?}", filename))?,
        ),
        None => Box::new(stdout()),
    };

    if args.json {
        from_json(input, output, &args)?;
    } else {
        from_yaml(input, output, &args)?;
    }

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
