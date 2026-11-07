//! Staging wave 211 (oc295): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0010_3480, focus_man_save_state, "FocusMan_SaveState"),
    (0x003d_e7b0, ea_file_free_request, "EAFile_FreeRequest"),
    (
        0x0011_9e38,
        rider_metrix_interrupt_trick,
        "RiderMetrix_InterruptTrick",
    ),
    (
        0x0030_2968,
        wscript_fn_set_self_object,
        "WScriptFn_SetSelfObject",
    ),
    (
        0x0035_5550,
        mod_obj_set_uv_scroll_modifier,
        "ModObj_SetUVScrollModifier",
    ),
    (0x0041_2608, fixdfsi, "__fixdfsi"),
    (0x0041_2280, divdf3, "__divdf3"),
    (0x0034_41a8, crowd_man3d_save_state, "CrowdMan3d_SaveState"),
    (
        0x0035_30d0,
        modifier_block_free_transform,
        "ModifierBlock_FreeTransform",
    ),
    (
        0x0010_65b0,
        rider_deflect_along_wall,
        "Rider_DeflectAlongWall",
    ),
];

// ---------------------------------------------------------------------------
// FocusMan_SaveState (0x103480)

/// FocusMan: entry count at +0x0, 0x20-byte entries from +0x20, a word at
/// +0xd0, value count at +0xd8, 32-bit values from +0xdc.
const FOCUS_COUNT: u32 = 0x0;
const FOCUS_ENTRIES: u32 = 0x20;
const FOCUS_HEADER: u32 = 0xd0;
const FOCUS_VALUE_COUNT: u32 = 0xd8;
const FOCUS_VALUES: u32 = 0xdc;

/// One `cStream` virtual call from FocusMan_SaveState: `this` adjusted by the
/// halfword at `vt + delta_off`, the method at `vt + fn_off` (s1 holds the
/// stream); v0/v1/a0 left as the original leaves them.
fn focus_man_stream_vcall(c: &mut Cpu, delta_off: u32, fn_off: u32, ret: u32) -> crate::call::Flow {
    let vt = c.g[V0][0] as u32;
    c.g[A0][0] = c.lh(vt.wrapping_add(delta_off));
    c.g[V1][0] = c.lw(vt.wrapping_add(fn_off));
    let method = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, method, ret)
}

/// `void FocusMan::SaveState(cStream* s)` (0x103480): writes the header word
/// (+0xd0), each of the `count` 0x10-byte focus entries (stride 0x20), the
/// value count (+0xd8) and then every value with `s->WriteInt`. The counts
/// are re-read after every call (s3 = this, s1 = stream, s2 = index, s0 =
/// cursor).
pub fn focus_man_save_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x50);
        c.g[A2][0] = 0x4;
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = 0;
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        // s->Write(&this->header, 4)
        c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(FOCUS_HEADER));
        c.g[V0][0] = c.lw(c.g[S1][0] as u32);
        focus_man_stream_vcall(c, 0x8, 0xc, 0x0010_34c0)?;
        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(FOCUS_COUNT));
        c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(FOCUS_ENTRIES));
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 1;
            c.g[V0][0] = c.lw(c.g[S1][0] as u32);
            loop {
                // s->Write(&this->entries[i], 0x10)
                c.cycles += 12;
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = 0x10;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x20));
                focus_man_stream_vcall(c, 0x8, 0xc, 0x0010_34f0)?;
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(FOCUS_COUNT));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                if event_due(c) {
                    deliver_event_here(c, 0x0010_34d0);
                }
            }
        }
        // s->Write(&this->value_count, 4)
        c.cycles += 11;
        c.g[V0][0] = c.lw(c.g[S1][0] as u32);
        c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(FOCUS_VALUE_COUNT));
        c.g[A2][0] = 0x4;
        c.g[S2][0] = 0;
        focus_man_stream_vcall(c, 0x8, 0xc, 0x0010_3520)?;
        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(FOCUS_VALUE_COUNT));
        c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(FOCUS_VALUES));
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 1;
            c.g[V0][0] = c.lw(c.g[S1][0] as u32);
            loop {
                // s->WriteInt(this->values[j])
                c.cycles += 11;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                focus_man_stream_vcall(c, 0x28, 0x2c, 0x0010_354c)?;
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(FOCUS_VALUE_COUNT));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                if event_due(c) {
                    deliver_event_here(c, 0x0010_3530);
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// EAFile_FreeRequest (0x3de7b0)

/// The EAFile request-pool mutex (`0x520000 - 0x653c`).
const EAFILE_REQUEST_MUTEX: u32 = 0x0051_9ac4;
/// Size of one EAFile request record.
const EAFILE_REQUEST_SIZE: u64 = 0x30;

/// `void EAFile_FreeRequest(EAFileRequest* req)` (0x3de7b0): under the
/// request-pool mutex, clears the 0x30-byte request record (a free slot is
/// an all-zero record).
pub fn ea_file_free_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(EAFILE_REQUEST_MUTEX);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x003e_5700,
            f: ee::s14::EAMutex_Lock,
        }
        .call(c, 0x003d_e7d4)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = EAFILE_REQUEST_SIZE;
        Callee {
            addr: 0x003e_6448,
            f: ee::s14::EA_MemSet,
        }
        .call(c, 0x003d_e7e4)?;
        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x003e_5760,
            f: ee::s14::EAMutex_Unlock,
        }
        .call(c, 0x003d_e7ec)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderMetrix_InterruptTrick (0x119e38)

/// RiderMetrix fields used by InterruptTrick.
const METRIX_INT_STANCE: u32 = 0x0;
const METRIX_INT_STANCE_2: u32 = 0x4;
const METRIX_INT_PREV_TRICKING: u32 = 0xc;
const METRIX_INT_PREV_28: u32 = 0x10;
const METRIX_INT_TRICKING: u32 = 0x20;
const METRIX_INT_TIMER: u32 = 0x24;
const METRIX_INT_28: u32 = 0x28;
const METRIX_INT_30: u32 = 0x30;
const METRIX_INT_34: u32 = 0x34;
/// Non-zero while the metrix is idle (nothing to bank).
const METRIX_INT_IDLE: u32 = 0x70;

/// `float RiderMetrix::InterruptTrick(int a, int b)` (0x119e38): keeps the
/// current tricking flag (+0x20) and +0x28 as the previous ones (+0xc,
/// +0x10). When idle (+0x70 set) it only clears +0x34, sets the timer +0x24
/// to -1 and clears +0x20; otherwise it banks the trick
/// (`FinishTrick(this, 0, 0, 0, 0, 1)`) and resets it. Then stores `a` at
/// +0x0, `b` at +0x4, clears +0x30 and returns the banked points (0 when
/// idle).
pub fn rider_metrix_interrupt_trick(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x50);
        frame.save(c, S3, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A2][0];
        frame.save_fpr(c, 20, 0x60);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0);
        c.mtc1(0, 20); // points = 0.0
        let m = c.g[S0][0] as u32;
        c.g[V0][0] = c.lw(m.wrapping_add(METRIX_INT_IDLE));
        c.g[S1][0] = c.lw(m.wrapping_add(METRIX_INT_TRICKING));
        c.g[S2][0] = c.lw(m.wrapping_add(METRIX_INT_28));
        if c.g[V0][0] != 0 {
            c.cycles += 6;
            c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0f
            c.mtc1(AT, 0);
            c.swc1(20, m.wrapping_add(METRIX_INT_34));
            c.swc1(0, m.wrapping_add(METRIX_INT_TIMER));
            c.sw(m.wrapping_add(METRIX_INT_TRICKING), 0);
        } else {
            c.cycles += 10;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0;
            c.g[T1][0] = 1;
            Callee {
                addr: 0x0011_a228,
                f: ee::s00::RiderMetrix_FinishTrick,
            }
            .call(c, 0x0011_9eac)?;
            c.mov_s(20, 0);
            c.g[A0][0] = c.g[S0][0];
            Callee {
                addr: 0x0011_7838,
                f: ee::s00::RiderMetrix_ResetTrick,
            }
            .call(c, 0x0011_9eb8)?;
        }
        c.cycles += 15;
        let m = c.g[S0][0] as u32;
        c.sw(m.wrapping_add(METRIX_INT_PREV_TRICKING), c.g[S1][0]);
        c.mov_s(0, 20);
        c.sw(m.wrapping_add(METRIX_INT_PREV_28), c.g[S2][0]);
        c.sw(m.wrapping_add(METRIX_INT_STANCE_2), c.g[S3][0]);
        c.sw(m.wrapping_add(METRIX_INT_STANCE), c.g[S4][0]);
        c.sw(m.wrapping_add(METRIX_INT_30), 0);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptFn_SetSelfObject (0x302968)

