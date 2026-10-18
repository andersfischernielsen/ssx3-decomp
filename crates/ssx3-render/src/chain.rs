//! A source-chain DMA walk (VIF1, `CHCR.MOD` = chain), as the DMAC does it
//! (`ssx3-ps2` `hw.rs` `dma_chain`).

/// EE memory as the DMAC sees it.
pub trait Mem {
    /// The quadword at `addr` (bit 31 set: scratchpad).
    fn qword(&self, addr: u32) -> u128;

    /// `n` quadwords from `addr` appended to `out`.
    fn read_qwords(&self, addr: u32, n: u32, out: &mut Vec<u128>) {
        out.extend((0..n).map(|i| self.qword(addr + 16 * i)));
    }

    /// An id of this memory's state, growing with each snapshot (0: none).
    fn snapshot(&self) -> u64 {
        0
    }

    /// No byte in `a..b` was written after snapshot `since` (false when
    /// not known).
    fn unchanged(&self, _a: u32, _b: u32, _since: u64) -> bool {
        false
    }
}

/// Where a walk stopped.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WalkEnd {
    /// TADR afterwards (where a restarted channel continues).
    pub tadr: u32,
    /// Call stack depth left (`CHCR.ASP`).
    pub asp: u32,
    pub tags: u32,
}

/// Walk the chain from `tadr`, handing each transfer's quadwords (and, with
/// `tte`, the upper half of each tag) to `sink`. Stops at END/REFE, at a
/// RET with an empty stack, or at a tag with the IRQ bit when `tie` is set.
pub fn walk(mem: &dyn Mem, tadr: u32, tte: bool, tie: bool, asp: u32, asr: &mut [u32; 2], sink: &mut dyn FnMut(&[u128])) -> WalkEnd {
    let mut q = Vec::new();
    walk_segments(mem, tadr, tte, tie, asp, asr, &mut |s| match s {
        Segment::Tag(t) => sink(&[t]),
        Segment::Data { addr, qwc } => {
            q.clear();
            q.extend((0..qwc).map(|i| mem.qword(addr + i * 16)));
            sink(&q);
        }
    })
}

/// What a chain transfers, without reading the data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Segment {
    /// The upper half of a tag (with `tte`).
    Tag(u128),
    /// `qwc` quadwords at `addr`.
    Data { addr: u32, qwc: u32 },
}

/// [`walk`], handing over where each transfer's data is instead of the data.
pub fn walk_segments(mem: &dyn Mem, mut tadr: u32, tte: bool, tie: bool, mut asp: u32, asr: &mut [u32; 2], sink: &mut dyn FnMut(Segment)) -> WalkEnd {
    let mut tags = 0;
    for _ in 0..1_000_000 {
        tags += 1;
        let tag = mem.qword(tadr);
        let qwc = (tag & 0xffff) as u32;
        let id = ((tag >> 28) & 7) as u32;
        let irq = (tag >> 31) & 1 != 0;
        let addr = ((tag >> 32) & 0x7fff_fff0) as u32;
        let tag_spr = (tag >> 63) & 1 != 0;
        if tte {
            sink(Segment::Tag(tag & !0xffff_ffff_ffff_ffffu128));
        }
        let spr = |a: u32, s: bool| if s { a | 0x8000_0000 } else { a };
        let (data, mut next, mut end) = match id {
            0 => (spr(addr, tag_spr), tadr, true),
            1 => (tadr + 16, tadr + 16 + qwc * 16, false),
            2 => (tadr + 16, addr, false),
            3 | 4 => (spr(addr, tag_spr), tadr + 16, false),
            5 => {
                if asp < 2 {
                    asr[asp as usize] = tadr + 16 + qwc * 16;
                    asp += 1;
                }
                (tadr + 16, addr, false)
            }
            6 => {
                if asp > 0 {
                    asp -= 1;
                    (tadr + 16, asr[asp as usize], false)
                } else {
                    (tadr + 16, tadr, true)
                }
            }
            _ => (tadr + 16, tadr, true),
        };
        sink(Segment::Data { addr: data, qwc });
        if irq && tie {
            end = true;
        }
        if id == 0 {
            next = tadr + 16;
        }
        tadr = next;
        if end {
            break;
        }
    }
    WalkEnd { tadr, asp, tags }
}
