//! The memory card, natively: libmc's RPC server (MCSERV, 0x80000400)
//! answered from `ssx3_mc::Fs` on the card image, instead of the
//! recompiled SIO2MAN/MCMAN/MCSERV modules talking to a modelled card.
//!
//! The game's libmc (0x409940..) is the old protocol (MCSERV 0x210 /
//! MCMAN 0x222). Requests (`args` = the EE send buffer):
//!
//! | fn   | call | arguments |
//! | ---- | ---- | --------- |
//! | 0xfe | Init | reply: 0, MCSERV version, MCMAN version |
//! | 0x01 | GetInfo | [1] port, [2] slot, [7] end-parameter block: [0] type, [1] free clusters, +0x90 formatted |
//! | 0x02 | Open (Mkdir: mode 0x40) | [0] port, [1] slot, [2] mode, name at +0x14 |
//! | 0x03 | Close | [0] fd |
//! | 0x04 | Seek | [0] fd, [4] offset, [5] origin |
//! | 0x05 | Read | [0] fd, [3] size, [6] buffer, [7] end-parameter block |
//! | 0x06 | Write | [0] fd, [5] head size (bytes at +0x20), [3] rest size, [6] rest buffer |
//! | 0x0a | Flush | [0] fd |
//! | 0x0c | Chdir | name, [4] where the previous directory goes |
//! | 0x0d | GetDir | name, [2] continue, [3] max entries, [4] table (64-byte records) |
//! | 0x0e | SetFileInfo / Rename | name, [2] fields, [4] a 64-byte record |
//! | 0x0f | Delete | name |
//! | 0x10 | Format, 0x11 Unformat | [1] port, [2] slot |
//! | 0x12 | GetEntSpace | name |
//!
//! The result is word 0 of the reply. A read's unaligned head and tail
//! normally come back through the end-parameter block, which the EE copies
//! in the RPC's end function (0x40a000: `[0]`/`[1]` sizes, `[2]`/`[3]`
//! destinations, data at +0x10/+0x50); here the whole read is written to
//! the buffer and both sizes are 0.

use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use ssx3_mc::{Card, Fs, Tod, CARD_BYTES, RAW};

use crate::cpu::Cpu;

struct Slot {
    fs: Fs,
    file: Option<File>,
    path: PathBuf,
}

fn slot() -> &'static Mutex<Option<Slot>> {
    static S: OnceLock<Mutex<Option<Slot>>> = OnceLock::new();
    S.get_or_init(|| Mutex::new(open_card()))
}

/// The card in port 1: `SSX3_MEMCARD` (default `saves/ssx3-mc0.ps2`;
/// `none` = empty slot). A missing file is created erased (the game offers
/// to format it).
fn open_card() -> Option<Slot> {
    let p = std::env::var("SSX3_MEMCARD").unwrap_or_else(|_| "saves/ssx3-mc0.ps2".into());
    if p.is_empty() || p == "none" {
        eprintln!("[mc] no memory card inserted (SSX3_MEMCARD=none)");
        return None;
    }
    let path = PathBuf::from(&p);
    let open = || -> std::io::Result<(Card, File, bool)> {
        if let Some(dir) = path.parent().filter(|d| !d.as_os_str().is_empty()) {
            std::fs::create_dir_all(dir)?;
        }
        let exists = path.exists();
        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&path)?;
        let mut v = Vec::new();
        f.read_to_end(&mut v)?;
        let card = if exists {
            Card::from_image(v)
        } else {
            Card::erased()
        };
        if f.metadata()?.len() as usize != CARD_BYTES {
            f.seek(SeekFrom::Start(0))?;
            f.write_all(&card.img)?;
            f.set_len(CARD_BYTES as u64)?;
        }
        Ok((card, f, !exists))
    };
    match open() {
        Ok((card, file, created)) => {
            eprintln!(
                "[mc] memory card in port 1 (native): {p}{}",
                if created { " (new, unformatted)" } else { "" }
            );
            Some(Slot {
                fs: Fs::new(card),
                file: Some(file),
                path,
            })
        }
        Err(e) => {
            eprintln!("[mc] cannot open {p}: {e}; no memory card inserted");
            None
        }
    }
}

impl Slot {
    /// Write the pages changed by the last request to the card file.
    fn persist(&mut self) {
        let pages = self.fs.card.take_dirty();
        let Some(f) = self.file.as_mut() else { return };
        for p in pages {
            let at = p * RAW;
            let r = f
                .seek(SeekFrom::Start(at as u64))
                .and_then(|_| f.write_all(&self.fs.card.img[at..at + RAW]));
            if let Err(e) = r {
                eprintln!("[mc] write to {}: {e}", self.path.display());
                return;
            }
        }
    }
}

/// Native memory card (the default). `SSX3_IOP_SIO2=1` runs the recompiled
/// IOP modules instead.
pub fn enabled() -> bool {
    !crate::iop_bridge::sio2_enabled()
}

