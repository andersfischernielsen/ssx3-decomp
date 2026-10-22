//! `src/lib/gcc.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call;
use crate::call::Callee;
use crate::call::Frame;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::call::guest;
use crate::call::Flow;
use crate::call::Unwound;
use crate::events::back_edge_unwinds;
use crate::ps2f::FpuF32;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0010_0218, do_global_dtors_aux, "__do_global_dtors_aux"),
    (0x0040_fb88, do_global_dtors, "__do_global_dtors"),
    (0x0040_fbe0, do_global_ctors, "__do_global_ctors"),
    (0x0040_fc90, main_, "__main"),
    (0x0041_03a0, default_terminate, "__default_terminate"),
    (0x0041_03b0, terminate, "__terminate"),
    (0x0041_03d8, throw_type_match, "__throw_type_match"),
    (0x0041_0468, get_eh_context, "__get_eh_context"),
    (0x0041_0490, get_eh_info, "__get_eh_info"),
    (0x0041_0548, get_dynamic_handler_chain, "__get_dynamic_handler_chain"),
    (0x0041_1538, udivdi3, "__udivdi3"),
    (0x0041_2500, cmpdf2, "__cmpdf2"),
    (0x0041_2740, negdf2, "__negdf2"),
    (0x0041_2778, make_dp, "__make_dp"),
    (0x0041_27a8, truncdfsf2, "__truncdfsf2"),
    (0x0041_2848, cp_exception_info, "__cp_exception_info"),
    (0x0041_2888, cp_eh_info, "__cp_eh_info"),
    (0x0041_28a8, start_cp_handler, "__start_cp_handler"),
    (0x0041_28e0, eh_alloc, "__eh_alloc"),
    (0x0041_2908, eh_free, "__eh_free"),
    (0x0041_2af0, uncatch_exception, "__uncatch_exception"),
    (0x0041_2e80, tf13bad_exception, "__tf13bad_exception"),
    (0x0041_2f18, tf9exception, "__tf9exception"),
    (0x0041_3860, cmpsf2, "__cmpsf2"),
    (0x0041_3a90, negsf2, "__negsf2"),
    (0x0041_3ac8, make_fp, "__make_fp"),
    (0x0041_3af8, extendsfdf2, "__extendsfdf2"),
    (0x0041_46d0, register_frame_info, "__register_frame_info"),
    (0x0041_46f8, register_frame, "__register_frame"),
    (0x0041_4728, register_frame_info_table, "__register_frame_info_table"),
    (0x0041_4750, register_frame_table, "__register_frame_table"),
    (0x0041_47f8, deregister_frame, "__deregister_frame"),
    (0x0041_4c00, rtti_si, "__rtti_si"),
    (0x0041_4c20, rtti_user, "__rtti_user"),
    (0x0041_4c40, user_type_info_dcast, "__user_type_info_dcast"),
    (0x0041_4f00, tf17_class_type_info, "__tf17__class_type_info"),
    (0x0041_4f50, class_type_info_dtor, "__class_type_info_dtor"),
    (0x0041_4f90, tf14_si_type_info, "__tf14__si_type_info"),
    (0x0041_4fe0, si_type_info_dtor, "__si_type_info_dtor"),
    (0x0041_5018, tf16_user_type_info, "__tf16__user_type_info"),
    (0x0041_5068, user_type_info_dtor, "__user_type_info_dtor"),
    (0x0041_50d0, tf10bad_typeid, "__tf10bad_typeid"),
    (0x0041_5168, tf8bad_cast, "__tf8bad_cast"),
    (0x0041_51f8, tf9type_info, "__tf9type_info"),
    (0x0041_58f0, tfv, "__tfv"),
    (0x0041_5ca8, tf17_array_type_info, "__tf17__array_type_info"),
    (0x0041_5cf8, array_type_info_dtor, "__array_type_info_dtor"),
    (0x0041_5d18, tf16_ptmd_type_info, "__tf16__ptmd_type_info"),
    (0x0041_5d68, ptmd_type_info_dtor, "__ptmd_type_info_dtor"),
    (0x0041_5d88, tf16_ptmf_type_info, "__tf16__ptmf_type_info"),
    (0x0041_5dd8, ptmf_type_info_dtor, "__ptmf_type_info_dtor"),
    (0x0041_5df8, tf16_func_type_info, "__tf16__func_type_info"),
    (0x0041_5e48, func_type_info_dtor, "__func_type_info_dtor"),
    (0x0041_5e68, tf19_builtin_type_info, "__tf19__builtin_type_info"),
    (0x0041_5eb8, builtin_type_info_dtor, "__builtin_type_info_dtor"),
    (0x0041_5ed8, tf16_attr_type_info, "__tf16__attr_type_info"),
    (0x0041_5f28, attr_type_info_dtor, "__attr_type_info_dtor"),
    (0x0041_5f48, tf19_pointer_type_info, "__tf19__pointer_type_info"),
    (0x0041_5f98, pointer_type_info_dtor, "__pointer_type_info_dtor"),
    (0x0041_5fb8, getreent, "__getreent"),
    (0x0041_7948, sread, "__sread"),
    (0x0041_79b0, swrite, "__swrite"),
    (0x0041_7a30, sseek, "__sseek"),
    (0x0041_7a98, sclose, "__sclose"),
    (0x0041_8ca8, malloc_lock, "__malloc_lock"),
    (0x0041_8d08, malloc_unlock, "__malloc_unlock"),
    (0x0042_0180, sflags, "__sflags"),
    (0x0042_0830, hi0bits, "__hi0bits"),
    (0x0042_08b8, lo0bits, "__lo0bits"),
    (0x0042_0e28, mcmp, "__mcmp"),
    (0x0042_1020, ulp, "__ulp"),
    (0x0042_d320, fixdfdi, "__fixdfdi"),
    (0x0042_d380, fixunsdfdi, "__fixunsdfdi"),
    (0x0042_dad8, umoddi3, "__umoddi3"),
];


// ---------------------------------------------------------------------------
// Direct-call targets (the readable aliases, which dispatch through the
// generated override slots).

pub(crate) const DEREGISTER_FRAME_INFO: Callee = Callee { addr: 0x0041_4780, f: ssx3_ee::s15::__deregister_frame_info };

pub(crate) const DO_GLOBAL_CTORS: Callee = Callee { addr: 0x0040_fbe0, f: ssx3_ee::s15::__do_global_ctors };

pub(crate) const ABORT: Callee = Callee { addr: 0x0041_8790, f: ssx3_ee::s15::abort };

pub(crate) const TERMINATE: Callee = Callee { addr: 0x0041_03b0, f: ssx3_ee::s15::__terminate };

/// The C++ `terminate()` (0x412800), which calls the installed handler and
/// then re-enters `set_terminate`.
pub(crate) const TERMINATE_CPP: Callee = Callee { addr: 0x0041_2800, f: ssx3_ee::s15::terminate };

pub(crate) const STR_CMP: Callee = Callee { addr: 0x0041_65a8, f: ssx3_ee::s15::strcmp };

pub(crate) const GET_EH_INFO: Callee = Callee { addr: 0x0041_0490, f: ssx3_ee::s15::__get_eh_info };

pub(crate) const MALLOC: Callee = Callee { addr: 0x0041_88c0, f: ssx3_ee::s15::malloc };

pub(crate) const FREE: Callee = Callee { addr: 0x0041_8910, f: ssx3_ee::s15::free };

pub(crate) const REGISTER_FRAME_INFO: Callee = Callee { addr: 0x0041_46d0, f: ssx3_ee::s15::__register_frame_info };

pub(crate) const REGISTER_FRAME_INFO_TABLE: Callee = Callee { addr: 0x0041_4728, f: ssx3_ee::s15::__register_frame_info_table };

pub(crate) const TYPE_INFO_DTOR: Callee = Callee { addr: 0x0041_4b68, f: ssx3_ee::s15::type_info_dtor };

pub(crate) const TYPE_INFO_EQ: Callee = Callee { addr: 0x0041_4b98, f: ssx3_ee::s15::type_info_eq };

pub(crate) const TF9_EXCEPTION: Callee = Callee { addr: 0x0041_2f18, f: ssx3_ee::s15::__tf9exception };

pub(crate) const TF16_USER_TYPE_INFO: Callee = Callee { addr: 0x0041_5018, f: ssx3_ee::s15::__tf16__user_type_info };

pub(crate) const TF9_TYPE_INFO: Callee = Callee { addr: 0x0041_51f8, f: ssx3_ee::s15::__tf9type_info };

pub(crate) const RTTI_SI: Callee = Callee { addr: 0x0041_4c00, f: ssx3_ee::s15::__rtti_si };

pub(crate) const RTTI_USER: Callee = Callee { addr: 0x0041_4c20, f: ssx3_ee::s15::__rtti_user };

pub(crate) const UNPACK_D: Callee = Callee { addr: 0x0041_1c38, f: ssx3_ee::s15::__unpack_d };

pub(crate) const PACK_D: Callee = Callee { addr: 0x0041_1b08, f: ssx3_ee::s15::__pack_d };

pub(crate) const UNPACK_F: Callee = Callee { addr: 0x0041_3068, f: ssx3_ee::s15::__unpack_f };

pub(crate) const PACK_F: Callee = Callee { addr: 0x0041_2f58, f: ssx3_ee::s15::__pack_f };

pub(crate) const MAKE_DP: Callee = Callee { addr: 0x0041_2778, f: ssx3_ee::s15::__make_dp };

pub(crate) const MAKE_FP: Callee = Callee { addr: 0x0041_3ac8, f: ssx3_ee::s15::__make_fp };

pub(crate) const FPCMP_PARTS_D: Callee = Callee { addr: 0x0041_23e8, f: ssx3_ee::s15::__fpcmp_parts_d };

pub(crate) const FPCMP_PARTS_F: Callee = Callee { addr: 0x0041_3748, f: ssx3_ee::s15::__fpcmp_parts_f };


// ---------------------------------------------------------------------------
// Frame helpers (the generator emits a mix of `sd`/`ld` and `sq`/`lq`).

#[inline]
pub(crate) fn frame_sd(c: &mut Cpu, f: Frame, off: u32, v: u64) {
    Addr(f.at(c, off)).sd(c, v);
}


#[inline]
pub(crate) fn frame_ld(c: &mut Cpu, f: Frame, off: u32) -> u64 {
    Addr(f.at(c, off)).ld(c)
}


#[inline]
pub(crate) fn add_sp(c: &mut Cpu, delta: u32) {
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(delta));
}


// ---------------------------------------------------------------------------
// Global constructor/destructor walks.

