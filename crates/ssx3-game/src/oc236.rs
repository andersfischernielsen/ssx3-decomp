//! Staging wave 204 (oc236): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

use crate::abi;
use crate::guest::Addr;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003a_9258, c_world_cache_streamer_update, "cWorldCache_StreamerUpdate"),
    (0x003f_fa58, sce_pad_read, "scePadRead"),
    (0x0010_3aa0, rider_anim_fire_sound_events, "RiderAnim_FireSoundEvents"),
    (0x0041_3068, unpack_f, "__unpack_f"),
    (0x001a_39f0, fe_async_manager_update, "FEAsyncManager_Update"),
    (0x003e_5928, ea_timer_service_callbacks, "EATimer_ServiceCallbacks"),
    (0x002b_b100, t_wpigd_lighting_pair_differs, "tWPIGD_Lighting_PairDiffers"),
    (0x0026_ce50, playback_man_read_input_frame, "PlaybackMan_ReadInputFrame"),
    (0x003f_f9a8, libpad_get_dma_buffer, "libpad_GetDmaBuffer"),
];

/// `sx(a + b)` on the low words (`addu`).
#[inline(always)]
fn add32(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_add(b as u32))
}

// ---------------------------------------------------------------------------
// cWorldCache streamer.

/// The streamer (`a0`): `+0` the world cache, `+4` the course info (`+8` its
/// section count), `+8` the section descriptors (0x58 bytes: name, `+0x18`
/// hull id, `+0x1c` node tree), `+0x14` one entry per section (8 bytes: state,
/// load priority), `+0x2e0`/`+0x2e4` the resident and fading counts.
mod streamer {
    pub const CACHE: u32 = 0x0;
    pub const INFO: u32 = 0x4;
    pub const SECTIONS: u32 = 0x8;
    pub const ENTRIES: u32 = 0x14;
    pub const RESIDENT: u32 = 0x2e0;
    pub const FADING: u32 = 0x2e4;
    /// `info + 8`: number of sections.
    pub const INFO_COUNT: u32 = 0x8;
    pub const SECTION_SIZE: u64 = 0x58;
    pub const SECTION_ID: u32 = 0x18;
    pub const SECTION_TREE: u32 = 0x1c;
    /// The sky section name prefixes `"ASKY"`..`"ESKY"` (`.rodata`, 8 apart).
    pub const SKY_NAMES: u32 = 0x004a_47e8;
    /// The state jump table (seven entries) in `.rodata`.
    pub const STATE_TABLE: u32 = 0x0049_4ef0;
}

const STRNCMP: u32 = 0x0041_6b18;
const CREATE_SECTION_TABLE: Callee = Callee { addr: 0x003a_81a8, f: ee::s13::f_003a81a8 };
const REQUEST_HULL: Callee = Callee { addr: 0x003a_8448, f: ee::s13::f_003a8448 };
const SET_HULL_PINNED: Callee = Callee { addr: 0x003a_84f8, f: ee::s13::f_003a84f8 };
const IS_HULL_RESIDENT: Callee = Callee { addr: 0x003a_8618, f: ee::s13::f_003a8618 };
const IS_HULL_FREE: Callee = Callee { addr: 0x003a_8638, f: ee::s13::f_003a8638 };
const ARE_NODE_HULLS_FREE: Callee = Callee { addr: 0x003a_95c0, f: ee::s13::f_003a95c0 };
const FREE_SECTION_TABLE: Callee = Callee { addr: 0x003a_8230, f: ee::s13::f_003a8230 };
const WORLD_RESET_LIGHTS: Callee = Callee { addr: 0x003a_d1a8, f: ee::s13::f_003ad1a8 };
const EXPIRE_HULL: Callee = Callee { addr: 0x003a_84c8, f: ee::s13::f_003a84c8 };
const RELEASE_NODE_HULLS: Callee = Callee { addr: 0x003a_9558, f: ee::s13::f_003a9558 };
const STREAMER_UPDATE_VIEW: Callee = Callee { addr: 0x003a_9d60, f: ee::s13::f_003a9d60 };

