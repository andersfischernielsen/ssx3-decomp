//! `src/lib/kernel.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0042_39c0, exit, "_Exit"),
    (0x0042_39e0, load_exec_ps2, "_LoadExecPS2"),
    (0x0042_3a80, add_intc_handler, "AddIntcHandler"),
    (0x0042_3a90, add_intc_handler_2, "AddIntcHandler"),
    (0x0042_3ab0, add_dmac_handler, "AddDmacHandler"),
    (0x0042_3ac0, add_dmac_handler_2, "AddDmacHandler"),
    (0x0042_3ae0, enable_intc, "_EnableIntc"),
    (0x0042_3af0, disable_intc, "_DisableIntc"),
    (0x0042_3b00, enable_dmac, "_EnableDmac"),
    (0x0042_3b10, disable_dmac, "_DisableDmac"),
    (0x0042_3b40, i_enable_intc, "_iEnableIntc"),
    (0x0042_3b50, i_disable_intc, "_iDisableIntc"),
    (0x0042_3b60, i_enable_dmac, "_iEnableDmac"),
    (0x0042_3b70, i_disable_dmac, "_iDisableDmac"),
    (0x0042_3bc0, start_thread, "_StartThread"),
    (0x0042_3c90, get_thread_id, "GetThreadId"),
    (0x0042_3ce0, i_wakeup_thread, "_iWakeupThread"),
    (0x0042_3d20, i_suspend_thread, "_iSuspendThread"),
    (0x0042_3dc0, signal_sema, "SignalSema"),
    (0x0042_3dd0, i_signal_sema, "iSignalSema"),
    (0x0042_3de0, wait_sema, "WaitSema"),
    (0x0042_3df0, poll_sema, "PollSema"),
    (0x0042_4020, flush_cache, "FlushCache"),
    (0x0042_41c0, exec_osd, "_ExecOSD"),
    (0x0042_4210, init_tlb, "_InitTLB"),
    (0x0042_45f0, sync_dcache, "_SyncDCache"),
    (0x0042_4730, invalid_dcache, "_InvalidDCache"),
];

// ---------------------------------------------------------------------------
// Kernel syscall wrappers. Each sets the syscall code in `v1`, traps (the
// return address is `ret`, the trap itself at `ret - 4`), delivers any due
// event and returns whatever the trap left in `v0`.

/// The `syscall` trap: `pc = ret`, invoke the syscall hook at `at`; `false`
/// when the hook unwound (a thread switch mid-syscall).
#[inline]
pub(crate) fn do_syscall(c: &mut Cpu, at: u32, ret: u32) -> bool {
    c.pc = ret;
    c.syscall(at);
    c.pc == ret
}

/// The event check the syscall wrappers make just before returning (the
/// result of `event` is ignored, as compiled code ignores it).
#[inline]
pub(crate) fn event_if_due(c: &mut Cpu) {
    if event_due(c) {
        deliver_event_here(c, c.pc);
    }
}

/// One `syscall` wrapper: 4 cycles, `v1 = code`, trap at `at`, return to
/// `ret`, then a due-event check and `jr ra`.
#[inline]
pub(crate) fn syscall_wrapper(c: &mut Cpu, code: u32, at: u32, ret: u32) {
    c.cycles += 4;
    c.g[V1][0] = sx(code);
    if !do_syscall(c, at, ret) {
        return;
    }
    event_if_due(c);
    abi::ret(c);
}

/// `void _Exit(int status)` (0x4239c0): syscall 4.
pub fn exit(c: &mut Cpu) {
    syscall_wrapper(c, 4, 0x0042_39c4, 0x0042_39c8);
}

/// `void _LoadExecPS2(const char *path, int argc, char **argv)` (0x4239e0):
/// syscall 6.
pub fn load_exec_ps2(c: &mut Cpu) {
    syscall_wrapper(c, 6, 0x0042_39e4, 0x0042_39e8);
}

