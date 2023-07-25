use std::fmt::Display;

pub struct InstructionWord {
    buffer: u32, // A bitfield, essentially. We only need 20 bits but this is still the easiest way
                 // to do it
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

    fn get_opcode(&self) -> u32 {
        // Since the lower 4 bits of 16 bit constants in LDC (load constant)
        // operations are stored in the lower 4 bits of the opcode,
        // and all opcodes above 0x80 are also just LDC ops, we might
        // as well clamp it here, because why not.
        self.get_bits(0, 7).unwrap().clamp(0, 0x80)
    }

    fn get_op_a(&self) -> u32 {
        self.get_bits(8, 10).unwrap()
    }

    fn get_op_b(&self) -> u32 {
        self.get_bits(11, 13).unwrap()
    }

    fn get_op_c(&self) -> u32 {
        self.get_bits(14, 16).unwrap()
    }

    fn get_target(&self) -> u32 {
        self.get_bits(17, 19).unwrap()
    }

    fn get_constant12(&self) -> u32 {
        self.get_bits(8, 19).unwrap()
    }

    fn get_constant16(&self) -> u32 {
        // TODO: Figure this out and how it might interfere with get_opcode
        let lower_4: u32 = self.get_bits(0, 3).unwrap();
        let upper_12: u32 = self.get_bits(8, 19).unwrap();

        lower_4 | (upper_12 << 4)
    }

    fn get_load(&self) -> u32 {
        self.get_bits(7, 7).unwrap()
    }

    fn get_load_address(&self) -> u32 {
        self.get_bits(4, 6).unwrap()
    }
}

impl Display for InstructionWord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#b}", self.buffer)
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
    const INSTRH: u32 = 0x0007f; // halt

    #[test]
    fn instructionword_getter_test() {
        let word2 = InstructionWord::from(INSTR2);
        assert_eq!(word2.get_opcode(), 0x5);
        assert_eq!(word2.get_target(), 0x0);
        assert_eq!(word2.get_op_a(), 0x0);

        let word1 = InstructionWord::from(INSTR1);
        assert_eq!(word1.get_opcode(), 0x80);
        assert_eq!(word1.get_load_address(), 2);
        assert_eq!(word1.get_constant16(), 0x8001);

        let word3 = InstructionWord::from(INSTR3);
        assert_eq!(word3.get_opcode(), 0x0);
        assert_eq!(word3.get_target(), 0x0);
        assert_eq!(word3.get_op_a(), 0x1);
        assert_eq!(word3.get_op_b(), 0x2);

        let word4 = InstructionWord::from(INSTR4);
        assert_eq!(word4.get_opcode(), 0x1);
        assert_eq!(word4.get_target(), 0x3);
        assert_eq!(word4.get_op_a(), 0x4);
        assert_eq!(word4.get_op_b(), 0x5);
        assert_eq!(word4.get_op_c(), 0x2);

        let word5 = InstructionWord::from(INSTR5);
        assert_eq!(word5.get_opcode(), 0x3);
        assert_eq!(word5.get_target(), 0x0);
        assert_eq!(word5.get_op_a(), 0x4);
        assert_eq!(word5.get_op_b(), 0x5);

        let word6 = InstructionWord::from(INSTR6);
        assert_eq!(word6.get_opcode(), 0x4);
        assert_eq!(word6.get_target(), 0x2);
        assert_eq!(word6.get_op_a(), 0x1);
        assert_eq!(word6.get_op_b(), 0x4);

        let word7 = InstructionWord::from(INSTR7);
        assert_eq!(word7.get_opcode(), 0x80);
        assert_eq!(word7.get_load_address(), 0x5);
        assert_eq!(word7.get_constant16(), 0x7832);

        let word8 = InstructionWord::from(INSTR8);
        assert_eq!(word8.get_opcode(), 0xa);
        assert_eq!(word8.get_target(), 0x0);
        assert_eq!(word8.get_op_a(), 0x1);
        assert_eq!(word8.get_op_b(), 0x4);

        let word9 = InstructionWord::from(INSTR9);
        assert_eq!(word9.get_opcode(), 0x6c);

        let word10 = InstructionWord::from(INSTR10);
        assert_eq!(word10.get_opcode(), 0x51);
        assert_eq!(word10.get_op_a(), 0x2);

        let word11 = InstructionWord::from(INSTR11);
        assert_eq!(word11.get_opcode(), 0x50);
        assert_eq!(word11.get_op_a(), 0x3);

        let word12 = InstructionWord::from(INSTR12);
        assert_eq!(word12.get_opcode(), 0x5B);
        assert_eq!(word12.get_constant12(), 4);
        // The argument is 5, but actually we enoly jumo
        // 4 steps due to program counter incrementing anyway

        let wordh = InstructionWord::from(INSTRH);
        assert_eq!(wordh.get_opcode(), 0x7f);
    }
}
