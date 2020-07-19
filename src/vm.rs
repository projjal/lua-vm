use crate::{OpCode, State, Value};

fn execute(state: &mut State) {
    let value = &state.stack[state.call_info_list[state.call_info_list.len() - 1].closure as usize];
    let closure = match value {
        Value::Closure(closure) => closure,
        _ => panic!("Not a closure"),
    };
    let mut pc = state.pc;
    loop {
        let op = &closure.proto.instructions[pc];
        pc += 1;
        
        match op {
            OpCode::Move {dst, src} => {
                // do work
            }
            OpCode::LoadK {dst, value} => {
                // do work
            }
        }
    }
}