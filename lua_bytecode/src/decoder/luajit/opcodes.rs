use super::{
    instruction::{ArgumentType, InstructionOperandsFormat},
    Error, Result,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LuaJitOpcode {
    Lj20(LuaJit20Opcode),
    Lj21(LuaJit21Opcode),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LuaJit20Opcode {
    ISLT,
    ISGE,
    ISLE,
    ISGT,

    ISEQV,
    ISNEV,

    ISEQS,
    ISNES,

    ISEQN,
    ISNEN,

    ISEQP,
    ISNEP,

    ISTC,
    ISFC,

    IST,
    ISF,

    MOV,
    NOT,
    UNM,
    LEN,

    ADDVN,
    SUBVN,
    MULVN,
    DIVVN,
    MODVN,

    ADDNV,
    SUBNV,
    MULNV,
    DIVNV,
    MODNV,

    ADDVV,
    SUBVV,
    MULVV,
    DIVVV,
    MODVV,

    POW,
    CAT,

    KSTR,
    KCDATA,
    KSHORT,
    KNUM,
    KPRI,

    KNIL,

    UGET,

    USETV,
    USETS,
    USETN,
    USETP,

    UCLO,

    FNEW,

    TNEW,

    TDUP,

    GGET,
    GSET,

    TGETV,
    TGETS,
    TGETB,

    TSETV,
    TSETS,
    TSETB,

    TSETM,

    CALLM,
    CALL,
    CALLMT,
    CALLT,

    ITERC,
    ITERN,

    VARG,

    ISNEXT,

    RETM,
    RET,
    RET0,
    RET1,

    FORI,
    JFORI,

    FORL,
    IFORL,
    JFORL,

    ITERL,
    IITERL,
    JITERL,

    LOOP,
    ILOOP,
    JLOOP,

    JMP,

    FUNCF,
    IFUNCF,
    JFUNCF,

    FUNCV,
    IFUNCV,
    JFUNCV,

    FUNCC,
    FUNCCW,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LuaJit21Opcode {
    ISLT = 0,
    ISGE,
    ISLE,
    ISGT,
    ISEQV,
    ISNEV,
    ISEQS,
    ISNES,
    ISEQN,
    ISNEN,
    ISEQP,
    ISNEP,
    ISTC,
    ISFC,
    IST,
    ISF,
    ISTYPE,
    ISNUM,
    MOV,
    NOT,
    UNM,
    LEN,
    ADDVN,
    SUBVN,
    MULVN,
    DIVVN,
    MODVN,
    ADDNV,
    SUBNV,
    MULNV,
    DIVNV,
    MODNV,
    ADDVV,
    SUBVV,
    MULVV,
    DIVVV,
    MODVV,
    POW,
    CAT,
    KSTR,
    KCDATA,
    KSHORT,
    KNUM,
    KPRI,
    KNIL,
    UGET,
    USETV,
    USETS,
    USETN,
    USETP,
    UCLO,
    FNEW,
    TNEW,
    TDUP,
    GGET,
    GSET,
    TGETV,
    TGETS,
    TGETB,
    TGETR,
    TSETV,
    TSETS,
    TSETB,
    TSETM,
    TSETR,
    CALLM,
    CALL,
    CALLMT,
    CALLT,
    ITERC,
    ITERN,
    VARG,
    ISNEXT,
    RETM,
    RET,
    RET0,
    RET1,
    FORI,
    JFORI,
    FORL,
    IFORL,
    JFORL,
    ITERL,
    IITERL,
    JITERL,
    LOOP,
    ILOOP,
    JLOOP,
    JMP,
    FUNCF,
    IFUNCF,
    JFUNCF,
    FUNCV,
    IFUNCV,
    JFUNCV,
    FUNCC,
    FUNCCW,
}

impl TryFrom<u32> for LuaJit21Opcode {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self> {
        match value {
            0 => Ok(Self::ISLT),
            1 => Ok(Self::ISGE),
            2 => Ok(Self::ISLE),
            3 => Ok(Self::ISGT),
            4 => Ok(Self::ISEQV),
            5 => Ok(Self::ISNEV),
            6 => Ok(Self::ISEQS),
            7 => Ok(Self::ISNES),
            8 => Ok(Self::ISEQN),
            9 => Ok(Self::ISNEN),
            10 => Ok(Self::ISEQP),
            11 => Ok(Self::ISNEP),
            12 => Ok(Self::ISTC),
            13 => Ok(Self::ISFC),
            14 => Ok(Self::IST),
            15 => Ok(Self::ISF),
            16 => Ok(Self::ISTYPE),
            17 => Ok(Self::ISNUM),
            18 => Ok(Self::MOV),
            19 => Ok(Self::NOT),
            20 => Ok(Self::UNM),
            21 => Ok(Self::LEN),
            22 => Ok(Self::ADDVN),
            23 => Ok(Self::SUBVN),
            24 => Ok(Self::MULVN),
            25 => Ok(Self::DIVVN),
            26 => Ok(Self::MODVN),
            27 => Ok(Self::ADDNV),
            28 => Ok(Self::SUBNV),
            29 => Ok(Self::MULNV),
            30 => Ok(Self::DIVNV),
            31 => Ok(Self::MODNV),
            32 => Ok(Self::ADDVV),
            33 => Ok(Self::SUBVV),
            34 => Ok(Self::MULVV),
            35 => Ok(Self::DIVVV),
            36 => Ok(Self::MODVV),
            37 => Ok(Self::POW),
            38 => Ok(Self::CAT),
            39 => Ok(Self::KSTR),
            40 => Ok(Self::KCDATA),
            41 => Ok(Self::KSHORT),
            42 => Ok(Self::KNUM),
            43 => Ok(Self::KPRI),
            44 => Ok(Self::KNIL),
            45 => Ok(Self::UGET),
            46 => Ok(Self::USETV),
            47 => Ok(Self::USETS),
            48 => Ok(Self::USETN),
            49 => Ok(Self::USETP),
            50 => Ok(Self::UCLO),
            51 => Ok(Self::FNEW),
            52 => Ok(Self::TNEW),
            53 => Ok(Self::TDUP),
            54 => Ok(Self::GGET),
            55 => Ok(Self::GSET),
            56 => Ok(Self::TGETV),
            57 => Ok(Self::TGETS),
            58 => Ok(Self::TGETB),
            59 => Ok(Self::TGETR),
            60 => Ok(Self::TSETV),
            61 => Ok(Self::TSETS),
            62 => Ok(Self::TSETB),
            63 => Ok(Self::TSETM),
            64 => Ok(Self::TSETR),
            65 => Ok(Self::CALLM),
            66 => Ok(Self::CALL),
            67 => Ok(Self::CALLMT),
            68 => Ok(Self::CALLT),
            69 => Ok(Self::ITERC),
            70 => Ok(Self::ITERN),
            71 => Ok(Self::VARG),
            72 => Ok(Self::ISNEXT),
            73 => Ok(Self::RETM),
            74 => Ok(Self::RET),
            75 => Ok(Self::RET0),
            76 => Ok(Self::RET1),
            77 => Ok(Self::FORI),
            78 => Ok(Self::JFORI),
            79 => Ok(Self::FORL),
            80 => Ok(Self::IFORL),
            81 => Ok(Self::JFORL),
            82 => Ok(Self::ITERL),
            83 => Ok(Self::IITERL),
            84 => Ok(Self::JITERL),
            85 => Ok(Self::LOOP),
            86 => Ok(Self::ILOOP),
            87 => Ok(Self::JLOOP),
            88 => Ok(Self::JMP),
            89 => Ok(Self::FUNCF),
            90 => Ok(Self::IFUNCF),
            91 => Ok(Self::JFUNCF),
            92 => Ok(Self::FUNCV),
            93 => Ok(Self::IFUNCV),
            94 => Ok(Self::JFUNCV),
            95 => Ok(Self::FUNCC),
            96 => Ok(Self::FUNCCW),
            _ => Err(Error::LuaJitInvalidOpcodeNumber(value)),
        }
    }
}

impl TryFrom<u32> for LuaJit20Opcode {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self> {
        match value {
            1 => Ok(Self::ISLT),
            2 => Ok(Self::ISGE),
            3 => Ok(Self::ISLE),
            4 => Ok(Self::ISGT),
            5 => Ok(Self::ISEQV),
            6 => Ok(Self::ISNEV),
            7 => Ok(Self::ISEQS),
            8 => Ok(Self::ISNES),
            9 => Ok(Self::ISEQN),
            10 => Ok(Self::ISNEN),
            11 => Ok(Self::ISEQP),
            12 => Ok(Self::ISNEP),
            13 => Ok(Self::ISTC),
            14 => Ok(Self::ISFC),
            15 => Ok(Self::IST),
            16 => Ok(Self::ISF),
            17 => Ok(Self::MOV),
            18 => Ok(Self::NOT),
            19 => Ok(Self::UNM),
            20 => Ok(Self::LEN),
            21 => Ok(Self::ADDVN),
            22 => Ok(Self::SUBVN),
            23 => Ok(Self::MULVN),
            24 => Ok(Self::DIVVN),
            25 => Ok(Self::MODVN),
            26 => Ok(Self::ADDNV),
            27 => Ok(Self::SUBNV),
            28 => Ok(Self::MULNV),
            29 => Ok(Self::DIVNV),
            30 => Ok(Self::MODNV),
            31 => Ok(Self::ADDVV),
            33 => Ok(Self::SUBVV),
            34 => Ok(Self::MULVV),
            35 => Ok(Self::DIVVV),
            36 => Ok(Self::MODVV),
            37 => Ok(Self::POW),
            38 => Ok(Self::CAT),
            39 => Ok(Self::KSTR),
            40 => Ok(Self::KCDATA),
            41 => Ok(Self::KSHORT),
            42 => Ok(Self::KNUM),
            43 => Ok(Self::KPRI),
            44 => Ok(Self::KNIL),
            45 => Ok(Self::UGET),
            46 => Ok(Self::USETV),
            47 => Ok(Self::USETS),
            48 => Ok(Self::USETN),
            49 => Ok(Self::USETP),
            50 => Ok(Self::UCLO),
            51 => Ok(Self::FNEW),
            52 => Ok(Self::TNEW),
            53 => Ok(Self::TDUP),
            54 => Ok(Self::GGET),
            55 => Ok(Self::GSET),
            56 => Ok(Self::TGETV),
            57 => Ok(Self::TGETS),
            58 => Ok(Self::TGETB),
            59 => Ok(Self::TSETV),
            60 => Ok(Self::TSETS),
            61 => Ok(Self::TSETB),
            62 => Ok(Self::TSETM),
            63 => Ok(Self::CALLM),
            64 => Ok(Self::CALL),
            65 => Ok(Self::CALLMT),
            66 => Ok(Self::CALLT),
            67 => Ok(Self::ITERC),
            68 => Ok(Self::ITERN),
            69 => Ok(Self::VARG),
            70 => Ok(Self::ISNEXT),
            71 => Ok(Self::RETM),
            72 => Ok(Self::RET),
            73 => Ok(Self::RET0),
            74 => Ok(Self::RET1),
            75 => Ok(Self::FORI),
            76 => Ok(Self::JFORI),
            77 => Ok(Self::FORL),
            78 => Ok(Self::IFORL),
            79 => Ok(Self::JFORL),
            80 => Ok(Self::ITERL),
            81 => Ok(Self::IITERL),
            82 => Ok(Self::JITERL),
            83 => Ok(Self::LOOP),
            84 => Ok(Self::ILOOP),
            85 => Ok(Self::JLOOP),
            86 => Ok(Self::JMP),
            87 => Ok(Self::FUNCF),
            88 => Ok(Self::IFUNCF),
            89 => Ok(Self::JFUNCF),
            90 => Ok(Self::FUNCV),
            91 => Ok(Self::IFUNCV),
            92 => Ok(Self::JFUNCV),
            93 => Ok(Self::FUNCC),
            94 => Ok(Self::FUNCCW),
            _ => Err(Error::LuaJitInvalidOpcodeNumber(value)),
        }
    }
}

impl From<LuaJitOpcode> for [Option<ArgumentType>; 3] {
    fn from(value: LuaJitOpcode) -> Self {
        (&value).into()
    }
}

impl From<&LuaJitOpcode> for [Option<ArgumentType>; 3] {
    fn from(value: &LuaJitOpcode) -> Self {
        match value {
            LuaJitOpcode::Lj20(lua_jit20_opcode) => lua_jit20_opcode.into(),
            LuaJitOpcode::Lj21(lua_jit21_opcode) => lua_jit21_opcode.into(),
        }
    }
}

impl From<LuaJit20Opcode> for [Option<ArgumentType>; 3] {
    fn from(value: LuaJit20Opcode) -> Self {
        (&value).into()
    }
}

impl From<&LuaJit20Opcode> for [Option<ArgumentType>; 3] {
    fn from(val: &LuaJit20Opcode) -> Self {
        use ArgumentType::*;
        use LuaJit20Opcode::*;
        match val {
            ISLT | ISGE | ISLE | ISGT | ISEQV | ISNEV => [Some(T_VAR), None, Some(T_VAR)],

            ISEQS | ISNES => [Some(T_VAR), None, Some(T_VAR)],

            ISEQN | ISNEN => [Some(T_VAR), None, Some(T_NUM)],

            ISEQP | ISNEP => [Some(T_VAR), None, Some(T_PRI)],

            ISTC | ISFC => [Some(T_DST), None, Some(T_VAR)],

            IST | ISF => [None, None, Some(T_VAR)],

            // ISTYPE | ISNUM => [Some(T_VAR), None, Some(T_LIT)],
            MOV | NOT | UNM | LEN => [Some(T_DST), None, Some(T_VAR)],

            CAT => [Some(T_DST), Some(T_RBS), Some(T_RBS)],

            KSTR => [Some(T_DST), None, Some(T_STR)],
            KCDATA => [Some(T_DST), None, Some(T_CDT)],
            KSHORT => [Some(T_DST), None, Some(T_SLIT)],
            KNUM => [Some(T_DST), None, Some(T_NUM)],
            KPRI => [Some(T_DST), None, Some(T_PRI)],

            KNIL => [Some(T_BS), None, Some(T_BS)],

            UGET => [Some(T_DST), None, Some(T_UV)],

            USETV => [Some(T_UV), None, Some(T_VAR)],
            USETS => [Some(T_UV), None, Some(T_STR)],
            USETN => [Some(T_UV), None, Some(T_NUM)],
            USETP => [Some(T_UV), None, Some(T_PRI)],

            UCLO => [Some(T_RBS), None, Some(T_JMP)],

            FNEW => [Some(T_DST), None, Some(T_FUN)],

            TNEW => [Some(T_DST), None, Some(T_LIT)],

            TDUP => [Some(T_DST), None, Some(T_TAB)],

            GGET => [Some(T_DST), None, Some(T_STR)],

            GSET => [Some(T_VAR), None, Some(T_STR)],

            TGETV => [Some(T_DST), Some(T_VAR), Some(T_VAR)],
            TGETS => [Some(T_DST), Some(T_VAR), Some(T_STR)],
            TGETB => [Some(T_DST), Some(T_VAR), Some(T_LIT)],
            //TGETR => [Some(T_DST), Some(T_VAR), Some(T_VAR)],
            TSETV => [Some(T_VAR), Some(T_VAR), Some(T_VAR)],
            TSETS => [Some(T_VAR), Some(T_VAR), Some(T_STR)],
            TSETB => [Some(T_VAR), Some(T_VAR), Some(T_LIT)],

            TSETM => [Some(T_BS), None, Some(T_NUM)],

            //TSETR => [Some(T_VAR), Some(T_VAR), Some(T_VAR)],
            CALL | CALLM => [Some(T_BS), Some(T_LIT), Some(T_LIT)],

            CALLMT | CALLT => [Some(T_BS), None, Some(T_LIT)],

            ITERC | ITERN | VARG => [Some(T_BS), Some(T_LIT), Some(T_LIT)],

            ISNEXT => [Some(T_BS), None, Some(T_JMP)],

            RETM => [Some(T_BS), None, Some(T_LIT)],
            RET | RET0 | RET1 => [Some(T_RBS), None, Some(T_LIT)],

            FORI | JFORI | FORL | IFORL | JFORL | ITERL | IITERL => [Some(T_BS), None, Some(T_JMP)],

            JITERL => [Some(T_BS), None, Some(T_LIT)],

            LOOP | ILOOP => [Some(T_RBS), None, Some(T_LIT)],
            JLOOP => [Some(T_RBS), None, Some(T_LIT)],

            JMP => [Some(T_RBS), None, Some(T_JMP)],

            FUNCF | IFUNCF | FUNCV | IFUNCV | FUNCC | FUNCCW => [Some(T_RBS), None, None],

            JFUNCF | JFUNCV => [Some(T_RBS), None, Some(T_LIT)],

            ADDVN | SUBVN | MULVN | DIVVN | MODVN | ADDNV | SUBNV | MULNV | DIVNV | MODNV
            | ADDVV | SUBVV | MULVV | DIVVV | MODVV | POW => {
                [Some(T_DST), Some(T_VAR), Some(T_NUM)]
            }
        }
    }
}

impl From<LuaJit21Opcode> for [Option<ArgumentType>; 3] {
    fn from(val: LuaJit21Opcode) -> Self {
        (&val).into()
    }
}

impl From<&LuaJit21Opcode> for [Option<ArgumentType>; 3] {
    fn from(val: &LuaJit21Opcode) -> Self {
        use ArgumentType::*;
        use LuaJit21Opcode::*;
        match val {
            ISLT | ISGE | ISLE | ISGT | ISEQV | ISNEV => [Some(T_VAR), None, Some(T_VAR)],

            ISEQS | ISNES => [Some(T_VAR), None, Some(T_VAR)],

            ISEQN | ISNEN => [Some(T_VAR), None, Some(T_NUM)],

            ISEQP | ISNEP => [Some(T_VAR), None, Some(T_PRI)],

            ISTC | ISFC => [Some(T_DST), None, Some(T_VAR)],

            IST | ISF => [None, None, Some(T_VAR)],

            ISTYPE | ISNUM => [Some(T_VAR), None, Some(T_LIT)],
            MOV | NOT | UNM | LEN => [Some(T_DST), None, Some(T_VAR)],

            CAT => [Some(T_DST), Some(T_RBS), Some(T_RBS)],

            KSTR => [Some(T_DST), None, Some(T_STR)],
            KCDATA => [Some(T_DST), None, Some(T_CDT)],
            KSHORT => [Some(T_DST), None, Some(T_SLIT)],
            KNUM => [Some(T_DST), None, Some(T_NUM)],
            KPRI => [Some(T_DST), None, Some(T_PRI)],

            KNIL => [Some(T_BS), None, Some(T_BS)],

            UGET => [Some(T_DST), None, Some(T_UV)],

            USETV => [Some(T_UV), None, Some(T_VAR)],
            USETS => [Some(T_UV), None, Some(T_STR)],
            USETN => [Some(T_UV), None, Some(T_NUM)],
            USETP => [Some(T_UV), None, Some(T_PRI)],

            UCLO => [Some(T_RBS), None, Some(T_JMP)],

            FNEW => [Some(T_DST), None, Some(T_FUN)],

            TNEW => [Some(T_DST), None, Some(T_LIT)],

            TDUP => [Some(T_DST), None, Some(T_TAB)],

            GGET => [Some(T_DST), None, Some(T_STR)],

            GSET => [Some(T_VAR), None, Some(T_STR)],

            TGETV => [Some(T_DST), Some(T_VAR), Some(T_VAR)],
            TGETS => [Some(T_DST), Some(T_VAR), Some(T_STR)],
            TGETB => [Some(T_DST), Some(T_VAR), Some(T_LIT)],
            TGETR => [Some(T_DST), Some(T_VAR), Some(T_VAR)],
            TSETV => [Some(T_VAR), Some(T_VAR), Some(T_VAR)],
            TSETS => [Some(T_VAR), Some(T_VAR), Some(T_STR)],
            TSETB => [Some(T_VAR), Some(T_VAR), Some(T_LIT)],

            TSETM => [Some(T_BS), None, Some(T_NUM)],

            TSETR => [Some(T_VAR), Some(T_VAR), Some(T_VAR)],
            CALL | CALLM => [Some(T_BS), Some(T_LIT), Some(T_LIT)],

            CALLMT | CALLT => [Some(T_BS), None, Some(T_LIT)],

            ITERC | ITERN | VARG => [Some(T_BS), Some(T_LIT), Some(T_LIT)],

            ISNEXT => [Some(T_BS), None, Some(T_JMP)],

            RETM => [Some(T_BS), None, Some(T_LIT)],
            RET | RET0 | RET1 => [Some(T_RBS), None, Some(T_LIT)],

            FORI | JFORI | FORL | IFORL | JFORL | ITERL | IITERL => [Some(T_BS), None, Some(T_JMP)],

            JITERL => [Some(T_BS), None, Some(T_LIT)],

            LOOP | ILOOP => [Some(T_RBS), None, Some(T_LIT)],
            JLOOP => [Some(T_RBS), None, Some(T_LIT)],

            JMP => [Some(T_RBS), None, Some(T_JMP)],

            FUNCF | IFUNCF | FUNCV | IFUNCV | FUNCC | FUNCCW => [Some(T_RBS), None, None],

            JFUNCF | JFUNCV => [Some(T_RBS), None, Some(T_LIT)],

            ADDVN | SUBVN | MULVN | DIVVN | MODVN | ADDNV | SUBNV | MULNV | DIVNV | MODNV
            | ADDVV | SUBVV | MULVV | DIVVV | MODVV | POW => {
                [Some(T_DST), Some(T_VAR), Some(T_NUM)]
            }
        }
    }
}

impl From<LuaJit20Opcode> for InstructionOperandsFormat {
    fn from(value: LuaJit20Opcode) -> Self {
        (&value).into()
    }
}

impl From<&LuaJit20Opcode> for InstructionOperandsFormat {
    fn from(value: &LuaJit20Opcode) -> Self {
        match value {
            LuaJit20Opcode::ADDVN
            | LuaJit20Opcode::SUBVN
            | LuaJit20Opcode::MULVN
            | LuaJit20Opcode::DIVVN
            | LuaJit20Opcode::MODVN
            | LuaJit20Opcode::ADDNV
            | LuaJit20Opcode::SUBNV
            | LuaJit20Opcode::MULNV
            | LuaJit20Opcode::DIVNV
            | LuaJit20Opcode::MODNV
            | LuaJit20Opcode::ADDVV
            | LuaJit20Opcode::SUBVV
            | LuaJit20Opcode::MULVV
            | LuaJit20Opcode::DIVVV
            | LuaJit20Opcode::MODVV
            | LuaJit20Opcode::POW
            | LuaJit20Opcode::CAT
            | LuaJit20Opcode::TGETV
            | LuaJit20Opcode::TGETS
            | LuaJit20Opcode::TGETB
            | LuaJit20Opcode::TSETV
            | LuaJit20Opcode::TSETS
            | LuaJit20Opcode::TSETB
            | LuaJit20Opcode::CALLM
            | LuaJit20Opcode::CALL
            | LuaJit20Opcode::ITERC
            | LuaJit20Opcode::ITERN
            | LuaJit20Opcode::VARG => Self::Abc,
            _ => Self::Ad,
        }
    }
}

impl From<LuaJit21Opcode> for InstructionOperandsFormat {
    fn from(value: LuaJit21Opcode) -> Self {
        (&value).into()
    }
}

impl From<&LuaJit21Opcode> for InstructionOperandsFormat {
    fn from(value: &LuaJit21Opcode) -> Self {
        match value {
            LuaJit21Opcode::ADDVN
            | LuaJit21Opcode::SUBVN
            | LuaJit21Opcode::MULVN
            | LuaJit21Opcode::DIVVN
            | LuaJit21Opcode::MODVN
            | LuaJit21Opcode::ADDNV
            | LuaJit21Opcode::SUBNV
            | LuaJit21Opcode::MULNV
            | LuaJit21Opcode::DIVNV
            | LuaJit21Opcode::MODNV
            | LuaJit21Opcode::ADDVV
            | LuaJit21Opcode::SUBVV
            | LuaJit21Opcode::MULVV
            | LuaJit21Opcode::DIVVV
            | LuaJit21Opcode::MODVV
            | LuaJit21Opcode::POW
            | LuaJit21Opcode::CAT
            | LuaJit21Opcode::TGETV
            | LuaJit21Opcode::TGETS
            | LuaJit21Opcode::TGETB
            | LuaJit21Opcode::TGETR
            | LuaJit21Opcode::TSETV
            | LuaJit21Opcode::TSETS
            | LuaJit21Opcode::TSETB
            | LuaJit21Opcode::TSETR
            | LuaJit21Opcode::CALLM
            | LuaJit21Opcode::CALL
            | LuaJit21Opcode::ITERC
            | LuaJit21Opcode::ITERN
            | LuaJit21Opcode::VARG => Self::Abc,
            _ => Self::Ad,
        }
    }
}

impl From<LuaJitOpcode> for InstructionOperandsFormat {
    fn from(value: LuaJitOpcode) -> Self {
        (&value).into()
    }
}

impl From<&LuaJitOpcode> for InstructionOperandsFormat {
    fn from(value: &LuaJitOpcode) -> Self {
        match value {
            LuaJitOpcode::Lj20(lua_jit20_opcode) => lua_jit20_opcode.into(),
            LuaJitOpcode::Lj21(lua_jit21_opcode) => lua_jit21_opcode.into(),
        }
    }
}
