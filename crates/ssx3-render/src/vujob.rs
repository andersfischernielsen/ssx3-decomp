//! A run of VU1 calls as bytes, for running it elsewhere (the browser
//! build's helper workers, which share no memory with the renderer): the
//! VU1 data memory it starts from and each call's program, TOP/ITOP and
//! inputs; the result is each call's GIF output and the final memory.
//!
//! A job runs exactly as the renderer runs a chunk of calls itself: each
//! call's inputs are written to memory, then the native program (if any)
//! runs ([`crate::run_native`]).
//!
//! Layout (little-endian): job = memory (16384 bytes), call count (u32),
//! per call: event index (u32), program hash (u64, 0 = none), entry (u32,
//! `u32::MAX` = none), top (u32), itop (u32), source key (u64,
//! `u64::MAX` = none; `VuCall::src`), the alpha maximum of its mesh data
//! when the renderer keeps that data already (f32, NaN = not kept; see
//! [`run_call`]), input count (u32), inputs
//! (u16 address, u128 value). Result = output count (u32), per output:
//! event index (u32), kind (u8: 0 GIF, 1 GPU), then for GIF the qword count
//! (u32) and the qwords, for GPU the [`crate::gpu::GpuCall`]; then the
//! memory.
//!
//! With `crate::gpu::enabled()`, calls with a GPU port are planned
//! ([`crate::gpu::plan`]) instead of run.

use crate::vif::VuCall;

pub const MEM: usize = 16384;

/// What a call produced: its GIF output (the program ran) or its GPU draw.
#[derive(Clone, Debug, PartialEq)]
pub enum CallOut {
    Gif(Vec<u128>),
    /// Its draws (in kick order).
    Gpu(Vec<crate::gpu::GpuCall>),
}

/// The output of each call that ran (event index, output) and the final
/// VU1 memory.
pub type Output = (Vec<(usize, CallOut)>, Box<[u8; MEM]>);

/// Debug: `SSX3_GPU_CHECK=1` compares planned GPU calls with the program.
fn check_gpu() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("SSX3_GPU_CHECK").is_some())
}

/// Run one call natively: its GPU plan (when enabled and ported) or the
/// program. None: no native program for it. `kept`: the renderer keeps the
/// call's mesh data (`crate::gpu::mesh_key` of `src`), with this alpha
/// maximum: the plan leaves the data out.
pub fn run_call(hash: u64, entry: u32, top: u32, itop: u32, src: Option<u64>, kept: Option<f32>, mem: &mut [u8; MEM]) -> Option<CallOut> {
    if crate::gpu::enabled() {
        let before = check_gpu().then(|| *mem);
        let kept = kept.filter(|_| src.is_some() && before.is_none());
        if let Some(mut g) = crate::gpu::plan(hash, entry, top, itop, kept, mem) {
            for c in g.iter_mut() {
                c.mesh_key = c.mesh_key.and(src).map(|s| crate::gpu::mesh_key(s, c.program));
            }
            if let Some(mut m) = before {
                let mut gif = Vec::new();
                if crate::run_native(hash, entry, top, itop, &mut m, &mut |p: &[u128]| gif.extend_from_slice(p)) {
                    if let Some(d) = crate::gpu::check(hash, &g, &gif) {
                        eprintln!("[gpu-check] entry {entry:#x} top {top:#x} itop {itop:#x}: {d}");
                    }
                }
            }
            return Some(CallOut::Gpu(g));
        }
    }
    let mut out = Vec::new();
    crate::run_native(hash, entry, top, itop, mem, &mut |p: &[u128]| out.extend_from_slice(p)).then_some(CallOut::Gif(out))
}

struct Reader<'a>(&'a [u8]);

impl Reader<'_> {
    fn take<const N: usize>(&mut self) -> [u8; N] {
        let (a, b) = self.0.split_at(N);
        self.0 = b;
        a.try_into().unwrap()
    }
    fn u16(&mut self) -> u16 {
        u16::from_le_bytes(self.take())
    }
    fn u32(&mut self) -> u32 {
        u32::from_le_bytes(self.take())
    }
    fn u64(&mut self) -> u64 {
        u64::from_le_bytes(self.take())
    }
    fn u128(&mut self) -> u128 {
        u128::from_le_bytes(self.take())
    }
}

/// A call of a job: event index, call, program hash, kept mesh data
/// ([`run_call`]).
pub type JobCall<'a> = (usize, &'a VuCall, Option<u64>, Option<f32>);

/// Encode a job.
pub fn encode(mem: &[u8; MEM], calls: &[JobCall]) -> Vec<u8> {
    let n: usize = calls.iter().map(|(_, c, _, _)| 40 + 18 * c.writes().count()).sum();
    let mut w = Vec::with_capacity(MEM + 4 + n);
    w.extend_from_slice(mem);
    w.extend_from_slice(&(calls.len() as u32).to_le_bytes());
    for &(i, c, h, kept) in calls {
        w.extend_from_slice(&(i as u32).to_le_bytes());
        w.extend_from_slice(&h.unwrap_or(0).to_le_bytes());
        w.extend_from_slice(&c.entry.unwrap_or(u32::MAX).to_le_bytes());
        w.extend_from_slice(&c.top.to_le_bytes());
        w.extend_from_slice(&c.itop.to_le_bytes());
        w.extend_from_slice(&c.src.unwrap_or(u64::MAX).to_le_bytes());
        w.extend_from_slice(&kept.unwrap_or(f32::NAN).to_le_bytes());
        w.extend_from_slice(&(c.writes().count() as u32).to_le_bytes());
        for &(a, v) in c.writes() {
            w.extend_from_slice(&a.to_le_bytes());
            w.extend_from_slice(&v.to_le_bytes());
        }
    }
    w
}

