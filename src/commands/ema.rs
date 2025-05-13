use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

pub fn handle_ema(
    alpha: Option<f64>,
    window_size: usize,
    input: Option<String>,
    output: Option<String>,
    timestamp: bool,
) -> io::Result<()> {
    //NOTE: compute alpha
    let alpha = alpha.unwrap_or_else(|| 2.0 / (window_size as f64 + 1.0));

    // NOTE: setup input
    let reader: Box<dyn BufRead> = match &input {
        Some(path) => Box::new(BufReader::new(File::open(path)?)),
        None => Box::new(BufReader::new(io::stdin())),
    };

    // NOTE: setup output
    let mut writer: Box<dyn Write> = match &output {
        Some(path) => Box::new(File::create(path)?),
        None => Box::new(io::stdout()),
    };

    let mut ema: Option<f64> = None;

    for line in reader.lines() {
        let line = line?;

        if timestamp {
            // NOTE: if the file contains a timestamp
            if let Some((timestamp, val_str)) = line.trim().split_once(',') {
                if let Ok(value) = val_str.trim().parse::<f64>() {
                    ema = Some(match ema {
                        Some(prev) => alpha * value + (1.0 - alpha) * prev,
                        None => value,
                    });
                    writeln!(writer, "{timestamp}, {}", ema.unwrap())?;
                    writer.flush()?;
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
                writer.flush()?;
            }
        }
    }
    Ok(())
}
