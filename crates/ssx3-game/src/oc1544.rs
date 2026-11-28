//! Staging module oc1544 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1544`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Unwound};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due, Resume};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0024_38b0, module_base_dtor_2, "ModuleBase_dtor_2"),
    (0x0024_47f0, gm_handler_set_title_w, "GMHandler_SetTitleW"),
    (0x0024_4810, gm_handler_set_title_a, "GMHandler_SetTitleA"),
    (0x0025_3748, vid_engine_player_play_blocking, "VidEnginePlayer_PlayBlocking"),
    (0x0025_3ad0, movie_player_ssx_alloc_mem, "cMoviePlayer_SSXAllocMemFunc"),
    (0x0025_6088, ssx_load_net_gui_elf, "ssx_LoadNetGuiElf"),
    (0x0025_60b0, dnas_auth_run_blocking, "DNASAuth_RunBlocking"),
    (0x0025_61b8, cfe_state_auth_dnas_create_ctor, "cFEStateAuthDNASCreate_ctor"),
];

/// `ModuleBase` deleting destructor (0x2438b0): stores the vtable value
/// `0x46d970` at `this` and, when bit 0 of the flags (`a1`) is set, frees the
/// object with `__builtin_delete`. v0 keeps the vtable value.
pub fn module_base_dtor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[V0][0] = 0x0047_0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2690));
        frame.save_ra(c, 0);
        c.g[A1][0] &= 1;
        let vtable = c.g[V0][0];
        Addr(c.g[A0][0] as u32).sw(c, vtable);
        if c.g[A1][0] != 0 {
            c.cycles += 2;
            call_ptr(c, 0x0031_7e50, 0x0024_38d4)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `GMHandler_SetTitleW` (0x2447f0): `strcpy1(overlayManager + 0x276)`, the
/// wide title copy into the overlay manager.
pub fn gm_handler_set_title_w(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x276));
        call_ptr(c, 0x002c_2508, 0x0024_4800)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `GMHandler_SetTitleA` (0x244810): the same as [`gm_handler_set_title_w`]
/// with the other title copy (0x2c2540).
pub fn gm_handler_set_title_a(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x276));
        call_ptr(c, 0x002c_2540, 0x0024_4820)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Loop head of `VidEnginePlayer_PlayBlocking` (0x253778), where the event
/// check at the back edge resumes.
const PLAY_BLOCKING_HEAD: u32 = 0x0025_3778;

/// The address `base + off` (a register value plus an offset).
fn vid_play_at(base: u64, off: u32) -> Addr {
    Addr((base as u32).wrapping_add(off))
}

/// One turn's item call (0x2537a0..0x2537bc). With `v0 != 0` it calls the
/// method at +0x2c of the item at `this+0x3c` (`a0 = item + delta`);
/// otherwise the method at +0x24 of the same item (its `a2`), with `a1` the
/// `this+0xc` word. Both reload `a1` from the grid manager at `gp+0x2a90`.
fn vid_play_items(c: &mut Cpu) -> Flow {
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        c.cycles += 8;
        c.g[A1][0] = vid_play_at(c.g[S0][0], 0x3c).lw(c);
        c.g[V0][0] = vid_play_at(c.g[A1][0], 4).lw(c);
        c.g[A0][0] = vid_play_at(c.g[V0][0], 0x28).lh(c);
        c.g[V1][0] = vid_play_at(c.g[V0][0], 0x2c).lw(c);
        let method = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, method, 0x0025_3798)?;
        c.g[A1][0] = vid_play_at(c.g[GP][0], 0x2a90).lw(c);
    } else {
        c.g[A2][0] = vid_play_at(c.g[S0][0], 0x3c).lw(c);
        c.cycles += 7;
        c.g[A1][0] = vid_play_at(c.g[S0][0], 0xc).lw(c);
        c.g[V0][0] = vid_play_at(c.g[A2][0], 4).lw(c);
        c.g[A0][0] = vid_play_at(c.g[V0][0], 0x20).lh(c);
        c.g[V1][0] = vid_play_at(c.g[V0][0], 0x24).lw(c);
        let method = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, method, 0x0025_37b8)?;
        c.g[A1][0] = vid_play_at(c.g[GP][0], 0x2a90).lw(c);
    }
    Ok(())
}

