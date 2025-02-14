mod date;
mod error;
pub mod types;

#[cfg(feature = "debug")]
pub mod debug;
#[cfg(not(feature = "debug"))]
mod debug;

#[cfg(feature="web")]
pub use self::process_nemo_data;

use date::DateTime;
use error::NemoError;
use std::{
    fmt::Display, fs, path::{Path, PathBuf}, rc::Rc
};
use types::{NemoFile, NemoPoint};

#[derive(Debug, Clone)]
pub struct NemoReaderOptions {
    filter: String,
}
#[derive(Debug, Default, Clone)]
pub struct NemoReader {
    pub options: NemoReaderOptions,
    pub files: Vec<Rc<NemoFile>>,
}
impl Default for NemoReaderOptions {
    fn default() -> Self {
        Self { filter: "MNEMO".into() }
    }
}
impl NemoReaderOptions {
    pub fn new() -> Self {
        Self::default()
    }
}

impl NemoReader {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn load<T: ToString>(&self, path: T) -> Result<Self, NemoError> {
        let options = self.options.clone();
        let path_buf = PathBuf::from(path.to_string());
        let mut vec_files: Vec<Rc<NemoFile>> = vec![];

        if path_buf.is_file() {
            self.load_from_file(path_buf, &mut vec_files)?;
        } else if path_buf.is_dir() {
            self.load_from_dir(path_buf, &mut vec_files)?;
        }

        Ok(Self {
            files: vec_files,
            options,
        })
    }
    fn load_contents(&self, path: &PathBuf) -> Result<String, NemoError> {
        match fs::read_to_string(path) {
            Ok(v) => Ok(v),
            Err(_) => Err(NemoError::new("Couldn't open file")),
        }
    }
    fn get_filename(&self, p: &Path) -> String {
        let s = p.file_name().unwrap().to_string_lossy().into_owned();
        s
    }
    fn load_from_file(&self, path_buf: PathBuf, vec_files: &mut Vec<Rc<NemoFile>>) -> Result<(), NemoError>{
        if !path_buf.is_file() { return Err(NemoError::new("Tried to open file when it's not a file"))}
        
        let filename = self.get_filename(&path_buf);
        let filter = self.options.filter.as_str();
        if filename.contains(filter){
            let contents = self.load_contents(&path_buf)?;
            let file = Rc::new(
                process_nemo_data(contents, filename)?);
            vec_files.push(file);
        }
        Ok(())
    }
    fn load_from_dir(&self, path_buf: PathBuf, vec_files: &mut Vec<Rc<NemoFile>>) -> Result<(), NemoError> {
        if !path_buf.is_dir() { return Err(NemoError::new("Tried to open file when it's not a file"))}

        let read_dir = match path_buf.read_dir() {
            Ok(d) => d,
            Err(_) => return Err(NemoError::new("Can't read directory"))
        };
        for file in read_dir {
            let file = match file {
                Ok(f) => f,
                Err(_) => return Err(NemoError::new("Can't process DirEntry"))
            };
            self.load_from_file(file.path(), vec_files)?;
        }
        Ok(())
    }
    pub fn options(&mut self, options: &NemoReaderOptions) -> Self {
        Self {
            options: options.clone(),
            files: self.files.clone(),
        }
    }
    pub fn get_files(self) -> Vec<Rc<NemoFile>> {
        self.files
    }

}
impl NemoReaderOptions {
    pub fn filter(self, filter: impl ToString) -> Self
    {
        let fil = filter.to_string();
        Self {
            filter: fil
        }
    }
    
}
impl Display for NemoReaderOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Filter: {}", self.filter)?;
        Ok(())
    }
}
impl Display for NemoReader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Options: {}", self.options)?;
        write!(f, "\nFiles: ")?;
        for file in &self.files {
            writeln!(f,"\t{}", file.filename)?;
        }
        Ok(())
    }
}

fn process_nemo_data(contents: String, filename: String) -> Result<NemoFile, NemoError> {
    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .trim(csv::Trim::Fields)
        .from_reader(contents.as_bytes());
    // Instead of creating an iterator with the `records` method, we create
    // an iterator with the `deserialize` method.
    // let mut nemo_file = NemoFile::default();
    let mut vec_of_points: Vec<NemoPoint> = vec![];

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
