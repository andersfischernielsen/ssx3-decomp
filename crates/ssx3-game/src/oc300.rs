//! Staging wave 211 (oc300): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi;
use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0035_4648, game_process_ctor, "GameProcess_ctor"),
    (0x0039_1418, vu0_load_regs_and_call, "Vu0_LoadRegsAndCall"),
    (
        0x0035_5b90,
        mod_obj_add_uv_scroll_modifier,
        "ModObj_AddUVScrollModifier",
    ),
    (
        0x0029_2508,
        ssx_audio_system_start_board_edge_loop,
        "SSXAudioSystem_StartBoardEdgeLoop",
    ),
    (0x0031_46d0, anim_seq_list_fade_out_all, "AnimSeqList_FadeOutAll"),
    (0x0036_0840, sync_process_id_equals, "SyncProcess_IdEquals"),
    (0x0027_07e0, replay_on_rider_launch, "Replay_OnRiderLaunch"),
    (
        0x002f_ff00,
        wscript_fn_play_object_sound,
        "WScriptFn_PlayObjectSound",
    ),
    (
        0x0023_5990,
        game_state_reset_save_state,
        "GameStateReset_SaveState",
    ),
];

// ---------------------------------------------------------------------------
// GameProcess_ctor (0x354648)

/// The `GameProcess` vtable (0x491f00).
const OC300_GAME_PROCESS_VTABLE: u32 = 0x0049_1f00;
/// Process field: its vtable pointer.
const OC300_PROCESS_VTABLE: u32 = 0xc;
/// The global process list (`gp + 0x2898`).
const OC300_PROCESS_LIST: u32 = 0x2898;

/// `ProcessList_Add` (0x354c08).
const OC300_PROCESS_LIST_ADD: Callee = Callee {
    addr: 0x0035_4c08,
    f: ee::s11::ProcessList_Add,
};

/// `GameProcess* GameProcess_ctor(GameProcess* self, int list)` (0x354648):
/// installs the `GameProcess` vtable and adds the process to the global
/// process list (`ProcessList_Add(&g_processList, self, list)`); returns
/// `self`.
pub fn game_process_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[V0][0] = sx(OC300_GAME_PROCESS_VTABLE);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[A1][0];
        Addr(r32(c, S0)).off(OC300_PROCESS_VTABLE).sw(c, c.g[V0][0]);
        c.g[A0][0] = sx(r32(c, GP).wrapping_add(OC300_PROCESS_LIST));
        c.g[A1][0] = c.g[S0][0];
        OC300_PROCESS_LIST_ADD.call(c, 0x0035_4674)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Vu0_LoadRegsAndCall (0x391418)

/// `void Vu0_LoadRegsAndCall(void* unused, const Vec4 regs[16])` (0x391418):
/// loads VF1..VF16 from the 16 quadwords at `regs`, sets VI27 to the
/// microprogram address 0 and runs the VU0 microprogram there
/// (`vcallmsr vi27`). Leaves v0 = 0.
pub fn vu0_load_regs_and_call(c: &mut Cpu) {
    c.cycles += 25;
    let regs = r32(c, A1);
    for vf in 1..=16u32 {
        c.lqc2(vf as usize, regs.wrapping_add((vf - 1) * 0x10));
    }
    // The microprogram address, as VI27 expects it (byte address >> 3).
    c.g[V0][0] = 0;
    c.ctc2(V0, 27);
    c.vu0().vnop();
    c.vu0().vnop();
    c.vcallmsr();
    ret(c);
}

// ---------------------------------------------------------------------------
// ModObj_AddUVScrollModifier (0x355b90)

/// `cMemMan_alloc(size, tag, flags, 0)` (0x317d70), reached through the
/// function table.
const OC300_MEM_ALLOC: u32 = 0x0031_7d70;
/// `UVScrollModifier_ctor(mem, desc)` (0x35f6e8), through the table.
const OC300_UV_SCROLL_MODIFIER_CTOR: u32 = 0x0035_f6e8;
/// The allocation tag of UV-scroll modifiers (0x48e940).
const OC300_UV_SCROLL_ALLOC_TAG: u32 = 0x0048_e940;
/// Size of a `UVScrollModifier`.
const OC300_UV_SCROLL_MODIFIER_SIZE: u64 = 0x50;

/// `ModObj_SetUVScrollModifier` (0x355550).
const OC300_SET_UV_SCROLL_MODIFIER: Callee = Callee {
    addr: 0x0035_5550,
    f: ee::s11::ModObj_SetUVScrollModifier,
};

