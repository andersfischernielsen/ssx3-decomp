//! What the renderer reads of guest memory besides the typed frame, taken
//! when the frame is built (on the game's thread, from live memory): the
//! flip packet, the PATH3 packets, and the ranges holding the textures the
//! frame can sample (TexMan's table, the TexInfos, pixels and CLUTs).

use crate::chain::Mem;

use super::{gm, word, Frame};

/// Renderer_SetDisplayRegs (0x382af0) builds a GS state packet at
/// 0x4ffcc0 on every flip and sends it by normal GIF DMA, outside every
/// chain. Its last tag writes SCISSOR_1/2 (NLOOP 2, A+D, EOP).
const FLIP_PACKET: u32 = 0x4ffcc0;

pub fn flip_packet(mem: &dyn Mem) -> Vec<u128> {
    let mut q = Vec::new();
    let mut a = FLIP_PACKET;
    while q.len() < 512 {
        let tag = mem.qword(a);
        let lo = tag as u64;
        let nloop = (lo & 0x7fff) as usize;
        let flg = (lo >> 58) & 3;
        let nreg = match (lo >> 60) as usize { 0 => 16, n => n };
        let n = match flg {
            0 => nloop * nreg,
            1 => (nloop * nreg).div_ceil(2),
            _ => nloop,
        };
        for k in 0..=n as u32 {
            q.push(mem.qword(a + 16 * k));
        }
        a += 16 * (n as u32 + 1);
        if lo == 0x1000_0000_0000_8002 && (tag >> 64) as u64 == 0xe {
            break;
        }
    }
    q
}