mod set_self_object {
    /// gp offsets: the static defaults `{object id, flag byte}`, their
    /// "initialised" flag, the declared argument types, the object manager
    /// and the world.
    pub const GP_DEFAULTS: u32 = 0x2678;
    pub const GP_DEFAULTS_FLAG: u32 = 0x267c;
    pub const GP_DEFAULTS_READY: u32 = 0x2680;
    pub const GP_ARG_TYPES: u32 = 0xbf8;
    pub const GP_OBJECT_MANAGER: u32 = 0x16c8;
    pub const GP_WORLD: u32 = 0xce8;
    /// The world's "self" object field.
    pub const WORLD_SELF: u32 = 0x290;
    /// The declared type that converts an int argument to float.
    pub const TYPE_FLOAT: u64 = 2;
    /// `EA_MemCopy`.
    pub const MEM_COPY: u32 = 0x003e_6574;
    /// The 12-byte "ok" result copied into the return register.
    pub const RESULT_OK: u32 = 0x004c_9098;
}

/// `LunoVMRegister* WScriptFn_SetSelfObject(LunoVMRegister* ret, int n,
/// Arg* args)` (0x302968): script function. The object id starts from the
/// default (-1) and takes each passed argument `{idx, val, _, type}` (an int
/// passed for a float slot is converted). The id (class in the low byte,
/// handle in the rest) is resolved through the object manager's class
/// table; the world's self object (+0x290) becomes the handle word >> 8,
/// times 4, or 0 when the class or handle is missing. Returns the 12-byte
/// "ok" result in `*ret`. Frame 0x70 (s0-s3; the arguments at +0x10).
pub fn wscript_fn_set_self_object(c: &mut Cpu) {
    use set_self_object::*;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x70);
        let gp = c.g[GP][0] as u32;
        let ready = c.lw(gp.wrapping_add(GP_DEFAULTS_READY));
        c.g[V0][0] = ready;
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x60);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        if ready == 0 {
            // First use: default id -1 (the flag byte kept, its copy at sp).
            c.cycles += 7;
            let flag = c.lbu(gp.wrapping_add(GP_DEFAULTS_FLAG));
            c.g[V0][0] = flag;
            c.g[V1][0] = u64::MAX;
            c.g[A0][0] = 1;
            c.sw(gp.wrapping_add(GP_DEFAULTS), u64::MAX);
            c.sw(gp.wrapping_add(GP_DEFAULTS_READY), 1);
            c.sb(sp(c), flag);
        }

        // buf (sp+0x10) = defaults.
        c.cycles += 10;
        c.g[V0][0] = u64::MAX;
        let buf = sp(c).wrapping_add(0x10);
        c.g[S0][0] = sx(buf);
        c.g[A2][0] = 0x8;
        c.g[A0][0] = sx(buf);
        c.g[A1][0] = sx(gp.wrapping_add(GP_DEFAULTS));
        c.sw(buf, u64::MAX);
        call_ptr(c, MEM_COPY, 0x0030_29d0)?;
        c.g[A2][0] = c.g[S2][0];
        if (c.g[S1][0] as i64) > 0 {
            // Each passed argument into its slot.
            c.cycles += 4;
            c.g[A1][0] = c.g[S1][0];
            c.g[T0][0] = sx((c.g[GP][0] as u32).wrapping_add(GP_ARG_TYPES));
            c.g[A3][0] = TYPE_FLOAT;
            loop {
                c.cycles += 8;
                let arg = c.g[A2][0] as u32;
                let idx = c.lw(arg);
                let ty = c.lw(arg.wrapping_add(0xc));
                c.g[V1][0] = ty;
                let off = (idx as u32) << 2;
                let slot = (c.g[S0][0] as u32).wrapping_add(off);
                c.g[A0][0] = sx(slot);
                let declared = c.lw((c.g[T0][0] as u32).wrapping_add(off));
                c.g[V0][0] = declared;
                if ty != declared && declared == c.g[A3][0] {
                    c.cycles += 6;
                    c.lwc1(0, arg.wrapping_add(0x4));
                    c.cvt_s_w(0, 0);
                    c.swc1(0, slot);
                } else {
                    if ty != declared {
                        c.cycles += 2;
                    }
                    c.cycles += 1;
                    let val = c.lw(arg.wrapping_add(0x4));
                    c.g[V0][0] = val;
                    c.sw(slot, val);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                c.g[A2][0] = sx(arg.wrapping_add(0x10));
                if c.g[A1][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_29e8);
                }
            }
        }

        // The object: class table[id & 0xff], handle table[id >> 8] >> 8,
        // times 4 (0 when either is missing).
        c.cycles += 10;
        let gp = c.g[GP][0] as u32;
        let manager = c.lw(gp.wrapping_add(GP_OBJECT_MANAGER));
        let id = c.lw(c.g[S0][0] as u32);
        c.g[A1][0] = id;
        let m = c.lw(manager as u32);
        c.g[A0][0] = m;
        let classes = c.lw((m as u32).wrapping_add(0x8));
        let entry = ((id as u32 & 0xff) << 2).wrapping_add(classes as u32);
        c.g[V0][0] = sx(entry);
        let class = c.lw(entry);
        c.g[V1][0] = class;
        c.g[A2][0] = c.lw(gp.wrapping_add(GP_WORLD));
        let mut object = 0u64;
        if class != 0 {
            c.cycles += 8;
            let handles = c.lw((class as u32).wrapping_add(0x1c));
            c.g[A0][0] = handles;
            let word = c.lw(((id as u32) >> 8 << 2).wrapping_add(handles as u32));
            c.g[V1][0] = word;
            let handle = (word as u32) >> 8;
            object = sx(handle << 2);
            if handle == 0 {
                c.cycles += 1;
                object = 0;
            }
        } else {
            c.cycles += 1;
        }
        c.g[V0][0] = object;

        // world->self = object; *ret = the ok result.
        c.cycles += 17;
        c.sw((c.g[A2][0] as u32).wrapping_add(WORLD_SELF), object);
        c.g[V1][0] = 0x004d_0000;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx(RESULT_OK);
        c.ldl(A0, RESULT_OK.wrapping_add(0x7));
        c.ldr(A0, RESULT_OK);
        c.g[A1][0] = c.lw(RESULT_OK.wrapping_add(0x8));
        let ret_reg = c.g[S3][0] as u32;
        c.sdl(A0, ret_reg.wrapping_add(0x7));
        c.sdr(A0, ret_reg);
        c.sw(ret_reg.wrapping_add(0x8), c.g[A1][0]);
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
// ModObj_SetUVScrollModifier (0x355550)

/// ModObj: its modifier block at +0x1c; the block's UV-scroll slot at +0x4.
const MOD_OBJ_BLOCK: u32 = 0x1c;
const MODIFIER_BLOCK_UV_SCROLL: u32 = 0x4;

/// `void ModObj_SetUVScrollModifier(ModObj* self, UVScrollModifier* v)`
/// (0x355550): creates the modifier block when absent, else frees the old
/// UV-scroll modifier, then installs `v` in the block's slot `+0x4`. Returns
/// the block in v0.
pub fn mod_obj_set_uv_scroll_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(MOD_OBJ_BLOCK));
        let have = c.g[A0][0] != 0;
        c.g[S1][0] = c.g[A1][0];
        if have {
            c.cycles += 3;
            Callee {
                addr: 0x0035_3118,
                f: ee::s11::ModifierBlock_FreeUVScroll,
            }
            .call(c, 0x0035_5588)?;
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            Callee {
                addr: 0x0035_5468,
                f: ee::s11::ModObj_CreateModifierBlock,
            }
            .call(c, 0x0035_5578)?;
        }
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(MOD_OBJ_BLOCK));
        c.cycles += 6;
        frame.restore(c, S0, 0x20);
        c.sw(
            (c.g[V0][0] as u32).wrapping_add(MODIFIER_BLOCK_UV_SCROLL),
            c.g[S1][0],
        );
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// __fixdfsi (0x412608)

/// The unpacked double `__unpack_d` writes (libgcc `fp_number_type`), and
/// its classes.
mod fixdfsi_unpacked {
    pub const CLASS: u32 = 0x0;
    pub const SIGN: u32 = 0x4;
    pub const EXP: u32 = 0x8;
    pub const FRACTION: u32 = 0x10;
    pub const CLASS_ZERO: u64 = 2;
    pub const CLASS_INFINITY: u64 = 4;
    /// The fraction's binary point (`FRACBITS + NGARDS`).
    pub const FRACBITS: u64 = 0x3c;
}

