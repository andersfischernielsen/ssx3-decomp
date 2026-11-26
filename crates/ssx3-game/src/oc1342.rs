//! Staging oc1342 (wave syscall): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1342`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call;
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::game::lib::gcc::{frame_ld, frame_sd};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0042_44a0, sbrk, "_sbrk"),
    (0x0042_4368, write, "_write"),
    (0x0042_43e8, read, "_read"),
    (0x0042_4460, open, "_open"),
    (0x0042_4498, lseek, "_lseek"),
    (0x0042_4488, close, "_close"),
    (0x0041_1310, exit, "exit"),
    (0x0041_78a8, eofread, "eofread"),
    (0x0041_2818, set_terminate, "set_terminate"),
];

// ---------------------------------------------------------------------------
// Callees (address, readable alias in the generated shard).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    /// The re-entrant errno getter (`_reent` in `v0`).
    pub(crate) use crate::game::misc::libm_wrap::GETREENT;

    /// The C++ `terminate()` (0x412800): runs the installed handler.
    pub(crate) use crate::game::lib::gcc::TERMINATE_CPP as TERMINATE;

    /// `void __do_global_dtors(void)` (0x40fb88): runs the destructor list.
    pub const DO_GLOBAL_DTORS: Callee = Callee {
        addr: 0x0040_fb88,
        f: ee::s15::__do_global_dtors,
    };

    /// `void _cleanup(void)` (0x416fe0): flushes and closes stdio.
    pub const CLEANUP: Callee = Callee {
        addr: 0x0041_6fe0,
        f: ee::s15::_cleanup,
    };

    /// `void *EndOfHeap(void)` (0x423d80): the current break (heap top).
    pub const END_OF_HEAP: Callee = Callee {
        addr: 0x0042_3d80,
        f: ee::s15::EndOfHeap,
    };

    /// `void libkernl_TtyInit(void)` (0x425260): opens the tty channel.
    pub const TTY_INIT: Callee = Callee {
        addr: 0x0042_5260,
        f: ee::s15::libkernl_TtyInit,
    };

    /// `int libkernl_TtyWrite(int fd, const void *buf)` (0x425040).
    pub const TTY_WRITE: Callee = Callee {
        addr: 0x0042_5040,
        f: ee::s15::libkernl_TtyWrite,
    };

    /// `int libkernl_TtyRead(int fd, void *buf)` (0x425190).
    pub const TTY_READ: Callee = Callee {
        addr: 0x0042_5190,
        f: ee::s15::libkernl_TtyRead,
    };
}

// ---------------------------------------------------------------------------
// _sbrk

/// Game heap base (`u32` at `0x450000 + 0x516c`): the break `sbrk` grows.
/// Bit `0x10000` of cop0 Status is the break lock `sbrk` spins on.
pub(crate) const SBRK_HEAP: u32 = 0x0045_0000;
pub(crate) const SBRK_HEAP_OFF: u32 = 0x516c;
pub(crate) const SBRK_BUSY_BIT: u64 = 0x10000;

/// `void *_sbrk(int incr)` (0x4244a0): grow the game heap by `incr` once
/// the break lock clears, returning the old break (`-1` when the locked
/// top already covers the new break). Spins with interrupts disabled
/// while the busy bit is set; the spin head checks events.
pub fn sbrk(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save_ra(c, 0x30);
        frame_sd(c, frame, 0x20, c.g[S2][0]);
        frame_sd(c, frame, 0x10, c.g[S1][0]);
        frame_sd(c, frame, 0, c.g[S0][0]);
        c.mfc0(17, 12);
        c.g[V0][0] = SBRK_BUSY_BIT;
        c.g[S1][0] = c.g[S1][0] & c.g[V0][0];
        let no_wait = c.g[S1][0] == 0;
        c.g[S2][0] = SBRK_HEAP as u64;
        if no_wait {
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(SBRK_HEAP_OFF));
        } else {
            loop {
                c.cycles += 7;
                c.di();
                c.mfc0(2, 12);
                c.g[V1][0] = SBRK_BUSY_BIT;
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0042_44c8);
                }
            }
            c.cycles += 2;
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(SBRK_HEAP_OFF));
        }
        c.cycles += 5;
        c.g[S0][0] = (c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32) as u64;
        c.g[S0][0] = sx(c.g[S0][0] as u32);
        callee::END_OF_HEAP.call(c, 0x0042_44f8)?;
        c.g[V0][0] = (c.g[V0][0] < c.g[S0][0]) as u64;
        let below_top = c.g[V0][0] == 0;
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(SBRK_HEAP_OFF));
        if below_top {
            c.cycles += 2;
            let unlocked = c.g[S1][0] == 0;
            c.sw((c.g[S2][0] as u32).wrapping_add(SBRK_HEAP_OFF), c.g[S0][0]);
            if !unlocked {
                c.cycles += 1;
                c.ei();
            }
        } else {
            c.cycles += 5;
            callee::GETREENT.call(c, 0x0042_450c)?;
            c.g[V1][0] = 0xc;
            let unlocked = c.g[S1][0] == 0;
            c.sw(c.g[V0][0] as u32, c.g[V1][0]);
            if !unlocked {
                c.cycles += 1;
                c.ei();
            }
            c.cycles += 3;
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[V0][0] = c.g[V0][0] | 0xffff;
        }
        c.cycles += 6;
        frame.restore_ra(c, 0x30);
        c.g[S2][0] = frame_ld(c, frame, 0x20);
        c.g[S1][0] = frame_ld(c, frame, 0x10);
        c.g[S0][0] = frame_ld(c, frame, 0);
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// _write

