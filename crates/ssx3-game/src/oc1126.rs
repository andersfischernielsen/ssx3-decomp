//! Staging module oc1126 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1126`.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003e_2df0, big_file_get_entry_data, "BigFile_GetEntryData"),
    (
        0x003e_2e20,
        big_file_get_entry_data_2,
        "BigFile_GetEntryData_2",
    ),
    (
        0x003e_2e50,
        big_file_get_entry_data_list,
        "BigFile_GetEntryDataList",
    ),
    (
        0x003e_2ee0,
        big_file_get_entry_count,
        "BigFile_GetEntryCount",
    ),
    (0x003e_2f70, big_file_get_entry_name, "BigFile_GetEntryName"),
    (
        0x003e_2fc8,
        big_file_get_entry_name_2,
        "BigFile_GetEntryName_2",
    ),
    (0x003e_3020, ea_file_platform_init, "EAFile_PlatformInit"),
    (0x003e_3098, ea_file_resolve_path, "EAFile_ResolvePath"),
    (0x003e_3478, ea_file_platform_write, "EAFile_PlatformWrite"),
    (
        0x003e_3538,
        ea_file_platform_get_lsn,
        "EAFile_PlatformGetLsn",
    ),
    (0x003e_3588, ea_file_alarm_handler, "EAFile_AlarmHandler"),
    (0x003e_35b0, ea_file_delay_ticks, "EAFile_DelayTicks"),
    (0x003e_36f8, ea_file_hash_name, "EAFile_HashName"),
    (
        0x003e_39a8,
        ea_file_cd_build_hash_table,
        "EAFile_CdBuildHashTable",
    ),
    (0x003e_3ad8, ea_file_cd_signal, "EAFile_CdSignal"),
];

mod callee {
    use super::ee;
    use crate::call::Callee;

    pub(crate) const BIG_FILE_GET_ENTRY_OFFSET: Callee = Callee {
        addr: 0x003e_2d00,
        f: ee::s14::BigFile_GetEntryOffset,
    };
    pub(crate) const BIG_FILE_GET_ENTRY_OFFSET_2: Callee = Callee {
        addr: 0x003e_2d30,
        f: ee::s14::BigFile_GetEntryOffset_2,
    };
    pub(crate) const BIG_FILE_FIND_ENTRY: Callee = Callee {
        addr: 0x003e_2ce0,
        f: ee::s14::BigFile_FindEntry,
    };
    pub(crate) const EA_FILE_BIG_FIND_ENTRY: Callee = Callee {
        addr: 0x003e_2768,
        f: ee::s14::EAFile_BigFindEntry,
    };
    pub(crate) const BIG_FILE_GET_TYPE: Callee = Callee {
        addr: 0x003e_2500,
        f: ee::s14::BigFile_GetType,
    };
    pub(crate) const BIG_FILE_GET_ENTRY_DATA: Callee = Callee {
        addr: 0x003e_2df0,
        f: ee::s14::BigFile_GetEntryData,
    };
    pub(crate) const EA_FILE_CD_GET_ENTRY: Callee = Callee {
        addr: 0x003e_48d8,
        f: ee::s14::EAFile_CdGetEntry,
    };
    pub(crate) const EA_FILE_HASH_NAME: Callee = Callee {
        addr: 0x003e_36f8,
        f: ee::s14::EAFile_HashName,
    };
    pub(crate) const EA_FILE_COMPLETE_REQUEST: Callee = Callee {
        addr: 0x003d_e420,
        f: ee::s14::EAFile_CompleteRequest,
    };
    pub(crate) const EA_TIMER_ADD_CALLBACK: Callee = Callee {
        addr: 0x003e_57f8,
        f: ee::s14::EATimer_AddCallback,
    };
}

