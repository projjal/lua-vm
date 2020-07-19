use crate::{RegisterIndex, ConstantIndex};

pub enum OpCode {
    Move {
        dst: RegisterIndex,
        src: RegisterIndex,
    },
    LoadK {
        dst: RegisterIndex,
        value: ConstantIndex,
    },
    // LoadKX {
    //     dst: RegisterIndex,
    // },
    // LoadBool {
    //     dst: RegisterIndex,
    //     val: bool,
    //     skip: bool,
    // },
    // LoadNil {
    //     dst: RegisterIndex,
    //     count: u8,
    // },
    // GetUpVal {
    //     dst: RegisterIndex,
    //     src: UpValueIndex,
    // },
    // GetTabUp,
    // GetTable,
    // SetTabUp,
    // SetUpVal,
    // SetTable,
    // NewTable,
    // LSefl,

}