use crate::date::DateTime;
use serde::Deserialize;
use std::{path::PathBuf, str::FromStr};

const HEADER: &str = "Section Name,TypeShot,Length(m),Depth IN(m),Depth OUT(m),Heading IN(dd),Heading OUT(dd),Pitch IN(dd),Pitch OUT(dd),Left(m),Right(m),Up(m),Down(m),Temperature(°C),Time,Marker";
#[derive(Default, Debug, Deserialize)]
#[allow(dead_code)]

pub struct Dimensions {
    #[serde(rename = "Left(m)")]
    pub left: f32,
    #[serde(rename = "Right(m)")]
    pub right: f32,
    #[serde(rename = "Up(m)")]
    pub up: f32,
    #[serde(rename = "Down(m)")]
    pub down: f32,
}
pub trait Print {
    fn print(&self);
}
#[derive(Default, Debug, Deserialize)]
/// Depth in meters, 0 = in, 1 = out
pub struct Depth {
    #[serde(rename = "Depth IN(m)")]
    _in: f32,
    #[serde(rename = "Depth OUT(m)")]
    _out: f32,
}
#[derive(Default, Debug, Deserialize)]
/// Heading in degrees, 0 = in, 1 = out
pub struct Heading {
    #[serde(rename = "Heading IN(dd)")]
    _in: f32,
    #[serde(rename = "Heading OUT(dd)")]
    _out: f32,
}
#[derive(Default, Debug, Deserialize)]
/// Pitch in degrees
pub struct Pitch {
    #[serde(rename = "Pitch IN(dd)")]
    _in: f32,
    #[serde(rename = "Pitch OUT(dd)")]
    _out: f32,
}
#[derive(Default, Debug, Deserialize)]
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
#[derive(Debug)]
#[allow(dead_code)]
pub struct NemoFile {
    header: &'static str,
    pub filename: String,
    pub points: Vec<NemoPoint>,
}
#[derive(Default, Debug, PartialEq, Eq, Deserialize)]
pub enum NemoTypeShot {
    STD,
    EOC,
    #[default]
    UND,
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
impl FromStr for NemoTypeShot {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "STD" => Ok(NemoTypeShot::STD),
            "EOC" => Ok(NemoTypeShot::EOC),
            _ => Err(()),
        }
    }
}
impl NemoFile {
    pub fn add_point(&mut self, point: NemoPoint) {
        self.points.push(point);
    }
    pub fn from_vec(&mut self, vec: Vec<NemoPoint>) {
        self.points = vec;
    }
    pub fn as_ref(&self) -> &Self {
        self
    }
    pub fn set_filename(&mut self, name: PathBuf){
        let name = name.file_name().unwrap().to_str().unwrap().strip_suffix(".csv").unwrap().to_string();
        self.filename = name;
    } 
}
impl Default for NemoFile {
    fn default() -> Self {
        Self {
            header: HEADER,
            filename: "".to_string(),
            points: vec![],
        }
    }
}
