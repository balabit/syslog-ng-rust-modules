// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::io::Read;
use std::fs::File;
use std::path::Path;

use serde_json;

use config::ContextConfig;
use ContextMap;
use super::Correlator;
use super::Error;

pub struct CorrelatorFactory;

impl CorrelatorFactory {
    pub fn from_path<T, P: AsRef<Path>>(path: P) -> Result<Correlator<T>, Error> {
        trace!("Trying to load contexts from file; path={}", path.as_ref().display());
        let mut file = try!(File::open(path));
        let mut buffer = String::new();
        try!(file.read_to_string(&mut buffer));
        let contexts = try!(serde_json::from_str::<Vec<ContextConfig>>(&buffer));
        Ok(Correlator::new(ContextMap::from_configs(contexts)))
    }
}
