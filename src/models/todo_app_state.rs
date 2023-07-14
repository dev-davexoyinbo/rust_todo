use std::{sync::{Mutex, Arc}, collections::HashMap};

use super::todo_item::TodoItem;


pub struct TodoAppState {
    pub map: Arc<Mutex<HashMap<usize, TodoItem>>>,
}