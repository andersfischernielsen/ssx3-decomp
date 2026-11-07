//! `src/lib/gcc.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::addu;
use crate::abi::r32;
use crate::abi::reg;
use crate::abi::ret;
use crate::abi::set_fcc;
use crate::abi::set_fpr;
use crate::abi::set_reg;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::misc::lit;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

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
    (
        0x0041_0548,
        get_dynamic_handler_chain,
        "__get_dynamic_handler_chain",
    ),
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
    (
        0x0041_4728,
        register_frame_info_table,
        "__register_frame_info_table",
    ),
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
    (
        0x0041_5e68,
        tf19_builtin_type_info,
        "__tf19__builtin_type_info",
    ),
    (
        0x0041_5eb8,
        builtin_type_info_dtor,
        "__builtin_type_info_dtor",
    ),
    (0x0041_5ed8, tf16_attr_type_info, "__tf16__attr_type_info"),
    (0x0041_5f28, attr_type_info_dtor, "__attr_type_info_dtor"),
    (
        0x0041_5f48,
        tf19_pointer_type_info,
        "__tf19__pointer_type_info",
    ),
    (
        0x0041_5f98,
        pointer_type_info_dtor,
        "__pointer_type_info_dtor",
    ),
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
    (0x0041_1c38, unpack_d, "__unpack_d"),
    (0x0041_1b08, pack_d, "__pack_d"),
    (0x0041_1fd8, muldf3, "__muldf3"),
    (0x0040_f5a8, kernel_tanf, "__kernel_tanf"),
    (0x0041_23e8, fpcmp_parts_d, "__fpcmp_parts_d"),
    (0x0041_2550, floatsidf, "__floatsidf"),
    (0x0040_e048, ieee754_expf, "__ieee754_expf"),
    (0x0040_e550, ieee754_logf, "__ieee754_logf"),
    (0x0041_26a0, fixunsdfsi, "__fixunsdfsi"),
    (0x0041_3068, unpack_f, "__unpack_f"),
    (0x0041_1438, floatdidf, "__floatdidf"),
    (0x0041_1f70, subdf3, "__subdf3"),
    (0x0041_39f8, fixunssfsi, "__fixunssfsi"),
    (0x0041_1f18, adddf3, "__adddf3"),
    (0x0041_2280, divdf3, "__divdf3"),
    (0x0041_2608, fixdfsi, "__fixdfsi"),
];

// ---------------------------------------------------------------------------
// Direct-call targets (the readable aliases, which dispatch through the
// generated override slots).

pub(crate) const DEREGISTER_FRAME_INFO: Callee = Callee {
    addr: 0x0041_4780,
    f: ssx3_ee::s15::__deregister_frame_info,
};

pub(crate) const DO_GLOBAL_CTORS: Callee = Callee {
    addr: 0x0040_fbe0,
    f: ssx3_ee::s15::__do_global_ctors,
};

pub(crate) const ABORT: Callee = Callee {
    addr: 0x0041_8790,
    f: ssx3_ee::s15::abort,
};

pub(crate) const TERMINATE: Callee = Callee {
    addr: 0x0041_03b0,
    f: ssx3_ee::s15::__terminate,
};

/// The C++ `terminate()` (0x412800), which calls the installed handler and
/// then re-enters `set_terminate`.
pub(crate) const TERMINATE_CPP: Callee = Callee {
    addr: 0x0041_2800,
    f: ssx3_ee::s15::terminate,
};

pub(crate) const STR_CMP: Callee = Callee {
    addr: 0x0041_65a8,
    f: ssx3_ee::s15::strcmp,
};

pub(crate) const GET_EH_INFO: Callee = Callee {
    addr: 0x0041_0490,
    f: ssx3_ee::s15::__get_eh_info,
};

pub(crate) const MALLOC: Callee = Callee {
    addr: 0x0041_88c0,
    f: ssx3_ee::s15::malloc,
};

pub(crate) const FREE: Callee = Callee {
    addr: 0x0041_8910,
    f: ssx3_ee::s15::free,
};

pub(crate) const REGISTER_FRAME_INFO: Callee = Callee {
    addr: 0x0041_46d0,
    f: ssx3_ee::s15::__register_frame_info,
};

pub(crate) const REGISTER_FRAME_INFO_TABLE: Callee = Callee {
    addr: 0x0041_4728,
    f: ssx3_ee::s15::__register_frame_info_table,
};

pub(crate) const TYPE_INFO_DTOR: Callee = Callee {
    addr: 0x0041_4b68,
    f: ssx3_ee::s15::type_info_dtor,
};

pub(crate) const TYPE_INFO_EQ: Callee = Callee {
    addr: 0x0041_4b98,
    f: ssx3_ee::s15::type_info_eq,
};

pub(crate) const TF9_EXCEPTION: Callee = Callee {
    addr: 0x0041_2f18,
    f: ssx3_ee::s15::__tf9exception,
};

pub(crate) const TF16_USER_TYPE_INFO: Callee = Callee {
    addr: 0x0041_5018,
    f: ssx3_ee::s15::__tf16__user_type_info,
};

pub(crate) const TF9_TYPE_INFO: Callee = Callee {
    addr: 0x0041_51f8,
    f: ssx3_ee::s15::__tf9type_info,
};

pub(crate) const RTTI_SI: Callee = Callee {
    addr: 0x0041_4c00,
    f: ssx3_ee::s15::__rtti_si,
};

pub(crate) const RTTI_USER: Callee = Callee {
    addr: 0x0041_4c20,
    f: ssx3_ee::s15::__rtti_user,
};

pub(crate) const UNPACK_D: Callee = Callee {
    addr: 0x0041_1c38,
    f: ssx3_ee::s15::__unpack_d,
};

pub(crate) const PACK_D: Callee = Callee {
    addr: 0x0041_1b08,
    f: ssx3_ee::s15::__pack_d,
};

pub(crate) const UNPACK_F: Callee = Callee {
    addr: 0x0041_3068,
    f: ssx3_ee::s15::__unpack_f,
};

pub(crate) const PACK_F: Callee = Callee {
    addr: 0x0041_2f58,
    f: ssx3_ee::s15::__pack_f,
};

pub(crate) const MAKE_DP: Callee = Callee {
    addr: 0x0041_2778,
    f: ssx3_ee::s15::__make_dp,
};

pub(crate) const MAKE_FP: Callee = Callee {
    addr: 0x0041_3ac8,
    f: ssx3_ee::s15::__make_fp,
};

pub(crate) const FPCMP_PARTS_D: Callee = Callee {
    addr: 0x0041_23e8,
    f: ssx3_ee::s15::__fpcmp_parts_d,
};

pub(crate) const FPCMP_PARTS_F: Callee = Callee {
    addr: 0x0041_3748,
    f: ssx3_ee::s15::__fpcmp_parts_f,
};

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
    singleton(
        c,
        Singleton {
            base: 0x0054_0000,
            obj: 0x0053_ea10,
            pre: Some((TF9_EXCEPTION, 0x0041_2ea4)),
            rtti: (RTTI_SI, 0x0041_2ebc),
            a1: 0x0049_98d0,
            a2: Some(0x004a_5ba8),
            init_cycles: 8,
        },
    );
}

/// `__tf9exception` (0x412f18).
pub fn tf9exception(c: &mut Cpu) {
    singleton(
        c,
        Singleton {
            base: 0x004a_0000,
            obj: 0x004a_5ba8,
            pre: None,
            rtti: (RTTI_USER, 0x0041_2f44),
            a1: 0x0049_98e0,
            a2: None,
            init_cycles: 4,
        },
    );
}

/// `__tf9type_info` (0x4151f8).
pub fn tf9type_info(c: &mut Cpu) {
    singleton(
        c,
        Singleton {
            base: 0x004a_0000,
            obj: 0x004a_5bb0,
            pre: None,
            rtti: (RTTI_USER, 0x0041_5224),
            a1: 0x0049_9ac0,
            a2: None,
            init_cycles: 4,
        },
    );
}

/// `__tf16__user_type_info` (0x415018).
pub fn tf16_user_type_info(c: &mut Cpu) {
    singleton(
        c,
        Singleton {
            base: 0x0054_0000,
            obj: 0x0053_ea40,
            pre: Some((TF9_TYPE_INFO, 0x0041_503c)),
            rtti: (RTTI_SI, 0x0041_5054),
            a1: 0x0049_9a88,
            a2: Some(0x004a_5bb0),
            init_cycles: 8,
        },
    );
}

/// `__tf17__class_type_info` (0x414f00).
pub fn tf17_class_type_info(c: &mut Cpu) {
    singleton(
        c,
        Singleton {
            base: 0x0054_0000,
            obj: 0x0053_ea60,
            pre: Some((TF16_USER_TYPE_INFO, 0x0041_4f24)),
            rtti: (RTTI_SI, 0x0041_4f3c),
            a1: 0x0049_9a58,
            a2: Some(0x0053_ea40),
            init_cycles: 8,
        },
    );
}

/// `__tf14__si_type_info` (0x414f90).
pub fn tf14_si_type_info(c: &mut Cpu) {
    singleton(
        c,
        Singleton {
            base: 0x0054_0000,
            obj: 0x0053_ea50,
            pre: Some((TF16_USER_TYPE_INFO, 0x0041_4fb4)),
            rtti: (RTTI_SI, 0x0041_4fcc),
            a1: 0x0049_9a70,
            a2: Some(0x0053_ea40),
            init_cycles: 8,
        },
    );
}

/// `__tf10bad_typeid` (0x4150d0).
pub fn tf10bad_typeid(c: &mut Cpu) {
    singleton(
        c,
        Singleton {
            base: 0x0054_0000,
            obj: 0x0053_ea20,
            pre: Some((TF9_EXCEPTION, 0x0041_50f4)),
            rtti: (RTTI_SI, 0x0041_510c),
            a1: 0x0049_9aa0,
            a2: Some(0x004a_5ba8),
            init_cycles: 8,
        },
    );
}

/// `__tf8bad_cast` (0x415168).
pub fn tf8bad_cast(c: &mut Cpu) {
    singleton(
        c,
        Singleton {
            base: 0x0054_0000,
            obj: 0x0053_ea30,
            pre: Some((TF9_EXCEPTION, 0x0041_518c)),
            rtti: (RTTI_SI, 0x0041_51a4),
            a1: 0x0049_9ab0,
            a2: Some(0x004a_5ba8),
            init_cycles: 8,
        },
    );
}

/// `__tf17__array_type_info` (0x415ca8).
pub fn tf17_array_type_info(c: &mut Cpu) {
    singleton(
        c,
        Singleton {
            base: 0x0054_0000,
            obj: 0x0053_eaa0,
            pre: Some((TF9_TYPE_INFO, 0x0041_5ccc)),
            rtti: (RTTI_SI, 0x0041_5ce4),
            a1: 0x0049_9bd8,
            a2: Some(0x004a_5bb0),
            init_cycles: 8,
        },
    );
}

/// `__tf16__ptmd_type_info` (0x415d18).
pub fn tf16_ptmd_type_info(c: &mut Cpu) {
    singleton(
        c,
        Singleton {
            base: 0x0054_0000,
            obj: 0x0053_eac0,
            pre: Some((TF9_TYPE_INFO, 0x0041_5d3c)),
            rtti: (RTTI_SI, 0x0041_5d54),
            a1: 0x0049_9bf0,
            a2: Some(0x004a_5bb0),
            init_cycles: 8,
        },
    );
}

/// `__tf16__ptmf_type_info` (0x415d88).
pub fn tf16_ptmf_type_info(c: &mut Cpu) {
    singleton(
        c,
        Singleton {
            base: 0x0054_0000,
            obj: 0x0053_ea90,
            pre: Some((TF9_TYPE_INFO, 0x0041_5dac)),
            rtti: (RTTI_SI, 0x0041_5dc4),
            a1: 0x0049_9c08,
            a2: Some(0x004a_5bb0),
            init_cycles: 8,
        },
    );
}

/// `__tf16__func_type_info` (0x415df8).
pub fn tf16_func_type_info(c: &mut Cpu) {
    singleton(
        c,
        Singleton {
            base: 0x0054_0000,
            obj: 0x0053_ea70,
            pre: Some((TF9_TYPE_INFO, 0x0041_5e1c)),
            rtti: (RTTI_SI, 0x0041_5e34),
            a1: 0x0049_9c20,
            a2: Some(0x004a_5bb0),
            init_cycles: 8,
        },
    );
}

/// `__tf19__builtin_type_info` (0x415e68).
pub fn tf19_builtin_type_info(c: &mut Cpu) {
    singleton(
        c,
        Singleton {
            base: 0x0054_0000,
            obj: 0x0053_ead0,
            pre: Some((TF9_TYPE_INFO, 0x0041_5e8c)),
            rtti: (RTTI_SI, 0x0041_5ea4),
            a1: 0x0049_9c38,
            a2: Some(0x004a_5bb0),
            init_cycles: 8,
        },
    );
}

/// `__tf16__attr_type_info` (0x415ed8).
pub fn tf16_attr_type_info(c: &mut Cpu) {
    singleton(
        c,
        Singleton {
            base: 0x0054_0000,
            obj: 0x0053_ea80,
            pre: Some((TF9_TYPE_INFO, 0x0041_5efc)),
            rtti: (RTTI_SI, 0x0041_5f14),
            a1: 0x0049_9c50,
            a2: Some(0x004a_5bb0),
            init_cycles: 8,
        },
    );
}

