use CsvRecord;
use std::collections::HashMap;

#[derive(Clone)]
pub struct LookupTable {
    map: HashMap<String, Vec<(String, String)>>
}

impl LookupTable {
    pub fn new(records: Vec<CsvRecord>) -> LookupTable {
        let mut table = HashMap::new();

        for record in records {
            let (lookup_key, macro_name, macro_value) = record;

            let mut entry = table.entry(lookup_key).or_insert_with(Vec::new);
            entry.push((macro_name, macro_value));
        }

        LookupTable {
            map: table
        }
    }

    pub fn get(&self, key: &str) -> Option<&Vec<(String, String)>> {
        self.map.get(key)
    }
}


#[cfg(test)]
mod tests {
    use super::LookupTable;
    use utils::make_expected_value_for_test_file;
    use utils::kv;

    fn assert_vec_eq(mut a: Vec<(String, String)>, mut b: Vec<(String, String)>) {
        a.sort();
        b.sort();

        assert_eq!(a, b);
    }

    #[test]
    fn test_ookup_table_can_be_built_from_vector_of_records() {
        let records = make_expected_value_for_test_file();
        let table = LookupTable::new(records);

        let key_1_expected = vec![
        kv("name1","value1"),
        kv("name2","value2"),
        kv("name3","value3"),
        kv("name4","value4"),
        kv("name5","value5"),
        kv("name6","value6"),
        kv("name7","value7"),
        kv("name8","value8"),
        kv("name9","value9"),
        kv("name10","value10"),
        kv("name11","value11"),
        kv("name12","value12"),
        ];

        let key_2_expected = vec![
        kv("name13","value13"),
        kv("name14","value14"),
        kv("name15","value15"),
        kv("name16","value16"),
        kv("name17","value17")
        ];

        let key_3_expected = vec![
        kv("name18","value18"),
        kv("name19","value19"),
        kv("name20","value20")
        ];

        assert_vec_eq(table.get("key1").unwrap().clone(), key_1_expected);
        assert_vec_eq(table.get("key2").unwrap().clone(), key_2_expected);
        assert_vec_eq(table.get("key3").unwrap().clone(), key_3_expected);
    }

    #[test]
    fn test_lookup_table_returns_none_on_getting_a_non_existing_key() {
        let table = LookupTable::new(Vec::new());

        assert_eq!(table.get("non_existing_key").is_none(), true);
    }
}
