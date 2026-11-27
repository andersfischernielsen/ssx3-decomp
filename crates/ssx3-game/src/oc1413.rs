//! Staging module oc1413 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1413`.

use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Callees of this module's ports, with main's names.
pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    /// `EA_MemAllocNamed` (`0x00317d70`).
    pub(crate) const EA_MEM_ALLOC_NAMED: Callee = Callee {
        addr: 0x0031_7d70,
        f: ee::s10::EA_MemAllocNamed,
    };

    /// `operator_new_named` (`0x00317e30`).
    pub(crate) const OPERATOR_NEW_NAMED: Callee = Callee {
        addr: 0x0031_7e30,
        f: ee::s10::operator_new_named,
    };

    /// `Serialize_ReadU32_2` (`0x002d2558`).
    pub(crate) const SERIALIZE_READ_U32_2: Callee = Callee {
        addr: 0x002d_2558,
        f: ee::s09::Serialize_ReadU32_2,
    };

    /// `Serialize_ReadU8` (`0x002d2790`).
    pub(crate) const SERIALIZE_READ_U8: Callee = Callee {
        addr: 0x002d_2790,
        f: ee::s09::Serialize_ReadU8,
    };

    /// `MD5_Update` (`0x0031d3a8`).
    pub(crate) const MD5_UPDATE: Callee = Callee {
        addr: 0x0031_d3a8,
        f: ee::s10::MD5_Update,
    };

    /// `MD5_Transform` (`0x0031c350`).
    pub(crate) const MD5_TRANSFORM: Callee = Callee {
        addr: 0x0031_c350,
        f: ee::s10::MD5_Transform,
    };

    /// `DLHeap_MoreCore` (`0x0031ffd8`).
    pub(crate) const DLHEAP_MORE_CORE: Callee = Callee {
        addr: 0x0031_ffd8,
        f: ee::s10::DLHeap_MoreCore,
    };

    /// `VidEnginePlayer_Close` (`0x002535f8`).
    pub(crate) const VIDENGINEPLAYER_CLOSE: Callee = Callee {
        addr: 0x0025_35f8,
        f: ee::s06::VidEnginePlayer_Close,
    };

    /// `VidEnginePlayer_StartPlayback` (`0x002536c8`).
    pub(crate) const VIDENGINEPLAYER_START_PLAYBACK: Callee = Callee {
        addr: 0x0025_36c8,
        f: ee::s06::VidEnginePlayer_StartPlayback,
    };

    /// `VD_TextureSink_DrawFrame` (`0x002548d0`).
    pub(crate) const VD_TEXTURE_SINK_DRAW_FRAME: Callee = Callee {
        addr: 0x0025_48d0,
        f: ee::s06::VD_TextureSink_DrawFrame,
    };

    /// `PS2_SONY_CODEC_INTERNAL_FreeFrame` (`0x003b08e0`).
    pub(crate) const SONY_CODEC_FREE_FRAME: Callee = Callee {
        addr: 0x003b_08e0,
        f: ee::s13::f_003b08e0,
    };
}

/// `void PS2_SONY_CODEC_INTERNAL_dtor(...)` (0x3b09a0): free the codec's
/// buffer through the video free hook, unlink and free both frame lists,
/// then the context itself.
pub fn ps2_sony_codec_internal_dtor(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x003b_09a0 => {
                    c.cycles += 14;
                    let frame = Frame::open(c, 0x60);
                    frame.save(c, 16, 0x50);
                    c.g[V0][0] = 0x450000;
                    frame.save(c, 17, 0x40);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6850));
                    frame.save(c, 18, 0x30);
                    frame.save(c, 19, 0x20);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save(c, 20, 0x10);
                    frame.save_ra(c, 0);
                    c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2c));
                    let t = c.g[A0][0] == 0u64;
                    c.g[S1][0] = c.g[A1][0];
                    if t {
                        pc = 0x003b_0a08;
                        continue;
                    }
                    c.cycles += 5;
                    c.g[V1][0] = 0x510000;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x6bcc));
                    let t = c.g[V0][0] as u32;
                    call_ptr(c, t, 0x003b_09e8)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), 0u64);
                    pc = 0x003b_0a08;
                    continue;
                }
                0x003b_0a08 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                    let t = c.g[A0][0] == 0u64;
                    c.g[V1][0] = 0x510000;
                    if t {
                        pc = 0x003b_0a68;
                        continue;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x6bcc));
                    let t = c.g[V0][0] as u32;
                    call_ptr(c, t, 0x003b_0a04)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x78), 0u64);
                    pc = 0x003b_0a68;
                    continue;
                }
                0x003b_0a68 => {
                    c.cycles += 6;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[S3][0] = c.g[V0][0];
                    c.g[S4][0] = c.g[S1][0] & 0x1;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x20));
                    if t {
                        pc = 0x003b_0ab8;
                        continue;
                    }
                    c.cycles += 4;
                    c.g[S1][0] = 0xb;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                    pc = 0x003b_0a30;
                    continue;
                }
                0x003b_0a30 => {
                    c.cycles += 8;
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x8));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = 0u64;
                    }
                    c.sw(c.g[A0][0] as u32, c.g[A1][0]);
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                    c.sw(c.g[V0][0] as u32, c.g[S1][0]);
                    let t = c.g[V1][0] == 0u64;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[S1][0]);
                    if !t {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[V1][0];
                        c.g[A1][0] = 0x3;
                        SONY_CODEC_FREE_FRAME.call(c, 0x003b_0a5c)?;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    if c.g[V0][0] != c.g[S3][0] {
                        c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                        if event_due(c) {
                            deliver_event_here(c, 0x003b_0a30);
                        }
                        pc = 0x003b_0a30;
                        continue;
                    }
                    pc = 0x003b_0ab8;
                    continue;
                }
                0x003b_0ab8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                    let t = c.g[V0][0] == c.g[S2][0];
                    c.g[S1][0] = 0xb;
                    if t {
                        pc = 0x003b_0aec;
                        continue;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                    c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                    pc = 0x003b_0a80;
                    continue;
                }
                0x003b_0a80 => {
                    c.cycles += 8;
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x8));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = 0u64;
                    }
                    c.sw(c.g[A0][0] as u32, c.g[A1][0]);
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                    c.sw(c.g[V0][0] as u32, c.g[S1][0]);
                    let t = c.g[V1][0] == 0u64;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[S1][0]);
                    if !t {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[V1][0];
                        c.g[A1][0] = 0x3;
                        SONY_CODEC_FREE_FRAME.call(c, 0x003b_0aac)?;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                    if c.g[V0][0] != c.g[S2][0] {
                        c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                        if event_due(c) {
                            deliver_event_here(c, 0x003b_0a80);
                        }
                        pc = 0x003b_0a80;
                        continue;
                    }
                    pc = 0x003b_0aec;
                    continue;
                }
                0x003b_0aec => {
                    c.cycles += 9;
                    c.g[V0][0] = 0x7;
                    c.g[V1][0] = 0x450000;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6810));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    let t = c.g[S4][0] == 0u64;
                    c.sw(c.g[S0][0] as u32, c.g[V1][0]);
                    if !t {
                        c.cycles += 4;
                        c.g[V1][0] = 0x510000;
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x6bcc));
                        let t = c.g[V0][0] as u32;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, t, 0x003b_0aec)?;
                    }
                    c.cycles += 8;
                    let frame = Frame { size: 0x60 };
                    frame.restore(c, 16, 0x50);
                    frame.restore(c, 17, 0x40);
                    frame.restore(c, 18, 0x30);
                    frame.restore(c, 19, 0x20);
                    frame.restore(c, 20, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003b_09a0, 0x003b_0af4);
                    return Ok(());
                }
            }
        }
    })
}

/// `int VD_TextureSink_UploadFrame(...)` (0x254c48): upload the frame rows
/// through the sink's method table, then redraw through
/// `VD_TextureSink_DrawFrame`.
pub fn vd_texture_sink_upload_frame(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0025_4c48 => {
                    c.cycles += 13;
                    let frame = Frame::open(c, 0xb0);
                    frame.save(c, 16, 0xa0);
                    frame.save(c, 17, 0x90);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save(c, 18, 0x80);
                    frame.save(c, 19, 0x70);
                    frame.save(c, 20, 0x60);
                    frame.save(c, 21, 0x50);
                    frame.save(c, 22, 0x40);
                    frame.save(c, 23, 0x30);
                    frame.save(c, 30, 0x20);
                    let t = c.g[A1][0] == 0u64;
                    frame.save_ra(c, 0x10);
                    if !t {
                        c.cycles += 1;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[A1][0]);
                    }
                    pc = 0x0025_4ca0;
                    continue;
                }
                0x0025_4ca0 => {
                    c.cycles += 6;
                    c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = c.g[V0][0] & 0x1000;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S7][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
                    if !t {
                        c.cycles += 2;
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
                        c.g[S7][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    }
                    pc = 0x0025_4d68;
                    continue;
                }
                0x0025_4d68 => {
                    c.cycles += 6;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[S5][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                    c.g[A0][0] = 0u64;
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[FP][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x6));
                    if t {
                        pc = 0x0025_4d70;
                        continue;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                    pc = 0x0025_4cc0;
                    continue;
                }
                0x0025_4cc0 => {
                    c.cycles += 4;
                    c.g[S3][0] = 0u64;
                    c.g[S2][0] = 0u64;
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[S6][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x0025_4d54;
                        continue;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[A0][0] as u32) << 6);
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24));
                    c.mult(0, 20, 30, 2);
                    pc = 0x0025_4ce0;
                    continue;
                }
                0x0025_4ce0 => {
                    c.cycles += 29;
                    c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
                    c.g[V0][0] = sx((c.g[S5][0] as u32) << 2);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                    c.g[A2][0] = sx((c.g[S3][0] as u32) << 6);
                    c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10d8));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[S4][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A2][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[T1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x188));
                    c.g[T2][0] = 0u64;
                    c.sw(c.g[SP][0] as u32, 0u64);
                    c.g[T3][0] = 0u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18c));
                    c.g[T0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[V0][0] as u32;
                    c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    call_ptr(c, t, 0x0025_4d3c)?;
                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0025_4ce0);
                        }
                        pc = 0x0025_4ce0;
                        continue;
                    }
                    pc = 0x0025_4d54;
                    continue;
                }
                0x0025_4d54 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                    c.g[A0][0] = c.g[S6][0];
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                        if event_due(c) {
                            deliver_event_here(c, 0x0025_4cc0);
                        }
                        pc = 0x0025_4cc0;
                        continue;
                    }
                    pc = 0x0025_4d70;
                    continue;
                }
                0x0025_4d70 => {
                    c.cycles += 14;
                    c.g[A0][0] = c.g[S0][0];
                    VD_TEXTURE_SINK_DRAW_FRAME.call(c, 0x0025_4d70)?;
                    let frame = Frame { size: 0xb0 };
                    frame.restore(c, 16, 0xa0);
                    frame.restore(c, 17, 0x90);
                    frame.restore(c, 18, 0x80);
                    frame.restore(c, 19, 0x70);
                    frame.restore(c, 20, 0x60);
                    frame.restore(c, 21, 0x50);
                    frame.restore(c, 22, 0x40);
                    frame.restore(c, 23, 0x30);
                    frame.restore(c, 30, 0x20);
                    frame.restore_ra(c, 0x10);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0025_4c48, 0x0025_4da0);
                    return Ok(());
                }
            }
        }
    })
}

