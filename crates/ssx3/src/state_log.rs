//! Per-frame state log for the headless run (`SSX3_STATE_LOG`).
//!
//! Once the byte-identical RAM gate is gone (`docs/plans/single-loop.md`), runs are
//! compared frame by frame: this file holds, for every logged frame, a hash of
//! each 4 KiB page of EE RAM. `tools/state_diff.py` finds the first frame and
//! pages where two logs differ and maps them to symbols.
//!
//! Format: the magic `SSX3STAT`, `u32` page size, `u32` page count, then per
//! record a `u32` frame number and one little-endian `u64` hash per page.

use std::io::Write;

pub const PAGE: usize = 4 * 1024;

pub struct StateLog {
    out: std::io::BufWriter<std::fs::File>,
    every: u64,
}

impl StateLog {
    pub fn create(path: &str, every: u64) -> std::io::Result<Self> {
        let mut out = std::io::BufWriter::new(std::fs::File::create(path)?);
        out.write_all(b"SSX3STAT")?;
        out.write_all(&(PAGE as u32).to_le_bytes())?;
        out.write_all(&(((32usize << 20) / PAGE) as u32).to_le_bytes())?;
        Ok(Self { out, every })
    }

    /// Log frame `f` (every `every`th one). Bytes inside `skip` (guest thread
    /// stacks: scheduler, not game, state) do not enter the hashes.
    pub fn frame(&mut self, f: u64, ram: &[u8], skip: &[(usize, usize)]) {
        if f % self.every != 0 {
            return;
        }
        let mut rec = Vec::with_capacity(4 + 8 * (ram.len() / PAGE));
        rec.extend_from_slice(&(f as u32).to_le_bytes());
        for (i, page) in ram.chunks(PAGE).enumerate() {
            let (lo, hi) = (i * PAGE, (i + 1) * PAGE);
            let hit: Vec<_> = skip.iter().filter(|&&(a, b)| a < hi && b > lo).collect();
            let h = if hit.is_empty() {
                hash(page)
            } else {
                let mut p = page.to_vec();
                for &&(a, b) in &hit {
                    p[a.saturating_sub(lo)..b.min(hi) - lo].fill(0);
                }
                hash(&p)
            };
            rec.extend_from_slice(&h.to_le_bytes());
        }
        let _ = self.out.write_all(&rec);
    }
}

/// FNV-1a over 8-byte words (the pages are 8-aligned).
fn hash(page: &[u8]) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for w in page.chunks_exact(8) {
        h = (h ^ u64::from_le_bytes(w.try_into().unwrap())).wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}