/// `void __do_global_dtors_aux(void)` (0x100218): tear down the frame
/// registration list once.
pub fn do_global_dtors_aux(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.sq(FP, frame.at(c, 0x10));
        frame_sd(c, frame, 0, c.g[RA][0]);
        c.g[FP][0] = c.g[SP][0];
        c.g[A0][0] = sx(0x0043_ce00);
        DEREGISTER_FRAME_INFO.call(c, 0x0010_0238)?;
        c.g[SP][0] = c.g[FP][0];
        c.lq(FP, frame.at(c, 0x10));
        c.g[RA][0] = frame_ld(c, frame, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void __do_global_dtors(void)` (0x40fb88): walk the destructor list at
/// `*(0x450000 + 0x343c)` (a null-terminated array of function pointers),
/// calling each and advancing the cursor.
pub fn do_global_dtors(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = 0x450000;
        frame_sd(c, frame, 0x10, c.g[RA][0]);
        frame_sd(c, frame, 0, c.g[S0][0]);
        c.g[V0][0] = Addr(0x0045_343c).lw(c);
        let mut v0 = c.g[V0][0] as u32;
        c.g[V1][0] = Addr(v0).lw(c);
        let first = c.g[V1][0];
        c.g[S0][0] = 0x450000;
        if first != 0 {
            c.cycles += 2;
            v0 = (Addr(0x0045_343c).lw(c) as u32).wrapping_add(4);
            loop {
                c.cycles += 7;
                let target = Addr(v0.wrapping_sub(4)).lw(c) as u32;
                c.g[V1][0] = sx(target);
                c.g[V0][0] = sx(v0);
                c.g[RA][0] = sx(0x0040_fbbc);
                Addr(0x0045_343c).sw(c, v0 as u64);
                call::call_ptr(c, target, 0x0040_fbbc)?;
                v0 = Addr(0x0045_343c).lw(c) as u32;
                c.g[V0][0] = sx(v0);
                c.g[V1][0] = Addr(v0).lw(c);
                c.g[V0][0] = sx(v0.wrapping_add(4));
                if c.g[V1][0] == 0 {
                    break;
                }
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x0040_fbb0);
                }
            }
        }
        c.cycles += 4;
        c.g[RA][0] = frame_ld(c, frame, 0x10);
        c.g[S0][0] = frame_ld(c, frame, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void __do_global_ctors(void)` (0x40fbe0): walk the constructor table at
/// `0x440000 - 0x31c8` (word 0 is the count/head marker, the table follows)
/// and call the entries in reverse.
pub fn do_global_ctors(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 9;
        c.g[A3][0] = 0x440000;
        let frame = Frame::open(c, 0x30);
        let base = 0x0044_0000u32.wrapping_sub(0x31c8);
        c.g[A1][0] = Addr(base).lw(c);
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        frame_sd(c, frame, 0x20, c.g[RA][0]);
        c.g[V1][0] = sx(base);
        frame_sd(c, frame, 0x10, c.g[S1][0]);
        let at_end = c.g[A1][0] != c.g[V0][0];
        frame_sd(c, frame, 0, c.g[S0][0]);
        if at_end {
            // head marker is -1: nothing to run
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(base.wrapping_add(4)).lw(c);
            let empty = c.g[V0][0] == 0;
            c.g[A1][0] = 0;
            if !empty {
                c.cycles += 4;
                c.g[A2][0] = 1;
                c.g[A1][0] = 1;
                c.g[A0][0] = sx(base);
                // Scan forward for the terminating zero word; a1 ends one
                // below it.
                loop {
                    c.cycles += 9;
                    c.g[V1][0] = c.g[A1][0].wrapping_add(1);
                    c.g[A2][0] = c.g[V1][0];
                    c.g[V0][0] = c.g[A2][0] << 32;
                    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                    if c.g[V1][0] == 0 {
                        break;
                    }
                    c.g[A1][0] = c.g[A2][0];
                    if crate::events::event_due(c) {
                        crate::events::deliver_event_here(c, 0x0040_fc20);
                    }
                }
            }
        }
        // Run the collected entries, highest index first.
        c.cycles += 4;
        c.g[S1][0] = c.g[A1][0] << 32;
        c.g[S1][0] = ((c.g[S1][0] as i64) >> 32) as u64;
        c.g[V0][0] = 0x440000;
        if c.g[S1][0] != 0 {
            c.cycles += 5;
            c.g[V1][0] = sx((c.g[S1][0] as u32) << 2);
            c.g[V0][0] = sx(base);
            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            loop {
                c.cycles += 5;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
                let target = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0040_fc74);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(4));
                call::call_ptr(c, target, 0x0040_fc74)?;
                if c.g[S1][0] == 0 {
                    break;
                }
                c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x0040_fc68);
                }
            }
        }
        c.cycles += 5;
        c.g[RA][0] = frame_ld(c, frame, 0x20);
        c.g[S1][0] = frame_ld(c, frame, 0x10);
        c.g[S0][0] = frame_ld(c, frame, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void __main(void)` (0x40fc90): if the "ctors run" flag is clear, set it
/// and run `__do_global_ctors`.
pub fn main_(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = 0x530000;
    let done = Addr(0x0052_ad98).lw(c) != 0;
    c.g[V0][0] = 0x1;
    if done {
        c.cycles += 2;
        abi::ret(c);
    } else {
        c.cycles += 2;
        Addr(0x0052_ad98).sw(c, 1);
        c.pc = DO_GLOBAL_CTORS.addr;
        (DO_GLOBAL_CTORS.f)(c);
    }
}


// ---------------------------------------------------------------------------
// Exception-handling termination and frame registration.

/// `void __default_terminate(void)` (0x4103a0): `abort()` then `__terminate()`.
pub fn default_terminate(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        frame_sd(c, frame, 0, c.g[RA][0]);
        ABORT.call(c, 0x0041_03b0)?;
        c.pc = TERMINATE.addr;
        (TERMINATE.f)(c);
        Ok(())
    });
}


/// `void __terminate(void)` (0x4103b0): call the installed terminate handler
/// (`*(0x450000 + 0x3440)`).
pub fn terminate(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 9;
        c.g[V0][0] = 0x450000;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = Addr(0x0045_3440).lw(c);
        frame_sd(c, frame, 0, c.g[RA][0]);
        call::call_ptr(c, c.g[V1][0] as u32, 0x0041_03c8)?;
        c.g[RA][0] = frame_ld(c, frame, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void *__throw_type_match(const char *a, const char *b, void *c)`
/// (0x4103d8): `strcmp(a, b) == 0 ? c : 0`.
pub fn throw_type_match(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame_sd(c, frame, 0, c.g[S0][0]);
        frame_sd(c, frame, 0x10, c.g[RA][0]);
        c.g[S0][0] = c.g[A2][0];
        STR_CMP.call(c, 0x0041_03ec)?;
        if c.g[V0][0] != 0 {
            c.g[S0][0] = 0;
        }
        c.g[RA][0] = frame_ld(c, frame, 0x10);
        c.g[V0][0] = c.g[S0][0];
        c.g[S0][0] = frame_ld(c, frame, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void *__get_eh_context(void)` (0x410468): tail-call the current thread's
/// exception-context getter (`*(0x450000 + 0x3444)`).
pub fn get_eh_context(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 9;
        c.g[V1][0] = 0x450000;
        let frame = Frame::open(c, 0x10);
        let target = Addr(0x0045_3444).lw(c);
        c.g[V0][0] = target;
        frame_sd(c, frame, 0, c.g[RA][0]);
        call::call_ptr(c, target as u32, 0x0041_0480)?;
        c.g[RA][0] = frame_ld(c, frame, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void *__get_eh_info(void)` (0x410490): `__get_eh_context() + 8`.
pub fn get_eh_info(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 10;
        c.g[V1][0] = 0x450000;
        let frame = Frame::open(c, 0x10);
        let target = Addr(0x0045_3444).lw(c);
        c.g[V0][0] = target;
        frame_sd(c, frame, 0, c.g[RA][0]);
        call::call_ptr(c, target as u32, 0x0041_04a8)?;
        c.g[RA][0] = frame_ld(c, frame, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8));
        frame.close(c);
        Ok(())
    });
}


/// `void *__get_dynamic_handler_chain(void)` (0x410548): like `__get_eh_info`
/// but `+ 4`.
pub fn get_dynamic_handler_chain(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 10;
        c.g[V1][0] = 0x450000;
        let frame = Frame::open(c, 0x10);
        let target = Addr(0x0045_3444).lw(c);
        c.g[V0][0] = target;
        frame_sd(c, frame, 0, c.g[RA][0]);
        call::call_ptr(c, target as u32, 0x0041_0560)?;
        c.g[RA][0] = frame_ld(c, frame, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
        frame.close(c);
        Ok(())
    });
}


/// `void *__cp_exception_info(void)` (0x412848): call the per-thread handler
/// getter (`*(0x450000 + 0x3a38)`), then return `*__get_eh_info() + 8`.
pub fn cp_exception_info(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 15;
        c.g[V0][0] = 0x450000;
        let f1 = Frame::open(c, 0x10);
        let target = Addr(0x0045_3a38).lw(c);
        c.g[V1][0] = target;
        frame_sd(c, f1, 0, c.g[RA][0]);
        call::call_ptr(c, target as u32, 0x0041_2860)?;
        // Note: the original opens a second frame without closing the first.
        let f2 = Frame::open(c, 0x10);
        frame_sd(c, f2, 0, c.g[RA][0]);
        GET_EH_INFO.call(c, 0x0041_2870)?;
        c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[RA][0] = frame_ld(c, f2, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8));
        f2.close(c);
        Ok(())
    });
}


/// `void *__cp_eh_info(void)` (0x412888): `*__get_eh_info()`.
pub fn cp_eh_info(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame_sd(c, frame, 0, c.g[RA][0]);
        GET_EH_INFO.call(c, 0x0041_2898)?;
        c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[RA][0] = frame_ld(c, frame, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void __start_cp_handler(void)` (0x4128a8): mark the current exception
/// handler active and bump its counter.
pub fn start_cp_handler(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        frame_sd(c, frame, 0, c.g[RA][0]);
        GET_EH_INFO.call(c, 0x0041_28b8)?;
        let v1 = Addr(c.g[V0][0] as u32).lw(c);
        c.g[V1][0] = v1;
        c.g[A1][0] = 0x1;
        c.g[RA][0] = frame_ld(c, frame, 0);
        c.g[A0][0] = Addr(v1 as u32).off(0x20).ld(c);
        c.g[V0][0] = c.g[V1][0];
        Addr((v1 as u32).wrapping_add(0x14)).sw(c, c.g[A1][0]);
        c.g[A0][0] = c.g[A0][0].wrapping_add(0x1);
        Addr((v1 as u32).wrapping_add(0x20)).sd(c, c.g[A0][0]);
        frame.close(c);
        Ok(())
    });
}


/// `void *__eh_alloc(size_t n)` (0x4128e0): `malloc` or die.
pub fn eh_alloc(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame_sd(c, frame, 0, c.g[RA][0]);
        MALLOC.call(c, 0x0041_28f0)?;
        let ok = c.g[V0][0] != 0;
        c.g[RA][0] = frame_ld(c, frame, 0);
        if !ok {
            c.cycles += 2;
            TERMINATE_CPP.call(c, 0x0041_2900)?;
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}


/// `void __eh_free(void *p)` (0x412908): `free`.
pub fn eh_free(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame_sd(c, frame, 0, c.g[RA][0]);
        FREE.call(c, 0x0041_2918)?;
        c.g[RA][0] = frame_ld(c, frame, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void __uncatch_exception(void)` (0x412af0): clear the active handler.
pub fn uncatch_exception(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame_sd(c, frame, 0, c.g[RA][0]);
        GET_EH_INFO.call(c, 0x0041_2b00)?;
        c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
        let active = c.g[V0][0] != 0;
        c.g[RA][0] = frame_ld(c, frame, 0);
        if !active {
            c.cycles += 2;
            TERMINATE_CPP.call(c, 0x0041_2b14)?;
        }
        c.cycles += 3;
        Addr((c.g[V0][0] as u32).wrapping_add(0x14)).sw(c, 0);
        frame.close(c);
        Ok(())
    });
}


// ---------------------------------------------------------------------------
// RTTI slot installers.

/// `void __rtti_si(void *obj, int vtable, int base)` (0x414c00): fill a
/// simple single-inheritance type_info (`vtable` at +0, `base` at +4,
/// second base at +8).
pub fn rtti_si(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x4a0000;
    let obj = c.g[A0][0] as u32;
    if obj != 0 {
        c.cycles += 4;
        Addr(obj.wrapping_add(8)).sw(c, c.g[A2][0]);
        c.g[V0][0] = sx(0x004a_0000u32.wrapping_sub(0x6618));
        Addr(obj).sw(c, c.g[A1][0]);
        Addr(obj.wrapping_add(4)).sw(c, c.g[V0][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}


/// `void __rtti_user(void *obj, int vtable)` (0x414c20): fill a user
/// type_info's vtable slot.
pub fn rtti_user(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x4a0000;
    let obj = c.g[A0][0] as u32;
    if obj != 0 {
        c.cycles += 3;
        Addr(obj).sw(c, c.g[A1][0]);
        c.g[V0][0] = sx(0x004a_0000u32.wrapping_sub(0x6600));
        Addr(obj.wrapping_add(4)).sw(c, c.g[V0][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}


/// `void *__user_type_info_dcast(void *sub, int, int, void *obj)`
/// (0x414c40): `type_info_eq(sub, obj) ? obj : 0`.
pub fn user_type_info_dcast(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame_sd(c, frame, 0, c.g[S0][0]);
        frame_sd(c, frame, 0x10, c.g[RA][0]);
        c.g[S0][0] = c.g[A3][0];
        TYPE_INFO_EQ.call(c, 0x0041_4c54)?;
        if c.g[V0][0] == 0 {
            c.g[S0][0] = 0;
        }
        c.g[RA][0] = frame_ld(c, frame, 0x10);
        c.g[V0][0] = c.g[S0][0];
        c.g[S0][0] = frame_ld(c, frame, 0);
        frame.close(c);
        Ok(())
    });
}


// ---------------------------------------------------------------------------
// `__tf*` lazy type_info singletons.

/// One lazy type_info singleton. `base` is the address constant the original
/// keeps in `v1`; `obj` is the (guarded) object address; `pre` optionally runs
/// a base-class `__tf*` constructor first; `rtti` installs the slots with
/// `a1` (and `a2` when present).
pub(crate) struct Singleton {
    base: u32,
    obj: u32,
    pre: Option<(Callee, u32)>,
    rtti: (Callee, u32),
    a1: u32,
    a2: Option<u32>,
    init_cycles: u64,
}


pub(crate) fn singleton(c: &mut Cpu, s: Singleton) {
    call::guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame_sd(c, frame, 0, c.g[S0][0]);
        frame_sd(c, frame, 0x10, c.g[RA][0]);
        c.g[V1][0] = sx(s.base);
        let guard = Addr(s.obj).lw(c);
        c.g[V0][0] = guard;
        c.g[S0][0] = sx(s.obj);
        if guard == 0 {
            c.cycles += s.init_cycles;
            if let Some((callee, ret)) = s.pre {
                callee.call(c, ret)?;
            }
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx(s.a1);
            if let Some(a2) = s.a2 {
                c.g[A2][0] = sx(a2);
            }
            let (callee, ret) = s.rtti;
            callee.call(c, ret)?;
        }
        c.cycles += 5;
        c.g[V0][0] = c.g[S0][0];
        c.g[RA][0] = frame_ld(c, frame, 0x10);
        c.g[S0][0] = frame_ld(c, frame, 0);
        frame.close(c);
        Ok(())
    });
}


/// `__tf13bad_exception` (0x412e80).
pub fn tf13bad_exception(c: &mut Cpu) {
    singleton(c, Singleton {
        base: 0x0054_0000,
        obj: 0x0053_ea10,
        pre: Some((TF9_EXCEPTION, 0x0041_2ea4)),
        rtti: (RTTI_SI, 0x0041_2ebc),
        a1: 0x0049_98d0,
        a2: Some(0x004a_5ba8),
        init_cycles: 8,
    });
}


/// `__tf9exception` (0x412f18).
pub fn tf9exception(c: &mut Cpu) {
    singleton(c, Singleton {
        base: 0x004a_0000,
        obj: 0x004a_5ba8,
        pre: None,
        rtti: (RTTI_USER, 0x0041_2f44),
        a1: 0x0049_98e0,
        a2: None,
        init_cycles: 4,
    });
}


/// `__tf9type_info` (0x4151f8).
pub fn tf9type_info(c: &mut Cpu) {
    singleton(c, Singleton {
        base: 0x004a_0000,
        obj: 0x004a_5bb0,
        pre: None,
        rtti: (RTTI_USER, 0x0041_5224),
        a1: 0x0049_9ac0,
        a2: None,
        init_cycles: 4,
    });
}


/// `__tf16__user_type_info` (0x415018).
pub fn tf16_user_type_info(c: &mut Cpu) {
    singleton(c, Singleton {
        base: 0x0054_0000,
        obj: 0x0053_ea40,
        pre: Some((TF9_TYPE_INFO, 0x0041_503c)),
        rtti: (RTTI_SI, 0x0041_5054),
        a1: 0x0049_9a88,
        a2: Some(0x004a_5bb0),
        init_cycles: 8,
    });
}


/// `__tf17__class_type_info` (0x414f00).
pub fn tf17_class_type_info(c: &mut Cpu) {
    singleton(c, Singleton {
        base: 0x0054_0000,
        obj: 0x0053_ea60,
        pre: Some((TF16_USER_TYPE_INFO, 0x0041_4f24)),
        rtti: (RTTI_SI, 0x0041_4f3c),
        a1: 0x0049_9a58,
        a2: Some(0x0053_ea40),
        init_cycles: 8,
    });
}


/// `__tf14__si_type_info` (0x414f90).
pub fn tf14_si_type_info(c: &mut Cpu) {
    singleton(c, Singleton {
        base: 0x0054_0000,
        obj: 0x0053_ea50,
        pre: Some((TF16_USER_TYPE_INFO, 0x0041_4fb4)),
        rtti: (RTTI_SI, 0x0041_4fcc),
        a1: 0x0049_9a70,
        a2: Some(0x0053_ea40),
        init_cycles: 8,
    });
}


/// `__tf10bad_typeid` (0x4150d0).
pub fn tf10bad_typeid(c: &mut Cpu) {
    singleton(c, Singleton {
        base: 0x0054_0000,
        obj: 0x0053_ea20,
        pre: Some((TF9_EXCEPTION, 0x0041_50f4)),
        rtti: (RTTI_SI, 0x0041_510c),
        a1: 0x0049_9aa0,
        a2: Some(0x004a_5ba8),
        init_cycles: 8,
    });
}


/// `__tf8bad_cast` (0x415168).
pub fn tf8bad_cast(c: &mut Cpu) {
    singleton(c, Singleton {
        base: 0x0054_0000,
        obj: 0x0053_ea30,
        pre: Some((TF9_EXCEPTION, 0x0041_518c)),
        rtti: (RTTI_SI, 0x0041_51a4),
        a1: 0x0049_9ab0,
        a2: Some(0x004a_5ba8),
        init_cycles: 8,
    });
}


/// `__tf17__array_type_info` (0x415ca8).
pub fn tf17_array_type_info(c: &mut Cpu) {
    singleton(c, Singleton {
        base: 0x0054_0000,
        obj: 0x0053_eaa0,
        pre: Some((TF9_TYPE_INFO, 0x0041_5ccc)),
        rtti: (RTTI_SI, 0x0041_5ce4),
        a1: 0x0049_9bd8,
        a2: Some(0x004a_5bb0),
        init_cycles: 8,
    });
}


/// `__tf16__ptmd_type_info` (0x415d18).
pub fn tf16_ptmd_type_info(c: &mut Cpu) {
    singleton(c, Singleton {
        base: 0x0054_0000,
        obj: 0x0053_eac0,
        pre: Some((TF9_TYPE_INFO, 0x0041_5d3c)),
        rtti: (RTTI_SI, 0x0041_5d54),
        a1: 0x0049_9bf0,
        a2: Some(0x004a_5bb0),
        init_cycles: 8,
    });
}


/// `__tf16__ptmf_type_info` (0x415d88).
pub fn tf16_ptmf_type_info(c: &mut Cpu) {
    singleton(c, Singleton {
        base: 0x0054_0000,
        obj: 0x0053_ea90,
        pre: Some((TF9_TYPE_INFO, 0x0041_5dac)),
        rtti: (RTTI_SI, 0x0041_5dc4),
        a1: 0x0049_9c08,
        a2: Some(0x004a_5bb0),
        init_cycles: 8,
    });
}


/// `__tf16__func_type_info` (0x415df8).
pub fn tf16_func_type_info(c: &mut Cpu) {
    singleton(c, Singleton {
        base: 0x0054_0000,
        obj: 0x0053_ea70,
        pre: Some((TF9_TYPE_INFO, 0x0041_5e1c)),
        rtti: (RTTI_SI, 0x0041_5e34),
        a1: 0x0049_9c20,
        a2: Some(0x004a_5bb0),
        init_cycles: 8,
    });
}


/// `__tf19__builtin_type_info` (0x415e68).
pub fn tf19_builtin_type_info(c: &mut Cpu) {
    singleton(c, Singleton {
        base: 0x0054_0000,
        obj: 0x0053_ead0,
        pre: Some((TF9_TYPE_INFO, 0x0041_5e8c)),
        rtti: (RTTI_SI, 0x0041_5ea4),
        a1: 0x0049_9c38,
        a2: Some(0x004a_5bb0),
        init_cycles: 8,
    });
}


/// `__tf16__attr_type_info` (0x415ed8).
pub fn tf16_attr_type_info(c: &mut Cpu) {
    singleton(c, Singleton {
        base: 0x0054_0000,
        obj: 0x0053_ea80,
        pre: Some((TF9_TYPE_INFO, 0x0041_5efc)),
        rtti: (RTTI_SI, 0x0041_5f14),
        a1: 0x0049_9c50,
        a2: Some(0x004a_5bb0),
        init_cycles: 8,
    });
}


/// `__tf19__pointer_type_info` (0x415f48).
pub fn tf19_pointer_type_info(c: &mut Cpu) {
    singleton(c, Singleton {
        base: 0x0054_0000,
        obj: 0x0053_eab0,
        pre: Some((TF9_TYPE_INFO, 0x0041_5f6c)),
        rtti: (RTTI_SI, 0x0041_5f84),
        a1: 0x0049_9c68,
        a2: Some(0x004a_5bb0),
        init_cycles: 8,
    });
}


// ---------------------------------------------------------------------------
// type_info destructors: each just forwards to `type_info_dtor`.

pub(crate) fn type_info_dtor_call(c: &mut Cpu, ret: u32) {
    call::guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame_sd(c, frame, 0, c.g[RA][0]);
        TYPE_INFO_DTOR.call(c, ret)?;
        c.g[RA][0] = frame_ld(c, frame, 0);
        frame.close(c);
        Ok(())
    });
}