/// `int VidEnginePlayer_Open(VidEnginePlayer *player, void *params)`
/// (0x2534a8): close the player, copy 32 param bytes, and open again.
pub fn vid_engine_player_open(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 17, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 16, 0x40);
        frame.save(c, 18, 0x20);
        frame.save(c, 19, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = 0x1;
        VIDENGINEPLAYER_CLOSE.call(c, 0x0025_34d0)?;
        c.g[S2][0] = c.g[S3][0];
        c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x7));
        c.ldr(2, c.g[S0][0] as u32);
        c.ldl(3, (c.g[S0][0] as u32).wrapping_add(0xf));
        c.ldr(3, (c.g[S0][0] as u32).wrapping_add(0x8));
        c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x17));
        c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x10));
        c.ldl(5, (c.g[S0][0] as u32).wrapping_add(0x1f));
        c.ldr(5, (c.g[S0][0] as u32).wrapping_add(0x18));
        c.sdl(2, (c.g[S1][0] as u32).wrapping_add(0x1b));
        c.sdr(2, (c.g[S1][0] as u32).wrapping_add(0x14));
        c.sdl(3, (c.g[S1][0] as u32).wrapping_add(0x23));
        c.sdr(3, (c.g[S1][0] as u32).wrapping_add(0x1c));
        c.sdl(4, (c.g[S1][0] as u32).wrapping_add(0x2b));
        c.sdr(4, (c.g[S1][0] as u32).wrapping_add(0x24));
        c.sdl(5, (c.g[S1][0] as u32).wrapping_add(0x33));
        c.sdr(5, (c.g[S1][0] as u32).wrapping_add(0x2c));
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x34), c.g[V0][0]);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
        if c.g[V0][0] == 0 {
            c.g[S2][0] = 0u64;
        }
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        if c.g[V1][0] != 0 {
            c.g[S3][0] = 0u64;
        }
        let t = c.g[A0][0] == 0u64;
        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x10));
        let opened = if t {
            false
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lb(c.g[A0][0] as u32);
            let t = c.g[V0][0] == 0u64;
            c.g[V1][0] = 0x510000;
            if t {
                false
            } else {
                c.cycles += 19;
                c.g[A0][0] = 0x4a0000;
                c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6bd0));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x6bd0));
                c.g[T0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2e98));
                c.g[A1][0] = 0x78;
                c.g[A2][0] = 0u64;
                let t = c.g[V0][0] as u32;
                c.g[A3][0] = 0u64;
                call_ptr(c, t, 0x0025_356c)?;
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = c.g[V0][0];
                c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                c.g[T1][0] = c.g[S2][0];
                c.g[T2][0] = c.g[S3][0];
                c.g[A2][0] = 0x60000;
                c.g[T0][0] = 0x20000;
                call_ptr(c, 0x003a_de08, 0x0025_358c)?;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x38), c.g[V0][0]);
                true
            }
        };
        if !opened {
            c.cycles += 17;
            c.g[V1][0] = 0x510000;
            c.g[A0][0] = 0x4a0000;
            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6bd0));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x6bd0));
            c.g[T0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2e98));
            c.g[A1][0] = 0x78;
            c.g[A2][0] = 0u64;
            let t = c.g[V0][0] as u32;
            c.g[A3][0] = 0u64;
            call_ptr(c, t, 0x0025_35bc)?;
            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
            c.g[A0][0] = c.g[V0][0];
            c.g[A3][0] = c.g[S2][0];
            c.g[T0][0] = c.g[S3][0];
            c.g[A2][0] = 0x80000;
            call_ptr(c, 0x003a_ddc0, 0x0025_35d4)?;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x38), c.g[V0][0]);
        }
        c.cycles += 8;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[V0][0] = 0x1;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void *DLHeap_SysTrim(...)` (0x31e6d8): release the heap's top chunk.
pub fn dlheap_sys_trim(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x60);
        c.g[A3][0] = c.g[A1][0] << 32;
        c.g[A3][0] = c.g[A3][0] >> 32;
        frame.save(c, 17, 0x40);
        frame.save(c, 18, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 19, 0x20);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, 16, 0x50);
        c.g[A2][0] = 0xffff_ffff_ffff_0000;
        frame.save(c, 20, 0x10);
        c.g[A2][0] = c.g[A2][0] | 0xfffc;
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
        c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x35c));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
        c.g[V1][0] = c.g[V1][0] & c.g[A2][0];
        c.g[V0][0] = c.g[V0][0] << 32;
        c.g[V1][0] = c.g[V1][0] << 32;
        c.g[S0][0] = c.g[S0][0] << 32;
        c.g[S3][0] = c.g[V1][0] >> 32;
        c.g[V0][0] = c.g[V0][0] >> 32;
        c.g[S0][0] = c.g[S0][0] >> 32;
        c.g[A3][0] = c.g[S3][0].wrapping_sub(c.g[A3][0]);
        c.g[V0][0] = c.g[V0][0].wrapping_sub(0x10);
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[A3][0].wrapping_add(c.g[V0][0]);
        call_ptr(c, 0x0040_fcb0, 0x0031_e74c)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0].wrapping_sub(0x1);
        call_ptr(c, 0x0041_14d0, 0x0031_e758)?;
        c.g[S0][0] = c.g[V0][0];
        let t = (c.g[S0][0] as i64) <= 0;
        c.g[A0][0] = c.g[S1][0];
        if t {
            c.cycles += 1;
            c.g[V0][0] = 0u64;
        } else {
            c.cycles += 9;
            c.g[A1][0] = 0u64;
            DLHEAP_MORE_CORE.call(c, 0x0031_e76c)?;
            c.g[S4][0] = c.g[V0][0];
            c.g[V1][0] = c.g[S3][0] << 32;
            c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            let t = c.g[S4][0] != c.g[V0][0];
            c.g[V0][0] = 0u64;
            if !t {
                c.cycles += 11;
                c.g[A1][0] = 0u64.wrapping_sub(c.g[S0][0]);
                c.g[A0][0] = c.g[S1][0];
                DLHEAP_MORE_CORE.call(c, 0x0031_e794)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = 0u64;
                DLHEAP_MORE_CORE.call(c, 0x0031_e7a0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[V0][0] = 0xffff_ffff_ffff_0000;
                c.g[V0][0] = c.g[V0][0] | 0xffff;
                let t = c.g[A0][0] == c.g[V0][0];
                c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                if !t {
                    c.cycles += 2;
                    let t = c.g[A0][0] == 0u64;
                    c.g[V1][0] = c.g[S3][0].wrapping_sub(c.g[A0][0]);
                    if !t {
                        c.cycles += 13;
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x368));
                        c.g[A0][0] = c.g[A0][0] << 32;
                        c.g[A0][0] = ((c.g[A0][0] as i64) >> 32) as u64;
                        c.g[A1][0] = 0x1;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                        c.g[V1][0] = c.g[V1][0] | c.g[A1][0];
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
                        c.g[V1][0] = c.g[V1][0] << 32;
                        c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x368), c.g[V0][0]);
                        c.g[V0][0] = 0x1;
                        c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                    } else {
                        // Nothing was released.
                        c.cycles += 1;
                        c.g[V0][0] = 0u64;
                    }
                } else {
                    // The shrink failed: `MoreCore(0)` answered -1.
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                }
            }
        }
        c.cycles += 8;
        frame.restore(c, 16, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MD5_Final(...)` (0x31d4d0): finalize the MD5 context into the
/// digest.
pub fn md5_final(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0031_d4d0 => {
                    c.cycles += 8;
                    let frame = Frame::open(c, 0x40);
                    c.g[A2][0] = 0u64;
                    frame.save(c, 16, 0x30);
                    frame.save(c, 17, 0x20);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save_ra(c, 0x10);
                    c.g[S1][0] = c.g[A1][0];
                    pc = 0x0031_d4f0;
                    continue;
                }
                0x0031_d4f0 => {
                    c.cycles += 12;
                    c.g[V0][0] = sx(((c.g[A2][0] as u32) as i32 >> 2) as u32);
                    c.g[A0][0] = c.g[A2][0] & 0x3;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[A2][0] as i64) < 8i64) as u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) >> ((c.g[A0][0] as u32) & 31));
                    let t = c.g[V0][0] != 0u64;
                    c.sb(c.g[A1][0] as u32, c.g[V1][0]);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0031_d4f0);
                        }
                        pc = 0x0031_d4f0;
                        continue;
                    }
                    c.cycles += 16;
                    c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                    c.g[A2][0] = 0x37;
                    c.g[A1][0] = 0x490000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2508));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[A2][0] = c.g[A2][0] & 0x3f;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    MD5_UPDATE.call(c, 0x0031_d548)?;
                    c.g[A2][0] = 0x8;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[SP][0];
                    MD5_UPDATE.call(c, 0x0031_d558)?;
                    c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A2][0] = 0u64;
                    pc = 0x0031_d560;
                    continue;
                }
                0x0031_d560 => {
                    c.cycles += 12;
                    c.g[V0][0] = sx(((c.g[A2][0] as u32) as i32 >> 2) as u32);
                    c.g[A0][0] = c.g[A2][0] & 0x3;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[A2][0] as i64) < 16i64) as u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) >> ((c.g[A0][0] as u32) & 31));
                    let t = c.g[V0][0] != 0u64;
                    c.sb(c.g[A1][0] as u32, c.g[V1][0]);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0031_d560);
                        }
                        pc = 0x0031_d560;
                        continue;
                    }
                    c.cycles += 5;
                    let frame = Frame { size: 0x40 };
                    frame.restore(c, 16, 0x30);
                    frame.restore(c, 17, 0x20);
                    frame.restore_ra(c, 0x10);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0031_d4d0, 0x0031_d57c);
                    return Ok(());
                }
            }
        }
    })
}

