//! Staging wave 207 (oc257): hand-written replacements for the hottest
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
use ssx3_ee_rt::vu0::{Src, W, Y, Z};
use ssx3_ee_rt::Cpu;

use crate::guest::Addr;
use crate::ps2f::FpuF32;

// ------------------------------------------------- GameViewMan_PrefetchViews

/// `GameViewMan.views` (`GameCamera*[]`, from +0x4) and `.count` (+0x10).
const GVM_VIEWS: u32 = 0x4;
const GVM_COUNT: u32 = 0x10;

/// `void GameViewMan_PrefetchViews(GameViewMan* self)` (0x22e8b8): tells the
/// world streamer where every active view camera looks
/// (`GameCamera_UpdateStreamView` for views 0..count).
pub fn game_view_man_prefetch_views(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 17, 0x20);
        frame.save(c, 18, 0x10);
        frame.save(c, 16, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S2).wrapping_add(GVM_COUNT)).lw(c);
        c.g[S1][0] = 0;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[S0][0] = sx(r32(c, S2).wrapping_add(GVM_VIEWS));
            c.g[A0][0] = Addr(r32(c, S0)).lw(c);
            loop {
                c.cycles += 7;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
                call_ptr(c, 0x0015_ec98, 0x0022_e8f4)?;
                c.g[V0][0] = Addr(r32(c, S2).wrapping_add(GVM_COUNT)).lw(c);
                c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A0][0] = Addr(r32(c, S0)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0022_e8e8);
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------------------ SND_GetPanGains

/// The sound library's pan state: `u8 channels` at +0x47.
const SND_PAN_STATE: u32 = 0x0050_a8e8;
const SND_PAN_CHANNELS: u32 = 0x47;
/// Per output channel a 256-entry `u8` gain table, indexed by `pan >> 8`.
const SND_PAN_TABLES: u32 = 0x0050_ac38;

/// `void SND_GetPanGains(u16 pan, float* gains)` (0x3bc4c0): one gain per
/// output channel, `table[ch][pan >> 8] / 255.0`.
pub fn snd_get_pan_gains(c: &mut Cpu) {
    c.cycles += 7;
    c.g[A0][0] = sx(((c.g[A0][0] & 0xffff) as u32) >> 8);
    c.g[V1][0] = sx(SND_PAN_STATE);
    c.g[V0][0] = Addr(SND_PAN_STATE + SND_PAN_CHANNELS).lbu(c);
    c.g[A2][0] = 0;
    if c.g[V0][0] != 0 {
        c.cycles += 5;
        c.g[V0][0] = 0x51_0000;
        c.g[AT][0] = 0x437f_0000;
        let k255 = FpuF32(0x437f_0000);
        c.f[1] = k255.0;
        c.g[T0][0] = c.g[V1][0];
        c.g[A3][0] = sx(SND_PAN_TABLES);
        loop {
            c.cycles += 13;
            let table = Addr(r32(c, A3)).lw(c);
            c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
            c.g[A3][0] = sx(r32(c, A3).wrapping_add(4));
            c.g[V1][0] = sx((table as u32).wrapping_add(r32(c, A0)));
            let level = Addr(r32(c, V1)).lbu(c);
            let gain = FpuF32::from_i32(level as i32).div(k255);
            c.f[0] = gain.0;
            Addr(r32(c, A1)).write_fpu(c, gain);
            let channels = Addr(r32(c, T0).wrapping_add(SND_PAN_CHANNELS)).lbu(c);
            c.g[V0][0] = ((c.g[A2][0] as i64) < (channels as i64)) as u64;
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(4));
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003b_c4f0);
            }
        }
    }
    c.cycles += 2;
    ret(c);
}

// --------------------------------------------------- MultiParticleMan_EmitAll

/// `MultiParticle_EmitAtInstances` (0x358260).
const MULTI_PARTICLE_EMIT_AT_INSTANCES: Callee = Callee {
    addr: 0x0035_8260,
    f: ee::s11::f_00358260,
};

/// `void MultiParticleMan_EmitAll(MultiParticle* slots[4])` (0x357c50):
/// emits every one of the manager's four multi-particle effects that is set.
pub fn multi_particle_man_emit_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = 3;
        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        loop {
            c.cycles += 4;
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
            c.g[A0][0] = c.g[V0][0];
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                MULTI_PARTICLE_EMIT_AT_INSTANCES.call(c, 0x0035_7c88)?;
            }
            c.cycles += 2;
            if (c.g[S1][0] as i64) < 0 {
                break;
            }
            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0035_7c70);
            }
        }
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// --------------------------------------------------- FEUIInput_PollUsbKeyboard

/// `cUsbKeyboard*` at gp+0x300 (0 when no USB keyboard is set up).
const FEUI_USB_KEYBOARD_GP: u32 = 0x300;
/// FEUIInput: key found (+0xc), key code (+0x10), modifier (+0x14).
const FEUI_KEY_FOUND: u32 = 0xc;
const FEUI_KEY_CODE: u32 = 0x10;
const FEUI_KEY_MOD: u32 = 0x14;

