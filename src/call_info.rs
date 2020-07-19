use crate::{StackIndex};

pub struct CallInfo {
    top: StackIndex,
    base: StackIndex,
    pub closure: StackIndex,
    num_results: u16,
}