/// Run an encoded job; returns the encoded result.
pub fn run(job: &[u8]) -> Vec<u8> {
    let mut r = Reader(job);
    let mut mem: Box<[u8; MEM]> = Box::new(r.take());
    let n = r.u32();
    let mut out = Vec::new();
    out.extend_from_slice(&0u32.to_le_bytes());
    let mut ran = 0u32;
    for _ in 0..n {
        let (i, hash, entry, top, itop) = (r.u32(), r.u64(), r.u32(), r.u32(), r.u32());
        let src = Some(r.u64()).filter(|&s| s != u64::MAX);
        let kept = Some(f32::from_bits(r.u32())).filter(|a| !a.is_nan());
        for _ in 0..r.u32() {
            let (a, v) = (r.u16(), r.u128());
            let a = (a as usize & 1023) * 16;
            mem[a..a + 16].copy_from_slice(&v.to_le_bytes());
        }
        if hash == 0 || entry == u32::MAX {
            continue;
        }
        let Some(o) = run_call(hash, entry, top, itop, src, kept, &mut mem) else { continue };
        out.extend_from_slice(&i.to_le_bytes());
        match o {
            CallOut::Gif(q) => {
                out.push(0);
                out.extend_from_slice(&(q.len() as u32).to_le_bytes());
                for x in q {
                    out.extend_from_slice(&x.to_le_bytes());
                }
            }
            CallOut::Gpu(g) => {
                out.push(1);
                out.extend_from_slice(&(g.len() as u32).to_le_bytes());
                for c in &g {
                    crate::gpu::encode(c, &mut out);
                }
            }
        }
        ran += 1;
    }
    out[0..4].copy_from_slice(&ran.to_le_bytes());
    out.extend_from_slice(&mem[..]);
    out
}

/// Decode a result.
pub fn decode(result: &[u8]) -> Output {
    let mut r = Reader(result);
    let n = r.u32();
    let outs = (0..n)
        .map(|_| {
            let i = r.u32() as usize;
            let kind = r.take::<1>()[0];
            if kind == 1 {
                let n = r.u32();
                let g = (0..n)
                    .map(|_| {
                        let (g, used) = crate::gpu::decode(r.0);
                        r.0 = &r.0[used..];
                        g
                    })
                    .collect();
                (i, CallOut::Gpu(g))
            } else {
                let q = r.u32();
                (i, CallOut::Gif((0..q).map(|_| r.u128()).collect()))
            }
        })
        .collect();
    (outs, Box::new(r.take()))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Encoding, running and decoding equals running the calls directly
    /// (rider-program entries that run on any memory: 0x40 kicks the GIF
    /// tag at qw 0x392, 0x1150 scales the lighting coefficients in place;
    /// calls without a program or entry only write their inputs).
    #[test]
    fn round_trip_equals_direct_run() {
        let mut seed = 0x1234_5678_9abc_def0u64;
        let mut next = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        let mut mem = [0u8; MEM];
        for b in mem.iter_mut() {
            *b = next() as u8;
        }
        let calls: Vec<VuCall> = (0..6)
            .map(|k| VuCall {
                entry: (k != 2).then_some(if k % 2 == 0 { 0x40 } else { 0x1150 }),
                top: (next() % 1024) as u32,
                itop: (next() % 64) as u32,
                input: (0..8).map(|_| ((next() % 1024) as u16, next() as u128 | (next() as u128) << 64)).collect(),
                shared: (k == 1).then(|| vec![((next() % 1024) as u16, next() as u128)].into()),
                src: (k == 3).then_some(42),
            })
            .collect();
        let refs: Vec<JobCall> = calls.iter().enumerate().map(|(k, c)| (k * 3, c, (k != 4).then_some(crate::mat6::HASH), (k == 3).then_some(64.0))).collect();
        let (outs, end) = decode(&run(&encode(&mem, &refs)));

        let mut direct = Box::new(mem);
        let mut want = Vec::new();
        for &(i, c, h, kept) in &refs {
            for &(a, v) in c.writes() {
                let a = (a as usize & 1023) * 16;
                direct[a..a + 16].copy_from_slice(&v.to_le_bytes());
            }
            let (Some(e), Some(h)) = (c.entry, h) else { continue };
            if let Some(o) = run_call(h, e, c.top, c.itop, c.src, kept, &mut direct) {
                want.push((i, o));
            }
        }
        assert!(want.iter().any(|(_, o)| matches!(o, CallOut::Gif(q) if !q.is_empty())), "no program output to compare");
        assert_eq!(outs, want);
        assert_eq!(end, direct);
    }
}
