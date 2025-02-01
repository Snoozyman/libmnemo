mod date;
mod error;
mod types;
mod utils;
#[cfg(feature="debug")]
pub mod test;
#[cfg(not(feature="debug"))]
mod test;

use error::NemoError;
use types::NemoFile;

pub fn read_nemo_file(path: String) -> Result<NemoFile, NemoError> {
    let pbuf = utils::get_nemo_file_path(&path)?;
    utils::process_nemo_data(pbuf)
}
pub fn read_nemo_dir(path: String, name: String) -> Result<Vec<NemoFile>, NemoError> {
    let vec_of_buf = utils::read_dir(path, name)?;
    let vec_files = utils::process_nemo_dir(vec_of_buf)?;
    Ok(vec_files)
}
