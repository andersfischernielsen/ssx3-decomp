//! The EE physical bus: main RAM, scratchpad, and memory-mapped hardware.
//!
//! Virtual → physical: the game runs with the standard PS2 kernel TLB setup,
//! so `0x0000_0000..0x01FF_FFFF` is RAM, `0x2000_0000`/`0x3000_0000` are the
//! uncached aliases, `0x7000_0000` is the scratchpad, and kseg0/kseg1
//! (`0x8000_0000`/`0xA000_0000`) map RAM and the I/O space directly.
//! I/O goes to the [`Platform`].

use crate::platform::{NullPlatform, Platform};

pub const RAM_SIZE: usize = 32 * 1024 * 1024;
pub const SPR_SIZE: usize = 16 * 1024;

pub struct Bus {
    pub ram: Vec<u8>,
    /// Test builds only (`dirty-pages`): one bit per 4 KiB RAM page written
    /// through the bus, so differential tests compare only touched pages.
    #[cfg(feature = "dirty-pages")]
    pub dirty: Vec<u64>,
    pub spr: Vec<u8>,
    /// IOP RAM (2 MiB), visible to the EE at 0x1C00_0000.
    /// The hardware model behind memory-mapped I/O (and VU0, see
    /// [`Cpu::vu0`](crate::Cpu::vu0)).
    pub plat: Box<dyn Platform>,
    /// Accesses to unmodelled addresses, for diagnostics.
    pub unmapped: u64,
    pub last_unmapped: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Target {
    Ram(usize),
    Spr(usize),
    Io(u32),
    None,
}

#[inline(always)]
fn target(addr: u32) -> Target {
    if addr < 0x0200_0000 {
        return Target::Ram(addr as usize);
    }
    match addr >> 28 {
        0x2 | 0x3 if addr & 0x0FFF_FFFF < 0x0200_0000 => Target::Ram((addr & 0x01FF_FFFF) as usize),
        0x7 => Target::Spr((addr & 0x3FFF) as usize),
        0x8 | 0x9 | 0xA | 0xB => {
            let p = addr & 0x1FFF_FFFF;
            if p < 0x0200_0000 {
                Target::Ram(p as usize)
            } else {
                Target::Io(p)
            }
        }
        0x1 => Target::Io(addr),
        _ => Target::None,
    }
}

macro_rules! rd {
    ($buf:expr, $o:expr, $t:ty, $n:expr) => {{
        let o = $o;
        <$t>::from_le_bytes($buf[o..o + $n].try_into().unwrap())
    }};
}

impl Default for Bus {
    fn default() -> Self {
        Self::new(Box::new(NullPlatform::default()))
    }
}

impl Bus {
    pub fn new(plat: Box<dyn Platform>) -> Self {
        Self {
            ram: vec![0; RAM_SIZE],
            #[cfg(feature = "dirty-pages")]
            dirty: vec![0; RAM_SIZE / 4096 / 64],
            spr: vec![0; SPR_SIZE],
            plat,
            unmapped: 0,
            last_unmapped: 0,
        }
    }

    /// Fast instruction fetch (RAM only; anything else returns a `break`).
    #[inline(always)]
    pub fn fetch(&self, addr: u32) -> u32 {
        match target(addr) {
            Target::Ram(o) => {
                let o = o & (RAM_SIZE - 1) & !3;
                rd!(self.ram, o, u32, 4)
            }
            _ => 0x0000_000D,
        }
    }

    fn miss(&mut self, addr: u32) {
        self.unmapped += 1;
        self.last_unmapped = addr;
    }

    pub fn read8(&mut self, addr: u32) -> u8 {
        match target(addr) {
            Target::Ram(o) => self.ram[o],
            Target::Spr(o) => self.spr[o],
            Target::Io(p) => (self.plat.read32(p & !3) >> ((p & 3) * 8)) as u8,
            Target::None => {
                self.miss(addr);
                0
            }
        }
    }

    pub fn read16(&mut self, addr: u32) -> u16 {
        match target(addr & !1) {
            Target::Ram(o) => rd!(self.ram, o, u16, 2),
            Target::Spr(o) => rd!(self.spr, o, u16, 2),
            Target::Io(p) => (self.plat.read32(p & !3) >> ((p & 2) * 8)) as u16,
            Target::None => {
                self.miss(addr);
                0
            }
        }
    }

    pub fn read32(&mut self, addr: u32) -> u32 {
        match target(addr & !3) {
            Target::Ram(o) => rd!(self.ram, o, u32, 4),
            Target::Spr(o) => rd!(self.spr, o, u32, 4),
            Target::Io(p) => self.plat.read32(p),
            Target::None => {
                self.miss(addr);
                0
            }
        }
    }

    pub fn read64(&mut self, addr: u32) -> u64 {
        match target(addr & !7) {
            Target::Ram(o) => rd!(self.ram, o, u64, 8),
            Target::Spr(o) => rd!(self.spr, o, u64, 8),
            Target::Io(p) => self.plat.read64(p),
            Target::None => {
                self.miss(addr);
                0
            }
        }
    }