/// `void MD5_Update(...)` (0x31d3a8): add bytes to the MD5 context.
pub fn md5_update(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0031_d3a8 => {
                    c.cycles += 18;
                    let frame = Frame::open(c, 0x70);
                    frame.save(c, 16, 0x60);
                    frame.save(c, 17, 0x50);
                    frame.save(c, 18, 0x40);
                    frame.save(c, 19, 0x30);
                    c.g[S2][0] = c.g[A0][0];
                    frame.save(c, 20, 0x20);
                    frame.save(c, 21, 0x10);
                    c.g[S4][0] = c.g[A1][0];
                    frame.save_ra(c, 0);
                    c.g[S5][0] = c.g[A2][0];
                    c.g[S0][0] = c.g[S5][0];
                    c.g[S1][0] = c.g[S4][0];
                    c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A1][0] = sx((c.g[S0][0] as u32) << 3);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 3);
                    let t = (c.g[S0][0] as i64) <= 0;
                    c.g[S3][0] = c.g[V0][0] & 0x3f;
                    if t {
                        pc = 0x0031_d4a8;
                        continue;
                    }
                    pc = 0x0031_d3f0;
                    continue;
                }
                0x0031_d3f0 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = sx(((c.g[S0][0] as u32) as i32 >> 29) as u32);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A1][0] = (c.g[V1][0] < c.g[A1][0]) as u64;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    let t = c.g[A1][0] == 0u64;
                    c.sw(c.g[S2][0] as u32, c.g[V1][0]);
                    if t {
                        pc = 0x0031_d418;
                        continue;
                    }
                    pc = 0x0031_d410;
                    continue;
                }
                0x0031_d410 => {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    pc = 0x0031_d418;
                    continue;
                }
                0x0031_d418 => {
                    c.cycles += 2;
                    let t = c.g[S3][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    if t {
                        pc = 0x0031_d488;
                        continue;
                    }
                    pc = 0x0031_d420;
                    continue;
                }
                0x0031_d420 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 65i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S1][0] = c.g[S0][0];
                    if t {
                        pc = 0x0031_d434;
                        continue;
                    }
                    pc = 0x0031_d42c;
                    continue;
                }
                0x0031_d42c => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x40;
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S3][0] as u32));
                    pc = 0x0031_d434;
                    continue;
                }
                0x0031_d434 => {
                    c.cycles += 9;
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    c.g[A2][0] = c.g[S1][0];
                    call_ptr(c, 0x0041_605c, 0x0031_d448)?;
                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.g[V1][0] = ((c.g[V1][0] as i64) < 64i64) as u64;
                    let t = c.g[V1][0] != 0u64;
                    if t {
                        pc = 0x0031_d4ac;
                        continue;
                    }
                    pc = 0x0031_d458;
                    continue;
                }
                0x0031_d458 => {
                    c.cycles += 7;
                    c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[S1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x18));
                    MD5_TRANSFORM.call(c, 0x0031_d46c)?;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 64i64) as u64;
                    pc = 0x0031_d48c;
                    continue;
                }
                0x0031_d474 => {
                    c.cycles += 1;
                    pc = 0x0031_d478;
                    continue;
                }
                0x0031_d478 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S2][0];
                    MD5_TRANSFORM.call(c, 0x0031_d480)?;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x40));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x40));
                    pc = 0x0031_d488;
                    continue;
                }
                0x0031_d488 => {
                    c.cycles += 1;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 64i64) as u64;
                    pc = 0x0031_d48c;
                    continue;
                }
                0x0031_d48c => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[S1][0];
                    if t {
                        if back_edge_unwinds(c, 0x0031_d478) {
                            return Ok(());
                        }
                        pc = 0x0031_d478;
                        continue;
                    }
                    pc = 0x0031_d494;
                    continue;
                }
                0x0031_d494 => {
                    c.cycles += 2;
                    let t = c.g[S0][0] == 0u64;
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x18));
                    if t {
                        pc = 0x0031_d4a8;
                        continue;
                    }
                    pc = 0x0031_d49c;
                    continue;
                }
                0x0031_d49c => {
                    c.cycles += 3;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = c.g[S0][0];
                    call_ptr(c, 0x0041_605c, 0x0031_d4a8)?;
                    pc = 0x0031_d4a8;
                    continue;
                }
                0x0031_d4a8 => {
                    c.cycles += 1;
                    pc = 0x0031_d4ac;
                    continue;
                }
                0x0031_d4ac => {
                    c.cycles += 8;
                    let frame = Frame { size: 0x70 };
                    frame.restore(c, 16, 0x60);
                    frame.restore(c, 17, 0x50);
                    frame.restore(c, 18, 0x40);
                    frame.restore(c, 19, 0x30);
                    frame.restore(c, 20, 0x20);
                    frame.restore(c, 21, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0031_d3a8, 0x0031_d4cc);
                    return Ok(());
                }
            }
        }
    })
}

/// `int VidEnginePlayer_PlayBlocking(...)` (0x253748): start playback and
/// spin until the frame is due, then decode it.
///
/// Not registered and untested: the frame wait at the bottom of the loop
/// ends only when the real AV thread delivers a frame, so with fakes every
/// playing fixture spins without bound, and a quiet player (`+0x38` == 0)
/// only takes the early exit, which leaves the loop unchecked.
#[allow(dead_code)]
pub fn vid_engine_player_play_blocking(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0025_3748 => {
                    c.cycles += 8;
                    let frame = Frame::open(c, 0x30);
                    frame.save(c, 16, 0x20);
                    frame.save(c, 17, 0x10);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save_ra(c, 0);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
                    if c.g[V0][0] == 0u64 {
                        let frame = Frame { size: 0x30 };
                        frame.restore(c, 16, 0x20);
                        pc = 0x0025_384c;
                        continue;
                    }
                    c.cycles += 4;
                    c.g[S1][0] = 0x1;
                    VIDENGINEPLAYER_START_PLAYBACK.call(c, 0x0025_3770)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    pc = 0x0025_3778;
                    continue;
                }
                0x0025_3778 => {
                    c.cycles += 2;
                    if c.g[V0][0] == 0u64 {
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3c));
                    } else {
                        c.cycles += 8;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3c));
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x0025_3798)?;
                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
                        pc = 0x0025_381c;
                        continue;
                    }
                    c.cycles += 7;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                    let t = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0025_37b8)?;
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
                    pc = 0x0025_381c;
                    continue;
                }
                0x0025_381c => {
                    c.cycles += 13;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xa0));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa4));
                    let t = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0025_37d0)?;
                    c.g[A0][0] = 0u64;
                    call_ptr(c, 0x003e_5928, 0x0025_37d8)?;
                    c.g[A0][0] = 0u64;
                    call_ptr(c, 0x003e_5398, 0x0025_37e0)?;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
                    call_ptr(c, 0x003a_e780, 0x0025_37e8)?;
                    if c.g[V0][0] == 0u64 {
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[S1][0]);
                    } else {
                        c.cycles += 11;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3c));
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x30));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x0025_3808)?;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
                        c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x48));
                        call_ptr(c, 0x003a_e6c8, 0x0025_3814)?;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), 0u64);
                    }
                    pc = 0x0025_383c;
                    continue;
                }
                0x0025_383c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                    } else {
                        c.cycles += 5;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[S1][0]);
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
                        call_ptr(c, 0x003a_e860, 0x0025_3834)?;
                        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                    }
                    c.cycles += 2;
                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        if back_edge_unwinds(c, 0x0025_3778) {
                            return Ok(());
                        }
                        pc = 0x0025_3778;
                        continue;
                    }
                    c.cycles += 2;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x40), 0u64);
                    let frame = Frame { size: 0x30 };
                    frame.restore(c, 16, 0x20);
                    pc = 0x0025_384c;
                    continue;
                }
                0x0025_384c => {
                    c.cycles += 4;
                    let frame = Frame { size: 0x30 };
                    frame.restore(c, 17, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0025_3748, 0x0025_3850);
                    return Ok(());
                }
            }
        }
    })
}

/// `int Serialize_ReadDataBlock(void *stream, ...)` (0x2d2868): read a
/// counted block of bytes, returning their sum.
pub fn serialize_read_data_block(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x002d_2868 => {
                    c.cycles += 35;
                    c.g[A2][0] = 0x100;
                    let frame = Frame::open(c, 0x60);
                    frame.save(c, 19, 0x20);
                    c.g[A3][0] = 0u64;
                    c.g[S3][0] = c.g[A1][0];
                    frame.save(c, 20, 0x10);
                    c.g[S4][0] = c.g[A0][0];
                    c.g[A1][0] = 0x480000;
                    frame.save(c, 16, 0x50);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6778));
                    frame.save(c, 17, 0x40);
                    c.g[A0][0] = 0x8;
                    frame.save(c, 18, 0x30);
                    frame.save_ra(c, 0);
                    c.g[S1][0] = 0u64;
                    EA_MEM_ALLOC_NAMED.call(c, 0x002d_28a8)?;
                    c.sw(c.g[V0][0] as u32, 0u64);
                    c.g[A1][0] = c.g[V0][0];
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
                    c.g[A0][0] = c.g[S4][0];
                    c.sw(c.g[S3][0] as u32, c.g[V0][0]);
                    SERIALIZE_READ_U32_2.call(c, 0x002d_28c0)?;
                    c.g[S0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[A1][0] = 0x480000;
                    c.g[S2][0] = c.g[V0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6788));
                    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                    c.g[A2][0] = 0x100;
                    c.g[A3][0] = 0u64;
                    OPERATOR_NEW_NAMED.call(c, 0x002d_28e0)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.g[V1][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.g[S2][0];
                    if t {
                        pc = 0x002d_2924;
                        continue;
                    }
                    c.cycles += 1;
                    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    pc = 0x002d_28f8;
                    continue;
                }
                0x002d_28f8 => {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    SERIALIZE_READ_U8.call(c, 0x002d_2904)?;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = c.lw(c.g[S3][0] as u32);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                        if event_due(c) {
                            deliver_event_here(c, 0x002d_28f8);
                        }
                        pc = 0x002d_28f8;
                        continue;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S2][0];
                    pc = 0x002d_2924;
                    continue;
                }
                0x002d_2924 => {
                    c.cycles += 8;
                    let frame = Frame { size: 0x60 };
                    frame.restore(c, 16, 0x50);
                    frame.restore(c, 17, 0x40);
                    frame.restore(c, 18, 0x30);
                    frame.restore(c, 19, 0x20);
                    frame.restore(c, 20, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002d_2868, 0x002d_2930);
                    return Ok(());
                }
            }
        }
    })
}