/// `void cWorldCache_StreamerUpdate(Streamer* self)` (0x3a9258): step every
/// section's streaming state machine once per frame, then update both views.
///
/// States: 0 idle; 1 load requested (create the section table, sky sections
/// with kind 0, others 2; request and pin its hull) -> 2; 2 loading (hull
/// resident: tell the app, vtable `+0x330`) -> 5; 3 unload requested (expire
/// the hull and the node tree's hulls) -> 4; 4 unloading (hull and all node
/// hulls free: free the table, reset lights) -> 0; 5 resident and 6 fading
/// are counted.
pub fn c_world_cache_streamer_update(c: &mut Cpu) {
    use streamer::*;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S0, 0x60);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save_ra(c, 0x10);
        let this = r32(c, S1);
        c.g[V0][0] = Addr(this + INFO).lw(c);
        frame.restore(c, S0, 0x60);
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            Addr(this + RESIDENT).sw(c, 0);
            Addr(this + FADING).sw(c, 0);
            c.g[V0][0] = Addr(r32(c, V0) + INFO_COUNT).lw(c);
            c.g[S2][0] = 0;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[V1][0] = sx(this + ENTRIES);
                loop {
                    // Head 0x3a92a0: s3 = i * 8, s4 = entries, v1 = state.
                    c.cycles += 8;
                    c.g[V0][0] = sx(r32(c, S2) << 3);
                    c.g[S4][0] = c.g[V1][0];
                    c.g[S3][0] = c.g[V0][0];
                    c.g[V1][0] = add32(c.g[V1][0], c.g[V0][0]);
                    c.g[V1][0] = Addr(r32(c, V1)).lw(c);
                    let in_table = c.g[V1][0] < 7;
                    c.g[V0][0] = 0x49_0000;
                    let entry = add32(c.g[S4][0], c.g[S3][0]) as u32;
                    // v1 at the tail: the info pointer, or set by the case.
                    let mut tail_loads_info = true;
                    if in_table {
                        c.cycles += 6;
                        c.g[V1][0] = sx(r32(c, V1) << 2);
                        c.g[V0][0] = sx(STATE_TABLE);
                        c.g[V1][0] = add32(c.g[V1][0], c.g[V0][0]);
                        c.g[A0][0] = Addr(r32(c, V1)).lw(c);
                        match r32(c, A0) {
                            0x003a_92d8 => {
                                // 1: load requested.
                                c.cycles += 10;
                                c.g[V0][0] = 0x58;
                                c.g[A0][0] = Addr(this + SECTIONS).lw(c);
                                c.mult(0, S0, S2, V0);
                                let mut sky = false;
                                for k in 0..5u32 {
                                    if k == 0 {
                                        c.g[A1][0] = sx(SKY_NAMES);
                                    } else {
                                        c.cycles += 7;
                                        c.g[A0][0] = Addr(this + SECTIONS).lw(c);
                                        c.g[A1][0] = sx(SKY_NAMES + 8 * k);
                                    }
                                    c.g[A2][0] = 4;
                                    c.g[A0][0] = add32(c.g[A0][0], c.g[S0][0]);
                                    let ret = [0x003a_92f8, 0x003a_9314, 0x003a_9330, 0x003a_934c, 0x003a_9368][k as usize];
                                    call_ptr(c, STRNCMP, ret)?;
                                    if k < 4 {
                                        c.g[A1][0] = 0x4a_0000;
                                    }
                                    if c.g[V0][0] == 0 {
                                        sky = true;
                                        break;
                                    }
                                }
                                if sky {
                                    c.cycles += 6;
                                    c.g[A0][0] = Addr(this + CACHE).lw(c);
                                    c.g[A1][0] = c.g[S2][0];
                                    c.g[A2][0] = 0;
                                    CREATE_SECTION_TABLE.call(c, 0x003a_9380)?;
                                } else {
                                    c.g[A0][0] = Addr(this + CACHE).lw(c);
                                    c.cycles += 4;
                                    c.g[A1][0] = c.g[S2][0];
                                    c.g[A2][0] = 2;
                                    CREATE_SECTION_TABLE.call(c, 0x003a_9394)?;
                                }
                                c.g[S0][0] = SECTION_SIZE;
                                c.cycles += 17;
                                c.g[V1][0] = Addr(this + SECTIONS).lw(c);
                                c.mult(0, S0, S2, S0);
                                c.g[V0][0] = add32(c.g[S1][0], c.g[S3][0]);
                                c.lwc1(12, (c.g[V0][0] as u32).wrapping_add(0x18));
                                c.g[A0][0] = Addr(this + CACHE).lw(c);
                                c.g[V1][0] = add32(c.g[S0][0], c.g[V1][0]);
                                c.g[A1][0] = Addr(r32(c, V1) + SECTION_ID).lw(c);
                                REQUEST_HULL.call(c, 0x003a_93b8)?;
                                c.g[V0][0] = Addr(this + SECTIONS).lw(c);
                                c.g[A2][0] = 1;
                                c.g[A0][0] = Addr(this + CACHE).lw(c);
                                c.g[S0][0] = add32(c.g[S0][0], c.g[V0][0]);
                                c.g[A1][0] = Addr(r32(c, S0) + SECTION_ID).lw(c);
                                SET_HULL_PINNED.call(c, 0x003a_93d0)?;
                                c.g[V1][0] = sx(entry);
                                c.g[V0][0] = 2;
                                c.cycles += 1;
                                Addr(entry).sw(c, 2);
                            }
                            0x003a_93dc => {
                                // 2: loading.
                                c.cycles += 9;
                                c.g[V0][0] = 0x58;
                                c.g[V1][0] = Addr(this + SECTIONS).lw(c);
                                c.mult(0, V0, S2, V0);
                                c.g[A0][0] = Addr(this + CACHE).lw(c);
                                c.g[V0][0] = add32(c.g[V0][0], c.g[V1][0]);
                                c.g[A1][0] = Addr(r32(c, V0) + SECTION_ID).lw(c);
                                IS_HULL_RESIDENT.call(c, 0x003a_93f8)?;
                                let resident = c.g[V0][0] != 0;
                                c.g[V1][0] = sx(entry);
                                if resident {
                                    c.cycles += 11;
                                    c.g[V0][0] = 5;
                                    Addr(entry).sw(c, 5);
                                    c.g[A1][0] = c.g[S2][0];
                                    c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2a90)).lw(c);
                                    c.g[V0][0] = Addr(r32(c, A2) + 0x10d8).lw(c);
                                    c.g[A0][0] = Addr(r32(c, V0) + 0x330).lh(c);
                                    c.g[V1][0] = Addr(r32(c, V0) + 0x334).lw(c);
                                    let target = r32(c, V1);
                                    c.g[A0][0] = add32(c.g[A2][0], c.g[A0][0]);
                                    call_ptr(c, target, 0x003a_9424)?;
                                    c.g[V1][0] = Addr(this + INFO).lw(c);
                                    tail_loads_info = false;
                                }
                            }
                            0x003a_942c => {
                                // 4: unloading.
                                c.cycles += 9;
                                c.g[V1][0] = 0x58;
                                c.g[V0][0] = Addr(this + SECTIONS).lw(c);
                                c.mult(0, S0, S2, V1);
                                c.g[A0][0] = Addr(this + CACHE).lw(c);
                                c.g[V0][0] = add32(c.g[S0][0], c.g[V0][0]);
                                c.g[A1][0] = Addr(r32(c, V0) + SECTION_ID).lw(c);
                                IS_HULL_FREE.call(c, 0x003a_9448)?;
                                let free = c.g[V0][0] != 0;
                                c.g[V1][0] = 1;
                                if free {
                                    c.cycles += 10;
                                    c.g[V0][0] = Addr(this + SECTIONS).lw(c);
                                    let ok = sp(c);
                                    Addr(ok).sw(c, 1);
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[V0][0] = add32(c.g[S0][0], c.g[V0][0]);
                                    c.g[A2][0] = c.g[SP][0];
                                    c.g[A1][0] = Addr(r32(c, V0) + SECTION_TREE).lw(c);
                                    ARE_NODE_HULLS_FREE.call(c, 0x003a_946c)?;
                                    c.g[V0][0] = Addr(sp(c)).lw(c);
                                    let all_free = c.g[V0][0] != 0;
                                    c.g[V0][0] = sx(entry);
                                    if all_free {
                                        c.cycles += 8;
                                        c.g[A1][0] = c.g[S2][0];
                                        Addr(entry).sw(c, 0);
                                        c.g[A0][0] = Addr(this + CACHE).lw(c);
                                        FREE_SECTION_TABLE.call(c, 0x003a_9488)?;
                                        WORLD_RESET_LIGHTS.call(c, 0x003a_9490)?;
                                        c.g[V1][0] = Addr(this + INFO).lw(c);
                                        tail_loads_info = false;
                                    }
                                }
                            }
                            0x003a_9498 => {
                                // 5: resident.
                                c.cycles += 4;
                                c.g[V0][0] = add32(Addr(this + RESIDENT).lw(c), 1);
                                Addr(this + RESIDENT).sw(c, c.g[V0][0]);
                            }
                            0x003a_94a8 => {
                                // 6: fading (also counted as resident).
                                c.cycles += 7;
                                c.g[V0][0] = add32(Addr(this + RESIDENT).lw(c), 1);
                                c.g[V1][0] = add32(Addr(this + FADING).lw(c), 1);
                                Addr(this + RESIDENT).sw(c, c.g[V0][0]);
                                Addr(this + FADING).sw(c, c.g[V1][0]);
                            }
                            0x003a_94c4 => {
                                // 3: unload requested.
                                c.cycles += 14;
                                c.g[S0][0] = SECTION_SIZE;
                                c.g[V0][0] = Addr(this + SECTIONS).lw(c);
                                c.mult(0, S0, S2, S0);
                                c.g[A0][0] = Addr(this + CACHE).lw(c);
                                c.g[V0][0] = add32(c.g[S0][0], c.g[V0][0]);
                                c.g[A1][0] = Addr(r32(c, V0) + SECTION_ID).lw(c);
                                EXPIRE_HULL.call(c, 0x003a_94e0)?;
                                c.g[V0][0] = Addr(this + SECTIONS).lw(c);
                                c.g[A0][0] = c.g[S1][0];
                                c.g[S0][0] = add32(c.g[S0][0], c.g[V0][0]);
                                c.g[A1][0] = Addr(r32(c, S0) + SECTION_TREE).lw(c);
                                RELEASE_NODE_HULLS.call(c, 0x003a_94f4)?;
                                c.g[V1][0] = sx(entry);
                                c.g[V0][0] = 4;
                                c.cycles += 1;
                                Addr(entry).sw(c, 4);
                            }
                            0x003a_9500 => {
                                // 0: idle.
                            }
                            t => {
                                c.jump_out(t, 0x003a_9258, 0x003a_9558);
                                return Err(crate::call::Unwound);
                            }
                        }
                    }
                    if tail_loads_info {
                        c.cycles += 1;
                        c.g[V1][0] = Addr(this + INFO).lw(c);
                    }
                    // Tail 0x3a9504.
                    c.cycles += 5;
                    c.g[S2][0] = add32(c.g[S2][0], 1);
                    c.g[V0][0] = Addr(r32(c, V1) + INFO_COUNT).lw(c);
                    c.g[V0][0] = (c.g[S2][0] < c.g[V0][0]) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[V1][0] = sx(this + ENTRIES);
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003a_92a0);
                    }
                }
            }
            c.cycles += 2;
            c.g[S0][0] = 0;
            c.g[A1][0] = 0;
            loop {
                c.cycles += 6;
                c.g[A0][0] = c.g[S1][0];
                STREAMER_UPDATE_VIEW.call(c, 0x003a_9528)?;
                c.g[S0][0] = add32(c.g[S0][0], 1);
                c.g[V0][0] = ((c.g[S0][0] as i64) < 2) as u64;
                c.g[A1][0] = c.g[S0][0];
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003a_9520);
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x60);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// libpad.

/// libpad's port table: 0x70 bytes per port, 0x1c per slot; `+0` the DMA
/// buffer (two 0x80-byte halves), `+0x10` non-zero while the port is open.
const PAD_PORTS: u32 = 0x0052_6250;
const PAD_PORT_SIZE: u64 = 0x70;
const PAD_SLOT_SIZE: u64 = 0x1c;
const PAD_OPEN: u32 = 0x10;
/// Offset of the data length word in a pad DMA buffer half.
const PAD_DATA_LEN: u32 = 0x60;

const D_INTR: Callee = Callee { addr: 0x0042_c078, f: ee::s15::f_0042c078 };
const E_INTR: Callee = Callee { addr: 0x0042_c0c0, f: ee::s15::f_0042c0c0 };
const MEMCPY: Callee = Callee { addr: 0x0041_605c, f: ee::s15::f_0041605c };
const SYNC_DCACHE: Callee = Callee { addr: 0x0042_4698, f: ee::s15::f_00424698 };
const LIBPAD_GET_DMA_BUFFER: Callee = Callee { addr: 0x003f_f9a8, f: ee::s15::f_003ff9a8 };

