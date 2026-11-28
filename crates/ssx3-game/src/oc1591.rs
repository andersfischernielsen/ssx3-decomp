//! Staging module oc1591 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1591`.
//!
//! The eight functions are the IPU / bitstream layer of libmpeg: the IPU
//! command and busy wait, the bit reader (`ShowBits`, `FlushBits`, `GetBits`),
//! the IPU macroblock decode (`IpuVdec`), and the two header parsers that
//! read the start codes (`GetHeader`, `ExtensionAndUserData`).

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0040_6df0, libmpeg_ipu_command, "libmpeg_IpuCommand"),
    (0x0040_6e20, libmpeg_ipu_wait_busy, "libmpeg_IpuWaitBusy"),
    (0x0040_6f78, libmpeg_ipu_vdec, "libmpeg_IpuVdec"),
    (0x0040_70f0, libmpeg_show_bits, "libmpeg_ShowBits"),
    (0x0040_71f8, libmpeg_flush_bits, "libmpeg_FlushBits"),
    (0x0040_72e8, libmpeg_get_bits, "libmpeg_GetBits"),
    (0x0040_7520, libmpeg_get_header, "libmpeg_GetHeader"),
    (
        0x0040_76f8,
        libmpeg_extension_and_user_data,
        "libmpeg_ExtensionAndUserData",
    ),
];

// IPU registers as the decoder sees them: command (write), control (busy and
// error bits) and the bit-stream reads.
const IPU_CMD: u32 = 0x1000_2000;
const IPU_CTRL: u32 = 0x1000_2010;
/// `0x80000000` sign-extended: the busy bit (31) of the control register,
/// compared under the mask `0xffffffff80004000` (busy and error bits).
const IPU_BUSY_SIGNED: u64 = 0xffff_ffff_8000_0000;

/// Direct-call targets of this module (address, readable alias in the shard).
const CALLEE_CALLBACK: Callee = Callee {
    addr: 0x0040_2c80,
    f: ee::s15::f_00402c80,
};
const CALLEE_DISPATCH_CALLBACK: Callee = Callee {
    addr: 0x0040_2c30,
    f: ee::s15::f_00402c30,
};
const CALLEE_SEQUENCE_HEADER: Callee = Callee {
    addr: 0x0040_3620,
    f: ee::s15::f_00403620,
};
const CALLEE_IPU_WAIT_CMD: Callee = Callee {
    addr: 0x0040_6ec8,
    f: ee::s15::f_00406ec8,
};
const CALLEE_SHOW_BITS: Callee = Callee {
    addr: 0x0040_70f0,
    f: ee::s15::f_004070f0,
};
const CALLEE_FLUSH_BITS: Callee = Callee {
    addr: 0x0040_71f8,
    f: ee::s15::f_004071f8,
};
const CALLEE_GET_BITS: Callee = Callee {
    addr: 0x0040_72e8,
    f: ee::s15::f_004072e8,
};
const CALLEE_NEXT_START_CODE: Callee = Callee {
    addr: 0x0040_7438,
    f: ee::s15::f_00407438,
};
const CALLEE_PICTURE_HEADER: Callee = Callee {
    addr: 0x0040_7630,
    f: ee::s15::f_00407630,
};
const CALLEE_GROUP_OF_PICTURES: Callee = Callee {
    addr: 0x0040_7ad0,
    f: ee::s15::f_00407ad0,
};

/// `lw reg, off(reg)` (sign-extended 32-bit load).
macro_rules! lw_at {
    ($c:ident, $r:ident, $off:expr) => {{
        let a = ($c.g[$r][0] as u32).wrapping_add($off);
        $c.lw(a)
    }};
}

/// `sw reg, off(reg)`.
macro_rules! sw_at {
    ($c:ident, $r:ident, $off:expr, $v:expr) => {{
        let v = $v;
        let a = ($c.g[$r][0] as u32).wrapping_add($off);
        $c.sw(a, v);
    }};
}

/// `sd reg, off(sp)`.
macro_rules! sd_sp {
    ($c:ident, $off:expr, $reg:ident) => {{
        let v = $c.g[$reg][0];
        let a = sp_slot($c, $off);
        $c.sd(a, v);
    }};
}

/// `ld reg, off(sp)`.
macro_rules! ld_sp {
    ($c:ident, $off:expr, $reg:ident) => {{
        let a = sp_slot($c, $off);
        $c.g[$reg][0] = $c.ld(a);
    }};
}

/// The address `off(sp)`.
#[inline]
fn sp_slot(c: &Cpu, off: u32) -> u32 {
    sp(c).wrapping_add(off)
}

/// `void libmpeg_IpuCommand(MpegDec* d, u32 cmd)` (0x406df0): writes `cmd` to
/// the IPU command register, then stores the decoder's word for the command
/// type (`cmd >> 28`, from the table at 0x4532c0) at `d+0x818`.
pub fn libmpeg_ipu_command(c: &mut Cpu) {
    c.cycles += 11;
    c.g[V0][0] = 0x1000_0000;
    c.g[A2][0] = sx((c.g[A1][0] as u32) >> 28);
    c.g[V0][0] |= 0x2000;
    c.g[V1][0] = 0x0045_0000;
    let cmd_reg = c.g[V0][0] as u32;
    c.sw(cmd_reg, c.g[A1][0]);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x32c0));
    c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
    let table_word = c.lw(c.g[A2][0] as u32);
    c.g[V0][0] = table_word;
    sw_at!(c, A0, 0x818, c.g[V0][0]);
    ret(c);
}

