extern crate csv;
#[macro_use]
extern crate log;
#[macro_use]
extern crate syslog_ng_common;

use std::path::Path;
use std::fs::File;
use std::marker::PhantomData;
use std::io::{self, Read};

use syslog_ng_common::{Parser, Pipe, LogMessage, Error, ParserBuilder, GlobalConfig,
                       LogTemplate, LogTimeZone, MessageFormatter};

pub use syslog_ng_common::LogPipe;

mod lookup_table;
pub mod utils;

pub use lookup_table::LookupTable;

pub type CsvRecord = (String, String, String);

pub mod options {
    pub const SELECTOR: &'static str = "selector";
    pub const DATABASE: &'static str = "database";
    pub const DEFAULT_SELECTOR: &'static str = "default-selector";
    pub const PREFIX: &'static str = "prefix";
}

pub struct KVTaggerBuilder<P: Pipe> {
    records: Option<Vec<CsvRecord>>,
    selector_template: Option<LogTemplate>,
    default_selector: Option<String>,
    formatter: MessageFormatter,
    cfg: GlobalConfig,
    _marker: PhantomData<P>,
}

#[derive(Debug)]
pub enum LoadError {
    Io(io::Error),
    Csv(csv::Error),
}

impl From<io::Error> for LoadError {
    fn from(error: io::Error) -> LoadError {
        LoadError::Io(error)
    }
}

impl<P: Pipe> KVTaggerBuilder<P> {
    pub fn set_database<PATH: AsRef<Path>>(&mut self, path: PATH) -> Result<(), Error> {
        match KVTaggerBuilder::<P>::load_database::<PATH>(path) {
            Ok(records) => {
                self.records = Some(records);
                Ok(())
            }
            Err(error) => {
                let errmsg = format!("Error loading CSV file in kvtagger-rs: {:?}", error);
                Err(Error::verbatim_error(errmsg))
            }
        }
    }

    pub fn set_selector(&mut self, key: LogTemplate) {
        self.selector_template = Some(key);
    }

    pub fn set_prefix(&mut self, prefix: String) {
        self.formatter.set_prefix(prefix);
    }

    pub fn set_default_selector(&mut self, default_selector: String) {
        self.default_selector = Some(default_selector);
    }

    pub fn load_database<PATH: AsRef<Path>>(path: PATH) -> Result<Vec<CsvRecord>, LoadError> {
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
            formatter: self.formatter.clone(),
            default_selector: self.default_selector.clone(),
            records: self.records.clone(),
            selector_template: self.selector_template.clone(),
            _marker: PhantomData,
        }
    }
}

pub struct KVTagger {
    pub map: LookupTable,
    pub formatter: MessageFormatter,
    pub default_selector: Option<String>,
    pub selector_template: LogTemplate,
}

impl KVTagger {
    fn tag_msg_with_looked_up_key_value_pairs(formatter: &mut MessageFormatter,
                                              msg: &mut LogMessage,
                                              kvpairs: &[(String, String)]) {
        for kv in kvpairs {
            let (key, value) = formatter.format(kv.0.as_ref(), kv.1.as_ref());
            msg.insert::<&str>(key, value.as_bytes());
        }
    }
}

impl<P: Pipe> Parser<P> for KVTagger {
    fn parse(&mut self, _: &mut P, msg: &mut LogMessage, _: &str) -> bool {
        let selector = self.selector_template.format(msg, None, LogTimeZone::Local, 0);

        if let Ok(str_selector) = ::std::str::from_utf8(selector) {
            let looked_up_kvpairs = self.map.get(str_selector);

            match (looked_up_kvpairs, self.default_selector.as_ref()) {
                (Some(kv_pairs), _) => {
                    KVTagger::tag_msg_with_looked_up_key_value_pairs(&mut self.formatter,
                                                                     msg,
                                                                     kv_pairs);
                    true
                }
                (None, Some(default_selector)) => {
                    if let Some(kv_pairs) = self.map.get(default_selector) {
                        KVTagger::tag_msg_with_looked_up_key_value_pairs(&mut self.formatter,
                                                                         msg,
                                                                         kv_pairs);
                        true
                    } else {
                        true
                    }
                }
                _ => true,
            }
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
            default_selector: None,
            cfg: cfg,
            formatter: MessageFormatter::new(),
            _marker: PhantomData,
        }
    }
    fn option(&mut self, _name: String, _value: String) -> Result<(), Error> {
        match _name.as_ref() {
            options::DATABASE => {
                self.set_database(_value)
            }
            options::SELECTOR => {
                match LogTemplate::compile(&self.cfg, _value.as_bytes()) {
                    Ok(template) => {
                        self.set_selector(template);
                        Ok(())
                    }
                    Err(error) => {
                        let errmsg = format!("{:?}", error);
                        Err(Error::verbatim_error(errmsg))
                    }
                }
            }
            options::DEFAULT_SELECTOR => {
                self.set_default_selector(_value);
                Ok(())
            }
            options::PREFIX => {
                self.set_prefix(_value);
                Ok(())
            }
            _ => {
                let errmsg = format!("Unknown configuration option for kvtagger: {}", _name);
                Err(Error::unknown_option(errmsg))
            }
        }
    }
    fn build(self) -> Result<Self::Parser, Error> {
        let records = try!(self.records.ok_or(Error::missing_required_option(options::DATABASE)));
        let selector_template = try!(self.selector_template.ok_or(Error::missing_required_option(options::SELECTOR)));

        Ok(KVTagger {
            map: LookupTable::new(records),
            formatter: self.formatter,
            default_selector: self.default_selector,
            selector_template: selector_template,
        })
    }
}

parser_plugin!(KVTaggerBuilder<LogParser>);
