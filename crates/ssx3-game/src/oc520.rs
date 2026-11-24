//! Staging module oc520 (wave wave230): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc520`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0023_7280,
        input_sync_add_remote_pad,
        "InputSync_AddRemotePad",
    ),
    (0x0023_7948, input_sync_empty_hook, "InputSync_EmptyHook"),
    (
        0x0023_8348,
        ov_screen_yn_dialog_copy_default_choice,
        "OVScreenYNDialog_CopyDefaultChoice",
    ),
    (
        0x0023_85d0,
        game_mode_man_save_state,
        "GameModeMan_SaveState",
    ),
    (
        0x0023_88a0,
        game_mode_man_load_state,
        "GameModeMan_LoadState",
    ),
    (
        0x0024_3b18,
        app_module_vt47d818_ctor,
        "AppModuleVt47d818_ctor",
    ),
    (0x0024_3bb0, c_game_load_screen_ctor, "cGameLoadScreen_ctor"),
    (0x0023_b170, gm_handler2_reset, "GMHandler2_Reset"),
];

/// `cInputSync_SyncedPad* InputSync_AddRemotePad(InputSync* self (a0), void*
/// realinputmap (a1))` (0x237280; reference `cInputSync_createremoteinput`).
///
/// Appends a pad record to `self->syncedpads` (the `numsyncedpads`-th, 0x18
/// bytes each, from +0x14). Its sample buffer is `latency * 100` bytes with
/// every sample's `NumButtons` (+0 of each 0x64-byte sample) set to 0x18;
/// the record gets `readpos = 0`, `writepos = sendlimit`, `sourcepad = 0`,
/// a 676-byte destination pad (its first word zeroed) and an 8-byte input
/// object, linked to the pad with `ValuePair_SetFirst/Second`. Returns the
/// input object.
pub fn input_sync_add_remote_pad(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = 0x18;
        frame.save(c, S0, 0x40);
        c.g[A2][0] = 0;
        frame.save(c, S1, 0x30);
        c.g[S0][0] = 0x64;
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[A3][0] = 0;
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A1][0];
        c.g[A1][0] = 0x47_c078; // InputSamples tag
        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x10).lw(c); // numsyncedpads
        c.g[A0][0] = Addr(c.g[S1][0] as u32).off(0x4).lw(c); // latency
        c.mult(0, V1, V0, V1); // v1 = numsyncedpads * 0x18
        c.mult(0, 0, A0, S0); // lo = latency * 0x64
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        let v0 = c.g[V0][0];
        Addr(c.g[S1][0] as u32).off(0x10).sw(c, v0); // numsyncedpads++
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x14));
        c.g[A0][0] = c.lo[0]; // latency * 100
        c.g[RA][0] = sx(0x002372e0);
        c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, 0x00317e30, 0x002372e0)?; // operator_new_named
        let v0 = c.g[V0][0];
        Addr(c.g[S2][0] as u32).sw(c, v0); // syncedPad->samples
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = Addr(c.g[S1][0] as u32).off(0x4).lw(c);
        c.mult(0, 0, A2, S0); // lo = latency * 0x64
        c.g[A2][0] = c.lo[0];
        c.g[RA][0] = sx(0x002372fc);
        c.g[A1][0] = 0;
        call_ptr(c, 0x003e6448, 0x002372fc)?; // EA_MemSet (sample buffer)
        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x4).lw(c);
        let skip = (c.g[V0][0] as i64) <= 0;
        c.g[V1][0] = 0;
        if !skip {
            c.cycles += 2;
            c.g[A1][0] = 0x18;
            c.g[A0][0] = 0;
            loop {
                c.cycles += 8;
                c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                let sample = c.g[V0][0];
                let buttons = c.g[A1][0];
                Addr(sample as u32).sw(c, buttons); // NumButtons = 0x18
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x4).lw(c);
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x64));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x00237310);
                }
            }
        }
        c.cycles += 34;
        Addr(c.g[S2][0] as u32).off(0x4).sw(c, 0); // readpos
        c.g[A1][0] = 0x47_c088; // SyncedPad tag
        c.g[A0][0] = 0x2a4; // sizeof(SyncedPad) = 676
        c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c); // sendlimit
        c.g[A2][0] = 0;
        Addr(c.g[S2][0] as u32).off(0xc).sw(c, 0); // sourcepad
        c.g[A3][0] = 0;
        c.g[RA][0] = sx(0x00237358);
        let v0 = c.g[V0][0];
        Addr(c.g[S2][0] as u32).off(0x8).sw(c, v0); // writepos = sendlimit
        call_ptr(c, 0x00317d70, 0x00237358)?; // EA_MemAllocNamed
        let v0 = c.g[V0][0];
        Addr(c.g[S2][0] as u32).off(0x10).sw(c, v0); // destinationpad
        c.g[A1][0] = 0x47_c098; // SyncedInput tag
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A0][0] = 0x8;
        c.g[RA][0] = sx(0x00237378);
        Addr(c.g[V0][0] as u32).sw(c, 0); // *destinationpad = 0
        call_ptr(c, 0x00317d70, 0x00237378)?; // EA_MemAllocNamed
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = Addr(c.g[S2][0] as u32).off(0x10).lw(c);
        c.g[RA][0] = sx(0x00237388);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x00244400, 0x00237388)?; // ValuePair_SetFirst
        c.g[A1][0] = c.g[S3][0];
        c.g[RA][0] = sx(0x00237394);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x00244408, 0x00237394)?; // ValuePair_SetSecond
        let s0 = c.g[S0][0];
        Addr(c.g[S2][0] as u32).off(0x14).sw(c, s0); // input
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void InputSync_EmptyHook(void)` (0x237948): a hook that does nothing.
pub fn input_sync_empty_hook(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void OVScreenYNDialog_CopyDefaultChoice(OVScreenYNDialog* d (a0))`
/// (0x238348): the field at +0x70 takes the value of +0x74.
pub fn ov_screen_yn_dialog_copy_default_choice(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x74).lw(c);
    let v0 = c.g[V0][0];
    Addr(c.g[A0][0] as u32).off(0x70).sw(c, v0);
    ret(c);
}

