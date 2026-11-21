//! Staging module oc1127 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1127`.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003e_4000, ea_file_cd_remount, "EAFile_CdRemount"),
    (0x003e_4508, ea_file_cd_lookup, "EAFile_CdLookup"),
    (0x003e_4648, ea_file_cd_read, "EAFile_CdRead"),
    (0x003e_48d8, ea_file_cd_get_entry, "EAFile_CdGetEntry"),
    (0x003e_4968, ea_file_cd_find_entry, "EAFile_CdFindEntry"),
    (0x003e_4af0, ea_timer_init, "EATimer_Init"),
    (0x003e_4d68, ea_timer_shutdown, "EATimer_Shutdown"),
    (0x003e_4ea8, ea_timer_get_delta, "EATimer_GetDelta"),
    (
        0x003e_4f08,
        ea_file_timer_set_deadline,
        "EATimer_SetDeadline",
    ),
    (
        0x003e_4f40,
        ea_file_timer_wait_deadline,
        "EATimer_WaitDeadline",
    ),
    (
        0x003e_4f80,
        ea_file_timer_deadline_passed,
        "EATimer_DeadlinePassed",
    ),
    (0x003e_4fb0, ea_file_timer_delay, "EATimer_Delay"),
    (
        0x003e_5008,
        ea_file_timer_get_tick_rate,
        "EATimer_GetTickRate",
    ),
    (0x003e_5018, ea_thread_init_main, "EAThread_InitMain"),
    (0x003e_5068, ea_thread_shutdown, "EAThread_Shutdown"),
    (0x003e_50c8, ea_thread_create, "EAThread_Create"),
    (0x003e_51a0, ea_thread_destroy, "EAThread_Destroy"),
    (
        0x003e_5268,
        ea_thread_destroy_thunk,
        "EAThread_DestroyThunk",
    ),
];

mod callee {
    use super::ee;
    use crate::call::Callee;

    pub(crate) const EA_FILE_CD_MOUNT_DISC: Callee = Callee {
        addr: 0x003e_3d78,
        f: ee::s14::EAFile_CdMountDisc,
    };
    pub(crate) const EA_FILE_HASH_NAME: Callee = Callee {
        addr: 0x003e_36f8,
        f: ee::s14::EAFile_HashName,
    };
    pub(crate) const EA_FILE_CD_LOOKUP: Callee = Callee {
        addr: 0x003e_4508,
        f: ee::s14::EAFile_CdLookup,
    };
    pub(crate) const EA_FILE_COMPLETE_REQUEST: Callee = Callee {
        addr: 0x003d_e420,
        f: ee::s14::EAFile_CompleteRequest,
    };
    pub(crate) const EA_MEM_COPY: Callee = Callee {
        addr: 0x003e_6574,
        f: ee::s14::EA_MemCopy,
    };
    pub(crate) const EA_MEM_ZERO: Callee = Callee {
        addr: 0x003e_6220,
        f: ee::s14::EA_MemZero,
    };
    pub(crate) const EA_TIMER_GET_TICKS: Callee = Callee {
        addr: 0x003e_4e98,
        f: ee::s14::EATimer_GetTicks,
    };
    pub(crate) const EA_TIMER_DEADLINE_PASSED: Callee = Callee {
        addr: 0x003e_4f80,
        f: ee::s14::EATimer_DeadlinePassed,
    };
    pub(crate) const EA_TIMER_SET_VIDEO_MODE: Callee = Callee {
        addr: 0x003e_6690,
        f: ee::s14::EATimer_SetVideoMode,
    };
    pub(crate) const EA_TIMER_SERVICE_CALLBACKS: Callee = Callee {
        addr: 0x003e_5928,
        f: ee::s14::EATimer_ServiceCallbacks,
    };
    pub(crate) const EA_THREAD_YIELD: Callee = Callee {
        addr: 0x003e_5398,
        f: ee::s14::EAThread_Yield,
    };
    pub(crate) const EA_THREAD_DESTROY: Callee = Callee {
        addr: 0x003e_51a0,
        f: ee::s14::EAThread_Destroy,
    };
    pub(crate) const EA_MUTEX_INIT: Callee = Callee {
        addr: 0x003e_5698,
        f: ee::s14::EAMutex_Init,
    };
    pub(crate) const EA_MUTEX_DESTROY: Callee = Callee {
        addr: 0x003e_56e0,
        f: ee::s14::EAMutex_Destroy,
    };
    pub(crate) const EA_ADD_SHUTDOWN_HANDLER: Callee = Callee {
        addr: 0x003e_4a30,
        f: ee::s14::EA_AddShutdownHandler,
    };
}

/// `u32 EAFile_CdRemount()` (0x3e4000): remount the disc if the
/// present flag says it went away.
pub fn ea_file_cd_remount(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x520000;
        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0x63c0));
        c.g[V1][0] = 0x6;
        c.g[V0][0] = c.g[A0][0] & 0x6;
        let gone = c.g[V0][0] != c.g[V1][0];
        frame.save_ra(c, 0);
        'b_3e402c: {
            if gone {
                break 'b_3e402c;
            }
            c.cycles += 4;
            c.g[V0][0] = 0xfffffffffffffff9;
            c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
            c.g[RA][0] = sx(0x003e_402c);
            c.sw((c.g[A1][0] as u32).wrapping_sub(0x63c0), c.g[V0][0]);
            callee::EA_FILE_CD_MOUNT_DISC.call(c, 0x003e_402c)?;
            break 'b_3e402c;
        }
        c.cycles += 4;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.g[V0][0] = 0;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}

/// `u32 EAFile_CdGetEntry(index, name_out, lba_out, size_out)`
/// (0x3e48d8): table entry fields for a 1-based index.
pub fn ea_file_cd_get_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        c.g[V1][0] = 0x450000;
        c.g[V0][0] = 0x520000;
        c.g[T0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc5c));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x6388));
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(0xc));
        frame.save(c, S0, 0x30);
        c.mult(0, 4, 4, 2);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A3][0];
        let no_name = c.g[A1][0] == 0;
        c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        'b_3e4928: {
            if no_name {
                break 'b_3e4928;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[A1][0];
            c.g[A2][0] = c.g[T0][0];
            c.g[RA][0] = sx(0x003e_4928);
            c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
            let hook = 0x0041_6ce0;
            c.pc = hook;
            call_ptr(c, hook, 0x003e_4928)?;
            break 'b_3e4928;
        }
        'b_3e4938: {
            c.cycles += 2;
            if c.g[S1][0] == 0 {
                break 'b_3e4938;
            }
            c.cycles += 2;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
            break 'b_3e4938;
        }
        'b_3e494c: {
            c.cycles += 2;
            if c.g[S2][0] == 0 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                break 'b_3e494c;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            c.sw(c.g[S2][0] as u32, c.g[V0][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
            break 'b_3e494c;
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