/// Tty-init flag (`u32` at `0x450000 + 0x5168`): set once the tty channel
/// is open, so later `_write`/`_read` calls go straight to it.
pub(crate) const TTY_FLAG: u32 = SBRK_HEAP;
pub(crate) const TTY_FLAG_OFF: u32 = 0x5168;

/// `int _write(int fd, const void *buf, int n)` (0x424368): only fds 1-2
/// reach the tty (anything else is `-1`); the first of them opens the
/// channel and retries, a failed open is `-1`.
pub fn write(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
        frame_sd(c, frame, 0x20, c.g[S2][0]);
        c.g[A0][0] = (c.g[A0][0] < 2) as u64;
        frame_sd(c, frame, 0x10, c.g[S1][0]);
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame_sd(c, frame, 0, c.g[S0][0]);
        if c.g[A0][0] != 0 {
            c.cycles += 4;
            c.g[S0][0] = TTY_FLAG as u64;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(TTY_FLAG_OFF));
            c.g[A0][0] = c.g[S1][0];
            let mut write_it = c.g[V0][0] != 0;
            if !write_it {
                c.cycles += 4;
                callee::TTY_INIT.call(c, 0x0042_43a8)?;
                write_it = c.g[V0][0] != 0;
                c.g[V0][0] = 1;
                if write_it {
                    c.cycles += 2;
                    c.sw((c.g[S0][0] as u32).wrapping_add(TTY_FLAG_OFF), c.g[V0][0]);
                    c.g[A0][0] = c.g[S1][0];
                }
            }
            if write_it {
                c.cycles += 4;
                c.g[A1][0] = c.g[S2][0];
                callee::TTY_WRITE.call(c, 0x0042_43c0)?;
                frame.restore_ra(c, 0x30);
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                frame.restore_ra(c, 0x30);
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            frame.restore_ra(c, 0x30);
        }
        c.cycles += 5;
        c.g[S2][0] = frame_ld(c, frame, 0x20);
        c.g[S1][0] = frame_ld(c, frame, 0x10);
        c.g[S0][0] = frame_ld(c, frame, 0);
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// _read

/// `int _read(int fd, void *buf, int n)` (0x4243e8): only fd 0 reads the
/// tty (anything else is `-1`); the first read opens the channel and
/// retries, a failed open is `-1`.
pub fn read(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame_sd(c, frame, 0x20, c.g[S2][0]);
        frame_sd(c, frame, 0x10, c.g[S1][0]);
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0x30);
        c.g[S1][0] = c.g[A1][0];
        let is_zero = c.g[A0][0] == 0;
        frame_sd(c, frame, 0, c.g[S0][0]);
        if !is_zero {
            c.cycles += 2;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            frame.restore_ra(c, 0x30);
        } else {
            c.cycles += 4;
            c.g[S0][0] = TTY_FLAG as u64;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(TTY_FLAG_OFF));
            c.g[A0][0] = c.g[S1][0];
            let mut read_it = c.g[V0][0] != 0;
            if !read_it {
                c.cycles += 4;
                callee::TTY_INIT.call(c, 0x0042_4420)?;
                read_it = c.g[V0][0] != 0;
                c.g[V0][0] = 1;
                if read_it {
                    c.cycles += 2;
                    c.sw((c.g[S0][0] as u32).wrapping_add(TTY_FLAG_OFF), c.g[V0][0]);
                    c.g[A0][0] = c.g[S1][0];
                }
            }
            if read_it {
                c.cycles += 4;
                c.g[A1][0] = c.g[S2][0];
                callee::TTY_READ.call(c, 0x0042_4438)?;
                frame.restore_ra(c, 0x30);
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                frame.restore_ra(c, 0x30);
            }
        }
        c.cycles += 5;
        c.g[S2][0] = frame_ld(c, frame, 0x20);
        c.g[S1][0] = frame_ld(c, frame, 0x10);
        c.g[S0][0] = frame_ld(c, frame, 0);
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// _open

