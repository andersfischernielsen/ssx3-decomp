//! The game data packed for a standalone build (`tools/bundle.py`).
//!
//! A pack is the disc image without what the game never reads: the padding
//! files (`PAD0.000`, `PAD1.000`) and all-zero sectors. It serves the same
//! 2048-byte sectors as the image ([`crate::iso::Source`]), so the emulated
//! CD, the native file layer and the executable's loader read it unchanged.
//!
//! Layout (little endian):
//!
//! ```text
//! "SSX3PAK1"  u32 total sectors  u32 extents
//! extents x { u32 first sector, u32 sectors, u64 data offset (from the pack start) }
//! data
//! ... and, when appended to an executable, a trailer at the very end:
//! u64 pack start (file offset)  "SSX3PAKT"
//! ```
//!
//! Sectors in no extent read as zeros.

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use crate::iso::{Source, SECTOR};

pub const MAGIC: &[u8; 8] = b"SSX3PAK1";
pub const TRAILER: &[u8; 8] = b"SSX3PAKT";

#[derive(Clone, Copy, Debug)]
struct Extent {
    first: u32,
    count: u32,
    offset: u64,
}

/// A pack opened from a file (on its own, or appended to an executable).
pub struct Pack {
    file: File,
    /// File offset of the pack.
    base: u64,
    sectors: u32,
    extents: Vec<Extent>,
}

impl Pack {
    /// Open the pack starting at file offset `base` of `path`.
    pub fn open_at(path: &Path, base: u64) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        file.seek(SeekFrom::Start(base))?;
        let mut head = [0u8; 16];
        file.read_exact(&mut head)?;
        if &head[..8] != MAGIC {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "not an SSX3 pack",
            ));
        }
        let sectors = u32::from_le_bytes(head[8..12].try_into().unwrap());
        let n = u32::from_le_bytes(head[12..16].try_into().unwrap()) as usize;
        let mut table = vec![0u8; n * 16];
        file.read_exact(&mut table)?;
        let extents = table
            .chunks_exact(16)
            .map(|e| Extent {
                first: u32::from_le_bytes(e[0..4].try_into().unwrap()),
                count: u32::from_le_bytes(e[4..8].try_into().unwrap()),
                offset: u64::from_le_bytes(e[8..16].try_into().unwrap()),
            })
            .collect();
        Ok(Self {
            file,
            base,
            sectors,
            extents,
        })
    }

    /// The pack appended to `path` (an executable), if it has one.
    pub fn appended(path: &Path) -> Option<Self> {
        let mut f = File::open(path).ok()?;
        let len = f.metadata().ok()?.len();
        if len < 16 {
            return None;
        }
        f.seek(SeekFrom::Start(len - 16)).ok()?;
        let mut t = [0u8; 16];
        f.read_exact(&mut t).ok()?;
        if &t[8..] != TRAILER {
            return None;
        }
        Self::open_at(path, u64::from_le_bytes(t[..8].try_into().unwrap())).ok()
    }

    /// `SSX3_PACK` or an explicit data root selects the pack exclusively.
    /// Otherwise search the executable's appended pack, adjacent `ssx3.pack`,
    /// then `../Resources/ssx3.pack`. Returns the file and pack offset.
    pub fn locate() -> Option<(PathBuf, u64)> {
        if let Some(p) = crate::platform_paths::pack_override() {
            return Self::open_at(&p, 0).ok().map(|_| (p, 0));
        }
        let exe = std::env::current_exe().ok()?;
        if let Some(p) = Self::appended(&exe) {
            return Some((exe, p.base));
        }
        let dir = exe.parent()?;
        for p in [dir.join("ssx3.pack"), dir.join("../Resources/ssx3.pack")] {
            if Self::open_at(&p, 0).is_ok() {
                return Some((p, 0));
            }
        }
        None
    }

    fn extent_of(&self, sector: u32) -> Option<&Extent> {
        let i = self
            .extents
            .partition_point(|e| e.first + e.count <= sector);
        self.extents.get(i).filter(|e| e.first <= sector)
    }
}

