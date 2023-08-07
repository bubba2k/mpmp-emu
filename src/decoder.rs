use std::fmt::Display;

use crate::ir::*;

pub struct InstructionWord {
    buffer: u32, // A bitfield, essentially. We only need 20 bits but this is still the easiest way
                 // to do it
}

// Enumeration of all possible opcodes
#[derive(FromPrimitive, Debug, PartialEq, Eq)]
enum Opcode {
    ADD = 0x0,
    ADD3 = 0x1,
    ADC = 0x2,
    SUB = 0x3,
    SUBC = 0x4,
    INC = 0x5,
    DEC = 0x6,
    MUL = 0x7,
    TST = 0x8,
    AND = 0x9,
    OR = 0xa,
    NOT = 0xb,
    NEG = 0xc,
    XOR = 0xd,
    XNOR = 0xe,
    SHL = 0xf,
    SHR = 0x10,

    MOV = 0x48,

    JMP = 0x50,
    JZ = 0x51,
    JNZ = 0x52,
    JC = 0x53,
    JNC = 0x54,
    JRCON = 0x58,
    JZR = 0x59,
    JNZR = 0x5a,
    JCR = 0x5b,
    JNCR = 0x5c,

    ST = 0x68,
    LD = 0x69,

    NOP = 0x6c,
    DBG = 0x7e,
    HLT = 0x7f,
    LDC = 0x80,
}

impl From<u32> for InstructionWord {
    fn from(value: u32) -> Self {
        InstructionWord { buffer: value }
    }
}

impl From<[u8; 3]> for InstructionWord {
    fn from(value: [u8; 3]) -> Self {
        let mut buffer: u32 = 0u32;

        // Iterate over the bytes and shift them into our bitfield
        for (byte_idx, &byte) in value.iter().rev().enumerate() {
            buffer |= (byte as u32) << (byte_idx * 8)
        }

        InstructionWord { buffer }
    }
}

impl InstructionWord {
    // Get the indexed bits and return them interpreted as an integer
    // Bounds are inclusive
    fn get_bits(&self, lower: u32, upper: u32) -> Result<u32, &'static str> {
        // Quick sanity check, might remove this later considering
        // the arguments to this function should be known at compile
        // for our use cases anyway
        if lower > upper || lower >= 20 || upper >= 20 {
            return Err("Invalid bounds");
        }

        // Construct the bitmask
        let mut mask: u32 = 0u32;
        for idx in lower..=upper {
            mask |= 0x1u32 << idx;
        }

        // Use the mask to get the bits and shift them back right
        let res_val: u32 = (self.buffer & mask) >> lower;

        Ok(res_val)
    }

    fn get_opcode(&self) -> Opcode {
        // Since the lower 4 bits of 16 bit constants in LDC (load constant)
        // operations are stored in the lower 4 bits of the opcode,
        // and all opcodes above 0x80 are also just LDC ops, we clamp it here.
        let value = self.get_bits(0, 7).unwrap().clamp(0, 0x80);

        // Attempt to parse an enum value from the u32
        return match num::FromPrimitive::from_u32(value as u32) {
            Some(val) => val,
            None => {
                // Print a warning indicating unknown opcode and return NOP
                // Might want more solid error handling later on
                eprintln!("Unknown opcode in {:#8x}", self.buffer);
                Opcode::NOP
            }
        };
    }

    fn get_op_a(&self) -> u8 {
        self.get_bits(8, 10).unwrap() as u8
    }

    fn get_op_b(&self) -> u8 {
        self.get_bits(11, 13).unwrap() as u8
    }

    fn get_op_c(&self) -> u8 {
        self.get_bits(14, 16).unwrap() as u8
    }

    fn get_target(&self) -> u8 {
        self.get_bits(17, 19).unwrap() as u8
    }

    fn get_constant12(&self) -> u16 {
        self.get_bits(8, 19).unwrap() as u16
    }

    fn get_constant16(&self) -> u16 {
        let lower_4: u16 = self.get_bits(0, 3).unwrap() as u16;
        let upper_12: u16 = self.get_bits(8, 19).unwrap() as u16;

        lower_4 | (upper_12 << 4)
    }

    fn get_load(&self) -> bool {
        self.get_bits(7, 7).unwrap() == 1
    }

    fn get_load_address(&self) -> u8 {
        self.get_bits(4, 6).unwrap() as u8
    }

    fn get_unary_op(&self) -> UnaryOp {
        UnaryOp {
            target: self.get_target() as usize,
            source_a: self.get_op_a() as usize,
        }
    }

    fn get_binary_op(&self) -> BinaryOp {
        BinaryOp {
            target: self.get_target() as usize,
            source_a: self.get_op_a() as usize,
            source_b: self.get_op_b() as usize,
        }
    }

    fn get_ternary_op(&self) -> TernaryOp {
        TernaryOp {
            target: self.get_target() as usize,
            source_a: self.get_op_a() as usize,
            source_b: self.get_op_b() as usize,
            source_c: self.get_op_c() as usize,
        }
    }
}

