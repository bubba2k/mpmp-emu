use super::ir::*;
use super::program::Program;

#[derive(Debug, PartialEq, Eq)]
pub struct Flags {
    pub carry: bool,
    pub overflow: bool,
    pub zero: bool,
}

type Registers = [u16; 6]; // the 6 registers

const RAM_SIZE: usize = 32_768; // 2^15
type Ram = [u16; RAM_SIZE];

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
    pub received_halt: bool,

    pub istream: IOStream,
    pub ostream: IOStream,
}

impl Default for CpuState {
    fn default() -> Self {
        CpuState {
            registers: [0u16; 6],
            flags: Flags {
                carry: false,
                overflow: false,
                zero: false,
            },
            ram: [0u16; RAM_SIZE],
            pmem: [0u32; PMEM_SIZE],
            received_halt: false,
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
    fn update_zero_flag(&mut self, res: u16) {
        self.flags.zero = res == 0;
    }

    pub fn execute_next_prog_op(&mut self, prog: &Program) {
        self.execute_operation(&prog.operations[self.pcounter as usize])
    }

    pub fn execute_operation(&mut self, op: &Operation) {
        match op {
            Operation::Halt => self.received_halt = true,
            Operation::Noop => {}
            Operation::Inc(op) => {
                let res = self.registers[op.source_a].overflowing_add(1);
                self.registers[op.source_a] = res.0;
                self.flags.carry = res.1;
                self.update_zero_flag(self.registers[op.source_a]);
            }
            Operation::Dec(op) => {
                let res = self.registers[op.source_a].overflowing_sub(1);
                self.registers[op.source_a] = res.0;
                self.flags.carry = res.1;
                self.update_zero_flag(self.registers[op.source_a]);
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
                    self.registers[op.source_a] >> self.registers[op.source_b];
                self.update_zero_flag(self.registers[op.target]);
            }
            Operation::Neg(op) => {
                self.registers[op.target] = !(self.registers[op.source_a]) + 0x1;
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
                self.registers[*target_register] = *data;
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
                        self.pcounter = self.pcounter.wrapping_add_signed(*offset)
                    }
                }
                JumpCondition::NotZero => {
                    if !self.flags.zero {
                        self.pcounter = self.pcounter.wrapping_add_signed(*offset)
                    }
                }
                JumpCondition::Carry => {
                    if self.flags.carry {
                        self.pcounter = self.pcounter.wrapping_add_signed(*offset)
                    }
                }
                JumpCondition::NotCarry => {
                    if !self.flags.carry {
                        self.pcounter = self.pcounter.wrapping_add_signed(*offset)
                    }
                }
                JumpCondition::Always => self.pcounter = self.pcounter.wrapping_add_signed(*offset),
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

        self.pcounter += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::super::program::Program;
    use super::CpuState;

    /*
    ldc %reg0 0x5
    ldc %reg1 0x1
    ldc %reg2 0x80
    add %reg3 %reg1 %reg1
    sub %reg3 %reg0 %reg1
    inc %reg3
    mov %reg3 %reg0
    tst %reg1 %reg2
    add3 %reg3 %reg0 %reg1 %reg2
    shl %reg3 %reg3 %reg1
    shr %reg3 %reg3 %reg1
    dec %reg3
    and %reg4 %reg3 %reg0
    or  %reg4 %reg3 %reg2
    not %reg1 %reg1
    ldc %reg5 0xffff
    inc %reg5
    dec %reg5
    add %reg5 %reg5 %reg1
    sub %reg5 %reg5 %reg1
    */
    const PMEM1: [u32; 21] = [
        0x00085u32, 0x00091u32, 0x008a0u32, 0x60900u32, 0x60803u32, 0x00305u32, 0x60048u32,
        0x01108u32, 0x68801u32, 0x60b0fu32, 0x60b10u32, 0x00306u32, 0x80309u32, 0x8130au32,
        0x2010bu32, 0xfffdfu32, 0x00505u32, 0x00506u32, 0xa0d00u32, 0xa0d03u32, 0x0007fu32,
    ];

    #[test]
    fn alu_tests() {
        let mut cpu = CpuState::default();

        let program1: Program = Program::from(PMEM1.as_slice());

        // ldc %reg0 0x5
        cpu.execute_next_prog_op(&program1);
        assert_eq!(cpu.registers[0], 0x5);
        assert_eq!(cpu.pcounter, 1);
        assert_eq!(cpu.flags.zero, false);
        assert_eq!(cpu.flags.carry, false);

        // ldc %reg1 0x1
        cpu.execute_next_prog_op(&program1);
        assert_eq!(cpu.registers[1], 0x1);

        // ldc %reg2 0x80
        cpu.execute_next_prog_op(&program1);
        assert_eq!(cpu.registers[2], 0x80);

        // add %reg3 %reg1 %reg1
        cpu.execute_next_prog_op(&program1);
        assert_eq!(cpu.registers[3], 0x2);
        assert_eq!(cpu.flags.carry, false);
        assert_eq!(cpu.flags.zero, false);
        assert_eq!(cpu.registers[0], 0x5);
        assert_eq!(cpu.registers[1], 0x1);
        assert_eq!(cpu.registers[4], 0x0);

        // sub %reg3 %reg0 %reg1
        cpu.execute_next_prog_op(&program1);
        assert_eq!(cpu.registers[3], 0x4);
        assert_eq!(cpu.flags.carry, false);
        assert_eq!(cpu.flags.zero, false);

        // inc %reg3
        cpu.execute_next_prog_op(&program1);
        assert_eq!(cpu.registers[3], 0x5);
        assert_eq!(cpu.flags.carry, false);
        assert_eq!(cpu.flags.zero, false);

        // mov %reg3 %reg0
        cpu.execute_next_prog_op(&program1);
        assert_eq!(cpu.registers[3], 0x5);
        assert_eq!(cpu.flags.carry, false);
        assert_eq!(cpu.flags.zero, false);

        // tst %reg1 %reg2
        cpu.execute_next_prog_op(&program1);
        assert_eq!(cpu.flags.carry, true);
        assert_eq!(cpu.flags.zero, false);

        // add3 %reg3 %reg0 %reg1 %reg2
        cpu.execute_next_prog_op(&program1);
        assert_eq!(cpu.registers[3], 0x86);
        assert_eq!(cpu.flags.carry, false);
        assert_eq!(cpu.flags.zero, false);

        // shl %reg3 %reg3 %reg1
        cpu.execute_next_prog_op(&program1);
        assert_eq!(cpu.registers[3], 0x10C);
        assert_eq!(cpu.flags.carry, false);
        assert_eq!(cpu.flags.zero, false);

        // shr %reg3 %reg3 %reg1
        cpu.execute_next_prog_op(&program1);
        assert_eq!(cpu.registers[3], 0x86);
        assert_eq!(cpu.flags.carry, false);
        assert_eq!(cpu.flags.zero, false);

        // dec %reg3
        cpu.execute_next_prog_op(&program1);
        assert_eq!(cpu.registers[3], 0x85);
        assert_eq!(cpu.flags.carry, false);
        assert_eq!(cpu.flags.zero, false);

        // and %reg4 $reg3 %reg0
        cpu.execute_next_prog_op(&program1);
        assert_eq!(cpu.registers[4], 0x85 & 0x5);
        assert_eq!(cpu.flags.carry, false);
        assert_eq!(cpu.flags.zero, false);

        // or %reg4 %reg3 %reg2
        cpu.execute_next_prog_op(&program1);
        assert_eq!(cpu.registers[4], 0x85 | 0x5);
        assert_eq!(cpu.flags.carry, false);
        assert_eq!(cpu.flags.zero, false);

        // not %reg1 %reg1
        cpu.execute_next_prog_op(&program1);
        assert_eq!(cpu.registers[1], !0x1);
        assert_eq!(cpu.flags.carry, false);
        assert_eq!(cpu.flags.zero, false);

        // ldc %reg5 0xffff
        cpu.execute_next_prog_op(&program1);
        assert_eq!(cpu.registers[5], 0xffff);
        assert_eq!(cpu.flags.carry, false);
        assert_eq!(cpu.flags.zero, false);

        // inc %reg5
        cpu.execute_next_prog_op(&program1);
        assert_eq!(cpu.registers[5], 0x0);
        assert_eq!(cpu.flags.carry, true);
        assert_eq!(cpu.flags.zero, true);

        // dec %reg5
        cpu.execute_next_prog_op(&program1);
        assert_eq!(cpu.registers[5], 0xffff);
        assert_eq!(cpu.flags.carry, true);
        assert_eq!(cpu.flags.zero, false);

        // add %reg5 %reg5 %reg1
        cpu.execute_next_prog_op(&program1);
        assert_eq!(cpu.registers[5], 0xfffd);
        assert_eq!(cpu.flags.carry, true);
        assert_eq!(cpu.flags.zero, false);

        // sub %reg5 %reg5 %reg1
        cpu.execute_next_prog_op(&program1);
        assert_eq!(cpu.registers[5], 0xffff);
        assert_eq!(cpu.flags.carry, true);
        assert_eq!(cpu.flags.zero, false);
    }

    /*
    # Compute the fibonacci numbers
    # Result is stored in %reg5
    main: # Setup
      ldc %reg0 1
      ldc %reg1 1
      ldc %reg2 3
    loop:
      add %reg0 %reg0 %reg1
      mov %reg5 %reg0
      dec %reg2
      jzr end
      add %reg1 %reg0 %reg1
      mov %reg5 %reg1
      dec %reg2
      jnzr loop
    end:
      hlt
    */
    const PMEM2: [u32; 12] = [
        0x00081u32, 0x00091u32, 0x000a3u32, 0x00800u32, 0xa0048u32, 0x00206u32, 0x00459u32,
        0x20800u32, 0xa0148u32, 0x00206u32, 0xff85au32, 0x0007fu32,
    ];

    /*
    # Compute the 15th fib number...
    # Result is stored in %reg5
    # Should be 233

    main: # Setup
      ldc %reg0 1
      ldc %reg1 1
      ldc %reg2 11
    loop:
      add %reg0 %reg0 %reg1
      mov %reg5 %reg0
      dec %reg2
      jzr end
      add %reg1 %reg0 %reg1
      mov %reg5 %reg1
      dec %reg2
      jnzr loop
    end:
      hlt

    */
    const PMEM3: [u32; 12] = [
        0x00081u32, 0x00091u32, 0x000abu32, 0x00800u32, 0xa0048u32, 0x00206u32, 0x00459u32,
        0x20800u32, 0xa0148u32, 0x00206u32, 0xff85au32, 0x0007fu32,
    ];

    #[test]
    fn misc_program_tests() {
        // Fibonacci
        let mut cpu = CpuState::default();
        let program2 = Program::from(PMEM2.as_slice());

        while (!cpu.received_halt) {
            cpu.execute_next_prog_op(&program2)
        }

        assert_eq!(cpu.registers[5], 0x5);
        assert_eq!(cpu.registers[2], 0x0);
        assert_eq!(cpu.registers[0], 0x5);
        assert_eq!(cpu.registers[1], 0x3);

        // Another fibonacci
        cpu = CpuState::default();
        let program3 = Program::from(PMEM3.as_slice());

        while (!cpu.received_halt) {
            cpu.execute_next_prog_op(&program3)
        }

        assert_eq!(cpu.registers[5], 233);
        assert_eq!(cpu.registers[2], 0x0);
    }
}