/// `__tf19__pointer_type_info` (0x415f48).
pub fn tf19_pointer_type_info(c: &mut Cpu) {
    singleton(
        c,
        Singleton {
            base: 0x0054_0000,
            obj: 0x0053_eab0,
            pre: Some((TF9_TYPE_INFO, 0x0041_5f6c)),
            rtti: (RTTI_SI, 0x0041_5f84),
            a1: 0x0049_9c68,
            a2: Some(0x004a_5bb0),
            init_cycles: 8,
        },
    );
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

pub(crate) const READ_R: Callee = Callee {
    addr: 0x0041_fd10,
    f: ssx3_ee::s15::f_0041fd10,
};

pub(crate) const WRITE_R: Callee = Callee {
    addr: 0x0041_fdd0,
    f: ssx3_ee::s15::f_0041fdd0,
};

pub(crate) const LSEEK_R: Callee = Callee {
    addr: 0x0041_fc50,
    f: ssx3_ee::s15::f_0041fc50,
};

pub(crate) const CLOSE_R: Callee = Callee {
    addr: 0x0041_fb98,
    f: ssx3_ee::s15::f_0041fb98,
};

pub(crate) const GET_THREAD_ID: Callee = Callee {
    addr: 0x0042_3c90,
    f: ssx3_ee::s15::f_00423c90,
};

pub(crate) const WAIT_SEMA: Callee = Callee {
    addr: 0x0042_3de0,
    f: ssx3_ee::s15::f_00423de0,
};

pub(crate) const SIGNAL_SEMA: Callee = Callee {
    addr: 0x0042_3dc0,
    f: ssx3_ee::s15::f_00423dc0,
};

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

pub(crate) const CMPDF2: Callee = Callee {
    addr: 0x0041_2500,
    f: ssx3_ee::s15::f_00412500,
};

pub(crate) const SUBDF3: Callee = Callee {
    addr: 0x0041_1f70,
    f: ssx3_ee::s15::f_00411f70,
};

pub(crate) const ADDDF3: Callee = Callee {
    addr: 0x0041_1f18,
    f: ssx3_ee::s15::f_00411f18,
};

pub(crate) const MULDF3: Callee = Callee {
    addr: 0x0041_1fd8,
    f: ssx3_ee::s15::f_00411fd8,
};

pub(crate) const FLOATDIDF: Callee = Callee {
    addr: 0x0041_1438,
    f: ssx3_ee::s15::f_00411438,
};

pub(crate) const FIXUNSDFSI: Callee = Callee {
    addr: 0x0041_26a0,
    f: ssx3_ee::s15::f_004126a0,
};

pub(crate) const FIXUNSDFDI: Callee = Callee {
    addr: 0x0042_d380,
    f: ssx3_ee::s15::f_0042d380,
};

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
pub(crate) fn qrnnd_step(
    c: &mut Cpu,
    top: u32,
    part: u32,
    d: u32,
    block: u64,
    retry: u64,
) -> (u32, u32) {
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

pub(crate) mod fp_number {

    /// Class: 0 signalling NaN, 1 quiet NaN, 2 zero, 3 normal, 4 infinity.
    pub const CLASS: u32 = 0x00;

    /// Sign bit (0 or 1).
    pub const SIGN: u32 = 0x04;

    /// Unbiased exponent.
    pub const NORMAL_EXP: u32 = 0x08;

    /// Fraction (u64); for normals the mantissa shifted left by 8 with the
    /// implicit one at bit 60.
    pub const FRACTION: u32 = 0x10;

    pub const CLASS_SNAN: u64 = 0;

    pub const CLASS_QNAN: u64 = 1;

    pub const CLASS_ZERO: u64 = 2;

    pub const CLASS_NUMBER: u64 = 3;

    pub const CLASS_INFINITY: u64 = 4;

    /// `+0x0` class: 0/1 NaN, 2 zero, 3 normal, 4 infinity.
    pub const CLASS_2: u32 = 0x0;

    pub const SIGN_2: u32 = 0x4;

    pub const EXP: u32 = 0x8;

    /// Fraction bits below the binary point.
    pub const FRACBITS: u64 = 0x3c;
}

/// `void __unpack_d(const FLO_union_type* src, fp_number_type* dst)`
/// (0x411c38, libgcc soft-float): splits the double at `a0` into sign,
/// exponent and fraction. This build has no denormals: a zero exponent field
/// is always class "zero" (fraction and exponent left unwritten). Exponent
/// 0x7ff is infinity (zero fraction) or a NaN (quiet when fraction bit 51 is
/// set); NaN and infinity leave the exponent field unwritten, infinity also
/// the fraction. Normals get the unbiased exponent and the mantissa
/// `<< 8` with the implicit one at bit 60.
pub fn unpack_d(c: &mut Cpu) {
    use crate::guest::Addr;
    use fp_number as fp;

    c.cycles += 10;
    let bits = Addr(c.g[A0][0] as u32).ld(c);
    let dst = Addr(c.g[A1][0] as u32);
    let fraction = bits & (u64::MAX >> 12);
    let exp = (bits >> 52) & 0x7ff;
    dst.off(fp::SIGN).sw(c, bits >> 63);
    c.g[A0][0] = exp;
    c.g[V1][0] = fraction;
    if exp == 0 {
        c.cycles += 3;
        c.g[V0][0] = fp::CLASS_ZERO;
        dst.off(fp::CLASS).sw(c, fp::CLASS_ZERO);
    } else if exp != 0x7ff {
        c.cycles += 11;
        let mantissa = (fraction << 8) | (1u64 << 60);
        let unbiased = sx((exp as u32).wrapping_sub(0x3ff));
        c.g[V1][0] = mantissa;
        c.g[A0][0] = unbiased;
        c.g[V0][0] = fp::CLASS_NUMBER;
        dst.off(fp::FRACTION).sd(c, mantissa);
        dst.off(fp::NORMAL_EXP).sw(c, unbiased);
        dst.off(fp::CLASS).sw(c, fp::CLASS_NUMBER);
    } else if fraction == 0 {
        c.cycles += 6;
        c.g[V0][0] = fp::CLASS_INFINITY;
        dst.off(fp::CLASS).sw(c, fp::CLASS_INFINITY);
    } else {
        c.cycles += 9;
        // v0 holds 1 on both paths (the quiet class, also the store value).
        c.g[V0][0] = 1;
        if fraction & (1u64 << 51) != 0 {
            c.cycles += 2;
            dst.off(fp::CLASS).sw(c, fp::CLASS_QNAN);
        } else {
            c.cycles += 1;
            dst.off(fp::CLASS).sw(c, fp::CLASS_SNAN);
        }
        c.cycles += 2;
        dst.off(fp::FRACTION).sd(c, fraction);
    }
    ret(c);
}

/// `double __pack_d(const fp_number_type* src)` (0x411b08, libgcc
/// soft-float, the inverse of `__unpack_d`): builds the IEEE double bits (in
/// `v0`) from the unpacked class, sign, unbiased exponent and fraction (the
/// mantissa `<< 8`, implicit one at bit 60). NaNs get exponent 0x7ff and the
/// quiet bit; infinity and an out-of-range exponent (>= 1024) give 0x7ff
/// with no fraction; zero (or a zero fraction) gives exponent 0. Below
/// -1022 the fraction is shifted down to a denormal (nothing left past 56
/// places) without rounding; normals round to nearest even on the 8 guard
/// bits and bump the exponent when the rounding carries past bit 60.
///
/// Leaves `a1` (final fraction before masking), `a3` (biased exponent),
/// `t0` (sign word), `a0` (`u64::MAX >> 1`), `a2` (result without sign) and
/// `v1` (exponent field) like the original.
pub fn pack_d(c: &mut Cpu) {
    use crate::game::lib::gcc::fp_number as fp;
    use crate::guest::Addr;

    const MASK52: u64 = u64::MAX >> 12;
    const QUIET_NAN: u64 = 1 << 51;
    const GUARD_MASK: u64 = 0xff;
    const GUARD_HALF: u64 = 0x80;
    const OVERFLOW_LIMIT: u64 = u64::MAX >> 3; // implicit one carried to bit 61
    const EXP_MAX: u64 = 0x7ff;
    const EXP_MIN: i64 = -1022;

    let src = Addr(c.g[A0][0] as u32);
    c.cycles += 6;
    let class = src.off(fp::CLASS).lw(c);
    let sign = src.off(fp::SIGN).lw(c);
    let mut fraction = src.off(fp::FRACTION).ld(c);
    let mut exp_field: u64 = 0;

    if class < fp::CLASS_ZERO {
        // signalling or quiet NaN
        c.cycles += 5;
        exp_field = EXP_MAX;
        fraction |= QUIET_NAN;
    } else if class == fp::CLASS_INFINITY {
        c.cycles += 3 + 2;
        exp_field = EXP_MAX;
        fraction = 0;
    } else if class == fp::CLASS_ZERO {
        c.cycles += 3 + 3 + 2;
        fraction = 0;
    } else if fraction == 0 {
        c.cycles += 3 + 3 + 2;
    } else {
        c.cycles += 3 + 3 + 2 + 4;
        let exp = src.off(fp::NORMAL_EXP).lw(c);
        if (exp as i64) < EXP_MIN {
            // denormal: shift down, no rounding
            c.cycles += 4;
            let shift = sx((EXP_MIN as u32).wrapping_sub(exp as u32));
            fraction >>= shift & 63;
            if (shift as i64) >= 57 {
                c.cycles += 2;
                fraction = 0;
            }
            c.cycles += 1;
            fraction >>= 8;
        } else {
            c.cycles += 3;
            exp_field = sx((exp as u32).wrapping_add(0x3ff));
            if (exp as i64) >= 1024 {
                // too large: infinity
                c.cycles += 1 + 2;
                exp_field = EXP_MAX;
                fraction = 0;
            } else {
                // round to nearest, ties to even
                c.cycles += 4;
                if fraction & GUARD_MASK != GUARD_HALF {
                    fraction = fraction.wrapping_add(0x7f);
                } else {
                    c.cycles += 3;
                    if fraction & 0x100 != 0 {
                        fraction = fraction.wrapping_add(0x80);
                    }
                }
                c.cycles += 5;
                if fraction > OVERFLOW_LIMIT {
                    c.cycles += 2 + 1;
                    fraction >>= 1;
                    exp_field = sx((exp_field as u32).wrapping_add(1));
                }
                fraction >>= 8;
            }
        }
    }

    c.cycles += 23;
    let exp_bits = (exp_field & EXP_MAX) << 52;
    let magnitude = (fraction & MASK52) | exp_bits;
    c.g[A1][0] = fraction;
    c.g[A3][0] = exp_field;
    c.g[T0][0] = sign;
    c.g[A0][0] = u64::MAX >> 1;
    c.g[A2][0] = magnitude;
    c.g[V1][0] = exp_bits;
    c.g[V0][0] = magnitude | (sign << 63);
    ret(c);
}

// ---------------------------------------------------------------- __muldf3

pub(crate) const UNPACK_D_2: Callee = Callee {
    addr: 0x0041_1c38,
    f: ee::s15::f_00411c38,
};

pub(crate) const MULDI3: Callee = Callee {
    addr: 0x0041_14d0,
    f: ee::s15::f_004114d0,
};

pub(crate) const PACK_D_2: Callee = Callee {
    addr: 0x0041_1b08,
    f: ee::s15::f_00411b08,
};

/// `double __muldf3(double a, double b)` (0x411fd8, libgcc soft-float
/// multiply; doubles travel as raw bits in `a0`/`a1`, the result in `v0`).
///
/// Unpacks both operands into `fp_number_type`s on the stack (a at sp+0,
/// b at sp+0x20) and handles the special classes like fp-bit.c's `_fpmul`:
/// a NaN operand (a checked first) is returned with the product's sign,
/// infinity times zero gives the canonical NaN `__thenan_df` (0x52adc0),
/// infinity or zero times a number returns that operand with the product's
/// sign. Otherwise the 61-bit
/// fractions are multiplied into a 128-bit product with four `__muldi3`
/// calls (32x32 halves), the product is normalised (down while the high
/// word exceeds `u64::MAX >> 3`, carrying the shifted-out bit into the low
/// word only when set; up while it is at most `u64::MAX >> 4`), rounded to
/// even on the 8 guard bits and packed from the temporary at sp+0x40.
pub fn muldf3(c: &mut Cpu) {
    use crate::game::lib::gcc::fp_number as fp;
    use crate::guest::Addr;

    /// The unpacked operands and the result in the 0x100 frame.
    const A: u32 = 0x00;
    const B: u32 = 0x20;
    const TMP: u32 = 0x40;
    /// The arguments are spilled here for `__unpack_d`.
    const ARG_A: u32 = 0x60;
    const ARG_B: u32 = 0x68;
    /// `__thenan_df`: the canonical NaN operand.
    const THE_NAN: u64 = 0x0052_adc0;
    const HIGH_MAX: u64 = u64::MAX >> 3;
    const NORMAL_MIN: u64 = u64::MAX >> 4;
    const TOP_BIT: u64 = 1 << 63;

    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x100);
        c.sd(frame.at(c, ARG_A), c.g[A0][0]);
        c.sd(frame.at(c, ARG_B), c.g[A1][0]);
        c.g[A0][0] = sx(frame.at(c, ARG_A));
        c.sd(frame.at(c, 0xe0), c.g[S7][0]);
        c.g[A1][0] = c.g[SP][0];
        c.sd(frame.at(c, 0x70), c.g[S0][0]);
        frame.save_ra(c, 0xf0);
        c.sd(frame.at(c, 0xd0), c.g[S6][0]);
        c.sd(frame.at(c, 0xc0), c.g[S5][0]);
        c.sd(frame.at(c, 0xb0), c.g[S4][0]);
        c.sd(frame.at(c, 0xa0), c.g[S3][0]);
        c.sd(frame.at(c, 0x90), c.g[S2][0]);
        c.sd(frame.at(c, 0x80), c.g[S1][0]);
        UNPACK_D_2.call(c, 0x0041_2014)?;
        c.g[S0][0] = sx(frame.at(c, B));
        c.g[A0][0] = sx(frame.at(c, ARG_B));
        c.g[A1][0] = c.g[S0][0];
        UNPACK_D_2.call(c, 0x0041_2024)?;

        let a = Addr(frame.at(c, A));
        let b = Addr(frame.at(c, B));
        let class_a = a.off(fp::CLASS).lw(c);
        c.g[A0][0] = class_a;
        c.g[S7][0] = sx(frame.at(c, TMP));

        // What gets packed when an operand is special.
        enum Special {
            /// a with sign a ^ b (sp+0x04 rewritten); entered at 0x412088
            /// (loads sign a first).
            A,
            /// The same, entered at 0x412090 (a infinite: sign a in `v0`).
            AInfinite,
            /// b with sign a ^ b (sp+0x24 rewritten).
            B,
            /// Infinity times zero.
            TheNan,
        }
        let special = if class_a < 2 {
            Some(Special::A)
        } else {
            c.cycles += 4;
            let class_b = b.off(fp::CLASS).lw(c);
            c.g[V1][0] = class_b;
            if class_b < 2 {
                Some(Special::B)
            } else {
                c.cycles += 3;
                if class_a == fp::CLASS_INFINITY {
                    c.cycles += 3;
                    c.g[V0][0] = a.off(fp::SIGN).lw(c);
                    if class_b == fp::CLASS_ZERO {
                        Some(Special::TheNan)
                    } else {
                        c.cycles += 2;
                        c.g[A0][0] = c.g[SP][0];
                        Some(Special::AInfinite)
                    }
                } else {
                    c.cycles += 2;
                    if class_b == fp::CLASS_INFINITY {
                        c.cycles += 2;
                        c.g[V1][0] = b.off(fp::SIGN).lw(c);
                        if class_a != fp::CLASS_ZERO {
                            Some(Special::B)
                        } else {
                            Some(Special::TheNan)
                        }
                    } else {
                        c.cycles += 2;
                        if class_a == fp::CLASS_ZERO {
                            Some(Special::A)
                        } else {
                            c.cycles += 2;
                            c.g[S3][0] = a.off(fp::FRACTION).ld(c);
                            if class_b == fp::CLASS_ZERO {
                                c.cycles += 1;
                                Some(Special::B)
                            } else {
                                c.g[V0][0] = class_b ^ fp::CLASS_ZERO;
                                None
                            }
                        }
                    }
                }
            }
        };

        match special {
            Some(sp_a @ (Special::A | Special::AInfinite)) => {
                if matches!(sp_a, Special::A) {
                    c.cycles += 2;
                    c.g[V0][0] = a.off(fp::SIGN).lw(c);
                    c.g[A0][0] = c.g[SP][0];
                }
                c.cycles += 5;
                let sign_b = b.off(fp::SIGN).lw(c);
                c.g[V1][0] = sign_b;
                let sign = ((c.g[V0][0] ^ sign_b) != 0) as u64;
                c.g[V0][0] = sign;
                a.off(fp::SIGN).sw(c, sign);
            }
            Some(Special::B) => {
                c.cycles += 6;
                let sign_b = b.off(fp::SIGN).lw(c);
                c.g[V1][0] = sign_b;
                c.g[A0][0] = c.g[S0][0];
                let sign = ((a.off(fp::SIGN).lw(c) ^ sign_b) != 0) as u64;
                c.g[V0][0] = sign;
                b.off(fp::SIGN).sw(c, sign);
            }
            Some(Special::TheNan) => {
                c.cycles += 3;
                c.g[V0][0] = 0x0053_0000;
                c.g[A0][0] = THE_NAN;
            }
            None => multiply(c, a, b)?,
        }

        c.cycles += 13;
        PACK_D_2.call(c, 0x0041_2254)?;
        frame.restore_ra(c, 0xf0);
        c.g[S7][0] = c.ld(frame.at(c, 0xe0));
        c.g[S6][0] = c.ld(frame.at(c, 0xd0));
        c.g[S5][0] = c.ld(frame.at(c, 0xc0));
        c.g[S4][0] = c.ld(frame.at(c, 0xb0));
        c.g[S3][0] = c.ld(frame.at(c, 0xa0));
        c.g[S2][0] = c.ld(frame.at(c, 0x90));
        c.g[S1][0] = c.ld(frame.at(c, 0x80));
        c.g[S0][0] = c.ld(frame.at(c, 0x70));
        frame.close(c);
        Ok(())
    });

    /// Both operands are numbers (class 3): multiply the fractions
    /// (fraction a already in `s3`) into a 128-bit product, normalise,
    /// round, and leave the result in the temporary at sp+0x40 (`a0`, also
    /// `s7`) with class 3.
    fn multiply(c: &mut Cpu, a: Addr, b: Addr) -> Flow {
        let tmp = Addr(c.g[S7][0] as u32);
        c.cycles += 47;
        // 32x32 partial products: s3/s0 = fraction a high/low, s2/s5 = b.
        c.g[S6][0] = 0xffff_ffff;
        c.g[S2][0] = b.off(fp::FRACTION).ld(c);
        c.g[S0][0] = c.g[S3][0] & c.g[S6][0];
        c.g[S5][0] = c.g[S2][0] & c.g[S6][0];
        c.g[S3][0] >>= 32;
        c.g[S2][0] >>= 32;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S5][0];
        MULDI3.call(c, 0x0041_20f0)?;
        c.g[S4][0] = c.g[V0][0]; // lo b * lo a
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S2][0];
        MULDI3.call(c, 0x0041_2100)?;
        c.g[S1][0] = c.g[V0][0]; // hi b * lo a
        c.g[A0][0] = c.g[S5][0];
        c.g[A1][0] = c.g[S3][0];
        MULDI3.call(c, 0x0041_2110)?;
        c.g[S0][0] = c.g[V0][0]; // lo b * hi a
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S3][0];
        MULDI3.call(c, 0x0041_2120)?;
        let hi_hi = c.g[V0][0];

        let lo_lo = c.g[S4][0];
        let cross = c.g[S1][0].wrapping_add(c.g[S0][0]);
        let cross_carry = (cross < c.g[S1][0]) as u64;
        let mut low = lo_lo.wrapping_add(cross << 32);
        let low_carry = (low < lo_lo) as u64;
        let exp_a = a.off(fp::NORMAL_EXP).lw(c);
        let exp_b = b.off(fp::NORMAL_EXP).lw(c);
        let sign_a = a.off(fp::SIGN).lw(c);
        let sign_b = b.off(fp::SIGN).lw(c);
        let cross_high = ((cross >> 32) & c.g[S6][0]).wrapping_add(hi_hi);
        let mut high = ((cross_carry << 32) | low_carry).wrapping_add(cross_high);
        let sign = ((sign_a ^ sign_b) != 0) as u64;
        let mut exp = sx((exp_a as u32).wrapping_add(exp_b as u32).wrapping_add(4));
        tmp.off(fp::SIGN).sw(c, sign);
        tmp.off(fp::NORMAL_EXP).sw(c, exp);
        c.g[A2][0] = sign_b;
        c.g[A3][0] = exp_b;
        c.g[S0][0] = cross_high;
        c.g[S4][0] = low_carry;
        c.g[V1][0] = sign;
        c.g[V0][0] = (high > HIGH_MAX) as u64;

        // Unreachable with unpacked operands (both fractions have their
        // implicit one at bit 60, so the product's high word is below
        // 2^58), kept for exactness.
        if high > HIGH_MAX {
            // Too big: shift down; the bit falling out of `high` goes into
            // the top of `low` only when it is set (as in fp-bit.c).
            c.cycles += 5;
            c.g[A2][0] = TOP_BIT;
            c.g[V1][0] = HIGH_MAX;
            loop {
                c.cycles += 4;
                exp = sx((exp as u32).wrapping_add(1));
                if high & 1 != 0 {
                    c.cycles += 2;
                    low = (low >> 1) | TOP_BIT;
                }
                c.cycles += 4;
                high >>= 1;
                c.g[V0][0] = high & 1;
                if high <= HIGH_MAX {
                    break;
                }
                if event_due(c) {
                    c.g[S1][0] = high;
                    c.g[A0][0] = low;
                    c.g[A1][0] = exp;
                    deliver_event_here(c, 0x0041_2198);
                }
            }
            c.cycles += 1;
            tmp.off(fp::NORMAL_EXP).sw(c, exp);
        }

        c.cycles += 5;
        c.g[V1][0] = high & 0xff;
        if high <= NORMAL_MIN {
            // Too small: shift up, pulling in the top bit of `low`.
            c.cycles += 6;
            exp = tmp.off(fp::NORMAL_EXP).lw(c);
            c.g[T0][0] = TOP_BIT;
            c.g[A3][0] = 1;
            c.g[A2][0] = NORMAL_MIN;
            loop {
                c.cycles += 8;
                let low_top = low & TOP_BIT;
                high <<= 1;
                exp = sx((exp as u32).wrapping_sub(1));
                if low_top != 0 {
                    high |= 1;
                }
                low <<= 1;
                if high > NORMAL_MIN {
                    break;
                }
                if event_due(c) {
                    c.g[S1][0] = high;
                    c.g[A0][0] = low;
                    c.g[A1][0] = exp;
                    c.g[V1][0] = low_top;
                    c.g[V0][0] = 0;
                    deliver_event_here(c, 0x0041_21f0);
                }
            }
            c.cycles += 2;
            tmp.off(fp::NORMAL_EXP).sw(c, exp);
            c.g[V1][0] = high & 0xff;
        }
        c.g[A1][0] = exp;

        // Round to nearest even on the 8 guard bits: an exact tie rounds up
        // when bit 8 is set, or (when it is clear) when anything is left
        // in `low`.
        c.cycles += 3;
        c.g[V0][0] = 0x80;
        if high & 0xff == 0x80 {
            c.cycles += 3;
            c.g[V0][0] = high.wrapping_add(0x80);
            if high & 0x100 != 0 {
                c.cycles += 2;
                high = high.wrapping_add(0x80);
            } else {
                c.cycles += 1;
                if low != 0 {
                    high = c.g[V0][0];
                }
            }
            c.cycles += 1;
        }
        tmp.off(fp::FRACTION).sd(c, high);
        c.g[S1][0] = high;
        c.g[A0][0] = low;

        c.cycles += 3;
        c.g[V0][0] = fp::CLASS_NUMBER;
        tmp.off(fp::CLASS).sw(c, fp::CLASS_NUMBER);
        c.g[A0][0] = c.g[S7][0];
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// __kernel_tanf

