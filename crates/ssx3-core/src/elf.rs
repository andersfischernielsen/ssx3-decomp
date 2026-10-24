//! Minimal ELF32 loader for the EE image.
//!
//! `SLUS_207.72` is a 32-bit little-endian MIPS ELF. We only need what the
//! recompiler and runtime use: the entry point, the `PT_LOAD` program headers,
//! random access by virtual address, and the ability to drop the image into
//! [`crate::Memory`].

use std::fmt;
use std::path::Path;

use crate::Memory;

const ELF_MAGIC: [u8; 4] = [0x7f, b'E', b'L', b'F'];
const PT_LOAD: u32 = 1;

#[derive(Debug)]
pub enum ElfError {
    TooSmall,
    BadMagic,
    Not32Bit,
    NotLittleEndian,
    BadProgramHeaders,
    Io(std::io::Error),
}

impl fmt::Display for ElfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ElfError::TooSmall => write!(f, "file is too small to be an ELF"),
            ElfError::BadMagic => write!(f, "missing ELF magic"),
            ElfError::Not32Bit => write!(f, "not a 32-bit ELF"),
            ElfError::NotLittleEndian => write!(f, "not little-endian"),
            ElfError::BadProgramHeaders => write!(f, "program headers are out of range"),
            ElfError::Io(e) => write!(f, "io error: {e}"),
        }
    }
}

impl std::error::Error for ElfError {}

impl From<std::io::Error> for ElfError {
    fn from(e: std::io::Error) -> Self {
        ElfError::Io(e)
    }
}

/// One `PT_LOAD` segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoadSegment {
    pub offset: u32,
    pub vaddr: u32,
    pub filesz: u32,
    pub memsz: u32,
    pub flags: u32,
}

/// A parsed ELF32 image.
pub struct Elf {
    bytes: Vec<u8>,
    /// Entry point from the ELF header.
    pub entry: u32,
    /// Loadable segments, ascending by `vaddr`.
    pub segments: Vec<LoadSegment>,
}

fn rd_u16(b: &[u8], off: usize) -> u16 {
    u16::from_le_bytes([b[off], b[off + 1]])
}
fn rd_u32(b: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([b[off], b[off + 1], b[off + 2], b[off + 3]])
}

impl Elf {
    pub fn parse(bytes: Vec<u8>) -> Result<Self, ElfError> {
        if bytes.len() < 52 {
            return Err(ElfError::TooSmall);
        }
        if bytes[0..4] != ELF_MAGIC {
            return Err(ElfError::BadMagic);
        }
        if bytes[4] != 1 {
            return Err(ElfError::Not32Bit);
        }
        if bytes[5] != 1 {
            return Err(ElfError::NotLittleEndian);
        }

        let entry = rd_u32(&bytes, 24);
        let phoff = rd_u32(&bytes, 28) as usize;
        let phentsize = rd_u16(&bytes, 42) as usize;
        let phnum = rd_u16(&bytes, 44) as usize;

        let end = phoff
            .checked_add(
                phentsize
                    .checked_mul(phnum)
                    .ok_or(ElfError::BadProgramHeaders)?,
            )
            .ok_or(ElfError::BadProgramHeaders)?;
        if phentsize < 32 || end > bytes.len() {
            return Err(ElfError::BadProgramHeaders);
        }

        let mut segments = Vec::new();
        for i in 0..phnum {
            let ph = phoff + i * phentsize;
            if rd_u32(&bytes, ph) != PT_LOAD {
                continue;
            }
            segments.push(LoadSegment {
                offset: rd_u32(&bytes, ph + 4),
                vaddr: rd_u32(&bytes, ph + 8),
                filesz: rd_u32(&bytes, ph + 16),
                memsz: rd_u32(&bytes, ph + 20),
                flags: rd_u32(&bytes, ph + 24),
            });
        }
        segments.sort_by_key(|s| s.vaddr);
        Ok(Self {
            bytes,
            entry,
            segments,
        })
    }

    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, ElfError> {
        Self::parse(std::fs::read(path)?)
    }

