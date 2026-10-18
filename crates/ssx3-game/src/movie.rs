//! N5: Sony libmpeg's API natively, for the game's movies
//! (`docs/plans/native-rendering.md`). EA's player (`PS2_SONY_CODEC_INTERNAL`)
//! creates a decoder, registers its "no data" callback (which feeds the next
//! `MPCh` chunk with `sceMpegAddBs`) and asks for one picture at a time with
//! `sceMpegGetPicture`. Here the pictures are decoded by
//! `ssx3_mpeg::decoder::Decoder` (the IPU model's arithmetic, so the RGB32
//! macroblocks are the bytes the emulated path writes) and neither the IPU
//! nor libmpeg's EE code runs. `SSX3_NATIVE_MPEG=0` keeps them.
//!
//! The decoder's state is host-side, per `sceMpeg` object; the guest-visible
//! fields libmpeg keeps (`mp->width/height/frameCount`, the work area
//! pointer at `mp+0x40` and its end flag) are kept as libmpeg does.

use std::sync::Mutex;

use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::Cpu;
use ssx3_mpeg::decoder::{Decoder, Feed, Got};

use ssx3_ee_rt::funcs::GuestFn;

/// Replacements: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0040_2708, mpeg_init, "sceMpegInit"),
    (0x0040_27b8, mpeg_create, "sceMpegCreate"),
    (0x0040_29d0, mpeg_add_bs, "sceMpegAddBs"),
    (0x0040_2a10, mpeg_get_picture, "sceMpegGetPicture"),
    (0x0040_2b38, mpeg_is_end, "sceMpegIsEnd"),
    (0x0040_2b58, mpeg_reset, "sceMpegReset"),
    (0x0040_2c08, mpeg_add_callback, "sceMpegAddCallback"),
];

/// Whether the native libmpeg is installed (`SSX3_NATIVE_MPEG`, default on).
pub fn on() -> bool {
    std::env::var("SSX3_NATIVE_MPEG").map_or(true, |v| v != "0")
}

/// The replacements to install: the native libmpeg, or with
/// `SSX3_NATIVE_MPEG=0 SSX3_MOVIE_HASH=1` only a wrapper around the
/// generated `sceMpegGetPicture` that prints each picture's hash (to compare
/// the two paths picture by picture).
pub fn overrides() -> &'static [(u32, GuestFn, &'static str)] {
    if on() {
        OVERRIDES
    } else if hash_on() {
        &[(0x0040_2a10, generated_get_picture_hashed, "sceMpegGetPicture")]
    } else {
        &[]
    }
}

fn hash_on() -> bool {
    std::env::var_os("SSX3_MOVIE_HASH").is_some()
}

/// FNV-1a of guest RAM `[a, a + n)`, printed with the picture number.
fn print_hash(c: &Cpu, a: u32, n: usize) {
    static N: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let o = (a & 0x01ff_ffff) as usize;
    let h = c.bus.ram.get(o..o + n).map_or(0, |b| b.iter().fold(0xcbf2_9ce4_8422_2325u64, |h, &x| (h ^ x as u64).wrapping_mul(0x100_0000_01b3)));
    let k = N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    eprintln!("[movie] picture {k} hash {h:016x}");
    // (`SSX3_MOVIE_DUMP=<dir>`: the first pictures' bytes, for a diff.)
    if let (Some(dir), true) = (std::env::var_os("SSX3_MOVIE_DUMP"), k < 8) {
        if let Some(b) = c.bus.ram.get(o..o + n) {
            let _ = std::fs::write(std::path::Path::new(&dir).join(format!("pic{k}.rgba")), b);
        }
    }
}

fn generated_get_picture_hashed(c: &mut Cpu) {
    let (dst, mbcount) = (arg(c, 1), arg(c, 2) as usize);
    ssx3_ee::s15::f_00402a10__generated(c);
    print_hash(c, dst, mbcount * 1024);
}

/// One `sceMpeg` object.
struct Stream {
    mp: u32,
    work: u32,
    dec: Decoder,
    /// Callbacks by type: (function, data).
    callbacks: [(u32, u32); 8],
}

