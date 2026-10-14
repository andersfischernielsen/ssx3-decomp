//! PS2 memory card (8 MB) on the SIO2 bus, backed by a host file.
//!
//! The card is NAND flash: 16,384 pages of 512 data bytes + 16 spare bytes
//! (MCMAN keeps its ECC there), erased in blocks of 16 pages (erased = all
//! `0xFF`); a page write can only clear bits. The host file is the raw image
//! (`0x4000 × 528` = 8,650,752 bytes, the same layout as PCSX2's `.ps2`
//! files). It is created erased (unformatted) when missing: the game's own
//! MCMAN formats it. Writes and erases go straight to the file.
//!
//! Packets start with `0x81`, then the command. Every reply starts with two
//! `0x00` bytes and ends with `0x2B` and the *terminator* byte the host
//! chose with command `0x27` (MCMAN checks it to detect a card that is not
//! answering). Command set (behaviour after PCSX2's `MemoryCardProtocol.cpp`):
//!
//! | cmd  | in | meaning |
//! | ---- | -- | ------- |
//! | 0x11 | | probe |
//! | 0x12 | | end of write/erase |
//! | 0x21/0x22/0x23 | page (4 bytes LE) + xor | set page for erase / write / read |
//! | 0x26 | | card specs: page size, pages per block, page count, xor |
//! | 0x27 | term | set terminator |
//! | 0x28 | | get terminator |
//! | 0x42 | n, n bytes | write n bytes at the current address (reply: xor) |
//! | 0x43 | n | read n bytes (reply: data, xor) |
//! | 0x81 | | end of read/write |
//! | 0x82 | | erase the current block |
//! | 0xBF | | ? (boot) |
//! | 0xF0 | mode, [8 bytes] | authentication steps; some modes XOR 8 bytes |
//! | 0xF3, 0xF7 | | authentication reset / end |
//!
//! The MagicGate challenge itself is answered by SECRMAN (`SecrAuthCard`),
//! which the native kernel provides; see `imports.rs`.

use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

pub const PAGE_DATA: usize = 512;
pub const PAGE_RAW: usize = 528;
pub const PAGES: usize = 0x4000;
pub const BLOCK_PAGES: usize = 16;
pub const CARD_BYTES: usize = PAGE_RAW * PAGES;

pub struct MemoryCard {
    pub data: Vec<u8>,
    file: Option<File>,
    pub path: PathBuf,
    term: u8,
    /// Byte offset of the next read/write (page × 528 + progress).
    addr: usize,
    pub reads: u64,
    pub writes: u64,
    pub erases: u64,
    pub created: bool,
}

impl MemoryCard {
    /// An erased card in memory only.
    pub fn blank() -> Self {
        Self {
            data: vec![0xFF; CARD_BYTES],
            file: None,
            path: PathBuf::new(),
            term: 0x55,
            addr: 0,
            reads: 0,
            writes: 0,
            erases: 0,
            created: true,
        }
    }

    /// Open (or create, erased) the card image at `path`.
    pub fn open(path: &Path) -> std::io::Result<Self> {
        let mut card = Self::blank();
        card.path = path.to_path_buf();
        if let Some(dir) = path.parent() {
            if !dir.as_os_str().is_empty() {
                std::fs::create_dir_all(dir)?;
            }
        }
        let exists = path.exists();
        let mut f = OpenOptions::new().read(true).write(true).create(true).truncate(false).open(path)?;
        if exists {
            let mut v = Vec::new();
            f.read_to_end(&mut v)?;
            let n = v.len().min(CARD_BYTES);
            card.data[..n].copy_from_slice(&v[..n]);
            card.created = false;
        }
        if !exists || f.metadata()?.len() as usize != CARD_BYTES {
            f.seek(SeekFrom::Start(0))?;
            f.write_all(&card.data)?;
            f.set_len(CARD_BYTES as u64)?;
        }
        card.file = Some(f);
        Ok(card)
    }

    /// The card configured by `SSX3_MEMCARD` (default `saves/ssx3-mc0.ps2`;
    /// `none` leaves the slot empty).
    pub fn from_env() -> Option<Self> {
        let p = std::env::var("SSX3_MEMCARD").unwrap_or_else(|_| "saves/ssx3-mc0.ps2".into());
        if p.is_empty() || p == "none" {
            eprintln!("[mc] no memory card inserted (SSX3_MEMCARD=none)");
            return None;
        }
        match Self::open(Path::new(&p)) {
            Ok(c) => {
                eprintln!("[mc] memory card in port 1: {p}{}", if c.created { " (new, unformatted)" } else { "" });
                Some(c)
            }
            Err(e) => {
                eprintln!("[mc] cannot open {p}: {e}; no memory card inserted");
                None
            }
        }
    }

