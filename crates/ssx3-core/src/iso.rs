//! Read-only ISO9660 access to the retail disc image (2048-byte sectors).

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

pub const SECTOR: usize = 2048;

#[derive(Clone, Debug)]
pub struct Entry {
    pub name: String,
    pub sector: u32,
    pub size: u32,
    pub dir: bool,
}

/// Where the disc image's bytes come from: a file, or (in the browser) the
/// image the user picked.
pub trait Source: Send {
    fn len(&self) -> u64;
    /// Fill `buf` from byte offset `off`.
    fn read_at(&mut self, off: u64, buf: &mut [u8]) -> std::io::Result<()>;
}

impl Source for File {
    fn len(&self) -> u64 {
        self.metadata().map_or(0, |m| m.len())
    }
    fn read_at(&mut self, off: u64, buf: &mut [u8]) -> std::io::Result<()> {
        self.seek(SeekFrom::Start(off))?;
        self.read_exact(buf)
    }
}

type Opener = Box<dyn Fn() -> std::io::Result<Box<dyn Source>> + Send + Sync>;
static OPENER: std::sync::OnceLock<Opener> = std::sync::OnceLock::new();

/// Make [`Disc::open`] use `f` whatever path it is given (the browser build:
/// the image comes from the page, not a path).
pub fn set_opener(f: Opener) {
    let _ = OPENER.set(f);
}

pub struct Disc {
    src: Box<dyn Source>,
    pub sectors: u64,
}

impl Disc {
    pub fn open(path: &Path) -> std::io::Result<Self> {
        let src: Box<dyn Source> = match OPENER.get() {
            Some(f) => f()?,
            None => Box::new(File::open(path)?),
        };
        Ok(Self::from_source(src))
    }

    pub fn from_source(src: Box<dyn Source>) -> Self {
        let sectors = src.len() / SECTOR as u64;
        Self { src, sectors }
    }

    pub fn read(&mut self, sector: u32, count: u32, out: &mut [u8]) -> bool {
        let n = count as usize * SECTOR;
        out.len() >= n && self.src.read_at(sector as u64 * SECTOR as u64, &mut out[..n]).is_ok()
    }

    pub fn read_vec(&mut self, sector: u32, count: u32) -> Vec<u8> {
        let mut v = vec![0; count as usize * SECTOR];
        self.read(sector, count, &mut v);
        v
    }

    fn list(&mut self, sector: u32, size: u32) -> Vec<Entry> {
        let data = self.read_vec(sector, size.div_ceil(SECTOR as u32));
        let mut out = Vec::new();
        let mut off = 0usize;
        while off < data.len().min(size as usize) {
            let len = data[off] as usize;
            if len == 0 {
                // Records never straddle sectors; skip to the next one.
                off = (off / SECTOR + 1) * SECTOR;
                continue;
            }
            let r = &data[off..off + len];
            let nlen = r[32] as usize;
            let raw = &r[33..33 + nlen];
            if !(nlen == 1 && (raw[0] == 0 || raw[0] == 1)) {
                let name = String::from_utf8_lossy(raw).to_string();
                out.push(Entry {
                    name,
                    sector: u32::from_le_bytes(r[2..6].try_into().unwrap()),
                    size: u32::from_le_bytes(r[10..14].try_into().unwrap()),
                    dir: r[25] & 2 != 0,
                });
            }
            off += len;
        }
        out
    }

    /// Resolve `cdrom0:\DIR\FILE.EXT;1` (or `/` separators, any case).
    pub fn find(&mut self, path: &str) -> Option<Entry> {
        let p = path.split_once(':').map(|(_, r)| r).unwrap_or(path);
        let pvd = self.read_vec(16, 1);
        let root = &pvd[156..156 + 34];
        let mut cur = Entry {
            name: String::new(),
            sector: u32::from_le_bytes(root[2..6].try_into().unwrap()),
            size: u32::from_le_bytes(root[10..14].try_into().unwrap()),
            dir: true,
        };
        for part in p.split(['\\', '/']).filter(|s| !s.is_empty()) {
            let want = part.split(';').next().unwrap().to_ascii_uppercase();
            let entries = self.list(cur.sector, cur.size);
            cur = entries
                .into_iter()
                .find(|e| e.name.split(';').next().unwrap().to_ascii_uppercase() == want)?;
        }
        Some(cur)
    }
}
