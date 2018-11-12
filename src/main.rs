use failure::Error;
use std::{fs::File, path::PathBuf};
use structopt::{clap::AppSettings::ColoredHelp, StructOpt};

/// Read YAML, write JSON
#[derive(Debug, StructOpt)]
#[structopt(raw(global_settings = "&[ColoredHelp]"))]
struct MyArgs {
    /// Use compact formatting for the JSON output.
    #[structopt(long = "compact", short = "c")]
    compact: bool,

    /// Output file name for the JSON. Defaults to stdout.
    #[structopt(long = "output", parse(from_os_str), short = "o")]
    output: Option<PathBuf>,

    /// Input YAML file name. Defaults to stdin.
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,
}

fn main() -> Result<(), Error> {
    let args = MyArgs::from_args();
    let data: serde_yaml::Value = match args.input {
        Some(input_filename) => serde_yaml::from_reader(File::open(&input_filename)?)?,
        None => serde_yaml::from_reader(std::io::stdin())?,
    };
    let out: Box<std::io::Write> = match args.output {
        Some(output_filename) => Box::new(File::create(output_filename)?),
        None => Box::new(std::io::stdout()),
    };

    if args.compact {
        serde_json::to_writer(out, &data)?
    } else {
        serde_json::to_writer_pretty(out, &data)?
    };
    Ok(())
}
