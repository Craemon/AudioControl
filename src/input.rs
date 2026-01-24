use std::io::{BufRead, ErrorKind};
pub struct SliderInput<R: BufRead> {
    reader: R,
}
impl<R: BufRead> SliderInput<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
    pub fn read(&mut self) -> Result<Option<Vec<f32>>, std::io::Error> {
        let mut line = String::new();

        match self.reader.read_line(&mut line) {
            Ok(0) => Ok(None),
            Ok(_) => {
                let values: Vec<f32> = line
                    .trim()
                    .split('|')
                    .filter_map(|s| s.parse::<u16>().ok())
                    .map(|v| v as f32 / 1023.0)
                    .collect();

                if values.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(values))
                }
            }
            Err(e) if e.kind() == ErrorKind::TimedOut => Ok(None),
            Err(e) if e.kind() == ErrorKind::InvalidData => Ok(None),
            Err(e) => Err(e),
        }
    }
}