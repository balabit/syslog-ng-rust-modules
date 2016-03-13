use syslog_ng_common::ParserBuilder;

use PythonParser;
use PythonParserBuilder;

pub fn build_parser_with_options(module_name: &str, class_name: &str, options: &[(&str, &str)]) -> PythonParser {
    let mut builder = PythonParserBuilder::new();
    builder.option(::options::MODULE.to_owned(), module_name.to_owned());
    builder.option(::options::CLASS.to_owned(), class_name.to_owned());
    for &(k, v) in options {
        builder.option(k.to_owned(), v.to_owned());
    }
    builder.build().unwrap()
}

pub fn build_parser(module_name: &str, class_name: &str) -> PythonParser {
    build_parser_with_options(module_name, class_name, &[])
}