    pub fn read128(&mut self, addr: u32) -> u128 {
        match target(addr & !15) {
            Target::Ram(o) => rd!(self.ram, o, u128, 16),
            Target::Spr(o) => rd!(self.spr, o, u128, 16),
            Target::Io(p) => self.plat.read128(p),
            Target::None => {
                self.miss(addr);
                0
            }
        }
    }

    /// Record a RAM write at offset `o` (test builds; no-op otherwise).
    #[inline(always)]
    pub fn mark(&mut self, o: usize) {
        #[cfg(feature = "dirty-pages")]
        {
            let page = o >> 12;
            self.dirty[page >> 6] |= 1 << (page & 63);
        }
        #[cfg(not(feature = "dirty-pages"))]
        let _ = o;
    }

    pub fn write8(&mut self, addr: u32, v: u8) {
        match target(addr) {
            Target::Ram(o) => {
                self.mark(o);
                self.ram[o] = v;
            }
            Target::Spr(o) => self.spr[o] = v,
            Target::Io(p) => self.plat.write8(p, v, &mut self.ram, &mut self.spr),
            Target::None => self.miss(addr),
        }
    }

    pub fn write16(&mut self, addr: u32, v: u16) {
        match target(addr & !1) {
            Target::Ram(o) => {
                self.mark(o);
                self.ram[o..o + 2].copy_from_slice(&v.to_le_bytes());
            }
            Target::Spr(o) => self.spr[o..o + 2].copy_from_slice(&v.to_le_bytes()),
            Target::Io(p) => self.plat.write16(p, v, &mut self.ram, &mut self.spr),
            Target::None => self.miss(addr),
        }
    }

    pub fn write32(&mut self, addr: u32, v: u32) {
        match target(addr & !3) {
            Target::Ram(o) => {
                self.mark(o);
                self.ram[o..o + 4].copy_from_slice(&v.to_le_bytes());
            }
            Target::Spr(o) => self.spr[o..o + 4].copy_from_slice(&v.to_le_bytes()),
            Target::Io(p) => self.plat.write32(p, v, &mut self.ram, &mut self.spr),
            Target::None => self.miss(addr),
        }
    }

    pub fn write64(&mut self, addr: u32, v: u64) {
        match target(addr & !7) {
            Target::Ram(o) => {
                self.mark(o);
                self.ram[o..o + 8].copy_from_slice(&v.to_le_bytes());
            }
            Target::Spr(o) => self.spr[o..o + 8].copy_from_slice(&v.to_le_bytes()),
            Target::Io(p) => self.plat.write64(p, v, &mut self.ram, &mut self.spr),
            Target::None => self.miss(addr),
        }
    }

    pub fn write128(&mut self, addr: u32, v: u128) {
        match target(addr & !15) {
            Target::Ram(o) => {
                self.mark(o);
                self.ram[o..o + 16].copy_from_slice(&v.to_le_bytes());
            }
            Target::Spr(o) => self.spr[o..o + 16].copy_from_slice(&v.to_le_bytes()),
            Target::Io(p) => self.plat.write128(p, v, &mut self.ram, &mut self.spr),
            Target::None => self.miss(addr),
        }
    }

    /// Copy bytes into guest memory (loader / HLE use; no side effects).
    pub fn write_bytes(&mut self, addr: u32, bytes: &[u8]) {
        for (i, b) in bytes.iter().enumerate() {
            let a = addr.wrapping_add(i as u32);
            match target(a) {
                Target::Ram(o) => {
                self.mark(o);
                self.ram[o] = *b;
            }
                Target::Spr(o) => self.spr[o] = *b,
                _ => {}
            }
        }
    }

    /// Read bytes from guest memory (HLE use; no side effects).
    pub fn read_bytes(&self, addr: u32, len: usize) -> Vec<u8> {
        (0..len)
            .map(|i| match target(addr.wrapping_add(i as u32)) {
                Target::Ram(o) => self.ram[o],
                Target::Spr(o) => self.spr[o],
                _ => 0,
            })
            .collect()
    }

    /// Read a NUL-terminated string from guest memory.
    pub fn read_cstr(&self, addr: u32) -> String {
        let mut out = Vec::new();
        let mut a = addr;
        while out.len() < 4096 {
            let b = match target(a) {
                Target::Ram(o) => self.ram[o],
                Target::Spr(o) => self.spr[o],
                _ => 0,
            };
            if b == 0 {
                break;
            }
            out.push(b);
            a = a.wrapping_add(1);
        }
        String::from_utf8_lossy(&out).into_owned()
    }

    pub fn ram_u32(&self, addr: u32) -> u32 {
        let o = (addr & 0x01FF_FFFC) as usize;
        rd!(self.ram, o, u32, 4)
    }

    pub fn set_ram_u32(&mut self, addr: u32, v: u32) {
        let o = (addr & 0x01FF_FFFC) as usize;
        self.mark(o);
        self.ram[o..o + 4].copy_from_slice(&v.to_le_bytes());
    }
}
