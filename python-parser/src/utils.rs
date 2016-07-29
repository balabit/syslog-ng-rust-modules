use syslog_ng_common::{ParserBuilder, GlobalConfig};

use PythonParser;
use PythonParserBuilder;

pub fn build_parser_with_options(module_name: &str,
                                 class_name: &str,
                                 options: &[(&str, &str)])
                                 -> PythonParser {
    let cfg = GlobalConfig::new(0x0308);
    let mut builder = PythonParserBuilder::new(cfg);
    builder.option(::options::MODULE.to_owned(), module_name.to_owned()).ok().unwrap();
    builder.option(::options::CLASS.to_owned(), class_name.to_owned()).ok().unwrap();
    for &(k, v) in options {
        builder.option(k.to_owned(), v.to_owned()).ok().unwrap();
    }
    builder.build().unwrap()
}

pub fn build_parser(module_name: &str, class_name: &str) -> PythonParser {
    build_parser_with_options(module_name, class_name, &[])
}
