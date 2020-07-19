use crate::{Closure, Table};

pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
    Table(Table),
    Closure(Closure)
}