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
//! [`run_call`]), input count (u32), inputs (u16 address, u128 value),
//! then the input kept in a [`SharedStore`] (u32 store offset, `u32::MAX`
//! = none; u32 count; the store holds the same pairs). Result = output count (u32), per output:
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
    /// A captured draw's outputs (`crate::direct::plan`), in kick order.
    Native(Vec<crate::direct::Out>),
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

/// One step of a job: a VU call, or a captured draw (event index,
/// command, VIF registers and VU1 header at it; `crate::direct::plan`),
/// which does not touch VU memory.
#[derive(Clone, Copy)]
pub enum JobItem<'a> {
    Call(JobCall<'a>),
    Native { event: usize, cmd: &'a crate::direct::Cmd, regs: crate::vif::VifRegs, header: &'a [u128; 6] },
}

impl JobItem<'_> {
    pub fn event(&self) -> usize {
        match self {
            JobItem::Call(c) => c.0,
            JobItem::Native { event, .. } => *event,
        }
    }

    /// The step's output with `mem` the VU memory before it (updated).
    pub fn run(&self, mem: &mut [u8; MEM]) -> Option<CallOut> {
        match *self {
            JobItem::Call((_, c, hash, kept)) => {
                for &(a, v) in c.writes() {
                    let a = (a as usize & 1023) * 16;
                    mem[a..a + 16].copy_from_slice(&v.to_le_bytes());
                }
                run_call(hash?, c.entry?, c.top, c.itop, c.src, kept, mem)
            }
            JobItem::Native { cmd, regs, header, .. } => Some(CallOut::Native(crate::direct::plan(cmd, regs, header, mem))),
        }
    }
}

fn put_regs(w: &mut Vec<u8>, r: &crate::vif::VifRegs) {
    let words = [r.cycle, r.mode, r.mask, r.row[0], r.row[1], r.row[2], r.row[3], r.col[0], r.col[1], r.col[2], r.col[3], r.base, r.ofst, r.tops, r.itops, r.dbf as u32];
    for x in words {
        w.extend_from_slice(&x.to_le_bytes());
    }
}

fn get_regs(r: &mut Reader) -> crate::vif::VifRegs {
    let mut x = [0u32; 16];
    for v in x.iter_mut() {
        *v = r.u32();
    }
    crate::vif::VifRegs { cycle: x[0], mode: x[1], mask: x[2], row: [x[3], x[4], x[5], x[6]], col: [x[7], x[8], x[9], x[10]], base: x[11], ofst: x[12], tops: x[13], itops: x[14], dbf: x[15] != 0 }
}

/// Where the shared input of replayed calls (`VuCall::shared`, the same
/// every frame) is kept for the jobs: stored once, then referenced.
pub trait SharedStore {
    /// The store offset of the data under `key` (`VuCall::src`), storing
    /// `bytes()` (input pairs as a job holds them) when it is new; `None`:
    /// not kept (the job carries it).
    fn offset(&mut self, key: u64, bytes: &mut dyn FnMut() -> Vec<u8>) -> Option<u32>;
}

fn put_pairs<'a>(w: &mut Vec<u8>, pairs: impl Iterator<Item = &'a (u16, u128)>) {
    for &(a, v) in pairs {
        w.extend_from_slice(&a.to_le_bytes());
        w.extend_from_slice(&v.to_le_bytes());
    }
}

/// Encode a job.
pub fn encode(mem: &[u8; MEM], items: &[JobItem]) -> Vec<u8> {
    encode_with(mem, items, None)
}

/// [`encode`], keeping shared input in `store`.
pub fn encode_with(mem: &[u8; MEM], items: &[JobItem], mut store: Option<&mut dyn SharedStore>) -> Vec<u8> {
    let n: usize = items.iter().map(|it| if let JobItem::Call((_, c, _, _)) = it { 41 + 18 * c.writes().count() } else { 256 }).sum();
    let mut w = Vec::with_capacity(MEM + 4 + n);
    w.extend_from_slice(mem);
    w.extend_from_slice(&(items.len() as u32).to_le_bytes());
    for it in items {
        let &(i, c, h, kept) = match it {
            JobItem::Call(c) => c,
            JobItem::Native { event, cmd, regs, header } => {
                w.push(1);
                w.extend_from_slice(&(*event as u32).to_le_bytes());
                put_regs(&mut w, regs);
                for q in header.iter() {
                    w.extend_from_slice(&q.to_le_bytes());
                }
                crate::direct::encode_cmd(cmd, &mut w);
                continue;
            }
        };
        w.push(0);
        w.extend_from_slice(&(i as u32).to_le_bytes());
        w.extend_from_slice(&h.unwrap_or(0).to_le_bytes());
        w.extend_from_slice(&c.entry.unwrap_or(u32::MAX).to_le_bytes());
        w.extend_from_slice(&c.top.to_le_bytes());
        w.extend_from_slice(&c.itop.to_le_bytes());
        w.extend_from_slice(&c.src.unwrap_or(u64::MAX).to_le_bytes());
        w.extend_from_slice(&kept.unwrap_or(f32::NAN).to_le_bytes());
        let stored = match (&c.shared, c.src, store.as_deref_mut()) {
            (Some(sh), Some(key), Some(st)) => st
                .offset(key, &mut || {
                    let mut b = Vec::with_capacity(18 * sh.len());
                    put_pairs(&mut b, sh.iter());
                    b
                })
                .map(|o| (o, sh.len() as u32)),
            _ => None,
        };
        match stored {
            Some((off, n)) => {
                w.extend_from_slice(&(c.input.len() as u32).to_le_bytes());
                put_pairs(&mut w, c.input.iter());
                w.extend_from_slice(&off.to_le_bytes());
                w.extend_from_slice(&n.to_le_bytes());
            }
            None => {
                w.extend_from_slice(&(c.writes().count() as u32).to_le_bytes());
                put_pairs(&mut w, c.writes());
                w.extend_from_slice(&u32::MAX.to_le_bytes());
                w.extend_from_slice(&0u32.to_le_bytes());
            }
        }
    }
    w
}

