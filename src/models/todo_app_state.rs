use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use super::todo_item::TodoItem;

pub struct TodoAppState {
    pub map: Arc<RwLock<HashMap<usize, TodoItem>>>,
    max_id: Arc<RwLock<Option<usize>>>,
}

impl TodoAppState {
    pub fn new() -> Self {
        return TodoAppState {
            map: Arc::new(RwLock::new(HashMap::new())),
            max_id: Arc::new(RwLock::new(None)),
        };
    }

    pub fn from_hash_map(hash_map: HashMap<usize, TodoItem>) -> Self {
        let max_id = hash_map.keys().max().copied();
        return TodoAppState {
            map: Arc::new(RwLock::new(hash_map)),
            max_id: Arc::new(RwLock::new(max_id)),
        };
    }

    pub fn get_max_id(&self) -> Option<usize> {
        return *self.max_id.read().unwrap();
    }

    pub fn push(&self, todo_item: TodoItem) {
        let mut todo_map = self.map.write().unwrap();
        if !todo_map.contains_key(&(todo_item.id as usize)) {
            todo_map.insert(todo_item.id.try_into().unwrap(), todo_item);
            let mut max_id = self.max_id.write().unwrap();
            *max_id = todo_map.keys().max().copied();
        }
    }

    // fn recalculate_max_id(&self) {
    //     let todo_map = self.map.read().unwrap();
    //     let mut max_id = self.max_id.write().unwrap();
    //     *max_id = todo_map.keys().max().copied();
    // }

    pub fn delete_todo_with_id(&self, id: usize) -> Option<TodoItem> {
        let mut todo_map = self.map.write().unwrap();
        let rv = todo_map.remove(&id);
        let mut max_id = self.max_id.write().unwrap();
        *max_id = todo_map.keys().max().copied();

        return rv;
    } //end method delete_todo_with_id
}