/// `float fabsf(float)` (0x40f8a0).
pub(crate) const FABSF: Callee = Callee {
    addr: 0x0040_f8a0,
    f: ee::s15::fabsf,
};

/// The polynomial coefficients `T[0..12]` of newlib's `__kernel_tanf`.
pub(crate) const TANF_T: u32 = 0x0049_9648;

/// The newlib `__kernel_tanf` tail for `|x| >= 0.6744`: undo the `pi/4`
/// reduction, `v = iy; w = x + r; (1 - ((hx >> 30) & 2)) * (v - 2(x -
/// (w*w/(w+v) - r)))` (f3 = r, f11 = w, f12 = x).
pub(crate) fn tanf_unreduce(c: &mut Cpu) {
    c.cycles += 19;
    c.mtc1(A0, 4);
    c.cvt_s_w(4, 4);
    c.g[V1][0] = sx(((c.g[A2][0] as u32) as i32 >> 30) as u32);
    c.mul_s(0, 11, 11);
    c.g[V1][0] &= 2;
    c.g[V0][0] = 1;
    c.add_s(1, 11, 4);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
    c.mtc1(V0, 2);
    c.cvt_s_w(2, 2);
    c.div_s(0, 0, 1);
    c.sub_s(0, 0, 3);
    c.sub_s(0, 12, 0);
    c.add_s(0, 0, 0);
    c.sub_s(0, 4, 0);
    c.mul_s(0, 2, 0);
}

/// `-1/w` computed carefully for `iy == -1` and small `|x|` (f3 = r,
/// f11 = w, f12 = x): `z = w` and `t = a = -1/w` truncated to 20 bits,
/// `s = 1 + t*z`, `t + a*(s + t*(r - (z - x)))`.
pub(crate) fn tanf_neg_recip(c: &mut Cpu) {
    c.cycles += 29;
    c.mfc1(V0, 11);
    c.g[V1][0] = c.g[V0][0];
    c.g[V0][0] = 0xffff_ffff_ffff_f000;
    c.g[V1][0] &= c.g[V0][0];
    c.mtc1(V1, 14);
    c.sub_s(1, 14, 12);
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(AT, 0);
    c.div_s(0, 0, 11);
    c.sub_s(4, 3, 1);
    c.mfc1(A0, 0);
    c.g[V1][0] = c.g[A0][0];
    c.g[V1][0] &= c.g[V0][0];
    c.mtc1(V1, 0);
    c.mul_s(1, 0, 14);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 0);
    c.add_s(15, 1, 0);
    c.mtc1(V1, 1);
    c.mul_s(0, 1, 4);
    c.mtc1(A0, 1);
    c.add_s(0, 15, 0);
    c.mul_s(0, 1, 0);
    c.mtc1(V1, 1);
    c.add_s(0, 1, 0);
}

/// `float __kernel_tanf(float x, float y, int iy)` (0x40f5a8, newlib):
/// tan(x + y) on `[-pi/4, pi/4]` (`iy == 1`), or `-1/tan` (`iy == -1`).
/// Tiny `|x|` returns x (or `1/|x|` for zero x with `iy == -1`, else
/// `-1/x`); above 0.6744 the argument is reduced to `pi/4 - x` first; the
/// odd polynomial uses the 13 coefficients at 0x499648.
pub fn kernel_tanf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.mfc1(V0, 12);
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[V0][0];
        c.g[V1][0] = 0x7fff_ffff;
        c.g[V0][0] = 0x317f_ffff;
        c.g[A1][0] = c.g[A2][0] & c.g[V1][0];
        let tiny_limit = c.g[V0][0];
        c.g[V0][0] = ((tiny_limit as i64) < (c.g[A1][0] as i64)) as u64;
        frame.save_ra(c, 0);
        let mut main = c.g[V0][0] != 0;
        if main {
            c.cycles += 1;
        } else {
            // |x| < 2**-28: x itself unless it does not truncate to 0.
            c.cycles += 5;
            c.cvt_w_s(0, 12);
            c.mfc1(V0, 0);
            main = c.g[V0][0] != 0;
        }
        c.g[V0][0] = 0x3f2c_0000;
        if !main {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
            c.g[V0][0] |= c.g[A1][0];
            let zero_neg = c.g[V0][0] == 0;
            c.g[V0][0] = 1;
            if zero_neg {
                // x == 0 and iy == -1: 1/|x|.
                c.cycles += 9;
                FABSF.call(c, 0x0040_f600)?;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(AT, 1);
                c.div_s(0, 1, 0);
                frame.restore_ra(c, 0);
            } else {
                c.cycles += 2;
                let tan = c.g[A0][0] == c.g[V0][0];
                frame.restore_ra(c, 0);
                if tan {
                    c.cycles += 2;
                    c.mov_s(0, 12);
                    c.cycles += 1;
                    frame.restore_ra(c, 0);
                } else {
                    c.cycles += 7;
                    c.g[AT][0] = 0xffff_ffff_bf80_0000;
                    c.mtc1(AT, 0);
                    c.div_s(0, 0, 12);
                }
            }
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }

        // z = x*x, after reducing x (and y) to pi/4 - x for |x| >= 0.6744
        // (0x3f2ca13f).
        c.cycles += 4;
        c.g[V0][0] |= 0xa13f;
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            c.mul_s(14, 12, 12);
        } else {
            c.cycles += 2;
            if (c.g[A2][0] as i64) < 0 {
                c.cycles += 2;
                c.neg_s(12, 12);
                c.neg_s(13, 13);
            }
            c.cycles += 11;
            c.g[AT][0] = 0x3f49_0fda; // pio4
            c.mtc1(AT, 0);
            c.g[AT][0] = 0x3322_2168; // pio4lo
            c.mtc1(AT, 1);
            c.sub_s(14, 0, 12);
            c.sub_s(11, 1, 13);
            c.mtc1(0, 13);
            c.add_s(12, 14, 11);
            c.mul_s(14, 12, 12);
        }

        // r = y + z*(s*(r + v) + y) + T[0]*s*x; w = x + r, with r the odd
        // and v the even half of the polynomial in w = z*z.
        c.cycles += 50;
        c.g[V1][0] = 0x4a_0000;
        c.g[V0][0] = sx(TANF_T);
        c.lwc1(9, TANF_T);
        c.lwc1(4, TANF_T + 0x30);
        c.g[V1][0] = 0x3f2c_0000;
        c.mul_s(11, 14, 14);
        c.lwc1(5, TANF_T + 0x2c);
        c.lwc1(6, TANF_T + 0x28);
        c.mul_s(15, 14, 12);
        c.lwc1(2, TANF_T + 0x24);
        c.g[V1][0] |= 0xa13f;
        c.mul_s(4, 11, 4);
        c.lwc1(3, TANF_T + 0x20);
        c.mul_s(5, 11, 5);
        c.lwc1(0, TANF_T + 0x1c);
        c.lwc1(7, TANF_T + 0x18);
        c.mul_s(9, 9, 15);
        c.add_s(6, 6, 4);
        c.lwc1(1, TANF_T + 0x14);
        c.add_s(2, 2, 5);
        c.lwc1(4, TANF_T + 0x10);
        c.lwc1(5, TANF_T + 0xc);
        c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
        c.mul_s(6, 11, 6);
        c.lwc1(8, TANF_T + 0x8);
        c.mul_s(2, 11, 2);
        c.lwc1(10, TANF_T + 0x4);
        c.add_s(3, 3, 6);
        c.add_s(0, 0, 2);
        c.mul_s(3, 11, 3);
        c.mul_s(0, 11, 0);
        c.add_s(7, 7, 3);
        c.add_s(1, 1, 0);
        c.mul_s(7, 11, 7);
        c.mul_s(1, 11, 1);
        c.add_s(4, 4, 7);
        c.add_s(5, 5, 1);
        c.mul_s(4, 11, 4);
        c.mul_s(5, 11, 5);
        c.add_s(8, 8, 4);
        c.add_s(3, 10, 5);
        c.mul_s(4, 14, 8);
        c.add_s(0, 3, 4);
        c.mul_s(0, 15, 0);
        c.add_s(0, 0, 13);
        c.mul_s(0, 14, 0);
        c.add_s(3, 13, 0);
        c.add_s(3, 3, 9);
        let reduced = c.g[V1][0] != 0;
        c.add_s(11, 12, 3);
        if reduced {
            tanf_unreduce(c);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 1;
            let tan = c.g[A0][0] == c.g[V0][0];
            c.mov_s(0, 11);
            if !tan {
                tanf_neg_recip(c);
            }
        }
        c.cycles += 1;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// __fpcmp_parts_d (0x004123e8)

/// libgcc's unpacked double (`fp_number_type`): class, sign, exponent,
/// 64-bit fraction.
pub(crate) const FP_CLASS: u32 = 0x0;

pub(crate) const FP_SIGN: u32 = 0x4;

pub(crate) const FP_EXP: u32 = 0x8;

pub(crate) const FP_FRAC: u32 = 0x10;

/// Classes: 0/1 = NaNs (below 2), 2 = zero, 3 = normal, 4 = infinity.
pub(crate) const CLASS_ZERO: u64 = 2;

pub(crate) const CLASS_INFINITY: u64 = 4;

/// `int __fpcmp_parts_d(fp_number_type* a, fp_number_type* b)` (0x004123e8):
/// the soft-float double compare core. 1 if either is a NaN; infinities and
/// zeros by sign; else by sign, exponent, then fraction. Result -1/0/1 in
/// v0 (two infinities: `b.sign - a.sign`). Leaves the original's scratch in
/// a0/a2/a3/v1.
pub fn fpcmp_parts_d(c: &mut Cpu) {
    use crate::guest::Addr;
    let a = c.g[A0][0] as u32;
    let b = c.g[A1][0] as u32;
    /// The shared tail: `v0` (already -1) becomes `v1` (1) when the sign
    /// is clear (gcc's `movz`).
    fn by_sign(c: &mut Cpu, cycles: u64, sign: u64) {
        c.cycles += cycles;
        if sign == 0 {
            c.g[V0][0] = c.g[V1][0];
        }
        ret(c);
    }

    c.cycles += 4;
    let class_a = Addr(a + FP_CLASS).lw(c);
    c.g[A2][0] = class_a;
    c.g[V0][0] = (class_a < 2) as u64;
    if class_a >= 2 {
        c.cycles += 4;
        let class_b = Addr(b + FP_CLASS).lw(c);
        c.g[V1][0] = class_b;
        c.g[V0][0] = class_a ^ CLASS_INFINITY;
        if class_b >= 2 {
            c.cycles += 2;
            c.g[V0][0] = class_b ^ CLASS_INFINITY;
            let sign_a;
            if class_a != CLASS_INFINITY {
                c.cycles += 2;
                if class_b == CLASS_INFINITY {
                    // Finite against infinity: the sign of b decides.
                    c.cycles += 5;
                    let sign_b = Addr(b + FP_SIGN).lw(c);
                    c.g[A0][0] = sign_b;
                    c.g[V1][0] = u64::MAX;
                    c.g[V0][0] = 1;
                    if sign_b == 0 {
                        c.g[V0][0] = c.g[V1][0];
                    }
                    return ret(c);
                }
                c.g[V0][0] = class_a ^ CLASS_ZERO;
                c.cycles += 2;
                c.g[V0][0] = class_b ^ CLASS_ZERO;
                if class_a == CLASS_ZERO {
                    c.cycles += 2;
                    if class_b != CLASS_ZERO {
                        // Zero against a number: the sign of b decides.
                        let sign_b = Addr(b + FP_SIGN).lw(c);
                        c.g[A0][0] = sign_b;
                        c.cycles += 4;
                        c.g[V1][0] = 1;
                        c.g[V0][0] = u64::MAX;
                        if sign_b != 0 {
                            c.g[V0][0] = c.g[V1][0];
                        }
                        return ret(c);
                    }
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    return ret(c);
                }
                c.cycles += 2;
                if class_b != CLASS_ZERO {
                    return compare_normals(c, a, b);
                }
                // A number against zero: the sign of a decides.
                sign_a = Addr(a + FP_SIGN).lw(c);
            } else {
                c.cycles += 2;
                if class_b == CLASS_INFINITY {
                    // Two infinities: b.sign - a.sign.
                    c.cycles += 4;
                    let sb = Addr(b + FP_SIGN).lw(c);
                    c.g[V1][0] = sb;
                    let sa = Addr(a + FP_SIGN).lw(c);
                    c.g[V0][0] = sx((sb as u32).wrapping_sub(sa as u32));
                    return ret(c);
                }
                // Infinity against a finite number: the sign of a decides.
                sign_a = Addr(a + FP_SIGN).lw(c);
            }
            c.g[A0][0] = sign_a;
            c.g[V1][0] = 1;
            c.g[V0][0] = u64::MAX;
            return by_sign(c, 4, sign_a);
        }
    }
    // A NaN: unordered.
    c.cycles += 2;
    c.g[V0][0] = 1;
    ret(c);
}