/// `void InputSync_SendSample(InputSync* self (a0), const void* src (a1))`
/// (0x237a98): packs `src` into a local buffer (`InputSync_PackSample`),
/// unpacks it into `self` (`InputSync_UnpackSample`), then, when the link is
/// up (`mSyncInput` set, `mError` 0, `mConnected` non-zero), sends the packed
/// bytes with `cGameComm_SendPacket`. The game comm is at `gp-0x204`.
/// Not installed: the test runs the real `cGameComm_SendPacket` away on fixture data.
#[allow(dead_code)]
pub fn input_sync_send_sample(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        c.g[A2][0] = c.g[SP][0];
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0x20);
        c.g[RA][0] = sx(0x00237abc);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, 0x00237b28, 0x00237abc)?; // InputSync_PackSample
        c.g[S2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x00237ad0);
        c.g[A2][0] = c.g[SP][0];
        call_ptr(c, 0x00237be8, 0x00237ad0)?; // InputSync_UnpackSample
                                              // The link state: mSyncInput (+0x40), mError (+0x4), mConnected (+0).
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x204)).lw(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x40)).lw(c);
        let no_sync = c.g[V0][0] == 0;
        c.g[A0][0] = 0;
        if !no_sync {
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x4)).lw(c);
            let error = c.g[V0][0] != 0;
            if !error {
                c.cycles += 2;
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                c.g[A0][0] = (0u64 < c.g[V0][0]) as u64; // mConnected != 0
            }
        }
        c.cycles += 2;
        let link_down = c.g[A0][0] == 0;
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x204)).lw(c);
        if !link_down {
            c.cycles += 4;
            c.g[A3][0] = c.g[S2][0];
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(0x00237b0c);
            c.g[A2][0] = c.g[SP][0];
            call_ptr(c, 0x00257e98, 0x00237b0c)?; // cGameComm_SendPacket
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// Return addresses of the 22 stream transfers and of the handler call, in
/// `GameModeMan_SaveState` (writes) and `GameModeMan_LoadState` (reads).
const GMM_SAVE_RETS: [u32; 22] = [
    0x0023_8604,
    0x0023_8620,
    0x0023_863c,
    0x0023_8658,
    0x0023_8674,
    0x0023_8690,
    0x0023_86ac,
    0x0023_86c8,
    0x0023_86e4,
    0x0023_8704,
    0x0023_8724,
    0x0023_8740,
    0x0023_875c,
    0x0023_8778,
    0x0023_8798,
    0x0023_87b4,
    0x0023_87d0,
    0x0023_87ec,
    0x0023_8808,
    0x0023_8824,
    0x0023_8840,
    0x0023_885c,
];
const GMM_LOAD_RETS: [u32; 22] = [
    0x0023_88d4,
    0x0023_88f0,
    0x0023_890c,
    0x0023_8928,
    0x0023_8944,
    0x0023_8960,
    0x0023_897c,
    0x0023_8998,
    0x0023_89b4,
    0x0023_89d4,
    0x0023_89f4,
    0x0023_8a10,
    0x0023_8a2c,
    0x0023_8a48,
    0x0023_8a68,
    0x0023_8a84,
    0x0023_8aa0,
    0x0023_8abc,
    0x0023_8ad8,
    0x0023_8af4,
    0x0023_8b10,
    0x0023_8b2c,
];

/// One stream transfer of the state passes: the virtual call through the
/// stream object in `s0`, slot `slot` (`this` adjusted by the short at
/// `adj`), with the compiled code's register writes.
pub(crate) fn game_mode_man_stream_call(
    c: &mut Cpu,
    ptr: u64,
    n: u64,
    ret: u32,
    adj: u32,
    slot: u32,
) -> Flow {
    c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
    c.g[A1][0] = ptr;
    c.g[A2][0] = n;
    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(adj).lh(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(slot).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[RA][0] = sx(ret);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// The body shared by `GameModeMan_SaveState` (`load == false`, stream
/// writes) and `GameModeMan_LoadState` (`load == true`, stream reads): 22
/// fields of the manager in order (current handler, game event, AI counts,
/// the character and rank tables, location and heat, the limits, the lock
/// flags), then the current mode handler saves or loads its own state with
/// the same stream. `s0` is the stream, `s1` the manager.
pub(crate) fn game_mode_man_stream_pass(c: &mut Cpu, load: bool) -> Flow {
    let (rets, adj, slot, handler_adj, handler_slot, handler_ret) = if load {
        (&GMM_LOAD_RETS, 0x10, 0x14, 0x58, 0x5c, 0x0023_8b5c)
    } else {
        (&GMM_SAVE_RETS, 0x8, 0xc, 0x50, 0x54, 0x0023_888c)
    };
    let s1_reg = c.g[S1][0];
    let s1 = s1_reg as u32;
    // (field address, byte count) in transfer order; the handler-table
    // entries and the rank tables are global (gp-free) addresses.
    let fields: [(u64, u64); 21] = [
        (sx(s1.wrapping_add(0x4)), 4),
        (s1_reg, 4),
        (sx(s1.wrapping_add(0x8)), 4),
        (sx(s1.wrapping_add(0x10)), 4),
        (sx(s1.wrapping_add(0x14)), 4),
        (sx(s1.wrapping_add(0x18)), 0x28),
        (sx(s1.wrapping_add(0x68)), 4),
        (sx(s1.wrapping_add(0x6c)), 4),
        (sx(s1.wrapping_add(0x70)), 4),
        (0x0053_6730, 0x28), // cGameModeMan_iRanking
        (0x0053_6708, 0x28), // cGameModeMan_iCharRank
        (sx(s1.wrapping_add(0x78)), 4),
        (sx(s1.wrapping_add(0x7c)), 4),
        (sx(s1.wrapping_add(0x80)), 4),
        (0x0053_6640, 0x28), // D_00536640
        (sx(s1.wrapping_add(0x84)), 4),
        (sx(s1.wrapping_add(0x88)), 4),
        (sx(s1.wrapping_add(0x8c)), 4),
        (sx(s1.wrapping_add(0x90)), 4),
        (sx(s1.wrapping_add(0x94)), 4),
        (sx(s1.wrapping_add(0x98)), 4),
    ];
    for (i, (ptr, n)) in fields.into_iter().enumerate() {
        game_mode_man_stream_call(c, ptr, n, rets[i], adj, slot)?;
    }
    // The last field goes through a3 (the stream's vtable), not v1.
    c.g[A3][0] = Addr(c.g[S0][0] as u32).lw(c);
    c.g[A1][0] = sx(s1.wrapping_add(0x9c));
    c.g[A2][0] = 0x4;
    c.g[A0][0] = Addr(c.g[A3][0] as u32).off(adj).lh(c);
    c.g[V1][0] = Addr(c.g[A3][0] as u32).off(slot).lw(c);
    let target = c.g[V1][0] as u32;
    c.g[RA][0] = sx(rets[21]);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, rets[21])?;
    // The current mode handler (cGameModeMan_aHandler[iCurrModeHandler]).
    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x4).lw(c);
    c.g[V1][0] = 0x0053_0000;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6668));
    c.g[A1][0] = c.g[S0][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A2][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(handler_adj).lh(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(handler_slot).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[RA][0] = sx(handler_ret);
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, handler_ret)?;
    Ok(())
}

