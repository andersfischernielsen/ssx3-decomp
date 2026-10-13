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

pub struct Disc {
    file: File,
    pub sectors: u64,
}

impl Disc {
    pub fn open(path: &Path) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let len = file.metadata()?.len();
        Ok(Self { file, sectors: len / SECTOR as u64 })
    }

    pub fn read(&mut self, sector: u32, count: u32, out: &mut [u8]) -> bool {
        let n = count as usize * SECTOR;
        if out.len() < n || self.file.seek(SeekFrom::Start(sector as u64 * SECTOR as u64)).is_err() {
            return false;
        }
        self.file.read_exact(&mut out[..n]).is_ok()
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