/// The GIF packets of a PATH3 chain (texture uploads),
/// read straight from its transfers. The data of IMAGE transfers longer
/// than a CLUT is left out (their tags keep NLOOP = 0): the renderer takes
/// textures from TexMan and keeps only small uploads (`gs::Gs::uploads`).
pub fn path3_packets(mem: &dyn Mem, head: u32) -> Vec<Vec<u128>> {
    use crate::chain::{walk_segments, Segment};
    const KEPT_IMAGE: usize = 256;
    let mut out = Vec::new();
    if head == 0 {
        return out;
    }
    let mut segs: Vec<(u32, u32)> = Vec::new();
    let mut asr = [0u32; 2];
    walk_segments(mem, head, false, false, 0, &mut asr, &mut |s| {
        if let Segment::Data { addr, qwc } = s {
            if qwc > 0 {
                segs.push((addr, qwc));
            }
        }
    });
    // A cursor over the transfers: (segment, quadword in it).
    let (mut si, mut qi) = (0usize, 0u32);
    let mut next = |skip: usize| -> Option<u128> {
        // (Past `skip` quadwords first.)
        let mut skip = skip as u32;
        loop {
            let &(a, n) = segs.get(si)?;
            if qi + skip < n {
                qi += skip + 1;
                return Some(mem.qword(a + 16 * (qi - 1)));
            }
            skip -= n - qi;
            si += 1;
            qi = 0;
        }
    };
    let mut cur: Vec<u128> = Vec::new();
    let mut pending = 0usize;
    while let Some(tag) = next(std::mem::take(&mut pending)) {
        let lo = tag as u64;
        let nloop = (lo & 0x7fff) as usize;
        let eop = (lo >> 15) & 1 != 0;
        let nreg = match (lo >> 60) as usize {
            0 => 16,
            n => n,
        };
        let flg = (lo >> 58) & 3;
        let n = match flg {
            0 => nloop * nreg,
            1 => (nloop * nreg).div_ceil(2),
            _ => nloop,
        };
        if flg >= 2 && n > KEPT_IMAGE {
            cur.push(tag & !0x7fff);
            pending = n;
        } else {
            cur.push(tag);
            for _ in 0..n {
                match next(0) {
                    Some(q) => cur.push(q),
                    None => break,
                }
            }
        }
        if eop {
            out.push(std::mem::take(&mut cur));
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

/// TexInfo fields read here (`structs/render.md` §TexMan; the renderer's
/// `tex::ti`).
mod ti {
    pub const SIZE: u32 = 0x58;
    pub const PIXELS: u32 = 0x1c;
    pub const CLUT: u32 = 0x20;
    pub const TBP: u32 = 0x28;
    pub const TEX0: u32 = 0x38;
}

/// TexMan's `TexInfo *tex[2000]`.
const TEX_TABLE: u32 = 0x8;
const TEX_MAX: u32 = 2000;

/// The guest memory ranges `[a, b)` the renderer may read for `frame`'s
/// textures: TexMan's table and every TexInfo (the resident textures are
/// looked up by VRAM address), and the pixels and CLUT of each texture a
/// draw binds or that has a permanent VRAM block. Pixels are bounded by
/// TEX0's size at 32 bits per texel (every format reads less).
pub fn texture_ranges(m: &dyn Mem, frame: &Frame, out: &mut Vec<(u32, u32)>) {
    let texman = word(m, frame.gm + gm::TEXMAN);
    if texman == 0 {
        return;
    }
    out.push((frame.gm + gm::TEXMAN, frame.gm + gm::TEXMAN + 4));
    out.push((texman + TEX_TABLE, texman + TEX_TABLE + 4 * TEX_MAX));
    let mut used = vec![false; TEX_MAX as usize];
    for d in frame.draws() {
        for id in d.tex.iter().flatten() {
            if let Some(u) = used.get_mut(*id as usize) {
                *u = true;
            }
        }
    }
    for id in 0..TEX_MAX {
        let info = word(m, texman + TEX_TABLE + 4 * id);
        if info == 0 {
            continue;
        }
        out.push((info, info + ti::SIZE));
        if !used[id as usize] && (word(m, info + ti::TBP) as i32) < 0 {
            continue;
        }
        let t0 = word(m, info + ti::TEX0) as u64 | (word(m, info + ti::TEX0 + 4) as u64) << 32;
        let (w, h) = (1u32 << ((t0 >> 26) & 15).min(10), 1u32 << ((t0 >> 30) & 15).min(10));
        let pixels = word(m, info + ti::PIXELS);
        if pixels != 0 {
            out.push((pixels, pixels.saturating_add(w * h * 4 + 16)));
        }
        let clut = word(m, info + ti::CLUT);
        if clut != 0 {
            out.push((clut, clut.saturating_add(0x410)));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::path3_packets;

    struct Ram(Vec<u128>);
    impl crate::chain::Mem for Ram {
        fn qword(&self, a: u32) -> u128 {
            self.0[(a / 16) as usize]
        }
    }

    #[test]
    fn path3_packets_end_at_eop_and_drop_large_images() {
        // CNT with: A+D tag (2 regs, no EOP), its 2 qwords; IMAGE tag (3
        // qwords, EOP), its data; then REF to: A+D tag with EOP and 1
        // qword, IMAGE of 300 qwords (EOP); END.
        let ad = |n: u128, eop: bool| n | (eop as u128) << 15 | 1 << 60 | 0xe << 64;
        let image = |n: u128| n | 1 << 15 | 2 << 58;
        let mut m = vec![0u128; 1024];
        let cnt = [ad(2, false), 1, 2, image(3), 3, 4, 5];
        // (Head at qword 0x10: a zero head is no chain.)
        m[0x10] = 0x1000_0000 | cnt.len() as u128;
        m[0x11..0x11 + cnt.len()].copy_from_slice(&cnt);
        let at = 0x11 + cnt.len();
        m[at] = 0x3000_0000 | 303 | (0x200u128 * 16) << 32;
        m[at + 1] = 0x7000_0000;
        m[0x200] = ad(1, true);
        m[0x201] = 6;
        m[0x202] = image(300);
        let p = path3_packets(&Ram(m), 16 * 0x10);
        assert_eq!(p.len(), 3);
        assert_eq!(p[0], cnt.to_vec());
        assert_eq!(p[1], vec![ad(1, true), 6]);
        assert_eq!(p[2], vec![image(300) & !0x7fff]);
    }
}
