//! IRX (IOP relocatable module) loader, shared by the static recompiler
//! (`recomp-iop`) and the runtime.
//!
//! An IRX is a little-endian MIPS ELF of type `0xFF80` (v1) or `0xFF81` (v2)
//! with a `.iopmod` header (program header type `0x7000_0080`): module-info
//! pointer, entry, gp, text/data/bss sizes and the module name. Sections are
//! laid out contiguously from address 0 and carry `.rel.*` tables
//! (`R_MIPS_32`, `R_MIPS_26`, `R_MIPS_HI16`, `R_MIPS_LO16`).
//!
//! Imports are stub tables inside `.text`: magic `0x41E0_0000`, a next
//! pointer, version, an 8-byte library name, then `jr $ra; addiu $0,$0,idx`
//! pairs, terminated by two zero words. Exports are the same header with magic
//! `0x41C0_0000` followed by a zero-terminated list of function pointers.
//!
//! The generator and the runtime must place a module at the same base
//! ([`layout`]): the generated code has absolute addresses baked in. The
//! runtime re-relocates the disc image and checks [`Irx::text_hash`] against
//! the value the generator recorded.

use std::collections::{BTreeMap, BTreeSet};

pub const IMPORT_MAGIC: u32 = 0x41E0_0000;
pub const EXPORT_MAGIC: u32 = 0x41C0_0000;

/// Fixed load addresses. IOP RAM below `0x2_0000` is the kernel area
/// (interrupt stack, SIF command receive buffer at `0x1_E000`); the native
/// IOP allocator hands out `HEAP_START..HEAP_END`. The EE's IOP heap RPC
/// (an HLE server in `ssx3-ps2`) keeps using `0x10_0000` and up.
pub mod layout {
    ///
    /// | module  | base      | end (text+data+bss) |
    /// | ------- | --------- | ------------------- |
    /// | LIBSD   | `0x20000` | `0x25b50` |
    /// | SNDDRV  | `0x30000` | `0x45c70` |
    /// | SIO2MAN | `0x48000` | `0x48ff0` |
    /// | MCSERV  | `0x4a000` | `0x4d760` |
    /// | PADMAN  | `0x4e000` | `0x57910` |
    /// | MCMAN   | `0x58000` | `0x7f274` |
    pub const MODULES: &[(&str, u32)] = &[
        ("LIBSD", 0x0002_0000),
        ("SNDDRV", 0x0003_0000),
        ("SIO2MAN", 0x0004_8000),
        ("MCSERV", 0x0004_A000),
        ("PADMAN", 0x0004_E000),
        ("MCMAN", 0x0005_8000),
    ];
    /// Modules that drive the SIO2 port (pads, memory cards). They run as a
    /// group with `SSX3_IOP_SIO2=1`; by default the native servers in
    /// `ssx3-ps2` (`pad.rs`, `mc_native.rs`) answer the EE instead.
    pub const SIO2_MODULES: &[&str] = &["SIO2MAN", "PADMAN", "MCMAN", "MCSERV"];
    pub const HEAP_START: u32 = 0x0008_0000;
    pub const HEAP_END: u32 = 0x0010_0000;
    pub const INTR_STACK: u32 = 0x0001_D000;
    pub const CMD_RECV: u32 = 0x0001_E000;

    pub fn base_of(name: &str) -> Option<u32> {
        MODULES.iter().find(|(n, _)| n.eq_ignore_ascii_case(name)).map(|(_, b)| *b)
    }
}

#[derive(Clone, Debug)]
pub struct Import {
    pub lib: String,
    pub version: u32,
    /// Table start (the magic word).
    pub table: u32,
    /// One past the terminator.
    pub table_end: u32,
    /// `(stub address, function index)`.
    pub stubs: Vec<(u32, u16)>,
}

#[derive(Clone, Debug)]
pub struct Export {
    pub lib: String,
    pub version: u32,
    pub table: u32,
    pub table_end: u32,
    /// Function index → absolute address.
    pub entries: Vec<u32>,
}

#[derive(Clone, Debug)]
pub struct Symbol {
    pub addr: u32,
    pub size: u32,
    pub name: String,
    pub func: bool,
}

