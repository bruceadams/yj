use failure::{Error, ResultExt};
use std::{
    fs::File,
    io::{stdin, stdout, Read, Write},
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

    let in_file: Box<Read> = match args.input {
        Some(filename) => Box::new(
            File::open(&filename).context(format!("Failed to open input file {:?}", filename))?,
        ),
        None => Box::new(stdin()),
    };

    let mut out_file: Box<Write> = match args.output {
        Some(filename) => Box::new(
            File::create(&filename)
                .context(format!("Failed to create output file {:?}", filename))?,
        ),
        None => Box::new(stdout()),
    };

    let data: serde_yaml::Value =
        serde_yaml::from_reader(in_file).context("Failed to parse input YAML file")?;

    // Failure to format JSON output should never happen.
    if args.compact {
        serde_json::to_writer(out_file, &data).context("Failed to format output as JSON")?;
    } else {
        serde_json::to_writer_pretty(out_file.as_mut(), &data)
            .context("Failed to format output as JSON")?;
        // We'd like to write out a final newline. Ignore any failure to do so.
        let _result = out_file.write(b"\n");
    };

    Ok(())
}
