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
use serde_yaml;

use config::ContextConfig;
use config::compile_templates;
use ContextMap;
use super::Correlator;
use super::Error;
use Event;
use TemplateFactory;

pub struct CorrelatorFactory;

impl CorrelatorFactory {
    pub fn from_path<T, P, E, TF>(path: P, template_factory: &TF) -> Result<Correlator<T, E, TF::Template>, Error>
        where P: AsRef<Path>, E: Event, TF: TemplateFactory<E> {
        let contexts = try!(CorrelatorFactory::load_file(path));
        let contexts_after_template_compilation = try!(compile_templates(contexts, template_factory));
        Ok(Correlator::new(ContextMap::from_configs(contexts_after_template_compilation)))
    }

    pub fn load_file<P: AsRef<Path>>(path: P) -> Result<Vec<ContextConfig<String>>, Error> {
        match path.as_ref().extension() {
            Some(extension) => {
                match try!(extension.to_str().ok_or(Error::NotUtf8FileName)) {
                    "json" => {
                        let content = try!(CorrelatorFactory::read(&path));
                        serde_json::from_str::<Vec<ContextConfig<String>>>(&content).map_err(Error::SerdeJson)
                    },
                    "yaml" | "yml" | "YAML" | "YML" => {
                        let content = try!(CorrelatorFactory::read(&path));
                        serde_yaml::from_str::<Vec<ContextConfig<String>>>(&content).map_err(Error::SerdeYaml)
                    },
                    _ => Err(Error::UnsupportedFileExtension),
                }
            },
            None => {
                Err(Error::FileExtensionNotFound)
            }
        }

    }

    fn read<P: AsRef<Path>>(path: P) -> Result<String, Error> {
        trace!("Trying to load contexts from file; path={}", path.as_ref().display());
        let mut file = try!(File::open(path));
        let mut buffer = String::new();
        try!(file.read_to_string(&mut buffer));
        Ok(buffer)
    }

}