/// `int scePadRead(int port, int slot, u8* data)` (0x3ffa58): 0 when the port
/// is closed; otherwise, with interrupts off, copy the current 0x80-byte DMA
/// half to the stack, copy its first `len` (`+0x60`) bytes to `data`, turn
/// interrupts back on if they were, and return `len`.
pub fn sce_pad_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xe0);
        c.g[V1][0] = PAD_PORT_SIZE;
        c.sd(frame.at(c, 0x90), c.g[S1][0]);
        c.g[V0][0] = 0x52_0000;
        c.sd(frame.at(c, 0xa0), c.g[S2][0]);
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        c.g[A0][0] = PAD_SLOT_SIZE;
        c.mult(1, V1, S1, V1);
        c.mult(0, A0, S2, A0);
        c.sd(frame.at(c, 0xc0), c.g[S4][0]);
        c.g[V0][0] = sx(PAD_PORTS);
        frame.save_ra(c, 0xd0);
        c.sd(frame.at(c, 0xb0), c.g[S3][0]);
        c.g[A0][0] = add32(c.g[A0][0], c.g[V1][0]);
        c.sd(frame.at(c, 0x80), c.g[S0][0]);
        c.g[V0][0] = add32(c.g[V0][0], c.g[A0][0]);
        c.g[V1][0] = Addr(r32(c, V0) + PAD_OPEN).lw(c);
        c.g[S4][0] = c.g[A2][0];
        if c.g[V1][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 11;
            D_INTR.call(c, 0x003f_fab8)?;
            c.g[S3][0] = c.g[V0][0];
            c.g[S0][0] = c.g[SP][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S2][0];
            LIBPAD_GET_DMA_BUFFER.call(c, 0x003f_facc)?;
            // Copy the 0x80-byte half to the stack, 32 bytes at a time.
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] = c.g[V1][0] & 7;
            let aligned = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[SP][0];
            c.cycles += 1;
            c.g[V0][0] = add32(c.g[V1][0], 0x80);
            if aligned {
                loop {
                    c.cycles += 13;
                    let (src, dst) = (r32(c, V1), r32(c, A0));
                    for (k, reg) in [A1, A2, A3, T0].into_iter().enumerate() {
                        c.g[reg][0] = c.ld(src + 8 * k as u32);
                    }
                    for (k, reg) in [A1, A2, A3, T0].into_iter().enumerate() {
                        c.sd(dst + 8 * k as u32, c.g[reg][0]);
                    }
                    c.g[V1][0] = add32(c.g[V1][0], 0x20);
                    c.g[A0][0] = add32(c.g[A0][0], 0x20);
                    if c.g[V1][0] == c.g[V0][0] {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003f_fb40);
                    }
                }
                c.cycles += 1;
            } else {
                loop {
                    c.cycles += 21;
                    let (src, dst) = (r32(c, V1), r32(c, A0));
                    for (k, reg) in [A1, A2, A3, T0].into_iter().enumerate() {
                        let at = src + 8 * k as u32;
                        c.ldl(reg, at + 7);
                        c.ldr(reg, at);
                    }
                    for (k, reg) in [A1, A2, A3, T0].into_iter().enumerate() {
                        let at = dst + 8 * k as u32;
                        c.sdl(reg, at + 7);
                        c.sdr(reg, at);
                    }
                    c.g[V1][0] = add32(c.g[V1][0], 0x20);
                    c.g[A0][0] = add32(c.g[A0][0], 0x20);
                    if c.g[V1][0] == c.g[V0][0] {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003f_fae0);
                    }
                }
                c.cycles += 2;
            }
            let buf = r32(c, S0);
            c.g[A2][0] = Addr(buf + PAD_DATA_LEN).lw(c);
            c.cycles += 6;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[S0][0];
            MEMCPY.call(c, 0x003f_fb84)?;
            c.g[V1][0] = 1;
            if c.g[S3][0] == c.g[V1][0] {
                c.cycles += 3;
                E_INTR.call(c, 0x003f_fb98)?;
            }
            c.g[V0][0] = Addr(buf + PAD_DATA_LEN).lw(c);
        }
        c.cycles += 8;
        frame.restore_ra(c, 0xd0);
        c.g[S4][0] = c.ld(frame.at(c, 0xc0));
        c.g[S3][0] = c.ld(frame.at(c, 0xb0));
        c.g[S2][0] = c.ld(frame.at(c, 0xa0));
        c.g[S1][0] = c.ld(frame.at(c, 0x90));
        c.g[S0][0] = c.ld(frame.at(c, 0x80));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider animation events.

/// `AnimBankSet* g_animBanks` (`gp+0xd08`): bank pointers by `id & 0xff`. A
/// bank: `+8` its sequences (0x14 bytes; `+0x10` first event, `+0x12` event
/// count), `+0x10` its events (4 bytes; `+2` the event code, bit 15 = sound).
const ANIM_BANKS: u32 = 0xd08;
/// The animation object (`a1`): `+4` its sequence id (bank | seq << 8),
/// `+0xb0` the per-event flags (two 64-bit masks: reached, and armed).
const ANIM_SEQ_ID: u32 = 0x4;
const ANIM_EVENT_FLAGS: u32 = 0xb0;
const SEQ_SIZE: u64 = 0x14;

const FLAGS64_CLEAR: u32 = 0x0014_4670;
const FLAGS64_TEST: u32 = 0x0014_46a0;
const FLAGS64_TEST_SECOND: u32 = 0x0014_46b8;
const RIDER_ANIM_PLAY_EVENT_SOUND: Callee = Callee { addr: 0x0010_4cc8, f: ee::s00::f_00104cc8 };

/// `void RiderAnim_FireSoundEvents(RiderAnim* self, Anim* anim)` (0x103aa0):
/// for each event of the animation's current sequence that is set in both
/// event masks and whose code has the sound bit, clear its flag and play the
/// event sound (`code & 0x7fff`).
pub fn rider_anim_fire_sound_events(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x90);
        c.g[A2][0] = SEQ_SIZE;
        frame.save(c, S1, 0x70);
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S0, 0x80);
        c.g[S5][0] = c.g[A0][0];
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        frame.save(c, S6, 0x20);
        frame.save(c, S7, 0x10);
        frame.save_ra(c, 0);
        let anim = r32(c, S4);
        let gp = c.g[GP][0] as u32;
        // The sequence's event count.
        c.g[A1][0] = Addr(gp + ANIM_BANKS).lw(c);
        c.g[V0][0] = Addr(anim + ANIM_SEQ_ID).lw(c);
        c.g[V1][0] = sx(((c.g[V0][0] & 0xff) as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 8);
        c.g[A1][0] = add32(c.g[A1][0], c.g[V1][0]);
        c.mult(0, V0, V0, A2);
        c.g[A0][0] = Addr(r32(c, A1)).lw(c);
        c.g[V1][0] = Addr(r32(c, A0) + 8).lw(c);
        c.g[V0][0] = add32(c.g[V0][0], c.g[V1][0]);
        c.g[S3][0] = Addr(r32(c, V0) + 0x12).lhu(c);
        c.g[S1][0] = 0;
        if c.g[S3][0] != 0 {
            c.cycles += 4;
            c.g[S2][0] = sx(anim + ANIM_EVENT_FLAGS);
            c.g[S7][0] = 1;
            c.g[S6][0] = SEQ_SIZE;
            c.g[S0][0] = 0;
            loop {
                // Head 0x103b18: s1 = event index, s0 = 0.
                c.cycles += 5;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, FLAGS64_TEST, 0x0010_3b24)?;
                let reached = c.g[V0][0] != 0;
                c.g[A0][0] = c.g[S2][0];
                if reached {
                    c.cycles += 3;
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, FLAGS64_TEST_SECOND, 0x0010_3b34)?;
                    if c.g[V0][0] != 0 {
                        c.g[S0][0] = c.g[S7][0];
                    }
                }
                c.cycles += 2;
                let fire = c.g[S0][0] != 0;
                c.g[A1][0] = Addr(gp + ANIM_BANKS).lw(c);
                let mut played = !fire;
                if fire {
                    // The event's code.
                    c.cycles += 18;
                    c.g[V1][0] = Addr(anim + ANIM_SEQ_ID).lw(c);
                    c.g[V0][0] = sx(((c.g[V1][0] & 0xff) as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) >> 8);
                    c.g[A1][0] = add32(c.g[A1][0], c.g[V0][0]);
                    c.mult(0, V1, V1, S6);
                    c.g[A0][0] = Addr(r32(c, A1)).lw(c);
                    c.g[A2][0] = Addr(r32(c, A0) + 8).lw(c);
                    c.g[A1][0] = Addr(r32(c, A0) + 0x10).lw(c);
                    c.g[V1][0] = add32(c.g[V1][0], c.g[A2][0]);
                    c.g[V0][0] = Addr(r32(c, V1) + 0x10).lhu(c);
                    c.g[V0][0] = add32(c.g[V0][0], c.g[S1][0]);
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    c.g[V0][0] = add32(c.g[V0][0], c.g[A1][0]);
                    c.g[S0][0] = Addr(r32(c, V0) + 2).lhu(c);
                    c.g[V1][0] = c.g[S0][0] & 0x8000;
                    if c.g[V1][0] != 0 {
                        c.cycles += 6;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = c.g[S1][0];
                        call_ptr(c, FLAGS64_CLEAR, 0x0010_3b94)?;
                        c.g[A1][0] = c.g[S0][0] & 0x7fff;
                        c.g[A0][0] = c.g[S5][0];
                        RIDER_ANIM_PLAY_EVENT_SOUND.call(c, 0x0010_3ba0)?;
                        played = true;
                    }
                }
                if played {
                    c.cycles += 1;
                }
                c.g[S1][0] = add32(c.g[S1][0], 1);
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S3][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = 0;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0010_3b18);
                }
            }
        }
        c.cycles += 11;
        for (reg, off) in [(S0, 0x80), (S1, 0x70), (S2, 0x60), (S3, 0x50), (S4, 0x40), (S5, 0x30), (S6, 0x20), (S7, 0x10)] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// libgcc soft float.

/// `fp_number_type` classes (libgcc `fp-bit`).
mod fp_class {
    pub const SNAN: u64 = 0;
    pub const QNAN: u64 = 1;
    pub const ZERO: u64 = 2;
    pub const NUMBER: u64 = 3;
    pub const INFINITY: u64 = 4;
}

