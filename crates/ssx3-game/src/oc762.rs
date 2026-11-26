//! Staging module oc762 (wave wave244): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc762`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, tail_wrapper, Callee, Frame, Target};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002b_1c50, song_instance_ctor, "SongInstance_ctor"),
    (0x0039_d758, ui_screen_handle_input, "UIScreen_HandleInput"),
    (
        0x0018_1bd0,
        cfe_state_char_select_update_attribute_bars,
        "cFEStateCharSelect_UpdateAttributeBars",
    ),
    (0x0028_bc58, snd_bank_man_load_bank, "SndBankMan_LoadBank"),
    (0x0039_1a48, font_upload_texture, "Font_UploadTexture"),
    (
        0x002c_03e8,
        world_painter_query_reset_all,
        "WorldPainterQuery_ResetAll",
    ),
];

/// `SndBank_IsLoadedWithName` (0x28b830).
const OC762_BANK_IS_LOADED: Callee = Callee {
    addr: 0x0028_b830,
    f: ee::s07::f_0028b830,
};
/// `SndLoadQueue_CancelBank` (0x28a728).
const OC762_CANCEL_BANK: Callee = Callee {
    addr: 0x0028_a728,
    f: ee::s07::f_0028a728,
};
/// `SndBank_Load` (0x28b320).
const OC762_BANK_LOAD: Callee = Callee {
    addr: 0x0028_b320,
    f: ee::s07::f_0028b320,
};
/// `MusicPlayer_LoadSongData` (0x2b4a20).
const OC762_LOAD_SONG_DATA: Callee = Callee {
    addr: 0x002b_4a20,
    f: ee::s08::f_002b4a20,
};
/// `SongInstance_Update` (0x2b21e0).
const OC762_SONG_UPDATE: Callee = Callee {
    addr: 0x002b_21e0,
    f: ee::s08::f_002b21e0,
};
/// `WorldPainterQuery_ResetList` (0x2c03a8).
const OC762_QUERY_RESET_LIST: Callee = Callee {
    addr: 0x002c_03a8,
    f: ee::s08::f_002c03a8,
};
/// `cUIGroup_HandleInput` (0x398798).
const OC762_UIGROUP_HANDLE_INPUT: Callee = Callee {
    addr: 0x0039_8798,
    f: ee::s13::f_00398798,
};

/// `void WorldPainterQuery_ResetAll(void)` (0x2c03e8): forwards to
/// `WorldPainterQuery_ResetList()` (7-cycle wrapper, 0x10 frame holding `ra`).
pub fn world_painter_query_reset_all(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(OC762_QUERY_RESET_LIST), 0x002c_03f8)
}

/// `void UIScreen_HandleInput(UIScreen* self)` (0x39d758):
/// `cUIGroup_HandleInput(&self->group)`, the group sitting at `self + 0x40`
/// (7 cycles, 0x10 frame holding `ra`).
pub fn ui_screen_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
        OC762_UIGROUP_HANDLE_INPUT.call(c, 0x0039_d768)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `SongInstance* SongInstance_ctor(SongInstance* self, name, ?, owner, track, ...)`
