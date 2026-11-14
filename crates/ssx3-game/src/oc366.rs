//! Staging module oc366 (wave wave216): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc366`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0036_7440, tex_man_load_texture, "TexMan_LoadTexture")];

// ---------------------------------------------------------------------------
// TexMan_LoadTexture (0x367440)

/// `TexInfo` fields written by `TexMan_LoadTexture`.
pub(crate) mod tex_load_info {
    /// Non-zero: the manager owns (copied) the pixel and CLUT data.
    pub const OWNED: u32 = 0x00;
    /// Eight-character texture name.
    pub const NAME: u32 = 0x04;
    pub const FORMAT: u32 = 0x0c;
    pub const DATA_BYTES: u32 = 0x10;
    pub const VRAM_BYTES: u32 = 0x14;
    pub const BPP: u32 = 0x18;
    pub const PIXELS: u32 = 0x1c;
    pub const CLUT: u32 = 0x20;
    pub const MIPS: u32 = 0x24;
    pub const VRAM_BLOCK: u32 = 0x28;
    pub const VRAM_NODE: u32 = 0x30;
    pub const UNUSED_2C: u32 = 0x2c;
    pub const UNUSED_34: u32 = 0x34;
    /// GS TEX0, TEX1 and MIPTBP1/2 register images.
    pub const TEX0: u32 = 0x38;
    pub const TEX1: u32 = 0x40;
    pub const MIPTBP1: u32 = 0x48;
    pub const MIPTBP2: u32 = 0x50;
}

/// `TexMan` (PS2 graphics manager) fields the loader reads.
pub(crate) mod tex_load_man {
    /// Texture slot pointers (`TexInfo*`) from +8.
    pub const SLOTS: u32 = 0x8;
    /// Allocation flags OR-ed into every `operator new`.
    pub const ALLOC_FLAGS: u32 = 0x1f58;
    /// The embedded VRAM heap and its node array (`0x1c`-byte nodes).
    pub const VRAM_HEAP: u32 = 0x1f60;
    pub const VRAM_NODES: u32 = 0x3f50;
    /// VRAM byte budget above which owned PSMT8 textures are swizzled into
    /// a temporary copy instead of being uploaded as they are.
    pub const T8_LIMIT: u32 = 0x6344;
}

const TEX_LOAD_NB_ALLOC: Callee = Callee {
    addr: 0x0036_7150,
    f: ee::s12::TexMan_AllocNBTexInfo,
};
const TEX_LOAD_B_ALLOC: Callee = Callee {
    addr: 0x0036_7260,
    f: ee::s12::TexMan_AllocBTexInfo,
};
const TEX_LOAD_VRAM_ALLOC: Callee = Callee {
    addr: 0x0036_6ce0,
    f: ee::s12::VramHeap_Alloc,
};
const TEX_LOAD_COPY_TO_TEMP: Callee = Callee {
    addr: 0x0036_6e90,
    f: ee::s12::TexMan_CopyToTempTex,
};
const TEX_LOAD_STRNCPY: u32 = 0x0041_6ce0;
const TEX_LOAD_NEW: u32 = 0x0031_7e30;
const TEX_LOAD_MEMCOPY: u32 = 0x003e_6574;

/// The jump tables (`.rodata`) the original switches through.
pub(crate) const TEX_LOAD_FMT_TABLE: u32 = 0x0049_2090;
pub(crate) const TEX_LOAD_PSM_TABLE: u32 = 0x0049_20d0;
pub(crate) const TEX_LOAD_PAGE_TABLE: u32 = 0x0049_21c0;
pub(crate) const TEX_LOAD_PAGE_TABLE2: u32 = 0x0049_2240;

/// Texture format (`fmt < 13`) -> (case address, GS PSM). Formats outside
/// the table use PSMCT32 (0).
fn tex_load_fmt_case(fmt: u64) -> (u32, u64) {
    match fmt {
        2 => (0x0036_7524, 0x02),
        8 => (0x0036_751c, 0x01),
        9 | 10 => (0x0036_752c, 0x13),
        11 | 12 => (0x0036_7534, 0x14),
        _ => (0x0036_753c, 0x00),
    }
}

/// GS PSM (`< 0x3b`) -> (case address, bits per pixel); 0 bpp for PSMs
/// outside the switch.
fn tex_load_psm_case(psm: u64) -> (u32, u64) {
    match psm {
        0x00 | 0x01 | 0x30 | 0x31 => (0x0036_7564, 0x20),
        0x02 | 0x0a | 0x32 | 0x3a => (0x0036_756c, 0x10),
        0x13 | 0x1b => (0x0036_7574, 0x08),
        0x14 | 0x24 | 0x2c => (0x0036_757c, 0x04),
        _ => (0x0036_7584, 0),
    }
}