/// `void FEUIInput_PollUsbKeyboard(FEUIInput* self)` (0x1804b8): pops the
/// next key of USB keyboard channel 0 (`cUsbKeyboard_GetKey`) into the
/// input's key / modifier, `found` = whether there was one.
pub fn feui_input_poll_usb_keyboard(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let input = Addr(r32(c, S0));
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(FEUI_USB_KEYBOARD_GP)).lw(c);
        input.off(FEUI_KEY_FOUND).sw(c, 0);
        if c.g[A0][0] != 0 {
            c.cycles += 8;
            input.off(FEUI_KEY_MOD).sw(c, 0);
            c.g[A1][0] = 0;
            input.off(FEUI_KEY_CODE).sw(c, 0);
            c.g[A2][0] = sx(input.0.wrapping_add(FEUI_KEY_CODE));
            c.g[A3][0] = sx(input.0.wrapping_add(FEUI_KEY_MOD));
            call_ptr(c, 0x0026_6ba8, 0x0018_04ec)?;
            let found = c.g[V0][0] != 0;
            c.g[V0][0] = 1;
            if found {
                c.cycles += 1;
                Addr(r32(c, S0).wrapping_add(FEUI_KEY_FOUND)).sw(c, 1);
            }
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// -------------------------------------------------------- HaloModifier_Render

/// `Instance_GetNodeMatrix` (0x34fed8).
const HALO_INSTANCE_GET_NODE_MATRIX: Callee = Callee {
    addr: 0x0034_fed8,
    f: ee::s11::f_0034fed8,
};
/// `HaloModifier_DrawQuad` (0x2d1d10, cross-shard).
const HALO_DRAW_QUAD: u32 = 0x002d_1d10;
/// HaloModifier: instance (+0xc), texture quad (+0x10), size (+0x20),
/// mode (+0x24), node (+0x28), spin (+0x30).
const HALO_INSTANCE: u32 = 0xc;
const HALO_TEXTURE: u32 = 0x10;
const HALO_SIZE: u32 = 0x20;
const HALO_MODE: u32 = 0x24;
const HALO_NODE: u32 = 0x28;
const HALO_SPIN: u32 = 0x30;

/// `void HaloModifier_Render(HaloModifier* self)` (0x3462a0): draws the
/// halo quad at the translation of its instance node's matrix.
pub fn halo_modifier_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x80);
        frame.save(c, 16, 0x70);
        c.g[A2][0] = c.g[SP][0];
        frame.save(c, 17, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x50);
        let halo = r32(c, S0);
        c.g[S1][0] = sx(halo.wrapping_add(HALO_TEXTURE));
        c.g[A0][0] = Addr(halo.wrapping_add(HALO_INSTANCE)).lw(c);
        c.g[A1][0] = Addr(halo.wrapping_add(HALO_NODE)).lw(c);
        // The node matrix goes to sp+0; its translation row is at sp+0x30.
        HALO_INSTANCE_GET_NODE_MATRIX.call(c, 0x0034_62c8)?;
        let halo = r32(c, S0);
        c.lq(V0, frame.at(c, 0x30));
        c.g[A1][0] = c.g[S1][0];
        c.lwc1(13, halo.wrapping_add(HALO_SPIN));
        c.g[A0][0] = sx(frame.at(c, 0x40));
        c.g[A2][0] = Addr(halo.wrapping_add(HALO_MODE)).lw(c);
        c.lwc1(12, halo.wrapping_add(HALO_SIZE));
        c.sq(V0, frame.at(c, 0x40));
        call_ptr(c, HALO_DRAW_QUAD, 0x0034_62e8)?;
        frame.restore(c, 16, 0x70);
        frame.restore(c, 17, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------- WorldPainter_GetLightGlow18 / GetWeather58

/// The world painter's per-world slots (0xf0 bytes each).
const WP_SLOTS: u32 = 0x004f_a370;
const WP_SLOT_SIZE: u32 = 0xf0;

/// The shape both getters share: `float WorldPainter_GetX(int world)` =
/// `*(**(slots[world] + field))->vt[method]()`, where the method returns a
/// `float*`. `vslot` is the method's `{delta: i16, fn}` offset in the
/// v-table at +4; `ret` the call's return address.
fn world_painter_slot_float(c: &mut Cpu, field: u32, vslot: u32, ret_addr: u32) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[V1][0] = WP_SLOT_SIZE as u64;
        let frame = Frame::open(c, 0x10);
        let world = r32(c, A0);
        let offset = crate::abi::mult(c, world, WP_SLOT_SIZE);
        c.g[V0][0] = sx(WP_SLOTS);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(offset.wrapping_add(WP_SLOTS));
        c.g[V1][0] = Addr(offset.wrapping_add(WP_SLOTS).wrapping_add(field)).lw(c);
        c.g[A2][0] = Addr(r32(c, V1)).lw(c);
        c.g[A1][0] = Addr(r32(c, A2).wrapping_add(4)).lw(c);
        let delta = Addr(r32(c, A1).wrapping_add(vslot)).lh(c);
        c.g[V0][0] = Addr(r32(c, A1).wrapping_add(vslot + 4)).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A2).wrapping_add(delta as u32));
        call_ptr(c, target, ret_addr)?;
        c.lwc1(0, r32(c, V0));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float WorldPainter_GetLightGlow18(int world)` (0x2eee40): the light
/// glow value of the world's render-state object (slot +0x8, method at
/// v-table +0x78).
pub fn world_painter_get_light_glow18(c: &mut Cpu) {
    world_painter_slot_float(c, 0x8, 0x78, 0x002e_ee78);
}

/// `float WorldPainter_GetWeather58(int world)` (0x2ee738): a weather value
/// of the world's lens-fx object (slot +0x20, method at v-table +0x1a8).
pub fn world_painter_get_weather58(c: &mut Cpu) {
    world_painter_slot_float(c, 0x20, 0x1a8, 0x002e_e770);
}

// ---------------------------------------------------------------- ModObj_v25

/// ModObj: the default transform (+0x18, its matrix at +0x10) and a handle
/// to the attached object (+0x1c).
const MODOBJ_DEFAULT: u32 = 0x18;
const MODOBJ_HANDLE: u32 = 0x1c;

/// The attached object's v-table at +0: `{delta: i16, fn}` slots.
const MODOBJ_VT_PRIMARY: u32 = 0x98;
const MODOBJ_VT_FALLBACK: u32 = 0x90;

/// `void* ModObj_v25(ModObj* self)` (0x356128): the matrix the object
/// moves with: the attached object's (virtual 19, else virtual 18), or the
/// default transform's matrix (+0x10) when nothing is attached.
pub fn mod_obj_v25(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[V1][0] = Addr(this.wrapping_add(MODOBJ_HANDLE)).lw(c);
        c.g[S0][0] = 0;
        if c.g[V1][0] != 0 {
            c.cycles += 1;
            c.g[S0][0] = Addr(r32(c, V1)).lw(c);
        }
        c.cycles += 2;
        if c.g[S0][0] == 0 {
            c.g[V0][0] = Addr(this.wrapping_add(MODOBJ_DEFAULT)).lw(c);
            c.cycles += 2;
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x10));
        } else {
            for (vslot, ret_addr) in [
                (MODOBJ_VT_PRIMARY, 0x0035_6160),
                (MODOBJ_VT_FALLBACK, 0x0035_617c),
            ] {
                c.cycles += 7;
                let obj = r32(c, S0);
                c.g[V1][0] = Addr(obj).lw(c);
                let vt = r32(c, V1);
                let delta = Addr(vt.wrapping_add(vslot)).lh(c);
                c.g[V0][0] = Addr(vt.wrapping_add(vslot + 4)).lw(c);
                let target = r32(c, V0);
                c.g[A0][0] = sx(obj.wrapping_add(delta as u32));
                call_ptr(c, target, ret_addr)?;
                if c.g[V0][0] != 0 {
                    break;
                }
            }
        }
        frame.restore(c, 16, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------ VramHeap_NodesToScratchpad

/// `SprStream_Upload` (0x38f708).
const VRAM_SPR_STREAM_UPLOAD: Callee = Callee {
    addr: 0x0038_f708,
    f: ee::s12::f_0038f708,
};
/// `SyncDCache` (0x424698, cross-shard).
const VRAM_SYNC_DCACHE: u32 = 0x0042_4698;
/// The heap's node table: 0x1ff0 bytes from +0, and the word after it
/// (+0x1ff0) where the table's current address is kept.
const VRAM_NODES_SIZE: u32 = 0x1ff0;

/// `void VramHeap_NodesToScratchpad(VramHeap* self)` (0x365f68): writes the
/// node table back from the data cache and DMAs it to the scratchpad
/// stream buffer (waiting for the transfer); `self->nodes` = the
/// scratchpad copy.
pub fn vram_heap_nodes_to_scratchpad(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(r32(c, S0).wrapping_add(VRAM_NODES_SIZE - 1));
        call_ptr(c, VRAM_SYNC_DCACHE, 0x0036_5f80)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = VRAM_NODES_SIZE as u64;
        c.g[A2][0] = 2;
        VRAM_SPR_STREAM_UPLOAD.call(c, 0x0036_5f90)?;
        let nodes = c.g[V0][0];
        Addr(r32(c, S0).wrapping_add(VRAM_NODES_SIZE)).sw(c, nodes);
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// -------------------------------------------- SSXAudioSystem_PlayWallHitSound

/// `SSXAudioSystem_FindViewInRange` (0x288b40, cross-shard).
const WALLHIT_FIND_VIEW_IN_RANGE: u32 = 0x0028_8b40;
/// `Audio_EvalCurveTable` (0x290c10, cross-shard).
const WALLHIT_EVAL_CURVE_TABLE: u32 = 0x0029_0c10;
/// `SSXAudioSystem_IsTrackedSoundPlaying` (0x295028).
const WALLHIT_IS_TRACKED_SOUND_PLAYING: Callee = Callee {
    addr: 0x0029_5028,
    f: ee::s08::f_00295028,
};
/// `WorldTriggerManager_PlayTriggerSound` (0x2b5f60).
const WALLHIT_PLAY_TRIGGER_SOUND: Callee = Callee {
    addr: 0x002b_5f60,
    f: ee::s08::f_002b5f60,
};
/// `SSXAudioSystem_TrackSound` (0x295628).
const WALLHIT_TRACK_SOUND: Callee = Callee {
    addr: 0x0029_5628,
    f: ee::s08::f_00295628,
};
/// The event's sound table (`{count, _, ids[]}` at +0x10 of it).
const WALLHIT_EVENT_SOUNDS: u32 = 0x8c;
/// The rider's physics object (v-table at +0: position at +0x28,
/// velocity at +0x10).
const WALLHIT_RIDER_BODY: u32 = 0x6c0;
/// `WorldTriggerManager*` at gp+0x21e4.
const WALLHIT_TRIGGER_MANAGER_GP: u32 = 0x21e4;
/// The audio system's player (+0x118, `**` -> clock at +0x260).
const WALLHIT_PLAYER: u32 = 0x118;

/// `void SSXAudioSystem_PlayWallHitSound(SSXAudioSystem* self, Rider* rider,
/// SoundEvent* evt, int idx)` (0x296088): when a view is within 100 m of
/// the rider and sound `idx` of the event is not already tracked, plays it
/// (volume from the rider's speed through curve table 0, clamped 0..127)
/// and tracks it for the next second.
pub fn ssx_audio_system_play_wall_hit_sound(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x60);
        frame.save(c, 16, 0x50);
        frame.save(c, 17, 0x40);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, 18, 0x30);
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, 19, 0x20);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, 20, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S2).wrapping_add(WALLHIT_EVENT_SOUNDS)).lw(c);
        c.g[S3][0] = c.g[A0][0];
        // Ends early (`true`) at the shared exit with its extra cycle, or
        // (`false`) straight at the epilogue.
        let extra_cycle = 'play: {
            if c.g[V0][0] == 0 {
                break 'play true;
            }
            // Any view near the rider's position?
            c.cycles += 13;
            c.g[V1][0] = Addr(r32(c, S0).wrapping_add(WALLHIT_RIDER_BODY)).lw(c);
            c.g[S4][0] = sx(r32(c, S0).wrapping_add(WALLHIT_RIDER_BODY));
            let vt = r32(c, V1);
            let delta = Addr(vt.wrapping_add(0x28)).lh(c);
            c.g[V0][0] = Addr(vt.wrapping_add(0x2c)).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = sx(r32(c, S4).wrapping_add(delta as u32));
            call_ptr(c, target, 0x0029_60d4)?;
            c.g[AT][0] = 0x42c8_0000;
            c.f[12] = 0x42c8_0000;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, WALLHIT_FIND_VIEW_IN_RANGE, 0x0029_60e8)?;
            let none = (c.g[V0][0] as i64) < 0;
            c.g[A0][0] = c.g[S3][0];
            if none {
                break 'play true;
            }
            // Already playing this one?
            c.cycles += 5;
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S1][0];
            WALLHIT_IS_TRACKED_SOUND_PLAYING.call(c, 0x0029_60fc)?;
            if c.g[V0][0] != 0 {
                break 'play false;
            }
            // Volume from the length of the rider's velocity.
            c.cycles += 42;
            c.g[V1][0] = Addr(r32(c, S0).wrapping_add(WALLHIT_RIDER_BODY)).lw(c);
            let vt = r32(c, V1);
            let delta = Addr(vt.wrapping_add(0x10)).lh(c);
            c.g[V0][0] = Addr(vt.wrapping_add(0x14)).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = sx(r32(c, S4).wrapping_add(delta as u32));
            call_ptr(c, target, 0x0029_6118)?;
            c.lqc2(3, r32(c, V0));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(3, 22);
            c.mtc1(3, 12);
            c.g[A0][0] = 0;
            call_ptr(c, WALLHIT_EVAL_CURVE_TABLE, 0x0029_6148)?;
            // Clamp the volume to 0..127 and pick the sound: ids[idx], or
            // ids[0] when idx is out of range or its id is 0.
            let sounds = Addr(r32(c, S2).wrapping_add(WALLHIT_EVENT_SOUNDS)).lw(c);
            c.g[V0][0] = sounds;
            c.cvt_w_s(1, 0);
            c.mfc1(T0, 1);
            c.g[A2][0] = 0x7f;
            c.g[V1][0] = u64::MAX;
            c.g[T1][0] = Addr((sounds as u32).wrapping_add(0x10)).lw(c);
            c.g[A3][0] = c.g[S0][0];
            c.g[A1][0] = ((c.g[T0][0] as i64) < 128) as u64;
            c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(WALLHIT_TRIGGER_MANAGER_GP)).lw(c);
            c.g[V0][0] = Addr(r32(c, T1)).lw(c);
            if c.g[A1][0] == 0 {
                c.g[T0][0] = c.g[A2][0];
            }
            c.g[T1][0] = sx(r32(c, T1).wrapping_add(8));
            c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[T0][0] as i64)) as u64;
            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V1][0] == 0 {
                c.g[T0][0] = 0;
            }
            if c.g[V0][0] == 0 {
                c.g[S1][0] = 0;
            }
            c.g[A1][0] = c.g[S2][0];
            c.g[V0][0] = sx((r32(c, S1) << 2).wrapping_add(r32(c, T1)));
            c.g[V1][0] = Addr(r32(c, V0)).lw(c);
            if c.g[V1][0] != 0 {
                c.g[T1][0] = c.g[V0][0];
            }
            c.g[A2][0] = Addr(r32(c, T1)).lw(c);
            WALLHIT_PLAY_TRIGGER_SOUND.call(c, 0x0029_61a4)?;
            let failed = (c.g[V0][0] as i64) < 0;
            c.g[A2][0] = c.g[V0][0];
            if failed {
                break 'play true;
            }
            // Track it until the player's clock + 1000.
            c.cycles += 9;
            c.g[V1][0] = Addr(r32(c, S3).wrapping_add(WALLHIT_PLAYER)).lw(c);
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[V0][0] = Addr(r32(c, V1)).lw(c);
            c.g[T1][0] = c.g[S1][0];
            c.g[T0][0] = 0;
            c.g[A3][0] = Addr(r32(c, V0).wrapping_add(0x260)).lw(c);
            c.g[A3][0] = sx(r32(c, A3).wrapping_add(1000));
            WALLHIT_TRACK_SOUND.call(c, 0x0029_61d0)?;
            true
        };
        if extra_cycle {
            c.cycles += 1;
        }
        frame.restore(c, 16, 0x50);
        c.cycles += 7;
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0022_e8b8,
        game_view_man_prefetch_views,
        "GameViewMan_PrefetchViews",
    ),
    (0x003b_c4c0, snd_get_pan_gains, "SND_GetPanGains"),
    (
        0x0035_7c50,
        multi_particle_man_emit_all,
        "MultiParticleMan_EmitAll",
    ),
    (
        0x0018_04b8,
        feui_input_poll_usb_keyboard,
        "FEUIInput_PollUsbKeyboard",
    ),
    (0x0034_62a0, halo_modifier_render, "HaloModifier_Render"),
    (
        0x002e_ee40,
        world_painter_get_light_glow18,
        "WorldPainter_GetLightGlow18",
    ),
    (
        0x002e_e738,
        world_painter_get_weather58,
        "WorldPainter_GetWeather58",
    ),
    (0x0035_6128, mod_obj_v25, "ModObj_v25"),
    (
        0x0036_5f68,
        vram_heap_nodes_to_scratchpad,
        "VramHeap_NodesToScratchpad",
    ),
    (
        0x0029_6088,
        ssx_audio_system_play_wall_hit_sound,
        "SSXAudioSystem_PlayWallHitSound",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;

    use crate::difftest::fakes::machine;
    use crate::difftest::{check, elf_image, fill, put_words, scramble, Rng, DATA, GAME_GP};

    /// `GameViewMan_PrefetchViews`: 0..3 views, each running the real
    /// `GameCamera_UpdateStreamView` on a world cache with no streamer
    /// nodes (the camera test's fixture), race mode 1 or not.
    #[test]
    fn game_view_man_prefetch_views_matches() {
        check(
            ssx3_ee::s05::f_0022e8b8__generated,
            game_view_man_prefetch_views,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0022_e8b8, 0);
                elf_image(c);
                c.pc = 0x0022_e8b8;
                let app = DATA + 0x1000;
                let module = DATA + 0x1100;
                let race = DATA + 0x1200;
                c.bus.write32(GAME_GP - 0x848, app);
                c.bus.write32(app + 0x84, module);
                c.bus.write32(module + 0x84, race);
                c.bus.write32(
                    race + 0x10,
                    if seed % 4 == 0 { 1 } else { r.next_u32() % 3 },
                );
                let cache_ptr = DATA + 0x2000;
                let cache = DATA + 0x3000;
                let nodes = DATA + 0x2100;
                let clock = DATA + 0x2200;
                c.bus.write32(GAME_GP + 0x16c8, cache_ptr);
                c.bus.write32(cache_ptr, cache);
                c.bus.write32(cache + 0x10 + 4, nodes);
                c.bus.write32(nodes + 8, 0);
                c.bus.write32(GAME_GP + 0x2a74, clock);
                c.bus.write32(clock + 0x18, r.next_u32());
                let man = DATA + 0x400;
                let count = seed % 4;
                for i in 0..3 {
                    let cam = DATA + 0x8000 + 0x100 * i;
                    c.bus.write32(man + 4 + 4 * i, cam);
                    c.bus
                        .write32(cam + 0x8, ((r.next_u32() % 40000) as f32).to_bits());
                    for k in 0..4 {
                        c.bus.write32(cam + 0x20 + 4 * k, r.f32(5000.0));
                    }
                    c.bus.write32(cam + 0xb0, r.next_u32() % 4);
                    c.bus.write32(cam + 0xb4, r.next_u32());
                }
                c.bus.write32(man + 0x10, count);
                c.g[A0][0] = sx(man);
            },
        );
    }

    /// `SND_GetPanGains`: 0..6 output channels, random tables and pans
    /// (the high pan bits are masked off).
    #[test]
    fn snd_get_pan_gains_matches() {
        check(
            ssx3_ee::s13::f_003bc4c0__generated,
            snd_get_pan_gains,
            300,
            &|c, seed| {
                let mut r = Rng(0x3bc4_c000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003b_c4c0;
                let channels = (seed % 7) as u8;
                c.bus.write8(SND_PAN_STATE + SND_PAN_CHANNELS, channels);
                for ch in 0..6u32 {
                    let table = DATA + 0x1000 * (ch + 1);
                    c.bus.write32(SND_PAN_TABLES + 4 * ch, table);
                    fill(c, &mut r, table, 0x40);
                }
                c.g[A0][0] = r.next_u32() as u64 | (r.next_u32() as u64) << 32;
                c.g[A1][0] = sx(DATA + 0x100);
            },
        );
    }

    /// `MultiParticleMan_EmitAll`: each of the four slots empty or set;
    /// a set effect has no instances (count 0 or negative), so the real
    /// `MultiParticle_EmitAtInstances` takes its short path.
    #[test]
    fn multi_particle_man_emit_all_matches() {
        check(
            ssx3_ee::s11::f_00357c50__generated,
            multi_particle_man_emit_all,
            300,
            &|c, seed| {
                let mut r = Rng(0x357c_5000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0035_7c50;
                let man = DATA + 0x100;
                for i in 0..4u32 {
                    let set = (seed >> i) & 1 != 0;
                    let effect = DATA + 0x1000 + 0x100 * i;
                    c.bus.write32(man + 4 * i, if set { effect } else { 0 });
                    let count = if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        r.next_u32() | 0x8000_0000
                    };
                    c.bus.write32(effect + 4, count);
                }
                c.g[A0][0] = sx(man);
            },
        );
    }

    /// `FEUIInput_PollUsbKeyboard`: no keyboard; a keyboard that is closed,
    /// has an empty channel 0, or has a queued key (zero or non-zero key /
    /// modifier bytes), through the real `cUsbKeyboard_GetKey`.
    #[test]
    fn feui_input_poll_usb_keyboard_matches() {
        crate::difftest::check_lohi(
            ssx3_ee::s02::f_001804b8__generated,
            feui_input_poll_usb_keyboard,
            300,
            &|c, seed| {
                let mut r = Rng(0x0018_04b8 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x0018_04b8;
                let (input, kb) = (DATA + 0x100, DATA + 0x1000);
                fill(c, &mut r, input, 8);
                c.bus.write32(
                    GAME_GP + FEUI_USB_KEYBOARD_GP,
                    if seed % 5 == 0 { 0 } else { kb },
                );
                // cUsbKeyboard: open flag (+0x460), channel 0 count / read
                // index / key ring; the keyboard limit `UsbKb_GetMaxKeyboards`
                // returns.
                c.bus.write32(0x0052_6010, 1 + r.next_u32() % 4);
                c.bus
                    .write32(kb + 0x460, if seed % 7 == 1 { 0 } else { r.next_u32() | 1 });
                let read = r.next_u32() % 64;
                let count = if seed % 3 == 0 {
                    read
                } else {
                    read + 1 + r.next_u32() % 4
                };
                c.bus.write32(kb, count);
                c.bus.write32(kb + 4, read);
                for b in 0..0x80u32 {
                    let v = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        r.next_u32() as u8
                    };
                    c.bus.write8(kb + 8 + b, v);
                }
                c.g[A0][0] = sx(input);
            },
        );
    }

    /// `HaloModifier_Render`: the real `Instance_GetNodeMatrix` (no owner;
    /// node -1, the root or a node with one or two parents; the instance
    /// node test's fixture) and the real `HaloModifier_DrawQuad`, culled
    /// (a fake visibility method) or visible (a real `return 0`), modes
    /// 0..5, spin or not (the draw-quad test's fixture; its drawer methods
    /// are fakes, some of which unwind).
    #[test]
    fn halo_modifier_render_matches() {
        use crate::difftest::fakes::FAKE;
        const RETURN_ZERO: u32 = 0x003a_5ab0;
        check(
            ssx3_ee::s11::f_003462a0__generated,
            halo_modifier_render,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3462_a0, 3);
                c.pc = 0x0034_62a0;
                // The instance, its skeleton nodes and matrices.
                let inst = DATA;
                let skel = DATA + 0x400;
                let nodes = DATA + 0x500;
                let mats = DATA + 0x1000;
                for k in 0..0x10 {
                    c.bus.write32(inst + 0x10 + 4 * k, r.f32(4.0));
                }
                c.bus.write32(inst + 0xc, 0);
                c.bus.write32(inst + 0x84, r.f32(2.0));
                c.bus.write32(inst + 0x80, skel);
                c.bus.write32(skel + 8, nodes);
                for (n, parent) in [(0u32, u32::MAX), (1, 0), (2, 1), (3, 0)] {
                    c.bus.write32(nodes + 0x10 * n, parent);
                    c.bus.write32(nodes + 0x10 * n + 0xc, mats + 0x40 * n);
                }
                for k in 0..0x40 {
                    c.bus.write32(mats + 4 * k, r.f32(4.0));
                }
                let node = match r.next_u32() % 5 {
                    0 => u32::MAX,
                    k => k - 1,
                };
                // The world (visibility method) and the draw context.
                let (world, wvt, ctx, cvt) = (
                    DATA + 0x10000,
                    DATA + 0x14000,
                    DATA + 0x18000,
                    DATA + 0x1c000,
                );
                let cur = DATA + 0x2000 + 4 * (seed % 4);
                c.bus.write32(GAME_GP.wrapping_sub(0x854), world);
                c.bus.write32(world + 0x10d8, wvt);
                c.bus.write16(wvt + 0x2d8, (r.next_u32() & 0x7c) as u16);
                c.bus
                    .write32(wvt + 0x2dc, if seed % 5 == 0 { FAKE } else { RETURN_ZERO });
                c.bus.write32(GAME_GP + 0x2a90, ctx);
                fill(c, &mut r, ctx + 0xf50, 0x30);
                c.bus.write32(ctx + 0xe84, cur);
                c.bus.write32(ctx + 0x10d8, cvt);
                for (k, off) in [0x270u32, 0x278].into_iter().enumerate() {
                    c.bus.write16(cvt + off, (r.next_u32() & 0x7c) as u16);
                    c.bus.write32(cvt + off + 4, FAKE + 0x10 * (k as u32 + 1));
                }
                fill(c, &mut r, cur - 0x20, 0x20);
                fill(c, &mut r, 0x0050_1420, 6);
                c.bus.write32(GAME_GP.wrapping_sub(0x3f98), r.f32(1.0));
                // The halo.
                let halo = DATA + 0x8000;
                c.bus.write32(halo + HALO_INSTANCE, inst);
                fill(c, &mut r, halo + HALO_TEXTURE, 4);
                c.bus.write32(halo + HALO_SIZE, r.f32(50.0));
                c.bus.write32(halo + HALO_MODE, (seed / 2) % 6);
                c.bus.write32(halo + HALO_NODE, node);
                c.bus
                    .write32(halo + HALO_SPIN, if seed % 3 == 0 { 0 } else { r.f32(6.0) });
                c.g[A0][0] = sx(halo);
            },
        );
    }

    /// The world painter getters' fixture: world index 0..4, its slot's
    /// field -> object pointer -> object with a fake v-table method (which
    /// returns a float pointer, or unwinds).
    fn world_painter_slot_setup(c: &mut Cpu, seed: u32, entry: u32, field: u32, vslot: u32) {
        let mut r = machine(c, seed, entry as u64, 1);
        c.pc = entry;
        let world = seed % 5;
        let slot = WP_SLOTS + WP_SLOT_SIZE * world;
        let (holder, obj) = (DATA + 0x20_0000, DATA + 0x20_0100);
        // The floats the returned pointer points at (fake results land in
        // DATA..DATA+0x40000; the v-table is written over them).
        fill(c, &mut r, DATA, 0x1_0000);
        c.bus.write32(slot + field, holder);
        c.bus.write32(holder, obj);
        let vt = crate::difftest::fakes::vtable(c, &mut r, &[vslot]);
        c.bus.write32(obj + 4, vt);
        c.g[A0][0] = (world as u64) | (r.next_u32() as u64) << 32;
    }

    /// `WorldPainter_GetLightGlow18`.
    #[test]
    fn world_painter_get_light_glow18_matches() {
        crate::difftest::fakes::covered(
            ssx3_ee::s09::f_002eee40__generated,
            world_painter_get_light_glow18,
            300,
            &|c, seed| world_painter_slot_setup(c, seed, 0x002e_ee40, 0x8, 0x78),
        );
    }

    /// `WorldPainter_GetWeather58`.
    #[test]
    fn world_painter_get_weather58_matches() {
        crate::difftest::fakes::covered(
            ssx3_ee::s09::f_002ee738__generated,
            world_painter_get_weather58,
            300,
            &|c, seed| world_painter_slot_setup(c, seed, 0x002e_e738, 0x20, 0x1a8),
        );
    }

    /// `ModObj_v25`: no handle, a handle to no object, or an object whose
    /// first method answers a matrix (a fake) or 0 (the real leaf
    /// 0x1406c0), then the fallback method (a fake or the leaf); fakes
    /// unwind at either call.
    #[test]
    fn mod_obj_v25_matches() {
        const RETURN_ZERO_LEAF: u32 = 0x0014_06c0;
        crate::difftest::fakes::covered(ssx3_ee::s11::f_00356128__generated, mod_obj_v25, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x0035_6128, 2);
            c.pc = 0x0035_6128;
            let (this, handle, obj) = (DATA + 0x20_0000, DATA + 0x20_0100, DATA + 0x20_0200);
            c.bus.write32(this + MODOBJ_DEFAULT, r.next_u32());
            let vt =
                crate::difftest::fakes::vtable(c, &mut r, &[MODOBJ_VT_PRIMARY, MODOBJ_VT_FALLBACK]);
            match seed % 4 {
                0 => c.bus.write32(this + MODOBJ_HANDLE, 0),
                1 => {
                    c.bus.write32(this + MODOBJ_HANDLE, handle);
                    c.bus.write32(handle, 0);
                }
                _ => {
                    c.bus.write32(this + MODOBJ_HANDLE, handle);
                    c.bus.write32(handle, obj);
                    c.bus.write32(obj, vt);
                    if seed % 8 >= 4 {
                        c.bus.write32(vt + MODOBJ_VT_PRIMARY + 4, RETURN_ZERO_LEAF);
                    }
                    if seed % 16 >= 8 {
                        c.bus.write32(vt + MODOBJ_VT_FALLBACK + 4, RETURN_ZERO_LEAF);
                    }
                }
            }
            c.g[A0][0] = sx(this);
        });
    }

    /// `VramHeap_NodesToScratchpad`: the real `SyncDCache` (interrupts
    /// enabled or not) and `SprStream_Upload` / `Dma_ToScratchpad`
    /// (waiting for the transfer) on a random node table, to scratchpad
    /// stream offsets 0..0x200.
    #[test]
    fn vram_heap_nodes_to_scratchpad_matches() {
        check(
            ssx3_ee::s12::f_00365f68__generated,
            vram_heap_nodes_to_scratchpad,
            200,
            &|c, seed| {
                let mut r = Rng(0x0036_5f68 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x0036_5f68;
                let status = r.next_u32() & !0x1_0000;
                c.cop0[12] = if seed % 2 == 0 {
                    status | 0x1_0000
                } else {
                    status
                };
                c.cop0[28] = r.next_u32();
                let heap = DATA + 0x10 * (r.next_u32() % 0x40);
                fill(c, &mut r, heap, VRAM_NODES_SIZE / 4 + 4);
                for i in 0..0x1000 {
                    c.bus.spr[i * 4..i * 4 + 4].copy_from_slice(&r.next_u32().to_le_bytes());
                }
                c.bus
                    .write32(GAME_GP + 0x2a94, 0x10 * (r.next_u32() % 0x20));
                c.g[A0][0] = sx(heap);
            },
        );
    }

    /// `SSXAudioSystem_PlayWallHitSound`: no sound table; no view in range
    /// (no views, or views far away; the real `FindViewInRange`); the sound
    /// already tracked (a ring record `{evt, idx, 0}`; the real
    /// `IsTrackedSoundPlaying`); otherwise the real curve table on the
    /// rider's speed, the id pick (idx in / out of range, zero ids) and the
    /// real `PlayTriggerSound`, which fails (trigger type 3). The rider's
    /// position / velocity methods are fakes (which may unwind).
    /// Not reached: a successful `PlayTriggerSound` and `TrackSound`.
    #[test]
    fn ssx_audio_system_play_wall_hit_sound_matches() {
        use crate::difftest::fakes::{vtable, LOG_RESULTS};
        crate::difftest::check_lohi(
            ssx3_ee::s08::f_00296088__generated,
            ssx_audio_system_play_wall_hit_sound,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0029_6088, 2);
                elf_image(c);
                c.pc = 0x0029_6088;
                let base = DATA + 0x20_0000;
                let (app, module, viewman) = (base, base + 0x100, base + 0x200);
                let (rider, pos, vel) = (base + 0x1000, base + 0x300, base + 0x340);
                let (evt, sounds, table) = (base + 0x2000, base + 0x2100, base + 0x2200);
                let (wtm, wtable) = (base + 0x3000, base + 0x3400);
                let (sys, ring) = (base + 0x1_0000, base + 0x2_0000);
                // Views.
                c.bus.write32(GAME_GP - 0x848, app);
                c.bus.write32(app + 0x84, module);
                c.bus.write32(module + 0x84, viewman);
                let views = if seed % 7 == 3 {
                    0
                } else {
                    1 + r.next_u32() % 2
                };
                c.bus.write32(viewman + 0x10, views);
                let far = seed % 7 == 5;
                for i in 0..2 {
                    let view = base + 0x400 + 0x40 * i;
                    c.bus.write32(viewman + 4 + 4 * i, view);
                    for k in 0..4 {
                        let v = if far {
                            2.0e5f32.to_bits()
                        } else {
                            r.f32(3000.0)
                        };
                        c.bus.write32(view + 0x20 + 4 * k, v);
                    }
                }
                // The rider's position and velocity methods (fakes).
                let vt = vtable(c, &mut r, &[0x28, 0x10]);
                c.bus.write32(rider + WALLHIT_RIDER_BODY, vt);
                for k in 0..4 {
                    c.bus.write32(pos + 4 * k, r.f32(3000.0));
                    c.bus.write32(vel + 4 * k, r.f32(60.0));
                }
                c.bus.write32(LOG_RESULTS, pos);
                c.bus.write32(LOG_RESULTS + 4, vel);
                // The event's sound table: count 1..4, ids 0..7 (some 0).
                c.bus.write32(
                    evt + WALLHIT_EVENT_SOUNDS,
                    if seed % 9 == 0 { 0 } else { sounds },
                );
                c.bus.write32(sounds + 0x10, table);
                c.bus.write32(table, 1 + r.next_u32() % 4);
                for k in 0..5 {
                    let id = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        r.next_u32() % 8
                    };
                    c.bus.write32(table + 8 + 4 * k, id);
                }
                let idx = (r.next_u32() % 7).wrapping_sub(1);
                // Trigger types: all 3 (PlayTriggerSound fails at once).
                c.bus.write32(GAME_GP + WALLHIT_TRIGGER_MANAGER_GP, wtm);
                c.bus.write32(wtm + 0x33c, wtable);
                for id in 0..8 {
                    c.bus.write32(wtable + 8 * id, 3);
                }
                // The tracked-sound ring: empty, or one record for this
                // event and index.
                let cap = 40u32;
                let tail = r.next_u32() % cap;
                c.bus.write32(sys + 0x597c, cap);
                c.bus.write32(sys + 0x5980, ring);
                c.bus.write32(sys + 0x5988, tail);
                let tracked = seed % 5 == 2;
                c.bus
                    .write32(sys + 0x5984, if tracked { (tail + 5) % cap } else { tail });
                let record = [evt, idx, 0, r.next_u32(), r.next_u32()];
                for (k, v) in record.into_iter().enumerate() {
                    c.bus.write32(ring + 4 * ((tail + k as u32) % cap), v);
                }
                c.g[A0][0] = sx(sys);
                c.g[A1][0] = sx(rider);
                c.g[A2][0] = sx(evt);
                c.g[A3][0] = sx(idx);
            },
        );
    }
}
