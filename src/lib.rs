mod date;
mod error;
pub mod types;

#[cfg(feature = "debug")]
pub mod debug;
#[cfg(not(feature = "debug"))]
mod debug;

use date::DateTime;
use error::NemoError;
use std::{
    fmt::Display, fs, ops::{Deref, DerefMut}, path::PathBuf, rc::Rc
};
use types::{NemoFile, NemoPoint};

#[derive(Debug, Clone, Copy)]
pub struct NemoReaderOptions {
    filter: &'static str,
}
#[derive(Debug, Default)]
pub struct NemoReader {
    options: NemoReaderOptions,
    pub files: Vec<Rc<NemoFile>>,
    paths: Vec<PathBuf>,
}
impl Default for NemoReaderOptions {
    fn default() -> Self {
        Self { filter: "MNEMO" }
    }
}

impl NemoReader {
    pub fn new(path: &str, options: NemoReaderOptions) -> Result<Self, NemoError> {
        let path_buf = PathBuf::from(path);
        let mut vec_paths: Vec<PathBuf> = vec![];
        let mut vec_files: Vec<Rc<NemoFile>> = vec![];
        vec_paths.push(path_buf.clone());
        if path_buf.is_file() {
            let file = Rc::new(process_nemo_data(&path_buf).unwrap());
            vec_files.push(file);
        } else if path_buf.is_dir() {
            let f = match fs::read_dir(path_buf) {
                Ok(p) => p,
                Err(_) => return Err(NemoError::new("Can't open dir")),
            };

            for file in f {
                let dir = match file {
                    Ok(p) => p.path(),
                    Err(_) => return Err(NemoError::new("Coulnd't parse DirEntry")),
                };
                let fname = dir.as_path().to_str().unwrap();
                if dir.is_file() && fname.contains(options.filter) {
                    let nm = Rc::new(process_nemo_data(&dir).unwrap());
                    vec_files.push(nm);
                }
            }
        }

        Ok(Self {
            files: vec_files,
            paths: vec_paths,
            options: NemoReaderOptions::default(),
        })
    }
    pub fn get_files(self) -> Vec<Rc<NemoFile>> {
        self.files
    }

}

impl Display for NemoReaderOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.filter).unwrap();
        Ok(())
    }
}
impl Display for NemoReader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Options: {}", self.options).unwrap();
        Ok(())
    }
}
/* pub struct CloneableReader(pub NemoReader);
impl Deref for CloneableReader {
    type Target = NemoReader;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for CloneableReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Clone for CloneableReader {
    fn clone(&self) -> Self {
        CloneableReader(NemoReader {
            options: self.options,
            files: self.files.clone(),
            paths: self.paths.clone(),
        })
    }
}
    */
fn process_nemo_data(path: &PathBuf) -> Result<NemoFile, NemoError> {
    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .trim(csv::Trim::Fields)
        .from_path(path)
        .unwrap();
    // Instead of creating an iterator with the `records` method, we create
    // an iterator with the `deserialize` method.
    // let mut nemo_file = NemoFile::default();
    let mut vec_of_points: Vec<NemoPoint> = vec![];
    // Sets filename
    let filename = path.to_str().unwrap().to_string();
    for result in rdr.deserialize() {
        // We must tell Serde what type we want to deserialize into.
        let mut record: NemoPoint = match result {
            Ok(o) => o,
            Err(_) => return Err(NemoError::new("Error parsing the csv")),
        };
        // Parsing time
        let time = &record.time;
        record.parsed_time = Some(DateTime::from(time));
        // if record.typeshot != NemoTypeShot::EOC {
            vec_of_points.push(record);
        // }
        
    }
    Ok(NemoFile {
        filename,
        points: vec_of_points,
        ..Default::default()
    })
}
#[cfg(feature="survex")]
fn to_survex(){}