/// `__class_type_info_dtor` (0x414f50).
pub fn class_type_info_dtor(c: &mut Cpu) {
    type_info_dtor_call(c, 0x0041_4f60);
}

/// `__si_type_info_dtor` (0x414fe0).
pub fn si_type_info_dtor(c: &mut Cpu) {
    type_info_dtor_call(c, 0x0041_4ff0);
}

/// `__user_type_info_dtor` (0x415068).
pub fn user_type_info_dtor(c: &mut Cpu) {
    type_info_dtor_call(c, 0x0041_5078);
}

/// `__array_type_info_dtor` (0x415cf8).
pub fn array_type_info_dtor(c: &mut Cpu) {
    type_info_dtor_call(c, 0x0041_5d08);
}

/// `__ptmd_type_info_dtor` (0x415d68).
pub fn ptmd_type_info_dtor(c: &mut Cpu) {
    type_info_dtor_call(c, 0x0041_5d78);
}

/// `__ptmf_type_info_dtor` (0x415dd8).
pub fn ptmf_type_info_dtor(c: &mut Cpu) {
    type_info_dtor_call(c, 0x0041_5de8);
}

/// `__func_type_info_dtor` (0x415e48).
pub fn func_type_info_dtor(c: &mut Cpu) {
    type_info_dtor_call(c, 0x0041_5e58);
}

/// `__builtin_type_info_dtor` (0x415eb8).
pub fn builtin_type_info_dtor(c: &mut Cpu) {
    type_info_dtor_call(c, 0x0041_5ec8);
}

/// `__attr_type_info_dtor` (0x415f28).
pub fn attr_type_info_dtor(c: &mut Cpu) {
    type_info_dtor_call(c, 0x0041_5f38);
}

/// `__pointer_type_info_dtor` (0x415f98).
pub fn pointer_type_info_dtor(c: &mut Cpu) {
    type_info_dtor_call(c, 0x0041_5fa8);
}


// ---------------------------------------------------------------------------
// Frame registration.

/// `void __register_frame_info(void *begin, void *obj)` (0x4146d0):
/// link `obj` into the frame list and stash the begin pointer.
pub fn register_frame_info(c: &mut Cpu) {
    c.cycles += 10;
    c.g[V0][0] = 0x530000;
    c.g[V1][0] = Addr(0x0052_ade8).lw(c);
    let a0 = c.g[A0][0];
    let a1 = c.g[A1][0] as u32;
    Addr(a1.wrapping_add(8)).sw(c, a0);
    Addr(a1.wrapping_add(0x14)).sw(c, c.g[V1][0]);
    Addr(0x0052_ade8).sw(c, c.g[A1][0]);
    Addr(a1.wrapping_add(4)).sw(c, 0);
    Addr(a1).sw(c, 0);
    Addr(a1.wrapping_add(0xc)).sw(c, 0);
    Addr(a1.wrapping_add(0x10)).sw(c, 0);
    abi::ret(c);
}


/// `void __register_frame(void *begin)` (0x4146f8): allocate the 0x18-byte
/// frame object and register it.
pub fn register_frame(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame_sd(c, frame, 0, c.g[S0][0]);
        c.g[S0][0] = c.g[A0][0];
        frame_sd(c, frame, 0x10, c.g[RA][0]);
        c.g[A0][0] = 0x18;
        MALLOC.call(c, 0x0041_4710)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = frame_ld(c, frame, 0x10);
        c.g[S0][0] = frame_ld(c, frame, 0);
        c.g[A1][0] = c.g[V0][0];
        add_sp(c, 0x20);
        c.pc = REGISTER_FRAME_INFO.addr;
        (REGISTER_FRAME_INFO.f)(c);
        Ok(())
    });
}


/// `void __register_frame_info_table(void *begin, void *obj)` (0x414728).
pub fn register_frame_info_table(c: &mut Cpu) {
    c.cycles += 10;
    c.g[V1][0] = 0x530000;
    c.g[V0][0] = Addr(0x0052_ade8).lw(c);
    let a0 = c.g[A0][0];
    let a1 = c.g[A1][0] as u32;
    Addr(a1.wrapping_add(0xc)).sw(c, a0);
    Addr(a1.wrapping_add(0x14)).sw(c, c.g[V0][0]);
    Addr(0x0052_ade8).sw(c, c.g[A1][0]);
    Addr(a1.wrapping_add(8)).sw(c, a0);
    Addr(a1.wrapping_add(4)).sw(c, 0);
    Addr(a1).sw(c, 0);
    Addr(a1.wrapping_add(0x10)).sw(c, 0);
    abi::ret(c);
}


/// `void __register_frame_table(void *begin)` (0x414750).
pub fn register_frame_table(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame_sd(c, frame, 0, c.g[S0][0]);
        c.g[S0][0] = c.g[A0][0];
        frame_sd(c, frame, 0x10, c.g[RA][0]);
        c.g[A0][0] = 0x18;
        MALLOC.call(c, 0x0041_4768)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = frame_ld(c, frame, 0x10);
        c.g[S0][0] = frame_ld(c, frame, 0);
        c.g[A1][0] = c.g[V0][0];
        add_sp(c, 0x20);
        c.pc = REGISTER_FRAME_INFO_TABLE.addr;
        (REGISTER_FRAME_INFO_TABLE.f)(c);
        Ok(())
    });
}


/// `void __deregister_frame(void *begin)` (0x4147f8): unregister and free.
pub fn deregister_frame(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame_sd(c, frame, 0, c.g[RA][0]);
        DEREGISTER_FRAME_INFO.call(c, 0x0041_4808)?;
        c.g[RA][0] = frame_ld(c, frame, 0);
        c.g[A0][0] = c.g[V0][0];
        add_sp(c, 0x10);
        c.pc = FREE.addr;
        (FREE.f)(c);
        Ok(())
    });
}


// ---------------------------------------------------------------------------
// Misc small utilities.

/// `void *__tfv(void)` (0x4158f0): the `void` type_info singleton.
pub fn tfv(c: &mut Cpu) {
    c.cycles += 4;
    let a0 = 0x0045_0000u32;
    c.g[A0][0] = sx(a0);
    let ptr = a0.wrapping_add(0x4158);
    c.g[V0][0] = Addr(ptr).lw(c);
    c.g[A1][0] = sx(ptr);
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        c.g[V1][0] = 0x4a0000;
        c.cycles += 5;
        c.g[V0][0] = sx(0x004a_0000u32.wrapping_sub(0x6458));
        c.g[V1][0] = sx(0x004a_0000u32.wrapping_sub(0x6520));
        Addr(ptr.wrapping_add(4)).sw(c, c.g[V0][0]);
        Addr(ptr).sw(c, c.g[V1][0]);
    }
    c.cycles += 2;
    c.g[V0][0] = sx(ptr);
    abi::ret(c);
}


/// `void *__getreent(void)` (0x415fb8): the current reentrancy struct
/// (`*(0x450000 + 0x4944)`).
pub fn getreent(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = 0x450000;
    c.g[V0][0] = Addr(0x0045_4944).lw(c);
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Software-float helpers (double/single unpack/pack wrappers).

/// `double __negdf2(double x)` (0x412740): flip the sign field.
pub fn negdf2(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        frame_sd(c, frame, 0x20, c.g[A0][0]);
        c.g[A1][0] = c.g[SP][0];
        frame_sd(c, frame, 0x30, c.g[RA][0]);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        UNPACK_D.call(c, 0x0041_2758)?;
        c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(4)).lw(c);
        c.g[A0][0] = c.g[SP][0];
        c.g[V0][0] = (c.g[V0][0] < 0x1) as u64;
        Addr((c.g[SP][0] as u32).wrapping_add(4)).sw(c, c.g[V0][0]);
        PACK_D.call(c, 0x0041_276c)?;
        c.g[RA][0] = frame_ld(c, frame, 0x30);
        frame.close(c);
        Ok(())
    });
}


/// `double __make_dp(int cls, int sign, int exp, u64 mant)` (0x412778).
pub fn make_dp(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        Addr(frame.at(c, 0)).sw(c, c.g[A0][0]);
        frame_sd(c, frame, 0x20, c.g[RA][0]);
        c.g[A0][0] = c.g[SP][0];
        Addr(frame.at(c, 4)).sw(c, c.g[A1][0]);
        Addr(frame.at(c, 8)).sw(c, c.g[A2][0]);
        frame_sd(c, frame, 0x10, c.g[A3][0]);
        PACK_D.call(c, 0x0041_2798)?;
        c.g[RA][0] = frame_ld(c, frame, 0x20);
        frame.close(c);
        Ok(())
    });
}