/// `u32 EAFile_CdFindEntry(index)` (0x3e4968): entry address through
/// the index lookup, or 0 when absent.
pub fn ea_file_cd_find_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x003e_497c);
        c.g[A2][0] = c.g[SP][0];
        callee::EA_FILE_CD_LOOKUP.call(c, 0x003e_497c)?;
        let found = c.g[V0][0] != 0;
        c.g[A0][0] = 0x450000;
        'b_3e49ac: {
            'b_3e498c: {
                if found {
                    break 'b_3e498c;
                }
                c.cycles += 2;
                c.g[V0][0] = 0;
                break 'b_3e49ac;
            }
            c.cycles += 8;
            c.g[V0][0] = c.lw(c.g[SP][0] as u32);
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc5c));
            c.g[A1][0] = 0x520000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0x6388));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc));
            c.mult(0, 2, 2, 3);
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            break 'b_3e49ac;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void EATimer_Shutdown()` (0x3e4d68): stop the timer interrupt.
pub fn ea_timer_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = 0x450000;
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xddc));
        let live = c.g[V0][0] != 0;
        c.g[V0][0] = 0x10000000;
        'b_3e4da8: {
            if !live {
                break 'b_3e4da8;
            }
            c.cycles += 9;
            c.g[A0][0] = 0xa;
            c.g[V0][0] = c.g[V0][0] | 0x810;
            c.sw(c.g[V0][0] as u32, 0);
            c.g[RA][0] = sx(0x003e_4d98);
            let hook = 0x0042_4880;
            c.pc = hook;
            call_ptr(c, hook, 0x003e_4d98)?;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xddc));
            c.g[RA][0] = sx(0x003e_4da4);
            c.g[A0][0] = 0xa;
            let hook = 0x0042_3aa0;
            c.pc = hook;
            call_ptr(c, hook, 0x003e_4da4)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0xddc), 0);
            break 'b_3e4da8;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EATimer_GetDelta()` (0x3e4ea8): ticks since the last call.
pub fn ea_timer_get_delta(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = 0x450000;
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xdec));
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_4ec8);
        callee::EA_TIMER_GET_TICKS.call(c, 0x003e_4ec8)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[S0][0] as u32));
        c.sw((c.g[S1][0] as u32).wrapping_add(0xdec), c.g[V1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `u32 EAFile_CdRead(lba, count, ...)` (0x3e4648): queue a CD read.
/// The `0x423dc0` gate is a syscall (faked in tests); the rest is
/// same-module calls.
pub fn ea_file_cd_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        c.g[T0][0] = 0x450000;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xc5c));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
        frame.save(c, S0, 0x40);
        c.g[V0][0] = 0x520000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc));
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.mult(0, 4, 4, 3);
        frame.save(c, S3, 0x10);
        c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x63c0));
        frame.save_ra(c, 0);
        c.g[T1][0] = c.g[A2][0];
        c.g[T0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x7ff));
        c.g[S1][0] = c.g[V0][0];
        c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x38));
        c.g[T3][0] = 0xffffffffffffffff;
        c.g[A2][0] = ((c.g[T3][0] as i64) < (c.g[T1][0] as i64)) as u64;
        c.g[V0][0] = c.g[T0][0];
        c.g[T4][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        if c.g[A2][0] != 0 {
            c.g[V0][0] = c.g[T1][0];
        }
        c.g[V1][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 11) as u32);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 11);
        c.g[S0][0] = sx((c.g[T2][0] as u32).wrapping_add(0x20));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[T1][0] as u32));
        c.g[A2][0] = sx((c.g[T1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        c.g[A0][0] = ((c.g[V1][0] as i64) < (c.g[A3][0] as i64)) as u64;
        c.g[S2][0] = c.g[V1][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
        if c.g[A0][0] == 0 {
            c.g[S2][0] = c.g[A3][0];
        }
        let t = c.g[A2][0] == 0;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[A1][0]);
        'b_3e4778: {
            'b_3e4708: {
                if t {
                    break 'b_3e4708;
                }
                'b_3e4700: {
                    'b_3e46fc: {
                        c.cycles += 8;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x63c0));
                        c.g[A0][0] = 0x1;
                        c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[S2][0] as u32));
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                        c.g[V0][0] = c.g[V0][0] | 0x1;
                        c.g[V1][0] = ((c.g[V1][0] as i64) < 2049) as u64;
                        let t = c.g[V1][0] != 0;
                        c.sw((c.g[S1][0] as u32).wrapping_sub(0x63c0), c.g[V0][0]);
                        if t {
                            break 'b_3e46fc;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = 0x800;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A2][0] as u32));
                        break 'b_3e4700;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S2][0];
                    break 'b_3e4700;
                }
                c.cycles += 2;
                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                let t = true;
                if t {
                    break 'b_3e4778;
                } else {
                    break 'b_3e4708;
                }
            }
            'b_3e472c: {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S2][0] as i64) < 2048) as u64;
                let small = c.g[V0][0] == 0;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x63c0));
                if small {
                    break 'b_3e472c;
                }
                c.cycles += 6;
                c.g[V0][0] = 0x1;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                c.g[V1][0] = c.g[V1][0] | 0x1;
                c.sw((c.g[T2][0] as u32).wrapping_add(0x20), c.g[S2][0]);
                c.sw((c.g[S1][0] as u32).wrapping_sub(0x63c0), c.g[V1][0]);
                let t = true;
                if t {
                    break 'b_3e4778;
                } else {
                    break 'b_3e472c;
                }
            }
            'b_3e4754: {
                c.cycles += 3;
                c.g[V0][0] = c.g[A1][0] & 0x3f;
                let aligned = c.g[V0][0] == 0;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x63c0));
                if aligned {
                    break 'b_3e4754;
                }
                c.cycles += 7;
                c.g[V1][0] = 0x1;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.g[A0][0] = 0x800;
                c.g[V0][0] = c.g[V0][0] | 0x1;
                c.sw((c.g[T2][0] as u32).wrapping_add(0x20), c.g[A0][0]);
                c.sw((c.g[S1][0] as u32).wrapping_sub(0x63c0), c.g[V0][0]);
                let t = true;
                if t {
                    break 'b_3e4778;
                } else {
                    break 'b_3e4754;
                }
            }
            c.cycles += 9;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x63c0));
            c.g[V0][0] = 0xfffffffffffffffe;
            c.g[V1][0] = 0xfffffffffffff800;
            c.g[A0][0] = c.g[A0][0] & c.g[V0][0];
            c.g[V1][0] = c.g[S2][0] & c.g[V1][0];
            c.sw((c.g[S1][0] as u32).wrapping_sub(0x63c0), c.g[A0][0]);
            c.g[A1][0] = sx(((c.g[V1][0] as u32) as i32 >> 11) as u32);
            c.sw((c.g[T2][0] as u32).wrapping_add(0x20), c.g[V1][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
            break 'b_3e4778;
        }
        'b_3e48bc: {
            'b_3e48a0: {
                'b_3e4898: {
                    c.cycles += 13;
                    c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                    c.g[S3][0] = 0xffffffffffffffff;
                    c.g[V1][0] = ((c.g[S3][0] as i64) < (c.g[T1][0] as i64)) as u64;
                    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x63c0));
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(c.g[A2][0] as u32));
                    if c.g[V1][0] != 0 {
                        c.g[T0][0] = c.g[T1][0];
                    }
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.g[V1][0] = sx(((c.g[T0][0] as u32) as i32 >> 11) as u32);
                    c.g[V0][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    if c.g[V0][0] != c.g[A0][0] {
                        break 'b_3e4898;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A2][0] as i64) < 2049) as u64;
                    if c.g[V0][0] == 0 {
                        break 'b_3e48a0;
                    }
                    'b_3e4878: {
                        c.cycles += 8;
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x3c));
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                        c.g[RA][0] = sx(0x003e_47cc);
                        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        callee::EA_MEM_COPY.call(c, 0x003e_47cc)?;
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        let small = (c.g[A2][0] as i64) <= 0;
                        c.g[A1][0] = ((c.g[A2][0] as i64) < 2048) as u64;
                        if small {
                            break 'b_3e4878;
                        }
                        'b_3e4864: {
                            'b_3e4818: {
                                c.cycles += 9;
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0);
                                c.g[A0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                                if c.g[A1][0] == 0 {
                                    break 'b_3e4818;
                                }
                                c.cycles += 7;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x63c0));
                                c.g[V0][0] = 0x1;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                                c.g[V1][0] = c.g[V1][0] | 0x1;
                                c.sw(c.g[S0][0] as u32, c.g[A2][0]);
                                c.sw((c.g[S1][0] as u32).wrapping_sub(0x63c0), c.g[V1][0]);
                                break 'b_3e4864;
                            }
                            'b_3e4840: {
                                c.cycles += 3;
                                c.g[V0][0] = c.g[A0][0] & 0x3f;
                                if c.g[V0][0] == 0 {
                                    break 'b_3e4840;
                                }
                                c.cycles += 7;
                                c.g[V1][0] = 0x1;
                                c.g[A0][0] = 0x800;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                                c.g[V0][0] = c.g[V0][0] | 0x1;
                                c.sw(c.g[S0][0] as u32, c.g[A0][0]);
                                break 'b_3e4864;
                            }
                            c.cycles += 9;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x63c0));
                            c.g[V0][0] = 0xfffffffffffffffe;
                            c.g[A0][0] = 0xfffffffffffff800;
                            c.g[A0][0] = c.g[A2][0] & c.g[A0][0];
                            c.g[V1][0] = c.g[V0][0] & c.g[V1][0];
                            c.sw((c.g[S1][0] as u32).wrapping_sub(0x63c0), c.g[V1][0]);
                            c.g[A1][0] = sx(((c.g[A0][0] as u32) as i32 >> 11) as u32);
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                            c.sw(c.g[S0][0] as u32, c.g[A0][0]);
                            break 'b_3e4864;
                        }
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                        let t = true;
                        if t {
                            break 'b_3e4898;
                        } else {
                            break 'b_3e4878;
                        }
                    }
                    c.cycles += 8;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x63c0));
                    c.g[V1][0] = 0xffffffffffffffff;
                    c.g[A0][0] = 0x1;
                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                    c.g[RA][0] = sx(0x003e_4890);
                    c.sw((c.g[S1][0] as u32).wrapping_sub(0x63c0), c.g[V0][0]);
                    callee::EA_FILE_COMPLETE_REQUEST.call(c, 0x003e_4890)?;
                    c.g[V0][0] = c.g[S2][0];
                    let t = true;
                    if t {
                        break 'b_3e48bc;
                    } else {
                        break 'b_3e4898;
                    }
                }
                c.cycles += 2;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x63c0));
                c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x63c0));
                break 'b_3e48a0;
            }
            c.cycles += 7;
            c.g[V1][0] = 0x3;
            c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
            c.g[V0][0] = c.g[V0][0] | 0xa;
            c.sw((c.g[A1][0] as u32).wrapping_add(0x10), c.g[V1][0]);
            c.g[RA][0] = sx(0x003e_48b8);
            c.sw((c.g[S1][0] as u32).wrapping_sub(0x63c0), c.g[V0][0]);
            let hook = 0x0042_3dc0;
            c.pc = hook;
            call_ptr(c, hook, 0x003e_48b8)?;
            c.g[V0][0] = c.g[S2][0];
            break 'b_3e48bc;
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