/// `void ModObj_AddUVScrollModifier(cMoveNode* self, void* desc)`
/// (0x355b90): allocates a UV-scroll modifier (0x50 bytes), constructs it
/// from `desc` and installs it in the node's modifier block
/// (`ModObj_SetUVScrollModifier`).
pub fn mod_obj_add_uv_scroll_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0x2000_0000;
        frame.save(c, S0, 0x20);
        c.g[A3][0] = 0;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = OC300_UV_SCROLL_MODIFIER_SIZE;
        c.g[A1][0] = sx(OC300_UV_SCROLL_ALLOC_TAG);
        call_ptr(c, OC300_MEM_ALLOC, 0x0035_5bc0)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, OC300_UV_SCROLL_MODIFIER_CTOR, 0x0035_5bcc)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[V0][0];
        OC300_SET_UV_SCROLL_MODIFIER.call(c, 0x0035_5bd8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_StartBoardEdgeLoop (0x292508)

/// `SSXAudioSystem` fields: the bank monitor (`+0`), the push tables of
/// bank and sample per queue entry (`+4`, `+8`), the sound root (`+0x118`,
/// `*root` is the monitor too).
const OC300_SYS_MONITOR: u32 = 0x0;
const OC300_SYS_PUSH_BANK: u32 = 0x4;
const OC300_SYS_PUSH_SAMPLE: u32 = 0x8;
const OC300_SYS_SND_ROOT: u32 = 0x118;
/// Rider audio fields: human (`+0x874`), local (`+0x87c`), listener index
/// (`+0x870`); the rider's sound position at `+0x110`.
const OC300_RIDER_HUMAN: u32 = 0x874;
const OC300_RIDER_LOCAL: u32 = 0x87c;
const OC300_RIDER_LISTENER: u32 = 0x870;
const OC300_RIDER_SOUND_POS: u32 = 0x110;
/// Bank monitor: the push queue (`+0x1d8`) and its entry count (`+0x1f4`,
/// = queue `+0x1c`); the 24-byte entry template at `+0x1dc`; per-entry
/// handles inline at `+0x1fc`.
const OC300_MON_QUEUE: u32 = 0x1d8;
const OC300_MON_COUNT: u32 = 0x1f4;
const OC300_MON_HANDLES: u32 = 0x1fc;
/// The game-mode byte (2: replays possible).
const OC300_MODE_BYTE: u32 = 0x0053_5c11;
/// The 8-byte callback key copied into the entry's slot (0x4a36a0).
const OC300_EDGE_KEY: u32 = 0x004a_36a0;

/// `SSXAudioSystem_IsReplayPlaying` (0x288ae0).
const OC300_IS_REPLAY_PLAYING: Callee = Callee {
    addr: 0x0028_8ae0,
    f: ee::s07::SSXAudioSystem_IsReplayPlaying,
};
/// `SSXAudioSystem_GetSurfaceBank` (0x2917b8).
const OC300_GET_SURFACE_BANK: Callee = Callee {
    addr: 0x0029_17b8,
    f: ee::s07::SSXAudioSystem_GetSurfaceBank,
};
/// `SSXAudioSystem_GetChannelGainPtr` (0x287968).
const OC300_GET_CHANNEL_GAIN_PTR: Callee = Callee {
    addr: 0x0028_7968,
    f: ee::s07::SSXAudioSystem_GetChannelGainPtr,
};
/// `SSXAudioSystem_GetRiderListenerIndex` (0x288a20).
const OC300_GET_RIDER_LISTENER_INDEX: Callee = Callee {
    addr: 0x0028_8a20,
    f: ee::s07::SSXAudioSystem_GetRiderListenerIndex,
};
/// `Audio_PlayPushedSound` (0x2906b8).
const OC300_PLAY_PUSHED_SOUND: Callee = Callee {
    addr: 0x0029_06b8,
    f: ee::s07::Audio_PlayPushedSound,
};

/// `table[count] = value` on the monitor's push queue: `cnt = monitor->count;
/// tab = queue[off]; *(tab + (cnt << shift) + extra) = value`, in the
/// registers the original uses.
#[allow(clippy::too_many_arguments)]
fn oc300_edge_store(
    c: &mut Cpu,
    (cnt, tab): (usize, usize),
    monitor: u32,
    queue: u32,
    off: u32,
    shift: u32,
    extra: u32,
    value: u64,
) {
    c.g[cnt][0] = Addr(monitor).off(OC300_MON_COUNT).lw(c);
    c.g[tab][0] = Addr(queue).off(off).lw(c);
    c.g[cnt][0] = sx((c.g[cnt][0] as u32) << shift);
    c.g[cnt][0] = sx((c.g[cnt][0] as u32).wrapping_add(c.g[tab][0] as u32));
    Addr(c.g[cnt][0] as u32).off(extra).sw(c, value);
}

/// `*(lw(lw(sys->sndRoot) + off) + count * 4) = value` (the root monitor's
/// table `off` at the current entry), in the registers the original uses:
/// `root`, `mon`, `cnt`, `tab`.
fn oc300_edge_root_store(c: &mut Cpu, regs: [usize; 4], off: u32, value: u64) {
    let [root, mon, cnt, tab] = regs;
    c.g[root][0] = Addr(r32(c, S5)).off(OC300_SYS_SND_ROOT).lw(c);
    c.g[mon][0] = Addr(c.g[root][0] as u32).lw(c);
    c.g[cnt][0] = Addr(c.g[mon][0] as u32).off(OC300_MON_COUNT).lw(c);
    c.g[tab][0] = Addr(c.g[mon][0] as u32).off(off).lw(c);
    c.g[cnt][0] = sx((c.g[cnt][0] as u32) << 2);
    c.g[cnt][0] = sx((c.g[cnt][0] as u32).wrapping_add(c.g[tab][0] as u32));
    Addr(c.g[cnt][0] as u32).sw(c, value);
}

/// `void SSXAudioSystem_StartBoardEdgeLoop(SSXAudioSystem* sys, Rider*
/// rider, int surface)` (0x292508): for a human local rider, push the
/// board-edge loop on the bank monitor's push queue (bank from
/// `GetSurfaceBank(sys, surface)`, sample `surface * 8 + 3`; volume 100,
/// pan 90, 50, 127), clamp its volume to 0..127, attach the channel-6 gain,
/// the rider's sound position, callback key and listener index, set radius
/// 100 / vanish -1; during a replay (mode byte 2) mark it as a replay sound
/// of the rider's listener. Then `Audio_PlayPushedSound(sys)`.
pub fn ssx_audio_system_start_board_edge_loop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xa0);
        c.g[V1][0] = 0;
        frame.save(c, S2, 0x70);
        frame.save(c, S5, 0x40);
        c.g[S2][0] = c.g[A2][0]; // surface
        frame.save(c, S6, 0x30);
        frame.save(c, S0, 0x90);
        c.g[S6][0] = c.g[A1][0]; // rider
        frame.save(c, S1, 0x80);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S7, 0x20);
        frame.save_ra(c, 0x10);
        let rider = r32(c, S6);
        c.g[V0][0] = Addr(rider).off(OC300_RIDER_HUMAN).lw(c);
        c.g[S5][0] = c.g[A0][0]; // sys
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = Addr(rider).off(OC300_RIDER_LOCAL).lw(c);
            c.g[V1][0] = (c.g[V0][0] != 0) as u64;
        }
        c.cycles += 2;
        let play = c.g[V1][0] != 0;
        frame.restore(c, S0, 0x90);
        if play {
            oc300_edge_push_and_play(c)?;
            frame.restore(c, S0, 0x90);
        }
        c.cycles += 10;
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// The push-and-play body of `StartBoardEdgeLoop` (s5 = sys, s6 = rider,
/// s2 = surface).
fn oc300_edge_push_and_play(c: &mut Cpu) -> crate::call::Flow {
    // In a replay-capable mode, remember whether no replay is playing.
    c.cycles += 10;
    c.g[S7][0] = 0;
    call_ptr(c, 0x0014_dc80, 0x0029_2560)?; // BELibrary_GetInstance
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, 0x0014_dd58, 0x0029_256c)?; // BELibrary_GetInterface(be, 0)
    c.g[V1][0] = 0x53_0000;
    c.g[A0][0] = 2;
    c.g[V0][0] = Addr(OC300_MODE_BYTE).lb(c);
    if c.g[V0][0] != c.g[A0][0] {
        c.g[A0][0] = c.g[S5][0];
    } else {
        c.cycles += 4;
        c.g[A0][0] = c.g[S5][0];
        OC300_IS_REPLAY_PLAYING.call(c, 0x0029_2588)?;
        c.g[S7][0] = (c.g[V0][0] < 1) as u64;
        c.g[A0][0] = c.g[S5][0];
    }

    // bank = GetSurfaceBank(sys, surface).
    c.cycles += 159;
    c.g[A1][0] = c.g[S2][0];
    OC300_GET_SURFACE_BANK.call(c, 0x0029_2598)?;
    // New queue entry: count += 1, copy the 24-byte template into the
    // entry's record (record size 0x18 in s3).
    c.g[S3][0] = 0x18;
    c.g[S0][0] = Addr(r32(c, S5)).off(OC300_SYS_MONITOR).lw(c);
    c.g[S4][0] = c.g[V0][0]; // bank
    c.g[S2][0] = sx(r32(c, S2) << 3);
    c.g[A1][0] = 0;
    let monitor = r32(c, S0);
    c.g[V0][0] = Addr(monitor).off(OC300_MON_COUNT).lw(c);
    c.g[S1][0] = sx(monitor.wrapping_add(OC300_MON_QUEUE));
    let queue = r32(c, S1);
    c.g[S2][0] = sx(r32(c, S2).wrapping_add(3)); // sample = surface * 8 + 3
    c.g[A2][0] = 0x8;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    Addr(monitor).off(OC300_MON_COUNT).sw(c, c.g[V0][0]);
    c.g[V1][0] = Addr(monitor).off(OC300_MON_COUNT).lw(c);
    c.g[V0][0] = Addr(queue).off(0x20).lw(c);
    c.mult(0, 3, 3, 19);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.ldl(2, monitor.wrapping_add(0x1e3));
    c.ldr(2, monitor.wrapping_add(0x1dc));
    c.ldl(4, monitor.wrapping_add(0x1eb));
    c.ldr(4, monitor.wrapping_add(0x1e4));
    c.ldl(7, monitor.wrapping_add(0x1f3));
    c.ldr(7, monitor.wrapping_add(0x1ec));
    let record = r32(c, V1);
    c.sdl(2, record.wrapping_add(0x7));
    c.sdr(2, record);
    c.sdl(4, record.wrapping_add(0xf));
    c.sdr(4, record.wrapping_add(0x8));
    c.sdl(7, record.wrapping_add(0x17));
    c.sdr(7, record.wrapping_add(0x10));
    // monitor->handles[count] = 0.
    c.g[V0][0] = Addr(monitor).off(OC300_MON_COUNT).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx(monitor.wrapping_add(c.g[V0][0] as u32));
    Addr(r32(c, V0)).off(OC300_MON_HANDLES).sw(c, 0);
    oc300_edge_store(c, (V1, V0), monitor, queue, 0x34, 2, 0, 0);
    // table 0x38 [count] = the entry record's first byte (sign-extended).
    c.g[A3][0] = Addr(monitor).off(OC300_MON_COUNT).lw(c);
    c.g[A0][0] = Addr(monitor).off(OC300_MON_COUNT).lw(c);
    c.g[T0][0] = Addr(queue).off(0x20).lw(c);
    c.g[A3][0] = sx(r32(c, A3) << 2);
    c.mult(0, 4, 4, 19);
    c.g[V0][0] = Addr(queue).off(0x38).lw(c);
    c.g[A3][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, T0)));
    c.g[V1][0] = Addr(r32(c, A0)).lb(c);
    Addr(r32(c, A3)).sw(c, c.g[V1][0]);
    oc300_edge_store(c, (V0, V1), monitor, queue, 0x3c, 2, 0, 0);
    // memset(&queue->slots[count] (8 bytes at +0x40), 0, 8).
    c.g[A0][0] = Addr(monitor).off(OC300_MON_COUNT).lw(c);
    c.g[A0][0] = sx(r32(c, A0) << 3);
    c.g[A0][0] = sx(queue.wrapping_add(r32(c, A0)));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x40));
    call_ptr(c, 0x0041_6210, 0x0029_2678)?;

    // The entry's parameters (volume 100, pan 90/50/127, flag 1).
    let monitor = r32(c, S0);
    let queue = r32(c, S1);
    c.g[A0][0] = 0x64;
    c.g[A1][0] = 0x5a;
    c.g[A2][0] = 0x32;
    c.g[A3][0] = 0x7f;
    oc300_edge_store(c, (V1, V0), monitor, queue, 0x60, 2, 0, 0);
    c.g[T0][0] = 0x1;
    oc300_edge_store(c, (V0, V1), monitor, queue, 0x64, 2, 0, 0);
    oc300_edge_store(c, (V1, V0), monitor, queue, 0x68, 2, 0, 0);
    oc300_edge_store(c, (V0, V1), monitor, queue, 0x6c, 2, 0, 0);
    oc300_edge_store(c, (V1, V0), monitor, queue, 0x70, 4, 0x4, 0);
    oc300_edge_store(c, (V0, V1), monitor, queue, 0x70, 4, 0x0, 0x64);
    oc300_edge_store(c, (V1, V0), monitor, queue, 0x70, 4, 0x8, 0x5a);
    oc300_edge_store(c, (V0, V1), monitor, queue, 0x70, 4, 0xc, 0x32);
    oc300_edge_store(c, (V1, V0), monitor, queue, 0x74, 2, 0, 0);
    oc300_edge_store(c, (V0, V1), monitor, queue, 0x80, 2, 0, 0);
    oc300_edge_store(c, (V1, V0), monitor, queue, 0x84, 2, 0, 0);
    oc300_edge_store(c, (V0, V1), monitor, queue, 0x78, 2, 0, 0x7f);
    oc300_edge_store(c, (V1, V0), monitor, queue, 0x7c, 2, 0, 1);

    // sys->pushBank[count] = bank (s4); sys->pushSample[count] = sample (s2).
    let sys = r32(c, S5);
    c.g[A0][0] = Addr(sys).off(OC300_SYS_MONITOR).lw(c);
    c.g[V1][0] = Addr(sys).off(OC300_SYS_PUSH_BANK).lw(c);
    c.g[V0][0] = Addr(r32(c, A0)).off(OC300_MON_COUNT).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
    Addr(r32(c, V0)).sw(c, c.g[S4][0]);
    c.g[V1][0] = Addr(sys).off(OC300_SYS_MONITOR).lw(c);
    c.g[A0][0] = Addr(sys).off(OC300_SYS_PUSH_SAMPLE).lw(c);
    c.g[V0][0] = Addr(r32(c, V1)).off(OC300_MON_COUNT).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
    Addr(r32(c, V0)).sw(c, c.g[S2][0]);

    // vals[count] (root queue table 0x38) := 0, then the entry record's
    // first byte := clamp(vals[count], 0, 127).
    c.g[V1][0] = Addr(sys).off(OC300_SYS_SND_ROOT).lw(c);
    c.g[A1][0] = Addr(r32(c, V1)).lw(c);
    let root_mon = r32(c, A1);
    c.g[V0][0] = Addr(root_mon).off(OC300_MON_COUNT).lw(c);
    c.g[A2][0] = sx(root_mon.wrapping_add(OC300_MON_QUEUE));
    let root_queue = r32(c, A2);
    c.g[V1][0] = Addr(root_queue).off(0x38).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
    Addr(r32(c, V0)).sw(c, 0);
    c.g[A0][0] = Addr(root_mon).off(OC300_MON_COUNT).lw(c);
    c.g[V1][0] = Addr(root_queue).off(0x1c).lw(c);
    c.g[A3][0] = Addr(root_queue).off(0x38).lw(c);
    c.mult(0, 4, 4, 19);
    c.g[V1][0] = sx(r32(c, V1) << 2);
    c.g[A1][0] = Addr(root_queue).off(0x20).lw(c);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A3)));
    c.g[V0][0] = Addr(r32(c, V1)).lw(c);
    c.g[V0][0] = ((c.g[V0][0] as i64) < 127) as u64;
    let below_max = c.g[V0][0] != 0;
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
    // The current volume `vals[cur]` (queue + 0x1c is the count).
    let volume_at = |c: &mut Cpu| {
        c.g[V0][0] = Addr(root_queue).off(0x1c).lw(c);
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A3)));
    };
    let mut negative = false;
    if below_max {
        c.cycles += 6;
        volume_at(c);
        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
        negative = (c.g[V1][0] as i64) < 0;
        c.g[V0][0] = 0;
    }
    if !negative {
        c.cycles += 7;
        volume_at(c);
        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
        c.g[V1][0] = ((c.g[V1][0] as i64) < 127) as u64;
        if c.g[V1][0] == 0 {
            c.g[V0][0] = 0x7f;
        } else {
            c.cycles += 4;
            volume_at(c);
            c.g[V0][0] = Addr(r32(c, V0)).lbu(c);
        }
    }
    c.cycles += 69;
    Addr(r32(c, A0)).sb(c, c.g[V0][0]);

    // root->gains[count] = GetChannelGainPtr(sys, 6, 0).
    c.g[A2][0] = 0;
    c.g[A0][0] = c.g[S5][0];
    c.g[A1][0] = 0x6;
    c.g[S0][0] = Addr(r32(c, S5)).off(OC300_SYS_SND_ROOT).lw(c);
    OC300_GET_CHANNEL_GAIN_PTR.call(c, 0x0029_2868)?;
    c.g[A3][0] = Addr(r32(c, S0)).lw(c);
    c.g[T0][0] = sx(r32(c, S6).wrapping_add(OC300_RIDER_SOUND_POS));
    c.g[T1][0] = 0x4a_0000;
    c.g[A0][0] = c.g[S5][0];
    let mon = r32(c, A3);
    c.g[V1][0] = Addr(mon).off(OC300_MON_COUNT).lw(c);
    c.g[A1][0] = c.g[S6][0];
    c.g[A2][0] = Addr(mon).off(0x214).lw(c);
    c.g[V1][0] = sx(r32(c, V1) << 2);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A2)));
    Addr(r32(c, V1)).sw(c, c.g[V0][0]);
    // root->handles[count] = &rider->soundPos.
    c.g[A2][0] = Addr(r32(c, S5)).off(OC300_SYS_SND_ROOT).lw(c);
    c.g[V1][0] = Addr(r32(c, A2)).lw(c);
    c.g[V0][0] = Addr(r32(c, V1)).off(OC300_MON_COUNT).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    Addr(r32(c, V1)).off(OC300_MON_HANDLES).sw(c, c.g[T0][0]);
    // The callback key (8 bytes at 0x4a36a0, through the stack) into the
    // entry's slot (queue + 0x40 + count * 8).
    c.g[V0][0] = Addr(r32(c, S5)).off(OC300_SYS_SND_ROOT).lw(c);
    c.g[A2][0] = Addr(r32(c, V0)).lw(c);
    c.g[T0][0] = sx(OC300_EDGE_KEY);
    c.ldl(2, OC300_EDGE_KEY.wrapping_add(0x7));
    c.ldr(2, OC300_EDGE_KEY);
    let stack = sp(c);
    c.sdl(2, stack.wrapping_add(0x7));
    c.sdr(2, stack);
    let mon = r32(c, A2);
    c.g[V1][0] = sx(mon.wrapping_add(OC300_MON_QUEUE));
    c.g[V0][0] = Addr(mon).off(OC300_MON_COUNT).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 3);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    c.ldl(2, stack.wrapping_add(0x7));
    c.ldr(2, stack);
    let slot = r32(c, V1);
    c.sdl(2, slot.wrapping_add(0x47));
    c.sdr(2, slot.wrapping_add(0x40));
    // root table 0x254 [count] = 0; root table 0x238 [count] = rider.
    oc300_edge_root_store(c, [A2, V1, V0, A2], 0x254, 0);
    let rider = c.g[S6][0];
    oc300_edge_root_store(c, [V1, A2, V0, V1], 0x238, rider);
    // root table 0x23c [count] = GetRiderListenerIndex(sys, rider).
    c.g[S0][0] = Addr(r32(c, S5)).off(OC300_SYS_SND_ROOT).lw(c);
    OC300_GET_RIDER_LISTENER_INDEX.call(c, 0x0029_2924)?;
    c.g[A0][0] = Addr(r32(c, S0)).lw(c);
    c.g[AT][0] = 0x42c8_0000;
    c.mtc1(1, 12); // radius 100.0
    let mon = r32(c, A0);
    c.g[V1][0] = Addr(mon).off(OC300_MON_COUNT).lw(c);
    c.g[A1][0] = Addr(mon).off(0x23c).lw(c);
    c.g[V1][0] = sx(r32(c, V1) << 2);
    c.g[AT][0] = sx(0xbf80_0000);
    c.mtc1(1, 13); // vanish -1.0
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
    Addr(r32(c, V1)).sw(c, c.g[V0][0]);
    // SndPlayParams_SetRadiusVanish(&root->queue, 100.0, -1.0).
    c.g[V0][0] = Addr(r32(c, S5)).off(OC300_SYS_SND_ROOT).lw(c);
    c.g[A0][0] = Addr(r32(c, V0)).lw(c);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(OC300_MON_QUEUE));
    call_ptr(c, 0x002a_9988, 0x0029_295c)?;
    let replay = c.g[S7][0] != 0;
    c.g[A1][0] = 1;
    if replay {
        // root table 0x258 [count] = 1; root table 0x25c [count] =
        // rider->listener.
        c.cycles += 15;
        oc300_edge_root_store(c, [A0, V1, V0, A0], 0x258, 1);
        c.g[A0][0] = Addr(r32(c, S5)).off(OC300_SYS_SND_ROOT).lw(c);
        c.g[A1][0] = Addr(r32(c, S6)).off(OC300_RIDER_LISTENER).lw(c);
        c.g[V1][0] = Addr(r32(c, A0)).lw(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(OC300_MON_COUNT).lw(c);
        c.g[A0][0] = Addr(r32(c, V1)).off(0x25c).lw(c);
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
        Addr(r32(c, V0)).sw(c, c.g[A1][0]);
    }
    c.cycles += 3;
    c.g[A0][0] = c.g[S5][0];
    OC300_PLAY_PUSHED_SOUND.call(c, 0x0029_29a8)
}