    /// The segment containing `vaddr` with at least `len` file-backed bytes.
    pub fn segment_for(&self, vaddr: u32, len: u32) -> Option<&LoadSegment> {
        self.segments.iter().find(|s| {
            vaddr >= s.vaddr
                && vaddr
                    .checked_add(len)
                    .is_some_and(|end| end <= s.vaddr + s.filesz)
        })
    }

    /// Borrow `len` bytes of file-backed image at virtual address `vaddr`.
    pub fn read(&self, vaddr: u32, len: u32) -> Option<&[u8]> {
        let s = self.segment_for(vaddr, len)?;
        let start = (s.offset + (vaddr - s.vaddr)) as usize;
        Some(&self.bytes[start..start + len as usize])
    }

    /// One little-endian word at `vaddr`.
    pub fn read_u32(&self, vaddr: u32) -> Option<u32> {
        let b = self.read(vaddr, 4)?;
        Some(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    /// Copy every `PT_LOAD` segment into `mem` at its virtual address.
    pub fn load_into(&self, mem: &mut Memory) {
        for s in &self.segments {
            let start = s.offset as usize;
            let end = start + s.filesz as usize;
            mem.load(s.vaddr, &self.bytes[start..end]);
        }
    }

    /// Lowest and highest virtual addresses covered by loadable file data.
    pub fn image_range(&self) -> Option<(u32, u32)> {
        let lo = self.segments.first()?.vaddr;
        let hi = self.segments.iter().map(|s| s.vaddr + s.filesz).max()?;
        Some((lo, hi))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn synthetic() -> Vec<u8> {
        // 1 PT_LOAD: program header at file offset 0x34, file data at 0x54,
        // vaddr 0x1000, filesz 8, memsz 16.
        let mut b = vec![0u8; 0x54 + 8];
        b[0..4].copy_from_slice(&ELF_MAGIC);
        b[4] = 1; // 32-bit
        b[5] = 1; // LE
        b[24..28].copy_from_slice(&0x1000u32.to_le_bytes()); // entry
        b[28..32].copy_from_slice(&0x34u32.to_le_bytes()); // phoff
        b[42..44].copy_from_slice(&32u16.to_le_bytes()); // phentsize
        b[44..46].copy_from_slice(&1u16.to_le_bytes()); // phnum
        let ph = 0x34;
        b[ph..ph + 4].copy_from_slice(&PT_LOAD.to_le_bytes());
        b[ph + 4..ph + 8].copy_from_slice(&0x54u32.to_le_bytes()); // offset
        b[ph + 8..ph + 12].copy_from_slice(&0x1000u32.to_le_bytes()); // vaddr
        b[ph + 16..ph + 20].copy_from_slice(&8u32.to_le_bytes()); // filesz
        b[ph + 20..ph + 24].copy_from_slice(&16u32.to_le_bytes()); // memsz
        b[0x34 + 32..].copy_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8]);
        b
    }

    #[test]
    fn parses_and_reads() {
        let elf = Elf::parse(synthetic()).unwrap();
        assert_eq!(elf.entry, 0x1000);
        assert_eq!(elf.segments.len(), 1);
        assert_eq!(elf.read(0x1000, 4).unwrap(), &[1, 2, 3, 4]);
        assert_eq!(elf.read_u32(0x1004).unwrap(), 0x0807_0605);
        assert_eq!(elf.image_range(), Some((0x1000, 0x1008)));
    }

    #[test]
    fn out_of_range_read_is_none() {
        let elf = Elf::parse(synthetic()).unwrap();
        assert!(elf.read(0x1006, 4).is_none(), "crosses filesz");
        assert!(elf.read(0x2000, 4).is_none(), "not mapped");
    }

    #[test]
    fn load_into_memory() {
        let elf = Elf::parse(synthetic()).unwrap();
        let mut mem = Memory::new();
        elf.load_into(&mut mem);
        assert_eq!(mem.read_u32(0x1000), 0x0403_0201);
    }

    #[test]
    fn rejects_garbage() {
        assert!(matches!(Elf::parse(vec![0u8; 10]), Err(ElfError::TooSmall)));
        assert!(matches!(Elf::parse(vec![0u8; 64]), Err(ElfError::BadMagic)));
    }
}
