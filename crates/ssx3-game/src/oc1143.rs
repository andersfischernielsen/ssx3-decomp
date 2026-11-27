//! Staging module oc1143 (wave wave272): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1143`.

#![allow(unused_imports)]

use crate::abi;
use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event, deliver_event_here, event_due};
use crate::guest::Addr;
use crate::sound::snd;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003b_69d8, snd_strm_close_all, "SNDSTRM_CloseAll"),
    (0x003b_7410, snd_strm_count_open, "SNDSTRM_CountOpen"),
    (0x003b_7838, snd_strm_close, "SNDSTRM_Close"),
    (0x003b_7ac8, snd_strm_calc_mem_size, "SNDSTRM_CalcMemSize"),
    (0x003b_7b00, snd_strm_calc_mem_size_ex, "SNDSTRM_CalcMemSizeEx"),
    (
        0x003b_80e0,
        snd_strm_set_channel_param6a,
        "SNDSTRM_SetChannelParam6a",
    ),
    (0x003b_8120, snd_strm_set_low_water, "SNDSTRM_SetLowWater"),
    (0x003b_8838, snd_set_channel_param6a, "SND_SetChannelParam6a"),
];

/// `SNDSTRM_*` return value for a handle with no open stream (-8).
const SNDSTRM_ERR_HANDLE: u64 = -8i64 as u64;

/// `u8` in the sound state (`D_0050AAD4`): cleared when the last stream closes
/// and the service client is removed.
const SERVICE_CLIENT_FLAG: u32 = 0x0050_aad4;

/// Callees, by address (each called through the function table).
const SNDSTRM_GET_STREAM: u32 = 0x003b_6a38;
const SNDSTRM_STOP: u32 = 0x003b_78f8;
const SNDSTRM_CLOSE: u32 = 0x003b_7838;
const SNDSTRM_COUNT_OPEN: u32 = 0x003b_7410;
const SNDSTRM_CALC_MEM_SIZE: u32 = 0x003b_7ac8;
const SNDSTRM_CALC_SLOT_MEM_SIZE: u32 = 0x003c_65f0;
const SNDSTRM_FREE_SLOT: u32 = 0x003c_6da0;
const SND_REMOVE_SERVICE_CALLBACK: u32 = 0x003c_6040;
const STREAM_DESTROY: u32 = 0x003e_0a28;
const STREAM_CALC_MEM_SIZE: u32 = 0x003e_06b0;
const STREAM_SET_LOW_WATER: u32 = 0x003e_0b28;
const SND_SET_CHANNEL_PARAM6A: u32 = 0x003b_8838;
const SND_HANDLE_TO_CHANNEL: u32 = 0x003b_a6e8;
const SND_NEXT_HANDLE_CHANNEL: u32 = 0x003b_aa00;
const SND_CHANNEL_APPLY_TIME_STRETCH: u32 = 0x003c_3010;

/// `SNDSTRM_CloseAll` (0x3b69d8): closes every stream slot below the stream
/// count, in index order.
pub fn snd_strm_close_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        c.g[V1][0] = sx(snd::STATE.0);
        frame.save(c, 17, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = snd::STREAM_COUNT.lbu(c);
        let none = c.g[V0][0] == 0;
        c.g[S0][0] = 0;
        if !none {
            c.cycles += 1;
            c.g[S1][0] = c.g[V1][0];
            loop {
                c.cycles += 7;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, SNDSTRM_CLOSE, 0x003b_6a08)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x46).lbu(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_6a00);
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, 16, 0x20);
        c.g[V0][0] = 0;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `SNDSTRM_CountOpen` (0x3b7410): how many of the first `stream count`
/// stream pointers are set.
pub fn snd_strm_count_open(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = snd::STATE_HI;
    c.g[A1][0] = snd::STREAM_COUNT.lbu(c);
    let none = c.g[A1][0] == 0;
    c.g[A0][0] = 0;
    if !none {
        c.cycles += 2;
        c.g[V0][0] = snd::STATE_HI;
        c.g[A2][0] = sx(snd::STREAMS.0);
        loop {
            c.cycles += 7;
            c.g[V1][0] = c.lw(c.g[A2][0] as u32);
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(4));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
            let more = c.g[A1][0] != 0;
            if c.g[V1][0] != 0 {
                c.g[A0][0] = c.g[V0][0];
            }
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003b_7428);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    abi::ret(c);
}