impl Display for InstructionWord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#b}", self.buffer)
    }
}

use super::ir;
// TODO: Test this. Test it good.
// Parse an operation from an instruction word.
impl From<InstructionWord> for ir::Operation {
    fn from(iw: InstructionWord) -> Self {
        return match iw.get_opcode() {
            Opcode::ADD => Operation::Add(iw.get_binary_op()),
            Opcode::ADD3 => Operation::Add3(iw.get_ternary_op()),
            Opcode::ADC => Operation::AddCarry(iw.get_binary_op()),
            Opcode::SUB => Operation::Sub(iw.get_binary_op()),
            Opcode::SUBC => Operation::SubCarry(iw.get_binary_op()),
            Opcode::INC => Operation::Inc(iw.get_unary_op()),
            Opcode::DEC => Operation::Dec(iw.get_unary_op()),
            Opcode::MUL => Operation::Multiply(iw.get_binary_op()),
            Opcode::TST => Operation::Test(iw.get_binary_op()),
            Opcode::AND => Operation::And(iw.get_binary_op()),
            Opcode::OR => Operation::Or(iw.get_binary_op()),
            Opcode::NOT => Operation::Not(iw.get_unary_op()),
            Opcode::NEG => Operation::Neg(iw.get_unary_op()),
            Opcode::XOR => Operation::Xor(iw.get_binary_op()),
            Opcode::XNOR => Operation::Xnor(iw.get_binary_op()),
            Opcode::SHL => Operation::ShiftLeft(iw.get_binary_op()),
            Opcode::SHR => Operation::ShiftRight(iw.get_binary_op()),
            Opcode::MOV => Operation::Move(iw.get_unary_op()),

            // Absolute jumps
            Opcode::JMP => Operation::Jump {
                target: JumpTarget::AbsoluteAdressRegister(iw.get_op_a() as usize),
                condition: JumpCondition::Always,
            },
            Opcode::JZ => Operation::Jump {
                target: JumpTarget::AbsoluteAdressRegister(iw.get_op_a() as usize),
                condition: JumpCondition::Zero,
            },
            Opcode::JNZ => Operation::Jump {
                target: JumpTarget::AbsoluteAdressRegister(iw.get_op_a() as usize),
                condition: JumpCondition::NotZero,
            },
            Opcode::JC => Operation::Jump {
                target: JumpTarget::AbsoluteAdressRegister(iw.get_op_a() as usize),
                condition: JumpCondition::Carry, // Less
            },
            Opcode::JNC => Operation::Jump {
                target: JumpTarget::AbsoluteAdressRegister(iw.get_op_a() as usize),
                condition: JumpCondition::NotCarry, // NotLess
            },

            // Relative jumps
            Opcode::JRCON => Operation::Jump {
                target: JumpTarget::AddressOffsetConstant(iw.get_constant12()),
                condition: JumpCondition::Always,
            },
            Opcode::JZR => Operation::Jump {
                target: JumpTarget::AddressOffsetConstant(iw.get_constant12()),
                condition: JumpCondition::Zero,
            },
            Opcode::JNZR => Operation::Jump {
                target: JumpTarget::AddressOffsetConstant(iw.get_constant12()),
                condition: JumpCondition::NotZero,
            },
            Opcode::JCR => Operation::Jump {
                target: JumpTarget::AddressOffsetConstant(iw.get_constant12()),
                condition: JumpCondition::Carry,
            },
            Opcode::JNCR => Operation::Jump {
                target: JumpTarget::AddressOffsetConstant(iw.get_constant12()),
                condition: JumpCondition::NotCarry,
            },

            Opcode::ST => Operation::Store {
                address_register: iw.get_op_b() as usize, // Yes, the operands just are this way.
                data_register: iw.get_op_a() as usize,
            },
            // TODO: The way the target / operand bits are read here makes me really uneasy.
            // Better check this out again later
            // Most likely this is just how the operands are encoded here for some reason.
            Opcode::LD => Operation::Load {
                target_register: iw.get_target() as usize,
                source: LoadSource::RAM {
                    address_register: iw.get_op_b() as usize,
                },
            },

            Opcode::NOP => Operation::Noop,
            // The DGB opcode really does not matter to our emu at the moment
            Opcode::DBG => Operation::Noop,
            Opcode::HLT => Operation::Halt,

            Opcode::LDC => Operation::Load {
                target_register: iw.get_load_address() as usize,
                source: LoadSource::Constant(iw.get_constant16()),
            },
        };
    }
}

impl From<u32> for Operation {
    fn from(value: u32) -> Self {
        Operation::from(InstructionWord::from(value))
    }
}