/// `void GameModeMan_SaveState(GameModeMan* self (a0), cGmStream* data (a1))`
/// (0x2385d0): writes the manager's state to `data` (see
/// [`game_mode_man_stream_pass`]).
pub fn game_mode_man_save_state(c: &mut Cpu) {
    game_mode_man_state_io(c, false)
}

/// `void GameModeMan_LoadState(GameModeMan* self (a0), cGmStream* data (a1))`
/// (0x2388a0): reads the manager's state from `data` (see
/// [`game_mode_man_stream_pass`]).
pub fn game_mode_man_load_state(c: &mut Cpu) {
    game_mode_man_state_io(c, true)
}

/// The frame shared by the two state functions.
fn game_mode_man_state_io(c: &mut Cpu, load: bool) {
    guest(c, |c| {
        c.cycles += 180;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0x4;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        game_mode_man_stream_pass(c, load)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cLoadScreen_cLoadScreen` (0x243b18; `AppModuleVt47d818_ctor`): the base
/// `AppModule` constructor, then the load-screen vtable at +0x0 and a zero
/// word at +0xc. Returns the object.
pub fn app_module_vt47d818_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x00243b2c);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x00231cd0, 0x00243b2c)?; // AppModule_ctor
        c.g[V1][0] = 0x0048_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x27e8)); // cLoadScreen_vtbl
        Addr(c.g[S0][0] as u32).off(0xc).sw(c, 0);
        let vtbl = c.g[V1][0];
        Addr(c.g[S0][0] as u32).sw(c, vtbl);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cGameLoadScreen_cGameLoadScreen` (0x243bb0; `cGameLoadScreen_ctor`): the
/// base `AppModule` constructor, then the game-load-screen vtable at +0x0,
/// zero words at +0xc, +0x10 and +0x14. Returns the object.
pub fn c_game_load_screen_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x00243bc4);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x00231cd0, 0x00243bc4)?; // AppModule_ctor
        c.g[V1][0] = 0x0048_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2878)); // cGameLoadScreen_vtbl
        Addr(c.g[S0][0] as u32).off(0xc).sw(c, 0);
        let vtbl = c.g[V1][0];
        Addr(c.g[S0][0] as u32).sw(c, vtbl);
        Addr(c.g[S0][0] as u32).off(0x10).sw(c, 0);
        Addr(c.g[S0][0] as u32).off(0x14).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void GameModeHandler2::Reset(GameModeHandler2* self (a0))` (0x23b170;
/// `GMHandler2_Reset`): the manager starts a new single-heat event. Sets the
/// heat and the AI counts, clears the character table (-1) and gives the
/// human rider its character, resets the rank and points (through the
/// handler's virtual `cleanRiderRanksPoints`), then the limits and flags.
pub fn gm_handler2_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 62;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0023b194);
        c.g[S2][0] = 0x1;
        call_ptr(c, 0x0014dc80, 0x0023b194)?; // cBE_getBE
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0023b1a0);
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0014dd58, 0x0023b1a0)?; // BELibrary_GetInterface
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x480)).lw(c);
        c.g[A2][0] = 0x28;
        c.g[S1][0] = c.g[V0][0];
        c.g[A1][0] = 0xffff_ffff_ffff_ffff;
        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x18));
        let one = c.g[S2][0];
        let gm = c.g[V1][0] as u32;
        Addr(gm).sw(c, one); // iHeatNum
        Addr(gm).off(0x98).sw(c, one); // bIsLastHeat
        Addr(gm).off(0x10).sw(c, 0); // iNumberAI
        c.g[RA][0] = sx(0x0023b1c8);
        Addr(gm).off(0x14).sw(c, 0); // iSimAI
        call_ptr(c, 0x00416210, 0x0023b1c8)?; // memset(iCharacters, -1, 40)
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x0023b1d4);
        c.g[A1][0] = 0;
        call_ptr(c, 0x00146e98, 0x0023b1d4)?; // BENewPlayerInt_GetHumanRider
        c.g[A0][0] = c.g[S1][0];
        c.g[S0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0023b1e4);
        c.g[A1][0] = 0;
        call_ptr(c, 0x00146e98, 0x0023b1e4)?; // BENewPlayerInt_GetHumanRider
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x0023b1f0);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x00147398, 0x0023b1f0)?; // BENewPlayerInt_GetCharacter
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x480)).lw(c);
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
        let char_id = c.g[V0][0];
        Addr(c.g[V1][0] as u32).off(0x18).sw(c, char_id); // iCharacters[humanRider]
        c.g[V0][0] = Addr(c.g[S3][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x20).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x24).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0023b214);
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023b214)?; // cleanRiderRanksPoints (virtual)
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x480)).lw(c);
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        frame.restore(c, S0, 0x40);
        let gm2 = c.g[V0][0] as u32;
        Addr(gm2).off(0x9c).sw(c, one); // iNextHeat
        Addr(gm2).off(0x70).sw(c, one); // iRestoreHeat
        Addr(gm2).off(0x74).sw(c, one); // bAreResultsLocked
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        let minus_one = c.g[V1][0];
        Addr(gm2).off(0x6c).sw(c, minus_one); // iNextLocation
        Addr(gm2).off(0x78).sw(c, 0); // uiTimeLimit
        Addr(gm2).off(0x7c).sw(c, 0); // uiPointTarget
        Addr(gm2).off(0x80).sw(c, 0); // unk_80
        Addr(gm2).off(0x84).sw(c, 0); // bIsEventDone
        Addr(gm2).off(0x88).sw(c, 0); // bIsTimedEvent
        Addr(gm2).off(0x8c).sw(c, 0); // bHasPointTarget
        Addr(gm2).off(0x90).sw(c, 0); // unk_90
        Addr(gm2).off(0x94).sw(c, 0); // unk_94
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes::machine, DATA};

    /// Write one 32-bit word (the fixtures' shorthand).
    pub(crate) fn oc520_w(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    #[test]
    fn input_sync_add_remote_pad_matches() {
        check(
            ssx3_ee::s06::f_00237280__generated,
            input_sync_add_remote_pad,
            120,
            &|c, seed| {
                let _ = machine(c, seed, 0x237280, 0);
                crate::game::be::beintstat::tests::allocator(c);
                c.pc = 0x0023_7280;
                let obj = DATA + 0x100;
                // latency -1..=3 (0 and below skip the sample loop), 0..=3 pads.
                oc520_w(c, obj, seed * 7 + 3);
                oc520_w(c, obj + 0x4, ((seed % 5) as i32 - 1) as u32);
                oc520_w(c, obj + 0x10, seed % 4);
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
            },
        );
    }

    #[test]
    fn input_sync_empty_hook_matches() {
        check(
            ssx3_ee::s06::f_00237948__generated,
            input_sync_empty_hook,
            8,
            &|c, seed| {
                let _ = machine(c, seed, 0x237948, 0);
                c.pc = 0x0023_7948;
            },
        );
    }

    #[test]
    fn ov_screen_yn_dialog_copy_default_choice_matches() {
        check(
            ssx3_ee::s06::f_00238348__generated,
            ov_screen_yn_dialog_copy_default_choice,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x238348, 0);
                let d = DATA + 0x100;
                oc520_w(c, d + 0x74, seed.wrapping_mul(0x9e37_79b9));
                oc520_w(c, d + 0x70, seed ^ 0x55);
                c.g[A0][0] = d as u64;
                c.pc = 0x0023_8348;
            },
        );
    }

    /// A stream and a handler for the state functions: every transfer goes to
    /// the fake (both vtables' read and write slots), with random manager fields.
    pub(crate) fn game_mode_man_state_setup(c: &mut Cpu, seed: u32) {
        use crate::difftest::fakes::FAKE;
        use crate::difftest::fill;
        let mut r = machine(c, seed, 0x2385d0, 0);
        let me = DATA + 0x100;
        let stream = DATA + 0x1000;
        let stream_vt = DATA + 0x2000;
        let handler = DATA + 0x3000;
        let handler_vt = DATA + 0x3100;
        fill(c, &mut r, me, 0x40);
        oc520_w(c, stream, stream_vt);
        oc520_w(c, stream_vt + 0xc, FAKE);
        oc520_w(c, stream_vt + 0x14, FAKE);
        oc520_w(c, me + 0x4, seed % 4); // iCurrModeHandler
        for i in 0..4 {
            oc520_w(c, 0x0053_6668 + 4 * i, handler);
        }
        oc520_w(c, handler, handler_vt);
        oc520_w(c, handler_vt + 0x54, FAKE);
        oc520_w(c, handler_vt + 0x5c, FAKE);
        c.g[A0][0] = me as u64;
        c.g[A1][0] = stream as u64;
    }

    #[test]
    fn game_mode_man_save_state_matches() {
        check(
            ssx3_ee::s06::f_002385d0__generated,
            game_mode_man_save_state,
            60,
            &|c, seed| {
                game_mode_man_state_setup(c, seed);
                c.pc = 0x0023_85d0;
            },
        );
    }

    #[test]
    fn game_mode_man_load_state_matches() {
        check(
            ssx3_ee::s06::f_002388a0__generated,
            game_mode_man_load_state,
            60,
            &|c, seed| {
                game_mode_man_state_setup(c, seed);
                c.pc = 0x0023_88a0;
            },
        );
    }

    #[test]
    fn app_module_vt47d818_ctor_matches() {
        check(
            ssx3_ee::s06::f_00243b18__generated,
            app_module_vt47d818_ctor,
            20,
            &|c, seed| {
                let mut r = machine(c, seed, 0x243b18, 0);
                let obj = DATA + 0x100;
                crate::difftest::fill(c, &mut r, obj, 8);
                c.g[A0][0] = obj as u64;
                c.pc = 0x0024_3b18;
            },
        );
    }

    #[test]
    fn c_game_load_screen_ctor_matches() {
        check(
            ssx3_ee::s06::f_00243bb0__generated,
            c_game_load_screen_ctor,
            20,
            &|c, seed| {
                let mut r = machine(c, seed, 0x243bb0, 0);
                let obj = DATA + 0x100;
                crate::difftest::fill(c, &mut r, obj, 8);
                c.g[A0][0] = obj as u64;
                c.pc = 0x0024_3bb0;
            },
        );
    }

    #[test]
    fn gm_handler2_reset_matches() {
        use crate::difftest::fakes::FAKE;
        use crate::difftest::{fill, GAME_GP};
        check(
            ssx3_ee::s06::f_0023b170__generated,
            gm_handler2_reset,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x23b170, 0);
                // The back end's singleton (gp-0x1f04) is a zeroed dummy.
                oc520_w(c, GAME_GP - 0x1f04, DATA + 0x6_0000);
                oc520_w(c, GAME_GP - 0x1efc, 0);
                let gm = DATA + 0x400;
                let me = DATA + 0x800;
                let vt = DATA + 0x900;
                fill(c, &mut r, gm, 0x30);
                oc520_w(c, GAME_GP - 0x480, gm);
                oc520_w(c, me, vt);
                oc520_w(c, vt + 0x24, FAKE); // cleanRiderRanksPoints: a fake
                fill(c, &mut r, me + 4, 4);
                c.g[A0][0] = me as u64;
                c.pc = 0x0023_b170;
            },
        );
    }
}