/// Both operands normal (the `'b_4124a0` block of `__fpcmp_parts_d`).
pub(crate) fn fpcmp_normals_tail(c: &mut Cpu, cycles: u64, v0: u64, sign_a: u64) {
    c.cycles += cycles;
    c.g[V0][0] = v0;
    if sign_a == 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    ret(c);
}

/// Two normal numbers: different signs -> by the sign of a; same sign ->
/// by exponent, then by fraction (`v1` = the result for a positive `a`,
/// negated by the sign).
pub(crate) fn compare_normals(c: &mut Cpu, a: u32, b: u32) {
    use crate::guest::Addr;
    c.cycles += 4;
    let sign_a = Addr(a + FP_SIGN).lw(c);
    c.g[A3][0] = sign_a;
    let sign_b = Addr(b + FP_SIGN).lw(c);
    c.g[V0][0] = sign_b;
    if sign_a != sign_b {
        c.cycles += 1;
        c.g[V1][0] = 1;
        return fpcmp_normals_tail(c, 3, u64::MAX, sign_a);
    }
    let exp_a = Addr(a + FP_EXP).lw(c);
    c.g[A2][0] = exp_a;
    c.cycles += 4;
    let exp_b = Addr(b + FP_EXP).lw(c);
    c.g[V1][0] = exp_b;
    c.g[V0][0] = ((exp_b as i64) < (exp_a as i64)) as u64;
    if (exp_b as i64) < (exp_a as i64) {
        // |a| > |b|.
        c.g[V1][0] = 1;
        return fpcmp_normals_tail(c, 3, u64::MAX, sign_a);
    }
    c.cycles += 3;
    c.g[V0][0] = ((exp_a as i64) < (exp_b as i64)) as u64;
    if (exp_a as i64) < (exp_b as i64) {
        // |a| < |b|.
        c.cycles += 1;
        c.g[V1][0] = u64::MAX;
        return fpcmp_normals_tail(c, 3, 1, sign_a);
    }
    let frac_a = Addr(a + FP_FRAC).ld(c);
    c.g[V1][0] = frac_a;
    c.cycles += 4;
    let frac_b = Addr(b + FP_FRAC).ld(c);
    c.g[A0][0] = frac_b;
    c.g[V0][0] = (frac_b < frac_a) as u64;
    if frac_b < frac_a {
        c.g[V1][0] = 1;
        return fpcmp_normals_tail(c, 3, u64::MAX, sign_a);
    }
    c.cycles += 3;
    c.g[V0][0] = (frac_a < frac_b) as u64;
    c.g[V1][0] = u64::MAX;
    if frac_a < frac_b {
        return fpcmp_normals_tail(c, 3, 1, sign_a);
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    ret(c);
}

// ---------------------------------------------------------------------------
// __floatsidf (0x412550)

pub(crate) const PACK_D_3: Callee = Callee {
    addr: 0x0041_1b08,
    f: ee::s15::__pack_d,
};

pub(crate) const CLASS_NUMBER: u64 = 3;

/// Fraction bits (FRACBITS + NGARDS = 60): the fraction is normalised so
/// its top set bit is bit 60.
pub(crate) const FRAC_EXP: u64 = 0x3c;

/// `double __floatsidf(int i)` (0x412550, libgcc soft-float): build the
/// unpacked number (zero, or sign and magnitude with exponent 60,
/// normalised) and pack it with `__pack_d`. INT_MIN returns -2^31 directly.
pub fn floatsidf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        let num = Addr(sp(c));
        c.g[V0][0] = CLASS_NUMBER;
        c.g[V1][0] = sx((c.g[A0][0] as u32) >> 31); // sign
        frame.save_ra(c, 0x20);
        num.off(FP_CLASS).sw(c, c.g[V0][0]);
        num.off(FP_SIGN).sw(c, c.g[V1][0]);
        if c.g[A0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = CLASS_ZERO;
            num.off(FP_CLASS).sw(c, c.g[V0][0]);
        } else {
            c.cycles += 3;
            c.g[V0][0] = FRAC_EXP;
            num.off(FP_EXP).sw(c, c.g[V0][0]);
            if c.g[V1][0] == 0 {
                c.cycles += 1;
                num.off(FP_FRAC).sd(c, c.g[A0][0]);
            } else {
                c.cycles += 3;
                let int_min = c.g[A0][0] == 0xffff_ffff_8000_0000;
                c.g[V0][0] = sx(0u32.wrapping_sub(c.g[A0][0] as u32));
                if int_min {
                    // -2^31 as a double.
                    c.cycles += 4;
                    c.g[V0][0] = 0xc1e0 << 48;
                    frame.restore_ra(c, 0x20);
                    c.cycles += 2;
                    frame.close(c);
                    return Ok(());
                }
                c.cycles += 2;
                num.off(FP_FRAC).sd(c, c.g[V0][0]);
            }
            // Normalise: shift the fraction up to bit 60.
            c.cycles += 6;
            c.g[A1][0] = num.off(FP_FRAC).ld(c);
            c.g[V0][0] = u64::MAX >> 4;
            c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
            c.g[A0][0] = num.off(FP_EXP).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.g[A2][0] = u64::MAX >> 4;
                loop {
                    c.cycles += 7;
                    c.g[V1][0] = c.g[A1][0] << 1;
                    c.g[A0][0] = addu(c.g[A0][0], (-1i64) as u64);
                    c.g[A1][0] = c.g[V1][0];
                    c.g[V0][0] = (c.g[A2][0] < c.g[V1][0]) as u64;
                    if c.g[V0][0] != 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0041_25d0);
                    }
                }
                c.cycles += 2;
                num.off(FP_EXP).sw(c, c.g[A0][0]);
                num.off(FP_FRAC).sd(c, c.g[V1][0]);
            }
        }
        c.cycles += 3;
        c.g[A0][0] = c.g[SP][0];
        PACK_D_3.call(c, 0x0041_25fc)?;
        frame.restore_ra(c, 0x20);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod expf {

    /// Above this |x| bit pattern the result overflows or underflows.
    pub const LIMIT: u64 = 0x42b1_7217;

    pub const INF: u64 = 0x7f80_0000;

    /// `o_threshold` 88.72168 and `u_threshold` -103.97208.
    pub const O_THRESHOLD: u64 = 0x42b1_7180;

    pub const U_THRESHOLD: u64 = 0xffff_ffff_c2cf_f1b5;

    /// `0.5 ln2`, `1.5 ln2` (|x| bit patterns) and `2^-28`.
    pub const HALF_LN2: u64 = 0x3eb1_7218;

    pub const THREE_HALVES_LN2: u64 = 0x3f85_1591;

    pub const TINY: u64 = 0x317f_ffff;

    /// `huge`, `invln2`, `twom100`.
    pub const HUGE: u64 = 0x7149_f2ca;

    pub const INVLN2: u64 = 0x3fb8_aa3b;

    pub const TWOM100: u64 = 0x0d80_0000;

    pub const ONE: u64 = 0x3f80_0000;

    pub const TWO: u64 = 0x4000_0000;

    /// Polynomial coefficients P1..P5 (applied P5 first).
    pub const P: [u64; 5] = [
        0x3331_bb4c,
        0xffff_ffff_b5dd_ea0e,
        0x388a_b355,
        0xffff_ffff_bb36_0b61,
        0x3e2a_aaab,
    ];

    /// `.rodata` tables (by sign): `halF[2]`, `ln2HI[2]`, `ln2LO[2]`, and
    /// the overflow result `huge * huge`.
    pub const HALF_F: u32 = 0x0049_91a0;

    pub const LN2_HI: u32 = 0x0049_91b8;

    pub const LN2_LO: u32 = 0x0049_91c0;

    pub const OVERFLOW: u32 = 0x0049_91e0;

    /// `k` below which the result is scaled through `2^-100`.
    pub const K_SUBNORMAL: i64 = -125;
}

pub(crate) mod fp_class {

    pub const SNAN: u64 = 0;

    pub const QNAN: u64 = 1;

    pub const ZERO: u64 = 2;

    pub const NUMBER: u64 = 3;

    pub const INFINITY: u64 = 4;
}

// ---------------------------------------------------------------------------
// __fixunsdfsi (0x4126a0)

/// `__unpack_d(src, dst)` (0x411c38).
pub(crate) const UNPACK_D_3: Callee = Callee {
    addr: 0x0041_1c38,
    f: ee::s15::__unpack_d,
};

/// `unsigned __fixunsdfsi(double x)` (0x4126a0, libgcc soft-float):
/// truncate `x` (in a0) to an unsigned int: zero, NaN, negative numbers
/// and |x| < 1 give 0, infinity and x >= 2^32 give 0xffffffff, otherwise
/// the fraction shifted down to the integer part (sign-extended in v0).
pub fn fixunsdfsi(c: &mut Cpu) {
    use fp_number::*;
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        let s = sp(c);
        c.sd(s.wrapping_add(0x20), c.g[A0][0]);
        c.g[A1][0] = c.g[SP][0];
        frame.save_ra(c, 0x30);
        c.g[A0][0] = sx(s.wrapping_add(0x20));
        UNPACK_D_3.call(c, 0x0041_26b8)?;
        let class = c.lw(s.wrapping_add(CLASS));
        c.g[V1][0] = class;
        let result: u64 = 'r: {
            if class == CLASS_ZERO {
                c.cycles += 2;
                break 'r 0;
            }
            // NaN.
            c.cycles += 2;
            c.g[V0][0] = c.lw(s.wrapping_add(SIGN));
            if class < 2 {
                c.cycles += 2;
                break 'r 0;
            }
            // Negative.
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                break 'r 0;
            }
            c.cycles += 3;
            let exp = c.lw(s.wrapping_add(EXP));
            c.g[A0][0] = exp;
            if class == CLASS_INFINITY {
                c.cycles += 3;
                break 'r u64::MAX;
            }
            // |x| < 1: the original's backward branch to the `return 0`
            // block checks for events.
            c.cycles += 2;
            if (exp as i64) < 0 {
                if event_due(c) {
                    deliver_event_here(c, 0x0041_26d0);
                }
                c.cycles += 2;
                break 'r 0;
            }
            // Too large.
            c.cycles += 2;
            if (exp as i64) >= 32 {
                c.cycles += 3;
                break 'r u64::MAX;
            }
            c.cycles += 2;
            c.g[V1][0] = FRACBITS;
            let fraction = c.ld(s.wrapping_add(FRACTION));
            let int = if (exp as i64) < 61 {
                c.cycles += 3;
                let shift = sx((FRACBITS as u32).wrapping_sub(exp as u32));
                c.g[V1][0] = shift;
                fraction >> (shift & 63)
            } else {
                c.cycles += 4;
                let shift = sx((exp as u32).wrapping_sub(FRACBITS as u32));
                c.g[V1][0] = shift;
                fraction << (shift & 63)
            };
            c.cycles += 2;
            sx(int as u32)
        };
        c.g[V0][0] = result;
        c.cycles += 3;
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `at = k; fN = at` (a `lui/ori; mtc1` float constant).
pub(crate) fn expf_const(c: &mut Cpu, f: usize, bits: u64) {
    c.g[AT][0] = bits;
    c.mtc1(AT, f);
}

/// `float __ieee754_expf(float x)` (0x40e048, fdlibm): `e^x`. Large |x|
/// overflows to `huge*huge` or underflows to 0 (inf gives inf or 0, NaN
/// gives NaN); otherwise `x = k ln2 + r` and `e^r` by the rational
/// approximation, scaled by `2^k`.
pub fn ieee754_expf(c: &mut Cpu) {
    use expf::*;
    c.cycles += 11;
    c.mfc1(V0, 12);
    let bits = c.g[V0][0];
    c.g[A0][0] = bits;
    let xsb = sx((bits as u32) >> 31);
    c.g[A3][0] = xsb;
    let hx = bits & 0x7fff_ffff;
    c.g[A0][0] = hx;
    c.g[V0][0] = (LIMIT < hx) as u64;
    c.g[V1][0] = INF;
    if LIMIT < hx {
        c.cycles += 3;
        c.g[V0][0] = (INF < hx) as u64;
        if INF < hx {
            // NaN.
            c.cycles += 2;
            c.add_s(0, 12, 12);
            ret(c);
            return;
        }
        c.cycles += 2;
        if hx == INF {
            // e^+inf = inf, e^-inf = 0.
            c.cycles += 3;
            c.mtc1(0, 0);
            if xsb == 0 {
                c.mov_s(0, 12);
            }
            c.cycles += 2;
            ret(c);
            return;
        }
        c.cycles += 7;
        expf_const(c, 0, O_THRESHOLD);
        c.c_lt_s(0, 12);
        c.g[V0][0] = 0x4a_0000;
        if c.fcc() {
            // Overflow.
            c.cycles += 2;
            c.lwc1(0, OVERFLOW);
            ret(c);
            return;
        }
        c.cycles += 7;
        expf_const(c, 0, U_THRESHOLD);
        c.c_lt_s(12, 0);
        c.g[V0][0] = 0x3eb1_0000;
        if c.fcc() {
            // Underflow.
            c.cycles += 3;
            c.mtc1(0, 0);
            ret(c);
            return;
        }
    } else {
        c.cycles += 1;
        c.g[V0][0] = 0x3eb1_0000;
    }

    // Argument reduction: x = hi - lo = x - k ln2 (k in a2); f5 = x*x.
    c.cycles += 4;
    c.g[V0][0] = (HALF_LN2 < hx) as u64;
    c.g[V0][0] = 0x3f85_0000;
    if HALF_LN2 < hx {
        c.cycles += 4;
        c.g[V0][0] = (THREE_HALVES_LN2 < hx) as u64;
        c.g[V0][0] = 0x4a_0000;
        if THREE_HALVES_LN2 < hx {
            // k = (int)(invln2 * x + halF[xsb]).
            c.cycles += 20;
            expf_const(c, 0, INVLN2);
            c.g[V0][0] = sx(HALF_F);
            let half = HALF_F.wrapping_add((xsb as u32) << 2);
            c.g[V1][0] = sx(half);
            c.mul_s(0, 12, 0);
            c.lwc1(2, half);
            c.g[V0][0] = 0x4a_0000;
            c.lwc1(1, LN2_HI);
            c.g[V1][0] = 0x4a_0000;
            c.add_s(0, 0, 2);
            c.lwc1(3, LN2_LO);
            c.cvt_w_s(2, 0);
            c.mfc1(A2, 2);
            c.mtc1(A2, 5);
            c.cvt_s_w(5, 5);
            c.mul_s(1, 5, 1);
            c.mul_s(6, 5, 3);
            c.sub_s(7, 12, 1);
        } else {
            // k = 1 - xsb - xsb.
            c.cycles += 14;
            let idx = sx((xsb as u32) << 2);
            c.g[A1][0] = idx;
            let hi_at = LN2_HI.wrapping_add(idx as u32);
            c.g[V0][0] = sx(hi_at);
            c.g[A0][0] = sx(LN2_LO);
            c.lwc1(0, hi_at);
            let lo_at = LN2_LO.wrapping_add(idx as u32);
            c.g[A1][0] = sx(lo_at);
            let one_minus = sx(1u32.wrapping_sub(xsb as u32));
            c.g[V1][0] = one_minus;
            c.sub_s(7, 12, 0);
            c.g[A2][0] = sx((one_minus as u32).wrapping_sub(xsb as u32));
            c.lwc1(6, lo_at);
        }
        c.cycles += 2;
        c.sub_s(12, 7, 6);
        c.cycles += 1;
        c.mul_s(5, 12, 12);
    } else {
        c.cycles += 5;
        c.g[V0][0] = (TINY < hx) as u64;
        if TINY < hx {
            c.g[A2][0] = 0;
            c.cycles += 1;
            c.mul_s(5, 12, 12);
        } else {
            // |x| < 2^-28: 1 + x (raising inexact).
            c.cycles += 10;
            expf_const(c, 0, HUGE);
            expf_const(c, 1, ONE);
            c.add_s(0, 12, 0);
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.cycles += 2;
                c.add_s(0, 12, 1);
                ret(c);
                return;
            }
            c.mul_s(5, 12, 12);
        }
    }

    // c = x - t*(P1 + t*(P2 + t*(P3 + t*(P4 + t*P5)))).
    c.cycles += 26;
    expf_const(c, 0, P[0]);
    expf_const(c, 1, P[1]);
    expf_const(c, 2, P[2]);
    c.mul_s(0, 5, 0);
    expf_const(c, 3, P[3]);
    expf_const(c, 4, P[4]);
    for f in 1..5 {
        c.add_s(0, 0, f);
        c.mul_s(0, 5, 0);
    }
    let k = c.g[A2][0];
    c.sub_s(3, 12, 0);
    if k != 0 {
        // y = 1 - ((lo - x*c/(2-c)) - hi), scaled by 2^k.
        c.cycles += 16;
        expf_const(c, 1, TWO);
        c.mul_s(0, 12, 3);
        expf_const(c, 2, ONE);
        let subnormal = (k as i64) < K_SUBNORMAL;
        c.g[V0][0] = subnormal as u64;
        c.sub_s(1, 1, 3);
        c.div_s(0, 0, 1);
        c.sub_s(0, 6, 0);
        c.sub_s(0, 0, 7);
        c.sub_s(2, 2, 0);
        c.mfc1(V1, 2);
        let y = c.g[V1][0];
        c.g[V0][0] = y;
        if subnormal {
            c.cycles += 9;
            let e = sx(((k as u32).wrapping_add(100)) << 23);
            c.g[V1][0] = sx((y as u32).wrapping_add(e as u32));
            expf_const(c, 0, TWOM100);
            c.mtc1(V1, 1);
            c.mul_s(0, 1, 0);
        } else {
            c.cycles += 5;
            let e = sx((k as u32) << 23);
            c.g[V0][0] = e;
            c.g[V1][0] = sx((y as u32).wrapping_add(e as u32));
            c.mtc1(V1, 0);
        }
    } else {
        // y = 1 - ((x*c)/(c-2) - x).
        c.cycles += 12;
        expf_const(c, 1, TWO);
        c.mul_s(0, 12, 3);
        expf_const(c, 2, ONE);
        c.sub_s(1, 3, 1);
        c.div_s(0, 0, 1);
        c.sub_s(0, 0, 12);
        c.sub_s(0, 2, 0);
    }
    ret(c);
}