fn bcd_free_secs() -> u64 {
    if let Some(v) = std::env::var("SSX3_RTC").ok().and_then(|v| v.parse().ok()) {
        v
    } else if std::env::var_os("SSX3_REAL_CLOCK").is_some() {
        web_time::SystemTime::now()
            .duration_since(web_time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    } else {
        // The fixed clock the CDVD server reports: 2026-10-14 12:00 UTC.
        1_791_979_200
    }
}

/// The card's time stamp for now (JST, like MCMAN's).
pub fn now() -> Tod {
    let t = bcd_free_secs() + 9 * 3600;
    let (days, rem) = ((t / 86400) as i64, t % 86400);
    // Days since 1970-01-01 → civil date (Howard Hinnant's algorithm).
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = (doy - (153 * mp + 2) / 5 + 1) as u8;
    let month = if mp < 10 { mp + 3 } else { mp - 9 } as u8;
    let year = (yoe + era * 400 + i64::from(month <= 2)) as u16;
    ssx3_mc::tod(
        (rem % 60) as u8,
        (rem / 60 % 60) as u8,
        (rem / 3600) as u8,
        day,
        month,
        year,
    )
}

/// RPC server for MCSERV (0x80000400).
pub fn rpc(c: &mut Cpu, method: u32, args: u32, _ssize: u32, ret: u32, rsize: u32) -> bool {
    let args = args & 0x01FF_FFFF;
    let w = |c: &mut Cpu, i: u32| c.bus.ram_u32(args + 4 * i);
    let name = |c: &mut Cpu| c.bus.read_cstr(args + 0x14);
    let put = |c: &mut Cpu, a: u32, v: u32| c.bus.set_ram_u32(a & 0x01FF_FFFF, v);
    if method == 0xfe {
        // Init: the versions libmc checks (>= 0x20a, >= 0x20e).
        put(c, ret, 0);
        if rsize >= 12 {
            put(c, ret + 4, 0x210);
            put(c, ret + 8, 0x222);
        }
        return true;
    }
    let mut guard = slot().lock().unwrap();
    let result: i32 = match guard.as_mut() {
        // An empty slot.
        None => -10,
        Some(s) => {
            let fs = &mut s.fs;
            let r = match method {
                0x01 => {
                    let (port, end) = (w(c, 1), w(c, 7));
                    if port != 0 {
                        -10
                    } else {
                        let (r, ty, free, fmt) = fs.get_info();
                        if end != 0 {
                            put(c, end, ty);
                            put(c, end + 4, free);
                            put(c, end + 0x90, fmt);
                        }
                        r
                    }
                }
                0x02 => {
                    let (port, mode, n) = (w(c, 0), w(c, 2), name(c));
                    if port != 0 {
                        -10
                    } else {
                        fs.open(&n, mode, now())
                    }
                }
                0x03 => fs.close(w(c, 0) as i32, now()),
                0x04 => fs.seek(w(c, 0) as i32, w(c, 4) as i32, w(c, 5)),
                0x05 => {
                    let (fd, size, buf, end) = (w(c, 0) as i32, w(c, 3), w(c, 6), w(c, 7));
                    match fs.read(fd, size) {
                        Ok(d) => {
                            c.bus.write_bytes(buf & 0x01FF_FFFF, &d);
                            if end != 0 {
                                put(c, end, 0);
                                put(c, end + 4, 0);
                            }
                            d.len() as i32
                        }
                        Err(e) => e,
                    }
                }
                0x06 => {
                    let (fd, rest, head, buf) = (w(c, 0) as i32, w(c, 3), w(c, 5), w(c, 6));
                    let mut data = c.bus.read_bytes(args + 0x20, head.min(16) as usize);
                    if rest > 0 {
                        data.extend_from_slice(&c.bus.read_bytes(buf & 0x01FF_FFFF, rest as usize));
                    }
                    fs.write(fd, &data)
                }
                0x0a => 0,
                0x0c => {
                    let (n, dest) = (name(c), w(c, 4));
                    match fs.chdir(&n) {
                        Ok(prev) => {
                            if dest != 0 {
                                let mut b = prev.into_bytes();
                                b.truncate(1023);
                                b.push(0);
                                c.bus.write_bytes(dest & 0x01FF_FFFF, &b);
                            }
                            0
                        }
                        Err(e) => e,
                    }
                }
                0x0d => {
                    let (cont, max, table, n) = (w(c, 2), w(c, 3), w(c, 4), name(c));
                    match fs.get_dir(&n, cont != 0, max) {
                        Ok(recs) => {
                            for (i, r) in recs.iter().enumerate() {
                                c.bus.write_bytes((table & 0x01FF_FFFF) + 64 * i as u32, r);
                            }
                            recs.len() as i32
                        }
                        Err(e) => e,
                    }
                }
                0x0e => {
                    let (flags, rec, n) = (w(c, 2), w(c, 4), name(c));
                    let rec = c.bus.read_bytes(rec & 0x01FF_FFFF, 64);
                    fs.set_file_info(&n, flags, &rec)
                }
                0x0f => {
                    let n = name(c);
                    fs.delete(&n, now())
                }
                0x10 => fs.format(now()),
                0x11 => fs.unformat(),
                0x12 => {
                    let n = name(c);
                    fs.ent_space(&n)
                }
                // ChangeThreadPriority, GetSlotMax (1), CheckNewCard.
                0x14 => 0,
                0x15 => 1,
                _ => {
                    eprintln!("[mc] unhandled MCSERV fn {method:#x}");
                    0
                }
            };
            s.persist();
            r
        }
    };
    if m!(c).sif.trace {
        eprintln!("[mc] fn {method:#x} -> {result}");
    }
    if rsize >= 4 {
        put(c, ret, result as u32);
    }
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn the_fixed_clock_is_the_cdvd_one() {
        // 2026-10-14 12:00 UTC = 21:00 JST.
        assert_eq!(super::now(), ssx3_mc::tod(0, 0, 21, 14, 10, 2026));
    }
}
