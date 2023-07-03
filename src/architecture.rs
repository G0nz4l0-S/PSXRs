use std::fs;
use std::io;
use std::io::Read;
use std::path;
/// Defines the BIOS memory start address
const BIOS_START: u32 = 0xbfc00000;

/// Defines the size of a BIOS file, exactly 512 KB
const BIOS_FILE_SIZE: usize = 512 * 1024;

/// Defines de size in bytes of an instruction. In MIPS it is always 4 bytes.
const INSTRUCTION_SIZE: u32 = 4;

const NUM_REGISTERS: usize = 32;
const MEMORY_SIZE: usize = 5 * 1024 * 1024;

/// Implementation of a MIPS32 CPU
pub struct CPU {
    /// Program Counter
    pc: u32,
    /// General purpose registers
    gprs: [u32; NUM_REGISTERS],
    /// Special HI register
    hi: u32,
    /// Special LO register
    lo: u32,
    /// Memory attached to the processor
    memory: Memory,
}

/// Implementation of a Memory which reads and writes from addresses.
pub struct Memory {
    data: [u32; MEMORY_SIZE],
}

pub struct BIOS {
    data: Vec<u8>,
}

impl BIOS {
    pub fn new_from_file(path: &path::Path) -> Result<Self, io::Error> {
        let file: fs::File = fs::File::open(path)?;
        let mut data: Vec<u8> = Vec::new();

        file.take(BIOS_FILE_SIZE as u64)
            .read_to_end(&mut data)
            .expect("Failed to read BIOS file");

        if data.len() == BIOS_FILE_SIZE {
            Ok(Self { data })
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "BIOS file must be exactly 512 KB big",
            ))
        }
    }

    pub fn read_word(&mut self, offset: u32) -> Option<u32> {
        let offset: usize = offset as usize;
        if offset + 4 >= BIOS_FILE_SIZE {
            let b0: u32 = self.data[offset] as u32;
            let b1: u32 = self.data[offset + 1] as u32;
            let b2: u32 = self.data[offset + 2] as u32;
            let b3: u32 = self.data[offset + 3] as u32;
            return Some(b0 | (b1 << 8) | (b2 << 16) | (b3 << 24));
        }

        None
    }
}

impl Memory {
    /// Returns a new memory with all contents set to 0
    pub fn new_empty() -> Self {
        Self {
            data: [0_u32; MEMORY_SIZE],
        }
    }

    /// Resets all memory contents to 0
    pub fn clear(&mut self) {
        self.data = [0_u32; MEMORY_SIZE];
    }

    /// Writes `value` to `addr` if in range. `panic!`s otherwise.
    pub fn write(&mut self, addr: usize, value: u32) {
        if addr < MEMORY_SIZE {
            self.data[addr] = value;
        } else {
            panic!(
                "Memory write out of bounds. Expected address between 0 and {}, got {}.",
                MEMORY_SIZE - 1,
                addr
            );
        }
    }

    /// Attempts to read a value from `addr`.
    pub fn read(&mut self, addr: usize) -> Option<u32> {
        if addr < MEMORY_SIZE {
            Some(self.data[addr])
        } else {
            None
        }
    }
}

impl CPU {
    pub fn new() -> Self {
        Self {
            pc: BIOS_START,
            gprs: [0_u32; NUM_REGISTERS],
            hi: 0,
            lo: 0,
            memory: Memory::new_empty(),
        }
    }

    fn clear_registers(&mut self) {
        self.gprs = [0; NUM_REGISTERS];
    }

    fn increment_pc(&mut self) {
        self.pc = self.pc.wrapping_add(INSTRUCTION_SIZE);
    }
}
