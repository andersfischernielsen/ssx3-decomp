//! Staging module oc534 (wave wave231): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc534`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, tail_wrapper, Callee, Flow, Frame, Target};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0027_16f0, replay_next_save_block, "Replay_NextSaveBlock"),
    (
        0x0027_18a8,
        replay_debug_printf_stub,
        "Replay_DebugPrintfStub",
    ),
    (0x0027_22c0, replay_global_ctors, "Replay_GlobalCtors"),
    (
        0x0027_1510,
        replay_save_write_cache,
        "Replay_SaveWriteCache",
    ),
    (
        0x0027_2530,
        insseq_heap_delete_thunk,
        "InSeq_HeapDelete_Thunk",
    ),
    (
        0x0027_2550,
        insseq_heap_delete_thunk2,
        "InSeq_HeapDelete_Thunk2",
    ),
    (0x0027_2a50, insseq_free_all, "InSeq_FreeAll"),
    (0x0027_2b00, insseq_play_all, "InSeq_PlayAll"),
    (0x0027_2b80, insseq_stop_all, "InSeq_StopAll"),
    (0x0027_2bd0, insseq_pause, "InSeq_Pause"),
    (0x0027_2bf8, insseq_pause_all, "InSeq_PauseAll"),
    (0x0027_2c48, insseq_resume, "InSeq_Resume"),
    (0x0027_2c70, insseq_resume_all, "InSeq_ResumeAll"),
    (
        0x0027_2ce0,
        insseq_get_instance_data,
        "InSeq_GetInstanceData",
    ),
    (
        0x0027_2d00,
        insseq_get_instance_owner,
        "InSeq_GetInstanceOwner",
    ),
    (
        0x0027_2d20,
        insseq_get_channel_count,
        "InSeq_GetSeqChannelCount",
    ),
    (
        0x0027_2d40,
        insseq_get_channel_block_field4,
        "InSeq_GetChannelBlockField4",
    ),
];

// Callees. Installed callees are reached through their override slots by the
// generated alias; the rest run as generated.
const CALLEE_REPLAY_SAVE_EMIT: Callee = Callee {
    addr: 0x0027_1058,
    f: ee::s07::f_00271058,
};
const CALLEE_REPLAY_STATIC_INIT_DESTROY: Callee = Callee {
    addr: 0x0027_18e8,
    f: ee::s07::f_002718e8,
};
const CALLEE_INSEQ_HEAP_DELETE: Callee = Callee {
    addr: 0x0027_24e8,
    f: ee::s07::f_002724e8,
};
const CALLEE_INSEQ_FREE_SEQUENCE: Callee = Callee {
    addr: 0x0027_29a8,
    f: ee::s07::f_002729a8,
};
const CALLEE_INSEQ_GET_INSTANCE: Callee = Callee {
    addr: 0x0027_2cc0,
    f: ee::s07::f_00272cc0,
};
const CALLEE_INSEQ_PLAY: Callee = Callee {
    addr: 0x0027_2ad0,
    f: ee::s07::f_00272ad0,
};
const CALLEE_INSEQ_STOP: Callee = Callee {
    addr: 0x0027_2b58,
    f: ee::s07::f_00272b58,
};
const CALLEE_INSEQ_PAUSE: Callee = Callee {
    addr: 0x0027_2bd0,
    f: ee::s07::f_00272bd0,
};
const CALLEE_INSEQ_RESUME: Callee = Callee {
    addr: 0x0027_2c48,
    f: ee::s07::f_00272c48,
};
const CALLEE_INSEQ_INSTANCE_PAUSE: Callee = Callee {
    addr: 0x0027_49e8,
    f: ee::s07::f_002749e8,
};
const CALLEE_INSEQ_INSTANCE_RESUME: Callee = Callee {
    addr: 0x0027_4a08,
    f: ee::s07::f_00274a08,
};
const CALLEE_INSEQ_INSTANCE_GET_DATA: Callee = Callee {
    addr: 0x0027_4c58,
    f: ee::s07::f_00274c58,
};
const CALLEE_INSEQ_INSTANCE_GET_OWNER: Callee = Callee {
    addr: 0x0027_4c60,
    f: ee::s07::f_00274c60,
};
const CALLEE_INSEQ_GET_CHANNEL_COUNT: Callee = Callee {
    addr: 0x0027_4c68,
    f: ee::s07::f_00274c68,
};
const CALLEE_INSEQ_FIND_CHANNEL_DATA: Callee = Callee {
    addr: 0x0027_4d90,
    f: ee::s07::f_00274d90,
};
const CALLEE_INSEQ_BLOCK_GET_FIELD4: Callee = Callee {
    addr: 0x0027_4398,
    f: ee::s07::f_00274398,
};

