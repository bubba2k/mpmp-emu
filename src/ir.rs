// Intermediate representation to be interpreted directly

// A register address
type Register = u32;

pub struct UnaryOp {
    pub target: Register,
    pub source_a: Register,
}

pub struct BinaryOp {
    pub target: Register,
    pub source_a: Register,
    pub source_b: Register,
}

pub struct TernaryOp {
    pub target: Register,
    pub source_a: Register,
    pub source_b: Register,
    pub source_c: Register,
}

pub enum JumpCondition {
    Always,
    Zero,
    NotZero,
    Carry,    // Or "less",
    NotCarry, // Or "Not less"
}

pub enum JumpTarget {
    AddressOffsetConstant(u32),
    AbsoluteAdressRegister(Register),
}

pub enum LoadSource {
    Constant(u32),
    RAM { address_register: Register },
}

pub enum Operation {
    Halt,
    Noop,

    Inc(UnaryOp),
    Dec(UnaryOp),
    Neg(UnaryOp),
    Not(UnaryOp),
    Move(UnaryOp),

    And(BinaryOp),
    Or(BinaryOp),
    Xor(BinaryOp),
    Xnor(BinaryOp),

    Add(BinaryOp),
    AddCarry(BinaryOp),
    Sub(BinaryOp),
    SubCarry(BinaryOp),
    Multiply(BinaryOp),
    ShiftLeft(BinaryOp),
    ShiftRight(BinaryOp),
    Test(BinaryOp),

    Add3(TernaryOp),

    Jump {
        target: JumpTarget,
        condition: JumpCondition,
    },

    Load {
        address: Register,
        source: LoadSource,
    },

    Store {
        address_register: Register,
        data_register: Register,
    },
}