    fn persist(&mut self, at: usize, len: usize) {
        let Some(f) = self.file.as_mut() else { return };
        let r = f.seek(SeekFrom::Start(at as u64)).and_then(|_| f.write_all(&self.data[at..at + len]));
        if let Err(e) = r {
            eprintln!("[mc] write to {}: {e}", self.path.display());
        }
    }

    pub fn flush(&mut self) {
        if let Some(f) = self.file.as_mut() {
            let _ = f.flush();
        }
    }

    fn end(&self, out: &mut Vec<u8>, len: usize) {
        while out.len() + 2 < len {
            out.push(0);
        }
        out.push(0x2B);
        out.push(self.term);
    }

    /// Answer one packet (`p[0]` = 0x81).
    pub fn transfer(&mut self, p: &[u8]) -> Vec<u8> {
        let at = |i: usize| p.get(i).copied().unwrap_or(0);
        let mut out = vec![0u8, 0u8];
        match at(1) {
            0x11 | 0x12 | 0x81 => self.end(&mut out, 4),
            0x21..=0x23 => {
                let page = u32::from_le_bytes([at(2), at(3), at(4), at(5)]) as usize;
                if at(2) ^ at(3) ^ at(4) ^ at(5) != at(6) {
                    eprintln!("[mc] page number checksum mismatch ({page:#x})");
                }
                self.addr = page.min(PAGES - 1) * PAGE_RAW;
                self.end(&mut out, 9);
            }
            0x26 => {
                let spec = [
                    PAGE_DATA as u8,
                    (PAGE_DATA >> 8) as u8,
                    BLOCK_PAGES as u8,
                    (BLOCK_PAGES >> 8) as u8,
                    PAGES as u8,
                    (PAGES >> 8) as u8,
                    (PAGES >> 16) as u8,
                    (PAGES >> 24) as u8,
                ];
                out.push(0x2B);
                out.extend_from_slice(&spec);
                out.push(spec.iter().fold(0, |a, b| a ^ b));
                out.push(self.term);
            }
            0x27 => {
                self.term = at(2);
                out.push(0);
                out.push(0x2B);
                out.push(self.term);
            }
            0x28 => {
                out.push(0x2B);
                out.push(self.term);
                out.push(self.term);
            }
            0x42 => {
                let n = at(2) as usize;
                let src: Vec<u8> = (0..n).map(|i| at(3 + i)).collect();
                out.push(0);
                out.push(0x2B);
                out.extend(std::iter::repeat(0).take(n));
                out.push(src.iter().fold(0, |a, b| a ^ b));
                out.push(self.term);
                let a = self.addr.min(CARD_BYTES);
                let n = n.min(CARD_BYTES - a);
                for i in 0..n {
                    // Flash: programming only clears bits.
                    self.data[a + i] &= src[i];
                }
                self.persist(a, n);
                self.addr += n;
                self.writes += 1;
            }
            0x43 => {
                let n = at(2) as usize;
                out.push(0);
                out.push(0x2B);
                let mut x = 0u8;
                for i in 0..n {
                    let b = self.data.get(self.addr + i).copied().unwrap_or(0xFF);
                    x ^= b;
                    out.push(b);
                }
                out.push(x);
                out.push(self.term);
                self.addr += n;
                self.reads += 1;
            }
            0x82 => {
                let blk = PAGE_RAW * BLOCK_PAGES;
                let a = (self.addr / blk) * blk;
                if a < CARD_BYTES {
                    self.data[a..a + blk].fill(0xFF);
                    self.persist(a, blk);
                }
                self.erases += 1;
                self.end(&mut out, 4);
            }
            0xBF => self.end(&mut out, 5),
            0xF0 => match at(2) {
                // Long, XOR of the 8 data bytes.
                0x01 | 0x02 | 0x04 | 0x0F | 0x11 | 0x13 => {
                    out.push(0);
                    out.push(0x2B);
                    out.extend(std::iter::repeat(0).take(8));
                    out.push((3..11).map(at).fold(0, |a, b| a ^ b));
                    out.push(self.term);
                }
                // Long without XOR.
                0x06 | 0x07 | 0x0B => self.end(&mut out, 14),
                // Short.
                _ => self.end(&mut out, 5),
            },
            0xF3 => {
                self.term = 0x55;
                self.end(&mut out, 5);
            }
            0xF7 => self.end(&mut out, 5),
            other => {
                eprintln!("[mc] unknown memory card command {other:#04x} ({} bytes)", p.len());
                self.end(&mut out, p.len().max(4));
            }
        }
        out
    }

    pub fn describe(&self) -> String {
        format!("{} r{} w{} e{}", self.path.display(), self.reads, self.writes, self.erases)
    }
}