/// `int __fixdfsi(double x)` (0x412608, libgcc soft-float): truncate `x`
/// (in a0) to a signed int: zero, NaN and |x| < 1 give 0, infinity and
/// |x| >= 2^31 saturate to 0x7fffffff / 0x80000000 by sign, otherwise the
/// fraction shifted down to the integer part, negated for negative `x`.
/// Resumable at 0x412638 (the "answer 0" block, reached through an event
/// check). Frame 0x40 (the unpacked number at +0, `x` at +0x20).
pub fn fixdfsi(c: &mut Cpu) {
    use fixdfsi_unpacked::*;
    guest(c, |c| {
        let frame = Frame { size: 0x40 };
        'zero: {
            match c.pc {
                FIXDFSI_ENTRY => {}
                FIXDFSI_ZERO => break 'zero,
                pc => {
                    c.jump_out(pc, FIXDFSI_ENTRY, FIXDFSI_END);
                    return Ok(());
                }
            }
            c.cycles += 10;
            let frame = Frame::open(c, 0x40);
            let s = sp(c);
            c.sd(s.wrapping_add(0x20), c.g[A0][0]);
            c.g[A1][0] = c.g[SP][0];
            frame.save_ra(c, 0x30);
            c.g[A0][0] = sx(s.wrapping_add(0x20));
            Callee {
                addr: 0x0041_1c38,
                f: ee::s15::__unpack_d,
            }
            .call(c, 0x0041_2620)?;
            let class = c.lw(s.wrapping_add(CLASS));
            c.g[V1][0] = class;
            c.g[V0][0] = (class < 2) as u64;
            if class == CLASS_ZERO {
                break 'zero;
            }
            // NaN.
            c.cycles += 2;
            let nan = c.g[V0][0] != 0;
            c.g[V0][0] = class ^ CLASS_INFINITY;
            if nan {
                break 'zero;
            }
            c.cycles += 2;
            let exp = c.lw(s.wrapping_add(EXP));
            c.g[A0][0] = exp;
            let mut saturate = class == CLASS_INFINITY;
            if !saturate {
                c.cycles += 2;
                c.g[V0][0] = ((exp as i64) < 31) as u64;
                if (exp as i64) < 0 {
                    // |x| < 1: a backward branch to the "answer 0" block,
                    // with an event check.
                    if back_edge_unwinds(c, FIXDFSI_ZERO) {
                        return Ok(());
                    }
                    break 'zero;
                }
                c.cycles += 2;
                c.g[V1][0] = FRACBITS;
                saturate = c.g[V0][0] == 0;
            }
            let result = if saturate {
                // Infinity or |x| >= 2^31.
                c.cycles += 6;
                let sign = c.lw(s.wrapping_add(SIGN));
                c.g[V1][0] = sign;
                c.g[A0][0] = 0xffff_ffff_8000_0000;
                if sign != 0 {
                    0xffff_ffff_8000_0000
                } else {
                    0x7fff_ffff
                }
            } else {
                c.cycles += 8;
                let fraction = c.ld(s.wrapping_add(FRACTION));
                let shift = sx((FRACBITS as u32).wrapping_sub(exp as u32));
                let int = sx((fraction >> (shift & 63)) as u32);
                let sign = c.lw(s.wrapping_add(SIGN));
                c.g[A0][0] = sign;
                let negated = sx(0u32.wrapping_sub(int as u32));
                c.g[V1][0] = negated;
                if sign != 0 {
                    negated
                } else {
                    int
                }
            };
            c.g[V0][0] = result;
            fixdfsi_exit(c, frame);
            return Ok(());
        }
        c.cycles += 2;
        c.g[V0][0] = 0;
        fixdfsi_exit(c, frame);
        Ok(())
    })
}

const FIXDFSI_ENTRY: u32 = 0x0041_2608;
/// The "answer 0" block (an event-check resume point).
const FIXDFSI_ZERO: u32 = 0x0041_2638;
const FIXDFSI_END: u32 = 0x0041_269c;