/// The frame source call of one turn (0x25381c): the texture manager's
/// method at +0xa4, the two stream calls, then `this+0x38`'s probe (0x3ae780).
/// A ready answer calls the item at `this+0x3c` (+0x30/+0x34) and the
/// 0x3ae6c8 accessor, storing its value at `this+0xc` and clearing `this+4`;
/// a not-ready answer sets `this+4 = s1`.
fn vid_play_frame(c: &mut Cpu) -> Flow {
    c.cycles += 13;
    c.g[V0][0] = vid_play_at(c.g[A1][0], 0x10d8).lw(c);
    c.g[A0][0] = vid_play_at(c.g[V0][0], 0xa0).lh(c);
    c.g[V1][0] = vid_play_at(c.g[V0][0], 0xa4).lw(c);
    let method = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, method, 0x0025_37d0)?;
    c.g[A0][0] = 0;
    call_ptr(c, 0x003e_5928, 0x0025_37d8)?;
    c.g[A0][0] = 0;
    call_ptr(c, 0x003e_5398, 0x0025_37e0)?;
    c.g[A0][0] = vid_play_at(c.g[S0][0], 0x38).lw(c);
    call_ptr(c, 0x003a_e780, 0x0025_37e8)?;
    if c.g[V0][0] == 0 {
        let s1 = c.g[S1][0];
        vid_play_at(c.g[S0][0], 4).sw(c, s1);
        return Ok(());
    }
    c.cycles += 11;
    c.g[A1][0] = vid_play_at(c.g[S0][0], 0x3c).lw(c);
    c.g[V0][0] = vid_play_at(c.g[A1][0], 4).lw(c);
    c.g[A0][0] = vid_play_at(c.g[V0][0], 0x30).lh(c);
    c.g[V1][0] = vid_play_at(c.g[V0][0], 0x34).lw(c);
    let method = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, method, 0x0025_3808)?;
    c.g[A0][0] = vid_play_at(c.g[S0][0], 0x38).lw(c);
    c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x48));
    call_ptr(c, 0x003a_e6c8, 0x0025_3814)?;
    let value = c.g[V0][0];
    vid_play_at(c.g[S0][0], 0xc).sw(c, value);
    vid_play_at(c.g[S0][0], 4).sw(c, 0);
    Ok(())
}

/// The loop test (0x25383c): when `this+0xc` is set, `v0 = this->word0`;
/// otherwise the 0x3ae860 call on `this+0x38` stores its value at word0 and
/// `v0` reloads it.
fn vid_play_probe(c: &mut Cpu) -> Flow {
    c.cycles += 3;
    c.g[V0][0] = vid_play_at(c.g[S0][0], 0xc).lw(c);
    if c.g[V0][0] != 0 {
        c.g[V0][0] = vid_play_at(c.g[S0][0], 0).lw(c);
        return Ok(());
    }
    c.cycles += 5;
    let s1 = c.g[S1][0];
    vid_play_at(c.g[S0][0], 4).sw(c, s1);
    c.g[A0][0] = vid_play_at(c.g[S0][0], 0x38).lw(c);
    call_ptr(c, 0x003a_e860, 0x0025_3834)?;
    let value = c.g[V0][0];
    vid_play_at(c.g[S0][0], 0).sw(c, value);
    c.g[V0][0] = vid_play_at(c.g[S0][0], 0).lw(c);
    Ok(())
}

