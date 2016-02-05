use std::collections::HashMap;

use config::ContextConfig;
use context::Context;

pub struct ContextMap {
    map: HashMap<String, Vec<usize>>,
    contexts: Vec<Context>,
}

impl ContextMap {
    pub fn new() -> ContextMap {
        ContextMap {
            map: HashMap::new(),
            contexts: Vec::new(),
        }
    }

    pub fn from_configs(configs: Vec<ContextConfig>) -> ContextMap {
        let mut context_map = ContextMap::new();
        for i in configs {
            context_map.insert(i.into());
        }
        context_map
    }

    pub fn insert(&mut self, context: Context) {
        self.contexts.push(context);
        let last_context = self.contexts
                               .last()
                               .expect("Failed to remove the last Context from a non empty vector");
        let index_of_last_context = self.contexts.len() - 1;
        let patterns = last_context.patterns();
        ContextMap::update_indices(&mut self.map, index_of_last_context, patterns);
    }

    fn update_indices(map: &mut HashMap<String, Vec<usize>>,
                      new_index: usize,
                      patterns: &[String]) {
        if patterns.is_empty() {
            ContextMap::add_index_to_every_index_vectors(map, new_index);
        } else {
            ContextMap::add_index_to_looked_up_index_vectors(map, new_index, patterns);
        }
    }

    fn add_index_to_every_index_vectors(map: &mut HashMap<String, Vec<usize>>, new_index: usize) {
        for (_, v) in map.iter_mut() {
            v.push(new_index);
        }
    }

    fn add_index_to_looked_up_index_vectors(map: &mut HashMap<String, Vec<usize>>,
                                            new_index: usize,
                                            patterns: &[String]) {
        for i in patterns {
            map.entry(i.clone()).or_insert_with(Vec::new).push(new_index);
        }
    }

    pub fn contexts_mut(&mut self) -> &mut Vec<Context> {
        &mut self.contexts
    }

    pub fn contexts_iter_mut(&mut self, key: &str) -> Iterator {
        let ids = self.map.get(key);
        Iterator {
            ids: ids,
            pos: 0,
            contexts: &mut self.contexts,
        }
    }
}

pub trait StreamingIterator {
    type Item;
    fn next(&mut self) -> Option<&mut Self::Item>;
}

pub struct Iterator<'a> {
    ids: Option<&'a Vec<usize>>,
    pos: usize,
    contexts: &'a mut Vec<Context>,
}

impl<'a> StreamingIterator for Iterator<'a> {
    type Item = Context;
    fn next(&mut self) -> Option<&mut Context> {
        if let Some(ids) = self.ids {
            if let Some(id) = ids.get(self.pos) {
                self.pos += 1;
                self.contexts.get_mut(*id)
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use conditions::ConditionsBuilder;
    use context::{Context, LinearContext, BaseContextBuilder};
    use uuid::Uuid;
    use std::time::Duration;

    fn assert_context_map_contains_uuid(context_map: &mut ContextMap, uuid: &Uuid, key: &str) {
        let mut iter = context_map.contexts_iter_mut(key);
        let context = iter.next().expect("Failed to get back an inserted context");
        if let Context::Linear(ref context) = *context {
            assert_eq!(uuid, context.uuid());
        } else {
            unreachable!();
        }
    }

    #[test]
    fn test_given_context_map_when_a_context_is_inserted_then_its_patters_are_inserted_to_the_map_with_its_id
        () {
        let mut context_map = ContextMap::new();
        let uuid = Uuid::new_v4();
        let context1 = {
            let conditions = {
                let patterns = vec!["A".to_owned(), "B".to_owned()];
                ConditionsBuilder::new(Duration::from_millis(100)).patterns(patterns).build()
            };
            let base = BaseContextBuilder::new(uuid.clone(), conditions).build();
            LinearContext::new(base)
        };
        context_map.insert(Context::Linear(context1));
        assert_eq!(context_map.contexts_mut().len(), 1);
        assert_context_map_contains_uuid(&mut context_map, &uuid, "A");
        assert_context_map_contains_uuid(&mut context_map, &uuid, "B");
    }
}