/// The epilogue of `__fixdfsi`.
fn fixdfsi_exit(c: &mut Cpu, frame: Frame) {
    c.cycles += 3;
    frame.restore_ra(c, 0x30);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// __divdf3 (0x412280)

const DIVDF3_ENTRY: u32 = 0x0041_2280;
/// The head of the long-division loop (an event-check resume point).
const DIVDF3_LOOP: u32 = 0x0041_2380;
const DIVDF3_END: u32 = 0x0041_23e8;
/// libgcc's canonical NaN (`__thenan_df`).
const DIVDF3_THE_NAN: u32 = 0x0052_adc0;

/// `double __divdf3(double a, double b)` (0x412280, libgcc soft-float):
/// unpack both (a at sp+0, b at sp+0x20), and pack, as the result: a NaN
/// operand (a first); for an infinite or zero `a`, `a` itself (with the
/// combined sign) unless `b` is of the same class, which gives the
/// canonical NaN; `a / inf` = zero; `a / 0` = infinity; else the quotient
/// of the fractions by restoring long division (one bit per step from
/// bit 60, the exponent difference adjusted when a's fraction is the
/// smaller), rounded to even on the guard bits. Resumable at the division
/// loop's head 0x412380 (an event check). Frame 0x70 (s0 and ra saved
/// with `sd`).
pub fn divdf3(c: &mut Cpu) {
    use fixdfsi_unpacked::{CLASS, EXP, FRACTION, SIGN};
    guest(c, |c| {
        let frame = Frame { size: 0x70 };
        let resumed = match c.pc {
            DIVDF3_ENTRY => false,
            DIVDF3_LOOP => true,
            pc => {
                c.jump_out(pc, DIVDF3_ENTRY, DIVDF3_END);
                return Ok(());
            }
        };
        'pack: {
            if !resumed {
                c.cycles += 16;
                let frame = Frame::open(c, 0x70);
                let s = sp(c);
                c.sd(s.wrapping_add(0x40), c.g[A0][0]);
                c.sd(s.wrapping_add(0x48), c.g[A1][0]);
                c.g[A0][0] = sx(s.wrapping_add(0x40));
                c.sd(s.wrapping_add(0x50), c.g[S0][0]);
                frame.save_ra(c, 0x60);
                c.g[A1][0] = c.g[SP][0];
                Callee {
                    addr: 0x0041_1c38,
                    f: ee::s15::__unpack_d,
                }
                .call(c, 0x0041_22a0)?;
                let a = sp(c);
                let b = a.wrapping_add(0x20);
                c.g[S0][0] = sx(b);
                c.g[A0][0] = sx(a.wrapping_add(0x48));
                c.g[A1][0] = sx(b);
                Callee {
                    addr: 0x0041_1c38,
                    f: ee::s15::__unpack_d,
                }
                .call(c, 0x0041_22b0)?;
                let a = sp(c);
                let a_class = c.lw(a.wrapping_add(CLASS));
                c.g[A3][0] = a_class;
                c.g[V0][0] = (a_class < 2) as u64;
                c.g[A1][0] = c.g[SP][0];
                if a_class < 2 {
                    // a is NaN.
                    c.cycles += 2;
                    c.g[A0][0] = c.g[SP][0];
                    break 'pack;
                }
                c.cycles += 4;
                let b_class = c.lw(a.wrapping_add(0x20 + CLASS));
                c.g[A2][0] = b_class;
                c.g[V0][0] = (b_class < 2) as u64;
                c.g[A0][0] = c.g[S0][0];
                if b_class < 2 {
                    // b is NaN.
                    break 'pack;
                }
                // a.sign ^= b.sign.
                c.cycles += 6;
                let a_sign = c.lw(a.wrapping_add(SIGN));
                c.g[A0][0] = a_class ^ 4;
                let b_sign = c.lw(a.wrapping_add(0x20 + SIGN));
                c.g[V1][0] = b_sign;
                c.g[V0][0] = a_sign ^ b_sign;
                c.sw(a.wrapping_add(SIGN), c.g[V0][0]);
                let mut a_special = a_class == 4;
                if !a_special {
                    c.cycles += 3;
                    a_special = a_class == 2;
                    c.g[V0][0] = b_class ^ 4;
                }
                if a_special {
                    // inf / x or 0 / x: a, or NaN when b is of the same class.
                    c.cycles += 2;
                    c.g[A0][0] = c.g[SP][0];
                    if a_class != b_class {
                        break 'pack;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 0x0053_0000;
                    c.g[A0][0] = sx(DIVDF3_THE_NAN);
                    break 'pack;
                }
                c.cycles += 2;
                c.g[V0][0] = b_class ^ 2;
                if b_class == 4 {
                    // a / inf = 0.
                    c.cycles += 4;
                    c.sd(a.wrapping_add(FRACTION), 0);
                    c.g[A0][0] = c.g[SP][0];
                    c.sw(a.wrapping_add(EXP), 0);
                    break 'pack;
                }
                c.cycles += 2;
                c.g[V1][0] = c.lw(a.wrapping_add(EXP));
                if b_class == 2 {
                    // a / 0 = infinity.
                    c.cycles += 4;
                    c.g[V0][0] = 4;
                    c.g[A0][0] = c.g[SP][0];
                    c.sw(a.wrapping_add(CLASS), 4);
                    break 'pack;
                }
                // The quotient's exponent; normalise the numerator.
                c.cycles += 7;
                let b_exp = c.lw(a.wrapping_add(0x20 + EXP));
                c.g[A0][0] = c.ld(a.wrapping_add(FRACTION));
                c.g[T0][0] = c.ld(a.wrapping_add(0x20 + FRACTION));
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(b_exp as u32));
                c.g[A2][0] = (c.g[A0][0] < c.g[T0][0]) as u64;
                c.sw(a.wrapping_add(EXP), c.g[V0][0]);
                if c.g[A2][0] != 0 {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    c.g[A0][0] <<= 1;
                    c.sw(a.wrapping_add(EXP), c.g[V0][0]);
                    c.g[A2][0] = (c.g[A0][0] < c.g[T0][0]) as u64;
                }
                c.cycles += 4;
                c.g[V0][0] = 1 << 60; // the quotient bit
                c.g[A3][0] = 0; // the quotient
            }
            // Restoring division: a0 numerator, t0 denominator, v0 bit, a3
            // quotient (registers: the loop can be resumed at its head).
            let mut at_head = resumed;
            loop {
                if at_head {
                    c.cycles += 1;
                    c.g[A2][0] = (c.g[A0][0] < c.g[T0][0]) as u64;
                }
                at_head = true;
                c.cycles += 2;
                if c.g[A2][0] == 0 {
                    c.cycles += 3;
                    c.g[A3][0] |= c.g[V0][0];
                    c.g[A0][0] = c.g[A0][0].wrapping_sub(c.g[T0][0]);
                }
                c.g[V0][0] >>= 1;
                c.cycles += 2;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] <<= 1;
                if !more {
                    break;
                }
                if back_edge_unwinds(c, DIVDF3_LOOP) {
                    return Ok(());
                }
            }
            // Round to even on the guard bits (0x80 exactly: up when odd,
            // or when a remainder is left).
            c.cycles += 4;
            let q = c.g[A3][0];
            c.g[V1][0] = q & 0xff;
            c.g[V0][0] = 0x80;
            let result_frac = c.g[A1][0] as u32;
            if q & 0xff == 0x80 {
                c.cycles += 3;
                c.g[V0][0] = q.wrapping_add(0x80);
                if q & 0x100 != 0 {
                    c.cycles += 2;
                    c.g[A3][0] = q.wrapping_add(0x80);
                } else {
                    c.cycles += 1;
                    if c.g[A0][0] != 0 {
                        c.g[A3][0] = c.g[V0][0];
                    }
                }
                c.cycles += 1;
            }
            c.sd(result_frac.wrapping_add(FRACTION), c.g[A3][0]);
            c.cycles += 1;
            c.g[A0][0] = c.g[A1][0];
        }
        c.cycles += 6;
        Callee {
            addr: 0x0041_1b08,
            f: ee::s15::__pack_d,
        }
        .call(c, 0x0041_23d8)?;
        let s = sp(c);
        frame.restore_ra(c, 0x60);
        c.g[S0][0] = c.ld(s.wrapping_add(0x50));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// CrowdMan3d_SaveState (0x3441a8)

/// CrowdMan3d: five crowd groups of 0x1f4 bytes; a group's id at +0x8 is
/// -1 when unused.
const CROWD_GROUP_SIZE: u32 = 0x1f4;
const CROWD_GROUP_ID: u32 = 0x8;
const CROWD_GROUPS_END: u32 = 5 * CROWD_GROUP_SIZE;

/// `void CrowdMan3d::SaveState(cStream* s)` (0x3441a8): for each of the five
/// crowd groups writes its id (`s->Write(&group->id, 4)`, re-read after the
/// call) and, for a used group (id != -1), the group itself
/// (`CrowdMan3d_GroupSave`). s0 = group, s1 = &group->id, s2 = stream,
/// s3 = -1, s4 = end.
pub fn crowd_man3d_save_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S3, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        c.g[S3][0] = u64::MAX;
        frame.save(c, S2, 0x30);
        frame.save(c, S4, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(CROWD_GROUPS_END));
        c.g[S1][0] = sx((c.g[A0][0] as u32).wrapping_add(CROWD_GROUP_ID));
        loop {
            // s->Write(&group->id, 4)
            c.cycles += 10;
            let stream = c.g[S2][0] as u32;
            let vt = c.lw(stream);
            c.g[V0][0] = vt;
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = 0x4;
            c.g[A0][0] = c.lh((vt as u32).wrapping_add(0x8));
            c.g[V1][0] = c.lw((vt as u32).wrapping_add(0xc));
            let write = c.g[V1][0] as u32;
            c.g[A0][0] = sx(stream.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, write, 0x0034_41fc)?;
            let id = c.lw((c.g[S0][0] as u32).wrapping_add(CROWD_GROUP_ID));
            c.g[V0][0] = id;
            c.g[A0][0] = c.g[S0][0];
            if id != c.g[S3][0] {
                c.cycles += 2;
                c.g[A1][0] = c.g[S2][0];
                Callee {
                    addr: 0x0034_4fc0,
                    f: ee::s11::CrowdMan3d_GroupSave,
                }
                .call(c, 0x0034_4210)?;
            }
            c.cycles += 4;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(CROWD_GROUP_SIZE));
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S4][0] as i64)) as u64;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(CROWD_GROUP_SIZE));
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_41e0);
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ModifierBlock_FreeTransform (0x3530d0)

/// ModifierBlock: the transform modifier at +0x0.
const MODIFIER_BLOCK_TRANSFORM: u32 = 0x0;