/// (0x2b1c50): fills the music song instance (fields `+0x1c..+0x5c`), allocates
/// its 0x37000-byte stream buffer and track memory, scales the track's volume
/// (`track.vol[idx] = clamp(vol[idx] * owner.gain * gp float, 0..127)`, then
/// stores it as a byte in the track table), builds the "audio/music.big" /
/// "audio/music2.big" path and the song file name, asks `MusicPlayer` to load
/// the song data and, when the caller's stack argument is set, runs
/// `SongInstance_Update(self, 1)`. Returns `self` in `v0`. Block structure
/// as in the generated body (the float clamp is a chain of branches).
pub fn song_instance_ctor(c: &mut Cpu) {
    guest(c, |c| {
        'b_2b1df8: {
            'b_2b1df4: {
                'b_2b1da4: {
                    c.cycles += 74;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x170));
                    c.g[V0][0] = 0xffffffffffffffff;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x150));
                    c.g[V1][0] = 0x1;
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x140));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x130));
                    c.g[S2][0] = c.g[T0][0];
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x120));
                    c.g[T0][0] = 0x18;
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x110));
                    c.g[S4][0] = c.g[A1][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x160));
                    c.g[S3][0] = c.g[A3][0];
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x100), c.g[RA][0]);
                    c.g[A1][0] = 0x480000;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x34), c.g[T3][0]);
                    c.g[A0][0] = 0x30000;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x3c), c.g[T2][0]);
                    c.g[A2][0] = 0u64;
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x40), c.g[T1][0]);
                    c.g[A3][0] = 0u64;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x54), c.g[V1][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x33f0));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.g[A0][0] = c.g[A0][0] | 0x7000;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x24), 0u64);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x28), 0u64);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x2c), 0u64);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x38), 0u64);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x48), 0u64);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x4c), 0u64);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x50), 0u64);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x5c), 0u64);
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
                    c.mult(0, 2, 2, 8);
                    c.g[S5][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x170));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[T0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x14));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x44), c.g[T0][0]);
                    call_ptr(c, 0x00317e30, 0x002b1d00)?;
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = 0x1;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                    call_ptr(c, 0x003d3058, 0x002b1d10)?;
                    c.g[A1][0] = 0x480000;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3408));
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317e30, 0x002b1d28)?;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4554));
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
                    c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x88));
                    c.cvt_s_w(0, 0);
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x38));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.mul_s(2, 0, 1);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[AT][0] = 0x42fe0000;
                    c.mtc1(1, 3);
                    c.lwc1(0, c.g[V0][0] as u32);
                    c.cvt_s_w(0, 0);
                    c.mul_s(0, 0, 2);
                    c.c_lt_s(0, 3);
                    let t = !c.fcc();
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    if t {
                        break 'b_2b1da4;
                    } else {
                        c.cycles += 11;
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
                        c.mtc1(0, 1);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.lwc1(0, c.g[V0][0] as u32);
                        c.cvt_s_w(0, 0);
                        c.mul_s(0, 0, 2);
                        c.c_le_s(1, 0);
                        let t = !c.fcc();
                        c.g[V0][0] = 0u64;
                        if t {
                            break 'b_2b1df4;
                        } else {
                            break 'b_2b1da4;
                        }
                    }
                }
                c.cycles += 10;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.lwc1(0, c.g[V0][0] as u32);
                c.cvt_s_w(0, 0);
                c.mul_s(0, 0, 2);
                c.c_lt_s(0, 3);
                let t = !c.fcc();
                c.g[V0][0] = 0x7f;
                if t {
                    break 'b_2b1df4;
                } else {
                    c.cycles += 10;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.lwc1(0, c.g[V0][0] as u32);
                    c.cvt_s_w(0, 0);
                    c.mul_s(0, 0, 2);
                    c.cvt_w_s(1, 0);
                    c.mfc1(2, 1);
                    let t = true;
                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                    if t {
                        break 'b_2b1df8;
                    } else {
                        break 'b_2b1df4;
                    }
                }
            }
            c.cycles += 1;
            c.sw(c.g[V1][0] as u32, c.g[V0][0]);
            break 'b_2b1df8;
        }
        'b_2b1e6c: {
            'b_2b1e40: {
                c.cycles += 12;
                c.g[V0][0] = 0x18;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
                c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x38));
                c.mult(0, 4, 4, 2);
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                c.g[V0][0] = ((c.g[V0][0] as i64) < 127i64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                if t {
                    break 'b_2b1e40;
                } else {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    let t = (c.g[V1][0] as i64) < 0;
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_2b1e6c;
                    } else {
                        break 'b_2b1e40;
                    }
                }
            }
            c.cycles += 7;
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            c.g[V1][0] = ((c.g[V1][0] as i64) < 127i64) as u64;
            if c.g[V1][0] == 0u64 {
                c.g[V0][0] = 0x7f;
                break 'b_2b1e6c;
            } else {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[V0][0] = c.lbu(c.g[V0][0] as u32);
                break 'b_2b1e6c;
            }
        }
        'b_2b1ec4: {
            'b_2b1ea4: {
                c.cycles += 5;
                c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                c.g[V1][0] = 0x1;
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xa4));
                let t = c.g[V0][0] != c.g[V1][0];
                c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x60));
                if t {
                    break 'b_2b1ea4;
                } else {
                    c.cycles += 9;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A2][0] = 0x4a0000;
                    c.g[A3][0] = 0x480000;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3590));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3588));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x3420));
                    c.g[T0][0] = c.g[S4][0];
                    let t = true;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        break 'b_2b1ec4;
                    } else {
                        break 'b_2b1ea4;
                    }
                }
            }
            c.cycles += 8;
            c.g[A1][0] = 0x4a0000;
            c.g[A2][0] = 0x4a0000;
            c.g[A3][0] = 0x480000;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3590));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3588));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x3430));
            c.g[T0][0] = c.g[S4][0];
            c.g[A0][0] = c.g[S0][0];
            break 'b_2b1ec4;
        }
        'b_2b1f4c: {
            c.cycles += 32;
            call_ptr(c, 0x00417828, 0x002b1ecc)?;
            c.g[A2][0] = c.g[S0][0];
            c.sw(c.g[S1][0] as u32, c.g[S3][0]);
            c.g[A0][0] = 0x18;
            c.g[A1][0] = 0x4a0000;
            c.g[A3][0] = c.g[S3][0];
            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x36e0));
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
            c.mult(0, 3, 3, 4);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.ldl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
            c.ldr(3, c.g[V0][0] as u32);
            c.ldl(8, (c.g[V0][0] as u32).wrapping_add(0xf));
            c.ldr(8, (c.g[V0][0] as u32).wrapping_add(0x8));
            c.ldl(9, (c.g[V0][0] as u32).wrapping_add(0x17));
            c.ldr(9, (c.g[V0][0] as u32).wrapping_add(0x10));
            c.sdl(3, (c.g[S1][0] as u32).wrapping_add(0xb));
            c.sdr(3, (c.g[S1][0] as u32).wrapping_add(0x4));
            c.sdl(8, (c.g[S1][0] as u32).wrapping_add(0x13));
            c.sdr(8, (c.g[S1][0] as u32).wrapping_add(0xc));
            c.sdl(9, (c.g[S1][0] as u32).wrapping_add(0x1b));
            c.sdr(9, (c.g[S1][0] as u32).wrapping_add(0x14));
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x00417828, 0x002b1f2c)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x6fc));
            c.g[A1][0] = c.g[SP][0];
            c.g[A2][0] = c.g[S5][0];
            OC762_LOAD_SONG_DATA.call(c, 0x002b1f3c)?;
            let t = c.g[S5][0] == 0u64;
            c.g[A0][0] = c.g[S1][0];
            if t {
                break 'b_2b1f4c;
            } else {
                c.cycles += 2;
                c.g[A1][0] = 0x1;
                OC762_SONG_UPDATE.call(c, 0x002b1f4c)?;
                break 'b_2b1f4c;
            }
        }
        c.cycles += 10;
        c.g[V0][0] = c.g[S1][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x150));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x100));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x170));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateCharSelect::UpdateAttributeBars(self, rider, char)`
/// (0x181bd0): reads the seven human-rider stats (accel, edging, speed, spin,
/// stability, toughness, trick) from the BE stat interface, writes each into
/// its text field (`sprintf` + `cUIText_setAsciiString`, `"%.1f"` of
/// stat * 0.2) and progress bar (value 55 into `+0x80`, then `setValue`), then
/// the overall rating (sum rounded down to a multiple of 5, / 7) and sets the
/// human character on the rider manager.
pub fn cfe_state_char_select_update_attribute_bars(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 84;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x170));
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 24);
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.g[V1][0] = 0x460000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
        c.g[S5][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.g[S4][0] = sx(((c.g[A2][0] as u32) as i32 >> 24) as u32);
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.g[V0][0] = 0x4a0000;
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.g[S6][0] = 0u64;
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.g[S7][0] = 0u64;
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.sw((c.g[SP][0] as u32).wrapping_add(0xb0), c.g[A0][0]);
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.sd((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[RA][0]);
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2920));
        c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0x7));
        c.ldr(4, c.g[A3][0] as u32);
        c.g[A1][0] = c.lb((c.g[A3][0] as u32).wrapping_add(0x8));
        c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x7));
        c.sdr(4, c.g[SP][0] as u32);
        c.sb((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x14));
        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x13a8));
        c.lwl(8, (c.g[A3][0] as u32).wrapping_add(0x3));
        c.lwr(8, c.g[A3][0] as u32);
        c.swl(8, (c.g[SP][0] as u32).wrapping_add(0x13));
        c.swr(8, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A2][0] = 0x4;
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x00416210, 0x00181c64)?;
        c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x5a1c));
        call_ptr(c, 0x0014dc80, 0x00181c70)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x3;
        call_ptr(c, 0x0014dd58, 0x00181c7c)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S5][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S4][0];
        call_ptr(c, 0x00148bc8, 0x00181c90)?;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x90), c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S5][0];
        c.g[A2][0] = c.g[S4][0];
        call_ptr(c, 0x00148c68, 0x00181ca4)?;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x94), c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S5][0];
        c.g[A2][0] = c.g[S4][0];
        call_ptr(c, 0x00148b78, 0x00181cb8)?;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x98), c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S5][0];
        c.g[A2][0] = c.g[S4][0];
        call_ptr(c, 0x00148cb8, 0x00181ccc)?;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x9c), c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S5][0];
        c.g[A2][0] = c.g[S4][0];
        call_ptr(c, 0x00148d58, 0x00181ce0)?;
        c.sw((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S5][0];
        c.g[A2][0] = c.g[S4][0];
        call_ptr(c, 0x00148d08, 0x00181cf4)?;
        c.sw((c.g[SP][0] as u32).wrapping_add(0xa4), c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S5][0];
        c.g[A2][0] = c.g[S4][0];
        call_ptr(c, 0x00148c18, 0x00181d08)?;
        c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw((c.g[SP][0] as u32).wrapping_add(0xa8), c.g[V0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xb4), c.g[T0][0]);
        c.g[FP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        'l_181d20: loop {
            'b_181ddc: {
                c.cycles += 18;
                c.g[S0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x30));
                c.g[A0][0] = c.g[SP][0];
                c.sb((c.g[SP][0] as u32).wrapping_add(0x7), c.g[S0][0]);
                call_ptr(c, 0x00317670, 0x00181d30)?;
                c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xb0));
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x40));
                call_ptr(c, 0x0039d860, 0x00181d40)?;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xb4));
                c.g[S1][0] = c.g[V0][0];
                c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S0][0]);
                call_ptr(c, 0x00317670, 0x00181d50)?;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xb0));
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x40));
                call_ptr(c, 0x0039d860, 0x00181d60)?;
                let t = c.g[S1][0] == 0u64;
                c.g[S2][0] = c.g[V0][0];
                if t {
                    break 'b_181ddc;
                } else {
                    c.cycles += 2;
                    let t = c.g[S2][0] == 0u64;
                    c.g[A0][0] = 0xffffffffffffffff;
                    if t {
                        break 'b_181ddc;
                    } else {
                        c.cycles += 27;
                        c.g[V1][0] = c.lw(c.g[S3][0] as u32);
                        c.g[S0][0] = 0u64;
                        c.mtc1(3, 12);
                        c.cvt_s_w(12, 12);
                        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                        if c.g[V0][0] != 0 {
                            c.g[S0][0] = c.g[V1][0];
                        }
                        c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.mul_s(12, 12, 20);
                        call_ptr(c, 0x00413af8, 0x00181d94)?;
                        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[A1][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x1d40));
                        c.g[A2][0] = c.g[V0][0];
                        call_ptr(c, 0x00417828, 0x00181da4)?;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = 0x1;
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x00181dbc)?;
                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x003a0c50, 0x00181dc8)?;
                        c.g[A1][0] = 0x37;
                        c.g[A0][0] = c.g[S2][0];
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x80), c.g[A1][0]);
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, 0x0039c428, 0x00181ddc)?;
                        break 'b_181ddc;
                    }
                }
            }
            c.cycles += 4;
            c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = ((c.g[S6][0] as i64) < 7i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00181d20);
                }
                continue 'l_181d20;
            } else {
                'b_181e04: {
                    c.cycles += 6;
                    c.g[V0][0] = 0x5;
                    c.g[A0][0] = 0x460000;
                    c.div(0, 23, 2);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2910));
                    if c.g[V0][0] == 0u64 {
                        break 'b_181e04;
                    } else {
                        break 'b_181e04;
                    }
                }
                'b_181e90: {
                    c.cycles += 35;
                    c.g[S0][0] = 0x4a0000;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x13b8));
                    c.g[V1][0] = c.hi[0];
                    c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                    call_ptr(c, 0x00317670, 0x00181e18)?;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x40));
                    call_ptr(c, 0x0039d860, 0x00181e28)?;
                    c.mtc1(23, 0);
                    c.cvt_s_w(0, 0);
                    c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x5a18));
                    c.g[S1][0] = c.g[V0][0];
                    c.mul_s(12, 0, 12);
                    call_ptr(c, 0x00413af8, 0x00181e40)?;
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A1][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x1d40));
                    c.g[A0][0] = c.g[FP][0];
                    call_ptr(c, 0x00417828, 0x00181e50)?;
                    c.g[A1][0] = c.g[FP][0];
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x003a0c50, 0x00181e5c)?;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00317670, 0x00181e64)?;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00181e74)?;
                    c.g[A2][0] = 0x7;
                    c.g[V1][0] = 0x37;
                    c.div(0, 23, 6);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x80), c.g[V1][0]);
                    c.g[A0][0] = c.g[V0][0];
                    if c.g[A2][0] == 0u64 {
                        break 'b_181e90;
                    } else {
                        break 'b_181e90;
                    }
                }
                c.cycles += 23;
                c.g[A1][0] = c.lo[0];
                call_ptr(c, 0x0039c428, 0x00181e9c)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[A1][0] = c.g[S5][0];
                c.g[A2][0] = c.g[S4][0];
                c.g[A3][0] = 0u64;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
                call_ptr(c, 0x001a0508, 0x00181eb8)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x130));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0xe0));
                c.lq(30, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xc0));
                c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x160));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x170));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `void SndBankMan_LoadBank(self, i, id, a3)` (0x28bc58,
/// `cBankManager::BANK_SyncLoad`): when the bank `id` is not loaded yet on
/// bank slot `i` (`self->banks + i * 0x60`), cancels its pending queue entry
/// and loads it synchronously.
pub fn snd_bank_man_load_bank(c: &mut Cpu) {
    guest(c, |c| {
        'b_28bcc8: {
            c.cycles += 19;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.g[V0][0] = 0x60;
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S2][0] = c.g[A1][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.mult(0, 19, 18, 2);
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S0][0] = c.g[A0][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.g[A2][0];
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S4][0] = c.g[A3][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xacc));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S3][0] as u32));
            OC762_BANK_IS_LOADED.call(c, 0x0028bc9c)?;
            if c.g[V0][0] != 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                break 'b_28bcc8;
            } else {
                c.cycles += 9;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S2][0];
                OC762_CANCEL_BANK.call(c, 0x0028bcb0)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xacc));
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = c.g[S4][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S3][0] as u32));
                OC762_BANK_LOAD.call(c, 0x0028bcc4)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                break 'b_28bcc8;
            }
        }
        c.cycles += 7;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    });
}

/// `void Font_UploadTexture(Font* self, FontTexHdr* img)` (0x391a48,
/// `cFont::downloadTexture`): works out the power-of-two texture height,
/// stores the UV scale (`1/height`, `1/width`) in the font, builds a 4-bit
/// pixel buffer (`w * height / 2` bytes, copied from the header's shape data)
/// and a 16-entry grey-alpha palette, uploads both through the graphics
/// manager (`AddTexture1`) and frees the temporaries.
pub fn font_upload_texture(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S0][0] = c.g[A1][0];
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S1][0] = 0x1;
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S4][0] = c.g[A0][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
        c.g[S5][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
        c.g[V1][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x6));
        c.g[S1][0] = sx((c.g[S1][0] as u32) << 1);
        'l_391a80: loop {
            c.cycles += 7;
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S1][0] as i64)) as u64;
            if c.g[V0][0] == 0u64 {
                c.g[S1][0] = sx((c.g[S1][0] as u32) << 1);
                if event_due(c) {
                    deliver_event_here(c, 0x00391a80);
                }
                continue 'l_391a80;
            } else {
                'b_391b58: {
                    c.cycles += 45;
                    c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.mtc1(17, 1);
                    c.cvt_s_w(1, 1);
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 0);
                    c.g[A1][0] = 0x490000;
                    c.mtc1(2, 2);
                    c.cvt_s_w(2, 2);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2f40));
                    c.div_s(1, 0, 1);
                    c.g[A2][0] = 0x100;
                    c.g[A3][0] = 0u64;
                    c.div_s(0, 0, 2);
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x24));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x27));
                    c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sdl(2, (c.g[S4][0] as u32).wrapping_add(0x27));
                    c.sdr(2, (c.g[S4][0] as u32).wrapping_add(0x20));
                    c.g[A0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.mult(0, 4, 4, 17);
                    c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 1) as u32);
                    call_ptr(c, 0x00317e30, 0x00391af8)?;
                    c.g[A1][0] = 0x490000;
                    c.g[A3][0] = 0u64;
                    c.g[A2][0] = 0x100;
                    c.g[S3][0] = c.g[V0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2f50));
                    c.g[A0][0] = 0x40;
                    call_ptr(c, 0x00317e30, 0x00391b14)?;
                    c.g[A2][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[S2][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = 0u64;
                    c.mult(0, 6, 6, 17);
                    c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 1) as u32);
                    call_ptr(c, 0x003e6448, 0x00391b30)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = 0x40;
                    call_ptr(c, 0x003e6448, 0x00391b40)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = c.g[V0][0] & 0x1000;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                    if t {
                        break 'b_391b58;
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        break 'b_391b58;
                    }
                }
                c.cycles += 12;
                c.g[A2][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.g[S3][0];
                c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x6));
                c.mult(0, 6, 6, 2);
                c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 1) as u32);
                call_ptr(c, 0x003e6574, 0x00391b70)?;
                c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x64));
                c.g[T0][0] = 0xffffffffffffff80;
                c.g[A3][0] = 0xf;
                c.g[T1][0] = 0u64;
                c.g[A1][0] = 0u64;
                c.g[A0][0] = c.g[S2][0];
                'l_391b88: loop {
                    'b_391ba8: {
                        c.cycles += 8;
                        c.g[V0][0] = sx((c.g[A1][0] as u32) << 7);
                        c.sb(c.g[A0][0] as u32, c.g[T0][0]);
                        c.div(0, 2, 7);
                        c.sb((c.g[A0][0] as u32).wrapping_add(0x1), c.g[T0][0]);
                        c.sb((c.g[A0][0] as u32).wrapping_add(0x2), c.g[T0][0]);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                        if c.g[A3][0] == c.g[T1][0] {
                            break 'b_391ba8;
                        } else {
                            break 'b_391ba8;
                        }
                    }
                    c.cycles += 5;
                    c.g[V1][0] = ((c.g[A1][0] as i64) < 16i64) as u64;
                    c.g[V0][0] = c.lo[0];
                    c.sb((c.g[A0][0] as u32).wrapping_add(0x3), c.g[V0][0]);
                    let t = c.g[V1][0] != 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00391b88);
                        }
                        continue 'l_391b88;
                    } else {
                        'b_391c10: {
                            c.cycles += 19;
                            c.g[T3][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x10d8));
                            c.g[V0][0] = 0x1;
                            c.g[A3][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[V1][0] = 0xffffffffffffffff;
                            c.g[A0][0] = c.lh((c.g[T3][0] as u32).wrapping_add(0x178));
                            c.g[T0][0] = c.g[S1][0];
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                            c.g[A1][0] = c.g[S3][0];
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                            c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.sw(c.g[SP][0] as u32, c.g[S2][0]);
                            c.g[T1][0] = 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
                            c.g[T2][0] = 0xb;
                            c.g[V0][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x17c));
                            let t = c.g[V0][0] as u32;
                            c.g[T3][0] = 0x1;
                            call_ptr(c, t, 0x00391c00)?;
                            let t = c.g[S3][0] == 0u64;
                            c.sw((c.g[S4][0] as u32).wrapping_add(0x60), c.g[V0][0]);
                            if t {
                                break 'b_391c10;
                            } else {
                                c.cycles += 2;
                                c.g[A0][0] = c.g[S3][0];
                                call_ptr(c, 0x00317e98, 0x00391c10)?;
                                break 'b_391c10;
                            }
                        }
                        'b_391c24: {
                            c.cycles += 2;
                            let t = c.g[S2][0] == 0u64;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                            if t {
                                break 'b_391c24;
                            } else {
                                c.cycles += 3;
                                c.g[A0][0] = c.g[S2][0];
                                call_ptr(c, 0x00317e98, 0x00391c20)?;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                                break 'b_391c24;
                            }
                        }
                        c.cycles += 8;
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    });
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{fill, Rng, DATA, GAME_GP, SP as TEST_SP};
    use std::sync::atomic::AtomicUsize;

    /// Control word of this module's gated fakes (they only act when the
    /// machine's RAM holds the magic here).
    pub(crate) const OC762_CTRL: u32 = DATA + 0x20_0000;
    pub(crate) const OC762_MAGIC: u32 = 0x4f43_3736;
    /// Forced results: 16 words per callee (one per call of that callee).
    const OC762_FORCE: u32 = DATA + 0x30_0000;
    /// Call counters, one word per callee.
    const OC762_COUNT: u32 = DATA + 0x80_0000;

    fn oc762_id(addr: u32) -> u32 {
        (addr >> 2) & 0xffff
    }

    /// Make the `nth` call (from 0) of the callee at `addr` return `v` in `v0`
    /// (any value, 0 included; stored +3 so 0 means "not forced").
    pub(crate) fn oc762_force(c: &mut Cpu, addr: u32, nth: u32, v: u32) {
        c.bus.write32(
            OC762_FORCE + oc762_id(addr) * 0x40 + nth * 4,
            v.wrapping_add(3),
        );
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns
    /// the forced result of this call (if any) unless it unwound.
    fn oc762_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let id = oc762_id(addr);
        let nth = c.bus.read32(OC762_COUNT + id * 4);
        c.bus.write32(OC762_COUNT + id * 4, nth + 1);
        let mail = if nth < 16 {
            c.bus.read32(OC762_FORCE + id * 0x40 + nth * 4)
        } else {
            0
        };
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = mail.wrapping_sub(3) as u64;
        }
    }

    macro_rules! oc762_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC762_CTRL) != OC762_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc762_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc762_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc762_fakes!(
        (
            oc762_fake_00317e30,
            OC762_PREV_00317E30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
        (
            oc762_fake_00317670,
            OC762_PREV_00317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc762_fake_00317e98,
            OC762_PREV_00317E98,
            ee::s10::OVERRIDE_00317E98,
            ee::s10::f_00317e98__generated,
            0x0031_7e98
        ),
        (
            oc762_fake_00417828,
            OC762_PREV_00417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
        (
            oc762_fake_00416210,
            OC762_PREV_00416210,
            ee::s15::OVERRIDE_00416210,
            ee::s15::f_00416210__generated,
            0x0041_6210
        ),
        (
            oc762_fake_00413af8,
            OC762_PREV_00413AF8,
            ee::s15::OVERRIDE_00413AF8,
            ee::s15::f_00413af8__generated,
            0x0041_3af8
        ),
        (
            oc762_fake_0014dc80,
            OC762_PREV_0014DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc762_fake_0014dd58,
            OC762_PREV_0014DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc762_fake_00148bc8,
            OC762_PREV_00148BC8,
            ee::s01::OVERRIDE_00148BC8,
            ee::s01::f_00148bc8__generated,
            0x0014_8bc8
        ),
        (
            oc762_fake_00148c68,
            OC762_PREV_00148C68,
            ee::s01::OVERRIDE_00148C68,
            ee::s01::f_00148c68__generated,
            0x0014_8c68
        ),
        (
            oc762_fake_00148b78,
            OC762_PREV_00148B78,
            ee::s01::OVERRIDE_00148B78,
            ee::s01::f_00148b78__generated,
            0x0014_8b78
        ),
        (
            oc762_fake_00148cb8,
            OC762_PREV_00148CB8,
            ee::s01::OVERRIDE_00148CB8,
            ee::s01::f_00148cb8__generated,
            0x0014_8cb8
        ),
        (
            oc762_fake_00148d58,
            OC762_PREV_00148D58,
            ee::s01::OVERRIDE_00148D58,
            ee::s01::f_00148d58__generated,
            0x0014_8d58
        ),
        (
            oc762_fake_00148d08,
            OC762_PREV_00148D08,
            ee::s01::OVERRIDE_00148D08,
            ee::s01::f_00148d08__generated,
            0x0014_8d08
        ),
        (
            oc762_fake_00148c18,
            OC762_PREV_00148C18,
            ee::s01::OVERRIDE_00148C18,
            ee::s01::f_00148c18__generated,
            0x0014_8c18
        ),
        (
            oc762_fake_0039d860,
            OC762_PREV_0039D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc762_fake_0039c428,
            OC762_PREV_0039C428,
            ee::s13::OVERRIDE_0039C428,
            ee::s13::f_0039c428__generated,
            0x0039_c428
        ),
        (
            oc762_fake_003a0c70,
            OC762_PREV_003A0C70,
            ee::s13::OVERRIDE_003A0C70,
            ee::s13::f_003a0c70__generated,
            0x003a_0c70
        ),
        (
            oc762_fake_00398798,
            OC762_PREV_00398798,
            ee::s13::OVERRIDE_00398798,
            ee::s13::f_00398798__generated,
            0x0039_8798
        ),
        (
            oc762_fake_001a0508,
            OC762_PREV_001A0508,
            ee::s03::OVERRIDE_001A0508,
            ee::s03::f_001a0508__generated,
            0x001a_0508
        ),
        (
            oc762_fake_003e6448,
            OC762_PREV_003E6448,
            ee::s14::OVERRIDE_003E6448,
            ee::s14::f_003e6448__generated,
            0x003e_6448
        ),
        (
            oc762_fake_003e6574,
            OC762_PREV_003E6574,
            ee::s14::OVERRIDE_003E6574,
            ee::s14::f_003e6574__generated,
            0x003e_6574
        ),
        (
            oc762_fake_002b4a20,
            OC762_PREV_002B4A20,
            ee::s08::OVERRIDE_002B4A20,
            ee::s08::f_002b4a20__generated,
            0x002b_4a20
        ),
        (
            oc762_fake_002b21e0,
            OC762_PREV_002B21E0,
            ee::s08::OVERRIDE_002B21E0,
            ee::s08::f_002b21e0__generated,
            0x002b_21e0
        ),
        (
            oc762_fake_002c03a8,
            OC762_PREV_002C03A8,
            ee::s08::OVERRIDE_002C03A8,
            ee::s08::f_002c03a8__generated,
            0x002c_03a8
        ),
        (
            oc762_fake_0028b830,
            OC762_PREV_0028B830,
            ee::s07::OVERRIDE_0028B830,
            ee::s07::f_0028b830__generated,
            0x0028_b830
        ),
        (
            oc762_fake_0028a728,
            OC762_PREV_0028A728,
            ee::s07::OVERRIDE_0028A728,
            ee::s07::f_0028a728__generated,
            0x0028_a728
        ),
        (
            oc762_fake_0028b320,
            OC762_PREV_0028B320,
            ee::s07::OVERRIDE_0028B320,
            ee::s07::f_0028b320__generated,
            0x0028_b320
        ),
    );

    /// Fixture objects, clear of the fake log and of the pointers the fakes
    /// return (DATA..DATA+0x40000).
    const FX_SELF: u32 = DATA + 0x21_0000;
    const FX_TRACK: u32 = DATA + 0x21_1000;
    const FX_TABLE: u32 = DATA + 0x21_2000;
    const FX_VOL: u32 = DATA + 0x21_3000;
    const FX_CFG: u32 = DATA + 0x21_4000;
    const FX_NAME: u32 = DATA + 0x21_5000;

    /// Start a test machine: scrambled registers, fakes armed.
    fn oc762_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC762_CTRL, OC762_MAGIC);
        r
    }

    /// `WorldPainterQuery_ResetAll`: the one forwarded call, answered or unwound.
    #[test]
    fn world_painter_query_reset_all_matches() {
        oc762_install_fakes();
        covered(
            ssx3_ee::s08::f_002c03e8__generated,
            world_painter_query_reset_all,
            100,
            &|c, seed| {
                oc762_machine(c, seed, 0x002c_03e8, 1);
            },
        );
    }

    /// `UIScreen_HandleInput`: forwards `self + 0x40` to `cUIGroup_HandleInput`.
    #[test]
    fn ui_screen_handle_input_matches() {
        oc762_install_fakes();
        covered(
            ssx3_ee::s13::f_0039d758__generated,
            ui_screen_handle_input,
            100,
            &|c, seed| {
                oc762_machine(c, seed, 0x0039_d758, 1);
            },
        );
    }

    /// `SndBankMan_LoadBank`: the bank is already loaded (forced `v0` nonzero)
    /// or not (0, so the cancel and the load calls follow); fakes unwind at
    /// any of the three calls.
    #[test]
    fn snd_bank_man_load_bank_matches() {
        oc762_install_fakes();
        covered(
            ssx3_ee::s07::f_0028bc58__generated,
            snd_bank_man_load_bank,
            200,
            &|c, seed| {
                oc762_machine(c, seed, 0x0028_bc58, 3);
                oc762_force(c, 0x0028_b830, 0, if seed % 2 == 0 { 0 } else { 1 });
                c.g[A0][0] = sx(FX_SELF);
                c.bus
                    .write32(FX_SELF + 0xacc, DATA + 0x21_6000 + (seed % 5) * 0x10);
            },
        );
    }

    /// `SongInstance_ctor`: the owner's gain, the gp float and the stored
    /// volume sweep every branch of the clamp chain (below 0, in range, 127
    /// and above, and the 0x7f path of the byte store); owner flag +0xa4 is 1
    /// or not (picks the music file name); the stack argument (the first
    /// load request) is 0 or 1 (skips or runs `SongInstance_Update`).
    #[test]
    fn song_instance_ctor_matches() {
        oc762_install_fakes();
        covered(
            ssx3_ee::s08::f_002b1c50__generated,
            song_instance_ctor,
            400,
            &|c, seed| {
                let mut r = oc762_machine(c, seed, 0x002b_1c50, 7);
                fill(c, &mut r, FX_SELF, 0x40);
                let idx = seed % 3;
                c.bus.write32(FX_TRACK + 0x1c, idx);
                c.bus.write32(FX_TRACK + 0x20, FX_TABLE);
                c.bus.write32(FX_TRACK + 0x38, FX_VOL);
                fill(c, &mut r, FX_TABLE, 0x20);
                let vols = [0i32, -50, 50, 100, 126, 127, 200, 1];
                let vol = vols[((seed / 3) % 8) as usize];
                c.bus.write32(FX_VOL + 4 * idx, vol as u32);
                let cfgs = [1i32, 0, -3, 2, 5, 0x1000];
                c.bus
                    .write32(FX_CFG + 0x88, cfgs[((seed / 24) % 6) as usize] as u32);
                c.bus
                    .write32(FX_CFG + 0xa4, if seed % 4 == 0 { 1 } else { 2 });
                let gains = [
                    0x3f80_0000u32,
                    0x3f00_0000,
                    0,
                    0x4020_0000,
                    0xbf80_0000,
                    0x3c23_d70a,
                ];
                c.bus
                    .write32(GAME_GP - 0x4554, gains[((seed / 7) % 6) as usize]);
                c.bus.write32(TEST_SP, if seed % 5 == 0 { 0 } else { 1 });
                c.g[A0][0] = sx(FX_SELF);
                c.g[A1][0] = sx(FX_NAME);
                c.g[A3][0] = sx(FX_CFG);
                c.g[T0][0] = sx(FX_TRACK);
            },
        );
    }

    /// `Font_UploadTexture`: image heights 1..200 (so 1 to 8 doublings),
    /// widths 2..64, with and without the 0x1000 "offset" flag; the two
    /// buffer allocations return a pointer or 0 (forced), skipping the
    /// matching `free`; the graphics-manager virtual call and the memory
    /// helpers are fakes, some of which unwind.
    #[test]
    fn font_upload_texture_matches() {
        oc762_install_fakes();
        covered(
            ssx3_ee::s12::f_00391a48__generated,
            font_upload_texture,
            300,
            &|c, seed| {
                let mut r = oc762_machine(c, seed, 0x0039_1a48, 7);
                fill(c, &mut r, FX_SELF, 0x40);
                let h = [1u32, 2, 3, 16, 31, 64, 100, 200, 0xffff][(seed % 9) as usize];
                let w = 2 + 2 * (r.next_u32() % 32);
                c.bus.write16(FX_NAME + 4, w as u16);
                c.bus.write16(FX_NAME + 6, h as u16);
                c.bus.write32(
                    FX_NAME + 0xc,
                    if seed % 2 == 0 {
                        0x1000
                    } else {
                        r.next_u32() & !0x1000
                    },
                );
                c.bus.write32(FX_NAME + 0x10, 0x20 + (r.next_u32() & 0x3c));
                if seed % 7 == 3 {
                    oc762_force(c, 0x0031_7e30, 0, 0);
                }
                if seed % 5 == 1 {
                    oc762_force(c, 0x0031_7e30, 1, 0);
                }
                c.g[A0][0] = sx(FX_SELF);
                c.g[A1][0] = sx(FX_NAME);
            },
        );
    }

    /// `cFEStateCharSelect_UpdateAttributeBars`: stats from the (forced)
    /// accessors (negative, zero, small, large); per iteration the two UI
    /// lookups answer zero or a pointer (forced), so the text/bar update is
    /// skipped or done; fakes unwind at any call.
    #[test]
    fn cfe_state_char_select_update_attribute_bars_matches() {
        oc762_install_fakes();
        covered(
            ssx3_ee::s02::f_00181bd0__generated,
            cfe_state_char_select_update_attribute_bars,
            300,
            &|c, seed| {
                let mut r = oc762_machine(c, seed, 0x0018_1bd0, 30);
                fill(c, &mut r, FX_SELF, 0x40);
                let stats = [-5i32, 0, 7, 40, 99, 130];
                for (k, a) in [
                    0x0014_8bc8u32,
                    0x0014_8c68,
                    0x0014_8b78,
                    0x0014_8cb8,
                    0x0014_8d58,
                    0x0014_8d08,
                    0x0014_8c18,
                ]
                .into_iter()
                .enumerate()
                {
                    let v = stats[((seed as usize) / 2 + k * 3) % 6];
                    oc762_force(c, a, 0, v as u32);
                }
                for nth in 0..14u32 {
                    if r.next_u32() % 4 == 0 {
                        oc762_force(c, 0x0039_d860, nth, 0);
                    }
                }
                c.g[A0][0] = sx(FX_SELF);
            },
        );
    }
}
