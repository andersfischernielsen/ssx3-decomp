//! `src/camera/trigger/cameratriggerlist.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::feasyncfile::r;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0016_c098,
        camera_trigger_man_destroy,
        "CameraTriggerMan_Destroy",
    ),
    (
        0x0016_c120,
        camera_trigger_man_add_trigger,
        "CameraTriggerMan_AddTrigger",
    ),
    (
        0x0016_c170,
        camera_trigger_man_load_file,
        "CameraTriggerMan_LoadFile",
    ),
    (
        0x0016_c3d0,
        camera_trigger_man_read_header,
        "CameraTriggerMan_ReadHeader",
    ),
    (
        0x0016_bf40,
        camera_trigger_man_free_triggers,
        "CameraTriggerMan_FreeTriggers",
    ),
    (
        0x0016_c228,
        camera_trigger_man_parse,
        "CameraTriggerMan_Parse",
    ),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {

    pub(crate) use crate::game::camera::camera::callee::CAMERA_TRIGGER_FREE_TRIGGERS;

    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_FREE_TRIGGERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_PARSE_2 as CAMERA_TRIGGER_MAN_PARSE;
}

/// Cross-shard direct calls the generator emitted as `call_indirect(c, addr)`.
pub(crate) mod table {

    pub(crate) use crate::game::wscript::wscriptman::table::SERIALIZE_READ_U32_2;
}