/// `void __unpack_f(const float* src, fp_number_type* dst)` (0x413068):
/// split a float into class (`+0`), sign (`+4`), unbiased exponent (`+8`)
/// and fraction (`+0xc`, with the implicit bit at 1 << 30). This build has
/// no denormals: a zero exponent is always ZERO. NaN fractions keep their
/// raw 23 bits; bit 20 (this build's quiet bit) selects QNAN over SNAN.
pub fn unpack_f(c: &mut Cpu) {
    c.cycles += 9;
    let dst = Addr(abi::arg(c, 1));
    c.g[V0][0] = Addr(abi::arg(c, 0)).lw(c);
    let bits = c.g[V0][0] as u32;
    c.g[V1][0] = 0x7f_ffff;
    c.g[A0][0] = sx(bits >> 31);
    c.g[A2][0] = sx(bits >> 23) & 0xff;
    let fraction = c.g[V0][0] & 0x7f_ffff;
    c.g[V1][0] = fraction;
    let exponent = c.g[A2][0];
    dst.off(4).sw(c, c.g[A0][0]);
    if exponent == 0 {
        c.cycles += 3;
        c.g[V0][0] = fp_class::ZERO;
        dst.sw(c, fp_class::ZERO);
    } else if exponent != 0xff {
        c.cycles += 3 + 8;
        c.g[V1][0] = sx((fraction as u32) << 7) | 0x4000_0000;
        c.g[A0][0] = sx((exponent as u32).wrapping_sub(0x7f));
        c.g[V0][0] = fp_class::NUMBER;
        dst.off(0xc).sw(c, c.g[V1][0]);
        dst.off(8).sw(c, c.g[A0][0]);
        dst.sw(c, fp_class::NUMBER);
    } else if fraction != 0 {
        c.cycles += 3 + 2 + 3;
        c.g[V0][0] = 1;
        if fraction & 0x10_0000 != 0 {
            c.cycles += 2;
            dst.sw(c, fp_class::QNAN);
        } else {
            c.cycles += 1;
            dst.sw(c, fp_class::SNAN);
        }
        c.cycles += 2;
        dst.off(0xc).sw(c, fraction);
    } else {
        c.cycles += 3 + 2 + 3;
        c.g[V0][0] = fp_class::INFINITY;
        dst.sw(c, fp_class::INFINITY);
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Front-end async file manager.

/// `FEAsyncManager`: five slots of 0x11c bytes from `+0x100`: `+0` the loaded
/// data, `+4` the async file handle, `+0x10` the registered resource id,
/// `+0x14` the state (0 free, 1 wanted, 2 loading, 3 loaded), `+0x18` the
/// frame the slot is needed until.
mod fe_async {
    pub const SLOTS: u32 = 0x100;
    pub const SLOT_SIZE: u32 = 0x11c;
    pub const SLOT_COUNT: i64 = 5;
    pub const SLOT_DATA: u32 = 0x0;
    pub const HANDLE: u32 = 0x4;
    pub const ID: u32 = 0x10;
    pub const KEEP_UNTIL: u32 = 0x18;
    /// The front-end interface (`gp-0x854`) and its vtable (`+0x10d8`).
    pub const IFACE: u32 = 0x854;
    pub const IFACE_VT: u32 = 0x10d8;
    /// Interface methods: current frame, unregister resource, register.
    pub const VT_FRAME: u32 = 0x390;
    pub const VT_UNREGISTER: u32 = 0x190;
    pub const VT_REGISTER: u32 = 0x170;
    /// The name passed when registering a loaded file (`.rodata`).
    pub const REGISTER_NAME: u32 = 0x004a_1a18;
}

const FE_ASYNC_GET_STATE: Callee = Callee { addr: 0x001a_39c0, f: ee::s03::f_001a39c0 };
const FE_ASYNC_SET_STATE: Callee = Callee { addr: 0x001a_39d8, f: ee::s03::f_001a39d8 };
const FE_ASYNC_LOAD_SLOT: Callee = Callee { addr: 0x001a_3cd8, f: ee::s03::f_001a3cd8 };
const MEM_FREE: u32 = 0x0031_7e98;
const ASYNC_FILE_STATUS: u32 = 0x003d_f980;
const ASYNC_FILE_RELEASE: u32 = 0x003d_f9d8;

/// Call interface method `slot` (`{delta, fn}` at `vt + slot`) on the
/// interface in `obj_reg`, as `lw vt; lh delta; lw fn; addu a0` with the
/// registers the original uses (`vt_reg`, `fn_reg`).
fn iface_call(c: &mut Cpu, obj_reg: usize, vt_reg: usize, fn_reg: usize, slot: u32, ret: u32) -> crate::call::Flow {
    c.g[vt_reg][0] = Addr(r32(c, obj_reg) + fe_async::IFACE_VT).lw(c);
    c.g[A0][0] = Addr(r32(c, vt_reg) + slot).lh(c);
    c.g[fn_reg][0] = Addr(r32(c, vt_reg) + slot + 4).lw(c);
    let target = r32(c, fn_reg);
    c.g[A0][0] = add32(c.g[obj_reg][0], c.g[A0][0]);
    call_ptr(c, target, ret)
}

/// `void FEAsyncManager_Update(FEAsyncManager* self)` (0x1a39f0): per slot,
/// (a) a free or wanted slot that is past its keep-until frame and still
/// holds data is released (unregister, free, id -1); (b) a wanted slot still
/// within its frame starts loading; (c) a loading slot whose file finished
/// becomes loaded: release the file and register its data.
pub fn fe_async_manager_update(c: &mut Cpu) {
    use fe_async::*;
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S2, 0x70);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x80);
        frame.save(c, S3, 0x60);
        // s1 = &slot.id, s4 = &slot.keep_until, s5 = &slot.data, s6 = i * 0x11c.
        c.g[S1][0] = add32(c.g[S2][0], (SLOTS + ID) as u64);
        frame.save(c, S4, 0x50);
        c.g[S3][0] = 0;
        frame.save(c, S5, 0x40);
        c.g[S4][0] = add32(c.g[S2][0], (SLOTS + KEEP_UNTIL) as u64);
        frame.save(c, S6, 0x30);
        c.g[S5][0] = add32(c.g[S2][0], (SLOTS + SLOT_DATA) as u64);
        frame.save(c, S7, 0x20);
        c.g[S6][0] = 0;
        frame.save(c, FP, 0x10);
        c.g[S7][0] = 1;
        frame.save(c, S0, 0x90);
        c.g[FP][0] = 0x4a_0000;
        frame.save_ra(c, 0);
        let gp = c.g[GP][0] as u32;
        loop {
            // Head 0x1a3a40.
            let id = r32(c, S1);
            // (a) Free or wanted: release when past the keep-until frame.
            c.cycles += 5;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S3][0];
            FE_ASYNC_GET_STATE.call(c, 0x001a_3a4c)?;
            let free = c.g[V0][0] == 0;
            c.g[A1][0] = Addr(gp.wrapping_sub(IFACE)).lw(c);
            let mut check = free;
            if !free {
                c.cycles += 5;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S3][0];
                FE_ASYNC_GET_STATE.call(c, 0x001a_3a60)?;
                if c.g[V0][0] == c.g[S7][0] {
                    c.cycles += 1;
                    c.g[A1][0] = Addr(gp.wrapping_sub(IFACE)).lw(c);
                    check = true;
                }
            }
            if check {
                c.cycles += 9;
                iface_call(c, A1, V1, V0, VT_FRAME, 0x001a_3a80)?;
                c.g[V1][0] = Addr(id + 8).lw(c);
                c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V1][0] != 0 {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(id - 0x10).lw(c);
                    c.g[A2][0] = Addr(gp.wrapping_sub(IFACE)).lw(c);
                    if c.g[V0][0] != 0 {
                        c.cycles += 9;
                        c.g[A1][0] = Addr(id).lw(c);
                        iface_call(c, A2, V0, V1, VT_UNREGISTER, 0x001a_3ab4)?;
                        c.g[A0][0] = Addr(id - 0x10).lw(c);
                        if c.g[A0][0] != 0 {
                            c.cycles += 3;
                            call_ptr(c, MEM_FREE, 0x001a_3ac8)?;
                        }
                        Addr(r32(c, S5)).sw(c, 0);
                        c.cycles += 3;
                        c.g[V0][0] = u64::MAX;
                        Addr(r32(c, S4)).sw(c, 0);
                        Addr(id).sw(c, u64::MAX);
                    }
                    c.cycles += 1;
                }
            }
            c.g[A0][0] = c.g[S2][0];
            // (b) Wanted and still needed: load it.
            c.cycles += 4;
            c.g[A1][0] = c.g[S3][0];
            FE_ASYNC_GET_STATE.call(c, 0x001a_3ae4)?;
            if c.g[V0][0] == c.g[S7][0] {
                c.cycles += 10;
                c.g[A1][0] = Addr(gp.wrapping_sub(IFACE)).lw(c);
                iface_call(c, A1, V1, V0, VT_FRAME, 0x001a_3b04)?;
                c.g[V1][0] = Addr(id + 8).lw(c);
                c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let needed = c.g[V1][0] != 0;
                c.g[A0][0] = c.g[S2][0];
                if needed {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S3][0];
                    FE_ASYNC_LOAD_SLOT.call(c, 0x001a_3b1c)?;
                }
                c.cycles += 1;
            }
            c.g[A0][0] = c.g[S2][0];
            // (c) Loading: finished files become loaded.
            c.cycles += 5;
            c.g[A1][0] = c.g[S3][0];
            FE_ASYNC_GET_STATE.call(c, 0x001a_3b28)?;
            c.g[V1][0] = 2;
            if c.g[V0][0] == c.g[V1][0] {
                c.cycles += 6;
                c.g[V0][0] = add32(c.g[S2][0], (SLOTS + HANDLE) as u64);
                c.g[S0][0] = add32(c.g[V0][0], c.g[S6][0]);
                c.g[A0][0] = Addr(r32(c, S0)).lw(c);
                call_ptr(c, ASYNC_FILE_STATUS, 0x001a_3b44)?;
                if c.g[V0][0] == c.g[S7][0] {
                    c.cycles += 23;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S3][0];
                    c.g[A2][0] = 3;
                    FE_ASYNC_SET_STATE.call(c, 0x001a_3b5c)?;
                    c.g[A0][0] = Addr(r32(c, S0)).lw(c);
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 0;
                    call_ptr(c, ASYNC_FILE_RELEASE, 0x001a_3b6c)?;
                    // register(iface, data + data->header, name, 0, 0, -1)
                    c.g[T3][0] = Addr(gp.wrapping_sub(IFACE)).lw(c);
                    c.g[A2][0] = add32(c.g[FP][0], 0x1a18);
                    c.g[T2][0] = Addr(id - 0x10).lw(c);
                    c.g[A3][0] = 0;
                    c.g[V1][0] = Addr(r32(c, T3) + IFACE_VT).lw(c);
                    c.g[T0][0] = 0;
                    c.g[A1][0] = Addr(r32(c, T2) + 0x14).lw(c);
                    c.g[T1][0] = u64::MAX;
                    c.g[A0][0] = Addr(r32(c, V1) + VT_REGISTER).lh(c);
                    c.g[V0][0] = Addr(r32(c, V1) + VT_REGISTER + 4).lw(c);
                    c.g[A1][0] = add32(c.g[T2][0], c.g[A1][0]);
                    let target = r32(c, V0);
                    c.g[A0][0] = add32(c.g[T3][0], c.g[A0][0]);
                    call_ptr(c, target, 0x001a_3ba0)?;
                    Addr(id).sw(c, c.g[V0][0]);
                }
            }
            c.g[S3][0] = add32(c.g[S3][0], 1);
            // Next slot (0x1a3ba8).
            c.cycles += 6;
            for reg in [S6, S1, S5] {
                c.g[reg][0] = add32(c.g[reg][0], SLOT_SIZE as u64);
            }
            c.g[V0][0] = ((c.g[S3][0] as i64) < SLOT_COUNT) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S4][0] = add32(c.g[S4][0], SLOT_SIZE as u64);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001a_3a40);
            }
        }
        c.cycles += 12;
        for (reg, off) in [(S0, 0x90), (S1, 0x80), (S2, 0x70), (S3, 0x60), (S4, 0x50), (S5, 0x40), (S6, 0x30), (S7, 0x20), (FP, 0x10)] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// EA timer.

