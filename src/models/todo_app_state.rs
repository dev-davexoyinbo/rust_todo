use std::{sync::{ Arc, RwLock}, collections::HashMap};

use super::todo_item::TodoItem;


pub struct TodoAppState {
    pub map: Arc<RwLock<HashMap<usize, TodoItem>>>,
}