/// `u32 EAFile_CdLookup(index, out0, out1, out2)` (0x3e4508): entry
/// address from a 1-based index into the sorted table.
pub fn ea_file_cd_lookup(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_68: u32 = 0x003e_4568;
        c.cycles += 19;
        let frame = Frame::open(c, 0x160);
        c.g[A3][0] = c.g[A0][0];
        frame.save(c, S2, 0x130);
        c.g[V0][0] = 0x2f;
        frame.save(c, S3, 0x120);
        c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S0, 0x150);
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1));
        frame.save(c, S1, 0x140);
        frame.save_ra(c, 0x110);
        c.sb(c.g[SP][0] as u32, c.g[V0][0]);
        c.g[V1][0] = c.lb(c.g[A3][0] as u32);
        c.g[A3][0] = sx(c.g[A3][0] as u32).wrapping_add(0x1);
        c.g[V1][0] = c.g[V1][0] ^ 0x2f;
        if c.g[V1][0] != 0 {
            c.g[A3][0] = c.g[A0][0];
        }
        c.g[V0][0] = c.lb(c.g[A3][0] as u32);
        c.g[V1][0] = c.lbu(c.g[A3][0] as u32);
        'b_3e45a8: {
            if c.g[V0][0] == 0 {
                break 'b_3e45a8;
            }
            c.cycles += 5;
            c.g[V0][0] = 0x4a0000;
            c.g[T2][0] = 0x5c;
            c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x637f));
            c.g[T0][0] = 0x2f;
            'l_3e4568: loop {
                'b_3e4594: {
                    'b_3e4580: {
                        c.cycles += 4;
                        c.g[V0][0] = sx((c.g[V1][0] as u32) << 24);
                        c.g[A1][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                        let not_bs = c.g[A1][0] != c.g[T2][0];
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T1][0] as u32));
                        if not_bs {
                            break 'b_3e4580;
                        }
                        c.cycles += 2;
                        c.sb(c.g[A2][0] as u32, c.g[T0][0]);
                        break 'b_3e4594;
                    }
                    c.cycles += 5;
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x20));
                    c.g[V0][0] = c.lbu(c.g[A0][0] as u32);
                    c.g[V0][0] = c.g[V0][0] & 0x2;
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = c.g[A1][0];
                    }
                    c.sb(c.g[A2][0] as u32, c.g[V1][0]);
                    break 'b_3e4594;
                }
                c.cycles += 5;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = c.lb(c.g[A3][0] as u32);
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = c.lbu(c.g[A3][0] as u32);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, HEAD_68);
                    }
                    continue 'l_3e4568;
                }
                break 'b_3e45a8;
            }
        }
        'b_3e462c: {
            'b_3e4624: {
                c.cycles += 20;
                c.sb(c.g[A2][0] as u32, 0);
                c.g[V0][0] = 0x520000;
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x63c0));
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                c.g[V0][0] = 0x450000;
                c.g[RA][0] = sx(0x003e_45c8);
                c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc58));
                callee::EA_FILE_HASH_NAME.call(c, 0x003e_45c8)?;
                c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[T0][0] = 0x3e0000;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
                c.g[A0][0] = c.g[S2][0];
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x3968));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0xc));
                c.g[RA][0] = sx(0x003e_45ec);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x100), c.g[V0][0]);
                let hook = 0x0041_87e8;
                c.pc = hook;
                call_ptr(c, hook, 0x003e_45ec)?;
                c.g[A1][0] = c.g[V0][0];
                let null = c.g[A1][0] == 0;
                c.g[V0][0] = 0x1;
                if null {
                    break 'b_3e4624;
                }
                'b_3e4614: {
                    c.cycles += 7;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
                    c.divu(0, 3, 4);
                    // Gen tests A0==0 here, but A0 was just bumped by 0xc
                    // off a mapped word: both arms break out. Kept faithful.
                    if c.g[A0][0] == 0 {
                        break 'b_3e4614;
                    } else {
                        break 'b_3e4614;
                    }
                }
                c.cycles += 4;
                c.g[V1][0] = c.lo[0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                c.sw(c.g[S3][0] as u32, c.g[V1][0]);
                break 'b_3e462c;
            }
            c.cycles += 2;
            c.sw(c.g[S3][0] as u32, 0);
            c.g[V0][0] = 0;
            break 'b_3e462c;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x150);
        frame.restore(c, S1, 0x140);
        frame.restore(c, S2, 0x130);
        frame.restore(c, S3, 0x120);
        frame.restore_ra(c, 0x110);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EATimer_SetDeadline(dt)` (0x3e4f08): deadline = now + dt.
pub fn ea_file_timer_set_deadline(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_4f24);
        c.g[S1][0] = 0x450000;
        callee::EA_TIMER_GET_TICKS.call(c, 0x003e_4f24)?;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.sw((c.g[S1][0] as u32).wrapping_add(0xde8), c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `void EATimer_WaitDeadline()` (0x3e4f40): pump callbacks and yield
/// until the deadline passes.
pub fn ea_file_timer_wait_deadline(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_50: u32 = 0x003e_4f50;
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        'wait: loop {
            c.cycles += 4;
            c.g[RA][0] = sx(0x003e_4f68);
            callee::EA_TIMER_DEADLINE_PASSED.call(c, 0x003e_4f68)?;
            let done = c.g[V0][0] != 0;
            c.g[A0][0] = 0;
            if done {
                break 'wait;
            }
            if event_due(c) {
                deliver_event_here(c, HEAD_50);
            }
            c.cycles += 4;
            c.g[RA][0] = sx(0x003e_4f58);
            callee::EA_TIMER_SERVICE_CALLBACKS.call(c, 0x003e_4f58)?;
            c.g[RA][0] = sx(0x003e_4f60);
            c.g[A0][0] = 0;
            callee::EA_THREAD_YIELD.call(c, 0x003e_4f60)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EATimer_DeadlinePassed()` (0x3e4f80): nonzero once now is past