/// `int _EnableIntc(int intr)` (0x423ae0): syscall 0x14.
pub fn enable_intc(c: &mut Cpu) {
    syscall_wrapper(c, 0x14, 0x0042_3ae4, 0x0042_3ae8);
}

/// `int _DisableIntc(int intr)` (0x423af0): syscall 0x15.
pub fn disable_intc(c: &mut Cpu) {
    syscall_wrapper(c, 0x15, 0x0042_3af4, 0x0042_3af8);
}

/// `int _EnableDmac(int channel)` (0x423b00): syscall 0x16.
pub fn enable_dmac(c: &mut Cpu) {
    syscall_wrapper(c, 0x16, 0x0042_3b04, 0x0042_3b08);
}

/// `int _DisableDmac(int channel)` (0x423b10): syscall 0x17.
pub fn disable_dmac(c: &mut Cpu) {
    syscall_wrapper(c, 0x17, 0x0042_3b14, 0x0042_3b18);
}

/// `int _iEnableIntc(int intr)` (0x423b40): interrupt-disabled syscall -26.
pub fn i_enable_intc(c: &mut Cpu) {
    syscall_wrapper(c, (-26i32) as u32, 0x0042_3b44, 0x0042_3b48);
}

/// `int _iDisableIntc(int intr)` (0x423b50): syscall -27.
pub fn i_disable_intc(c: &mut Cpu) {
    syscall_wrapper(c, (-27i32) as u32, 0x0042_3b54, 0x0042_3b58);
}

/// `int _iEnableDmac(int channel)` (0x423b60): syscall -28.
pub fn i_enable_dmac(c: &mut Cpu) {
    syscall_wrapper(c, (-28i32) as u32, 0x0042_3b64, 0x0042_3b68);
}

/// `int _iDisableDmac(int channel)` (0x423b70): syscall -29.
pub fn i_disable_dmac(c: &mut Cpu) {
    syscall_wrapper(c, (-29i32) as u32, 0x0042_3b74, 0x0042_3b78);
}

/// `int _StartThread(struct thread *th, void *arg)` (0x423bc0): syscall 0x22.
pub fn start_thread(c: &mut Cpu) {
    syscall_wrapper(c, 0x22, 0x0042_3bc4, 0x0042_3bc8);
}

/// `int _iWakeupThread(int thid)` (0x423ce0): syscall -52.
pub fn i_wakeup_thread(c: &mut Cpu) {
    syscall_wrapper(c, (-52i32) as u32, 0x0042_3ce4, 0x0042_3ce8);
}

/// `int _iSuspendThread(int thid)` (0x423d20): syscall -56.
pub fn i_suspend_thread(c: &mut Cpu) {
    syscall_wrapper(c, (-56i32) as u32, 0x0042_3d24, 0x0042_3d28);
}

/// `void _ExecOSD(int argc, char **argv)` (0x4241c0): syscall 0x7b.
pub fn exec_osd(c: &mut Cpu) {
    syscall_wrapper(c, 0x7b, 0x0042_41c4, 0x0042_41c8);
}

/// `void _InitTLB(void)` (0x424210): syscall 0x82.
pub fn init_tlb(c: &mut Cpu) {
    syscall_wrapper(c, 0x82, 0x0042_4214, 0x0042_4218);
}

/// One pass of the cache-range loop: compute `address` from the CP0 tag
/// register (`mfc0 $2, $28`), masked to a page and offset by the index, and
/// classify both sub-blocks (cycle count only; the branches always fall
/// through). Returns the cycles charged.
#[inline]
pub(crate) fn cache_pass(c: &mut Cpu) {
    c.cycles += 10;
    c.mfc0(2, 28);
    c.g[V0][0] &= c.g[A3][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V1][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
    c.g[V0][0] = (c.g[V0][0] < c.g[A0][0]) as u64;
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        if c.g[V1][0] == 0 {
            c.cycles += 3;
        }
    }
    c.cycles += 10;
    c.mfc0(2, 28);
    c.g[V0][0] &= c.g[A3][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V1][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
    c.g[V0][0] = (c.g[V0][0] < c.g[A0][0]) as u64;
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        if c.g[V1][0] == 0 {
            c.cycles += 3;
        }
    }
}