/// `u32 BigFile_GetEntryData(header, name)` (0x3e2df0): entry address
/// through the offset helper, or 0 for a missing name.
pub fn big_file_get_entry_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_2e04);
        c.g[S0][0] = c.g[A0][0];
        callee::BIG_FILE_GET_ENTRY_OFFSET.call(c, 0x003e_2e04)?;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        if c.g[V0][0] == 0 {
            c.g[S0][0] = 0;
        }
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 BigFile_GetEntryData_2(header, name)` (0x3e2e20): same through
/// the second offset helper.
pub fn big_file_get_entry_data_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_2e34);
        c.g[S0][0] = c.g[A0][0];
        callee::BIG_FILE_GET_ENTRY_OFFSET_2.call(c, 0x003e_2e34)?;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        if c.g[V0][0] == 0 {
            c.g[S0][0] = 0;
        }
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BigFile_GetEntryDataList(header, names, out)` (0x3e2e50):
/// comma-split list lookup, one address per name.
pub fn big_file_get_entry_data_list(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_80: u32 = 0x003e_2e80;
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        c.g[S4][0] = 0x2c;
        frame.save_ra(c, 0);
        'l_3e2e80: loop {
            'b_3e2e98: {
                c.cycles += 6;
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x003e_2e8c);
                c.g[A1][0] = 0x2c;
                let hook = 0x0041_6404;
                c.pc = hook;
                call_ptr(c, hook, 0x003e_2e8c)?;
                c.g[S0][0] = c.g[V0][0];
                if c.g[S0][0] != 0 {
                    c.sb(c.g[S0][0] as u32, 0);
                }
                break 'b_3e2e98;
            }
            'b_3e2ec0: {
                c.cycles += 6;
                c.g[A1][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x003e_2ea4);
                c.g[A0][0] = c.g[S3][0];
                callee::BIG_FILE_GET_ENTRY_DATA.call(c, 0x003e_2ea4)?;
                c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                let done = c.g[S0][0] == 0;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                if done {
                    break 'b_3e2ec0;
                }
                c.cycles += 4;
                c.sb(c.g[S0][0] as u32, c.g[S4][0]);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                let more = c.g[S0][0] != 0;
                c.g[S1][0] = c.g[S0][0];
                if more {
                    if event_due(c) {
                        deliver_event_here(c, HEAD_80);
                    }
                    continue 'l_3e2e80;
                }
                break 'b_3e2ec0;
            }
            // Both break arms converge here, inside the loop: the epilogue
            // returns, so the loop only repeats via `continue`.
            c.cycles += 8;
            frame.restore(c, S0, 0x50);
            frame.restore(c, S1, 0x40);
            frame.restore(c, S2, 0x30);
            frame.restore(c, S3, 0x20);
            frame.restore(c, S4, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
    })
}

/// `u32 BigFile_GetEntryCount(header)` (0x3e2ee0): entry count from the
/// header words, by header type.
pub fn big_file_get_entry_count(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_2ef8);
        c.g[S0][0] = c.g[A0][0];
        callee::BIG_FILE_GET_TYPE.call(c, 0x003e_2ef8)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = 0x1;
        let not_one = c.g[V1][0] != c.g[V0][0];
        c.g[S1][0] = 0;
        'b_3e2f54: {
            'b_3e2f1c: {
                if not_one {
                    break 'b_3e2f1c;
                }
                c.cycles += 5;
                c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x5));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[S1][0] = c.g[V0][0] | c.g[V1][0];
                break 'b_3e2f54;
            }
            // Tail: gen re-tests V1/V0 here (not S1): the t==1 arm above
            // leaves V1 = type and V0 = 1, so the fall-through reuses them.
            c.cycles += 2;
            let small = (c.g[V1][0] as i64) <= 0;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 4) as u64;
            if small {
                break 'b_3e2f54;
            }
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                break 'b_3e2f54;
            }
            c.cycles += 10;
            c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x9));
            c.g[A1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xa));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 16);
            c.g[A2][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xb));
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32) << 8);
            c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
            c.g[S1][0] = c.g[V0][0] | c.g[A2][0];
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 6;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `char *BigFile_GetEntryName(header, name, dst)` (0x3e2f70): entry
/// path through the finder, copied to the caller buffer.
pub fn big_file_get_entry_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[T0][0] = 0;
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[A1][0];
        c.g[RA][0] = sx(0x003e_2f94);
        c.g[A1][0] = 0;
        callee::BIG_FILE_FIND_ENTRY.call(c, 0x003e_2f94)?;
        let found = c.g[V0][0] != 0;
        c.g[A1][0] = c.g[V0][0];
        'b_3e2fb4: {
            if !found {
                c.cycles += 3;
                c.sb(c.g[S0][0] as u32, 0);
                c.g[V0][0] = 0;
                break 'b_3e2fb4;
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x003e_2fb0);
            c.g[A0][0] = c.g[S0][0];
            let hook = 0x0041_66f4;
            c.pc = hook;
            call_ptr(c, hook, 0x003e_2fb0)?;
            c.g[V0][0] = c.g[S0][0];
            break 'b_3e2fb4;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `char *BigFile_GetEntryName_2(header, name, dst)` (0x3e2fc8): same
/// through the EAFile finder.
pub fn big_file_get_entry_name_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[T0][0] = 0;
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[A1][0];
        c.g[RA][0] = sx(0x003e_2fec);
        c.g[A1][0] = 0;
        callee::EA_FILE_BIG_FIND_ENTRY.call(c, 0x003e_2fec)?;
        let found = c.g[V0][0] != 0;
        c.g[A1][0] = c.g[V0][0];
        'b_3e3004: {
            if !found {
                c.cycles += 3;
                c.sb(c.g[S0][0] as u32, 0);
                c.g[V0][0] = 0;
                break 'b_3e3004;
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x003e_3008);
            c.g[A0][0] = c.g[S0][0];
            let hook = 0x0041_66f4;
            c.pc = hook;
            call_ptr(c, hook, 0x003e_3008)?;
            c.g[V0][0] = c.g[S0][0];
            break 'b_3e3004;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void EAFile_PlatformInit(use_cd)` (0x3e3020): timer callback setup
