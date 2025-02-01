use std::{
    fs::{self},
    path::PathBuf
};

use crate::types::{NemoFile, NemoPoint};
use crate::{
    date::DateTime,
    error::{NemoError, NemoErrorKind},
};

pub fn get_nemo_file_path(path: &String) -> Result<PathBuf, NemoError> {
    let is_file = fs::exists(path);
    match is_file {
        Ok(t) => {
            if t {
                Ok(PathBuf::from(path))
            } else {
                Err(NemoError::new("File not found").kind(NemoErrorKind::NotFound))
            }
        }
        Err(_) => {
            Err(NemoError::new("Insufficent permissions").kind(NemoErrorKind::Permissions))
        }
    }
}
pub fn read_dir(dir_path: String, name: String) -> Result<Vec<PathBuf>, NemoError> {
    let paths = match fs::read_dir(dir_path) {
        Ok(paths) => paths,
        Err(_) => return Err(NemoError::new("Couldn't read path")),
    };
    let mut mnemo_files: Vec<PathBuf> = vec![];
    for p in paths {
        let path = match p {
            Ok(path) => path.path(),
            Err(_) => return Err(NemoError::new("Couldn't open DirEntry")),
        };
        
        let fname = path.as_path().to_str().unwrap();
        if path.is_file() && fname.contains(name.as_str()) {
            mnemo_files.push(path);
        }
    }

    Ok(mnemo_files)
}
pub fn process_nemo_dir(paths: Vec<PathBuf>) -> Result<Vec<NemoFile>, NemoError> {
    let mut vec: Vec<NemoFile> = vec![];
    for path in paths {
        let file = process_nemo_data(path)?;
        vec.push(file);
    }
    if vec.is_empty() {
        return Err(NemoError::new("Didn't process any files"));
    }
    Ok(vec)
}
pub fn process_nemo_data(path: PathBuf) -> Result<NemoFile, NemoError> {
    let new_path = path.clone();
    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .trim(csv::Trim::Fields)
        .from_path(new_path)
        .unwrap();
    // Instead of creating an iterator with the `records` method, we create
    // an iterator with the `deserialize` method.
    let mut nemo_file = NemoFile::default();

    // Sets filename    
    nemo_file.set_filename(path);

    for result in rdr.deserialize() {
        // We must tell Serde what type we want to deserialize into.
        let mut record: NemoPoint = match result {
            Ok(o) => o,
            Err(_) => return Err(NemoError::new("Error parsing the csv")),
        };

        // Parsing time
        let time = &record.time;
        record.parsed_time = Some(DateTime::from(time));
        nemo_file.add_point(record);
    }
    Ok(nemo_file)
}
