//! Binds the embedded function inventory and instruction boundaries to the ELF
//! image so functions can be sliced out and decoded correctly.
//!
//! Extents come from Ghidra (`body_end`); instruction boundaries come from
//! Ghidra's per-instruction map, so embedded data and holes are skipped.

use ssx3_core::elf::{Elf, ElfError};

use crate::boundaries::boundaries;
use crate::r5900::{decode, Instruction};
use crate::symbols::{index, Function};

/// The EE image plus its function table.
pub struct Program {
    pub elf: Elf,
}

impl Program {
    pub fn from_elf(elf: Elf) -> Self {
        Self { elf }
    }

    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self, ElfError> {
        Ok(Self::from_elf(Elf::from_file(path)?))
    }

    /// Every function, ascending by address.
    pub fn functions(&self) -> &'static [Function] {
        index().functions()
    }

    /// The raw bytes of a function's exact body (which may contain data).
    pub fn code(&self, f: &Function) -> Option<&[u8]> {
        self.elf.read(f.address, f.len())
    }

    /// Decode a function into `(address, instruction)` pairs, honouring Ghidra's
    /// authoritative instruction boundaries. Falls back to a linear sweep of the
    /// body when a function has no boundary entry.
    pub fn decode_with_addresses(&self, f: &Function) -> Vec<(u32, Instruction)> {
        match boundaries().runs(f.address) {
            Some(runs) => {
                let mut out = Vec::new();
                for &(lo, hi) in runs {
                    if let Some(code) = self.elf.read(lo, hi - lo) {
                        for (k, c) in code.chunks_exact(4).enumerate() {
                            let addr = lo + (k as u32) * 4;
                            let word = u32::from_le_bytes([c[0], c[1], c[2], c[3]]);
                            out.push((addr, decode(word)));
                        }
                    }
                }
                out
            }
            None => self
                .code(f)
                .map(|code| {
                    code.chunks_exact(4)
                        .enumerate()
                        .map(|(k, c)| {
                            let addr = f.address + (k as u32) * 4;
                            let word = u32::from_le_bytes([c[0], c[1], c[2], c[3]]);
                            (addr, decode(word))
                        })
                        .collect()
                })
                .unwrap_or_default(),
        }
    }

    /// Decode a function into instructions, skipping embedded data.
    pub fn decode(&self, f: &Function) -> Vec<Instruction> {
        self.decode_with_addresses(f)
            .into_iter()
            .map(|(_, i)| i)
            .collect()
    }

    /// Find a function by exact name (first hit).
    pub fn function_by_name(&self, name: &str) -> Option<&'static Function> {
        index().functions().iter().find(|f| f.name == name)
    }

    /// Decode by name.
    pub fn decode_by_name(&self, name: &str) -> Vec<Instruction> {
        match self.function_by_name(name) {
            Some(f) => self.decode(f),
            None => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn open() -> Option<Program> {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../orig/SLUS_207.72");
        if !path.exists() {
            eprintln!("skipping: {} not found", path.display());
            return None;
        }
        Some(Program::open(&path).expect("open SLUS_207.72"))
    }

    #[test]
    fn every_function_is_sliceable() {
        let Some(p) = open() else { return };
        for f in p.functions() {
            assert!(
                p.code(f).is_some(),
                "no code for {} at {:08x}",
                f.name,
                f.address
            );
        }
    }

    #[test]
    fn entry_is_112_instructions() {
        let Some(p) = open() else { return };
        let entry = p.function_by_name("entry").unwrap();
        let decoded = p.decode(entry);
        assert_eq!(decoded.len(), 112);
        assert_eq!(decoded[0].mnemonic, "clear");
    }

    #[test]
    fn embedded_data_is_skipped() {
        // FUN_0042d470 has a jump table inside its body: Ghidra disassembles 407
        // instructions over 4 runs, not 410.
        let Some(p) = open() else { return };
        let f = index().by_address(0x0042_d470).unwrap();
        let decoded = p.decode(f);
        assert_eq!(decoded.len(), 407);

        // Addresses must be ascending and skip the data runs.
        let addrs: Vec<u32> = p.decode_with_addresses(f).iter().map(|(a, _)| *a).collect();
        assert!(addrs.windows(2).all(|w| w[0] < w[1]));
        assert!(!addrs.contains(&0x0042_d584), "0x42d584 is data");
    }

    #[test]
    fn decoded_total_matches_boundary_total() {
        let Some(p) = open() else { return };
        let total: usize = p.functions().iter().map(|f| p.decode(f).len()).sum();
        assert_eq!(total, boundaries().instruction_count());
        assert_eq!(total, 783_966);
    }

    #[test]
    fn bodies_are_nonempty_and_fit_the_image() {
        let Some(p) = open() else { return };
        let (_, image_end) = p.elf.image_range().unwrap();
        for f in p.functions() {
            assert!(f.end > f.address, "empty body: {}", f.name);
            assert!(f.end <= image_end, "body past image: {}", f.name);
        }
    }
}
