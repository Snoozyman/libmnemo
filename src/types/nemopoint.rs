use super::{Dimensions, Depth, Heading, DateTime, NemoTypeShot, Pitch};
use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Deserialize, Serialize, Clone)]
#[allow(dead_code)]
pub struct NemoPoint {
    #[serde(rename = "Section Name")]
    pub name: String,
    #[serde(rename = "TypeShot")]
    pub typeshot: NemoTypeShot,
    #[serde(rename = "Length(m)")]
    pub length: f32,
    #[serde(flatten)]
    pub depth: Depth,
    #[serde(flatten)]
    pub heading: Heading,
    #[serde(flatten)]
    pub pitch: Pitch,
    #[serde(flatten)]
    pub dimensions: Dimensions,
    #[serde(rename = "Temperature(°C)")]
    pub temperature: f32,
    #[serde(rename = "Time")]
    pub time: String,
    #[serde(rename = "Marker")]
    marker: Option<String>,
    pub parsed_time: Option<DateTime>,
}
impl NemoPoint {
    pub fn new(name: String, t: &str) -> Self {
        Self {
            name,
            typeshot: t.parse().unwrap(),
            ..Default::default()
        }
    }
    pub fn set_dimensions(&mut self, d: Dimensions) {
        // set dimensions
        self.dimensions = d;
    }
    pub fn set_depth(&mut self, d: Depth) {
        // set depth
        self.depth = d;
    }
    pub fn set_heading(&mut self, h: Heading) {
        // set heading
        self.heading = h;
    }
    pub fn get_datetime(self) -> DateTime {
        DateTime::from(self.time)
    }
}