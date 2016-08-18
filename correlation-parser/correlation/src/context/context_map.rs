// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::collections::HashMap;

use config::ContextConfig;
use context::Context;
use Event;
use Template;

pub struct ContextMap<E, T> where E: Event, T: Template<Event=E> {
    map: HashMap<Vec<u8>, Vec<usize>>,
    contexts: Vec<Context<E, T>>,
}

impl<E, T> Default for ContextMap<E, T> where E: Event, T: Template<Event=E> {
    fn default() -> ContextMap<E, T> {
        ContextMap {
            map: HashMap::default(),
            contexts: Vec::default()
        }
    }
}

impl<E, T> ContextMap<E, T> where E: Event, T: Template<Event=E> {
    pub fn new() -> ContextMap<E, T> {
        ContextMap::default()
    }

    pub fn from_configs(configs: Vec<ContextConfig<T>>) -> ContextMap<E, T>
        where T: Template<Event=E> {
        let mut context_map = ContextMap::new();
        for i in configs {
            context_map.insert(i.into());
        }
        context_map
    }

    pub fn insert(&mut self, context: Context<E, T>) {
        self.contexts.push(context);
        let last_context = self.contexts
                               .last()
                               .expect("Failed to remove the last Context from a non empty vector");
        let index_of_last_context = self.contexts.len() - 1;
        let patterns = last_context.patterns();
        ContextMap::<E, T>::update_indices(&mut self.map, index_of_last_context, patterns);
    }

    fn update_indices(map: &mut HashMap<Vec<u8>, Vec<usize>>,
                      new_index: usize,
                      patterns: &[String]) {
        if patterns.is_empty() {
            ContextMap::<E, T>::add_index_to_every_index_vectors(map, new_index);
        } else {
            ContextMap::<E, T>::add_index_to_looked_up_index_vectors(map, new_index, patterns);
        }
    }

    fn add_index_to_every_index_vectors(map: &mut HashMap<Vec<u8>, Vec<usize>>, new_index: usize) {
        for (_, v) in map.iter_mut() {
            v.push(new_index);
        }
    }

    fn add_index_to_looked_up_index_vectors(map: &mut HashMap<Vec<u8>, Vec<usize>>,
                                            new_index: usize,
                                            patterns: &[String]) {
        for i in patterns {
            map.entry(i.as_bytes().to_vec()).or_insert_with(Vec::new).push(new_index);
        }
    }

    pub fn contexts_mut(&mut self) -> &mut Vec<Context<E, T>> {
        &mut self.contexts
    }

    pub fn contexts_iter_mut<'a, I: ::std::iter::Iterator<Item=&'a [u8]>>(&mut self, keys: I) -> Iterator<E, T> {
        let mut index_vector = Vec::new();
        for index_list in keys.filter_map(|key| self.map.get(key)) {
            index_vector.extend_from_slice(index_list);
        }

        index_vector.sort();
        index_vector.dedup();

        Iterator {
            indices: index_vector,
            pos: 0,
            contexts: &mut self.contexts
        }
    }
}

pub trait StreamingIterator {
    type Item;
    fn next(&mut self) -> Option<&mut Self::Item>;
}

pub struct Iterator<'a, E, T> where E: 'a + Event, T: 'a + Template<Event=E> {
    indices: Vec<usize>,
    pos: usize,
    contexts: &'a mut Vec<Context<E, T>>
}

impl<'a, E, T> StreamingIterator for Iterator<'a, E, T> where E: Event, T: Template<Event=E> {
    type Item = Context<E, T>;
    fn next(&mut self) -> Option<&mut Context<E, T>> {
        if let Some(index) = self.indices.get(self.pos) {
            self.pos += 1;
            self.contexts.get_mut(*index)
        }
        else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use conditions::ConditionsBuilder;
    use context::{Context, LinearContext};
    use uuid::Uuid;
    use std::time::Duration;
    use Message;
    use test_utils::{MockTemplate, BaseContextBuilder};

    fn assert_context_map_contains_uuid<'a, I>(context_map: &mut ContextMap<Message, MockTemplate>, uuid: &Uuid, keys: I)
        where I: ::std::iter::Iterator<Item=&'a [u8]>
    {
        let mut iter = context_map.contexts_iter_mut(keys);
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
        let mut context_map = ContextMap::<Message, MockTemplate>::new();
        let uuid = Uuid::new_v4();
        let context1 = {
            let conditions = {
                ConditionsBuilder::new(Duration::from_millis(100)).build()
            };
            let patterns = vec!["A".to_owned(), "B".to_owned()];
            let base = BaseContextBuilder::new(uuid.to_owned(), conditions).patterns(patterns).build();
            LinearContext::new(base)
        };
        context_map.insert(Context::Linear(context1));
        assert_eq!(context_map.contexts_mut().len(), 1);
        assert_context_map_contains_uuid(&mut context_map, &uuid, vec!["A".as_bytes(), "B".as_bytes()].into_iter());
    }
}
