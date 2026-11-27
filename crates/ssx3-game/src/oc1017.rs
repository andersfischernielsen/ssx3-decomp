//! Staging module oc1017 (wave wave261): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1017`.

#![allow(unused_imports)]

use crate::call::{call_ptr, guest, sp, Flow, Frame};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0024_61e0,
        cfe_progressive_verify_on_widget_action,
        "cFEStateProgressiveVerifyPS2_OnWidgetAction",
    ),
    (0x0024_7e20, ssxapp_global_ctors, "SSXApp_GlobalCtors"),
    (0x0024_7e40, ssxapp_global_dtors, "SSXApp_GlobalDtors"),
    (
        0x0028_61d0,
        ssx_audio_system_set_music_event1,
        "SSXAudioSystem_SetMusicEvent1",
    ),
    (
        0x0028_6a80,
        ssx_audio_system_exit_game,
        "SSXAudioSystem_ExitGame",
    ),
    (
        0x0028_6d18,
        ssx_audio_system_unload_memory_bank,
        "SSXAudioSystem_UnloadMemoryBank",
    ),
    (
        0x0028_70a0,
        ssx_audio_system_on_quick_restart,
        "SSXAudioSystem_OnQuickRestart",
    ),
    (
        0x0028_72a8,
        ssx_audio_system_start_pre_race_music,
        "SSXAudioSystem_StartPreRaceMusic",
    ),
];

