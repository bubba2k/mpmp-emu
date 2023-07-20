struct Flags {
    pub carry: bool,
    pub overflow: bool,
    pub zero: bool,
}

type Registers = [i16; 6]; // the 6 registers

const RAM_SIZE: u32 = 32_768; // 2^15
type RAM = [i16; RAM_SIZE];

const PMEM_SIZE: u32 = 65_536; // 2^16
type PMEM = [u32; PMEM_SIZE];
