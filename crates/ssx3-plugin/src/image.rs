//! Disc images the plugin layer reads itself: plain `.iso` (2048-byte sectors)
//! and raw single-track CD images (`.bin`, 2352-byte sectors, MODE1 or MODE2).
//! Detecting which image a plugin needs only has to find the boot serial.
//! Everything else (a multi-track `.cue`, `.chd`, `.nrg`, ...) is the
//! plugin's to read: its manifest names a `detect` command
//! ([`crate::manifest::DiscInput`]). The engine's own disc reader
//! (`ssx3_core::iso`) stays plain-ISO only.

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use ssx3_core::iso::{Disc, Source, SECTOR};

/// A raw CD image seen as plain 2048-byte sectors: `head` bytes of header,
/// 2048 of data, the rest error correction, in each `stride`-byte sector.
struct RawSource {
    file: File,
    stride: u64,
    head: u64,
    len: u64,
}

impl Source for RawSource {
    fn len(&self) -> u64 {
        self.len / self.stride * SECTOR as u64
    }

    fn read_at(&mut self, off: u64, buf: &mut [u8]) -> std::io::Result<()> {
        let mut done = 0;
        while done < buf.len() {
            let pos = off + done as u64;
            let (sector, inner) = (pos / SECTOR as u64, pos % SECTOR as u64);
            let n = (SECTOR - inner as usize).min(buf.len() - done);
            self.file
                .seek(SeekFrom::Start(sector * self.stride + self.head + inner))?;
            self.file.read_exact(&mut buf[done..done + n])?;
            done += n;
        }
        Ok(())
    }
}

/// The layout whose sector 16 holds the primary volume descriptor.
fn layout(file: &mut File) -> Option<(u64, u64)> {
    for (stride, head) in [(SECTOR as u64, 0), (2352, 24), (2352, 16)] {
        let mut b = [0u8; 6];
        if file.seek(SeekFrom::Start(16 * stride + head)).is_ok()
            && file.read_exact(&mut b).is_ok()
            && b[0] == 1
            && &b[1..6] == b"CD001"
        {
            return Some((stride, head));
        }
    }
    None
}

/// Open an `.iso` or a raw `.bin` image; `None` for anything else.
pub fn open(path: &Path) -> Option<Disc> {
    let mut file = File::open(path).ok()?;
    let (stride, head) = layout(&mut file)?;
    if stride == SECTOR as u64 {
        return Some(Disc::from_source(Box::new(file)));
    }
    let len = file.metadata().ok()?.len();
    Some(Disc::from_source(Box::new(RawSource {
        file,
        stride,
        head,
        len,
    })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::discover::testutil::*;

    #[test]
    fn raw_images_read_like_plain_ones_across_sector_boundaries() {
        let t = Tmp::new("image");
        for (name, stride, head) in [
            ("p.iso", 2048usize, 0usize),
            ("m2.bin", 2352, 24),
            ("m1.bin", 2352, 16),
        ] {
            // Sector n's data is n repeated; sector 16 is the volume descriptor.
            let mut img = vec![0xEEu8; stride * 20];
            for n in 0..20 {
                img[n * stride + head..n * stride + head + SECTOR].fill(n as u8);
            }
            let pvd = 16 * stride + head;
            img[pvd] = 1;
            img[pvd + 1..pvd + 6].copy_from_slice(b"CD001");
            std::fs::write(t.path().join(name), img).unwrap();
        }
        let mut sources: Vec<Box<dyn Source>> = Vec::new();
        for name in ["p.iso", "m2.bin", "m1.bin"] {
            let mut f = File::open(t.path().join(name)).unwrap();
            let (stride, head) = layout(&mut f).unwrap();
            sources.push(if stride == SECTOR as u64 {
                Box::new(f)
            } else {
                let len = f.metadata().unwrap().len();
                Box::new(RawSource {
                    file: f,
                    stride,
                    head,
                    len,
                })
            });
        }
        for (i, s) in sources.iter_mut().enumerate() {
            assert_eq!(s.len(), 20 * SECTOR as u64, "image {i}");
            // Three sectors' worth starting in the middle of sector 2.
            let mut buf = vec![0u8; 2 * SECTOR + 100];
            s.read_at(2 * SECTOR as u64 + 1000, &mut buf).unwrap();
            assert!(buf[..SECTOR - 1000].iter().all(|&b| b == 2), "image {i}");
            assert!(
                buf[SECTOR - 1000..2 * SECTOR - 1000]
                    .iter()
                    .all(|&b| b == 3),
                "image {i}"
            );
            assert!(
                buf[2 * SECTOR - 1000..].iter().all(|&b| b == 4),
                "image {i}"
            );
            assert!(
                s.read_at(19 * SECTOR as u64 + 2000, &mut [0u8; 200])
                    .is_err(),
                "past the end"
            );
        }
    }

    #[test]
    fn open_reads_the_boot_file_of_both_and_refuses_other_files() {
        let t = Tmp::new("open");
        std::fs::write(t.path().join("a.iso"), fake_disc("SLUS_207.72", false)).unwrap();
        std::fs::write(t.path().join("b.bin"), fake_disc("SLES_500.30", true)).unwrap();
        std::fs::write(t.path().join("c.chd"), b"not a disc image").unwrap();
        assert!(open(&t.path().join("a.iso")).is_some());
        assert!(open(&t.path().join("b.bin")).is_some());
        assert!(open(&t.path().join("c.chd")).is_none());
        assert!(open(&t.path().join("missing.iso")).is_none());
    }
}