/// `float __truncdfsf2(double x)` (0x4127a8).
pub fn truncdfsf2(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x40);
        frame_sd(c, frame, 0x20, c.g[A0][0]);
        c.g[A1][0] = c.g[SP][0];
        frame_sd(c, frame, 0x30, c.g[RA][0]);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        UNPACK_D.call(c, 0x0041_27c0)?;
        c.g[V0][0] = frame_ld(c, frame, 0x10);
        c.g[V1][0] = 0x3fff_0000;
        c.g[V1][0] = c.g[V1][0] | 0xffff;
        c.g[A0][0] = Addr(frame.at(c, 0)).lw(c);
        c.g[T0][0] = c.g[V0][0] << 2;
        c.g[T0][0] = ((c.g[T0][0] as i64) >> 32) as u64;
        c.g[A1][0] = Addr(frame.at(c, 4)).lw(c);
        c.g[V0][0] &= c.g[V1][0];
        c.g[A3][0] = c.g[T0][0] | 0x1;
        c.g[A2][0] = Addr(frame.at(c, 8)).lw(c);
        c.g[RA][0] = sx(0x0041_27f0);
        if c.g[V0][0] == 0 {
            c.g[A3][0] = c.g[T0][0];
        }
        MAKE_FP.call(c, 0x0041_27f0)?;
        c.g[RA][0] = frame_ld(c, frame, 0x30);
        frame.close(c);
        Ok(())
    });
}


/// `int __cmpdf2(double a, double b)` (0x412500): three-way compare.
pub fn cmpdf2(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x70);
        Addr(frame.at(c, 0x40)).sd(c, c.g[A0][0]);
        Addr(frame.at(c, 0x48)).sd(c, c.g[A1][0]);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        frame_sd(c, frame, 0x50, c.g[S0][0]);
        frame_sd(c, frame, 0x60, c.g[RA][0]);
        c.g[A1][0] = c.g[SP][0];
        UNPACK_D.call(c, 0x0041_2520)?;
        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x48));
        c.g[A1][0] = c.g[S0][0];
        UNPACK_D.call(c, 0x0041_2530)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[SP][0];
        FPCMP_PARTS_D.call(c, 0x0041_253c)?;
        c.g[RA][0] = frame_ld(c, frame, 0x60);
        c.g[S0][0] = frame_ld(c, frame, 0x50);
        frame.close(c);
        Ok(())
    });
}


/// `float __negsf2(float x)` (0x413a90).
pub fn negsf2(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame_sd(c, frame, 0x20, c.g[RA][0]);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.g[SP][0];
        UNPACK_F.call(c, 0x0041_3aa8)?;
        c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(4)).lw(c);
        c.g[A0][0] = c.g[SP][0];
        c.g[V0][0] = (c.g[V0][0] < 0x1) as u64;
        Addr((c.g[SP][0] as u32).wrapping_add(4)).sw(c, c.g[V0][0]);
        PACK_F.call(c, 0x0041_3abc)?;
        c.g[RA][0] = frame_ld(c, frame, 0x20);
        frame.close(c);
        Ok(())
    });
}


/// `float __make_fp(int cls, int sign, int exp, int mant)` (0x413ac8).
pub fn make_fp(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        Addr(frame.at(c, 0)).sw(c, c.g[A0][0]);
        frame_sd(c, frame, 0x10, c.g[RA][0]);
        c.g[A0][0] = c.g[SP][0];
        Addr(frame.at(c, 4)).sw(c, c.g[A1][0]);
        Addr(frame.at(c, 8)).sw(c, c.g[A2][0]);
        Addr(frame.at(c, 0xc)).sw(c, c.g[A3][0]);
        PACK_F.call(c, 0x0041_3ae8)?;
        c.g[RA][0] = frame_ld(c, frame, 0x10);
        frame.close(c);
        Ok(())
    });
}


/// `double __extendsfdf2(float x)` (0x413af8).
pub fn extendsfdf2(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame_sd(c, frame, 0x20, c.g[RA][0]);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.g[SP][0];
        UNPACK_F.call(c, 0x0041_3b10)?;
        c.g[A3][0] = Addr(frame.at(c, 0xc)).lw(c);
        c.g[A0][0] = Addr(frame.at(c, 0)).lw(c);
        c.g[A3][0] = c.g[A3][0] << 32;
        c.g[A1][0] = Addr(frame.at(c, 4)).lw(c);
        c.g[A2][0] = Addr(frame.at(c, 8)).lw(c);
        c.g[A3][0] = c.g[A3][0] >> 2;
        MAKE_DP.call(c, 0x0041_3b2c)?;
        c.g[RA][0] = frame_ld(c, frame, 0x20);
        frame.close(c);
        Ok(())
    });
}


/// `int __cmpsf2(float a, float b)` (0x413860): three-way compare.
pub fn cmpsf2(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x50);
        frame_sd(c, frame, 0x30, c.g[S0][0]);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        frame_sd(c, frame, 0x40, c.g[RA][0]);
        c.g[A1][0] = c.g[SP][0];
        c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.swc1(13, (c.g[SP][0] as u32).wrapping_add(0x24));
        UNPACK_F.call(c, 0x0041_3880)?;
        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x24));
        c.g[A1][0] = c.g[S0][0];
        UNPACK_F.call(c, 0x0041_3890)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[SP][0];
        FPCMP_PARTS_F.call(c, 0x0041_389c)?;
        c.g[RA][0] = frame_ld(c, frame, 0x40);
        c.g[S0][0] = frame_ld(c, frame, 0x30);
        frame.close(c);
        Ok(())
    });
}


// ---------------------------------------------------------------------------
// stdio stream hooks (`struct __sFILE` at `cookie`; the reent pointer is at
// +0x54, the fd at +0xe, the flags halfword at +0xc, the seek position at
// +0x50). These call the reentrant I/O wrappers above.

pub(crate) const READ_R: Callee = Callee { addr: 0x0041_fd10, f: ssx3_ee::s15::f_0041fd10 };

pub(crate) const WRITE_R: Callee = Callee { addr: 0x0041_fdd0, f: ssx3_ee::s15::f_0041fdd0 };

pub(crate) const LSEEK_R: Callee = Callee { addr: 0x0041_fc50, f: ssx3_ee::s15::f_0041fc50 };

pub(crate) const CLOSE_R: Callee = Callee { addr: 0x0041_fb98, f: ssx3_ee::s15::f_0041fb98 };

pub(crate) const GET_THREAD_ID: Callee = Callee { addr: 0x0042_3c90, f: ssx3_ee::s15::f_00423c90 };

pub(crate) const WAIT_SEMA: Callee = Callee { addr: 0x0042_3de0, f: ssx3_ee::s15::f_00423de0 };

pub(crate) const SIGNAL_SEMA: Callee = Callee { addr: 0x0042_3dc0, f: ssx3_ee::s15::f_00423dc0 };


