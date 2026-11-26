//! Staging module oc1323 (sceio-1 wave, FS path wrappers + TTY ring):
//! hand-written replacements, each with its differential test against the
//! generated body. Moved into the source-tree units by
//! `tools/reorg_modules.py apply oc1323`.

use crate::call::{guest, Callee};
use crate::difftest::fakes::machine;
use crate::difftest::{check, check_paths, DATA};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0042_8fc0, sce_remove, "sceRemove"),
    (0x0042_9190, sce_rmdir, "sceRmdir"),
    (0x0042_9540, sce_del_drv, "sceDelDrv"),
    (0x0042_9eb8, sce_chdir, "sceChdir"),
    (0x0042_4e00, libkernl_tty_ring_init, "libkernl_TtyRingInit"),
    (0x0042_4e28, libkernl_tty_ring_push, "libkernl_TtyRingPush"),
    (0x0042_4e68, libkernl_tty_ring_pop, "libkernl_TtyRingPop"),
    (0x0042_5320, libkernl_sio_putc, "libkernl_SioPutc"),
    (0x0042_5408, libkernl_sio_putc_crlf, "libkernl_SioPutcCRLF"),
];

pub(crate) mod oc1323_callee {
    use super::ee;
    use super::Callee;

    /// `sceFs_PathCmd` (`0x00428e18`, dropped from this lane: never runs).
    pub(crate) const OC1323_PATH_CMD: Callee = Callee {
        addr: 0x0042_8e18,
        f: ee::s15::f_00428e18,
    };
}

/// One path-command wrapper: frame the call, run path command `cmd` on
/// `a0`, restore and return. Shared by the remove/mkdir/rmdir/rename/
/// symlink/readlink/chdir/sync/umount family (each names its own command).
pub(crate) fn fs_path_cmd(c: &mut Cpu, cmd: u32, ret: u32) -> crate::call::Flow {
    use oc1323_callee::*;
    c.cycles += 7;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
    c.g[RA][0] = sx(ret);
    c.g[A1][0] = cmd as u64;
    OC1323_PATH_CMD.call(c, ret)?;
    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = t;
    Ok(())
}

/// `int sceRemove(const char *path)` (0x00428fc0): path command 6.
pub fn sce_remove(c: &mut Cpu) {
    guest(c, |c| {
        fs_path_cmd(c, 0x6, 0x00428fd0)?;
        return Ok(());
    });
}

/// `int sceRmdir(const char *path)` (0x00429190): path command 8.
pub fn sce_rmdir(c: &mut Cpu) {
    guest(c, |c| {
        fs_path_cmd(c, 0x8, 0x004291a0)?;
        return Ok(());
    });
}

/// `int sceDelDrv(const char *name)` (0x00429540): path command 0x10.
pub fn sce_del_drv(c: &mut Cpu) {
    guest(c, |c| {
        fs_path_cmd(c, 0x10, 0x00429550)?;
        return Ok(());
    });
}

/// `int sceChdir(const char *path)` (0x00429eb8): path command 0x12.
pub fn sce_chdir(c: &mut Cpu) {
    guest(c, |c| {
        fs_path_cmd(c, 0x12, 0x00429ec8)?;
        return Ok(());
    });
}

/// TTY ring-buffer header words (see `TtyRingPush`/`TtyRingPop`).
pub(crate) mod tty_ring {
    /// Live-byte count: decremented by pop, incremented by push.
    pub(crate) const COUNT_OFF: u32 = 0x4;
    /// Head offset: advanced by pop, wrapped at the end.
    pub(crate) const HEAD_OFF: u32 = 0x8;
    /// Tail offset: advanced by push, wrapped at the end.
    pub(crate) const TAIL_OFF: u32 = 0xc;
    /// Ring capacity in bytes: the base is advanced past it.
    pub(crate) const CAP: u32 = 0x10;
}

/// `void libkernl_TtyRingInit(void *ring)` (0x00424e00): point the ring at
/// the TTY buffer (`a0`).
pub fn libkernl_tty_ring_init(c: &mut Cpu) {
    use tty_ring::*;
    c.cycles += 9;
    c.g[V0][0] = 0x530000;
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4840));
    c.sw((c.g[V0][0] as u32).wrapping_sub(0x4840), c.g[A0][0]);
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(CAP));
    c.g[V0][0] = c.g[V1][0];
    c.sw((c.g[V1][0] as u32).wrapping_add(HEAD_OFF), c.g[A0][0]);
    c.sw((c.g[V1][0] as u32).wrapping_add(COUNT_OFF), 0u64);
    let t = c.g[RA][0] as u32;
    c.sw((c.g[V1][0] as u32).wrapping_add(TAIL_OFF), c.g[A0][0]);
    c.pc = t;
}

