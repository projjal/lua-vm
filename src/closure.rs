use crate::{OpCode, Value};

pub struct UpValueDesc {
    name: String,
    idx: u16,
}

pub struct FunctionProto {
    pub fixed_params: u8,
    pub is_var_arg: bool,
    pub stack_size: u16,
    pub constants: Vec<Value>,
    pub instructions: Vec<OpCode>,
    pub protos: Vec<FunctionProto>,
    pub up_values: Vec<UpValueDesc>,
    pub source: String,
}

pub enum UpValueState{
    Open(),
    Closed(Value),
}

pub struct Closure {
    pub proto: FunctionProto,
    pub upvalues: Vec<UpValueState>,
}