/// `bpp - 4` -> which GS page shape: 4 bpp 128x128, 8 bpp 128x64,
/// 15/16 bpp 64x64, else (32 bpp) 64x32.
fn tex_load_page_case(idx: u64) -> usize {
    match idx {
        0 => 0,
        4 => 1,
        11 | 12 => 2,
        _ => 3,
    }
}

/// Page (width, height) per case, the case addresses of the loop's switch
/// and of the no-data switch.
const TEX_LOAD_PAGE_DIMS: [(u64, u64); 4] =
    [(0x80, 0x80), (0x80, 0x40), (0x40, 0x40), (0x40, 0x20)];
const TEX_LOAD_PAGE_CASES: [u32; 4] = [0x0036_7640, 0x0036_764c, 0x0036_7658, 0x0036_7664];
const TEX_LOAD_PAGE_CASES2: [u32; 4] = [0x0036_7734, 0x0036_7740, 0x0036_774c, 0x0036_7758];

/// The page switch on `bpp - 4` held in `idx_reg` (`sltiu 29`, table at
/// `table`): leaves `v0`, `v1` (table slot) and `a0` (case address) as the
/// original does and returns the page (width, height). The jump costs 7
/// cycles, the case 3 (2 for the default).
fn tex_load_page_switch(c: &mut Cpu, idx: u64, table: u32, cases: &[u32; 4]) -> (u64, u64) {
    let case = if idx < 0x1d {
        c.cycles += 7;
        c.g[V0][0] = sx(table);
        c.g[V1][0] = sx(((idx as u32) << 2).wrapping_add(table));
        let k = tex_load_page_case(idx);
        c.g[A0][0] = sx(cases[k]);
        k
    } else {
        3
    };
    c.cycles += if case == 3 { 2 } else { 3 };
    TEX_LOAD_PAGE_DIMS[case]
}

