//! Read-only ISO9660 access to the retail disc image: plain 2048-byte sectors
//! (`.iso`) or raw CD sectors (`.bin`, MODE2/2352).

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

/// How the image stores a sector's 2048 data bytes: at `head` bytes into each
/// `stride`-byte sector (a raw CD image: 2352 bytes, 24 of header in
/// MODE2/2352, 16 in MODE1/2352).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Layout {
    stride: u64,
    head: u64,
}

const PLAIN: Layout = Layout {
    stride: SECTOR as u64,
    head: 0,
};

/// The layout whose sector 16 holds the primary volume descriptor, else plain.
fn detect_layout(src: &mut dyn Source) -> Layout {
    for layout in [
        PLAIN,
        Layout {
            stride: 2352,
            head: 24,
        },
        Layout {
            stride: 2352,
            head: 16,
        },
    ] {
        let mut b = [0u8; 6];
        if src
            .read_at(16 * layout.stride + layout.head, &mut b)
            .is_ok()
            && b[0] == 1
            && &b[1..6] == b"CD001"
        {
            return layout;
        }
    }
    PLAIN
}

pub struct Disc {
    src: Box<dyn Source>,
    layout: Layout,
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

    pub fn from_source(mut src: Box<dyn Source>) -> Self {
        let layout = detect_layout(&mut *src);
        let sectors = src.len() / layout.stride;
        Self {
            src,
            layout,
            sectors,
        }
    }

    pub fn read(&mut self, sector: u32, count: u32, out: &mut [u8]) -> bool {
        let n = count as usize * SECTOR;
        if out.len() < n {
            return false;
        }
        let Layout { stride, head } = self.layout;
        if self.layout == PLAIN {
            return self
                .src
                .read_at(sector as u64 * stride, &mut out[..n])
                .is_ok();
        }
        // A raw image: the data of each sector sits between its header and
        // its error correction.
        (0..count as u64).all(|i| {
            let at = (sector as u64 + i) * stride + head;
            let o = i as usize * SECTOR;
            self.src.read_at(at, &mut out[o..o + SECTOR]).is_ok()
        })
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

#[cfg(test)]
mod tests {
    use super::*;

    struct Mem(Vec<u8>);

    impl Source for Mem {
        fn len(&self) -> u64 {
            self.0.len() as u64
        }
        fn read_at(&mut self, off: u64, buf: &mut [u8]) -> std::io::Result<()> {
            let o = off as usize;
            match self.0.get(o..o + buf.len()) {
                Some(d) => {
                    buf.copy_from_slice(d);
                    Ok(())
                }
                None => Err(std::io::ErrorKind::UnexpectedEof.into()),
            }
        }
    }

    /// An image of `sectors` sectors, each with `head` bytes before its data and
    /// `stride - head - 2048` after; sector n's data starts with n as a byte,
    /// and sector 16 is a primary volume descriptor.
    fn image(stride: usize, head: usize, sectors: usize) -> Vec<u8> {
        let mut v = vec![0xEE; stride * sectors];
        for n in 0..sectors {
            let d = n * stride + head;
            v[d..d + 2048].fill(n as u8);
        }
        let pvd = 16 * stride + head;
        v[pvd] = 1;
        v[pvd + 1..pvd + 6].copy_from_slice(b"CD001");
        v
    }

    #[test]
    fn plain_and_raw_images_read_the_same_sector_data() {
        for (stride, head) in [(2048, 0), (2352, 24), (2352, 16)] {
            let mut d = Disc::from_source(Box::new(Mem(image(stride, head, 20))));
            assert_eq!(d.sectors, 20, "{stride}+{head}");
            let v = d.read_vec(3, 2);
            assert!(v[..2048].iter().all(|&b| b == 3), "{stride}+{head}");
            assert!(v[2048..].iter().all(|&b| b == 4), "{stride}+{head}");
            let pvd = d.read_vec(16, 1);
            assert_eq!(&pvd[..6], b"\x01CD001", "{stride}+{head}");
            assert!(!d.read(19, 2, &mut vec![0; 4096]), "past the end");
        }
    }

    #[test]
    fn an_image_without_a_volume_descriptor_is_plain() {
        let mut d = Disc::from_source(Box::new(Mem(vec![7; 2048 * 4])));
        assert_eq!(d.sectors, 4);
        assert!(d.read_vec(1, 1).iter().all(|&b| b == 7));
    }
}