impl Source for Pack {
    fn len(&self) -> u64 {
        self.sectors as u64 * SECTOR as u64
    }

    fn read_at(&mut self, off: u64, buf: &mut [u8]) -> std::io::Result<()> {
        // Sector by sector run: each run is inside one extent or a hole.
        let mut done = 0usize;
        while done < buf.len() {
            let pos = off + done as u64;
            let sector = (pos / SECTOR as u64) as u32;
            let in_sector = (pos % SECTOR as u64) as usize;
            let (run, src) = match self.extent_of(sector) {
                Some(e) => {
                    let left = (e.first + e.count - sector) as usize * SECTOR - in_sector;
                    let src =
                        e.offset + (sector - e.first) as u64 * SECTOR as u64 + in_sector as u64;
                    (left, Some(src))
                }
                None => {
                    // Up to the next extent (or the end).
                    let next = self
                        .extents
                        .iter()
                        .find(|e| e.first > sector)
                        .map_or(self.sectors, |e| e.first);
                    (
                        (next.max(sector + 1) - sector) as usize * SECTOR - in_sector,
                        None,
                    )
                }
            };
            let n = run.min(buf.len() - done);
            match src {
                Some(src) => {
                    self.file.seek(SeekFrom::Start(self.base + src))?;
                    self.file.read_exact(&mut buf[done..done + n])?;
                }
                None => buf[done..done + n].fill(0),
            }
            done += n;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    /// A pack with sectors 2..4 and 7 present (sector k filled with k),
    /// appended after `prefix` bytes, read back through the trailer.
    #[test]
    fn appended_pack_reads_sectors_and_holes() {
        let dir = std::env::temp_dir().join(format!("ssx3-pack-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("exe");
        let mut f = File::create(&path).unwrap();
        let prefix = vec![0xaau8; 1000];
        f.write_all(&prefix).unwrap();
        let extents = [(2u32, 3u32), (7, 1)];
        let mut head = Vec::new();
        head.extend_from_slice(MAGIC);
        head.extend_from_slice(&10u32.to_le_bytes());
        head.extend_from_slice(&(extents.len() as u32).to_le_bytes());
        let mut off = (16 + 16 * extents.len()) as u64;
        let mut data = Vec::new();
        for (first, count) in extents {
            head.extend_from_slice(&first.to_le_bytes());
            head.extend_from_slice(&count.to_le_bytes());
            head.extend_from_slice(&off.to_le_bytes());
            for s in first..first + count {
                data.extend(std::iter::repeat_n(s as u8, SECTOR));
            }
            off += count as u64 * SECTOR as u64;
        }
        f.write_all(&head).unwrap();
        f.write_all(&data).unwrap();
        f.write_all(&(prefix.len() as u64).to_le_bytes()).unwrap();
        f.write_all(TRAILER).unwrap();
        drop(f);
        let mut p = Pack::appended(&path).expect("trailer");
        assert_eq!(p.len(), 10 * SECTOR as u64);
        let mut buf = vec![0xffu8; 10 * SECTOR];
        p.read_at(0, &mut buf).unwrap();
        for s in 0..10u32 {
            let want = if (2..5).contains(&s) || s == 7 {
                s as u8
            } else {
                0
            };
            assert!(
                buf[s as usize * SECTOR..(s as usize + 1) * SECTOR]
                    .iter()
                    .all(|&b| b == want),
                "sector {s}"
            );
        }
        // A read inside a sector across an extent boundary.
        let mut b = [0u8; 100];
        p.read_at(5 * SECTOR as u64 - 50, &mut b).unwrap();
        assert!(b[..50].iter().all(|&x| x == 4) && b[50..].iter().all(|&x| x == 0));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