/// The decoders, by `sceMpeg` address. A stream being decoded is taken out
/// (the "no data" callback re-enters `sceMpegAddBs`, which then queues in
/// [`PENDING`]).
static STREAMS: Mutex<Vec<Stream>> = Mutex::new(Vec::new());
static PENDING: Mutex<Vec<(u32, Vec<u8>)>> = Mutex::new(Vec::new());

fn arg(c: &Cpu, i: usize) -> u32 {
    c.g[A0 + i][0] as u32
}

fn ret(c: &mut Cpu, v: u32, cycles: u64) {
    c.g[V0][0] = v as i32 as i64 as u64;
    c.cycles += cycles;
    c.pc = c.g[RA][0] as u32;
}

fn w32(c: &mut Cpu, a: u32, v: u32) {
    c.bus.write32(a, v);
}

fn w64(c: &mut Cpu, a: u32, v: u64) {
    c.bus.write64(a, v);
}

/// Guest RAM bytes `[a, a + n)` (any segment alias).
fn read_bytes(c: &Cpu, a: u32, n: usize) -> Vec<u8> {
    let o = (a & 0x01ff_ffff) as usize;
    match c.bus.ram.get(o..o + n) {
        Some(s) => s.to_vec(),
        None => Vec::new(),
    }
}

/// `int sceMpegInit(void)`: nothing to set up. Returns 1.
fn mpeg_init(c: &mut Cpu) {
    ret(c, 1, 40);
}

/// `int sceMpegCreate(sceMpeg *mp, u_char *work, int size)`: the public
/// fields as libmpeg sets them, the work area pointer (`mp+0x40`, aligned)
/// with its end flag cleared. Returns whether the work area is big enough.
fn mpeg_create(c: &mut Cpu) {
    let (mp, work, size) = (arg(c, 0), arg(c, 1), arg(c, 2) as i32);
    let aligned = (work + 3) & !3;
    let left = size - (aligned - work) as i32;
    if left <= 0x10bf {
        return ret(c, 0, 100);
    }
    for o in [0, 4, 8] {
        w32(c, mp + o, 0);
    }
    for (o, v) in [(0x10, !0u64), (0x18, !0), (0x20, 0), (0x28, !0), (0x30, !0), (0x38, 0)] {
        w64(c, mp + o, v);
    }
    w32(c, mp + 0x40, aligned);
    for o in [0, 4, 8] {
        w32(c, aligned + o, 0);
    }
    let mut s = STREAMS.lock().unwrap();
    s.retain(|x| x.mp != mp);
    s.push(Stream { mp, work: aligned, dec: Decoder::new(), callbacks: [(0, 0); 8] });
    ret(c, 1, 2000);
}

/// `int sceMpegAddBs(sceMpeg *mp, u_long128 *bs, int size)`: the bytes
/// libmpeg's DMA chain covers (`size` rounded up as it does, which takes
/// the zero quadword the codec writes after each chunk). Returns 1.
fn mpeg_add_bs(c: &mut Cpu) {
    let (mp, bs, size) = (arg(c, 0), arg(c, 1), arg(c, 2) as i32);
    let n = ((size + 19) >> 4 << 4).max(0) as usize;
    let bytes = read_bytes(c, bs, n);
    let mut s = STREAMS.lock().unwrap();
    match s.iter_mut().find(|x| x.mp == mp) {
        Some(st) => st.dec.add(&bytes),
        None => {
            drop(s);
            let mut p = PENDING.lock().unwrap();
            match p.iter_mut().find(|x| x.0 == mp) {
                Some(q) => q.1.extend_from_slice(&bytes),
                None => p.push((mp, bytes)),
            }
        }
    }
    ret(c, 1, 60 + n as u64 / 16);
}

/// `int sceMpegAddCallback(sceMpeg *mp, int type, func, void *data)`:
/// returns the previous function.
fn mpeg_add_callback(c: &mut Cpu) {
    let (mp, ty, f, data) = (arg(c, 0), arg(c, 1) as usize, arg(c, 2), arg(c, 3));
    let mut s = STREAMS.lock().unwrap();
    let old = match s.iter_mut().find(|x| x.mp == mp) {
        Some(st) if ty < 8 => std::mem::replace(&mut st.callbacks[ty], (f, data)).0,
        _ => 0,
    };
    ret(c, old, 20);
}