/// Run an encoded job; returns the encoded result. `store`: what the
/// [`SharedStore`] the job was encoded with holds.
pub fn run(job: &[u8], store: &[u8]) -> Vec<u8> {
    let mut r = Reader(job);
    let mut mem: Box<[u8; MEM]> = Box::new(r.take());
    let n = r.u32();
    let mut out = Vec::new();
    out.extend_from_slice(&0u32.to_le_bytes());
    let mut ran = 0u32;
    for _ in 0..n {
        if r.take::<1>()[0] == 1 {
            let i = r.u32();
            let regs = get_regs(&mut r);
            let header: [u128; 6] = std::array::from_fn(|_| r.u128());
            let mut b = crate::direct::Bytes(r.0);
            let cmd = crate::direct::decode_cmd(&mut b).expect("job command");
            r.0 = b.0;
            out.extend_from_slice(&i.to_le_bytes());
            put_out(&mut out, CallOut::Native(crate::direct::plan(&cmd, regs, &header, &mem)));
            ran += 1;
            continue;
        }
        let (i, hash, entry, top, itop) = (r.u32(), r.u64(), r.u32(), r.u32(), r.u32());
        let src = Some(r.u64()).filter(|&s| s != u64::MAX);
        let kept = Some(f32::from_bits(r.u32())).filter(|a| !a.is_nan());
        for _ in 0..r.u32() {
            let (a, v) = (r.u16(), r.u128());
            let a = (a as usize & 1023) * 16;
            mem[a..a + 16].copy_from_slice(&v.to_le_bytes());
        }
        let (off, n) = (r.u32(), r.u32());
        if off != u32::MAX {
            let mut s = Reader(&store[off as usize..off as usize + 18 * n as usize]);
            for _ in 0..n {
                let (a, v) = (s.u16(), s.u128());
                let a = (a as usize & 1023) * 16;
                mem[a..a + 16].copy_from_slice(&v.to_le_bytes());
            }
        }
        if hash == 0 || entry == u32::MAX {
            continue;
        }
        let Some(o) = run_call(hash, entry, top, itop, src, kept, &mut mem) else { continue };
        out.extend_from_slice(&i.to_le_bytes());
        put_out(&mut out, o);
        ran += 1;
    }
    out[0..4].copy_from_slice(&ran.to_le_bytes());
    out.extend_from_slice(&mem[..]);
    out
}

fn put_gif(out: &mut Vec<u8>, q: &[u128]) {
    out.extend_from_slice(&(q.len() as u32).to_le_bytes());
    for x in q {
        out.extend_from_slice(&x.to_le_bytes());
    }
}

fn put_gpu(out: &mut Vec<u8>, g: &[crate::gpu::GpuCall]) {
    out.extend_from_slice(&(g.len() as u32).to_le_bytes());
    for c in g {
        crate::gpu::encode(c, out);
    }
}

/// An output: kind (u8: 0 GIF, 1 GPU, 2 native) and its data.
fn put_out(out: &mut Vec<u8>, o: CallOut) {
    match o {
        CallOut::Gif(q) => {
            out.push(0);
            put_gif(out, &q);
        }
        CallOut::Gpu(g) => {
            out.push(1);
            put_gpu(out, &g);
        }
        CallOut::Native(v) => {
            out.push(2);
            out.extend_from_slice(&(v.len() as u32).to_le_bytes());
            for o in v {
                match o {
                    crate::direct::Out::Gif(q) => {
                        out.push(0);
                        put_gif(out, &q);
                    }
                    crate::direct::Out::Gpu(c) => {
                        out.push(1);
                        crate::gpu::encode(&c, out);
                    }
                }
            }
        }
    }
}