// ---------------------------------------------------------------------------
// AnimSeqList_FadeOutAll (0x3146d0)

/// `AnimSeqList`: first sequence node (`+4`); `AnimSequence`: next node
/// (`+0xc8`).
const OC300_SEQLIST_FIRST: u32 = 0x4;
const OC300_SEQ_NEXT: u32 = 0xc8;

/// `AnimSequence_FadeOut` (0x313a20).
const OC300_SEQ_FADE_OUT: Callee = Callee {
    addr: 0x0031_3a20,
    f: ee::s10::AnimSequence_FadeOut,
};

/// `void AnimSeqList_FadeOutAll(AnimSeqList* list, float time)` (0x3146d0):
/// fades out every sequence of the list (`AnimSequence_FadeOut(seq, time)`
/// along the `+0xc8` links); `time` is kept in f20 across the calls.
pub fn anim_seq_list_fade_out_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save_fpr(c, 20, 0x20);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = Addr(r32(c, A0)).off(OC300_SEQLIST_FIRST).lw(c);
        c.f[20] = c.f[12];
        if c.g[S0][0] != 0 {
            c.cycles += 1;
            c.g[A0][0] = c.g[S0][0];
            loop {
                c.cycles += 5;
                c.f[12] = c.f[20];
                OC300_SEQ_FADE_OUT.call(c, 0x0031_46f8)?;
                c.g[S0][0] = Addr(r32(c, S0)).off(OC300_SEQ_NEXT).lw(c);
                c.g[A0][0] = c.g[S0][0];
                if c.g[S0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0031_46f0);
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SyncProcess_IdEquals (0x360840)

/// `SyncProcess`: its sync id (`+0x14`).
const OC300_SYNC_ID: u32 = 0x14;
/// Object: its vtable (`+0xc`); the vtable's "get id" slot (`+0x40`).
const OC300_OBJ_VTABLE: u32 = 0xc;
const OC300_VT_GET_ID: u32 = 0x40;

/// `bool SyncProcess_IdEquals(SyncProcess* self, Object* obj)` (0x360840):
/// whether `self`'s sync id equals the id `obj` reports through its
/// virtual "get id" method (vtable slot `+0x40`).
pub fn sync_process_id_equals(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let obj = c.g[A1][0];
        let vt = Addr(obj as u32).off(OC300_OBJ_VTABLE).lw(c);
        c.g[V1][0] = vt;
        let slot = crate::call::VSlot::read(c, vt, OC300_VT_GET_ID);
        c.g[A0][0] = slot.delta;
        c.g[V0][0] = slot.func;
        c.g[A0][0] = sx((obj as u32).wrapping_add(slot.delta as u32));
        call_ptr(c, slot.func as u32, 0x0036_0864)?;
        c.g[V1][0] = Addr(r32(c, S0)).off(OC300_SYNC_ID).lw(c);
        frame.restore(c, S0, 0x10);
        c.g[V1][0] ^= c.g[V0][0];
        frame.restore_ra(c, 0);
        c.g[V0][0] = (c.g[V1][0] < 1) as u64;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Replay_OnRiderLaunch (0x2707e0)

/// `Replay` busy flags: any non-zero one blocks new highlights (`+0x61c`,
/// `+0x620`, `+0x610`, `+0x60c`, and the playing state at `+0`).
const OC300_REPLAY_BUSY_FLAGS: [u32; 4] = [0x61c, 0x620, 0x610, 0x60c];
const OC300_REPLAY_PLAYING: u32 = 0x0;

/// `Replay_FindRiderIndex` (0x270730).
const OC300_REPLAY_FIND_RIDER_INDEX: Callee = Callee {
    addr: 0x0027_0730,
    f: ee::s07::Replay_FindRiderIndex,
};
/// `Replay_BeginHighlight` (0x270b88).
const OC300_REPLAY_BEGIN_HIGHLIGHT: Callee = Callee {
    addr: 0x0027_0b88,
    f: ee::s07::Replay_BeginHighlight,
};
/// `Replay_EmptyRiderHook` (0x2705a8).
const OC300_REPLAY_EMPTY_RIDER_HOOK: Callee = Callee {
    addr: 0x0027_05a8,
    f: ee::s07::Replay_EmptyRiderHook,
};

/// `void Replay_OnRiderLaunch(Replay* self, int riderId, void* data)`
/// (0x2707e0): unless the replay is busy (any of the flags at
/// `+0x61c/+0x620/+0x610/+0x60c` or playing at `+0`), find the rider's index
/// (`Replay_FindRiderIndex(riderId)`) and, when found, begin a highlight
/// for it (`Replay_BeginHighlight(self, index, data)`) and run the (empty)
/// rider hook.
pub fn replay_on_rider_launch(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        let this = r32(c, S0);
        c.g[V0][0] = Addr(this).off(OC300_REPLAY_BUSY_FLAGS[0]).lw(c);
        c.g[S1][0] = c.g[A2][0];
        // The first flag set: the tail charges one more cycle.
        let mut tail = c.g[V0][0] != 0;
        if !tail {
            let mut busy = false;
            for off in OC300_REPLAY_BUSY_FLAGS[1..]
                .iter()
                .copied()
                .chain([OC300_REPLAY_PLAYING])
            {
                c.cycles += 3;
                c.g[V0][0] = Addr(this).off(off).lw(c);
                if c.g[V0][0] != 0 {
                    busy = true;
                    break;
                }
            }
            if !busy {
                tail = true;
                c.cycles += 4;
                OC300_REPLAY_FIND_RIDER_INDEX.call(c, 0x0027_083c)?;
                let found = (c.g[V0][0] as i64) >= 0;
                c.g[A0][0] = c.g[S0][0];
                if found {
                    c.cycles += 6;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A2][0] = c.g[S1][0];
                    OC300_REPLAY_BEGIN_HIGHLIGHT.call(c, 0x0027_0850)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S1][0];
                    OC300_REPLAY_EMPTY_RIDER_HOOK.call(c, 0x0027_085c)?;
                }
            }
        }
        if tail {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptFn_PlayObjectSound (0x2fff00)

/// `gp` word: the static parameter defaults are initialised.
const OC300_POS_DEFAULTS_READY_GP: u32 = 0x2574;
/// The static parameter defaults (3 words: object id, sound, -; 0x4fbb08).
const OC300_POS_DEFAULTS: u32 = 0x004f_bb08;
/// `gp` table: the kind of each parameter slot (2 = float).
const OC300_POS_PARAM_KINDS_GP: u32 = 0xb38;
const OC300_POS_KIND_FLOAT: u64 = 2;
/// `gp` words: the object directory, the world (its default object at
/// `+0x290`).
const OC300_POS_DIRECTORY_GP: u32 = 0x16c8;
const OC300_POS_WORLD_GP: u32 = 0xce8;
const OC300_POS_WORLD_DEFAULT_OBJECT: u32 = 0x290;
/// The script functions' shared result value (12 bytes at 0x4c9098).
const OC300_POS_RESULT: u32 = 0x004c_9098;

/// `Result WScriptFn_PlayObjectSound(Param* params, int count)` (0x2fff00;
/// a0 = the hidden result slot, a1 = count, a2 = params): fills a 3-word
/// parameter block (object id, sound) from the defaults and the
/// `{slot, value, -, kind}` parameters (an int given for a float slot is
/// converted), looks the object up (id -1: the world's default object) and,
/// if there is one, plays the sound on it
/// (`SSXAudioSystem_PlayObjectSound(GetInstance(), obj, sound)`). Returns
/// the shared script result.
pub fn wscript_fn_play_object_sound(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        let gp = r32(c, GP);
        let ready = Addr(gp).off(OC300_POS_DEFAULTS_READY_GP).lw(c);
        c.g[V0][0] = ready;
        frame.save(c, S2, 0x40);
        frame.save(c, S1, 0x50);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S3, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x60);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[A2][0] = 0x50_0000;
        let defaults = Addr(OC300_POS_DEFAULTS);
        if ready == 0 {
            // First use: initialise the static defaults (guarded local).
            c.cycles += 10;
            c.g[V1][0] = sx(OC300_POS_DEFAULTS);
            c.g[V0][0] = u64::MAX;
            let tag = defaults.off(8).lbu(c);
            c.g[A1][0] = tag;
            defaults.sw(c, u64::MAX);
            c.g[A0][0] = 1;
            defaults.off(8).sb(c, tag);
            defaults.off(4).sw(c, u64::MAX);
            Addr(gp).off(OC300_POS_DEFAULTS_READY_GP).sw(c, 1);
            Addr(sp(c)).sb(c, tag);
        }
        // The parameter block at 0x10(sp), copied from the defaults.
        c.cycles += 10;
        c.g[V0][0] = u64::MAX;
        let block = frame.at(c, 0x10);
        c.g[S0][0] = sx(block);
        c.g[A1][0] = sx(OC300_POS_DEFAULTS);
        c.g[A2][0] = 0xc;
        Addr(block).sw(c, u64::MAX);
        c.g[A0][0] = sx(block);
        call_ptr(c, 0x003e_6574, 0x002f_ff78)?; // memcpy
        c.g[A2][0] = c.g[S2][0];
        if (c.g[S1][0] as i64) > 0 {
            c.cycles += 4;
            c.g[A1][0] = c.g[S1][0];
            c.g[T0][0] = sx(r32(c, GP).wrapping_add(OC300_POS_PARAM_KINDS_GP));
            c.g[A3][0] = OC300_POS_KIND_FLOAT;
            loop {
                c.cycles += 8;
                let param = Addr(r32(c, A2));
                let slot4 = (param.lw(c) as u32) << 2;
                let given_kind = param.off(0xc).lw(c);
                c.g[V1][0] = given_kind;
                let dest = Addr(slot4.wrapping_add(r32(c, S0)));
                c.g[A0][0] = sx(dest.0);
                let kind = Addr(slot4.wrapping_add(r32(c, T0))).lw(c);
                c.g[V0][0] = kind;
                if given_kind != kind && kind == c.g[A3][0] {
                    // An int for a float slot.
                    c.cycles += 2 + 4;
                    let value = param.off(4).lw(c) as u32;
                    let converted = FpuF32::from_i32(value as i32);
                    c.f[0] = converted.0;
                    dest.write_fpu(c, converted);
                } else {
                    c.cycles += if given_kind == kind { 1 } else { 2 + 1 };
                    let value = param.off(4).lw(c);
                    c.g[V0][0] = value;
                    dest.sw(c, value);
                }
                c.cycles += 3;
                let left = r32(c, A1).wrapping_sub(1);
                c.g[A1][0] = sx(left);
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x10));
                if left == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_ff90);
                }
            }
        }
        // Look the object up.
        c.cycles += 4;
        let id = Addr(r32(c, S0)).lw(c);
        c.g[A2][0] = id;
        c.g[V0][0] = !id;
        let directory = Addr(r32(c, GP)).off(OC300_POS_DIRECTORY_GP).lw(c);
        c.g[A1][0] = directory;
        if !id == 0 {
            c.cycles += 3;
            let world = Addr(r32(c, GP)).off(OC300_POS_WORLD_GP).lw(c);
            c.g[V0][0] = world;
            c.g[S1][0] = Addr(world as u32).off(OC300_POS_WORLD_DEFAULT_OBJECT).lw(c);
        } else {
            c.cycles += 8;
            let id = id as u32;
            c.g[V0][0] = sx((id & 0xff) << 2);
            let sections = Addr(directory as u32).lw(c);
            c.g[V1][0] = sections;
            let table = Addr(sections as u32).off(8).lw(c);
            c.g[A0][0] = table;
            let at = ((id & 0xff) << 2).wrapping_add(table as u32);
            c.g[V0][0] = sx(at);
            let section = Addr(at).lw(c);
            c.g[V1][0] = section;
            let object = if section != 0 {
                c.cycles += 7;
                let entries = Addr(section as u32).off(0x1c).lw(c);
                c.g[A0][0] = entries;
                let entry = Addr(((id >> 8) << 2).wrapping_add(entries as u32)).lw(c);
                c.g[V1][0] = entry;
                let index = (entry as u32) >> 8;
                let object = if index != 0 {
                    sx(index << 2)
                } else {
                    c.cycles += 1;
                    0
                };
                c.g[V0][0] = object;
                c.cycles += 1;
                object
            } else {
                c.cycles += 2;
                0
            };
            c.g[A1][0] = object;
            c.cycles += 1;
            c.g[S1][0] = object;
        }
        c.cycles += 2;
        c.g[V1][0] = 0x4d_0000;
        if c.g[S1][0] != 0 {
            // SSXAudioSystem_PlayObjectSound(GetInstance(), obj, sound).
            c.cycles += 7;
            c.g[S0][0] = Addr(r32(c, S0)).off(4).lw(c);
            call_ptr(c, 0x0028_b180, 0x0030_0054)?; // SSXAudioSystem_GetInstance
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, 0x0029_74a0, 0x0030_0064)?; // SSXAudioSystem_PlayObjectSound
            c.g[V1][0] = 0x4d_0000;
        }
        // *result = the shared script result (12 bytes).
        c.cycles += 15;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx(OC300_POS_RESULT);
        let out = r32(c, S3);
        c.ldl(4, OC300_POS_RESULT.wrapping_add(0x7));
        c.ldr(4, OC300_POS_RESULT);
        c.g[A1][0] = Addr(OC300_POS_RESULT).off(8).lw(c);
        c.sdl(4, out.wrapping_add(0x7));
        c.sdr(4, out);
        Addr(out).off(8).sw(c, c.g[A1][0]);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// GameStateReset_SaveState (0x235990)