/// `int __sread(void *cookie, char *buf, int n)` (0x417948): `_read_r` on the
/// stream's fd; on error clear flag 0x1000 in the stream flags, else advance
/// the seek position by the bytes read.
pub fn sread(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        let cookie = c.g[A0][0];
        frame.save(c, S0, 0x10);
        c.g[A3][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = cookie;
        c.g[A2][0] = c.g[A1][0];
        c.g[A0][0] = Addr(cookie as u32).off(0x54).lw(c);
        c.g[A1][0] = Addr(cookie as u32).off(0xe).lh(c);
        READ_R.call(c, 0x0041_7970)?;
        c.g[V1][0] = (c.g[V0][0] as u32 as i32 as i64) as u64;
        if (c.g[V1][0] as i64) < 0 {
            c.g[V0][0] = Addr(cookie as u32).off(0xc).lhu(c);
            c.cycles += 2;
            c.g[V0][0] &= 0xefff;
            Addr(cookie as u32).off(0xc).sh(c, c.g[V0][0]);
        } else {
            c.cycles += 4;
            let pos = Addr(cookie as u32).off(0x50).lw(c);
            c.g[V0][0] = sx((pos as u32).wrapping_add(c.g[V1][0] as u32));
            Addr(cookie as u32).off(0x50).sw(c, c.g[V0][0]);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = c.g[V1][0];
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int __swrite(void *cookie, const char *buf, int n)` (0x4179b0): when the
/// stream is in append mode (flag 0x100), seek to the end first, then
/// `_write_r`; clear flag 0x1000 and return the sign-extended count.
pub fn swrite(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        let cookie = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = cookie;
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(cookie as u32).off(0xc).lhu(c);
        c.g[V0][0] &= 0x100;
        c.g[S2][0] = c.g[A2][0];
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            c.g[A0][0] = Addr(cookie as u32).off(0x54).lw(c);
            c.g[A2][0] = 0;
            c.g[A1][0] = Addr(cookie as u32).off(0xe).lh(c);
            c.g[A3][0] = 2;
            LSEEK_R.call(c, 0x0041_79f0)?;
        }
        c.cycles += 16;
        let flags = Addr(cookie as u32).off(0xc).lhu(c);
        c.g[V0][0] = flags;
        c.g[A2][0] = c.g[S1][0];
        c.g[A1][0] = Addr(cookie as u32).off(0xe).lh(c);
        c.g[A3][0] = c.g[S2][0];
        c.g[A0][0] = Addr(cookie as u32).off(0x54).lw(c);
        c.g[V0][0] = flags & 0xefff;
        Addr(cookie as u32).off(0xc).sh(c, c.g[V0][0]);
        WRITE_R.call(c, 0x0041_7a10)?;
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = (c.g[V0][0] as u32 as i32 as i64) as u64;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int __sseek(void *cookie, int off, int whence)` (0x417a30):
/// `_lseek_r`; on success store the new position and set flag 0x1000, on
/// failure clear it and return -1.
pub fn sseek(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        let cookie = c.g[A0][0];
        let off = c.g[A1][0];
        frame.save(c, S0, 0x10);
        c.g[A3][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = cookie;
        c.g[A2][0] = off;
        c.g[A0][0] = Addr(cookie as u32).off(0x54).lw(c);
        c.g[A1][0] = Addr(cookie as u32).off(0xe).lh(c);
        LSEEK_R.call(c, 0x0041_7a58)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = u64::MAX;
        let ok = c.g[A0][0] != u64::MAX;
        c.g[V0][0] = Addr(cookie as u32).off(0xc).lhu(c);
        if ok {
            c.cycles += 4;
            c.g[V1][0] = (c.g[A0][0] as u32 as i32 as i64) as u64;
            Addr(cookie as u32).off(0x50).sw(c, c.g[V1][0]);
            c.g[V0][0] |= 0x1000;
        } else {
            c.cycles += 2;
            c.g[V0][0] &= 0xefff;
        }
        c.cycles += 6;
        Addr(cookie as u32).off(0xc).sh(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int __sclose(void *cookie)` (0x417a98): `_close_r` on the stream's fd.
pub fn sclose(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let cookie = c.g[A0][0];
        c.g[A1][0] = Addr(cookie as u32).off(0xe).lh(c);
        c.g[A0][0] = Addr(cookie as u32).off(0x54).lw(c);
        CLOSE_R.call(c, 0x0041_7aac)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// malloc mutex (`0x450000`: owner at +0x4ca8, recursion count at +0x4cac,
// semaphore id at +0x5248).

/// `void __malloc_lock(struct _reent *r)` (0x418ca8): a recursive lock; the
/// first acquisition by a thread waits on the mutex semaphore, and the
/// recursion count is bumped.
pub fn malloc_lock(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = 0x0045_0000;
        GET_THREAD_ID.call(c, 0x0041_8cc0)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[V0][0] = Addr(0x0045_0000).off(0x4ca8).lw(c);
        let not_owner = c.g[V0][0] != c.g[S0][0];
        c.g[V0][0] = 0x0045_0000;
        if not_owner {
            c.cycles += 4;
            c.g[A0][0] = Addr(0x0045_0000).off(0x5248).lw(c);
            WAIT_SEMA.call(c, 0x0041_8ce0)?;
            c.g[V1][0] = 0x0045_0000;
            Addr(0x0045_0000).off(0x4ca8).sw(c, c.g[S0][0]);
        } else {
            c.cycles += 2;
            c.g[V1][0] = 0x0045_0000;
        }
        c.cycles += 8;
        let count = Addr(c.g[V1][0] as u32).off(0x4cac).lw(c);
        c.g[V0][0] = sx((count as u32).wrapping_add(1));
        Addr(c.g[V1][0] as u32).off(0x4cac).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void __malloc_unlock(struct _reent *r)` (0x418d08): drop the recursion
/// count; on the last release clear the owner and signal the mutex semaphore.
pub fn malloc_unlock(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 7;
        c.g[V1][0] = 0x0045_0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(0x0045_0000).off(0x4cac).lw(c);
        frame.save_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        Addr(0x0045_0000).off(0x4cac).sw(c, c.g[V0][0]);
        if c.g[V0][0] == 0 {
            c.cycles += 6;
            c.g[V0][0] = 0x0045_0000;
            c.g[A1][0] = 0x0045_0000;
            c.g[V1][0] = u64::MAX;
            c.g[A0][0] = Addr(0x0045_0000).off(0x5248).lw(c);
            Addr(0x0045_0000).off(0x4ca8).sw(c, u64::MAX);
            SIGNAL_SEMA.call(c, 0x0041_8d3c)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// newlib bignum / bit helpers.

pub(crate) const CMPDF2: Callee = Callee { addr: 0x0041_2500, f: ssx3_ee::s15::f_00412500 };

pub(crate) const SUBDF3: Callee = Callee { addr: 0x0041_1f70, f: ssx3_ee::s15::f_00411f70 };

pub(crate) const ADDDF3: Callee = Callee { addr: 0x0041_1f18, f: ssx3_ee::s15::f_00411f18 };

pub(crate) const MULDF3: Callee = Callee { addr: 0x0041_1fd8, f: ssx3_ee::s15::f_00411fd8 };

pub(crate) const FLOATDIDF: Callee = Callee { addr: 0x0041_1438, f: ssx3_ee::s15::f_00411438 };

pub(crate) const FIXUNSDFSI: Callee = Callee { addr: 0x0041_26a0, f: ssx3_ee::s15::f_004126a0 };

pub(crate) const FIXUNSDFDI: Callee = Callee { addr: 0x0042_d380, f: ssx3_ee::s15::f_0042d380 };


/// `int __hi0bits(uint32 x)` (0x420830): number of leading zero bits, 32
/// when `x` is 0 (the original returns 32 for `x == 0`, and never more).
pub fn hi0bits(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0xffff_ffff_ffff_0000;
    c.g[V0][0] &= c.g[A0][0];
    let t = c.g[V0][0] != 0;
    c.g[A1][0] = 0;
    if !t {
        c.cycles += 2;
        c.g[A1][0] = 0x10;
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 16);
    }
    c.cycles += 4;
    c.g[V0][0] = 0xffff_ffff_ff00_0000;
    c.g[V0][0] &= c.g[A0][0];
    let t = c.g[V0][0] != 0;
    if !t {
        c.cycles += 2;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 8);
    }
    c.cycles += 4;
    c.g[V0][0] = 0xffff_ffff_f000_0000;
    c.g[V0][0] &= c.g[A0][0];
    let t = c.g[V0][0] != 0;
    if !t {
        c.cycles += 2;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 4);
    }
    c.cycles += 4;
    c.g[V0][0] = 0xffff_ffff_c000_0000;
    c.g[V0][0] &= c.g[A0][0];
    let t = c.g[V0][0] != 0;
    if !t {
        c.cycles += 2;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(2));
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
    }
    c.cycles += 2;
    c.g[V1][0] = 0x4000_0000;
    if (c.g[A0][0] as i64) < 0 {
        c.cycles += 1;
        c.g[V0][0] = c.g[A1][0];
    } else {
        c.cycles += 4;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
        c.g[V1][0] = c.g[A0][0] & 0x4000_0000;
        c.g[V0][0] = 0x20;
        if c.g[V1][0] != 0 {
            c.cycles += 1;
            c.g[V0][0] = c.g[A1][0];
        }
    }
    c.cycles += 2;
    abi::ret(c);
}


/// `int __lo0bits(uint32 *x)` (0x4208b8): number of trailing zero bits of
/// `*x`, storing `*x >> n` back (unchanged when `*x` is odd).
pub fn lo0bits(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
    c.g[V0][0] = c.g[V1][0] & 0x7;
    let aligned = c.g[V0][0] == 0;
    c.g[V0][0] = c.g[V1][0] & 0x1;
    if aligned {
        c.cycles += 3;
        c.g[V0][0] = c.g[V1][0] & 0xffff;
        let t = c.g[V0][0] != 0;
        c.g[A1][0] = 0;
        if !t {
            c.cycles += 2;
            c.g[A1][0] = 0x10;
            c.g[V1][0] = sx((c.g[V1][0] as u32) >> 16);
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[V1][0] & 0xff;
        let t = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[V1][0] & 0xf;
        if !t {
            c.cycles += 3;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
            c.g[V1][0] = sx((c.g[V1][0] as u32) >> 8);
            c.g[V0][0] = c.g[V1][0] & 0xf;
        }
        c.cycles += 2;
        let t = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[V1][0] & 0x3;
        if !t {
            c.cycles += 3;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
            c.g[V1][0] = sx((c.g[V1][0] as u32) >> 4);
            c.g[V0][0] = c.g[V1][0] & 0x3;
        }
        c.cycles += 2;
        let t = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[V1][0] & 0x1;
        if !t {
            c.cycles += 3;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(2));
            c.g[V1][0] = sx((c.g[V1][0] as u32) >> 2);
            c.g[V0][0] = c.g[V1][0] & 0x1;
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            Addr(c.g[A0][0] as u32).sw(c, c.g[V1][0]);
            c.cycles += 2;
            c.g[V0][0] = c.g[A1][0];
            abi::ret(c);
            return;
        }
        c.cycles += 3;
        c.g[V1][0] = sx((c.g[V1][0] as u32) >> 1);
        let nonzero = c.g[V1][0] != 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
        if nonzero {
            c.cycles += 1;
            Addr(c.g[A0][0] as u32).sw(c, c.g[V1][0]);
            c.cycles += 2;
            c.g[V0][0] = c.g[A1][0];
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0x20;
        }
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    let odd = c.g[V0][0] == 0;
    c.g[V0][0] = c.g[V1][0] & 0x2;
    if odd {
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.g[V1][0] = sx((c.g[V1][0] as u32) >> 2);
            c.cycles += 3;
            c.g[V0][0] = 0x2;
        } else {
            c.cycles += 4;
            c.g[V1][0] = sx((c.g[V1][0] as u32) >> 1);
            c.g[V0][0] = 0x1;
        }
        Addr(c.g[A0][0] as u32).sw(c, c.g[V1][0]);
        abi::ret(c);
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
    }
}


/// `int __mcmp(struct _Bigint *a, struct _Bigint *b)` (0x420e28): compare the
/// magnitudes from the most significant component down; returns the
/// difference of the lengths when they differ.
pub fn mcmp(c: &mut Cpu) {
    c.cycles += 5;
    let a = c.g[A0][0] as u32;
    let b = c.g[A1][0] as u32;
    c.g[V1][0] = Addr(b).off(0x10).lw(c);
    c.g[V0][0] = Addr(a).off(0x10).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
    let same = c.g[V0][0] == 0;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    if !same {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 5;
    c.g[A3][0] = sx(a.wrapping_add(0x14));
    c.g[V0][0] = sx(b.wrapping_add(0x14));
    c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(4));
    loop {
        c.cycles += 5;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(4));
        c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
        if c.g[V0][0] != c.g[A0][0] {
            c.g[A0][0] = (c.g[V0][0] < c.g[A0][0]) as u64;
            c.cycles += 4;
            c.g[V1][0] = 0x1;
            c.g[V0][0] = u64::MAX;
            if c.g[A0][0] == 0 {
                c.g[V0][0] = c.g[V1][0];
            }
            abi::ret(c);
            return;
        }
        c.cycles += 3;
        c.g[V0][0] = (c.g[A3][0] < c.g[A2][0]) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(4));
        if !more {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            crate::events::deliver_event_here(c, 0x0042_0e68);
        }
    }
}


/// `double __ulp(double x)` (0x421020): the unit in the last place of `x` —
/// the smallest power of two that changes it (a small positive value; 0 for
/// out-of-range input).
pub fn ulp(c: &mut Cpu) {
    c.cycles += 7;
    c.g[A0][0] = ((c.g[A0][0] as i64) >> 32) as u64;
    c.g[V0][0] = 0x7ff0_0000;
    c.g[A0][0] &= c.g[V0][0];
    c.g[V1][0] = 0xffff_ffff_fcc0_0000;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    let small = (c.g[A0][0] as i64) <= 0;
    c.g[V0][0] = sx(0u32.wrapping_sub(c.g[A0][0] as u32));
    if small {
        c.cycles += 4;
        c.g[A0][0] = sx((((c.g[V0][0] as u32) as i32) >> 20) as u32);
        c.g[V1][0] = ((c.g[A0][0] as i64) < 20) as u64;
        let t = c.g[V1][0] == 0;
        c.g[V0][0] = 0x8_0000;
        if t {
            // exponent far from the boundary: subtract 20 and shift.
            c.cycles += 6;
            c.g[V1][0] = u64::MAX >> 32;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x14));
            c.g[V0][0] = ((c.g[A0][0] as i64) < 31) as u64;
            let t = c.g[V0][0] == 0;
            c.g[A1][0] &= c.g[V1][0];
            if t {
                c.cycles += 1;
                c.g[V0][0] = 0x1;
            } else {
                c.cycles += 5;
                c.g[V0][0] = 0x1f;
                c.g[V1][0] = 0x1;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                c.g[V0][0] = sx((c.g[V1][0] as u32) << (c.g[V0][0] as u32 & 31));
            }
            c.cycles += 6;
            c.g[V0][0] <<= 32;
            c.g[V1][0] = u64::MAX << 32;
            c.g[V0][0] >>= 32;
            c.g[A1][0] &= c.g[V1][0];
            c.g[A1][0] |= c.g[V0][0];
        } else {
            // exponent near the boundary: shift the 0x80000 bit right.
            c.cycles += 10;
            c.g[V1][0] = u64::MAX >> 32;
            c.g[V0][0] = sx((((c.g[V0][0] as u32) as i32) >> (c.g[A0][0] as u32 & 31)) as u32);
            c.g[A1][0] &= c.g[V1][0];
            c.g[V0][0] <<= 32;
            c.g[V1][0] = u64::MAX << 32;
            c.g[A1][0] |= c.g[V0][0];
            c.g[A1][0] &= c.g[V1][0];
        }
    } else {
        c.cycles += 9;
        c.g[V0][0] = u64::MAX >> 32;
        c.g[A0][0] <<= 32;
        c.g[A1][0] &= c.g[V0][0];
        c.g[V1][0] = u64::MAX << 32;
        c.g[A1][0] |= c.g[A0][0];
        c.g[A1][0] &= c.g[V1][0];
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A1][0];
    abi::ret(c);
}


/// `long long __fixdfdi(double x)` (0x42d320): truncate `x` toward zero,
/// negating the unsigned conversion for negative `x`.
pub fn fixdfdi(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        // `sd` (not `sq`): this function saves only the low 64 bits.
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.sd(frame.at(c, 0x0), c.g[S0][0]);
        c.g[S1][0] = 0;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[A1][0] = c.g[S1][0];
        CMPDF2.call(c, 0x0042_d340)?;
        let neg = (c.g[V0][0] as i64) < 0;
        c.g[A0][0] = c.g[S1][0];
        if neg {
            c.cycles += 6;
            c.g[A1][0] = c.g[S0][0];
            SUBDF3.call(c, 0x0042_d350)?;
            c.g[A0][0] = c.g[V0][0];
            FIXUNSDFDI.call(c, 0x0042_d358)?;
            c.g[V0][0] = 0u64.wrapping_sub(c.g[V0][0]);
        } else {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            FIXUNSDFDI.call(c, 0x0042_d368)?;
        }
        c.cycles += 5;
        frame.restore_ra(c, 0x20);
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0x0));
        frame.close(c);
        Ok(())
    })
}


/// `unsigned long long __fixunsdfdi(double x)` (0x42d380): truncate a
/// non-negative `x`; negative input gives 0.
pub fn fixunsdfdi(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = 0;
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.g[S1][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x30), c.g[RA][0]);
        c.sd(frame.at(c, 0x20), c.g[S2][0]);
        c.sd(frame.at(c, 0x0), c.g[S0][0]);
        CMPDF2.call(c, 0x0042_d3a0)?;
        let neg = (c.g[V0][0] as i64) < 0;
        c.g[V0][0] = 0;
        if !neg {
            c.cycles += 9;
            c.g[A1][0] = 0xf7c0;
            c.g[A1][0] <<= 46; // 2^46 as a double
            c.g[A0][0] = c.g[S1][0];
            MULDF3.call(c, 0x0042_d3b8)?;
            c.g[A0][0] = c.g[V0][0];
            FIXUNSDFSI.call(c, 0x0042_d3c0)?;
            c.g[S0][0] = c.g[V0][0] << 32;
            let high_set = (c.g[S0][0] as i64) < 0;
            c.g[V0][0] = c.g[S0][0] & 1;
            if !high_set {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                FLOATDIDF.call(c, 0x0042_d3d4)?;
                c.g[A0][0] = c.g[S1][0];
            } else {
                c.cycles += 7;
                c.g[A0][0] = c.g[S0][0] >> 1;
                c.g[A0][0] = c.g[V0][0] | c.g[A0][0];
                FLOATDIDF.call(c, 0x0042_d3e8)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[A0][0];
                ADDDF3.call(c, 0x0042_d3f4)?;
                c.g[A0][0] = c.g[S1][0];
            }
            c.cycles += 9;
            c.g[A1][0] = c.g[V0][0];
            SUBDF3.call(c, 0x0042_d400)?;
            c.g[S2][0] = 0;
            c.g[S1][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.g[S1][0];
            CMPDF2.call(c, 0x0042_d414)?;
            let nonneg = (c.g[V0][0] as i64) >= 0;
            c.g[A0][0] = c.g[S2][0];
            if nonneg {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                FIXUNSDFSI.call(c, 0x0042_d444)?;
                c.g[V0][0] <<= 32;
                c.g[V0][0] >>= 32;
                c.g[S0][0] = c.g[S0][0].wrapping_add(c.g[V0][0]);
            } else {
                c.cycles += 8;
                c.g[A1][0] = c.g[S1][0];
                SUBDF3.call(c, 0x0042_d424)?;
                c.g[A0][0] = c.g[V0][0];
                FIXUNSDFSI.call(c, 0x0042_d42c)?;
                c.g[V0][0] <<= 32;
                c.g[V0][0] >>= 32;
                c.g[S0][0] = c.g[S0][0].wrapping_sub(c.g[V0][0]);
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 6;
        frame.restore_ra(c, 0x30);
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0x0));
        frame.close(c);
        Ok(())
    })
}


/// `int __sflags(struct _reent *r, const char *mode, int *optr)` (0x420180):
/// decode a stdio mode ("r", "w", "a", optionally with "+"): returns the
/// `__SL64`/`__SWR`/`__SRD` flags and stores the open flags through `optr`.
pub fn sflags(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = Addr(c.g[A1][0] as u32).lb(c);
    c.g[V0][0] = 0x72; // 'r'
    let is_r = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = (c.g[V1][0] < 115) as u64; // < 's'
    if is_r {
        c.cycles += 4;
        c.g[T1][0] = 0x4;
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
    } else {
        c.cycles += 2; // entry into the first-char class block
        if c.g[V0][0] != 0 {
            // first char below 's': only 'a' remains.
            c.cycles += 2;
            let is_a = c.g[V1][0] == 0x61; // 'a'
            c.g[V1][0] = 0x16;
            if is_a {
                c.cycles += 4;
                c.g[T1][0] = 0x108;
                c.g[A3][0] = 1;
                c.g[T0][0] = 0x208;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
                c.cycles += 2;
                Addr(c.g[A0][0] as u32).sw(c, c.g[V1][0]);
                abi::ret(c);
                return;
            }
        } else {
            // first char at or above 's': only 'w' remains.
            c.cycles += 3;
            c.g[V0][0] = 0x77; // 'w'
            let is_w = c.g[V1][0] == c.g[V0][0];
            c.g[V1][0] = 0x16;
            if is_w {
                c.cycles += 4;
                c.g[T1][0] = 0x8;
                c.g[A3][0] = 1;
                c.g[T0][0] = 0x600;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
                c.cycles += 2;
                Addr(c.g[A0][0] as u32).sw(c, c.g[V1][0]);
                abi::ret(c);
                return;
            }
        }
    }
    // The "+" (update) suffix, at mode[1] or mode[2].
    c.cycles += 4;
    c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(1)).lb(c);
    c.g[V1][0] = 0x2b; // '+'
    if c.g[V0][0] == c.g[V1][0] {
        c.g[T1][0] = 0x10;
        c.cycles += 2;
        c.g[A3][0] = 2;
        c.g[V1][0] = c.g[A3][0] | c.g[T0][0];
    } else {
        c.cycles += 3;
        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(2)).lb(c);
        let not_plus = c.g[V0][0] != c.g[V1][0];
        c.g[V1][0] = c.g[A3][0] | c.g[T0][0];
        if !not_plus {
            c.cycles += 1;
            c.g[T1][0] = 0x10;
            c.cycles += 2;
            c.g[A3][0] = 2;
            c.g[V1][0] = c.g[A3][0] | c.g[T0][0];
        }
    }
    c.cycles += 3;
    c.g[V0][0] = c.g[T1][0];
    Addr(c.g[A2][0] as u32).sw(c, c.g[V1][0]);
    abi::ret(c);
}


