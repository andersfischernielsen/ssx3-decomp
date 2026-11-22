//! Course-local mountain-bank bytes behind the already mounted audio BIG.
//!
//! The archive directory and every other member stay fixed. Native bank
//! allocation, sample uploads, object voices and spatial mixing are unchanged.

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

pub(crate) struct AudioBanks {
    root: PathBuf,
    start: u64,
    size: u64,
    banks: HashMap<&'static str, Option<File>>,
}

impl AudioBanks {
    pub(crate) fn open(root: &Path) -> Option<Self> {
        let meta = std::fs::read(root.join("DATA/AUDIO/TRICKY/MOUNTAIN.BIN")).ok()?;
        if meta.len() != 16 || &meta[..4] != b"TAU1" { return None }
        let word = |at| u32::from_le_bytes(meta[at..at + 4].try_into().unwrap());
        if word(4) != 1 { return None }
        let (start, size) = (word(8) as u64, word(12) as u64);
        let mut archive = File::open(root.join("DATA/AUDIO/AUDIO.BIG")).ok()?;
        let length = archive.metadata().ok()?.len();
        let mut fixed = [0; 16];
        archive.read_exact(&mut fixed).ok()?;
        let count = u32::from_be_bytes(fixed[8..12].try_into().ok()?);
        let header = u32::from_be_bytes(fixed[12..16].try_into().ok()?) as usize;
        if &fixed[..4] != b"BIGF" || count > 1024 || !(16..=1024 * 1024).contains(&header)
            || start < header as u64 || size == 0 || start.checked_add(size)? > length
        { return None }
        let mut directory = vec![0; header - 16];
        archive.read_exact(&mut directory).ok()?;
        let mut at = 0;
        for _ in 0..count {
            let offset = u32::from_be_bytes(directory.get(at..at + 4)?.try_into().ok()?) as u64;
            let bytes = u32::from_be_bytes(directory.get(at + 4..at + 8)?.try_into().ok()?) as u64;
            let name = at + 8;
            let end = name + directory.get(name..)?.iter().position(|byte| *byte == 0)?;
            let entry = directory.get(name..end)?;
            if entry.eq_ignore_ascii_case(b"data/audio/mtn.bnk")
                || entry.eq_ignore_ascii_case(b"data\\audio\\mtn.bnk") {
                return (offset == start && bytes == size).then(|| Self {
                    root: root.join("DATA/AUDIO/TRICKY"), start, size, banks: HashMap::new(),
                });
            }
            at = end + 1;
        }
        None
    }

    /// Called after the ordinary archive read. Replace only the intersection
    /// with Mtn's fixed extent; spanning/header/other-member bytes stay native.
    pub(crate) fn apply(&mut self, name: &str, offset: u32, out: &mut [u8]) -> bool {
        if !name.eq_ignore_ascii_case("DATA/AUDIO/AUDIO.BIG") { return true }
        let Some(track) = crate::custom_tracks::active() else { return true };
        let low = (offset as u64).max(self.start);
        let high = (offset as u64 + out.len() as u64).min(self.start + self.size);
        if low >= high { return true }
        let bank = self.banks.entry(track.world_location).or_insert_with(|| {
            let path = self.root.join(format!("{}.BNK", track.world_location));
            let file = File::open(&path).ok().filter(|file|
                file.metadata().is_ok_and(|meta| meta.len() == self.size));
            if file.is_none() {
                eprintln!("[tricky audio] missing/incomplete mountain bank: {}", path.display());
            }
            file
        });
        // An older/incomplete overlay retains retail audio. The added IDs
        // then miss safely in the native range check instead of aliasing SFX.
        let Some(file) = bank else { return true };
        let begin = (low - offset as u64) as usize;
        let end = (high - offset as u64) as usize;
        file.seek(SeekFrom::Start(low - self.start)).is_ok()
            && file.read_exact(&mut out[begin..end]).is_ok()
    }
}
