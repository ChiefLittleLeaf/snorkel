use core::f64;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

pub fn handle_zscore(
    threshold: f64,
    window_size: usize,
    input: Option<String>,
    output: Option<String>,
    timestamp: bool,
) -> io::Result<()> {
    // NOTE: Set input
    let reader: Box<dyn BufRead> = match &input {
        Some(path) => Box::new(BufReader::new(File::open(path)?)),
        None => Box::new(BufReader::new(io::stdin())),
    };
    // NOTE: Set output
    let mut writer: Box<dyn Write> = match &output {
        Some(path) => Box::new(File::create(path)?),
        None => Box::new(io::stdout()),
    };
    // NOTE: Set window_size
    let mut window: VecDeque<f64> = VecDeque::with_capacity(window_size);

    for line in reader.lines() {
        let line = line?;

        let (timestamp, value) = if timestamp {
            match line.trim().split_once(',') {
                Some((ts, val_str)) => match val_str.trim().parse::<f64>() {
                    Ok(v) => (Some(ts.to_string()), v),
                    Err(_) => continue,
                },
                None => continue,
            }
        } else {
            match line.trim().parse::<f64>() {
                Ok(v) => (None, v),
                Err(_) => continue,
            }
        };

        if window.len() == window_size {
            window.pop_front();
        }
        window.push_back(value);

        let mean = window.iter().copied().sum::<f64>() / window.len() as f64;
        let std_dev =
            (window.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / window.len() as f64).sqrt();
        let z = if std_dev != 0.0 {
            (value - mean) / std_dev
        } else {
            0.0
        };
        let is_anomaly = z.abs() > threshold;

        match timestamp {
            Some(ts) => {
                writeln!(writer, "{ts},{},z={},anomaly={}", value, z, is_anomaly)?;
                writer.flush()?;
            }
            None => {
                writeln!(writer, "{},z={},anomaly={}", value, z, is_anomaly)?;
                writer.flush()?;
            }
        }
    }
    Ok(())
}