/// the deadline.
pub fn ea_file_timer_deadline_passed(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_4f90);
        callee::EA_TIMER_GET_TICKS.call(c, 0x003e_4f90)?;
        c.g[A0][0] = 0x450000;
        c.g[A1][0] = 0xffffffffffffffff;
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xde8));
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}

/// `void EATimer_Delay(dt)` (0x3e4fb0): pump callbacks and yield for
/// `dt` ticks.
pub fn ea_file_timer_delay(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_D0: u32 = 0x003e_4fd0;
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_4fc4);
        c.g[S0][0] = c.g[A0][0];
        callee::EA_TIMER_GET_TICKS.call(c, 0x003e_4fc4)?;
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        'delay: loop {
            c.cycles += 5;
            c.g[RA][0] = sx(0x003e_4fe8);
            callee::EA_TIMER_GET_TICKS.call(c, 0x003e_4fe8)?;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S0][0] as u32));
            let wait = (c.g[V0][0] as i64) < 0;
            c.g[A0][0] = 0;
            if !wait {
                break 'delay;
            }
            if event_due(c) {
                deliver_event_here(c, HEAD_D0);
            }
            c.cycles += 4;
            c.g[RA][0] = sx(0x003e_4fd8);
            callee::EA_TIMER_SERVICE_CALLBACKS.call(c, 0x003e_4fd8)?;
            c.g[RA][0] = sx(0x003e_4fe0);
            c.g[A0][0] = 0;
            callee::EA_THREAD_YIELD.call(c, 0x003e_4fe0)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EATimer_GetTickRate()` (0x3e5008): cached ticks-per-second.
pub fn ea_file_timer_get_tick_rate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        c.g[V1][0] = 0x450000;
        let t = c.g[RA][0] as u32;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xdc8));
        c.pc = t;
        Ok(())
    })
}

/// `void EAThread_InitMain()` (0x3e5018): tag this thread in the main
/// thread slot.
pub fn ea_thread_init_main(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_5028);
        let hook = 0x0042_3c90;
        c.pc = hook;
        call_ptr(c, hook, 0x003e_5028)?;
        c.g[V1][0] = 0x50530000;
        c.g[A0][0] = 0x450000;
        c.g[V1][0] = c.g[V1][0] | 0x3254;
        c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0xdf8));
        c.sw((c.g[A0][0] as u32).wrapping_add(0xdf8), c.g[V1][0]);
        c.g[A1][0] = 0x64;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x003e_504c);
        c.sw((c.g[A2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        let hook = 0x0042_3c30;
        c.pc = hook;
        call_ptr(c, hook, 0x003e_504c)?;
        c.g[A0][0] = 0x520000;
        c.g[RA][0] = sx(0x003e_5058);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1278));
        callee::EA_MUTEX_INIT.call(c, 0x003e_5058)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void EAThread_Shutdown()` (0x3e5068): destroy the main-thread