/// `void libmpeg_IpuWaitBusy(MpegDec* d)` (0x406e20): waits while the IPU is
/// busy (control bit 31 set, error bit 14 clear). Every 5001 polls it calls the
/// host callback (`d+0x858`). Returns with the decoder pointer in `s2`.
pub fn libmpeg_ipu_wait_busy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x50));
        c.g[V0][0] = 0x1000_0000;
        sd_sp!(c, 0x20, S2);
        c.g[V0][0] |= 0x2010;
        sd_sp!(c, 0x40, RA);
        c.g[A1][0] = IPU_BUSY_SIGNED;
        sd_sp!(c, 0x30, S3);
        c.g[A1][0] |= 0x4000;
        sd_sp!(c, 0x10, S1);
        c.g[S2][0] = c.g[A0][0];
        sd_sp!(c, 0x0, S0);
        c.g[A0][0] = 0;
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        c.g[V0][0] = IPU_BUSY_SIGNED;
        c.g[V1][0] &= c.g[A1][0];
        let not_busy = c.g[V1][0] != c.g[V0][0];
        c.g[RA][0] = {
            let a = sp_slot(c, 0x40);
            c.ld(a)
        };
        if !not_busy {
            c.cycles += 7;
            c.g[S1][0] = 0x1000_0000;
            c.g[S0][0] = IPU_BUSY_SIGNED;
            c.g[S1][0] |= 0x2010;
            c.g[S0][0] |= 0x4000;
            c.g[S3][0] = IPU_BUSY_SIGNED;
            c.g[V0][0] = c.g[A0][0];
            loop {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V0][0] as i64) < 5001) as u64;
                let below = c.g[V0][0] != 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                if !below {
                    c.cycles += 3;
                    c.g[A0][0] = lw_at!(c, S2, 0x858);
                    CALLEE_CALLBACK.call(c, 0x0040_6e94)?;
                    c.g[A0][0] = 0;
                }
                c.cycles += 4;
                c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                c.g[V0][0] &= c.g[S0][0];
                let still_busy = c.g[V0][0] == c.g[S3][0];
                c.g[V0][0] = c.g[A0][0];
                if still_busy {
                    if event_due(c) {
                        deliver_event_here(c, 0x0040_6e80);
                    }
                } else {
                    c.cycles += 1;
                    let a = sp_slot(c, 0x40);
                    c.g[RA][0] = c.ld(a);
                    break;
                }
            }
        }
        c.cycles += 6;
        ld_sp!(c, 0x30, S3);
        ld_sp!(c, 0x20, S2);
        ld_sp!(c, 0x10, S1);
        ld_sp!(c, 0x0, S0);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// `u64 libmpeg_ShowBits(MpegDec* d, int n)` (0x4070f0): the next `n` bits of
/// the stream, not consumed. Waits for the IPU command word first (the
/// decoder's pending word at `d+0x818`, or the bit position at `d+0x83c`), and
/// returns the bits shifted down to the top `n` of the window.
pub fn libmpeg_show_bits(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
        sd_sp!(c, 0x20, S2);
        sd_sp!(c, 0x0, S0);
        sd_sp!(c, 0x30, RA);
        c.g[S0][0] = c.g[A0][0];
        sd_sp!(c, 0x10, S1);
        c.g[V0][0] = lw_at!(c, S0, 0x818);
        let pending = c.g[V0][0] != 0;
        c.g[S2][0] = c.g[A1][0];
        if !pending {
            c.cycles += 4;
            c.g[V0][0] = lw_at!(c, S0, 0x83c);
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S2][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.g[V1][0] = lw_at!(c, S0, 0x838);
                show_bits_tail(c);
                return Ok(());
            }
        }
        // The IPU wait: stalled while busy, polled to the callback.
        c.cycles += 9;
        c.g[V0][0] = 0x1000_0000;
        c.g[A0][0] = IPU_BUSY_SIGNED;
        c.g[V0][0] |= 0x2010;
        c.g[A0][0] |= 0x4000;
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        c.g[V0][0] = IPU_BUSY_SIGNED;
        c.g[V1][0] &= c.g[A0][0];
        let not_busy = c.g[V1][0] != c.g[V0][0];
        c.g[A2][0] = 0;
        if not_busy {
            c.cycles += 3;
            c.g[S1][0] = 0x0045_0000;
            c.g[V0][0] = 0x1000_0000;
            c.g[V1][0] = 0x4000_0000;
        } else {
            c.cycles += 2;
            c.g[S1][0] = 0x0045_0000;
            loop {
                c.cycles += 4;
                c.g[V0][0] = c.g[A2][0];
                c.g[V0][0] = ((c.g[V0][0] as i64) < 5001) as u64;
                let below = c.g[V0][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                if !below {
                    c.cycles += 3;
                    c.g[A0][0] = lw_at!(c, S0, 0x858);
                    CALLEE_CALLBACK.call(c, 0x0040_7168)?;
                    c.g[A2][0] = 0;
                }
                c.cycles += 9;
                c.g[V1][0] = 0x1000_0000;
                c.g[A0][0] = IPU_BUSY_SIGNED;
                c.g[V1][0] |= 0x2010;
                c.g[A0][0] |= 0x4000;
                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                c.g[A1][0] = IPU_BUSY_SIGNED;
                c.g[V0][0] &= c.g[A0][0];
                let still_busy = c.g[V0][0] == c.g[A1][0];
                c.g[V0][0] = 0x1000_0000;
                if still_busy {
                    if event_due(c) {
                        deliver_event_here(c, 0x0040_7150);
                    }
                } else {
                    c.cycles += 2;
                    c.g[V1][0] = 0x4000_0000;
                    break;
                }
            }
        }
        // Issue the read command, wait for the IPU, store the bit position.
        c.cycles += 13;
        c.g[V0][0] |= 0x2000;
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x32c0));
        c.sw(c.g[V0][0] as u32, c.g[V1][0]);
        c.g[A0][0] = c.g[S0][0];
        c.g[V0][0] = lw_at!(c, A1, 0x10);
        sw_at!(c, S0, 0x818, c.g[V0][0]);
        CALLEE_IPU_WAIT_CMD.call(c, 0x0040_71c0)?;
        c.g[V0][0] = c.g[V0][0] << 32;
        c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
        c.g[V1][0] = 0x20;
        sw_at!(c, S0, 0x838, c.g[V0][0]);
        sw_at!(c, S0, 0x83c, c.g[V1][0]);
        c.g[V1][0] = lw_at!(c, S0, 0x838);
        show_bits_tail(c);
        Ok(())
    })
}

/// The common tail of `ShowBits`: the shifted result from `v1`.
fn show_bits_tail(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = sx(0u32.wrapping_sub(c.g[S2][0] as u32));
    ld_sp!(c, 0x30, RA);
    ld_sp!(c, 0x20, S2);
    c.g[V0][0] = sx((c.g[V1][0] as u32) >> ((c.g[V0][0] as u32) & 31));
    ld_sp!(c, 0x10, S1);
    ld_sp!(c, 0x0, S0);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
    c.pc = t;
}