/// `VidEnginePlayer_PlayBlocking(this)` (0x253748): when `this+0x38` is set,
/// starts playback (0x2536c8), then loops over the item and frame calls until
/// the probe reports a result (`this+0x40 = 0` on that exit). Restores s0/s1
/// and returns.
pub fn vid_engine_player_play_blocking(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = vid_play_at(c.g[S0][0], 0x38).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[S1][0] = 1;
            Callee {
                addr: 0x0025_36c8,
                f: ee::s06::f_002536c8,
            }
            .call(c, 0x0025_3770)?;
            c.g[V0][0] = vid_play_at(c.g[S0][0], 4).lw(c);
            loop {
                vid_play_items(c)?;
                vid_play_frame(c)?;
                vid_play_probe(c)?;
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = vid_play_at(c.g[S0][0], 4).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, PLAY_BLOCKING_HEAD);
                    }
                } else {
                    c.cycles += 2;
                    vid_play_at(c.g[S0][0], 0x40).sw(c, 0);
                    break;
                }
            }
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cMoviePlayer_SSXAllocMemFunc(size, ..., align)` (0x253ad0): calls
/// `MEM_AllocNamedAlignedHook` with the alignment (`a2`) raised to 0x40 when
/// it is at most 0x3f.
pub fn movie_player_ssx_alloc_mem(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[V0][0] = 0x3f;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 0x40;
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A2][0] as i64)) as u64;
        frame.save_ra(c, 0);
        if c.g[V0][0] == 0 {
            c.g[A2][0] = c.g[V1][0];
        }
        Callee {
            addr: 0x0025_2f60,
            f: ee::s06::f_00252f60,
        }
        .call(c, 0x0025_3aec)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `ssx_LoadNetGuiElf()` (0x256088): `0x31b008("cdrom0:\NETGUI\NTGUI.ELF", 1,
/// &D_004A2EC0)`, the NetGui ELF load.
pub fn ssx_load_net_gui_elf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x0048_0000;
        frame.save_ra(c, 0);
        c.g[A1][0] = 1;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x230));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x378));
        call_ptr(c, 0x0031_b008, 0x0025_60a4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Resume points of `DNASAuth_RunBlocking` (0x2560b0). The function is a
/// `match pc` dispatcher whose loops return on an unwind request, so every
/// loop head (and each block after a call) is an entry point.
#[derive(Clone, Copy, PartialEq, Debug)]
enum DnasRunStage {
    /// The entry: allocate, construct, start (0x2560b0).
    Create,
    /// 0x2560f4: one cycle, then the first update loop.
    PadA,
    /// 0x2560f8: the first update loop's body (a loop head).
    UpdateA,
    /// 0x256114: loop while the state is below 3.
    CheckA,
    /// 0x256124: `BeginAuth`, then the second loop's test.
    Begin,
    /// 0x256134: one cycle, then the second update loop.
    PadB,
    /// 0x256138: the second update loop's body (a loop head).
    UpdateB,
    /// 0x256154: loop while the state is not 5.
    CheckB,
    /// 0x256160: free the buffers, then the destructor when `this` is set.
    Free,
    /// 0x256170: the destructor (`DNASAuth_dtor(this, 3)`).
    Dtor,
    /// 0x256178: restore s0/ra and return.
    Epilogue,
}

/// The stage result: the next stage, return, or an unwind (the callee left
/// `pc` elsewhere).
type DnasStep = Result<Resume<DnasRunStage>, Unwound>;

/// `DNASAuth_RunBlocking` (0x2560b0): the blocking authentication loop. It
/// creates the DNAS auth object (`operator new(0xa50)`, the ctor, `Start`),
/// then updates it until its state reaches 3 and then 5, frees the buffers and
/// destroys the object when the object is set.
const DNAS_RUN: u32 = 0x0025_60b0;
const DNAS_RUN_END: u32 = 0x0025_6188;
/// The frame of the entry (`sp -= 0x20`).
const DNAS_RUN_FRAME: u32 = 0x20;
/// The loop heads with an event check (the back edges).
const DNAS_UPDATE_A: u32 = 0x0025_60f8;
const DNAS_UPDATE_B: u32 = 0x0025_6138;

pub fn dnas_auth_run_blocking(c: &mut Cpu) {
    let mut stage = match c.pc {
        DNAS_RUN => DnasRunStage::Create,
        0x0025_60f4 => DnasRunStage::PadA,
        DNAS_UPDATE_A => DnasRunStage::UpdateA,
        0x0025_6114 => DnasRunStage::CheckA,
        0x0025_6124 => DnasRunStage::Begin,
        0x0025_6134 => DnasRunStage::PadB,
        DNAS_UPDATE_B => DnasRunStage::UpdateB,
        0x0025_6154 => DnasRunStage::CheckB,
        0x0025_6160 => DnasRunStage::Free,
        0x0025_6170 => DnasRunStage::Dtor,
        0x0025_6178 => DnasRunStage::Epilogue,
        pc => {
            c.jump_out(pc, DNAS_RUN, DNAS_RUN_END);
            return;
        }
    };
    loop {
        let step = match stage {
            DnasRunStage::Create => dnas_run_create(c),
            DnasRunStage::PadA => dnas_run_pad_a(c),
            DnasRunStage::UpdateA => dnas_run_update_a(c),
            DnasRunStage::CheckA => dnas_run_check_a(c),
            DnasRunStage::Begin => dnas_run_begin(c),
            DnasRunStage::PadB => dnas_run_pad_b(c),
            DnasRunStage::UpdateB => dnas_run_update_b(c),
            DnasRunStage::CheckB => dnas_run_check_b(c),
            DnasRunStage::Free => dnas_run_free(c),
            DnasRunStage::Dtor => dnas_run_dtor(c),
            DnasRunStage::Epilogue => dnas_run_epilogue(c),
        };
        match step {
            Ok(Resume::Go(next)) => stage = next,
            Ok(Resume::Done) | Ok(Resume::Unwind) | Err(Unwound) => return,
        }
    }
}

/// Entry: `this = new(0xa50)` (0x317d70), `cFEStateAuthDNASCreate_ctor(this)`,
/// `DNASAuth_Start(this, 0)`, then the state word is read.
fn dnas_run_create(c: &mut Cpu) -> DnasStep {
    c.cycles += 17;
    let frame = Frame::open(c, DNAS_RUN_FRAME);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[A1][0] = 0x004a_2ec8;
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    c.g[A0][0] = 0xa50;
    call_ptr(c, 0x0031_7d70, 0x0025_60d4)?;
    c.g[A0][0] = c.g[V0][0];
    Callee {
        addr: 0x0025_61b8,
        f: ee::s06::f_002561b8,
    }
    .call(c, 0x0025_60dc)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    c.g[A0][0] = c.g[S0][0];
    Callee {
        addr: 0x0025_6258,
        f: ee::s06::f_00256258,
    }
    .call(c, 0x0025_60ec)?;
    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
    Ok(Resume::Go(DnasRunStage::CheckA))
}

fn dnas_run_pad_a(c: &mut Cpu) -> DnasStep {
    c.cycles += 1;
    Ok(Resume::Go(DnasRunStage::UpdateA))
}

/// The first update: `DNASAuth_Update(this)`, two calls, then the state.
fn dnas_run_update_a(c: &mut Cpu) -> DnasStep {
    c.cycles += 7;
    c.g[A0][0] = c.g[S0][0];
    Callee {
        addr: 0x0025_63f8,
        f: ee::s06::f_002563f8,
    }
    .call(c, 0x0025_6100)?;
    c.g[A0][0] = 0;
    call_ptr(c, 0x003e_5928, 0x0025_6108)?;
    c.g[A0][0] = 1;
    call_ptr(c, 0x003e_52b0, 0x0025_6110)?;
    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
    Ok(Resume::Go(DnasRunStage::CheckA))
}

/// Loop while `state < 3` (signed); the back edge checks events.
fn dnas_run_check_a(c: &mut Cpu) -> DnasStep {
    c.cycles += 4;
    c.g[V0][0] = ((c.g[V0][0] as i64) < 3) as u64;
    c.g[V0][0] ^= 1;
    if c.g[V0][0] == 0 {
        if back_edge_unwinds(c, DNAS_UPDATE_A) {
            return Ok(Resume::Unwind);
        }
        return Ok(Resume::Go(DnasRunStage::UpdateA));
    }
    Ok(Resume::Go(DnasRunStage::Begin))
}

fn dnas_run_begin(c: &mut Cpu) -> DnasStep {
    c.cycles += 4;
    c.g[A0][0] = c.g[S0][0];
    Callee {
        addr: 0x0025_62d0,
        f: ee::s06::f_002562d0,
    }
    .call(c, 0x0025_612c)?;
    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
    Ok(Resume::Go(DnasRunStage::CheckB))
}

fn dnas_run_pad_b(c: &mut Cpu) -> DnasStep {
    c.cycles += 1;
    Ok(Resume::Go(DnasRunStage::UpdateB))
}

/// The second update: as the first one, with the returns of its own.
fn dnas_run_update_b(c: &mut Cpu) -> DnasStep {
    c.cycles += 7;
    c.g[A0][0] = c.g[S0][0];
    Callee {
        addr: 0x0025_63f8,
        f: ee::s06::f_002563f8,
    }
    .call(c, 0x0025_6140)?;
    c.g[A0][0] = 0;
    call_ptr(c, 0x003e_5928, 0x0025_6148)?;
    c.g[A0][0] = 1;
    call_ptr(c, 0x003e_52b0, 0x0025_6150)?;
    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
    Ok(Resume::Go(DnasRunStage::CheckB))
}

/// Loop while `state != 5` (the xor with 5 is zero then); the back edge
/// checks events.
fn dnas_run_check_b(c: &mut Cpu) -> DnasStep {
    c.cycles += 3;
    c.g[V0][0] ^= 5;
    if c.g[V0][0] != 0 {
        if back_edge_unwinds(c, DNAS_UPDATE_B) {
            return Ok(Resume::Unwind);
        }
        return Ok(Resume::Go(DnasRunStage::UpdateB));
    }
    Ok(Resume::Go(DnasRunStage::Free))
}

/// `DNASAuth_FreeBuffers(this)`; the destructor runs only when `this` is set
/// (`a0` holds `this` in both cases).
fn dnas_run_free(c: &mut Cpu) -> DnasStep {
    c.cycles += 4;
    c.g[A0][0] = c.g[S0][0];
    Callee {
        addr: 0x0025_63a8,
        f: ee::s06::f_002563a8,
    }
    .call(c, 0x0025_6168)?;
    c.g[A0][0] = c.g[S0][0];
    if c.g[S0][0] == 0 {
        return Ok(Resume::Go(DnasRunStage::Epilogue));
    }
    Ok(Resume::Go(DnasRunStage::Dtor))
}

fn dnas_run_dtor(c: &mut Cpu) -> DnasStep {
    c.cycles += 2;
    c.g[A1][0] = 3;
    Callee {
        addr: 0x0025_61e0,
        f: ee::s06::f_002561e0,
    }
    .call(c, 0x0025_6178)?;
    Ok(Resume::Go(DnasRunStage::Epilogue))
}

fn dnas_run_epilogue(c: &mut Cpu) -> DnasStep {
    c.cycles += 4;
    let frame = Frame {
        size: DNAS_RUN_FRAME,
    };
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(Resume::Done)
}

/// `cFEStateAuthDNASCreate_ctor(this)` (0x2561b8, `DNASMan_DNASMan`): calls
/// `DNASAuth_Init()` and returns `this`.
pub fn cfe_state_auth_dnas_create_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x0025_6228,
            f: ee::s06::f_00256228,
        }
        .call(c, 0x0025_61cc)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes;
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    const MAGIC: u32 = 0x1544_c0de;

    // One gate word per fake, each in its own machine's RAM.
    const G_DELETE: u32 = DATA + 0x21_0000;
    const G_STRCPY: u32 = DATA + 0x21_0040;
    const G_STRCPY_A: u32 = DATA + 0x21_0080;
    const G_LOAD_ELF: u32 = DATA + 0x21_00c0;
    const G_ALLOC: u32 = DATA + 0x21_0100;
    const G_PLAY_START: u32 = DATA + 0x21_0140;
    const G_PLAY_E5928: u32 = DATA + 0x21_0180;
    const G_PLAY_E5398: u32 = DATA + 0x21_01c0;
    const G_PLAY_780: u32 = DATA + 0x21_0200;
    const G_PLAY_860: u32 = DATA + 0x21_0240;
    const G_PLAY_6C8: u32 = DATA + 0x21_0280;
    /// Zero answers left for `0x3ae780` (decremented per call).
    const CTR_PLAY_780: u32 = DATA + 0x21_1000;
    /// Non-zero: `0x3ae6c8` answers zero.
    const ZERO_PLAY_6C8: u32 = DATA + 0x21_1004;
    const G_DNAS_NEW: u32 = DATA + 0x21_2000;
    const G_DNAS_INIT: u32 = DATA + 0x21_2040;
    const G_DNAS_START: u32 = DATA + 0x21_2080;
    const G_DNAS_UPDATE: u32 = DATA + 0x21_20c0;
    const G_DNAS_E5928: u32 = DATA + 0x21_2100;
    const G_DNAS_E52B0: u32 = DATA + 0x21_2140;
    const G_DNAS_BEGIN: u32 = DATA + 0x21_2180;
    const G_DNAS_FREE: u32 = DATA + 0x21_21c0;
    const G_DNAS_DTOR: u32 = DATA + 0x21_2200;
    /// The DNAS auth object (its first word is the state).
    const DNAS_OBJ: u32 = DATA + 0x5000;

    /// Gated fakes: each logs its call when its gate word is `MAGIC` and runs
    /// the body it replaced otherwise.
    macro_rules! gated_fakes {
        ($( ($fake:ident, $gate:expr, $target:expr, $slot:path, $generated:path) ),* $(,)?) => {
            $(
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32($gate) != MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), $generated);
                    }
                    let ra = c.g[RA][0] as u32;
                    fakes::fake(c, $target, ra);
                }
            )*

            /// Hooks the fakes above (once per process).
            fn install_fakes() {
                $(
                    fakes::hook(addr_of_mut!($slot), $fake, fakes::prev_cell($fake));
                )*
            }
        };
    }

    gated_fakes! {
        (oc1544_fake_delete, G_DELETE, 0x0031_7e50, ee::s10::OVERRIDE_00317E50, ee::s10::f_00317e50__generated),
        (oc1544_fake_strcpy, G_STRCPY, 0x002c_2508, ee::s08::OVERRIDE_002C2508, ee::s08::f_002c2508__generated),
        (oc1544_fake_strcpy_a, G_STRCPY_A, 0x002c_2540, ee::s08::OVERRIDE_002C2540, ee::s08::f_002c2540__generated),
        (oc1544_fake_load_elf, G_LOAD_ELF, 0x0031_b008, ee::s10::OVERRIDE_0031B008, ee::s10::f_0031b008__generated),
        (oc1544_fake_alloc, G_ALLOC, 0x0025_2f60, ee::s06::OVERRIDE_00252F60, ee::s06::f_00252f60__generated),
        (oc1544_fake_play_start, G_PLAY_START, 0x0025_36c8, ee::s06::OVERRIDE_002536C8, ee::s06::f_002536c8__generated),
        (oc1544_fake_play_e5928, G_PLAY_E5928, 0x003e_5928, ee::s14::OVERRIDE_003E5928, ee::s14::f_003e5928__generated),
        (oc1544_fake_play_e5398, G_PLAY_E5398, 0x003e_5398, ee::s14::OVERRIDE_003E5398, ee::s14::f_003e5398__generated),
        (oc1544_fake_play_860, G_PLAY_860, 0x003a_e860, ee::s13::OVERRIDE_003AE860, ee::s13::f_003ae860__generated),
        (oc1544_fake_dnas_new, G_DNAS_NEW, 0x0031_7d70, ee::s10::OVERRIDE_00317D70, ee::s10::f_00317d70__generated),
        (oc1544_fake_dnas_init, G_DNAS_INIT, 0x0025_6228, ee::s06::OVERRIDE_00256228, ee::s06::f_00256228__generated),
        (oc1544_fake_dnas_start, G_DNAS_START, 0x0025_6258, ee::s06::OVERRIDE_00256258, ee::s06::f_00256258__generated),
        (oc1544_fake_dnas_e5928, G_DNAS_E5928, 0x003e_5928, ee::s14::OVERRIDE_003E5928, ee::s14::f_003e5928__generated),
        (oc1544_fake_dnas_e52b0, G_DNAS_E52B0, 0x003e_52b0, ee::s14::OVERRIDE_003E52B0, ee::s14::f_003e52b0__generated),
        (oc1544_fake_dnas_begin, G_DNAS_BEGIN, 0x0025_62d0, ee::s06::OVERRIDE_002562D0, ee::s06::f_002562d0__generated),
        (oc1544_fake_dnas_free, G_DNAS_FREE, 0x0025_63a8, ee::s06::OVERRIDE_002563A8, ee::s06::f_002563a8__generated),
        (oc1544_fake_dnas_dtor, G_DNAS_DTOR, 0x0025_61e0, ee::s06::OVERRIDE_002561E0, ee::s06::f_002561e0__generated),
    }

    /// `0x3ae780` answers zero for the first `CTR_PLAY_780` calls.
    fn oc1544_fake_play_780(c: &mut Cpu) {
        if c.bus.read32(G_PLAY_780) != MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1544_fake_play_780),
                ee::s13::f_003ae780__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        let left = c.bus.read32(CTR_PLAY_780);
        if left > 0 {
            c.bus.write32(CTR_PLAY_780, left - 1);
        }
        fakes::fake(c, 0x003a_e780, ra);
        if left > 0 {
            c.g[V0][0] = 0;
        }
    }

    /// `0x3ae6c8` answers zero when `ZERO_PLAY_6C8` is set.
    fn oc1544_fake_play_6c8(c: &mut Cpu) {
        if c.bus.read32(G_PLAY_6C8) != MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1544_fake_play_6c8),
                ee::s13::f_003ae6c8__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        let zero = c.bus.read32(ZERO_PLAY_6C8) != 0;
        fakes::fake(c, 0x003a_e6c8, ra);
        if zero {
            c.g[V0][0] = 0;
        }
    }

    /// `DNASAuth_Update(this)`: the state word at `this` advances by one.
    fn oc1544_fake_dnas_update(c: &mut Cpu) {
        if c.bus.read32(G_DNAS_UPDATE) != MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1544_fake_dnas_update),
                ee::s06::f_002563f8__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        let state = c.g[A0][0] as u32;
        fakes::fake(c, 0x0025_63f8, ra);
        let next = c.bus.read32(state).wrapping_add(1);
        c.bus.write32(state, next);
    }

    fn install_custom_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003AE780),
            oc1544_fake_play_780,
            fakes::prev_cell(oc1544_fake_play_780),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003AE6C8),
            oc1544_fake_play_6c8,
            fakes::prev_cell(oc1544_fake_play_6c8),
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_002563F8),
            oc1544_fake_dnas_update,
            fakes::prev_cell(oc1544_fake_dnas_update),
        );
    }

    fn install_all() {
        install_fakes();
        install_custom_fakes();
    }

    fn open_gates(c: &mut Cpu, gates: &[u32]) {
        for g in gates {
            c.bus.write32(*g, MAGIC);
        }
    }

    #[test]
    fn module_base_dtor_2_matches() {
        install_all();
        check(
            ee::s06::f_002438b0__generated,
            module_base_dtor_2,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x2438_b0, 0);
                open_gates(c, &[G_DELETE]);
                c.g[A0][0] = sx(DATA + 0x1000);
                // Bit 0 of the flags: the object is freed on every other seed.
                c.g[A1][0] = (r.next_u32() as u64 & !1) | (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn gm_handler_set_title_w_matches() {
        install_all();
        check(
            ee::s06::f_002447f0__generated,
            gm_handler_set_title_w,
            200,
            &|c, seed| {
                fakes::machine(c, seed, 0x2447_f0, 0);
                open_gates(c, &[G_STRCPY]);
                c.g[A0][0] = sx(DATA + 0x1000 + 0x100 * (seed % 4));
            },
        );
    }

    #[test]
    fn gm_handler_set_title_a_matches() {
        install_all();
        check(
            ee::s06::f_00244810__generated,
            gm_handler_set_title_a,
            200,
            &|c, seed| {
                fakes::machine(c, seed, 0x2448_10, 0);
                open_gates(c, &[G_STRCPY_A]);
                c.g[A0][0] = sx(DATA + 0x1000 + 0x100 * (seed % 4));
            },
        );
    }

    #[test]
    fn movie_player_ssx_alloc_mem_matches() {
        install_all();
        check(
            ee::s06::f_00253ad0__generated,
            movie_player_ssx_alloc_mem,
            200,
            &|c, seed| {
                fakes::machine(c, seed, 0x253a_d0, 0);
                open_gates(c, &[G_ALLOC]);
                // Alignments below, at and above 0x3f (and negative ones).
                c.g[A2][0] = sx((seed % 128).wrapping_sub(16));
            },
        );
    }

    #[test]
    fn ssx_load_net_gui_elf_matches() {
        install_all();
        check(
            ee::s06::f_00256088__generated,
            ssx_load_net_gui_elf,
            100,
            &|c, seed| {
                fakes::machine(c, seed, 0x2560_88, 0);
                open_gates(c, &[G_LOAD_ELF]);
            },
        );
    }

    #[test]
    fn cfe_state_auth_dnas_create_ctor_matches() {
        install_all();
        check(
            ee::s06::f_002561b8__generated,
            cfe_state_auth_dnas_create_ctor,
            200,
            &|c, seed| {
                fakes::machine(c, seed, 0x2561_b8, 0);
                open_gates(c, &[G_DNAS_INIT]);
                c.g[A0][0] = sx(DATA + 0x5800 + 0x40 * (seed % 4));
            },
        );
    }

    driven!(dnas_run_generated, ee::s06::f_002560b0__generated);
    driven!(dnas_run_hand, dnas_auth_run_blocking);

    #[test]
    fn dnas_auth_run_blocking_matches() {
        install_all();
        check_events(dnas_run_generated, dnas_run_hand, 400, &|c, seed| {
            let mut r = fakes::machine(c, seed, 0x2560_b0, 0);
            open_gates(
                c,
                &[
                    G_DNAS_NEW,
                    G_DNAS_INIT,
                    G_DNAS_START,
                    G_DNAS_UPDATE,
                    G_DNAS_E5928,
                    G_DNAS_E52B0,
                    G_DNAS_BEGIN,
                    G_DNAS_FREE,
                    G_DNAS_DTOR,
                ],
            );
            // The first logged call is `operator new`: it returns the object.
            c.bus.write32(fakes::LOG_RESULTS, DNAS_OBJ);
            // Initial state 0..=5: the two loops run for the states below 3
            // and below 5 and end at 5.
            c.bus.write32(DNAS_OBJ, seed % 6);
            schedule(c, &mut r, seed, DNAS_RUN)
        });
    }

    /// Fixture for `VidEnginePlayer_PlayBlocking`: the player at `this`,
    /// its item and the texture manager, with the method table entries all
    /// on the miss-hook fake. Each seed picks one path through the loop:
    /// early exit (seed % 8 == 1), the back edge (even seeds), and the
    /// ready and not-ready shapes of the other odd seeds.
    fn play_blocking_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x253748, 0);
        let this = DATA + 0x7000;
        let obj = DATA + 0x6000;
        let vt = DATA + 0x6100;
        let grman = DATA + 0x3000;
        let texman = DATA + 0x4000;
        let sub = DATA + 0x7800;
        open_gates(
            c,
            &[
                G_PLAY_START,
                G_PLAY_E5928,
                G_PLAY_E5398,
                G_PLAY_780,
                G_PLAY_860,
                G_PLAY_6C8,
            ],
        );
        c.bus.write32(obj + 4, vt);
        c.bus.write16(vt + 0x20, r.next_u32() as u16);
        c.bus.write32(vt + 0x24, fakes::FAKE);
        c.bus.write16(vt + 0x28, r.next_u32() as u16);
        c.bus.write32(vt + 0x2c, fakes::FAKE);
        c.bus.write16(vt + 0x30, r.next_u32() as u16);
        c.bus.write32(vt + 0x34, fakes::FAKE);
        c.bus.write32(GAME_GP + 0x2a90, grman);
        c.bus.write32(grman + 0x10d8, texman);
        c.bus.write16(texman + 0xa0, r.next_u32() as u16);
        c.bus.write32(texman + 0xa4, fakes::FAKE);

        let early = seed % 8 == 1;
        // (word0, wc, zero answers of 0x3ae780, 0x3ae6c8 answers zero)
        let (w0, wc, zero_780, zero_6c8) = if seed % 2 == 0 {
            (0, 0x1234_5678, 1, 1)
        } else {
            match seed % 8 {
                3 => (0x55, 0x66, 0, 0),
                5 => (0x77, 0, 1, 0),
                7 => (0x88, 0x99, 1, 0),
                _ => (0x33, 0x44, 0, 0),
            }
        };
        c.bus.write32(CTR_PLAY_780, zero_780);
        c.bus.write32(ZERO_PLAY_6C8, zero_6c8);
        c.bus.write32(this + 0x38, if early { 0 } else { sub });
        c.bus.write32(this + 0x3c, obj);
        c.bus.write32(this, w0);
        c.bus.write32(this + 0xc, wc);
        let s4 = if (seed / 2) % 2 == 0 { 0 } else { 1 };
        c.bus.write32(this + 4, s4);
        c.g[A0][0] = sx(this);
    }

    #[test]
    fn vid_engine_player_play_blocking_matches() {
        install_all();
        check_events(
            ee::s06::f_00253748__generated,
            vid_engine_player_play_blocking,
            400,
            &|c, seed| {
                play_blocking_setup(c, seed);
                let mut r = Rng(0x2537_48 ^ (seed as u64) << 8);
                schedule(c, &mut r, seed, 0x0025_3748)
            },
        );
    }
}
