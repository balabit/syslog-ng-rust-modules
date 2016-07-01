extern crate csv;
#[macro_use]
extern crate log;
#[macro_use]
extern crate syslog_ng_common;

use std::path::Path;
use std::fs::File;
use std::marker::PhantomData;
use std::io::{self, Read};

use syslog_ng_common::{Parser, Pipe, LogMessage, OptionError, ParserBuilder, GlobalConfig};

pub use syslog_ng_common::LogPipe;

mod lookup_table;
pub mod utils;

pub use lookup_table::LookupTable;

#[derive(Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub struct KVPair {
    key: String,
    value: String
}

pub type CsvRecord = (String, String, String);

pub struct KVTaggerBuilder<P: Pipe> {
    records: Option<Vec<CsvRecord>>,
    built_parser: Option<KVTagger>,
    lookup_key: Option<String>,
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

    pub fn set_lookup_key(&mut self, key: String) {
        self.lookup_key = Some(key);
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
            records: self.records.clone(),
            built_parser: self.built_parser.clone(),
            lookup_key: self.lookup_key.clone(),
            _marker: PhantomData
        }
    }
}

#[derive(Clone)]
pub struct KVTagger {
    pub map: LookupTable,
    pub lookup_key: String
}

impl<P: Pipe> From<KVTagger> for KVTaggerBuilder<P> {
    fn from(parser: KVTagger) -> Self {
        KVTaggerBuilder {
            records: None,
            built_parser: Some(parser),
            lookup_key: None,
            _marker: PhantomData
        }
    }
}

impl<P: Pipe> Parser<P> for KVTagger {
    fn parse(&mut self, _: &mut P, msg: &mut LogMessage, _: &str) -> bool {
        if let Some(kv_pairs) = self.map.get(&self.lookup_key) {
            for kv in kv_pairs {
                msg.insert::<&str>(kv.0.as_ref(), kv.1.as_ref());
            }
        }
        true
    }
}

impl<P: Pipe> ParserBuilder<P> for KVTaggerBuilder<P> {
    type Parser = KVTagger;

    fn new(_: GlobalConfig) -> Self {
        KVTaggerBuilder {
            records: None,
            built_parser: None,
            lookup_key: None,
            _marker: PhantomData
        }
    }
    fn option(&mut self, _name: String, _value: String) {
        match _name.as_ref() {
            "csv-file" => {
                self.set_csv_file(_value);
            },
            "lookup-key" => {
                self.set_lookup_key(_value);
            },
            _ => {
                debug!("Unknown configuration option for kvtagger: {}", _name);
            }
        }
    }
    fn build(self) -> Result<Self::Parser, OptionError> {
        if let Some(built_parser) = self.built_parser {
            return Ok(built_parser);
        }

        match (self.records, self.lookup_key) {
            (Some(records), Some(lookup_key)) => {
                let parser = KVTagger {
                    map: LookupTable::new(records),
                    lookup_key: lookup_key
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
