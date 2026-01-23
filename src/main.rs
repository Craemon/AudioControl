use std::io::{BufRead, BufReader, ErrorKind};
use std::time::Duration;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Audio Control!");
    let port_name = "/dev/ttyUSB0";
    let baud_rate:u32 = 9600;

    let port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(1000))
        .open()?;

    let mut reader = BufReader::new(port);

    loop {
        let mut line = String::new();

        match reader.read_line(&mut line) {
            Ok(0) => {} //no data
            Ok(_) => {
                //got data, try parsing
                let line = line.trim();
                if line.is_empty() {continue;}

                println!("Received: {}", line);
                let values: Vec<u16> = line
                    .split('|')
                    .filter_map(|s| s.parse::<u16>().ok())
                    .collect();

                if values.is_empty() {continue;}

                for (idx, val) in values.iter().enumerate() {
                    println!("Slider {} = {}", idx, val);
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