/// `int sceMpegIsEnd(sceMpeg *mp)`: the work area's end flag.
fn mpeg_is_end(c: &mut Cpu) {
    let mp = arg(c, 0);
    let work = c.bus.read32(mp + 0x40);
    let v = c.bus.read32(work);
    ret(c, v, 6);
}

/// `int sceMpegReset(sceMpeg *mp)`: back to the start (the bitstream and
/// the reference pictures go). Returns 1.
fn mpeg_reset(c: &mut Cpu) {
    let mp = arg(c, 0);
    let work = c.bus.read32(mp + 0x40);
    for o in [0, 4, 8] {
        w32(c, work + o, 0);
    }
    w32(c, mp + 8, 0);
    if let Some(st) = STREAMS.lock().unwrap().iter_mut().find(|x| x.mp == mp) {
        st.dec.reset();
    }
    PENDING.lock().unwrap().retain(|x| x.0 != mp);
    ret(c, 1, 200);
}

/// The "no data" callback (type 1), called when the decoder runs dry: it
/// feeds the next chunk through `sceMpegAddBs`, which queues it for us.
struct GuestFeed<'a> {
    c: &'a mut Cpu,
    mp: u32,
    work: u32,
    callback: (u32, u32),
}

impl Feed for GuestFeed<'_> {
    fn more(&mut self, out: &mut Vec<u8>) -> bool {
        let (f, data) = self.callback;
        if f == 0 {
            return false;
        }
        // (libmpeg passes a `sceMpegCbData` whose first word is the type;
        // it lives in the work area here.)
        let cbdata = self.work + 0x100;
        self.c.bus.write32(cbdata, 1);
        crate::packetless::guest_call(self.c, f, &[self.mp, cbdata, data], 0x0040_2ca0);
        let mut p = PENDING.lock().unwrap();
        match p.iter().position(|x| x.0 == self.mp) {
            Some(i) => {
                let (_, bytes) = p.swap_remove(i);
                let got = !bytes.is_empty();
                out.extend_from_slice(&bytes);
                got
            }
            None => false,
        }
    }
}

/// `int sceMpegGetPicture(sceMpeg *mp, sceIpuRGB32 *rgb32, int mbcount)`:
/// decode until a picture is ready and write it as RGB32 macroblocks. At
/// the end of the sequence the work area's end flag is set.
fn mpeg_get_picture(c: &mut Cpu) {
    let (mp, dst, mbcount) = (arg(c, 0), arg(c, 1), arg(c, 2) as usize);
    let st = {
        let mut s = STREAMS.lock().unwrap();
        s.iter().position(|x| x.mp == mp).map(|i| s.swap_remove(i))
    };
    let Some(mut st) = st else {
        return ret(c, 0, 100);
    };
    let ra = c.g[RA];
    let work = st.work;
    c.bus.write32(work, 0);
    let got = {
        let mut feed = GuestFeed { c: &mut *c, mp, work, callback: st.callbacks[1] };
        st.dec.picture(&mut feed)
    };
    c.g[RA] = ra;
    if got == Got::Picture {
        let n = (mbcount * 1024).min(st.dec.width * st.dec.height * 4);
        let o = (dst & 0x01ff_ffff) as usize;
        if let Some(out) = c.bus.ram.get_mut(o..o + n) {
            st.dec.output_rgb32(out);
            for p in (o..o + n).step_by(4096).chain([o + n - 1]) {
                ssx3_ee_rt::bus::mark_written(p);
            }
        }
        w32(c, mp + 8, st.dec.pictures - 1);
        if hash_on() {
            print_hash(c, dst, n);
        }
    }
    w32(c, mp, st.dec.width as u32);
    w32(c, mp + 4, st.dec.height as u32);
    if st.dec.ended {
        w32(c, work, 1);
    }
    if st.dec.errors != 0 {
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(|| eprintln!("[movie] decoder errors: {} (unsupported stream syntax?)", st.dec.errors));
    }
    // (Cycles: about what the IPU path takes per picture.)
    let cycles = 400 + mbcount as u64 * 600;
    STREAMS.lock().unwrap().push(st);
    ret(c, 1, cycles);
}