/// and boot flag writes.
pub fn ea_file_platform_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 0x3e0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x33b0));
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = 0x450000;
        frame.save_ra(c, 0);
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc48), 0);
        c.g[A1][0] = 0;
        c.g[RA][0] = sx(0x003e_3050);
        c.g[A2][0] = 0;
        callee::EA_TIMER_ADD_CALLBACK.call(c, 0x003e_3050)?;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc48));
        c.g[V0][0] = c.g[V1][0] | 0x2;
        let cold = c.g[S0][0] == 0;
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc48), c.g[V0][0]);
        'b_3e3084: {
            'b_3e3078: {
                if cold {
                    break 'b_3e3078;
                }
                c.cycles += 6;
                c.g[V0][0] = c.g[V1][0] | 0x3;
                c.g[A0][0] = 0x450000;
                c.g[V1][0] = 0x1;
                c.sw((c.g[S1][0] as u32).wrapping_add(0xc48), c.g[V0][0]);
                c.sw((c.g[A0][0] as u32).wrapping_add(0xc44), c.g[V1][0]);
                break 'b_3e3084;
            }
            c.cycles += 3;
            c.g[V1][0] = 0x450000;
            c.g[V0][0] = 0x2;
            c.sw((c.g[V1][0] as u32).wrapping_add(0xc44), c.g[V0][0]);
            break 'b_3e3084;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_PlatformWrite(fd_gate, buf, len, gate)` (0x3e3478):
/// host write pair plus the completion callback.
pub fn ea_file_platform_write(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0xff0000;
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0] & c.g[V0][0];
        c.g[A1][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x003e_34b0);
        c.g[A2][0] = 0;
        let hook = 0x0042_8168;
        c.pc = hook;
        call_ptr(c, hook, 0x003e_34b0)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x003e_34c0);
        c.g[A0][0] = c.g[S0][0];
        let hook = 0x0042_8600;
        c.pc = hook;
        call_ptr(c, hook, 0x003e_34c0)?;
        c.g[RA][0] = sx(0x003e_34c8);
        c.g[A0][0] = 0x1;
        callee::EA_FILE_COMPLETE_REQUEST.call(c, 0x003e_34c8)?;
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_PlatformGetLsn(drive_fd)` (0x3e3538): LSN of the disc
/// entry for host drives, 0 otherwise.
pub fn ea_file_platform_get_lsn(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        c.g[V0][0] = 0xff0000;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        c.g[A1][0] = sx(((c.g[A0][0] as u32) >> 24) as u32);
        frame.save_ra(c, 0x10);
        c.g[V1][0] = 0x1;
        let host = c.g[A1][0] != c.g[V1][0];
        c.g[A0][0] = c.g[A0][0] & c.g[V0][0];
        'b_3e3578: {
            'b_3e3570: {
                if host {
                    break 'b_3e3570;
                }
                c.cycles += 6;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                c.g[RA][0] = sx(0x003e_3568);
                c.g[A3][0] = c.g[SP][0];
                callee::EA_FILE_CD_GET_ENTRY.call(c, 0x003e_3568)?;
                c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                break 'b_3e3578;
            }
            c.cycles += 2;
            c.sw(c.g[SP][0] as u32, 0);
            c.g[V0][0] = c.lw(c.g[SP][0] as u32);
            break 'b_3e3578;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void EAFile_AlarmHandler(arg)` (0x3e3588): host alarm ack plus
/// an EE interrupt enable.
pub fn ea_file_alarm_handler(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_3598);
        c.g[A0][0] = c.g[A2][0];
        let hook = 0x0042_3dd0;
        c.pc = hook;
        call_ptr(c, hook, 0x003e_3598)?;
        c.ei();
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void EAFile_DelayTicks(ticks)` (0x3e35b0): timer-primitive wait.
pub fn ea_file_delay_ticks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x1;
        frame.save(c, S1, 0x30);
        c.g[S1][0] = c.g[A0][0] & 0xffff;
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0x20);
        c.g[A0][0] = c.g[SP][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0);
        c.g[RA][0] = sx(0x003e_35dc);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), 0);
        let hook = 0x0042_3da0;
        c.pc = hook;
        call_ptr(c, hook, 0x003e_35dc)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 0x3e0000;
        c.g[A2][0] = c.g[S0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3588));
        c.g[RA][0] = sx(0x003e_35f4);
        c.g[A0][0] = c.g[S1][0];
        let hook = 0x0042_3b20;
        c.pc = hook;
        call_ptr(c, hook, 0x003e_35f4)?;
        c.g[RA][0] = sx(0x003e_35fc);
        c.g[A0][0] = c.g[S0][0];
        let hook = 0x0042_3de0;
        c.pc = hook;
        call_ptr(c, hook, 0x003e_35fc)?;
        c.g[RA][0] = sx(0x003e_3604);
        c.g[A0][0] = c.g[S0][0];
        let hook = 0x0042_3db0;
        c.pc = hook;
        call_ptr(c, hook, 0x003e_3604)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_HashName(seed, path)` (0x3e36f8): trailing-slash
