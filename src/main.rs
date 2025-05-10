use atty::Stream;
use clap::{CommandFactory, Parser};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

//NOTE: struct to capture required args
#[derive(Parser)]
#[command(name = "snorkel")]
#[command(about = "EMA algorithm for exponential moving point average", long_about = None)]
struct CliArgs {
    /// Alpha smoothing factor (0 < alpha <= 1)
    #[arg(long)]
    alpha: Option<f64>,

    /// Number of samples to simulate window size (used if alpha is not set)
    #[arg(long, default_value_t = 9)]
    window_size: usize,
    /// Input file (defaults to stdin)
    #[arg(short, long)]
    input: Option<String>,
    /// Output file (defaults to stdout)
    #[arg(short, long)]
    output: Option<String>,
    /// Timestamp present in the file
    #[arg(long)]
    timestamped: bool,
}

fn main() -> io::Result<()> {
    //NOTE: EMA algorithm: exponential moving average
    let args = CliArgs::parse();

    // NOTE: show the options for snorkel
    if std::env::args().len() == 1 && atty::is(Stream::Stdin) {
        CliArgs::command().print_help().unwrap();
        println!();
        return Ok(());
    }

    //NOTE: compute alpha
    let alpha = args
        .alpha
        .unwrap_or_else(|| 2.0 / (args.window_size as f64 + 1.0));

    // NOTE: setup input
    let reader: Box<dyn BufRead> = match &args.input {
        Some(path) => Box::new(BufReader::new(File::open(path)?)),
        None => Box::new(BufReader::new(io::stdin())),
    };

    // NOTE: setup output
    let mut writer: Box<dyn Write> = match &args.output {
        Some(path) => Box::new(File::create(path)?),
        None => Box::new(io::stdout()),
    };

    let mut ema: Option<f64> = None;

    for line in reader.lines() {
        let line = line?;

        if args.timestamped {
            // NOTE: if the file contains a timestamp
            if let Some((timestamp, val_str)) = line.trim().split_once(',') {
                if let Ok(value) = val_str.trim().parse::<f64>() {
                    ema = Some(match ema {
                        Some(prev) => alpha * value + (1.0 - alpha) * prev,
                        None => value,
                    });
                    writeln!(writer, "{timestamp}, {}", ema.unwrap())?;
                }
            }
        } else {
            // NOTE: parse the simple numeric input
            if let Ok(value) = line.trim().parse::<f64>() {
                ema = Some(match ema {
                    Some(prev) => alpha * value + (1.0 - alpha) * prev,
                    None => value,
                });
                writeln!(writer, "{}", ema.unwrap())?;
            }
        }
    }
    Ok(())
}
