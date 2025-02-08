use crate::date::DateTime;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
mod nemopoint;
mod nemofile;

const HEADER: &str = "Section Name,TypeShot,Length(m),Depth IN(m),Depth OUT(m),Heading IN(dd),Heading OUT(dd),Pitch IN(dd),Pitch OUT(dd),Left(m),Right(m),Up(m),Down(m),Temperature(°C),Time,Marker";
#[derive(Default, Debug, Deserialize, Serialize, Clone)]
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
#[derive(Default, Debug, Deserialize, Serialize, Clone)]
/// Depth in meters, 0 = in, 1 = out
pub struct Depth {
    #[serde(rename = "Depth IN(m)")]
    pub _in: f32,
    #[serde(rename = "Depth OUT(m)")]
    pub _out: f32,
}
#[derive(Default, Debug, Deserialize, Serialize, Clone)]
/// Heading in degrees, 0 = in, 1 = out
pub struct Heading {
    #[serde(rename = "Heading IN(dd)")]
    pub _in: f32,
    #[serde(rename = "Heading OUT(dd)")]
    pub _out: f32,
}
#[derive(Default, Debug, Deserialize, Serialize, Clone)]
/// Pitch in degrees
pub struct Pitch {
    #[serde(rename = "Pitch IN(dd)")]
    _in: f32,
    #[serde(rename = "Pitch OUT(dd)")]
    _out: f32,
}
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
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct NemoFile {
    pub(super) header: &'static str,
    pub filename: String,
    pub points: Vec<NemoPoint>,
}
#[derive(Default, Debug, PartialEq, Eq, Deserialize, Serialize, Clone)]
pub enum NemoTypeShot {
    STD,
    EOC,
    #[default]
    UND,
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