/// `void libmpeg_FlushBits(MpegDec* d, int n)` (0x4071f8): consumes `n` bits:
/// waits for the IPU, issues the flush command `n | 0x40000000` and waits for
/// the decoder command word of the next bit window.
pub fn libmpeg_flush_bits(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
        c.g[V0][0] = 0x1000_0000;
        sd_sp!(c, 0x10, S1);
        c.g[V0][0] |= 0x2010;
        sd_sp!(c, 0x0, S0);
        c.g[A2][0] = IPU_BUSY_SIGNED;
        sd_sp!(c, 0x30, RA);
        c.g[A2][0] |= 0x4000;
        sd_sp!(c, 0x20, S2);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        c.g[A3][0] = 0;
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        c.g[V0][0] = IPU_BUSY_SIGNED;
        c.g[V1][0] &= c.g[A2][0];
        let not_busy = c.g[V1][0] != c.g[V0][0];
        c.g[S2][0] = 0x0045_0000;
        if not_busy {
            c.cycles += 2;
            c.g[V0][0] = 0x4000_0000;
            c.g[V1][0] = 0x1000_0000;
        } else {
            c.cycles += 1;
            loop {
                c.cycles += 4;
                c.g[V0][0] = c.g[A3][0];
                c.g[V0][0] = ((c.g[V0][0] as i64) < 5001) as u64;
                let below = c.g[V0][0] != 0;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                if !below {
                    c.cycles += 3;
                    c.g[A0][0] = lw_at!(c, S0, 0x858);
                    CALLEE_CALLBACK.call(c, 0x0040_7258)?;
                    c.g[A3][0] = 0;
                }
                c.cycles += 9;
                c.g[V1][0] = 0x1000_0000;
                c.g[A0][0] = IPU_BUSY_SIGNED;
                c.g[V1][0] |= 0x2010;
                c.g[A0][0] |= 0x4000;
                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                c.g[A1][0] = IPU_BUSY_SIGNED;
                c.g[V0][0] &= c.g[A0][0];
                let still_busy = c.g[V0][0] == c.g[A1][0];
                c.g[V0][0] = 0x4000_0000;
                if still_busy {
                    if event_due(c) {
                        deliver_event_here(c, 0x0040_7240);
                    }
                } else {
                    c.cycles += 2;
                    c.g[V1][0] = 0x1000_0000;
                    break;
                }
            }
        }
        flush_bits_tail(c)
    })
}

/// The tail of `FlushBits` (after the IPU wait): the flush command, the
/// command word of the window, and the bit position store.
fn flush_bits_tail(c: &mut Cpu) -> Flow {
    c.cycles += 22;
    c.g[V0][0] = c.g[S1][0] | c.g[V0][0];
    c.g[V1][0] = c.g[V1][0] | 0x2000;
    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
    c.g[A1][0] = sx((c.g[V0][0] as u32) >> 28);
    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x32c0));
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = c.g[S0][0];
    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
    sw_at!(c, S0, 0x818, c.g[V0][0]);
    CALLEE_IPU_WAIT_CMD.call(c, 0x0040_72bc)?;
    c.g[V0][0] = c.g[V0][0] << 32;
    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
    c.g[V1][0] = 0x20;
    sw_at!(c, S0, 0x83c, c.g[V1][0]);
    sw_at!(c, S0, 0x838, c.g[V0][0]);
    ld_sp!(c, 0x30, RA);
    ld_sp!(c, 0x20, S2);
    ld_sp!(c, 0x10, S1);
    ld_sp!(c, 0x0, S0);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
    c.pc = t;
    Ok(())
}

