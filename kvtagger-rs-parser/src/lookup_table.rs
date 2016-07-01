use {KVPair, CsvRecord};
use std::collections::HashMap;

#[derive(Clone)]
pub struct LookupTable {
    big_fuckin_vector: Vec<KVPair>,
    map: HashMap<String, DbRecord>
}

#[derive(Clone)]
pub struct DbRecord {
    offset: usize,
    length: usize
}

pub type Iter<'a> = ::std::iter::Take<::std::iter::Skip<::std::slice::Iter<'a, KVPair>>>;

impl LookupTable {
    pub fn new(mut records: Vec<CsvRecord>) -> LookupTable {
        records.sort();

        let mut table = HashMap::new();
        let mut big_fuckin_vector = Vec::new();

        for record in records.into_iter().enumerate() {
            let (index, (lookup_key, macro_name, macro_value)) = record;

            let mut entry = table.entry(lookup_key).or_insert_with(|| DbRecord {offset: index, length: 0});
            entry.length += 1;

            let kvpair = KVPair { key: macro_name, value: macro_value };
            big_fuckin_vector.push(kvpair);
        }

        LookupTable {
            big_fuckin_vector: big_fuckin_vector,
            map: table
        }
    }

    pub fn get(&self, key: &str) -> Option<Iter> {
        self.map.get(key).map(|db_record| {
            self.big_fuckin_vector.iter().skip(db_record.offset).take(db_record.length)
        })
    }
}


#[cfg(test)]
mod tests {
    use KVPair;
    use super::Iter;
    use super::LookupTable;
    use utils::make_expected_value_for_test_file;
    use utils::kv;

    fn iter_to_vec(iter: Iter) -> Vec<KVPair> {
        iter.map(|v| v.clone()).collect()
    }

    fn assert_vec_eq(mut a: Vec<KVPair>, mut b: Vec<KVPair>) {
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

        assert_vec_eq(iter_to_vec(table.get("key1").unwrap()), key_1_expected);
        assert_vec_eq(iter_to_vec(table.get("key2").unwrap()), key_2_expected);
        assert_vec_eq(iter_to_vec(table.get("key3").unwrap()), key_3_expected);

    }

    #[test]
    fn test_lookup_table_returns_none_on_getting_a_non_existing_key() {
        let table = LookupTable::new(Vec::new());

        assert_eq!(table.get("non_existing_key").is_none(), true);
    }
}
