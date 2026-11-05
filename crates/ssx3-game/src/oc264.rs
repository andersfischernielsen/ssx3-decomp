//! Staging wave 208 (oc264): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use crate::abi;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002a_ae08, stream_array_get_state, "StreamArray_GetState"),
    (0x002e_2ff8, lensfx_add_sources, "LensFX_AddSources"),
    (0x0032_9a28, rider_sphere_tree_assign, "RiderSphereTree_Assign"),
    (0x003e_6328, ea_shell_sort_pairs, "EA_ShellSortPairs"),
    (0x002a_cfe0, bank_monitor_add_instance, "BankMonitor_AddInstance"),
    (0x0032_0408, mem_pool256_alloc, "MEM_Pool256Alloc"),
    (
        0x0039_5f60,
        ps2_graphics_man_try_acquire_frame,
        "PS2GraphicsMan_TryAcquireFrame",
    ),
    (
        0x0014_fea8,
        bebagt_int_get_uber_trick_slot_value,
        "BEBAGTInt_GetUberTrickSlotValue",
    ),
    (0x0030_a460, wscript_man_focus_leave, "WScriptMan_FocusLeave"),
];

// ---------------------------------------------------------------------------
// StreamArray_GetState (0x2aae08, streamsys.cpp)

/// StreamArray: pointer to its array of StreamBuffer pointers.
const OC264_SA_BUFFERS: u32 = 0x8;

const OC264_STREAM_BUFFER_GET_STATE: Callee = Callee {
    addr: 0x002a_a428,
    f: ee::s08::StreamBuffer_GetState,
};