/// `u64 libmpeg_GetBits(MpegDec* d, int n)` (0x4072e8): reads `n` bits and
/// consumes them. Waits for the IPU first; then, when the stream word needs a
/// refill (`d+0x83c` below `n`), issues the refill command and waits again.
pub fn libmpeg_get_bits(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x50));
        c.g[V0][0] = 0x1000_0000;
        sd_sp!(c, 0x20, S2);
        c.g[V0][0] |= 0x2010;
        sd_sp!(c, 0x10, S1);
        c.g[A2][0] = IPU_BUSY_SIGNED;
        sd_sp!(c, 0x40, RA);
        c.g[A2][0] |= 0x4000;
        sd_sp!(c, 0x30, S3);
        c.g[S1][0] = c.g[A0][0];
        sd_sp!(c, 0x0, S0);
        c.g[S2][0] = c.g[A1][0];
        c.g[A3][0] = 0;
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        c.g[V0][0] = IPU_BUSY_SIGNED;
        c.g[V1][0] &= c.g[A2][0];
        let not_busy = c.g[V1][0] != c.g[V0][0];
        c.g[S3][0] = 0x0045_0000;
        if not_busy {
            c.cycles += 1;
            c.g[V0][0] = lw_at!(c, S1, 0x818);
        } else {
            c.cycles += 2;
            c.g[V0][0] = c.g[A3][0];
            loop {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V0][0] as i64) < 5001) as u64;
                let below = c.g[V0][0] != 0;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                if !below {
                    c.cycles += 3;
                    c.g[A0][0] = lw_at!(c, S1, 0x858);
                    CALLEE_CALLBACK.call(c, 0x0040_734c)?;
                    c.g[A3][0] = 0;
                }
                c.cycles += 9;
                c.g[V1][0] = 0x1000_0000;
                c.g[A0][0] = IPU_BUSY_SIGNED;
                c.g[V1][0] |= 0x2010;
                c.g[A0][0] |= 0x4000;
                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                c.g[A1][0] = IPU_BUSY_SIGNED;
                c.g[V0][0] &= c.g[A0][0];
                let still_busy = c.g[V0][0] == c.g[A1][0];
                c.g[V0][0] = c.g[A3][0];
                if still_busy {
                    if event_due(c) {
                        deliver_event_here(c, 0x0040_7338);
                    }
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = lw_at!(c, S1, 0x818);
                    break;
                }
            }
        }
        // Refill check: the stream word at +0x838 is refilled when the bit
        // position at +0x83c is below `n`.
        c.cycles += 2;
        let nonzero = c.g[V0][0] != 0;
        c.g[V0][0] = 0x1000_0000;
        let mut refill = nonzero;
        if !nonzero {
            c.cycles += 4;
            c.g[V0][0] = lw_at!(c, S1, 0x83c);
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S2][0] as i64)) as u64;
            let not_below = c.g[V0][0] == 0;
            c.g[V0][0] = 0x1000_0000;
            refill = !not_below;
        }
        if refill {
            c.cycles += 11;
            c.g[V1][0] = 0x4000_0000;
            c.g[V0][0] |= 0x2000;
            c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x32c0));
            c.sw(c.g[V0][0] as u32, c.g[V1][0]);
            c.g[A0][0] = c.g[S1][0];
            c.g[V0][0] = lw_at!(c, A1, 0x10);
            sw_at!(c, S1, 0x818, c.g[V0][0]);
            CALLEE_IPU_WAIT_CMD.call(c, 0x0040_73b8)?;
            c.g[V0][0] = c.g[V0][0] << 32;
            c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
            sw_at!(c, S1, 0x838, c.g[V0][0]);
        }
        // The read itself: the bits from the window at +0x838.
        c.cycles += 29;
        c.g[A1][0] = 0x20;
        c.g[A0][0] = 0x4000_0000;
        c.g[V0][0] = 0x1000_0000;
        c.g[A0][0] = c.g[S2][0] | c.g[A0][0];
        sw_at!(c, S1, 0x83c, c.g[A1][0]);
        c.g[V0][0] |= 0x2000;
        c.g[S0][0] = lw_at!(c, S1, 0x838);
        c.g[V1][0] = sx((c.g[A0][0] as u32) >> 28);
        c.sw(c.g[V0][0] as u32, c.g[A0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x32c0));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[S2][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[S0][0] = sx((c.g[S0][0] as u32) >> ((c.g[A1][0] as u32) & 31));
        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
        c.g[A0][0] = c.g[S1][0];
        sw_at!(c, S1, 0x818, c.g[V0][0]);
        CALLEE_IPU_WAIT_CMD.call(c, 0x0040_740c)?;
        c.g[V0][0] = c.g[V0][0] << 32;
        c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
        ld_sp!(c, 0x40, RA);
        sw_at!(c, S1, 0x838, c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        ld_sp!(c, 0x30, S3);
        ld_sp!(c, 0x20, S2);
        ld_sp!(c, 0x10, S1);
        ld_sp!(c, 0x0, S0);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// `void libmpeg_IpuVdec(MpegDec* d, int type)` (0x406f78): starts the IPU
/// variable-length decode of one macroblock for the picture type `type`
/// (`a1 << 26`, with the picture-type bit) and waits for it to finish, polling
/// to the callback. The result (the command word) is stored in `d+0x838`,
/// `d+0x83c`, `d+0x11c` and `d+0x818`.
pub fn libmpeg_ipu_vdec(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x50));
        c.g[V0][0] = 0x1000_0000;
        sd_sp!(c, 0x20, S2);
        c.g[V0][0] |= 0x2010;
        sd_sp!(c, 0x10, S1);
        c.g[V1][0] = IPU_BUSY_SIGNED;
        sd_sp!(c, 0x40, RA);
        c.g[V1][0] |= 0x4000;
        sd_sp!(c, 0x30, S3);
        c.g[S1][0] = c.g[A0][0];
        sd_sp!(c, 0x0, S0);
        c.g[S2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A2][0] = c.lw(c.g[V0][0] as u32);
        c.g[V0][0] = IPU_BUSY_SIGNED;
        c.g[A2][0] &= c.g[V1][0];
        let not_busy = c.g[A2][0] != c.g[V0][0];
        c.g[S0][0] = sx((c.g[A1][0] as u32) << 26);
        if not_busy {
            c.cycles += 3;
            c.g[S3][0] = 0x0045_0000;
            c.g[V1][0] = 0x3000_0000;
            c.g[A0][0] = 0x1000_0000;
        } else {
            c.cycles += 2;
            c.g[S3][0] = 0x0045_0000;
            loop {
                c.cycles += 4;
                c.g[V0][0] = c.g[A3][0];
                c.g[V0][0] = ((c.g[V0][0] as i64) < 5001) as u64;
                let below = c.g[V0][0] != 0;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                if !below {
                    c.cycles += 3;
                    c.g[A0][0] = lw_at!(c, S1, 0x858);
                    CALLEE_CALLBACK.call(c, 0x0040_6fe0)?;
                    c.g[A3][0] = 0;
                }
                c.cycles += 9;
                c.g[V1][0] = 0x1000_0000;
                c.g[A0][0] = IPU_BUSY_SIGNED;
                c.g[V1][0] |= 0x2010;
                c.g[A0][0] |= 0x4000;
                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                c.g[A1][0] = IPU_BUSY_SIGNED;
                c.g[V0][0] &= c.g[A0][0];
                let still_busy = c.g[V0][0] == c.g[A1][0];
                c.g[V1][0] = 0x3000_0000;
                if still_busy {
                    if event_due(c) {
                        deliver_event_here(c, 0x0040_6fc8);
                    }
                } else {
                    c.cycles += 2;
                    c.g[A0][0] = 0x1000_0000;
                    break;
                }
            }
        }
        // Store the result word, start the command window and wait for it.
        c.cycles += 11;
        c.g[V1][0] = c.g[S0][0] | c.g[V1][0];
        c.g[A0][0] |= 0x2000;
        c.g[V0][0] = sx((((c.g[V1][0] as u32) as i32) >> 28) as u32);
        c.sw(c.g[A0][0] as u32, c.g[V1][0]);
        c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x32c0));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[A2][0] = c.ld(c.g[A0][0] as u32);
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        let ready = (c.g[A2][0] as i64) >= 0;
        sw_at!(c, S1, 0x818, c.g[V1][0]);
        if !ready {
            c.cycles += 4;
            c.g[S0][0] = 0x1000_0000;
            c.g[S0][0] |= 0x2000;
            c.g[V0][0] = c.g[S2][0];
            loop {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V0][0] as i64) < 5001) as u64;
                let below = c.g[V0][0] != 0;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                if !below {
                    c.cycles += 3;
                    c.g[A0][0] = lw_at!(c, S1, 0x858);
                    c.g[S2][0] = 0;
                    CALLEE_CALLBACK.call(c, 0x0040_7070)?;
                }
                c.cycles += 3;
                c.g[A2][0] = c.ld(c.g[S0][0] as u32);
                let negative = (c.g[A2][0] as i64) < 0;
                c.g[V0][0] = c.g[S2][0];
                if negative {
                    if event_due(c) {
                        deliver_event_here(c, 0x0040_7058);
                    }
                } else {
                    break;
                }
            }
        }
        // The bit count of the result: the top bits of the word at 0x10002020
        // (or the negated one at 0x10002030), stored at +0x838 and +0x83c.
        c.cycles += 9;
        c.g[V0][0] = 0x1000_0000;
        c.g[A0][0] = 0x1000_0000;
        c.g[A0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x2030));
        c.g[V0][0] |= 0x2020;
        c.g[V0][0] = c.lw(c.g[V0][0] as u32);
        c.g[V1][0] = c.g[A0][0] << 32;
        c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
        let a0_nonneg = (c.g[A0][0] as i64) >= 0;
        sw_at!(c, S1, 0x838, c.g[V1][0]);
        if !a0_nonneg {
            c.cycles += 4;
            c.g[V0][0] &= 0x1f;
            c.g[V0][0] = sx(0u32.wrapping_sub(c.g[V0][0] as u32));
            c.g[V0][0] &= 0x1f;
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0x20;
        }
        c.cycles += 15;
        sw_at!(c, S1, 0x83c, c.g[V0][0]);
        c.g[V1][0] = c.g[A2][0] << 32;
        c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
        c.g[V0][0] = c.g[A2][0] & 0xffff;
        c.g[V1][0] = (c.g[V1][0] < 0x1u64) as u64;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
        sw_at!(c, S1, 0x11c, c.g[V1][0]);
        c.g[V0][0] = sx((((c.g[V0][0] as u32) as i32) >> 16) as u32);
        ld_sp!(c, 0x40, RA);
        ld_sp!(c, 0x30, S3);
        ld_sp!(c, 0x20, S2);
        ld_sp!(c, 0x10, S1);
        ld_sp!(c, 0x0, S0);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// `void libmpeg_GetHeader(MpegDec* d)` (0x407520): scans the start codes of
