extern crate csv;
#[macro_use]
extern crate log;
#[macro_use]
extern crate syslog_ng_common;

use std::path::Path;
use std::fs::File;
use std::marker::PhantomData;
use std::io::{self, Read};

use syslog_ng_common::{Parser, Pipe, LogMessage, OptionError, ParserBuilder, GlobalConfig, LogTemplate, LogTimeZone};

pub use syslog_ng_common::LogPipe;

mod lookup_table;
pub mod utils;

pub use lookup_table::LookupTable;

pub type CsvRecord = (String, String, String);

pub struct KVTaggerBuilder<P: Pipe> {
    records: Option<Vec<CsvRecord>>,
    selector_template: Option<LogTemplate>,
    cfg: GlobalConfig,
    _marker: PhantomData<P>
}

#[derive(Debug)]
pub enum LoadError {
    Io(io::Error),
    Csv(csv::Error)
}

impl From<io::Error> for LoadError {
    fn from(error: io::Error) -> LoadError {
        LoadError::Io(error)
    }
}

impl<P: Pipe> KVTaggerBuilder<P> {
    pub fn set_csv_file<PATH: AsRef<Path>>(&mut self, path: PATH) {
        match KVTaggerBuilder::<P>::load_csv_file::<PATH>(path) {
            Ok(records) => {
                self.records = Some(records);
            },
            Err(error) => {
                error!("Error loading CSV file in kvtagger-rs: {:?}", error);
                self.records = None;
            }
        }
    }

    pub fn set_lookup_key(&mut self, key: LogTemplate) {
        self.selector_template = Some(key);
    }

    pub fn load_csv_file<PATH: AsRef<Path>>(path: PATH) -> Result<Vec<CsvRecord>, LoadError> {
        let mut file = try!(File::open(path));
        let mut contents = String::new();

        let _ = file.read_to_string(&mut contents);

        let mut csv_reader = csv::Reader::from_string(contents).has_headers(false);

        csv_reader.decode().collect::<csv::Result<Vec<CsvRecord>>>().map_err(LoadError::Csv)
    }

    pub fn records(&self) -> Option<&Vec<CsvRecord>> {
        self.records.as_ref()
    }
}

impl<P: Pipe> Clone for KVTaggerBuilder<P> {
    fn clone(&self) -> Self {
        KVTaggerBuilder {
            cfg: self.cfg.clone(),
            records: self.records.clone(),
            selector_template: self.selector_template.clone(),
            _marker: PhantomData
        }
    }
}

pub struct KVTagger {
    pub map: LookupTable,
    pub selector_template: LogTemplate
}

impl<P: Pipe> Parser<P> for KVTagger {
    fn parse(&mut self, _: &mut P, msg: &mut LogMessage, _: &str) -> bool {
        let selector = self.selector_template.format(msg, None, LogTimeZone::Local, 0);

        if let Ok(str_selector) = ::std::str::from_utf8(selector) {
            if let Some(kv_pairs) = self.map.get(str_selector) {
                for kv in kv_pairs {
                    msg.insert::<&str>(kv.0.as_ref(), kv.1.as_ref());
                }
            }
            true
        } else {
            false
        }

    }
}

impl<P: Pipe> ParserBuilder<P> for KVTaggerBuilder<P> {
    type Parser = KVTagger;

    fn new(cfg: GlobalConfig) -> Self {
        KVTaggerBuilder {
            records: None,
            selector_template: None,
            cfg: cfg,
            _marker: PhantomData
        }
    }
    fn option(&mut self, _name: String, _value: String) {
        match _name.as_ref() {
            "csv-file" => {
                self.set_csv_file(_value);
            },
            "lookup-key" => {
                match LogTemplate::compile(&self.cfg, _value.as_bytes()) {
                    Ok(template) => {
                        self.set_lookup_key(template);
                    },
                    Err(error) => {
                        error!("{:?}", error);
                        self.selector_template = None;
                    }
                }
            },
            _ => {
                debug!("Unknown configuration option for kvtagger: {}", _name);
            }
        }
    }
    fn build(self) -> Result<Self::Parser, OptionError> {
        match (self.records, self.selector_template) {
            (Some(records), Some(selector_template)) => {
                let parser = KVTagger {
                    map: LookupTable::new(records),
                    selector_template: selector_template
                };

                return Ok(parser);
            },
            (Some(_), None) => {
                error!("Failed to intialize kvtagger-rs: csv-file() was not specified");
                return Err(OptionError::missing_required_option("csv-file"));
            },
            (None, Some(_)) => {
                error!("Failed to intialize kvtagger-rs: lookup-key() was not specified");
                return Err(OptionError::missing_required_option("lookup-key"));
            },
            (None, None) => {
                error!("Failed to intialize kvtagger-rs: neither lookup-key() or csv-file() was not specified");
                return Err(OptionError::missing_required_option("lookup-key & csv-file"));
            }
        }
    }
}

parser_plugin!(KVTaggerBuilder<LogParser>);