/// The shared body of `_SyncDCache`/`_InvalidDCache`: walk 4 KiB in 0x40
/// steps; `head` is the loop head for event delivery.
pub(crate) fn cache_range(c: &mut Cpu, head: u32) {
    c.cycles += 4;
    c.g[A3][0] = 0xffff_ffff_ffff_0000;
    c.g[A2][0] = 0;
    c.g[A3][0] |= 0xf000;
    loop {
        cache_pass(c);
        c.cycles += 5;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x40));
        c.g[V0][0] = ((c.g[A2][0] as i64) < 4096) as u64;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            crate::events::deliver_event_here(c, head);
        }
    }
}

/// `void _SyncDCache(void *start, void *end)` (0x4245f0): write back the
/// cache lines covering `[start, end)`.
pub fn sync_dcache(c: &mut Cpu) {
    cache_range(c, 0x0042_4600);
}

/// `void _InvalidDCache(void *start, void *end)` (0x424730): invalidate the
/// cache lines covering `[start, end)`.
pub fn invalid_dcache(c: &mut Cpu) {
    cache_range(c, 0x0042_4740);
}

/// One `syscall` wrapper: 4 cycles, `v1 = code`, trap, a due-event check and
/// `jr ra`.
#[inline]
pub(crate) fn syscall_wrapper_2(c: &mut Cpu, code: u32, at: u32, ret: u32) {
    c.cycles += 4;
    c.g[V1][0] = sx(code);
    if !do_syscall(c, at, ret) {
        return;
    }
    if event_due(c) {
        deliver_event_here(c, c.pc);
    }
    abi::ret(c);
}

/// `int AddIntcHandler(int intr, int (*handler)(), int arg, int flag)`
/// (0x423a80): syscall 0x10.
pub fn add_intc_handler(c: &mut Cpu) {
    syscall_wrapper_2(c, 0x10, 0x0042_3a84, 0x0042_3a88);
}

/// `int AddIntcHandler(...)` (0x423a90): identical syscall-0x10 wrapper.
pub fn add_intc_handler_2(c: &mut Cpu) {
    syscall_wrapper_2(c, 0x10, 0x0042_3a94, 0x0042_3a98);
}

/// `int AddDmacHandler(int channel, int (*handler)(), int arg, int flag)`
/// (0x423ab0): syscall 0x12.
pub fn add_dmac_handler(c: &mut Cpu) {
    syscall_wrapper_2(c, 0x12, 0x0042_3ab4, 0x0042_3ab8);
}

/// `int AddDmacHandler(...)` (0x423ac0): identical syscall-0x12 wrapper.
pub fn add_dmac_handler_2(c: &mut Cpu) {
    syscall_wrapper_2(c, 0x12, 0x0042_3ac4, 0x0042_3ac8);
}

/// `int GetThreadId(void)` (0x423c90): syscall 0x2f, then one event check
/// (the generated code tests for a due event before returning).
pub fn get_thread_id(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = 0x2f;
    c.pc = 0x0042_3c98;
    c.syscall(0x0042_3c94);
    if c.pc != 0x0042_3c98 {
        return;
    }
    // Single post-syscall check (no loop back edge): the generated body
    // runs the handler inline, it does not reschedule.
    if c.cycles >= c.next_event {
        c.event();
    }
    abi::ret(c);
}

/// `int SignalSema(int sema)` (0x423dc0): syscall 0x42.
pub fn signal_sema(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = 0x42;
    c.pc = 0x0042_3dc8;
    c.syscall(0x0042_3dc4);
    if c.pc != 0x0042_3dc8 {
        return;
    }
    // Single post-syscall check (no loop back edge): the generated body
    // runs the handler inline, it does not reschedule.
    if c.cycles >= c.next_event {
        c.event();
    }
    abi::ret(c);
}