/// `void CameraTriggerMan_AddTrigger(CameraTriggerMan *man, int value, int index)`
/// (0x16c120): append `value` at `index` (or at the running count when `index`
/// is negative and there is room).
pub fn camera_trigger_man_add_trigger(c: &mut Cpu) {
    let man = Addr(abi::arg(c, 0));
    let value = abi::arg64(c, 1);
    let index = c.g[A2][0];
    c.cycles += 2;
    if (index as i64) < 0 {
        c.cycles += 4;
        let count = man.off(0x4).lw(c);
        c.g[V1][0] = count;
        let room = (count as i64) < 120;
        c.g[V0][0] = room as u64;
        if room {
            let base = man.lw(c);
            c.g[V0][0] = base;
            c.cycles += 7;
            let slot = (count as u32).wrapping_shl(2).wrapping_add(base as u32);
            c.g[V1][0] = sx(slot);
            Addr(slot).sw(c, value);
            c.g[V0][0] = man.off(0x4).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            man.off(0x4).sw(c, c.g[V0][0]);
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        c.g[V0][0] = 1;
        abi::ret(c);
        return;
    }
    c.g[V1][0] = man.lw(c);
    c.cycles += 3;
    c.g[V0][0] = sx((index as u32) << 2);
    let slot = (c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32);
    c.g[V0][0] = sx(slot);
    Addr(slot).sw(c, value);
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `EA_MemFree` (0x317e98).
pub(crate) const MEM_FREE: u32 = 0x0031_7e98;

// ---------------------------------------------------------------------------
// CameraTriggerMan_Destroy.

/// `void CameraTriggerMan_Destroy(this)` (0x16c098): free the trigger array,
/// then free the manager itself and clear the caller's pointer.
pub fn camera_trigger_man_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        callee::CAMERA_TRIGGER_FREE_TRIGGERS.call(c, 0x0016_c0ac)?;
        c.g[A0][0] = Addr(this as u32).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 4;
            call_ptr(c, MEM_FREE, 0x0016_c0c0)?;
            c.sw(this as u32, 0);
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cCameraTriggerMan_ReadHeader(cCameraTriggerMan* self, void* buffer)`
/// (0x16c3d0): read three words of a trigger list header.
pub fn camera_trigger_man_read_header(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0]; // the serialised buffer
        c.g[S1][0] = c.g[A0][0]; // the header object
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx(r32(c, S1).wrapping_add(0x10));
        call_ptr(c, table::SERIALIZE_READ_U32_2, 0x0016_c3f4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx(r32(c, S1).wrapping_add(4));
        call_ptr(c, table::SERIALIZE_READ_U32_2, 0x0016_c400)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx(r32(c, S1).wrapping_add(8));
        call_ptr(c, table::SERIALIZE_READ_U32_2, 0x0016_c40c)?;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Camera trigger file loading.

/// `int CameraTriggerMan_LoadFile(CameraTriggerMan* m, const char* name)`
/// (0x16c170): free the old trigger list, load and parse the named file,
/// returning 0 on success, 1 when the file is missing and 2 when the buffer
/// could not be allocated.
pub fn camera_trigger_man_load_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        'outer: {
            frame.save(c, S2, 0x20);
            frame.save(c, S3, 0x10);
            c.g[S2][0] = c.g[A1][0];
            let this = r(c, A0);
            c.g[S3][0] = sx(this);
            frame.save(c, S0, 0x40);
            frame.save(c, S1, 0x30);
            frame.save_ra(c, 0);
            c.g[RA][0] = sx(0x0016_c198);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0031_7f38, 0x0016_c198)?;
            let missing = c.g[V0][0] == 0;
            c.g[V0][0] = 1;
            if missing {
                break 'outer;
            }
            'mid: {
                c.cycles += 18;
                c.g[RA][0] = sx(0x0016_c1a8);
                c.g[A0][0] = c.g[S3][0];
                callee::CAMERA_TRIGGER_MAN_FREE_TRIGGERS.call(c, 0x0016_c1a8)?;
                c.g[RA][0] = sx(0x0016_c1b0);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0031_7f98, 0x0016_c1b0)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = sx(0x0045_bf30);
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = 0x100;
                c.g[RA][0] = sx(0x0016_c1cc);
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7e30, 0x0016_c1cc)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0016_c1e0);
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x003e_2470, 0x0016_c1e0)?;
                let ok = c.g[S1][0] != 0;
                c.g[A0][0] = c.g[S3][0];
                if ok {
                    break 'mid;
                }
                c.cycles += 2;
                c.g[V0][0] = 2;
                break 'outer;
            }
            c.cycles += 6;
            c.g[RA][0] = sx(0x0016_c1f8);
            c.g[A1][0] = c.g[S1][0];
            callee::CAMERA_TRIGGER_MAN_PARSE.call(c, 0x0016_c1f8)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0016_c204);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e98, 0x0016_c204)?;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 7;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int CameraTriggerMan_Parse(self, void** cursor)` (0x16c228): reads the
/// camera trigger list from a stream. Resets the header, reads the version
/// cookie (`self + 0xc`; anything but 7 resets it to 7 and returns 3), reads
/// the header (count at +4; a non-zero status is returned as is), allocates
/// the `count * 4` array (stored at `self + 0`) and for each trigger allocates
/// 0x24 bytes (`{kind 4, type 3}`, fields cleared), reads it with
/// `ReadTrigger` and adds it with `AddTrigger`. A failing read frees the
/// trigger with `CameraTrigger_dtor(t, 3)` (when allocated) and returns its
/// status. Returns 0 and sets `self + 0xc = 7` on success. Frame 0x80 (s0-s5).
pub fn camera_trigger_man_parse(c: &mut Cpu) {
    const RESET_HEADER: Callee = Callee {
        addr: 0x0016_c390,
        f: ee::s02::f_0016c390,
    };
    const READ_VERSION: Callee = Callee {
        addr: 0x0016_c3a8,
        f: ee::s02::f_0016c3a8,
    };
    const READ_HEADER: Callee = Callee {
        addr: 0x0016_c3d0,
        f: ee::s02::f_0016c3d0,
    };
    const READ_TRIGGER: Callee = Callee {
        addr: 0x0016_c428,
        f: ee::s02::f_0016c428,
    };
    const ADD_TRIGGER: Callee = Callee {
        addr: 0x0016_c120,
        f: ee::s02::f_0016c120,
    };
    const TRIGGER_DTOR: Callee = Callee {
        addr: 0x0016_c5a8,
        f: ee::s02::f_0016c5a8,
    };
    const COUNT: u32 = 0x4;
    const VERSION: u32 = 0xc;
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S1, 0x60);
        let cursor_slot = r32(c, SP);
        c.sw(cursor_slot, c.g[A1][0]);
        frame.save(c, S0, 0x70);
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        frame.save(c, S5, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A0][0];
        RESET_HEADER.call(c, 0x0016_c254)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[SP][0];
        READ_VERSION.call(c, 0x0016_c260)?;
        let this = r32(c, S1);
        c.g[V0][0] = c.lw(this.wrapping_add(VERSION));
        c.g[V1][0] = 7;
        let version_ok = c.g[V0][0] == c.g[V1][0];
        c.g[V0][0] = 3;
        // 'status: the epilogue after `lq s0`; 'tail: the `cycles += 1; lq s0`
        // exit every non-early path takes.
        'status: {
            'tail: {
                if !version_ok {
                    c.cycles += 2;
                    c.sw(this.wrapping_add(VERSION), c.g[V1][0]);
                    break 'tail;
                }
                c.cycles += 7;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[SP][0];
                c.g[A2][0] = 7;
                READ_HEADER.call(c, 0x0016_c288)?;
                c.g[S0][0] = c.g[V0][0];
                let failed = c.g[S0][0] != 0;
                frame.restore(c, S0, 0x70);
                if failed {
                    break 'status;
                }
                // Trigger array: operator_new(count * 4).
                c.cycles += 11;
                c.g[A0][0] = c.lw(this.wrapping_add(COUNT));
                c.g[A1][0] = sx(0x46_0000u32.wrapping_sub(0x40c0));
                c.g[A2][0] = 0;
                c.g[A0][0] = sx(r32(c, A0) << 2);
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7e30, 0x0016_c2b0)?;
                c.g[S2][0] = 0;
                c.g[V1][0] = c.lw(this.wrapping_add(COUNT));
                let empty = (c.g[V1][0] as i64) <= 0;
                c.sw(this, c.g[V0][0]);
                'done: {
                    if empty {
                        break 'done;
                    }
                    c.cycles += 4;
                    c.g[S5][0] = 0x46_0000;
                    c.g[S4][0] = 4;
                    c.g[S3][0] = 3;
                    c.g[A0][0] = 0x24;
                    loop {
                        c.cycles += 18;
                        c.g[A1][0] = sx(r32(c, S5).wrapping_sub(0x40a8));
                        c.g[A2][0] = 0;
                        c.g[A3][0] = 0;
                        call_ptr(c, 0x0031_7d70, 0x0016_c2e0)?;
                        c.g[A2][0] = c.lw(this.wrapping_add(VERSION));
                        c.g[A0][0] = c.g[SP][0];
                        let slot = cursor_slot.wrapping_add(4);
                        c.sw(slot, c.g[V0][0]);
                        c.g[A1][0] = sx(slot);
                        let trig = r32(c, V0);
                        c.sw(trig.wrapping_add(0x1c), c.g[S4][0]);
                        c.sw(trig.wrapping_add(0x8), 0);
                        c.sw(trig.wrapping_add(0xc), 0);
                        c.sw(trig.wrapping_add(0x10), 0);
                        c.sw(trig, 0);
                        c.sw(trig.wrapping_add(0x4), c.g[S3][0]);
                        READ_TRIGGER.call(c, 0x0016_c30c)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A0][0] = c.lw(slot);
                        if c.g[S0][0] != 0 {
                            c.cycles += 2;
                            let none = c.g[A0][0] == 0;
                            c.g[V0][0] = c.g[S0][0];
                            if !none {
                                c.cycles += 4;
                                c.g[A1][0] = 3;
                                TRIGGER_DTOR.call(c, 0x0016_c328)?;
                                c.g[V0][0] = c.g[S0][0];
                            }
                            break 'tail;
                        }
                        c.cycles += 9;
                        c.g[A1][0] = c.lw(slot);
                        c.g[A2][0] = c.g[S2][0];
                        c.g[A0][0] = c.g[S1][0];
                        c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                        ADD_TRIGGER.call(c, 0x0016_c344)?;
                        c.g[V1][0] = c.lw(this.wrapping_add(COUNT));
                        c.g[V1][0] = ((c.g[S2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                        let more = c.g[V1][0] != 0;
                        c.g[A0][0] = 0x24;
                        if !more {
                            break 'done;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0016_c2d0);
                        }
                    }
                }
                c.cycles += 3;
                c.g[V1][0] = 7;
                c.g[V0][0] = 0;
                c.sw(this.wrapping_add(VERSION), c.g[V1][0]);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x70);
        }
        c.cycles += 8;
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// One owned part of a camera trigger in `CameraTriggerMan_FreeTriggers`:
/// if `trigger[off]` (+0xc or +0x10) is set, call its virtual destructor
/// (`vtable = *(part + 4)`, `fn = *(vtable + 0xc)`, `this = part +
/// s16(vtable + 8)`), delete the part and clear the field. Entered and left
/// with `v0 = triggers array`.
pub(crate) fn camera_trigger_free_part(
    c: &mut Cpu,
    s1: u32,
    off: u32,
    ra_virtual: u32,
    ra_delete: u32,
) -> Flow {
    c.cycles += 5;
    c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
    c.g[V1][0] = c.lw(r32(c, V0));
    c.g[A1][0] = c.lw(r32(c, V1).wrapping_add(off));
    if c.g[A1][0] == 0 {
        c.g[V0][0] = c.lw(s1);
        return Ok(());
    }
    c.cycles += 15;
    c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(4));
    c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(8));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0xc));
    let target = r32(c, V1);
    c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
    call_ptr(c, target, ra_virtual)?;
    c.g[V0][0] = c.lw(s1);
    c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
    c.g[V1][0] = c.lw(r32(c, V0));
    c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(off));
    call_ptr(c, 0x0031_7e50, ra_delete)?;
    c.g[V0][0] = c.lw(s1);
    c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
    c.g[V1][0] = c.lw(r32(c, V0));
    c.sw(r32(c, V1).wrapping_add(off), 0);
    c.g[V0][0] = c.lw(s1);
    Ok(())
}