/// ln2_hi, ln2_lo (newlib's split of ln 2).
pub(crate) const LN2_HI: u32 = 0x3f31_7180;

pub(crate) const LN2_LO: u32 = 0x3717_f7d1;

/// `float __ieee754_logf(float x)` (0x40e550): newlib's single-precision
/// natural logarithm. x = 2^k (1 + f): subnormals are scaled by 2^25
/// first; log(+-0) is the constant at 0x499224 (-huge), log(x < 0) =
/// (x - x) / 0, log(inf/NaN) = x + x. For tiny f a short polynomial; else
/// s = f / (2 + f) and the Remez series in s^2, combined with k ln 2
/// (split hi/lo). A leaf; every scratch register (a0-a3, t0, v0, v1, at,
/// f0-f10) left as the compiled code leaves it.
pub fn ieee754_logf(c: &mut Cpu) {
    c.cycles += 7;
    c.mfc1(A0, 12);
    c.g[A3][0] = c.g[A0][0];
    c.g[V0][0] = 0x7f_ffff;
    let normal = 0x7f_ffff < (c.g[A3][0] as i64);
    c.g[V0][0] = normal as u64;
    c.g[T0][0] = 0;
    if !normal {
        // 0x40e574: zero, negative or subnormal.
        c.cycles += 5;
        let nonzero = c.g[A3][0] & 0x7fff_ffff != 0;
        c.g[V0][0] = 0x4a_0000;
        if !nonzero {
            c.cycles += 2;
            c.lwc1(0, 0x004a_0000u32.wrapping_sub(0x6ddc));
            return ret(c);
        }
        c.cycles += 2;
        c.g[T0][0] = sx(-25i32 as u32);
        if (c.g[A3][0] as i64) < 0 {
            // (x - x) / 0.
            c.cycles += 8;
            c.mtc1(A0, 0);
            c.sub_s(1, 0, 0);
            c.mtc1(0, 0);
            c.div_s(0, 1, 0);
            return ret(c);
        }
        // Subnormal: x * 2^25.
        c.cycles += 6;
        lit(c, 0, 0x4c00_0000);
        c.mtc1(A0, 1);
        c.mul_s(1, 1, 0);
        c.mfc1(A0, 1);
        c.g[A3][0] = c.g[A0][0];
    }
    // 0x40e5c8
    c.cycles += 5;
    let finite = (c.g[A3][0] as i64) <= 0x7f7f_ffff;
    c.g[V0][0] = (!finite) as u64;
    c.g[A1][0] = 0x7f_0000;
    if !finite {
        c.cycles += 3;
        c.mtc1(A0, 2);
        c.add_s(0, 2, 2);
        return ret(c);
    }

    // 0x40e5e0: k (t0), the mantissa normalised into [sqrt(2)/2, sqrt(2))
    // (a0), f = x - 1 (f12).
    c.cycles += 23;
    let hx = r32(c, A3);
    c.g[A2][0] = sx(((hx as i32) >> 23) as u32);
    c.g[A1][0] = 0x7f_ffff;
    let mant = hx & 0x7f_ffff;
    c.g[A3][0] = mant as u64;
    c.g[V0][0] = sx(r32(c, T0).wrapping_sub(0x7f));
    let i = mant.wrapping_add(0x4a_fb20) & 0x80_0000;
    c.g[A0][0] = 0x80_0000;
    c.g[T0][0] = sx(r32(c, V0).wrapping_add(r32(c, A2)));
    c.g[V0][0] = (i ^ 0x3f80_0000) as u64;
    c.g[A0][0] = (mant | i ^ 0x3f80_0000) as u64;
    c.g[V1][0] = sx(((i as i32) >> 23) as u32);
    lit(c, 0, 0x3f80_0000);
    let tiny = (mant.wrapping_add(0xf) & 0x7f_ffff) < 16;
    c.mtc1(A0, 1);
    c.g[T0][0] = sx(r32(c, T0).wrapping_add(r32(c, V1)));
    c.g[V0][0] = tiny as u64;
    c.sub_s(12, 1, 0);
    let k_zero = c.g[T0][0] == 0;
    'tail: {
        if tiny {
            // 0x40e7f4: |f| < 2^-20.
            c.cycles += 5;
            c.mtc1(0, 0);
            c.c_eq_s(12, 0);
            if c.fcc() {
                // f == 0: k ln 2 (0 when k == 0).
                c.cycles += 2;
                if k_zero {
                    break 'tail;
                }
                c.cycles += 12;
                c.mtc1(T0, 9);
                c.cvt_s_w(9, 9);
                lit(c, 1, LN2_HI);
                lit(c, 0, LN2_LO);
                c.mul_s(1, 9, 1);
                c.mul_s(0, 9, 0);
                c.add_s(0, 1, 0);
                return ret(c);
            }
            // R = f^2 (0.5 - f / 3).
            c.mul_s(2, 12, 12);
            c.cycles += 9;
            lit(c, 0, 0x3eaa_aaab);
            lit(c, 1, 0x3f00_0000);
            c.mul_s(0, 12, 0);
            c.sub_s(1, 1, 0);
            c.mul_s(2, 2, 1);
            if !k_zero {
                c.cycles += 14;
                c.mtc1(T0, 9);
                c.cvt_s_w(9, 9);
                lit(c, 0, LN2_LO);
                lit(c, 1, LN2_HI);
                c.mul_s(0, 9, 0);
                c.mul_s(1, 9, 1);
                c.sub_s(0, 2, 0);
                c.sub_s(0, 0, 12);
                c.sub_s(0, 1, 0);
            } else {
                c.cycles += 2;
                c.sub_s(0, 12, 2);
            }
            return ret(c);
        }

        // 0x40e660: s = f / (2 + f), R = the series in s^2 (f2).
        c.cycles += 52;
        lit(c, 0, 0x4000_0000);
        c.g[V1][0] = 0xffff_ffff_ffcf_0000;
        lit(c, 1, 0x3e17_8897); // Lg7
        c.g[V0][0] = 0x35_0000;
        c.add_s(0, 12, 0);
        lit(c, 8, 0x3e3a_3325); // Lg5
        lit(c, 2, 0x3e1c_d04f); // Lg6
        c.g[V1][0] |= 0x5c30;
        lit(c, 4, 0x3e63_8e29); // Lg4
        c.g[V0][0] |= 0xc288;
        c.div_s(10, 12, 0);
        lit(c, 5, 0x3e92_4925); // Lg3
        lit(c, 6, 0x3ecc_cccd); // Lg2
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(mant));
        lit(c, 7, 0x3f2a_aaab); // Lg1
        c.g[V1][0] = sx(mant.wrapping_add(r32(c, V1)));
        c.g[V1][0] |= c.g[V0][0];
        c.mtc1(T0, 9);
        c.cvt_s_w(9, 9);
        c.mul_s(3, 10, 10);
        c.mul_s(0, 3, 3);
        c.mul_s(1, 0, 1);
        c.mul_s(2, 0, 2);
        c.add_s(1, 1, 8);
        c.add_s(2, 2, 4);
        c.mul_s(1, 0, 1);
        c.mul_s(2, 0, 2);
        c.add_s(1, 1, 5);
        c.add_s(2, 2, 6);
        c.mul_s(1, 0, 1);
        c.mul_s(0, 0, 2);
        c.add_s(1, 1, 7);
        c.mul_s(3, 3, 1);
        let small = (c.g[V1][0] as i64) <= 0;
        c.add_s(2, 3, 0);
        if small {
            // f - s (f - R).
            c.cycles += 2;
            c.sub_s(0, 12, 2);
            if k_zero {
                c.cycles += 3;
                c.mul_s(0, 10, 0);
                c.sub_s(0, 12, 0);
                return ret(c);
            }
            c.cycles += 12;
            lit(c, 1, LN2_LO);
            lit(c, 2, LN2_HI);
            c.mul_s(1, 9, 1);
            c.mul_s(0, 10, 0);
            c.mul_s(2, 9, 2);
            c.sub_s(0, 0, 1);
            c.sub_s(0, 0, 12);
            c.sub_s(0, 2, 0);
            break 'tail;
        }
        // hfsq = f^2 / 2 (f4).
        c.cycles += 5;
        lit(c, 0, 0x3f00_0000);
        c.mul_s(0, 12, 0);
        c.mul_s(4, 0, 12);
        if !k_zero {
            c.cycles += 15;
            c.add_s(0, 4, 2);
            lit(c, 1, LN2_LO);
            lit(c, 2, LN2_HI);
            c.mul_s(1, 9, 1);
            c.mul_s(0, 10, 0);
            c.mul_s(2, 9, 2);
            c.add_s(0, 0, 1);
            c.sub_s(0, 4, 0);
            c.sub_s(0, 0, 12);
            c.sub_s(0, 2, 0);
        } else {
            c.cycles += 5;
            c.add_s(0, 4, 2);
            c.mul_s(0, 10, 0);
            c.sub_s(0, 4, 0);
            c.sub_s(0, 12, 0);
        }
        return ret(c);
    }
    c.cycles += 2;
    ret(c);
}

/// `void __unpack_f(const float* src, fp_number_type* dst)` (0x413068):
/// split a float into class (`+0`), sign (`+4`), unbiased exponent (`+8`)
/// and fraction (`+0xc`, with the implicit bit at 1 << 30). This build has
/// no denormals: a zero exponent is always ZERO. NaN fractions keep their
/// raw 23 bits; bit 20 (this build's quiet bit) selects QNAN over SNAN.
pub fn unpack_f(c: &mut Cpu) {
    c.cycles += 9;
    let dst = Addr(abi::arg(c, 1));
    c.g[V0][0] = Addr(abi::arg(c, 0)).lw(c);
    let bits = c.g[V0][0] as u32;
    c.g[V1][0] = 0x7f_ffff;
    c.g[A0][0] = sx(bits >> 31);
    c.g[A2][0] = sx(bits >> 23) & 0xff;
    let fraction = c.g[V0][0] & 0x7f_ffff;
    c.g[V1][0] = fraction;
    let exponent = c.g[A2][0];
    dst.off(4).sw(c, c.g[A0][0]);
    if exponent == 0 {
        c.cycles += 3;
        c.g[V0][0] = fp_class::ZERO;
        dst.sw(c, fp_class::ZERO);
    } else if exponent != 0xff {
        c.cycles += 3 + 8;
        c.g[V1][0] = sx((fraction as u32) << 7) | 0x4000_0000;
        c.g[A0][0] = sx((exponent as u32).wrapping_sub(0x7f));
        c.g[V0][0] = fp_class::NUMBER;
        dst.off(0xc).sw(c, c.g[V1][0]);
        dst.off(8).sw(c, c.g[A0][0]);
        dst.sw(c, fp_class::NUMBER);
    } else if fraction != 0 {
        c.cycles += 3 + 2 + 3;
        c.g[V0][0] = 1;
        if fraction & 0x10_0000 != 0 {
            c.cycles += 2;
            dst.sw(c, fp_class::QNAN);
        } else {
            c.cycles += 1;
            dst.sw(c, fp_class::SNAN);
        }
        c.cycles += 2;
        dst.off(0xc).sw(c, fraction);
    } else {
        c.cycles += 3 + 2 + 3;
        c.g[V0][0] = fp_class::INFINITY;
        dst.sw(c, fp_class::INFINITY);
    }
    abi::ret(c);
}

pub(crate) mod fixunssfsi {
    use super::ee;
    use super::Callee;

    pub const ENTRY: u32 = 0x0041_39f8;

    /// The "answer 0" block the negative-exponent branch (event-checked)
    /// goes to: a resume point.
    pub const ZERO: u32 = 0x0041_3a28;

    pub const END: u32 = 0x0041_3a90;

    /// `__unpack_f(const float* in, fp_number_type* out)`.
    pub const UNPACK_F: Callee = Callee {
        addr: 0x0041_3068,
        f: ee::s15::f_00413068,
    };

    /// `fp_number_type` at sp+0: class, sign, normal exponent, fraction
    /// (`FRACBITS` 30, the implicit one at bit 30).
    pub const CLASS: u32 = 0x0;

    pub const SIGN: u32 = 0x4;

    pub const EXP: u32 = 0x8;

    pub const FRAC: u32 = 0xc;

    /// Classes: 0 sNaN, 1 qNaN, 2 zero, 4 infinity (3 normal).
    pub const CLASS_ZERO: u64 = 2;

    pub const CLASS_INFINITY: u64 = 4;
}

pub(crate) mod softfp {
    use super::*;

    pub const UNPACK_D: Callee = Callee {
        addr: 0x0041_1c38,
        f: ee::s15::__unpack_d,
    };

    pub const FPADD_PARTS_D: Callee = Callee {
        addr: 0x0041_1cd8,
        f: ee::s15::_fpadd_parts_d,
    };

    pub const PACK_D: Callee = Callee {
        addr: 0x0041_1b08,
        f: ee::s15::__pack_d,
    };

    /// The frame: unpacked a at 0, unpacked b at 0x20, the sum at 0x40,
    /// the packed arguments at 0x60 / 0x68.
    pub const A_PARTS: u32 = 0x0;

    pub const B_PARTS: u32 = 0x20;

    pub const SUM_PARTS: u32 = 0x40;

    pub const A_PACKED: u32 = 0x60;

    pub const B_PACKED: u32 = 0x68;

    /// The sign word of an unpacked number.
    pub const SIGN: u32 = 0x4;
}

