mod config;

use std::io::{BufRead, BufReader, ErrorKind};
use std::time::Duration;
use std::fs;
use config::Config;
use std::process::{Command, Stdio};
use std::io::Write;
use std::collections::HashMap;
use std::time::Instant;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    //load  and validate config
    let config_str = fs::read_to_string("config/default.toml")?;
    let config: Config = toml::from_str(&config_str)?;

    config.validate().map_err(|e| {
        eprintln!("Config error: {}", e);
        e
    })?;
    let port_name = &config.serial.port;
    let baud_rate:u32 = config.serial.baud;

    println!("Loaded {} sliders", config.slider.len());

    let port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(100))
        .open()?;

    let mut backend = Command::new("python3")
        .arg("backend/audio.py")
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    std::thread::sleep(Duration::from_millis(100));

    if let Some(status) = backend.try_wait()? {
        return Err(format!("Audio backend failed to start (exit code {})", status).into());
    }

    let backend_stdin = backend.stdin.as_mut().unwrap();

    let mut reader = BufReader::new(port);

    let mut last_sent_value: HashMap<usize, f32> = HashMap::new();
    let mut last_sent_time: HashMap<usize, Instant> = HashMap::new();
    //run
    loop {
        let mut line = String::new();

        match reader.read_line(&mut line) {
            Ok(0) => {}
            Ok(_) => {
                let line = line.trim();
                if line.is_empty() {continue;}

                let values: Vec<u16> = line
                    .split('|')
                    .filter_map(|s| s.parse::<u16>().ok())
                    .collect();

                if values.is_empty() {continue;}

                for slider in &config.slider {
                    if let Some (raw) = values.get(slider.id) {
                        let volume = *raw as f32 / 1023.0;

                        let now = Instant::now();
                        let should_send = match last_sent_time.get(&slider.id) {
                            Some(t) => t.elapsed().as_millis() >= 50, // 20 Hz
                            None => true,
                        };

                        if should_send {
                            let applied = match last_sent_value.get(&slider.id) {
                                Some(prev) => smooth(*prev, volume, 0.3),
                                None => volume,
                            };

                            let applied = applied.clamp(0.0, 1.0);

                            writeln!(
                                backend_stdin,
                                "SET {} {:.3}",
                                slider.target,
                                applied
                            )?;

                            last_sent_value.insert(slider.id, applied);
                            last_sent_time.insert(slider.id, now);
                        }
                        println!("{} -> {:.2}", slider.target, volume);
                    } else {
                        eprintln!("Warning: no value for slider id {} ({})", slider.id, slider.target);
                    }
                }
            }
            Err(e) if e.kind() == ErrorKind::TimedOut => {
                // timeout, ignore, continue
            }
            Err(e) if e.kind() == ErrorKind::InvalidData => {
                eprintln!("Warning: invalid data received, skipping line");
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
}

fn smooth(prev: f32, next: f32, factor: f32) -> f32 {
    prev + (next - prev) * factor
}