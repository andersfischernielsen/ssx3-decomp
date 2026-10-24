//! Minimal ELF32 loader: copies `PT_LOAD` segments into EE RAM.

use crate::cpu::Cpu;

fn u16_at(b: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([b[o], b[o + 1]])
}
fn u32_at(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}

/// Load `elf` and return its entry point.
pub fn load(c: &mut Cpu, elf: &[u8]) -> Result<u32, String> {
    if elf.len() < 52 || &elf[0..4] != b"\x7fELF" || elf[4] != 1 || elf[5] != 1 {
        return Err("not a little-endian ELF32 image".into());
    }
    let entry = u32_at(elf, 24);
    let phoff = u32_at(elf, 28) as usize;
    let phentsize = u16_at(elf, 42) as usize;
    let phnum = u16_at(elf, 44) as usize;
    for i in 0..phnum {
        let ph = phoff + i * phentsize;
        if u32_at(elf, ph) != 1 {
            continue;
        }
        let off = u32_at(elf, ph + 4) as usize;
        let vaddr = u32_at(elf, ph + 8);
        let filesz = u32_at(elf, ph + 16) as usize;
        let memsz = u32_at(elf, ph + 20) as usize;
        let data = elf.get(off..off + filesz).ok_or("segment out of range")?;
        c.bus.write_bytes(vaddr, data);
        if memsz > filesz {
            c.bus
                .write_bytes(vaddr + filesz as u32, &vec![0; memsz - filesz]);
        }
    }
    Ok(entry)
}