/// `SNDSTRM_Close` (0x3b7838) of stream handle a0: stops the stream, frees
/// its slot and its object (unless the flag byte says it is borrowed). Returns
/// -8 for a handle with no stream, else 0.
pub fn snd_strm_close(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 17, 0x10);
        frame.save(c, 16, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, SNDSTRM_GET_STREAM, 0x003b_7850)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[V0][0] = SNDSTRM_ERR_HANDLE;
        if c.g[S0][0] != 0 {
            c.cycles += 7;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, SNDSTRM_STOP, 0x003b_7864)?;
            call_ptr(c, SNDSTRM_COUNT_OPEN, 0x003b_786c)?;
            c.g[V1][0] = 1;
            c.g[A0][0] = 0x003b_0000;
            if c.g[V0][0] == c.g[V1][0] {
                // The last open stream: drop the service client too.
                c.cycles += 4;
                c.g[A0][0] = sx(0x003b_0000u32.wrapping_add(0x71d8));
                call_ptr(c, SND_REMOVE_SERVICE_CALLBACK, 0x003b_7880)?;
                c.g[V0][0] = snd::STATE_HI;
                c.sw(SERVICE_CLIENT_FLAG, 0);
            }
            c.cycles += 12;
            c.g[A0][0] = Addr(c.g[S0][0] as u32).off(8).lw(c);
            call_ptr(c, SNDSTRM_FREE_SLOT, 0x003b_7894)?;
            c.g[V0][0] = sx(snd::STREAMS.0);
            c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x11).lbu(c);
            c.g[V1][0] = sx((c.g[S1][0] as u32) << 2);
            c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 24);
            let borrowed = c.g[A0][0] != 0;
            c.sw(c.g[V1][0] as u32, 0);
            if !borrowed {
                c.cycles += 2;
                c.g[A0][0] = c.g[A1][0];
                call_ptr(c, STREAM_DESTROY, 0x003b_78c0)?;
            }
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `SNDSTRM_CalcMemSize` (0x3b7ac8): the memory for `maxRequests` (a0) stream
/// requests plus the slot memory for `maxChunks` (a1).
pub fn snd_strm_calc_mem_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x28;
        frame.save(c, 16, 0x10);
        c.mult(0, 16, 4, 2);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x138));
        call_ptr(c, SNDSTRM_CALC_SLOT_MEM_SIZE, 0x003b_7ae8)?;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.restore_ra(c, 0);
        frame.restore(c, 16, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `SNDSTRM_CalcMemSizeEx` (0x3b7b00): `SNDSTRM_CalcMemSize` plus the stream
/// file's memory for `maxRequests + 2` buffers.
pub fn snd_strm_calc_mem_size_ex(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, SNDSTRM_CALC_MEM_SIZE, 0x003b_7b18)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(2));
        c.g[A1][0] = 1;
        c.g[A2][0] = 1;
        call_ptr(c, STREAM_CALC_MEM_SIZE, 0x003b_7b2c)?;
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `SNDSTRM_SetChannelParam6a` (0x3b80e0) of stream a0 with parameter a1: the
/// stream's channel gets it (through `SND_SetChannelParam6a`) and the
/// stream's halfword at +0xfa. Returns -8 for a handle with no stream.
pub fn snd_strm_set_channel_param6a(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, SNDSTRM_GET_STREAM, 0x003b_80f4)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = SNDSTRM_ERR_HANDLE;
        } else {
            c.cycles += 5;
            let stream = Addr(c.g[V0][0] as u32);
            c.g[A0][0] = stream.off(4).lw(c);
            c.g[A1][0] = c.g[S0][0];
            c.sh(stream.0.wrapping_add(0xfa), c.g[S0][0]);
            call_ptr(c, SND_SET_CHANNEL_PARAM6A, 0x003b_810c)?;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `SNDSTRM_SetLowWater` (0x3b8120) of stream a0 with low-water mark a1: the
/// stream file gets it. Returns -8 for a handle with no stream.
pub fn snd_strm_set_low_water(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, SNDSTRM_GET_STREAM, 0x003b_8134)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = SNDSTRM_ERR_HANDLE;
        } else {
            c.cycles += 4;
            c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, STREAM_SET_LOW_WATER, 0x003b_8148)?;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `SND_SetChannelParam6a` (0x3b8838) of channel handle a0 with parameter a1:
/// every voice channel that handle is playing on gets the parameter as its
/// halfword at +0x6a (time-stretch applied). Always returns 0.
pub fn snd_set_channel_param6a(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        frame.save(c, 16, 0x50);
        frame.save(c, 17, 0x40);
        frame.save(c, 18, 0x30);
        frame.save(c, 19, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, SND_HANDLE_TO_CHANNEL, 0x003b_8858)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V1][0] = u64::MAX;
        if (c.g[S1][0] as i64) >= 0 {
            c.cycles += 5;
            c.g[V0][0] = snd::STATE_HI;
            c.g[S3][0] = sx(snd::STATE.0);
            let setchan = frame.at(c, 0);
            c.sw(setchan, c.g[V1][0]);
            c.g[S2][0] = 0x8c;
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, SND_NEXT_HANDLE_CHANNEL, 0x003b_889c)?;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = c.lw(setchan);
                if !more {
                    break;
                }
                if event_due(c) && deliver_event(c, 0x003b_8878) {
                    return Ok(());
                }
                c.cycles += 6;
                c.g[A1][0] = c.g[S0][0];
                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1f0));
                c.mult(0, 2, 4, 18);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.sh((c.g[V0][0] as u32).wrapping_add(0x6a), c.g[S0][0]);
                call_ptr(c, SND_CHANNEL_APPLY_TIME_STRETCH, 0x003b_8890)?;
            }
        }
        c.cycles += 8;
        frame.restore(c, 16, 0x50);
        c.g[V0][0] = 0;
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, scramble, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// A fake answers from the real body unless its gate word holds this.
    const OC1143_MAGIC: u32 = 0x1143_c0de;
    /// One gate word per fake, 4 bytes apart.
    const GATE_CLOSE: u32 = DATA + 0x0020_0000;
    const GATE_STOP: u32 = GATE_CLOSE + 0x04;
    const GATE_REMOVE: u32 = GATE_CLOSE + 0x08;
    const GATE_FREE: u32 = GATE_CLOSE + 0x0c;
    const GATE_DESTROY: u32 = GATE_CLOSE + 0x10;
    const GATE_CALC_SLOT: u32 = GATE_CLOSE + 0x14;
    const GATE_CALC_STREAM: u32 = GATE_CLOSE + 0x18;
    const GATE_LOW_WATER: u32 = GATE_CLOSE + 0x1c;
    const GATE_HANDLE: u32 = GATE_CLOSE + 0x20;
    const GATE_NEXT: u32 = GATE_CLOSE + 0x24;
    const GATE_STRETCH: u32 = GATE_CLOSE + 0x28;
    /// Channel mailbox: the handle the handle fake returns, the channel count,
    /// the next-channel cursor, then the channel indices.
    const MAILBOX_HANDLE: u32 = GATE_CLOSE + 0x40;
    const MAILBOX_COUNT: u32 = GATE_CLOSE + 0x44;
    const MAILBOX_CURSOR: u32 = GATE_CLOSE + 0x48;
    const MAILBOX_CHANNELS: u32 = GATE_CLOSE + 0x50;
    /// Stream objects (0x200 apart) and the voice channel array.
    const STREAM_OBJECTS: u32 = DATA + 0x1000;
    const CHANNEL_ARRAY: u32 = DATA + 0x4000;

    static OC1143_PREV_CLOSE: AtomicUsize = AtomicUsize::new(0);
    static OC1143_PREV_STOP: AtomicUsize = AtomicUsize::new(0);
    static OC1143_PREV_REMOVE: AtomicUsize = AtomicUsize::new(0);
    static OC1143_PREV_FREE: AtomicUsize = AtomicUsize::new(0);
    static OC1143_PREV_DESTROY: AtomicUsize = AtomicUsize::new(0);
    static OC1143_PREV_CALC_SLOT: AtomicUsize = AtomicUsize::new(0);
    static OC1143_PREV_CALC_STREAM: AtomicUsize = AtomicUsize::new(0);
    static OC1143_PREV_LOW_WATER: AtomicUsize = AtomicUsize::new(0);
    static OC1143_PREV_HANDLE: AtomicUsize = AtomicUsize::new(0);
    static OC1143_PREV_NEXT: AtomicUsize = AtomicUsize::new(0);
    static OC1143_PREV_STRETCH: AtomicUsize = AtomicUsize::new(0);

    fn oc1143_fake_close(c: &mut Cpu) {
        if c.bus.read32(GATE_CLOSE) != OC1143_MAGIC {
            return fakes::fall_back(c, &OC1143_PREV_CLOSE, ee::s13::f_003b7838__generated);
        }
        fakes::fake_call(c, SNDSTRM_CLOSE);
    }

    fn oc1143_fake_stop(c: &mut Cpu) {
        if c.bus.read32(GATE_STOP) != OC1143_MAGIC {
            return fakes::fall_back(c, &OC1143_PREV_STOP, ee::s13::f_003b78f8__generated);
        }
        fakes::fake_call(c, SNDSTRM_STOP);
    }

    fn oc1143_fake_remove(c: &mut Cpu) {
        if c.bus.read32(GATE_REMOVE) != OC1143_MAGIC {
            return fakes::fall_back(c, &OC1143_PREV_REMOVE, ee::s13::f_003c6040__generated);
        }
        fakes::fake_call(c, SND_REMOVE_SERVICE_CALLBACK);
    }

    fn oc1143_fake_free(c: &mut Cpu) {
        if c.bus.read32(GATE_FREE) != OC1143_MAGIC {
            return fakes::fall_back(c, &OC1143_PREV_FREE, ee::s13::f_003c6da0__generated);
        }
        fakes::fake_call(c, SNDSTRM_FREE_SLOT);
    }

    fn oc1143_fake_destroy(c: &mut Cpu) {
        if c.bus.read32(GATE_DESTROY) != OC1143_MAGIC {
            return fakes::fall_back(c, &OC1143_PREV_DESTROY, ee::s14::f_003e0a28__generated);
        }
        fakes::fake_call(c, STREAM_DESTROY);
    }

    fn oc1143_fake_calc_slot(c: &mut Cpu) {
        if c.bus.read32(GATE_CALC_SLOT) != OC1143_MAGIC {
            return fakes::fall_back(c, &OC1143_PREV_CALC_SLOT, ee::s13::f_003c65f0__generated);
        }
        fakes::fake_call(c, SNDSTRM_CALC_SLOT_MEM_SIZE);
    }

    fn oc1143_fake_calc_stream(c: &mut Cpu) {
        if c.bus.read32(GATE_CALC_STREAM) != OC1143_MAGIC {
            return fakes::fall_back(c, &OC1143_PREV_CALC_STREAM, ee::s14::f_003e06b0__generated);
        }
        fakes::fake_call(c, STREAM_CALC_MEM_SIZE);
    }

    fn oc1143_fake_low_water(c: &mut Cpu) {
        if c.bus.read32(GATE_LOW_WATER) != OC1143_MAGIC {
            return fakes::fall_back(c, &OC1143_PREV_LOW_WATER, ee::s14::f_003e0b28__generated);
        }
        fakes::fake_call(c, STREAM_SET_LOW_WATER);
    }

    /// `SND_HandleToChannel`: the mailbox's handle, sign-extended.
    fn oc1143_fake_handle(c: &mut Cpu) {
        if c.bus.read32(GATE_HANDLE) != OC1143_MAGIC {
            return fakes::fall_back(c, &OC1143_PREV_HANDLE, ee::s13::f_003ba6e8__generated);
        }
        c.g[V0][0] = sx(c.bus.read32(MAILBOX_HANDLE));
        abi::ret(c);
    }

    /// `SND_NextHandleChannel`: hands out the mailbox's channels in order,
    /// writing each into the `int*` in a1; 0 when none is left.
    fn oc1143_fake_next(c: &mut Cpu) {
        if c.bus.read32(GATE_NEXT) != OC1143_MAGIC {
            return fakes::fall_back(c, &OC1143_PREV_NEXT, ee::s13::f_003baa00__generated);
        }
        let cursor = c.bus.read32(MAILBOX_CURSOR);
        if cursor < c.bus.read32(MAILBOX_COUNT) {
            let channel = c.bus.read32(MAILBOX_CHANNELS + 4 * cursor);
            c.bus.write32(MAILBOX_CURSOR, cursor + 1);
            c.bus.write32(c.g[A1][0] as u32, channel);
            c.g[V0][0] = 1;
        } else {
            c.g[V0][0] = 0;
        }
        abi::ret(c);
    }

    fn oc1143_fake_stretch(c: &mut Cpu) {
        if c.bus.read32(GATE_STRETCH) != OC1143_MAGIC {
            return fakes::fall_back(c, &OC1143_PREV_STRETCH, ee::s13::f_003c3010__generated);
        }
        fakes::fake_call(c, SND_CHANNEL_APPLY_TIME_STRETCH);
    }

    /// Installs the fakes in their override slots (once per process).
    fn oc1143_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B7838),
            oc1143_fake_close,
            &OC1143_PREV_CLOSE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B78F8),
            oc1143_fake_stop,
            &OC1143_PREV_STOP,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003C6040),
            oc1143_fake_remove,
            &OC1143_PREV_REMOVE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003C6DA0),
            oc1143_fake_free,
            &OC1143_PREV_FREE,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E0A28),
            oc1143_fake_destroy,
            &OC1143_PREV_DESTROY,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003C65F0),
            oc1143_fake_calc_slot,
            &OC1143_PREV_CALC_SLOT,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E06B0),
            oc1143_fake_calc_stream,
            &OC1143_PREV_CALC_STREAM,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E0B28),
            oc1143_fake_low_water,
            &OC1143_PREV_LOW_WATER,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003BA6E8),
            oc1143_fake_handle,
            &OC1143_PREV_HANDLE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003BAA00),
            oc1143_fake_next,
            &OC1143_PREV_NEXT,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003C3010),
            oc1143_fake_stretch,
            &OC1143_PREV_STRETCH,
        );
    }

    /// Opens every gate: the fakes answer in this machine.
    fn oc1143_gates(c: &mut Cpu) {
        for gate in [
            GATE_CLOSE,
            GATE_STOP,
            GATE_REMOVE,
            GATE_FREE,
            GATE_DESTROY,
            GATE_CALC_SLOT,
            GATE_CALC_STREAM,
            GATE_LOW_WATER,
            GATE_HANDLE,
            GATE_NEXT,
            GATE_STRETCH,
        ] {
            c.bus.write32(gate, OC1143_MAGIC);
        }
    }

    /// A scrambled machine and the seed's RNG.
    fn start(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = Rng(0x9e37_79b9_7f4a_7c15 ^ ((seed as u64) << 20) ^ salt);
        scramble(c, &mut r);
        r
    }

    /// The stream object of handle `h`.
    fn stream_object(h: u32) -> u32 {
        STREAM_OBJECTS + 0x200 * h
    }

    /// The stream table: the first `live` entries point at fixture objects,
    /// the rest are empty. Each object gets random fields; its flag byte is
    /// `flag` (nonzero: the stream is borrowed and not destroyed).
    fn streams(c: &mut Cpu, r: &mut Rng, live: u32, flag: u8) {
        for h in 0..8u32 {
            let s = stream_object(h);
            let entry = if h < live { s } else { 0 };
            c.bus.write32(snd::STREAMS.0 + 4 * h, entry);
            c.bus.write32(s, r.next_u32());
            c.bus.write32(s + 4, r.next_u32());
            c.bus.write32(s + 8, r.next_u32());
            c.bus.write8(s + 0x11, flag);
        }
    }

    /// The channel mailbox read by the handle and next-channel fakes.
    fn mailbox(c: &mut Cpu, handle: u32, channels: &[u32]) {
        c.bus.write32(MAILBOX_HANDLE, handle);
        c.bus.write32(MAILBOX_COUNT, channels.len() as u32);
        c.bus.write32(MAILBOX_CURSOR, 0);
        for (i, channel) in channels.iter().enumerate() {
            c.bus.write32(MAILBOX_CHANNELS + 4 * i as u32, *channel);
        }
        c.bus.write32(snd::CHANNELS.0, CHANNEL_ARRAY);
    }

    #[test]
    fn snd_strm_close_all_matches() {
        check(
            ee::s13::f_003b69d8__generated,
            snd_strm_close_all,
            200,
            &|c, seed| {
                start(c, seed, 1);
                oc1143_install_fakes();
                oc1143_gates(c);
                c.bus.write8(snd::STREAM_COUNT.0, (seed % 5) as u8);
            },
        );
    }

    #[test]
    fn snd_strm_count_open_matches() {
        check(
            ee::s13::f_003b7410__generated,
            snd_strm_count_open,
            200,
            &|c, seed| {
                let mut r = start(c, seed, 2);
                c.bus.write8(snd::STREAM_COUNT.0, (seed % 6) as u8);
                for h in 0..8u32 {
                    let v = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        r.next_u32() | 1
                    };
                    c.bus.write32(snd::STREAMS.0 + 4 * h, v);
                }
            },
        );
    }

    #[test]
    fn snd_strm_close_matches() {
        check(
            ee::s13::f_003b7838__generated,
            snd_strm_close,
            300,
            &|c, seed| {
                let mut r = start(c, seed, 3);
                oc1143_install_fakes();
                oc1143_gates(c);
                c.bus.write8(snd::STREAM_COUNT.0, (seed % 5) as u8);
                let live = (seed / 3) % 5;
                let flag = ((seed / 5) % 2) as u8;
                streams(c, &mut r, live, flag);
                c.g[A0][0] = sx(seed % 6);
            },
        );
    }

    #[test]
    fn snd_strm_calc_mem_size_matches() {
        check(
            ee::s13::f_003b7ac8__generated,
            snd_strm_calc_mem_size,
            200,
            &|c, seed| {
                let mut r = start(c, seed, 4);
                oc1143_install_fakes();
                oc1143_gates(c);
                c.g[A0][0] = sx(r.next_u32());
                c.g[A1][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn snd_strm_calc_mem_size_ex_matches() {
        check(
            ee::s13::f_003b7b00__generated,
            snd_strm_calc_mem_size_ex,
            200,
            &|c, seed| {
                let mut r = start(c, seed, 5);
                oc1143_install_fakes();
                oc1143_gates(c);
                c.g[A0][0] = sx(r.next_u32());
                c.g[A1][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn snd_strm_set_channel_param6a_matches() {
        check(
            ee::s13::f_003b80e0__generated,
            snd_strm_set_channel_param6a,
            300,
            &|c, seed| {
                let mut r = start(c, seed, 6);
                oc1143_install_fakes();
                oc1143_gates(c);
                c.bus.write8(snd::STREAM_COUNT.0, (seed % 5) as u8);
                streams(c, &mut r, (seed / 5) % 4, 0);
                c.g[A0][0] = sx(seed % 6);
                c.g[A1][0] = sx(r.next_u32());
                let handle = if seed % 7 == 0 {
                    r.next_u32() | 0x8000_0000
                } else {
                    r.next_u32() % 8
                };
                let n = (seed / 11) % 3;
                let channels: Vec<u32> = (0..n).map(|_| r.next_u32() % 8).collect();
                mailbox(c, handle, &channels);
            },
        );
    }

    #[test]
    fn snd_strm_set_low_water_matches() {
        check(
            ee::s13::f_003b8120__generated,
            snd_strm_set_low_water,
            300,
            &|c, seed| {
                let mut r = start(c, seed, 7);
                oc1143_install_fakes();
                oc1143_gates(c);
                c.bus.write8(snd::STREAM_COUNT.0, (seed % 5) as u8);
                streams(c, &mut r, (seed / 5) % 4, 0);
                c.g[A0][0] = sx(seed % 6);
                c.g[A1][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn snd_set_channel_param6a_matches() {
        check(
            ee::s13::f_003b8838__generated,
            snd_set_channel_param6a,
            300,
            &|c, seed| {
                let mut r = start(c, seed, 8);
                oc1143_install_fakes();
                oc1143_gates(c);
                c.pc = 0x003b_8838;
                c.g[A0][0] = sx(r.next_u32() % 8);
                c.g[A1][0] = sx(r.next_u32());
                let handle = if seed % 4 == 0 {
                    r.next_u32() | 0x8000_0000
                } else {
                    r.next_u32() % 8
                };
                let n = (seed / 4) % 4;
                let channels: Vec<u32> = (0..n).map(|_| r.next_u32() % 8).collect();
                mailbox(c, handle, &channels);
            },
        );
    }
}
