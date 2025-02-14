use super::{NemoPoint,HEADER};
use std::path::Path;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct NemoFile {
    pub header: &'static str,
    pub filename: String,
    pub path: String,
    pub points: Vec<NemoPoint>,
}
impl NemoFile {
    pub fn add_point(&mut self, point: NemoPoint) {
        self.points.push(point);
    }
    pub fn from_vec(&mut self, vec: Vec<NemoPoint>) {
        self.points = vec;
    }
    pub fn set_filename(&mut self, name: &Path) {
        let name = name
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .strip_suffix(".csv")
            .unwrap()
            .to_string();
        self.filename = name;
    }
}
impl Default for NemoFile {
    fn default() -> Self {
        Self {
            header: HEADER,
            filename: "".to_string(),
            points: vec![],
            path: "".to_string()
        }
    }
}