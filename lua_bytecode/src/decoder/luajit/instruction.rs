trait Named {
    const NAME: &'static str;
}

pub enum InstructionPretty {
    FuncV,
    FuncF,
}

#[allow(non_camel_case_types)]
enum ArgumentType {
    T_VAR = 0,
    T_DST = 1,

    T_BS = 2,
    T_RBS = 3,

    T_UV = 4,

    T_LIT = 5,
    T_SLIT = 6,

    T_PRI = 7,
    T_NUM = 8,
    T_STR = 9,

    T_TAB = 10,
    T_FUN = 11,
    T_CDT = 12,
    T_JMP = 13,

    SLOT_FALSE = 30000,
    SLOT_TRUE = 30001,
}

pub struct LuaJitInstruction {
    name: String,
    //opcode:
    a_type: Option<ArgumentType>,
    b_type: Option<ArgumentType>,
    cd_type: Option<ArgumentType>,
}