impl From<[u8; 3]> for Operation {
    fn from(value: [u8; 3]) -> Self {
        Operation::from(InstructionWord::from(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BYTES1: [u8; 3] = [0x03, 0x6f, 0x66];
    const BUFFER1: u32 = 0x03_6f_66u32;
    const BYTES2: [u8; 3] = [0x0f, 0x0f, 0x0f];
    const BUFFER2: u32 = 0x0f_0f_0fu32;
    const BYTES3: [u8; 3] = [0x07, 0x23, 0xf7];
    const BUFFER3: u32 = 0x07_23_f7u32;
    const BYTES4: [u8; 3] = [0b0000_1010, 0b0010_1001, 0b0011_0011];
    const BUFFER4: u32 = 0b0000_1010_0010_1001_0011_0011;

    #[test]
    fn instructionword_construction_test() {
        let word1 = InstructionWord::from(BYTES1);
        assert_eq!(word1.buffer, BUFFER1);

        let word2 = InstructionWord::from(BYTES2);
        assert_eq!(word2.buffer, BUFFER2);

        let word3 = InstructionWord::from(BYTES3);
        assert_eq!(word3.buffer, BUFFER3);

        let word4 = InstructionWord::from(BYTES4);
        assert_eq!(word4.buffer, BUFFER4);

        assert_eq!(word1.get_bits(0, 3).unwrap(), 0x6);
        assert_eq!(word2.get_bits(8, 11).unwrap(), 0xf);
        assert_eq!(word3.get_bits(4, 4).unwrap(), 0x1);
        assert_eq!(word1.get_bits(0, 0).unwrap(), 0x0);
        assert_eq!(word4.get_bits(10, 12).unwrap(), 0b010);
    }

    const INSTR1: u32 = 0x800a1; // ldc %reg2 0x8001
    const INSTR2: u32 = 0x00005; // inc %reg0
    const INSTR3: u32 = 0x01100; // add %reg0 %reg1 %reg2
    const INSTR4: u32 = 0x6ac01; // add3 %reg3 %reg4 %reg5 %reg2
    const INSTR5: u32 = 0x02c03; // sub %reg0 %reg4 %reg5
    const INSTR6: u32 = 0x42104; // subc %reg2 %reg1 %reg4
    const INSTR7: u32 = 0x783d2; // ldc %reg5 0x7832
    const INSTR8: u32 = 0x0210a; // or $reg0 %reg1 %reg4
    const INSTR9: u32 = 0x0006c; // nop
    const INSTR10: u32 = 0x00251; // jz %reg2
    const INSTR11: u32 = 0x00350; // jmp %reg3
    const INSTR12: u32 = 0x0045b; // jcr 5
    const INSTR13: u32 = 0x01968; // st %reg3 %reg1
    const INSTR14: u32 = 0x42869; // ld %reg2 %reg5
    const INSTRH: u32 = 0x0007f; // halt

    #[test]
    fn instructionword_getter_test() {
        let word1 = InstructionWord::from(INSTR1);
        assert_eq!(word1.get_opcode(), Opcode::LDC);
        assert_eq!(word1.get_load_address(), 2);
        assert_eq!(word1.get_constant16(), 0x8001);

        let op1 = Operation::from(word1);
        assert_eq!(
            op1,
            Operation::Load {
                target_register: 0x2,
                source: LoadSource::Constant(0x8001),
            }
        );

        let word2 = InstructionWord::from(INSTR2);
        assert_eq!(word2.get_opcode(), Opcode::INC);
        assert_eq!(word2.get_target(), 0x0);
        assert_eq!(word2.get_op_a(), 0x0);

        let op2 = Operation::from(word2);
        assert_eq!(
            op2,
            Operation::Inc(UnaryOp {
                target: 0x0,
                source_a: 0x0
            }),
        );

        let word3 = InstructionWord::from(INSTR3);
        assert_eq!(word3.get_opcode(), Opcode::ADD);
        assert_eq!(word3.get_target(), 0x0);
        assert_eq!(word3.get_op_a(), 0x1);
        assert_eq!(word3.get_op_b(), 0x2);

        let op3 = Operation::from(word3);
        assert_eq!(
            op3,
            Operation::Add(BinaryOp {
                target: 0x0,
                source_a: 0x1,
                source_b: 0x2
            })
        );

        let word4 = InstructionWord::from(INSTR4);
        assert_eq!(word4.get_opcode(), Opcode::ADD3);
        assert_eq!(word4.get_target(), 0x3);
        assert_eq!(word4.get_op_a(), 0x4);
        assert_eq!(word4.get_op_b(), 0x5);
        assert_eq!(word4.get_op_c(), 0x2);

        let op4 = Operation::from(word4);
        assert_eq!(
            op4,
            Operation::Add3(TernaryOp {
                target: 0x3,
                source_a: 0x4,
                source_b: 0x5,
                source_c: 0x2
            })
        );

        let word5 = InstructionWord::from(INSTR5);
        assert_eq!(word5.get_opcode(), Opcode::SUB);
        assert_eq!(word5.get_target(), 0x0);
        assert_eq!(word5.get_op_a(), 0x4);
        assert_eq!(word5.get_op_b(), 0x5);

        let op5 = Operation::from(word5);
        assert_eq!(
            op5,
            Operation::Sub(BinaryOp {
                target: 0x0,
                source_a: 0x4,
                source_b: 0x5
            })
        );

        let word6 = InstructionWord::from(INSTR6);
        assert_eq!(word6.get_opcode(), Opcode::SUBC);
        assert_eq!(word6.get_target(), 0x2);
        assert_eq!(word6.get_op_a(), 0x1);
        assert_eq!(word6.get_op_b(), 0x4);

        let op6 = Operation::from(word6);
        assert_eq!(
            op6,
            Operation::SubCarry(BinaryOp {
                target: 0x2,
                source_a: 0x1,
                source_b: 0x4
            })
        );

        let word7 = InstructionWord::from(INSTR7);
        assert_eq!(word7.get_opcode(), Opcode::LDC);
        assert_eq!(word7.get_load_address(), 0x5);
        assert_eq!(word7.get_constant16(), 0x7832);

        let op7 = Operation::from(word7);
        assert_eq!(
            op7,
            Operation::Load {
                target_register: 0x5,
                source: LoadSource::Constant(0x7832),
            }
        );

        let word8 = InstructionWord::from(INSTR8);
        assert_eq!(word8.get_opcode(), Opcode::OR);
        assert_eq!(word8.get_target(), 0x0);
        assert_eq!(word8.get_op_a(), 0x1);
        assert_eq!(word8.get_op_b(), 0x4);

        let op8 = Operation::from(word8);
        assert_eq!(
            op8,
            Operation::Or(BinaryOp {
                target: 0x0,
                source_a: 0x1,
                source_b: 0x4
            })
        );

        let word9 = InstructionWord::from(INSTR9);
        assert_eq!(word9.get_opcode(), Opcode::NOP);

        let op9 = Operation::from(word9);
        assert_eq!(op9, Operation::Noop);

        let word10 = InstructionWord::from(INSTR10);
        assert_eq!(word10.get_opcode(), Opcode::JZ);
        assert_eq!(word10.get_op_a(), 0x2);

        let op10 = Operation::from(word10);
        assert_eq!(
            op10,
            Operation::Jump {
                target: JumpTarget::AbsoluteAdressRegister(0x2),
                condition: JumpCondition::Zero,
            }
        );

        let word11 = InstructionWord::from(INSTR11);
        assert_eq!(word11.get_opcode(), Opcode::JMP);
        assert_eq!(word11.get_op_a(), 0x3);

        let op11 = Operation::from(word11);
        assert_eq!(
            op11,
            Operation::Jump {
                target: JumpTarget::AbsoluteAdressRegister(0x3),
                condition: JumpCondition::Always
            }
        );

        let word12 = InstructionWord::from(INSTR12);
        assert_eq!(word12.get_opcode(), Opcode::JCR);
        assert_eq!(word12.get_constant12(), 4);
        // The argument is 5, but actually we only jump
        // 4 steps due to program counter incrementing anyway

        let op12 = Operation::from(word12);
        assert_eq!(
            op12,
            Operation::Jump {
                target: JumpTarget::AddressOffsetConstant(0x4),
                condition: JumpCondition::Carry
            }
        );

        // The encodings of LD and ST operands are super confusing
        // but I think what I did here should be correct
        let word13 = InstructionWord::from(INSTR13);
        assert_eq!(word13.get_opcode(), Opcode::ST);
        assert_eq!(word13.get_op_a(), 0x1); // Data register
        assert_eq!(word13.get_op_b(), 0x3); // Address register

        let op13 = Operation::from(word13);
        assert_eq!(
            op13,
            Operation::Store {
                address_register: 0x3,
                data_register: 0x1
            }
        );

        let word14 = InstructionWord::from(INSTR14);
        assert_eq!(word14.get_opcode(), Opcode::LD);
        assert_eq!(word14.get_target(), 0x2); // Target register
        assert_eq!(word14.get_op_b(), 0x5); // Source address register

        let op14 = Operation::from(word14);
        assert_eq!(
            op14,
            Operation::Load {
                target_register: 0x2,
                source: LoadSource::RAM {
                    address_register: 0x5
                }
            }
        );

        let wordh = InstructionWord::from(INSTRH);
        assert_eq!(wordh.get_opcode(), Opcode::HLT);

        let oph = Operation::from(wordh);
        assert_eq!(oph, Operation::Halt);
    }
}
