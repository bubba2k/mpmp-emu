use crate::decoder::InstructionWord;

use super::ir::*;

pub struct Program {
    pub operations: Vec<Operation>,
    pub breakpoints: Vec<bool>,
    pub instruction_words: Vec<InstructionWord>,
}

impl Default for Program {
    fn default() -> Self {
        Program {
            operations: Vec::new(),
            breakpoints: Vec::new(),
            instruction_words: Vec::new(),
        }
    }
}

impl From<&[u32]> for Program {
    fn from(coll: &[u32]) -> Self {
        let mut op_vec = Vec::new();
        let mut instr_vec = Vec::new();

        for i in 0..coll.len() {
            let instr = InstructionWord::from(coll[i]);
            instr_vec.push(instr.clone());
            op_vec.push(Operation::from(instr))
        }

        let mut brk_vec = Vec::new();
        brk_vec.resize(op_vec.len(), false);

        Program {
            operations: op_vec,
            breakpoints: brk_vec,
            instruction_words: instr_vec,
        }
    }
}

impl From<&[[u8; 3]]> for Program {
    fn from(coll: &[[u8; 3]]) -> Self {
        let mut op_vec = Vec::new();
        let mut instr_vec = Vec::new();

        for buffer in coll {
            let instr = InstructionWord::from(*buffer);
            instr_vec.push(instr.clone());
            op_vec.push(Operation::from(instr));
        }

        let mut brk_vec = Vec::new();
        brk_vec.resize(op_vec.len(), false);

        Program {
            operations: op_vec,
            breakpoints: brk_vec,
            instruction_words: instr_vec,
        }
    }
}