/// `int _open(const char *path, int flags, int mode)` (0x424460): the
/// newlib glue has no filesystem of its own, so it reports the errno the
/// re-entrant struct carries and returns `-1`.
pub fn open(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::GETREENT.call(c, 0x0042_4470)?;
        c.g[V1][0] = 0x5;
        frame.restore_ra(c, 0);
        c.sw(c.g[V0][0] as u32, c.g[V1][0]);
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// _close

/// `int _close(int fd)` (0x424488): always `-1` (no fds to close).
pub fn close(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    c.pc = t;
}

// ---------------------------------------------------------------------------
// _lseek

/// `off_t _lseek(int fd, off_t off, int whence)` (0x424498): always `-1`
/// (nothing seekable).
pub fn lseek(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    c.pc = t;
}

// ---------------------------------------------------------------------------
// exit

/// The reset vector `exit` jumps to after running the destructors.
pub(crate) const EXIT_RESET: u32 = 0x0010_01c8;

/// `void exit(int status)` (0x411310): run the global destructors and the
/// stdio cleanup, then reset (a fatal-error path; never returns).
pub fn exit(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame_sd(c, frame, 0, c.g[S0][0]);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        callee::DO_GLOBAL_DTORS.call(c, 0x0041_1324)?;
        callee::CLEANUP.call(c, 0x0041_132c)?;
        c.g[A0][0] = c.g[S0][0];
        frame.restore_ra(c, 0x10);
        c.g[S0][0] = frame_ld(c, frame, 0);
        frame.close(c);
        c.pc = EXIT_RESET;
        ssx3_ee_rt::funcs::call_indirect(c, EXIT_RESET);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// eofread

/// `int eofread(void)` (0x4178a8): always 0 (no sticky EOF flag).
pub fn eofread(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = 0;
    c.pc = t;
}

// ---------------------------------------------------------------------------
// set_terminate

/// The terminate-handler slot `set_terminate` installs into.
pub(crate) const TERMINATE_SLOT_OFF: u32 = 0x3440;

/// `void set_terminate(void (*fn)(void))` (0x412818): install `a0` as the
/// terminate handler (`*(0x450000 + 0x3440)`). The resume entry re-runs the
/// store after `terminate` returns (it only returns on the test machine).
pub fn set_terminate(c: &mut Cpu) {
    call::guest(c, |c| {
        let resumed = match c.pc {
            0x0041_2818 => false,
            0x0041_2828 => true,
            pc => {
                c.jump_out(pc, 0x0041_2818, 0x0041_2838);
                return Ok(());
            }
        };
        if !resumed {
            c.cycles += 8;
            let frame = Frame::open(c, 0x10);
            frame.save_ra(c, 0);
            callee::TERMINATE.call(c, 0x0041_2828)?;
        } else {
            // Re-entry at the resume head carries its own charge (the
            // inline fall-through after the call does not).
            c.cycles += 4;
        }
        c.g[V1][0] = 0x450000;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(TERMINATE_SLOT_OFF));
        let t = c.g[RA][0] as u32;
        c.sw(
            (c.g[V1][0] as u32).wrapping_add(TERMINATE_SLOT_OFF),
            c.g[A0][0],
        );
        c.pc = t;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_paths;
    use crate::difftest::fakes::fall_back;
    use crate::difftest::fakes::hook;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::prev_cell;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::seed_count;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::SP;

    /// Seeds for the syscall-glue fixtures (independent digits).
    pub(crate) const SEEDS: u32 = 96;

    /// Heap word / tty flag scratch (absolute: the game link address).
    pub(crate) const HEAP_WORD: u32 = SBRK_HEAP + SBRK_HEAP_OFF;
    pub(crate) const TTY_WORD: u32 = TTY_FLAG + TTY_FLAG_OFF;

    /// Gate word arming the `exit`/`set_terminate` callee fakes below.
    pub(crate) const GLUE_GATE: u32 = DATA + 0x5050;
    pub(crate) const GLUE_MAGIC: u32 = 0x474c_5545;

    /// Gated `__do_global_dtors` (slot 0x40fb88): return at once when the
    /// exit fixture arms the gate, else run the installed body.
    fn oc1342_fake_dtors(c: &mut Cpu) {
        if c.bus.read32(GLUE_GATE) != GLUE_MAGIC {
            return fall_back(c, prev_cell(oc1342_fake_dtors), ee::s15::__do_global_dtors);
        }
        c.cycles += 3;
        crate::abi::ret(c);
    }

    /// Gated `_cleanup` (slot 0x416fe0): return at once when armed.
    fn oc1342_fake_cleanup(c: &mut Cpu) {
        if c.bus.read32(GLUE_GATE) != GLUE_MAGIC {
            return fall_back(c, prev_cell(oc1342_fake_cleanup), ee::s15::_cleanup);
        }
        c.cycles += 3;
        crate::abi::ret(c);
    }

    /// Gated `terminate` (slot 0x412800): return at once when armed (the
    /// real one re-enters `set_terminate` through the handler, which has
    /// no bottom on a test machine).
    fn oc1342_fake_terminate(c: &mut Cpu) {
        if c.bus.read32(GLUE_GATE) != GLUE_MAGIC {
            return fall_back(c, prev_cell(oc1342_fake_terminate), ee::s15::terminate);
        }
        c.cycles += 3;
        crate::abi::ret(c);
    }

    /// Hook the three fakes (once per process; the gate word stays
    /// per-Cpu, so unarmed tests still run the installed bodies).
    #[allow(unused_unsafe)]
    pub(crate) fn oc1342_install_fakes() {
        unsafe {
            hook(
                std::ptr::addr_of_mut!(ee::s15::OVERRIDE_0040FB88),
                oc1342_fake_dtors,
                prev_cell(oc1342_fake_dtors),
            );
            hook(
                std::ptr::addr_of_mut!(ee::s15::OVERRIDE_00416FE0),
                oc1342_fake_cleanup,
                prev_cell(oc1342_fake_cleanup),
            );
            hook(
                std::ptr::addr_of_mut!(ee::s15::OVERRIDE_00412800),
                oc1342_fake_terminate,
                prev_cell(oc1342_fake_terminate),
            );
        }
    }

    /// Arm the gate on this machine.
    pub(crate) fn oc1342_arm(c: &mut Cpu) {
        oc1342_install_fakes();
        c.bus.write32(GLUE_GATE, GLUE_MAGIC);
    }

    /// Fixture base: scrambled machine, cop0 Status from the seed (bit
    /// `0x10000` steers the `sbrk` wait; the low bit steers its `ei`).
    fn glue_machine(c: &mut Cpu, seed: u32, status: u32) -> Rng {
        let mut r = machine(c, seed, 0x4342, 0);
        c.g[T0][0] = status as u64;
        c.mtc0(T0, 12);
        c.g[A0][0] = (r.next_u32() % 0x2000) as u64;
        c.g[A1][0] = (DATA + 0x1000) as u64;
        c.g[A2][0] = (1 + r.next_u32() % 16) as u64;
        for i in 0..16 {
            c.bus.write8((DATA + 0x1000) + i, 0xa0 + (i as u8));
        }
        r
    }

    #[test]
    fn sbrk_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let store_path = seed % 2 == 0;
            // Status low bit steers the `ei`; the busy bit enters the
            // wait loop once (`di` clears it, so it always exits).
            let status = [0, 1, 0x10000, 0x10001][seed as usize % 4] as u32;
            glue_machine(c, seed, status);
            // Heap and increment: the new break lands on either side of
            // the forced `EndOfHeap` result.
            c.bus.write32(HEAP_WORD, DATA + 0x40000);
            c.g[A0][0] = 0x1000;
            c.bus.write32(
                LOG_RESULTS,
                if store_path {
                    DATA + 0x41000
                } else {
                    DATA + 0x1000
                },
            );
        };
        check_paths(ee::s15::f_004244a0, sbrk, SEEDS, &[37, 45, 46, 54], &setup);
    }

    #[test]
    fn write_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            glue_machine(c, seed, 0);
            let mut r = Rng(0x4368 ^ seed as u64);
            // fd: 0 and 3+ take the `-1` path; 1-2 reach the tty.
            c.g[A0][0] = match seed % 5 {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3 + r.next_u32() as u64 % 8,
                _ => 1 + r.next_u32() as u64 % 2,
            };
            // Tty already open on odd seeds; even seeds run `TtyInit`.
            let init_path = seed % 2 == 0;
            c.bus.write32(TTY_WORD, (!init_path) as u32);
            // A busy tty makes `TtyWrite` report `-1` at once (its send
            // and poll loops need live Deci2 state).
            c.bus.write32(0x0052_b8dc, (seed % 3 == 0) as u32);
            // Fail the open on every fourth init seed: force the
            // `Deci2Call` inside `sceDeci2Open` (fake call 1; call 0 is
            // `FlushCache`) negative, so `TtyInit` reports failure.
            if init_path && seed % 8 < 4 {
                c.bus.write32(LOG_RESULTS + 4, 0xffff_ffff);
            }
        };
        check_paths(
            ee::s15::f_00424368,
            write,
            SEEDS,
            &[17, 49, 84, 1101522],
            &setup,
        );
    }

    #[test]
    fn read_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            glue_machine(c, seed, 0);
            let mut r = Rng(0x43e8 ^ seed as u64);
            // Only fd 0 reaches the tty.
            c.g[A0][0] = match seed % 4 {
                0 => 0,
                1 => 0,
                _ => 1 + r.next_u32() as u64 % 8,
            };
            c.bus.write32(TTY_WORD, (seed % 2) as u32);
            // A zero count makes `TtyRead` return 0 at once (its ring
            // loops need live Deci2 state, which no fixture builds).
            c.g[A2][0] = 0;
            c.bus.write32(0x0052_b8dc, 0);
            // Fail the open on every fourth init seed (same forcing as
            // `write_matches`: the `Deci2Call` result goes negative).
            if seed % 2 == 0 && seed % 8 < 4 {
                c.bus.write32(LOG_RESULTS + 4, 0xffff_ffff);
            }
        };
        check_paths(ee::s15::f_004243e8, read, SEEDS, &[15, 41, 84, 138], &setup);
    }

    #[test]
    fn open_matches() {
        check_paths(ee::s15::f_00424460, open, SEEDS, &[13], &|c, seed| {
            glue_machine(c, seed, 0);
        });
    }

    #[test]
    fn close_matches() {
        check_paths(ee::s15::f_00424488, close, SEEDS, &[2], &|c, seed| {
            glue_machine(c, seed, 0);
        });
    }

    #[test]
    fn lseek_matches() {
        check_paths(ee::s15::f_00424498, lseek, SEEDS, &[2], &|c, seed| {
            glue_machine(c, seed, 0);
        });
    }

    #[test]
    fn exit_matches() {
        check(ee::s15::f_00411310, exit, SEEDS, &|c, seed| {
            glue_machine(c, seed, 0);
            let mut r = Rng(0x1310 ^ seed as u64);
            c.g[A0][0] = (r.next_u32() % 256) as u64;
            oc1342_arm(c);
        });
    }

    #[test]
    fn eofread_matches() {
        check_paths(ee::s15::f_004178a8, eofread, SEEDS, &[2], &|c, seed| {
            glue_machine(c, seed, 0);
        });
    }

    #[test]
    fn set_terminate_matches() {
        check(ee::s15::f_00412818, set_terminate, SEEDS, &|c, seed| {
            glue_machine(c, seed, 0);
            let mut r = Rng(0x2818 ^ seed as u64);
            c.g[A0][0] = (DATA + (r.next_u32() % 0x1000)) as u64;
            c.pc = 0x0041_2818;
            oc1342_arm(c);
        });
    }

    #[test]
    fn set_terminate_resume_matches() {
        check(ee::s15::f_00412818, set_terminate, SEEDS, &|c, seed| {
            glue_machine(c, seed, 0);
            let mut r = Rng(0x2828 ^ seed as u64);
            c.g[A0][0] = (DATA + (r.next_u32() % 0x1000)) as u64;
            c.g[RA][0] = 0x0012_3450 + seed as u64;
            c.pc = 0x0041_2828;
            oc1342_arm(c);
        });
    }
}