/// The current timer tick (`0x450dd0`) and its copy taken by the service
/// pass (`0x450e04`).
const EA_TIMER_NOW: u32 = 0x0045_0dd0;
const EA_TIMER_SERVICED_AT: u32 = 0x0045_0e04;
/// Sixteen timer callbacks of 0x10 bytes: `+0` the function, `+4` its
/// period, `+8` the tick it is next due, `+0xc` set while it runs.
const EA_TIMERS: u32 = 0x0051_ed98;
const EA_TIMER_COUNT: u64 = 16;

/// `int EATimer_ServiceCallbacks(void* arg)` (0x3e5928): run every timer
/// callback that is due and not already running, as `fn(arg, now - due)`,
/// and schedule it one period later (from the tick after the call). Returns
/// the callbacks' results OR'ed together. Walks the table from the end.
pub fn ea_timer_service_callbacks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = 0x45_0000;
        frame.save(c, S3, 0x40);
        c.g[A1][0] = 0x45_0000;
        c.g[S3][0] = c.g[V0][0];
        c.g[V1][0] = Addr(EA_TIMER_NOW).lw(c);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        c.g[V0][0] = sx(EA_TIMERS);
        frame.save(c, S2, 0x50);
        // s0 = &timer.running.
        c.g[S0][0] = sx(EA_TIMERS + 0xc);
        frame.save(c, S4, 0x30);
        c.g[S2][0] = 0;
        frame.save(c, S5, 0x20);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S6, 0x10);
        c.g[S5][0] = 1;
        frame.save_ra(c, 0);
        c.g[S6][0] = c.g[S3][0];
        Addr(EA_TIMER_SERVICED_AT).sw(c, c.g[V1][0]);
        c.g[S1][0] = EA_TIMER_COUNT - 1;
        loop {
            // Head 0x3e5980.
            let running = r32(c, S0);
            c.cycles += 3;
            c.g[A2][0] = Addr(running - 0xc).lw(c);
            if c.g[A2][0] != 0 {
                c.cycles += 5;
                c.g[V0][0] = Addr(EA_TIMER_NOW).lw(c);
                c.g[V1][0] = Addr(running - 4).lw(c);
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(running).lw(c);
                    if c.g[V0][0] == 0 {
                        c.cycles += 12;
                        c.g[A1][0] = Addr(EA_TIMER_NOW).lw(c);
                        c.g[A0][0] = c.g[S4][0];
                        Addr(running).sw(c, c.g[S5][0]);
                        let callback = r32(c, A2);
                        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(r32(c, V1)));
                        call_ptr(c, callback, 0x003e_59c0)?;
                        c.g[S2][0] |= c.g[V0][0];
                        c.g[V1][0] = Addr(EA_TIMER_NOW).lw(c);
                        c.g[V0][0] = Addr(running - 8).lw(c);
                        Addr(running).sw(c, 0);
                        c.g[V1][0] = add32(c.g[V1][0], c.g[V0][0]);
                        Addr(running - 4).sw(c, c.g[V1][0]);
                    }
                }
            }
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            c.cycles += 2;
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = add32(c.g[S0][0], 0x10);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003e_5980);
            }
        }
        c.cycles += 11;
        c.g[V0][0] = c.g[S2][0];
        for (reg, off) in [(S0, 0x70), (S1, 0x60), (S2, 0x50), (S3, 0x40), (S4, 0x30), (S5, 0x20), (S6, 0x10)] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// World painter.