/// `Replay_NextSaveBlock(Replay* r)` (0x2716f0): clears the two save words
/// at +0x644/+0x648 and advances the save block index at +0x638.
pub fn replay_next_save_block(c: &mut Cpu) {
    c.cycles += 6;
    let r = Addr(c.g[A0][0] as u32);
    c.g[V0][0] = r.off(0x638).lw(c);
    r.off(0x644).sw(c, 0);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    r.off(0x648).sw(c, 0);
    let v0 = c.g[V0][0];
    r.off(0x638).sw(c, v0);
    ret(c);
}

/// `Replay_DebugPrintfStub(a0..a3, t0..t3, f12..f18)` (0x2718a8): a varargs
/// spill of the argument registers into the caller's save area, then
/// returns `a0` in v0.
pub fn replay_debug_printf_stub(c: &mut Cpu) {
    c.cycles += 15;
    let frame = Frame::open(c, 0x80);
    c.g[V0][0] = c.g[A0][0];
    let a1 = c.g[A1][0];
    c.sd(frame.at(c, 0x48), a1);
    let a2 = c.g[A2][0];
    c.sd(frame.at(c, 0x50), a2);
    let a3 = c.g[A3][0];
    c.sd(frame.at(c, 0x58), a3);
    let t0 = c.g[T0][0];
    c.sd(frame.at(c, 0x60), t0);
    let t1 = c.g[T1][0];
    c.sd(frame.at(c, 0x68), t1);
    let t2 = c.g[T2][0];
    c.sd(frame.at(c, 0x70), t2);
    let t3 = c.g[T3][0];
    c.sd(frame.at(c, 0x78), t3);
    c.swc1(12, frame.at(c, 0x38));
    c.swc1(14, frame.at(c, 0x3c));
    c.swc1(16, frame.at(c, 0x40));
    c.swc1(18, frame.at(c, 0x44));
    frame.close(c);
}