/// `int iSignalSema(int sema)` (0x423dd0): syscall 0x43 (the interrupt-level
/// variant; the generated code leaves `v1` sign-extended from 0xffffffbd,
/// i.e. the 64-bit syscall word -0x43).
pub fn i_signal_sema(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = 0xffff_ffff_ffff_ffbd;
    c.pc = 0x0042_3dd8;
    c.syscall(0x0042_3dd4);
    if c.pc != 0x0042_3dd8 {
        return;
    }
    // Single post-syscall check (no loop back edge): the generated body
    // runs the handler inline, it does not reschedule.
    if c.cycles >= c.next_event {
        c.event();
    }
    abi::ret(c);
}

/// `int WaitSema(int sema)` (0x423de0): syscall 0x44.
pub fn wait_sema(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = 0x44;
    c.pc = 0x0042_3de8;
    c.syscall(0x0042_3de4);
    if c.pc != 0x0042_3de8 {
        return;
    }
    // Single post-syscall check (no loop back edge): the generated body
    // runs the handler inline, it does not reschedule.
    if c.cycles >= c.next_event {
        c.event();
    }
    abi::ret(c);
}

/// `void FlushCache(int mode)` (0x424020): syscall 0x64.
pub fn flush_cache(c: &mut Cpu) {
    syscall_wrapper(c, 0x64, 0x0042_4024, 0x0042_4028);
}

