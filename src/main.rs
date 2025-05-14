// src/main.rs
mod commands;

use clap::{Parser, Subcommand};
use commands::{handle_cycles, handle_ema, handle_zscore};

#[derive(Parser, Debug)]
#[command(name = "snorkel")]
#[command(about = "system monitoring cli tool", long_about = None)]
struct SnorkleArgs {
    #[command(subcommand)]
    command: SnorkleCommand,
}

#[derive(Subcommand, Debug)]
enum SnorkleCommand {
    /// Exponential moving average smoothing algorithm
    Ema {
        #[arg(long)]
        alpha: Option<f64>,
        #[arg(long, default_value_t = 9)]
        window_size: usize,
        #[arg(short, long)]
        input: Option<String>,
        #[arg(short, long)]
        output: Option<String>,
        #[arg(long, default_value_t = false)]
        timestamp: bool,
    },
    /// Algorithm for detecting anomalies using z-score in a rolling window
    Zscore {
        #[arg(long, default_value_t = 3.0)]
        threshold: f64,
        #[arg(long, default_value_t = 20)]
        window_size: usize,
        #[arg(short, long)]
        input: Option<String>,
        #[arg(short, long)]
        output: Option<String>,
        #[arg(long, default_value_t = false)]
        timestamp: bool,
    },
    /// cycle detection in a directed or undirected graph
    Cycles {
        #[arg(short, long)]
        input: Option<String>,
        #[arg(long, default_value_t = true)]
        directed: bool,
    },
}

fn main() -> std::io::Result<()> {
    // NOTE: Parse arguements
    let args = SnorkleArgs::parse();

    // NOTE: execute program logic
    match args.command {
        SnorkleCommand::Ema {
            alpha,
            window_size,
            input,
            output,
            timestamp,
        } => handle_ema(alpha, window_size, input, output, timestamp),
        SnorkleCommand::Zscore {
            threshold,
            window_size,
            input,
            output,
            timestamp,
        } => handle_zscore(threshold, window_size, input, output, timestamp),
        SnorkleCommand::Cycles { input, directed } => handle_cycles(input, directed),
    }
}