/// `int Serialize_ReadString(void *stream, ...)` (0x2d27b0): read bytes up
/// to a terminator (at most 1000), returning the count.
pub fn serialize_read_string(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x002d_27b0 => {
                    c.cycles += 14;
                    let frame = Frame::open(c, 0x60);
                    c.g[V0][0] = 0x1;
                    frame.save(c, 16, 0x50);
                    c.g[V1][0] = 0x4d0000;
                    frame.save(c, 17, 0x40);
                    c.g[S0][0] = 0u64;
                    frame.save(c, 18, 0x30);
                    c.g[S1][0] = c.g[A0][0];
                    frame.save(c, 19, 0x20);
                    c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x49b8));
                    frame.save_ra(c, 0x10);
                    c.g[S3][0] = c.g[A1][0];
                    c.sb(c.g[SP][0] as u32, c.g[V0][0]);
                    c.g[A0][0] = c.g[S1][0];
                    pc = 0x002d_27e8;
                    continue;
                }
                0x002d_27e8 => {
                    c.cycles += 9;
                    c.g[A1][0] = c.g[SP][0];
                    SERIALIZE_READ_U8.call(c, 0x002d_27f0)?;
                    c.g[V1][0] = c.lbu(c.g[SP][0] as u32);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    c.sb((c.g[V0][0] as u32).wrapping_sub(0x1), c.g[V1][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 24);
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 1000i64) as u64;
                    if !t {
                        c.cycles += 2;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A0][0] = c.g[S1][0];
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x002d_27e8);
                            }
                            pc = 0x002d_27e8;
                            continue;
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 0x3e8;
                    if c.g[S0][0] == c.g[V0][0] {
                        c.sb((c.g[S2][0] as u32).wrapping_add(0x3e7), 0u64);
                    }
                    c.cycles += 18;
                    c.g[A1][0] = 0x480000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = 0x100;
                    c.g[A3][0] = 0u64;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6768));
                    OPERATOR_NEW_NAMED.call(c, 0x002d_2838)?;
                    c.g[A1][0] = c.g[S2][0];
                    c.sw(c.g[S3][0] as u32, c.g[V0][0]);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0041_66f4, 0x002d_2848)?;
                    c.g[V0][0] = c.g[S0][0];
                    let frame = Frame { size: 0x60 };
                    frame.restore(c, 17, 0x40);
                    frame.restore(c, 16, 0x50);
                    frame.restore(c, 18, 0x30);
                    frame.restore(c, 19, 0x20);
                    frame.restore_ra(c, 0x10);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002d_27b0, 0x002d_2868);
                    return Ok(());
                }
            }
        }
    })
}

