//! A flat, little-endian view of the Emotion Engine address space.
//!
//! This is deliberately simple and allocation-light: it backs main RAM,
//! scratchpad and IOP RAM with `Vec<u8>` and folds the cached/uncached aliases
//! of RAM onto a single buffer. Anything that falls into a device window is
//! handled by a stub so the machine can boot without panicking; real device
//! behaviour is filled in by the runtime crate.

use crate::memory_map::{self as map, device};

const RAM_MASK: usize = map::MAIN_RAM_SIZE - 1;
const SP_MASK: usize = device::SCRATCHPAD_SIZE - 1;

/// The EE's addressable state that the CPU core actually needs.
pub struct Memory {
    /// 32 MiB of main RDRAM.
    pub ram: Vec<u8>,
    /// 16 KiB scratchpad.
    pub scratchpad: Vec<u8>,
    /// IOP RAM as seen through the EE window (only the low 2 MiB is real).
    pub iop_ram: Vec<u8>,
    /// Number of CPU accesses that landed in a device window with no model yet.
    pub unmapped_reads: u64,
    pub unmapped_writes: u64,
    /// Most recent address that hit an unmodelled device window, for debugging.
    pub last_device_addr: Option<u32>,
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    pub fn new() -> Self {
        Self {
            ram: vec![0; map::MAIN_RAM_SIZE],
            scratchpad: vec![0; device::SCRATCHPAD_SIZE],
            iop_ram: vec![0; device::IOP_RAM_SIZE],
            unmapped_reads: 0,
            unmapped_writes: 0,
            last_device_addr: None,
        }
    }

    /// Translate a virtual/physical EE address to `(region, offset)`, folding
    /// the cached/uncached RAM aliases onto the backing buffers.
    fn resolve(addr: u32) -> Region {
        use map::{MAIN_RAM_BASE, MAIN_RAM_SIZE, MAIN_RAM_UNCACHED, MAIN_RAM_UNCACHED_ACCEL};

        if (MAIN_RAM_BASE..MAIN_RAM_BASE + MAIN_RAM_SIZE as u32).contains(&addr) {
            return Region::Ram(addr as usize);
        }
        if (MAIN_RAM_UNCACHED..MAIN_RAM_UNCACHED + MAIN_RAM_SIZE as u32).contains(&addr) {
            return Region::Ram((addr - MAIN_RAM_UNCACHED) as usize);
        }
        // The accelerated alias is a full 32 MiB mirror starting at 0x3000_0000.
        if (MAIN_RAM_UNCACHED_ACCEL..MAIN_RAM_UNCACHED_ACCEL + MAIN_RAM_SIZE as u32)
            .contains(&addr)
        {
            return Region::Ram((addr - MAIN_RAM_UNCACHED_ACCEL) as usize);
        }
        if (device::SCRATCHPAD_BASE..device::SCRATCHPAD_BASE + device::SCRATCHPAD_SIZE as u32)
            .contains(&addr)
        {
            return Region::Scratchpad((addr - device::SCRATCHPAD_BASE) as usize);
        }
        if (device::IOP_RAM_BASE..device::IOP_RAM_END + 1).contains(&addr) {
            let off = (addr - device::IOP_RAM_BASE) as usize % device::IOP_RAM_SIZE;
            return Region::IopRam(off);
        }
        Region::Device(addr)
    }

    /// Read four bytes starting at the 4-byte-aligned `addr`.
    pub fn read_u32(&mut self, addr: u32) -> u32 {
        let a = addr & !3;
        let mut buf = [0u8; 4];
        match Self::resolve(a) {
            Region::Ram(o) => {
                for (i, b) in buf.iter_mut().enumerate() {
                    *b = self.ram[(o + i) & RAM_MASK];
                }
            }
            Region::Scratchpad(o) => {
                for (i, b) in buf.iter_mut().enumerate() {
                    *b = self.scratchpad[(o + i) & SP_MASK];
                }
            }
            Region::IopRam(o) => {
                for (i, b) in buf.iter_mut().enumerate() {
                    *b = self.iop_ram[(o + i) % device::IOP_RAM_SIZE];
                }
            }
            Region::Device(d) => {
                self.last_device_addr = Some(d);
                self.unmapped_reads += 1;
                return 0;
            }
        }
        u32::from_le_bytes(buf)
    }

    /// Write four bytes starting at the 4-byte-aligned `addr`.
    pub fn write_u32(&mut self, addr: u32, val: u32) {
        let a = addr & !3;
        let bytes = val.to_le_bytes();
        match Self::resolve(a) {
            Region::Ram(o) => {
                for (i, b) in bytes.iter().enumerate() {
                    self.ram[(o + i) & RAM_MASK] = *b;
                }
            }
            Region::Scratchpad(o) => {
                for (i, b) in bytes.iter().enumerate() {
                    self.scratchpad[(o + i) & SP_MASK] = *b;
                }
            }
            Region::IopRam(o) => {
                for (i, b) in bytes.iter().enumerate() {
                    self.iop_ram[(o + i) % device::IOP_RAM_SIZE] = *b;
                }
            }
            Region::Device(d) => {
                self.last_device_addr = Some(d);
                self.unmapped_writes += 1;
            }
        }
    }

    pub fn read_u8(&mut self, addr: u32) -> u8 {
        (self.read_u32(addr) >> ((addr & 3) * 8)) as u8
    }

    pub fn read_u16(&mut self, addr: u32) -> u16 {
        (self.read_u32(addr) >> ((addr & 3) * 8)) as u16
    }

    pub fn write_u8(&mut self, addr: u32, val: u8) {
        let shift = (addr & 3) * 8;
        let cur = self.read_u32(addr);
        let merged = (cur & !(0xff << shift)) | ((val as u32) << shift);
        self.write_u32(addr, merged);
    }

    pub fn write_u16(&mut self, addr: u32, val: u16) {
        let shift = (addr & 3) * 8;
        let cur = self.read_u32(addr);
        let merged = (cur & !(0xffff << shift)) | ((val as u32) << shift);
        self.write_u32(addr, merged);
    }

    /// Copy a block of bytes into memory at `addr` (used when laying down a
    /// loaded ELF image). Does not count as a CPU device access.
    pub fn load(&mut self, addr: u32, bytes: &[u8]) {
        for (i, b) in bytes.iter().enumerate() {
            self.store_byte(addr.wrapping_add(i as u32), *b);
        }
    }

    fn store_byte(&mut self, addr: u32, val: u8) {
        match Self::resolve(addr) {
            Region::Ram(o) => self.ram[o & RAM_MASK] = val,
            Region::Scratchpad(o) => self.scratchpad[o & SP_MASK] = val,
            Region::IopRam(o) => self.iop_ram[o] = val,
            Region::Device(_) => {}
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Region {
    Ram(usize),
    Scratchpad(usize),
    IopRam(usize),
    Device(u32),
}
