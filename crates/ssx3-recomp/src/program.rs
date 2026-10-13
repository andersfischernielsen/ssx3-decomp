//! Binds the embedded function inventory to the ELF image so functions can be
//! sliced out and decoded.
//!
//! Extents come straight from Ghidra (`body_end`), so a function's code is
//! exactly its body — no guessing from neighbouring entries.

use ssx3_core::elf::{Elf, ElfError};

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

    /// The raw machine code of a function (its exact body).
    pub fn code(&self, f: &Function) -> Option<&[u8]> {
        self.elf.read(f.address, f.len())
    }

    /// Decode a function into instructions (4 bytes each).
    ///
    /// This is a linear sweep. A function body can contain embedded data (e.g.
    /// jump tables) that a linear sweep will misinterpret as instructions, and
    /// Ghidra's `body_end` can include trailing alignment; both are handled
    /// properly once we have per-instruction boundaries (recursive descent).
    pub fn decode(&self, f: &Function) -> Vec<Instruction> {
        let Some(code) = self.code(f) else {
            return Vec::new();
        };
        code.chunks_exact(4)
            .map(|c| decode(u32::from_le_bytes([c[0], c[1], c[2], c[3]])))
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
            assert!(p.code(f).is_some(), "no code for {} at {:08x}", f.name, f.address);
        }
    }

    #[test]
    fn entry_is_112_instructions() {
        // Ghidra disassembles `entry` as 112 instructions.
        let Some(p) = open() else { return };
        let entry = p.function_by_name("entry").unwrap();
        let decoded = p.decode(entry);
        assert_eq!(decoded.len(), 112);
        // The first instruction is `clear at` = MMI mmiop=0x10 fct=0x28 rd=1.
        assert_eq!(decoded[0].mnemonic, "clear");
        assert_eq!(
            decoded[0].operands,
            crate::r5900::Operands::Mmi {
                mmiop: 0x10,
                fct: 0x28,
                rd: 1,
                rs: 0,
                rt: 0,
            }
        );
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