pub struct Irx {
    pub name: String,
    pub base: u32,
    pub entry: u32,
    pub gp: u32,
    pub text_size: u32,
    pub data_size: u32,
    pub bss_size: u32,
    /// Relocated memory image (`text + data`, bss zero-filled), starting at `base`.
    pub image: Vec<u8>,
    pub imports: Vec<Import>,
    pub exports: Vec<Export>,
    pub symbols: Vec<Symbol>,
    /// Absolute addresses inside `.text` that relocated data or `%hi/%lo`
    /// pairs produce: function pointers and jump-table entries.
    pub code_refs: BTreeSet<u32>,
    /// Same, split by where the reference sits: `R_MIPS_32` in `.data`.
    pub data_code_ptrs: BTreeSet<u32>,
    /// Code addresses built with `lui`/`addiu` (handlers, thread entries).
    pub hilo_code_ptrs: BTreeSet<u32>,
}

fn u16_at(d: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([d[o], d[o + 1]])
}
fn u32_at(d: &[u8], o: usize) -> u32 {
    u32::from_le_bytes(d[o..o + 4].try_into().unwrap())
}

impl Irx {
    pub fn text_start(&self) -> u32 {
        self.base
    }
    pub fn text_end(&self) -> u32 {
        self.base + self.text_size
    }
    pub fn word(&self, addr: u32) -> Option<u32> {
        let o = addr.checked_sub(self.base)? as usize;
        (o + 4 <= self.image.len()).then(|| u32_at(&self.image, o))
    }
    /// FNV-1a over the relocated text: ties generated code to the disc image.
    pub fn text_hash(&self) -> u64 {
        let mut h = 0xcbf2_9ce4_8422_2325u64;
        for &b in &self.image[..self.text_size as usize] {
            h ^= b as u64;
            h = h.wrapping_mul(0x100_0000_01b3);
        }
        h
    }
    /// Address ranges inside `.text` that are tables, not code.
    pub fn table_ranges(&self) -> Vec<(u32, u32)> {
        let mut v: Vec<(u32, u32)> = self.imports.iter().map(|i| (i.table, i.table_end)).collect();
        v.extend(self.exports.iter().map(|e| (e.table, e.table_end)));
        v.sort();
        v
    }
    pub fn in_tables(&self, a: u32) -> bool {
        self.table_ranges().iter().any(|&(s, e)| a >= s && a < e)
    }
}

