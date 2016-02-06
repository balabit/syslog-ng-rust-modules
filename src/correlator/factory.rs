use std::io::Read;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;

use super::Correlator;
use super::Error;

pub struct CorrelatorFactory;

impl CorrelatorFactory {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Correlator, Error> {
        trace!("Trying to load contexts from file; path={}", path.as_ref().display());
        let mut file = try!(File::open(path));
        let mut buffer = String::new();
        try!(file.read_to_string(&mut buffer));
        Correlator::from_str(&buffer)
    }
}