/// `int TexMan_LoadTexture(TexMan* man, void* pixels, const char* name,
/// int w, int h, int mips, int fmt, int clutFmt, void* clut, int vramMode,
/// int owned, int id)` (0x367440; the last four on the stack): takes a
/// texture slot (a fresh one for `id < 0`, else slot `id`), names it,
/// works out the GS PSM and bits per pixel from `fmt`, the data and VRAM
/// sizes of every mip level, copies (owned) or references the pixels and
/// the CLUT, builds the TEX0/TEX1/MIPTBP register images, reserves VRAM now
/// for `vramMode` 1 (or 3: 0x2000 extra, 32-block aligned), and for PSMT8
/// either swizzles into a temporary copy (owned, over budget, mode 0) or
/// retags the PSM as T8H. Returns the slot id (-1 for an unknown PSM).
pub fn tex_man_load_texture(c: &mut Cpu) {
    use tex_load_info as ti;
    use tex_load_man as tm;
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0xe0);
        frame.save(c, S0, 0xd0);
        frame.save(c, S1, 0xc0);
        frame.save(c, S2, 0xb0);
        c.g[S1][0] = c.g[T2][0]; // fmt
        frame.save(c, S3, 0xa0);
        c.g[S2][0] = c.g[T0][0]; // h
        frame.save(c, S4, 0x90);
        c.g[S3][0] = c.g[A3][0]; // w
        frame.save(c, S5, 0x80);
        c.g[S4][0] = c.g[T3][0]; // clutFmt
        frame.save(c, S6, 0x70);
        c.g[S5][0] = c.g[T1][0]; // mips
        frame.save(c, S7, 0x60);
        c.g[S6][0] = c.g[A0][0]; // man
        frame.save(c, FP, 0x50);
        c.g[S7][0] = c.g[A1][0]; // pixels
        c.g[S0][0] = Addr(frame.at(c, 0xf8)).lw(c); // id
        c.g[FP][0] = c.g[A2][0]; // name
        frame.save_ra(c, 0x40);

        // The slot: `id` (spilled at sp+0).
        c.cycles += 4;
        if (c.g[S0][0] as i64) >= 0 {
            c.g[A0][0] = c.g[S6][0];
            c.g[A1][0] = c.g[S0][0];
            TEX_LOAD_B_ALLOC.call(c, 0x0036_74b0)?;
            Addr(sp(c)).sw(c, c.g[S0][0]);
        } else {
            TEX_LOAD_NB_ALLOC.call(c, 0x0036_749c)?;
            Addr(sp(c)).sw(c, c.g[V0][0]);
        }

        // tex = man->slots[id]; strncpy(tex->name, name, 8).
        c.cycles += 19;
        c.g[V1][0] = Addr(sp(c)).lw(c);
        c.g[A1][0] = c.g[FP][0];
        c.g[A2][0] = 8;
        c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[S0][0] = Addr(c.g[V0][0] as u32).off(tm::SLOTS).lw(c);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(ti::NAME));
        call_ptr(c, TEX_LOAD_STRNCPY, 0x0036_74d4)?;
        let tex = Addr(c.g[S0][0] as u32);
        c.g[V0][0] = Addr(frame.at(c, 0xf0)).lw(c); // owned
        c.g[V1][0] = u64::MAX;
        tex.off(ti::UNUSED_34).sw(c, c.g[V1][0]);
        tex.off(ti::OWNED).sw(c, c.g[V0][0]);
        c.g[V0][0] = (c.g[S1][0] < 0xd) as u64;
        tex.off(ti::FORMAT).sw(c, c.g[S1][0]);
        tex.off(ti::VRAM_BLOCK).sw(c, c.g[V1][0]);
        tex.off(ti::VRAM_NODE).sw(c, c.g[V1][0]);
        tex.off(ti::UNUSED_2C).sw(c, c.g[V1][0]);
        tex.off(ti::MIPS).sw(c, c.g[S5][0]);

        // fmt -> PSM (a1).
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            c.g[A1][0] = 0;
        } else {
            c.cycles += 7;
            let fmt = c.g[S1][0];
            c.g[V0][0] = sx(TEX_LOAD_FMT_TABLE);
            c.g[V1][0] = sx(((fmt as u32) << 2).wrapping_add(TEX_LOAD_FMT_TABLE));
            let (case, psm) = tex_load_fmt_case(fmt);
            c.g[A0][0] = sx(case);
            c.cycles += if psm == 0 { 1 } else { 2 };
            c.g[A1][0] = psm;
        }

        // PSM -> bits per pixel (t3).
        c.cycles += 3;
        let psm = c.g[A1][0];
        c.g[V0][0] = sx(0x0049_0000);
        if psm < 0x3b {
            c.cycles += 6;
            c.g[V1][0] = sx(((psm as u32) << 2).wrapping_add(TEX_LOAD_PSM_TABLE));
            c.g[V0][0] = sx(TEX_LOAD_PSM_TABLE);
            let (case, bpp) = tex_load_psm_case(psm);
            c.g[A0][0] = sx(case);
            c.cycles += if bpp == 0 { 1 } else { 2 };
            c.g[T3][0] = bpp;
        } else {
            c.cycles += 1;
            c.g[T3][0] = 0;
        }
        c.cycles += 2;
        tex.off(ti::BPP).sw(c, c.g[T3][0]);
        if (c.g[T3][0] as i64) <= 0 {
            // Not reachable: every PSM the format switch yields has a size.
            c.cycles += 2;
            c.g[V0][0] = u64::MAX;
            tex_load_texture_exit(c, frame);
            return Ok(());
        }

        c.cycles += 2;
        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(1)); // w - 1
        if c.g[S7][0] != 0 {
            // Pixels given: sum the data and VRAM bytes of every mip level.
            c.cycles += 16;
            Addr(frame.at(c, 0x18)).sd(c, c.g[A1][0]); // psm
            c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
            c.g[A1][0] = sx(tex.off(ti::TEX0).0);
            Addr(frame.at(c, 0x20)).sw(c, c.g[V0][0]);
            Addr(frame.at(c, 0x8)).sw(c, c.g[A1][0]);
            c.g[V0][0] = ((c.g[T3][0] as i64) < 9) as u64;
            Addr(frame.at(c, 0x30)).sw(c, c.g[V1][0]);
            c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x3f));
            c.g[V1][0] = c.g[S5][0] << 32;
            c.g[S1][0] = 0; // data bytes
            c.g[T5][0] = 0; // VRAM bytes
            c.g[T4][0] = 0; // level
            Addr(frame.at(c, 0x4)).sw(c, c.g[A1][0]);
            Addr(frame.at(c, 0xc)).sw(c, c.g[V0][0]);
            Addr(frame.at(c, 0x10)).sd(c, c.g[V1][0]);
            if (c.g[S5][0] as i64) >= 0 {
                loop {
                    c.cycles += 17;
                    let lv = c.g[T4][0] as u32 & 31;
                    c.g[T1][0] = sx((c.g[S3][0] as u32) >> lv);
                    c.g[T2][0] = sx((c.g[S2][0] as u32) >> lv);
                    c.mult(0, V1, T1, T2);
                    c.g[A0][0] = u64::MAX;
                    c.g[A1][0] = 0xff;
                    c.g[A2][0] = 0x100;
                    c.g[T0][0] = sx((c.g[T3][0] as u32).wrapping_sub(4));
                    c.g[A3][0] = (c.g[T0][0] < 0x1d) as u64;
                    c.mult(0, V1, V1, T3);
                    c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(7));
                    if c.g[A0][0] != 0 {
                        c.g[V0][0] = c.g[V1][0];
                    }
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
                    c.g[A1][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[A1][0] == 0 {
                        c.g[V0][0] = c.g[A2][0];
                    }
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let idx = if c.g[A3][0] != 0 {
                        c.g[T0][0]
                    } else {
                        u64::MAX
                    };
                    let (pw, ph) =
                        tex_load_page_switch(c, idx, TEX_LOAD_PAGE_TABLE, &TEX_LOAD_PAGE_CASES);
                    c.g[A1][0] = pw;
                    c.g[V1][0] = ph;

                    c.cycles += 18;
                    c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    c.g[A0][0] = ((c.g[T2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[A1][0] = c.g[T1][0];
                    }
                    if c.g[A0][0] == 0 {
                        c.g[V1][0] = c.g[T2][0];
                    }
                    c.mult(0, V1, A1, V1);
                    c.g[A0][0] = u64::MAX;
                    c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(1));
                    c.g[A1][0] = ((c.g[S5][0] as i64) < (c.g[T4][0] as i64)) as u64;
                    c.mult(0, V1, V1, T3);
                    c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(7));
                    if c.g[A0][0] != 0 {
                        c.g[V0][0] = c.g[V1][0];
                    }
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
                    c.g[V1][0] = sx(0u32.wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] &= 0xff;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[T5][0] = sx((c.g[T5][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    if c.g[A1][0] != 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0036_75e0);
                    }
                }
            }
            c.cycles += 4;
            c.g[V0][0] = tex.off(ti::OWNED).lw(c);
            tex.off(ti::VRAM_BYTES).sw(c, c.g[T5][0]);
            tex.off(ti::DATA_BYTES).sw(c, c.g[S1][0]);
            if c.g[V0][0] != 0 {
                // Owned: copy the pixels into a fresh block.
                c.cycles += 14;
                c.g[V0][0] = Addr(c.g[S6][0] as u32).off(tm::ALLOC_FLAGS).lw(c);
                c.g[A2][0] = 0x0500_0000;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[FP][0];
                c.g[A2][0] |= c.g[V0][0];
                c.g[A3][0] = 0;
                call_ptr(c, TEX_LOAD_NEW, 0x0036_76e0)?;
                c.g[A1][0] = c.g[S7][0];
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S1][0];
                Addr(c.g[S0][0] as u32).off(ti::PIXELS).sw(c, c.g[V0][0]);
                call_ptr(c, TEX_LOAD_MEMCOPY, 0x0036_76f4)?;
                c.g[T3][0] = Addr(frame.at(c, 0xe0)).lw(c); // clut
            } else {
                c.cycles += 2;
                tex.off(ti::PIXELS).sw(c, c.g[S7][0]);
                c.cycles += 1;
                c.g[T3][0] = Addr(frame.at(c, 0xe0)).lw(c);
            }
        } else {
            // No pixels: only the top level's VRAM size.
            c.cycles += 5;
            c.g[V1][0] = sx((c.g[T3][0] as u32).wrapping_sub(4));
            tex.off(ti::PIXELS).sw(c, 0);
            c.g[V0][0] = (c.g[V1][0] < 0x1d) as u64;
            tex.off(ti::DATA_BYTES).sw(c, 0);
            let idx = if c.g[V0][0] != 0 {
                c.g[V1][0]
            } else {
                u64::MAX
            };
            let (pw, ph) =
                tex_load_page_switch(c, idx, TEX_LOAD_PAGE_TABLE2, &TEX_LOAD_PAGE_CASES2);
            c.g[A2][0] = pw;
            c.g[A0][0] = ph;

            c.cycles += 28;
            c.g[V1][0] = ((c.g[S2][0] as i64) < (c.g[A0][0] as i64)) as u64;
            c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[A2][0] as i64)) as u64;
            if c.g[V1][0] == 0 {
                c.g[A0][0] = c.g[S2][0];
            }
            if c.g[V0][0] == 0 {
                c.g[A2][0] = c.g[S3][0];
            }
            c.mult(0, A0, A2, A0);
            c.g[V0][0] = sx(tex.off(ti::TEX0).0);
            Addr(frame.at(c, 0x8)).sw(c, c.g[V0][0]);
            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x3f));
            Addr(frame.at(c, 0x18)).sd(c, c.g[A1][0]); // psm
            Addr(frame.at(c, 0x4)).sw(c, c.g[V0][0]);
            c.g[V1][0] = u64::MAX;
            c.mult(0, A0, A0, T3);
            c.g[A1][0] = ((c.g[T3][0] as i64) < 9) as u64;
            Addr(frame.at(c, 0xc)).sw(c, c.g[A1][0]);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
            c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_sub(1));
            Addr(frame.at(c, 0x30)).sw(c, c.g[S2][0]);
            Addr(frame.at(c, 0x20)).sw(c, c.g[A1][0]);
            c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(7));
            if c.g[V1][0] != 0 {
                c.g[V0][0] = c.g[A0][0];
            }
            c.g[A1][0] = c.g[S5][0] << 32;
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
            Addr(frame.at(c, 0x10)).sd(c, c.g[A1][0]);
            c.g[V1][0] = sx(0u32.wrapping_sub(c.g[V0][0] as u32));
            c.g[V1][0] &= 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            tex.off(ti::VRAM_BYTES).sw(c, c.g[V0][0]);
            c.cycles += 1;
            c.g[T3][0] = Addr(frame.at(c, 0xe0)).lw(c); // clut
        }

        // The CLUT: entry size from clutFmt (s2: 1 -> 4, 2 -> 2, else 0).
        let tex = Addr(c.g[S0][0] as u32);
        c.cycles += 2;
        c.g[V0][0] = 1;
        let mut has_clut = false;
        if c.g[T3][0] != 0 {
            c.cycles += 2;
            let is1 = c.g[S4][0] == 1;
            c.g[V0][0] = ((c.g[S4][0] as i64) < 2) as u64;
            if is1 {
                c.cycles += 1;
                c.g[S2][0] = 4;
            } else {
                c.cycles += 2;
                c.g[S2][0] = 0;
                if c.g[V0][0] == 0 {
                    c.cycles += 3;
                    c.g[V0][0] = 2;
                    if c.g[S4][0] == 2 {
                        c.g[S2][0] = 2;
                    } else {
                        c.cycles += 2;
                    }
                }
            }
            c.cycles += 2;
            c.g[V1][0] = 1;
            has_clut = c.g[S2][0] != 0;
        }
        if has_clut {
            // 16 entries for the 4-bit formats (11, 12), else 256.
            c.cycles += 8;
            c.g[V0][0] = tex.off(ti::FORMAT).lw(c);
            c.g[S1][0] = 0x100;
            c.g[A1][0] = 0x10;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xb));
            c.g[A0][0] = tex.off(ti::OWNED).lw(c);
            c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
            if c.g[V1][0] == 0 {
                c.g[S1][0] = c.g[A1][0];
            }
            if c.g[A0][0] != 0 {
                c.cycles += 15;
                c.g[V0][0] = Addr(c.g[S6][0] as u32).off(tm::ALLOC_FLAGS).lw(c);
                c.g[A2][0] = 0x0500_0000;
                c.g[A1][0] = c.g[FP][0];
                c.g[A0][0] = 0x400;
                c.g[A2][0] |= c.g[V0][0];
                c.g[A3][0] = 0;
                call_ptr(c, TEX_LOAD_NEW, 0x0036_7848)?;
                c.mult(0, 0, S1, S2);
                c.g[A1][0] = Addr(frame.at(c, 0xe0)).lw(c);
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.lo[0];
                Addr(c.g[S0][0] as u32).off(ti::CLUT).sw(c, c.g[V0][0]);
                call_ptr(c, TEX_LOAD_MEMCOPY, 0x0036_7860)?;
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(frame.at(c, 0xe0)).lw(c);
                tex.off(ti::CLUT).sw(c, c.g[V0][0]);
                c.cycles += 1;
            }
        } else {
            c.cycles += 1;
            tex.off(ti::CLUT).sw(c, 0);
            c.cycles += 1;
        }
        let tex = Addr(c.g[S0][0] as u32);
        c.g[T0][0] = 0;

        // TEX1 fields: LCM (t0), MXL, MMAG, MMIN (a3), L (t2), K (t1).
        c.cycles += 4;
        c.g[T2][0] = 0;
        c.g[T1][0] = sx(-0xb9i32 as u32);
        c.g[A3][0] = 5;
        if c.g[S5][0] == 0 {
            c.cycles += 4;
            c.g[T0][0] = 1;
            c.g[T2][0] = c.g[S5][0];
            c.g[T1][0] = 0;
            c.g[A3][0] = 1;
        }

        // log2(w - 1) and log2(h - 1) (plzcw).
        c.cycles += 19;
        c.g[V1][0] = Addr(frame.at(c, 0x20)).lw(c);
        c.g[V0][0] = (c.g[V1][0] << 32) >> 32;
        c.plzcw(V0, V0);
        c.g[V0][0] ^= 0x1f;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        c.g[A1][0] = Addr(frame.at(c, 0x30)).lw(c);
        c.g[A0][0] = ((c.g[V0][0] << 32) as i64 >> 32) as u64;
        c.g[V1][0] = (c.g[A1][0] << 32) >> 32;
        c.plzcw(V1, V1);
        c.g[V1][0] ^= 0x1f;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        c.g[V1][0] = ((c.g[V1][0] << 32) as i64 >> 32) as u64;
        c.g[V0][0] = 1;
        let is1 = c.g[S4][0] == 1;
        c.g[V0][0] = ((c.g[S4][0] as i64) < 2) as u64;
        // CLUT PSM (a2): PSMCT16 (2) for clutFmt 2, else PSMCT32.
        if is1 {
            c.cycles += 1;
            c.g[A2][0] = 0;
        } else {
            c.cycles += 2;
            c.g[A2][0] = 0;
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.g[V0][0] = 2;
                c.g[A2][0] = 2;
                if c.g[S4][0] != 2 {
                    c.cycles += 1;
                    c.g[A2][0] = 0;
                }
            }
        }

        // TEX0 = TBW | PSM | TW | TH | TCC | CPSM | (bpp <= 8: CLD 4).
        c.cycles += 21;
        c.g[T3][0] = Addr(frame.at(c, 0x4)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
        c.g[A0][0] <<= 26;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        c.g[V0][0] = sx((c.g[T3][0] as u32) >> 6);
        c.g[V1][0] <<= 30;
        c.g[T3][0] = Addr(frame.at(c, 0x18)).ld(c);
        c.g[V0][0] <<= 32;
        c.g[V0][0] >>= 18;
        c.g[A1][0] = c.g[T3][0] << 20;
        c.g[V0][0] |= c.g[A1][0];
        c.g[V0][0] |= c.g[A0][0];
        c.g[V0][0] |= c.g[V1][0];
        c.g[A0][0] = 0x8000 << 19;
        c.g[V0][0] |= c.g[A0][0];
        c.g[V1][0] = c.g[A2][0] << 51;
        c.g[A2][0] = c.g[V0][0] | c.g[V1][0];
        c.g[V0][0] = Addr(frame.at(c, 0xc)).lw(c);
        c.g[A1][0] = Addr(frame.at(c, 0x10)).ld(c);
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = 0x8000 << 46;
            c.g[A2][0] |= c.g[V0][0];
        }

        c.cycles += 15;
        c.g[A0][0] = c.g[A3][0] << 6;
        c.g[V0][0] = c.g[T2][0];
        c.g[T3][0] = Addr(frame.at(c, 0x8)).lw(c);
        c.g[V1][0] = c.g[A1][0] >> 30;
        c.g[V0][0] <<= 19;
        c.g[A1][0] = 0x20;
        c.g[V1][0] |= c.g[T0][0];
        c.g[A0][0] |= c.g[A1][0];
        Addr(c.g[T3][0] as u32).sd(c, c.g[A2][0]);
        c.g[V1][0] |= c.g[A0][0];
        c.g[V1][0] |= c.g[V0][0];
        c.g[A0][0] = c.g[T1][0] << 32;
        c.g[V1][0] |= c.g[A0][0];
        tex.off(ti::TEX1).sd(c, c.g[V1][0]);
        if c.g[S5][0] != 0 {
            // MIPTBP1/2: the buffer widths of levels 1..6.
            c.cycles += 26;
            let w = c.g[S3][0] as u32;
            let width = |add: u32, shift: u32| sx(w.wrapping_add(add) >> shift);
            c.g[V0][0] = width(0x7f, 7) << 32 >> 18;
            c.g[V1][0] = width(0x3ff, 10) << 32 >> 18;
            c.g[A2][0] = width(0xff, 8) << 34;
            c.g[A3][0] = width(0x7ff, 11) << 34;
            c.g[A0][0] = width(0x1ff, 9) << 54;
            c.g[A1][0] = width(0xfff, 12) << 54;
            c.g[V0][0] |= c.g[A2][0];
            c.g[V1][0] |= c.g[A3][0];
            c.g[V0][0] |= c.g[A0][0];
            c.g[V1][0] |= c.g[A1][0];
            tex.off(ti::MIPTBP1).sd(c, c.g[V0][0]);
            tex.off(ti::MIPTBP2).sd(c, c.g[V1][0]);
        }

        // VRAM now for modes 1 and 3.
        c.cycles += 4;
        c.g[V1][0] = Addr(frame.at(c, 0xe8)).lw(c);
        c.g[V0][0] = 1;
        c.g[A1][0] = Addr(frame.at(c, 0xe8)).lw(c);
        let mode = c.g[V1][0];
        let mut reserve = mode == 1;
        if !reserve {
            c.cycles += 3;
            c.g[V0][0] = 3;
            reserve = c.g[A1][0] == 3;
        }
        if reserve {
            let three = mode != 1;
            c.cycles += if three { 21 } else { 18 };
            c.g[A1][0] = tex.off(ti::VRAM_BYTES).lw(c);
            c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(tm::VRAM_HEAP));
            c.g[A2][0] = Addr(sp(c)).lw(c);
            c.g[A3][0] = u64::MAX;
            if three {
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2000));
            }
            c.g[T0][0] = u64::MAX;
            c.g[T1][0] = 1;
            TEX_LOAD_VRAM_ALLOC.call(c, if three { 0x0036_7a88 } else { 0x0036_7a30 })?;
            c.g[A0][0] = 0x1c;
            c.g[A1][0] = Addr(c.g[S6][0] as u32).off(tm::VRAM_NODES).lw(c);
            c.mult(0, A0, V0, A0);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            let tex = Addr(c.g[S0][0] as u32);
            if three {
                c.g[V1][0] |= 0x22;
                c.g[A2][0] = sx(((c.g[V1][0] as u32) as i32 >> 8) as u32);
                Addr(c.g[A1][0] as u32).sw(c, c.g[V1][0]);
                c.g[A0][0] = sx(0u32.wrapping_sub(c.g[A2][0] as u32));
                tex.off(ti::VRAM_NODE).sw(c, c.g[V0][0]);
                c.g[A0][0] &= 0x1f;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                tex.off(ti::VRAM_BLOCK).sw(c, c.g[A2][0]);
            } else {
                c.g[V1][0] |= 2;
                Addr(c.g[A1][0] as u32).sw(c, c.g[V1][0]);
                c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 8) as u32);
                tex.off(ti::VRAM_NODE).sw(c, c.g[V0][0]);
                tex.off(ti::VRAM_BLOCK).sw(c, c.g[V1][0]);
            }
            c.cycles += 1;
        }
        let tex = Addr(c.g[S0][0] as u32);
        c.g[V0][0] = tex.lw(c);

        // PSMT8 (format 9): swizzle an owned, over-budget texture loaded in
        // mode 0 into a temporary copy; otherwise retag TEX0's PSM as T8H.
        c.cycles += 2;
        let owned = c.g[V0][0] != 0;
        c.g[V0][0] = 9;
        c.cycles += 3;
        c.g[V1][0] = tex.off(ti::FORMAT).lw(c);
        let t8 = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = Addr(sp(c)).lw(c);
        if t8 {
            let mut retag = true;
            if owned {
                c.cycles += 5;
                c.g[V0][0] = Addr(c.g[S6][0] as u32).off(tm::T8_LIMIT).lw(c);
                c.g[V1][0] = tex.off(ti::VRAM_BYTES).lw(c);
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                c.g[T3][0] = Addr(frame.at(c, 0xe8)).lw(c);
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    if c.g[T3][0] == 0 {
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S6][0];
                        c.g[A1][0] = c.g[S0][0];
                        TEX_LOAD_COPY_TO_TEMP.call(c, 0x0036_7afc)?;
                        c.g[V0][0] = Addr(sp(c)).lw(c);
                        retag = false;
                    } else {
                        c.g[V0][0] = tex.off(ti::TEX0).ld(c);
                    }
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = tex.off(ti::TEX0).ld(c);
                }
            } else {
                c.cycles += 1;
                c.g[V0][0] = tex.off(ti::TEX0).ld(c);
            }
            if retag {
                c.cycles += 7;
                c.g[V1][0] = 0xffff_ffff_fc0f_ffff;
                c.g[A0][0] = 0x01b0_0000;
                c.g[V0][0] &= c.g[V1][0];
                c.g[V0][0] |= c.g[A0][0];
                tex.off(ti::TEX0).sd(c, c.g[V0][0]);
                c.g[V0][0] = Addr(sp(c)).lw(c);
            }
        }
        tex_load_texture_exit(c, frame);
        Ok(())
    });
}

