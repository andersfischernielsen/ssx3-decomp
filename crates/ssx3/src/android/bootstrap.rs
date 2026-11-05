//! Validate before any settings, guest, renderer worker or audio work.
use std::io::{Read, Write};
use std::path::Path;
use winit::platform::android::activity::AndroidApp;

pub fn initialize(app: &AndroidApp) -> Result<(), String> {
    let data = app.external_data_path().ok_or("External app storage is unavailable")?;
    let saves = app.internal_data_path().ok_or("Internal app storage is unavailable")?;
    let pack = data.join("ssx3.pack");
    validate_pack(&pack).map_err(|e| format!("{}: {e}. Copy a complete SSX 3 USA pack, then restart.", pack.display()))?;
    let mut source = ssx3_core::pack::Pack::open_at(&pack, 0).map_err(|e| e.to_string())?;
    validate_disc(&mut source)?;
    writable(&saves).map_err(|e| format!("Save storage unavailable: {e}. No alternate card was created."))?;
    ssx3_core::platform_paths::set(ssx3_core::platform_paths::Roots { data, saves })
        .map_err(|_| "A game session already selected storage roots")?;
    Ok(())
}

// Bound the table before Pack::open_at allocates it. Check every extent
// against both the virtual disc and the physical file, with checked sums.
fn validate_pack(path: &Path) -> std::io::Result<()> {
    let bad = || std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid pack index");
    let mut file = std::fs::File::open(path)?;
    let len = file.metadata()?.len();
    let mut h = [0; 16];
    file.read_exact(&mut h)?;
    let u32_at = |v: &[u8], o| u32::from_le_bytes(v[o..o + 4].try_into().unwrap());
    let sectors = u32_at(&h, 8);
    let count = u32_at(&h, 12);
    let table_end = 16 + u64::from(count) * 16;
    if &h[..8] != ssx3_core::pack::MAGIC || sectors == 0 || count > 1_048_576 || table_end > len {
        return Err(bad());
    }
    let mut previous = 0;
    for _ in 0..count {
        file.read_exact(&mut h)?;
        let first = u32_at(&h, 0);
        let n = u32_at(&h, 4);
        let offset = u64::from_le_bytes(h[8..].try_into().unwrap());
        let end = first.checked_add(n).ok_or_else(bad)?;
        let file_end = offset.checked_add(u64::from(n) * 2048).ok_or_else(bad)?;
        if n == 0 || first < previous || end > sectors || offset < table_end || file_end > len {
            return Err(bad());
        }
        previous = end;
    }
    Ok(())
}

fn writable(root: &Path) -> std::io::Result<()> {
    let path = root.join(".storage-check");
    let mut file = std::fs::OpenOptions::new().write(true).create_new(true).open(&path)?;
    let result = file.write_all(b"storage check").and_then(|_| file.sync_all());
    drop(file);
    let removed = std::fs::remove_file(path);
    result.and(removed)
}

fn validate_disc(source: &mut dyn ssx3_core::iso::Source) -> Result<String, String> {
    use sha2::{Digest, Sha256};
    const EXPECTED: &str = "1b49d05ca2793922180851b9e1ce9ae2291d61a7863565ac4e71f12e967af7bc";
    let invalid = |s: &str| format!("invalid or unsupported disc: {s}");
    let mut pvd = [0; 2048];
    source.read_at(16 * 2048, &mut pvd).map_err(|e| format!("truncated disc header: {e}"))?;
    if pvd[0] != 1 || &pvd[1..6] != b"CD001" || pvd[6] != 1 {
        return Err(invalid("missing ISO9660 volume descriptor"));
    }
    let read_u32 = |v: &[u8], off| u32::from_le_bytes(v[off..off + 4].try_into().unwrap()) as u64;
    let root = &pvd[156..190];
    let (offset, length) = (read_u32(root, 2) * 2048, read_u32(root, 10));
    if root[0] < 34 || length == 0 || length > 1024 * 1024 || offset + length > source.len() {
        return Err(invalid("root directory bounds"));
    }
    let mut directory = vec![0; length as usize];
    source.read_at(offset, &mut directory).map_err(|e| format!("truncated root directory: {e}"))?;
    let mut off = 0;
    while off < directory.len() {
        let n = directory[off] as usize;
        if n == 0 { off = (off / 2048 + 1) * 2048; continue; }
        if n < 34 || off + n > directory.len() || off % 2048 + n > 2048 {
            return Err(invalid("root directory record bounds"));
        }
        let record = &directory[off..off + n];
        let names = record[32] as usize;
        if names > n - 33 { return Err(invalid("directory name bounds")); }
        let name = &record[33..33 + names];
        if name == b"SLUS_207.72;1" || name == b"SLUS_207.72" {
            let (start, len) = (read_u32(record, 2) * 2048, read_u32(record, 10));
            if record[25] & 2 != 0 || len < 52 || len > 16 * 1024 * 1024 || start + len > source.len() {
                return Err(invalid("executable bounds"));
            }
            let mut elf = vec![0; len as usize];
            source.read_at(start, &mut elf).map_err(|e| format!("truncated executable: {e}"))?;
            let hash = format!("{:x}", Sha256::digest(&elf));
            if hash != EXPECTED { return Err(format!("unsupported disc ELF SHA256 {hash}; expected SSX 3 USA {EXPECTED}")); }
            return Ok(hash);
        }
        off += n;
    }
    Err(invalid("SLUS_207.72 missing (requires SSX 3 USA)"))
}