/// `int StreamArray_GetState(StreamArray* self, int i)` (0x2aae08): the
/// play state of stream buffer `i` (`StreamBuffer_GetState`), or 0 when
/// that slot is empty.
pub fn stream_array_get_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        let slot_off = r32(c, A1) << 2;
        c.g[A1][0] = sx(slot_off);
        frame.save_ra(c, 0);
        let buffers = Addr(r32(c, A0).wrapping_add(OC264_SA_BUFFERS)).lw(c);
        c.g[V0][0] = buffers;
        let slot = slot_off.wrapping_add(buffers as u32);
        c.g[A1][0] = sx(slot);
        c.g[A0][0] = Addr(slot).lw(c);
        let empty = c.g[A0][0] == 0;
        c.g[V0][0] = 0;
        if !empty {
            c.cycles += 2;
            OC264_STREAM_BUFFER_GET_STATE.call(c, 0x002a_ae30)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// LensFX_AddSources (0x2e2ff8, boardwakefx.cpp)

const OC264_LENSFX_ADD_SOURCE: Callee = Callee {
    addr: 0x002e_2fa8,
    f: ee::s09::LensFX_AddSource,
};

/// `void LensFX_AddSources(LensFX* self, void** sources, int count, int,
/// int list)` (0x2e2ff8): adds each of the `count` light sources to the
/// lens-flare source list `list` (`LensFX_AddSource(self, sources[n], list)`).
pub fn lensfx_add_sources(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[T0][0];
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        let none = (c.g[A2][0] as i64) <= 0;
        frame.save_ra(c, 0);
        if !none {
            c.cycles += 3;
            c.g[S1][0] = c.g[A1][0];
            c.g[S0][0] = c.g[A2][0];
            c.g[A1][0] = Addr(r32(c, S1)).lw(c);
            loop {
                c.cycles += 7;
                c.g[A0][0] = c.g[S3][0];
                c.g[A2][0] = c.g[S2][0];
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
                OC264_LENSFX_ADD_SOURCE.call(c, 0x002e_3038)?;
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(1));
                if c.g[S0][0] == 0 {
                    break;
                }
                c.g[A1][0] = Addr(r32(c, S1)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x002e_3028);
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
// RiderSphereTree_Assign (0x329a28, riderspheretree.cpp)

/// Sphere tree node fields: type, center (quad), radius, user word, parent,
/// child count, child spheres (0x20 bytes each).
const OC264_RST_TYPE: u32 = 0x0;
const OC264_RST_CENTER: u32 = 0x10;
const OC264_RST_RADIUS: u32 = 0x20;
const OC264_RST_USER: u32 = 0x24;
const OC264_RST_PARENT: u32 = 0x28;
const OC264_RST_COUNT: u32 = 0x2c;
const OC264_RST_CHILDREN: u32 = 0x30;

/// `EA_MemCopy` (reached through the function table).
const OC264_EA_MEM_COPY: u32 = 0x003e_6574;

/// `RiderSphereTree* RiderSphereTree_Assign(RiderSphereTree* self, const
/// RiderSphereTree* src)` (0x329a28): copy assignment: the header fields,
/// then `count` child spheres (`EA_MemCopy(self->children, src->children,
/// count * 0x20)`); returns `self`.
pub fn rider_sphere_tree_assign(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = c.g[A1][0];
        let src = r32(c, A3);
        frame.save(c, S0, 0x10);
        c.g[A1][0] = sx(src.wrapping_add(OC264_RST_CHILDREN));
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        c.g[A0][0] = sx(this.wrapping_add(OC264_RST_CHILDREN));
        let kind = Addr(src.wrapping_add(OC264_RST_TYPE)).lw(c);
        c.g[V0][0] = kind;
        Addr(this.wrapping_add(OC264_RST_TYPE)).sw(c, kind);
        c.lwc1(0, src.wrapping_add(OC264_RST_RADIUS));
        c.swc1(0, this.wrapping_add(OC264_RST_RADIUS));
        c.lq(V0, src.wrapping_add(OC264_RST_CENTER));
        c.sq(V0, this.wrapping_add(OC264_RST_CENTER));
        let user = Addr(src.wrapping_add(OC264_RST_USER)).lw(c);
        c.g[V1][0] = user;
        Addr(this.wrapping_add(OC264_RST_USER)).sw(c, user);
        let count = Addr(src.wrapping_add(OC264_RST_COUNT)).lw(c);
        Addr(this.wrapping_add(OC264_RST_COUNT)).sw(c, count);
        c.g[A2][0] = sx((count as u32) << 5);
        let parent = Addr(src.wrapping_add(OC264_RST_PARENT)).lw(c);
        c.g[V0][0] = parent;
        Addr(this.wrapping_add(OC264_RST_PARENT)).sw(c, parent);
        call_ptr(c, OC264_EA_MEM_COPY, 0x0032_9a7c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// EA_ShellSortPairs (0x3e6328, seg_2DCAE8.c)

/// Shell sort gap sequence (1, 4, 13, 40, ...: 3h+1), `D_00451218`.
const OC264_SHELL_GAPS: u32 = 0x0045_1218;

/// `void EA_ShellSortPairs(int n, int* keys, int* vals)` (0x3e6328): sorts
/// `n` key/value pairs by key, descending, with a 3h+1 Shell sort (gap
/// table at 0x451218). A leaf; its loops keep their state in the original's
/// registers (t5 n, t2 keys, a2 vals, t6 gap pointer, t0 gap, a1/t3 i,
/// t1/t4 the pair being inserted, a3 j) so events see the same machine.
pub fn ea_shell_sort_pairs(c: &mut Cpu) {
    #[inline(always)]
    fn reg(c: &Cpu, r: usize) -> u32 {
        c.g[r][0] as u32
    }
    #[inline(always)]
    fn set(c: &mut Cpu, r: usize, v: u32) {
        c.g[r][0] = sx(v);
    }
    #[inline(always)]
    fn less(c: &Cpu, a: usize, b: usize) -> u64 {
        ((c.g[a][0] as i64) < (c.g[b][0] as i64)) as u64
    }

    // Find the first gap not below n: k = 1; while gaps[k] < n: k++.
    c.cycles += 8;
    c.g[T5][0] = c.g[A0][0];
    set(c, A0, OC264_SHELL_GAPS);
    c.g[T2][0] = c.g[A1][0];
    c.g[V0][0] = Addr(OC264_SHELL_GAPS + 4).lw(c);
    c.g[V0][0] = less(c, V0, T5);
    c.g[V1][0] = 1;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        set(c, A0, OC264_SHELL_GAPS + 4);
        loop {
            c.cycles += 7;
            let v = reg(c, A0).wrapping_add(4);
            set(c, A0, v);
            c.g[V0][0] = Addr(reg(c, A0)).lw(c);
            c.g[V0][0] = less(c, V0, T5);
            let v = reg(c, V1).wrapping_add(1);
            set(c, V1, v);
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003e_6350);
            }
        }
    }

    // t6 = &gaps[k - 1]; one insertion-sort pass per gap, down to gap 1.
    c.cycles += 5;
    let v = reg(c, V1) << 2;
    set(c, V1, v);
    set(c, V0, OC264_SHELL_GAPS);
    let v = reg(c, V1).wrapping_add(OC264_SHELL_GAPS).wrapping_sub(4);
    set(c, T6, v);
    loop {
        c.cycles += 4;
        c.g[T0][0] = Addr(reg(c, T6)).lw(c);
        c.g[V0][0] = less(c, T0, T5);
        c.g[A1][0] = c.g[T0][0];
        if c.g[V0][0] != 0 {
            // for i = gap; i < n; i++: insert (keys[i], vals[i]).
            loop {
                c.cycles += 8;
                let v = reg(c, A1) << 2;
                set(c, V0, v);
                let v = reg(c, A1).wrapping_add(1);
                set(c, T3, v);
                let v = reg(c, V0).wrapping_add(reg(c, A2));
                set(c, V1, v);
                let v = reg(c, A1).wrapping_sub(reg(c, T0));
                set(c, A3, v);
                let v = reg(c, V0).wrapping_add(reg(c, T2));
                set(c, V0, v);
                c.g[T4][0] = Addr(reg(c, V1)).lw(c);
                c.g[T1][0] = Addr(reg(c, V0)).lw(c);
                // j = i - gap; shift smaller keys up while j >= 0.
                loop {
                    c.cycles += 6;
                    let v = reg(c, A3) << 2;
                    set(c, A1, v);
                    c.g[V0][0] = Addr(reg(c, A1).wrapping_add(reg(c, T2))).lw(c);
                    let smaller = less(c, V0, T1) != 0;
                    let v = reg(c, A3).wrapping_add(reg(c, T0));
                    set(c, V1, v);
                    if !smaller {
                        break;
                    }
                    c.cycles += 9;
                    let v = reg(c, A1).wrapping_add(reg(c, A2));
                    set(c, A1, v);
                    let v = reg(c, V1) << 2;
                    set(c, V1, v);
                    let v = reg(c, A3).wrapping_sub(reg(c, T0));
                    set(c, A3, v);
                    let v = reg(c, V1).wrapping_add(reg(c, T2));
                    set(c, A0, v);
                    let key = c.g[V0][0];
                    Addr(reg(c, A0)).sw(c, key);
                    let v = reg(c, V1).wrapping_add(reg(c, A2));
                    set(c, V1, v);
                    c.g[V0][0] = Addr(reg(c, A1)).lw(c);
                    let more = (c.g[A3][0] as i64) >= 0;
                    let val = c.g[V0][0];
                    Addr(reg(c, V1)).sw(c, val);
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003e_63b0);
                    }
                }
                // keys[j + gap] = key; vals[j + gap] = val.
                c.cycles += 9;
                let v = reg(c, A3).wrapping_add(reg(c, T0));
                set(c, V0, v);
                c.g[A1][0] = c.g[T3][0];
                let v = reg(c, V0) << 2;
                set(c, V0, v);
                c.g[A0][0] = less(c, A1, T5);
                let v = reg(c, V0).wrapping_add(reg(c, T2));
                set(c, V1, v);
                let v = reg(c, V0).wrapping_add(reg(c, A2));
                set(c, V0, v);
                let (key, val) = (c.g[T1][0], c.g[T4][0]);
                Addr(reg(c, V1)).sw(c, key);
                Addr(reg(c, V0)).sw(c, val);
                if c.g[A0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003e_6390);
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[T0][0] as i64) < 2) as u64;
        let v = reg(c, T6).wrapping_sub(4);
        set(c, T6, v);
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003e_6380);
        }
    }
}

// ---------------------------------------------------------------------------
// BankMonitor_AddInstance (0x2acfe0, bankmonitor.cpp)

