use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Config {
    pub serial: SerialConfig,
    pub slider: Vec<SliderConfig>,
}
#[derive(Debug, Deserialize)]
pub struct SerialConfig {
    pub port: String,
    pub baud: u32,
}
#[derive(Debug, Deserialize)]
pub struct SliderConfig {
    pub id: usize,
    pub target: String,
}
impl Config {
    pub fn validate(&self) -> Result<(), String> {
        let mut ids = self.slider.iter().map(|s| s.id).collect::<Vec<_>>();
        ids.sort();
        ids.dedup();

        if ids.len() != self.slider.len() {
            return Err("Number of sliders does not match number of serials".into());
        }

        if ids.first() != Some(&0) {
            return Err("Slider IDs must start at 0".into());
        }

        Ok(())
    }
}