/// `int VidEnginePlayer_Render(...)` (0x253890): run the sink's render
/// method for the player, or nothing when there is no frame.
pub fn vid_engine_player_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
        if c.g[V0][0] != 0u64 {
            c.cycles += 3;
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3c));
            let t = c.g[A2][0] == 0u64;
            c.g[A3][0] = c.g[A2][0];
            if t {
                c.cycles += 1;
                frame.restore(c, 16, 0x10);
            } else {
                c.cycles += 2;
                // Selection: the A1!=0 and S0+8 arms break out past the
                // join straight to the render call; the deeper arms fall
                // into the join below.
                enum Sel {
                    Direct(u64),
                    Join(u64),
                }
                let sel = if c.g[A1][0] != 0u64 {
                    Sel::Direct(c.lw((c.g[A3][0] as u32).wrapping_add(0x4)))
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    if c.g[V0][0] != 0u64 {
                        Sel::Direct(c.lw((c.g[A3][0] as u32).wrapping_add(0x4)))
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                        if c.g[V0][0] != 0u64 {
                            Sel::Join(c.lw(c.g[S0][0] as u32))
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            if c.g[V0][0] != 0u64 {
                                Sel::Direct(c.lw((c.g[A3][0] as u32).wrapping_add(0x4)))
                            } else {
                                c.cycles += 1;
                                Sel::Join(c.lw(c.g[S0][0] as u32))
                            }
                        }
                    }
                };
                match sel {
                    Sel::Direct(v0) => {
                        c.g[V0][0] = v0;
                        c.cycles += 6;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x0025_3904)?;
                        frame.restore(c, 16, 0x10);
                    }
                    Sel::Join(v0) => {
                        c.g[V0][0] = v0;
                        c.cycles += 2;
                        if c.g[V0][0] == 0u64 {
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                            c.cycles += 6;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0025_3920)?;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x40), 0u64);
                            c.cycles += 1;
                            frame.restore(c, 16, 0x10);
                        } else {
                            c.cycles += 1;
                            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                            c.cycles += 6;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0025_3904)?;
                            frame.restore(c, 16, 0x10);
                        }
                    }
                }
            }
        } else {
            frame.restore(c, 16, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void PS2ExecMan_v03(PS2ExecMan *man)` (0x31a920): install the default
/// exception/signal handlers and semaphores on `man`.
pub fn ps2_exec_man_v03(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 45;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 17, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 16, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x3;
        call_ptr(c, 0x0042_4880, 0x0031_a93c)?;
        c.g[S0][0] = 0x10000;
        c.g[A0][0] = 0x3;
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4290));
        call_ptr(c, 0x0042_3aa0, 0x0031_a950)?;
        c.g[A0][0] = 0x3;
        call_ptr(c, 0x0042_48e8, 0x0031_a958)?;
        c.g[V1][0] = 0x1;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x20), c.g[V1][0]);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4044));
        call_ptr(c, 0x0042_3dc0, 0x0031_a968)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4034));
        call_ptr(c, 0x0042_3dc0, 0x0031_a970)?;
        call_ptr(c, 0x0042_3cc0, 0x0031_a978)?;
        call_ptr(c, 0x0042_3cc0, 0x0031_a980)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4030));
        call_ptr(c, 0x0042_3bf0, 0x0031_a988)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4030));
        call_ptr(c, 0x0042_3bb0, 0x0031_a990)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4040));
        call_ptr(c, 0x0042_3bf0, 0x0031_a998)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4040));
        call_ptr(c, 0x0042_3bb0, 0x0031_a9a0)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4034));
        call_ptr(c, 0x0042_3db0, 0x0031_a9a8)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4044));
        call_ptr(c, 0x0042_3db0, 0x0031_a9b0)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0042_3db0, 0x0031_a9b8)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        call_ptr(c, 0x0042_3db0, 0x0031_a9c0)?;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x003b_09a0,
        ps2_sony_codec_internal_dtor,
        "PS2_SONY_CODEC_INTERNAL_dtor",
    ),
    (
        0x0025_4c48,
        vd_texture_sink_upload_frame,
        "VD_TextureSink_UploadFrame",
    ),
    (0x0025_34a8, vid_engine_player_open, "VidEnginePlayer_Open"),
    (0x0031_e6d8, dlheap_sys_trim, "DLHeap_SysTrim"),
    (0x0031_d3a8, md5_update, "MD5_Update"),
    (
        0x002d_2868,
        serialize_read_data_block,
        "Serialize_ReadDataBlock",
    ),
    (0x0031_d4d0, md5_final, "MD5_Final"),
    (0x002d_27b0, serialize_read_string, "Serialize_ReadString"),
    (0x0031_a920, ps2_exec_man_v03, "PS2ExecMan_v03"),
    (
        0x0025_3890,
        vid_engine_player_render,
        "VidEnginePlayer_Render",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::events::Events;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes::{machine, machine_rt, FAKE};
    use crate::difftest::{require_paths, Rng, DATA};
    use crate::oc1418::tests::heap_arena;

    /// The `miss` hook with a fixed 4-cycle cost.
    fn norm_miss(c: &mut Cpu, target: u32) {
        let before = c.cycles;
        crate::difftest::fakes::fake_call(c, target);
        c.cycles = before + 4;
    }

    /// The video free hook the dtor calls through (`0x510000 - 0x6bcc`).
    const DTOR_FREE_HOOK: u32 = 0x0050_9434;

    /// Link `n` nodes (0x40 apart from `first`) into the circular list
    /// whose head is `head`; each node's frame word at -4 is null so the
    /// real `FreeFrame` skips its SHP free.
    fn dtor_list(c: &mut Cpu, head: u32, first: u32, n: u32) {
        let nodes: Vec<u32> = (0..n).map(|i| first + 0x40 * i).collect();
        let ring: Vec<u32> = std::iter::once(head).chain(nodes.iter().copied()).collect();
        for (i, &x) in ring.iter().enumerate() {
            let next = if i + 1 < ring.len() {
                ring[i + 1]
            } else {
                head
            };
            let prev = if i == 0 {
                *ring.last().unwrap()
            } else {
                ring[i - 1]
            };
            c.bus.write32(x, next);
            c.bus.write32(x + 4, prev);
        }
        for &x in &nodes {
            c.bus.write32(x - 4, 0);
        }
    }

    /// The dtor fixture: the `+0x2c` buffer and `+0x78` IPU words (null or
    /// live, freed through the hook), the self-free flag in `a1`, and 0-2
    /// live nodes on each of the `+0x18` and `+0x20` frame lists, all
    /// chosen by separate seed digits.
    fn dtor_fixture(c: &mut Cpu, seed: u32) {
        const CTX: u32 = DATA + 0x100;
        for i in 0..0x30u32 {
            c.bus.write32(CTX + 4 * i, 0x5555_0000 + i);
        }
        c.bus
            .write32(CTX + 0x2c, if seed % 2 == 0 { 0 } else { DATA + 0x3000 });
        c.bus.write32(
            CTX + 0x78,
            if (seed / 2) % 2 == 0 {
                0
            } else {
                DATA + 0x3100
            },
        );
        dtor_list(c, CTX + 0x18, DATA + 0x1000, (seed / 8) % 3);
        dtor_list(c, CTX + 0x20, DATA + 0x1800, (seed / 24) % 3);
        c.bus.write32(DTOR_FREE_HOOK, FAKE);
        c.g[A0][0] = CTX as u64;
        c.g[A1][0] = ((seed / 4) % 2) as u64;
        c.pc = 0x003b_09a0;
    }

    #[test]
    fn ps2_sony_codec_internal_dtor_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 0x003b_09a0, 0);
            c.hooks.miss = norm_miss;
            dtor_fixture(c, seed);
        };
        check(
            ee::s13::f_003b09a0__generated,
            ps2_sony_codec_internal_dtor,
            72,
            &setup,
        );
        // The 72 seeds are every combination of the five choices; pinned: no
        // lists with each word/self-free mix (43, 51, 60, 68), and both
        // lists two deep with everything freed (235).
        require_paths(
            ee::s13::f_003b09a0__generated,
            72,
            &[43, 51, 60, 68, 235],
            &setup,
        );
    }

    driven!(dtor_gen, ee::s13::f_003b09a0__generated);

    driven!(dtor_hand, ps2_sony_codec_internal_dtor);

    /// The dtor list-teardown loops under delivered events (the freed
    /// frames go through the leaf `FreeFrame`). The first loop's head
    /// (0x3b0a30) is structured: the port must carry on, never resume
    /// there (the second head is covered by
    /// `ps2_sony_codec_internal_dtor_second_loop_carries_on`).
    #[test]
    fn ps2_sony_codec_internal_dtor_events() {
        let resumes = check_events(dtor_gen, dtor_hand, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x003b_09a0, 0);
            c.hooks.miss = norm_miss;
            // Both lists live (two nodes each), both words freed.
            dtor_fixture(c, 3 + 8 * 2 + 24 * 2 + 4 * (seed % 2));
            schedule(c, &mut r, seed, 0x003b_09a0)
        });
        assert!(
            resumes.get(&0x003b_0a30).is_none(),
            "unwound at structured head 0x3b0a30: {resumes:x?}"
        );
    }

    driven!(dtor_edge2_gen, ee::s13::f_003b09a0__generated);

    driven!(dtor_edge2_hand, ps2_sony_codec_internal_dtor);

    /// The dtor's second loop at 0x3b0a80 never sees an unwind
    /// under `schedule` (delivery fires in the first loop), so this test
    /// lengthens the first list (12 nodes) and sweeps the VBlank past it.
    /// Fails on the `back_edge_unwinds` body.
    #[test]
    fn ps2_sony_codec_internal_dtor_second_loop_carries_on() {
        for k in [400u64, 600] {
            let resumes = check_events(dtor_edge2_gen, dtor_edge2_hand, 4, &|c, seed| {
                machine(c, seed, 0x003b_09a0, 0);
                c.hooks.miss = norm_miss;
                dtor_fixture(c, 3 + 8 * 2 + 24 * 2 + 4 * (seed % 2));
                const CTX: u32 = DATA + 0x100;
                dtor_list(c, CTX + 0x18, DATA + 0x1000, 12);
                dtor_list(c, CTX + 0x20, DATA + 0x1800, 6);
                let saved = (c.g[A0], c.g[A1], c.g[31]);
                ssx3_ps2::kernel::start_main_thread(c, 0x003b_09a0, 0, 0);
                (c.g[A0], c.g[A1], c.g[31]) = saved;
                c.pc = 0x003b_09a0;
                c.cycles = 1000;
                c.next_event = 1000 + k;
                ssx3_ps2::platform::machine_of(&mut c.bus.plat)
                    .sched
                    .next_vblank = 1000 + k;
                Events::Unwind
            });
            assert!(
                resumes.get(&0x003b_0a80).is_none(),
                "k={k}: unwound at structured head 0x3b0a80: {resumes:x?}"
            );
        }
    }

    /// A video sink with small row/column counts. `a1` is the frame (null
    /// one seed in four), whose `+4` descriptor has a row stride at `+6`
    /// and the 0x1000 flag at `+0xc` (data at `+0x10` or at `+[+0x10]`).
    /// The graphics manager at `gp+0x2a90` has the upload method (FAKE) at
    /// `[+0x10d8]+0x18c`; the row records the sink's `+0x14` table points
    /// at hold random words.
    fn upload_fixture(c: &mut Cpu, seed: u32) {
        const SINK: u32 = DATA + 0x100;
        const MID: u32 = DATA + 0x200;
        const VT: u32 = DATA + 0x300;
        const ROWS: u32 = DATA + 0x800;
        const SCRATCH: u32 = DATA + 0x900;
        const FRAME: u32 = DATA + 0xa00;
        const DESC: u32 = DATA + 0xa40;
        const RECORDS: u32 = DATA + 0xc00;
        let mut r = Rng(0x0025_4c48 ^ u64::from(seed) << 20);
        for i in 0..0x40u32 {
            c.bus.write32(SINK + 4 * i, 0x2222_0000 + i);
        }
        c.bus.write32(SINK + 8, MID);
        c.bus.write32(SINK + 0xc, 0x1111_0000 + seed);
        c.bus.write32(SINK + 0x10, SCRATCH);
        c.bus.write32(SINK + 0x14, ROWS);
        c.bus
            .write32(SINK + 0x1c, if seed % 3 == 0 { 0 } else { 1 + seed % 3 });
        c.bus
            .write32(SINK + 0x20, if seed % 2 == 0 { 0 } else { 1 + seed % 2 });
        c.bus.write32(SINK + 0x3c, 0x4444_0000 + seed * 7);
        crate::difftest::fill(c, &mut r, RECORDS, 0x40);
        for i in 0..0x10u32 {
            c.bus.write32(ROWS + 4 * i, RECORDS + 0x10 * i);
        }
        crate::difftest::fill(c, &mut r, DESC, 0x10);
        c.bus.write16(DESC + 6, 0x10 + (seed % 5) as u16 * 0x10);
        let flags = r.next_u32() & !0x1000 | if (seed / 6) % 2 == 0 { 0 } else { 0x1000 };
        c.bus.write32(DESC + 0xc, flags);
        c.bus.write32(DESC + 0x10, 0x40 + 4 * (seed % 7));
        c.bus.write32(FRAME + 4, DESC);
        c.bus.write32(crate::difftest::GAME_GP + 0x2a90, MID);
        c.bus.write32(MID + 0x10d8, VT);
        for i in 0..0x90u32 {
            c.bus.write32(VT + 4 * i, FAKE);
        }
        c.bus.write16(VT + 0x188, 0x20);
        c.bus.write32(MID + 0xe84, SCRATCH);
        c.bus.write32(SCRATCH + 8, 0x6666_0000 + seed);
        c.g[A0][0] = SINK as u64;
        c.g[A1][0] = if (seed / 12) % 4 == 3 {
            0
        } else {
            FRAME as u64
        };
        c.pc = 0x0025_4c48;
    }

    /// `VD_TextureSink_UploadFrame`: rows upload through the fake methods,
    /// then the real draw runs.
    #[test]
    fn vd_texture_sink_upload_frame_matches() {
        check(
            ee::s06::f_00254c48__generated,
            vd_texture_sink_upload_frame,
            60,
            &|c, seed| {
                machine(c, seed, 0x0025_4c48, 0);
                upload_fixture(c, seed);
            },
        );
        require_paths(
            ee::s06::f_00254c48__generated,
            60,
            // Rows {0, 2, 3} and columns {0, 2}, each with and without
            // the descriptor flag and the null frame.
            &[253, 298, 306, 319, 326, 680, 697, 903, 916, 932],
            &|c, seed| {
                machine(c, seed, 0x0025_4c48, 0);
                upload_fixture(c, seed);
            },
        );
    }

    driven!(upl_gen, ee::s06::f_00254c48__generated);

    driven!(upl_hand, vd_texture_sink_upload_frame);

    /// The upload loops under delivered events. Both heads (0x254cc0,
    /// 0x254ce0) are structured: the port must carry on, never resume
    /// there.
    #[test]
    fn vd_texture_sink_upload_frame_events() {
        let resumes = check_events(upl_gen, upl_hand, 60, &|c, seed| {
            machine(c, seed, 0x0025_4c48, 0);
            upload_fixture(c, seed);
            schedule(c, &mut Rng(seed as u64), seed, 0x0025_4c48)
        });
        for head in [0x0025_4cc0, 0x0025_4ce0] {
            assert!(
                resumes.get(&head).is_none(),
                "unwound at structured head {head:#x}: {resumes:x?}"
            );
        }
    }

    /// The video alloc hook `VidEnginePlayer_Open` calls (`0x510000 - 0x6bd0`).
    const OPEN_ALLOC_HOOK: u32 = 0x0050_9430;

    /// Gate of the two `AV_Player` constructor fakes (the by-name one loads
    /// the file for real): logged at a fixed 4 cycles under `OPEN_MAGIC`.
    const OPEN_CTRL: u32 = DATA + 0x1f80;
    const OPEN_MAGIC: u32 = 0x0a7e_2534;
    static OPEN_PREV_CTOR: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    static OPEN_PREV_CTOR_2: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    fn open_fake_ctor(c: &mut Cpu) {
        if c.bus.read32(OPEN_CTRL) != OPEN_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                &OPEN_PREV_CTOR,
                ee::s13::f_003addc0__generated,
            );
        }
        norm_miss(c, 0x003a_ddc0);
    }

    fn open_fake_ctor_2(c: &mut Cpu) {
        if c.bus.read32(OPEN_CTRL) != OPEN_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                &OPEN_PREV_CTOR_2,
                ee::s13::f_003ade08__generated,
            );
        }
        norm_miss(c, 0x003a_de08);
    }

    /// `P` is the player (Close runs for real: `+0x38` stream and `+0x3c`
    /// method table steered), `PARAMS` the 0x24-byte open parameters copied
    /// to `P + 0x14`: `+4` the file name (null, empty or a name), `+8` and
    /// `+0x10` the two flags that pick the `movz`/`movn` arguments.
    fn open_fixture(c: &mut Cpu, seed: u32) {
        const P: u32 = DATA + 0x100;
        const PARAMS: u32 = DATA + 0x400;
        const NAME: u32 = DATA + 0x600;
        for i in 0..0x20u32 {
            c.bus.write32(P + 4 * i, 0x8888_0000 + i);
        }
        for i in 0..0x10u32 {
            c.bus.write32(PARAMS + 4 * i, 0x9999_0000 + i);
        }
        c.bus.write8(NAME, 0);
        c.bus.write32(NAME + 4, 0x0076_6964);
        let name = [0, NAME, NAME + 4][seed as usize % 3];
        c.bus.write32(PARAMS + 4, name);
        c.bus.write32(PARAMS + 8, (seed / 3) % 2);
        c.bus.write32(PARAMS + 0x10, (seed / 6) % 2);
        c.bus.write32(P + 0x38, (seed / 12) % 2);
        for i in 0..8u32 {
            c.bus.write32(DATA + 0x500 + 4 * i, FAKE);
        }
        c.bus.write32(
            P + 0x3c,
            if (seed / 24) % 2 == 0 {
                0
            } else {
                DATA + 0x500
            },
        );
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_003ADDC0),
            open_fake_ctor,
            &OPEN_PREV_CTOR,
        );
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_003ADE08),
            open_fake_ctor_2,
            &OPEN_PREV_CTOR_2,
        );
        c.bus.write32(OPEN_CTRL, OPEN_MAGIC);
        c.bus.write32(OPEN_ALLOC_HOOK, FAKE);
        c.bus.write32(OPEN_ALLOC_HOOK + 4, FAKE);
        c.bus.write32(OPEN_ALLOC_HOOK + 8, 0x7777_0000 + seed);
        c.g[A0][0] = P as u64;
        c.g[A1][0] = PARAMS as u64;
        c.pc = 0x0025_34a8;
    }

    /// `VidEnginePlayer_Open`: close (quiet or loud), copy params, open by
    /// name or by stream with each flag mix; the alloc hook and the open
    /// callees are fakes.
    #[test]
    fn vid_engine_player_open_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 0x0025_34a8, 0);
            c.hooks.miss = norm_miss;
            open_fixture(c, seed);
        };
        check(
            ee::s06::f_002534a8__generated,
            vid_engine_player_open,
            48,
            &setup,
        );
        // Stream or by-name open (null/empty name vs a name), times the two
        // flags, times a quiet or loud Close.
        require_paths(
            ee::s06::f_002534a8__generated,
            48,
            &[114, 117, 119, 127, 130, 132, 185, 188, 190, 207, 210, 212],
            &setup,
        );
    }

    /// Gate and mailbox of the `DLHeap_MoreCore` fake for the SysTrim test:
    /// with `SYSTRIM_MAGIC` at `SYSTRIM_CTRL` the n-th call returns the
    /// sign-extended word `SYSTRIM_RESULTS + 4n` (so -1 can be forced).
    const SYSTRIM_CTRL: u32 = DATA + 0x1f40;
    const SYSTRIM_MAGIC: u32 = 0x5e75_7219;
    const SYSTRIM_CALLS: u32 = DATA + 0x1f44;
    const SYSTRIM_RESULTS: u32 = DATA + 0x1f48;
    static SYSTRIM_PREV_MORE_CORE: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    fn systrim_fake_more_core(c: &mut Cpu) {
        if c.bus.read32(SYSTRIM_CTRL) != SYSTRIM_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                &SYSTRIM_PREV_MORE_CORE,
                ee::s10::f_0031ffd8__generated,
            );
        }
        let n = c.bus.read32(SYSTRIM_CALLS);
        c.bus.write32(SYSTRIM_CALLS, n + 1);
        let v = c.bus.read32(SYSTRIM_RESULTS + 4 * n);
        c.g[V0][0] = ssx3_ee_rt::cpu::sx(v);
        c.cycles += 5;
        c.pc = c.g[RA][0] as u32;
    }

    /// The SysTrim fixture: a real arena (`A2` = its `malloc_state`, the
    /// after one real allocation the top chunk holds the rest of
    /// the first `MoreCore` grant) and a gated `MoreCore` that
    /// answers per seed: 0 the pad covers the top chunk (`extra <= 0`), 1 the
    /// break moved (`MoreCore(0) != top + size`), 2 the shrink failed
    /// (`MoreCore(0)` after it returns -1), 3 nothing was released, 4 the
    /// release worked (footprint and top size updated).
    fn systrim_fixture(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x0031_e6d8, 0);
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_0031FFD8),
            systrim_fake_more_core,
            &SYSTRIM_PREV_MORE_CORE,
        );
        let heap = heap_arena(c);
        // One real allocation makes the arena take its memory (the top chunk).
        let ra = c.g[RA][0];
        c.g[A0][0] = heap as u64;
        c.g[A1][0] = 0x40 + (r.next_u32() % 0x400) as u64;
        c.g[RA][0] = 0x0000_7770;
        c.pc = 0x0031_e818;
        ee::s10::DLHeap_Malloc(c);
        assert_eq!(c.pc, 0x0000_7770, "Malloc did not return");
        c.g[RA][0] = ra;
        let state = c.bus.read32(heap);
        let top = c.bus.read32(state + 0x30);
        let size = c.bus.read32(top + 4) & !3;
        assert!(
            size > 0x1000,
            "no top chunk: state {state:#x} top {top:#x} size {size:#x}"
        );
        let page = c.bus.read32(state + 0x35c);
        assert!(page.is_power_of_two(), "pagesize {page:#x}");
        let brk = top + size;
        let case = seed % 5;
        let pad = if case == 0 {
            // A pad past the top chunk leaves nothing to trim (extra <= 0).
            [size + 0x1000, 0x7fff_0000, 0x10_0000][(seed / 5) as usize % 3]
        } else {
            [0, 0x100, 0x400][(seed / 5) as usize % 3]
        };
        let release = (r.next_u32() % 2 + 1) * page;
        let results = match case {
            1 => [brk + 0x1000, 0, 0],
            2 => [brk, brk, u32::MAX],
            3 => [brk, brk, brk],
            _ => [brk, brk, brk - release],
        };
        crate::difftest::put_words(c, SYSTRIM_RESULTS, &results);
        c.bus.write32(SYSTRIM_CALLS, 0);
        c.bus.write32(SYSTRIM_CTRL, SYSTRIM_MAGIC);
        c.g[A0][0] = heap as u64;
        c.g[A1][0] = pad as u64;
        c.g[A2][0] = state as u64;
        c.pc = 0x0031_e6d8;
    }

    /// `DLHeap_SysTrim`: every exit of the trim on a real arena, with the
    /// break answers from the gated `MoreCore` fake.
    #[test]
    fn dlheap_sys_trim_matches() {
        check(
            ee::s10::f_0031e6d8__generated,
            dlheap_sys_trim,
            60,
            &systrim_fixture,
        );
        // Moved break 173, shrink failed 195, nothing released 197, nothing
        // to trim 199, released 209.
        require_paths(
            ee::s10::f_0031e6d8__generated,
            60,
            &[173, 195, 197, 199, 209],
            &systrim_fixture,
        );
    }

    /// The MD5 Update/Final fixture: a context whose bit count puts the
    /// buffered bytes at index 0, 10, 50, 55, 60 or 63 (one count wraps the
    /// low word, so the carry into the high word happens), and inputs of 0
    /// to 200 bytes; the game image holds Final's padding block. Outputs
    /// (`a1` for Final) are prefilled.
    fn md5uf_fixture(c: &mut Cpu, seed: u32, addr: u32) -> Rng {
        let mut r = machine(c, seed, addr as u64, 0);
        crate::difftest::elf_image(c);
        const CTX: u32 = DATA + 0x100;
        const BUF: u32 = DATA + 0x400;
        crate::difftest::fill(c, &mut r, CTX, 0x20);
        let index = [0u32, 10, 50, 55, 60, 63][seed as usize % 6];
        let low = if seed % 7 == 6 {
            0xffff_fe00 | index << 3
        } else {
            (r.next_u32() & !0x1ff) | index << 3
        };
        c.bus.write32(CTX, low);
        let len = match (seed / 6) % 8 {
            0 => 0,
            1 => 1,
            2 => 3,
            3 => 9,
            4 => 64,
            5 => 64 - index as usize,
            6 => 100,
            _ => (r.next_u32() % 200) as usize,
        };
        crate::difftest::fill(c, &mut r, BUF, 0x40);
        for i in 0..len {
            c.bus.write8(BUF + i as u32, 1 + (r.next_u32() % 95) as u8);
        }
        c.bus.write8(BUF + len as u32, 0);
        c.g[A0][0] = CTX as u64;
        c.g[A1][0] = BUF as u64;
        c.g[A2][0] = len as u64;
        c.pc = addr;
        r
    }

    /// `MD5_Update` / `MD5_Final`: every buffer index against short and
    /// long inputs (both the "fits in the buffer" and the transform paths),
    /// the count carry, and both padding lengths of Final.
    #[test]
    fn md5_update_final_matches() {
        for (gen, hand, addr, pins) in [
            (
                ee::s10::f_0031d3a8__generated as fn(&mut Cpu),
                md5_update as fn(&mut Cpu),
                0x0031_d3a8,
                // Empty input 27; buffered-only copies 62..163; transforms
                // from index 0 (1045) up to 200 bytes from index 63 (3523).
                &[27, 62, 65, 76, 79, 118, 121, 163, 1045, 3523][..],
            ),
            (
                ee::s10::f_0031d4d0__generated as fn(&mut Cpu),
                md5_final as fn(&mut Cpu),
                0x0031_d4d0,
                // Both padding lengths (index below / from 56) at each index.
                &[
                    1504, 1506, 1539, 1541, 1822, 1824, 1889, 1891, 2922, 2924, 2943, 2945,
                ][..],
            ),
        ] {
            check(gen, hand, 96, &|c, seed| {
                md5uf_fixture(c, seed, addr);
            });
            require_paths(gen, 96, pins, &|c, seed| {
                md5uf_fixture(c, seed, addr);
            });
        }
    }

    driven!(upd_gen, ee::s10::f_0031d3a8__generated);

    driven!(upd_hand, md5_update);

    driven!(muf_gen, ee::s10::f_0031d4d0__generated);

    driven!(muf_hand, md5_final);

    /// The MD5 loops under delivered events. Final's first head (0x31d4f0)
    /// is structured: the port must carry on, never resume there (Update's
    /// 0x31d478 head is a correct dispatcher edge; the second Final head is
    /// covered by `md5_final_second_loop_carries_on`).
    #[test]
    fn md5_update_final_events() {
        for (gen, hand, addr, no_resume) in [
            (
                upd_gen as fn(&mut Cpu),
                upd_hand as fn(&mut Cpu),
                0x0031_d3a8,
                None,
            ),
            (
                muf_gen as fn(&mut Cpu),
                muf_hand as fn(&mut Cpu),
                0x0031_d4d0,
                Some(0x0031_d4f0),
            ),
        ] {
            let resumes = check_events(gen, hand, 60, &|c, seed| {
                let mut r = machine(c, seed, addr as u64, 0);
                const CTX: u32 = DATA + 0x100;
                const BUF: u32 = DATA + 0x400;
                for i in 0..0x20u32 {
                    c.bus.write32(CTX + 4 * i, 0x7777_0000 + i);
                }
                let len = 40 + (r.next_u32() % 200) as usize;
                for i in 0..len {
                    c.bus.write8(BUF + i as u32, 1 + (r.next_u32() % 95) as u8);
                }
                c.bus.write8(BUF + len as u32, 0);
                c.g[A0][0] = CTX as u64;
                c.g[A1][0] = BUF as u64;
                c.g[A2][0] = len as u64;
                schedule(c, &mut r, seed, addr)
            });
            if let Some(head) = no_resume {
                assert!(
                    resumes.get(&head).is_none(),
                    "unwound at structured head {head:#x}: {resumes:x?}"
                );
            }
        }
    }

    driven!(fin_carry_gen, ee::s10::f_0031d4d0__generated);

    driven!(fin_carry_hand, md5_final);

    /// An unwind requested at a structured back edge must not unwind
    /// the port — the generated loop carries on (`deliver_event_here`).
    /// Fails on the `back_edge_unwinds` body (resume at 0x31d4f0).
    ///
    /// The VBlank is due at the t-th back edge of the first loop by
    /// construction (prologue 8 cycles, body 12, start 1000): `schedule`
    /// cannot do this, its k < 48 fires before a loop this short.
    #[test]
    fn md5_final_back_edge_carries_on() {
        for t in 1..=7u64 {
            let resumes = check_events(fin_carry_gen, fin_carry_hand, 4, &|c, seed| {
                md5uf_fixture(c, seed, 0x0031_d4d0);
                let saved = (c.g[A0], c.g[A1], c.g[31]);
                ssx3_ps2::kernel::start_main_thread(c, 0x0031_d4d0, 0, 0);
                (c.g[A0], c.g[A1], c.g[31]) = saved;
                c.pc = 0x0031_d4d0;
                c.cycles = 1000;
                let at = 1000 + 8 + 12 * t;
                c.next_event = at;
                ssx3_ps2::platform::machine_of(&mut c.bus.plat)
                    .sched
                    .next_vblank = at;
                Events::Unwind
            });
            assert!(
                resumes.get(&0x0031_d4f0).is_none(),
                "t={t}: unwound at structured head 0x31d4f0: {resumes:x?}"
            );
        }
    }

    driven!(fin_edge2_gen, ee::s10::f_0031d4d0__generated);

    driven!(fin_edge2_hand, md5_final);

    /// The second loop at 0x31d560 never sees an unwind under
    /// `schedule` (delivery fires in the first loop), so this test sweeps
    /// the VBlank past the first loop and the two Updates (short seeds:
    /// lens 0, 1, 3, 9). Fails on the `back_edge_unwinds` body.
    #[test]
    fn md5_final_second_loop_carries_on() {
        for k in [1100u64, 1150, 1200, 1250, 1300, 1350, 1400, 1500] {
            let resumes = check_events(fin_edge2_gen, fin_edge2_hand, 4, &|c, seed| {
                md5uf_fixture(c, seed, 0x0031_d4d0);
                let saved = (c.g[A0], c.g[A1], c.g[31]);
                ssx3_ps2::kernel::start_main_thread(c, 0x0031_d4d0, 0, 0);
                (c.g[A0], c.g[A1], c.g[31]) = saved;
                c.pc = 0x0031_d4d0;
                c.cycles = 1000;
                c.next_event = 1000 + k;
                ssx3_ps2::platform::machine_of(&mut c.bus.plat)
                    .sched
                    .next_vblank = 1000 + k;
                Events::Unwind
            });
            assert!(
                resumes.get(&0x0031_d560).is_none(),
                "k={k}: unwound at structured head 0x31d560: {resumes:x?}"
            );
        }
    }

    /// `Serialize_ReadDataBlock`: a zero count skips the loop; live counts

    /// run the real byte reads.
    #[test]
    fn serialize_read_data_block_matches() {
        check(
            ee::s09::f_002d2868__generated,
            serialize_read_data_block,
            60,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x002d_2868, 0);
                const STREAM: u32 = DATA + 0x100;
                const BUF: u32 = DATA + 0x200;
                const OUT: u32 = DATA + 0x400;
                c.bus.write32(STREAM, BUF);
                for i in 0..0x20u32 {
                    c.bus.write32(BUF + 4 * i, 0x3333_0000 + i);
                }
                // Stream format: count word first, then the bytes. (The
                // function overwrites [OUT] with its header; the header's
                // count word is the stream's first word.)
                let n = (r.next_u32() % 24) as usize;
                c.bus.write32(BUF, n as u32);
                for i in 0..n {
                    c.bus.write8(BUF + 4 + i as u32, 0x41 + (i % 26) as u8);
                }
                for i in n..0x20 {
                    c.bus.write8(BUF + 4 + i as u32, 0x77);
                }
                c.bus.write32(OUT, 0xdead_beef);
                c.g[A0][0] = STREAM as u64;
                c.g[A1][0] = OUT as u64;
                c.pc = 0x002d_2868;
            },
        );
        require_paths(
            ee::s09::f_002d2868__generated,
            60,
            &[
                464, 482, 500, 518, 536, 554, 590, 607, 643, 679, 697, 715, 746, 850, 868, 886,
                904, 922, 940, 958, 1409,
            ],
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x002d_2868, 0);
                const STREAM: u32 = DATA + 0x100;
                const BUF: u32 = DATA + 0x200;
                const OUT: u32 = DATA + 0x400;
                c.bus.write32(STREAM, BUF);
                for i in 0..0x20u32 {
                    c.bus.write32(BUF + 4 * i, 0x3333_0000 + i);
                }
                // Stream format: count word first, then the bytes.
                let n = (r.next_u32() % 24) as usize;
                c.bus.write32(BUF, n as u32);
                for i in 0..n {
                    c.bus.write8(BUF + 4 + i as u32, 0x41 + (i % 26) as u8);
                }
                for i in n..0x20 {
                    c.bus.write8(BUF + 4 + i as u32, 0x77);
                }
                c.bus.write32(OUT, 0xdead_beef);
                c.g[A0][0] = STREAM as u64;
                c.g[A1][0] = OUT as u64;
                c.pc = 0x002d_2868;
            },
        );
    }

    driven!(rdb_gen, ee::s09::f_002d2868__generated);

    driven!(rdb_hand, serialize_read_data_block);

    /// The data-block read loop under delivered events (the 0x2d28f8 head
    /// itself is covered by `serialize_read_data_block_back_edge_carries_on`:
    /// `schedule` fires in the prologue calls).
    #[test]
    fn serialize_read_data_block_events() {
        check_events(rdb_gen, rdb_hand, 60, &|c, seed| {
            let mut r = machine_rt(c, seed, 0x002d_2868, 0);
            const STREAM: u32 = DATA + 0x100;
            const BUF: u32 = DATA + 0x200;
            const OUT: u32 = DATA + 0x400;
            c.bus.write32(STREAM, BUF);
            for i in 0..0x20u32 {
                c.bus.write32(BUF + 4 * i, 0x3333_0000 + i);
            }
            // Stream format: count word first, then the bytes.
            let n = 8 + (r.next_u32() % 24) as usize;
            c.bus.write32(BUF, n as u32);
            for i in 0..n {
                c.bus.write8(BUF + 4 + i as u32, 0x41 + (i % 26) as u8);
            }
            for i in n..0x40 {
                c.bus.write8(BUF + 4 + i as u32, 0x77);
            }
            c.bus.write32(OUT, 0xdead_beef);
            c.g[A0][0] = STREAM as u64;
            c.g[A1][0] = OUT as u64;
            schedule(c, &mut r, seed, 0x002d_2868)
        });
    }

    driven!(rdb_edge_gen, ee::s09::f_002d2868__generated);

    driven!(rdb_edge_hand, serialize_read_data_block);

    /// The data-block loop at 0x2d28f8 never sees an unwind under
    /// `schedule` (delivery fires in the prologue calls), so this test puts
    /// the VBlank past the prologue by construction (fixed n = 200, k
    /// sweep). Fails on the `back_edge_unwinds` body.
    #[test]
    fn serialize_read_data_block_back_edge_carries_on() {
        for k in [1000u64, 3000] {
            let resumes = check_events(rdb_edge_gen, rdb_edge_hand, 4, &|c, seed| {
                machine_rt(c, seed, 0x002d_2868, 0);
                const STREAM: u32 = DATA + 0x100;
                const BUF: u32 = DATA + 0x200;
                const OUT: u32 = DATA + 0x400;
                c.bus.write32(STREAM, BUF);
                for i in 0..0x20u32 {
                    c.bus.write32(BUF + 4 * i, 0x3333_0000 + i);
                }
                let n = 200usize;
                c.bus.write32(BUF, n as u32);
                for i in 0..n {
                    c.bus.write8(BUF + 4 + i as u32, 0x41 + (i % 26) as u8);
                }
                c.bus.write32(OUT, 0xdead_beef);
                c.g[A0][0] = STREAM as u64;
                c.g[A1][0] = OUT as u64;
                let saved = (c.g[A0], c.g[A1], c.g[31]);
                ssx3_ps2::kernel::start_main_thread(c, 0x002d_2868, 0, 0);
                (c.g[A0], c.g[A1], c.g[31]) = saved;
                c.pc = 0x002d_2868;
                c.cycles = 1000;
                c.next_event = 1000 + k;
                ssx3_ps2::platform::machine_of(&mut c.bus.plat)
                    .sched
                    .next_vblank = 1000 + k;
                Events::Unwind
            });
            assert!(
                resumes.get(&0x002d_28f8).is_none(),
                "k={k}: unwound at structured head 0x2d28f8: {resumes:x?}"
            );
        }
    }

    /// `Serialize_ReadString`: short strings and boundary lengths land in
    /// the out area on both sides.
    #[test]
    fn serialize_read_string_matches() {
        check(
            ee::s09::f_002d27b0__generated,
            serialize_read_string,
            100,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x002d_27b0, 0);
                const STREAM: u32 = DATA + 0x100;
                const BUF: u32 = DATA + 0x200;
                const OUT: u32 = DATA + 0x400;
                c.bus.write32(STREAM, BUF);
                for i in 0..0x40u32 {
                    c.bus.write32(BUF + 4 * i, 0x5555_0000 + i);
                }
                let len = match seed % 6 {
                    0 => 0,
                    1 => 1,
                    2 => 31,
                    3 => 32,
                    4 => 999,
                    _ => (r.next_u32() % 40) as usize,
                };
                for i in 0..len {
                    c.bus.write8(BUF + i as u32, 0x61 + (i % 26) as u8);
                }
                c.bus.write8(BUF + len as u32, 0);
                c.g[A0][0] = STREAM as u64;
                c.g[A1][0] = OUT as u64;
                c.pc = 0x002d_27b0;
            },
        );
        require_paths(
            ee::s09::f_002d27b0__generated,
            100,
            &[
                290, 311, 371, 472, 530, 732, 755, 782, 860, 993, 1023, 1025, 1052, 1086, 1142,
                1171, 21393,
            ],
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x002d_27b0, 0);
                const STREAM: u32 = DATA + 0x100;
                const BUF: u32 = DATA + 0x200;
                const OUT: u32 = DATA + 0x400;
                c.bus.write32(STREAM, BUF);
                for i in 0..0x40u32 {
                    c.bus.write32(BUF + 4 * i, 0x5555_0000 + i);
                }
                let len = match seed % 6 {
                    0 => 0,
                    1 => 1,
                    2 => 31,
                    3 => 32,
                    4 => 999,
                    _ => (r.next_u32() % 40) as usize,
                };
                for i in 0..len {
                    c.bus.write8(BUF + i as u32, 0x61 + (i % 26) as u8);
                }
                c.bus.write8(BUF + len as u32, 0);
                c.g[A0][0] = STREAM as u64;
                c.g[A1][0] = OUT as u64;
                c.pc = 0x002d_27b0;
            },
        );
    }

    driven!(rst_gen, ee::s09::f_002d27b0__generated);

    driven!(rst_hand, serialize_read_string);

    /// The string read loop under delivered events. The head (0x2d27e8) is
    /// structured: the port must carry on, never resume there.
    #[test]
    fn serialize_read_string_events() {
        let resumes = check_events(rst_gen, rst_hand, 100, &|c, seed| {
            let mut r = machine_rt(c, seed, 0x002d_27b0, 0);
            const STREAM: u32 = DATA + 0x100;
            const BUF: u32 = DATA + 0x200;
            const OUT: u32 = DATA + 0x400;
            c.bus.write32(STREAM, BUF);
            for i in 0..0x40u32 {
                c.bus.write32(BUF + 4 * i, 0x5555_0000 + i);
            }
            let len = 20 + (r.next_u32() % 60) as usize;
            for i in 0..len {
                c.bus.write8(BUF + i as u32, 0x61 + (i % 26) as u8);
            }
            c.bus.write8(BUF + len as u32, 0);
            c.g[A0][0] = STREAM as u64;
            c.g[A1][0] = OUT as u64;
            schedule(c, &mut r, seed, 0x002d_27b0)
        });
        assert!(
            resumes.get(&0x002d_27e8).is_none(),
            "unwound at structured head 0x2d27e8: {resumes:x?}"
        );
    }

    /// `VidEnginePlayer_Render`: null frames and null sinks skip the calls;
    /// live ones run the fake render method.
    #[test]
    fn vid_engine_player_render_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 0x0025_3890, 0);
            c.hooks.miss = norm_miss;
            const P: u32 = DATA + 0x100;
            const TAB: u32 = DATA + 0x300;
            const VT: u32 = DATA + 0x400;
            for i in 0..0x20u32 {
                c.bus.write32(P + 4 * i, 0x6666_0000 + i);
            }
            // The slot word rides bit 1 while `a1` rides bit 0: the
            // join needs `a1 == 0` with a null slot for one arm and a
            // nonzero slot for the other.
            c.bus
                .write32(P, if seed / 2 % 2 == 0 { 0 } else { 0x6666_0000 });
            c.bus.write32(P + 0x38, if seed % 3 == 0 { 0 } else { 1 });
            c.bus
                .write32(P + 0x3c, if (seed / 3) % 2 == 0 { 0 } else { TAB });
            c.bus.write32(
                P + 0x8,
                if (seed / 5) % 2 == 0 {
                    0
                } else {
                    0x7777_0000 + seed
                },
            );
            c.bus.write32(
                P + 0x40,
                if (seed / 7) % 2 == 0 {
                    0
                } else {
                    0x8888_0000 + seed
                },
            );
            c.bus.write32(
                P + 0x4,
                if (seed / 11) % 2 == 0 {
                    0
                } else {
                    0x9999_0000 + seed
                },
            );
            for i in 0..0x10u32 {
                c.bus.write32(TAB + 4 * i, FAKE);
            }
            c.bus.write32(TAB + 4, VT);
            for i in 0..0x20u32 {
                c.bus.write32(VT + 4 * i, FAKE);
            }
            c.g[A0][0] = P as u64;
            c.g[A1][0] = (seed % 2) as u64;
            c.pc = 0x0025_3890;
        };
        check(
            ee::s06::f_00253890__generated,
            vid_engine_player_render,
            100,
            &setup,
        );
        require_paths(
            ee::s06::f_00253890__generated,
            100,
            &[10, 14, 25, 28, 34, 38],
            &setup,
        );
        // The null-slot arm is the only writer of P+0x40 in
        // `Render`, clearing it after the 0x253920 call; the nonzero-slot
        // arm (0x253904 call) leaves it. The arms are cycle-identical, so
        // the clobber (and its absence) is what shows each ran.
        let mut clobbered = false;
        let mut left = false;
        for seed in 0..crate::difftest::seed_count(100) {
            let mut c = crate::difftest::fresh();
            setup(&mut c, seed);
            // The Join is reached with A1 == 0 (seed % 2 == 0), P+0x38
            // and P+0x3c (TAB) set, P+0x8 clear and P+0x40 set; the slot
            // word rides bit 1, so each arm is reachable.
            let slot_null = c.bus.read32(DATA + 0x100) == 0;
            let reaches_join = seed % 2 == 0
                && c.bus.read32(DATA + 0x138) != 0
                && c.bus.read32(DATA + 0x13c) == DATA + 0x300
                && c.bus.read32(DATA + 0x108) == 0
                && c.bus.read32(DATA + 0x140) != 0;
            crate::difftest::run_generated(&mut c, ee::s06::f_00253890__generated);
            if reaches_join && slot_null {
                assert_eq!(
                    c.bus.read32(DATA + 0x140),
                    0,
                    "seed {seed}: slot-0x20 arm did not clear P+0x40"
                );
                clobbered = true;
            }
            if reaches_join && !slot_null {
                assert_ne!(
                    c.bus.read32(DATA + 0x140),
                    0,
                    "seed {seed}: nonzero-slot arm cleared P+0x40"
                );
                left = true;
            }
        }
        assert!(clobbered, "slot-0x20 arm never ran with a nonzero word");
        assert!(left, "nonzero-slot arm never ran with a1 == 0");
    }

    /// `PS2ExecMan_v03`: syscalls run against the test machine on both
    /// sides.
    #[test]
    fn ps2_exec_man_v03_matches() {
        check(
            ee::s10::f_0031a920__generated,
            ps2_exec_man_v03,
            30,
            &|c, seed| {
                machine(c, seed, 0x0031_a920, 0);
                c.hooks.miss = norm_miss;
                const MAN: u32 = DATA + 0x100;
                const MEM: u32 = DATA + 0x10000;
                for i in 0..0x10u32 {
                    c.bus.write32(MAN + 4 * i, 0xaaaa_0000 + i);
                }
                for i in 0..0x1100u32 {
                    c.bus.write32(MEM + 4 * i, 0xbbbb_0000 + i + seed);
                }
                c.g[A0][0] = MAN as u64;
                c.pc = 0x0031_a920;
            },
        );
        require_paths(
            ee::s10::f_0031a920__generated,
            30,
            &[
                224, 232, 233, 234, 235, 236, 239, 240, 242, 243, 244, 245, 246, 248, 249, 251,
                252, 256,
            ],
            &|c, seed| {
                machine(c, seed, 0x0031_a920, 0);
                c.hooks.miss = norm_miss;
                const MAN: u32 = DATA + 0x100;
                const MEM: u32 = DATA + 0x10000;
                for i in 0..0x10u32 {
                    c.bus.write32(MAN + 4 * i, 0xaaaa_0000 + i);
                }
                for i in 0..0x1100u32 {
                    c.bus.write32(MEM + 4 * i, 0xbbbb_0000 + i + seed);
                }
                c.g[A0][0] = MAN as u64;
                c.pc = 0x0031_a920;
            },
        );
    }
}