/// BankMonitor fields: current sound type, per-type tables (inline arrays
/// at 0x24 and 0x40, pointers to per-type arrays elsewhere), the owner's
/// vtable, the busy counter and the instance array (64 x 0xc0 bytes).
const OC264_BM_KIND: u32 = 0x1c;
const OC264_BM_SOURCES: u32 = 0x24;
const OC264_BM_TABLE_FADE: u32 = 0x34;
const OC264_BM_TABLE_SLOT: u32 = 0x3c;
const OC264_BM_PARAMS: u32 = 0x40;
const OC264_BM_TABLE_80: u32 = 0x60;
const OC264_BM_TABLE_84: u32 = 0x64;
const OC264_BM_TABLE_88: u32 = 0x68;
const OC264_BM_TABLE_8C: u32 = 0x6c;
const OC264_BM_TABLE_94: u32 = 0x74;
const OC264_BM_TABLE_98: u32 = 0x78;
const OC264_BM_TABLE_9C: u32 = 0x7c;
const OC264_BM_TABLE_A0: u32 = 0x80;
const OC264_BM_TABLE_A4: u32 = 0x84;
const OC264_BM_ADD_COUNT: u32 = 0x8dc;
const OC264_BM_INSTANCES: u32 = 0x8ec;
const OC264_BM_INSTANCE_SIZE: u64 = 0xc0;

/// BankMonitorInstance fields written here.
const OC264_BMI_TYPE: u32 = 0x0;
const OC264_BMI_ID: u32 = 0x4;
const OC264_BMI_ACTIVE: u32 = 0x8;
const OC264_BMI_FLAGS: u32 = 0x58;
const OC264_BMI_SLOT: u32 = 0x60;
const OC264_BMI_VOLUME: u32 = 0x64;
const OC264_BMI_SOURCE: u32 = 0x68;
const OC264_BMI_SOURCE_COPY: u32 = 0x70;
const OC264_BMI_PARAMS: u32 = 0xa8;

/// "no free instance" message (`Audio_DebugPrintf` format string).
const OC264_BM_FULL_MSG: u32 = 0x0048_3250;
const OC264_SND_STOP: u32 = 0x003b_89e0;
const OC264_SNDMIDI_STOP: u32 = 0x003b_9920;

const OC264_BM_FIND_FREE_INSTANCE: Callee = Callee {
    addr: 0x002a_cf60,
    f: ee::s08::BankMonitor_FindFreeInstance,
};
const OC264_BMI_RESET: Callee = Callee {
    addr: 0x002a_bb38,
    f: ee::s08::BankMonitorInstance_Reset,
};
const OC264_BMI_UPDATE_DISTANCE: Callee = Callee {
    addr: 0x002a_bd48,
    f: ee::s08::BankMonitorInstance_UpdateDistance,
};
const OC264_AUDIO_DEBUG_PRINTF: Callee = Callee {
    addr: 0x002b_9100,
    f: ee::s08::Audio_DebugPrintf,
};

/// `&table[self->kind]` for the per-type table pointer at `self + field`
/// (the pointer is loaded into `base_reg`, the slot address into v0).
fn oc264_bm_kind_slot(c: &mut Cpu, this: u32, base_reg: usize, field: u32) -> u32 {
    let kind = Addr(this.wrapping_add(OC264_BM_KIND)).lw(c) as u32;
    let base = Addr(this.wrapping_add(field)).lw(c);
    c.g[base_reg][0] = base;
    let slot = (kind << 2).wrapping_add(base as u32);
    c.g[V0][0] = sx(slot);
    slot
}

/// `table[self->kind]` for the per-type table pointer at `self + field`.
fn oc264_bm_kind_word(c: &mut Cpu, this: u32, base_reg: usize, field: u32) -> u64 {
    let slot = oc264_bm_kind_slot(c, this, base_reg, field);
    Addr(slot).lw(c)
}

/// `&self->sources[self->kind]` (inline array at 0x24, base in a3).
fn oc264_bm_source_slot(c: &mut Cpu, this: u32) -> u32 {
    let kind = Addr(this.wrapping_add(OC264_BM_KIND)).lw(c) as u32;
    let slot = r32(c, A3).wrapping_add(kind << 2);
    c.g[V0][0] = sx(slot);
    slot
}

