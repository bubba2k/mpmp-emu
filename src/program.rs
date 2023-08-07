use crate::decoder::InstructionWord;

use super::ir::*;

pub struct Program {
    pub operations: Vec<Operation>,
}

impl From<&[u32]> for Program {
    fn from(coll: &[u32]) -> Self {
        let mut vec = Vec::new();

        for i in 0..coll.len() {
            vec.push(Operation::from(coll[i]))
        }

        Program { operations: vec }
    }
}

impl From<&[[u8; 3]]> for Program {
    fn from(coll: &[[u8; 3]]) -> Self {
        let mut vec = Vec::new();

        for buffer in coll {
            vec.push(Operation::from(*buffer))
        }

        Program { operations: vec }
    }
}