/// `unsigned __fixunssfsi(float f)` (libgcc soft-float, 0x4139f8): float to
/// unsigned int. NaN, zero, negative and below-one values give 0, infinity
/// and values of 2^32 and above give 0xffffffff, else the fraction shifted
/// by the exponent. Resumable at 0x413a28 (the "answer 0" block, reached
/// through an event check). Frame 0x30 (the unpacked number at +0, `f` at
/// +0x10).
pub fn fixunssfsi(c: &mut Cpu) {
    use fixunssfsi::*;
    guest(c, |c| {
        let frame = Frame { size: 0x30 };
        'zero: {
            match c.pc {
                ENTRY => {}
                ZERO => break 'zero,
                pc => {
                    c.jump_out(pc, ENTRY, END);
                    return Ok(());
                }
            }
            c.cycles += 10;
            let frame = Frame::open(c, 0x30);
            frame.save_ra(c, 0x20);
            let s = sp(c);
            c.g[A0][0] = sx(s.wrapping_add(0x10));
            c.swc1(12, s.wrapping_add(0x10));
            c.g[A1][0] = sx(s);
            UNPACK_F.call(c, 0x0041_3a10)?;
            let s = sp(c);
            let class = c.lw(s.wrapping_add(CLASS));
            c.g[V1][0] = class;
            c.g[V0][0] = (class < 2) as u64;
            if class == CLASS_ZERO {
                break 'zero;
            }
            // NaN: 0.
            c.cycles += 2;
            let sign = c.lw(s.wrapping_add(SIGN));
            c.g[V0][0] = sign;
            if class < 2 {
                break 'zero;
            }
            // Negative: 0.
            c.cycles += 2;
            c.g[V0][0] = 0;
            if sign != 0 {
                fixunssfsi_exit(c, frame);
                return Ok(());
            }
            c.cycles += 3;
            c.g[V0][0] = class ^ CLASS_INFINITY;
            let exp = c.lw(s.wrapping_add(EXP));
            c.g[A0][0] = exp;
            let exp = exp as i64;
            let saturate = 'sat: {
                if class == CLASS_INFINITY {
                    break 'sat true;
                }
                // Below one: 0 (an event check on the way).
                c.cycles += 2;
                c.g[V0][0] = (exp < 32) as u64;
                if exp < 0 {
                    if back_edge_unwinds(c, ZERO) {
                        return Ok(());
                    }
                    break 'zero;
                }
                c.cycles += 2;
                exp >= 32
            };
            if saturate {
                c.cycles += 3;
                c.g[V0][0] = u64::MAX;
                fixunssfsi_exit(c, frame);
                return Ok(());
            }
            c.g[V0][0] = (exp < 31) as u64;
            c.cycles += 2;
            if exp < 31 {
                // fraction >> (30 - exp).
                c.g[V0][0] = 0x1e;
                c.cycles += 3;
                let frac = c.lw(s.wrapping_add(FRAC));
                c.g[V1][0] = frac;
                let shift = 30u32.wrapping_sub(exp as u32);
                c.g[V0][0] = sx(shift);
                c.g[V0][0] = sx((frac as u32) >> (shift & 31));
            } else {
                // exp == 31: fraction << 1.
                c.cycles += 4;
                let frac = c.lw(s.wrapping_add(FRAC));
                c.g[V1][0] = frac;
                let shift = (exp as u32).wrapping_sub(0x1e);
                c.g[V0][0] = sx((frac as u32) << (shift & 31));
            }
            fixunssfsi_exit(c, frame);
            return Ok(());
        }
        c.cycles += 2;
        c.g[V0][0] = 0;
        fixunssfsi_exit(c, frame);
        Ok(())
    })
}

/// The epilogue of `__fixunssfsi`.
pub(crate) fn fixunssfsi_exit(c: &mut Cpu, frame: Frame) {
    c.cycles += 3;
    frame.restore_ra(c, 0x20);
    frame.close(c);
}

/// `double __subdf3(double a /*a0*/, double b /*a1*/)` (0x411f70):
/// libgcc's soft-float `a - b`: unpack both, flip b's sign, add the parts,
/// pack the result (v0).
pub fn subdf3(c: &mut Cpu) {
    use softfp::*;
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x90);
        let sp0 = sp(c);
        c.sd(sp0 + A_PACKED, c.g[A0][0]);
        c.sd(sp0 + B_PACKED, c.g[A1][0]);
        c.g[A0][0] = sx(sp0 + A_PACKED);
        c.sd(sp0 + 0x70, c.g[S0][0]);
        frame.save_ra(c, 0x80);
        c.g[A1][0] = c.g[SP][0];
        UNPACK_D.call(c, 0x0041_1f90)?;
        let sp0 = sp(c);
        c.g[S0][0] = sx(sp0 + B_PARTS);
        c.g[A0][0] = sx(sp0 + B_PACKED);
        c.g[A1][0] = c.g[S0][0];
        UNPACK_D.call(c, 0x0041_1fa0)?;
        // b.sign ^= 1
        let sp0 = sp(c);
        c.g[V0][0] = c.lw(sp0 + B_PARTS + SIGN);
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = sx(sp0 + SUM_PARTS);
        c.g[A0][0] = c.g[SP][0];
        c.g[V0][0] ^= 1;
        c.sw(sp0 + B_PARTS + SIGN, c.g[V0][0]);
        FPADD_PARTS_D.call(c, 0x0041_1fbc)?;
        c.g[A0][0] = c.g[V0][0];
        PACK_D.call(c, 0x0041_1fc4)?;
        frame.restore_ra(c, 0x80);
        let a = frame.at(c, 0x70);
        c.g[S0][0] = c.ld(a);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// __floatdidf (0x411438)

pub(crate) const FLOATSIDF: Callee = Callee {
    addr: 0x0041_2550,
    f: ee::s15::__floatsidf,
};

pub(crate) const MULDF3_2: Callee = Callee {
    addr: 0x0041_1fd8,
    f: ee::s15::__muldf3,
};

pub(crate) const ADDDF3_2: Callee = Callee {
    addr: 0x0041_1f18,
    f: ee::s15::__adddf3,
};

/// 65536.0 and 2^32 as IEEE doubles (`0x81e0 << 47`, `0x83e0 << 47`).
pub(crate) const DF_TWO_16: u64 = 0x81e0 << 47;

pub(crate) const DF_TWO_32: u64 = 0x83e0 << 47;

/// `double __floatdidf(long long x)` (0x411438, libgcc soft-float): the
/// high word converted and scaled by 65536 twice, plus the low word
/// converted as signed and corrected by 2^32 when its top bit is set.
pub fn floatdidf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        let at = frame.at(c, 0);
        let v = reg(c, S0);
        c.sd(at, v);
        let x = reg(c, A0);
        set_reg(c, S0, x);
        let at = frame.at(c, 0x10);
        let v = reg(c, S1);
        c.sd(at, v);
        set_reg(c, S1, DF_TWO_16);
        set_reg(c, A0, ((x as i64) >> 32) as u64);
        frame.save_ra(c, 0x20);
        FLOATSIDF.call(c, 0x0041_1460)?;
        // hi * 65536 * 65536.
        set_reg(c, A1, reg(c, S1));
        set_reg(c, A0, reg(c, V0));
        MULDF3_2.call(c, 0x0041_146c)?;
        set_reg(c, A1, reg(c, S1));
        set_reg(c, A0, reg(c, V0));
        MULDF3_2.call(c, 0x0041_1478)?;
        set_reg(c, S1, reg(c, V0));
        // The low word, as a signed int.
        set_reg(c, V0, 0xffff_ffff);
        let lo = x as u32 as i32 as i64 as u64;
        set_reg(c, S0, lo);
        set_reg(c, A0, lo);
        FLOATSIDF.call(c, 0x0041_1498)?;
        set_reg(c, A0, reg(c, S1));
        if (lo as i64) < 0 {
            c.cycles += 5;
            set_reg(c, A1, DF_TWO_32);
            set_reg(c, A0, reg(c, V0));
            ADDDF3_2.call(c, 0x0041_14b0)?;
            set_reg(c, A0, reg(c, S1));
        }
        c.cycles += 7;
        set_reg(c, A1, reg(c, V0));
        ADDDF3_2.call(c, 0x0041_14bc)?;
        frame.restore_ra(c, 0x20);
        let at = frame.at(c, 0x10);
        let v = c.ld(at);
        set_reg(c, S1, v);
        let at = frame.at(c, 0);
        let v = c.ld(at);
        set_reg(c, S0, v);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// __adddf3 (0x411f18)

/// `double __adddf3(double a /*a0*/, double b /*a1*/)` (0x411f18): libgcc's
/// soft-float `a + b`: unpack both into the frame, add the parts, pack the
/// result (v0). Frame 0x90 (parts of a at 0, of b at 0x20, the sum at 0x40,
/// the packed arguments at 0x60 / 0x68).
pub fn adddf3(c: &mut Cpu) {
    use crate::game::lib::gcc::softfp::*;
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x90);
        let sp0 = sp(c);
        c.sd(sp0 + A_PACKED, c.g[A0][0]);
        c.sd(sp0 + B_PACKED, c.g[A1][0]);
        c.g[A0][0] = sx(sp0 + A_PACKED);
        c.sd(sp0 + 0x70, c.g[S0][0]);
        frame.save_ra(c, 0x80);
        c.g[A1][0] = c.g[SP][0];
        UNPACK_D.call(c, 0x0041_1f38)?;
        let sp0 = sp(c);
        c.g[S0][0] = sx(sp0 + B_PARTS);
        c.g[A0][0] = sx(sp0 + B_PACKED);
        c.g[A1][0] = c.g[S0][0];
        UNPACK_D.call(c, 0x0041_1f48)?;
        let sp0 = sp(c);
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = sx(sp0 + SUM_PARTS);
        c.g[A0][0] = c.g[SP][0];
        FPADD_PARTS_D.call(c, 0x0041_1f58)?;
        c.g[A0][0] = c.g[V0][0];
        PACK_D.call(c, 0x0041_1f60)?;
        frame.restore_ra(c, 0x80);
        let a = frame.at(c, 0x70);
        c.g[S0][0] = c.ld(a);
        frame.close(c);
        Ok(())
    });
}