/// Parse and relocate `data` at `base`.
pub fn load(data: &[u8], base: u32) -> Result<Irx, String> {
    if data.len() < 52 || &data[..4] != b"\x7fELF" {
        return Err("not an ELF file".into());
    }
    let etype = u16_at(data, 16);
    let phoff = u32_at(data, 28) as usize;
    let shoff = u32_at(data, 32) as usize;
    let phentsize = u16_at(data, 42) as usize;
    let phnum = u16_at(data, 44) as usize;
    let shentsize = u16_at(data, 46) as usize;
    let shnum = u16_at(data, 48) as usize;
    let shstrndx = u16_at(data, 50) as usize;

    let mut iopmod = None;
    let mut load_seg = None;
    for i in 0..phnum {
        let ph = phoff + i * phentsize;
        match u32_at(data, ph) {
            0x7000_0080 => iopmod = Some(u32_at(data, ph + 4) as usize),
            1 if load_seg.is_none() => {
                load_seg = Some((u32_at(data, ph + 4) as usize, u32_at(data, ph + 16) as usize, u32_at(data, ph + 20) as usize))
            }
            _ => {}
        }
    }
    let iopmod = iopmod.ok_or("no .iopmod header")?;
    let (load_off, file_size, mem_size) = load_seg.ok_or("no load segment")?;
    let entry = u32_at(data, iopmod + 4);
    let gp = u32_at(data, iopmod + 8);
    let text_size = u32_at(data, iopmod + 12);
    let data_size = u32_at(data, iopmod + 16);
    let bss_size = u32_at(data, iopmod + 20);
    let name: String = data[iopmod + 26..]
        .iter()
        .take_while(|&&b| b != 0)
        .map(|&b| b as char)
        .collect();
    let total = (mem_size as u32).max(text_size + data_size + bss_size) as usize;
    let mut image = vec![0u8; total];
    image[..file_size].copy_from_slice(&data[load_off..load_off + file_size]);

    // Sections.
    struct Sh {
        name: String,
        kind: u32,
        addr: u32,
        off: usize,
        size: usize,
        link: usize,
        info: usize,
    }
    let str_off = u32_at(data, shoff + shstrndx * shentsize + 16) as usize;
    let cstr = |o: usize| -> String { data[o..].iter().take_while(|&&b| b != 0).map(|&b| b as char).collect() };
    let shs: Vec<Sh> = (0..shnum)
        .map(|i| {
            let s = shoff + i * shentsize;
            Sh {
                name: cstr(str_off + u32_at(data, s) as usize),
                kind: u32_at(data, s + 4),
                addr: u32_at(data, s + 12),
                off: u32_at(data, s + 16) as usize,
                size: u32_at(data, s + 20) as usize,
                link: u32_at(data, s + 24) as usize,
                info: u32_at(data, s + 28) as usize,
            }
        })
        .collect();

    // Relocations. HI16s are kept pending until the LO16 that completes them
    // (standard MIPS pairing; several HI16 may share one LO16). IRX v2
    // (0xFF81) uses the non-standard pairing where each HI16 is immediately
    // followed by its LO16.
    let v2 = etype == 0xFF81;
    let mut code_refs = BTreeSet::new();
    let mut data_code_ptrs = BTreeSet::new();
    let mut hilo_code_ptrs = BTreeSet::new();
    let text_end = base + text_size;
    let get = |img: &[u8], o: u32| -> u32 { u32_at(img, o as usize) };
    let put = |img: &mut [u8], o: u32, v: u32| img[o as usize..o as usize + 4].copy_from_slice(&v.to_le_bytes());
    let data_start = shs.iter().find(|s| s.name == ".data").map(|s| s.addr).unwrap_or(u32::MAX);
    for sh in shs.iter().filter(|s| s.kind == 9) {
        let target_is_text = shs.get(sh.info).map(|t| t.name == ".text").unwrap_or(false);
        let _ = target_is_text;
        let count = sh.size / 8;
        let mut pending_hi: Vec<u32> = Vec::new();
        let mut k = 0;
        while k < count {
            let at = u32_at(data, sh.off + k * 8);
            let kind = u32_at(data, sh.off + k * 8 + 4) & 0xFF;
            if at as usize + 4 > image.len() {
                k += 1;
                continue;
            }
            let insn = get(&image, at);
            match kind {
                0 => {}
                2 => {
                    let v = insn.wrapping_add(base);
                    put(&mut image, at, v);
                    if v >= base && v < text_end {
                        code_refs.insert(v);
                        if at >= data_start {
                            data_code_ptrs.insert(v);
                        }
                    }
                }
                4 => {
                    let t = ((insn & 0x03FF_FFFF) << 2).wrapping_add(base);
                    put(&mut image, at, (insn & !0x03FF_FFFF) | ((t >> 2) & 0x03FF_FFFF));
                }
                5 => {
                    if v2 && k + 1 < count {
                        let next = u32_at(data, sh.off + (k + 1) * 8);
                        let lo = get(&image, next) as u16 as i16 as i32 as u32;
                        let v = (insn << 16).wrapping_add(lo).wrapping_add(base);
                        let hi = (v.wrapping_add(0x8000)) >> 16;
                        put(&mut image, at, (insn & 0xFFFF_0000) | hi);
                    } else {
                        pending_hi.push(at);
                    }
                }
                6 => {
                    let lo = insn as u16 as i16 as i32 as u32;
                    if v2 {
                        put(&mut image, at, (insn & 0xFFFF_0000) | (lo.wrapping_add(base) & 0xFFFF));
                    } else {
                        let hi_word = pending_hi.first().map(|&h| get(&image, h)).unwrap_or(0);
                        let v = (hi_word << 16).wrapping_add(lo).wrapping_add(base);
                        put(&mut image, at, (insn & 0xFFFF_0000) | (v & 0xFFFF));
                        let hi = (v.wrapping_add(0x8000)) >> 16;
                        for h in pending_hi.drain(..) {
                            let w = get(&image, h);
                            put(&mut image, h, (w & 0xFFFF_0000) | hi);
                        }
                        // A %hi/%lo address into .text from an `addiu`
                        // (opcode 9) or `ori` is a code pointer.
                        let op = insn >> 26;
                        if (op == 0x09 || op == 0x0D) && v >= base && v < text_end {
                            code_refs.insert(v);
                            hilo_code_ptrs.insert(v);
                        }
                    }
                }
                other => return Err(format!("unsupported relocation type {other} at {at:#x}")),
            }
            k += 1;
        }
    }

    // Symbols (only present in some modules).
    let mut symbols = Vec::new();
    for sh in shs.iter().filter(|s| s.kind == 2) {
        let strtab = &shs[sh.link];
        for i in 0..sh.size / 16 {
            let o = sh.off + i * 16;
            let nm = u32_at(data, o) as usize;
            let value = u32_at(data, o + 4);
            let size = u32_at(data, o + 8);
            let info = data[o + 12];
            let shndx = u16_at(data, o + 14);
            let name = cstr(strtab.off + nm);
            if name.is_empty() || shndx == 0 || shndx >= 0xFF00 {
                continue;
            }
            symbols.push(Symbol { addr: base + value, size, name, func: info & 15 == 2 });
        }
    }
    symbols.sort_by_key(|s| s.addr);

    // Import / export tables inside .text.
    let mut imports = Vec::new();
    let mut exports = Vec::new();
    let mut a = 0u32;
    while a + 20 <= text_size {
        let w = get(&image, a);
        if w == IMPORT_MAGIC || w == EXPORT_MAGIC {
            let lib: String = image[a as usize + 12..a as usize + 20]
                .iter()
                .take_while(|&&b| b != 0)
                .map(|&b| b as char)
                .collect();
            let version = get(&image, a + 8);
            if w == IMPORT_MAGIC {
                let mut stubs = Vec::new();
                let mut s = a + 20;
                while s + 8 <= text_size {
                    let (w0, w1) = (get(&image, s), get(&image, s + 4));
                    if w0 == 0x03E0_0008 && w1 >> 16 == 0x2400 {
                        stubs.push((base + s, w1 as u16));
                        s += 8;
                    } else {
                        break;
                    }
                }
                // Terminator: two zero words.
                let end = if s + 8 <= text_size && get(&image, s) == 0 && get(&image, s + 4) == 0 { s + 8 } else { s };
                imports.push(Import { lib, version, table: base + a, table_end: base + end, stubs });
                a = end;
                continue;
            } else {
                let mut entries = Vec::new();
                let mut s = a + 20;
                while s + 4 <= text_size {
                    let v = get(&image, s);
                    if v == 0 {
                        break;
                    }
                    entries.push(v);
                    s += 4;
                }
                exports.push(Export { lib, version, table: base + a, table_end: base + s + 4, entries });
                a = s + 4;
                continue;
            }
        }
        a += 4;
    }
    // Export table entries are relocated pointers into .text but are not
    // "data references" in the jump-table sense; keep them as code refs.
    for e in &exports {
        for &v in &e.entries {
            code_refs.insert(v);
        }
    }

    Ok(Irx {
        name,
        base,
        entry: base + entry,
        gp: base + gp,
        text_size,
        data_size,
        bss_size,
        image,
        imports,
        exports,
        symbols,
        code_refs,
        data_code_ptrs,
        hilo_code_ptrs,
    })
}

