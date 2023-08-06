use std::mem::transmute;

use super::ir::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Flags {
    pub carry: bool,
    pub overflow: bool,
    pub zero: bool,
}

type Registers = [i16; 6]; // the 6 registers

const RAM_SIZE: usize = 32_768; // 2^15
type Ram = [i16; RAM_SIZE];

const PMEM_SIZE: usize = 65_536; // 2^16
type Pmem = [u32; PMEM_SIZE];

#[derive(Debug, PartialEq, Eq)]
pub struct IOStream {
    pub string: String,
}

impl IOStream {
    pub fn clear(&mut self) {
        self.string.clear();
    }

    pub fn append_char(&mut self, ch: char) {
        self.string += &ch.to_string();
    }

    pub fn consume_first(&mut self) -> char {
        let res = self.string.chars().next();

        match res {
            Some(ch) => {
                self.string.remove(0);
                ch
            }
            None => char::from_u32(0).unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CpuState {
    pub registers: Registers,
    pub flags: Flags,
    pub ram: Ram,
    pub pmem: Pmem,
    pub pcounter: u16,
    pub running: bool,

    pub istream: IOStream,
    pub ostream: IOStream,
}

impl Default for CpuState {
    fn default() -> Self {
        CpuState {
            registers: [0i16; 6],
            flags: Flags {
                carry: false,
                overflow: false,
                zero: false,
            },
            ram: [0i16; RAM_SIZE],
            pmem: [0u32; PMEM_SIZE],
            running: false,
            pcounter: 0,

            istream: IOStream {
                string: String::new(),
            },
            ostream: IOStream {
                string: String::new(),
            },
        }
    }
}

// TODO: Test this!
impl CpuState {
    // Inspect the supplied value and update zero flag accordingly
    fn update_zero_flag(&mut self, res: i16) {
        self.flags.zero = res == 0;
    }

    pub fn execute_operation(&mut self, op: &Operation) {
        match op {
            Operation::Halt => self.running = false,
            Operation::Noop => {}
            Operation::Inc(op) => {
                let res = self.registers[op.source_a].overflowing_add(1);
                self.registers[op.target] = res.0;
                self.flags.carry = res.1;
                self.update_zero_flag(self.registers[op.target]);
            }
            Operation::Dec(op) => {
                let res = self.registers[op.source_a].overflowing_sub(1);
                self.registers[op.target] = res.0;
                self.flags.carry = res.1;
                self.update_zero_flag(self.registers[op.target]);
            }
            Operation::Not(op) => {
                self.registers[op.target] = !self.registers[op.source_a];
                self.update_zero_flag(self.registers[op.target]);
            }
            Operation::Or(op) => {
                self.registers[op.target] =
                    self.registers[op.source_a] | self.registers[op.source_b];
                self.update_zero_flag(self.registers[op.target]);
            }
            Operation::And(op) => {
                self.registers[op.target] =
                    self.registers[op.source_a] & self.registers[op.source_b];
                self.update_zero_flag(self.registers[op.target]);
            }
            Operation::Xor(op) => {
                self.registers[op.target] =
                    self.registers[op.source_a] ^ self.registers[op.source_b];
                self.update_zero_flag(self.registers[op.target]);
            }
            Operation::Xnor(op) => {
                // `!` is supposedly the bitwise NOT operator...
                self.registers[op.target] =
                    !(self.registers[op.source_a] ^ self.registers[op.source_b]);
                self.update_zero_flag(self.registers[op.target]);
            }
            Operation::ShiftLeft(op) => {
                self.registers[op.target] =
                    self.registers[op.source_a] << self.registers[op.source_b];
                self.update_zero_flag(self.registers[op.target]);
            }
            Operation::ShiftRight(op) => {
                self.registers[op.target] =
                    self.registers[op.source_a] << self.registers[op.source_b];
                self.update_zero_flag(self.registers[op.target]);
            }
            Operation::Neg(op) => {
                self.registers[op.target] = -(self.registers[op.source_a]);
                self.update_zero_flag(self.registers[op.target]);
            }
            Operation::Add(op) => {
                let res = self.registers[op.source_a].overflowing_add(self.registers[op.source_b]);
                self.registers[op.target] = res.0;
                self.flags.carry = res.1;
                self.update_zero_flag(self.registers[op.target]);
            }
            Operation::AddCarry(op) => {
                let res_a =
                    self.registers[op.source_a].overflowing_add(self.registers[op.source_b]);
                let res_b = res_a.0.overflowing_add(match self.flags.carry {
                    true => 1,
                    false => 0,
                });
                self.registers[op.target] = res_b.0;

                self.flags.carry = res_a.1 || res_b.1;
                self.update_zero_flag(self.registers[op.target]);
            }
            Operation::Add3(op) => {
                let res_a =
                    self.registers[op.source_a].overflowing_add(self.registers[op.source_b]);
                let res_b = res_a.0.overflowing_add(self.registers[op.source_c]);
                self.registers[op.target] = res_b.0;

                self.flags.carry = res_a.1 || res_b.1;
                self.update_zero_flag(self.registers[op.target]);
            }
            Operation::Sub(op) => {
                let res = self.registers[op.source_a].overflowing_sub(self.registers[op.source_b]);
                self.registers[op.target] = res.0;

                self.flags.carry = res.1;
                self.update_zero_flag(self.registers[op.target]);
            }
            // TODO: This implementation might be wrong
            Operation::SubCarry(op) => {
                let res_a =
                    self.registers[op.source_a].overflowing_sub(self.registers[op.source_b]);
                let res_b = res_a.0.overflowing_sub(match self.flags.carry {
                    true => 1,
                    false => 0,
                });
                self.registers[op.target] = res_b.0;

                self.flags.carry = res_a.1 || res_b.1;
                self.update_zero_flag(self.registers[op.target]);
            }
            Operation::Multiply(op) => {
                // Technically, the "real" CPU would only implement 8bit*8bit multiplication
                // in order to avoid having to deal with overflows. We will just go ahead and
                // ignore that limitation. (Might be a bad idea)
                self.registers[op.target] =
                    self.registers[op.source_a] * self.registers[op.source_b];
                self.update_zero_flag(self.registers[op.target]);
            }
            Operation::Move(op) => {
                self.registers[op.target] = self.registers[op.source_a];
            }

            Operation::Test(op) => {
                let res = self.registers[op.source_a].overflowing_sub(self.registers[op.source_b]);
                self.flags.carry = res.1;
                self.update_zero_flag(res.0);
            }

            // TODO: Implement the fancy memory mapped IO stuff here
            Operation::Load {
                target_register,
                source: LoadSource::RAM { address_register },
            } => {
                self.registers[*target_register] =
                    self.ram[self.registers[*address_register] as usize];
            }
            Operation::Load {
                target_register,
                source: LoadSource::Constant(data),
            } => {
                // We don't want to fool around with the bits, just copy them
                self.registers[*target_register] = unsafe { std::mem::transmute(*data) };
            }

            Operation::Store {
                address_register,
                data_register,
            } => {
                self.ram[self.registers[*address_register] as usize] =
                    self.registers[*data_register];
            }

            // Relative jumps
            Operation::Jump {
                target: JumpTarget::AddressOffsetConstant(offset),
                condition,
            } => match condition {
                JumpCondition::Zero => {
                    if self.flags.zero {
                        self.pcounter += *offset
                    }
                }
                JumpCondition::NotZero => {
                    if !self.flags.zero {
                        self.pcounter += offset
                    }
                }
                JumpCondition::Carry => {
                    if self.flags.carry {
                        self.pcounter += offset
                    }
                }
                JumpCondition::NotCarry => {
                    if !self.flags.carry {
                        self.pcounter += offset
                    }
                }
                JumpCondition::Always => self.pcounter += offset,
            },

            // Absolute jumps
            Operation::Jump {
                target: JumpTarget::AbsoluteAdressRegister(address_register),
                condition,
            } => match condition {
                JumpCondition::Zero => {
                    if self.flags.zero {
                        self.pcounter = self.registers[*address_register as usize] as u16
                    }
                }
                JumpCondition::NotZero => {
                    if !self.flags.zero {
                        self.pcounter = self.registers[*address_register as usize] as u16
                    }
                }
                JumpCondition::Carry => {
                    if self.flags.carry {
                        self.pcounter = self.registers[*address_register as usize] as u16
                    }
                }
                JumpCondition::NotCarry => {
                    if !self.flags.carry {
                        self.pcounter = self.registers[*address_register as usize] as u16
                    }
                }
                JumpCondition::Always => {
                    self.pcounter = self.registers[*address_register as usize] as u16
                }
            },
        }
    }
}
