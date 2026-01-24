mod config;
mod mixer;
mod backend;
mod input;

use std::io::BufReader;
use std::time::Duration;
use std::fs;
use config::Config;
use crate::backend::AudioBackend;
use crate::input::SliderInput;
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

    let mut backend = AudioBackend::start()?;
    let reader = BufReader::new(port);
    let mut input = SliderInput::new(reader);
    let mut mixer = Mixer::new(50,0.3);

    //run
    loop {
        if let Some(values) = input.read()? {
            for slider in &config.slider {
                if let Some(raw) = values.get(slider.id) {
                    if let Some(applied) = mixer.update(slider.id, *raw) {
                        backend.set_volume(&slider.target, applied)?;
                    }
                    println!("{} -> {:.3}", slider.target, raw);
                }
            }
        }
    }
}