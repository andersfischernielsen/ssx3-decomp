//! The driver's compiled pipelines, saved next to the descriptor keys so a
//! repeat launch skips the compile.

use super::persist::{self, Writer};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

const MAGIC: &[u8; 8] = b"SSX3PLC1";
/// Header layout: magic, renderer hash, payload length, checksum.
const RENDERER: std::ops::Range<usize> = 8..16;
const LEN: std::ops::Range<usize> = 16..20;
const SUM: std::ops::Range<usize> = 20..28;
const HEADER: usize = SUM.end;
/// Caps one driver payload (708 KB measured on an Odin 3, with room for
/// larger drivers).
const MAX_BLOB: usize = 32 * 1024 * 1024;
/// At most one driver-file write per interval; the rest waits for the next
/// flush.
const DRIVER_FLUSH_EVERY: Duration = Duration::from_secs(5);

/// The payload, and its checksum from the header (checked).
fn parse(bytes: &[u8], renderer: u64) -> Option<(&[u8], u64)> {
    if bytes.len() < HEADER || bytes.len() > HEADER + MAX_BLOB || &bytes[..8] != MAGIC {
        return None;
    }
    if u64::from_le_bytes(bytes[RENDERER].try_into().ok()?) != renderer {
        return None;
    }
    let len = u32::from_le_bytes(bytes[LEN].try_into().ok()?) as usize;
    let blob = &bytes[HEADER..];
    let sum = u64::from_le_bytes(bytes[SUM].try_into().ok()?);
    if len != blob.len() || sum != payload_hash(blob) {
        return None;
    }
    Some((blob, sum))
}

fn serialize(blob: &[u8], renderer: u64) -> Option<Vec<u8>> {
    if blob.len() > MAX_BLOB {
        return None;
    }
    let mut out = Vec::with_capacity(HEADER + blob.len());
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&renderer.to_le_bytes());
    out.extend_from_slice(&(blob.len() as u32).to_le_bytes());
    out.extend_from_slice(&payload_hash(blob).to_le_bytes());
    out.extend_from_slice(blob);
    Some(out)
}

fn payload_hash(blob: &[u8]) -> u64 {
    persist::hash(blob, persist::FNV_BASIS)
}

pub(super) struct DriverCache {
    pub(super) cache: wgpu::PipelineCache,
    path: PathBuf,
    renderer: u64,
    /// Hash of the payload on disk (0 for none): an unchanged cache is not
    /// written again.
    on_disk: Arc<AtomicU64>,
    dirty: bool,
    last_write: Option<Instant>,
    writer: Arc<Writer>,
}

impl DriverCache {
    /// The cache, from the saved data unless `fresh`; none with no cache folder.
    pub(super) fn open(
        dev: &wgpu::Device,
        info: &wgpu::AdapterInfo,
        writer: Arc<Writer>,
        fresh: bool,
    ) -> Option<Self> {
        if !dev.features().contains(wgpu::Features::PIPELINE_CACHE) {
            return None;
        }
        let key = wgpu::util::pipeline_cache_key(info)?;
        let folder = persist::dir()?;
        let path = folder.join(format!("ssx3-{key}.bin"));
        let renderer = persist::renderer_hash();
        let bytes = persist::read_saved(&path, HEADER + MAX_BLOB, fresh);
        let parsed = bytes.as_deref().and_then(|b| parse(b, renderer));
        let blob = parsed.map(|(blob, _)| blob);
        // SAFETY: this assumes the file was written by this app, from
        // `PipelineCache::get_data`. wgpu checks only its own header
        // (adapter, driver key, size) and, with `fallback`, starts empty if
        // that does not match. The envelope's checksum catches corruption,
        // not substitution (wgpu's docs say this cannot be proven).
        let cache = unsafe {
            dev.create_pipeline_cache(&wgpu::PipelineCacheDescriptor {
                label: Some("ssx3-gfx persisted pipelines"),
                data: blob,
                fallback: true,
            })
        };
        let mut store = Self {
            cache,
            path,
            renderer,
            on_disk: Arc::new(AtomicU64::new(parsed.map_or(0, |(_, sum)| sum))),
            dirty: bytes.is_some() && parsed.is_none(),
            last_write: None,
            writer,
        };
        store.flush_due();
        Some(store)
    }

    /// New pipelines may have added compiled code worth keeping.
    pub(super) fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub(super) fn flush_due(&mut self) {
        let due = self
            .last_write
            .is_none_or(|t| t.elapsed() >= DRIVER_FLUSH_EVERY);
        if !self.dirty || !due {
            return;
        }
        self.dirty = false;
        self.last_write = Some(Instant::now());
        let (cache, path, renderer) = (self.cache.clone(), self.path.clone(), self.renderer);
        let on_disk = self.on_disk.clone();
        self.writer.run(move || {
            let Some(blob) = cache.get_data() else {
                return;
            };
            let hash = payload_hash(&blob);
            if hash == on_disk.load(Ordering::Relaxed) {
                return;
            }
            // Over the cap: not written, rather than growing the cache file.
            if let Some(bytes) = serialize(&blob, renderer) {
                if persist::write(&path, &bytes).is_ok() {
                    on_disk.store(hash, Ordering::Relaxed);
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_envelope_round_trip_and_rejection() {
        let data = serialize(b"opaque wgpu payload", 123).unwrap();
        let payload = &b"opaque wgpu payload"[..];
        assert_eq!(parse(&data, 123), Some((payload, payload_hash(payload))));
        assert!(parse(&data, 124).is_none());
        let mut corrupt = data.clone();
        corrupt[HEADER] ^= 1;
        assert!(parse(&corrupt, 123).is_none());
        assert!(serialize(&vec![0; MAX_BLOB + 1], 123).is_none());
    }
}