pub(crate) mod fixdfsi_unpacked {

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

pub(crate) const FIXDFSI_ENTRY: u32 = 0x0041_2608;

/// The "answer 0" block (an event-check resume point).
pub(crate) const FIXDFSI_ZERO: u32 = 0x0041_2638;

pub(crate) const FIXDFSI_END: u32 = 0x0041_269c;

/// The epilogue of `__fixdfsi`.
pub(crate) fn fixdfsi_exit(c: &mut Cpu, frame: Frame) {
    c.cycles += 3;
    frame.restore_ra(c, 0x30);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// __divdf3 (0x412280)

pub(crate) const DIVDF3_ENTRY: u32 = 0x0041_2280;

/// The head of the long-division loop (an event-check resume point).
pub(crate) const DIVDF3_LOOP: u32 = 0x0041_2380;

pub(crate) const DIVDF3_END: u32 = 0x0041_23e8;

/// libgcc's canonical NaN (`__thenan_df`).
pub(crate) const DIVDF3_THE_NAN: u32 = 0x0052_adc0;

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

#[cfg(test)]
pub(crate) mod tests {
    crate::difftest::events::driven!(oc238_gen_fixunssfsi, ee::s15::f_004139f8__generated);
    crate::difftest::events::driven!(oc238_hand_fixunssfsi, fixunssfsi);
    use super::*;
    use crate::difftest::check;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fill;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
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
            c.bus.write32(
                0x0052_ad98,
                if seed % 3 == 0 { 0 } else { r.next_u32() | 1 },
            );
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
        check(
            ssx3_ee::s15::f_004103d8,
            throw_type_match,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x106, 0);
                c.bus.write32(DATA, 0);
                for i in 0..0x30u32 {
                    let b = (r.next_u32() & 0x3f) as u8 + 1;
                    c.bus.write8(DATA + i, b);
                    let b2 = if seed % 3 == 0 {
                        b
                    } else {
                        (r.next_u32() & 0x3f) as u8 + 1
                    };
                    c.bus.write8(DATA + 0x40 + i, b2);
                }
                c.bus.write8(DATA + 0x30, 0);
                c.bus.write8(DATA + 0x70, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x40) as u64;
                c.g[A2][0] = (DATA + 0x80) as u64;
            },
        );
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
        check(
            ssx3_ee::s15::f_004128a8,
            start_cp_handler,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x10c, 1);
                let p = DATA + 0x300;
                c.bus.write32(0x0045_3444, FAKE);
                // The fake returns `p`; make the dereference land in RAM.
                c.bus.write32(LOG_RESULTS, p);
            },
        );
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
        check(
            ssx3_ee::s15::f_00412af0,
            uncatch_exception,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x10f, 1);
                // Make the fake EH-info pointer land in RAM with a non-null
                // "active handler" so the noreturn terminate path isn't taken.
                c.bus.write32(LOG_RESULTS, DATA + 0x400);
                c.bus.write32(DATA + 0x408, 1);
            },
        );
    }

    // ------------------------------------------------------------- singletons

    pub(crate) fn singleton_test(gen: GuestFn, hand: GuestFn, obj: u32, salt: u64) {
        check(gen, hand, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, salt, 0);
            c.bus
                .write32(obj, if seed % 3 == 0 { r.next_u32() | 1 } else { 0 });
        });
    }

    pub(crate) fn singleton_test_entries(
        gen: GuestFn,
        hand: GuestFn,
        obj: u32,
        start: u32,
        salt: u64,
    ) {
        check(gen, hand, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, salt, 0);
            c.bus
                .write32(obj, if seed % 3 == 0 { r.next_u32() | 1 } else { 0 });
            c.pc = start;
        });
    }

    #[test]
    fn tf13bad_exception_matches() {
        singleton_test(
            ssx3_ee::s15::f_00412e80,
            tf13bad_exception,
            0x0053_ea10,
            0x120,
        );
    }

    #[test]
    fn tf9exception_matches() {
        singleton_test(ssx3_ee::s15::f_00412f18, tf9exception, 0x004a_5ba8, 0x121);
    }

    #[test]
    fn tf17_class_type_info_matches() {
        singleton_test(
            ssx3_ee::s15::f_00414f00,
            tf17_class_type_info,
            0x0053_ea60,
            0x122,
        );
    }

    #[test]
    fn tf14_si_type_info_matches() {
        singleton_test(
            ssx3_ee::s15::f_00414f90,
            tf14_si_type_info,
            0x0053_ea50,
            0x123,
        );
    }

    #[test]
    fn tf16_user_type_info_matches() {
        singleton_test_entries(
            ssx3_ee::s15::f_00415018,
            tf16_user_type_info,
            0x0053_ea40,
            0x0041_5018,
            0x124,
        );
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
        singleton_test(
            ssx3_ee::s15::f_00415ca8,
            tf17_array_type_info,
            0x0053_eaa0,
            0x128,
        );
    }

    #[test]
    fn tf16_ptmd_type_info_matches() {
        singleton_test(
            ssx3_ee::s15::f_00415d18,
            tf16_ptmd_type_info,
            0x0053_eac0,
            0x129,
        );
    }

    #[test]
    fn tf16_ptmf_type_info_matches() {
        singleton_test(
            ssx3_ee::s15::f_00415d88,
            tf16_ptmf_type_info,
            0x0053_ea90,
            0x12a,
        );
    }

    #[test]
    fn tf16_func_type_info_matches() {
        singleton_test(
            ssx3_ee::s15::f_00415df8,
            tf16_func_type_info,
            0x0053_ea70,
            0x12b,
        );
    }

    #[test]
    fn tf19_builtin_type_info_matches() {
        singleton_test(
            ssx3_ee::s15::f_00415e68,
            tf19_builtin_type_info,
            0x0053_ead0,
            0x12c,
        );
    }

    #[test]
    fn tf16_attr_type_info_matches() {
        singleton_test(
            ssx3_ee::s15::f_00415ed8,
            tf16_attr_type_info,
            0x0053_ea80,
            0x12d,
        );
    }

    #[test]
    fn tf19_pointer_type_info_matches() {
        singleton_test(
            ssx3_ee::s15::f_00415f48,
            tf19_pointer_type_info,
            0x0053_eab0,
            0x12e,
        );
    }

    // --------------------------------------------------------------- rtti

    #[test]
    fn rtti_si_matches() {
        check(ssx3_ee::s15::f_00414c00, rtti_si, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x130, 0);
            let obj = if seed % 3 == 0 {
                0
            } else {
                (DATA + (r.next_u32() & 0xff0)) as u64
            };
            c.g[A0][0] = obj;
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn rtti_user_matches() {
        check(ssx3_ee::s15::f_00414c20, rtti_user, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x131, 0);
            let obj = if seed % 3 == 0 {
                0
            } else {
                (DATA + (r.next_u32() & 0xff0)) as u64
            };
            c.g[A0][0] = obj;
            c.g[A1][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn user_type_info_dcast_matches() {
        check(
            ssx3_ee::s15::f_00414c40,
            user_type_info_dcast,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x132, 0);
                for i in 0..4 {
                    c.g[[A0, A1, A2, A3][i]][0] = (DATA + (r.next_u32() & 0xff0)) as u64;
                }
                c.pc = 0x0041_4c40;
            },
        );
    }

    // ----------------------------------------------------------- type_info dtor

    pub(crate) fn dtor_test(gen: GuestFn, hand: GuestFn, salt: u64) {
        plain(gen, hand, salt);
    }

    #[test]
    fn class_type_info_dtor_matches() {
        check(
            ssx3_ee::s15::f_00414f50,
            class_type_info_dtor,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x140, 1);
                c.g[A0][0] = (DATA + (r.next_u32() & 0xff0)) as u64;
            },
        );
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
        check(
            ssx3_ee::s15::f_004146d0,
            register_frame_info,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x150, 0);
                c.g[A0][0] = (DATA + (r.next_u32() & 0xff0)) as u64;
                c.g[A1][0] = (DATA + 0x1000) as u64;
            },
        );
    }

    #[test]
    fn register_frame_info_table_matches() {
        check(
            ssx3_ee::s15::f_00414728,
            register_frame_info_table,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x151, 0);
                c.g[A0][0] = (DATA + (r.next_u32() & 0xff0)) as u64;
                c.g[A1][0] = (DATA + 0x1000) as u64;
            },
        );
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
            stream_cookie(
                c,
                seed,
                31,
                if seed % 2 == 0 {
                    0
                } else {
                    3 + (seed as u16 % 8)
                },
            );
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
            c.bus.write32(
                0x0045_0000 + 0x4ca8,
                if seed % 2 == 0 { tid } else { r.next_u32() },
            );
            c.bus.write32(0x0045_0000 + 0x4cac, r.next_u32());
            c.bus.write32(0x0045_0000 + 0x5248, r.next_u32());
        };
        covered(ssx3_ee::s15::f_00418ca8, malloc_lock, SEEDS, &setup);
    }

    #[test]
    fn malloc_unlock_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 41, 1);
            c.bus.write32(
                0x0045_0000 + 0x4cac,
                if seed % 2 == 0 {
                    1
                } else {
                    2 + r.next_u32() % 5
                },
            );
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
            let kb = if seed % 5 == 0 {
                1 + r.next_u32() % 96
            } else {
                ka
            };
            c.bus.write32(a + 0x10, ka);
            c.bus.write32(b + 0x10, kb);
            let differ_at = if seed % 2 == 0 {
                u32::MAX
            } else {
                r.next_u32() % ka
            };
            for i in 0..96 {
                let x = r.next_u32();
                let y = if i == differ_at {
                    x ^ (1 << (r.next_u32() % 32))
                } else {
                    x
                };
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
            3 => {
                (0x8000_0000 | r.next_u32()) as u64
                    | ((r.next_u32() as u64) << 32 | 0x8000_0000_0000_0000)
            }
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
        check(ee::s15::f_00411538__generated, udivdi3, 500, &|c, seed| {
            div_setup(c, seed, 0x0049_97a0)
        });
    }

    #[test]
    fn umoddi3_matches() {
        check(ee::s15::f_0042dad8__generated, umoddi3, 500, &|c, seed| {
            div_setup(c, seed, 0x0049_af18)
        });
    }
    use super::*;

    // ------------------------------------------------------------ __unpack_d

    /// `__unpack_d` against the generated body: random doubles plus each
    /// class (zero exponent, normal, infinity, quiet and signalling NaN) with
    /// both signs; the destination is pre-filled with junk so unwritten
    /// fields show.
    #[test]
    fn unpack_d_matches() {
        use crate::difftest::{check, scramble, Rng, DATA};

        const SRC: u32 = DATA;
        const DST: u32 = DATA + 0x40;
        check(ee::s15::f_00411c38__generated, unpack_d, 300, &|c, seed| {
            let mut r = Rng(0x0411_c38d ^ seed as u64);
            scramble(c, &mut r);
            let sign = (r.next_u32() as u64 & 1) << 63;
            let frac = ((r.next_u32() as u64) << 32 | r.next_u32() as u64) & (u64::MAX >> 12);
            let rnd_exp = (r.next_u32() as u64) & 0x7ff;
            let bits = match seed % 8 {
                0 => sign | frac,                             // zero exponent (with fraction)
                1 => sign,                                    // +-0
                2 => sign | (0x7ff << 52),                    // +-inf
                3 => sign | (0x7ff << 52) | (1 << 51) | frac, // quiet NaN
                4 => sign | (0x7ff << 52) | ((frac & !(1 << 51)) | 1), // signalling NaN
                5 => sign | (0x3ff << 52) | frac,             // exponent 0
                _ => sign | (rnd_exp << 52) | frac,           // anything
            };
            for i in 0..8 {
                c.bus.write32(DST + i * 4, r.next_u32());
            }
            c.bus.write32(SRC, bits as u32);
            c.bus.write32(SRC + 4, (bits >> 32) as u32);
            c.g[A0][0] = sx(SRC);
            c.g[A1][0] = sx(DST);
        });
    }
    use super::*;
    use super::*;

    // -------------------------------------------------------------- __pack_d

    /// `__pack_d` against the generated body: every class (both NaNs, zero,
    /// normal, infinity, junk classes), zero and random fractions, exponents
    /// in range, at the edges (-1022, 1023, 1024), denormal (with shifts
    /// below and past 56 places) and huge; guard bytes forcing the
    /// round-to-even tie (both parities) and a carry past bit 60 that bumps
    /// the exponent.
    #[test]
    fn pack_d_matches() {
        use crate::difftest::{check, scramble, Rng, DATA};
        use crate::game::lib::gcc::fp_number as fp;

        const SRC: u32 = DATA;
        check(ee::s15::f_00411b08__generated, pack_d, 300, &|c, seed| {
            let mut r = Rng(0x0411_b08d ^ seed as u64);
            scramble(c, &mut r);
            let rnd64 = |r: &mut Rng| (r.next_u32() as u64) << 32 | r.next_u32() as u64;
            let class: u32 = match seed % 10 {
                0 => 0,            // signalling NaN
                1 => 1,            // quiet NaN
                2 => 2,            // zero
                3 => 4,            // infinity
                4 => r.next_u32(), // junk class (negative / large)
                _ => 3,            // normal number
            };
            let sign = if seed % 7 == 0 {
                r.next_u32()
            } else {
                r.next_u32() & 1
            };
            let exp: i32 = match (seed / 10) % 9 {
                0 => -1022,
                1 => 1023,
                2 => 1024,
                3 => -1022 - 1 - (r.next_u32() % 56) as i32, // shallow denormal
                4 => -1022 - 56 - (r.next_u32() % 8) as i32, // around the 57 cut
                5 => r.next_u32() as i32,                    // anything
                _ => (r.next_u32() % 2046) as i32 - 1022,    // in range
            };
            // mantissa with the implicit one at bit 60 and 8 guard bits
            let mut frac = (rnd64(&mut r) & (u64::MAX >> 4)) | (1 << 60);
            match (seed / 3) % 8 {
                0 => frac = 0,
                1 => frac = (frac & !0xff) | 0x80, // tie, odd/even random
                2 => frac = (frac & !0x1ff) | 0x80, // tie, even
                3 => frac = (frac & !0x1ff) | 0x180, // tie, odd
                4 => frac = ((u64::MAX >> 3) & !0x7f) | 0x81, // rounds to bit 61
                5 => frac = rnd64(&mut r),         // anything
                _ => {}
            }
            c.bus.write32(SRC + fp::CLASS, class);
            c.bus.write32(SRC + fp::SIGN, sign);
            c.bus.write32(SRC + fp::NORMAL_EXP, exp as u32);
            c.bus.write32(SRC + 0x0c, r.next_u32());
            c.bus.write32(SRC + fp::FRACTION, frac as u32);
            c.bus.write32(SRC + fp::FRACTION + 4, (frac >> 32) as u32);
            c.g[A0][0] = sx(SRC);
            c.pc = 0x0041_1b08;
        });
    }

    // -------------------------------------------------------------- __muldf3

    /// `__muldf3` against the generated body (real `__unpack_d`,
    /// `__muldi3`, `__pack_d`): every pair of operand classes (signalling
    /// and quiet NaN, zero, infinity, number, with random signs), so each
    /// special result (a, b, the canonical NaN) is taken; numbers with
    /// random and edge exponents (overflow and underflow in `__pack_d`),
    /// and fractions built to hit each rounding case: an exact guard tie
    /// with the result's low bit set, a tie with the low bit clear and
    /// bits left over (rounds up) or none left (stays).
    #[test]
    fn muldf3_matches() {
        use crate::difftest::{check, scramble, Rng};

        const EXP_INF: u64 = 0x7ff;
        const QUIET: u64 = 1 << 51;
        const FRAC: u64 = u64::MAX >> 12;

        fn double(sign: u64, exp: u64, frac: u64) -> u64 {
            (sign << 63) | ((exp & EXP_INF) << 52) | (frac & FRAC)
        }

        check(ee::s15::f_00411fd8__generated, muldf3, 300, &|c, seed| {
            let mut r = Rng(0x0411_fd8d ^ seed as u64);
            scramble(c, &mut r);
            let rnd64 = |r: &mut Rng| (r.next_u32() as u64) << 32 | r.next_u32() as u64;
            let exp = |r: &mut Rng| match r.next_u32() % 6 {
                0 => 1,     // smallest normal
                1 => 0x7fe, // largest
                2 => 0x3ff, // 1.x
                _ => 1 + (r.next_u32() as u64 % 0x7fe),
            };
            // class: 0 signalling NaN, 1 quiet NaN, 2 zero, 3 infinity, 4 number
            let operand = |r: &mut Rng, class: u32| {
                let sign = r.next_u32() as u64 & 1;
                let frac = rnd64(r);
                match class {
                    0 => double(sign, EXP_INF, (frac & !QUIET) | 1),
                    1 => double(sign, EXP_INF, frac | QUIET),
                    2 => double(sign, 0, frac), // no denormals: zero
                    3 => double(sign, EXP_INF, 0),
                    _ => {
                        let e = exp(r);
                        double(sign, e, frac)
                    }
                }
            };
            let (a, b) = if seed < 125 {
                let a = operand(&mut r, seed % 5);
                let b = operand(&mut r, (seed / 5) % 5);
                (a, b)
            } else {
                // Two numbers. Fraction a = 2, fraction b = 2^50 + k: the
                // product's 8 guard bits are exactly 0x80; k picks the
                // result's low bit (k odd) and what is left below the guard
                // bits (2k).
                let sign_a = r.next_u32() as u64 & 1;
                let sign_b = r.next_u32() as u64 & 1;
                let (ea, eb) = (exp(&mut r), exp(&mut r));
                match seed % 5 {
                    0 => (double(sign_a, ea, 2), double(sign_b, eb, 1 << 50)), // tie, even, exact
                    1 => (double(sign_a, ea, 2), double(sign_b, eb, (1 << 50) | 1)), // tie, odd
                    2 => (double(sign_a, ea, 2), double(sign_b, eb, (1 << 50) | 2)), // tie, even, rest
                    3 => (
                        double(sign_a, ea, 2),
                        double(sign_b, eb, (1 << 50) | (rnd64(&mut r) & ((1 << 43) - 1))),
                    ),
                    _ => (operand(&mut r, 4), operand(&mut r, 4)),
                }
            };
            c.g[A0][0] = a;
            c.g[A1][0] = b;
            c.pc = 0x0041_1fd8;
        });
    }
    use super::*;
    use crate::difftest::put_vec;
    pub(crate) use crate::game::camera::camera::tests::f;

    /// __kernel_tanf: x in f12 from tiny (zero, 1e-10, values that do not
    /// truncate to 0 cannot be tiny, so also NaN/inf edges), the middle range
    /// and the reduced range (|x| >= 0.6744, both signs), y in f13, and iy
    /// 1, -1 or other; the real coefficient table is written.
    #[test]
    fn kernel_tanf_matches() {
        check(
            ee::s15::f_0040f5a8__generated,
            kernel_tanf,
            300,
            &|c, seed| {
                let mut r = Rng(0x40f5_a8 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.pc = 0x0040_f5a8;
                let coeffs = [
                    3.333_333_4e-1f32,
                    1.333_340_8e-1,
                    5.396_875_4e-2,
                    2.186_948_8e-2,
                    8.863_239_5e-3,
                    3.592_079_3e-3,
                    1.456_209_9e-3,
                    5.880_412_3e-4,
                    2.464_631_3e-4,
                    7.817_944_4e-5,
                    7.140_725_2e-5,
                    -1.855_863_9e-5,
                    2.590_730_7e-5,
                ];
                for (i, k) in coeffs.iter().enumerate() {
                    c.bus.write32(0x0049_9648 + 4 * i as u32, k.to_bits());
                }
                let x = match seed % 8 {
                    0 => 0.0f32.to_bits() | (r.next_u32() & 0x8000_0000),
                    1 => (1.0e-10f32 * if r.next_u32() % 2 == 0 { 1.0 } else { -1.0 }).to_bits(),
                    2 => r.edge_f32(),
                    3 | 4 => r.f32(0.6),
                    _ => {
                        let v = 0.68 + (r.next_u32() % 1000) as f32 * 1.0e-4;
                        (if r.next_u32() % 2 == 0 { v } else { -v }).to_bits()
                    }
                };
                c.f[12] = x;
                c.f[13] = r.f32(1.0e-8);
                c.g[A0][0] = match r.next_u32() % 3 {
                    0 => 1,
                    1 => u64::MAX,
                    _ => r.next_u32() as u64,
                };
            },
        );
    }
    use super::*;
    use super::*;
    use crate::difftest::elf_image;
    use crate::difftest::{check_lohi, check_paths};

    /// Two unpacked doubles covering every class pair (NaN, zero, normal,
    /// infinity, and an out-of-range negative class), both signs, and
    /// equal/greater/smaller exponents and fractions.
    #[test]
    fn fpcmp_parts_d_matches() {
        use crate::difftest::{scramble, Rng};
        check(
            ee::s15::f_004123e8__generated,
            fpcmp_parts_d,
            400,
            &|c, seed| {
                let mut r = Rng(0x4123_e8 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                let (a, b) = (DATA, DATA + 0x40);
                let classes = [0u32, 1, 2, 3, 3, 3, 4, 0xffff_fff0];
                c.bus.write32(a + FP_CLASS, classes[seed as usize % 8]);
                c.bus
                    .write32(b + FP_CLASS, classes[(seed as usize / 8) % 8]);
                let pick = |r: &mut Rng, n: u32| r.next_u32() % n;
                let sa = pick(&mut r, 2);
                let sb = if pick(&mut r, 2) == 0 {
                    sa
                } else {
                    pick(&mut r, 2)
                };
                c.bus.write32(a + FP_SIGN, sa);
                c.bus.write32(b + FP_SIGN, sb);
                let ea = pick(&mut r, 5).wrapping_sub(2);
                let eb = if pick(&mut r, 2) == 0 {
                    ea
                } else {
                    pick(&mut r, 5).wrapping_sub(2)
                };
                c.bus.write32(a + FP_EXP, ea);
                c.bus.write32(b + FP_EXP, eb);
                let fa = ((r.next_u32() as u64) << 32 | r.next_u32() as u64) & 0xf;
                let fb = if pick(&mut r, 3) == 0 {
                    fa
                } else {
                    fa ^ (pick(&mut r, 4) as u64)
                };
                c.bus.write32(a + FP_FRAC, fa as u32);
                c.bus.write32(
                    a + FP_FRAC + 4,
                    (fa >> 32) as u32 | 0x8000_0000 * pick(&mut r, 2),
                );
                c.bus.write32(b + FP_FRAC, fb as u32);
                c.bus.write32(b + FP_FRAC + 4, (fb >> 32) as u32);
                c.g[A0][0] = sx(a);
                c.g[A1][0] = sx(b);
                c.pc = 0x0041_23e8;
            },
        );
    }

    /// __floatsidf: zero, positive, negative, INT_MIN, small and large
    /// magnitudes (long and short normalisation), with the real __pack_d.
    #[test]
    fn floatsidf_matches() {
        check(
            ee::s15::f_00412550__generated,
            floatsidf,
            300,
            &|c, seed| {
                let mut r = Rng(0x4125_5000 ^ seed as u64);
                scramble(c, &mut r);
                let v = match seed % 6 {
                    0 => 0,
                    1 => i32::MIN,
                    2 => (r.next_u32() % 16) as i32 - 8,
                    3 => r.next_u32() as i32,
                    4 => i32::MAX - (r.next_u32() % 4) as i32,
                    _ => ((r.next_u32() >> (r.next_u32() % 31)) as i32)
                        .wrapping_mul(if r.next_u32() & 1 == 0 { 1 } else { -1 }),
                };
                c.g[A0][0] = sx(v as u32);
            },
        );
    }
    use super::*;
    use super::*;
    use super::*;
    use crate::difftest::events::driven;
    use crate::difftest::fakes::ptr;
    use crate::difftest::{fresh, require_paths};
    use ssx3_ee_rt::Cpu;

    /// __fixunsdfsi: zero, NaN, negative, infinity, |x| < 1, x >= 2^32 and
    /// in-range values (with fractions) through the real `__unpack_d`.
    #[test]
    fn fixunsdfsi_matches() {
        use crate::difftest::scramble;
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = Rng(0x4126_a0 ^ (seed as u64) << 20);
            scramble(c, &mut r);
            c.pc = 0x0041_26a0;
            let sign = (r.next_u32() as u64 & 1) << 63;
            let frac = (r.next_u32() as u64) << 20 | r.next_u32() as u64 & 0xf_ffff;
            let x: u64 = match seed % 8 {
                0 => sign,
                1 => 0x7ff0_0000_0000_0000 | frac | 1,
                2 => 0x7ff0_0000_0000_0000 | sign,
                3 => (1u64 << 63) | ((1023 + r.next_u32() % 40) as u64) << 52 | frac,
                4 => ((1000 + r.next_u32() % 23) as u64) << 52 | frac,
                5 => ((1023 + 32 + r.next_u32() % 100) as u64) << 52 | frac,
                _ => ((1023 + r.next_u32() % 32) as u64) << 52 | frac,
            };
            c.g[A0][0] = x;
        };
        check(ee::s15::f_004126a0__generated, fixunsdfsi, 300, &setup);
    }

    /// __ieee754_expf: NaN/inf bit patterns, overflow, underflow, the
    /// three reduction ranges (k from the table, k = ±1, k = 0), tiny x,
    /// and results scaled through 2^-100 (k < -125).
    #[test]
    fn ieee754_expf_matches() {
        use crate::difftest::scramble;
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = Rng(0x40e0_48 ^ (seed as u64) << 24);
            scramble(c, &mut r);
            elf_image(c);
            c.pc = 0x0040_e048;
            let sign = (r.next_u32() & 1) << 31;
            let u = r.next_u32() as f32 / u32::MAX as f32;
            let x: u32 = match seed % 12 {
                0 => sign | 0x7f80_0000 | (1 + (r.next_u32() & 0x7f_fffe)),
                1 => sign | 0x7f80_0000,
                2 => (89.0 + u * 1e20).to_bits(),
                3 => (-(104.0 + u * 1e20)).to_bits(),
                4 => (-(87.0 + u * 16.9)).to_bits(),
                5 => sign | (1.04 + u * 87.0).to_bits(),
                6 => sign | (0.35 + u * 0.69).to_bits(),
                7 => sign | (u * 0.346).to_bits(),
                8 => sign | (u * 3.7e-9).to_bits(),
                9 => sign | (r.next_u32() & 0x317f_ffff),
                10 => (88.0 + u * 0.72).to_bits(),
                _ => r.edge_f32(),
            };
            c.f[12] = x;
        };
        check(ee::s15::f_0040e048__generated, ieee754_expf, 300, &setup);
    }

    // ---------------------------------------------------------- __ieee754_logf

    /// __ieee754_logf on every input class: +-0 (the -huge constant written
    /// at 0x499224), negatives, subnormals, exponent 255, exactly 1 and
    /// powers of two (f == 0, k == 0 or not), mantissas within 2^-20 of 1
    /// (the tiny polynomial), mantissas just above/below the series' split
    /// (k == 0 or not), and random positives.
    #[test]
    fn ieee754_logf_matches() {
        check(
            ee::s15::f_0040e550__generated,
            ieee754_logf,
            400,
            &|c, seed| {
                let mut r = Rng(0x40_e550 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                c.pc = 0x0040_e550;
                c.bus.write32(0x0049_9224, 0xff7f_ffff);
                let exp = |r: &mut Rng| (1 + r.next_u32() % 253) << 23;
                let x = match seed % 12 {
                    0 => (r.next_u32() & 1) << 31,
                    1 => 0x8000_0000 | r.next_u32(),
                    2 => r.next_u32() & 0x007f_ffff,
                    3 => 0x7f80_0000 | (r.next_u32() & 0x807f_ffff),
                    4 => 0x3f80_0000,
                    5 => exp(&mut r),
                    6 => 0x3f80_0000 + r.next_u32() % 16,
                    7 => exp(&mut r) | (r.next_u32() % 16),
                    8 => exp(&mut r) | (0x7f_fff0 + r.next_u32() % 16),
                    9 => (0x7e_u32 << 23) | (0x7f_fff0 + r.next_u32() % 16),
                    10 => 0x3f80_0000 + (r.next_u32() % 0x8_0000) | ((r.next_u32() & 1) << 22),
                    _ => r.next_u32() & 0x7fff_ffff,
                };
                c.f[12] = x;
            },
        );
    }

    /// `__unpack_f` on zeros, normal numbers, infinities and both NaN kinds
    /// of either sign.
    #[test]
    fn unpack_f_matches() {
        check(ee::s15::f_00413068__generated, unpack_f, 300, &|c, seed| {
            let mut r = Rng(0x41_3068 ^ (seed as u64) << 12);
            crate::difftest::scramble(c, &mut r);
            let sign = r.next_u32() & 0x8000_0000;
            let frac = match seed % 3 {
                0 => 0,
                1 => r.next_u32() & 0x10_0000,
                _ => r.next_u32() & 0x7f_ffff,
            };
            let exp = match seed % 5 {
                0 => 0,
                1 => 0xff,
                _ => r.next_u32() & 0xff,
            };
            c.bus.write32(DATA, sign | exp << 23 | frac);
            crate::difftest::fill(c, &mut r, DATA + 0x10, 4);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x10);
            c.pc = 0x0041_3068;
        });
    }
    use super::*;
    use super::*;
    use super::*;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::oc238_uf;

    /// The __fixunssfsi argument: edge cases, NaN / infinity bit patterns,
    /// negatives, values below one, around 2^30 / 2^31 / 2^32 and random
    /// magnitudes. `small`: mostly in (0, 1) (the event-checked branch).
    /// Some seeds enter at the "answer 0" resume point inside the frame.
    pub(crate) fn oc238_fixunssfsi_setup(c: &mut Cpu, seed: u32, small: bool) -> u32 {
        let mut r = machine(c, seed, 0x41_39f8, 0);
        let bits = if small && seed % 4 != 0 {
            oc238_uf(&mut r, 0.0, 0.999).to_bits()
        } else {
            match seed % 9 {
                0 => r.edge_f32(),
                1 => 0x7f80_0000 | (r.next_u32() & 0x807f_ffff),
                2 => oc238_uf(&mut r, -1e6, 0.0).to_bits(),
                3 => oc238_uf(&mut r, 0.0, 1.0).to_bits(),
                4 => (0x4e80_0000u32 - 2 + r.next_u32() % 5) as u32,
                5 => (0x4f00_0000u32 - 2 + r.next_u32() % 5) as u32,
                6 => (0x4f80_0000u32 - 2 + r.next_u32() % 5) as u32,
                7 => (0x3f80_0000 + (r.next_u32() % 32 << 23)) | (r.next_u32() & 0x7f_ffff),
                _ => r.next_u32(),
            }
        };
        c.f[12] = bits;
        if seed % 10 == 7 {
            c.g[SP][0] = (crate::difftest::SP - 0x30) as u64;
            fill(c, &mut r, crate::difftest::SP - 0x30, 12);
            // The saved ra (sp+0x20) returns to the test's sentinel.
            c.bus
                .write64(crate::difftest::SP - 0x10, crate::difftest::RA as u64);
            0x0041_3a28
        } else {
            0x0041_39f8
        }
    }

    /// __fixunssfsi: every class and exponent range with the real
    /// __unpack_f; the resume entry; events at the below-one branch.
    #[test]
    fn fixunssfsi_matches() {
        check(
            ee::s15::f_004139f8__generated,
            fixunssfsi,
            400,
            &|c, seed| {
                let entry = oc238_fixunssfsi_setup(c, seed, false);
                c.pc = entry;
            },
        );
        crate::difftest::events::check_events(
            oc238_gen_fixunssfsi,
            oc238_hand_fixunssfsi,
            120,
            &|c, seed| {
                let entry = oc238_fixunssfsi_setup(c, seed, true);
                let mut r = Rng(0x41_39f8 ^ seed as u64);
                let mode = crate::difftest::events::schedule(c, &mut r, seed, entry);
                c.next_event = c.cycles + 4;
                mode
            },
        );
    }

    // -------------------------------------------------------------- __subdf3

    /// `__subdf3`: random doubles and edge cases (zeros of both signs,
    /// infinities, NaNs, denormals, equal operands) through the real
    /// soft-float helpers.
    #[test]
    fn subdf3_matches() {
        use crate::difftest::{check, scramble, Rng};
        let edge = |r: &mut Rng| -> u64 {
            let sign = ((r.next_u32() & 1) as u64) << 63;
            let mant = ((r.next_u32() as u64) << 32 | r.next_u32() as u64) & ((1u64 << 52) - 1);
            sign | match r.next_u32() % 8 {
                0 => 0,
                1 => mant,
                2 => 0x7ff0_0000_0000_0000,
                3 => 0x7ff0_0000_0000_0000 | mant | 1,
                4 => 1.0f64.to_bits(),
                _ => (((r.next_u32() % 200 + 923) as u64) << 52) | mant,
            }
        };
        check(ee::s15::f_00411f70__generated, subdf3, 300, &|c, seed| {
            let mut r = Rng(0x0041_1f70 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.pc = 0x0041_1f70;
            let a = edge(&mut r);
            let b = match seed % 5 {
                0 => a,
                1 => a ^ (1 << 63),
                _ => edge(&mut r),
            };
            c.g[A0][0] = a;
            c.g[A1][0] = b;
        });
    }

    /// `__floatdidf`: 64-bit values with every combination of high-word
    /// sign and low-word top bit, zero, small numbers, the extremes; the
    /// soft-float callees run for real.
    #[test]
    fn floatdidf_matches() {
        crate::difftest::check(
            ee::s15::f_00411438__generated,
            floatdidf,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x411438, 0);
                c.pc = 0x0041_1438;
                let hi = r.next_u32() as u64;
                let lo = r.next_u32() as u64;
                let x = match seed % 8 {
                    0 => 0,
                    1 => lo & 0x7fff_ffff,
                    2 => (lo | 0x8000_0000) & 0xffff_ffff,
                    3 => (-(lo as i64 & 0xffff)) as u64,
                    4 => i64::MIN as u64,
                    5 => i64::MAX as u64,
                    _ => hi << 32 | lo,
                };
                c.g[A0][0] = x;
            },
        );
    }
    use super::*;

    // -------------------------------------------------------------- __adddf3

    /// `__adddf3`: random doubles and edge cases (zeros of both signs,
    /// infinities, NaNs, denormals, opposite operands) through the real
    /// soft-float helpers.
    #[test]
    fn adddf3_matches() {
        use crate::difftest::{check, scramble, Rng};
        let edge = |r: &mut Rng| -> u64 {
            let sign = ((r.next_u32() & 1) as u64) << 63;
            let mant = ((r.next_u32() as u64) << 32 | r.next_u32() as u64) & ((1u64 << 52) - 1);
            sign | match r.next_u32() % 8 {
                0 => 0,
                1 => mant,
                2 => 0x7ff0_0000_0000_0000,
                3 => 0x7ff0_0000_0000_0000 | mant | 1,
                4 => 1.0f64.to_bits(),
                _ => (((r.next_u32() % 200 + 923) as u64) << 52) | mant,
            }
        };
        check(ee::s15::f_00411f18__generated, adddf3, 300, &|c, seed| {
            let mut r = Rng(0x0041_1f18 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.pc = 0x0041_1f18;
            let a = edge(&mut r);
            let b = match seed % 5 {
                0 => a,
                1 => a ^ (1 << 63),
                _ => edge(&mut r),
            };
            c.g[A0][0] = a;
            c.g[A1][0] = b;
        });
    }
    use super::*;
    use crate::difftest::fakes::vtable;
    pub(crate) use crate::game::object::debouncenode::crowd_man3d_save_state;
    use crate::game::scripter::ssxscriptengine::tests::{
        alloc as uv_scroll_alloc, heap_init as uv_scroll_heap_init,
    };
    pub(crate) use crate::game::ui::focusman::focus_man_save_state;

    pub(crate) const FOCUS: u32 = DATA;

    pub(crate) const FOCUS_STREAM: u32 = DATA + 0x400;

    pub(crate) const FOCUS_STREAM_VT: u32 = DATA + 0x500;

    /// A FocusMan with 0..4 entries (sometimes a negative count) and 0..4
    /// values, and a stream whose Write (+0xc) and WriteInt (+0x2c) are fakes.
    pub(crate) fn focus_man_setup(c: &mut Cpu, seed: u32, calls: u32) -> Rng {
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
        c.bus
            .write32(FOCUS + crate::game::ui::focusman::FOCUS_COUNT, count);
        c.bus
            .write32(FOCUS + crate::game::ui::focusman::FOCUS_VALUE_COUNT, values);
        for i in 0..4 {
            c.bus.write32(
                FOCUS + crate::game::ui::focusman::FOCUS_VALUES + 4 * i,
                r.next_u32(),
            );
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

    driven!(
        focus_man_save_state_gen_driven,
        ee::s00::f_00103480__generated
    );

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

    /// A double for the soft-float conversions: zero, NaN, infinity, |x| < 1,
    /// |x| >= 2^31 (around the 2^31 edge too) and in-range values, either
    /// sign.
    pub(crate) fn fixdfsi_operand(r: &mut Rng, seed: u32) -> u64 {
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
    pub(crate) fn fixdfsi_setup(c: &mut Cpu, seed: u32, small: bool) -> u32 {
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
    pub(crate) fn divdf3_operand(r: &mut Rng) -> u64 {
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
    pub(crate) fn divdf3_setup(c: &mut Cpu, seed: u32, plain: bool) -> u32 {
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

    pub(crate) const CROWD: u32 = DATA;

    pub(crate) const CROWD_STREAM: u32 = DATA + 0x1000;

    pub(crate) const CROWD_STREAM_VT: u32 = DATA + 0x1100;

    /// Five crowd groups, used (random id) or unused (-1), and a stream
    /// whose Write (+0xc) is a fake.
    pub(crate) fn crowd_man3d_save_setup(c: &mut Cpu, seed: u32, calls: u32) -> Rng {
        let mut r = machine(c, seed, 0x0034_41a8, calls);
        c.pc = 0x0034_41a8;
        for g in 0..5 {
            let id = if (seed >> g) & 1 == 0 || seed % 5 == 0 {
                0xffff_ffff
            } else {
                r.next_u32() % 64
            };
            c.bus.write32(
                CROWD
                    + g * crate::game::object::debouncenode::CROWD_GROUP_SIZE
                    + crate::game::object::debouncenode::CROWD_GROUP_ID,
                id,
            );
        }
        c.bus.write32(CROWD_STREAM, CROWD_STREAM_VT);
        c.bus
            .write16(CROWD_STREAM_VT + 0x8, (r.next_u32() % 0x40) as u16 & !3);
        c.bus.write32(CROWD_STREAM_VT + 0xc, FAKE);
        c.g[A0][0] = sx(CROWD);
        c.g[A1][0] = sx(CROWD_STREAM);
        r
    }

    driven!(
        crowd_man3d_save_state_gen_driven,
        ee::s11::f_003441a8__generated
    );

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
}