/// Decode a result.
pub fn decode(result: &[u8]) -> Output {
    let mut r = Reader(result);
    let n = r.u32();
    let outs = (0..n)
        .map(|_| {
            let i = r.u32() as usize;
            let gpu = |r: &mut Reader| {
                let (g, used) = crate::gpu::decode(r.0);
                r.0 = &r.0[used..];
                g
            };
            let gif = |r: &mut Reader| {
                let q = r.u32();
                (0..q).map(|_| r.u128()).collect::<Vec<u128>>()
            };
            match r.take::<1>()[0] {
                1 => {
                    let n = r.u32();
                    (i, CallOut::Gpu((0..n).map(|_| gpu(&mut r)).collect()))
                }
                2 => {
                    let n = r.u32();
                    let v = (0..n)
                        .map(|_| match r.take::<1>()[0] {
                            1 => crate::direct::Out::Gpu(gpu(&mut r)),
                            _ => crate::direct::Out::Gif(gif(&mut r)),
                        })
                        .collect();
                    (i, CallOut::Native(v))
                }
                _ => (i, CallOut::Gif(gif(&mut r))),
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
                native: false,
            })
            .collect();
        let refs: Vec<JobCall> = calls.iter().enumerate().map(|(k, c)| (k * 3, c, (k != 4).then_some(crate::mat6::HASH), (k == 3).then_some(64.0))).collect();
        let items: Vec<JobItem> = refs.iter().map(|&c| JobItem::Call(c)).collect();
        let (outs, end) = decode(&run(&encode(&mem, &items), &[]));

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

    /// A captured draw through a job equals planning it directly (a strip
    /// of 5 vertices in front of an identity header).
    #[test]
    fn native_item_round_trip() {
        let f = |v: [f32; 4]| v.iter().enumerate().fold(0u128, |q, (l, x)| q | (x.to_bits() as u128) << (32 * l));
        let i = |v: [u32; 4]| v.iter().enumerate().fold(0u128, |q, (l, x)| q | (*x as u128) << (32 * l));
        let verts: Vec<[u128; 3]> = (0..5).map(|k| [f([0.1 * k as f32, 0.5, 1.0, 0.0]), i([200, 100, 50, 128]), f([k as f32, (k % 2) as f32, 5.0, 1.0])]).collect();
        let cmd = crate::direct::Cmd::Strip { entry: 0x12b8, prim: 0x5c, verts };
        let header = [f([1.0, 0.0, 0.0, 0.0]), f([0.0, 1.0, 0.0, 0.0]), f([0.0, 0.0, 1.0, 0.0]), f([0.0, 0.0, 0.0, 1.0]), f([320.0, -224.0, 1000.0, 0.0]), f([2048.0, 2048.0, 1000.0, 0.0])];
        let regs = crate::vif::VifRegs { base: 0x10, ofst: 0x1f0, tops: 0x10, ..Default::default() };
        let mem = [0u8; MEM];
        let items = [JobItem::Native { event: 7, cmd: &cmd, regs, header: &header }];
        let (outs, _) = decode(&run(&encode(&mem, &items), &[]));
        let want = crate::direct::plan(&cmd, regs, &header, &mem);
        assert!(!want.is_empty());
        assert_eq!(outs, vec![(7, CallOut::Native(want))]);
    }

    /// Shared input kept in a store: the same results as carried inline.
    #[test]
    fn shared_store_round_trip() {
        struct Store(Vec<u8>, std::collections::HashMap<u64, u32>);
        impl SharedStore for Store {
            fn offset(&mut self, key: u64, bytes: &mut dyn FnMut() -> Vec<u8>) -> Option<u32> {
                if let Some(&o) = self.1.get(&key) {
                    return Some(o);
                }
                let o = self.0.len() as u32;
                self.0.extend(bytes());
                self.1.insert(key, o);
                Some(o)
            }
        }
        let mut mem = [0u8; MEM];
        for (i, b) in mem.iter_mut().enumerate() {
            *b = (i * 7) as u8;
        }
        let shared: std::sync::Arc<[(u16, u128)]> = (0..40u16).map(|k| (0x200 + k, (k as u128) << 90 | 0x3f80_0000)).collect::<Vec<_>>().into();
        let calls: Vec<VuCall> = (0..3)
            .map(|k| VuCall { entry: Some(0x1150), top: 0, itop: 0, input: vec![(0x10 + k, 99)], shared: Some(shared.clone()), src: Some(5 << 16 | k as u64), native: false })
            .collect();
        let items: Vec<JobItem> = calls.iter().map(|c| JobItem::Call((1, c, Some(crate::mat6::HASH), None))).collect();
        let inline = run(&encode(&mem, &items), &[]);
        let mut st = Store(Vec::new(), Default::default());
        let job = encode_with(&mem, &items, Some(&mut st));
        assert!(job.len() < encode(&mem, &items).len());
        assert_eq!(run(&job, &st.0), inline);
    }
}