/// `int libkernl_TtyRingPush(void *ring, ...)` (0x00424e28): append to the
/// TTY ring; wraps the tail back to the buffer start when it runs past
/// the end.
pub fn libkernl_tty_ring_push(c: &mut Cpu) {
    use tty_ring::*;
    'b_424e5c: {
        c.cycles += 11;
        c.g[A1][0] = c.g[A0][0];
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(COUNT_OFF));
        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(TAIL_OFF));
        c.g[V1][0] = c.lw(c.g[A1][0] as u32);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
        c.sw((c.g[A1][0] as u32).wrapping_add(COUNT_OFF), c.g[V0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(CAP));
        c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
        let t = c.g[A0][0] != c.g[V1][0];
        c.sw((c.g[A1][0] as u32).wrapping_add(TAIL_OFF), c.g[A0][0]);
        if t {
            break 'b_424e5c;
        } else {
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(CAP));
            c.sw((c.g[A1][0] as u32).wrapping_add(TAIL_OFF), c.g[V0][0]);
            break 'b_424e5c;
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.pc = t;
}

/// `int libkernl_TtyRingPop(void *ring)` (0x00424e68): take a byte off the
/// TTY ring; wraps the head back to the buffer start when it runs past
/// the end.
pub fn libkernl_tty_ring_pop(c: &mut Cpu) {
    use tty_ring::*;
    'b_424e9c: {
        c.cycles += 11;
        c.g[A1][0] = c.g[A0][0];
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(COUNT_OFF));
        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(HEAD_OFF));
        c.g[V1][0] = c.lw(c.g[A1][0] as u32);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
        c.sw((c.g[A1][0] as u32).wrapping_add(COUNT_OFF), c.g[V0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(CAP));
        c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
        let t = c.g[A0][0] != c.g[V1][0];
        c.sw((c.g[A1][0] as u32).wrapping_add(HEAD_OFF), c.g[A0][0]);
        if t {
            break 'b_424e9c;
        } else {
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(CAP));
            c.sw((c.g[A1][0] as u32).wrapping_add(HEAD_OFF), c.g[V0][0]);
            break 'b_424e9c;
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.pc = t;
}

/// SIO registers.
pub(crate) mod sio {
    /// Status (`0x1000_f130`): bit 15 set while the port is busy.
    pub(crate) const STAT: u32 = 0x1000_f130;
    /// Transmit holding (`0x1000_f180`).
    pub(crate) const TX: u32 = 0x1000_f180;
    /// Busy bit of the status word.
    pub(crate) const BUSY: u64 = 0x8000;
}

/// `void libkernl_SioPutc(int ch)` (0x00425320): wait for the SIO port to
/// go idle, then send the byte.
pub fn libkernl_sio_putc(c: &mut Cpu) {
    use sio::*;
    guest(c, |c| {
        c.cycles += 2;
        // The port builds the register address in two steps; both words
        // are named in `sio` above.
        c.g[V1][0] = STAT as u64;
        loop {
            c.cycles += 7;
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            c.g[V0][0] = c.g[V0][0] & BUSY;
            if c.g[V0][0] == 0u64 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x00425328);
            }
        }
        c.cycles += 5;
        c.g[V0][0] = c.g[A0][0];
        c.g[V1][0] = TX as u64;
        let t = c.g[RA][0] as u32;
        c.sb(c.g[V1][0] as u32, c.g[A0][0]);
        c.pc = t;
        return Ok(());
    });
}