/// `cFEStateProgressiveVerifyPS2::OnWidgetAction(self = a0, item = a1, msg = a2)`
/// (0x2461e0, 132 B). For the widget message 5 on a live item whose hash
/// (`item+0x38`) equals the hash of the "yes" string, it creates a memory-card
/// state (584 bytes) and pushes it on the engine's state stack.
///
/// Register and cycle behaviour follows the generated body: the S0 reload
/// happens after each compare, and the end block restores S0 again on the
/// allocation path, as the compiled code does.
pub fn cfe_progressive_verify_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        // Prologue: 0x30-byte frame, s0/s1/ra saved.
        c.cycles += 6;
        let f = Frame::open(c, 0x30);
        f.save(c, S1, 0x10);
        f.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        let no_item = c.g[A1][0] == 0;
        f.save_ra(c, 0);

        // `reached_end`: the body ran to its end block (cycles and S0 reload
        // there), not an early exit.
        let reached_end = if no_item {
            true
        } else {
            c.cycles += 3;
            c.g[V0][0] = 5;
            let other_msg = c.g[A2][0] != c.g[V0][0];
            f.restore(c, S0, 0x20);
            if other_msg {
                false
            } else {
                c.cycles += 6;
                let item = c.g[A1][0] as u32;
                c.g[A0][0] = 0x004a_0000u64;
                c.g[S0][0] = Addr(item).off(0x38).lw(c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2ac8));
                call_ptr(c, 0x0031_7670, 0x0024_6214)?;
                let hash_differs = c.g[S0][0] != c.g[V0][0];
                f.restore(c, S0, 0x20);
                if hash_differs {
                    false
                } else {
                    // cMemMan_alloc(584, "cFEStateMemCard", 0, 0)
                    c.cycles += 13;
                    c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x4688));
                    c.g[A0][0] = 0x248;
                    c.g[A2][0] = 0;
                    c.g[A3][0] = 0;
                    call_ptr(c, 0x0031_7d70, 0x0024_6234)?;
                    // func_00186A08(state, self->mpEngine)
                    let this = c.g[S1][0] as u32;
                    c.g[A1][0] = Addr(this).off(0x10).lw(c);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0018_6a08, 0x0024_6240)?;
                    // cUIStateStack_push(&engine->mStack, state)
                    c.g[A0][0] = Addr(this).off(0x10).lw(c);
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039_f400, 0x0024_6250)?;
                    true
                }
            }
        };

        if reached_end {
            c.cycles += 1;
            f.restore(c, S0, 0x20);
        }

        // Epilogue.
        c.cycles += 4;
        f.restore(c, S1, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `SSXApp_GlobalCtors` (0x247e20, 32 B): `SSXApp_StaticInitDestruct(1, 0xffff)`,
/// the static constructors' pass of the static-init table.
pub fn ssxapp_global_ctors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let f = Frame::open(c, 0x10);
        c.g[A0][0] = 1;
        f.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        call_ptr(c, 0x0024_2eb8, 0x0024_7e34)?;
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `SSXApp_GlobalDtors` (0x247e40, 32 B): `SSXApp_StaticInitDestruct(0, 0xffff)`,
/// the static destructors' pass of the same table.
pub fn ssxapp_global_dtors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let f = Frame::open(c, 0x10);
        c.g[A0][0] = 0;
        f.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        call_ptr(c, 0x0024_2eb8, 0x0024_7e54)?;
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem::SetMusicEvent1(self)` (0x2861d0, 48 B): calls the audio
/// object's music callback (the function pointer at `self->mpAudio+0x24`)
/// with `self+0x118 + lh(mpAudio+0x20)` as its object and 1 as its argument.
pub fn ssx_audio_system_set_music_event1(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let f = Frame::open(c, 0x10);
        c.g[A1][0] = 1;
        f.save_ra(c, 0);
        let this = c.g[A0][0] as u32;
        c.g[V1][0] = Addr(this).off(0x5558).lw(c);
        c.g[A0][0] = sx(this.wrapping_add(0x118));
        let obj = c.g[V1][0] as u32;
        c.g[A2][0] = Addr(obj).off(0x20).lh(c);
        c.g[V0][0] = Addr(obj).off(0x24).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        call_ptr(c, target, 0x0028_61f4)?;
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// One call of the exit chain: `obj->vt->fn(obj + vt->delta, 3)`, where the
/// caller set A1 = 3. Leaves V0 = the vtable, V1 = the function, A0 = the
/// object argument, as the compiled code does.
fn ssx_exit_call(c: &mut Cpu, obj: u32, ret: u32) -> Flow {
    c.g[V0][0] = Addr(obj).off(4).lw(c);
    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// One later link of the exit chain: `obj = self->load` (into A2), the link
/// `self->clear` is zeroed after the load, and a non-null object gets the
/// [`ssx_exit_call`] (with cycles 3 before the load and 6 on the call path).
fn ssx_exit_link(c: &mut Cpu, this: u32, load: u32, clear: u32, ret: u32) -> Flow {
    c.cycles += 3;
    c.g[A2][0] = Addr(this).off(load).lw(c);
    let empty = c.g[A2][0] == 0;
    Addr(this).off(clear).sw(c, 0);
    if !empty {
        c.cycles += 6;
        c.g[A1][0] = 3;
        let obj = c.g[A2][0] as u32;
        ssx_exit_call(c, obj, ret)?;
    }
    Ok(())
}

/// `SSXAudioSystem::ExitGame(self)` (0x286a80, 380 B): stops the audio
/// subsystems in order (resume, mode, music, loops, speech, object loops,
/// mix preset, timers), calls the speech callback and the speech stop.
pub fn ssx_audio_system_exit_game(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let f = Frame::open(c, 0x20);
        f.save(c, S0, 0x10);
        f.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        call_ptr(c, 0x0028_9bb8, 0x0028_6a94)?;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0028_5be0, 0x0028_6aa0)?;
        c.g[A0][0] = sx(this.wrapping_add(0x118));
        call_ptr(c, 0x002b_3ac0, 0x0028_6aa8)?;
        c.mtc1(0, 12);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0029_d678, 0x0028_6ab4)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x002a_4550, 0x0028_6abc)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0029_2b48, 0x0028_6ac4)?;

        // The object chain: the first link has no clear, the rest zero
        // their own link after loading it.
        c.g[A2][0] = Addr(this).off(0x625c).lw(c);
        let empty = c.g[A2][0] == 0;
        c.g[A1][0] = 3;
        if !empty {
            c.cycles += 5;
            let obj = c.g[A2][0] as u32;
            ssx_exit_call(c, obj, 0x0028_6ae4)?;
        }
        ssx_exit_link(c, this, 0x6260, 0x625c, 0x0028_6b08)?;
        ssx_exit_link(c, this, 0x5fa0, 0x6260, 0x0028_6b2c)?;
        ssx_exit_link(c, this, 0x5fa4, 0x5fa0, 0x0028_6b50)?;
        ssx_exit_link(c, this, 0x6c80, 0x5fa4, 0x0028_6b74)?;
        ssx_exit_link(c, this, 0x6c84, 0x6c80, 0x0028_6b98)?;

        c.cycles += 25;
        Addr(this).off(0x6c84).sw(c, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0029_80b0, 0x0028_6ba4)?;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0028_83b0, 0x0028_6bb0)?;

        // mgr = *self->mpMgr; mgr->timers stop with mgr+0x1d8.
        c.g[V0][0] = Addr(this).off(0x118).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
        call_ptr(c, 0x002a_dda0, 0x0028_6bc0)?;

        // The speech callback: (mgr+0x1d8) + lh(cfg+0x10), cfg+0x14.
        c.g[V0][0] = Addr(this).off(0x118).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0xab0).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x10).lh(c);
        c.g[A1][0] = Addr(c.g[V1][0] as u32).off(0x14).lw(c);
        let target = c.g[A1][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        call_ptr(c, target, 0x0028_6be0)?;

        c.g[A0][0] = sx(this.wrapping_add(0x5560));
        c.g[A1][0] = 0;
        call_ptr(c, 0x002b_11b0, 0x0028_6bec)?;

        f.restore(c, S0, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// The probe of the unload chain: `mgr = *(self+0x118)`, `cfg = mgr->0xacc`,
/// `busy = (cfg[off] ^ 1) != 0`, the call's A0 = `mgr + 0x1d8`; not busy takes
/// the next word (`cfg[off_next]`) into A1, busy sets A1 = -1.
fn ssx_unload_probe(c: &mut Cpu, this: u32, off: u32, off_next: u32) {
    c.g[V0][0] = Addr(this).off(0x118).lw(c);
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[A1][0] = Addr(c.g[V1][0] as u32).off(0xacc).lw(c);
    c.g[V0][0] = Addr(c.g[A1][0] as u32).off(off).lw(c) ^ 1;
    let busy = c.g[V0][0] != 0;
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1d8));
    if busy {
        c.cycles += 1;
        c.g[A1][0] = u64::MAX;
    } else {
        c.cycles += 2;
        c.g[A1][0] = Addr(c.g[A1][0] as u32).off(off_next).lw(c);
    }
}

/// The body of `SSXAudioSystem::UnloadMemoryBank` once the bank's handle
/// matches: bank 0 stops its music with the probe at 0x300, the other banks
/// stop their loops with the probe at 0x360, then the bank's object (the table
/// at `self+0x6c70`) is unloaded and its handle slot cleared.
fn ssx_unload_matched(c: &mut Cpu, this: u32) -> Flow {
    let s2 = sx((c.g[S3][0] as u32) << 2);
    c.cycles += 2;
    if c.g[S3][0] == 0 {
        c.cycles += 7;
        ssx_unload_probe(c, this, 0x300, 0x304);
        c.cycles += 5;
        c.mtc1(0, 12);
        c.g[S2][0] = s2;
        call_ptr(c, 0x002a_d650, 0x0028_6d8c)?;
    } else {
        c.cycles += 10;
        c.mtc1(0, 12);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0029_d678, 0x0028_6da0)?;
        ssx_unload_probe(c, this, 0x360, 0x364);
        c.cycles += 4;
        c.mtc1(0, 12);
        c.g[S2][0] = s2;
        call_ptr(c, 0x002a_d650, 0x0028_6dd4)?;
    }
    c.g[V0][0] = sx(this.wrapping_add(0x6c70));

    // The bank's object slot: unload it and clear both words.
    c.cycles += 4;
    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
    let slot = c.g[S1][0] as u32;
    c.g[V1][0] = Addr(slot).lw(c);
    let empty = c.g[V1][0] == 0;
    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(8));
    if !empty {
        c.cycles += 7;
        c.g[V0][0] = Addr(this).off(0x118).lw(c);
        c.g[RA][0] = sx(0x0028_6df4);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        call_ptr(c, 0x0028_be90, 0x0028_6df4)?;
        c.g[V1][0] = sx(this.wrapping_add(c.g[S2][0] as u32));
        Addr(slot).sw(c, 0);
        c.g[V0][0] = u64::MAX;
        let v = c.g[V0][0];
        let handle = c.g[V1][0] as u32;
        Addr(handle).off(0x6c78).sw(c, v);
    }
    Ok(())
}

/// `SSXAudioSystem::UnloadMemoryBank(self = a0, handle = a1)` (0x286d18,
/// 264 B): `slot = handle >> 8`, and only if `self->slot[slot] == handle & 0xff`
/// (the word at `self + 4*slot + 0x6c78`) the bank is unloaded.
pub fn ssx_audio_system_unload_memory_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let f = Frame::open(c, 0x50);
        f.save(c, S3, 0x10);
        f.save(c, S2, 0x20);
        f.save(c, S1, 0x30);
        f.save(c, S0, 0x40);
        c.g[S3][0] = sx((c.g[A1][0] as u32) >> 8);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
        f.save_ra(c, 0);
        c.g[V0][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
        c.g[A1][0] &= 0xff;
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x6c78).lw(c);
        if c.g[V1][0] == c.g[A1][0] {
            ssx_unload_matched(c, this)?;
            c.cycles += 1;
        }
        f.restore(c, S0, 0x40);

        // Epilogue.
        c.cycles += 6;
        f.restore(c, S1, 0x30);
        f.restore(c, S2, 0x20);
        f.restore(c, S3, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem::OnQuickRestart(self)` (0x2870a0, 104 B): resets the race
/// sounds, and unless the event is in mode 4 and the tracked sound is not
/// playing, tracks a new sound (`TrackSound(self, 0, 0, -1, 0, 0)`).
pub fn ssx_audio_system_on_quick_restart(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let f = Frame::open(c, 0x20);
        f.save(c, S0, 0x10);
        f.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0028_71b0, 0x0028_70b4)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x002a_4040, 0x0028_70bc)?;
        if c.g[V0][0] == 0 {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0029_5028, 0x0028_70d4)?;
            if c.g[V0][0] == 0 {
                c.cycles += 8;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                c.g[A3][0] = u64::MAX;
                c.g[T0][0] = 0;
                c.g[T1][0] = 0;
                call_ptr(c, 0x0029_5628, 0x0028_70f8)?;
            }
        }
        f.restore(c, S0, 0x10);

        // Epilogue.
        c.cycles += 3;
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem::StartPreRaceMusic(self)` (0x2872a8, 304 B): reads the
/// audio game module's config bytes (0x535bc8+0x48/0x49/0x4a); for the two
/// valid ranges either plays a random song (no song loaded) or asks the
/// speech system to say the DJ line, then records the played sound ids.
pub fn ssx_audio_system_start_pre_race_music(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let f = Frame::open(c, 0x40);
        f.save(c, S0, 0x30);
        f.save(c, S1, 0x20);
        f.save(c, S2, 0x10);
        f.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        call_ptr(c, 0x0028_b1c8, 0x0028_72c4)?;
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(8).lw(c);
        let v = c.g[V0][0];
        Addr(this).off(0x582c).sw(c, v);
        call_ptr(c, 0x0014_dc80, 0x0028_72d4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0028_72e0)?;

        c.g[V1][0] = 0x0053_0000;
        c.g[S2][0] = c.g[V1][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x48).lb(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(5));
        c.g[V0][0] = (c.g[V0][0] < 2) as u64;

        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(this).off(0x530).lw(c);
            let loaded = c.g[V0][0] != 0;
            c.g[S1][0] = sx(this.wrapping_add(0x118));
            if loaded {
                c.cycles += 8;
                c.g[V1][0] = Addr(this).off(0x5558).lw(c);
                c.g[A1][0] = 0;
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x20).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x24).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0028_7324)?;
                c.g[A0][0] = c.g[S0][0];
            } else {
                c.cycles += 9;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0028_d488, 0x0028_7334)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                c.g[A3][0] = u64::MAX;
                c.g[T0][0] = 0;
                call_ptr(c, 0x0028_cf98, 0x0028_734c)?;
                c.g[A0][0] = c.g[S0][0];
            }

            c.cycles += 5;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0029_5028, 0x0028_735c)?;
            if c.g[V0][0] == 0 {
                let mut skip_tail = false;
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x5bc8));
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x4a).lb(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
                c.g[V0][0] = (c.g[V0][0] < 2) as u64;
                let in_range = c.g[V0][0] != 0;
                c.g[V0][0] = 2;
                if in_range {
                    c.cycles += 3;
                    c.g[V1][0] = Addr(c.g[V1][0] as u32).off(0x49).lb(c);
                    let is_two = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 1;
                    if is_two {
                        c.cycles += 7;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0;
                        c.g[A2][0] = 0;
                        call_ptr(c, 0x002a_26f0, 0x0028_73a0)?;
                        c.g[A0][0] = sx(this.wrapping_add(0x5560));
                        c.g[A1][0] = 0;
                        call_ptr(c, 0x002b_1758, 0x0028_73ac)?;
                    } else {
                        c.cycles += 2;
                        let v = c.g[V0][0];
                        Addr(this).off(0x5788).sw(c, v);
                        skip_tail = true;
                    }
                }
                if !skip_tail {
                    c.cycles += 1;
                    c.g[V0][0] = 1;
                }

                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                let v = c.g[V0][0];
                Addr(this).off(0x5774).sw(c, v);
                call_ptr(c, 0x002b_4908, 0x0028_73bc)?;
                let v = c.g[V0][0];
                Addr(this).off(0x5778).sw(c, v);
            }
        }

        f.restore(c, S0, 0x30);

        // Epilogue.
        c.cycles += 5;
        f.restore(c, S1, 0x20);
        f.restore(c, S2, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    // Fixture objects live in DATA+0x200000 and up: clear of the fake log
    // (DATA+0x100000) and of the pointers the fakes return (DATA..DATA+0x40000).

    /// Gate word of this module's fakes, and its magic value.
    const OC1017_CTRL: u32 = DATA + 0x20_0000;
    const OC1017_MAGIC: u32 = 0x4f43_1017;
    /// The audio system (`self`) and the objects its fields point to.
    const OC1017_SELF: u32 = DATA + 0x21_0000;
    const OC1017_OBJ: u32 = DATA + 0x23_0000;
    /// Chain links (`LINK + 0x100*i`), their vtables (`VT + 0x100*i`).
    const OC1017_LINK: u32 = DATA + 0x24_0000;
    const OC1017_VT: u32 = DATA + 0x25_0000;
    /// The manager (`self->mpMgr` = `*(self+0x118)`), its config objects.
    const OC1017_MGR: u32 = DATA + 0x26_0000;
    const OC1017_CFG: u32 = DATA + 0x27_0000;
    const OC1017_CFG2: u32 = DATA + 0x28_0000;

    fn oc1017_w(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns 0
    /// if the mailbox word at `CTRL + mail` is set (the unused word `0x1fc`
    /// never is).
    fn oc1017_fake_call(c: &mut Cpu, addr: u32, mail: u32) {
        let zero = c.bus.read32(OC1017_CTRL + mail) != 0;
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, addr);
        if zero && c.pc == ra {
            c.g[V0][0] = 0;
        }
    }

    macro_rules! oc1017_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr, $mail:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1017_CTRL) != OC1017_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc1017_fake_call(c, $addr, $mail);
                }
            )*
            fn oc1017_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc1017_fakes!(
        (
            oc1017_fake_317670,
            OC1017_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670,
            0x100
        ),
        (
            oc1017_fake_317d70,
            OC1017_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70,
            0x1fc
        ),
        (
            oc1017_fake_186a08,
            OC1017_PREV_186A08,
            ee::s02::OVERRIDE_00186A08,
            ee::s02::f_00186a08__generated,
            0x0018_6a08,
            0x1fc
        ),
        (
            oc1017_fake_39f400,
            OC1017_PREV_39F400,
            ee::s13::OVERRIDE_0039F400,
            ee::s13::f_0039f400__generated,
            0x0039_f400,
            0x1fc
        ),
        (
            oc1017_fake_242eb8,
            OC1017_PREV_242EB8,
            ee::s06::OVERRIDE_00242EB8,
            ee::s06::f_00242eb8__generated,
            0x0024_2eb8,
            0x1fc
        ),
        (
            oc1017_fake_289bb8,
            OC1017_PREV_289BB8,
            ee::s07::OVERRIDE_00289BB8,
            ee::s07::f_00289bb8__generated,
            0x0028_9bb8,
            0x1fc
        ),
        (
            oc1017_fake_285be0,
            OC1017_PREV_285BE0,
            ee::s07::OVERRIDE_00285BE0,
            ee::s07::f_00285be0__generated,
            0x0028_5be0,
            0x1fc
        ),
        (
            oc1017_fake_2b3ac0,
            OC1017_PREV_2B3AC0,
            ee::s08::OVERRIDE_002B3AC0,
            ee::s08::f_002b3ac0__generated,
            0x002b_3ac0,
            0x1fc
        ),
        (
            oc1017_fake_29d678,
            OC1017_PREV_29D678,
            ee::s08::OVERRIDE_0029D678,
            ee::s08::f_0029d678__generated,
            0x0029_d678,
            0x1fc
        ),
        (
            oc1017_fake_2a4550,
            OC1017_PREV_2A4550,
            ee::s08::OVERRIDE_002A4550,
            ee::s08::f_002a4550__generated,
            0x002a_4550,
            0x1fc
        ),
        (
            oc1017_fake_292b48,
            OC1017_PREV_292B48,
            ee::s07::OVERRIDE_00292B48,
            ee::s07::f_00292b48__generated,
            0x0029_2b48,
            0x1fc
        ),
        (
            oc1017_fake_2980b0,
            OC1017_PREV_2980B0,
            ee::s08::OVERRIDE_002980B0,
            ee::s08::f_002980b0__generated,
            0x0029_80b0,
            0x1fc
        ),
        (
            oc1017_fake_2883b0,
            OC1017_PREV_2883B0,
            ee::s07::OVERRIDE_002883B0,
            ee::s07::f_002883b0__generated,
            0x0028_83b0,
            0x1fc
        ),
        (
            oc1017_fake_2adda0,
            OC1017_PREV_2ADDA0,
            ee::s08::OVERRIDE_002ADDA0,
            ee::s08::f_002adda0__generated,
            0x002a_dda0,
            0x1fc
        ),
        (
            oc1017_fake_2b11b0,
            OC1017_PREV_2B11B0,
            ee::s08::OVERRIDE_002B11B0,
            ee::s08::f_002b11b0__generated,
            0x002b_11b0,
            0x1fc
        ),
        (
            oc1017_fake_2ad650,
            OC1017_PREV_2AD650,
            ee::s08::OVERRIDE_002AD650,
            ee::s08::f_002ad650__generated,
            0x002a_d650,
            0x1fc
        ),
        (
            oc1017_fake_28be90,
            OC1017_PREV_28BE90,
            ee::s07::OVERRIDE_0028BE90,
            ee::s07::f_0028be90__generated,
            0x0028_be90,
            0x1fc
        ),
        (
            oc1017_fake_2871b0,
            OC1017_PREV_2871B0,
            ee::s07::OVERRIDE_002871B0,
            ee::s07::f_002871b0__generated,
            0x0028_71b0,
            0x1fc
        ),
        (
            oc1017_fake_2a4040,
            OC1017_PREV_2A4040,
            ee::s08::OVERRIDE_002A4040,
            ee::s08::f_002a4040__generated,
            0x002a_4040,
            0x104
        ),
        (
            oc1017_fake_295028,
            OC1017_PREV_295028,
            ee::s08::OVERRIDE_00295028,
            ee::s08::f_00295028__generated,
            0x0029_5028,
            0x108
        ),
        (
            oc1017_fake_295628,
            OC1017_PREV_295628,
            ee::s08::OVERRIDE_00295628,
            ee::s08::f_00295628__generated,
            0x0029_5628,
            0x1fc
        ),
        (
            oc1017_fake_28b1c8,
            OC1017_PREV_28B1C8,
            ee::s07::OVERRIDE_0028B1C8,
            ee::s07::f_0028b1c8__generated,
            0x0028_b1c8,
            0x1fc
        ),
        (
            oc1017_fake_14dc80,
            OC1017_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80,
            0x1fc
        ),
        (
            oc1017_fake_14dd58,
            OC1017_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58,
            0x1fc
        ),
        (
            oc1017_fake_28d488,
            OC1017_PREV_28D488,
            ee::s07::OVERRIDE_0028D488,
            ee::s07::f_0028d488__generated,
            0x0028_d488,
            0x1fc
        ),
        (
            oc1017_fake_28cf98,
            OC1017_PREV_28CF98,
            ee::s07::OVERRIDE_0028CF98,
            ee::s07::f_0028cf98__generated,
            0x0028_cf98,
            0x1fc
        ),
        (
            oc1017_fake_2a26f0,
            OC1017_PREV_2A26F0,
            ee::s08::OVERRIDE_002A26F0,
            ee::s08::f_002a26f0__generated,
            0x002a_26f0,
            0x1fc
        ),
        (
            oc1017_fake_2b1758,
            OC1017_PREV_2B1758,
            ee::s08::OVERRIDE_002B1758,
            ee::s08::f_002b1758__generated,
            0x002b_1758,
            0x1fc
        ),
        (
            oc1017_fake_2b4908,
            OC1017_PREV_2B4908,
            ee::s08::OVERRIDE_002B4908,
            ee::s08::f_002b4908__generated,
            0x002b_4908,
            0x1fc
        ),
    );

    /// One seed's machine: the fakes gated on, the mailboxes cleared.
    fn oc1017_setup(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        oc1017_install_fakes();
        let r = machine(c, seed, salt, 0);
        oc1017_w(c, OC1017_CTRL, OC1017_MAGIC);
        r
    }

    /// `self->mpMgr` (`*(self+0x118)`) with its config objects, and the
    /// chain links (`self+0x625c` .. `self+0x6c84`) of a seed's subset.
    fn oc1017_manager(c: &mut Cpu, r: &mut Rng, seed: u32) {
        oc1017_w(c, OC1017_SELF + 0x118, OC1017_MGR + 0x100);
        oc1017_w(c, OC1017_MGR + 0x100, OC1017_MGR);
        oc1017_w(c, OC1017_MGR + 0xab0, OC1017_CFG2);
        oc1017_w(c, OC1017_MGR + 0xacc, OC1017_CFG);
        c.bus.write16(OC1017_CFG2 + 0x10, r.next_u32() as u16);
        oc1017_w(c, OC1017_CFG2 + 0x14, fakes::FAKE + 0x20 * (seed % 3));
        oc1017_w(c, OC1017_CFG + 0x300, (seed % 2) as u32);
        oc1017_w(c, OC1017_CFG + 0x304, r.next_u32());
        oc1017_w(c, OC1017_CFG + 0x360, ((seed / 2) % 2) as u32);
        oc1017_w(c, OC1017_CFG + 0x364, r.next_u32());
    }

    /// A chain link object `i`: `link+4` is its vtable, `vt+8` a halfword
    /// delta and `vt+0xc` the function (an address the machine's miss hook
    /// answers).
    fn oc1017_link(c: &mut Cpu, r: &mut Rng, i: u32, seed: u32) -> u32 {
        let link = OC1017_LINK + 0x100 * i;
        let vt = OC1017_VT + 0x100 * i;
        oc1017_w(c, link + 4, vt);
        c.bus.write16(vt + 8, r.next_u32() as u16);
        oc1017_w(c, vt + 0xc, fakes::FAKE + 0x10 * ((seed + i) % 4));
        link
    }

    #[test]
    fn cfe_progressive_verify_on_widget_action_matches() {
        check(
            ee::s06::f_002461e0__generated,
            cfe_progressive_verify_on_widget_action,
            300,
            &|c, seed| {
                let _ = oc1017_setup(c, seed, 0x0024_61e0);
                // The hash the callee returns is 0 on even seeds: the item's
                // hash then matches and the allocation path runs.
                let zero_hash = seed % 2 == 0;
                oc1017_w(c, OC1017_CTRL + 0x100, zero_hash as u32);
                oc1017_w(c, OC1017_OBJ + 0x38, if zero_hash { 0 } else { 0x1234 });
                oc1017_w(c, OC1017_SELF + 0x10, OC1017_OBJ + 0x800);
                let item = if seed % 4 == 0 { 0 } else { OC1017_OBJ };
                c.g[A0][0] = sx(OC1017_SELF);
                c.g[A1][0] = item as u64;
                c.g[A2][0] = if seed % 7 == 0 { 4 } else { 5 };
            },
        );
    }

    #[test]
    fn ssxapp_global_ctors_matches() {
        check(
            ee::s06::f_00247e20__generated,
            ssxapp_global_ctors,
            100,
            &|c, seed| {
                let _ = oc1017_setup(c, seed, 0x0024_7e20);
            },
        );
    }

    #[test]
    fn ssxapp_global_dtors_matches() {
        check(
            ee::s06::f_00247e40__generated,
            ssxapp_global_dtors,
            100,
            &|c, seed| {
                let _ = oc1017_setup(c, seed, 0x0024_7e40);
            },
        );
    }

    #[test]
    fn ssx_audio_system_set_music_event1_matches() {
        check(
            ee::s07::f_002861d0__generated,
            ssx_audio_system_set_music_event1,
            300,
            &|c, seed| {
                let mut r = oc1017_setup(c, seed, 0x0028_61d0);
                oc1017_w(c, OC1017_SELF + 0x5558, OC1017_OBJ);
                c.bus.write16(OC1017_OBJ + 0x20, r.next_u32() as u16);
                oc1017_w(c, OC1017_OBJ + 0x24, fakes::FAKE + 0x10 * (seed % 3));
                c.g[A0][0] = sx(OC1017_SELF);
            },
        );
    }

    #[test]
    fn ssx_audio_system_exit_game_matches() {
        check(
            ee::s07::f_00286a80__generated,
            ssx_audio_system_exit_game,
            300,
            &|c, seed| {
                let mut r = oc1017_setup(c, seed, 0x0028_6a80);
                oc1017_manager(c, &mut r, seed);
                // Each chain link present on the seed's bits.
                let fields = [0x625c, 0x6260, 0x5fa0, 0x5fa4, 0x6c80, 0x6c84];
                for (i, &off) in fields.iter().enumerate() {
                    if (seed >> i) & 1 == 1 {
                        let link = oc1017_link(c, &mut r, i as u32, seed);
                        oc1017_w(c, OC1017_SELF + off, link);
                    }
                }
                c.g[A0][0] = sx(OC1017_SELF);
            },
        );
    }

    #[test]
    fn ssx_audio_system_unload_memory_bank_matches() {
        check(
            ee::s07::f_00286d18__generated,
            ssx_audio_system_unload_memory_bank,
            300,
            &|c, seed| {
                let mut r = oc1017_setup(c, seed, 0x0028_6d18);
                oc1017_manager(c, &mut r, seed);
                let bank = seed % 3;
                let id = 7u32;
                // The handle slot matches unless seed % 5 == 0; the bank's
                // object slot is empty on even seeds.
                let stored = if seed % 5 == 0 { id ^ 1 } else { id };
                oc1017_w(c, OC1017_SELF + 0x6c78 + 4 * bank, stored);
                let obj_slot = if seed % 2 == 0 { 0 } else { OC1017_OBJ };
                oc1017_w(c, OC1017_SELF + 0x6c70 + 4 * bank, obj_slot);
                c.g[A0][0] = sx(OC1017_SELF);
                c.g[A1][0] = ((bank << 8) | id) as u64;
            },
        );
    }

    #[test]
    fn ssx_audio_system_on_quick_restart_matches() {
        check(
            ee::s07::f_002870a0__generated,
            ssx_audio_system_on_quick_restart,
            300,
            &|c, seed| {
                let _ = oc1017_setup(c, seed, 0x0028_70a0);
                // IsEventMode4 is 0 on seeds % 3 == 0 (the chain goes on);
                // IsTrackedSoundPlaying is 0 on even seeds (TrackSound runs).
                oc1017_w(c, OC1017_CTRL + 0x104, (seed % 3 == 0) as u32);
                oc1017_w(c, OC1017_CTRL + 0x108, (seed % 2 == 0) as u32);
                c.g[A0][0] = sx(OC1017_SELF);
            },
        );
    }

    #[test]
    fn ssx_audio_system_start_pre_race_music_matches() {
        check(
            ee::s07::f_002872a8__generated,
            ssx_audio_system_start_pre_race_music,
            300,
            &|c, seed| {
                let _ = oc1017_setup(c, seed, 0x0028_72a8);
                // The game module's config bytes at 0x535bc8+0x48/0x49/0x4a.
                c.bus
                    .write8(0x0053_5c10, [5u8, 6, 0x20][(seed % 3) as usize]);
                c.bus
                    .write8(0x0053_5c12, [4u8, 5, 9][((seed / 3) % 3) as usize]);
                c.bus
                    .write8(0x0053_5c11, [2u8, 1][((seed / 9) % 2) as usize]);
                oc1017_w(
                    c,
                    OC1017_SELF + 0x530,
                    if (seed / 18) % 2 == 0 { 0 } else { 0x1000 },
                );
                oc1017_w(c, OC1017_SELF + 0x5558, OC1017_LINK);
                c.bus.write16(OC1017_LINK + 0x20, (seed * 37) as u16);
                oc1017_w(c, OC1017_LINK + 0x24, fakes::FAKE + 0x10);
                oc1017_w(c, OC1017_CTRL + 0x108, ((seed / 36) % 2 == 0) as u32);
                c.g[A0][0] = sx(OC1017_SELF);
            },
        );
    }
}