/// the stream and parses each header: the sequence header (0x1b3), the group
/// of pictures (0x1b8), the picture (0x100, which also calls back), and stops at
/// the sequence end (0x1b7). Other codes are skipped.
pub fn libmpeg_get_header(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0xa0));
        sd_sp!(c, 0x80, S6);
        sd_sp!(c, 0x70, S5);
        c.g[S6][0] = 5;
        sd_sp!(c, 0x60, S4);
        c.g[S5][0] = 0x1b3;
        sd_sp!(c, 0x50, S3);
        c.g[S4][0] = 0x100;
        sd_sp!(c, 0x40, S2);
        c.g[S3][0] = 0x1b7;
        sd_sp!(c, 0x30, S1);
        c.g[S2][0] = 0x1b8;
        sd_sp!(c, 0x20, S0);
        c.g[S1][0] = u64::MAX;
        sd_sp!(c, 0x90, RA);
        c.g[S0][0] = c.g[A0][0];
        loop {
            c.cycles += 8;
            c.g[A0][0] = c.g[S0][0];
            CALLEE_NEXT_START_CODE.call(c, 0x0040_7568)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x20;
            CALLEE_GET_BITS.call(c, 0x0040_7574)?;
            let code = c.g[V0][0];
            c.g[V1][0] = code;
            let sequence = c.g[V1][0] == c.g[S5][0];
            c.g[V0][0] = (c.g[V1][0] < 0x1b4) as u64;
            if sequence {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                CALLEE_SEQUENCE_HEADER.call(c, 0x0040_75b8)?;
                event_or_continue(c, 0x0040_7560);
                continue;
            }
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                if c.g[V1][0] == c.g[S4][0] {
                    // A picture: its header, the callback, then the exit.
                    c.cycles += 13;
                    c.g[A0][0] = c.g[S0][0];
                    CALLEE_PICTURE_HEADER.call(c, 0x0040_75d8)?;
                    c.g[A0][0] = lw_at!(c, S0, 0x858);
                    c.g[A1][0] = c.g[SP][0];
                    let s = sp(c);
                    c.sw(s, c.g[S6][0]);
                    sd_sp!(c, 0x10, S1);
                    sd_sp!(c, 0x8, S1);
                    CALLEE_DISPATCH_CALLBACK.call(c, 0x0040_75f0)?;
                    ld_sp!(c, 0x10, V0);
                    ld_sp!(c, 0x8, V1);
                    let a830 = (c.g[S0][0] as u32).wrapping_add(0x830);
                    c.sd(a830, c.g[V0][0]);
                    let a828 = (c.g[S0][0] as u32).wrapping_add(0x828);
                    c.sd(a828, c.g[V1][0]);
                    c.g[V0][0] = lw_at!(c, S0, 0x150);
                    get_header_exit(c);
                    return Ok(());
                }
                c.cycles += 2;
                event_or_continue(c, 0x0040_7560);
                continue;
            }
            // V0 == 0: the stop code, the group of pictures, or an unknown one.
            c.cycles += 2;
            let stop = c.g[V1][0] == c.g[S3][0];
            c.g[V0][0] = 0;
            if stop {
                get_header_exit(c);
                return Ok(());
            }
            c.cycles += 2;
            if c.g[V1][0] == c.g[S2][0] {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                CALLEE_GROUP_OF_PICTURES.call(c, 0x0040_75c8)?;
                event_or_continue(c, 0x0040_7560);
                continue;
            }
            c.cycles += 2;
            event_or_continue(c, 0x0040_7560);
        }
    })
}

/// A loop back edge with an event check (the caller continues the loop).
#[inline]
fn event_or_continue(c: &mut Cpu, head: u32) {
    if event_due(c) {
        deliver_event_here(c, head);
    }
}