/// `bool tWPIGD_Lighting_PairDiffers(const int a[2], const int b[2])`
/// (0x2bb100): whether either of the two words differs.
pub fn t_wpigd_lighting_pair_differs(c: &mut Cpu) {
    let (a, b) = (Addr(abi::arg(c, 0)), Addr(abi::arg(c, 1)));
    c.cycles += 4;
    c.g[V1][0] = a.lw(c);
    c.g[V0][0] = b.lw(c);
    c.g[A2][0] = 0;
    let mut differs = c.g[V1][0] != c.g[V0][0];
    if !differs {
        c.cycles += 4;
        c.g[V1][0] = a.off(4).lw(c);
        c.g[V0][0] = b.off(4).lw(c);
        differs = c.g[V1][0] != c.g[V0][0];
    }
    if differs {
        c.cycles += 1;
        c.g[A2][0] = 1;
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A2][0];
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Replay playback.

/// `PlaybackMan`: `+4` the replay stream (`+4` capacity, `+8` read/write
/// position, `+0xc` length; bytes from `+0x10`), `+0x64` the live input pad
/// (`+0` button count, 0x1c-byte buttons with the value at `+4`), `+0x68`
/// the pad the frame is fed to, `+0x70` the mode (1 play back, 3 record).
mod playback {
    pub const STREAM: u32 = 0x4;
    pub const LIVE_PAD: u32 = 0x64;
    pub const OUT_PAD: u32 = 0x68;
    pub const MODE: u32 = 0x70;
    pub const MODE_PLAY: u64 = 1;
    pub const MODE_RECORD: u64 = 3;
    pub const STREAM_CAP: u32 = 0x4;
    pub const STREAM_POS: u32 = 0x8;
    pub const STREAM_LEN: u32 = 0xc;
    pub const STREAM_BYTES: u64 = 0x10;
    /// `1/255` (byte to button value, `gp-0x4920` and `gp-0x491c`).
    pub const PLAY_SCALE: u32 = 0x4920;
    pub const RECORD_SCALE: u32 = 0x491c;
    pub const BUTTON_SIZE: u64 = 0x1c;
    /// Resume entries (the original's loop heads that can unwind).
    pub const ENTRY: u32 = 0x0026_ce50;
    pub const PLAY_LOOP: u32 = 0x0026_cea0;
    pub const RECORD_LOOP: u32 = 0x0026_cf48;
    pub const FEED: u32 = 0x0026_cee0;
}

const INPUT_PAD_UPDATE: u32 = 0x0032_1298;

/// Stages of [`playback_man_read_input_frame`] (the original's blocks).
#[derive(Clone, Copy)]
enum ReadFrame {
    PlayLoop,
    PlayDone,
    Feed,
    RecordLoop,
    RecordDone,
    Failed,
    Exit,
}

/// `bool PlaybackMan_ReadInputFrame(PlaybackMan* self)` (0x26ce50): one
/// replay frame of input. Playing back: read `n` and `n` button bytes from
/// the stream, scale them to floats and feed them to the pad. Recording:
/// write the live pad's `n` button values as bytes (`value * 255`), feed
/// the quantised values back the same way and grow the stream. Returns 1
/// when the stream is exhausted/full, else 0 (any other mode: 0).
///
/// The three loop back edges can unwind; the function is re-entered at
/// the loop head (or the feed call) with its state in the registers.
pub fn playback_man_read_input_frame(c: &mut Cpu) {
    use crate::events::back_edge_unwinds;
    use crate::ps2f::FpuF32;
    use playback::*;
    guest(c, |c| {
        let mut stage = match c.pc {
            ENTRY => {
                c.cycles += 7;
                let frame = Frame::open(c, 0x70);
                c.g[T1][0] = c.g[A0][0];
                frame.save_ra(c, 0x60);
                let this = r32(c, T1);
                c.g[V0][0] = 1;
                c.g[V1][0] = Addr(this + MODE).lw(c);
                let mode = c.g[V1][0];
                c.g[V0][0] = MODE_RECORD;
                if mode == MODE_PLAY {
                    c.cycles += 6;
                    c.g[V1][0] = Addr(this + STREAM).lw(c);
                    let stream = r32(c, V1);
                    c.g[A0][0] = Addr(stream + STREAM_POS).lw(c);
                    c.g[V0][0] = Addr(stream + STREAM_LEN).lw(c);
                    let more = (c.g[A0][0] as i64) < (c.g[V0][0] as i64);
                    c.g[V0][0] = add32(c.g[A0][0], STREAM_BYTES);
                    if !more {
                        ReadFrame::Failed
                    } else {
                        c.cycles += 4;
                        c.g[V1][0] = add32(c.g[V1][0], c.g[V0][0]);
                        c.g[A3][0] = Addr(r32(c, V1)).lbu(c);
                        c.g[V1][0] = add32(c.g[V1][0], 1);
                        if c.g[A3][0] == 0 {
                            ReadFrame::PlayDone
                        } else {
                            c.cycles += 3;
                            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(PLAY_SCALE));
                            c.g[A1][0] = c.g[SP][0];
                            c.g[A0][0] = c.g[A3][0];
                            ReadFrame::PlayLoop
                        }
                    }
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    if mode != MODE_RECORD {
                        ReadFrame::Exit
                    } else {
                        c.cycles += 7;
                        c.g[A0][0] = Addr(this + STREAM).lw(c);
                        let stream = r32(c, A0);
                        c.g[V0][0] = Addr(stream + STREAM_CAP).lw(c);
                        c.g[A2][0] = Addr(stream + STREAM_POS).lw(c);
                        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x18));
                        let room = (c.g[A2][0] as i64) < (c.g[V0][0] as i64);
                        c.g[V0][0] = add32(c.g[A2][0], STREAM_BYTES);
                        if !room {
                            ReadFrame::Failed
                        } else {
                            c.cycles += 7;
                            c.g[V1][0] = Addr(this + LIVE_PAD).lw(c);
                            c.g[A2][0] = add32(c.g[A0][0], c.g[V0][0]);
                            c.g[A3][0] = 0;
                            c.g[T0][0] = Addr(r32(c, V1)).lw(c);
                            Addr(r32(c, A2)).sb(c, c.g[T0][0]);
                            c.g[A2][0] = add32(c.g[A2][0], 1);
                            if (c.g[T0][0] as i64) <= 0 {
                                ReadFrame::RecordDone
                            } else {
                                c.cycles += 6;
                                c.g[AT][0] = 0x437f_0000;
                                c.f[2] = 0x437f_0000; // 255.0
                                c.g[A1][0] = c.g[SP][0];
                                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(RECORD_SCALE));
                                c.g[T2][0] = 0;
                                ReadFrame::RecordLoop
                            }
                        }
                    }
                }
            }
            PLAY_LOOP => ReadFrame::PlayLoop,
            RECORD_LOOP => ReadFrame::RecordLoop,
            FEED => ReadFrame::Feed,
            pc => {
                c.jump_out(pc, ENTRY, 0x0026_cfe8);
                return Ok(());
            }
        };
        loop {
            stage = match stage {
                ReadFrame::PlayLoop => loop {
                    // Byte -> float: `value = byte * scale`.
                    c.cycles += 9;
                    c.g[V0][0] = Addr(r32(c, V1)).lbu(c);
                    c.g[A0][0] = sx(r32(c, A0).wrapping_sub(1));
                    c.g[V1][0] = add32(c.g[V1][0], 1);
                    let v = FpuF32::from_i32(c.g[V0][0] as i32) * FpuF32(c.f[1]);
                    c.f[0] = v.0;
                    c.swc1(0, r32(c, A1));
                    let more = c.g[A0][0] != 0;
                    c.g[A1][0] = add32(c.g[A1][0], 4);
                    if !more {
                        break ReadFrame::PlayDone;
                    }
                    if back_edge_unwinds(c, PLAY_LOOP) {
                        return Ok(());
                    }
                },
                ReadFrame::PlayDone => {
                    c.cycles += 7;
                    c.g[V1][0] = Addr(r32(c, T1) + STREAM).lw(c);
                    c.g[A1][0] = c.g[A3][0];
                    c.g[A2][0] = c.g[SP][0];
                    let pos = Addr(r32(c, V1) + STREAM_POS);
                    c.g[V0][0] = pos.lw(c);
                    c.g[V0][0] = add32(add32(c.g[V0][0], 1), c.g[A3][0]);
                    pos.sw(c, c.g[V0][0]);
                    ReadFrame::Feed
                }
                ReadFrame::Feed => {
                    // InputPad_Update(out_pad, n, values)
                    c.cycles += 4;
                    c.g[A0][0] = Addr(r32(c, T1) + OUT_PAD).lw(c);
                    call_ptr(c, INPUT_PAD_UPDATE, 0x0026_cee8)?;
                    c.g[V0][0] = 0;
                    ReadFrame::Exit
                }
                ReadFrame::RecordLoop => loop {
                    // Button value -> byte (`value * 255`, truncated) and back.
                    c.cycles += 5;
                    c.g[V1][0] = Addr(r32(c, T1) + LIVE_PAD).lw(c);
                    c.g[V0][0] = Addr(r32(c, V1)).lw(c);
                    let present = (c.g[A3][0] as i64) < (c.g[V0][0] as i64);
                    c.g[V0][0] = add32(c.g[V1][0], c.g[T2][0]);
                    if present {
                        c.cycles += 2;
                        c.lwc1(0, r32(c, V0) + 4);
                    } else {
                        c.cycles += 1;
                        c.f[0] = 0;
                    }
                    c.cycles += 15;
                    let scaled = FpuF32(c.f[0]) * FpuF32(c.f[2]);
                    c.f[0] = scaled.0;
                    c.g[A3][0] = add32(c.g[A3][0], 1);
                    c.g[T2][0] = add32(c.g[T2][0], BUTTON_SIZE);
                    c.g[A0][0] = ((c.g[A3][0] as i64) < (c.g[T0][0] as i64)) as u64;
                    c.f[3] = scaled.to_i32() as u32;
                    c.g[V0][0] = sx(c.f[3]);
                    Addr(r32(c, A2)).sb(c, c.g[V0][0]);
                    c.g[V1][0] = c.g[V0][0] & 0xff;
                    c.g[A2][0] = add32(c.g[A2][0], 1);
                    let v = FpuF32::from_i32(c.g[V1][0] as i32) * FpuF32(c.f[1]);
                    c.f[0] = v.0;
                    c.swc1(0, r32(c, A1));
                    let more = c.g[A0][0] != 0;
                    c.g[A1][0] = add32(c.g[A1][0], 4);
                    if !more {
                        break ReadFrame::RecordDone;
                    }
                    if back_edge_unwinds(c, RECORD_LOOP) {
                        return Ok(());
                    }
                },
                ReadFrame::RecordDone => {
                    // pos += n + 1; len += n + 1.
                    c.cycles += 13;
                    c.g[A0][0] = Addr(r32(c, T1) + STREAM).lw(c);
                    c.g[A1][0] = c.g[T0][0];
                    c.g[A2][0] = c.g[SP][0];
                    let pos = Addr(r32(c, A0) + STREAM_POS);
                    c.g[V0][0] = add32(add32(pos.lw(c), 1), c.g[T0][0]);
                    pos.sw(c, c.g[V0][0]);
                    c.g[V1][0] = Addr(r32(c, T1) + STREAM).lw(c);
                    let len = Addr(r32(c, V1) + STREAM_LEN);
                    c.g[V0][0] = add32(add32(len.lw(c), 1), c.g[T0][0]);
                    len.sw(c, c.g[V0][0]);
                    if back_edge_unwinds(c, FEED) {
                        return Ok(());
                    }
                    ReadFrame::Feed
                }
                ReadFrame::Failed => {
                    c.cycles += 1;
                    c.g[V0][0] = 1;
                    ReadFrame::Exit
                }
                ReadFrame::Exit => {
                    c.cycles += 3;
                    let frame = Frame { size: 0x70 };
                    frame.restore_ra(c, 0x60);
                    frame.close(c);
                    return Ok(());
                }
            };
        }
    })
}

