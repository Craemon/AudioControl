use std::process::{Command, Child, Stdio};
use std::io::{Write, Result};
use std::time::Duration;

const BACKEND: &str = if cfg!(debug_assertions) {
    "backend/audio.py"
} else {
    "/usr/local/lib/audiocontrol/backend/audio.py"
};
pub struct AudioBackend {
    child: Child,
}
impl AudioBackend {
    pub fn start() -> Result<Self> {
        let mut child = Command::new("python3")
            .arg(BACKEND)
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        std::thread::sleep(Duration::from_millis(100));

        if let Some(status) = child.try_wait()? {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Audio backend exited with status (code {})", status),
            ));
        }

        Ok(Self { child })
    }
    pub fn set_volume(&mut self, target: &str, volume: f32) -> Result<()> {
        if let Some(stdin) = self.child.stdin.as_mut() {
            writeln!(stdin, "SET {} {:.3}", target, volume)?;
        }
        Ok(())
    }
}