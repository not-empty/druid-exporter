
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Metrics {
    #[serde(default)]
    pub ignore: Vec<String>,
    #[serde(default)]
    pub allowed: Vec<String>,
}