/// `void CameraTriggerMan_FreeTriggers(self)` (0x16bf40): destroys every
/// trigger of the list (`self + 0` = array, `self + 4` = count): deletes its
/// buffer at +8, destroys and deletes its parts at +0xc and +0x10 (virtual
/// destructor), runs `CameraTrigger_dtor(t, 3)` on it, clears the slot, then
/// resets the header. Frame 0x40 (s0-s2).
pub fn camera_trigger_man_free_triggers(c: &mut Cpu) {
    const RESET_HEADER: Callee = Callee {
        addr: 0x0016_c390,
        f: ee::s02::f_0016c390,
    };
    const TRIGGER_DTOR: Callee = Callee {
        addr: 0x0016_c5a8,
        f: ee::s02::f_0016c5a8,
    };
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        let this = r32(c, S1);
        c.g[V0][0] = c.lw(this.wrapping_add(4));
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S2][0] = 0;
        if !empty {
            c.cycles += 1;
            c.g[V0][0] = c.lw(this);
            loop {
                c.cycles += 6;
                c.g[S0][0] = sx(r32(c, S2) << 2);
                c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
                c.g[V1][0] = c.lw(r32(c, V0));
                c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x8));
                if c.g[A0][0] == 0 {
                    c.g[V0][0] = c.lw(this);
                } else {
                    c.cycles += 7;
                    call_ptr(c, 0x0031_7e50, 0x0016_bf88)?;
                    c.g[V0][0] = c.lw(this);
                    c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
                    c.g[V1][0] = c.lw(r32(c, V0));
                    c.sw(r32(c, V1).wrapping_add(8), 0);
                    c.g[V0][0] = c.lw(this);
                }
                camera_trigger_free_part(c, this, 0xc, 0x0016_bfc4, 0x0016_bfd8)?;
                camera_trigger_free_part(c, this, 0x10, 0x0016_c014, 0x0016_c028)?;
                c.cycles += 4;
                c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
                c.g[A0][0] = c.lw(r32(c, V0));
                if c.g[A0][0] == 0 {
                    c.g[V1][0] = c.lw(this);
                } else {
                    c.cycles += 3;
                    c.g[A1][0] = 3;
                    TRIGGER_DTOR.call(c, 0x0016_c054)?;
                    c.g[V1][0] = c.lw(this);
                }
                c.cycles += 7;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                c.g[V1][0] = sx(r32(c, S0).wrapping_add(r32(c, V1)));
                c.sw(r32(c, V1), 0);
                c.g[V0][0] = c.lw(this.wrapping_add(4));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V0][0] = c.lw(this);
                if event_due(c) {
                    deliver_event_here(c, 0x0016_bf68);
                }
            }
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S1][0];
        RESET_HEADER.call(c, 0x0016_c07c)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::ee;
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::machine;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;

    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_4 as alloc_base;

    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear;

    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS;

    #[test]
    fn camera_trigger_man_add_trigger_matches() {
        check(
            ssx3_ee::s02::f_0016c120,
            camera_trigger_man_add_trigger,
            300,
            &|c, seed| {
                let mut r = Rng(0x16c1 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.g[A2][0] = match seed % 3 {
                    0 => 0xffff_ffff_ffff_ffff,
                    1 => (r.next_u32() % 30) as u64,
                    _ => (r.next_u32() % 120) as u64,
                };
                c.bus.write32(DATA, DATA + 0x400);
                c.bus.write32(
                    DATA + 4,
                    if seed % 5 == 0 {
                        200
                    } else {
                        r.next_u32() % 100
                    },
                );
            },
        );
    }

    /// The cache pointer is small, so `MEM_IsPoolPointer` sees it inside the
    /// (zeroed) 8-byte pool and `MEM_PoolFree` is two stores — no heap walk.
    pub(crate) fn small_pointer(seed: u32) -> u32 {
        0x100 + (seed % 0x700)
    }

    // -----------------------------------------------------------------------
    // CameraTriggerMan_Destroy.

    /// The freed pointer is small, so `MEM_IsPoolPointer` sees it inside the
    /// (zeroed) 8-byte pool — `MEM_PoolFree` is two stores, never a heap walk.
    #[test]
    fn camera_trigger_man_destroy_matches() {
        check(
            ssx3_ee::s02::f_0016c098__generated,
            camera_trigger_man_destroy,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x16c0_98, 0);
                let this = DATA;
                c.g[A0][0] = this as u64;
                // An empty trigger list (count at `+4` is 0) and, for most seeds,
                // a small "pool" pointer to free.
                c.bus.write32(this + 0x4, 0);
                c.bus.write32(
                    this + 0x0,
                    if seed % 3 == 0 {
                        0
                    } else {
                        small_pointer(seed)
                    },
                );
                c.bus.write32(this + 0x10, r.next_u32());
            },
        );
    }

    #[test]
    fn camera_trigger_man_read_header_matches() {
        check(
            ee::s02::f_0016c3d0__generated,
            camera_trigger_man_read_header,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x16c3_d0, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let reader = DATA + 0x800;
                let bytes = DATA + 0x900;
                clear(c, this, 0x40);
                clear(c, reader, 0x20);
                for i in 0..0x40 {
                    c.bus.write32(bytes + 4 * i, r.next_u32());
                }
                c.bus.write32(reader, bytes);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(reader);
            },
        );
    }

    pub(crate) fn wb(c: &mut Cpu, a: u32, v: u8) {
        c.bus.write8(a, v);
    }

    #[test]
    fn camera_trigger_man_load_file_matches() {
        check(
            ee::s02::f_0016c170__generated,
            camera_trigger_man_load_file,
            30,
            &|c, seed| {
                alloc_base(c, seed, 0x16c1, 4);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                for (i, b) in b"triggers.bin\0".iter().enumerate() {
                    wb(c, DATA + 0x800 + i as u32, *b);
                }
            },
        );
    }

    use crate::difftest::fakes;
    pub(crate) use crate::game::ai::ai::tests::oc432_fake;
    pub(crate) use crate::game::ai::ai::tests::OC432_CTRL;
    pub(crate) use crate::game::ai::ai::tests::OC432_MAGIC;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    oc432_fake!(
        fake_delete,
        PREV_DELETE,
        ee::s10::f_00317e50__generated,
        0x0031_7e50
    );

    oc432_fake!(
        fake_trigger_reset_header,
        PREV_TRIGGER_RESET_HEADER,
        ee::s02::f_0016c390__generated,
        0x0016_c390
    );

    oc432_fake!(
        fake_trigger_read_header,
        PREV_TRIGGER_READ_HEADER,
        ee::s02::f_0016c3d0__generated,
        0x0016_c3d0,
        |c: &mut Cpu| {
            c.g[V0][0] = c.bus.read32(OC432_CTRL + 8) as u64;
        }
    );

    oc432_fake!(
        fake_trigger_add,
        PREV_TRIGGER_ADD,
        ee::s02::f_0016c120__generated,
        0x0016_c120
    );

    oc432_fake!(
        fake_operator_new_named,
        PREV_OPERATOR_NEW_NAMED,
        ee::s10::f_00317e30__generated,
        0x0031_7e30,
        |c: &mut Cpu| {
            c.g[V0][0] = (DATA + 0x6_0000) as u64;
        }
    );

    oc432_fake!(
        fake_mem_alloc_named,
        PREV_MEM_ALLOC_NAMED,
        ee::s10::f_00317d70__generated,
        0x0031_7d70,
        |c: &mut Cpu| {
            if c.bus.read32(OC432_CTRL + 0x18) != 0 {
                c.g[V0][0] = 0;
            }
        }
    );

    oc432_fake!(
        fake_read_u32_2,
        PREV_READ_U32_2,
        ee::s09::f_002d2558__generated,
        0x002d_2558
    );

    oc432_fake!(
        fake_read_sphere_trigger,
        PREV_READ_SPHERE_TRIGGER,
        ee::s02::f_00172e18__generated,
        0x0017_2e18,
        |c: &mut Cpu| {
            c.g[V0][0] = c.bus.read32(OC432_CTRL + 0x10) as u64;
        }
    );

    oc432_fake!(
        fake_read_action,
        PREV_READ_ACTION,
        ee::s02::f_001712b0__generated,
        0x0017_12b0,
        |c: &mut Cpu| {
            c.g[V0][0] = c.bus.read32(OC432_CTRL + 0x14) as u64;
        }
    );

    pub(crate) fn camera_trigger_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            fake_delete,
            &PREV_DELETE,
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_0016C390),
            fake_trigger_reset_header,
            &PREV_TRIGGER_RESET_HEADER,
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_0016C3D0),
            fake_trigger_read_header,
            &PREV_TRIGGER_READ_HEADER,
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_0016C120),
            fake_trigger_add,
            &PREV_TRIGGER_ADD,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            fake_operator_new_named,
            &PREV_OPERATOR_NEW_NAMED,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            fake_mem_alloc_named,
            &PREV_MEM_ALLOC_NAMED,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D2558),
            fake_read_u32_2,
            &PREV_READ_U32_2,
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_00172E18),
            fake_read_sphere_trigger,
            &PREV_READ_SPHERE_TRIGGER,
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_001712B0),
            fake_read_action,
            &PREV_READ_ACTION,
        );
    }

    /// `CameraTriggerMan_Parse`: the stream readers, allocator, header
    /// reset/read and AddTrigger are fakes; ReadTrigger and ReadVersion run
    /// real on top of them (version 7 or not, header status 0 or not, 0..3
    /// triggers, allocation that fails, sphere/action reads that fail).
    #[test]
    fn camera_trigger_man_parse_matches() {
        camera_trigger_install_fakes();
        fakes::covered(
            ee::s02::f_0016c228__generated,
            camera_trigger_man_parse,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x16c228, 14);
                let this = DATA + 0x5_0000;
                c.bus.write32(OC432_CTRL, OC432_MAGIC);
                c.bus
                    .write32(OC432_CTRL + 8, if r.next_u32() % 4 == 0 { 5 } else { 0 });
                c.bus
                    .write32(OC432_CTRL + 0x10, (r.next_u32() % 5 != 0) as u32);
                c.bus
                    .write32(OC432_CTRL + 0x14, (r.next_u32() % 5 != 0) as u32);
                c.bus
                    .write32(OC432_CTRL + 0x18, (r.next_u32() % 6 == 0) as u32);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x5_8000) as u64;
                c.bus.write32(this + 0x4, r.next_u32() % 4);
                c.bus
                    .write32(this + 0xc, if r.next_u32() % 5 == 0 { 3 } else { 7 });
            },
        );
    }

    /// `CameraTriggerMan_FreeTriggers`: 0..3 triggers whose buffer (+8) and
    /// parts (+0xc, +0x10) are set or not (parts have a vtable whose
    /// destructor is a fake), some trigger slots null.
    #[test]
    fn camera_trigger_man_free_triggers_matches() {
        camera_trigger_install_fakes();
        fakes::covered(
            ee::s02::f_0016bf40__generated,
            camera_trigger_man_free_triggers,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x16bf40, 14);
                c.bus.write32(OC432_CTRL, OC432_MAGIC);
                let this = DATA + 0x5_0000;
                let arr = this + 0x100;
                let vt = this + 0x3000;
                let n = r.next_u32() % 4;
                c.g[A0][0] = this as u64;
                c.bus.write32(this, arr);
                c.bus.write32(this + 4, n);
                c.bus.write16(vt + 8, (r.next_u32() % 64) as u16);
                c.bus.write32(vt + 0xc, fakes::FAKE);
                for i in 0..4u32 {
                    let trig = if r.next_u32() % 6 == 0 {
                        0
                    } else {
                        this + 0x200 + i * 0x40
                    };
                    c.bus.write32(arr + 4 * i, trig);
                    for (k, off) in [0x8u32, 0xc, 0x10].into_iter().enumerate() {
                        let v = if r.next_u32() % 3 == 0 {
                            0
                        } else {
                            let part = this + 0x1000 + (i * 3 + k as u32) * 0x40;
                            c.bus.write32(part + 4, vt);
                            part
                        };
                        c.bus.write32(this + 0x200 + i * 0x40 + off, v);
                    }
                }
            },
        );
    }
}