/// `void ModifierBlock_FreeTransform(ModifierBlock* self)` (0x3530d0): when
/// the block has a transform modifier, deletes it through its virtual
/// destructor (vtable +0x8, flags 3: destroy and free) and clears the slot.
pub fn modifier_block_free_transform(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let block = c.g[S0][0] as u32;
        let transform = c.lw(block.wrapping_add(MODIFIER_BLOCK_TRANSFORM));
        c.g[A2][0] = transform;
        c.g[A1][0] = 0x3;
        if transform != 0 {
            c.cycles += 6;
            let vt = c.lw(transform as u32);
            c.g[V0][0] = vt;
            c.g[A0][0] = c.lh((vt as u32).wrapping_add(0x8));
            c.g[V1][0] = c.lw((vt as u32).wrapping_add(0xc));
            let dtor = c.g[V1][0] as u32;
            c.g[A0][0] = sx((transform as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, dtor, 0x0035_3100)?;
            c.sw(
                (c.g[S0][0] as u32).wrapping_add(MODIFIER_BLOCK_TRANSFORM),
                0,
            );
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider_DeflectAlongWall (0x1065b0)

mod deflect_along_wall {
    /// Rider fields: the orientation quaternion, forward and up vectors.
    pub const ORIENTATION: u32 = 0x120;
    pub const FORWARD: u32 = 0x1b0;
    pub const UP: u32 = 0x1c0;
    /// The wall contact's normal (in the second argument).
    pub const CONTACT_NORMAL: u32 = 0x10;
    /// gp-relative constants: -pi/2 and pi/2 (atan2 with x = 0), +pi and
    /// -pi (quadrant fix), the deflection limit (110 degrees) and the
    /// largest step per call (20 degrees).
    pub const GP_MINUS_HALF_PI: u32 = 0x7eec;
    pub const GP_HALF_PI: u32 = 0x7ee8;
    pub const GP_PI_ADD: u32 = 0x7ee4;
    pub const GP_PI_SUB: u32 = 0x7ee0;
    pub const GP_LIMIT: u32 = 0x7edc;
    pub const GP_STEP: u32 = 0x7ed8;
    /// `float atanf(float)` and `void Math_SinCos(float, float* s, float* c)`.
    pub const ATANF: u32 = 0x0031_c228;
    pub const SIN_COS: u32 = 0x0031_be50;
}

/// `vf6 = vf4 x vf5` (the outer-product pair; vf6.x is then zeroed by
/// `vsub.x vf6, vf6, vf6`), as the original's macro-mode sequence.
fn deflect_cross(c: &mut Cpu) {
    use ssx3_ee_rt::vu0::Src;
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
}

/// `vf4.w = dot3(vf3, vf<b>)` through `vmul` / `vadda` / `vmadda` / `vmadd`
/// (vf6.w = vf0.w first), as the original does.
fn deflect_dot(c: &mut Cpu, b: u8) {
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(b));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// `void Rider::DeflectAlongWall(WallContact* contact)` (0x1065b0): with the
/// wall normal `n = norm(up x contact.normal)` and `t = up x n`, the heading
/// angle relative to the wall is `atan2(fwd . n, fwd . t)`. A positive angle
/// is pushed towards +110 degrees, a negative one towards -110 degrees, by
/// at most 20 degrees per call; when that turn `d` is non-zero the rider's
/// orientation (+0x120) is rotated by `-d` about up (a quaternion from
/// `Math_SinCos(-d / 2)`, multiplied on the left). Frame 0xc0 (s0, f20-f22).
pub fn rider_deflect_along_wall(c: &mut Cpu) {
    use deflect_along_wall::*;
    use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
    guest(c, |c| {
        c.cycles += 63;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S0, 0x90);
        frame.save_fpr(c, 22, 0xb0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x80);
        frame.save_fpr(c, 21, 0xa8);
        frame.save_fpr(c, 20, 0xa0);
        c.mtc1(0, 22); // 0.0
        let rider = c.g[S0][0] as u32;
        let s = sp(c);
        // n = norm(up x contact.normal) at sp+0x0 (via sp+0x30/+0x10).
        c.lq(V0, rider.wrapping_add(UP));
        c.sq(V0, s.wrapping_add(0x20));
        c.lqc2(4, s.wrapping_add(0x20));
        c.lqc2(5, (c.g[A1][0] as u32).wrapping_add(CONTACT_NORMAL));
        deflect_cross(c);
        c.sqc2(6, s.wrapping_add(0x30));
        c.lq(V0, s.wrapping_add(0x30));
        c.sq(V0, s.wrapping_add(0x10));
        c.lqc2(3, s.wrapping_add(0x10));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().rsqrt(0, 3, 4, 0);
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 3, Src::Q);
        c.sqc2(5, s.wrapping_add(0x30));
        // t = up x n (sp+0x10).
        c.lq(V1, rider.wrapping_add(UP));
        c.lq(V0, s.wrapping_add(0x30));
        c.sq(V1, s.wrapping_add(0x20));
        c.sq(V0, s);
        c.lqc2(4, s.wrapping_add(0x20));
        c.lqc2(5, s);
        deflect_cross(c);
        c.sqc2(6, s.wrapping_add(0x30));
        // x = fwd . t (f20), y = fwd . n (f21).
        c.lq(V1, rider.wrapping_add(FORWARD));
        c.lq(V0, s.wrapping_add(0x30));
        c.sq(V1, s.wrapping_add(0x20));
        c.sq(V0, s.wrapping_add(0x10));
        c.lqc2(3, s.wrapping_add(0x20));
        c.lqc2(5, s.wrapping_add(0x10));
        deflect_dot(c, 5);
        c.qmfc2(A0, 4);
        c.mtc1(A0, 20);
        c.lqc2(3, s.wrapping_add(0x20));
        c.lqc2(5, s);
        deflect_dot(c, 5);
        c.qmfc2(V0, 4);
        c.mtc1(V0, 21);

        // angle (f12) = atan2(y, x).
        let gp = c.g[GP][0] as u32;
        c.c_eq_s(20, 22);
        if c.fcc() {
            // x == 0: 0, -pi/2 or pi/2 by y.
            c.cycles += 4;
            c.c_eq_s(21, 22);
            let y_zero = c.fcc();
            c.mov_s(12, 21);
            if !y_zero {
                c.cycles += 4;
                c.c_le_s(22, 21);
                c.lwc1(12, gp.wrapping_sub(GP_MINUS_HALF_PI));
                if c.fcc() {
                    c.cycles += 3;
                    c.lwc1(12, gp.wrapping_sub(GP_HALF_PI));
                }
            }
        } else {
            c.cycles += 7;
            c.div_s(12, 21, 20);
            call_ptr(c, ATANF, 0x0010_66e4)?;
            c.c_lt_s(20, 22);
            let x_negative = c.fcc();
            c.mov_s(12, 0);
            if x_negative {
                c.cycles += 3;
                let gp = c.g[GP][0] as u32;
                c.c_lt_s(22, 21);
                if c.fcc() {
                    c.cycles += 3;
                    c.lwc1(0, gp.wrapping_sub(GP_PI_ADD));
                    c.add_s(12, 12, 0);
                } else {
                    c.cycles += 2;
                    c.lwc1(0, gp.wrapping_sub(GP_PI_SUB));
                    c.sub_s(12, 12, 0);
                }
            }
        }

        // d (f3) = clamp(limit - angle, 0, step) for a positive angle, else
        // clamp(-limit - angle, -step, 0).
        c.cycles += 6;
        let gp = c.g[GP][0] as u32;
        c.mtc1(0, 2);
        c.lwc1(0, gp.wrapping_sub(GP_LIMIT));
        c.c_lt_s(2, 12);
        c.lwc1(1, gp.wrapping_sub(GP_STEP));
        if c.fcc() {
            c.cycles += 5;
            c.sub_s(0, 0, 12);
            c.c_le_s(2, 0);
            c.mov_s(3, 2);
            if c.fcc() {
                c.cycles += 2;
                c.min_s(3, 0, 1);
            }
        } else {
            c.neg_s(0, 0);
            c.cycles += 6;
            c.neg_s(1, 1);
            c.sub_s(0, 0, 12);
            c.c_le_s(1, 0);
            c.mov_s(3, 1);
            if c.fcc() {
                c.cycles += 1;
                c.min_s(3, 0, 2);
            }
        }

        c.cycles += 6;
        c.mov_s(1, 3);
        c.mtc1(0, 0);
        c.c_eq_s(1, 0);
        if !c.fcc() {
            // Rotate the orientation by -d about up.
            c.cycles += 37;
            c.neg_s(0, 1);
            c.g[AT][0] = 0x3f00_0000; // 0.5f
            c.mtc1(AT, 12);
            let rider = c.g[S0][0] as u32;
            let s = sp(c);
            c.lq(V0, rider.wrapping_add(UP));
            c.g[A0][0] = sx(s.wrapping_add(0x70));
            c.g[A1][0] = sx(s.wrapping_add(0x74));
            c.mul_s(12, 0, 12);
            c.sq(V0, s.wrapping_add(0x20));
            call_ptr(c, SIN_COS, 0x0010_67a0)?;
            let s = sp(c);
            // q = (up * sin, cos) at sp+0x50.
            c.lwc1(0, s.wrapping_add(0x70));
            c.lwc1(1, s.wrapping_add(0x20));
            c.lwc1(2, s.wrapping_add(0x24));
            c.lwc1(4, s.wrapping_add(0x28));
            c.mul_s(1, 0, 1);
            c.mul_s(2, 0, 2);
            c.lwc1(3, s.wrapping_add(0x74));
            c.mul_s(0, 0, 4);
            c.swc1(3, s.wrapping_add(0x5c));
            c.swc1(1, s.wrapping_add(0x50));
            c.swc1(2, s.wrapping_add(0x54));
            c.swc1(0, s.wrapping_add(0x58));
            // orientation = orientation * q (VU0 quaternion product).
            let rider = c.g[S0][0] as u32;
            c.lqc2(4, rider.wrapping_add(ORIENTATION));
            c.lqc2(5, s.wrapping_add(0x50));
            c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
            c.vu0().opmula(4, 5);
            c.vu0().opmsub(6, 5, 4);
            c.vu0().mula(0b1110, 4, Src::Bc(5, W));
            c.vu0().madda(0b1110, 5, Src::Bc(4, W));
            c.vu0().suba(0b0001, 7, Src::Bc(7, X));
            c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
            c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
            c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
            c.sqc2(8, s.wrapping_add(0x60));
            c.lq(V0, s.wrapping_add(0x60));
            c.sq(V0, rider.wrapping_add(ORIENTATION));
            c.sq(V0, s.wrapping_add(0x40));
        }
        frame.restore(c, S0, 0x90);
        c.cycles += 6;
        frame.restore_ra(c, 0x80);
        frame.restore_fpr(c, 22, 0xb0);
        frame.restore_fpr(c, 21, 0xa8);
        frame.restore_fpr(c, 20, 0xa0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes::{covered, machine, FAKE};
    use crate::difftest::{check, Rng, DATA};

    const FOCUS: u32 = DATA;
    const FOCUS_STREAM: u32 = DATA + 0x400;
    const FOCUS_STREAM_VT: u32 = DATA + 0x500;

    /// A FocusMan with 0..4 entries (sometimes a negative count) and 0..4
    /// values, and a stream whose Write (+0xc) and WriteInt (+0x2c) are fakes.
    fn focus_man_setup(c: &mut Cpu, seed: u32, calls: u32) -> Rng {
        let mut r = machine(c, seed, 0x0010_3480, calls);
        c.pc = 0x0010_3480;
        let count = match seed % 5 {
            0 => 0,
            1 => 0xffff_fffe,
            _ => 1 + r.next_u32() % 4,
        };
        let values = match seed % 7 {
            0 => 0,
            1 => 0x8000_0000,
            _ => 1 + r.next_u32() % 4,
        };
        c.bus.write32(FOCUS + FOCUS_COUNT, count);
        c.bus.write32(FOCUS + FOCUS_VALUE_COUNT, values);
        for i in 0..4 {
            c.bus.write32(FOCUS + FOCUS_VALUES + 4 * i, r.next_u32());
        }
        c.bus.write32(FOCUS_STREAM, FOCUS_STREAM_VT);
        c.bus
            .write16(FOCUS_STREAM_VT + 0x8, (r.next_u32() % 0x40) as u16 & !3);
        c.bus.write32(FOCUS_STREAM_VT + 0xc, FAKE);
        c.bus
            .write16(FOCUS_STREAM_VT + 0x28, (r.next_u32() % 0x40) as u16 & !3);
        c.bus.write32(FOCUS_STREAM_VT + 0x2c, FAKE + 0x10);
        c.g[A0][0] = sx(FOCUS);
        c.g[A1][0] = sx(FOCUS_STREAM);
        r
    }

    /// `FocusMan_SaveState`: empty, negative and 1..4 counts for both lists;
    /// fakes unwind at random calls.
    #[test]
    fn focus_man_save_state_matches() {
        covered(
            ee::s00::f_00103480__generated,
            focus_man_save_state,
            200,
            &|c, seed| {
                focus_man_setup(c, seed, 10);
            },
        );
    }

    driven!(focus_man_save_state_gen_driven, ee::s00::f_00103480__generated);
    driven!(focus_man_save_state_hand_driven, focus_man_save_state);

    /// `FocusMan_SaveState` with events due at the loops' back edges.
    #[test]
    fn focus_man_save_state_events_match() {
        check_events(
            focus_man_save_state_gen_driven,
            focus_man_save_state_hand_driven,
            96,
            &|c, seed| {
                let mut r = focus_man_setup(c, seed | 2, 0);
                let mode = schedule(c, &mut r, seed, 0x0010_3480);
                c.next_event = c.cycles + 30 + (r.next_u32() % 40) as u64;
                mode
            },
        );
    }

    use crate::difftest::fakes::LOG_RESULTS;

    /// `EAFile_FreeRequest`: the real mutex lock/unlock (thread id from a
    /// fake GetThreadId: the owner, so the fast path, or another thread, so
    /// the semaphore path through fakes) and the real memset on a random
    /// request record.
    #[test]
    fn ea_file_free_request_matches() {
        covered(
            ee::s14::f_003de7b0__generated,
            ea_file_free_request,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003d_e7b0, 4);
                c.pc = 0x003d_e7b0;
                let owner = 0x40 + seed % 3;
                c.bus.write32(EAFILE_REQUEST_MUTEX + 4, owner);
                c.bus.write32(EAFILE_REQUEST_MUTEX + 8, 1 + seed % 2);
                c.bus.write32(EAFILE_REQUEST_MUTEX, r.next_u32() & 0xff);
                c.bus.write32(LOG_RESULTS, 0x41);
                c.bus.write32(LOG_RESULTS + 4, 0x41);
                c.bus.write32(LOG_RESULTS + 8, 0x41);
                c.bus.write32(LOG_RESULTS + 12, 0x41);
                let req = DATA + 0x100 + (r.next_u32() % 0x40) * 4;
                for i in 0..16 {
                    c.bus.write32(req - 4 + 4 * i, r.next_u32());
                }
                c.g[A0][0] = sx(req);
            },
        );
    }

    use crate::difftest::{elf_image, GAME_GP};

    /// `RiderMetrix_InterruptTrick`: idle metrix (no calls) or the real
    /// FinishTrick/ResetTrick on the RiderMetrix_FinishTrick fixture (a
    /// world with one player, finished or not, zeroed BELibrary dummy and
    /// audio system, random trick record).
    #[test]
    fn rider_metrix_interrupt_trick_matches() {
        check(
            ee::s00::f_00119e38__generated,
            rider_metrix_interrupt_trick,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0011_9e38, 0);
                elf_image(c);
                c.pc = 0x0011_9e38;
                let be = DATA + 0x20_0000;
                let world = DATA + 0x21_0000;
                let game = world + 0x1000;
                let race = world + 0x2000;
                let player = world + 0x3000;
                let player_state = world + 0x4000;
                let audio = DATA + 0x22_0000;
                for a in [be, world, audio] {
                    for i in 0..0x1000 / 4 {
                        c.bus.write32(a + 4 * i, 0);
                    }
                }
                c.bus.write32(GAME_GP.wrapping_sub(0x1f04), be);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(GAME_GP + 0x410, audio);
                c.bus
                    .write32(GAME_GP.wrapping_sub(0x7b0c), 1.0f32.to_bits());
                c.bus.write32(world + 0x84, game);
                c.bus.write32(game + 0xc, race);
                c.bus.write32(race + 0x7c, 1);
                c.bus.write32(race + 0x40, player);
                c.bus.write32(player + 0x18, player_state);
                let finished = seed % 5 == 1;
                c.bus.write32(
                    player_state + 0x470,
                    (if finished { 1.0f32 } else { -1.0 }).to_bits(),
                );
                c.bus.write8(0x0053_5bc8 + 0x48, (r.next_u32() % 5) as u8);
                c.bus.write8(0x0053_5bc8 + 0x49, (seed / 3 % 2) as u8);
                let m = DATA + 0x1000;
                for i in 0..0x200 / 4 {
                    c.bus.write32(m + 4 * i, 0);
                }
                c.bus
                    .write32(m + 0x14, ((r.next_u32() % 500) as f32).to_bits());
                let points = if seed % 7 == 0 {
                    0.0f32
                } else {
                    (r.next_u32() % 4000) as f32
                };
                c.bus.write32(m + 0x1c4, points.to_bits());
                for off in [
                    0x34, 0x38, 0x44, 0x30, 0x48, 0x24, 0x78, 0x14c, 0x150, 0x154, 0x158, 0x144,
                ] {
                    c.bus.write32(m + off, r.f32(10.0));
                }
                for off in [
                    0x4c, 0x80, 0x58, 0x74, 0x84, 0xfc, 0x100, 0x104, 0x108, 0x114, 0x118, 0x11c,
                ] {
                    c.bus.write32(m + off, r.next_u32() % 100);
                }
                for off in [0x198, 0x19c, 0x110, 0x18c] {
                    c.bus.write32(m + off, r.next_u32() % 3000);
                }
                c.bus.write32(m + 0xa4, r.f32(2.0));
                c.bus.write32(m + METRIX_INT_TRICKING, r.next_u32() % 2);
                c.bus.write32(m + METRIX_INT_28, r.next_u32() % 2);
                c.bus
                    .write32(m + METRIX_INT_IDLE, if seed % 3 == 0 { 1 } else { 0 });
                c.g[A0][0] = sx(m);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = sx(r.next_u32());
            },
        );
    }

    use crate::difftest::fill;

    /// `WScriptFn_SetSelfObject`: defaults initialised or not; 0, negative or
    /// 1..3 arguments (ids, ints and floats into int and float slots); class
    /// tables with missing classes and handle words with a zero handle; the
    /// real EA_MemCopy.
    #[test]
    fn wscript_fn_set_self_object_matches() {
        use set_self_object::*;
        check(
            ee::s10::f_00302968__generated,
            wscript_fn_set_self_object,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0030_2968, 0);
                c.pc = 0x0030_2968;
                let gp = |off: u32| GAME_GP.wrapping_add(off);
                c.bus.write32(gp(GP_DEFAULTS_READY), (seed % 4 != 0) as u32);
                fill(c, &mut r, RESULT_OK, 3);
                // An object id: class (low byte) 0..7 and handle 0..15, or one
                // with class 0xff (never present).
                let some_id = |r: &mut Rng| -> u32 {
                    if r.next_u32() % 4 == 0 {
                        r.next_u32() | 0xff00_00ff
                    } else {
                        (r.next_u32() % 16) << 8 | r.next_u32() % 8
                    }
                };
                let default_id = some_id(&mut r);
                c.bus.write32(gp(GP_DEFAULTS), default_id);
                c.bus.write32(gp(GP_DEFAULTS_FLAG), r.next_u32());
                // Declared types: the id slot int (never converted: a float's
                // bits would index far outside the handle table), the others
                // int or float.
                c.bus.write32(gp(GP_ARG_TYPES), r.next_u32() % 2);
                for k in 1..4 {
                    c.bus.write32(gp(GP_ARG_TYPES + 4 * k), r.next_u32() % 3);
                }
                let mgr = DATA + 0x100;
                let m = DATA + 0x200;
                let tables = DATA + 0x300;
                let class = DATA + 0x400;
                let handles = DATA + 0x800;
                let world = DATA + 0xc00;
                c.bus.write32(gp(GP_OBJECT_MANAGER), mgr);
                c.bus.write32(mgr, m);
                c.bus.write32(m + 8, tables);
                for k in 0..256 {
                    let present = k < 8 && r.next_u32() % 4 != 0;
                    c.bus
                        .write32(tables + 4 * k, if present { class } else { 0 });
                }
                c.bus.write32(class + 0x1c, handles);
                for k in 0..16 {
                    let word = if r.next_u32() % 4 == 0 {
                        r.next_u32() & 0xff
                    } else {
                        r.next_u32()
                    };
                    c.bus.write32(handles + 4 * k, word);
                }
                c.bus.write32(gp(GP_WORLD), world);
                let args = DATA + 0x2000;
                let n = match seed % 6 {
                    0 => 0,
                    1 => 0xffff_ffff,
                    _ => 1 + r.next_u32() % 3,
                };
                for k in 0..3 {
                    let a = args + 0x10 * k;
                    let idx = if k == 0 { 0 } else { r.next_u32() % 2 };
                    c.bus.write32(a, idx);
                    let val = if idx == 0 && r.next_u32() % 4 != 0 {
                        some_id(&mut r)
                    } else {
                        r.next_u32() % 1000
                    };
                    c.bus.write32(a + 4, val);
                    c.bus.write32(a + 8, r.next_u32());
                    c.bus.write32(a + 0xc, r.next_u32() % 3);
                }
                c.g[A0][0] = sx(DATA + 0x3000 + (seed % 2) * 4);
                c.g[A1][0] = sx(n);
                c.g[A2][0] = sx(args);
            },
        );
    }

    use crate::game::scripter::ssxscriptengine::tests::{
        alloc as uv_scroll_alloc, heap_init as uv_scroll_heap_init,
    };

    /// `ModObj_SetUVScrollModifier`: no block (the real CreateModifierBlock
    /// over a DLHeap arena), a block with no UV-scroll modifier, and one with
    /// a live modifier the real FreeUVScroll frees.
    #[test]
    fn mod_obj_set_uv_scroll_modifier_matches() {
        check(
            ee::s11::f_00355550__generated,
            mod_obj_set_uv_scroll_modifier,
            150,
            &|c, seed| {
                let _ = machine(c, seed, 0x0035_5550, 0);
                uv_scroll_heap_init(c);
                c.pc = 0x0035_5550;
                let this = DATA;
                let block = DATA + 0x200;
                for i in 0..0x20 {
                    c.bus.write32(this + 4 * i, 0);
                    c.bus.write32(block + 4 * i, 0);
                }
                match seed % 3 {
                    0 => {}
                    1 => c.bus.write32(this + MOD_OBJ_BLOCK, block),
                    _ => {
                        let old = uv_scroll_alloc(c, 0x20);
                        c.bus.write32(this + MOD_OBJ_BLOCK, block);
                        c.bus.write32(block + MODIFIER_BLOCK_UV_SCROLL, old);
                    }
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x400);
            },
        );
    }

    use crate::difftest::scramble;

    /// A double for the soft-float conversions: zero, NaN, infinity, |x| < 1,
    /// |x| >= 2^31 (around the 2^31 edge too) and in-range values, either
    /// sign.
    fn fixdfsi_operand(r: &mut Rng, seed: u32) -> u64 {
        let sign = (r.next_u32() as u64 & 1) << 63;
        let frac = (r.next_u32() as u64) << 20 | r.next_u32() as u64 & 0xf_ffff;
        match seed % 8 {
            0 => sign,
            1 => 0x7ff0_0000_0000_0000 | frac | 1,
            2 => 0x7ff0_0000_0000_0000 | sign,
            3 => sign | ((1023 + 30 + r.next_u32() % 3) as u64) << 52 | frac,
            4 => sign | ((1000 + r.next_u32() % 23) as u64) << 52 | frac,
            5 => sign | ((1023 + 31 + r.next_u32() % 100) as u64) << 52 | frac,
            _ => sign | ((1023 + r.next_u32() % 31) as u64) << 52 | frac,
        }
    }

    /// A `__fixdfsi` call: the operand (|x| < 1 on most seeds when `small`),
    /// sometimes entered at the resume point 0x412638 inside a frame whose
    /// saved ra returns to the test's sentinel. Returns the entry.
    fn fixdfsi_setup(c: &mut Cpu, seed: u32, small: bool) -> u32 {
        let mut r = Rng(0x0041_2608 ^ (seed as u64) << 20);
        scramble(c, &mut r);
        let x = fixdfsi_operand(&mut r, if small && seed % 4 != 0 { 4 } else { seed });
        c.g[A0][0] = x;
        if !small && seed % 10 == 7 {
            c.g[SP][0] = (crate::difftest::SP - 0x40) as u64;
            fill(c, &mut r, crate::difftest::SP - 0x40, 16);
            c.bus
                .write64(crate::difftest::SP - 0x10, crate::difftest::RA as u64);
            FIXDFSI_ZERO
        } else {
            FIXDFSI_ENTRY
        }
    }

    driven!(fixdfsi_gen_driven, ee::s15::f_00412608__generated);
    driven!(fixdfsi_hand_driven, fixdfsi);

    /// `__fixdfsi` through the real `__unpack_d`: every class and exponent
    /// range, the resume entry, and events at the |x| < 1 branch.
    #[test]
    fn fixdfsi_matches() {
        check(ee::s15::f_00412608__generated, fixdfsi, 300, &|c, seed| {
            let entry = fixdfsi_setup(c, seed, false);
            c.pc = entry;
        });
        check_events(fixdfsi_gen_driven, fixdfsi_hand_driven, 120, &|c, seed| {
            let entry = fixdfsi_setup(c, seed, true);
            let mut r = Rng(0x0041_2608 ^ seed as u64);
            let mode = schedule(c, &mut r, seed, entry);
            c.next_event = c.cycles + 4;
            mode
        });
    }

    /// A double for `__divdf3`: zeros, NaNs, infinities, denormals, 1.0 and
    /// normal numbers of either sign.
    fn divdf3_operand(r: &mut Rng) -> u64 {
        let sign = ((r.next_u32() & 1) as u64) << 63;
        let mant = ((r.next_u32() as u64) << 32 | r.next_u32() as u64) & ((1u64 << 52) - 1);
        sign | match r.next_u32() % 10 {
            0 => 0,
            1 => mant,
            2 => 0x7ff0_0000_0000_0000,
            3 => 0x7ff0_0000_0000_0000 | mant | 1,
            4 => 1.0f64.to_bits(),
            // Short mantissas: exact quotients and halfway roundings.
            5 => (((r.next_u32() % 200 + 923) as u64) << 52) | (mant & 0xffff_0000_0000_0000 >> 12),
            _ => (((r.next_u32() % 200 + 923) as u64) << 52) | mant,
        }
    }

    /// `__divdf3` setup: two operands, the game image (the canonical NaN), and
    /// sometimes a resume at the loop head with random division registers
    /// inside a frame whose saved ra returns to the test's sentinel.
    fn divdf3_setup(c: &mut Cpu, seed: u32, plain: bool) -> u32 {
        let mut r = Rng(0x0041_2280 ^ (seed as u64) << 8);
        scramble(c, &mut r);
        elf_image(c); // the canonical NaN (__thenan_df)
        let a = divdf3_operand(&mut r);
        let b = if seed % 9 == 0 {
            a ^ (r.next_u32() as u64 & 1) << 63
        } else {
            divdf3_operand(&mut r)
        };
        c.g[A0][0] = a;
        c.g[A1][0] = b;
        if !plain && seed % 10 == 7 {
            // Resume: a's unpacked number at sp, the loop registers.
            let s = crate::difftest::SP - 0x70;
            c.g[SP][0] = s as u64;
            fill(c, &mut r, s, 0x70 / 4);
            c.bus.write32(s, 3);
            c.bus.write64(s + 0x60, crate::difftest::RA as u64);
            c.g[A1][0] = s as u64;
            let den = (1u64 << 60) | (r.next_u32() as u64) << 20;
            c.g[T0][0] = den;
            c.g[A0][0] = (1u64 << 60) | (r.next_u32() as u64) << 24;
            c.g[V0][0] = 1u64 << (r.next_u32() % 61);
            c.g[A3][0] = (r.next_u32() as u64) << 20 & !(c.g[V0][0] * 2 - 1);
            DIVDF3_LOOP
        } else {
            DIVDF3_ENTRY
        }
    }

    driven!(divdf3_gen_driven, ee::s15::f_00412280__generated);
    driven!(divdf3_hand_driven, divdf3);

    /// `__divdf3` through the real `__unpack_d`/`__pack_d`: every class pair,
    /// equal magnitudes, normal quotients, the resume entry, and events in
    /// the division loop.
    #[test]
    fn divdf3_matches() {
        check(ee::s15::f_00412280__generated, divdf3, 300, &|c, seed| {
            let entry = divdf3_setup(c, seed, false);
            c.pc = entry;
        });
        check_events(divdf3_gen_driven, divdf3_hand_driven, 120, &|c, seed| {
            let entry = divdf3_setup(c, seed, true);
            let mut r = Rng(0x0041_2280 ^ seed as u64);
            // Normal operands: the division loop runs.
            c.g[A0][0] = ((1000 + r.next_u32() % 40) as u64) << 52 | r.next_u32() as u64;
            c.g[A1][0] = ((1000 + r.next_u32() % 40) as u64) << 52 | (r.next_u32() as u64) << 12;
            let mode = schedule(c, &mut r, seed, entry);
            c.next_event = c.cycles + 40 + (r.next_u32() % 300) as u64;
            mode
        });
    }

    const CROWD: u32 = DATA;
    const CROWD_STREAM: u32 = DATA + 0x1000;
    const CROWD_STREAM_VT: u32 = DATA + 0x1100;

    /// Five crowd groups, used (random id) or unused (-1), and a stream
    /// whose Write (+0xc) is a fake.
    fn crowd_man3d_save_setup(c: &mut Cpu, seed: u32, calls: u32) -> Rng {
        let mut r = machine(c, seed, 0x0034_41a8, calls);
        c.pc = 0x0034_41a8;
        for g in 0..5 {
            let id = if (seed >> g) & 1 == 0 || seed % 5 == 0 {
                0xffff_ffff
            } else {
                r.next_u32() % 64
            };
            c.bus
                .write32(CROWD + g * CROWD_GROUP_SIZE + CROWD_GROUP_ID, id);
        }
        c.bus.write32(CROWD_STREAM, CROWD_STREAM_VT);
        c.bus
            .write16(CROWD_STREAM_VT + 0x8, (r.next_u32() % 0x40) as u16 & !3);
        c.bus.write32(CROWD_STREAM_VT + 0xc, FAKE);
        c.g[A0][0] = sx(CROWD);
        c.g[A1][0] = sx(CROWD_STREAM);
        r
    }

    /// `CrowdMan3d_SaveState`: used and unused groups (the real GroupSave
    /// writes through the same fake stream); fakes unwind at random calls.
    #[test]
    fn crowd_man3d_save_state_matches() {
        covered(
            ee::s11::f_003441a8__generated,
            crowd_man3d_save_state,
            200,
            &|c, seed| {
                crowd_man3d_save_setup(c, seed, 10);
            },
        );
    }

    driven!(crowd_man3d_save_state_gen_driven, ee::s11::f_003441a8__generated);
    driven!(crowd_man3d_save_state_hand_driven, crowd_man3d_save_state);

    /// `CrowdMan3d_SaveState` with events due at the loop's back edge.
    #[test]
    fn crowd_man3d_save_state_events_match() {
        check_events(
            crowd_man3d_save_state_gen_driven,
            crowd_man3d_save_state_hand_driven,
            96,
            &|c, seed| {
                let mut r = crowd_man3d_save_setup(c, seed, 0);
                let mode = schedule(c, &mut r, seed, 0x0034_41a8);
                c.next_event = c.cycles + 20 + (r.next_u32() % 60) as u64;
                mode
            },
        );
    }

    use crate::difftest::fakes::vtable;

    /// `ModifierBlock_FreeTransform`: no transform, or one whose destructor
    /// is a fake (sometimes unwinding).
    #[test]
    fn modifier_block_free_transform_matches() {
        covered(
            ee::s11::f_003530d0__generated,
            modifier_block_free_transform,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0035_30d0, 1);
                c.pc = 0x0035_30d0;
                let block = DATA + 0x20_0000 + 0x100;
                let transform = DATA + 0x20_0000 + 0x200;
                if seed % 4 == 0 {
                    c.bus.write32(block + MODIFIER_BLOCK_TRANSFORM, 0);
                } else {
                    c.bus.write32(block + MODIFIER_BLOCK_TRANSFORM, transform);
                    let vt = vtable(c, &mut r, &[0x8]);
                    c.bus.write32(transform, vt);
                }
                c.g[A0][0] = sx(block);
            },
        );
    }

    /// A random 3-vector (w random too) of components in [-1, 1].
    fn deflect_vec(c: &mut Cpu, r: &mut Rng, addr: u32) {
        for i in 0..4 {
            c.bus.write32(addr + 4 * i, r.f32(1.0));
        }
    }

    /// `Rider_DeflectAlongWall` with the real atanf and Math_SinCos on the
    /// game image: random up/forward/normal vectors (forward sometimes
    /// along the wall normal or the tangent, so x = 0 or y = 0, and
    /// sometimes zero: no turn), unit orientation quaternions.
    #[test]
    fn rider_deflect_along_wall_matches() {
        use deflect_along_wall::*;
        check(
            ee::s00::f_001065b0__generated,
            rider_deflect_along_wall,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_65b0, 0);
                elf_image(c);
                c.pc = 0x0010_65b0;
                let rider = DATA;
                let contact = DATA + 0x800;
                // Up mostly vertical, the wall normal mostly horizontal.
                let up = [r.f32(0.3), 1.0f32.to_bits(), r.f32(0.3), r.next_u32()];
                for (i, w) in up.iter().enumerate() {
                    c.bus.write32(rider + UP + 4 * i as u32, *w);
                }
                deflect_vec(c, &mut r, contact + CONTACT_NORMAL);
                c.bus.write32(contact + CONTACT_NORMAL + 4, r.f32(0.2));
                deflect_vec(c, &mut r, rider + ORIENTATION);
                match seed % 6 {
                    0 => {
                        // Zero forward: x = y = 0.
                        for i in 0..4 {
                            c.bus.write32(rider + FORWARD + 4 * i, 0);
                        }
                    }
                    1 => {
                        // Forward along up: both dots ~0 (x exactly 0 when up
                        // is exactly vertical).
                        c.bus.write32(rider + UP, 0);
                        c.bus.write32(rider + UP + 8, 0);
                        c.bus.write32(rider + FORWARD, 0);
                        c.bus.write32(rider + FORWARD + 4, r.f32(1.0));
                        c.bus.write32(rider + FORWARD + 8, 0);
                    }
                    2 => {
                        // Up vertical, normal along x: n along z, t along x; a
                        // forward along z gives x = 0.
                        c.bus.write32(rider + UP, 0);
                        c.bus.write32(rider + UP + 8, 0);
                        c.bus.write32(contact + CONTACT_NORMAL, 1.0f32.to_bits());
                        c.bus.write32(contact + CONTACT_NORMAL + 4, 0);
                        c.bus.write32(contact + CONTACT_NORMAL + 8, 0);
                        c.bus.write32(rider + FORWARD, 0);
                        c.bus.write32(rider + FORWARD + 4, r.f32(1.0));
                        c.bus.write32(rider + FORWARD + 8, r.f32(1.0));
                    }
                    _ => deflect_vec(c, &mut r, rider + FORWARD),
                }
                c.g[A0][0] = sx(rider);
                c.g[A1][0] = sx(contact);
            },
        );
    }
}