/// `BankMonitorInstance* BankMonitor_AddInstance(BankMonitor* self, int id,
/// int volume, int type)` (0x2acfe0): takes a free instance slot
/// (`BankMonitor_FindFreeInstance`), resets it and fills it from the
/// monitor's per-sound-type tables (slot, source position, params, fades,
/// pitch/volume curves), then measures its distance when it has a source.
/// With no free slot it prints a warning and stops the sound `id` that was
/// to be tracked (`SND_Stop` for type 1, `SNDMIDI_Stop` for type 2).
/// Returns the instance or 0.
pub fn bank_monitor_add_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        c.g[S2][0] = c.g[A3][0];
        frame.save(c, S4, 0x30);
        c.g[S3][0] = c.g[A1][0];
        frame.save_ra(c, 0x20);
        c.g[S4][0] = c.g[A2][0];
        c.g[S1][0] = 0;
        let this = r32(c, S0);
        let count = Addr(this.wrapping_add(OC264_BM_ADD_COUNT));
        let n = sx((count.lw(c) as u32).wrapping_add(1));
        c.g[V0][0] = n;
        count.sw(c, n);
        c.g[A1][0] = count.lw(c);
        OC264_BM_FIND_FREE_INSTANCE.call(c, 0x002a_d028)?;
        c.g[A1][0] = c.g[V0][0];
        let none = (c.g[A1][0] as i64) < 0;
        c.g[V0][0] = OC264_BM_INSTANCE_SIZE;
        if !none {
            c.cycles += 66;
            c.g[V1][0] = Addr(this.wrapping_add(OC264_BM_INSTANCES)).lw(c);
            c.mult(0, V0, A1, V0);
            c.g[S1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[A0][0] = c.g[S1][0];
            OC264_BMI_RESET.call(c, 0x002a_d048)?;
            let inst = r32(c, S1);
            let at = |off: u32| Addr(inst.wrapping_add(off));
            let (kind, id, volume) = (c.g[S2][0], c.g[S3][0], c.g[S4][0]);
            at(OC264_BMI_TYPE).sw(c, kind);
            c.g[A1][0] = 0xffff_ffff_0000_0000;
            at(OC264_BMI_ID).sw(c, id);
            c.g[A0][0] &= c.g[A1][0];
            c.g[A3][0] = sx(this.wrapping_add(OC264_BM_SOURCES));
            c.g[A2][0] = 0x0000_0000_ffff_ffff;

            // Bank slot and volume.
            let slot = oc264_bm_kind_word(c, this, V1, OC264_BM_TABLE_SLOT);
            c.g[A1][0] = slot;
            at(OC264_BMI_VOLUME).sw(c, volume);
            at(OC264_BMI_SLOT).sw(c, slot);
            // Source position pointer.
            let p = oc264_bm_source_slot(c, this);
            let source = Addr(p).lw(c);
            c.g[V1][0] = source;
            at(OC264_BMI_SOURCE).sw(c, source);
            // The type's 8-byte parameter pair, packed into a doubleword and
            // stored unaligned through the stack.
            let kind_ix = Addr(this.wrapping_add(OC264_BM_KIND)).lw(c) as u32;
            let pair = this.wrapping_add(kind_ix << 3);
            c.g[V0][0] = sx(pair);
            c.g[A1][0] = Addr(pair.wrapping_add(OC264_BM_PARAMS)).lwu(c);
            c.g[V1][0] = Addr(pair.wrapping_add(OC264_BM_PARAMS + 4)).lwu(c);
            c.g[A0][0] |= c.g[A1][0];
            c.g[V1][0] <<= 32;
            c.g[A0][0] &= c.g[A2][0];
            c.g[A0][0] |= c.g[V1][0];
            c.g[V0][0] = (c.g[A0][0] << 32) >> 32;
            c.g[A0][0] >>= 32;
            let (lo, hi) = (c.g[V0][0], c.g[A0][0]);
            Addr(frame.at(c, 0x10)).sw(c, lo);
            Addr(frame.at(c, 0x14)).sw(c, hi);
            let s = sp(c);
            c.ldl(V0, s.wrapping_add(0x17));
            c.ldr(V0, s.wrapping_add(0x10));
            c.sdl(V0, s.wrapping_add(0x7));
            c.sdr(V0, s);
            c.ldl(V0, s.wrapping_add(0x7));
            c.ldr(V0, s);
            c.sdl(V0, inst.wrapping_add(OC264_BMI_PARAMS + 7));
            c.sdr(V0, inst.wrapping_add(OC264_BMI_PARAMS));
            let w = oc264_bm_kind_word(c, this, A0, OC264_BM_TABLE_80);
            c.g[V1][0] = w;
            at(0x80).sw(c, w);
            let w = oc264_bm_kind_word(c, this, A0, OC264_BM_TABLE_84);
            c.g[V1][0] = w;
            at(0x84).sw(c, w);
            // A fade table entry replaces the source with a private copy.
            let fade = oc264_bm_kind_word(c, this, A0, OC264_BM_TABLE_FADE);
            c.g[V1][0] = fade;
            c.g[A0][0] = sx(inst.wrapping_add(OC264_BMI_SOURCE_COPY));
            if fade != 0 {
                c.cycles += 4;
                c.g[V0][0] = at(OC264_BMI_SOURCE).lw(c);
                let source = r32(c, V0);
                c.lq(V1, source);
                let copy = c.g[A0][0];
                at(OC264_BMI_SOURCE).sw(c, copy);
                c.sq(V1, inst.wrapping_add(OC264_BMI_SOURCE_COPY));
            }

            c.cycles += 31;
            let p = oc264_bm_kind_slot(c, this, V1, OC264_BM_TABLE_88);
            c.g[A1][0] = at(OC264_BMI_FLAGS).lw(c);
            c.lwc1(0, p);
            c.swc1(0, inst.wrapping_add(0x88));
            let p = oc264_bm_kind_slot(c, this, V1, OC264_BM_TABLE_8C);
            c.lwc1(0, p);
            c.swc1(0, inst.wrapping_add(0x8c));
            let w = oc264_bm_kind_word(c, this, A0, OC264_BM_TABLE_94);
            c.g[V1][0] = w;
            at(0x94).sw(c, w);
            let w = oc264_bm_kind_word(c, this, A0, OC264_BM_TABLE_98);
            c.g[V1][0] = w;
            at(0x98).sw(c, w);
            let p = oc264_bm_kind_slot(c, this, V1, OC264_BM_TABLE_9C);
            let flagged = c.g[A1][0] != 0;
            let w = Addr(p).lw(c);
            c.g[V0][0] = w;
            if !flagged {
                c.cycles += 1;
                at(0x9c).sw(c, w);
            }

            c.cycles += 21;
            c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0f
            c.mtc1(AT, 0);
            c.swc1(0, inst.wrapping_add(0x90));
            let w = oc264_bm_kind_word(c, this, A0, OC264_BM_TABLE_A0);
            c.g[V1][0] = w;
            at(0xa0).sw(c, w);
            let w = oc264_bm_kind_word(c, this, A0, OC264_BM_TABLE_A4);
            c.g[V1][0] = w;
            at(0xa4).sw(c, w);
            let p = oc264_bm_source_slot(c, this);
            c.g[V1][0] = Addr(p).lw(c);
            c.g[A0][0] = c.g[S1][0];
            if c.g[V1][0] != 0 {
                c.cycles += 2;
                c.g[A1][0] = c.g[S0][0];
                OC264_BMI_UPDATE_DISTANCE.call(c, 0x002a_d228)?;
            }
            c.cycles += 4;
            // active = id >= 0.
            let active = sx((!c.g[S3][0] as u32) >> 31);
            c.g[V0][0] = active;
            Addr(r32(c, S1).wrapping_add(OC264_BMI_ACTIVE)).sw(c, active);
        } else {
            c.cycles += 6;
            c.g[A0][0] = sx(OC264_BM_FULL_MSG);
            OC264_AUDIO_DEBUG_PRINTF.call(c, 0x002a_d244)?;
            let kind = c.g[S2][0];
            c.g[V0][0] = 2;
            if kind == 1 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, OC264_SND_STOP, 0x002a_d268)?;
            } else {
                c.cycles += 2;
                if kind == 2 {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, OC264_SNDMIDI_STOP, 0x002a_d278)?;
                } else {
                    c.cycles += 2;
                }
            }
        }
        c.cycles += 12;
        let count = Addr(r32(c, S0).wrapping_add(OC264_BM_ADD_COUNT));
        c.g[V1][0] = count.lw(c);
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S1, 0x60);
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
        frame.restore(c, S2, 0x50);
        let left = c.g[V1][0];
        count.sw(c, left);
        frame.restore(c, S0, 0x70);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// MEM_Pool256Alloc (0x320408, cubicspline.cpp)

/// The 256-byte pool's free-list head (offset in the pool object).
const OC264_POOL256_FREE_HEAD: u32 = 0x2_0000;

const OC264_MEM_POOL_ALLOC: Callee = Callee {
    addr: 0x0031_9e48,
    f: ee::s10::MEM_PoolAlloc,
};

