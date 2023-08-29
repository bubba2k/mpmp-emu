// Intermediate representation to be interpreted directly

// A register address
pub type Register = usize;

#[derive(Debug, PartialEq, Eq)]
pub struct UnaryOp {
    pub target: Register,
    pub source_a: Register,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BinaryOp {
    pub target: Register,
    pub source_a: Register,
    pub source_b: Register,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TernaryOp {
    pub target: Register,
    pub source_a: Register,
    pub source_b: Register,
    pub source_c: Register,
}

#[derive(Debug, PartialEq, Eq)]
pub enum JumpCondition {
    Always,
    Zero,
    NotZero,
    Carry,    // Or "less",
    NotCarry, // Or "Not less"
}

#[derive(Debug, PartialEq, Eq)]
pub enum JumpTarget {
    AddressOffsetConstant(i16),
    AbsoluteAdressRegister(Register),
}

#[derive(Debug, PartialEq, Eq)]
pub enum LoadSource {
    Constant(u16),
    RAM { address_register: Register },
}

#[derive(Debug, PartialEq, Eq)]
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
        target_register: Register,
        source: LoadSource,
    },

    Store {
        address_register: Register,
        data_register: Register,
    },
}

// Functionalities for disassembling instructions (untested!)
use Operation::*;
impl Operation {
    pub fn get_assembly_string(&self) -> String {
        match self {
            Noop | Halt => String::from(self.get_assembly_opname()),
            Inc(unop) | Dec(unop) => {
                format!("{} %reg{}", self.get_assembly_opname(), unop.source_a)
            }
            Neg(unop) | Not(unop) | Move(unop) => {
                format!(
                    "{} %reg{} %reg{}",
                    self.get_assembly_opname(),
                    unop.target,
                    unop.source_a
                )
            }
            Add(o) | AddCarry(o) | Sub(o) | SubCarry(o) | And(o) | Or(o) | Xor(o) | Xnor(o)
            | ShiftLeft(o) | ShiftRight(o) | Multiply(o) | Test(o) => {
                format!(
                    "{} %reg{} %reg{} %reg{}",
                    self.get_assembly_opname(),
                    o.target,
                    o.source_a,
                    o.source_b
                )
            }
            Add3(o) => {
                format!(
                    "add3 %reg{} %reg{} %reg{} %reg{}",
                    o.target, o.source_a, o.source_b, o.source_c
                )
            }
            Jump { target, .. } => match target {
                JumpTarget::AbsoluteAdressRegister(reg) => {
                    format!("{} %reg{}", self.get_assembly_opname(), *reg)
                }
                JumpTarget::AddressOffsetConstant(offset) => {
                    format!("{} {}", self.get_assembly_opname(), *offset)
                }
            },
            Load {
                source,
                target_register,
            } => match source {
                LoadSource::Constant(constant) => {
                    format!(
                        "{} %reg{} {:#X}",
                        self.get_assembly_opname(),
                        *target_register,
                        *constant
                    )
                }
                LoadSource::RAM { address_register } => {
                    format!("{} %reg{}", self.get_assembly_opname(), *address_register)
                }
            },
            Store {
                address_register,
                data_register,
            } => {
                format!(
                    "{} %reg{} %reg{}",
                    self.get_assembly_opname(),
                    *address_register,
                    *data_register
                )
            }
        }
    }

    fn get_assembly_opname(&self) -> &str {
        match self {
            Self::Noop => "nop",
            Self::Halt => "hlt",
            Self::Add(_) => "add",
            Self::AddCarry(_) => "addc",
            Self::Add3(_) => "add3",
            Self::Sub(_) => "sub",
            Self::SubCarry(_) => "subc",
            Self::And(_) => "and",
            Self::Or(_) => "or",
            Self::Xor(_) => "xor",
            Self::Xnor(_) => "xnor",
            Self::ShiftLeft(_) => "shl",
            Self::ShiftRight(_) => "shr",
            Self::Inc(_) => "inc",
            Self::Neg(_) => "neg",
            Self::Test(_) => "tst",
            Self::Dec(_) => "dec",
            Self::Move(_) => "mov",
            Self::Multiply(_) => "mul",
            Self::Not(_) => "not",
            Self::Jump { target, condition } => match target {
                JumpTarget::AddressOffsetConstant(_) => match condition {
                    JumpCondition::Zero => "jzr",
                    JumpCondition::Carry => "jcr",
                    JumpCondition::NotZero => "jnzr",
                    JumpCondition::Always => "jrcon",
                    JumpCondition::NotCarry => "jncr",
                },
                JumpTarget::AbsoluteAdressRegister(_) => match condition {
                    JumpCondition::Zero => "jz",
                    JumpCondition::Carry => "jc",
                    JumpCondition::NotZero => "jnz",
                    JumpCondition::Always => "jmp",
                    JumpCondition::NotCarry => "jnc",
                },
            },
            Self::Load { source, .. } => match source {
                LoadSource::RAM { .. } => "ld",
                LoadSource::Constant(_) => "ldc",
            },
            Self::Store { .. } => "st",
        }
    }
}