/// The epilogue of `GetHeader`: restores the saved registers and returns.
fn get_header_exit(c: &mut Cpu) {
    c.cycles += 10;
    ld_sp!(c, 0x90, RA);
    ld_sp!(c, 0x80, S6);
    ld_sp!(c, 0x70, S5);
    ld_sp!(c, 0x60, S4);
    ld_sp!(c, 0x50, S3);
    ld_sp!(c, 0x40, S2);
    ld_sp!(c, 0x30, S1);
    ld_sp!(c, 0x20, S0);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0xa0));
    c.pc = t;
}

/// `void libmpeg_ExtensionAndUserData(MpegDec* d)` (0x4076f8): skips the
/// extension and user-data packets that follow a picture header. A table on
/// the stack (copied from 0x497fe0) maps the extension id to its parser. The
/// function is a resumable state machine: each state is one entry of the
/// `match pc` below, so an event delivered at a loop head resumes there.
pub fn libmpeg_extension_and_user_data(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0040_76f8 => {
                    c.cycles += 37;
                    c.g[SP][0] = sx(sp(c).wrapping_sub(0x80));
                    c.g[V0][0] = 0x0049_0000;
                    sd_sp!(c, 0x30, S0);
                    sd_sp!(c, 0x60, S3);
                    c.g[S0][0] = c.g[A0][0];
                    sd_sp!(c, 0x50, S2);
                    sd_sp!(c, 0x40, S1);
                    c.g[S3][0] = 0x1b2;
                    sd_sp!(c, 0x70, RA);
                    c.g[S1][0] = 0x1b5;
                    c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7fe0));
                    // The 0x2c-byte table, copied to the frame with unaligned moves.
                    let a3 = c.g[A3][0] as u32;
                    let s = sp(c);
                    c.ldl(3, a3.wrapping_add(0x7));
                    c.ldr(3, a3);
                    c.ldl(5, a3.wrapping_add(0xf));
                    c.ldr(5, a3.wrapping_add(0x8));
                    c.ldl(6, a3.wrapping_add(0x17));
                    c.ldr(6, a3.wrapping_add(0x10));
                    c.sdl(3, s.wrapping_add(0x7));
                    c.sdr(3, s);
                    c.sdl(5, s.wrapping_add(0xf));
                    c.sdr(5, s.wrapping_add(0x8));
                    c.sdl(6, s.wrapping_add(0x17));
                    c.sdr(6, s.wrapping_add(0x10));
                    c.ldl(3, a3.wrapping_add(0x1f));
                    c.ldr(3, a3.wrapping_add(0x18));
                    c.ldl(5, a3.wrapping_add(0x27));
                    c.ldr(5, a3.wrapping_add(0x20));
                    c.g[A2][0] = c.lw(a3.wrapping_add(0x28));
                    c.sdl(3, s.wrapping_add(0x1f));
                    c.sdr(3, s.wrapping_add(0x18));
                    c.sdl(5, s.wrapping_add(0x27));
                    c.sdr(5, s.wrapping_add(0x20));
                    sw_at_sp(c, 0x28, A2);
                    c.g[S2][0] = 0xa;
                    CALLEE_NEXT_START_CODE.call(c, 0x0040_7784)?;
                    c.g[A0][0] = c.g[S0][0];
                    pc = 0x0040_77ec;
                }
                0x0040_778c => {
                    c.cycles += 1;
                    pc = 0x0040_7790;
                }
                0x0040_7790 => {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[S1][0] {
                        c.g[A0][0] = c.g[S0][0];
                        pc = 0x0040_77d8;
                    } else {
                        pc = 0x0040_7798;
                    }
                }
                0x0040_7798 => {
                    c.cycles += 16;
                    CALLEE_FLUSH_BITS.call(c, 0x0040_77a0)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 4;
                    CALLEE_GET_BITS.call(c, 0x0040_77ac)?;
                    c.g[V1][0] = (c.g[S2][0] < c.g[V0][0]) as u64;
                    if c.g[V1][0] != 0 {
                        c.g[V0][0] = 0;
                    }
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = sx(sp(c).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, target, 0x0040_77c8)?;
                    c.g[A0][0] = c.g[S0][0];
                    CALLEE_NEXT_START_CODE.call(c, 0x0040_77d0)?;
                    c.g[A0][0] = c.g[S0][0];
                    pc = 0x0040_77ec;
                }
                0x0040_77d8 => {
                    c.cycles += 5;
                    c.g[A1][0] = 0x20;
                    CALLEE_FLUSH_BITS.call(c, 0x0040_77e0)?;
                    c.g[A0][0] = c.g[S0][0];
                    CALLEE_NEXT_START_CODE.call(c, 0x0040_77e8)?;
                    c.g[A0][0] = c.g[S0][0];
                    pc = 0x0040_77ec;
                }
                0x0040_77ec => {
                    c.cycles += 5;
                    c.g[A1][0] = 0x20;
                    CALLEE_SHOW_BITS.call(c, 0x0040_77f4)?;
                    c.g[A0][0] = c.g[S0][0];
                    let is_extension = c.g[V0][0] == c.g[S1][0];
                    c.g[A1][0] = 0x20;
                    if is_extension {
                        if back_edge_unwinds(c, 0x0040_7798) {
                            return Ok(());
                        }
                        pc = 0x0040_7798;
                    } else {
                        pc = 0x0040_7800;
                    }
                }
                0x0040_7800 => {
                    c.cycles += 2;
                    let user_data = c.g[V0][0] == c.g[S3][0];
                    ld_sp!(c, 0x70, RA);
                    if user_data {
                        if back_edge_unwinds(c, 0x0040_7790) {
                            return Ok(());
                        }
                        pc = 0x0040_7790;
                    } else {
                        pc = 0x0040_7808;
                    }
                }
                0x0040_7808 => {
                    c.cycles += 6;
                    ld_sp!(c, 0x60, S3);
                    ld_sp!(c, 0x50, S2);
                    ld_sp!(c, 0x40, S1);
                    ld_sp!(c, 0x30, S0);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx(sp(c).wrapping_add(0x80));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0040_76f8, 0x0040_7820);
                    return Ok(());
                }
            }
        }
    })
}