/// `int PollSema(int sema)` (0x423df0): syscall 0x45.
pub fn poll_sema(c: &mut Cpu) {
    syscall_wrapper(c, 0x45, 0x0042_3df4, 0x0042_3df8);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::SP as STACK;
    pub(crate) use crate::game::lib::sif::tests::syscall_test as syscall_test_2;
    pub(crate) use crate::game::world::wscriptcache::tests::SEEDS as SEEDS_2;

    pub(crate) const SEEDS: u32 = 200;

    // ------------------------------------------------------------ syscalls

    /// Every syscall wrapper: fake syscalls (which may unwind), all argument
    /// registers scrambled.
    pub(crate) fn syscall_test(gen: GuestFn, hand: GuestFn, salt: u64) {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, salt, 1);
            // A few varied pointers in the argument registers.
            for i in 0..4 {
                c.g[[A0, A1, A2, A3][i]][0] = (DATA + (r.next_u32() & 0xfff0)) as u64;
            }
        };
        covered(gen, hand, SEEDS, &setup);
    }

    #[test]
    fn exit_matches() {
        syscall_test(ssx3_ee::s15::f_004239c0, exit, 1);
    }

    #[test]
    fn load_exec_ps2_matches() {
        syscall_test(ssx3_ee::s15::f_004239e0, load_exec_ps2, 2);
    }

    #[test]
    fn enable_intc_matches() {
        syscall_test(ssx3_ee::s15::f_00423ae0, enable_intc, 3);
    }

    #[test]
    fn disable_intc_matches() {
        syscall_test(ssx3_ee::s15::f_00423af0, disable_intc, 4);
    }

    #[test]
    fn enable_dmac_matches() {
        syscall_test(ssx3_ee::s15::f_00423b00, enable_dmac, 5);
    }

    #[test]
    fn disable_dmac_matches() {
        syscall_test(ssx3_ee::s15::f_00423b10, disable_dmac, 6);
    }

    #[test]
    fn i_enable_intc_matches() {
        syscall_test(ssx3_ee::s15::f_00423b40, i_enable_intc, 7);
    }

    #[test]
    fn i_disable_intc_matches() {
        syscall_test(ssx3_ee::s15::f_00423b50, i_disable_intc, 8);
    }

    #[test]
    fn i_enable_dmac_matches() {
        syscall_test(ssx3_ee::s15::f_00423b60, i_enable_dmac, 9);
    }

    #[test]
    fn i_disable_dmac_matches() {
        syscall_test(ssx3_ee::s15::f_00423b70, i_disable_dmac, 10);
    }

    #[test]
    fn start_thread_matches() {
        syscall_test(ssx3_ee::s15::f_00423bc0, start_thread, 11);
    }

    #[test]
    fn i_wakeup_thread_matches() {
        syscall_test(ssx3_ee::s15::f_00423ce0, i_wakeup_thread, 12);
    }

    #[test]
    fn i_suspend_thread_matches() {
        syscall_test(ssx3_ee::s15::f_00423d20, i_suspend_thread, 13);
    }

    #[test]
    fn exec_osd_matches() {
        syscall_test(ssx3_ee::s15::f_004241c0, exec_osd, 14);
    }

    #[test]
    fn init_tlb_matches() {
        syscall_test(ssx3_ee::s15::f_00424210, init_tlb, 15);
    }

    pub(crate) fn cache_test(gen: GuestFn, hand: GuestFn, entry: u32, salt: u64) {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, salt, 0);
            c.g[A0][0] = (r.next_u32() & 0xffff) as u64;
            c.g[A1][0] = (r.next_u32() & 0xffff) as u64;
            schedule(c, &mut r, seed, entry)
        };
        check_events(gen, hand, SEEDS, &setup);
    }

    #[test]
    fn sync_dcache_matches() {
        cache_test(ssx3_ee::s15::f_004245f0, sync_dcache, 0x0042_45f0, 70);
    }

    #[test]
    fn invalid_dcache_matches() {
        cache_test(ssx3_ee::s15::f_00424730, invalid_dcache, 0x0042_4730, 71);
    }

    // ------------------------------------------------------- syscall wrappers

    #[test]
    fn add_intc_and_dmac_handler_match() {
        for (gen, hand) in [
            (
                ee::s15::f_00423a80 as fn(&mut Cpu),
                add_intc_handler as fn(&mut Cpu),
            ),
            (ee::s15::f_00423a90, add_intc_handler_2),
            (ee::s15::f_00423ab0, add_dmac_handler),
            (ee::s15::f_00423ac0, add_dmac_handler_2),
        ] {
            covered(gen, hand, SEEDS_2, &|c, seed| {
                let _ = machine(c, seed, 0x423a80, 1);
            });
        }
    }

    #[test]
    fn get_thread_id_matches() {
        check(
            ssx3_ee::s15::f_00423c90__generated,
            get_thread_id,
            40,
            &|c, seed| {
                machine(c, seed, 0x423c_90, 0);
                c.g[A0][0] = sx(seed);
            },
        );
    }

    #[test]
    fn signal_sema_matches() {
        check(
            ssx3_ee::s15::f_00423dc0__generated,
            signal_sema,
            40,
            &|c, seed| {
                machine(c, seed, 0x423d_c0, 0);
                c.g[A0][0] = sx(seed);
            },
        );
    }

    #[test]
    fn i_signal_sema_matches() {
        check(
            ssx3_ee::s15::f_00423dd0__generated,
            i_signal_sema,
            40,
            &|c, seed| {
                machine(c, seed, 0x423d_d0, 0);
                c.g[A0][0] = sx(seed);
            },
        );
    }

    #[test]
    fn wait_sema_matches() {
        check(
            ssx3_ee::s15::f_00423de0__generated,
            wait_sema,
            40,
            &|c, seed| {
                machine(c, seed, 0x423d_e0, 0);
                c.g[A0][0] = sx(seed);
            },
        );
    }

    #[test]
    fn flush_cache_matches() {
        syscall_test_2(ssx3_ee::s15::f_00424020__generated, flush_cache, 36);
    }

    #[test]
    fn poll_sema_matches() {
        syscall_test_2(ssx3_ee::s15::f_00423df0__generated, poll_sema, 37);
    }
}
