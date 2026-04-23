mod config;
mod mixer;
mod backend;
mod input;

use std::io::BufReader;
use std::time::Duration;
use std::fs;
use std::path::PathBuf;
use config::Config;
use crate::backend::AudioBackend;
use crate::input::SliderInput;
use crate::mixer::Mixer;
const DEFAULT_CONFIG: &str = "/usr/local/lib/audiocontrol/config/default.toml";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //load  and validate config
    let config_str = load_config()?;
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
                        for target in &slider.targets {
                            backend.set_volume(target, applied)?;
                        }
                    }
                    println!("{} -> {:.3} ({})", slider.id, raw, slider.targets.join(", "));
                }
            }
        }
    }
}

fn config_path() -> PathBuf {
    dirs::home_dir()
        .unwrap()
        .join(".config/audiocontrol/config.toml")
}

fn load_config() -> Result<String, Box<dyn std::error::Error>> {
    let user_config = config_path();

    if !user_config.exists() {
        if let Some(parent) = user_config.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::copy(DEFAULT_CONFIG, &user_config)?;
        println!("Created config at {:?}", user_config);
    }

    Ok(fs::read_to_string(user_config)?)
}