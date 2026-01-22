use std::io::{BufRead, BufReader};
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
        if reader.read_line(&mut line)? > 0 {
            println!("Received: {}", line.trim());
        }
    }
}