/// `void* MEM_Pool256Alloc(Pool256* pool)` (0x320408): pops a block off the
/// 256-byte pool's free list; when the list is empty falls back to
/// `MEM_PoolAlloc(0x104, align)` (the caller's a1 is passed through).
pub fn mem_pool256_alloc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 0x2_0000;
        frame.save_ra(c, 0);
        let head = r32(c, A0).wrapping_add(OC264_POOL256_FREE_HEAD);
        c.g[A0][0] = sx(head);
        let block = Addr(head).lw(c);
        c.g[V0][0] = block;
        if block != 0 {
            c.cycles += 3;
            let next = Addr(block as u32).lw(c);
            c.g[V1][0] = next;
            Addr(head).sw(c, next);
        } else {
            c.cycles += 4;
            c.g[A0][0] = 0x104;
            OC264_MEM_POOL_ALLOC.call(c, 0x0032_042c)?;
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PS2GraphicsMan_TryAcquireFrame (0x395f60, font.cpp)

/// `PS2GraphicsMan_AcquireFrameBuffer` (reached through the function table).
const OC264_GM_ACQUIRE_FRAME_BUFFER: u32 = 0x0038_26e0;

/// `bool PS2GraphicsMan_TryAcquireFrame(PS2GraphicsMan* self)` (0x395f60):
/// forwards to `PS2GraphicsMan_AcquireFrameBuffer(self)` (claims the
/// current frame buffer if it is free) and returns its result.
pub fn ps2_graphics_man_try_acquire_frame(c: &mut Cpu) {
    crate::call::tail_wrapper(
        c,
        None,
        crate::call::Target::Table(OC264_GM_ACQUIRE_FRAME_BUFFER),
        0x0039_5f70,
    );
}

// ---------------------------------------------------------------------------
// BEBAGTInt_GetUberTrickSlotValue (0x14fea8, beintbagt.cpp)

/// Uber trick table (`D_0045AEB8`, 0x14-byte entries; +0xc: slot enabled).
const OC264_UBER_TRICKS: u32 = 0x0045_aeb8;
const OC264_UBER_TRICK_SIZE: u64 = 0x14;
const OC264_UBER_TRICK_ENABLED: u32 = 0xc;
/// Per profile/character/trick slot bytes (`D_00530EC0`): 6 bytes per
/// trick, 0x1fe per character, 0x13ec per profile.
const OC264_UBER_SLOTS: u32 = 0x0053_0ec0;

const OC264_BE_GET_RIDER_PROFILE: Callee = Callee {
    addr: 0x0014_a0e0,
    f: ee::s01::BE_GetRiderProfile,
};
const OC264_BE_GET_RIDER_CHARACTER: Callee = Callee {
    addr: 0x0014_a080,
    f: ee::s01::BE_GetRiderCharacter,
};

/// `int BEBAGTInt_GetUberTrickSlotValue(void* self, int rider, int trick,
/// int second)` (0x14fea8): -1 when uber trick `trick` has no slot,
/// else the first (or, with `second`, the second) slot byte of that trick
/// for the rider's profile and character.
pub fn bebagt_int_get_uber_trick_slot_value(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x50);
        c.g[A0][0] = OC264_UBER_TRICK_SIZE;
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S1, 0x30);
        c.mult(0, A0, S0, A0);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[V0][0] = sx(OC264_UBER_TRICKS);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        let entry = r32(c, A0).wrapping_add(OC264_UBER_TRICKS);
        c.g[A0][0] = sx(entry);
        c.g[V1][0] = Addr(entry.wrapping_add(OC264_UBER_TRICK_ENABLED)).lhu(c);
        c.g[S2][0] = c.g[A3][0];
        if c.g[V1][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = u64::MAX;
        } else {
            c.cycles += 7;
            c.g[A0][0] = c.g[S1][0];
            OC264_BE_GET_RIDER_PROFILE.call(c, 0x0014_fef8)?;
            c.g[S3][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            OC264_BE_GET_RIDER_CHARACTER.call(c, 0x0014_ff04)?;
            c.g[V1][0] = c.g[V0][0];
            let second = c.g[S2][0] != 0;
            c.cycles += if second { 13 } else { 12 };
            // slots + trick * 6 + character * 0x1fe + profile * 0x13ec
            c.g[A1][0] = 0x1fe;
            c.g[V0][0] = 6;
            c.mult(0, A1, V1, A1);
            c.mult(0, V0, S0, V0);
            c.g[V1][0] = 0x13ec;
            c.mult(0, V1, S3, V1);
            c.g[A0][0] = sx(OC264_UBER_SLOTS);
            let at = r32(c, V0)
                .wrapping_add(r32(c, A1))
                .wrapping_add(r32(c, V1))
                .wrapping_add(OC264_UBER_SLOTS);
            c.g[V0][0] = Addr(at.wrapping_add(second as u32)).lbu(c);
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
// WScriptMan_FocusLeave (0x30a460, wscriptman.cpp)

/// World object: its script controller (or 0).
const OC264_WOBJ_CTL: u32 = 0xc;
/// Script controller: vtable, kind (6: no focus scripts), owner object.
const OC264_WCTL_VTABLE: u32 = 0xc;
const OC264_WCTL_KIND: u32 = 0x10;
const OC264_WCTL_OWNER: u32 = 0x18;
const OC264_WCTL_KIND_PASSIVE: u64 = 6;
/// Controller vtable entries (this-adjust halfword, method): v20 "handles
/// focus itself", v22 "focus leave", v35 "lost focus".
const OC264_WCTL_V20: u32 = 0xa0;
const OC264_WCTL_V22: u32 = 0xb0;
const OC264_WCTL_V35: u32 = 0x118;

const OC264_WSCRIPT_HAS_OBJ_SCRIPT3: Callee = Callee {
    addr: 0x0030_a2e8,
    f: ee::s10::WScriptMan_HasObjScript3,
};
const OC264_WSCRIPT_RUN_OBJ_SCRIPT3: Callee = Callee {
    addr: 0x0030_a548,
    f: ee::s10::WScriptMan_RunObjScript3,
};

/// Call virtual method `slot` (this-adjust halfword at `slot`, function
/// at `slot + 4`) of the controller in `ctl_reg`, loading the vtable into
/// `vt_reg` and the method into `fn_reg` as the original does.
fn oc264_wctl_call(
    c: &mut Cpu,
    ctl_reg: usize,
    vt_reg: usize,
    fn_reg: usize,
    slot: u32,
    ret: u32,
) -> crate::call::Flow {
    let ctl = r32(c, ctl_reg);
    let vtable = Addr(ctl.wrapping_add(OC264_WCTL_VTABLE)).lw(c);
    c.g[vt_reg][0] = vtable;
    let adjust = Addr((vtable as u32).wrapping_add(slot)).lh(c);
    let method = Addr((vtable as u32).wrapping_add(slot + 4)).lw(c);
    c.g[fn_reg][0] = method;
    c.g[A0][0] = sx(ctl.wrapping_add(adjust as u32));
    call_ptr(c, method as u32, ret)
}

/// `void WScriptMan_FocusLeave(WScriptMan* self, WorldObj* obj)` (0x30a460):
/// the camera/focus leaves `obj`. A scripted controller that handles focus
/// itself (v20) gets v22; a passive one (kind 6) nothing; otherwise the
/// controller's owner runs its "focus leave" object script (slot 3) if it
/// has one, else the controller's v35. Without a controller the object's
/// own slot-3 script runs if present.
pub fn wscript_man_focus_leave(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr(r32(c, S0).wrapping_add(OC264_WOBJ_CTL)).lw(c);
        c.g[S1][0] = c.g[A0][0];
        if c.g[A1][0] != 0 {
            c.cycles += 7;
            oc264_wctl_call(c, A1, V1, V0, OC264_WCTL_V20, 0x0030_a494)?;
            let handles = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S0][0];
            if handles {
                c.cycles += 8;
                c.g[A2][0] = Addr(r32(c, S0).wrapping_add(OC264_WOBJ_CTL)).lw(c);
                oc264_wctl_call(c, A2, V1, V0, OC264_WCTL_V22, 0x0030_a4b4)?;
                frame.restore(c, S0, 0x20);
            } else {
                c.cycles += 5;
                let ctl = Addr(r32(c, S0).wrapping_add(OC264_WOBJ_CTL)).lw(c);
                c.g[A1][0] = ctl;
                c.g[V1][0] = OC264_WCTL_KIND_PASSIVE;
                c.g[V0][0] = Addr((ctl as u32).wrapping_add(OC264_WCTL_KIND)).lh(c);
                let passive = c.g[V0][0] == c.g[V1][0];
                c.g[A0][0] = c.g[S1][0];
                if passive {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x20);
                } else {
                    c.cycles += 5;
                    c.g[S0][0] = Addr((ctl as u32).wrapping_add(OC264_WCTL_OWNER)).lw(c);
                    c.g[A1][0] = c.g[S0][0];
                    OC264_WSCRIPT_HAS_OBJ_SCRIPT3.call(c, 0x0030_a4dc)?;
                    let has = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[S1][0];
                    if has {
                        c.cycles += 4;
                        c.g[A1][0] = c.g[S0][0];
                        OC264_WSCRIPT_RUN_OBJ_SCRIPT3.call(c, 0x0030_a4ec)?;
                    } else {
                        c.cycles += 8;
                        c.g[A1][0] = Addr(r32(c, S0).wrapping_add(OC264_WOBJ_CTL)).lw(c);
                        oc264_wctl_call(c, A1, V0, V1, OC264_WCTL_V35, 0x0030_a50c)?;
                    }
                    frame.restore(c, S0, 0x20);
                }
            }
        } else {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            OC264_WSCRIPT_HAS_OBJ_SCRIPT3.call(c, 0x0030_a520)?;
            let has = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S1][0];
            if has {
                c.cycles += 2;
                c.g[A1][0] = c.g[S0][0];
                OC264_WSCRIPT_RUN_OBJ_SCRIPT3.call(c, 0x0030_a530)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, fill, put_words, Rng, DATA};

    /// StreamArray_GetState: empty and non-empty slots; the real
    /// StreamBuffer_GetState runs on a buffer whose stream ids are small.
    #[test]
    fn stream_array_get_state_matches() {
        check(ee::s08::f_002aae08__generated, stream_array_get_state, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x2aae_08, 0);
            c.pc = 0x002a_ae08;
            let arr = DATA + 0x100;
            let list = DATA + 0x200;
            let buffer = DATA + 0x1000;
            c.g[A0][0] = sx(arr);
            c.g[A1][0] = sx(seed % 16);
            c.bus.write32(arr + OC264_SA_BUFFERS, list);
            let entry = if seed % 3 == 0 { 0 } else { buffer };
            c.bus.write32(list + 4 * (seed % 16), entry);
            for i in 0..8u32 {
                c.bus.write32(buffer + 4 * i, r.next_u32() % 4);
            }
        });
    }

    /// LensFX_AddSources: counts -1..5 (none, one, several) into lists 0..3;
    /// the real LensFX_AddSource (a leaf) appends each source.
    #[test]
    fn lensfx_add_sources_matches() {
        check(ee::s09::f_002e2ff8__generated, lensfx_add_sources, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x2e2f_f8, 0);
            c.pc = 0x002e_2ff8;
            let this = DATA + 0x2_0000;
            let sources = DATA + 0x100;
            let list = seed % 4;
            let count = (seed % 7) as i32 - 1;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(sources);
            c.g[A2][0] = sx(count as u32);
            c.g[T0][0] = sx(list);
            fill(c, &mut r, sources, 8);
            // Per-list source counts at self + 0x60.
            for i in 0..4u32 {
                c.bus.write32(this + 0x60 + 4 * i, r.next_u32() % 4);
            }
        });
    }

    /// RiderSphereTree_Assign: random headers with 0..5 children, copied
    /// by the real EA_MemCopy.
    #[test]
    fn rider_sphere_tree_assign_matches() {
        check(ee::s10::f_00329a28__generated, rider_sphere_tree_assign, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x329a_28, 0);
            c.pc = 0x0032_9a28;
            let this = DATA + 0x1000;
            let src = DATA + 0x2000;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(src);
            fill(c, &mut r, this, 0xd0 / 4);
            fill(c, &mut r, src, 0xd0 / 4);
            c.bus.write32(src + OC264_RST_COUNT, seed % 6);
        });
    }

    /// EA_ShellSortPairs: 0..60 pairs (gaps 1, 4, 13, 40) with keys from a
    /// small range (ties) or the full range (negatives).
    #[test]
    fn ea_shell_sort_pairs_matches() {
        const GAPS: [u32; 14] = [
            1, 4, 13, 40, 121, 364, 1093, 3280, 9841, 29524, 88573, 265720, 797161,
            0x7fff_ffff,
        ];
        check(ee::s14::f_003e6328__generated, ea_shell_sort_pairs, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x3e63_28, 0);
            c.pc = 0x003e_6328;
            put_words(c, OC264_SHELL_GAPS, &GAPS);
            let n = seed % 61;
            let keys = DATA + 0x1000;
            let vals = DATA + 0x2000;
            c.g[A0][0] = sx(n);
            c.g[A1][0] = sx(keys);
            c.g[A2][0] = sx(vals);
            for i in 0..n {
                let k = if seed % 2 == 0 { r.next_u32() % 8 } else { r.next_u32() };
                c.bus.write32(keys + 4 * i, k);
                c.bus.write32(vals + 4 * i, r.next_u32());
            }
        });
    }

    /// BankMonitor_AddInstance: a monitor with 64 instances, some free (the
    /// real FindFreeInstance/PollFree/Reset run), sound types 0..3 with
    /// filled per-type tables, a fade entry or not, a source or not (the
    /// owner's distance method is a fake); every fourth seed has no free
    /// instance and stops the sound (types 0..3, invalid handles take the
    /// SND library's short paths).
    #[test]
    fn bank_monitor_add_instance_matches() {
        const THIS: u32 = DATA + 0x2_0000;
        const TABLES: u32 = DATA + 0x1000;
        const SOURCES: u32 = DATA + 0x3000;
        const INSTANCES: u32 = DATA + 0x4000;
        const VTABLE: u32 = DATA + 0x8000;
        check(ee::s08::f_002acfe0__generated, bank_monitor_add_instance, 240, &|c, seed| {
            let mut r = machine(c, seed, 0x2acf_e0, 1);
            c.pc = 0x002a_cfe0;
            c.g[A0][0] = sx(THIS);
            let id = if seed % 5 == 0 { u32::MAX } else { r.next_u32() };
            c.g[A1][0] = sx(id);
            c.g[A2][0] = sx(r.next_u32());
            c.g[A3][0] = sx(seed % 4);
            let kind = r.next_u32() % 4;
            c.bus.write32(THIS + OC264_BM_KIND, kind);
            c.bus.write32(THIS + OC264_BM_ADD_COUNT, r.next_u32() % 3);
            c.bus.write32(THIS + OC264_BM_INSTANCES, INSTANCES);
            c.bus.write32(THIS + 0x8d8, VTABLE);
            c.bus.write16(VTABLE + 0x30, (r.next_u32() % 0x40) as u16);
            c.bus.write32(VTABLE + 0x34, FAKE);
            // Inline per-type source pointers (0 = no source) and params.
            for k in 0..4u32 {
                let src = if r.next_u32() % 3 == 0 { 0 } else { SOURCES + 0x10 * k };
                c.bus.write32(THIS + OC264_BM_SOURCES + 4 * k, src);
            }
            fill(c, &mut r, THIS + OC264_BM_PARAMS, 8);
            fill(c, &mut r, SOURCES, 0x40 / 4);
            // Per-type tables of 4 words each.
            let fields = [
                OC264_BM_TABLE_FADE,
                OC264_BM_TABLE_SLOT,
                OC264_BM_TABLE_80,
                OC264_BM_TABLE_84,
                OC264_BM_TABLE_88,
                OC264_BM_TABLE_8C,
                OC264_BM_TABLE_94,
                OC264_BM_TABLE_98,
                OC264_BM_TABLE_9C,
                OC264_BM_TABLE_A0,
                OC264_BM_TABLE_A4,
            ];
            for (i, &f) in fields.iter().enumerate() {
                let table = TABLES + 0x10 * i as u32;
                c.bus.write32(THIS + f, table);
                fill(c, &mut r, table, 4);
            }
            if seed % 2 == 0 {
                c.bus.write32(TABLES + 4 * kind, 0); // no fade for this type
            }
            // Instances: free (type 0) or busy (type != 0, not done).
            let all_busy = seed % 4 == 0;
            for i in 0..64u32 {
                let inst = INSTANCES + 0xc0 * i;
                let busy = all_busy || r.next_u32() % 3 == 0;
                c.bus.write32(inst, if busy { 1 + r.next_u32() % 3 } else { 0 });
                c.bus.write32(inst + 0x9c, 0);
                c.bus.write32(inst + 0x54, r.next_u32() % 2);
            }
        });
    }

    /// MEM_Pool256Alloc: a non-empty free list (pop), or an empty one with
    /// the real MEM_PoolAlloc behind it: pools ready (served by the 512-byte
    /// pool) or not yet (dlmalloc on the arena the setup initialises).
    #[test]
    fn mem_pool256_alloc_matches() {
        use crate::difftest::GAME_GP;
        use crate::game::bx::cubicspline::tests::oc238_run;
        check(ee::s10::f_00320408__generated, mem_pool256_alloc, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x32_0408, 0);
            let pool = DATA + 0x1000;
            let blocks = DATA + 0x2_8000;
            match seed % 3 {
                0 => {
                    // Free list of a few 0x100-byte blocks.
                    let n = 1 + r.next_u32() % 4;
                    for i in 0..n {
                        let node = blocks + 0x100 * i;
                        c.bus.write32(node, if i + 1 == n { 0 } else { node + 0x100 });
                    }
                    c.bus.write32(pool + OC264_POOL256_FREE_HEAD, blocks);
                }
                1 => {
                    // Empty: MEM_PoolAlloc takes the ready 512-byte pool.
                    c.bus.write32(pool + OC264_POOL256_FREE_HEAD, 0);
                    c.bus.write32(GAME_GP + 0xdd8, 1);
                    let p512 = DATA + 0x4_0000;
                    let base = DATA + 0x5_0000;
                    for n in 0..8u32 {
                        let node = base + 0x200 * n;
                        c.bus.write32(node, if n == 7 { 0 } else { node + 0x200 });
                    }
                    c.bus.write32(p512 + 0x2_0000, base);
                    c.bus.write32(GAME_GP + 0x2a68, p512);
                }
                _ => {
                    // Empty, pools not ready: the dlmalloc heap.
                    oc238_run(
                        c,
                        0x0031_ff60,
                        ee::s10::f_0031ff60,
                        &[0x004f_f250, (DATA + 0x4_0000) as u64, 0x4_0000],
                    );
                    c.bus.write32(pool + OC264_POOL256_FREE_HEAD, 0);
                    c.bus.write32(GAME_GP + 0xdd8, 0);
                }
            }
            c.pc = 0x0032_0408;
            c.g[A0][0] = sx(pool);
            c.g[A1][0] = [0u64, 4, 8, 16, 64][(r.next_u32() % 5) as usize];
        });
    }

    /// PS2GraphicsMan_TryAcquireFrame: the current frame buffer's state
    /// free (0), claimed (1) or busy (2); the real AcquireFrameBuffer runs.
    #[test]
    fn ps2_graphics_man_try_acquire_frame_matches() {
        check(
            ee::s13::f_00395f60__generated,
            ps2_graphics_man_try_acquire_frame,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x39_5f60, 0);
                c.pc = 0x0039_5f60;
                let this = DATA + 0x1_0000;
                let cur = seed % 2;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x5a10, cur);
                c.bus.write32(this + 0x5a90 + 4 * cur, (seed / 2) % 3);
            },
        );
    }

    /// BEBAGTInt_GetUberTrickSlotValue: tricks with and without a slot,
    /// first and second byte; the real BE_GetRiderProfile/Character read a
    /// rider table whose profile entries are -1 (profile 2) or flag words.
    #[test]
    fn bebagt_int_get_uber_trick_slot_value_matches() {
        const RIDER_SLOTS: u32 = 0x0053_05b0;
        const RIDERS: u32 = 0x0053_5b20;
        check(
            ee::s01::f_0014fea8__generated,
            bebagt_int_get_uber_trick_slot_value,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x14_fea8, 0);
                c.pc = 0x0014_fea8;
                let rider = seed % 4;
                let trick = r.next_u32() % 24;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(rider);
                c.g[A2][0] = sx(trick);
                c.g[A3][0] = if seed % 3 == 0 { 0 } else { sx(r.next_u32() | 1) };
                let enabled = if seed % 5 == 0 { 0 } else { 1 + r.next_u32() % 0xffff };
                c.bus.write16(
                    OC264_UBER_TRICKS + 0x14 * trick + OC264_UBER_TRICK_ENABLED,
                    enabled as u16,
                );
                let slot = r.next_u32() % 4;
                c.bus.write32(RIDER_SLOTS + 4 * rider, slot);
                let rec = RIDERS + 0x1c * slot;
                let profile = if seed % 2 == 0 { u32::MAX } else { r.next_u32() };
                c.bus.write32(rec + 0xc, profile);
                c.bus.write32(rec + 0x10, r.next_u32());
                c.bus.write8(rec + 0x11, (r.next_u32() % 16) as u8);
                // The slot bytes this trick/character/profile can reach.
                for p in 0..3u32 {
                    for ch in 0..16u32 {
                        let at = OC264_UBER_SLOTS + 6 * trick + 0x1fe * ch + 0x13ec * p;
                        let v = r.next_u32();
                        c.bus.write8(at, v as u8);
                        c.bus.write8(at + 1, (v >> 8) as u8);
                    }
                }
            },
        );
    }

    /// WScriptMan_FocusLeave: no controller / a controller whose v20 is a
    /// fake (non-zero: v22) or the real zero leaf 0x1406c0, kinds 6 and
    /// other; the real HasObjScript3/RunObjScript3 and script cache lookup
    /// run. The "has script" path is reached by pointing the object's cache
    /// entry id at `man + 0x290`: RunObjScript3 stores the object there
    /// before its own lookup, which then finds -1 and skips the script call.
    #[test]
    fn wscript_man_focus_leave_matches() {
        const MAN: u32 = DATA;
        const TABLE: u32 = DATA + 0x400;
        const CACHE: u32 = DATA + 0xc00;
        const SECTION: u32 = DATA + 0x1200;
        const NO_SCRIPT: u32 = DATA + 0x1400;
        const OBJ: u32 = DATA + 0x800;
        const OWNER: u32 = DATA + 0x900;
        const CTL: u32 = DATA + 0xa00;
        const OWNER_CTL: u32 = DATA + 0xb00;
        const VTABLE: u32 = DATA + 0x1800;
        const FUNCS: u32 = DATA + 0x1_0000;
        const ZERO_LEAF: u32 = 0x0014_06c0;
        // Function entry `id` of slot 3.
        let func_at = |id: u32| FUNCS + (id >> 8) * 0x18 + 12;
        check(ee::s10::f_0030a460__generated, wscript_man_focus_leave, 240, &|c, seed| {
            let mut r = machine(c, seed, 0x30_a460, 2);
            c.pc = 0x0030_a460;
            c.g[A0][0] = sx(MAN);
            c.g[A1][0] = sx(OBJ);
            // Script cache: man -> table -> cache -> section -> functions.
            c.bus.write32(MAN + 0x28c, TABLE);
            c.bus.write32(TABLE + 4, DATA + 0x600);
            c.bus.write32(DATA + 0x604, CACHE);
            c.bus.write32(CACHE + 8, SECTION);
            c.bus.write32(SECTION + 0x1c, FUNCS);
            c.bus.write32(NO_SCRIPT + 8, u32::MAX);
            c.bus.write32(MAN + 0x290, 0x100);
            c.bus.write32(func_at(0x100), r.next_u32() & 0x7fff_ffff);
            c.bus.write32(func_at(OBJ), u32::MAX);
            c.bus.write32(func_at(OWNER), u32::MAX);
            let has_script = seed % 2 == 0;
            for o in [OBJ, OWNER] {
                c.bus.write8(o + 0x78, 0);
                let entry = if has_script { MAN + 0x290 - 8 } else { NO_SCRIPT };
                c.bus.write32(o + 0x88, entry);
            }
            // Controllers.
            let has_ctl = seed % 6 >= 2;
            c.bus.write32(OBJ + OC264_WOBJ_CTL, if has_ctl { CTL } else { 0 });
            c.bus.write32(OWNER + OC264_WOBJ_CTL, OWNER_CTL);
            for ctl in [CTL, OWNER_CTL] {
                c.bus.write32(ctl + OC264_WCTL_VTABLE, VTABLE);
            }
            let kind = if seed % 6 == 4 { 6 } else { r.next_u32() % 6 };
            c.bus.write16(CTL + OC264_WCTL_KIND, kind as u16);
            c.bus.write32(CTL + OC264_WCTL_OWNER, OWNER);
            let v20 = if seed % 3 == 0 { FAKE } else { ZERO_LEAF };
            for (slot, f) in [
                (OC264_WCTL_V20, v20),
                (OC264_WCTL_V22, FAKE + 0x10),
                (OC264_WCTL_V35, FAKE + 0x20),
            ] {
                c.bus.write16(VTABLE + slot, (r.next_u32() % 0x20) as u16);
                c.bus.write32(VTABLE + slot + 4, f);
            }
        });
    }
}