/// mutex and clear its slot.
pub fn ea_thread_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x520000;
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_507c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1278));
        callee::EA_MUTEX_DESTROY.call(c, 0x003e_507c)?;
        c.g[A0][0] = 0x450000;
        c.g[A1][0] = 0x8;
        c.g[RA][0] = sx(0x003e_508c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xdf8));
        callee::EA_MEM_ZERO.call(c, 0x003e_508c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAThread_Create(kind, ...)` (0x3e50c8): create and start a
/// thread of the given kind (`T1` names the priority slot).
pub fn ea_thread_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x60);
        c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x61));
        frame.save(c, S0, 0x50);
        c.g[V0][0] = (c.g[V1][0] < 0x7) as u64;
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        let tabled = c.g[V0][0] != 0;
        c.g[S1][0] = c.g[A2][0];
        'b_3e513c: {
            if !tabled {
                c.cycles += 1;
                c.g[V1][0] = 0x64;
                break 'b_3e513c;
            }
            c.cycles += 7;
            c.g[V0][0] = 0x490000;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5e00));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
            // Jump table on the loaded handler: each arm names the same
            // priority slot gen dispatches to. The table is fixed game
            // data, so the fallback never fires on fixture.
            c.g[V1][0] = match c.g[A0][0] as u32 {
                0x003e_5108 => 0x61,
                0x003e_5110 => 0x62,
                0x003e_5118 => 0x63,
                0x003e_5120 => 0x65,
                0x003e_5128 => 0x66,
                0x003e_5130 => 0x67,
                0x003e_5138 => 0x67,
                t => {
                    c.jump_out(t, 0x003e_50c8, 0x003e_519c);
                    return Ok(());
                }
            };
            break 'b_3e513c;
        }
        'b_3e5188: {
            c.cycles += 13;
            c.g[V0][0] = 0x4a0000;
            c.g[A0][0] = c.g[SP][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x30f0));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A1][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A3][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[T0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V1][0]);
            c.g[RA][0] = sx(0x003e_5160);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
            let hook = 0x0042_3ba0;
            c.pc = hook;
            call_ptr(c, hook, 0x003e_5160)?;
            c.g[A0][0] = c.g[V0][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[A0][0]);
            let failed = c.g[A0][0] == 0;
            c.g[V0][0] = 0;
            if failed {
                break 'b_3e5188;
            }
            c.cycles += 6;
            c.g[V0][0] = 0x50530000;
            c.g[A1][0] = c.g[S1][0];
            c.g[V0][0] = c.g[V0][0] | 0x3254;
            c.g[RA][0] = sx(0x003e_5184);
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            let hook = 0x0042_3bc0;
            c.pc = hook;
            call_ptr(c, hook, 0x003e_5184)?;
            c.g[V0][0] = 0x1;
            break 'b_3e5188;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void EAThread_Destroy(thread)` (0x3e51a0): stop the thread unless
/// it is this one, then release it.
pub fn ea_thread_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0xffffffffffff0000;
        frame.save(c, S0, 0x50);
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        frame.save(c, S1, 0x40);
        let is_self = c.g[A0][0] == c.g[V0][0];
        frame.save_ra(c, 0x30);
        'b_3e51d0: {
            'b_3e51cc: {
                if !is_self {
                    break 'b_3e51cc;
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x003e_51c4);
                let hook = 0x0042_3c90;
                c.pc = hook;
                call_ptr(c, hook, 0x003e_51c4)?;
                c.g[S0][0] = c.g[V0][0];
                break 'b_3e51d0;
            }
            c.cycles += 1;
            c.g[S0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
            break 'b_3e51d0;
        }
        'b_3e5204: {
            'b_3e51f4: {
                c.cycles += 4;
                c.g[V1][0] = 0x450000;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xde4));
                let no_main = c.g[V0][0] == 0;
                c.g[S1][0] = c.g[V1][0];
                if no_main {
                    break 'b_3e51f4;
                }
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x003e_51ec);
                c.g[A1][0] = c.g[SP][0];
                let hook = 0x0042_3cb0;
                c.pc = hook;
                call_ptr(c, hook, 0x003e_51ec)?;
                c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                break 'b_3e5204;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x003e_5200);
            c.g[A1][0] = c.g[SP][0];
            let hook = 0x0042_3ca0;
            c.pc = hook;
            call_ptr(c, hook, 0x003e_5200)?;
            c.g[V1][0] = c.lw(c.g[SP][0] as u32);
            break 'b_3e5204;
        }
        'b_3e5258: {
            'b_3e5240: {
                c.cycles += 3;
                c.g[V0][0] = c.g[V1][0] & 0xf;
                let plain = c.g[V0][0] == 0;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xde4));
                if plain {
                    break 'b_3e5240;
                }
                'b_3e5230: {
                    'b_3e5228: {
                        c.cycles += 2;
                        let dead = c.g[V0][0] == 0;
                        if dead {
                            break 'b_3e5228;
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x003e_5220);
                        c.g[A0][0] = c.g[S0][0];
                        let hook = 0x0042_3c00;
                        c.pc = hook;
                        call_ptr(c, hook, 0x003e_5220)?;
                        break 'b_3e5230;
                    }
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x003e_5230);
                    c.g[A0][0] = c.g[S0][0];
                    let hook = 0x0042_3bf0;
                    c.pc = hook;
                    call_ptr(c, hook, 0x003e_5230)?;
                    break 'b_3e5230;
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x003e_5238);
                c.g[A0][0] = c.g[S0][0];
                let hook = 0x0042_3bb0;
                c.pc = hook;
                call_ptr(c, hook, 0x003e_5238)?;
                frame.restore(c, S0, 0x50);
                break 'b_3e5258;
            }
            c.cycles += 3;
            c.g[V0][0] = c.g[V1][0] & 0x10;
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x50);
                break 'b_3e5258;
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x003e_5254);
            c.g[A0][0] = c.g[S0][0];
            let hook = 0x0042_3bb0;
            c.pc = hook;
            call_ptr(c, hook, 0x003e_5254)?;
            frame.restore(c, S0, 0x50);
            break 'b_3e5258;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void EAThread_DestroyThunk(thread)` (0x3e5268): destroy the