/// The epilogue (12 cycles): restore s0..s7, fp and ra, pop the frame.
fn tex_load_texture_exit(c: &mut Cpu, frame: Frame) {
    c.cycles += 12;
    frame.restore(c, S0, 0xd0);
    frame.restore(c, S1, 0xc0);
    frame.restore(c, S2, 0xb0);
    frame.restore(c, S3, 0xa0);
    frame.restore(c, S4, 0x90);
    frame.restore(c, S5, 0x80);
    frame.restore(c, S6, 0x70);
    frame.restore(c, S7, 0x60);
    frame.restore(c, FP, 0x50);
    frame.restore_ra(c, 0x40);
    frame.close(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check_lohi, Rng, DATA, SP as STACK_TOP};

    /// Copy the loader's four jump tables (`0x492090..0x4922b4`) from the
    /// game image: the generated body jumps through them.
    fn tex_load_texture_tables(c: &mut Cpu) {
        static TABLES: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
        let bytes = TABLES.get_or_init(|| {
            let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../orig/SLUS_207.72");
            let d = std::fs::read(path).expect("orig/SLUS_207.72");
            let u32_at = |o: usize| u32::from_le_bytes(d[o..o + 4].try_into().unwrap());
            let ph = u32_at(0x1c) as usize;
            let (off, vaddr) = (u32_at(ph + 4) as usize, u32_at(ph + 8));
            let at = off + (TEX_LOAD_FMT_TABLE - vaddr) as usize;
            d[at..at + (TEX_LOAD_PAGE_TABLE2 + 29 * 4 - TEX_LOAD_FMT_TABLE) as usize].to_vec()
        });
        let at = TEX_LOAD_FMT_TABLE as usize;
        c.bus.ram[at..at + bytes.len()].copy_from_slice(bytes);
    }

    /// `TexMan_LoadTexture`: fresh and fixed slots, every format (and one
    /// past the table), with and without pixels, 0..3 mip levels (and none),
    /// owned and referenced data, every CLUT format with and without a CLUT,
    /// VRAM modes 0..3, and the PSMT8 swizzle budget on both sides. Real
    /// callees: the slot allocators, strncpy, operator new over the game
    /// heap, EA_MemCopy, VramHeap_Alloc over a heap built by VramHeap_Init
    /// and TexMan_CopyToTempTex.
    #[test]
    fn tex_man_load_texture_matches() {
        use crate::game::fe::feasyncfile::tests::{be_library_5, heap_init_5, world_3};
        use crate::game::render::ps2graphicsman::tests::{vram_heap, TEXMAN};
        check_lohi(
            ee::s12::f_00367440__generated,
            tex_man_load_texture,
            240,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x0036_7440, 0);
                tex_load_texture_tables(c);
                be_library_5(c);
                world_3(c);
                heap_init_5(c);
                vram_heap(c);
                let man = TEXMAN;
                let pixels = DATA + 0x20_0000;
                let clut = DATA + 0x21_0000;
                let name = DATA + 0x22_0000;
                for (i, b) in b"snowtex\0".iter().enumerate() {
                    c.bus.write8(name + i as u32, *b);
                }
                for i in 0..0x40 {
                    c.bus.write32(pixels + 4 * i, r.next_u32());
                    c.bus.write32(clut + 4 * i, r.next_u32());
                }
                // A free slot for the non-bound allocator, slot counters.
                let k = 1 + seed % 16;
                let node = man + tex_load_man::SLOTS + 4 * k;
                c.bus.write32(man + 0x1f48, node);
                c.bus.write32(node, 0);
                c.bus.write32(man, seed % 5);
                c.bus.write32(man + 4, seed % 7);
                c.bus.write32(man + tex_load_man::ALLOC_FLAGS, 0);
                let limit = match seed % 3 {
                    0 => 0,
                    1 => 0x7fff_ffff,
                    _ => r.next_u32() % 0x8000,
                };
                c.bus.write32(man + tex_load_man::T8_LIMIT, limit);

                let fmt: u64 = match (seed / 2) % 10 {
                    0 => 0,
                    1 => 2,
                    2 => 8,
                    3 | 4 => 9,
                    5 => 10,
                    6 => 11,
                    7 => 12,
                    8 => 13,
                    _ => r.next_u32() as u64,
                };
                let has_pixels = seed % 5 != 4;
                let mips: i32 = match (seed / 3) % 5 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => -1,
                    _ => (r.next_u32() % 3) as i32,
                };
                let sizes: &[u32] = if fmt == 9 {
                    &[16, 32, 64]
                } else {
                    &[8, 16, 32, 64]
                };
                let w = sizes[(r.next_u32() as usize) % sizes.len()];
                let h = sizes[(r.next_u32() as usize) % sizes.len()];
                let clut_fmt: u64 = match (seed / 7) % 5 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    _ => u64::MAX,
                };
                let mode = if mips < 0 {
                    2 * (seed % 2)
                } else {
                    (seed / 11) % 4
                };
                let owned = (seed / 13) % 2;
                let id: u32 = if seed % 4 == 0 {
                    0x20 + seed % 8
                } else {
                    u32::MAX
                };
                let has_clut = (seed / 17) % 3 != 0;

                c.g[A0][0] = sx(man);
                c.g[A1][0] = if has_pixels { sx(pixels) } else { 0 };
                c.g[A2][0] = sx(name);
                c.g[A3][0] = sx(w);
                c.g[T0][0] = sx(h);
                c.g[T1][0] = sx(mips as u32);
                c.g[T2][0] = fmt;
                c.g[T3][0] = clut_fmt;
                c.bus
                    .write64(STACK_TOP, if has_clut { clut as u64 } else { 0 });
                c.bus.write64(STACK_TOP + 0x8, mode as u64);
                c.bus.write64(STACK_TOP + 0x10, owned as u64);
                c.bus.write64(STACK_TOP + 0x18, sx(id));
                c.g[SP][0] = sx(STACK_TOP);
                c.g[RA][0] = sx(crate::difftest::RA);
                c.pc = 0x0036_7440;
            },
        );
    }
}