/// `sw reg, off(sp)` (32-bit store of a register).
#[inline]
fn sw_at_sp(c: &mut Cpu, off: u32, reg: usize) {
    let v = c.g[reg][0];
    let a = sp_slot(c, off);
    c.sw(a, v);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, scramble, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The gate word of this module's fakes, the mailbox of answers (one per
    /// logged call, in order) and the word answered once the mailbox is used up
    /// (so every loop of the callers ends).
    const GATE: u32 = DATA + 0x20_0000;
    const MAGIC: u32 = 0x1591_c0de;
    const TAIL: u32 = GATE + 0x08;
    const MBOX: u32 = GATE + 0x100;
    const MBOX_LEN: usize = 40;

    static PREV_CALLBACK: AtomicUsize = AtomicUsize::new(0);
    static PREV_NEXT_START: AtomicUsize = AtomicUsize::new(0);
    static PREV_FLUSH: AtomicUsize = AtomicUsize::new(0);
    static PREV_GET_BITS: AtomicUsize = AtomicUsize::new(0);
    static PREV_SHOW_BITS: AtomicUsize = AtomicUsize::new(0);
    static PREV_SEQUENCE: AtomicUsize = AtomicUsize::new(0);
    static PREV_GROUP: AtomicUsize = AtomicUsize::new(0);
    static PREV_PICTURE: AtomicUsize = AtomicUsize::new(0);
    static PREV_DISPATCH: AtomicUsize = AtomicUsize::new(0);

    /// Log the call (returning to `ra`) and answer the next mailbox word in v0.
    fn oc1591_answer(c: &mut Cpu, target: u32) {
        let n = c.bus.read32(fakes::LOG_COUNT);
        let result = if (n as usize) < MBOX_LEN {
            c.bus.read32(MBOX + 4 * n)
        } else {
            c.bus.read32(TAIL)
        };
        fakes::fake_call(c, target);
        c.g[V0][0] = result as u64;
    }

    /// The host callback: logs the call and makes the IPU idle and its command
    /// word ready, so every poll loop ends.
    fn oc1591_fake_callback(c: &mut Cpu) {
        if c.bus.read32(GATE) != MAGIC {
            return fakes::fall_back(c, &PREV_CALLBACK, ee::s15::f_00402c80__generated);
        }
        fakes::fake_call(c, 0x0040_2c80);
        c.bus.write32(IPU_CTRL, 0);
        c.bus.write64(IPU_CMD, 5);
    }

    fn oc1591_fake_next_start(c: &mut Cpu) {
        if c.bus.read32(GATE) != MAGIC {
            return fakes::fall_back(c, &PREV_NEXT_START, ee::s15::f_00407438__generated);
        }
        oc1591_answer(c, 0x0040_7438);
    }

    fn oc1591_fake_flush(c: &mut Cpu) {
        if c.bus.read32(GATE) != MAGIC {
            return fakes::fall_back(c, &PREV_FLUSH, ee::s15::f_004071f8__generated);
        }
        oc1591_answer(c, 0x0040_71f8);
    }

    fn oc1591_fake_get_bits(c: &mut Cpu) {
        if c.bus.read32(GATE) != MAGIC {
            return fakes::fall_back(c, &PREV_GET_BITS, ee::s15::f_004072e8__generated);
        }
        oc1591_answer(c, 0x0040_72e8);
    }

    fn oc1591_fake_show_bits(c: &mut Cpu) {
        if c.bus.read32(GATE) != MAGIC {
            return fakes::fall_back(c, &PREV_SHOW_BITS, ee::s15::f_004070f0__generated);
        }
        oc1591_answer(c, 0x0040_70f0);
    }

    fn oc1591_fake_sequence(c: &mut Cpu) {
        if c.bus.read32(GATE) != MAGIC {
            return fakes::fall_back(c, &PREV_SEQUENCE, ee::s15::f_00403620__generated);
        }
        oc1591_answer(c, 0x0040_3620);
    }

    fn oc1591_fake_group(c: &mut Cpu) {
        if c.bus.read32(GATE) != MAGIC {
            return fakes::fall_back(c, &PREV_GROUP, ee::s15::f_00407ad0__generated);
        }
        oc1591_answer(c, 0x0040_7ad0);
    }

    fn oc1591_fake_picture(c: &mut Cpu) {
        if c.bus.read32(GATE) != MAGIC {
            return fakes::fall_back(c, &PREV_PICTURE, ee::s15::f_00407630__generated);
        }
        oc1591_answer(c, 0x0040_7630);
    }

    fn oc1591_fake_dispatch(c: &mut Cpu) {
        if c.bus.read32(GATE) != MAGIC {
            return fakes::fall_back(c, &PREV_DISPATCH, ee::s15::f_00402c30__generated);
        }
        oc1591_answer(c, 0x0040_2c30);
    }

    /// Hook this module's fakes into their override slots (once per process).
    fn oc1591_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00402C80),
            oc1591_fake_callback,
            &PREV_CALLBACK,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00407438),
            oc1591_fake_next_start,
            &PREV_NEXT_START,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004071F8),
            oc1591_fake_flush,
            &PREV_FLUSH,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004072E8),
            oc1591_fake_get_bits,
            &PREV_GET_BITS,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004070F0),
            oc1591_fake_show_bits,
            &PREV_SHOW_BITS,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00403620),
            oc1591_fake_sequence,
            &PREV_SEQUENCE,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00407AD0),
            oc1591_fake_group,
            &PREV_GROUP,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00407630),
            oc1591_fake_picture,
            &PREV_PICTURE,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00402C30),
            oc1591_fake_dispatch,
            &PREV_DISPATCH,
        );
    }

    /// Gate this module's fakes on, with the mailbox words and the tail word.
    fn oc1591_arm(c: &mut Cpu, results: &[u32], tail: u32) {
        c.bus.write32(GATE, MAGIC);
        c.bus.write32(TAIL, tail);
        for (k, v) in results.iter().enumerate() {
            c.bus.write32(MBOX + 4 * k as u32, *v);
        }
    }

    #[test]
    fn libmpeg_ipu_command_matches() {
        check(
            ee::s15::f_00406df0__generated,
            libmpeg_ipu_command,
            200,
            &|c, seed| {
                let mut r = Rng(0x6df0_0000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                // The command type (top nibble) is one of the three cheap ones.
                let nibble = (seed % 3) << 28;
                let low = nibble | (r.next_u32() & 0x0fff_ffff);
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | low as u64;
            },
        );
    }

    #[test]
    fn libmpeg_ipu_wait_busy_matches() {
        oc1591_install_fakes();
        check(
            ee::s15::f_00406e20__generated,
            libmpeg_ipu_wait_busy,
            200,
            &|c, seed| {
                let mut r = Rng(0x6e20_0000 ^ seed as u64);
                scramble(c, &mut r);
                oc1591_arm(c, &[], 0);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x858, DATA + 0x100);
                // Idle (0), busy until the callback (1), or stopped by the error bit (2).
                let ctrl = match seed % 3 {
                    0 => 0,
                    1 => 0x8000_0000,
                    _ => 0x8000_4000,
                };
                c.bus.write32(IPU_CTRL, ctrl);
            },
        );
    }

    #[test]
    fn libmpeg_show_bits_matches() {
        oc1591_install_fakes();
        check(
            ee::s15::f_004070f0__generated,
            libmpeg_show_bits,
            200,
            &|c, seed| {
                let mut r = Rng(0x70f0_0000 ^ seed as u64);
                scramble(c, &mut r);
                oc1591_arm(c, &[], 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 33) as u64;
                c.bus.write32(DATA + 0x818, if seed % 4 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(DATA + 0x83c, seed % 40);
                c.bus.write32(DATA + 0x838, r.next_u32());
                c.bus.write32(DATA + 0x858, DATA + 0x100);
                c.bus.write32(IPU_CTRL, if seed % 5 == 0 { 0x8000_0000 } else { 0 });
                c.bus.write64(IPU_CMD, if seed % 3 == 0 { 0x8000_0000_0000_0000 } else { 5 });
            },
        );
    }

    #[test]
    fn libmpeg_flush_bits_matches() {
        oc1591_install_fakes();
        check(
            ee::s15::f_004071f8__generated,
            libmpeg_flush_bits,
            200,
            &|c, seed| {
                let mut r = Rng(0x71f8_0000 ^ seed as u64);
                scramble(c, &mut r);
                oc1591_arm(c, &[], 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 17) as u64;
                c.bus.write32(DATA + 0x858, DATA + 0x100);
                c.bus.write32(IPU_CTRL, if seed % 2 == 0 { 0x8000_0000 } else { 0 });
                c.bus.write64(IPU_CMD, if seed % 3 == 0 { 0x8000_0000_0000_0000 } else { 5 });
            },
        );
    }

    #[test]
    fn libmpeg_get_bits_matches() {
        oc1591_install_fakes();
        check(
            ee::s15::f_004072e8__generated,
            libmpeg_get_bits,
            200,
            &|c, seed| {
                let mut r = Rng(0x72e8_0000 ^ seed as u64);
                scramble(c, &mut r);
                oc1591_arm(c, &[], 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = 1 + (seed % 16) as u64;
                c.bus.write32(DATA + 0x818, if seed % 3 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(DATA + 0x83c, seed % 24);
                c.bus.write32(DATA + 0x838, r.next_u32());
                c.bus.write32(DATA + 0x858, DATA + 0x100);
                c.bus.write32(IPU_CTRL, if seed % 2 == 0 { 0x8000_0000 } else { 0 });
                c.bus.write64(IPU_CMD, if seed % 5 == 0 { 0x8000_0000_0000_0000 } else { 5 });
            },
        );
    }

    #[test]
    fn libmpeg_ipu_vdec_matches() {
        oc1591_install_fakes();
        check(
            ee::s15::f_00406f78__generated,
            libmpeg_ipu_vdec,
            200,
            &|c, seed| {
                let mut r = Rng(0x6f78_0000 ^ seed as u64);
                scramble(c, &mut r);
                oc1591_arm(c, &[], 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = r.next_u32() as u64;
                c.bus.write32(DATA + 0x858, DATA + 0x100);
                c.bus.write32(IPU_CTRL, if seed % 3 == 1 { 0x8000_0000 } else { 0 });
                c.bus.write64(IPU_CMD, if seed % 4 == 0 { 0x8000_0000_0000_0000 } else { 5 });
                c.bus.write64(0x1000_2030, r.next_u32() as u64);
                c.bus.write32(0x1000_2020, r.next_u32());
            },
        );
    }

    #[test]
    fn libmpeg_get_header_matches() {
        oc1591_install_fakes();
        check(
            ee::s15::f_00407520__generated,
            libmpeg_get_header,
            200,
            &|c, seed| {
                let mut r = Rng(0x7520_0000 ^ seed as u64);
                scramble(c, &mut r);
                // Start codes: sequence (1b3), group (1b8), picture (100), other, end (1b7).
                let codes = [0x1b3, 0x1b8, 0x100, 0x5, 0x1b5, 0x1b7];
                let mut results = [0u32; MBOX_LEN];
                for (k, v) in results.iter_mut().enumerate() {
                    *v = codes[(seed as usize + k * 7) % codes.len()];
                }
                oc1591_arm(c, &results, 0x1b7);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x858, DATA + 0x100);
                c.bus.write32(DATA + 0x150, r.next_u32() & 7);
            },
        );
    }

    #[test]
    fn libmpeg_extension_and_user_data_matches() {
        oc1591_install_fakes();
        check(
            ee::s15::f_004076f8__generated,
            libmpeg_extension_and_user_data,
            200,
            &|c, seed| {
                let mut r = Rng(0x76f8_0000 ^ seed as u64);
                scramble(c, &mut r);
                // The parser table at 0x497fe0: call targets in the fake range.
                for k in 0..11u32 {
                    c.bus.write32(0x0049_7fe0 + 4 * k, fakes::FAKE + 0x40 + 0x10 * k);
                }
                // Peeks: extension (1b5), user data (1b2), other, end (1b7); and
                // the 4-bit extension ids (0..15).
                let peeks = [0x1b5, 0x1b2, 0x1b5, 0x7, 0x1b7];
                let ids = [0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 15];
                let mut results = [0u32; MBOX_LEN];
                for (k, v) in results.iter_mut().enumerate() {
                    *v = if k % 2 == 0 {
                        peeks[(seed as usize + k) % peeks.len()]
                    } else {
                        ids[(seed as usize + k) % ids.len()]
                    };
                }
                oc1591_arm(c, &results, 0x1b7);
                c.g[A0][0] = sx(DATA);
                c.pc = 0x0040_76f8;
            },
        );
    }
}
