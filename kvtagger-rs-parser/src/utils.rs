use syslog_ng_common::{Pipe, ParserBuilder, GlobalConfig};

pub fn make_expected_value_for_test_file() -> Vec<(String, String, String)> {
    [("key1","name1","value1"),
    ("key1","name11","value11"),
    ("key2","name17","value17"),
    ("key2","name14","value14"),
    ("key1","name2","value2"),
    ("key1","name5","value5"),
    ("key2","name15","value15"),
    ("key3","name19","value19"),
    ("key1","name7","value7"),
    ("key3","name20","value20"),
    ("key2","name16","value16"),
    ("key1","name4","value4"),
    ("key1","name3","value3"),
    ("key3","name18","value18"),
    ("key1","name10","value10"),
    ("key1","name6","value6"),
    ("key1","name9","value9"),
    ("key2","name13","value13"),
    ("key1","name8","value8"),
    ("key1","name12","value12")]
    .iter()
    .map(|&(lookup_key, macro_name, macro_value)| {
        (lookup_key.to_string(), macro_name.to_string(), macro_value.to_string())
    }).collect()
}

pub fn kv(key: &str, v: &str) -> (String, String) {
    (key.to_string(), v.to_string())
}

pub fn build_parser<P, PB>(cfg: GlobalConfig, options: &[(&str, &str)]) -> PB
    where P: Pipe, PB: ParserBuilder<P> {
    let mut builder = PB::new(cfg);

    for option in options {
        builder.option(option.0.to_string(), option.1.to_string());
    }

    builder
}