/// exiting thread.
pub fn ea_thread_destroy_thunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_5278);
        callee::EA_THREAD_DESTROY.call(c, 0x003e_5278)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EATimer_Init(hz)` (0x3e4af0): start the timer interrupt at
/// `hz` (default 100) and pick the video mode.
pub fn ea_timer_init(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_B0: u32 = 0x003e_4cb0;
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[V1][0] = 0x450000;
        frame.save(c, S2, 0x10);
        c.g[A3][0] = c.g[A0][0];
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xddc));
        c.g[S2][0] = c.g[V1][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        let fresh = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        'b_3e4d4c: {
            'b_3e4b24: {
                if fresh {
                    break 'b_3e4b24;
                }
                c.cycles += 3;
                c.g[V1][0] = 0x450000;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xdc8));
                break 'b_3e4d4c;
            }
            'b_3e4c14: {
                'b_3e4bc8: {
                    c.cycles += 5;
                    c.g[V0][0] = 0x64;
                    if c.g[A3][0] == 0 {
                        c.g[A3][0] = c.g[V0][0];
                    }
                    c.g[V1][0] = ((c.g[A3][0] as i64) < 3150) as u64;
                    let hi = c.g[V1][0] == 0;
                    c.g[V0][0] = ((c.g[A3][0] as i64) < 196) as u64;
                    if hi {
                        break 'b_3e4bc8;
                    }
                    'b_3e4bac: {
                        'b_3e4b78: {
                            c.cycles += 2;
                            let lo = c.g[V0][0] == 0;
                            c.g[V1][0] = sx((c.g[A3][0] as u32) >> 31);
                            if lo {
                                break 'b_3e4b78;
                            }
                            'b_3e4b58: {
                                c.cycles += 6;
                                c.g[A0][0] = 0x80000;
                                c.g[V1][0] =
                                    sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                c.g[A0][0] = c.g[A0][0] | 0xca00;
                                c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 1) as u32);
                                break 'b_3e4b58;
                            }
                            c.cycles += 8;
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.g[A1][0] = 0x450000;
                            c.div(0, 3, 7);
                            c.g[A2][0] = 0x450000;
                            c.g[S1][0] = c.g[A2][0];
                            c.g[T0][0] = c.g[A1][0];
                            c.g[A3][0] = 0x2;
                            break 'b_3e4bac;
                        }
                        'b_3e4b90: {
                            c.cycles += 6;
                            c.g[A0][0] = 0x8c0000;
                            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            c.g[A0][0] = c.g[A0][0] | 0xa000;
                            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 1) as u32);
                            break 'b_3e4b90;
                        }
                        c.cycles += 7;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[A1][0] = 0x450000;
                        c.div(0, 3, 7);
                        c.g[A2][0] = 0x450000;
                        c.g[S1][0] = c.g[A2][0];
                        c.g[T0][0] = c.g[A1][0];
                        c.g[A3][0] = 0x1;
                        break 'b_3e4bac;
                    }
                    c.cycles += 7;
                    c.g[V1][0] = c.lo[0];
                    c.sh((c.g[A1][0] as u32).wrapping_add(0xde0), c.g[V1][0]);
                    c.g[V0][0] = c.g[V1][0] & 0xffff;
                    c.div(0, 4, 2);
                    c.g[A0][0] = c.lo[0];
                    c.sw((c.g[A2][0] as u32).wrapping_add(0xdc8), c.g[A0][0]);
                    break 'b_3e4c14;
                }
                'b_3e4bdc: {
                    c.cycles += 5;
                    c.g[V1][0] = sx((c.g[A3][0] as u32) >> 31);
                    c.g[A1][0] = 0x8ca0000;
                    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    break 'b_3e4bdc;
                }
                c.cycles += 14;
                c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 1) as u32);
                c.g[A0][0] = 0x450000;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[A2][0] = 0x450000;
                c.div(0, 3, 7);
                c.g[S1][0] = c.g[A2][0];
                c.g[T0][0] = c.g[A0][0];
                c.g[A3][0] = 0;
                c.g[V1][0] = c.lo[0];
                c.sh((c.g[A0][0] as u32).wrapping_add(0xde0), c.g[V1][0]);
                c.g[V0][0] = c.g[V1][0] & 0xffff;
                c.div(0, 5, 2);
                c.g[A1][0] = c.lo[0];
                c.sw((c.g[A2][0] as u32).wrapping_add(0xdc8), c.g[A1][0]);
                break 'b_3e4c14;
            }
            'b_3e4d40: {
                c.cycles += 24;
                c.g[V0][0] = 0x10000000;
                c.g[A2][0] = 0x10000000;
                c.g[V0][0] = c.g[V0][0] | 0x800;
                c.g[A2][0] = c.g[A2][0] | 0x820;
                c.sw(c.g[V0][0] as u32, 0);
                c.g[V1][0] = 0x10000000;
                c.g[V1][0] = c.g[V1][0] | 0x810;
                c.g[A3][0] = c.g[A3][0] | 0x180;
                c.g[V0][0] = c.lhu((c.g[T0][0] as u32).wrapping_add(0xde0));
                c.g[A1][0] = 0x3e0000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4db8));
                c.g[A0][0] = 0xa;
                c.sw(c.g[A2][0] as u32, c.g[V0][0]);
                c.g[S0][0] = 0x3e0000;
                c.sw(c.g[V1][0] as u32, c.g[A3][0]);
                c.g[RA][0] = sx(0x003e_4c58);
                c.g[A2][0] = 0;
                let hook = 0x0042_3a80;
                c.pc = hook;
                call_ptr(c, hook, 0x003e_4c58)?;
                c.sw((c.g[S2][0] as u32).wrapping_add(0xddc), c.g[V0][0]);
                c.g[RA][0] = sx(0x003e_4c64);
                c.g[A0][0] = 0xa;
                let hook = 0x0042_48e8;
                c.pc = hook;
                call_ptr(c, hook, 0x003e_4c64)?;
                c.g[V1][0] = 0x450000;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xda0));
                let again = c.g[V0][0] != 0;
                c.g[V1][0] = 0x10000000;
                if again {
                    break 'b_3e4d40;
                }
                'b_3e4ccc: {
                    c.cycles += 14;
                    c.g[V0][0] = 0x10000000;
                    c.g[A2][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = c.g[V0][0] | 0x10;
                    c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V1][0] = 0x83;
                    c.sw(c.g[V0][0] as u32, c.g[V1][0]);
                    c.g[A1][0] = 0x450000;
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xdcc));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xdc8));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xdcc));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    let skip = c.g[V0][0] == 0;
                    c.g[V1][0] = c.g[A1][0];
                    if skip {
                        break 'b_3e4ccc;
                    }
                    c.cycles += 1;
                    'l_3e4cb0: loop {
                        c.cycles += 7;
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xdcc));
                        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                        let more = c.g[V0][0] != 0;
                        if more {
                            if event_due(c) {
                                deliver_event_here(c, HEAD_B0);
                            }
                            continue 'l_3e4cb0;
                        }
                        break 'b_3e4ccc;
                    }
                }
                'b_3e4cf4: {
                    c.cycles += 7;
                    c.g[V0][0] = 0x10000000;
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.sw(c.g[V0][0] as u32, c.g[A2][0]);
                    c.sw(c.g[V0][0] as u32, c.g[A3][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A2][0] as u32));
                    let nonneg = (c.g[V1][0] as i64) >= 0;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3d55));
                    if nonneg {
                        break 'b_3e4cf4;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 0x10000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3d55));
                    break 'b_3e4cf4;
                }
                'b_3e4d10: {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V0][0] < 0x63) as u64;
                    let pal = c.g[V0][0] == 0;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3cd8));
                    if pal {
                        break 'b_3e4d10;
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003e_4d08);
                    c.g[A0][0] = 0x1;
                    callee::EA_TIMER_SET_VIDEO_MODE.call(c, 0x003e_4d08)?;
                    break 'b_3e4d40;
                }
                'b_3e4d2c: {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V0][0] < 0x63) as u64;
                    let ntsc = c.g[V0][0] == 0;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7081));
                    if ntsc {
                        break 'b_3e4d2c;
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003e_4d24);
                    c.g[A0][0] = 0x2;
                    callee::EA_TIMER_SET_VIDEO_MODE.call(c, 0x003e_4d24)?;
                    break 'b_3e4d40;
                }
                c.cycles += 3;
                c.g[V0][0] = (c.g[V0][0] < 0xc7f) as u64;
                if c.g[V0][0] == 0 {
                    break 'b_3e4d40;
                }
                c.cycles += 2;
                c.g[RA][0] = sx(0x003e_4d40);
                c.g[A0][0] = 0x3;
                callee::EA_TIMER_SET_VIDEO_MODE.call(c, 0x003e_4d40)?;
                break 'b_3e4d40;
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x003e_4d48);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4d68));
            callee::EA_ADD_SHUTDOWN_HANDLER.call(c, 0x003e_4d48)?;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xdc8));
            break 'b_3e4d4c;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::events::driven;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;

    /// Write `s` as bytes at `addr`, NUL-terminated.
    fn put_ascii(c: &mut Cpu, s: &str, addr: u32) {
        for (i, &b) in s.as_bytes().iter().enumerate() {
            c.bus.write8(addr + i as u32, b);
        }
        c.bus.write8(addr + s.len() as u32, 0);
    }

    /// A zeroed data block.
    fn zero_block(c: &mut Cpu, base: u32, words: u32) {
        for i in 0..words {
            c.bus.write32(base + 4 * i, 0);
        }
    }

    driven!(gen_ea_file_cd_remount, ee::s14::f_003e4000__generated);
    driven!(hand_ea_file_cd_remount, ea_file_cd_remount);

    /// Flag clear always takes the early exit (mount arm is replay-gated).
    #[test]
    fn ea_file_cd_remount_matches() {
        check(
            gen_ea_file_cd_remount,
            hand_ea_file_cd_remount,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_4000, 0);
                c.bus.write32(0x0051_9c40, 0);
                c.pc = 0x003e_4000;
            },
        );
    }

    driven!(gen_ea_file_cd_get_entry, ee::s14::f_003e48d8__generated);
    driven!(hand_ea_file_cd_get_entry, ea_file_cd_get_entry);

    /// Null outs skip the copy; live outs take it.
    #[test]
    fn ea_file_cd_get_entry_matches() {
        crate::difftest::check_lohi(
            gen_ea_file_cd_get_entry,
            hand_ea_file_cd_get_entry,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_48d8, 0);
                c.bus.write32(0x0045_0c5c, 0x20);
                c.bus.write32(0x0051_9c78, DATA + 0x700);
                zero_block(c, DATA + 0x700, 32);
                put_ascii(c, "entry-one", DATA + 0x700 + 0x20 + 0xc);
                c.bus.write32(DATA + 0x700 + 0x20 + 0x8, 0x40);
                c.bus.write32(DATA + 0x700 + 0x20 + 0x4, 0x10);
                c.g[A0][0] = (seed % 3 + 1) as u64;
                c.g[A1][0] = if seed / 3 % 2 == 0 {
                    0
                } else {
                    sx(DATA + 0x500)
                };
                c.g[A2][0] = if seed / 6 % 2 == 0 {
                    0
                } else {
                    sx(DATA + 0x580)
                };
                c.g[A3][0] = if seed / 12 % 2 == 0 {
                    0
                } else {
                    sx(DATA + 0x5c0)
                };
                c.pc = 0x003e_48d8;
            },
        );
    }

    driven!(gen_ea_file_cd_find_entry, ee::s14::f_003e4968__generated);
    driven!(hand_ea_file_cd_find_entry, ea_file_cd_find_entry);

    /// Index lookup through the table, or zero when absent.
    #[test]
    fn ea_file_cd_find_entry_matches() {
        crate::difftest::check_lohi(
            gen_ea_file_cd_find_entry,
            hand_ea_file_cd_find_entry,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_4968, 0);
                c.bus.write32(0x0045_0c5c, 0x20);
                c.bus.write32(0x0051_9c78, DATA + 0x700);
                zero_block(c, DATA + 0x700, 32);
                put_ascii(c, "entry-one", DATA + 0x700 + 0x20 + 0xc);
                c.bus.write32(DATA + 0x700 + 0x20 + 0x8, 0x40);
                c.bus.write32(DATA + 0x700 + 0x20 + 0x4, 0x10);
                c.g[A0][0] = (seed % 3 + 1) as u64;
                c.g[A1][0] = 0;
                c.pc = 0x003e_4968;
            },
        );
    }

    driven!(gen_ea_file_cd_lookup, ee::s14::f_003e4508__generated);
    driven!(hand_ea_file_cd_lookup, ea_file_cd_lookup);

    /// Empty names skip the walk; a name walks the normalizer.
    /// (Zeroed table misses; hits need a coherent table.)
    #[test]
    fn ea_file_cd_lookup_matches() {
        crate::difftest::check_lohi(
            gen_ea_file_cd_lookup,
            hand_ea_file_cd_lookup,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_4508, 0);
                let names = ["", "a", "entry", "host0:dir/file"];
                put_ascii(c, names[seed as usize % 4], DATA + 0x400);
                zero_block(c, DATA + 0x500, 8);
                c.bus.write32(0x0045_0c58, DATA + 0x700);
                zero_block(c, DATA + 0x700, 16);
                c.bus.write32(0x0051_9c40 + 0x18, 0);
                c.bus.write32(0x0051_9c40 + 0x4, 0);
                c.bus.write32(0x0051_9c40 + 0x38, 0);
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = 0;
                c.g[A2][0] = sx(DATA + 0x600);
                c.g[A3][0] = 0;
                c.pc = 0x003e_4508;
            },
        );
    }

    driven!(gen_ea_file_cd_read, ee::s14::f_003e4648__generated);
    driven!(hand_ea_file_cd_read, ea_file_cd_read);

    /// Zeroed table with a live count drives the queue path.
    #[test]
    fn ea_file_cd_read_matches() {
        check(gen_ea_file_cd_read, hand_ea_file_cd_read, 48, &|c, seed| {
            let _r = machine(c, seed, 0x3e_4648, 0);
            // Point the entry table into controlled memory.
            c.bus.write32(0x0045_0c5c, DATA + 0x700 + 0x63c0 - 0xc);
            c.bus.write32(0x0051_9c40 + 0x38, DATA + 0x700);
            zero_block(c, DATA + 0x700, 32);
            c.bus.write32(0x0051_9c40, 0);
            c.g[A0][0] = (seed % 3 + 1) as u64;
            c.g[A1][0] = sx(DATA + 0x500);
            c.g[A2][0] = [0u32, 1, 0x800][seed as usize / 3 % 3] as u64;
            c.g[A3][0] = [0u32, 0x800, 0x1000][seed as usize / 9 % 3] as u64;
            c.pc = 0x003e_4648;
        });
    }

    driven!(gen_ea_timer_shutdown, ee::s14::f_003e4d68__generated);
    driven!(hand_ea_timer_shutdown, ea_timer_shutdown);

    /// Set and clear interrupt cells both converge.
    #[test]
    fn ea_timer_shutdown_matches() {
        check(
            gen_ea_timer_shutdown,
            hand_ea_timer_shutdown,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_4d68, 0);
                c.bus.write32(0x0045_0ddc, (seed % 2) as u32);
                c.pc = 0x003e_4d68;
            },
        );
    }

    driven!(gen_ea_timer_get_delta, ee::s14::f_003e4ea8__generated);
    driven!(hand_ea_timer_get_delta, ea_timer_get_delta);

    /// Stale and fresh last-tick cells.
    #[test]
    fn ea_timer_get_delta_matches() {
        check(
            gen_ea_timer_get_delta,
            hand_ea_timer_get_delta,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_4ea8, 0);
                c.bus
                    .write32(0x0045_0dec, [0u32, 1, 0x1000][seed as usize % 3]);
                c.pc = 0x003e_4ea8;
            },
        );
    }

    driven!(
        gen_ea_file_timer_set_deadline,
        ee::s14::f_003e4f08__generated
    );
    driven!(hand_ea_file_timer_set_deadline, ea_file_timer_set_deadline);

    /// Zero and nonzero deltas.
    #[test]
    fn ea_file_timer_set_deadline_matches() {
        check(
            gen_ea_file_timer_set_deadline,
            hand_ea_file_timer_set_deadline,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_4f08, 0);
                c.g[A0][0] = [0u32, 1, 0x1e0, 0x10000][seed as usize % 4] as u64;
                c.bus.write32(0x0045_0de8, 0);
                c.pc = 0x003e_4f08;
            },
        );
    }

    driven!(
        gen_ea_file_timer_wait_deadline,
        ee::s14::f_003e4f40__generated
    );
    driven!(
        hand_ea_file_timer_wait_deadline,
        ea_file_timer_wait_deadline
    );

    /// Deadline already past exits without pumping.
    #[test]
    fn ea_file_timer_wait_deadline_matches() {
        check(
            gen_ea_file_timer_wait_deadline,
            hand_ea_file_timer_wait_deadline,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_4f40, 0);
                c.bus.write32(0x0045_0de8, 0);
                c.pc = 0x003e_4f40;
            },
        );
    }

    driven!(
        gen_ea_file_timer_deadline_passed,
        ee::s14::f_003e4f80__generated
    );
    driven!(
        hand_ea_file_timer_deadline_passed,
        ea_file_timer_deadline_passed
    );

    /// Past, present and future deadlines.
    #[test]
    fn ea_file_timer_deadline_passed_matches() {
        check(
            gen_ea_file_timer_deadline_passed,
            hand_ea_file_timer_deadline_passed,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_4f80, 0);
                c.bus.write32(
                    0x0045_0de8,
                    [0u32, 0x7fff_ffff, 0xffff_ffff][seed as usize % 3],
                );
                c.pc = 0x003e_4f80;
            },
        );
    }

    driven!(gen_ea_file_timer_delay, ee::s14::f_003e4fb0__generated);
    driven!(hand_ea_file_timer_delay, ea_file_timer_delay);

    /// Zero delay exits at once (larger delays spin on the live
    /// timer and cannot terminate on frozen test ticks).
    #[test]
    fn ea_file_timer_delay_matches() {
        check(
            gen_ea_file_timer_delay,
            hand_ea_file_timer_delay,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_4fb0, 0);
                c.g[A0][0] = 0;
                c.pc = 0x003e_4fb0;
            },
        );
    }

    driven!(
        gen_ea_file_timer_get_tick_rate,
        ee::s14::f_003e5008__generated
    );
    driven!(
        hand_ea_file_timer_get_tick_rate,
        ea_file_timer_get_tick_rate
    );

    /// Pure cached-word load.
    #[test]
    fn ea_file_timer_get_tick_rate_matches() {
        check(
            gen_ea_file_timer_get_tick_rate,
            hand_ea_file_timer_get_tick_rate,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_5008, 0);
                c.bus.write32(0x0045_0dc8, 100 + (seed % 3));
                c.pc = 0x003e_5008;
            },
        );
    }

    driven!(gen_ea_thread_init_main, ee::s14::f_003e5018__generated);
    driven!(hand_ea_thread_init_main, ea_thread_init_main);

    /// Main-thread tag through the syscall fakes.
    #[test]
    fn ea_thread_init_main_matches() {
        check(
            gen_ea_thread_init_main,
            hand_ea_thread_init_main,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_5018, 0);
                c.pc = 0x003e_5018;
            },
        );
    }

    driven!(gen_ea_thread_shutdown, ee::s14::f_003e5068__generated);
    driven!(hand_ea_thread_shutdown, ea_thread_shutdown);

    /// Mutex destroy plus slot clear.
    #[test]
    fn ea_thread_shutdown_matches() {
        check(
            gen_ea_thread_shutdown,
            hand_ea_thread_shutdown,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_5068, 0);
                c.pc = 0x003e_5068;
            },
        );
    }

    driven!(gen_ea_thread_create, ee::s14::f_003e50c8__generated);
    driven!(hand_ea_thread_create, ea_thread_create);

    /// Default and tabled priority slots.
    #[test]
    fn ea_thread_create_matches() {
        check(
            gen_ea_thread_create,
            hand_ea_thread_create,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_50c8, 0);
                zero_block(c, DATA + 0x500, 16);
                c.g[A0][0] = sx(DATA + 0x500);
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = sx(DATA + 0x500);
                c.g[A3][0] = 0;
                c.g[T0][0] = 0;
                c.g[T1][0] = [0u64, 0x61, 0x64, 0x67][seed as usize % 4];
                c.pc = 0x003e_50c8;
            },
        );
    }

    driven!(gen_ea_thread_destroy, ee::s14::f_003e51a0__generated);
    driven!(hand_ea_thread_destroy, ea_thread_destroy);

    /// Self (-1) and other threads; live and dead main slots.
    #[test]
    fn ea_thread_destroy_matches() {
        check(
            gen_ea_thread_destroy,
            hand_ea_thread_destroy,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_51a0, 0);
                zero_block(c, DATA + 0x500, 8);
                c.bus.write32(DATA + 0x504, 0x1234);
                c.bus.write32(0x0045_0de4, (seed % 2) as u32);
                c.g[A0][0] = if seed / 2 % 2 == 0 {
                    0xffff_ffff_ffff_ffff
                } else {
                    sx(DATA + 0x500)
                };
                c.pc = 0x003e_51a0;
            },
        );
    }

    driven!(gen_ea_thread_destroy_thunk, ee::s14::f_003e5268__generated);
    driven!(hand_ea_thread_destroy_thunk, ea_thread_destroy_thunk);

    /// Exiting-thread destroy through the syscall fakes.
    #[test]
    fn ea_thread_destroy_thunk_matches() {
        check(
            gen_ea_thread_destroy_thunk,
            hand_ea_thread_destroy_thunk,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_5268, 0);
                zero_block(c, DATA + 0x500, 8);
                c.bus.write32(DATA + 0x504, 0x1234);
                c.bus.write32(0x0045_0de4, 0);
                c.g[A0][0] = sx(DATA + 0x500);
                c.pc = 0x003e_5268;
            },
        );
    }

    driven!(gen_ea_timer_init, ee::s14::f_003e4af0__generated);
    driven!(hand_ea_timer_init, ea_timer_init);

    /// Fresh (all three divisor arms) and already-init exits. The
    /// table-wait arm is skipped via the live flag: it spins on the
    /// hardware timer and cannot terminate on frozen test ticks
    /// (covered by the replay gates instead).
    #[test]
    fn ea_timer_init_matches() {
        crate::difftest::check_lohi(gen_ea_timer_init, hand_ea_timer_init, 32, &|c, seed| {
            let _r = machine(c, seed, 0x3e_4af0, 0);
            let fresh = seed % 2 == 0;
            c.bus.write32(0x0045_0ddc, if fresh { 0 } else { 1 });
            c.bus.write32(0x0045_0dc8, 0);
            c.bus.write32(0x0045_0dcc, 0);
            // Always skip the hardware wait: it spins on live timer
            // ticks and cannot terminate on frozen test RAM.
            c.bus.write32(0x0045_0da0, 1);
            c.bus.write32(0xdcc, 0);
            c.g[A0][0] = [0u32, 100, 200, 5000][seed as usize / 4 % 4] as u64;
            c.pc = 0x003e_4af0;
        });
    }
}
