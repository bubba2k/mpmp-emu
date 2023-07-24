use std::fmt::Display;

pub struct InstructionWord {
    buffer: [bool; 20],
}

impl From<[u8; 3]> for InstructionWord {
    fn from(value: [u8; 3]) -> Self {
        let mut buffer: [bool; 20] = [false; 20];

        for (byte_idx, &byte) in value.iter().enumerate() {
            let bit_mask: u8 = 0x1u8;

            for bit_idx in 0..=7 {
                buffer[byte_idx * 8 + bit_idx] = ((byte >> bit_idx) & bit_mask) != 0
            }
        }

        InstructionWord { buffer }
    }
}

impl InstructionWord {
    // Get the indexed bits and return them interpreted as an integer
    fn get_bits(&self, upper: u32, lower: u32) -> Result<u32, &'static str> {
        if lower >= upper {
            return Err("Invalid bounds");
        }

        let mut res_val: u32 = 0;
        for bit_idx in upper..=lower {
            res_val &= (self.buffer[bit_idx as usize] as u32) << (bit_idx - lower);
        }

        Ok(res_val)
    }
}

impl Display for InstructionWord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str: String = String::from("");
        for val in self.buffer {
            match val {
                true => str.push('1'),
                false => str.push('0'),
            }
        }

        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BYTES1: [u8; 3] = [0xff, 0xff, 0xff];

    #[test]
    fn instructionword_construction() {
        let word = InstructionWord::from(BYTES1);
        assert_eq!(true, true);
    }
}
