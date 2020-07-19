use crate::{CallInfo, Value};

pub struct State {
    pub call_info_list: Vec<CallInfo>,
    pub pc: usize,
    pub stack: Vec<Value>,
}

pub struct GlobalState {

}