// ---------------------------------------------------------------- libgcc

/// `count_leading_zeros` of libgcc's `longlong.h` (the C version: range
/// tests pick a byte, `__clz_tab` gives its bit length):
/// `32 - (bitlen(x >> a) + a)`. Charges the second range-test block (+6
/// above 0xffff, else +4) and the table block (+10); the caller charges the
/// first test. (`__clz_tab` is constant data, the bit length is computed.)
pub(crate) fn clz_tab(c: &mut Cpu, x: u32) -> u32 {
    let a = if x > 0xffff {
        c.cycles += 6;
        if x > 0x00ff_ffff {
            24
        } else {
            16
        }
    } else {
        c.cycles += 4;
        if x < 0x100 {
            0
        } else {
            8
        }
    };
    c.cycles += 10;
    let byte = x >> a;
    32 - ((32 - byte.leading_zeros()) + a)
}


/// One 16-bit digit of `__udiv_qrnnd_c` (`d` normalized, `top < d`):
/// `(top << 16 | part) / d` from the estimate `top / (d >> 16)`, corrected
/// at most twice. Returns the digit and the remainder. `block` = cycles of
/// the block that takes the estimate (the `divu` itself is charged by the
/// caller's block), `retry` = cycles of the second correction.
pub(crate) fn qrnnd_step(c: &mut Cpu, top: u32, part: u32, d: u32, block: u64, retry: u64) -> (u32, u32) {
    c.cycles += block;
    let (mut q, r) = abi::divu(c, top, d >> 16);
    let m = abi::mult(c, q, d & 0xffff);
    let mut r = (r << 16) | part;
    if r < m {
        c.cycles += 4;
        q = q.wrapping_sub(1);
        r = r.wrapping_add(d);
        if r < d {
            // carry out of `r + d`: certainly big enough
            c.cycles += 1;
        } else {
            c.cycles += 3;
            if r < m {
                c.cycles += retry + 1;
                q = q.wrapping_sub(1);
                r = r.wrapping_add(d);
            }
        }
    }
    (q, r.wrapping_sub(m))
}


/// The last digit of a `__udiv_qrnnd_c` whose remainder is not used (the
/// compiler drops it and adds `d` speculatively): 13 cycles, +3 per
/// correction test.
pub(crate) fn qrnnd_step_quotient(c: &mut Cpu, top: u32, part: u32, d: u32) -> u32 {
    c.cycles += 13;
    let (mut q, r) = abi::divu(c, top, d >> 16);
    let m = abi::mult(c, q, d & 0xffff);
    let r = (r << 16) | part;
    if r < m {
        c.cycles += 3;
        q = q.wrapping_sub(1);
        let r = r.wrapping_add(d);
        if r >= d {
            c.cycles += 3;
            if r < m {
                q = q.wrapping_sub(1);
            }
        }
    }
    q
}


/// `u64 __udivdi3(u64 n, u64 d)` (0x411538): libgcc's `__udivmoddi4`
/// without remainder (`UDIV_NEEDS_NORMALIZATION`, 16-bit digit division).
/// Returns the quotient in v0; v1 is left as `q_low << 32`. Division by zero
/// gives what the EE's `divu` by zero gives (`d0 = 1 / 0 = 0xffffffff`).
pub fn udivdi3(c: &mut Cpu) {
    c.cycles += 8;
    let (n, d) = (c.g[A0][0], c.g[A1][0]);
    let (n1, n0, d1, d0) = ((n >> 32) as u32, n as u32, (d >> 32) as u32, d as u32);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    // (high quotient word, low quotient word, cycles of the exit blocks)
    let (q1, q0, exit): (u32, u32, u64) = if d1 == 0 {
        c.cycles += 3;
        if d0 > n1 {
            // 0q = nn / 0D
            c.cycles += 3;
            let bm = clz_tab(c, d0);
            let (mut d0, mut n1, mut n0) = (d0, n1, n0);
            if bm != 0 {
                c.cycles += 5;
                n1 = (n1 << bm) | (n0 >> (32 - bm));
                d0 <<= bm;
                n0 <<= bm;
            }
            c.cycles += 6;
            let (hi, r) = qrnnd_step(c, n1, n0 >> 16, d0, 11, 3);
            c.cycles += 2;
            let lo = qrnnd_step_quotient(c, r, n0 & 0xffff, d0);
            c.cycles += 5;
            (0, hi << 16 | lo, 8)
        } else {
            // qq = NN / 0d
            c.cycles += 2;
            let mut d0 = d0;
            if d0 == 0 {
                // `d0 = 1 / d0`: the intentional division by zero.
                c.cycles += 3 + 6;
                d0 = abi::divu(c, 1, d1).0;
            }
            c.cycles += 2;
            let bm = clz_tab(c, d0);
            let (mut n1, mut n0) = (n1, n0);
            let q1;
            if bm == 0 {
                c.cycles += 5;
                n1 = n1.wrapping_sub(d0);
                q1 = 1;
            } else {
                c.cycles += 13;
                let b = 32 - bm;
                d0 <<= bm;
                let n2 = n1 >> b;
                n1 = (n1 << bm) | (n0 >> b);
                n0 <<= bm;
                let (hi, r) = qrnnd_step(c, n2, n1 >> 16, d0, 12, 3);
                c.cycles += 2;
                let (lo, r) = qrnnd_step(c, r, n1 & 0xffff, d0, 13, 2);
                c.cycles += 4;
                q1 = hi << 16 | lo;
                n1 = r;
            }
            c.cycles += 6;
            let (hi, r) = qrnnd_step(c, n1, n0 >> 16, d0, 11, 2);
            c.cycles += 2;
            let lo = qrnnd_step_quotient(c, r, n0 & 0xffff, d0);
            c.cycles += 5;
            (q1, hi << 16 | lo, 6)
        }
    } else {
        c.cycles += 3;
        if d1 > n1 {
            // 00 = nn / DD
            c.cycles += 2;
            (0, 0, 8)
        } else {
            // 0q = NN / dd
            c.cycles += 3;
            let bm = clz_tab(c, d1);
            if bm == 0 {
                c.cycles += 3;
                let q0 = if n1 > d1 {
                    1
                } else {
                    c.cycles += 3;
                    if n0 < d0 {
                        0
                    } else {
                        c.cycles += 2;
                        1
                    }
                };
                (0, q0, 8)
            } else {
                c.cycles += 15;
                let b = 32 - bm;
                let d1 = (d1 << bm) | (d0 >> b);
                let d0 = d0 << bm;
                let n2 = n1 >> b;
                let n1 = (n1 << bm) | (n0 >> b);
                let n0 = n0 << bm;
                let (hi, r) = qrnnd_step(c, n2, n1 >> 16, d1, 11, 3);
                c.cycles += 2;
                let (lo, n1) = qrnnd_step(c, r, n1 & 0xffff, d1, 13, 2);
                c.cycles += 10;
                let q0 = hi << 16 | lo;
                // umul_ppmm(m1, m0, q0, d0): one too many if q0*d0 > n1:n0
                abi::multu(c, q0, d0);
                let p = q0 as u64 * d0 as u64;
                let (m1, m0) = ((p >> 32) as u32, p as u32);
                if m1 > n1 {
                    (0, q0.wrapping_sub(1), 8)
                } else {
                    c.cycles += 2;
                    if m1 != n1 {
                        (0, q0, 6)
                    } else {
                        c.cycles += 3;
                        if m0 > n0 {
                            c.cycles += 1;
                            (0, q0.wrapping_sub(1), 8)
                        } else {
                            (0, q0, 5)
                        }
                    }
                }
            }
        }
    };
    c.cycles += exit;
    c.g[V1][0] = (q0 as u64) << 32;
    c.g[V0][0] = (q1 as u64) << 32 | q0 as u64;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    abi::ret(c);
}


/// A remainder-only digit step of `__umoddi3`'s inlined `udiv_qrnnd` (the
/// high digit of the `n2:n1 / d0` division): estimate block of 11 cycles,
/// corrections +4, then +3 and +1 (+1 more without a second correction).
/// Returns `r - m`. The `divu` is charged by the caller's block.
pub(crate) fn mod_step_first(c: &mut Cpu, top: u32, part: u32, d: u32) -> u32 {
    c.cycles += 11;
    let (q, r) = abi::divu(c, top, d >> 16);
    let m = abi::mult(c, q, d & 0xffff);
    let mut r = (r << 16) | part;
    if r < m {
        c.cycles += 4;
        r = r.wrapping_add(d);
        if r >= d {
            c.cycles += 3;
            if r < m {
                r = r.wrapping_add(d);
            } else {
                c.cycles += 1;
            }
            c.cycles += 1;
        }
    }
    r.wrapping_sub(m)
}


/// The other remainder-only digit steps of `__umoddi3`: estimate block of
/// 10 cycles, corrections +4, then +3 +1. Returns `r - m`.
pub(crate) fn mod_step(c: &mut Cpu, top: u32, part: u32, d: u32) -> u32 {
    c.cycles += 10;
    let (q, r) = abi::divu(c, top, d >> 16);
    let m = abi::mult(c, q, d & 0xffff);
    let mut r = (r << 16) | part;
    if r < m {
        c.cycles += 4;
        r = r.wrapping_add(d);
        if r >= d {
            c.cycles += 3 + 1;
            if r < m {
                r = r.wrapping_add(d);
            }
        }
    }
    r.wrapping_sub(m)
}


/// The last digit step of `__umoddi3`'s `d1 == 0` path: estimate block of
/// 10 cycles, corrections +4 then +2. Returns `(r - m, r)`; the corrected
/// `r` stays in v1.
pub(crate) fn mod_step_last(c: &mut Cpu, top: u32, part: u32, d: u32) -> (u32, u32) {
    c.cycles += 10;
    let (q, r) = abi::divu(c, top, d >> 16);
    let m = abi::mult(c, q, d & 0xffff);
    let mut r = (r << 16) | part;
    if r < m {
        c.cycles += 4;
        r = r.wrapping_add(d);
        if r >= d {
            c.cycles += 2;
            if r < m {
                r = r.wrapping_add(d);
            }
        }
    }
    (r.wrapping_sub(m), r)
}