/// Module name from a path like `cdrom0:\DATA\MODULES\SNDDRV.IRX;1`.
pub fn module_name_of(path: &str) -> String {
    let n = path.rsplit(['\\', '/', ':']).next().unwrap_or(path);
    let n = n.split(';').next().unwrap_or(n);
    let n = n.split('.').next().unwrap_or(n);
    n.to_ascii_uppercase()
}

/// Libraries the native IOP kernel provides, by name. The generator emits
/// `imports::call(c, LIB, index)` with these numeric ids.
pub const LIBRARIES: &[&str] = &[
    "sysmem", "loadcore", "intrman", "thbase", "thsemap", "thevent", "thmsgbx", "sifman", "sifcmd", "sysclib", "stdio",
    "timrman", "dmacman", "vblank", "ssbusc", "modload", "ioman", "secrman", "cdvdman",
];

pub fn library_id(name: &str) -> Option<usize> {
    LIBRARIES.iter().position(|l| *l == name)
}

/// Quick summary used by tools.
pub fn describe(irx: &Irx) -> String {
    let mut s = format!(
        "{} @ {:#x}: entry {:#x} gp {:#x} text {:#x} data {:#x} bss {:#x}\n",
        irx.name, irx.base, irx.entry, irx.gp, irx.text_size, irx.data_size, irx.bss_size
    );
    for i in &irx.imports {
        let idx: Vec<u16> = i.stubs.iter().map(|s| s.1).collect();
        s += &format!("  import {} v{:#x}: {:?}\n", i.lib, i.version, idx);
    }
    for e in &irx.exports {
        s += &format!("  export {} v{:#x}: {} entries\n", e.lib, e.version, e.entries.len());
    }
    let _ = BTreeMap::<u32, u32>::new();
    s
}
