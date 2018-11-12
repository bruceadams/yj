use failure::Error;
use std::{
    fs::File,
    io::{stdin, stdout, Write},
    path::PathBuf,
};
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
        Some(filename) => serde_yaml::from_reader(File::open(&filename)?)?,
        None => serde_yaml::from_reader(stdin())?,
    };

    let mut out: Box<Write> = match args.output {
        Some(filename) => Box::new(File::create(filename)?),
        None => Box::new(stdout()),
    };

    if args.compact {
        serde_json::to_writer(out, &data)?;
    } else {
        serde_json::to_writer_pretty(out.as_mut(), &data)?;
        out.write(b"\n")?;
    };

    Ok(())
}
