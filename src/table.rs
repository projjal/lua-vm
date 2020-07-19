use std::collections::HashMap;

use crate::Value;

pub struct Table {
    array: Vec<Value>,
    map: HashMap<TableKey, Value>,
}

pub struct TableKey {
    
}