/// `u8* libpad_GetDmaBuffer(int port, int slot)` (0x3ff9a8): the port's
/// 0x100-byte DMA buffer (written back from the data cache first), and of
/// its two 0x80-byte halves the one with the larger frame counter (`+0x58`
/// against `+0xd8`).
pub fn libpad_get_dma_buffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        c.g[V0][0] = PAD_SLOT_SIZE;
        c.g[V1][0] = PAD_PORT_SIZE;
        c.mult(0, A1, A1, V0);
        c.mult(1, A0, A0, V1);
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x52_0000;
        frame.save_ra(c, 0x10);
        c.g[V0][0] = sx(PAD_PORTS);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.g[A1][0] = add32(c.g[A1][0], c.g[A0][0]);
        c.g[A1][0] = add32(c.g[A1][0], c.g[V0][0]);
        c.g[S0][0] = Addr(r32(c, A1)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = add32(c.g[S0][0], 0xff);
        SYNC_DCACHE.call(c, 0x003f_f9e4)?;
        let buf = r32(c, S0);
        c.g[V0][0] = Addr(buf + 0x58).lw(c);
        c.g[V1][0] = Addr(buf + 0xd8).lw(c);
        frame.restore_ra(c, 0x10);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        c.g[V0][0] = sx(r32(c, V0) << 7);
        c.g[V0][0] = add32(c.g[S0][0], c.g[V0][0]);
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::events::driven;
    use crate::difftest::{check, put_words, Rng, DATA, GAME_GP};

    /// Write a NUL-terminated string.
    fn put_str(c: &mut Cpu, at: u32, s: &[u8]) {
        for (i, &b) in s.iter().enumerate() {
            c.bus.write8(at + i as u32, b);
        }
        c.bus.write8(at + s.len() as u32, 0);
    }

    /// A streamer with up to six sections in random states (every state and
    /// out-of-table values), sky and non-sky names, hull ref counts that make
    /// the resident/free tests go both ways, node trees with or without a
    /// section hull, both views disabled (the view update takes its short
    /// path). The section-table creation runs for real on a dlmalloc arena.
    #[test]
    fn c_world_cache_streamer_update_matches() {
        check(
            ee::s13::f_003a9258__generated,
            c_world_cache_streamer_update,
            120,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x3a_9258, 2);
                crate::game::fe::feasyncfile::tests::be_library_7(c);
                crate::game::fe::feasyncfile::tests::world_4(c);
                crate::game::bx::bxstring::tests::heap_init_3(c);
                c.pc = 0x003a_9258;
                // `.rodata` the body reads: the state jump table and the sky
                // name prefixes.
                put_words(
                    c,
                    streamer::STATE_TABLE,
                    &[0x3a9500, 0x3a92d8, 0x3a93dc, 0x3a94c4, 0x3a942c, 0x3a9498, 0x3a94a8],
                );
                for (k, n) in [b"ASKY", b"BSKY", b"CSKY", b"DSKY", b"ESKY"].iter().enumerate() {
                    put_str(c, streamer::SKY_NAMES + 8 * k as u32, &n[..]);
                }
                let this = DATA + 0x1_0000;
                let info = DATA + 0x1_0400;
                let sections = DATA + 0x1_0800;
                let cache = DATA + 0x1_2000;
                let desc = DATA + 0x1_4000;
                let frames = DATA + 0x1_5000;
                let app = DATA + 0x1_6000;
                let vt = DATA + 0x1_8000;
                let lights = DATA + 0x1_a000;
                // World_ResetLights: the lights object of the game (world_4's
                // `+0x84`) is empty, no race manager.
                c.bus.write32(DATA + 0x1200 + 0x90, lights);
                c.bus.write32(GAME_GP + 0x2a74, frames);
                c.bus.write32(frames + 0x1c, r.next_u32() % 1000);
                c.bus.write32(cache + 0x1bf0, r.next_u32() % 1000);
                c.bus.write32(GAME_GP + 0x2a90, app);
                c.bus.write32(app + 0x10d8, vt);
                c.bus.write16(vt + 0x330, (r.next_u32() % 0x40) as u16 * 4);
                c.bus.write32(vt + 0x334, FAKE);
                // The cache: descriptors, table array, memory manager.
                c.bus.write32(cache, desc);
                c.bus.write32(desc + 0x14, desc + 0x100);
                c.bus.write32(cache + 8, desc + 0x800);
                c.bus.write32(cache + 0xc, desc + 0xc00);
                for id in 0..5u32 {
                    c.bus.write32(cache + 0x3f0 + 0x18 * id, [0, 3, 1, 4, 0, 3][(r.next_u32() % 6) as usize]);
                }
                c.bus.write32(this + streamer::CACHE, cache);
                c.bus.write32(this + streamer::INFO, if seed % 10 == 0 { 0 } else { info });
                c.bus.write32(this + streamer::SECTIONS, sections);
                let count = if seed % 10 == 1 { 0 } else { 1 + seed % 6 };
                c.bus.write32(info + streamer::INFO_COUNT, count);
                c.bus.write32(this + streamer::RESIDENT, r.next_u32());
                c.bus.write32(this + streamer::FADING, r.next_u32());
                const NAMES: [&[u8]; 8] =
                    [b"ASKY01", b"BSKY", b"CSKYx", b"DSKY", b"ESKY_a", b"TERR", b"AS", b"ESKX"];
                for i in 0..count {
                    let e = this + streamer::ENTRIES + 8 * i;
                    c.bus.write32(e, r.next_u32() % 9);
                    c.bus.write32(e + 4, r.f32(100.0));
                    let s = sections + 0x58 * i;
                    put_str(c, s, NAMES[(r.next_u32() % 8) as usize]);
                    c.bus.write32(s + streamer::SECTION_ID, r.next_u32() % 5);
                    let node = DATA + 0x1_9000 + 0x100 * i;
                    c.bus.write32(s + streamer::SECTION_TREE, node);
                    let hull = node + 0x80;
                    c.bus.write32(node + 0x58, if r.next_u32() % 2 == 0 { 0 } else { hull });
                    c.bus.write16(hull + 2, (r.next_u32() % 5) as u16);
                }
                c.g[A0][0] = sx(this);
            },
        );
    }
    /// `scePadRead` on closed and open ports, with an 8-byte aligned and a
    /// 4-byte aligned DMA buffer (the two copy loops), either half current,
    /// interrupts on or off, data lengths 0..0x20.
    #[test]
    fn sce_pad_read_matches() {
        check(ee::s15::f_003ffa58__generated, sce_pad_read, 120, &|c, seed| {
            let mut r: Rng = machine(c, seed, 0x3f_fa58, 0);
            c.cop0[12] = if seed % 2 == 0 { 0 } else { 0x1_0000 };
            let port = (seed / 2) % 2;
            let slot = (seed / 4) % 4;
            let at = PAD_PORTS + 0x70 * port + 0x1c * slot;
            let buf = DATA + 0x1000 + if seed % 3 == 0 { 4 } else { 0 };
            c.bus.write32(at, buf);
            c.bus.write32(at + PAD_OPEN, if seed % 7 == 0 { 0 } else { 1 });
            for k in 0..0x40 {
                c.bus.write32(buf + 4 * k, r.next_u32());
            }
            // The current half: the one with the larger frame counter.
            c.bus.write32(buf + 0x58, r.next_u32() % 4);
            c.bus.write32(buf + 0xd8, r.next_u32() % 4);
            c.bus.write32(buf + PAD_DATA_LEN, r.next_u32() % 0x21);
            c.bus.write32(buf + 0x80 + PAD_DATA_LEN, r.next_u32() % 0x21);
            c.g[A0][0] = port as u64;
            c.g[A1][0] = slot as u64;
            c.g[A2][0] = sx(DATA + 0x3000);
            c.pc = 0x003f_fa58;
        });
    }
    /// `RiderAnim_FireSoundEvents` over sequences of 0..8 events with random
    /// reached/armed masks and codes with and without the sound bit. The
    /// rider is null, so the event sound call takes its short path.
    #[test]
    fn rider_anim_fire_sound_events_matches() {
        check(ee::s00::f_00103aa0__generated, rider_anim_fire_sound_events, 150, &|c, seed| {
            let mut r: Rng = machine(c, seed, 0x10_3aa0, 0);
            let (this, anim, banks, bank, seqs, events) =
                (DATA, DATA + 0x100, DATA + 0x400, DATA + 0x500, DATA + 0x600, DATA + 0x800);
            c.bus.write32(GAME_GP + ANIM_BANKS, banks);
            let b = r.next_u32() % 4;
            let seq = r.next_u32() % 6;
            c.bus.write32(banks + 4 * b, bank);
            c.bus.write32(bank + 8, seqs);
            c.bus.write32(bank + 0x10, events);
            let first = r.next_u32() % 8;
            c.bus.write16(seqs + 0x14 * seq + 0x10, first as u16);
            c.bus.write16(seqs + 0x14 * seq + 0x12, (seed % 9) as u16);
            for k in 0..20 {
                let code = r.next_u32() as u16;
                c.bus.write16(events + 4 * k + 2, code);
            }
            c.bus.write32(anim + ANIM_SEQ_ID, b | seq << 8);
            for k in 0..4 {
                c.bus.write32(anim + ANIM_EVENT_FLAGS + 4 * k, r.next_u32() | r.next_u32());
            }
            c.bus.write32(this + 0x60, 0);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(anim);
            c.pc = 0x0010_3aa0;
        });
    }
    /// `__unpack_f` on zeros, normal numbers, infinities and both NaN kinds
    /// of either sign.
    #[test]
    fn unpack_f_matches() {
        check(ee::s15::f_00413068__generated, unpack_f, 300, &|c, seed| {
            let mut r = Rng(0x41_3068 ^ (seed as u64) << 12);
            crate::difftest::scramble(c, &mut r);
            let sign = r.next_u32() & 0x8000_0000;
            let frac = match seed % 3 {
                0 => 0,
                1 => r.next_u32() & 0x10_0000,
                _ => r.next_u32() & 0x7f_ffff,
            };
            let exp = match seed % 5 {
                0 => 0,
                1 => 0xff,
                _ => r.next_u32() & 0xff,
            };
            c.bus.write32(DATA, sign | exp << 23 | frac);
            crate::difftest::fill(c, &mut r, DATA + 0x10, 4);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x10);
            c.pc = 0x0041_3068;
        });
    }
    /// `FEAsyncManager_Update` over five slots in random states: free/wanted
    /// slots past or within their keep-until frame with and without data (a
    /// real heap block, so the free is valid), loading slots whose async file
    /// is done, busy or an invalid handle. Interface methods are fakes.
    /// Wanted slots are always within their frame here (the load call is not
    /// reached: it would start a real file read).
    #[test]
    fn fe_async_manager_update_matches() {
        use fe_async::*;
        check(ee::s03::f_001a39f0__generated, fe_async_manager_update, 120, &|c, seed| {
            let mut r = crate::game::fe::feasyncfile::tests::base_13(c, seed, 0x1a_39f0, 6);
            let iface = crate::game::fe::feasyncfile::tests::IFACE;
            let mgr = DATA + 0x8000;
            let vt = DATA + 0x9000;
            let files = DATA + 0xa000;
            c.bus.write32(iface + IFACE_VT, vt);
            for (k, slot) in [VT_FRAME, VT_UNREGISTER, VT_REGISTER].into_iter().enumerate() {
                c.bus.write16(vt + slot, (r.next_u32() % 0x10) as u16 * 4);
                c.bus.write32(vt + slot + 4, FAKE + 0x10 * k as u32);
            }
            // The async file table: count, then 0x30-byte files.
            c.bus.write32(0x0051_9bf0, 8);
            c.bus.write32(0x0045_0c34, files);
            for i in 0..5u32 {
                let slot = mgr + SLOTS + SLOT_SIZE * i;
                let state = r.next_u32() % 5;
                c.bus.write32(slot + 0x14, state);
                c.bus.write32(slot + ID, r.next_u32());
                let past = r.next_u32() % 2 == 0 && state != 1;
                c.bus.write32(slot + KEEP_UNTIL, if past { r.next_u32() % 0x100 } else { 0x7fff_ffff });
                let data = if state == 2 {
                    let d = DATA + 0xb000 + 0x100 * i;
                    c.bus.write32(d + 0x14, r.next_u32() % 0x40);
                    d
                } else if r.next_u32() % 2 == 0 {
                    crate::game::ui::uiengine::tests::alloc(c, 0x40)
                } else {
                    0
                };
                c.bus.write32(slot + SLOT_DATA, data);
                let handle = if r.next_u32() % 5 == 0 { i } else { 0x100 + i };
                c.bus.write32(slot + HANDLE, handle);
                let file = files + 0x30 * i;
                c.bus.write32(file, 0x100 + i);
                c.bus.write32(file + 0x10, 1);
                c.bus.write32(file + 0x1c, r.next_u32() % 2);
            }
            c.g[A0][0] = sx(mgr);
            c.pc = 0x001a_39f0;
        });
    }
    /// `EATimer_ServiceCallbacks` over sixteen timers: empty, not yet due,
    /// already running, and due ones (fake callbacks, some of which unwind).
    #[test]
    fn ea_timer_service_callbacks_matches() {
        check(ee::s14::f_003e5928__generated, ea_timer_service_callbacks, 150, &|c, seed| {
            let mut r: Rng = machine(c, seed, 0x3e_5928, 16);
            let now = r.next_u32() % 1000;
            c.bus.write32(EA_TIMER_NOW, now);
            for i in 0..16u32 {
                let t = EA_TIMERS + 0x10 * i;
                let kind = r.next_u32() % 4;
                c.bus.write32(t, if kind == 0 { 0 } else { FAKE + 0x10 * i });
                c.bus.write32(t + 4, r.next_u32() % 100);
                c.bus.write32(t + 8, now.wrapping_add(r.next_u32() % 40).wrapping_sub(20));
                c.bus.write32(t + 0xc, (kind == 1) as u32);
            }
            c.g[A0][0] = r.next_u32() as u64;
            c.pc = 0x003e_5928;
        });
    }
    /// `tWPIGD_Lighting_PairDiffers` with equal pairs and either word
    /// differing.
    #[test]
    fn t_wpigd_lighting_pair_differs_matches() {
        check(ee::s08::f_002bb100__generated, t_wpigd_lighting_pair_differs, 200, &|c, seed| {
            let mut r = Rng(0x2b_b100 ^ (seed as u64) << 16);
            crate::difftest::scramble(c, &mut r);
            let (x, y) = (r.next_u32(), r.next_u32());
            c.bus.write32(DATA, x);
            c.bus.write32(DATA + 4, y);
            c.bus.write32(DATA + 0x10, if seed % 3 == 1 { x ^ 1 << (seed % 32) } else { x });
            c.bus.write32(DATA + 0x14, if seed % 3 == 2 { !y } else { y });
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x10);
            c.pc = 0x002b_b100;
        });
    }
    /// A playback manager in play, record or another mode (`sel % 6`), its
    /// stream (exhausted/full or not, 0..12 buttons) and an output pad.
    fn read_input_frame_setup(c: &mut Cpu, r: &mut Rng, sel: u32) {
        use playback::*;
        let (this, stream, live, out) = (DATA, DATA + 0x100, DATA + 0x800, DATA + 0x1000);
        let mode = [MODE_PLAY, MODE_RECORD, MODE_PLAY, MODE_RECORD, 0, 2][(sel % 6) as usize];
        c.bus.write32(this + MODE, mode as u32);
        c.bus.write32(this + STREAM, stream);
        c.bus.write32(this + LIVE_PAD, live);
        c.bus.write32(this + OUT_PAD, out);
        c.bus.write32(GAME_GP - PLAY_SCALE, 0x3b80_8081); // 1/255
        c.bus.write32(GAME_GP - RECORD_SCALE, 0x3b80_8081);
        let n = r.next_u32() % 13;
        let pos = r.next_u32() % 0x40;
        c.bus.write32(stream + STREAM_POS, pos);
        let full = r.next_u32() % 6 == 0;
        c.bus.write32(stream + STREAM_LEN, if full { pos } else { pos + 1 + n + r.next_u32() % 8 });
        c.bus.write32(stream + STREAM_CAP, if full { pos + 0x18 } else { pos + 0x40 });
        let bytes = stream + 0x10 + pos;
        c.bus.write8(bytes, n as u8);
        for k in 0..n {
            c.bus.write8(bytes + 1 + k, r.next_u32() as u8);
        }
        c.bus.write32(live, n);
        for k in 0..n {
            let v = match r.next_u32() % 4 {
                0 => 0,
                1 => 0x3f80_0000,
                _ => r.f32(1.0) & 0x7fff_ffff,
            };
            c.bus.write32(live + 4 + 0x1c * k, v);
        }
        c.bus.write32(out, r.next_u32() % (n + 1));
        c.g[A0][0] = sx(this);
        c.pc = ENTRY;
    }

    #[test]
    fn playback_man_read_input_frame_matches() {
        check(ee::s07::f_0026ce50__generated, playback_man_read_input_frame, 150, &|c, seed| {
            let mut r: Rng = machine(c, seed, 0x26_ce50, 0);
            read_input_frame_setup(c, &mut r, seed);
        });
    }

    driven!(gen_read_input_frame, ee::s07::f_0026ce50__generated);
    driven!(hand_read_input_frame, playback_man_read_input_frame);

    /// The same with events (delivered, or unwinding and re-entering at the
    /// loop heads) at the back edges.
    #[test]
    fn playback_man_read_input_frame_events_match() {
        use crate::difftest::events::{check_events, schedule};
        check_events(gen_read_input_frame, hand_read_input_frame, 96, &|c, seed| {
            let mut r = Rng(0x26_ce50 ^ (seed as u64) << 20);
            crate::difftest::scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            // Play and record only (the loops).
            read_input_frame_setup(c, &mut r, (seed / 3) % 4);
            // The output pad already holds every button, so InputPad_Update
            // skips its memset (whose own event checks would take the
            // scheduled event outside this function's resume heads).
            c.bus.write32(DATA + 0x1000, 0x10);
            let mode = schedule(c, &mut r, seed, playback::ENTRY);
            c.g[GP][0] = GAME_GP as u64;
            c.next_event = c.cycles + 4 + (r.next_u32() % 40) as u64;
            mode
        });
    }
    /// `libpad_GetDmaBuffer` for every port/slot, either half newer,
    /// interrupts on or off (the cache sync's two paths).
    #[test]
    fn libpad_get_dma_buffer_matches() {
        check(ee::s15::f_003ff9a8__generated, libpad_get_dma_buffer, 120, &|c, seed| {
            let mut r: Rng = machine(c, seed, 0x3f_f9a8, 0);
            c.cop0[12] = if seed % 2 == 0 { 0 } else { 0x1_0000 };
            let port = (seed / 2) % 2;
            let slot = (seed / 4) % 4;
            let buf = DATA + 0x1000 + 0x100 * (r.next_u32() % 8);
            c.bus.write32(PAD_PORTS + 0x70 * port + 0x1c * slot, buf);
            c.bus.write32(buf + 0x58, r.next_u32() % 4);
            c.bus.write32(buf + 0xd8, r.next_u32() % 4);
            c.g[A0][0] = port as u64;
            c.g[A1][0] = slot as u64;
            c.pc = 0x003f_f9a8;
        });
    }
}
