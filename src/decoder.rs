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