/// `u64 __umoddi3(u64 n, u64 d)` (0x42dad8): libgcc's `__udivmoddi4` for the
/// remainder only. The remainder goes through a stack slot (`rp = sp`), so
/// it is also left at `sp - 0x30`; v1 keeps a path-dependent intermediate
/// (the last partial remainder, `n1 << 32`, or `n1 - m1`).
pub fn umoddi3(c: &mut Cpu) {
    c.cycles += 9;
    let (n, d) = (c.g[A0][0], c.g[A1][0]);
    let (n1, n0, d1, d0) = ((n >> 32) as u32, n as u32, (d >> 32) as u32, d as u32);
    let sp = (c.g[SP][0] as u32).wrapping_sub(0x30);
    c.g[SP][0] = sx(sp);
    // (remainder, v1 left behind, cycles of the store blocks)
    let (rem, v1, tail): (u64, u64, u64) = if d1 == 0 {
        c.cycles += 3;
        let (bm, d0, n1, n0) = if d0 > n1 {
            // 0q = nn / 0D
            c.cycles += 3;
            let bm = clz_tab(c, d0);
            let (mut d0, mut n1, mut n0) = (d0, n1, n0);
            if bm != 0 {
                c.cycles += 5;
                n1 = (n1 << bm) | (n0 >> (32 - bm));
                d0 <<= bm;
                n0 <<= bm;
            }
            c.cycles += 4;
            (bm, d0, n1, n0)
        } else {
            // qq = NN / 0d: reduce n1 first
            c.cycles += 2;
            let mut d0 = d0;
            if d0 == 0 {
                // `d0 = 1 / d0`: the intentional division by zero.
                c.cycles += 3 + 5;
                d0 = abi::divu(c, 1, d1).0;
            }
            c.cycles += 2;
            let bm = clz_tab(c, d0);
            let (mut n1, mut n0) = (n1, n0);
            if bm == 0 {
                c.cycles += 4;
                n1 = n1.wrapping_sub(d0);
            } else {
                c.cycles += 13;
                let b = 32 - bm;
                d0 <<= bm;
                let n2 = n1 >> b;
                n1 = (n1 << bm) | (n0 >> b);
                n0 <<= bm;
                let r = mod_step_first(c, n2, n1 >> 16, d0);
                c.cycles += 4;
                n1 = mod_step(c, r, n1 & 0xffff, d0);
            }
            c.cycles += 3;
            (bm, d0, n1, n0)
        };
        c.cycles += 3;
        let r = mod_step(c, n1, n0 >> 16, d0);
        c.cycles += 4;
        let (r0, last) = mod_step_last(c, r, n0 & 0xffff, d0);
        c.cycles += 2 + 4;
        ((r0 >> bm) as u64, sx(last), 1 + 3)
    } else {
        c.cycles += 3;
        if n1 < d1 {
            // 00 = nn / DD: the remainder is n
            c.cycles += 5;
            (n, (n1 as u64) << 32, 3)
        } else {
            c.cycles += 4;
            let bm = clz_tab(c, d1);
            if bm == 0 {
                c.cycles += 3;
                let (mut n1, mut n0) = (n1, n0);
                let sub = if n1 > d1 {
                    true
                } else {
                    c.cycles += 3;
                    n0 >= d0
                };
                if sub {
                    c.cycles += 4;
                    let low = n0.wrapping_sub(d0);
                    n1 = n1.wrapping_sub(d1).wrapping_sub((n0 < low) as u32);
                    n0 = low;
                }
                c.cycles += 2 + 4;
                ((n1 as u64) << 32 | n0 as u64, (n1 as u64) << 32, 1 + 3)
            } else {
                c.cycles += 15;
                let b = 32 - bm;
                let d1 = (d1 << bm) | (d0 >> b);
                let d0 = d0 << bm;
                let n2 = n1 >> b;
                let n1 = (n1 << bm) | (n0 >> b);
                let n0 = n0 << bm;
                let (hi, r) = qrnnd_step(c, n2, n1 >> 16, d1, 11, 3);
                c.cycles += 2;
                let (lo, n1) = qrnnd_step(c, r, n1 & 0xffff, d1, 13, 2);
                c.cycles += 11;
                let q0 = hi << 16 | lo;
                abi::multu(c, q0, d0);
                let p = q0 as u64 * d0 as u64;
                let (mut m1, mut m0) = ((p >> 32) as u32, p as u32);
                let too_big = if m1 > n1 {
                    true
                } else {
                    c.cycles += 2;
                    if m1 != n1 {
                        false
                    } else {
                        c.cycles += 2;
                        if m0 > n0 {
                            c.cycles += 1;
                            true
                        } else {
                            false
                        }
                    }
                };
                if too_big {
                    // sub_ddmmss(m1, m0, m1, m0, d1, d0)
                    c.cycles += 4;
                    let low = m0.wrapping_sub(d0);
                    m1 = m1.wrapping_sub(d1).wrapping_sub((m0 < low) as u32);
                    m0 = low;
                }
                c.cycles += 2 + 12;
                let low = n0.wrapping_sub(m0);
                let high = n1.wrapping_sub(m1).wrapping_sub((n0 < low) as u32);
                let rem = ((high >> bm) as u64) << 32 | ((high << b) | (low >> bm)) as u64;
                (rem, sx(n1.wrapping_sub(m1)), 1 + 3)
            }
        }
    };
    c.cycles += tail;
    Addr(sp).sd(c, rem);
    c.g[V0][0] = Addr(sp).ld(c);
    c.g[V1][0] = v1;
    c.g[SP][0] = sx(sp.wrapping_add(0x30));
    abi::ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::check;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fill;
    use crate::difftest::scramble;
    use crate::difftest::put_words;
    use ssx3_ee as ee;


    pub(crate) const SEEDS: u32 = 200;


    /// Scrambled registers, fake hooks and a game-like `gp`.
    pub(crate) fn plain(gen: GuestFn, hand: GuestFn, salt: u64) {
        check(gen, hand, SEEDS, &|c, seed| {
            machine(c, seed, salt, 0);
            c.g[GP][0] = GAME_GP as u64;
        });
    }


    // -------------------------------------------------------- ctor/dtor walks

    #[test]
    fn do_global_dtors_aux_matches() {
        plain(ssx3_ee::s00::f_00100218, do_global_dtors_aux, 0x100);
    }


    #[test]
    fn do_global_dtors_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let _ = machine(c, seed, 0x101, 1);
            let p = DATA + 0x200;
            if seed % 4 == 0 {
                c.bus.write32(0x0045_343c, 0);
            } else {
                c.bus.write32(0x0045_343c, p);
                c.bus.write32(p, FAKE);
                c.bus.write32(p + 4, 0);
            }
        };
        covered(ssx3_ee::s15::f_0040fb88, do_global_dtors, SEEDS, &setup);
    }


    #[test]
    fn do_global_ctors_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let _ = machine(c, seed, 0x102, 1);
            match seed % 3 {
                0 => {
                    c.bus.write32(0x0043_ce38, 0xffff_ffff);
                }
                1 => {
                    c.bus.write32(0x0043_ce38, 1);
                    c.bus.write32(0x0043_ce3c, 0);
                }
                _ => {
                    c.bus.write32(0x0043_ce38, 1);
                    c.bus.write32(0x0043_ce3c, FAKE);
                    c.bus.write32(0x0043_ce40, 0);
                }
            }
        };
        covered(ssx3_ee::s15::f_0040fbe0, do_global_ctors, SEEDS, &setup);
    }


    #[test]
    fn main_matches() {
        check(ssx3_ee::s15::f_0040fc90, main_, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x103, 0);
            c.bus.write32(0x0052_ad98, if seed % 3 == 0 { 0 } else { r.next_u32() | 1 });
        });
    }


    // ------------------------------------------------------ terminate / eh

    #[test]
    fn default_terminate_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let _ = machine(c, seed, 0x104, 2);
        };
        covered(ssx3_ee::s15::f_004103a0, default_terminate, SEEDS, &setup);
    }


    #[test]
    fn terminate_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let _ = machine(c, seed, 0x105, 1);
        };
        covered(ssx3_ee::s15::f_004103b0, terminate, SEEDS, &setup);
    }


    #[test]
    fn throw_type_match_matches() {
        check(ssx3_ee::s15::f_004103d8, throw_type_match, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x106, 0);
            c.bus.write32(DATA, 0);
            for i in 0..0x30u32 {
                let b = (r.next_u32() & 0x3f) as u8 + 1;
                c.bus.write8(DATA + i, b);
                let b2 = if seed % 3 == 0 { b } else { (r.next_u32() & 0x3f) as u8 + 1 };
                c.bus.write8(DATA + 0x40 + i, b2);
            }
            c.bus.write8(DATA + 0x30, 0);
            c.bus.write8(DATA + 0x70, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x40) as u64;
            c.g[A2][0] = (DATA + 0x80) as u64;
        });
    }


    #[test]
    fn get_eh_context_matches() {
        covered(
            ssx3_ee::s15::f_00410468,
            get_eh_context,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x107, 1);
            },
        );
    }


    #[test]
    fn get_eh_info_matches() {
        covered(ssx3_ee::s15::f_00410490, get_eh_info, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x108, 1);
        });
    }


    #[test]
    fn get_dynamic_handler_chain_matches() {
        covered(
            ssx3_ee::s15::f_00410548,
            get_dynamic_handler_chain,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x109, 1);
            },
        );
    }


    #[test]
    fn cp_exception_info_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let _ = machine(c, seed, 0x10a, 2);
            c.pc = 0x0041_2848;
        };
        covered(ssx3_ee::s15::f_00412848, cp_exception_info, SEEDS, &setup);
    }


    #[test]
    fn cp_eh_info_matches() {
        covered(ssx3_ee::s15::f_00412888, cp_eh_info, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x10b, 1);
        });
    }


    #[test]
    fn start_cp_handler_matches() {
        check(ssx3_ee::s15::f_004128a8, start_cp_handler, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x10c, 1);
            let p = DATA + 0x300;
            c.bus.write32(0x0045_3444, FAKE);
            // The fake returns `p`; make the dereference land in RAM.
            c.bus.write32(LOG_RESULTS, p);
        });
    }


    #[test]
    fn eh_alloc_matches() {
        check(ssx3_ee::s15::f_004128e0, eh_alloc, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x10d, 0);
            // A small request keeps the real allocator on its fast path.
            c.g[A0][0] = ((seed % 4) * 8 + 8) as u64;
        });
    }


    #[test]
    fn eh_free_matches() {
        plain(ssx3_ee::s15::f_00412908, eh_free, 0x10e);
    }


    #[test]
    fn uncatch_exception_matches() {
        check(ssx3_ee::s15::f_00412af0, uncatch_exception, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x10f, 1);
            // Make the fake EH-info pointer land in RAM with a non-null
            // "active handler" so the noreturn terminate path isn't taken.
            c.bus.write32(LOG_RESULTS, DATA + 0x400);
            c.bus.write32(DATA + 0x408, 1);
        });
    }


    // ------------------------------------------------------------- singletons

    pub(crate) fn singleton_test(gen: GuestFn, hand: GuestFn, obj: u32, salt: u64) {
        check(gen, hand, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, salt, 0);
            c.bus.write32(obj, if seed % 3 == 0 { r.next_u32() | 1 } else { 0 });
        });
    }


    pub(crate) fn singleton_test_entries(gen: GuestFn, hand: GuestFn, obj: u32, start: u32, salt: u64) {
        check(gen, hand, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, salt, 0);
            c.bus.write32(obj, if seed % 3 == 0 { r.next_u32() | 1 } else { 0 });
            c.pc = start;
        });
    }


    #[test]
    fn tf13bad_exception_matches() {
        singleton_test(ssx3_ee::s15::f_00412e80, tf13bad_exception, 0x0053_ea10, 0x120);
    }

    #[test]
    fn tf9exception_matches() {
        singleton_test(ssx3_ee::s15::f_00412f18, tf9exception, 0x004a_5ba8, 0x121);
    }

    #[test]
    fn tf17_class_type_info_matches() {
        singleton_test(ssx3_ee::s15::f_00414f00, tf17_class_type_info, 0x0053_ea60, 0x122);
    }

    #[test]
    fn tf14_si_type_info_matches() {
        singleton_test(ssx3_ee::s15::f_00414f90, tf14_si_type_info, 0x0053_ea50, 0x123);
    }

    #[test]
    fn tf16_user_type_info_matches() {
        singleton_test_entries(ssx3_ee::s15::f_00415018, tf16_user_type_info, 0x0053_ea40, 0x0041_5018, 0x124);
    }

    #[test]
    fn tf10bad_typeid_matches() {
        singleton_test(ssx3_ee::s15::f_004150d0, tf10bad_typeid, 0x0053_ea20, 0x125);
    }

    #[test]
    fn tf8bad_cast_matches() {
        singleton_test(ssx3_ee::s15::f_00415168, tf8bad_cast, 0x0053_ea30, 0x126);
    }

    #[test]
    fn tf9type_info_matches() {
        singleton_test(ssx3_ee::s15::f_004151f8, tf9type_info, 0x004a_5bb0, 0x127);
    }

    #[test]
    fn tf17_array_type_info_matches() {
        singleton_test(ssx3_ee::s15::f_00415ca8, tf17_array_type_info, 0x0053_eaa0, 0x128);
    }

    #[test]
    fn tf16_ptmd_type_info_matches() {
        singleton_test(ssx3_ee::s15::f_00415d18, tf16_ptmd_type_info, 0x0053_eac0, 0x129);
    }

    #[test]
    fn tf16_ptmf_type_info_matches() {
        singleton_test(ssx3_ee::s15::f_00415d88, tf16_ptmf_type_info, 0x0053_ea90, 0x12a);
    }

    #[test]
    fn tf16_func_type_info_matches() {
        singleton_test(ssx3_ee::s15::f_00415df8, tf16_func_type_info, 0x0053_ea70, 0x12b);
    }

    #[test]
    fn tf19_builtin_type_info_matches() {
        singleton_test(ssx3_ee::s15::f_00415e68, tf19_builtin_type_info, 0x0053_ead0, 0x12c);
    }

    #[test]
    fn tf16_attr_type_info_matches() {
        singleton_test(ssx3_ee::s15::f_00415ed8, tf16_attr_type_info, 0x0053_ea80, 0x12d);
    }

    #[test]
    fn tf19_pointer_type_info_matches() {
        singleton_test(ssx3_ee::s15::f_00415f48, tf19_pointer_type_info, 0x0053_eab0, 0x12e);
    }


    // --------------------------------------------------------------- rtti

    #[test]
    fn rtti_si_matches() {
        check(ssx3_ee::s15::f_00414c00, rtti_si, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x130, 0);
            let obj = if seed % 3 == 0 { 0 } else { (DATA + (r.next_u32() & 0xff0)) as u64 };
            c.g[A0][0] = obj;
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = r.next_u32() as u64;
        });
    }


    #[test]
    fn rtti_user_matches() {
        check(ssx3_ee::s15::f_00414c20, rtti_user, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x131, 0);
            let obj = if seed % 3 == 0 { 0 } else { (DATA + (r.next_u32() & 0xff0)) as u64 };
            c.g[A0][0] = obj;
            c.g[A1][0] = r.next_u32() as u64;
        });
    }


    #[test]
    fn user_type_info_dcast_matches() {
        check(ssx3_ee::s15::f_00414c40, user_type_info_dcast, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x132, 0);
            for i in 0..4 {
                c.g[[A0, A1, A2, A3][i]][0] = (DATA + (r.next_u32() & 0xff0)) as u64;
            }
            c.pc = 0x0041_4c40;
        });
    }


    // ----------------------------------------------------------- type_info dtor

    pub(crate) fn dtor_test(gen: GuestFn, hand: GuestFn, salt: u64) {
        plain(gen, hand, salt);
    }


    #[test]
    fn class_type_info_dtor_matches() {
        check(ssx3_ee::s15::f_00414f50, class_type_info_dtor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x140, 1);
            c.g[A0][0] = (DATA + (r.next_u32() & 0xff0)) as u64;
        });
    }

    #[test]
    fn si_type_info_dtor_matches() {
        dtor_test(ssx3_ee::s15::f_00414fe0, si_type_info_dtor, 0x141);
    }

    #[test]
    fn user_type_info_dtor_matches() {
        dtor_test(ssx3_ee::s15::f_00415068, user_type_info_dtor, 0x142);
    }

    #[test]
    fn array_type_info_dtor_matches() {
        dtor_test(ssx3_ee::s15::f_00415cf8, array_type_info_dtor, 0x143);
    }

    #[test]
    fn ptmd_type_info_dtor_matches() {
        dtor_test(ssx3_ee::s15::f_00415d68, ptmd_type_info_dtor, 0x144);
    }

    #[test]
    fn ptmf_type_info_dtor_matches() {
        dtor_test(ssx3_ee::s15::f_00415dd8, ptmf_type_info_dtor, 0x145);
    }

    #[test]
    fn func_type_info_dtor_matches() {
        dtor_test(ssx3_ee::s15::f_00415e48, func_type_info_dtor, 0x146);
    }

    #[test]
    fn builtin_type_info_dtor_matches() {
        dtor_test(ssx3_ee::s15::f_00415eb8, builtin_type_info_dtor, 0x147);
    }

    #[test]
    fn attr_type_info_dtor_matches() {
        dtor_test(ssx3_ee::s15::f_00415f28, attr_type_info_dtor, 0x148);
    }

    #[test]
    fn pointer_type_info_dtor_matches() {
        dtor_test(ssx3_ee::s15::f_00415f98, pointer_type_info_dtor, 0x149);
    }


    // ------------------------------------------------------- frame registration

    #[test]
    fn register_frame_info_matches() {
        check(ssx3_ee::s15::f_004146d0, register_frame_info, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x150, 0);
            c.g[A0][0] = (DATA + (r.next_u32() & 0xff0)) as u64;
            c.g[A1][0] = (DATA + 0x1000) as u64;
        });
    }


    #[test]
    fn register_frame_info_table_matches() {
        check(ssx3_ee::s15::f_00414728, register_frame_info_table, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x151, 0);
            c.g[A0][0] = (DATA + (r.next_u32() & 0xff0)) as u64;
            c.g[A1][0] = (DATA + 0x1000) as u64;
        });
    }


    #[test]
    fn register_frame_matches() {
        plain(ssx3_ee::s15::f_004146f8, register_frame, 0x152);
    }


    #[test]
    fn register_frame_table_matches() {
        plain(ssx3_ee::s15::f_00414750, register_frame_table, 0x153);
    }


    #[test]
    fn deregister_frame_matches() {
        plain(ssx3_ee::s15::f_004147f8, deregister_frame, 0x154);
    }


    // ------------------------------------------------------------------ misc

    #[test]
    fn tfv_matches() {
        check(ssx3_ee::s15::f_004158f0, tfv, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x160, 0);
            c.g[A0][0] = r.next_u32() as u64;
        });
    }


    #[test]
    fn getreent_matches() {
        plain(ssx3_ee::s15::f_00415fb8, getreent, 0x161);
    }


    // -------------------------------------------------------------- soft float

    pub(crate) fn float_arg_test(gen: GuestFn, hand: GuestFn, salt: u64) {
        check(gen, hand, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, salt, 0);
            for i in 0..4 {
                c.g[[A0, A1, A2, A3][i]][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            }
            c.f[12] = r.edge_f32();
            c.f[13] = r.edge_f32();
        });
    }


    #[test]
    fn negdf2_matches() {
        float_arg_test(ssx3_ee::s15::f_00412740, negdf2, 0x170);
    }

    #[test]
    fn make_dp_matches() {
        float_arg_test(ssx3_ee::s15::f_00412778, make_dp, 0x171);
    }

    #[test]
    fn truncdfsf2_matches() {
        float_arg_test(ssx3_ee::s15::f_004127a8, truncdfsf2, 0x172);
    }

    #[test]
    fn cmpdf2_matches() {
        float_arg_test(ssx3_ee::s15::f_00412500, cmpdf2, 0x173);
    }

    #[test]
    fn negsf2_matches() {
        float_arg_test(ssx3_ee::s15::f_00413a90, negsf2, 0x174);
    }

    #[test]
    fn make_fp_matches() {
        float_arg_test(ssx3_ee::s15::f_00413ac8, make_fp, 0x175);
    }

    #[test]
    fn extendsfdf2_matches() {
        float_arg_test(ssx3_ee::s15::f_00413af8, extendsfdf2, 0x176);
    }

    #[test]
    fn cmpsf2_matches() {
        float_arg_test(ssx3_ee::s15::f_00413860, cmpsf2, 0x177);
    }


    // -------------------------------------------------------- stream hooks

    /// The common fields of a `FILE` cookie at `DATA + 0x400`: flags, fd,
    /// seek position and the reent pointer; returns the RNG for the caller
    /// to pick buffers and arguments.
    pub(crate) fn stream_cookie(c: &mut Cpu, seed: u32, salt: u64, fd: u16) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        let cookie = DATA + 0x400;
        c.g[A0][0] = cookie as u64;
        let flags = (r.next_u32() as u16) | if seed % 2 == 0 { 0x0100 } else { 0 };
        c.bus.write16(cookie + 0xc, flags);
        c.bus.write16(cookie + 0xe, fd);
        c.bus.write32(cookie + 0x50, r.next_u32());
        c.bus.write32(cookie + 0x54, DATA + 0x800);
        c.g[A1][0] = (DATA + 0x1000) as u64;
        c.g[A2][0] = (r.next_u32() % 0x1000) as u64;
        r
    }


    #[test]
    fn sread_matches() {
        check(ssx3_ee::s15::f_00417948, sread, SEEDS, &|c, seed| {
            let mut r = stream_cookie(c, seed, 30, 1);
            let fd = 1 + (r.next_u32() % 100) as u16;
            c.bus.write16(DATA + 0x400 + 0xe, fd);
        });
    }


    #[test]
    fn swrite_matches() {
        check(ssx3_ee::s15::f_004179b0, swrite, SEEDS, &|c, seed| {
            stream_cookie(c, seed, 31, if seed % 2 == 0 { 0 } else { 3 + (seed as u16 % 8) });
        });
    }


    #[test]
    fn sseek_matches() {
        check(ssx3_ee::s15::f_00417a30, sseek, SEEDS, &|c, seed| {
            stream_cookie(c, seed, 32, (seed as u16 % 32) + 3);
        });
    }


    #[test]
    fn sclose_matches() {
        check(ssx3_ee::s15::f_00417a98, sclose, SEEDS, &|c, seed| {
            stream_cookie(c, seed, 33, (seed as u16 % 32) + 3);
        });
    }


    // --------------------------------------------------------- malloc lock

    #[test]
    fn malloc_lock_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 40, 2);
            // Force GetThreadId's result so the owner-equals-self branch is
            // reachable.
            let tid = 1 + r.next_u32() % 0xffff;
            c.bus.write32(crate::difftest::fakes::LOG_RESULTS, tid);
            c.bus.write32(0x0045_0000 + 0x4ca8, if seed % 2 == 0 { tid } else { r.next_u32() });
            c.bus.write32(0x0045_0000 + 0x4cac, r.next_u32());
            c.bus.write32(0x0045_0000 + 0x5248, r.next_u32());
        };
        covered(ssx3_ee::s15::f_00418ca8, malloc_lock, SEEDS, &setup);
    }


    #[test]
    fn malloc_unlock_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 41, 1);
            c.bus.write32(0x0045_0000 + 0x4cac, if seed % 2 == 0 { 1 } else { 2 + r.next_u32() % 5 });
            c.bus.write32(0x0045_0000 + 0x5248, r.next_u32());
        };
        covered(ssx3_ee::s15::f_00418d08, malloc_unlock, SEEDS, &setup);
    }


    #[test]
    fn hi0bits_matches() {
        check(ssx3_ee::s15::f_00420830, hi0bits, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 61, 0);
            let x = match seed % 6 {
                0 => 0,
                1 => 1,
                2 => 0x8000_0000,
                3 => 0x0001_0000,
                _ => r.next_u32(),
            };
            c.g[A0][0] = x as u64;
        });
    }


    #[test]
    fn lo0bits_matches() {
        check(ssx3_ee::s15::f_004208b8, lo0bits, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 62, 0);
            let x = match seed % 6 {
                0 => 0,
                1 => 1,
                2 => 0x8000_0000,
                3 => 0x0000_0010,
                4 => 0x0000_0004,
                _ => r.next_u32(),
            };
            let cell = DATA + 0x600;
            c.bus.write32(cell, x);
            c.g[A0][0] = cell as u64;
        });
    }


    #[test]
    fn mcmp_matches() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 63, 0);
            let a = DATA + 0x1000;
            let b = DATA + 0x2000;
            // Long enough that the compare loop reaches a scheduled event.
            let ka = 16 + r.next_u32() % 96;
            let kb = if seed % 5 == 0 { 1 + r.next_u32() % 96 } else { ka };
            c.bus.write32(a + 0x10, ka);
            c.bus.write32(b + 0x10, kb);
            let differ_at = if seed % 2 == 0 { u32::MAX } else { r.next_u32() % ka };
            for i in 0..96 {
                let x = r.next_u32();
                let y = if i == differ_at { x ^ (1 << (r.next_u32() % 32)) } else { x };
                c.bus.write32(a + 0x14 + 4 * i, x);
                c.bus.write32(b + 0x14 + 4 * i, y);
            }
            c.g[A0][0] = a as u64;
            c.g[A1][0] = b as u64;
            schedule(c, &mut r, seed, 0x0042_0e28)
        };
        check_events(ssx3_ee::s15::f_00420e28, mcmp, SEEDS, &setup);
    }


    #[test]
    fn ulp_matches() {
        check(ssx3_ee::s15::f_00421020, ulp, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 64, 0);
            let x = match seed % 6 {
                0 => 0,
                1 => 0x3ff0_0000_0000_0000,
                2 => 0x0010_0000_0000_0000,
                3 => 0x7ff0_0000_0000_0000,
                4 => 0x7fef_ffff_ffff_ffff,
                _ => r.next_u32() as u64 | ((r.next_u32() as u64) << 32),
            };
            c.g[A0][0] = x;
            c.g[A1][0] = r.next_u32() as u64;
        });
    }


    #[test]
    fn fixdfdi_matches() {
        check(ssx3_ee::s15::f_0042d320, fixdfdi, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 66, 0);
            let x = match seed % 6 {
                0 => 0x3ff0_0000_0000_0000,
                1 => 0xbff0_0000_0000_0000, // negative
                2 => 0x41e0_0000_0000_0000,
                3 => 0xc1e0_0000_0000_0000,
                _ => r.next_u32() as u64 | ((r.next_u32() as u64) << 32),
            };
            c.g[A0][0] = x;
            c.g[A1][0] = r.next_u32() as u64;
        });
    }


    #[test]
    fn fixunsdfdi_matches() {
        check(ssx3_ee::s15::f_0042d380, fixunsdfdi, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 67, 0);
            let x = match seed % 7 {
                0 => 0x3ff0_0000_0000_0000,
                1 => 0xbff0_0000_0000_0000, // negative
                2 => 0x4000_0000_0000_0000, // high bit of the int part
                3 => 0x4330_0000_0000_0000, // large
                4 => 0x0000_0000_0000_0000,
                _ => r.next_u32() as u64 | ((r.next_u32() as u64) << 32),
            };
            c.g[A0][0] = x;
            c.g[A1][0] = r.next_u32() as u64;
        });
    }


    #[test]
    fn sflags_matches() {
        check(ssx3_ee::s15::f_00420180, sflags, SEEDS, &|c, seed| {
            machine(c, seed, 72, 0);
            let mode = DATA + 0x400;
            let modes: [&[u8]; 10] = [
                b"r", b"w", b"a", b"x", b"s", b"r+", b"w+", b"a+", b"r+b", b"wb",
            ];
            let m = modes[(seed as usize) % modes.len()];
            for (i, b) in m.iter().enumerate() {
                c.bus.write8(mode + i as u32, *b);
            }
            c.bus.write8(mode + m.len() as u32, 0);
            c.g[A0][0] = (DATA + 0x500) as u64;
            c.g[A1][0] = mode as u64;
            c.g[A2][0] = (DATA + 0x504) as u64;
        });
    }


    // ------------------------------------------------------------ libgcc

    /// `__clz_tab` (bit length of each byte) where the generated code reads
    /// it (the test machine's RAM starts empty).
    pub(crate) fn clz_table(c: &mut Cpu, at: u32) {
        for i in 0..256u32 {
            c.bus.write8(at + i, (32 - i.leading_zeros()) as u8);
        }
    }


    /// Operands for every path of `__udivmoddi4`: d1 == 0 with d0 above or
    /// below n1, d == 0, normalized and unnormalized d1, q estimates that
    /// need one or two corrections, q0*d0 just above n.
    pub(crate) fn div_setup(c: &mut Cpu, seed: u32, table: u32) {
        let mut r = Rng(0x0d1f_d1f0_5eed_0001 ^ (seed as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15));
        scramble(c, &mut r);
        clz_table(c, table);
        let w = |r: &mut Rng| -> u32 {
            match r.next_u32() % 6 {
                0 => 0,
                1 => 0xffff_ffff,
                2 => 0x8000_0000 | r.next_u32() >> 16,
                3 => r.next_u32() >> (r.next_u32() % 32),
                _ => r.next_u32(),
            }
        };
        let n = (w(&mut r) as u64) << 32 | w(&mut r) as u64;
        let d: u64 = match seed % 10 {
            0 => w(&mut r) as u64,
            1 => (r.next_u32() >> (r.next_u32() % 32)) as u64,
            2 => 0,
            3 => (0x8000_0000 | r.next_u32()) as u64 | ((r.next_u32() as u64) << 32 | 0x8000_0000_0000_0000),
            4 => {
                // d0 > n1 with a high half that makes the estimate too big
                let d0 = 0x8000_ffffu32.wrapping_sub(r.next_u32() % 16);
                let n1 = d0.wrapping_sub(1 + r.next_u32() % 4);
                c.g[A0][0] = (n1 as u64) << 32 | r.next_u32() as u64;
                c.g[A1][0] = d0 as u64;
                return;
            }
            5 => {
                // d1 normalized after a shift, n just above a multiple
                let d = ((r.next_u32() >> (r.next_u32() % 24)) as u64) << 32 | r.next_u32() as u64;
                let q = (r.next_u32() % 5) as u64;
                c.g[A0][0] = d.wrapping_mul(q).wrapping_add(r.next_u32() as u64 % 3);
                c.g[A1][0] = d;
                return;
            }
            6 => n >> (r.next_u32() % 64),
            7 => n.wrapping_sub(r.next_u32() as u64 % 4),
            _ => (w(&mut r) as u64) << 32 | w(&mut r) as u64,
        };
        c.g[A0][0] = n;
        c.g[A1][0] = d;
    }


    #[test]
    fn udivdi3_matches() {
        check(ee::s15::f_00411538__generated, udivdi3, 500, &|c, seed| div_setup(c, seed, 0x0049_97a0));
    }


    #[test]
    fn umoddi3_matches() {
        check(ee::s15::f_0042dad8__generated, umoddi3, 500, &|c, seed| div_setup(c, seed, 0x0049_af18));
    }
}