/// `GameState_SaveState` (0x233af0).
const OC300_GAME_STATE_SAVE_STATE: Callee = Callee {
    addr: 0x0023_3af0,
    f: ee::s06::GameState_SaveState,
};
/// The stream's `Write(data, size)` vtable slot.
const OC300_STREAM_WRITE: u32 = 0x8;
/// `GameStateReset`: the three state words written after the base state.
const OC300_RESET_SAVED_WORDS: u32 = 0x10;

/// `void GameStateReset_SaveState(GameStateReset* self, cAsStream* s)`
/// (0x235990): saves the base game state (`GameState_SaveState`), then
/// writes the three words at `self+0x10/+0x14/+0x18` (`s->Write(p, 4)`,
/// vtable slot `+0x8`).
pub fn game_state_reset_save_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        OC300_GAME_STATE_SAVE_STATE.call(c, 0x0023_59ac)?;
        for (i, ret) in [0x0023_59c8u32, 0x0023_59e4, 0x0023_5a00]
            .into_iter()
            .enumerate()
        {
            let stream = r32(c, S0);
            let vt = Addr(stream).lw(c);
            c.g[V1][0] = vt;
            c.g[A1][0] = sx(r32(c, S1).wrapping_add(OC300_RESET_SAVED_WORDS + 4 * i as u32));
            c.g[A2][0] = 0x4;
            let slot = crate::call::VSlot::read(c, vt, OC300_STREAM_WRITE);
            c.g[A0][0] = slot.delta;
            c.g[V0][0] = slot.func;
            c.g[A0][0] = sx(stream.wrapping_add(slot.delta as u32));
            call_ptr(c, slot.func as u32, ret)?;
        }
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP, RA as SENTINEL};

    /// A small process list at `gp + 0x2898`: a table of four list heads
    /// (0x44 bytes each: a flag word, then the two sub-list heads at
    /// +8/+0x28).
    fn oc300_process_list(c: &mut Cpu, r: &mut Rng) {
        let list = GAME_GP + OC300_PROCESS_LIST;
        let table = DATA + 0x2000;
        c.bus.write32(list + 4, table);
        for k in 0..4 {
            let e = table + 0x44 * k;
            c.bus.write32(e, r.next_u32());
            c.bus.write32(e + 0x8, DATA + 0x3000 + 0x40 * k);
            c.bus.write32(e + 0x28, DATA + 0x3400 + 0x40 * k);
        }
    }

    /// Vtable install and the real `ProcessList_Add` into each of the four
    /// lists (both kinds by the head's flag bit).
    #[test]
    fn game_process_ctor_matches() {
        check(ee::s11::f_00354648__generated, game_process_ctor, 300, &|c, seed| {
            let mut r = Rng(0x3546_4800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            oc300_process_list(c, &mut r);
            c.g[A0][0] = sx(DATA + 0x100);
            c.g[A1][0] = (r.next_u32() % 4) as u64;
            c.g[RA][0] = SENTINEL as u64;
            c.pc = 0x0035_4648;
        });
    }

    /// A microprogram at VU0 address 0 for `Vu0_LoadRegsAndCall`:
    /// `vi1 = vi14`, then end. Hand-written microcode has no translation:
    /// run it with the reference interpreter.
    fn oc300_put_microprogram_at_0(c: &mut Cpu) {
        ssx3_ps2::platform::machine_of(&mut c.bus.plat)
            .hw
            .vu0
            .fallback = Some(ssx3_ps2::vu::reference::run_program);
        let at = 0x1100_0000;
        c.bus.write32(at, 0x8000_7070); // IADD vi1, vi14, vi0
        c.bus.write32(at + 4, 0x4000_02ff); // NOP, E
    }

    /// Loads 16 random quadwords into VF1..VF16 and runs the microprogram
    /// at address 0.
    #[test]
    fn vu0_load_regs_and_call_matches() {
        check(ee::s12::f_00391418__generated, vu0_load_regs_and_call, 200, &|c, seed| {
            let mut r = Rng(0x3914_1800 ^ seed as u64);
            scramble(c, &mut r);
            oc300_put_microprogram_at_0(c);
            c.g[14][0] = r.next_u32() as u64 & 0xffff;
            c.ctc2(14, 14);
            let regs = DATA + 0x400 + 0x10 * (r.next_u32() % 16);
            for q in 0..16u32 {
                crate::difftest::put_vec(
                    c,
                    regs + 0x10 * q,
                    [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                );
            }
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | regs as u64;
            c.g[RA][0] = SENTINEL as u64;
            c.pc = 0x0039_1418;
        });
    }

    /// The real allocator, UV-scroll constructor and installer: a node with
    /// or without a modifier block (the block's UV-scroll slot empty, so
    /// nothing is freed), a random descriptor and a game rate object at
    /// `gp + 0x2a74` (ticks per second at +0x10).
    #[test]
    fn mod_obj_add_uv_scroll_modifier_matches() {
        use crate::game::fe::debugfe::tests::{pool_allocator, OBJ, OBJ2};
        const BLOCK: u32 = DATA + 0x4000;
        const RATE: u32 = DATA + 0x5000;
        check(
            ee::s11::f_00355b90__generated,
            mod_obj_add_uv_scroll_modifier,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x355b90, 0);
                pool_allocator(c);
                crate::difftest::fill(c, &mut r, OBJ2, 0x10);
                for i in 0..0x0cu32 {
                    c.bus.write32(OBJ2 + 8 + 4 * i, r.f32(100.0));
                }
                c.bus.write32(GAME_GP + 0x2a74, RATE);
                c.bus.write32(RATE + 0x10, 1 + r.next_u32() % 120);
                c.bus.write32(OBJ + 0x1c, if seed % 2 == 0 { 0 } else { BLOCK });
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                c.pc = 0x0035_5b90;
            },
        );
    }

    /// `SSXAudioSystem_StartBoardEdgeLoop` on the shared audio world: human
    /// and local or not (early out), mode byte 2 (the real
    /// `IsReplayPlaying`, in and out of replays: the replay listener
    /// fields) or not, surfaces 0..5 (both banks of the real
    /// `GetSurfaceBank`), the real push, memset, gain, listener lookup,
    /// `SetRadiusVanish` and `Audio_PlayPushedSound` (no bank). The
    /// `BELibrary_GetInterface` method is a fake; some seeds unwind there or
    /// in the SND lock hooks. (The volume clamp always sees the 0 it just
    /// stored, so only its in-range path runs.)
    #[test]
    fn ssx_audio_system_start_board_edge_loop_matches() {
        use crate::difftest::check_lohi;
        use crate::difftest::fakes::{FAKE, LOG_UNWIND_AT};
        use crate::game::sound::soundsys::tests::audio;
        check_lohi(
            ee::s07::f_00292508__generated,
            ssx_audio_system_start_board_edge_loop,
            150,
            &|c, seed| {
                let mut r = audio::world(c, seed, 0x29_2508, 0x0029_2508);
                // BELibrary_GetInterface: interface table entry 0 with a
                // fake method.
                let iface = DATA + 0x26_1000;
                let vt = DATA + 0x26_1100;
                c.bus.write32(GAME_GP - 0x1efc, 0);
                c.bus.write32(0x0053_08a8, iface);
                c.bus.write32(iface + 4, vt);
                c.bus.write16(vt + 8, 0);
                c.bus.write32(vt + 0xc, FAKE + 0x40);
                c.bus
                    .write8(OC300_MODE_BYTE, if seed % 5 < 3 { 2 } else { 1 });
                c.bus.write32(
                    audio::RIDER + OC300_RIDER_LISTENER,
                    r.next_u32() % 4,
                );
                c.g[A2][0] = match seed % 7 {
                    6 => (r.next_u32() % 6) as u64 | (r.next_u32() as u64) << 32,
                    k => k as u64 % 6,
                };
                if seed % 10 == 7 {
                    c.bus.write32(LOG_UNWIND_AT, r.next_u32() % 3);
                }
            },
        );
    }

    /// A list of 0..4 sequences (linked through `+0xc8`) with fade fields
    /// that reach every path of the real `AnimSequence_FadeOut` (`+0x98`
    /// and `+0x9c` zero or not, the new time below or above the old one).
    fn oc300_fade_setup(c: &mut Cpu, seed: u32, count: u32) {
        let mut r = Rng(0x3146_d000 ^ seed as u64);
        scramble(c, &mut r);
        let list = DATA + 0x100;
        let mut next = 0;
        for k in (0..count).rev() {
            let seq = DATA + 0x1000 + 0x100 * k;
            crate::difftest::fill(c, &mut r, seq, 0x40);
            let fading = match r.next_u32() % 3 {
                0 => 0,
                _ => r.f32(5.0),
            };
            c.bus.write32(seq + 0x98, fading);
            let old = match r.next_u32() % 3 {
                0 => 0,
                _ => r.f32(5.0),
            };
            c.bus.write32(seq + 0x9c, old);
            c.bus.write32(seq + OC300_SEQ_NEXT, next);
            next = seq;
        }
        c.bus.write32(list + OC300_SEQLIST_FIRST, next);
        c.g[A0][0] = sx(list);
        c.f[12] = if seed % 7 == 3 { r.edge_f32() } else { r.f32(5.0) };
        c.g[RA][0] = SENTINEL as u64;
        c.pc = 0x0031_46d0;
    }

    use crate::difftest::events::driven;
    driven!(oc300_gen_fade_out_all, ee::s10::f_003146d0__generated);
    driven!(oc300_hand_fade_out_all, anim_seq_list_fade_out_all);

    /// Empty list and lists of 1..4 sequences with the real
    /// `AnimSequence_FadeOut`; then with events at the loop's back edge.
    #[test]
    fn anim_seq_list_fade_out_all_matches() {
        use crate::difftest::events::{check_events, schedule};
        check(
            ee::s10::f_003146d0__generated,
            anim_seq_list_fade_out_all,
            300,
            &|c, seed| oc300_fade_setup(c, seed, seed % 5),
        );
        check_events(
            oc300_gen_fade_out_all,
            oc300_hand_fade_out_all,
            90,
            &|c, seed| {
                oc300_fade_setup(c, seed, 2 + seed / 3 % 3);
                let mut r = Rng(0x3146_d0 ^ seed as u64);
                let mode = schedule(c, &mut r, seed, 0x0031_46d0);
                c.next_event = c.cycles + 4 + (r.next_u32() % 16) as u64;
                mode
            },
        );
    }

    /// The id method is a fake: its result equals the process's id (a
    /// non-negative one, as fakes return zero-extended values) or not; some
    /// seeds unwind in it.
    #[test]
    fn sync_process_id_equals_matches() {
        use crate::difftest::fakes::{covered, vtable, LOG_RESULTS};
        covered(
            ee::s12::f_00360840__generated,
            sync_process_id_equals,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x36_0840, 1);
                let this = DATA + 0x100;
                let obj = DATA + 0x200;
                let vt = vtable(c, &mut r, &[OC300_VT_GET_ID]);
                c.bus.write32(obj + OC300_OBJ_VTABLE, vt);
                let id = if seed % 3 == 0 {
                    r.next_u32()
                } else {
                    r.next_u32() & 0x7fff_ffff
                };
                c.bus.write32(this + OC300_SYNC_ID, id);
                if seed % 3 != 0 {
                    let result = if seed % 2 == 0 { id } else { id ^ 1 };
                    c.bus.write32(LOG_RESULTS, result);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(obj);
                c.pc = 0x0036_0840;
            },
        );
    }

    /// The world and riders of `Replay_FindRiderIndex`'s test (found at any
    /// index, not found, no riders; fake id getters, some unwinding), the
    /// busy flags each set or all clear, and the highlight slot of the found
    /// rider in use (early out) or free (the real `HighlightSlotBegin`:
    /// snapshot bit, value, and the 0xa0-byte copy from `data`, aligned or
    /// not).
    #[test]
    fn replay_on_rider_launch_matches() {
        use crate::game::replay::replay2::tests::replay_find_rider_fixture;
        check(
            ee::s07::f_002707e0__generated,
            replay_on_rider_launch,
            300,
            &|c, seed| {
                replay_find_rider_fixture(c, seed);
                let id = c.g[A0][0];
                let mut r = Rng(0x2707_e000 ^ seed as u64);
                let this = DATA + 0x3_0000;
                let snapshot = DATA + 0x3_8000;
                let data = DATA + 0x3_9000 + (r.next_u32() % 2) * 4;
                crate::difftest::fill(c, &mut r, data, 0x30);
                crate::difftest::fill(c, &mut r, snapshot, 0x40);
                let busy = seed / 2 % 7;
                for (i, off) in OC300_REPLAY_BUSY_FLAGS
                    .into_iter()
                    .chain([OC300_REPLAY_PLAYING])
                    .enumerate()
                {
                    let set = busy as usize == i + 1;
                    c.bus.write32(this + off, if set { 1 + r.next_u32() % 4 } else { 0 });
                }
                c.bus.write32(this + 0x3b4, snapshot);
                for k in 0..5u32 {
                    let slot = this + 0x494 + 0xb4 * k;
                    c.bus.write32(slot, r.next_u32() % 8);
                    let used = r.next_u32() % 3 == 0;
                    c.bus.write32(slot + 0xb0, if used { 1 } else { 0 });
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = id;
                c.g[A2][0] = sx(data);
                c.pc = 0x0027_07e0;
            },
        );
    }

    /// `WScriptFn_PlayObjectSound`: defaults initialised or not (default id
    /// -1 or random), 0..3 parameters (or a negative count) of matching,
    /// mismatching and int-for-float kinds, object ids -1 (the world's
    /// default object, present or not), sections present or empty, entries
    /// with and without an object; when an object is found, the real
    /// `GetInstance` and `PlayObjectSound` (sound 0, or object sounds
    /// suppressed: its early exits).
    #[test]
    fn wscript_fn_play_object_sound_matches() {
        const RESULT_SLOT: u32 = DATA + 0x3000;
        const PARAMS: u32 = DATA + 0x1000;
        const AUDIO: u32 = DATA + 0x8000;
        const DIR: u32 = DATA + 0x20_0000;
        const SECTIONS: u32 = DIR + 0x100;
        const TABLE: u32 = DIR + 0x200;
        const SECTION: u32 = DIR + 0x300;
        const ENTRIES: u32 = DIR + 0x400;
        const WORLD: u32 = DIR + 0xa00;
        const OBJS: u32 = DIR + 0x2000;
        check(
            ee::s10::f_002fff00__generated,
            wscript_fn_play_object_sound,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2f_ff00, 0);
                c.pc = 0x002f_ff00;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                // Static defaults.
                let ready = seed % 4 != 0;
                w(c, GAME_GP + OC300_POS_DEFAULTS_READY_GP, ready as u32);
                crate::difftest::fill(c, &mut r, OC300_POS_DEFAULTS, 3);
                let id = match seed % 5 {
                    0 => u32::MAX,
                    k => (r.next_u32() % 4) << 8 | (k - 1),
                };
                w(c, OC300_POS_DEFAULTS, id);
                // Sound 0 or a non-0x66 one (suppressed below).
                let sound = if seed % 3 == 0 { 0 } else { 1 + r.next_u32() % 0x60 };
                w(c, OC300_POS_DEFAULTS + 4, sound);
                for i in 0..3 {
                    w(c, GAME_GP + OC300_POS_PARAM_KINDS_GP + 4 * i, r.next_u32() % 4);
                }
                // Parameters: never slot 0 (the id) or slot 1 with a 0x66.
                let count = match seed % 7 {
                    6 => (r.next_u32() % 3).wrapping_neg(),
                    k => k % 4,
                };
                for i in 0..4u32 {
                    let p = PARAMS + 0x10 * i;
                    w(c, p, 1 + r.next_u32() % 2);
                    w(c, p + 4, 1 + r.next_u32() % 0x60);
                    w(c, p + 8, r.next_u32());
                    w(c, p + 0xc, r.next_u32() % 4);
                }
                // Directory: sections 0..2 present, 3 empty; entries 0..2
                // objects, 3 empty.
                w(c, GAME_GP + OC300_POS_DIRECTORY_GP, DIR);
                w(c, DIR, SECTIONS);
                w(c, SECTIONS + 8, TABLE);
                for i in 0..4 {
                    w(c, TABLE + 4 * i, if i == 3 { 0 } else { SECTION });
                    let obj = OBJS + 0x100 * i;
                    w(
                        c,
                        ENTRIES + 4 * i,
                        if i == 3 {
                            r.next_u32() & 0xff
                        } else {
                            (obj >> 2) << 8 | (r.next_u32() & 0xff)
                        },
                    );
                    crate::difftest::fill(c, &mut r, obj, 0x20);
                }
                w(c, SECTION + 0x1c, ENTRIES);
                w(c, GAME_GP + OC300_POS_WORLD_GP, WORLD);
                w(
                    c,
                    WORLD + OC300_POS_WORLD_DEFAULT_OBJECT,
                    if seed % 10 == 5 { 0 } else { OBJS + 0x100 },
                );
                // The audio system: object sounds suppressed (non-negative).
                w(c, GAME_GP + 0x410, AUDIO);
                w(c, AUDIO + 0x62a8, r.next_u32() & 0x7fff_ffff);
                crate::difftest::fill(c, &mut r, OC300_POS_RESULT, 3);
                c.g[A0][0] = sx(RESULT_SLOT);
                c.g[A1][0] = count as i32 as i64 as u64;
                c.g[A2][0] = sx(PARAMS);
            },
        );
    }

    /// The base `GameState_SaveState` (real) and the three word writes go to
    /// a fake `Write` method (random `this` delta); some seeds unwind at any
    /// of the four calls.
    #[test]
    fn game_state_reset_save_state_matches() {
        use crate::difftest::fakes::{covered, FAKE};
        covered(
            ee::s06::f_00235990__generated,
            game_state_reset_save_state,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x23_5990, 4);
                let state = DATA + 0x100;
                let stream = DATA + 0x200;
                let vt = DATA + 0x300;
                crate::difftest::fill(c, &mut r, state, 8);
                c.bus.write32(stream, vt);
                let delta = (r.next_u32() as i16 as i32 & !3) as u32;
                c.bus.write16(vt + OC300_STREAM_WRITE, delta as u16);
                c.bus.write32(vt + OC300_STREAM_WRITE + 4, FAKE);
                c.g[A0][0] = sx(state);
                c.g[A1][0] = sx(stream);
                c.pc = 0x0023_5990;
            },
        );
    }
}
