mod config;
mod mixer;

use std::io::{BufRead, BufReader, ErrorKind};
use std::time::Duration;
use std::fs;
use config::Config;
use std::process::{Command, Stdio};
use std::io::Write;
use crate::mixer::Mixer;

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

    let mut mixer = Mixer::new(50,0.3);

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

                        if let Some(applied) = mixer.update(slider.id, volume) {
                            writeln!(backend_stdin, "SET {} {:.3}", slider.target, applied)?;
                        }
                        println!("{} -> {:.3}", slider.target, volume);
                    } else {
                        eprintln!("Warning: no value for slider id {} ({})", slider.id, slider.target);
                    }
                }
            }
            Err(e) if e.kind() == ErrorKind::TimedOut => {}
            Err(e) if e.kind() == ErrorKind::InvalidData => {
                eprintln!("Warning: invalid data received, skipping line");
            }
            Err(e) => return Err(e.into()),
        }
    }
}