/// `void libkernl_SioPutcCRLF(int ch)` (0x00425408): send `\r` first when
/// the byte is `\n`, then send the byte itself.
pub fn libkernl_sio_putc_crlf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.g[V0][0] = 0xa;
        let newline = c.g[A0][0] == c.g[V0][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        // A plain byte tail-calls the putc below (the delay-slot `RA`
        // save runs on both paths).
        if !newline {
            c.cycles += 3;
            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.pc = 0x00425320;
            (ee::s15::libkernl_SioPutc)(c);
            return Ok(());
        }
        c.cycles += 6;
        c.g[RA][0] = sx(0x00425420);
        c.g[A0][0] = 0xd;
        c.pc = 0x00425320;
        (ee::s15::libkernl_SioPutc)(c);
        if c.pc != 0x00425420 {
            return Ok(());
        }
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.g[A0][0] = 0xa;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = 0x00425320;
        (ee::s15::libkernl_SioPutc)(c);
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) const SEEDS: u32 = 200;

    /// A path string in the fixture area.
    pub(crate) fn path_setup(c: &mut Cpu, seed: u32, salt: u64) {
        machine(c, seed, salt, 0);
        c.g[A0][0] = (DATA + 0x100) as u64;
        let name = b"host0:/x\0";
        for (i, b) in name.iter().enumerate() {
            c.bus.write8(DATA + 0x100 + i as u32, *b);
        }
    }

    /// Path-command wrappers: each names its own command word.
    #[test]
    fn sce_remove_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            path_setup(c, seed, 1_323_001);
            c.pc = 0x0042_8fc0;
        };
        check_paths(ee::s15::f_00428fc0, sce_remove, SEEDS, &[21169], &setup);
    }

    #[test]
    fn sce_rmdir_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            path_setup(c, seed, 1_323_002);
            c.pc = 0x0042_9190;
        };
        check_paths(ee::s15::f_00429190, sce_rmdir, SEEDS, &[21176], &setup);
    }

    #[test]
    fn sce_del_drv_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            path_setup(c, seed, 1_323_003);
            c.pc = 0x0042_9540;
        };
        check_paths(ee::s15::f_00429540, sce_del_drv, SEEDS, &[21176], &setup);
    }

    #[test]
    fn sce_chdir_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            path_setup(c, seed, 1_323_004);
            c.pc = 0x0042_9eb8;
        };
        check_paths(ee::s15::f_00429eb8, sce_chdir, SEEDS, &[21169], &setup);
    }

    /// `libkernl_TtyRingInit`: the ring header points at the TTY buffer.
    #[test]
    fn libkernl_tty_ring_init_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_323_005, 0);
            c.pc = 0x0042_4e00;
            c.g[A0][0] = (DATA + 0x100) as u64;
            c.bus
                .write32(0x0052_b7c0, seed.wrapping_mul(0x9e37_79b9) | 1);
        };
        check_paths(
            ee::s15::f_00424e00,
            libkernl_tty_ring_init,
            SEEDS,
            &[9],
            &setup,
        );
    }

    /// `libkernl_TtyRingPush`/`Pop`: plain advance, and the exact wrap
    /// value (the wrap threshold is `ring + base + 0x10`, so only that
    /// tail/head value wraps; the game never produces it).
    #[test]
    fn libkernl_tty_ring_push_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_323_006, 0);
            c.pc = 0x0042_4e28;
            c.g[A0][0] = (DATA + 0x100) as u64;
            c.bus.write32(DATA + 0x100, DATA + 0x200);
            c.bus.write32(DATA + 0x104, seed);
            // Tail digit: mid-buffer (advance) or exactly the wrap
            // threshold minus one (wrap).
            let wrap = (DATA + 0x100 + DATA + 0x200 + 0x10).wrapping_sub(1);
            c.bus
                .write32(DATA + 0x10c, [DATA + 0x205, wrap][seed as usize % 2]);
            for i in 0..16 {
                c.bus.write32(DATA + 0x200 + 4 * i, 0x0101_0100 + seed);
            }
        };
        check_paths(
            ee::s15::f_00424e28,
            libkernl_tty_ring_push,
            SEEDS,
            &[13, 15],
            &setup,
        );
    }

    #[test]
    fn libkernl_tty_ring_pop_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_323_007, 0);
            c.pc = 0x0042_4e68;
            c.g[A0][0] = (DATA + 0x100) as u64;
            c.bus.write32(DATA + 0x100, DATA + 0x200);
            c.bus.write32(DATA + 0x104, seed);
            // Head digit: mid-buffer (advance) or exactly the wrap
            // threshold minus one (wrap).
            let wrap = (DATA + 0x100 + DATA + 0x200 + 0x10).wrapping_sub(1);
            c.bus
                .write32(DATA + 0x108, [DATA + 0x205, wrap][seed as usize % 2]);
            for i in 0..16 {
                c.bus.write32(DATA + 0x200 + 4 * i, 0x0101_0100 + seed);
            }
        };
        check_paths(
            ee::s15::f_00424e68,
            libkernl_tty_ring_pop,
            SEEDS,
            &[13, 15],
            &setup,
        );
    }

    /// `libkernl_SioPutc`: the idle port sends at once. (The poll loop's
    /// back edge is unreachable in the harness: SIO_STAT reads 0, so the
    /// busy arm never runs; see branch-tables.)
    #[test]
    fn libkernl_sio_putc_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_323_008, 0);
            c.pc = 0x0042_5320;
            c.g[A0][0] = (seed & 0x7f) as u64 | 0x20;
        };
        check_paths(ee::s15::f_00425320, libkernl_sio_putc, SEEDS, &[14], &setup);
    }

    /// `libkernl_SioPutcCRLF`: plain bytes and `\n` (sent as `\r\n`). The
    /// putc underneath touches only MMIO, so no fake ever fires: plain
    /// `check` compares exactly.
    #[test]
    fn libkernl_sio_putc_crlf_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_323_009, 0);
            c.pc = 0x0042_5408;
            // Byte digit: `\n` (two sends) and plain bytes (one send).
            c.g[A0][0] = [0x0a, 0x41, 0x0d, 0x00][seed as usize % 4] as u64;
        };
        check(ee::s15::f_00425408, libkernl_sio_putc_crlf, SEEDS, &setup);
    }
}