/// `Replay_GlobalCtors` (0x2722c0): the static constructor entry, calling
/// `Replay_StaticInitDestroy(1, 0xffff)` (initialise the replay's globals).
pub fn replay_global_ctors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 1;
        c.g[A1][0] = 0xffff;
        CALLEE_REPLAY_STATIC_INIT_DESTROY.call(c, 0x0027_22d4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Replay_SaveWriteCache(Replay* r)` (0x271510): one step of the replay
/// write-out. Emits a pending setup block, or Huffman-encodes the pending
/// command frame and emits it, or moves on to the next input cache.
pub fn replay_save_write_cache(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        'done: {
            c.g[S0][0] = c.g[A0][0];
            let me = c.g[S0][0] as u32;
            c.g[A1][0] = Addr(me).off(0x674).lw(c);
            if c.g[A1][0] != 0 {
                c.cycles += 4;
                c.g[A2][0] = 8;
                CALLEE_REPLAY_SAVE_EMIT.call(c, 0x0027_1534)?;
                Addr(me).off(0x674).sw(c, 0);
                break 'done;
            }
            c.g[A0][0] = Addr(me).off(0x66c).lw(c);

            // A pending command frame: Huffman-encode it into the temporary.
            c.cycles += 2;
            let no_frame = c.g[A0][0] == 0;
            c.g[A3][0] = c.g[SP][0];
            if !no_frame {
                c.cycles += 13;
                c.g[A2][0] = Addr(me).off(0x640).lw(c);
                c.g[A1][0] = Addr(me).off(0x670).lw(c);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
                Addr(sp(c)).sw(c, 0);
                call_ptr(c, 0x002c_a100, 0x0027_1558)?;
                c.g[V1][0] = Addr(me).off(0x640).lw(c);
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.g[S0][0];
                let v0 = c.g[V0][0];
                Addr(c.g[V1][0] as u32).sw(c, v0);
                c.g[A1][0] = Addr(me).off(0x640).lw(c);
                CALLEE_REPLAY_SAVE_EMIT.call(c, 0x0027_1570)?;
                Addr(me).off(0x66c).sw(c, 0);
                break 'done;
            }

            // No command frame: the next input cache, if any.
            c.cycles += 5;
            c.g[V1][0] = Addr(me).off(0x654).lw(c);
            c.g[V0][0] = Addr(me).off(0x658).lw(c);
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let past_end = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
            if past_end {
                // Past the last index: count the block and finish.
                c.cycles += 5;
                c.g[V0][0] = Addr(me).off(0x638).lw(c);
                Addr(me).off(0x644).sw(c, 0);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                Addr(me).off(0x648).sw(c, 0);
                let v0 = c.g[V0][0];
                Addr(me).off(0x638).sw(c, v0);
                break 'done;
            }
            c.cycles += 7;
            Addr(me).off(0x66c).sw(c, 0);
            Addr(me).off(0x674).sw(c, 0);
            c.g[V0][0] = sx(me.wrapping_add(c.g[V0][0] as u32));
            Addr(me).off(0x670).sw(c, 0);
            c.g[A2][0] = Addr(c.g[V0][0] as u32).off(0x48c).lw(c);
            let empty_slot = c.g[A2][0] == 0;
            c.g[A0][0] = 0x8000;
            if !empty_slot {
                // The slot holds a cache: start its command frame.
                c.cycles += 15;
                c.g[V1][0] = Addr(c.g[A2][0] as u32).off(0xc).lw(c);
                c.g[A1][0] = Addr(me).off(0x640).lw(c);
                c.g[V0][0] = 0x1111_0000;
                let v1 = c.g[V1][0];
                Addr(me).off(0x66c).sw(c, v1);
                c.g[V0][0] = c.g[V0][0] | 0x1112;
                let a1 = c.g[A1][0];
                Addr(me).off(0x674).sw(c, a1);
                let a0 = c.g[A0][0];
                Addr(me).off(0x670).sw(c, a0);
                let v0 = c.g[V0][0];
                Addr(c.g[A1][0] as u32).sw(c, v0);
                c.g[A0][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[V1][0] = Addr(me).off(0x674).lw(c);
                let a0 = c.g[A0][0];
                Addr((c.g[V1][0] as u32).wrapping_add(0x4)).sw(c, a0);
                c.g[V0][0] = Addr(me).off(0x654).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                let v0 = c.g[V0][0];
                Addr(me).off(0x654).sw(c, v0);
                break 'done;
            }
            c.cycles += 5;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
            Addr(me).off(0x644).sw(c, 0);
            let v0 = c.g[V0][0];
            Addr(me).off(0x654).sw(c, v0);
            Addr(me).off(0x648).sw(c, 0);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `InSeq_HeapDelete_Thunk(p)` (0x272530): tail call of `InSeq_HeapDelete`.
pub fn insseq_heap_delete_thunk(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(CALLEE_INSEQ_HEAP_DELETE),
        0x0027_2540,
    );
}

/// `InSeq_HeapDelete_Thunk2(p)` (0x272550): the second copy of the thunk.
pub fn insseq_heap_delete_thunk2(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(CALLEE_INSEQ_HEAP_DELETE),
        0x0027_2560,
    );
}

/// `InSeq_FreeAll(self)` (0x272a50): frees every sequence of the channel list
/// (the list head at +0x1c, next at +0x50). The next pointer is loaded
/// before each `InSeq_FreeSequence(self, node)` call.
pub fn insseq_free_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = Addr(c.g[S1][0] as u32).off(0x1c).lw(c);
        if c.g[S0][0] != 0 {
            c.cycles += 2;
            c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                c.g[S0][0] = Addr(c.g[S0][0] as u32).off(0x50).lw(c);
                CALLEE_INSEQ_FREE_SEQUENCE.call(c, 0x0027_2a84)?;
                if c.g[S0][0] == 0 {
                    c.cycles += 1;
                    break;
                }
                c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0027_2a78);
                }
            }
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `InSeq_PlayAll(self, a1, a2)` (0x272b00): `InSeq_Play(self, node, a2)` for
/// every node of the channel list.
pub fn insseq_play_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = Addr(c.g[S1][0] as u32).off(0x1c).lw(c);
        c.g[S2][0] = c.g[A1][0];
        if c.g[S0][0] != 0 {
            c.cycles += 1;
            c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
            loop {
                c.cycles += 6;
                c.g[A0][0] = c.g[S1][0];
                c.g[A2][0] = c.g[S2][0];
                CALLEE_INSEQ_PLAY.call(c, 0x0027_2b34)?;
                c.g[S0][0] = Addr(c.g[S0][0] as u32).off(0x50).lw(c);
                if c.g[S0][0] == 0 {
                    break;
                }
                c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0027_2b28);
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The loop shared by `InSeq_StopAll`, `PauseAll` and `ResumeAll`: calls
/// `callee(self, node)` for every node of the channel list. The next pointer
/// is loaded after the call.
fn insseq_each_channel(c: &mut Cpu, callee: Callee, ret: u32, head: u32) -> Flow {
    c.cycles += 8;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S1, 0x10);
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S0, 0x20);
    c.g[S0][0] = Addr(c.g[S1][0] as u32).off(0x1c).lw(c);
    if c.g[S0][0] != 0 {
        c.cycles += 2;
        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            callee.call(c, ret)?;
            c.g[S0][0] = Addr(c.g[S0][0] as u32).off(0x50).lw(c);
            if c.g[S0][0] == 0 {
                c.cycles += 1;
                break;
            }
            c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
            if event_due(c) {
                deliver_event_here(c, head);
            }
        }
    }
    frame.restore(c, S0, 0x20);
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `InSeq_StopAll(self)` (0x272b80): `InSeq_Stop(self, node)` for each node.
pub fn insseq_stop_all(c: &mut Cpu) {
    guest(c, |c| {
        insseq_each_channel(c, CALLEE_INSEQ_STOP, 0x0027_2bb0, 0x0027_2ba8)
    });
}

/// `InSeq_PauseAll(self)` (0x272bf8): `InSeq_Pause(self, node)` for each node.
pub fn insseq_pause_all(c: &mut Cpu) {
    guest(c, |c| {
        insseq_each_channel(c, CALLEE_INSEQ_PAUSE, 0x0027_2c28, 0x0027_2c20)
    });
}

/// `InSeq_ResumeAll(self)` (0x272c70): `InSeq_Resume(self, node)` for each node.
pub fn insseq_resume_all(c: &mut Cpu) {
    guest(c, |c| {
        insseq_each_channel(c, CALLEE_INSEQ_RESUME, 0x0027_2ca0, 0x0027_2c98)
    });
}

/// `InSeq_Pause(self, id)` (0x272bd0): pauses the player of channel `id`
/// (`InSeq_GetInstance`, then `InstancePause` on it).
pub fn insseq_pause(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        CALLEE_INSEQ_GET_INSTANCE.call(c, 0x0027_2be0)?;
        c.g[A0][0] = c.g[V0][0];
        CALLEE_INSEQ_INSTANCE_PAUSE.call(c, 0x0027_2be8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `InSeq_Resume(self, id)` (0x272c48): resumes the player of channel `id`.
pub fn insseq_resume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        CALLEE_INSEQ_GET_INSTANCE.call(c, 0x0027_2c58)?;
        c.g[A0][0] = c.g[V0][0];
        CALLEE_INSEQ_INSTANCE_RESUME.call(c, 0x0027_2c60)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `InSeq_GetInstanceData(self, id)` (0x272ce0): tail call of
/// `cScriptPlayerMain_GetProjectID`-style accessor (`+4` of the instance).
pub fn insseq_get_instance_data(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(CALLEE_INSEQ_INSTANCE_GET_DATA),
        0x0027_2cf0,
    );
}

/// `InSeq_GetInstanceOwner(self, id)` (0x272d00): tail call of the owner
/// accessor (`+8` of the instance).
pub fn insseq_get_instance_owner(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(CALLEE_INSEQ_INSTANCE_GET_OWNER),
        0x0027_2d10,
    );
}

/// `InSeq_GetSeqChannelCount(self)` (0x272d20): tail call of the channel
/// count accessor (byte at +1 of the instance).
pub fn insseq_get_channel_count(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(CALLEE_INSEQ_GET_CHANNEL_COUNT),
        0x0027_2d30,
    );
}

/// `InSeq_GetChannelBlockField4(self, id)` (0x272d40): the field-4 word of
/// the channel block that `InSeq_FindChannelData` finds.
pub fn insseq_get_channel_block_field4(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        CALLEE_INSEQ_FIND_CHANNEL_DATA.call(c, 0x0027_2d50)?;
        c.g[A0][0] = c.g[V0][0];
        CALLEE_INSEQ_BLOCK_GET_FIELD4.call(c, 0x0027_2d58)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, scramble, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word for this module's callee fakes (test machines only). The
    /// fakes run their gate-off body unless the machine holds this magic.
    pub(crate) const OC534_GATE: u32 = DATA + 0x200000;
    pub(crate) const OC534_MAGIC: u32 = 0x0534_0001;

    /// A gated fake for an installed callee: logs the call (`fakes::fake`)
    /// and returns to `ra`, or runs the body it replaced when the gate is off.
    macro_rules! gated_fake {
        ($fake:ident, $prev:ident, $gen:path, $target:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC534_GATE) != OC534_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $target, ra);
            }
        };
    }

    gated_fake!(
        oc534_fake_save_emit,
        OC534_PREV_SAVE_EMIT,
        ee::s07::f_00271058__generated,
        0x0027_1058
    );
    gated_fake!(
        oc534_fake_huff,
        OC534_PREV_HUFF,
        ee::s09::f_002ca100__generated,
        0x002c_a100
    );
    gated_fake!(
        oc534_fake_heap_delete,
        OC534_PREV_HEAP_DELETE,
        ee::s07::f_002724e8__generated,
        0x0027_24e8
    );
    gated_fake!(
        oc534_fake_free_sequence,
        OC534_PREV_FREE_SEQUENCE,
        ee::s07::f_002729a8__generated,
        0x0027_29a8
    );
    gated_fake!(
        oc534_fake_get_instance,
        OC534_PREV_GET_INSTANCE,
        ee::s07::f_00272cc0__generated,
        0x0027_2cc0
    );
    gated_fake!(
        oc534_fake_play,
        OC534_PREV_PLAY,
        ee::s07::f_00272ad0__generated,
        0x0027_2ad0
    );
    gated_fake!(
        oc534_fake_stop,
        OC534_PREV_STOP,
        ee::s07::f_00272b58__generated,
        0x0027_2b58
    );
    gated_fake!(
        oc534_fake_instance_pause,
        OC534_PREV_INSTANCE_PAUSE,
        ee::s07::f_002749e8__generated,
        0x0027_49e8
    );
    gated_fake!(
        oc534_fake_instance_resume,
        OC534_PREV_INSTANCE_RESUME,
        ee::s07::f_00274a08__generated,
        0x0027_4a08
    );

    /// Hooks every fake of this module (each once per process).
    fn hook_fakes(which: &[&str]) {
        for w in which {
            match *w {
                "save_emit" => fakes::hook(
                    addr_of_mut!(ee::s07::OVERRIDE_00271058),
                    oc534_fake_save_emit,
                    &OC534_PREV_SAVE_EMIT,
                ),
                "huff" => fakes::hook(
                    addr_of_mut!(ee::s09::OVERRIDE_002CA100),
                    oc534_fake_huff,
                    &OC534_PREV_HUFF,
                ),
                "heap_delete" => fakes::hook(
                    addr_of_mut!(ee::s07::OVERRIDE_002724E8),
                    oc534_fake_heap_delete,
                    &OC534_PREV_HEAP_DELETE,
                ),
                "free_sequence" => fakes::hook(
                    addr_of_mut!(ee::s07::OVERRIDE_002729A8),
                    oc534_fake_free_sequence,
                    &OC534_PREV_FREE_SEQUENCE,
                ),
                "get_instance" => fakes::hook(
                    addr_of_mut!(ee::s07::OVERRIDE_00272CC0),
                    oc534_fake_get_instance,
                    &OC534_PREV_GET_INSTANCE,
                ),
                "play" => fakes::hook(
                    addr_of_mut!(ee::s07::OVERRIDE_00272AD0),
                    oc534_fake_play,
                    &OC534_PREV_PLAY,
                ),
                "stop" => fakes::hook(
                    addr_of_mut!(ee::s07::OVERRIDE_00272B58),
                    oc534_fake_stop,
                    &OC534_PREV_STOP,
                ),
                "instance_pause" => fakes::hook(
                    addr_of_mut!(ee::s07::OVERRIDE_002749E8),
                    oc534_fake_instance_pause,
                    &OC534_PREV_INSTANCE_PAUSE,
                ),
                "instance_resume" => fakes::hook(
                    addr_of_mut!(ee::s07::OVERRIDE_00274A08),
                    oc534_fake_instance_resume,
                    &OC534_PREV_INSTANCE_RESUME,
                ),
                other => panic!("unknown fake {other}"),
            }
        }
    }

    /// Machine state with the fakes gated on, registers scrambled.
    fn gate_on(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = fakes::machine(c, seed, salt, 0);
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(OC534_GATE, OC534_MAGIC);
        r
    }

    /// A channel list of `seed % 4` nodes behind the self object at DATA+0x1000
    /// (head at +0x1c, next at +0x50, data word at +4).
    fn chain_setup(c: &mut Cpu, seed: u32, salt: u64, entry: u32) -> Rng {
        let mut r = gate_on(c, seed, salt);
        let me = DATA + 0x1000;
        let n = seed % 4;
        let node = |i: u32| DATA + 0x2000 + 0x100 * i;
        for i in 0..n {
            let next = if i + 1 < n { node(i + 1) } else { 0 };
            c.bus.write32(node(i) + 0x50, next);
            c.bus.write32(node(i) + 0x4, r.next_u32());
        }
        c.bus.write32(me + 0x1c, if n > 0 { node(0) } else { 0 });
        c.g[A0][0] = sx(me);
        c.g[A1][0] = r.next_u32() as u64;
        c.g[A2][0] = r.next_u32() as u64;
        c.pc = entry;
        r
    }

    #[test]
    fn replay_next_save_block_matches() {
        check(
            ee::s07::f_002716f0__generated,
            replay_next_save_block,
            200,
            &|c, seed| {
                let mut r = Rng(0x0027_16f0 ^ seed as u64);
                scramble(c, &mut r);
                let base = DATA + 0x1000 * (seed % 8);
                c.g[A0][0] = sx(base);
                c.bus.write32(base + 0x638, r.next_u32());
                c.bus.write32(base + 0x644, r.next_u32());
                c.bus.write32(base + 0x648, r.next_u32());
                c.pc = 0x0027_16f0;
            },
        );
    }

    #[test]
    fn replay_debug_printf_stub_matches() {
        check(
            ee::s07::f_002718a8__generated,
            replay_debug_printf_stub,
            200,
            &|c, seed| {
                let mut r = Rng(0x0027_18a8 ^ seed as u64);
                scramble(c, &mut r);
                for f in 12..=18 {
                    c.f[f] = r.next_u32();
                }
                c.pc = 0x0027_18a8;
            },
        );
    }

    #[test]
    fn replay_global_ctors_matches() {
        check(
            ee::s07::f_002722c0__generated,
            replay_global_ctors,
            60,
            &|c, seed| {
                let mut r = Rng(0x0027_22c0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x0027_22c0;
            },
        );
    }

    #[test]
    fn replay_save_write_cache_matches() {
        hook_fakes(&["save_emit", "huff"]);
        check(
            ee::s07::f_00271510__generated,
            replay_save_write_cache,
            200,
            &|c, seed| {
                let mut r = gate_on(c, seed, 0x0027_1510);
                let me = DATA + 0x1000;
                c.g[A0][0] = sx(me);
                c.bus.write32(me + 0x640, DATA + 0xb000);
                c.bus.write32(me + 0x670, 0x100);
                // The five branches, by seed.
                let (frame, emit, idx, max) = match seed % 5 {
                    0 => (0, DATA + 0xd000, 0, 4),
                    1 => (DATA + 0xc000, 0, 0, 4),
                    2 | 3 => (0, 0, 0, 4),
                    _ => (0, 0, 5, 2),
                };
                c.bus.write32(me + 0x66c, frame);
                c.bus.write32(me + 0x674, emit);
                c.bus.write32(me + 0x654, idx);
                c.bus.write32(me + 0x658, max);
                c.bus.write32(me + 0x638, r.next_u32() & 0xff);
                if seed % 5 == 2 {
                    // A cache in the first slot: its command frame and record cursor.
                    let cache = DATA + 0xe000;
                    c.bus.write32(me + 0x48c, cache);
                    c.bus.write32(cache, r.next_u32());
                    c.bus.write32(cache + 0xc, r.next_u32());
                }
                c.pc = 0x0027_1510;
            },
        );
    }

    #[test]
    fn insseq_heap_delete_thunk_matches() {
        hook_fakes(&["heap_delete"]);
        check(
            ee::s07::f_00272530__generated,
            insseq_heap_delete_thunk,
            200,
            &|c, seed| {
                gate_on(c, seed, 0x0027_2530);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.pc = 0x0027_2530;
            },
        );
    }

    #[test]
    fn insseq_heap_delete_thunk2_matches() {
        hook_fakes(&["heap_delete"]);
        check(
            ee::s07::f_00272550__generated,
            insseq_heap_delete_thunk2,
            200,
            &|c, seed| {
                gate_on(c, seed, 0x0027_2550);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.pc = 0x0027_2550;
            },
        );
    }

    #[test]
    fn insseq_free_all_matches() {
        hook_fakes(&["free_sequence"]);
        check(
            ee::s07::f_00272a50__generated,
            insseq_free_all,
            200,
            &|c, seed| {
                chain_setup(c, seed, 0x0027_2a50, 0x0027_2a50);
            },
        );
    }

    #[test]
    fn insseq_play_all_matches() {
        hook_fakes(&["play"]);
        check(
            ee::s07::f_00272b00__generated,
            insseq_play_all,
            200,
            &|c, seed| {
                chain_setup(c, seed, 0x0027_2b00, 0x0027_2b00);
            },
        );
    }

    #[test]
    fn insseq_stop_all_matches() {
        hook_fakes(&["stop"]);
        check(
            ee::s07::f_00272b80__generated,
            insseq_stop_all,
            200,
            &|c, seed| {
                chain_setup(c, seed, 0x0027_2b80, 0x0027_2b80);
            },
        );
    }

    #[test]
    fn insseq_pause_matches() {
        hook_fakes(&["get_instance", "instance_pause"]);
        check(
            ee::s07::f_00272bd0__generated,
            insseq_pause,
            200,
            &|c, seed| {
                gate_on(c, seed, 0x0027_2bd0);
                // The instance `InSeq_GetInstance` returns (fake call 0).
                c.bus.write32(fakes::LOG_RESULTS, DATA + 0x3000);
                c.pc = 0x0027_2bd0;
            },
        );
    }

    #[test]
    fn insseq_pause_all_matches() {
        // The generated `InSeq_Pause` runs for real (unported): both sides
        // call it, so the fakes below serve its two callees.
        hook_fakes(&["get_instance", "instance_pause"]);
        check(
            ee::s07::f_00272bf8__generated,
            insseq_pause_all,
            200,
            &|c, seed| {
                chain_setup(c, seed, 0x0027_2bf8, 0x0027_2bf8);
                c.bus.write32(fakes::LOG_RESULTS, DATA + 0x3000);
            },
        );
    }

    #[test]
    fn insseq_resume_matches() {
        hook_fakes(&["get_instance", "instance_resume"]);
        check(
            ee::s07::f_00272c48__generated,
            insseq_resume,
            200,
            &|c, seed| {
                gate_on(c, seed, 0x0027_2c48);
                c.bus.write32(fakes::LOG_RESULTS, DATA + 0x3000);
                c.pc = 0x0027_2c48;
            },
        );
    }

    #[test]
    fn insseq_resume_all_matches() {
        hook_fakes(&["get_instance", "instance_resume"]);
        check(
            ee::s07::f_00272c70__generated,
            insseq_resume_all,
            200,
            &|c, seed| {
                chain_setup(c, seed, 0x0027_2c70, 0x0027_2c70);
                c.bus.write32(fakes::LOG_RESULTS, DATA + 0x3000);
            },
        );
    }

    #[test]
    fn insseq_get_instance_data_matches() {
        check(
            ee::s07::f_00272ce0__generated,
            insseq_get_instance_data,
            200,
            &|c, seed| {
                let mut r = gate_on(c, seed, 0x0027_2ce0);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1004, r.next_u32());
                c.pc = 0x0027_2ce0;
            },
        );
    }

    #[test]
    fn insseq_get_instance_owner_matches() {
        check(
            ee::s07::f_00272d00__generated,
            insseq_get_instance_owner,
            200,
            &|c, seed| {
                let mut r = gate_on(c, seed, 0x0027_2d00);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1008, r.next_u32());
                c.pc = 0x0027_2d00;
            },
        );
    }

    #[test]
    fn insseq_get_channel_count_matches() {
        check(
            ee::s07::f_00272d20__generated,
            insseq_get_channel_count,
            200,
            &|c, seed| {
                let mut r = gate_on(c, seed, 0x0027_2d20);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000, r.next_u32());
                c.pc = 0x0027_2d20;
            },
        );
    }

    #[test]
    fn insseq_get_channel_block_field4_matches() {
        check(
            ee::s07::f_00272d40__generated,
            insseq_get_channel_block_field4,
            200,
            &|c, seed| {
                let mut r = gate_on(c, seed, 0x0027_2d40);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = (seed % 3) as u64;
                c.bus.write32(DATA + 0x1000, r.next_u32() & 0xff);
                c.pc = 0x0027_2d40;
            },
        );
    }
}