/// tolerant `h = h*33 + c` fold over the name.
pub fn ea_file_hash_name(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_20: u32 = 0x003e_3720;
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_3714);
        c.g[S0][0] = c.g[A1][0];
        let hook = 0x0041_6810;
        c.pc = hook;
        call_ptr(c, hook, 0x003e_3714)?;
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        'l_3e3720: loop {
            c.cycles += 7;
            c.g[V0][0] = c.lb(c.g[A2][0] as u32);
            c.g[V1][0] = sx((c.g[S0][0] as u32) << 5);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
            c.g[A0][0] = (c.g[A2][0] < c.g[S1][0]) as u64;
            let more = c.g[A0][0] == 0;
            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            if more {
                if event_due(c) {
                    deliver_event_here(c, HEAD_20);
                }
                continue 'l_3e3720;
            }
            c.cycles += 6;
            c.g[V0][0] = c.g[S0][0];
            frame.restore(c, S1, 0x10);
            frame.restore(c, S0, 0x20);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
    })
}

/// `void EAFile_CdSignal(arg)` (0x3e3ad8): host signal ack.
pub fn ea_file_cd_signal(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[V0][0] = 0x520000;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x63b4));
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_3af0);
        let hook = 0x0042_3dd0;
        c.pc = hook;
        call_ptr(c, hook, 0x003e_3af0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_ResolvePath(src, dst, status_out)` (0x3e3098): split a
/// `host:name` path, compare the host prefix, and join the remainder.
pub fn ea_file_resolve_path(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_B8: u32 = 0x003e_31b8;
        c.cycles += 19;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S5, 0x20);
        frame.save(c, S6, 0x10);
        c.g[S5][0] = 0x450000;
        frame.save(c, S2, 0x50);
        c.g[S6][0] = c.g[A2][0];
        frame.save(c, S4, 0x30);
        c.g[A1][0] = 0x3a;
        frame.save_ra(c, 0);
        c.sb(c.g[S0][0] as u32, 0);
        c.g[RA][0] = sx(0x003e_30dc);
        c.g[S3][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xc44));
        let hook = 0x0041_6404;
        c.pc = hook;
        call_ptr(c, hook, 0x003e_30dc)?;
        c.g[S4][0] = 0x450000;
        'b_3e31e4: {
            'b_3e31e0: {
                'b_3e3150: {
                    if c.g[V0][0] == 0 {
                        break 'b_3e3150;
                    }
                    'b_3e3118: {
                        c.cycles += 10;
                        c.g[RA][0] = sx(0x003e_30ec);
                        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xc50));
                        let hook = 0x0041_6810;
                        c.pc = hook;
                        call_ptr(c, hook, 0x003e_30ec)?;
                        c.g[S2][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S1][0];
                        c.g[V0][0] = 0x450000;
                        c.g[A2][0] = 0x3;
                        c.g[RA][0] = sx(0x003e_3104);
                        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc3c));
                        let hook = 0x0041_6b18;
                        c.pc = hook;
                        call_ptr(c, hook, 0x003e_3104)?;
                        c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xc50));
                        if c.g[V0][0] != 0 {
                            break 'b_3e3118;
                        }
                        c.cycles += 3;
                        c.g[S3][0] = 0x1;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3));
                        break 'b_3e3150;
                    }
                    'b_3e3138: {
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x003e_3124);
                        c.g[A2][0] = c.g[S2][0];
                        let hook = 0x0041_6b18;
                        c.pc = hook;
                        call_ptr(c, hook, 0x003e_3124)?;
                        c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xc44));
                        if c.g[V0][0] != 0 {
                            break 'b_3e3138;
                        }
                        c.cycles += 3;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S2][0] as u32));
                        c.g[S3][0] = 0x2;
                        break 'b_3e3150;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    let is_boot = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x3a;
                    if is_boot {
                        break 'b_3e31e0;
                    }
                    c.cycles += 3;
                    c.g[V1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x2));
                    if c.g[V1][0] != c.g[V0][0] {
                        frame.restore(c, S0, 0x70);
                        break 'b_3e31e4;
                    }
                    break 'b_3e3150;
                }
                {
                    // S3 != 2 skips the host combine; both arms converge.
                    c.cycles += 3;
                    c.g[V0][0] = 0x2;
                    if c.g[S3][0] != c.g[V0][0] {
                        c.g[V1][0] = c.lb(c.g[S1][0] as u32);
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = 0x450000;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x003e_316c);
                        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc50));
                        let hook = 0x0041_62d0;
                        c.pc = hook;
                        call_ptr(c, hook, 0x003e_316c)?;
                        c.g[V1][0] = c.lb(c.g[S1][0] as u32);
                        c.g[V1][0] = c.lb(c.g[S1][0] as u32);
                    }
                }
                'b_3e3198: {
                    c.cycles += 3;
                    c.g[V0][0] = 0x5c;
                    let is_bs = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x2f;
                    if is_bs {
                        break 'b_3e3198;
                    }
                    c.cycles += 2;
                    let is_sl = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x7e;
                    if is_sl {
                        break 'b_3e3198;
                    }
                    c.cycles += 2;
                    let is_tl = c.g[V1][0] == c.g[V0][0];
                    c.g[A1][0] = 0x520000;
                    if is_tl {
                        break 'b_3e3198;
                    }
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003e_3198);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6510));
                    let hook = 0x0041_62d0;
                    c.pc = hook;
                    call_ptr(c, hook, 0x003e_3198)?;
                    break 'b_3e3198;
                }
                'b_3e31d8: {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003e_31a4);
                    c.g[A1][0] = c.g[S1][0];
                    let hook = 0x0041_62d0;
                    c.pc = hook;
                    call_ptr(c, hook, 0x003e_31a4)?;
                    c.g[V1][0] = c.lb(c.g[S0][0] as u32);
                    let empty = c.g[V1][0] == 0;
                    c.g[A0][0] = c.lbu(c.g[S0][0] as u32);
                    if empty {
                        break 'b_3e31d8;
                    }
                    c.cycles += 2;
                    c.g[A1][0] = 0x5c;
                    c.g[V1][0] = 0x2f;
                    'l_3e31b8: loop {
                        'b_3e31c8: {
                            c.cycles += 4;
                            c.g[V0][0] = sx((c.g[A0][0] as u32) << 24);
                            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                            if c.g[V0][0] == c.g[A1][0] {
                                c.sb(c.g[S0][0] as u32, c.g[V1][0]);
                            }
                            break 'b_3e31c8;
                        }
                        c.cycles += 4;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                        let more = c.g[V0][0] != 0;
                        c.g[A0][0] = c.lbu(c.g[S0][0] as u32);
                        if more {
                            if event_due(c) {
                                deliver_event_here(c, HEAD_B8);
                            }
                            continue 'l_3e31b8;
                        }
                        break 'b_3e31d8;
                    }
                }
                c.cycles += 2;
                if c.g[S6][0] != 0 {
                    c.sw(c.g[S6][0] as u32, c.g[S3][0]);
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x70);
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_CdBuildHashTable(dst, n, src, seed)` (0x3e39a8):
/// hash every name, then sort the table.
pub fn ea_file_cd_build_hash_table(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_A08: u32 = 0x003e_3a08;
        const HEAD_9F0: u32 = 0x003e_39f0;
        c.cycles += 18;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S3, 0x70);
        frame.save(c, S5, 0x50);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S6, 0x40);
        c.g[S5][0] = c.g[A2][0];
        frame.save(c, S7, 0x30);
        c.g[S6][0] = c.g[A1][0];
        frame.save(c, FP, 0x20);
        c.g[S7][0] = 0x520000;
        frame.save(c, S0, 0xa0);
        c.g[FP][0] = 0;
        frame.save(c, S1, 0x90);
        frame.save(c, S2, 0x80);
        frame.save(c, S4, 0x60);
        frame.save_ra(c, 0x10);
        c.sw(c.g[SP][0] as u32, c.g[A0][0]);
        c.g[S1][0] = c.g[S6][0];
        'l_3e39f0: loop {
            'b_3e3a54: {
                c.cycles += 2;
                let empty = (c.g[S5][0] as i64) <= 0;
                c.g[S0][0] = c.lw(c.g[SP][0] as u32);
                if empty {
                    break 'b_3e3a54;
                }
                c.cycles += 4;
                c.g[V0][0] = 0x450000;
                c.g[S2][0] = c.g[S5][0];
                c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc58));
                'l_3e3a08: loop {
                    'b_3e3a40: {
                        c.cycles += 11;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x003e_3a14);
                        c.g[A1][0] = c.g[S3][0];
                        callee::EA_FILE_HASH_NAME.call(c, 0x003e_3a14)?;
                        c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x100));
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x104));
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                        c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                        let no_copy = (c.g[V0][0] as i64) <= 0;
                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
                        if no_copy {
                            break 'b_3e3a40;
                        }
                        c.cycles += 3;
                        c.g[RA][0] = sx(0x003e_3a3c);
                        c.g[A1][0] = c.g[S0][0];
                        let hook = 0x0041_66f4;
                        c.pc = hook;
                        call_ptr(c, hook, 0x003e_3a3c)?;
                        c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                        break 'b_3e3a40;
                    }
                    c.cycles += 5;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x108));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc));
                    let more = c.g[S2][0] != 0;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, HEAD_A08);
                        }
                        continue 'l_3e3a08;
                    }
                    break 'b_3e3a54;
                }
            }
            'b_3e3a98: {
                'b_3e3a94: {
                    c.cycles += 13;
                    c.g[V1][0] = 0x450000;
                    c.g[V0][0] = 0x3e0000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc58));
                    c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3968));
                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.g[S6][0];
                    c.sw((c.g[S7][0] as u32).wrapping_sub(0x1280), 0);
                    c.g[A1][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x003e_3a7c);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0xc));
                    let hook = 0x0041_8ef8;
                    c.pc = hook;
                    call_ptr(c, hook, 0x003e_3a7c)?;
                    c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_sub(0x1280));
                    let clean = c.g[V0][0] == 0;
                    c.g[V1][0] = 0x65;
                    if clean {
                        break 'b_3e3a94;
                    }
                    c.cycles += 3;
                    c.mult(0, 2, 19, 3);
                    c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7));
                    break 'b_3e3a98;
                }
                c.cycles += 1;
                c.g[FP][0] = 0x1;
                break 'b_3e3a98;
            }
            c.cycles += 2;
            let retry = c.g[FP][0] == 0;
            c.g[S1][0] = c.g[S6][0];
            if retry {
                if event_due(c) {
                    deliver_event_here(c, HEAD_9F0);
                }
                continue 'l_3e39f0;
            }
            c.cycles += 13;
            c.g[V0][0] = c.g[S3][0];
            frame.restore(c, S0, 0xa0);
            frame.restore(c, S1, 0x90);
            frame.restore(c, S2, 0x80);
            frame.restore(c, S3, 0x70);
            frame.restore(c, S4, 0x60);
            frame.restore(c, S5, 0x50);
            frame.restore(c, S6, 0x40);
            frame.restore(c, S7, 0x30);
            frame.restore(c, FP, 0x20);
            frame.restore_ra(c, 0x10);
            frame.close(c);
            return Ok(());
        }
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

    driven!(gen_big_file_get_entry_data, ee::s14::f_003e2df0__generated);
    driven!(hand_big_file_get_entry_data, big_file_get_entry_data);

    /// Missing names report zero; present ones report header+offset.
    #[test]
    fn big_file_get_entry_data_matches() {
        check(
            gen_big_file_get_entry_data,
            hand_big_file_get_entry_data,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_2df0, 0);
                let block = DATA + 0x700 + (seed % 3) * 0x10;
                zero_block(c, block, 16);
                put_ascii(c, "entry", DATA + 0x400);
                c.g[A0][0] = sx(block);
                c.g[A1][0] = if seed % 4 == 0 { 0 } else { sx(DATA + 0x400) };
                c.pc = 0x003e_2df0;
            },
        );
    }

    driven!(
        gen_big_file_get_entry_data_2,
        ee::s14::f_003e2e20__generated
    );
    driven!(hand_big_file_get_entry_data_2, big_file_get_entry_data_2);

    /// Same via the second offset helper.
    #[test]
    fn big_file_get_entry_data_2_matches() {
        check(
            gen_big_file_get_entry_data_2,
            hand_big_file_get_entry_data_2,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_2e20, 0);
                let block = DATA + 0x700 + (seed % 3) * 0x10;
                zero_block(c, block, 16);
                put_ascii(c, "entry", DATA + 0x400);
                c.g[A0][0] = sx(block);
                c.g[A1][0] = if seed % 4 == 0 { 0 } else { sx(DATA + 0x400) };
                c.pc = 0x003e_2e20;
            },
        );
    }

    driven!(
        gen_big_file_get_entry_data_list,
        ee::s14::f_003e2e50__generated
    );
    driven!(
        hand_big_file_get_entry_data_list,
        big_file_get_entry_data_list
    );

    /// Comma counts drive 1/multi-iteration walks; no-comma ends at once.
    #[test]
    fn big_file_get_entry_data_list_matches() {
        check(
            gen_big_file_get_entry_data_list,
            hand_big_file_get_entry_data_list,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_2e50, 0);
                let block = DATA + 0x700;
                zero_block(c, block, 16);
                let names = ["one", "one,two", "a,b,c", "x"][seed as usize % 4];
                put_ascii(c, names, DATA + 0x500);
                zero_block(c, DATA + 0x600, 8);
                c.g[A0][0] = sx(block);
                c.g[A1][0] = sx(DATA + 0x500);
                c.g[A2][0] = sx(DATA + 0x600);
                c.pc = 0x003e_2e50;
            },
        );
    }

    driven!(gen_big_file_get_entry_count, ee::s14::f_003e2ee0__generated);
    driven!(hand_big_file_get_entry_count, big_file_get_entry_count);

    /// Header magic selects the count word: C0 FB, BIGF, or neither.
    #[test]
    fn big_file_get_entry_count_matches() {
        check(
            gen_big_file_get_entry_count,
            hand_big_file_get_entry_count,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_2ee0, 0);
                let block = DATA + 0x700;
                zero_block(c, block, 16);
                match seed % 3 {
                    0 => {
                        c.bus.write8(block, 0xc0);
                        c.bus.write8(block + 1, 0xfb);
                        c.bus.write8(block + 4, (seed / 3 % 5) as u8);
                        c.bus.write8(block + 5, 0x11);
                    }
                    1 => {
                        put_ascii(c, "BIGF", block);
                        c.bus.write8(block + 8, (seed / 3 % 7) as u8);
                        c.bus.write8(block + 9, 0x22);
                        c.bus.write8(block + 10, 0x33);
                        c.bus.write8(block + 11, 0x44);
                    }
                    _ => {
                        c.bus.write8(block, (seed / 3 % 251) as u8 + 1);
                    }
                }
                c.g[A0][0] = sx(block);
                c.pc = 0x003e_2ee0;
            },
        );
    }

    driven!(gen_big_file_get_entry_name, ee::s14::f_003e2f70__generated);
    driven!(hand_big_file_get_entry_name, big_file_get_entry_name);

    /// Null lookups clear the dst word; hits copy through strcpy.
    #[test]
    fn big_file_get_entry_name_matches() {
        check(
            gen_big_file_get_entry_name,
            hand_big_file_get_entry_name,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_2f70, 0);
                let block = DATA + 0x700;
                zero_block(c, block, 16);
                put_ascii(c, "entry", DATA + 0x400);
                c.g[A0][0] = sx(block);
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x400) };
                c.g[A2][0] = sx(DATA + 0x500);
                c.pc = 0x003e_2f70;
            },
        );
    }

    driven!(
        gen_big_file_get_entry_name_2,
        ee::s14::f_003e2fc8__generated
    );
    driven!(hand_big_file_get_entry_name_2, big_file_get_entry_name_2);

    /// Same through the EAFile finder.
    #[test]
    fn big_file_get_entry_name_2_matches() {
        check(
            gen_big_file_get_entry_name_2,
            hand_big_file_get_entry_name_2,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_2fc8, 0);
                let block = DATA + 0x700;
                zero_block(c, block, 16);
                put_ascii(c, "entry", DATA + 0x400);
                c.g[A0][0] = sx(block);
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x400) };
                c.g[A2][0] = sx(DATA + 0x500);
                c.pc = 0x003e_2fc8;
            },
        );
    }

    driven!(gen_ea_file_platform_init, ee::s14::f_003e3020__generated);
    driven!(hand_ea_file_platform_init, ea_file_platform_init);

    /// Cold boot skips the flag-3 write; warm boot takes it.
    #[test]
    fn ea_file_platform_init_matches() {
        check(
            gen_ea_file_platform_init,
            hand_ea_file_platform_init,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_3020, 0);
                c.bus.write32(0x0045_0c48, (seed % 5) as u32);
                c.bus.write32(0x0045_0c44, 0);
                c.g[A1][0] = (seed / 5 % 2) as u64;
                c.pc = 0x003e_3020;
            },
        );
    }

    driven!(gen_ea_file_platform_write, ee::s14::f_003e3478__generated);
    driven!(hand_ea_file_platform_write, ea_file_platform_write);

    /// Host write pair; sizes drive the completion path.
    #[test]
    fn ea_file_platform_write_matches() {
        check(
            gen_ea_file_platform_write,
            hand_ea_file_platform_write,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_3478, 0);
                zero_block(c, DATA + 0x500, 8);
                c.g[A0][0] = 0x1_0000 + (seed % 3) as u64;
                c.g[A1][0] = sx(DATA + 0x500);
                c.g[A2][0] = [0u32, 1, 0x1000][seed as usize / 3 % 3] as u64;
                c.g[A3][0] = (seed / 9 % 2) as u64;
                c.pc = 0x003e_3478;
            },
        );
    }

    driven!(gen_ea_file_platform_get_lsn, ee::s14::f_003e3538__generated);
    driven!(hand_ea_file_platform_get_lsn, ea_file_platform_get_lsn);

    /// Host-drive bit selects the CdGetEntry arm or the zero store.
    #[test]
    fn ea_file_platform_get_lsn_matches() {
        check(
            gen_ea_file_platform_get_lsn,
            hand_ea_file_platform_get_lsn,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_3538, 0);
                c.bus.write32(0x0045_0c5c, (seed % 7) as u32);
                c.bus.write32(0x0051_9c70, 0x100 + seed);
                c.g[A0][0] = (((seed % 2) << 24 | 0x34_5678) & 0xffff_ffff) as u64;
                c.pc = 0x003e_3538;
            },
        );
    }

    driven!(gen_ea_file_alarm_handler, ee::s14::f_003e3588__generated);
    driven!(hand_ea_file_alarm_handler, ea_file_alarm_handler);

    /// Host alarm ack through the syscall fake.
    #[test]
    fn ea_file_alarm_handler_matches() {
        check(
            gen_ea_file_alarm_handler,
            hand_ea_file_alarm_handler,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_3588, 0);
                c.g[A2][0] = (seed % 5) as u64;
                c.pc = 0x003e_3588;
            },
        );
    }

    driven!(gen_ea_file_delay_ticks, ee::s14::f_003e35b0__generated);
    driven!(hand_ea_file_delay_ticks, ea_file_delay_ticks);

    /// Timer-primitive wait; tick counts drive the config word.
    #[test]
    fn ea_file_delay_ticks_matches() {
        check(
            gen_ea_file_delay_ticks,
            hand_ea_file_delay_ticks,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_35b0, 0);
                c.g[A0][0] = [0u32, 1, 0x1e0, 0x1_0000][seed as usize % 4] as u64;
                c.pc = 0x003e_35b0;
            },
        );
    }

    driven!(gen_ea_file_hash_name, ee::s14::f_003e36f8__generated);
    driven!(hand_ea_file_hash_name, ea_file_hash_name);

    /// Empty names exit at once; longer ones fold every byte.
    #[test]
    fn ea_file_hash_name_matches() {
        check(
            gen_ea_file_hash_name,
            hand_ea_file_hash_name,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_36f8, 0);
                let names = ["", "a", "entry", "longer/name/here"];
                put_ascii(c, names[seed as usize % 4], DATA + 0x400);
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = (seed / 4 % 251) as u64;
                c.pc = 0x003e_36f8;
            },
        );
    }

    driven!(gen_ea_file_cd_signal, ee::s14::f_003e3ad8__generated);
    driven!(hand_ea_file_cd_signal, ea_file_cd_signal);

    /// Host signal ack through the syscall fake.
    #[test]
    fn ea_file_cd_signal_matches() {
        check(
            gen_ea_file_cd_signal,
            hand_ea_file_cd_signal,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_3ad8, 0);
                c.bus.write32(0x0051_9c4c, seed);
                c.pc = 0x003e_3ad8;
            },
        );
    }

    driven!(gen_ea_file_resolve_path, ee::s14::f_003e3098__generated);
    driven!(hand_ea_file_resolve_path, ea_file_resolve_path);

    /// Colon split, dot handling and slash normalization across seeds.
    #[test]
    fn ea_file_resolve_path_matches() {
        check(
            gen_ea_file_resolve_path,
            hand_ea_file_resolve_path,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_3098, 0);
                let paths = [
                    "host0:dir/file",
                    "host0:dir\\sub\\file",
                    "host0:..\\up",
                    "plainname",
                ];
                put_ascii(c, paths[seed as usize % 4], DATA + 0x400);
                zero_block(c, DATA + 0x500, 8);
                c.bus.write32(0x0045_0c44, (seed / 4 % 2) as u32);
                put_ascii(c, "host0:", DATA + 0x600);
                c.bus.write32(0x0045_0c50, DATA + 0x600);
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.g[A2][0] = if seed / 8 % 2 == 0 {
                    0
                } else {
                    sx(DATA + 0x700)
                };
                c.pc = 0x003e_3098;
            },
        );
    }

    driven!(
        gen_ea_file_cd_build_hash_table,
        ee::s14::f_003e39a8__generated
    );
    driven!(
        hand_ea_file_cd_build_hash_table,
        ea_file_cd_build_hash_table
    );

    /// Empty tables skip the walk; small tables hash, copy and sort.
    #[test]
    fn ea_file_cd_build_hash_table_matches() {
        crate::difftest::check_lohi(
            gen_ea_file_cd_build_hash_table,
            hand_ea_file_cd_build_hash_table,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_39a8, 0);
                let n = seed % 3;
                let src = DATA + 0x700;
                zero_block(c, src, 64);
                zero_block(c, DATA + 0x500, 64);
                for i in 0..n {
                    put_ascii(c, ["alpha", "betaa", "gamma!"][i as usize], src + i * 0x108);
                    c.bus.write32(src + i * 0x108 + 0x100, 0x10 + i);
                    c.bus.write32(src + i * 0x108 + 0x104, 0x20 + i);
                }
                c.bus.write32(0x0045_0c58 + 4, 0);
                c.g[A0][0] = sx(src);
                c.g[A1][0] = sx(DATA + 0x500);
                c.g[A2][0] = n as u64;
                c.g[A3][0] = (seed / 3 % 251) as u64;
                c.pc = 0x003e_39a8;
            },
        );
    }
}
