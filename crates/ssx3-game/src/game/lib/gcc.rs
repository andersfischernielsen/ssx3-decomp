//! `src/lib/gcc.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::addu;
use crate::abi::r32;
use crate::abi::reg;
use crate::abi::ret;
use crate::abi::set_reg;
use crate::call;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::misc::lit;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
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
    (0x0040_db58, ieee754_log, "__ieee754_log"),
    (0x0040_e300, ieee754_fmodf, "__ieee754_fmodf"),
    (0x0041_0408, empty, "__empty"),
    (0x0041_0570, sjthrow, "__sjthrow"),
    (0x0041_06e8, sjpopnthrow, "__sjpopnthrow"),
    (0x0041_0800, eh_rtime_match, "__eh_rtime_match"),
    (0x0041_0848, get_eh_table_version, "__get_eh_table_version"),
    (
        0x0041_0850,
        get_eh_table_language,
        "__get_eh_table_language",
    ),
    (0x0041_0c10, unwinding_cleanup, "__unwinding_cleanup"),
    (0x0041_0f48, throw_, "__throw"),
    (0x0041_1128, rethrow, "__rethrow"),
    (0x0041_1530, pure_virtual, "__pure_virtual"),
    (0x0041_2928, cplus_type_matcher, "__cplus_type_matcher"),
    (0x0041_2998, cp_push_exception, "__cp_push_exception"),
    (0x0041_2a28, cp_pop_exception, "__cp_pop_exception"),
    (0x0041_2b20, check_eh_spec, "__check_eh_spec"),
    (0x0041_2cd8, throw_bad_cast, "__throw_bad_cast"),
    (0x0041_2d58, throw_bad_typeid, "__throw_bad_typeid"),
    (0x0041_2f58, pack_f, "__pack_f"),
    (0x0041_3330, addsf3, "__addsf3"),
    (0x0041_3388, subsf3, "__subsf3"),
    (0x0041_33f0, mulsf3, "__mulsf3"),
    (0x0041_35e8, divsf3, "__divsf3"),
    (0x0041_3748, fpcmp_parts_f, "__fpcmp_parts_f"),
    (0x0041_38b0, floatsisf, "__floatsisf"),
    (0x0041_3968, fixsfsi, "__fixsfsi"),
    (
        0x0041_4780,
        deregister_frame_info,
        "__deregister_frame_info",
    ),
    (0x0041_4818, frame_state_for, "__frame_state_for"),
    (0x0041_4bd8, rtti_class, "__rtti_class"),
    (0x0041_4c70, si_type_info_dcast, "__si_type_info_dcast"),
    (
        0x0041_4d08,
        class_type_info_dcast,
        "__class_type_info_dcast",
    ),
    (0x0041_4ee0, class_type_info_ctor, "__class_type_info_ctor"),
    (0x0041_4f70, si_type_info_ctor, "__si_type_info_ctor"),
    (0x0041_5000, user_type_info_ctor, "__user_type_info_ctor"),
    (
        0x0041_5260,
        throw_type_match_rtti_2,
        "__throw_type_match_rtti_2",
    ),
    (0x0041_5728, is_pointer, "__is_pointer"),
    (0x0041_5778, rtti_ptr, "__rtti_ptr"),
    (0x0041_5798, rtti_attr, "__rtti_attr"),
    (0x0041_57c0, rtti_func, "__rtti_func"),
    (0x0041_57e0, rtti_ptmf, "__rtti_ptmf"),
    (0x0041_5800, rtti_ptmd, "__rtti_ptmd"),
    (0x0041_5820, rtti_array, "__rtti_array"),
    (0x0041_5840, dynamic_cast, "__dynamic_cast"),
    (0x0041_5928, tfx, "__tfx"),
    (0x0041_5960, tfl, "__tfl"),
    (0x0041_5998, tfi, "__tfi"),
    (0x0041_59d0, tfs, "__tfs"),
    (0x0041_5a08, tfb, "__tfb"),
    (0x0041_5a40, tfc, "__tfc"),
    (0x0041_5a78, tfw, "__tfw"),
    (0x0041_5ab0, tfr, "__tfr"),
    (0x0041_5ae8, tfd, "__tfd"),
    (0x0041_5b20, tff, "__tff"),
    (0x0041_5b58, tfui, "__tfUi"),
    (0x0041_5b90, tful, "__tfUl"),
    (0x0041_5bc8, tfux, "__tfUx"),
    (0x0041_5c00, tfus, "__tfUs"),
    (0x0041_5c38, tfuc, "__tfUc"),
    (0x0041_5c70, tfsc, "__tfSc"),
    (0x0041_ef50, ieee754_fmod, "__ieee754_fmod"),
    (0x0041_f9d8, ieee754_log10, "__ieee754_log10"),
    (0x0041_0410, new_eh_context, "new_eh_context"),
    (
        0x0041_0858,
        old_find_exception_handler,
        "old_find_exception_handler",
    ),
    (
        0x0041_0930,
        find_exception_handler,
        "find_exception_handler",
    ),
    (0x0041_0a78, get_reg_addr, "get_reg_addr"),
    (0x0041_0b00, copy_reg, "copy_reg"),
    (0x0041_0b80, next_stack_level, "next_stack_level"),
    (0x0041_0c18, throw_helper, "throw_helper"),
    (0x0041_04b8, eh_context_initialize, "eh_context_initialize"),
    (0x0041_04e0, eh_context_static, "eh_context_static"),
    (0x0041_2800, terminate_cpp, "terminate"),
    (0x0041_2838, set_unexpected, "set_unexpected"),
    (0x0041_2dd8, uncaught_exception, "uncaught_exception"),
    (0x0041_2e10, exception_what, "exception_what"),
    (0x0041_2e38, bad_exception_dtor, "bad_exception_dtor"),
    (0x0041_2e68, bad_exception_ctor, "bad_exception_ctor"),
    (0x0041_2ed0, exception_dtor, "exception_dtor"),
    (0x0041_2f00, exception_ctor, "exception_ctor"),
    (0x0041_3b38, decode_uleb128, "decode_uleb128"),
    (0x0041_3b80, decode_sleb128, "decode_sleb128"),
    (0x0041_4b68, type_info_dtor, "type_info_dtor"),
    (0x0041_4b98, type_info_eq, "type_info_eq"),
    (0x0041_51b8, type_info_ne, "type_info_ne"),
    (0x0041_3f38, count_fdes, "count_fdes"),
    (0x0041_4008, frame_init, "frame_init"),
    (0x0041_3bd0, fde_merge, "fde_merge"),
    (0x0041_3cd0, end_fde_sort, "end_fde_sort"),
    (0x0041_51d8, type_info_name, "type_info_name"),
    (0x0041_51e0, type_info_ctor, "type_info_ctor"),
    (0x0041_5238, type_info_before, "type_info_before"),
    (0x0041_2818, set_terminate, "set_terminate"),
    (0x0041_4148, find_fde, "find_fde"),
    (0x0041_49f0, fde_split, "fde_split"),
    (0x0041_5088, bad_typeid_dtor, "bad_typeid_dtor"),
    (0x0041_50b8, bad_typeid_ctor, "bad_typeid_ctor"),
    (0x0041_5120, bad_cast_dtor, "bad_cast_dtor"),
    (0x0041_5150, bad_cast_ctor, "bad_cast_ctor"),
    (
        0x0042_e200,
        tf23ps2_sony_codec_internal,
        "__tf23PS2_SONY_CODEC_INTERNAL",
    ),
    (0x0042_e2b0, tfq24rcmp5codec, "__tfQ24RCMP5CODEC"),
    (0x0042_e458, tfq24rcmp7decoder, "__tfQ24RCMP7DECODER"),
    (
        0x0042_e4c8,
        tfq24rcmp11rcmp_system,
        "__tfQ24RCMP11RCMP_SYSTEM",
    ),
    (0x0040_fcb0, divdi3, "__divdi3"),
    (0x0041_f3a8, ieee754_log_2, "__ieee754_log_2"),
    (0x0042_d470, moddi3, "__moddi3"),
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

// ---------------------------------------------------------------------------
// __ieee754_log (0x40db58, fdlibm e_log.c on the soft-float double ABI)

pub(crate) const IEEE_LOG_ADDDF3: Callee = Callee {
    addr: 0x0041_1f18,
    f: ee::s15::__adddf3,
};

pub(crate) const IEEE_LOG_SUBDF3: Callee = Callee {
    addr: 0x0041_1f70,
    f: ee::s15::__subdf3,
};

pub(crate) const IEEE_LOG_MULDF3: Callee = Callee {
    addr: 0x0041_1fd8,
    f: ee::s15::__muldf3,
};

pub(crate) const IEEE_LOG_DIVDF3: Callee = Callee {
    addr: 0x0041_2280,
    f: ee::s15::__divdf3,
};

pub(crate) const IEEE_LOG_CMPDF2: Callee = Callee {
    addr: 0x0041_2500,
    f: ee::s15::__cmpdf2,
};

pub(crate) const IEEE_LOG_FLOATSIDF: Callee = Callee {
    addr: 0x0041_2550,
    f: ee::s15::__floatsidf,
};

/// The constant pool of e_log.c (doubles at `0x4a0000 - off`): -two54/zero,
/// ln2_hi, ln2_lo, 1/3, ln2_hi, ln2_lo, Lg7, Lg5, Lg3, Lg1, Lg6, Lg4, Lg2,
/// then ln2_hi/ln2_lo for the two `k != 0` tails.
pub(crate) const IEEE_LOG_POOL: u32 = 0x004a_0000;

/// `a1 = pool constant` (`lui at, 0x4a; ld a1, -off(at)`).
pub(crate) fn ieee_log_k(c: &mut Cpu, off: u32) {
    c.g[AT][0] = IEEE_LOG_POOL as u64;
    c.g[A1][0] = c.ld(IEEE_LOG_POOL - off);
}

/// Where a path of `__ieee754_log` joins the shared tail.
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub(crate) enum IeeeLogTail {
    /// `a1 = v0`, then `a0 - a1`.
    SubResult,
    /// `v0 = a0 - a1`.
    Sub,
    /// Return v0 (reload ra).
    Ret,
    /// Return v0 (ra already reloaded).
    Epilogue,
}

/// `double __ieee754_log(double x)` (0x40db58; x and the result as 64-bit
/// integers in a0 / v0): fdlibm's natural logarithm on the soft-float
/// helpers. x = 2^k * (1 + f) with sqrt(2)/2 < 1 + f < sqrt(2); small f
/// take the short series, the rest `s = f / (2 + f)` with the Lg1..Lg7
/// polynomial. log(+-0) = -inf, log(x < 0) = NaN, log(inf/NaN) = x + x.
pub fn ieee754_log(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0xa0);
        c.g[A2][0] = c.g[A0][0];
        let stk = sp(c);
        c.sd(stk.wrapping_add(0x90), c.g[RA][0]);
        c.sd(stk.wrapping_add(0x80), c.g[FP][0]);
        c.sd(stk.wrapping_add(0x70), c.g[S7][0]);
        c.sd(stk.wrapping_add(0x60), c.g[S6][0]);
        c.sd(stk.wrapping_add(0x50), c.g[S5][0]);
        c.sd(stk.wrapping_add(0x40), c.g[S4][0]);
        c.sd(stk.wrapping_add(0x30), c.g[S3][0]);
        c.sd(stk.wrapping_add(0x20), c.g[S2][0]);
        c.sd(stk.wrapping_add(0x10), c.g[S1][0]);
        c.sd(stk, c.g[S0][0]);
        c.g[V0][0] = c.g[A2][0];
        c.g[S3][0] = ((c.g[V0][0] as i64) >> 32) as u64; // hx
        c.g[V1][0] = ((c.g[V0][0] << 32) as i64 >> 32) as u64; // lx
        c.g[V0][0] = 0xfffff;
        let normal = (c.g[V0][0] as i64) < (c.g[S3][0] as i64);
        c.g[V0][0] = normal as u64;
        c.g[FP][0] = 0; // k

        let tail = 'body: {
            if !normal {
                // hx < 0x00100000: zero, negative or subnormal.
                c.cycles += 6;
                c.g[V0][0] = 0x7fff_ffff;
                c.g[V0][0] &= c.g[S3][0];
                c.g[V0][0] |= c.g[V1][0];
                let zero = c.g[V0][0] == 0;
                c.g[V1][0] = 0x4a0000;
                if zero {
                    // -two54 / zero = -inf
                    c.cycles += 2;
                    c.g[V0][0] = c.ld(IEEE_LOG_POOL - 0x6ef0);
                    break 'body IeeeLogTail::Ret;
                }
                c.cycles += 2;
                let negative = (c.g[S3][0] as i64) < 0;
                c.g[A0][0] = c.g[A2][0];
                if negative {
                    // (x - x) / zero = NaN
                    c.cycles += 7;
                    c.g[A1][0] = c.g[A0][0];
                    IEEE_LOG_SUBDF3.call(c, 0x0040_dbe0)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0;
                    IEEE_LOG_DIVDF3.call(c, 0x0040_dbec)?;
                    frame.restore_ra(c, 0x90);
                    break 'body IeeeLogTail::Epilogue;
                }
                // Subnormal: scale up by two54.
                c.cycles += 7;
                c.g[A1][0] = 0x4350_0000_0000_0000;
                c.g[FP][0] = (-54i64) as u64;
                IEEE_LOG_MULDF3.call(c, 0x0040_dc04)?;
                c.g[A2][0] = c.g[V0][0];
                c.g[S3][0] = ((c.g[V0][0] as i64) >> 32) as u64;
            }

            c.cycles += 5;
            c.g[V0][0] = 0x7fef_ffff;
            let inf_nan = (c.g[V0][0] as i64) < (c.g[S3][0] as i64);
            c.g[V0][0] = inf_nan as u64;
            c.g[S0][0] = 0xf0000;
            if inf_nan {
                // x + x
                c.cycles += 5;
                c.g[A0][0] = c.g[A2][0];
                c.g[A1][0] = c.g[A0][0];
                IEEE_LOG_ADDDF3.call(c, 0x0040_dc30)?;
                frame.restore_ra(c, 0x90);
                break 'body IeeeLogTail::Epilogue;
            }

            // k += (hx >> 20) - 1023; normalise x (or x/2) into [sqrt(2)/2, sqrt(2)).
            c.cycles += 31;
            c.g[A0][0] = sx((r32(c, S3) as i32 >> 20) as u32);
            c.g[S0][0] |= 0xffff;
            c.g[V0][0] = 0x95f64;
            c.g[S3][0] &= c.g[S0][0];
            c.g[V0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
            c.g[V1][0] = sx(r32(c, FP).wrapping_sub(0x3ff));
            c.g[A1][0] = 0x100000;
            c.g[FP][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
            c.g[S4][0] = c.g[V0][0] & c.g[A1][0]; // i
            c.g[A0][0] = c.g[A2][0];
            c.g[V0][0] = 0x3ff00000;
            c.g[V1][0] = 0xffff_ffff;
            c.g[V0][0] = c.g[S4][0] ^ c.g[V0][0];
            c.g[A0][0] &= c.g[V1][0];
            c.g[V0][0] |= c.g[S3][0];
            c.g[V0][0] <<= 32;
            c.g[A2][0] = c.g[A0][0] | c.g[V0][0];
            c.g[V1][0] = sx((r32(c, S4) as i32 >> 20) as u32);
            c.g[A1][0] = 0x3ff0_0000_0000_0000; // 1.0
            c.g[FP][0] = sx(r32(c, FP).wrapping_add(r32(c, V1)));
            c.g[A0][0] = c.g[A2][0];
            IEEE_LOG_SUBDF3.call(c, 0x0040_dc9c)?;
            c.g[S5][0] = c.g[V0][0]; // f = x - 1
            c.g[V0][0] = sx(r32(c, S3).wrapping_add(2));
            c.g[V0][0] &= c.g[S0][0];
            let tiny = (c.g[V0][0] as i64) < 3;
            c.g[V0][0] = tiny as u64;
            c.g[S0][0] = 0;

            if tiny {
                // |f| < 2^-20
                c.cycles += 5;
                c.g[A0][0] = c.g[S5][0];
                c.g[A1][0] = c.g[S0][0];
                IEEE_LOG_CMPDF2.call(c, 0x0040_dcc0)?;
                let f_nonzero = c.g[V0][0] != 0;
                c.g[A0][0] = c.g[S5][0];
                if !f_nonzero {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S0][0];
                    if c.g[FP][0] == 0 {
                        break 'body IeeeLogTail::Ret; // log(1) = 0
                    }
                    // dk * ln2_hi + dk * ln2_lo
                    c.cycles += 17;
                    c.g[A0][0] = c.g[FP][0];
                    IEEE_LOG_FLOATSIDF.call(c, 0x0040_dcd8)?;
                    c.g[S6][0] = c.g[V0][0];
                    ieee_log_k(c, 0x6ee8);
                    c.g[A0][0] = c.g[S6][0];
                    IEEE_LOG_MULDF3.call(c, 0x0040_dcec)?;
                    ieee_log_k(c, 0x6ee0);
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S6][0];
                    IEEE_LOG_MULDF3.call(c, 0x0040_dd00)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[V0][0];
                    IEEE_LOG_ADDDF3.call(c, 0x0040_dd0c)?;
                    frame.restore_ra(c, 0x90);
                    break 'body IeeeLogTail::Epilogue;
                }
                // R = f*f*(0.5 - 1/3*f)
                c.cycles += 16;
                c.g[A1][0] = c.g[S5][0];
                IEEE_LOG_MULDF3.call(c, 0x0040_dd20)?;
                ieee_log_k(c, 0x6ed8);
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S5][0];
                IEEE_LOG_MULDF3.call(c, 0x0040_dd34)?;
                c.g[A0][0] = 0x3fe0_0000_0000_0000; // 0.5
                c.g[A1][0] = c.g[V0][0];
                IEEE_LOG_SUBDF3.call(c, 0x0040_dd44)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                IEEE_LOG_MULDF3.call(c, 0x0040_dd50)?;
                c.g[S2][0] = c.g[V0][0];
                if c.g[FP][0] == 0 {
                    // f - R
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S5][0];
                    c.g[A1][0] = c.g[S2][0];
                    break 'body IeeeLogTail::Sub;
                }
                // dk*ln2_hi - ((R - dk*ln2_lo) - f)
                c.cycles += 20;
                c.g[A0][0] = c.g[FP][0];
                IEEE_LOG_FLOATSIDF.call(c, 0x0040_dd6c)?;
                c.g[S6][0] = c.g[V0][0];
                ieee_log_k(c, 0x6ed0);
                c.g[A0][0] = c.g[S6][0];
                IEEE_LOG_MULDF3.call(c, 0x0040_dd80)?;
                ieee_log_k(c, 0x6ec8);
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S6][0];
                IEEE_LOG_MULDF3.call(c, 0x0040_dd94)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[V0][0];
                IEEE_LOG_SUBDF3.call(c, 0x0040_dda0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S5][0];
                IEEE_LOG_SUBDF3.call(c, 0x0040_ddac)?;
                c.g[A0][0] = c.g[S0][0];
                break 'body IeeeLogTail::SubResult;
            }

            // s = f / (2 + f); z = s*s; w = z*z; R = t2 + t1 (Lg polynomial)
            c.cycles += 75;
            c.g[A1][0] = 0x4000_0000_0000_0000; // 2.0
            c.g[A0][0] = c.g[S5][0];
            IEEE_LOG_ADDDF3.call(c, 0x0040_ddc4)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S5][0];
            IEEE_LOG_DIVDF3.call(c, 0x0040_ddd0)?;
            c.g[S7][0] = c.g[V0][0]; // s
            c.g[A0][0] = c.g[FP][0];
            IEEE_LOG_FLOATSIDF.call(c, 0x0040_dddc)?;
            c.g[S6][0] = c.g[V0][0]; // dk
            c.g[A0][0] = c.g[S7][0];
            c.g[A1][0] = c.g[S7][0];
            IEEE_LOG_MULDF3.call(c, 0x0040_ddec)?;
            c.g[V1][0] = 0xffff_ffff_fff9_eb86;
            c.g[S2][0] = c.g[V0][0]; // z
            c.g[A0][0] = c.g[S2][0];
            c.g[S4][0] = sx(r32(c, S3).wrapping_add(r32(c, V1))); // i = hx - 0x6147a
            c.g[A1][0] = c.g[S2][0];
            IEEE_LOG_MULDF3.call(c, 0x0040_de08)?;
            c.g[S0][0] = 0x6b851;
            c.g[S1][0] = c.g[V0][0]; // w
            ieee_log_k(c, 0x6ec0);
            c.g[S0][0] = sx(r32(c, S0).wrapping_sub(r32(c, S3))); // j
            c.g[A0][0] = c.g[S1][0];
            IEEE_LOG_MULDF3.call(c, 0x0040_de28)?;
            c.g[S4][0] |= c.g[S0][0];
            ieee_log_k(c, 0x6eb8);
            c.g[A0][0] = c.g[V0][0];
            IEEE_LOG_ADDDF3.call(c, 0x0040_de3c)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            IEEE_LOG_MULDF3.call(c, 0x0040_de48)?;
            ieee_log_k(c, 0x6eb0);
            c.g[A0][0] = c.g[V0][0];
            IEEE_LOG_ADDDF3.call(c, 0x0040_de58)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            IEEE_LOG_MULDF3.call(c, 0x0040_de64)?;
            ieee_log_k(c, 0x6ea8);
            c.g[S0][0] = c.g[V0][0]; // t1
            c.g[A0][0] = c.g[S1][0];
            IEEE_LOG_MULDF3.call(c, 0x0040_de78)?;
            ieee_log_k(c, 0x6ea0);
            c.g[A0][0] = c.g[V0][0];
            IEEE_LOG_ADDDF3.call(c, 0x0040_de88)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            IEEE_LOG_MULDF3.call(c, 0x0040_de94)?;
            ieee_log_k(c, 0x6e98);
            c.g[A0][0] = c.g[V0][0];
            IEEE_LOG_ADDDF3.call(c, 0x0040_dea4)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[V0][0];
            IEEE_LOG_MULDF3.call(c, 0x0040_deb0)?;
            ieee_log_k(c, 0x6e90);
            c.g[A0][0] = c.g[V0][0];
            IEEE_LOG_ADDDF3.call(c, 0x0040_dec0)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[V0][0];
            IEEE_LOG_MULDF3.call(c, 0x0040_decc)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S0][0];
            IEEE_LOG_ADDDF3.call(c, 0x0040_ded8)?;
            let far = (c.g[S4][0] as i64) > 0;
            c.g[S2][0] = c.g[V0][0]; // R

            if far {
                // hfsq = 0.5*f*f
                c.cycles += 9;
                c.g[A1][0] = 0x3fe0_0000_0000_0000;
                c.g[A0][0] = c.g[S5][0];
                IEEE_LOG_MULDF3.call(c, 0x0040_def0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S5][0];
                IEEE_LOG_MULDF3.call(c, 0x0040_defc)?;
                c.g[S3][0] = c.g[V0][0];
                if c.g[FP][0] == 0 {
                    // f - (hfsq - s*(hfsq + R))
                    c.cycles += 11;
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[S3][0];
                    IEEE_LOG_ADDDF3.call(c, 0x0040_df10)?;
                    c.g[A0][0] = c.g[S7][0];
                    c.g[A1][0] = c.g[V0][0];
                    IEEE_LOG_MULDF3.call(c, 0x0040_df1c)?;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[V0][0];
                    IEEE_LOG_SUBDF3.call(c, 0x0040_df28)?;
                    c.g[A0][0] = c.g[S5][0];
                    break 'body IeeeLogTail::SubResult;
                }
                // dk*ln2_hi - ((hfsq - (s*(hfsq + R) + dk*ln2_lo)) - f)
                c.cycles += 21;
                ieee_log_k(c, 0x6e88);
                c.g[A0][0] = c.g[S6][0];
                IEEE_LOG_MULDF3.call(c, 0x0040_df40)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S3][0];
                IEEE_LOG_ADDDF3.call(c, 0x0040_df50)?;
                c.g[A0][0] = c.g[S7][0];
                c.g[A1][0] = c.g[V0][0];
                IEEE_LOG_MULDF3.call(c, 0x0040_df5c)?;
                ieee_log_k(c, 0x6e80);
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S6][0];
                IEEE_LOG_MULDF3.call(c, 0x0040_df70)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                IEEE_LOG_ADDDF3.call(c, 0x0040_df7c)?;
                c.g[A0][0] = c.g[S3][0];
            } else {
                c.cycles += 2;
                if c.g[FP][0] == 0 {
                    // f - s*(f - R)
                    c.cycles += 8;
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[S5][0];
                    IEEE_LOG_SUBDF3.call(c, 0x0040_df98)?;
                    c.g[A0][0] = c.g[S7][0];
                    c.g[A1][0] = c.g[V0][0];
                    IEEE_LOG_MULDF3.call(c, 0x0040_dfa4)?;
                    c.g[A0][0] = c.g[S5][0];
                    break 'body IeeeLogTail::SubResult;
                }
                // dk*ln2_hi - ((s*(f - R) - dk*ln2_lo) - f)
                c.cycles += 17;
                ieee_log_k(c, 0x6e78);
                c.g[A0][0] = c.g[S6][0];
                IEEE_LOG_MULDF3.call(c, 0x0040_dfbc)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S5][0];
                IEEE_LOG_SUBDF3.call(c, 0x0040_dfcc)?;
                c.g[A0][0] = c.g[S7][0];
                c.g[A1][0] = c.g[V0][0];
                IEEE_LOG_MULDF3.call(c, 0x0040_dfd8)?;
                ieee_log_k(c, 0x6e70);
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S6][0];
                IEEE_LOG_MULDF3.call(c, 0x0040_dfec)?;
                c.g[A0][0] = c.g[S0][0];
            }
            c.cycles += 6;
            c.g[A1][0] = c.g[V0][0];
            IEEE_LOG_SUBDF3.call(c, 0x0040_dff8)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S5][0];
            IEEE_LOG_SUBDF3.call(c, 0x0040_e004)?;
            c.g[A0][0] = c.g[S1][0];
            IeeeLogTail::SubResult
        };

        if tail <= IeeeLogTail::SubResult {
            c.cycles += 1;
            c.g[A1][0] = c.g[V0][0];
        }
        if tail <= IeeeLogTail::Sub {
            c.cycles += 2;
            IEEE_LOG_SUBDF3.call(c, 0x0040_e014)?;
        }
        if tail <= IeeeLogTail::Ret {
            c.cycles += 1;
            frame.restore_ra(c, 0x90);
        }
        c.cycles += 11;
        let stk = sp(c);
        c.g[FP][0] = c.ld(stk.wrapping_add(0x80));
        c.g[S7][0] = c.ld(stk.wrapping_add(0x70));
        c.g[S6][0] = c.ld(stk.wrapping_add(0x60));
        c.g[S5][0] = c.ld(stk.wrapping_add(0x50));
        c.g[S4][0] = c.ld(stk.wrapping_add(0x40));
        c.g[S3][0] = c.ld(stk.wrapping_add(0x30));
        c.g[S2][0] = c.ld(stk.wrapping_add(0x20));
        c.g[S1][0] = c.ld(stk.wrapping_add(0x10));
        c.g[S0][0] = c.ld(stk);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod callee {
    pub(crate) use super::ABORT;
    pub(crate) use super::GET_EH_INFO;
    pub(crate) use super::IEEE_LOG_DIVDF3;
    pub(crate) use super::MALLOC;
    pub(crate) use super::TERMINATE;
    pub(crate) use super::TERMINATE_CPP;
    pub(crate) use super::TYPE_INFO_EQ;
    pub(crate) use crate::game::lib::libc::callee::MEMCPY;
    pub(crate) use crate::game::misc::MEMSET;

    /// `__sjthrow` (installed slot).
    pub(crate) const SJTHROW: Callee = Callee {
        addr: 0x0041_0570,
        f: ee::s15::f_00410570,
    };

    /// `__eh_rtime_match` (installed slot).
    pub(crate) const EH_RTIME_MATCH: Callee = Callee {
        addr: 0x0041_0800,
        f: ee::s15::f_00410800,
    };

    /// `__frame_state_for` (installed slot).
    pub(crate) const FRAME_STATE_FOR: Callee = Callee {
        addr: 0x0041_4818,
        f: ee::s15::f_00414818,
    };

    /// `throw_helper` (installed slot).
    pub(crate) const THROW_HELPER: Callee = Callee {
        addr: 0x0041_0c18,
        f: ee::s15::f_00410c18,
    };

    /// `__unwinding_cleanup` (installed slot).
    pub(crate) const UNWINDING_CLEANUP: Callee = Callee {
        addr: 0x0041_0c10,
        f: ee::s15::f_00410c10,
    };

    /// `old_find_exception_handler` (installed slot).
    pub(crate) const OLD_FIND_HANDLER: Callee = Callee {
        addr: 0x0041_0858,
        f: ee::s15::f_00410858,
    };

    /// `find_exception_handler` (installed slot).
    pub(crate) const FIND_HANDLER: Callee = Callee {
        addr: 0x0041_0930,
        f: ee::s15::f_00410930,
    };

    /// `get_reg_addr` (installed slot).
    pub(crate) const GET_REG_ADDR: Callee = Callee {
        addr: 0x0041_0a78,
        f: ee::s15::f_00410a78,
    };

    /// `copy_reg` (installed slot).
    pub(crate) const COPY_REG: Callee = Callee {
        addr: 0x0041_0b00,
        f: ee::s15::f_00410b00,
    };

    /// `next_stack_level` (installed slot).
    pub(crate) const NEXT_STACK_LEVEL: Callee = Callee {
        addr: 0x0041_0b80,
        f: ee::s15::f_00410b80,
    };

    /// `set_terminate` (generated, has resume entries; call at the entry).
    pub(crate) const SET_TERMINATE: Callee = Callee {
        addr: 0x0041_2818,
        f: ee::s15::f_00412818,
    };

    /// `__uncatch_exception` (installed slot).
    pub(crate) const UNCATCH_EXCEPTION: Callee = Callee {
        addr: 0x0041_2af0,
        f: ee::s15::__uncatch_exception,
    };

    /// `__cp_exception_info` (installed slot).
    pub(crate) const CP_EXCEPTION_INFO: Callee = Callee {
        addr: 0x0041_2848,
        f: ee::s15::__cp_exception_info,
    };

    /// `__start_cp_handler` (installed slot).
    pub(crate) const START_CP_HANDLER: Callee = Callee {
        addr: 0x0041_28a8,
        f: ee::s15::__start_cp_handler,
    };

    /// `__tf13bad_exception` (installed slot).
    pub(crate) const TF13BAD_EXCEPTION: Callee = Callee {
        addr: 0x0041_2e80,
        f: ee::s15::__tf13bad_exception,
    };

    /// `__eh_alloc` (installed slot).
    pub(crate) const EH_ALLOC: Callee = Callee {
        addr: 0x0041_28e0,
        f: ee::s15::__eh_alloc,
    };

    /// `__eh_free` (installed slot).
    pub(crate) const EH_FREE: Callee = Callee {
        addr: 0x0041_2908,
        f: ee::s15::__eh_free,
    };

    /// `__throw` (generated; the slot runs the hand port after install).
    pub(crate) const THROW: Callee = Callee {
        addr: 0x0041_0f48,
        f: ee::s15::f_00410f48,
    };

    /// `__throw_type_match_rtti_2` (generated).
    pub(crate) const THROW_TYPE_MATCH_RTTI_2: Callee = Callee {
        addr: 0x0041_5260,
        f: ee::s15::f_00415260,
    };

    /// `__is_pointer` (generated).
    pub(crate) const IS_POINTER: Callee = Callee {
        addr: 0x0041_5728,
        f: ee::s15::f_00415728,
    };

    /// `__cp_push_exception` (installed slot).
    pub(crate) const CP_PUSH_EXCEPTION: Callee = Callee {
        addr: 0x0041_2998,
        f: ee::s15::f_00412998,
    };

    /// `_fpadd_parts_f` (installed slot).
    pub(crate) const FPADD_PARTS_F: Callee = Callee {
        addr: 0x0041_30f8,
        f: ee::s15::f_004130f8,
    };

    /// `fde_merge` (installed slot).
    pub(crate) const FDE_MERGE: Callee = Callee {
        addr: 0x0041_3bd0,
        f: ee::s15::f_00413bd0,
    };

    /// `fde_split` (generated).
    pub(crate) const FDE_SPLIT: Callee = Callee {
        addr: 0x0041_49f0,
        f: ee::s15::f_004149f0,
    };

    /// `count_fdes` (installed slot).
    pub(crate) const COUNT_FDES: Callee = Callee {
        addr: 0x0041_3f38,
        f: ee::s15::f_00413f38,
    };

    /// `add_fdes` (generated).
    pub(crate) const ADD_FDES: Callee = Callee {
        addr: 0x0041_3f80,
        f: ee::s15::f_00413f80,
    };

    /// `end_fde_sort` (installed slot).
    pub(crate) const END_FDE_SORT: Callee = Callee {
        addr: 0x0041_3cd0,
        f: ee::s15::f_00413cd0,
    };

    /// `decode_uleb128` (installed slot).
    pub(crate) const DECODE_ULEB128: Callee = Callee {
        addr: 0x0041_3b38,
        f: ee::s15::f_00413b38,
    };

    /// `execute_cfa_insn` (generated).
    pub(crate) const EXECUTE_CFA_INSN: Callee = Callee {
        addr: 0x0041_4338,
        f: ee::s15::f_00414338,
    };

    /// `extract_cie_info` (generated).
    pub(crate) const EXTRACT_CIE_INFO: Callee = Callee {
        addr: 0x0041_4230,
        f: ee::s15::f_00414230,
    };

    /// `find_fde` (generated).
    pub(crate) const FIND_FDE: Callee = Callee {
        addr: 0x0041_4148,
        f: ee::s15::f_00414148,
    };

    /// `__dynamic_cast` (installed slot).
    pub(crate) const DYNAMIC_CAST: Callee = Callee {
        addr: 0x0041_5840,
        f: ee::s15::f_00415840,
    };

    /// `__tfv` (already installed upstream).
    pub(crate) const TFV: Callee = Callee {
        addr: 0x0041_58f0,
        f: ee::s15::f_004158f0,
    };

    /// `__ieee754_log_2` (generated).
    pub(crate) const IEEE754_LOG_2: Callee = Callee {
        addr: 0x0041_f3a8,
        f: ee::s15::f_0041f3a8,
    };
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::lib::gcc::RTTI_SI;
    pub(crate) use crate::game::lib::gcc::RTTI_USER;
    pub(crate) use crate::game::misc::callee::FRAME_INIT;
}

pub(crate) mod fmodf {

    /// Below this unbiased exponent the result is subnormal.
    pub const SUB_EXP: i64 = -126;
}

pub(crate) mod sj {

    /// Saved context pointer.
    pub const CTX: u32 = 0x640;

    /// Handler-chain head (`ctx + 4`).
    pub const HEAD: u32 = 0x644;

    /// Handler-chain link word.
    pub const LINK: u32 = 0x648;

    /// Restored frame pointer slot.
    pub const FP_SLOT: u32 = 0x64c;
}

// ---------------------------------------------------------------------------
// __ieee754_fmodf (0x0040e300)

/// `float __ieee754_fmodf(float x, float y)` (0x40e300, fdlibm): the
/// floating-point remainder of `x / y` in f0. `x` arrives in f12, `y` in f13.
/// Infinite or NaN `x`, or zero or NaN `y`: `x*y/(x*y)`. Finite `|x| < |y|`
/// runs the reduction below (which returns `x`). Otherwise the exponents are
/// aligned and the mantissas reduced by long division. Dispatcher shape:
/// the entry plus every loop head and the exact-zero join, so an event
/// unwind resumes where generated does.
pub fn ieee754_fmodf(c: &mut Cpu) {
    call::guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                // Entry: classify `x` and `y`, then reduce or return.
                // The reduction starts inline (align `|x|`); tiny `|x|`
                // detours through the shift loop below.
                0x0040_e300 => {
                    c.mfc1(A3, 13);
                    c.mfc1(A0, 12);
                    c.g[A1][0] = c.g[A0][0];
                    c.g[A2][0] = c.g[A3][0];
                    c.g[V1][0] = 0xffff_ffff_8000_0000;
                    c.g[V0][0] = 0x7fff_0000;
                    c.g[T1][0] = c.g[A1][0] & c.g[V1][0];
                    c.g[V0][0] |= 0xffff;
                    c.g[A2][0] &= c.g[V0][0];
                    c.cycles += 11;
                    if c.g[A2][0] != 0 {
                        c.g[A1][0] ^= c.g[T1][0];
                        // Any |x| above 0x7f7fffff is NaN or infinite: x*y/(x*y).
                        c.g[V0][0] = 0x7f7f_0000;
                        c.g[V0][0] |= 0xffff;
                        c.cycles += 5;
                        let x_nan_inf = (c.g[V0][0] as i64) < (c.g[A1][0] as i64);
                        c.g[V0][0] = 0x7f80_0000;
                        if !x_nan_inf {
                            // |x| is finite. A |y| above 0x7f800000 is NaN (infinity
                            // is exactly 0x7f800000): also x*y/(x*y).
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A2][0] as i64)) as u64;
                            if c.g[V0][0] == 0 {
                                // Finite y: |x| >= |y| reduces (|x| == |y| via the
                                // table), else x. (The +2 both 0x370 exits charge.)
                                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                                c.cycles += 2;
                                if c.g[V0][0] == 0 {
                                    // The 0x384 select: +2, V0 preset, then reduce or
                                    // table.
                                    c.g[V0][0] = 0x7f_0000;
                                    c.cycles += 2;
                                    if c.g[A1][0] != c.g[A2][0] {
                                        // Reduce: align |x| first (unbiased exponent
                                        // into t0, or shift up denormals).
                                        c.g[V0][0] |= 0xffff;
                                        c.cycles += 4;
                                        let x_big = (c.g[V0][0] as i64) < (c.g[A1][0] as i64);
                                        c.g[V0][0] = sx(((c.g[A1][0] as u32) as i32 >> 23) as u32);
                                        if x_big {
                                            // Normal |x|: exponent direct.
                                            c.cycles += 1;
                                            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7f));
                                            c.cycles += 1;
                                            c.g[V0][0] = 0x7f_0000;
                                            pc = 0x0040_e3f4;
                                            continue;
                                        }
                                        // Tiny |x|: shift up from -126.
                                        c.cycles += 3;
                                        c.g[V0][0] = sx((c.g[A1][0] as u32) << 8);
                                        let t = (c.g[V0][0] as i64) <= 0;
                                        c.g[T0][0] = 0xffff_ffff_ffff_ff82;
                                        if t {
                                            c.cycles += 1;
                                            c.g[V0][0] = 0x7f_0000;
                                            pc = 0x0040_e3f4;
                                            continue;
                                        }
                                        // The 0x3c4 entry: fall into the shift loop.
                                        c.cycles += 1;
                                        pc = 0x0040_e3c8;
                                        continue;
                                    }
                                    pc = 0x0040_e38c;
                                    continue;
                                }
                                // |x| < |y|: plain x (the mov_s tail).
                                c.mtc1(A0, 0);
                                c.cycles += 3;
                                let t = c.g[RA][0] as u32;
                                c.pc = t;
                                return Ok(());
                            }
                            // 0x34c join: y is NaN, fall through to x*y/(x*y).
                            c.cycles += 1;
                        }
                    } else {
                        c.g[A1][0] ^= c.g[T1][0];
                    }
                    // y is NaN/infinite, or x is infinite: x*y/(x*y).
                    c.mtc1(A0, 1);
                    c.mtc1(A3, 2);
                    c.mul_s(0, 1, 2);
                    c.div_s(0, 0, 0);
                    c.cycles += 8;
                    let t = c.g[RA][0] as u32;
                    c.pc = t;
                    return Ok(());
                }
                // The 0x3c8 shift loop: align tiny |x| upward.
                0x0040_e3c8 => {
                    c.cycles += 7;
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(1));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                    if (c.g[V0][0] as i64) > 0 {
                        if back_edge_unwinds(c, 0x0040_e3c8) {
                            return Ok(());
                        }
                        pc = 0x0040_e3c8;
                        continue;
                    }
                    // The 0x3e4 exit.
                    c.cycles += 2;
                    c.g[V0][0] = 0x7f_0000;
                    pc = 0x0040_e3f4;
                    continue;
                }
                // Align |y| the same way, counting in a3.
                0x0040_e3f4 => {
                    c.g[V0][0] |= 0xffff;
                    c.cycles += 4;
                    let y_big = (c.g[V0][0] as i64) < (c.g[A2][0] as i64);
                    c.g[V0][0] = sx(((c.g[A2][0] as u32) as i32 >> 23) as u32);
                    if y_big {
                        // Normal |y|: exponent direct into a3.
                        c.cycles += 3;
                        c.g[A0][0] = ((c.g[T0][0] as i64) < fmodf::SUB_EXP) as u64;
                        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7f));
                        pc = 0x0040_e44c;
                        continue;
                    }
                    // Tiny |y|: shift up from -126.
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[A2][0] as u32) << 8);
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[A3][0] = 0xffff_ffff_ffff_ff82;
                    if t {
                        // The 0x448 exit: top bit set, no shift loop.
                        c.cycles += 1;
                        c.g[A0][0] = ((c.g[T0][0] as i64) < fmodf::SUB_EXP) as u64;
                        pc = 0x0040_e44c;
                        continue;
                    }
                    // The 0x410 entry and 0x418 shift loop.
                    c.cycles += 2;
                    c.g[A0][0] = ((c.g[T0][0] as i64) < fmodf::SUB_EXP) as u64;
                    pc = 0x0040_e418;
                    continue;
                }
                // The 0x418 shift loop.
                0x0040_e418 => {
                    c.cycles += 7;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(1));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                    if (c.g[V0][0] as i64) >= 0 {
                        if back_edge_unwinds(c, 0x0040_e418) {
                            return Ok(());
                        }
                        pc = 0x0040_e418;
                        continue;
                    }
                    // The 0x434 exit.
                    c.cycles += 2;
                    pc = 0x0040_e44c;
                    continue;
                }
                // Pre-scale the aligned mantissas, set the round count.
                0x0040_e44c => {
                    c.cycles += 2;
                    if c.g[A0][0] == 0 {
                        c.g[V0][0] = 0x7f_0000;
                        c.g[V1][0] = 0x80_0000;
                        c.g[V0][0] |= 0xffff;
                        c.g[V0][0] = c.g[A1][0] & c.g[V0][0];
                        c.g[A1][0] = c.g[V0][0] | c.g[V1][0];
                        c.cycles += 6;
                    } else {
                        c.g[V0][0] = 0xffff_ffff_ffff_ff82;
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[T0][0] as u32));
                        c.g[A1][0] = sx((c.g[A1][0] as u32) << (c.g[A0][0] as u32 & 31));
                        c.cycles += 2;
                    }
                    c.g[T2][0] = ((c.g[A3][0] as i64) < fmodf::SUB_EXP) as u64;
                    c.cycles += 3;
                    if c.g[T2][0] == 0 {
                        c.g[V0][0] = 0x7f_0000;
                        c.g[V1][0] = 0x80_0000;
                        c.g[V0][0] |= 0xffff;
                        c.cycles += 6;
                        c.g[V0][0] = c.g[A2][0] & c.g[V0][0];
                        c.g[A2][0] = c.g[V0][0] | c.g[V1][0];
                    } else {
                        c.g[V0][0] = 0xffff_ffff_ffff_ff82;
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A3][0] as u32));
                        c.g[A2][0] = sx((c.g[A2][0] as u32) << (c.g[A0][0] as u32 & 31));
                        c.cycles += 2;
                    }
                    // Long division of the aligned mantissas; the 0x4a0 head sets the
                    // round count, entered through the 0x4bc round check below.
                    c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_sub(c.g[A3][0] as u32));
                    c.cycles += 2;
                    pc = 0x0040_e4bc;
                    continue;
                }
                // The 0x4bc round check.
                0x0040_e4bc => {
                    c.g[V0][0] = c.g[A0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
                    c.cycles += 3;
                    if c.g[V0][0] == 0 {
                        pc = 0x0040_e4c8;
                        continue;
                    }
                    if back_edge_unwinds(c, 0x0040_e4a8) {
                        return Ok(());
                    }
                    pc = 0x0040_e4a8;
                    continue;
                }
                // The 0x4a8 head: one subtraction trial.
                0x0040_e4a8 => {
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[A2][0] as u32));
                    c.g[A1][0] = sx((c.g[A1][0] as u32) << 1);
                    c.cycles += 3;
                    if (c.g[V1][0] as i64) >= 0 {
                        // The 0x4b4 shift: an exact zero remainder returns the
                        // table float (signed zero in f0); otherwise rejoin at 4bc.
                        c.cycles += 2;
                        c.g[A1][0] = sx((c.g[V1][0] as u32) << 1);
                        if c.g[V1][0] == 0 {
                            if back_edge_unwinds(c, 0x0040_e38c) {
                                return Ok(());
                            }
                            pc = 0x0040_e38c;
                            continue;
                        }
                    }
                    pc = 0x0040_e4bc;
                    continue;
                }
                // The exact-zero join: signed zero through the table.
                0x0040_e38c => {
                    c.cycles += 7;
                    c.g[V1][0] = sx((c.g[T1][0] as u32) >> 31);
                    c.g[V0][0] = sx(0x004a_0000u32.wrapping_sub(0x6e10));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.lwc1(0, c.g[V1][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.pc = t;
                    return Ok(());
                }
                // The 0x4c8 remainder test: keep the non-negative difference, and a
                // zero remainder returns the table float (signed zero in f0). The
                // 0x7f0000 preset carries no cycles (0x4dc charges +4).
                0x0040_e4c8 => {
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[A2][0] as u32));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 0) as u64;
                    c.cycles += 5;
                    if c.g[V0][0] == 0 {
                        c.g[A1][0] = c.g[V1][0];
                    }
                    c.g[V0][0] = 0x7f_0000;
                    if c.g[A1][0] == 0 {
                        if back_edge_unwinds(c, 0x0040_e38c) {
                            return Ok(());
                        }
                        pc = 0x0040_e38c;
                        continue;
                    }
                    pc = 0x0040_e4dc;
                    continue;
                }
                // The 0x4dc level test, then the shift-up loop or the tail.
                0x0040_e4dc => {
                    c.g[V0][0] |= 0xffff;
                    c.cycles += 4;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    if c.g[V0][0] != 0 {
                        // Big enough already: rejoin at the 0x518 exponent test
                        // (t2 still holds the 0x474 value).
                        pc = 0x0040_e518;
                        continue;
                    }
                    c.g[V1][0] = 0x7f_0000;
                    c.g[V1][0] |= 0xffff;
                    c.cycles += 3;
                    pc = 0x0040_e4f8;
                    continue;
                }
                // The 0x4f8 shift-up loop.
                0x0040_e4f8 => {
                    c.g[A1][0] = sx((c.g[A1][0] as u32) << 1);
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(1));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    c.cycles += 7;
                    if c.g[V0][0] != 0 {
                        // Done shifting: the 0x514 exit.
                        c.cycles += 1;
                        c.g[T2][0] = ((c.g[A3][0] as i64) < fmodf::SUB_EXP) as u64;
                        pc = 0x0040_e518;
                        continue;
                    }
                    if back_edge_unwinds(c, 0x0040_e4f8) {
                        return Ok(());
                    }
                    pc = 0x0040_e4f8;
                    continue;
                }
                // The 0x518 exponent test and the rebuild.
                0x0040_e518 => {
                    c.cycles += 2;
                    // Delay slot, both arms: the subnormal arm shifts by
                    // -126 - exponent.
                    c.g[V0][0] = fmodf::SUB_EXP as u64;
                    if c.g[T2][0] == 0 {
                        c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x7f));
                        c.g[V0][0] = 0xffff_ffff_ff80_0000;
                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 23);
                        c.g[A1][0] = c.g[V0][0] | c.g[V1][0];
                        c.cycles += 6;
                    } else {
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A3][0] as u32));
                        c.g[A1][0] =
                            sx(((c.g[A1][0] as u32) as i32 >> (c.g[A0][0] as u32 & 31)) as u32);
                        c.cycles += 2;
                    }
                    c.g[A0][0] = c.g[A1][0] | c.g[T1][0];
                    c.mtc1(A0, 0);
                    c.cycles += 4;
                    let t = c.g[RA][0] as u32;
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0040_e300, 0x0040_e550);
                    return Ok(());
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// Trivial EH stubs

/// `void __empty(void)` (0x410408): return, charging 2 cycles.
pub fn empty(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.pc = t;
}

/// `void __unwinding_cleanup(void)` (0x410c10): return, charging 2 cycles.
pub fn unwinding_cleanup(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.pc = t;
}

/// `short __get_eh_table_version(void *x)` (0x410848): the halfword at
/// `x + 6` (the exception-table version).
pub fn get_eh_table_version(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x6));
    c.pc = t;
}

/// `short __get_eh_table_language(void *x)` (0x410850): the halfword at
/// `x + 4` (the exception-table language).
pub fn get_eh_table_language(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x4));
    c.pc = t;
}

/// `void __pure_virtual(void)` (0x411530): a pure virtual call is fatal;
/// tail-calls `__terminate` (the default-terminate shape: set `pc`, call
/// the slot, leave `pc` where it lands).
pub fn pure_virtual(c: &mut Cpu) {
    c.cycles += 2;
    c.pc = 0x0041_03b0;
    (callee::TERMINATE.f)(c);
}

/// `void __sjthrow(void)` (0x410570, setjmp/longjmp throw): deliver the
/// pending exception to the handler chain of the current thread's
/// exception context. The chain is walked with per-link matcher callbacks
/// (register-indirect calls); the surviving handler longjmps, restoring
/// `fp`/`sp` from the target context, and control re-enters here
/// (`continue`) or leaves (`jump_out`). With no handler the frame-state
/// search below runs and terminates.
///
/// Control flow as generated, minus its unreachable block: the `+2` /
/// `__terminate` after the handler walk is dead (both live paths break
/// out to the `0x6d4` search first), so both paths fall through to it.
pub fn sjthrow(c: &mut Cpu) {
    call::guest(c, |c| {
        'entry: loop {
            c.cycles += 36;
            let frame = Frame::open(c, 0x750);
            c.g[V1][0] = 0x450000;
            c.sd(frame.at(c, 0x6d0), c.g[FP][0]);
            c.sd(frame.at(c, 0x6e0), c.g[RA][0]);
            c.sd(frame.at(c, 0x6c0), c.g[S7][0]);
            c.sd(frame.at(c, 0x6b0), c.g[S6][0]);
            c.sd(frame.at(c, 0x6a0), c.g[S5][0]);
            c.sd(frame.at(c, 0x690), c.g[S4][0]);
            c.sd(frame.at(c, 0x680), c.g[S3][0]);
            c.sd(frame.at(c, 0x670), c.g[S2][0]);
            c.sd(frame.at(c, 0x660), c.g[S1][0]);
            c.sd(frame.at(c, 0x650), c.g[S0][0]);
            c.swc1(31, frame.at(c, 0x748));
            c.swc1(30, frame.at(c, 0x740));
            c.swc1(29, frame.at(c, 0x738));
            c.swc1(28, frame.at(c, 0x730));
            c.swc1(27, frame.at(c, 0x728));
            c.swc1(26, frame.at(c, 0x720));
            c.swc1(25, frame.at(c, 0x718));
            c.swc1(24, frame.at(c, 0x710));
            c.swc1(23, frame.at(c, 0x708));
            c.swc1(22, frame.at(c, 0x700));
            c.swc1(21, frame.at(c, 0x6f8));
            c.swc1(20, frame.at(c, 0x6f0));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3444));
            c.g[RA][0] = sx(0x004105dc);
            c.g[FP][0] = c.g[SP][0];
            let target = c.g[V0][0] as u32;
            call::call_ptr(c, target, 0x0041_05dc)?;
            c.sw((c.g[FP][0] as u32).wrapping_add(sj::CTX), c.g[V0][0]);
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
            c.sw((c.g[FP][0] as u32).wrapping_add(sj::HEAD), c.g[V0][0]);
            c.sw((c.g[FP][0] as u32).wrapping_add(sj::LINK), c.g[A1][0]);
            if c.lw((c.g[V1][0] as u32).wrapping_add(0x4)) != 0 {
                // The handler chain is live: preserve the current
                // registration, then run each link's matcher.
                sj_preserve(c);
                // The 0x690 head: V0 is still 0 from the preserve
                // block, so its break is dead and the walk runs.
                c.cycles += 2;
                c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(sj::FP_SLOT));
                'links: {
                    c.cycles += 6;
                    c.g[A2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(sj::HEAD));
                    c.sw(c.g[A2][0] as u32, c.g[V0][0]);
                    c.g[A1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(sj::LINK));
                    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                    c.g[A1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(sj::FP_SLOT));
                    if c.g[V1][0] == 0 {
                        break 'links;
                    }
                    c.cycles += 1;
                    'match_: loop {
                        c.cycles += 11;
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        c.g[A1][0] = 0x2;
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                        c.g[A2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(sj::LINK));
                        c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                        let target = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x0041066c);
                        c.sw(c.g[A2][0] as u32, c.g[V0][0]);
                        call::call_ptr(c, target, 0x0041_066c)?;
                        c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(sj::LINK));
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        c.g[V1][0] = c.g[V0][0];
                        if c.g[V0][0] != 0 {
                            if event_due(c) {
                                deliver_event_here(c, 0x0041_0650);
                            }
                            continue 'match_;
                        }
                        c.cycles += 1;
                        c.g[A1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(sj::FP_SLOT));
                        break 'links;
                    }
                }
                c.cycles += 4;
                c.g[A2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(sj::HEAD));
                c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                c.sw(c.g[A2][0] as u32, c.g[V0][0]);
            }
            // The `0x6d4` frame-state search: find the landing context.
            'search: {
                'probe: {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(sj::CTX));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(sj::HEAD));
                    if c.g[V0][0] == 0 {
                        break 'probe;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.lw(c.g[A1][0] as u32);
                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
                    if c.g[V0][0] != 0 {
                        break 'search;
                    }
                    break 'probe;
                }
                c.cycles += 7;
                callee::TERMINATE.call(c, 0x0041_06c0)?;
                c.g[A2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(sj::HEAD));
                c.g[A0][0] = c.lw(c.g[A2][0] as u32);
                c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(sj::HEAD));
                break 'search;
            }
            c.cycles += 5;
            c.sw(c.g[A1][0] as u32, c.g[V0][0]);
            c.g[FP][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
            let target = c.g[V0][0] as u32;
            c.g[SP][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
            if target == 0x0041_0570 {
                if event_due(c) {
                    deliver_event_here(c, 0x0041_0570);
                }
                continue 'entry;
            }
            c.jump_out(target, 0x0041_0570, 0x0041_06e8);
            return Ok(());
        }
    })
}

/// The `0x62c` preserve block of `__sjthrow`: stash the current handler
/// registration on the frame so the walk below can overwrite it.
pub(crate) fn sj_preserve(c: &mut Cpu) {
    c.cycles += 10;
    c.sw(c.g[FP][0] as u32, c.g[V1][0]);
    c.g[V0][0] = sx(0x0041_0000u32.wrapping_add(0x628));
    c.sw((c.g[FP][0] as u32).wrapping_add(0x4), 0);
    c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x8));
    c.sw((c.g[FP][0] as u32).wrapping_add(0x8), c.g[FP][0]);
    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    c.sw((c.g[FP][0] as u32).wrapping_add(sj::FP_SLOT), c.g[FP][0]);
    c.g[V0][0] = 0;
    c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[SP][0]);
}

// ---------------------------------------------------------------------------
// __sjpopnthrow (0x004106e8)

/// `void __sjpopnthrow(void)` (0x4106e8): pop one `setjmp` handler frame
/// and rethrow: like `__sjthrow` but the handler list is unwound first
/// (the `0x7e0` block unlinks the head), then the previous context's
/// `__sjthrow` runs and the runtime matcher re-checks the new head.
pub fn sjpopnthrow(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 34;
        c.g[V1][0] = 0x450000;
        let frame = Frame::open(c, 0x750);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3444));
        c.sd(frame.at(c, 0x6e0), c.g[RA][0]);
        c.sd(frame.at(c, 0x6d0), c.g[FP][0]);
        c.sd(frame.at(c, 0x6c0), c.g[S7][0]);
        c.sd(frame.at(c, 0x6b0), c.g[S6][0]);
        c.sd(frame.at(c, 0x6a0), c.g[S5][0]);
        c.sd(frame.at(c, 0x690), c.g[S4][0]);
        c.sd(frame.at(c, 0x680), c.g[S3][0]);
        c.sd(frame.at(c, 0x670), c.g[S2][0]);
        c.sd(frame.at(c, 0x660), c.g[S1][0]);
        c.sd(frame.at(c, 0x650), c.g[S0][0]);
        c.swc1(31, frame.at(c, 0x748));
        c.swc1(30, frame.at(c, 0x740));
        c.swc1(29, frame.at(c, 0x738));
        c.swc1(28, frame.at(c, 0x730));
        c.swc1(27, frame.at(c, 0x728));
        c.swc1(26, frame.at(c, 0x720));
        c.swc1(25, frame.at(c, 0x718));
        c.swc1(24, frame.at(c, 0x710));
        c.swc1(23, frame.at(c, 0x708));
        c.swc1(22, frame.at(c, 0x700));
        c.swc1(21, frame.at(c, 0x6f8));
        c.g[RA][0] = sx(0x00410750);
        c.swc1(20, frame.at(c, 0x6f0));
        let target = c.g[V0][0] as u32;
        call::call_ptr(c, target, 0x0041_0750)?;
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
        c.sw(frame.at(c, 0x640), c.g[V0][0]);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
        c.sw(frame.at(c, 0x644), c.g[V0][0]);
        if c.lw((c.g[V1][0] as u32).wrapping_add(0x4)) != 0 {
            // Unlink the head link, then walk the rest like `sjthrow`.
            sjpopn_unlink(c);
            'links: {
                c.g[V1][0] = c.lw(frame.at(c, 0x640));
                c.g[A2][0] = c.lw(frame.at(c, 0x648));
                c.sw(c.g[V1][0] as u32, c.g[A2][0]);
                c.g[V0][0] = c.lw(frame.at(c, 0x644));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.g[A2][0] = c.lw(frame.at(c, 0x648));
                if c.g[V1][0] != 0 {
                    c.cycles += 1;
                    'match_: loop {
                        c.cycles += 11;
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        c.g[A1][0] = 0x2;
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                        c.g[A2][0] = c.lw(frame.at(c, 0x644));
                        c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                        let target = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x004107cc);
                        c.sw(c.g[A2][0] as u32, c.g[V0][0]);
                        call::call_ptr(c, target, 0x0041_07cc)?;
                        c.g[V1][0] = c.lw(frame.at(c, 0x644));
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        c.g[V1][0] = c.g[V0][0];
                        if c.g[V0][0] != 0 {
                            if event_due(c) {
                                deliver_event_here(c, 0x0041_07b0);
                            }
                            continue 'match_;
                        }
                        c.cycles += 1;
                        c.g[A2][0] = c.lw(frame.at(c, 0x648));
                        break 'links;
                    }
                }
            }
            c.cycles += 3;
            c.g[V1][0] = c.lw(frame.at(c, 0x640));
            c.g[V0][0] = c.lw(c.g[A2][0] as u32);
            c.sw(c.g[V1][0] as u32, c.g[V0][0]);
        }
        c.cycles += 5;
        c.g[A2][0] = c.lw(frame.at(c, 0x640));
        c.g[V0][0] = c.lw(c.g[A2][0] as u32);
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        c.sw(c.g[A2][0] as u32, c.g[V1][0]);
        callee::SJTHROW.call(c, 0x0041_0800)?;
        c.pc = 0x0041_0800;
        (callee::EH_RTIME_MATCH.f)(c);
        Ok(())
    })
}

/// The `0x7e0` unlink block of `__sjpopnthrow`: drop the head handler
/// link into the frame slots before the walk.
pub(crate) fn sjpopn_unlink(c: &mut Cpu) {
    c.cycles += 15;
    c.sw(c.g[SP][0] as u32, c.g[V1][0]);
    c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0);
    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x8));
    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[SP][0]);
    c.g[V0][0] = sx(0x0041_0000u32.wrapping_add(0x790));
    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    c.sw((c.g[SP][0] as u32).wrapping_add(0x648), c.g[SP][0]);
    c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[SP][0]);
}

// ---------------------------------------------------------------------------
// __throw (0x00410f48)

/// `void __throw(void)` (0x410f48): throw the pending exception: resolve
/// the current frame state, snapshot the register window into the frame,
/// and run `throw_helper` to find the landing pad. Restores the saved
/// registers and returns to the handler (the generated restart/jump-out
/// tail is dead: the landing always returns).
pub fn throw_(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0x6f0);
        c.g[V1][0] = 0x450000;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x670), c.g[FP][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x680), c.g[RA][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x660), c.g[S7][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x650), c.g[S6][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x640), c.g[S5][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x630), c.g[S4][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x620), c.g[S3][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x610), c.g[S2][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x600), c.g[S1][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x5f0), c.g[S0][0]);
        c.swc1(31, (c.g[SP][0] as u32).wrapping_add(0x6e8));
        c.swc1(30, (c.g[SP][0] as u32).wrapping_add(0x6e0));
        c.swc1(29, (c.g[SP][0] as u32).wrapping_add(0x6d8));
        c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0x6d0));
        c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0x6c8));
        c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0x6c0));
        c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x6b8));
        c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x6b0));
        c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x6a8));
        c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x6a0));
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x698));
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x690));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3444));
        c.g[RA][0] = sx(0x00410fb4);
        c.g[FP][0] = c.g[SP][0];
        let target = c.g[V0][0] as u32;
        call::call_ptr(c, target, 0x0041_0fb4)?;
        c.sw((c.g[FP][0] as u32).wrapping_add(0x5e4), c.g[V0][0]);
        c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x2f0));
        c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x5e4));
        c.sw((c.g[FP][0] as u32).wrapping_add(0x5e8), c.g[V0][0]);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
        c.sw((c.g[FP][0] as u32).wrapping_add(0x5e0), c.g[FP][0]);
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            callee::TERMINATE.call(c, 0x0041_0fd8)?;
            c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x5e0));
            c.sw((c.g[FP][0] as u32).wrapping_add(0x5ec), c.g[A1][0]);
        } else {
            c.cycles += 2;
            c.g[A2][0] = sx((c.g[FP][0] as u32).wrapping_add(0x5e0));
            c.sw((c.g[FP][0] as u32).wrapping_add(0x5ec), c.g[A2][0]);
        }
        // The 0x1014 frame-state search.
        c.cycles += 7;
        c.g[A3][0] = 0x410000;
        c.g[A1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x5e0));
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(0xfec));
        callee::FRAME_STATE_FOR.call(c, 0x0041_0ffc)?;
        c.g[V1][0] = c.g[V0][0];
        c.sw((c.g[FP][0] as u32).wrapping_add(0x5e0), c.g[V0][0]);
        if c.g[V1][0] == 0 {
            c.cycles += 3;
            callee::TERMINATE.call(c, 0x0041_1010)?;
            c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x5e0));
        }
        // Snapshot the register window into the frame (0x1028 loop).
        c.cycles += 5;
        c.g[T0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x6f0));
        c.g[A0][0] = c.g[V1][0];
        c.sw(c.g[V1][0] as u32, c.g[T0][0]);
        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2e0));
        c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x5e8));
        'snapshot: loop {
            c.cycles += 13;
            c.g[A1][0] = c.ld(c.g[A0][0] as u32);
            c.g[A2][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x8));
            c.g[A3][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x10));
            c.g[T0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x18));
            c.sd(c.g[V0][0] as u32, c.g[A1][0]);
            c.sd((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
            c.sd((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A3][0]);
            c.sd((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T0][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
            if c.g[A0][0] == c.g[V1][0] {
                break 'snapshot;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0041_1028);
            }
        }
        // Run the landing-pad search on the snapshot.
        c.cycles += 23;
        c.g[V1][0] = c.ld(c.g[A0][0] as u32);
        c.g[A1][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x8));
        c.sd(c.g[V0][0] as u32, c.g[V1][0]);
        c.sd((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x680));
        c.g[A0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x5e4));
        c.g[A1][0] = c.lw(c.g[V0][0] as u32);
        c.g[A2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x5e8));
        c.g[A3][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x5ec));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        callee::THROW_HELPER.call(c, 0x0041_1088)?;
        // Land: restore the saved registers and return to the handler.
        c.g[A2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x5e0));
        c.g[V1][0] = 0x410000;
        c.g[T0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x5e8));
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10b8));
        c.g[V1][0] = c.lw(c.g[A2][0] as u32);
        c.g[A0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x680));
        c.g[A3][0] = c.lw(c.g[T0][0] as u32);
        c.sw(c.g[A0][0] as u32, c.g[A1][0]);
        c.g[A0][0] = c.g[V0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A3][0] as u32));
        c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x5e4));
        c.cycles += 25;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x680));
        c.g[FP][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x670));
        c.g[S7][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x660));
        c.g[S6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x650));
        c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x640));
        c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x630));
        c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x620));
        c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x610));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x600));
        c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x5f0));
        c.lwc1(31, (c.g[SP][0] as u32).wrapping_add(0x6e8));
        c.lwc1(30, (c.g[SP][0] as u32).wrapping_add(0x6e0));
        c.lwc1(29, (c.g[SP][0] as u32).wrapping_add(0x6d8));
        c.lwc1(28, (c.g[SP][0] as u32).wrapping_add(0x6d0));
        c.lwc1(27, (c.g[SP][0] as u32).wrapping_add(0x6c8));
        c.lwc1(26, (c.g[SP][0] as u32).wrapping_add(0x6c0));
        c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x6b8));
        c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x6b0));
        c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x6a8));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x6a0));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x698));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x690));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// __rethrow (0x00411128)

/// `void __rethrow(void)` (0x411128): rethrow the in-flight exception:
/// like `__throw`, but first stashes the incoming object (`a0`) as the
/// new `fp + 0x5e4` context and links the current handler address into
/// `+0xc` before resolving the frame state. Same dead restart tail.
pub fn rethrow(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x700);
        c.g[V1][0] = 0x450000;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x680), c.g[FP][0]);
        c.g[FP][0] = c.g[SP][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x690), c.g[RA][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x670), c.g[S7][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x660), c.g[S6][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x650), c.g[S5][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x640), c.g[S4][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x630), c.g[S3][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x620), c.g[S2][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x610), c.g[S1][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x600), c.g[S0][0]);
        c.swc1(31, (c.g[SP][0] as u32).wrapping_add(0x6f8));
        c.swc1(30, (c.g[SP][0] as u32).wrapping_add(0x6f0));
        c.swc1(29, (c.g[SP][0] as u32).wrapping_add(0x6e8));
        c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0x6e0));
        c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0x6d8));
        c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0x6d0));
        c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x6c8));
        c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x6c0));
        c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x6b8));
        c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x6b0));
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x6a8));
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x6a0));
        c.sw((c.g[FP][0] as u32).wrapping_add(0x5e4), c.g[A0][0]);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3444));
        c.g[RA][0] = sx(0x0041119c);
        let target = c.g[V0][0] as u32;
        call::call_ptr(c, target, 0x0041_119c)?;
        c.sw((c.g[FP][0] as u32).wrapping_add(0x5e8), c.g[V0][0]);
        c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x2f0));
        c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x5e8));
        c.sw((c.g[FP][0] as u32).wrapping_add(0x5ec), c.g[V0][0]);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
        c.sw((c.g[FP][0] as u32).wrapping_add(0x5e0), c.g[FP][0]);
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            callee::TERMINATE.call(c, 0x0041_11c0)?;
        }
        // The 0x11fc frame-state search with the handler link.
        c.cycles += 12;
        c.g[A2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x5e4));
        c.g[A3][0] = sx((c.g[FP][0] as u32).wrapping_add(0x5e0));
        c.g[A1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x5e8));
        c.sw((c.g[A1][0] as u32).wrapping_add(0xc), c.g[A2][0]);
        c.sw((c.g[FP][0] as u32).wrapping_add(0x5f0), c.g[A3][0]);
        c.g[T0][0] = 0x410000;
        c.g[A1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x5e0));
        c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x11d4));
        callee::FRAME_STATE_FOR.call(c, 0x0041_11e4)?;
        c.g[V1][0] = c.g[V0][0];
        c.sw((c.g[FP][0] as u32).wrapping_add(0x5e0), c.g[V0][0]);
        if c.g[V1][0] == 0 {
            c.cycles += 3;
            callee::TERMINATE.call(c, 0x0041_11f8)?;
            c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x5e0));
        }
        // Snapshot, search, land: as in `__throw`.
        c.cycles += 5;
        c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x700));
        c.g[A0][0] = c.g[V1][0];
        c.sw(c.g[V1][0] as u32, c.g[V0][0]);
        c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x5ec));
        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2e0));
        'snapshot: loop {
            c.cycles += 13;
            c.g[A1][0] = c.ld(c.g[A0][0] as u32);
            c.g[A2][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x8));
            c.g[A3][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x10));
            c.g[T0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x18));
            c.sd(c.g[V0][0] as u32, c.g[A1][0]);
            c.sd((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
            c.sd((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A3][0]);
            c.sd((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T0][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
            if c.g[A0][0] == c.g[V1][0] {
                break 'snapshot;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0041_1210);
            }
        }
        c.cycles += 23;
        c.g[V1][0] = c.ld(c.g[A0][0] as u32);
        c.g[A1][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x8));
        c.sd(c.g[V0][0] as u32, c.g[V1][0]);
        c.sd((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x690));
        c.g[A0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x5e8));
        c.g[A1][0] = c.lw(c.g[V0][0] as u32);
        c.g[A2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x5ec));
        c.g[A3][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x5f0));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        callee::THROW_HELPER.call(c, 0x0041_1270)?;
        c.g[A2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x5e0));
        c.g[V1][0] = 0x410000;
        c.g[T0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x5ec));
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x12a0));
        c.g[V1][0] = c.lw(c.g[A2][0] as u32);
        c.g[A0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x690));
        c.g[A3][0] = c.lw(c.g[T0][0] as u32);
        c.sw(c.g[A0][0] as u32, c.g[A1][0]);
        c.g[A0][0] = c.g[V0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A3][0] as u32));
        c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x5e8));
        c.cycles += 25;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x690));
        c.g[FP][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x680));
        c.g[S7][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x670));
        c.g[S6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x660));
        c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x650));
        c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x640));
        c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x630));
        c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x620));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x610));
        c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x600));
        c.lwc1(31, (c.g[SP][0] as u32).wrapping_add(0x6f8));
        c.lwc1(30, (c.g[SP][0] as u32).wrapping_add(0x6f0));
        c.lwc1(29, (c.g[SP][0] as u32).wrapping_add(0x6e8));
        c.lwc1(28, (c.g[SP][0] as u32).wrapping_add(0x6e0));
        c.lwc1(27, (c.g[SP][0] as u32).wrapping_add(0x6d8));
        c.lwc1(26, (c.g[SP][0] as u32).wrapping_add(0x6d0));
        c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x6c8));
        c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x6c0));
        c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x6b8));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x6b0));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x6a8));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x6a0));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// __eh_rtime_match (0x00410800)

/// `int __eh_rtime_match(void *type, void *info)` (0x410800): ask the
/// runtime matcher (`info->handler`) whether `type` matches; 1 when it
/// does, 0 when it does not. A null handler cell returns 0 without
/// calling.
pub fn eh_rtime_match(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        callee::GET_EH_INFO.call(c, 0x0041_0814)?;
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
        c.g[A1][0] = c.g[S0][0];
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 3;
            let target = c.g[V0][0] as u32;
            c.g[A2][0] = 0;
            call::call_ptr(c, target, 0x0041_0834)?;
            c.g[V0][0] = (0 < c.g[V0][0]) as u64;
        }
        c.cycles += 4;
        c.g[RA][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// __cplus_type_matcher (0x00412928)

/// `int __cplus_type_matcher(const void *a, const void *b, int sel,
/// table)` (0x412928): compare two type descriptors for the catch
/// matcher. A null `a2` (or a class tag of 4 at `a2 + 4`) takes the
/// descriptor path below; any other tag returns 0. Otherwise the
/// indirect matcher at `a1` runs, and a match calls
/// `__throw_type_match_rtti_2`.
pub fn cplus_type_matcher(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        if c.g[A2][0] != 0 {
            c.cycles += 4;
            c.g[V1][0] = c.lh((c.g[A2][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = 0x4;
            c.g[V0][0] = 0;
            if c.g[V1][0] != 4 {
                return found_matcher_result(c, frame, 0);
            }
        }
        // The 0x12964 descriptor path.
        c.cycles += 4;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[V0][0] |= 0xffff;
        if c.g[A1][0] != c.g[V0][0] {
            // Indirect matcher + RTTI re-match.
            c.cycles += 8;
            let target = c.g[A1][0] as u32;
            c.g[RA][0] = sx(0x0041_296c);
            call::call_ptr(c, target, 0x0041_296c)?;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
            callee::THROW_TYPE_MATCH_RTTI_2.call(c, 0x0041_297c)?;
            if c.g[V0][0] != 0 {
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        }
        found_matcher_result(c, frame, c.g[V0][0])
    })
}

/// Shared epilogue of `__cplus_type_matcher`: restore and return `v0`.
pub(crate) fn found_matcher_result(c: &mut Cpu, frame: Frame, v0: u64) -> call::Flow {
    c.cycles += 4;
    c.g[RA][0] = c.ld(frame.at(c, 0x10));
    c.g[S0][0] = c.ld(frame.at(c, 0));
    c.g[V0][0] = v0;
    let t = c.g[RA][0] as u32;
    frame.close(c);
    c.pc = t;
    Ok(())
}

// ---------------------------------------------------------------------------
// __cp_push_exception (0x00412998)

/// `void __cp_push_exception(void *obj, void *info, void *handler)`
/// (0x412998): push a catch-handler record for the in-flight exception:
/// allocate the record with `__eh_alloc`, fill the object/info links and
/// the handler slot, and link it into the thread's handler chain from
/// `__get_eh_info`.
pub fn cp_push_exception(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x50);
        c.sd(frame.at(c, 0x30), c.g[S3][0]);
        c.g[S3][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x20), c.g[S2][0]);
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.g[S2][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A2][0];
        c.sd(frame.at(c, 0), c.g[S0][0]);
        frame.save_ra(c, 0x40);
        c.g[A0][0] = 0x30;
        callee::EH_ALLOC.call(c, 0x0041_29c4)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = 0x4;
        c.g[V0][0] = 0x410000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[S2][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[S1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2928));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[S3][0]);
        c.g[V1][0] = 0x1;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[S3][0]);
        c.sd((c.g[S0][0] as u32).wrapping_add(0x20), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0);
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        c.sh((c.g[S0][0] as u32).wrapping_add(0x4), c.g[A0][0]);
        c.g[RA][0] = sx(0x0041_2a00);
        c.sh((c.g[S0][0] as u32).wrapping_add(0x6), c.g[V1][0]);
        GET_EH_INFO.call(c, 0x0041_2a00)?;
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        // The frame restore runs before the two stores (delay-slot
        // order): the stores cannot alias the frame, but keep it exact.
        c.g[RA][0] = c.ld(frame.at(c, 0x40));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
        c.sw(c.g[V0][0] as u32, c.g[S0][0]);
        c.g[S3][0] = c.ld(frame.at(c, 0x30));
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// __cp_pop_exception (0x00412a28)

/// `void __cp_pop_exception(void *obj)` (0x412a28): pop a catch-handler
/// record: decrement the handler's use count, and when it reaches zero
/// unlink it (running the cleanup/indirect destructor when present),
/// free the two record halves, and continue unwinding. Mismatched
/// handlers terminate.
pub fn cp_pop_exception(c: &mut Cpu) {
    call::guest(c, |c| {
        // Dispatcher: the entry plus the chain-walk head, so an event
        // unwind resumes where generated does (the frame stays open).
        let frame = Frame { size: 0x20 };
        let resumed = match c.pc {
            0x0041_2a28 => false,
            0x0041_2a80 => true,
            pc => {
                c.jump_out(pc, 0x0041_2a28, 0x0041_2aec);
                return Ok(());
            }
        };
        if resumed {
            return pop_chain_resume(c, frame);
        }
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        GET_EH_INFO.call(c, 0x0041_2a3c)?;
        c.g[V1][0] = c.ld((c.g[S0][0] as u32).wrapping_add(0x20));
        c.g[A0][0] = c.g[V0][0];
        c.g[V1][0] = c.g[V1][0].wrapping_sub(1);
        c.sd((c.g[S0][0] as u32).wrapping_add(0x20), c.g[V1][0]);
        if c.g[V1][0] != 0 {
            // Still referenced: just return (0x12adc/0x12ae0).
            c.cycles += 1;
            c.g[RA][0] = c.ld(frame.at(c, 0x10));
            c.cycles += 3;
            c.g[S0][0] = c.ld(frame.at(c, 0));
            let t = c.g[RA][0] as u32;
            frame.close(c);
            c.pc = t;
            return Ok(());
        }
        // Last use: check the head record (0x12a50).
        c.cycles += 3;
        c.g[V1][0] = c.lw(c.g[A0][0] as u32);
        if c.g[S0][0] != c.g[V1][0] {
            // Not the head: walk the chain or terminate (0x12a68).
            return pop_chain_or_terminate(c, frame);
        }
        // Head record: return early when no cleanup is pending (0x12a5c),
        // else walk for it below.
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.g[RA][0] = c.ld(frame.at(c, 0x10));
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[S0][0] = c.ld(frame.at(c, 0));
            let t = c.g[RA][0] as u32;
            frame.close(c);
            c.pc = t;
            return Ok(());
        }
        pop_chain_or_terminate(c, frame)
    })
}

/// The 0x12a68+ chain search of `__cp_pop_exception`: find `s0` in the
/// handler list (terminating on a null link or a mismatch), then destroy
/// the record halves below. Enters with `v1` = the head link.
pub(crate) fn pop_chain_or_terminate(c: &mut Cpu, frame: Frame) -> call::Flow {
    // The 0x12a68 head.
    c.cycles += 2;
    if c.g[V1][0] == 0 {
        // Null link: terminate, then destroy below (0x12a9c).
        return pop_terminate_destroy(c, frame);
    }
    // The 0x12a70 head-link check.
    c.cycles += 2;
    if c.g[V1][0] == c.g[S0][0] {
        return pop_splice(c, frame);
    }
    // The 0x12a78 first link (2 cycles; loop-backs use the 0x12a80
    // 1-cycle arm instead).
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
    pop_chain_loop(c, frame)
}

/// Resume the chain search at the 0x12a80 head after an event unwind:
/// the link is already in `v1`, one cycle reloads the next link.
pub(crate) fn pop_chain_resume(c: &mut Cpu, frame: Frame) -> call::Flow {
    c.cycles += 1;
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
    pop_chain_loop(c, frame)
}

/// The 0x12a84 chain-walk loop.
pub(crate) fn pop_chain_loop(c: &mut Cpu, frame: Frame) -> call::Flow {
    loop {
        // The 0x12a84 head.
        c.cycles += 2;
        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x18));
        if c.g[V0][0] == 0 {
            // Dead link: terminate, then destroy below (0x12a9c).
            return pop_terminate_destroy(c, frame);
        }
        // The 0x12a8c link check.
        c.cycles += 2;
        let found = c.g[V0][0] == c.g[S0][0];
        c.g[V1][0] = c.lw(c.g[A0][0] as u32);
        if !found {
            if back_edge_unwinds(c, 0x0041_2a80) {
                return Ok(());
            }
            // The 0x12a80 next link.
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
            continue;
        }
        return pop_splice(c, frame);
    }
}

/// The terminate-then-destroy edge of `__cp_pop_exception` (0x12a9c):
/// `terminate` (0x412800), then destroy below.
pub(crate) fn pop_terminate_destroy(c: &mut Cpu, frame: Frame) -> call::Flow {
    c.cycles += 2;
    c.g[RA][0] = sx(0x0041_2aa4);
    TERMINATE_CPP.call(c, 0x0041_2aa4)?;
    pop_destroy(c, frame)
}

/// The splice point of `__cp_pop_exception` (0x12a94): when the chain
/// continues (`v1` nonzero) load the bypass link and destroy below,
/// else terminate then destroy (0x12a9c).
pub(crate) fn pop_splice(c: &mut Cpu, frame: Frame) -> call::Flow {
    c.cycles += 2;
    if c.g[V1][0] == 0 {
        return pop_terminate_destroy(c, frame);
    }
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
    pop_destroy(c, frame)
}

/// The destroy tail of `__cp_pop_exception` (0x12aa4+): store the link,
/// run the record's indirect destructor when one is installed, free the
/// object half unless it is a pointer, free the record, and return.
pub(crate) fn pop_destroy(c: &mut Cpu, frame: Frame) -> call::Flow {
    c.cycles += 3;
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
    c.sw(c.g[A0][0] as u32, c.g[V0][0]);
    if c.g[V1][0] != 0 {
        c.cycles += 3;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        let target = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0041_2abc);
        c.g[A1][0] = 0x2;
        call::call_ptr(c, target, 0x0041_2abc)?;
    }
    c.cycles += 4;
    c.g[RA][0] = sx(0x0041_2ac4);
    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
    callee::IS_POINTER.call(c, 0x0041_2ac4)?;
    if c.g[V0][0] == 0 {
        // Not a pointer: free the object half, then the record below.
        c.cycles += 2;
        c.g[RA][0] = sx(0x0041_2ad4);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
        callee::EH_FREE.call(c, 0x0041_2ad4)?;
    }
    // Free the record.
    c.cycles += 2;
    c.g[A0][0] = c.g[S0][0];
    c.g[RA][0] = sx(0x0041_2adc);
    callee::EH_FREE.call(c, 0x0041_2adc)?;
    c.cycles += 1;
    c.g[RA][0] = c.ld(frame.at(c, 0x10));
    c.cycles += 3;
    c.g[S0][0] = c.ld(frame.at(c, 0));
    let t = c.g[RA][0] as u32;
    frame.close(c);
    c.pc = t;
    Ok(())
}

// ---------------------------------------------------------------------------
// __throw_bad_cast (0x00412cd8) / __throw_bad_typeid (0x00412d58)

/// Shared prologue of the bad-cast/bad-typeid throws: allocate the
/// 0x30-byte exception object with `__eh_alloc`, then tail-call the
/// generated setup trampoline, which stores the vtable, pushes a catch
/// handler and throws. The trampoline chain never returns normally
/// (there is no epilogue in the generated body either).
pub(crate) fn throw_bad_prologue(c: &mut Cpu, vtable_off: u32, ret: u32) -> call::Flow {
    c.cycles += 12;
    let frame = Frame::open(c, 0x50);
    c.g[V0][0] = 0x4a0000;
    c.sd(frame.at(c, 0x20), c.g[S1][0]);
    c.g[A0][0] = 0x4;
    c.sd(frame.at(c, 0x30), c.g[S2][0]);
    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(vtable_off));
    c.sd(frame.at(c, 0x40), c.g[RA][0]);
    c.g[S2][0] = 0x4a0000;
    c.sd(frame.at(c, 0x10), c.g[S0][0]);
    c.g[RA][0] = sx(ret);
    c.sw(frame.at(c, 0), c.g[S1][0]);
    callee::EH_ALLOC.call(c, ret)?;
    c.g[S0][0] = c.g[V0][0];
    Ok(())
}

/// `void __throw_bad_cast(void)` (0x412cd8): throw a `bad_cast`.
pub fn throw_bad_cast(c: &mut Cpu) {
    call::guest(c, |c| {
        throw_bad_prologue(c, 0x65d0, 0x0041_2d04)?;
        c.pc = 0x0041_2d08;
        (ee::s15::f_00412d08)(c);
        Ok(())
    })
}

/// `void __throw_bad_typeid(void)` (0x412d58): throw a `bad_typeid`.
pub fn throw_bad_typeid(c: &mut Cpu) {
    call::guest(c, |c| {
        throw_bad_prologue(c, 0x65e8, 0x0041_2d84)?;
        c.pc = 0x0041_2d88;
        (ee::s15::f_00412d88)(c);
        Ok(())
    })
}

/// `void __check_eh_spec(const void *list, int n)` (0x412b20): verify the
/// thrown exception against the function exception specification. Kept in the
/// generated dispatcher shape.
pub fn check_eh_spec(c: &mut Cpu) {
    call::guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0041_2b20 => {
                    c.cycles += 13;
                    let frame = Frame::open(c, 0x80);
                    c.sd(frame.at(c, 0x60), c.g[S5][0]);
                    c.sd(frame.at(c, 0x40), c.g[S3][0]);
                    c.g[S5][0] = c.g[A1][0];
                    c.sd(frame.at(c, 0x30), c.g[S2][0]);
                    c.g[S3][0] = c.g[A0][0];
                    c.sd(frame.at(c, 0x70), c.g[RA][0]);
                    c.sd(frame.at(c, 0x50), c.g[S4][0]);
                    c.sd(frame.at(c, 0x20), c.g[S1][0]);
                    c.g[RA][0] = sx(0x0041_2b4c);
                    c.sd(frame.at(c, 0x10), c.g[S0][0]);
                    GET_EH_INFO.call(c, 0x0041_2b4c)?;
                    let t = (c.g[S3][0] as i64) <= 0;
                    c.g[S2][0] = c.lw(c.g[V0][0] as u32);
                    if t {
                        pc = 0x0041_2b90;
                        continue;
                    }
                    pc = 0x0041_2b54;
                    continue;
                }
                0x0041_2b54 => {
                    c.cycles += 3;
                    c.g[S1][0] = c.g[S5][0];
                    c.g[S0][0] = c.g[S3][0];
                    pc = 0x0041_2b60;
                    continue;
                }
                0x0041_2b60 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                    c.g[RA][0] = sx(0x0041_2b70);
                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    callee::THROW_TYPE_MATCH_RTTI_2.call(c, 0x0041_2b70)?;
                    if c.g[V0][0] == 0u64 {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                        pc = 0x0041_2b88;
                        continue;
                    }
                    pc = 0x0041_2b78;
                    continue;
                }
                0x0041_2b78 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0041_2b80);
                    callee::UNCATCH_EXCEPTION.call(c, 0x0041_2b80)?;
                    c.g[RA][0] = sx(0x0041_2b88);
                    callee::THROW.call(c, 0x0041_2b88)?;
                    pc = 0x0041_2b88;
                    continue;
                }
                0x0041_2b88 => {
                    c.cycles += 2;
                    let t = c.g[S0][0] != 0u64;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x0041_2b60) {
                            return Ok(());
                        }
                        pc = 0x0041_2b60;
                        continue;
                    }
                    pc = 0x0041_2b90;
                    continue;
                }
                0x0041_2b90 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0041_2b98);
                    callee::CP_EXCEPTION_INFO.call(c, 0x0041_2b98)?;
                    pc = 0x0041_2b98;
                    continue;
                }
                0x0041_2b98 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0041_2ba0);
                    callee::THROW.call(c, 0x0041_2ba0)?;
                    pc = 0x0041_2ba0;
                    continue;
                }
                0x0041_2ba0 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x0041_2ba8);
                    callee::START_CP_HANDLER.call(c, 0x0041_2ba8)?;
                    c.g[S4][0] = c.g[V0][0];
                    pc = 0x0041_2bac;
                    continue;
                }
                0x0041_2bac => {
                    c.cycles += 2;
                    let t = c.g[S2][0] == c.g[S4][0];
                    if t {
                        pc = 0x0041_2bf8;
                        continue;
                    }
                    pc = 0x0041_2bb4;
                    continue;
                }
                0x0041_2bb4 => {
                    c.cycles += 2;
                    let t = (c.g[S3][0] as i64) <= 0;
                    c.g[S2][0] = c.g[S4][0];
                    if t {
                        pc = 0x0041_2bf8;
                        continue;
                    }
                    pc = 0x0041_2bbc;
                    continue;
                }
                0x0041_2bbc => {
                    c.cycles += 3;
                    c.g[S1][0] = c.g[S5][0];
                    c.g[S0][0] = c.g[S3][0];
                    pc = 0x0041_2bc8;
                    continue;
                }
                0x0041_2bc8 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                    c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xc));
                    c.g[RA][0] = sx(0x0041_2bd8);
                    c.g[A2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x8));
                    callee::THROW_TYPE_MATCH_RTTI_2.call(c, 0x0041_2bd8)?;
                    if c.g[V0][0] == 0u64 {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                        pc = 0x0041_2bf0;
                        continue;
                    }
                    pc = 0x0041_2be0;
                    continue;
                }
                0x0041_2be0 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0041_2be8);
                    callee::UNCATCH_EXCEPTION.call(c, 0x0041_2be8)?;
                    c.g[RA][0] = sx(0x0041_2bf0);
                    callee::THROW.call(c, 0x0041_2bf0)?;
                    pc = 0x0041_2bf0;
                    continue;
                }
                0x0041_2bf0 => {
                    c.cycles += 2;
                    let t = c.g[S0][0] != 0u64;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x0041_2bc8) {
                            return Ok(());
                        }
                        pc = 0x0041_2bc8;
                        continue;
                    }
                    pc = 0x0041_2bf8;
                    continue;
                }
                0x0041_2bf8 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0041_2c00);
                    c.g[S0][0] = 0u64;
                    callee::TF13BAD_EXCEPTION.call(c, 0x0041_2c00)?;
                    let t = (c.g[S3][0] as i64) <= 0;
                    c.g[S1][0] = c.g[V0][0];
                    if t {
                        pc = 0x0041_2c98;
                        continue;
                    }
                    pc = 0x0041_2c08;
                    continue;
                }
                0x0041_2c08 => {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                    pc = 0x0041_2c10;
                    continue;
                }
                0x0041_2c10 => {
                    c.cycles += 7;
                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x0041_2c24);
                    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                    callee::THROW_TYPE_MATCH_RTTI_2.call(c, 0x0041_2c24)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x0041_2c88;
                        continue;
                    }
                    pc = 0x0041_2c2c;
                    continue;
                }
                0x0041_2c2c => {
                    c.cycles += 6;
                    c.g[S2][0] = 0x4a0000;
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6760));
                    c.g[A0][0] = 0x4;
                    c.g[RA][0] = sx(0x0041_2c40);
                    c.sw(Frame { size: 0x80 }.at(c, 0), c.g[S1][0]);
                    callee::EH_ALLOC.call(c, 0x0041_2c40)?;
                    c.g[S0][0] = c.g[V0][0];
                    pc = 0x0041_2c44;
                    continue;
                }
                0x0041_2c44 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw(c.g[S0][0] as u32, c.g[S1][0]);
                    if t {
                        pc = 0x0041_2c54;
                        continue;
                    }
                    pc = 0x0041_2c4c;
                    continue;
                }
                0x0041_2c4c => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0041_2c54);
                    callee::THROW.call(c, 0x0041_2c54)?;
                    pc = 0x0041_2c54;
                    continue;
                }
                0x0041_2c54 => {
                    c.cycles += 13;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x6748));
                    c.sw(Frame { size: 0x80 }.at(c, 0), c.g[V0][0]);
                    c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0041_2c6c);
                    call::call_ptr(c, t, 0x0041_2c6c)?;
                    c.g[A2][0] = 0x410000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0041_2c80);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2e38));
                    callee::CP_PUSH_EXCEPTION.call(c, 0x0041_2c80)?;
                    c.g[RA][0] = sx(0x0041_2c88);
                    callee::THROW.call(c, 0x0041_2c88)?;
                    pc = 0x0041_2c88;
                    continue;
                }
                0x0041_2c88 => {
                    c.cycles += 4;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S3][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                    if t {
                        if back_edge_unwinds(c, 0x0041_2c10) {
                            return Ok(());
                        }
                        pc = 0x0041_2c10;
                        continue;
                    }
                    pc = 0x0041_2c98;
                    continue;
                }
                0x0041_2c98 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0041_2ca0);
                    TERMINATE_CPP.call(c, 0x0041_2ca0)?;
                    pc = 0x0041_2ca0;
                    continue;
                }
                0x0041_2ca0 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0041_2ca8);
                    callee::THROW.call(c, 0x0041_2ca8)?;
                    pc = 0x0041_2ca8;
                    continue;
                }
                0x0041_2ca8 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x6748));
                    let t = true;
                    c.sw(Frame { size: 0x80 }.at(c, 0), c.g[V0][0]);
                    if t {
                        if back_edge_unwinds(c, 0x0041_2c4c) {
                            return Ok(());
                        }
                        pc = 0x0041_2c4c;
                        continue;
                    }
                }
                _ => {
                    c.jump_out(pc, 0x0041_2b20, 0x0041_2cb4);
                    return Ok(());
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// __pack_f (0x00412f58)

/// `float __pack_f(const fp_number_type *src)` (0x412f58, libgcc
/// soft-float, the single-precision inverse of `__unpack_f`): builds the
/// IEEE float bits from the unpacked class, sign, unbiased exponent and
/// fraction. Kept in the generated block shape.
pub fn pack_f(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = c.lw(c.g[A0][0] as u32);
    c.g[A3][0] = 0u64;
    c.g[T0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = (c.g[V1][0] < 0x2u64) as u64;
    let t = c.g[V0][0] == 0u64;
    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
    // 0x413018: only the V1^4 == 0 path skips the shared mid join below
    // (0x413014); the round tail (0x413010) runs its own join first.
    let mut mid_join = true;
    if !t {
        c.cycles += 4;
        c.g[V0][0] = 0x100000;
        c.g[A3][0] = 0xff;
        c.g[A1][0] = c.g[A1][0] | c.g[V0][0];
    } else {
        c.cycles += 3;
        c.g[V0][0] = c.g[V1][0] ^ 0x4;
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = c.g[V1][0] ^ 0x2;
        if !t {
            c.cycles += 2;
            let t = c.g[V0][0] != 0u64;
            if !t {
                c.cycles += 2;
                c.g[A1][0] = 0u64;
            } else {
                c.cycles += 2;
                let t = c.g[A1][0] == 0u64;
                c.g[V1][0] = 0xffffffffff800000;
                if t {
                    mid_join = false;
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < -126i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0xffffffffffffff82;
                    if !t {
                        c.cycles += 4;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                        c.g[V1][0] = ((c.g[V0][0] as i64) < 26i64) as u64;
                        let t = c.g[V1][0] != 0u64;
                        c.g[A1][0] = sx((c.g[A1][0] as u32) >> ((c.g[V0][0] as u32) & 31));
                        if !t {
                            c.cycles += 2;
                            c.g[A1][0] = 0u64;
                        }
                        c.cycles += 1;
                        c.g[A1][0] = sx((c.g[A1][0] as u32) >> 7);
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[V1][0] as i64) < 128i64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x7f));
                        if t {
                            c.cycles += 4;
                            c.g[V0][0] = 0x40;
                            c.g[V1][0] = c.g[A1][0] & 0x7f;
                            if c.g[V1][0] != c.g[V0][0] {
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3f));
                            } else {
                                c.cycles += 3;
                                c.g[V1][0] = c.g[A1][0] & 0x80;
                                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40));
                                if c.g[V1][0] != 0 {
                                    c.g[A1][0] = c.g[V0][0];
                                }
                            }
                            c.cycles += 2;
                            if (c.g[A1][0] as i64) >= 0 {
                                c.g[A1][0] = sx((c.g[A1][0] as u32) >> 7);
                            } else {
                                c.cycles += 2;
                                c.g[A1][0] = sx((c.g[A1][0] as u32) >> 1);
                                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                                c.cycles += 1;
                                c.g[A1][0] = sx((c.g[A1][0] as u32) >> 7);
                            }
                        } else {
                            c.cycles += 3;
                            c.g[A3][0] = 0xff;
                            c.g[A1][0] = 0u64;
                        }
                    }
                }
            }
        } else {
            c.cycles += 3;
            c.g[A3][0] = 0xff;
            c.g[A1][0] = 0u64;
        }
    }
    if mid_join {
        c.cycles += 1;
        c.g[V1][0] = 0xffffffffff800000;
    }
    c.cycles += 19;
    c.g[V0][0] = 0x7f0000;
    c.g[A2][0] = c.g[A2][0] & c.g[V1][0];
    c.g[V0][0] = c.g[V0][0] | 0xffff;
    c.g[V0][0] = c.g[A1][0] & c.g[V0][0];
    c.g[V1][0] = 0xffffffff807f0000;
    c.g[A2][0] = c.g[A2][0] | c.g[V0][0];
    c.g[V1][0] = c.g[V1][0] | 0xffff;
    c.g[A0][0] = c.g[A3][0] & 0xff;
    c.g[A2][0] = c.g[A2][0] & c.g[V1][0];
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 23);
    c.g[V0][0] = 0x7fff0000;
    c.g[A2][0] = c.g[A2][0] | c.g[A0][0];
    c.g[V0][0] = c.g[V0][0] | 0xffff;
    c.g[V1][0] = sx((c.g[T0][0] as u32) << 31);
    c.g[A2][0] = c.g[A2][0] & c.g[V0][0];
    c.g[A2][0] = c.g[A2][0] | c.g[V1][0];
    c.mtc1(6, 0);
    let t = c.g[RA][0] as u32;
    c.pc = t;
}

// ---------------------------------------------------------------------------
// __addsf3 (0x00413330) / __subsf3 (0x00413388)

/// `float __addsf3(float a, float b)` (0x413330, libgcc soft-float):
/// unpack both operands, add with `_fpadd_parts_f`, pack the result.
pub fn addsf3(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x60);
        c.sd(frame.at(c, 0x40), c.g[S0][0]);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        frame.save_ra(c, 0x50);
        c.g[A1][0] = c.g[SP][0];
        c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = sx(0x0041_3350);
        c.swc1(13, (c.g[SP][0] as u32).wrapping_add(0x34));
        UNPACK_F.call(c, 0x0041_3350)?;
        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x34));
        c.g[RA][0] = sx(0x0041_3360);
        c.g[A1][0] = c.g[S0][0];
        UNPACK_F.call(c, 0x0041_3360)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = sx(0x0041_3370);
        c.g[A0][0] = c.g[SP][0];
        callee::FPADD_PARTS_F.call(c, 0x0041_3370)?;
        c.g[RA][0] = sx(0x0041_3378);
        c.g[A0][0] = c.g[V0][0];
        PACK_F.call(c, 0x0041_3378)?;
        c.g[RA][0] = c.ld(frame.at(c, 0x50));
        c.g[S0][0] = c.ld(frame.at(c, 0x40));
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        Ok(())
    })
}

/// `float __subsf3(float a, float b)` (0x413388, libgcc soft-float):
/// unpack both operands, flip the sign of `b`, add with
/// `_fpadd_parts_f`, pack the result.
pub fn subsf3(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x60);
        c.sd(frame.at(c, 0x40), c.g[S0][0]);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        frame.save_ra(c, 0x50);
        c.g[A1][0] = c.g[SP][0];
        c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = sx(0x0041_33a8);
        c.swc1(13, (c.g[SP][0] as u32).wrapping_add(0x34));
        UNPACK_F.call(c, 0x0041_33a8)?;
        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x34));
        c.g[RA][0] = sx(0x0041_33b8);
        c.g[A1][0] = c.g[S0][0];
        UNPACK_F.call(c, 0x0041_33b8)?;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[A0][0] = c.g[SP][0];
        c.g[V0][0] ^= 0x1;
        c.g[RA][0] = sx(0x0041_33d4);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V0][0]);
        callee::FPADD_PARTS_F.call(c, 0x0041_33d4)?;
        c.g[RA][0] = sx(0x0041_33dc);
        c.g[A0][0] = c.g[V0][0];
        PACK_F.call(c, 0x0041_33dc)?;
        c.g[RA][0] = c.ld(frame.at(c, 0x50));
        c.g[S0][0] = c.ld(frame.at(c, 0x40));
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// __mulsf3 (0x004133f0)

/// `float __mulsf3(float a, float b)` (0x4133f0, libgcc soft-float): unpack both
/// operands, multiply the fractions, round and pack. Kept in the generated block
/// shape.
pub fn mulsf3(c: &mut Cpu) {
    call::guest(c, |c| {
        let frame = Frame::open(c, 0x60);
        c.cycles += 16;
        c.sd(frame.at(c, 0x40), c.g[S0][0]);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        frame.save_ra(c, 0x50);
        c.g[A1][0] = c.g[SP][0];
        c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = sx(0x00413410);
        c.swc1(13, (c.g[SP][0] as u32).wrapping_add(0x34));
        UNPACK_F.call(c, 0x00413410)?;
        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x34));
        c.g[RA][0] = sx(0x00413420);
        c.g[A1][0] = c.g[S0][0];
        UNPACK_F.call(c, 0x00413420)?;
        c.g[A0][0] = c.lw(c.g[SP][0] as u32);
        c.g[V0][0] = (c.g[A0][0] < 0x2u64) as u64;
        let t = c.g[V0][0] != 0u64;
        c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        if t {
            c.cycles += 2;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = c.g[SP][0];
            c.cycles += 5;
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
            c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
            c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            c.cycles += 6;
            c.g[RA][0] = sx(0x004135d4);
            PACK_F.call(c, 0x004135d4)?;
            c.g[RA][0] = c.ld(frame.at(c, 0x50));
            c.g[S0][0] = c.ld(frame.at(c, 0x40));
            let t = c.g[RA][0] as u32;
            frame.close(c);
            c.pc = t;
            return Ok(());
        } else {
            c.cycles += 4;
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = (c.g[V1][0] < 0x2u64) as u64;
            if c.g[V0][0] != 0u64 {
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
                c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                c.cycles += 6;
                c.g[RA][0] = sx(0x004135d4);
                PACK_F.call(c, 0x004135d4)?;
                c.g[RA][0] = c.ld(frame.at(c, 0x50));
                c.g[S0][0] = c.ld(frame.at(c, 0x40));
                let t = c.g[RA][0] as u32;
                frame.close(c);
                c.pc = t;
                return Ok(());
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.g[A0][0] ^ 0x4;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = c.g[V1][0] ^ 0x4;
                if !t {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V1][0] ^ 0x2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    if t {
                        c.cycles += 3;
                        c.g[V0][0] = 0x530000;
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5228));
                        c.cycles += 6;
                        c.g[RA][0] = sx(0x004135d4);
                        PACK_F.call(c, 0x004135d4)?;
                        c.g[RA][0] = c.ld(frame.at(c, 0x50));
                        c.g[S0][0] = c.ld(frame.at(c, 0x40));
                        let t = c.g[RA][0] as u32;
                        frame.close(c);
                        c.pc = t;
                        return Ok(());
                    } else {
                        c.cycles += 2;
                        c.g[A0][0] = c.g[SP][0];
                        c.cycles += 5;
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                        c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
                        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                        c.cycles += 6;
                        c.g[RA][0] = sx(0x004135d4);
                        PACK_F.call(c, 0x004135d4)?;
                        c.g[RA][0] = c.ld(frame.at(c, 0x50));
                        c.g[S0][0] = c.ld(frame.at(c, 0x40));
                        let t = c.g[RA][0] as u32;
                        frame.close(c);
                        c.pc = t;
                        return Ok(());
                    }
                }
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = c.g[A0][0] ^ 0x2;
                if t {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.g[V1][0] ^ 0x2;
                    if t {
                        c.cycles += 2;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        if !t {
                            c.cycles += 1;
                            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                            c.cycles += 6;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
                            c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                            c.cycles += 6;
                            c.g[RA][0] = sx(0x004135d4);
                            PACK_F.call(c, 0x004135d4)?;
                            c.g[RA][0] = c.ld(frame.at(c, 0x50));
                            c.g[S0][0] = c.ld(frame.at(c, 0x40));
                            let t = c.g[RA][0] as u32;
                            frame.close(c);
                            c.pc = t;
                            return Ok(());
                        }
                        // t_c-true falls through past pure closes and
                        // auto-skipped else arms to the multu tail.
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                        c.g[A0][0] = c.g[SP][0];
                        c.cycles += 5;
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                        c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
                        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                        c.cycles += 6;
                        c.g[RA][0] = sx(0x004135d4);
                        PACK_F.call(c, 0x004135d4)?;
                        c.g[RA][0] = c.ld(frame.at(c, 0x50));
                        c.g[S0][0] = c.ld(frame.at(c, 0x40));
                        let t = c.g[RA][0] as u32;
                        frame.close(c);
                        c.pc = t;
                        return Ok(());
                    }
                } else {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                    if t {
                        c.cycles += 6;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
                        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                        c.cycles += 6;
                        c.g[RA][0] = sx(0x004135d4);
                        PACK_F.call(c, 0x004135d4)?;
                        c.g[RA][0] = c.ld(frame.at(c, 0x50));
                        c.g[S0][0] = c.ld(frame.at(c, 0x40));
                        let t = c.g[RA][0] as u32;
                        frame.close(c);
                        c.pc = t;
                        return Ok(());
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x530000;
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5228));
                        c.cycles += 6;
                        c.g[RA][0] = sx(0x004135d4);
                        PACK_F.call(c, 0x004135d4)?;
                        c.g[RA][0] = c.ld(frame.at(c, 0x50));
                        c.g[S0][0] = c.ld(frame.at(c, 0x40));
                        let t = c.g[RA][0] as u32;
                        frame.close(c);
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
        c.cycles += 22;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
        c.multu(0, 0, 4, 2);
        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.g[V0][0] ^ c.g[A2][0];
        c.g[A1][0] = c.lo[0];
        c.g[A0][0] = c.hi[0];
        c.g[A1][0] = c.g[A1][0] << 32;
        c.g[A0][0] = c.g[A0][0] << 32;
        c.g[A1][0] = c.g[A1][0] >> 32;
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        c.g[A0][0] = c.g[A0][0] | c.g[A1][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[V0][0]);
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
        c.g[A1][0] = c.g[A0][0] << 32;
        c.g[A1][0] = ((c.g[A1][0] as i64) >> 32) as u64;
        c.g[A0][0] = ((c.g[A0][0] as i64) >> 32) as u64;
        let t = (c.g[A0][0] as i64) >= 0;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x28), c.g[V1][0]);
        if !t {
            c.cycles += 3;
            c.g[A2][0] = 0xffffffff80000000;
            c.g[V0][0] = c.g[A0][0] & 0x1;
            loop {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                if !t {
                    c.cycles += 2;
                    c.g[A1][0] = sx((c.g[A1][0] as u32) >> 1);
                    c.g[A1][0] = c.g[A1][0] | c.g[A2][0];
                }
                c.cycles += 3;
                c.g[A0][0] = sx((c.g[A0][0] as u32) >> 1);
                let t = (c.g[A0][0] as i64) < 0;
                c.g[V0][0] = c.g[A0][0] & 0x1;
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00413528);
                    }
                } else {
                    c.cycles += 1;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x28), c.g[V1][0]);
                    break;
                }
            }
        }
        c.cycles += 5;
        c.g[V0][0] = 0x3fff0000;
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        c.g[V0][0] = (c.g[V0][0] < c.g[A0][0]) as u64;
        let t = c.g[V0][0] != 0u64;
        c.g[V1][0] = c.g[A0][0] & 0x7f;
        if !t {
            c.cycles += 5;
            c.g[A3][0] = 0x3fff0000;
            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
            c.g[T0][0] = 0xffffffff80000000;
            c.g[A3][0] = c.g[A3][0] | 0xffff;
            loop {
                c.cycles += 8;
                c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
                c.g[V1][0] = c.g[A1][0] & c.g[T0][0];
                c.g[V0][0] = c.g[A0][0] | 0x1;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                if c.g[V1][0] != 0 {
                    c.g[A0][0] = c.g[V0][0];
                }
                c.g[V0][0] = (c.g[A3][0] < c.g[A0][0]) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32) << 1);
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00413570);
                    }
                } else {
                    c.cycles += 2;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x28), c.g[A2][0]);
                    c.g[V1][0] = c.g[A0][0] & 0x7f;
                    break;
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = 0x40;
        if c.g[V1][0] != c.g[V0][0] {
            c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), c.g[A0][0]);
            c.cycles += 3;
            c.g[V0][0] = 0x3;
            c.sw(c.g[T1][0] as u32, c.g[V0][0]);
            c.g[A0][0] = c.g[T1][0];
            c.cycles += 6;
            c.g[RA][0] = sx(0x004135d4);
            PACK_F.call(c, 0x004135d4)?;
            c.g[RA][0] = c.ld(frame.at(c, 0x50));
            c.g[S0][0] = c.ld(frame.at(c, 0x40));
            let t = c.g[RA][0] as u32;
            frame.close(c);
            c.pc = t;
            return Ok(());
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.g[A0][0] & 0x80;
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
            if !t {
                c.cycles += 2;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
                c.cycles += 1;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), c.g[A0][0]);
                c.cycles += 3;
                c.g[V0][0] = 0x3;
                c.sw(c.g[T1][0] as u32, c.g[V0][0]);
                c.g[A0][0] = c.g[T1][0];
                c.cycles += 6;
                c.g[RA][0] = sx(0x004135d4);
                PACK_F.call(c, 0x004135d4)?;
                c.g[RA][0] = c.ld(frame.at(c, 0x50));
                c.g[S0][0] = c.ld(frame.at(c, 0x40));
                let t = c.g[RA][0] as u32;
                frame.close(c);
                c.pc = t;
                return Ok(());
            }
            c.cycles += 1;
            if c.g[A1][0] != 0 {
                c.g[A0][0] = c.g[V0][0];
            }
            c.cycles += 1;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), c.g[A0][0]);
            c.cycles += 3;
            c.g[V0][0] = 0x3;
            c.sw(c.g[T1][0] as u32, c.g[V0][0]);
            c.g[A0][0] = c.g[T1][0];
            c.cycles += 6;
            c.g[RA][0] = sx(0x004135d4);
            PACK_F.call(c, 0x004135d4)?;
            c.g[RA][0] = c.ld(frame.at(c, 0x50));
            c.g[S0][0] = c.ld(frame.at(c, 0x40));
            let t = c.g[RA][0] as u32;
            frame.close(c);
            c.pc = t;
            return Ok(());
        }
    })
}

// ---------------------------------------------------------------------------
// __divsf3 (0x004135e8)

/// `float __divsf3(float a, float b)` (0x4135e8, libgcc soft-float): unpack both
/// operands, divide the fractions, round and pack. Kept in the generated dispatcher
/// shape.
pub fn divsf3(c: &mut Cpu) {
    call::guest(c, |c| {
        let mut pc: u32 = c.pc;
        let frame = Frame { size: 0x50 };
        loop {
            match pc {
                0x0041_35e8 => {
                    c.cycles += 16;
                    Frame::open(c, 0x50);
                    c.sd(frame.at(c, 0x30), c.g[S0][0]);
                    c.g[A0][0] = sx(frame.at(c, 0x20));
                    c.sd(frame.at(c, 0x40), c.g[RA][0]);
                    c.g[A1][0] = sx(frame.at(c, 0));
                    c.swc1(12, frame.at(c, 0x20));
                    c.g[RA][0] = sx(0x0041_3608);
                    c.swc1(13, frame.at(c, 0x24));
                    UNPACK_F.call(c, 0x0041_3608)?;
                    c.g[S0][0] = sx(frame.at(c, 0x10));
                    c.g[A0][0] = sx(frame.at(c, 0x24));
                    c.g[RA][0] = sx(0x0041_3618);
                    c.g[A1][0] = c.g[S0][0];
                    UNPACK_F.call(c, 0x0041_3618)?;
                    c.g[A3][0] = c.lw(frame.at(c, 0));
                    c.g[V0][0] = (c.g[A3][0] < 0x2u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = sx(frame.at(c, 0));
                    if t {
                        pc = 0x0041_3630;
                        continue;
                    }
                    pc = 0x0041_3628;
                    continue;
                }
                0x0041_3628 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = sx(frame.at(c, 0));
                    if t {
                        pc = 0x0041_3730;
                        continue;
                    }
                    pc = 0x0041_3630;
                    continue;
                }
                0x0041_3630 => {
                    c.cycles += 4;
                    c.g[A2][0] = c.lw(frame.at(c, 0x10));
                    c.g[V0][0] = (c.g[A2][0] < 0x2u64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        pc = 0x0041_3730;
                        continue;
                    }
                    pc = 0x0041_3640;
                    continue;
                }
                0x0041_3640 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw(frame.at(c, 0x4));
                    c.g[A0][0] = c.g[A3][0] ^ 0x4;
                    c.g[V1][0] = c.lw(frame.at(c, 0x14));
                    c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
                    let t = c.g[A0][0] == 0u64;
                    c.sw(frame.at(c, 0x4), c.g[V0][0]);
                    if t {
                        pc = 0x0041_3664;
                        continue;
                    }
                    pc = 0x0041_3658;
                    continue;
                }
                0x0041_3658 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[A3][0] ^ 0x2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.g[A2][0] ^ 0x4;
                    if t {
                        pc = 0x0041_3678;
                        continue;
                    }
                    pc = 0x0041_3664;
                    continue;
                }
                0x0041_3664 => {
                    c.cycles += 2;
                    let t = c.g[A3][0] != c.g[A2][0];
                    c.g[A0][0] = sx(frame.at(c, 0));
                    if t {
                        pc = 0x0041_3730;
                        continue;
                    }
                    pc = 0x0041_366c;
                    continue;
                }
                0x0041_366c => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x530000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5228));
                    if t {
                        pc = 0x0041_3730;
                        continue;
                    }
                    pc = 0x0041_3678;
                    continue;
                }
                0x0041_3678 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.g[A2][0] ^ 0x2;
                    if t {
                        pc = 0x0041_3690;
                        continue;
                    }
                    pc = 0x0041_3680;
                    continue;
                }
                0x0041_3680 => {
                    c.cycles += 4;
                    c.sw(frame.at(c, 0xc), 0u64);
                    c.g[A0][0] = sx(frame.at(c, 0));
                    let t = true;
                    c.sw(frame.at(c, 0x8), 0u64);
                    if t {
                        pc = 0x0041_3730;
                        continue;
                    }
                    pc = 0x0041_3690;
                    continue;
                }
                0x0041_3690 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = c.lw(frame.at(c, 0x8));
                    if t {
                        pc = 0x0041_36a8;
                        continue;
                    }
                    pc = 0x0041_3698;
                    continue;
                }
                0x0041_3698 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x4;
                    c.g[A0][0] = sx(frame.at(c, 0));
                    let t = true;
                    c.sw(c.g[SP][0] as u32, c.g[V0][0]);
                    if t {
                        pc = 0x0041_3730;
                        continue;
                    }
                    pc = 0x0041_36a8;
                    continue;
                }
                0x0041_36a8 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw(frame.at(c, 0x18));
                    c.g[A0][0] = c.lw(frame.at(c, 0xc));
                    c.g[T0][0] = c.lw(frame.at(c, 0x1c));
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[A2][0] = (c.g[A0][0] < c.g[T0][0]) as u64;
                    let t = c.g[A2][0] == 0u64;
                    c.sw(frame.at(c, 0x8), c.g[V0][0]);
                    if t {
                        pc = 0x0041_36d4;
                        continue;
                    }
                    pc = 0x0041_36c4;
                    continue;
                }
                0x0041_36c4 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
                    c.sw(frame.at(c, 0x8), c.g[V0][0]);
                    c.g[A2][0] = (c.g[A0][0] < c.g[T0][0]) as u64;
                    pc = 0x0041_36d4;
                    continue;
                }
                0x0041_36d4 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x40000000;
                    let t = true;
                    c.g[A3][0] = 0u64;
                    if t {
                        pc = 0x0041_36e4;
                        continue;
                    }
                    pc = 0x0041_36e0;
                    continue;
                }
                0x0041_36e0 => {
                    c.cycles += 1;
                    c.g[A2][0] = (c.g[A0][0] < c.g[T0][0]) as u64;
                    pc = 0x0041_36e4;
                    continue;
                }
                0x0041_36e4 => {
                    c.cycles += 2;
                    if c.g[A2][0] != 0u64 {
                        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
                        pc = 0x0041_36f8;
                        continue;
                    }
                    pc = 0x0041_36ec;
                    continue;
                }
                0x0041_36ec => {
                    c.cycles += 3;
                    c.g[A3][0] = c.g[A3][0] | c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[T0][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
                    pc = 0x0041_36f8;
                    continue;
                }
                0x0041_36f8 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
                    if t {
                        if back_edge_unwinds(c, 0x0041_36e0) {
                            return Ok(());
                        }
                        pc = 0x004136e0;
                        continue;
                    }
                    pc = 0x0041_3700;
                    continue;
                }
                0x0041_3700 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.g[A3][0] & 0x7f;
                    c.g[V0][0] = 0x40;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.sw((c.g[A1][0] as u32).wrapping_add(0xc), c.g[A3][0]);
                        pc = 0x0041_372c;
                        continue;
                    }
                    pc = 0x0041_3710;
                    continue;
                }
                0x0041_3710 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[A3][0] & 0x80;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x40));
                    if t {
                        pc = 0x0041_3724;
                        continue;
                    }
                    pc = 0x0041_371c;
                    continue;
                }
                0x0041_371c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x40));
                    if t {
                        pc = 0x0041_3728;
                        continue;
                    }
                    pc = 0x0041_3724;
                    continue;
                }
                0x0041_3724 => {
                    c.cycles += 1;
                    if c.g[A0][0] != 0 {
                        c.g[A3][0] = c.g[V0][0];
                    }
                    pc = 0x0041_3728;
                    continue;
                }
                0x0041_3728 => {
                    c.cycles += 1;
                    c.sw((c.g[A1][0] as u32).wrapping_add(0xc), c.g[A3][0]);
                    pc = 0x0041_372c;
                    continue;
                }
                0x0041_372c => {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[A1][0];
                    pc = 0x0041_3730;
                    continue;
                }
                0x0041_3730 => {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x0041_3738);
                    PACK_F.call(c, 0x0041_3738)?;
                    frame.restore_ra(c, 0x40);
                    c.g[S0][0] = c.ld(frame.at(c, 0x30));
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0041_35e8, 0x0041_3748);
                    return Ok(());
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// __fpcmp_parts_f (0x00413748)

/// `int __fpcmp_parts_f(const fp_number_type *a, const fp_number_type *b)` (0x413748, libgcc
/// soft-float): compare two unpacked single-precision numbers, returning -1, 0 or 1
/// in `v0` (and related discriminants in `v1`/`a3`). Kept in the generated block shape.
pub fn fpcmp_parts_f(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A2][0] = c.lw(c.g[A0][0] as u32);
    c.g[V0][0] = (c.g[A2][0] < 0x2u64) as u64;
    let t = c.g[V0][0] != 0u64;
    if t {
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[V0][0] = 0x1;
        c.pc = t;
        return;
    } else {
        c.cycles += 4;
        c.g[V1][0] = c.lw(c.g[A1][0] as u32);
        c.g[V0][0] = (c.g[V1][0] < 0x2u64) as u64;
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = c.g[A2][0] ^ 0x4;
        if t {
            c.cycles += 2;
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = c.g[V1][0] ^ 0x4;
            if t {
                c.cycles += 2;
                if c.g[V0][0] != 0u64 {
                    c.g[V0][0] = c.g[A2][0] ^ 0x2;
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.g[V1][0] ^ 0x2;
                    if t {
                        c.cycles += 2;
                        if c.g[V0][0] == 0u64 {
                            c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                            c.cycles += 4;
                            c.g[V1][0] = 0x1;
                            c.g[V0][0] = 0xffffffffffffffff;
                            let t = c.g[RA][0] as u32;
                            if c.g[A0][0] == 0 {
                                c.g[V0][0] = c.g[V1][0];
                            }
                            c.pc = t;
                            return;
                        } else {
                            c.cycles += 4;
                            c.g[A3][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                            if c.g[A3][0] == c.g[V0][0] {
                                c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                                c.cycles += 4;
                                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                                if c.g[V0][0] != 0u64 {
                                    c.g[V1][0] = 0x1;
                                    c.cycles += 3;
                                    c.g[V0][0] = 0xffffffffffffffff;
                                    let t = c.g[RA][0] as u32;
                                    if c.g[A3][0] == 0 {
                                        c.g[V0][0] = c.g[V1][0];
                                    }
                                    c.pc = t;
                                    return;
                                } else {
                                    // Compare the fraction words to break the
                                    // magnitude tie, then the low words.
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                                        c.cycles += 4;
                                        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                                        c.g[V0][0] = (c.g[A0][0] < c.g[V1][0]) as u64;
                                        if c.g[V0][0] != 0u64 {
                                            c.g[V1][0] = 0x1;
                                            c.cycles += 3;
                                            c.g[V0][0] = 0xffffffffffffffff;
                                            let t = c.g[RA][0] as u32;
                                            if c.g[A3][0] == 0 {
                                                c.g[V0][0] = c.g[V1][0];
                                            }
                                            c.pc = t;
                                            return;
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = (c.g[V1][0] < c.g[A0][0]) as u64;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[V1][0] = 0xffffffffffffffff;
                                        if t {
                                            c.cycles += 3;
                                            c.g[V0][0] = 0x1;
                                            let t = c.g[RA][0] as u32;
                                            if c.g[A3][0] == 0 {
                                                c.g[V0][0] = c.g[V1][0];
                                            }
                                            c.pc = t;
                                            return;
                                        }
                                        c.cycles += 2;
                                        let t = c.g[RA][0] as u32;
                                        c.g[V0][0] = 0u64;
                                        c.pc = t;
                                        return;
                                    }
                                    c.cycles += 1;
                                    c.g[V1][0] = 0xffffffffffffffff;
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x1;
                                    let t = c.g[RA][0] as u32;
                                    if c.g[A3][0] == 0 {
                                        c.g[V0][0] = c.g[V1][0];
                                    }
                                    c.pc = t;
                                    return;
                                }
                            } else {
                                c.cycles += 1;
                                c.g[V1][0] = 0x1;
                                c.cycles += 3;
                                c.g[V0][0] = 0xffffffffffffffff;
                                let t = c.g[RA][0] as u32;
                                if c.g[A3][0] == 0 {
                                    c.g[V0][0] = c.g[V1][0];
                                }
                                c.pc = t;
                                return;
                            }
                        }
                    } else {
                        c.cycles += 2;
                        if c.g[V0][0] != 0u64 {
                            c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                            c.cycles += 4;
                            c.g[V1][0] = 0x1;
                            c.g[V0][0] = 0xffffffffffffffff;
                            let t = c.g[RA][0] as u32;
                            if c.g[A0][0] != 0 {
                                c.g[V0][0] = c.g[V1][0];
                            }
                            c.pc = t;
                            return;
                        } else {
                            c.cycles += 2;
                            let t = c.g[RA][0] as u32;
                            c.g[V0][0] = 0u64;
                            c.pc = t;
                            return;
                        }
                    }
                } else {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                    c.g[V1][0] = 0xffffffffffffffff;
                    c.g[V0][0] = 0x1;
                    let t = c.g[RA][0] as u32;
                    if c.g[A0][0] == 0 {
                        c.g[V0][0] = c.g[V1][0];
                    }
                    c.pc = t;
                    return;
                }
            } else {
                c.cycles += 2;
                if c.g[V0][0] != 0u64 {
                    c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                    c.cycles += 4;
                    c.g[V1][0] = 0x1;
                    c.g[V0][0] = 0xffffffffffffffff;
                    let t = c.g[RA][0] as u32;
                    if c.g[A0][0] == 0 {
                        c.g[V0][0] = c.g[V1][0];
                    }
                    c.pc = t;
                    return;
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.pc = t;
                    return;
                }
            }
        } else {
            c.cycles += 2;
            let t = c.g[RA][0] as u32;
            c.g[V0][0] = 0x1;
            c.pc = t;
            return;
        }
    }
}

// ---------------------------------------------------------------------------
// __floatsisf (0x004138b0)

/// `float __floatsisf(int x)` (0x4138b0, libgcc soft-float): convert a signed
/// integer to float. Kept in the generated block shape.
pub fn floatsisf(c: &mut Cpu) {
    call::guest(c, |c| {
        let frame = Frame::open(c, 0x20);
        c.cycles += 7;
        c.g[V0][0] = 0x3;
        c.g[V1][0] = sx((c.g[A0][0] as u32) >> 31);
        frame.save_ra(c, 0x10);
        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
        let t = c.g[A0][0] != 0u64;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V1][0]);
        if !t {
            c.cycles += 3;
            c.g[V0][0] = 0x2;
            c.sw(c.g[SP][0] as u32, c.g[V0][0]);
            c.cycles += 3;
            c.g[RA][0] = sx(0x0041395c);
            c.g[A0][0] = c.g[SP][0];
            PACK_F.call(c, 0x0041_395c)?;
            c.g[RA][0] = c.ld(frame.at(c, 0x10));
            c.cycles += 2;
            let t = c.g[RA][0] as u32;
            frame.close(c);
            c.pc = t;
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = 0x1e;
        let t = c.g[V1][0] == 0u64;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        if t {
            c.cycles += 1;
            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[A0][0]);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0xffffffff80000000;
            let t = c.g[A0][0] != c.g[V0][0];
            c.g[V0][0] = sx(0u32.wrapping_sub(c.g[A0][0] as u32));
            if t {
                c.cycles += 2;
                c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            } else {
                c.cycles += 4;
                c.g[AT][0] = 0xffffffffcf000000;
                c.mtc1(1, 0);
                c.g[RA][0] = c.ld(frame.at(c, 0x10));
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                frame.close(c);
                c.pc = t;
                return Ok(());
            }
        }
        c.cycles += 6;
        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = 0x3fff0000;
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        c.g[V0][0] = (c.g[V0][0] < c.g[A2][0]) as u64;
        let t = c.g[V0][0] != 0u64;
        c.g[A1][0] = 0x3fff0000;
        if !t {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
            c.g[A1][0] = c.g[A1][0] | 0xffff;
            loop {
                c.cycles += 7;
                c.g[V1][0] = sx((c.g[A2][0] as u32) << 1);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                c.g[A2][0] = c.g[V1][0];
                c.g[V0][0] = (c.g[A1][0] < c.g[V1][0]) as u64;
                let t = c.g[V0][0] == 0u64;
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0041_3930);
                    }
                } else {
                    c.cycles += 2;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                    break;
                }
            }
        }
        c.cycles += 3;
        c.g[RA][0] = sx(0x0041395c);
        c.g[A0][0] = c.g[SP][0];
        PACK_F.call(c, 0x0041_395c)?;
        c.g[RA][0] = c.ld(frame.at(c, 0x10));
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        return Ok(());
    })
}

// ---------------------------------------------------------------------------
// __fixsfsi (0x00413968)

/// `int __fixsfsi(float x)` (0x413968, libgcc soft-float): convert a float to
/// a signed integer, saturating out-of-range values. Kept in the generated dispatcher
/// shape.
pub fn fixsfsi(c: &mut Cpu) {
    call::guest(c, |c| {
        let mut pc: u32 = c.pc;
        let frame = Frame { size: 0x30 };
        loop {
            match pc {
                0x0041_3968 => {
                    c.cycles += 10;
                    Frame::open(c, 0x30);
                    c.sd(frame.at(c, 0x20), c.g[RA][0]);
                    c.g[A0][0] = sx(frame.at(c, 0x10));
                    c.swc1(12, frame.at(c, 0x10));
                    c.g[RA][0] = sx(0x0041_3980);
                    c.g[A1][0] = sx(frame.at(c, 0));
                    UNPACK_F.call(c, 0x0041_3980)?;
                    c.g[V1][0] = c.lw(frame.at(c, 0));
                    c.g[V0][0] = c.g[V1][0] ^ 0x2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = (c.g[V1][0] < 0x2u64) as u64;
                    if t {
                        pc = 0x0041_3998;
                        continue;
                    }
                    pc = 0x0041_3990;
                    continue;
                }
                0x0041_3990 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.g[V1][0] ^ 0x4;
                    if t {
                        pc = 0x0041_39a0;
                        continue;
                    }
                    pc = 0x0041_3998;
                    continue;
                }
                0x0041_3998 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0041_39e8;
                        continue;
                    }
                    pc = 0x0041_39a0;
                    continue;
                }
                0x0041_39a0 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.lw(frame.at(c, 0x8));
                    if t {
                        pc = 0x0041_39b8;
                        continue;
                    }
                    pc = 0x0041_39a8;
                    continue;
                }
                0x0041_39a8 => {
                    c.cycles += 2;
                    let t = (c.g[A0][0] as i64) < 0;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 31i64) as u64;
                    if t {
                        if back_edge_unwinds(c, 0x0041_3998) {
                            return Ok(());
                        }
                        pc = 0x0041_3998;
                        continue;
                    }
                    pc = 0x0041_39b0;
                    continue;
                }
                0x0041_39b0 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = 0x1e;
                    if t {
                        pc = 0x0041_39d0;
                        continue;
                    }
                    pc = 0x0041_39b8;
                    continue;
                }
                0x0041_39b8 => {
                    c.cycles += 6;
                    c.g[V1][0] = c.lw(frame.at(c, 0x4));
                    c.g[V0][0] = 0x7fff0000;
                    c.g[V0][0] = c.g[V0][0] | 0xffff;
                    c.g[A0][0] = 0xffffffff80000000;
                    let t = true;
                    if c.g[V1][0] != 0 {
                        c.g[V0][0] = c.g[A0][0];
                    }
                    if t {
                        pc = 0x0041_39e8;
                        continue;
                    }
                    pc = 0x0041_39d0;
                    continue;
                }
                0x0041_39d0 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw(frame.at(c, 0xc));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                    c.g[A1][0] = c.lw(frame.at(c, 0x4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> ((c.g[V1][0] as u32) & 31));
                    c.g[A0][0] = sx(0u32.wrapping_sub(c.g[V0][0] as u32));
                    if c.g[A1][0] != 0 {
                        c.g[V0][0] = c.g[A0][0];
                    }
                    pc = 0x0041_39e8;
                    continue;
                }
                0x0041_39e8 => {
                    c.cycles += 3;
                    frame.restore_ra(c, 0x20);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00413968, 0x004139f4);
                    return Ok(());
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// __rtti_class (0x00414bd8)

/// `void __rtti_class(void *obj, ...)` (0x414bd8): fill in the
/// `__class_type_info` vtable and links for `obj`. Kept in the generated
/// block shape.
pub fn rtti_class(c: &mut Cpu) {
    'b_414bf4: {
        c.cycles += 2;
        let t = c.g[A0][0] == 0u64;
        c.g[V0][0] = 0x4a0000;
        if t {
            break 'b_414bf4;
        } else {
            c.cycles += 5;
            c.sw((c.g[A0][0] as u32).wrapping_add(0xc), c.g[A3][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6630));
            c.sw(c.g[A0][0] as u32, c.g[A1][0]);
            c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
            break 'b_414bf4;
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.pc = t;
}

// ---------------------------------------------------------------------------
// __si_type_info_dcast (0x00414c70)

/// `void *__si_type_info_dcast(...)` (0x414c70): dynamic-cast helper for
/// single-inheritance type info: name match via `type_info_eq`, else
/// dispatch through the target's handler slot. Kept in the generated
/// block shape.
pub fn si_type_info_dcast(c: &mut Cpu) {
    call::guest(c, |c| {
        let frame = Frame::open(c, 0x70);
        c.cycles += 17;
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.g[S1][0] = c.g[A1][0];
        c.sd(frame.at(c, 0x50), c.g[S5][0]);
        c.g[S0][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x40), c.g[S4][0]);
        c.g[S5][0] = c.g[T1][0];
        c.sd(frame.at(c, 0x30), c.g[S3][0]);
        c.g[S4][0] = c.g[T0][0];
        c.sd(frame.at(c, 0x20), c.g[S2][0]);
        c.g[S3][0] = c.g[A2][0];
        frame.save_ra(c, 0x60);
        c.g[RA][0] = sx(0x0041_4cac);
        c.g[S2][0] = c.g[A3][0];
        callee::TYPE_INFO_EQ.call(c, 0x0041_4cac)?;
        let t = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[S2][0];
        if !t {
            c.cycles += 11;
            c.g[T2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S3][0];
            c.g[A3][0] = c.g[S2][0];
            c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x4));
            c.g[T0][0] = c.g[S4][0];
            c.g[T1][0] = c.g[S5][0];
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
            let target = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x0041_4ce0);
            c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call::call_ptr(c, target, 0x0041_4ce0)?;
        }
        c.cycles += 9;
        c.g[RA][0] = c.ld(frame.at(c, 0x60));
        c.g[S5][0] = c.ld(frame.at(c, 0x50));
        c.g[S4][0] = c.ld(frame.at(c, 0x40));
        c.g[S3][0] = c.ld(frame.at(c, 0x30));
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// __class_type_info_dcast (0x00414d08)

/// `void *__class_type_info_dcast(...)` (0x414d08): dynamic-cast helper for
/// multiple-inheritance type info: walks the base-class table, dispatching through
/// handler slots. Kept in the generated block shape.
pub fn class_type_info_dcast(c: &mut Cpu) {
    call::guest(c, |c| {
        let frame = Frame::open(c, 0xb0);
        c.cycles += 20;
        c.sd(frame.at(c, 0x50), c.g[S4][0]);
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        c.g[S4][0] = c.g[A1][0];
        c.sd(frame.at(c, 0x90), c.g[FP][0]);
        c.g[S2][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x80), c.g[S7][0]);
        c.g[FP][0] = c.g[A2][0];
        c.sd(frame.at(c, 0x60), c.g[S5][0]);
        c.g[S7][0] = c.g[T1][0];
        c.sd(frame.at(c, 0xa0), c.g[RA][0]);
        c.g[S5][0] = c.g[T0][0];
        c.sd(frame.at(c, 0x70), c.g[S6][0]);
        c.sd(frame.at(c, 0x40), c.g[S3][0]);
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[RA][0] = sx(0x0041_4d50);
        c.sw(c.g[SP][0] as u32, c.g[A3][0]);
        callee::TYPE_INFO_EQ.call(c, 0x0041_4d50)?;
        let t = c.g[V0][0] != 0u64;
        c.g[V0][0] = c.lw(c.g[SP][0] as u32);
        if t {
            c.cycles += 12;
            c.g[RA][0] = c.ld(frame.at(c, 0xa0));
            c.g[FP][0] = c.ld(frame.at(c, 0x90));
            c.g[S7][0] = c.ld(frame.at(c, 0x80));
            c.g[S6][0] = c.ld(frame.at(c, 0x70));
            c.g[S5][0] = c.ld(frame.at(c, 0x60));
            c.g[S4][0] = c.ld(frame.at(c, 0x50));
            c.g[S3][0] = c.ld(frame.at(c, 0x40));
            c.g[S2][0] = c.ld(frame.at(c, 0x30));
            c.g[S1][0] = c.ld(frame.at(c, 0x20));
            c.g[S0][0] = c.ld(frame.at(c, 0x10));
            let t = c.g[RA][0] as u32;
            frame.close(c);
            c.pc = t;
            return Ok(());
        } else {
            c.cycles += 4;
            c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
            c.g[S3][0] = 0u64;
            let t = c.g[A1][0] == 0u64;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
            if !t {
                c.cycles += 2;
                c.g[S6][0] = 0u64;
                'l_414d70: loop {
                    'b_414da0: {
                        c.cycles += 2;
                        let t = c.g[FP][0] == 0u64;
                        c.g[A0][0] = 0x1;
                        if !t {
                            c.cycles += 6;
                            c.g[T2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                            c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[T2][0] as u32));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) >> 30);
                            let t = c.g[V0][0] != c.g[A0][0];
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                            if t {
                                c.cycles += 5;
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x8));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0041_4d70);
                                    }
                                    continue 'l_414d70;
                                } else {
                                    break 'l_414d70;
                                }
                            } else {
                                c.cycles += 2;
                                c.g[T2][0] =
                                    sx((c.g[S6][0] as u32).wrapping_add(c.g[T2][0] as u32));
                                break 'b_414da0;
                            }
                        }
                        c.cycles += 2;
                        c.g[T2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                        c.g[T2][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[T2][0] as u32));
                        break 'b_414da0;
                    }
                    c.cycles += 9;
                    c.g[V0][0] = 0x1fff0000;
                    c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.g[V0][0] | 0xffff;
                    c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                    c.g[V0][0] = c.g[V1][0] & c.g[V0][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32) >> 29);
                    c.g[V1][0] = c.g[V1][0] & 0x1;
                    let t = c.g[V1][0] == 0u64;
                    c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    if !t {
                        c.cycles += 1;
                        c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                    }
                    c.cycles += 14;
                    c.g[T2][0] = c.lw(c.g[T2][0] as u32);
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A2][0] = c.g[FP][0];
                    c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x4));
                    c.g[T0][0] = c.g[S5][0];
                    c.g[T1][0] = c.g[S7][0];
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0041_4df4);
                    c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call::call_ptr(c, t, 0x0041_4df4)?;
                    c.g[S0][0] = c.g[V0][0];
                    if c.g[S0][0] == 0u64 {
                        c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                        c.cycles += 5;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                        c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x8));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0041_4d70);
                            }
                            continue 'l_414d70;
                        } else {
                            break 'l_414d70;
                        }
                    } else {
                        c.cycles += 2;
                        let t = c.g[S3][0] != 0u64;
                        if !t {
                            c.cycles += 2;
                            c.g[S3][0] = c.g[S0][0];
                            c.cycles += 1;
                            c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                            c.cycles += 1;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                            c.cycles += 5;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                            c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x8));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0041_4d70);
                                }
                                continue 'l_414d70;
                            } else {
                                break 'l_414d70;
                            }
                        }
                        c.cycles += 2;
                        if c.g[S3][0] == c.g[S0][0] {
                            c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                            c.cycles += 1;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                            c.cycles += 5;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                            c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x8));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0041_4d70);
                                }
                                continue 'l_414d70;
                            } else {
                                break 'l_414d70;
                            }
                        } else {
                            c.cycles += 2;
                            let t = c.g[S5][0] == 0u64;
                            c.g[A1][0] = c.g[S5][0];
                            if t {
                                c.cycles += 2;
                                c.g[V0][0] = 0u64;
                                c.cycles += 12;
                                c.g[RA][0] = c.ld(frame.at(c, 0xa0));
                                c.g[FP][0] = c.ld(frame.at(c, 0x90));
                                c.g[S7][0] = c.ld(frame.at(c, 0x80));
                                c.g[S6][0] = c.ld(frame.at(c, 0x70));
                                c.g[S5][0] = c.ld(frame.at(c, 0x60));
                                c.g[S4][0] = c.ld(frame.at(c, 0x50));
                                c.g[S3][0] = c.ld(frame.at(c, 0x40));
                                c.g[S2][0] = c.ld(frame.at(c, 0x30));
                                c.g[S1][0] = c.ld(frame.at(c, 0x20));
                                c.g[S0][0] = c.ld(frame.at(c, 0x10));
                                let t = c.g[RA][0] as u32;
                                frame.close(c);
                                c.pc = t;
                                return Ok(());
                            } else {
                                c.cycles += 22;
                                c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                                c.g[A2][0] = 0x1;
                                c.g[A3][0] = c.g[S3][0];
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
                                c.g[T0][0] = 0u64;
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                                c.g[T1][0] = 0u64;
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x0041_4e44);
                                c.g[A0][0] =
                                    sx((c.g[S4][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call::call_ptr(c, t, 0x0041_4e44)?;
                                c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                                c.g[S1][0] = c.g[V0][0];
                                c.g[A1][0] = c.g[S5][0];
                                c.g[A2][0] = 0x1;
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
                                c.g[A3][0] = c.g[S0][0];
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                                c.g[T0][0] = 0u64;
                                c.g[A0][0] =
                                    sx((c.g[S4][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x0041_4e70);
                                c.g[T1][0] = 0u64;
                                call::call_ptr(c, t, 0x0041_4e70)?;
                                if c.g[S1][0] == c.g[V0][0] {
                                    c.g[V0][0] = 0u64;
                                    c.cycles += 12;
                                    c.g[RA][0] = c.ld(frame.at(c, 0xa0));
                                    c.g[FP][0] = c.ld(frame.at(c, 0x90));
                                    c.g[S7][0] = c.ld(frame.at(c, 0x80));
                                    c.g[S6][0] = c.ld(frame.at(c, 0x70));
                                    c.g[S5][0] = c.ld(frame.at(c, 0x60));
                                    c.g[S4][0] = c.ld(frame.at(c, 0x50));
                                    c.g[S3][0] = c.ld(frame.at(c, 0x40));
                                    c.g[S2][0] = c.ld(frame.at(c, 0x30));
                                    c.g[S1][0] = c.ld(frame.at(c, 0x20));
                                    c.g[S0][0] = c.ld(frame.at(c, 0x10));
                                    let t = c.g[RA][0] as u32;
                                    frame.close(c);
                                    c.pc = t;
                                    return Ok(());
                                } else {
                                    c.cycles += 2;
                                    if c.g[S1][0] == c.g[S7][0] {
                                        c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                                        c.cycles += 1;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                        c.cycles += 5;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                        c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x8));
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x0041_4d70);
                                            }
                                            continue 'l_414d70;
                                        } else {
                                            break 'l_414d70;
                                        }
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == c.g[S7][0];
                                        c.g[S3][0] = c.g[S0][0];
                                        if t {
                                            c.cycles += 1;
                                            c.g[A1][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                                            c.cycles += 1;
                                            c.g[V0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                            c.cycles += 5;
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                            c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x8));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x0041_4d70);
                                                }
                                                continue 'l_414d70;
                                            } else {
                                                break 'l_414d70;
                                            }
                                        } else {
                                            c.cycles += 2;
                                            c.g[V0][0] = 0u64;
                                            c.cycles += 12;
                                            c.g[RA][0] = c.ld(frame.at(c, 0xa0));
                                            c.g[FP][0] = c.ld(frame.at(c, 0x90));
                                            c.g[S7][0] = c.ld(frame.at(c, 0x80));
                                            c.g[S6][0] = c.ld(frame.at(c, 0x70));
                                            c.g[S5][0] = c.ld(frame.at(c, 0x60));
                                            c.g[S4][0] = c.ld(frame.at(c, 0x50));
                                            c.g[S3][0] = c.ld(frame.at(c, 0x40));
                                            c.g[S2][0] = c.ld(frame.at(c, 0x30));
                                            c.g[S1][0] = c.ld(frame.at(c, 0x20));
                                            c.g[S0][0] = c.ld(frame.at(c, 0x10));
                                            let t = c.g[RA][0] as u32;
                                            frame.close(c);
                                            c.pc = t;
                                            return Ok(());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0];
            c.cycles += 12;
            c.g[RA][0] = c.ld(frame.at(c, 0xa0));
            c.g[FP][0] = c.ld(frame.at(c, 0x90));
            c.g[S7][0] = c.ld(frame.at(c, 0x80));
            c.g[S6][0] = c.ld(frame.at(c, 0x70));
            c.g[S5][0] = c.ld(frame.at(c, 0x60));
            c.g[S4][0] = c.ld(frame.at(c, 0x50));
            c.g[S3][0] = c.ld(frame.at(c, 0x40));
            c.g[S2][0] = c.ld(frame.at(c, 0x30));
            c.g[S1][0] = c.ld(frame.at(c, 0x20));
            c.g[S0][0] = c.ld(frame.at(c, 0x10));
            let t = c.g[RA][0] as u32;
            frame.close(c);
            c.pc = t;
            return Ok(());
        }
    })
}

// ---------------------------------------------------------------------------
// __class_type_info_ctor (0x00414ee0)

/// `__class_type_info_ctor(...)` (0x414ee0): fill in a
/// `__class_type_info` object (vtable + links). Straight line.
pub fn class_type_info_ctor(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x4a0000;
    c.sw(c.g[A0][0] as u32, c.g[A1][0]);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6630));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    c.g[V0][0] = c.g[A0][0];
    let t = c.g[RA][0] as u32;
    c.sw((c.g[A0][0] as u32).wrapping_add(0xc), c.g[A3][0]);
    c.pc = t;
}

// ---------------------------------------------------------------------------
// __si_type_info_ctor (0x00414f70)

/// `__si_type_info_ctor(...)` (0x414f70): fill in an
/// `__si_class_type_info` object (vtable + links). Straight line.
pub fn si_type_info_ctor(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = 0x4a0000;
    c.sw(c.g[A0][0] as u32, c.g[A1][0]);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6618));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = c.g[A0][0];
    c.pc = t;
}

// ---------------------------------------------------------------------------
// __user_type_info_ctor (0x00415000)

/// `__user_type_info_ctor(...)` (0x415000): fill in a
/// `__user_type_info` object (vtable + links). Straight line.
pub fn user_type_info_ctor(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = 0x4a0000;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6600));
    c.sw(c.g[V0][0] as u32, c.g[A1][0]);
    let t = c.g[RA][0] as u32;
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
    c.pc = t;
}

/// `void __deregister_frame_info(void *base)` (0x414780): walk the frame
/// list for the record matching `base` and unlink it (freeing its object
/// half when one is installed). An empty registry, or a full walk with no
/// match, aborts (noreturn). Kept in the generated block shape.
pub fn deregister_frame_info(c: &mut Cpu) {
    call::guest(c, |c| {
        let frame = Frame::open(c, 0x20);
        // Walk the frame list for the matching record; an empty list
        // drops to the abort tail below.
        c.cycles += 7;
        c.g[V0][0] = 0x530000;
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x5218));
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5218));
        frame.save_ra(c, 0x10);
        let t = c.g[V1][0] == 0u64;
        c.sd(frame.at(c, 0), c.g[S0][0]);
        if !t {
            c.cycles += 1;
            loop {
                c.cycles += 4;
                c.g[S0][0] = c.lw(c.g[A1][0] as u32);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                let t = c.g[V0][0] != c.g[A0][0];
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                if !t {
                    // Match: splice it out (freeing its object half when
                    // one is installed), then restore and return.
                    c.cycles += 3;
                    c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                    let t = c.g[V1][0] == 0u64;
                    c.sw(c.g[A1][0] as u32, c.g[V0][0]);
                    if t {
                        c.cycles += 1;
                        c.g[V0][0] = c.g[S0][0];
                    } else {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x0041_47c4);
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                        FREE.call(c, 0x0041_47c4)?;
                        c.g[V0][0] = c.g[S0][0];
                    }
                    c.cycles += 4;
                    c.g[RA][0] = c.ld(frame.at(c, 0x10));
                    c.g[S0][0] = c.ld(frame.at(c, 0));
                    let t = c.g[RA][0] as u32;
                    frame.close(c);
                    c.pc = t;
                    return Ok(());
                }
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64;
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0041_47a0);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 4;
        c.g[RA][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        // Tail call: abandon the frame (close sets pc to ra, overwritten
        // below like the normal return path).
        frame.close(c);
        c.pc = ABORT.addr;
        (ABORT.f)(c);
        return Ok(());
    })
}

// ---------------------------------------------------------------------------
// __frame_state_for (0x00414818)

/// `void *__frame_state_for(...)` (0x414818): locate the frame state (FDE + CIE)
/// for a program counter, running CFA instructions. Kept in the generated block
/// shape.
pub fn frame_state_for(c: &mut Cpu) {
    call::guest(c, |c| {
        let frame = Frame::open(c, 0x3c0);
        'b_4149c4: {
            c.cycles += 16;
            c.sd(frame.at(c, 0x390), c.g[S6][0]);
            c.sd(frame.at(c, 0x3a0), c.g[S7][0]);
            c.g[S6][0] = c.g[A0][0];
            c.sd(frame.at(c, 0x350), c.g[S2][0]);
            c.g[S7][0] = c.g[A1][0];
            frame.save_ra(c, 0x3b0);
            c.sd(frame.at(c, 0x380), c.g[S5][0]);
            c.sd(frame.at(c, 0x370), c.g[S4][0]);
            c.sd(frame.at(c, 0x360), c.g[S3][0]);
            c.sd(frame.at(c, 0x340), c.g[S1][0]);
            c.g[RA][0] = sx(0x0041_484c);
            c.sd(frame.at(c, 0x330), c.g[S0][0]);
            callee::FIND_FDE.call(c, 0x0041_484c)?;
            c.g[S2][0] = c.g[V0][0];
            let t = c.g[S2][0] == 0u64;
            c.g[A0][0] = c.g[S2][0];
            if t {
                c.cycles += 2;
                c.g[V0][0] = 0u64;
                break 'b_4149c4;
            } else {
                c.cycles += 5;
                c.g[RA][0] = sx(0x0041_4860);
                c.g[A1][0] = c.g[SP][0];
                callee::EXTRACT_CIE_INFO.call(c, 0x0041_4860)?;
                c.g[S0][0] = c.g[V0][0];
                let t = c.g[S0][0] != 0u64;
                c.g[S4][0] = sx(frame.at(c, 0x20));
                if !t {
                    c.cycles += 2;
                    c.g[V0][0] = 0u64;
                    break 'b_4149c4;
                }
            }
            c.cycles += 16;
            c.g[A1][0] = 0u64;
            c.g[A0][0] = c.g[S4][0];
            c.g[RA][0] = sx(0x0041_4884);
            c.g[A2][0] = 0x2f8;
            callee::MEMSET.call(c, 0x0041_4884)?;
            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = c.lhu(frame.at(c, 0x10));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x4));
            c.g[A1][0] = c.lw(frame.at(c, 0x4));
            c.sh(frame.at(c, 0x2ba), c.g[A0][0]);
            c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(c.g[V1][0] as u32));
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = (c.g[S0][0] < c.g[S1][0]) as u64;
            let t = c.g[V0][0] == 0u64;
            c.sw(frame.at(c, 0x24), c.g[A1][0]);
            if t {
                c.cycles += 3;
                c.g[S3][0] = sx((c.g[S2][0] as u32).wrapping_add(0x10));
                c.g[S5][0] = sx(frame.at(c, 0x300));
                c.g[A0][0] = c.lw(frame.at(c, 0));
            } else {
                c.cycles += 3;
                c.g[S3][0] = sx((c.g[S2][0] as u32).wrapping_add(0x10));
                c.g[S5][0] = sx(frame.at(c, 0x300));
                c.g[A0][0] = c.g[S0][0];
                'l_4148c0: loop {
                    c.cycles += 8;
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A2][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x0041_48d0);
                    c.g[A3][0] = 0u64;
                    callee::EXECUTE_CFA_INSN.call(c, 0x0041_48d0)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V0][0] = (c.g[S0][0] < c.g[S1][0]) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0041_48c0);
                        }
                        continue 'l_4148c0;
                    } else {
                        c.cycles += 2;
                        c.g[A0][0] = c.lw(frame.at(c, 0));
                        break;
                    }
                }
            }
            c.cycles += 4;
            c.g[V1][0] = 0x7a;
            c.g[V0][0] = c.lb(c.g[A0][0] as u32);
            let t = c.g[V0][0] != c.g[V1][0];
            c.g[S0][0] = c.g[S3][0];
            if !t {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0041_4910);
                c.g[A1][0] = sx(frame.at(c, 0x320));
                callee::DECODE_ULEB128.call(c, 0x0041_4910)?;
                c.g[V1][0] = c.lw(frame.at(c, 0x320));
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            }
            c.cycles += 7;
            c.g[V0][0] = c.lw(c.g[S2][0] as u32);
            c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = (c.g[S0][0] < c.g[S1][0]) as u64;
            let t = c.g[V0][0] == 0u64;
            c.sw(frame.at(c, 0x324), c.g[A1][0]);
            if t {
                c.cycles += 1;
                c.g[A0][0] = c.g[S4][0];
            } else {
                c.cycles += 3;
                c.g[V0][0] = (c.g[S6][0] < c.g[A1][0]) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[A0][0] = c.g[S4][0];
                if !t {
                    c.cycles += 2;
                    c.g[S2][0] = sx(frame.at(c, 0x324));
                    c.g[A0][0] = c.g[S0][0];
                    'l_414948: loop {
                        c.cycles += 8;
                        c.g[A1][0] = c.g[S4][0];
                        c.g[A2][0] = c.g[SP][0];
                        c.g[RA][0] = sx(0x0041_4958);
                        c.g[A3][0] = c.g[S2][0];
                        callee::EXECUTE_CFA_INSN.call(c, 0x0041_4958)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[V0][0] = (c.g[S0][0] < c.g[S1][0]) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = c.lw(frame.at(c, 0x324));
                        if t {
                            c.cycles += 1;
                            c.g[A0][0] = c.g[S4][0];
                            break;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = (c.g[S6][0] < c.g[V0][0]) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A0][0] = c.g[S0][0];
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0041_4948);
                                }
                                continue 'l_414948;
                            } else {
                                c.cycles += 1;
                                c.g[A0][0] = c.g[S4][0];
                                break;
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S7][0];
            'l_41497c: loop {
                c.cycles += 13;
                c.g[V1][0] = c.ld(c.g[A0][0] as u32);
                c.g[A1][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[A2][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x10));
                c.g[A3][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x18));
                c.sd(c.g[V0][0] as u32, c.g[V1][0]);
                c.sd((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                c.sd((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                c.sd((c.g[V0][0] as u32).wrapping_add(0x18), c.g[A3][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
                let t = c.g[A0][0] != c.g[S5][0];
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0041_497c);
                    }
                    continue 'l_41497c;
                } else {
                    c.cycles += 5;
                    c.g[V1][0] = c.ld(c.g[A0][0] as u32);
                    c.g[A1][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[V0][0] as u32, c.g[V1][0]);
                    c.sd((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                    c.g[V0][0] = c.g[S7][0];
                    break 'b_4149c4;
                }
            }
        }
        c.cycles += 11;
        c.g[RA][0] = c.ld(frame.at(c, 0x3b0));
        c.g[S7][0] = c.ld(frame.at(c, 0x3a0));
        c.g[S6][0] = c.ld(frame.at(c, 0x390));
        c.g[S5][0] = c.ld(frame.at(c, 0x380));
        c.g[S4][0] = c.ld(frame.at(c, 0x370));
        c.g[S3][0] = c.ld(frame.at(c, 0x360));
        c.g[S2][0] = c.ld(frame.at(c, 0x350));
        c.g[S1][0] = c.ld(frame.at(c, 0x340));
        c.g[S0][0] = c.ld(frame.at(c, 0x330));
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        return Ok(());
    })
}

// ---------------------------------------------------------------------------
// __is_pointer (0x00415728)

/// `bool __is_pointer(const void *)` (0x415728): whether the typeinfo at
/// `a0` describes a pointer, via `__dynamic_cast` on its kind slot.
pub fn is_pointer(c: &mut Cpu) {
    call::guest(c, |c| {
        let frame = Frame::open(c, 0x10);
        'b_415768: {
            c.cycles += 5;
            c.g[V1][0] = c.g[A0][0];
            frame.save_ra(c, 0);
            c.g[V0][0] = 0;
            if c.g[V1][0] == 0 {
                break 'b_415768;
            }
            c.cycles += 11;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
            c.g[A1][0] = 0x410000;
            c.g[T0][0] = 0x410000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5f48));
            c.g[A3][0] = c.lh(c.g[V0][0] as u32);
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x51f8));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
            c.g[T1][0] = c.g[V1][0];
            c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
            c.g[RA][0] = sx(0x0041_5768);
            c.g[A2][0] = 0x1;
            callee::DYNAMIC_CAST.call(c, 0x0041_5768)?;
            break 'b_415768;
        }
        c.cycles += 4;
        c.g[RA][0] = c.ld(frame.at(c, 0));
        c.g[V0][0] = (0 < c.g[V0][0]) as u64;
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// __rtti_ptr (0x00415778)

/// `__rtti_ptr(...)` (0x415778): fill in a `__pointer_type_info` object.
pub fn rtti_ptr(c: &mut Cpu) {
    'b_415790: {
        c.cycles += 2;
        let t = c.g[A0][0] == 0u64;
        c.g[V0][0] = 0x4a0000;
        if t {
            break 'b_415790;
        }
        c.cycles += 4;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6438));
        c.sw(c.g[A0][0] as u32, c.g[A1][0]);
        c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        break 'b_415790;
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.pc = t;
}

// ---------------------------------------------------------------------------
// __rtti_attr (0x00415798)

/// `__rtti_attr(...)` (0x415798): fill in an `__attr_type_info` object.
pub fn rtti_attr(c: &mut Cpu) {
    'b_4157b4: {
        c.cycles += 2;
        let t = c.g[A0][0] == 0u64;
        c.g[V0][0] = 0x4a0000;
        if t {
            break 'b_4157b4;
        }
        c.cycles += 5;
        c.sw((c.g[A0][0] as u32).wrapping_add(0xc), c.g[A2][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6448));
        c.sw(c.g[A0][0] as u32, c.g[A1][0]);
        c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A3][0]);
        break 'b_4157b4;
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.pc = t;
}

// ---------------------------------------------------------------------------
// __rtti_func (0x004157c0)

/// `__rtti_func(...)` (0x4157c0): fill in a `__func_type_info` object.
pub fn rtti_func(c: &mut Cpu) {
    'b_4157d4: {
        c.cycles += 2;
        let t = c.g[A0][0] == 0u64;
        c.g[V0][0] = 0x4a0000;
        if t {
            break 'b_4157d4;
        }
        c.cycles += 3;
        c.sw(c.g[A0][0] as u32, c.g[A1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6468));
        c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        break 'b_4157d4;
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.pc = t;
}

// ---------------------------------------------------------------------------
// __rtti_ptmf (0x004157e0)

/// `__rtti_ptmf(...)` (0x4157e0): fill in a `__ptmf_type_info` object.
pub fn rtti_ptmf(c: &mut Cpu) {
    'b_4157f4: {
        c.cycles += 2;
        let t = c.g[A0][0] == 0u64;
        c.g[V0][0] = 0x4a0000;
        if t {
            break 'b_4157f4;
        }
        c.cycles += 3;
        c.sw(c.g[A0][0] as u32, c.g[A1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6478));
        c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        break 'b_4157f4;
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.pc = t;
}

// ---------------------------------------------------------------------------
// __rtti_ptmd (0x00415800)

/// `__rtti_ptmd(...)` (0x415800): fill in a `__ptmd_type_info` object.
pub fn rtti_ptmd(c: &mut Cpu) {
    'b_415814: {
        c.cycles += 2;
        let t = c.g[A0][0] == 0u64;
        c.g[V0][0] = 0x4a0000;
        if t {
            break 'b_415814;
        }
        c.cycles += 3;
        c.sw(c.g[A0][0] as u32, c.g[A1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6488));
        c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        break 'b_415814;
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.pc = t;
}

// ---------------------------------------------------------------------------
// __rtti_array (0x00415820)

/// `__rtti_array(...)` (0x415820): fill in an `__array_type_info` object.
pub fn rtti_array(c: &mut Cpu) {
    'b_415834: {
        c.cycles += 2;
        let t = c.g[A0][0] == 0u64;
        c.g[V0][0] = 0x4a0000;
        if t {
            break 'b_415834;
        }
        c.cycles += 3;
        c.sw(c.g[A0][0] as u32, c.g[A1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6498));
        c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        break 'b_415834;
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.pc = t;
}

// ---------------------------------------------------------------------------
// __tfx (0x00415928)

/// `void __tfx(void)` (0x0041_5928, libgcc): install this fundamental
/// type's info into its registry slot when that slot is empty.
pub fn tfx(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A0][0] = 0x450000;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4160));
    let t = c.g[V0][0] != 0u64;
    c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4160));
    if !t {
        c.cycles += 2;
        let t = c.g[A1][0] == 0u64;
        c.g[V1][0] = 0x4a0000;
        if !t {
            c.cycles += 5;
            c.g[V0][0] = 0x4a0000;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6518));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6458));
            c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            c.sw((c.g[A0][0] as u32).wrapping_add(0x4160), c.g[V1][0]);
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4160));
    c.pc = t;
}

// ---------------------------------------------------------------------------
// __tfl (0x00415960)

/// `void __tfl(void)` (0x0041_5960, libgcc): install this fundamental
/// type's info into its registry slot when that slot is empty.
pub fn tfl(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A0][0] = 0x450000;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4168));
    let t = c.g[V0][0] != 0u64;
    c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4168));
    if !t {
        c.cycles += 2;
        let t = c.g[A1][0] == 0u64;
        c.g[V1][0] = 0x4a0000;
        if !t {
            c.cycles += 5;
            c.g[V0][0] = 0x4a0000;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6510));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6458));
            c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            c.sw((c.g[A0][0] as u32).wrapping_add(0x4168), c.g[V1][0]);
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4168));
    c.pc = t;
}

// ---------------------------------------------------------------------------
// __tfi (0x00415998)

/// `void __tfi(void)` (0x0041_5998, libgcc): install this fundamental
/// type's info into its registry slot when that slot is empty.
pub fn tfi(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A0][0] = 0x450000;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4170));
    let t = c.g[V0][0] != 0u64;
    c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4170));
    if !t {
        c.cycles += 2;
        let t = c.g[A1][0] == 0u64;
        c.g[V1][0] = 0x4a0000;
        if !t {
            c.cycles += 5;
            c.g[V0][0] = 0x4a0000;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6508));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6458));
            c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            c.sw((c.g[A0][0] as u32).wrapping_add(0x4170), c.g[V1][0]);
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4170));
    c.pc = t;
}

// ---------------------------------------------------------------------------
// __tfs (0x004159d0)

/// `void __tfs(void)` (0x0041_59d0, libgcc): install this fundamental
/// type's info into its registry slot when that slot is empty.
pub fn tfs(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A0][0] = 0x450000;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4178));
    let t = c.g[V0][0] != 0u64;
    c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4178));
    if !t {
        c.cycles += 2;
        let t = c.g[A1][0] == 0u64;
        c.g[V1][0] = 0x4a0000;
        if !t {
            c.cycles += 5;
            c.g[V0][0] = 0x4a0000;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6500));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6458));
            c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            c.sw((c.g[A0][0] as u32).wrapping_add(0x4178), c.g[V1][0]);
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4178));
    c.pc = t;
}

// ---------------------------------------------------------------------------
// __tfb (0x00415a08)

/// `void __tfb(void)` (0x0041_5a08, libgcc): install this fundamental
/// type's info into its registry slot when that slot is empty.
pub fn tfb(c: &mut Cpu) {
    'b_415a34: {
        c.cycles += 4;
        c.g[A0][0] = 0x450000;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4180));
        let t = c.g[V0][0] != 0u64;
        c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4180));
        if t {
            break 'b_415a34;
        } else {
            c.cycles += 2;
            let t = c.g[A1][0] == 0u64;
            c.g[V1][0] = 0x4a0000;
            if t {
                break 'b_415a34;
            } else {
                c.cycles += 5;
                c.g[V0][0] = 0x4a0000;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x64f8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6458));
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4180), c.g[V1][0]);
                break 'b_415a34;
            }
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4180));
    c.pc = t;
}

// ---------------------------------------------------------------------------
// __tfc (0x00415a40)

/// `void __tfc(void)` (0x0041_5a40, libgcc): install this fundamental
/// type's info into its registry slot when that slot is empty.
pub fn tfc(c: &mut Cpu) {
    'b_415a6c: {
        c.cycles += 4;
        c.g[A0][0] = 0x450000;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4188));
        let t = c.g[V0][0] != 0u64;
        c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4188));
        if t {
            break 'b_415a6c;
        } else {
            c.cycles += 2;
            let t = c.g[A1][0] == 0u64;
            c.g[V1][0] = 0x4a0000;
            if t {
                break 'b_415a6c;
            } else {
                c.cycles += 5;
                c.g[V0][0] = 0x4a0000;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x64f0));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6458));
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4188), c.g[V1][0]);
                break 'b_415a6c;
            }
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4188));
    c.pc = t;
}

// ---------------------------------------------------------------------------
// __tfw (0x00415a78)

/// `void __tfw(void)` (0x0041_5a78, libgcc): install this fundamental
/// type's info into its registry slot when that slot is empty.
pub fn tfw(c: &mut Cpu) {
    'b_415aa4: {
        c.cycles += 4;
        c.g[A0][0] = 0x450000;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4190));
        let t = c.g[V0][0] != 0u64;
        c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4190));
        if t {
            break 'b_415aa4;
        } else {
            c.cycles += 2;
            let t = c.g[A1][0] == 0u64;
            c.g[V1][0] = 0x4a0000;
            if t {
                break 'b_415aa4;
            } else {
                c.cycles += 5;
                c.g[V0][0] = 0x4a0000;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x64e8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6458));
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4190), c.g[V1][0]);
                break 'b_415aa4;
            }
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4190));
    c.pc = t;
}

// ---------------------------------------------------------------------------
// __tfr (0x00415ab0)

/// `void __tfr(void)` (0x0041_5ab0, libgcc): install this fundamental
/// type's info into its registry slot when that slot is empty.
pub fn tfr(c: &mut Cpu) {
    'b_415adc: {
        c.cycles += 4;
        c.g[A0][0] = 0x450000;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4198));
        let t = c.g[V0][0] != 0u64;
        c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4198));
        if t {
            break 'b_415adc;
        } else {
            c.cycles += 2;
            let t = c.g[A1][0] == 0u64;
            c.g[V1][0] = 0x4a0000;
            if t {
                break 'b_415adc;
            } else {
                c.cycles += 5;
                c.g[V0][0] = 0x4a0000;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x64e0));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6458));
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4198), c.g[V1][0]);
                break 'b_415adc;
            }
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4198));
    c.pc = t;
}

// ---------------------------------------------------------------------------
// __dynamic_cast (0x00415840)

/// `void *__dynamic_cast(...)` (0x415840): run a dynamic cast through a
/// chain of handler slots. Kept in the generated block shape.
pub fn dynamic_cast(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 43;
        let frame = Frame::open(c, 0x90);
        c.sd(frame.at(c, 0x70), c.g[S7][0]);
        c.sd(frame.at(c, 0x60), c.g[S6][0]);
        c.g[S7][0] = c.g[T1][0];
        c.sd(frame.at(c, 0x50), c.g[S5][0]);
        c.g[S6][0] = c.g[A3][0];
        c.sd(frame.at(c, 0x40), c.g[S4][0]);
        c.g[S5][0] = c.g[A2][0];
        c.sd(frame.at(c, 0x30), c.g[S3][0]);
        c.g[S4][0] = c.g[A1][0];
        c.sd(frame.at(c, 0x20), c.g[S2][0]);
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        frame.save_ra(c, 0x80);
        let t = c.g[A0][0] as u32;
        c.g[RA][0] = sx(0x0041_5880);
        c.g[S3][0] = c.g[T0][0];
        call::call_ptr(c, t, 0x0041_5880)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
        let t = c.g[S4][0] as u32;
        c.g[RA][0] = sx(0x0041_5898);
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        call::call_ptr(c, t, 0x0041_5898)?;
        let t = c.g[S3][0] as u32;
        c.g[RA][0] = sx(0x0041_58a0);
        c.g[S2][0] = c.g[V0][0];
        call::call_ptr(c, t, 0x0041_58a0)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S5][0];
        c.g[A3][0] = c.g[S6][0];
        c.g[T1][0] = c.g[S7][0];
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0041_58c0);
        c.g[T0][0] = c.g[V0][0];
        call::call_ptr(c, t, 0x0041_58c0)?;
        c.g[RA][0] = c.ld(frame.at(c, 0x80));
        c.g[S7][0] = c.ld(frame.at(c, 0x70));
        c.g[S6][0] = c.ld(frame.at(c, 0x60));
        c.g[S5][0] = c.ld(frame.at(c, 0x50));
        c.g[S4][0] = c.ld(frame.at(c, 0x40));
        c.g[S3][0] = c.ld(frame.at(c, 0x30));
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// __throw_type_match_rtti_2 (0x00415260)

/// `int __throw_type_match_rtti_2(...)` (0x415260): match a thrown object
/// against a catch type using RTTI. Kept in the generated dispatcher shape.
pub fn throw_type_match_rtti_2(c: &mut Cpu) {
    call::guest(c, |c| {
        let mut pc: u32 = c.pc;
        let frame = Frame { size: 0xb0 };
        loop {
            match pc {
                0x0041_5260 => {
                    c.cycles += 17;
                    Frame::open(c, 0xb0);
                    c.sd(frame.at(c, 0x20), c.g[S1][0]);
                    c.sd(frame.at(c, 0x10), c.g[S0][0]);
                    c.g[S1][0] = c.g[A0][0];
                    c.sd(frame.at(c, 0x70), c.g[S6][0]);
                    c.g[S0][0] = c.g[A1][0];
                    c.sd(frame.at(c, 0xa0), c.g[RA][0]);
                    c.g[S6][0] = c.g[A2][0];
                    c.sd(frame.at(c, 0x90), c.g[FP][0]);
                    c.sd(frame.at(c, 0x80), c.g[S7][0]);
                    c.sd(frame.at(c, 0x60), c.g[S5][0]);
                    c.sd(frame.at(c, 0x50), c.g[S4][0]);
                    c.sd(frame.at(c, 0x40), c.g[S3][0]);
                    c.g[RA][0] = sx(0x0041_529c);
                    c.sd(frame.at(c, 0x30), c.g[S2][0]);
                    callee::TYPE_INFO_EQ.call(c, 0x0041_529c)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.g[S6][0];
                    if t {
                        pc = 0x0041_56f8;
                        continue;
                    }
                    pc = 0x0041_52a4;
                    continue;
                }
                0x0041_52a4 => {
                    c.cycles += 2;
                    let t = c.g[S0][0] != 0u64;
                    c.sw(c.g[SP][0] as u32, 0u64);
                    if t {
                        pc = 0x0041_52b4;
                        continue;
                    }
                    pc = 0x0041_52ac;
                    continue;
                }
                0x0041_52ac => {
                    c.cycles += 2;
                    let t = true;
                    c.g[T2][0] = 0u64;
                    if t {
                        pc = 0x0041_52e4;
                        continue;
                    }
                    pc = 0x0041_52b4;
                    continue;
                }
                0x0041_52b4 => {
                    c.cycles += 12;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[A1][0] = 0x410000;
                    c.g[T0][0] = 0x410000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5018));
                    c.g[A3][0] = c.lh(c.g[V0][0] as u32);
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x51f8));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    c.g[A2][0] = 0x1;
                    c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[RA][0] = sx(0x0041_52e0);
                    c.g[T1][0] = c.g[S0][0];
                    callee::DYNAMIC_CAST.call(c, 0x0041_52e0)?;
                    c.g[T2][0] = c.g[V0][0];
                    pc = 0x0041_52e4;
                    continue;
                }
                0x0041_52e4 => {
                    c.cycles += 2;
                    if c.g[T2][0] != 0u64 {
                        c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x4));
                        pc = 0x0041_54d4;
                        continue;
                    }
                    pc = 0x0041_52ec;
                    continue;
                }
                0x0041_52ec => {
                    c.cycles += 2;
                    if c.g[S0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        pc = 0x0041_52fc;
                        continue;
                    }
                    pc = 0x0041_52f4;
                    continue;
                }
                0x0041_52f4 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = 0u64;
                    if t {
                        pc = 0x0041_5328;
                        continue;
                    }
                    pc = 0x0041_52fc;
                    continue;
                }
                0x0041_52fc => {
                    c.cycles += 11;
                    c.g[A1][0] = 0x410000;
                    c.g[T0][0] = 0x410000;
                    c.g[T1][0] = c.g[S0][0];
                    c.g[A3][0] = c.lh(c.g[V0][0] as u32);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5f48));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x51f8));
                    c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[RA][0] = sx(0x0041_5324);
                    c.g[A2][0] = 0x1;
                    callee::DYNAMIC_CAST.call(c, 0x0041_5324)?;
                    c.g[S0][0] = c.g[V0][0];
                    pc = 0x0041_5328;
                    continue;
                }
                0x0041_5328 => {
                    c.cycles += 2;
                    let t = c.g[S0][0] == 0u64;
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    if t {
                        pc = 0x0041_56f8;
                        continue;
                    }
                    pc = 0x0041_5330;
                    continue;
                }
                0x0041_5330 => {
                    c.cycles += 2;
                    let t = c.g[S1][0] == 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0041_5364;
                        continue;
                    }
                    pc = 0x0041_5338;
                    continue;
                }
                0x0041_5338 => {
                    c.cycles += 11;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[A1][0] = 0x410000;
                    c.g[T0][0] = 0x410000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5f48));
                    c.g[A3][0] = c.lh(c.g[V0][0] as u32);
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x51f8));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    c.g[T1][0] = c.g[S1][0];
                    c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[RA][0] = sx(0x0041_5364);
                    c.g[A2][0] = 0x1;
                    callee::DYNAMIC_CAST.call(c, 0x0041_5364)?;
                    pc = 0x0041_5364;
                    continue;
                }
                0x0041_5364 => {
                    c.cycles += 2;
                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                        pc = 0x0041_56f8;
                        continue;
                    }
                    pc = 0x0041_536c;
                    continue;
                }
                0x0041_536c => {
                    c.cycles += 4;
                    c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                    let t = c.g[S0][0] == 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0041_53a8;
                        continue;
                    }
                    pc = 0x0041_537c;
                    continue;
                }
                0x0041_537c => {
                    c.cycles += 11;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[A1][0] = 0x410000;
                    c.g[T0][0] = 0x410000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5ed8));
                    c.g[A3][0] = c.lh(c.g[V0][0] as u32);
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x51f8));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    c.g[A2][0] = 0x1;
                    c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[RA][0] = sx(0x0041_53a8);
                    c.g[T1][0] = c.g[S0][0];
                    callee::DYNAMIC_CAST.call(c, 0x0041_53a8)?;
                    pc = 0x0041_53a8;
                    continue;
                }
                0x0041_53a8 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S3][0] = 0u64;
                    if t {
                        pc = 0x0041_53b8;
                        continue;
                    }
                    pc = 0x0041_53b0;
                    continue;
                }
                0x0041_53b0 => {
                    c.cycles += 2;
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[S3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    pc = 0x0041_53b8;
                    continue;
                }
                0x0041_53b8 => {
                    c.cycles += 2;
                    let t = c.g[S1][0] == 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0041_53ec;
                        continue;
                    }
                    pc = 0x0041_53c0;
                    continue;
                }
                0x0041_53c0 => {
                    c.cycles += 11;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[A1][0] = 0x410000;
                    c.g[T0][0] = 0x410000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5ed8));
                    c.g[A3][0] = c.lh(c.g[V0][0] as u32);
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x51f8));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    c.g[A2][0] = 0x1;
                    c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[RA][0] = sx(0x0041_53ec);
                    c.g[T1][0] = c.g[S1][0];
                    callee::DYNAMIC_CAST.call(c, 0x0041_53ec)?;
                    pc = 0x0041_53ec;
                    continue;
                }
                0x0041_53ec => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S2][0] = 0u64;
                    if t {
                        pc = 0x0041_53fc;
                        continue;
                    }
                    pc = 0x0041_53f4;
                    continue;
                }
                0x0041_53f4 => {
                    c.cycles += 2;
                    c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[S2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    pc = 0x0041_53fc;
                    continue;
                }
                0x0041_53fc => {
                    c.cycles += 5;
                    c.g[V0][0] = c.g[S3][0] & 0x1;
                    c.g[S4][0] = c.g[S2][0] & 0x1;
                    c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    if t {
                        pc = 0x0041_56f8;
                        continue;
                    }
                    pc = 0x0041_5410;
                    continue;
                }
                0x0041_5410 => {
                    c.cycles += 5;
                    c.g[V1][0] = c.g[S3][0] & 0x2;
                    c.g[V0][0] = c.g[S2][0] & 0x2;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    if t {
                        pc = 0x0041_56f8;
                        continue;
                    }
                    pc = 0x0041_5424;
                    continue;
                }
                0x0041_5424 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x0041_5430);
                    c.g[A1][0] = c.g[S0][0];
                    callee::TYPE_INFO_EQ.call(c, 0x0041_5430)?;
                    if c.g[V0][0] != 0u64 {
                        c.sw(c.g[SP][0] as u32, c.g[S6][0]);
                        pc = 0x0041_56f4;
                        continue;
                    }
                    pc = 0x0041_5438;
                    continue;
                }
                0x0041_5438 => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x0041_5440);
                    callee::TFV.call(c, 0x0041_5440)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0041_544c);
                    c.g[A0][0] = c.g[S1][0];
                    callee::TYPE_INFO_EQ.call(c, 0x0041_544c)?;
                    let t = c.g[V0][0] == 0u64;
                    if t {
                        pc = 0x0041_548c;
                        continue;
                    }
                    pc = 0x0041_5454;
                    continue;
                }
                0x0041_5454 => {
                    c.cycles += 2;
                    let t = c.g[S0][0] == 0u64;
                    c.g[T0][0] = 0x410000;
                    if t {
                        pc = 0x0041_56f0;
                        continue;
                    }
                    pc = 0x0041_545c;
                    continue;
                }
                0x0041_545c => {
                    c.cycles += 12;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[A1][0] = 0x410000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5df8));
                    c.g[A3][0] = c.lh(c.g[V0][0] as u32);
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x51f8));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    c.g[A2][0] = 0x1;
                    c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[RA][0] = sx(0x0041_5484);
                    c.g[T1][0] = c.g[S0][0];
                    callee::DYNAMIC_CAST.call(c, 0x0041_5484)?;
                    if c.g[V0][0] == 0u64 {
                        c.sw(c.g[SP][0] as u32, c.g[S6][0]);
                        pc = 0x0041_56f4;
                        continue;
                    }
                    pc = 0x0041_548c;
                    continue;
                }
                0x0041_548c => {
                    c.cycles += 2;
                    if c.g[S0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        pc = 0x0041_549c;
                        continue;
                    }
                    pc = 0x0041_5494;
                    continue;
                }
                0x0041_5494 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[T2][0] = 0u64;
                    if t {
                        pc = 0x0041_54c8;
                        continue;
                    }
                    pc = 0x0041_549c;
                    continue;
                }
                0x0041_549c => {
                    c.cycles += 11;
                    c.g[A1][0] = 0x410000;
                    c.g[T0][0] = 0x410000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5018));
                    c.g[A3][0] = c.lh(c.g[V0][0] as u32);
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x51f8));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    c.g[A2][0] = 0x1;
                    c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[RA][0] = sx(0x0041_54c4);
                    c.g[T1][0] = c.g[S0][0];
                    callee::DYNAMIC_CAST.call(c, 0x0041_54c4)?;
                    c.g[T2][0] = c.g[V0][0];
                    pc = 0x0041_54c8;
                    continue;
                }
                0x0041_54c8 => {
                    c.cycles += 2;
                    let t = c.g[T2][0] == 0u64;
                    if t {
                        pc = 0x0041_5500;
                        continue;
                    }
                    pc = 0x0041_54d0;
                    continue;
                }
                0x0041_54d0 => {
                    c.cycles += 1;
                    c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x4));
                    pc = 0x0041_54d4;
                    continue;
                }
                0x0041_54d4 => {
                    c.cycles += 11;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A3][0] = c.g[S6][0];
                    c.g[A2][0] = 0x1;
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[T0][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                    c.g[T1][0] = 0u64;
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0041_54f8);
                    c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call::call_ptr(c, t, 0x0041_54f8)?;
                    let t = true;
                    c.sw(c.g[SP][0] as u32, c.g[V0][0]);
                    if t {
                        pc = 0x0041_56f4;
                        continue;
                    }
                    pc = 0x0041_5500;
                    continue;
                }
                0x0041_5500 => {
                    c.cycles += 2;
                    if c.g[S0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        pc = 0x0041_5510;
                        continue;
                    }
                    pc = 0x0041_5508;
                    continue;
                }
                0x0041_5508 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = 0u64;
                    if t {
                        pc = 0x0041_553c;
                        continue;
                    }
                    pc = 0x0041_5510;
                    continue;
                }
                0x0041_5510 => {
                    c.cycles += 11;
                    c.g[A1][0] = 0x410000;
                    c.g[T0][0] = 0x410000;
                    c.g[T1][0] = c.g[S0][0];
                    c.g[A3][0] = c.lh(c.g[V0][0] as u32);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5f48));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x51f8));
                    c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[RA][0] = sx(0x0041_5538);
                    c.g[A2][0] = 0x1;
                    callee::DYNAMIC_CAST.call(c, 0x0041_5538)?;
                    c.g[S0][0] = c.g[V0][0];
                    pc = 0x0041_553c;
                    continue;
                }
                0x0041_553c => {
                    c.cycles += 2;
                    let t = c.g[S0][0] == 0u64;
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    if t {
                        pc = 0x0041_56f8;
                        continue;
                    }
                    pc = 0x0041_5544;
                    continue;
                }
                0x0041_5544 => {
                    c.cycles += 2;
                    if c.g[S1][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                        pc = 0x0041_5554;
                        continue;
                    }
                    pc = 0x0041_554c;
                    continue;
                }
                0x0041_554c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S1][0] = 0u64;
                    if t {
                        pc = 0x0041_5580;
                        continue;
                    }
                    pc = 0x0041_5554;
                    continue;
                }
                0x0041_5554 => {
                    c.cycles += 11;
                    c.g[A1][0] = 0x410000;
                    c.g[T0][0] = 0x410000;
                    c.g[T1][0] = c.g[S1][0];
                    c.g[A3][0] = c.lh(c.g[V0][0] as u32);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5f48));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x51f8));
                    c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[RA][0] = sx(0x0041_557c);
                    c.g[A2][0] = 0x1;
                    callee::DYNAMIC_CAST.call(c, 0x0041_557c)?;
                    c.g[S1][0] = c.g[V0][0];
                    pc = 0x0041_5580;
                    continue;
                }
                0x0041_5580 => {
                    c.cycles += 2;
                    let t = c.g[S1][0] == 0u64;
                    c.g[FP][0] = 0x410000;
                    if t {
                        pc = 0x0041_56f4;
                        continue;
                    }
                    pc = 0x0041_5588;
                    continue;
                }
                0x0041_5588 => {
                    c.cycles += 5;
                    c.g[S1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[S5][0] = 0x410000;
                    let t = true;
                    c.g[S7][0] = 0x410000;
                    if t {
                        pc = 0x0041_5628;
                        continue;
                    }
                    pc = 0x0041_559c;
                    continue;
                }
                0x0041_559c => {
                    c.cycles += 1;
                    pc = 0x0041_55a0;
                    continue;
                }
                0x0041_55a0 => {
                    c.cycles += 2;
                    if c.g[S1][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                        pc = 0x0041_55b0;
                        continue;
                    }
                    pc = 0x0041_55a8;
                    continue;
                }
                0x0041_55a8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S1][0] = 0u64;
                    if t {
                        pc = 0x0041_55d4;
                        continue;
                    }
                    pc = 0x0041_55b0;
                    continue;
                }
                0x0041_55b0 => {
                    c.cycles += 9;
                    c.g[T1][0] = c.g[S1][0];
                    c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x5f48));
                    c.g[A2][0] = 0x1;
                    c.g[A3][0] = c.lh(c.g[V0][0] as u32);
                    c.g[T0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x51f8));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    c.g[RA][0] = sx(0x0041_55d0);
                    c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    callee::DYNAMIC_CAST.call(c, 0x0041_55d0)?;
                    c.g[S1][0] = c.g[V0][0];
                    pc = 0x0041_55d4;
                    continue;
                }
                0x0041_55d4 => {
                    c.cycles += 2;
                    if c.g[S0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        pc = 0x0041_55e4;
                        continue;
                    }
                    pc = 0x0041_55dc;
                    continue;
                }
                0x0041_55dc => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = 0u64;
                    if t {
                        pc = 0x0041_5608;
                        continue;
                    }
                    pc = 0x0041_55e4;
                    continue;
                }
                0x0041_55e4 => {
                    c.cycles += 9;
                    c.g[T1][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x5f48));
                    c.g[A2][0] = 0x1;
                    c.g[A3][0] = c.lh(c.g[V0][0] as u32);
                    c.g[T0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x51f8));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    c.g[RA][0] = sx(0x0041_5604);
                    c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    callee::DYNAMIC_CAST.call(c, 0x0041_5604)?;
                    c.g[S0][0] = c.g[V0][0];
                    pc = 0x0041_5608;
                    continue;
                }
                0x0041_5608 => {
                    c.cycles += 2;
                    let t = c.g[S1][0] == 0u64;
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    if t {
                        pc = 0x0041_56f8;
                        continue;
                    }
                    pc = 0x0041_5610;
                    continue;
                }
                0x0041_5610 => {
                    c.cycles += 2;
                    let t = c.g[S0][0] == 0u64;
                    c.g[V0][0] = c.g[S2][0] ^ 0x1;
                    if t {
                        pc = 0x0041_56f4;
                        continue;
                    }
                    pc = 0x0041_5618;
                    continue;
                }
                0x0041_5618 => {
                    c.cycles += 4;
                    c.g[S1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    if c.g[V0][0] != 0 {
                        c.g[S4][0] = 0u64;
                    }
                    pc = 0x0041_5628;
                    continue;
                }
                0x0041_5628 => {
                    c.cycles += 2;
                    let t = c.g[S0][0] == 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0041_5654;
                        continue;
                    }
                    pc = 0x0041_5630;
                    continue;
                }
                0x0041_5630 => {
                    c.cycles += 9;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x5ed8));
                    c.g[A2][0] = 0x1;
                    c.g[T0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x51f8));
                    c.g[A3][0] = c.lh(c.g[V0][0] as u32);
                    c.g[T1][0] = c.g[S0][0];
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    c.g[RA][0] = sx(0x0041_5654);
                    c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    callee::DYNAMIC_CAST.call(c, 0x0041_5654)?;
                    pc = 0x0041_5654;
                    continue;
                }
                0x0041_5654 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S3][0] = 0u64;
                    if t {
                        pc = 0x0041_5664;
                        continue;
                    }
                    pc = 0x0041_565c;
                    continue;
                }
                0x0041_565c => {
                    c.cycles += 2;
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[S3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    pc = 0x0041_5664;
                    continue;
                }
                0x0041_5664 => {
                    c.cycles += 2;
                    let t = c.g[S1][0] == 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0041_5690;
                        continue;
                    }
                    pc = 0x0041_566c;
                    continue;
                }
                0x0041_566c => {
                    c.cycles += 9;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x5ed8));
                    c.g[A2][0] = 0x1;
                    c.g[T0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x51f8));
                    c.g[A3][0] = c.lh(c.g[V0][0] as u32);
                    c.g[T1][0] = c.g[S1][0];
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    c.g[RA][0] = sx(0x0041_5690);
                    c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    callee::DYNAMIC_CAST.call(c, 0x0041_5690)?;
                    pc = 0x0041_5690;
                    continue;
                }
                0x0041_5690 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S2][0] = 0u64;
                    if t {
                        pc = 0x0041_56a0;
                        continue;
                    }
                    pc = 0x0041_5698;
                    continue;
                }
                0x0041_5698 => {
                    c.cycles += 2;
                    c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[S2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    pc = 0x0041_56a0;
                    continue;
                }
                0x0041_56a0 => {
                    c.cycles += 5;
                    c.g[A2][0] = c.g[S3][0] & 0x1;
                    c.g[A1][0] = c.g[S2][0] & 0x1;
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.g[S3][0] & 0x2;
                    if t {
                        pc = 0x0041_56f4;
                        continue;
                    }
                    pc = 0x0041_56b4;
                    continue;
                }
                0x0041_56b4 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.g[S2][0] & 0x2;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    if t {
                        pc = 0x0041_56f8;
                        continue;
                    }
                    pc = 0x0041_56c4;
                    continue;
                }
                0x0041_56c4 => {
                    c.cycles += 2;
                    if c.g[S4][0] != 0u64 {
                        c.g[A0][0] = c.g[S1][0];
                        pc = 0x0041_56e0;
                        continue;
                    }
                    pc = 0x0041_56cc;
                    continue;
                }
                0x0041_56cc => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if t {
                        pc = 0x0041_56f4;
                        continue;
                    }
                    pc = 0x0041_56d8;
                    continue;
                }
                0x0041_56d8 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.g[S1][0];
                    if t {
                        pc = 0x0041_56f4;
                        continue;
                    }
                    pc = 0x0041_56e0;
                    continue;
                }
                0x0041_56e0 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0041_56e8);
                    c.g[A1][0] = c.g[S0][0];
                    callee::TYPE_INFO_EQ.call(c, 0x0041_56e8)?;
                    let t = c.g[V0][0] == 0u64;
                    if t {
                        if back_edge_unwinds(c, 0x0041_55a0) {
                            return Ok(());
                        }
                        pc = 0x0041_55a0;
                        continue;
                    }
                    pc = 0x0041_56f0;
                    continue;
                }
                0x0041_56f0 => {
                    c.cycles += 1;
                    c.sw(c.g[SP][0] as u32, c.g[S6][0]);
                    pc = 0x0041_56f4;
                    continue;
                }
                0x0041_56f4 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    pc = 0x0041_56f8;
                    continue;
                }
                0x0041_56f8 => {
                    c.cycles += 12;
                    c.g[RA][0] = c.ld(frame.at(c, 0xa0));
                    c.g[FP][0] = c.ld(frame.at(c, 0x90));
                    c.g[S7][0] = c.ld(frame.at(c, 0x80));
                    c.g[S6][0] = c.ld(frame.at(c, 0x70));
                    c.g[S5][0] = c.ld(frame.at(c, 0x60));
                    c.g[S4][0] = c.ld(frame.at(c, 0x50));
                    c.g[S3][0] = c.ld(frame.at(c, 0x40));
                    c.g[S2][0] = c.ld(frame.at(c, 0x30));
                    c.g[S1][0] = c.ld(frame.at(c, 0x20));
                    c.g[S0][0] = c.ld(frame.at(c, 0x10));
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00415260, 0x00415728);
                    return Ok(());
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// __tfd (0x00415ae8)

/// `void __tfd(void)` (0x0041_5ae8, libgcc): install this fundamental
/// type's info into its registry slot when that slot is empty.
pub fn tfd(c: &mut Cpu) {
    'b_415b14: {
        c.cycles += 4;
        c.g[A0][0] = 0x450000;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x41a0));
        let t = c.g[V0][0] != 0u64;
        c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x41a0));
        if t {
            break 'b_415b14;
        } else {
            c.cycles += 2;
            let t = c.g[A1][0] == 0u64;
            c.g[V1][0] = 0x4a0000;
            if t {
                break 'b_415b14;
            } else {
                c.cycles += 5;
                c.g[V0][0] = 0x4a0000;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x64d8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6458));
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.sw((c.g[A0][0] as u32).wrapping_add(0x41a0), c.g[V1][0]);
                break 'b_415b14;
            }
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x41a0));
    c.pc = t;
}

// ---------------------------------------------------------------------------
// __tff (0x00415b20)

/// `void __tff(void)` (0x0041_5b20, libgcc): install this fundamental
/// type's info into its registry slot when that slot is empty.
pub fn tff(c: &mut Cpu) {
    'b_415b4c: {
        c.cycles += 4;
        c.g[A0][0] = 0x450000;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x41a8));
        let t = c.g[V0][0] != 0u64;
        c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x41a8));
        if t {
            break 'b_415b4c;
        } else {
            c.cycles += 2;
            let t = c.g[A1][0] == 0u64;
            c.g[V1][0] = 0x4a0000;
            if t {
                break 'b_415b4c;
            } else {
                c.cycles += 5;
                c.g[V0][0] = 0x4a0000;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x64d0));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6458));
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.sw((c.g[A0][0] as u32).wrapping_add(0x41a8), c.g[V1][0]);
                break 'b_415b4c;
            }
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x41a8));
    c.pc = t;
}

// ---------------------------------------------------------------------------
// __tfui (0x00415b58)

/// `void __tfUi(void)` (0x0041_5b58, libgcc): install this fundamental
/// type's info into its registry slot when that slot is empty.
pub fn tfui(c: &mut Cpu) {
    'b_415b84: {
        c.cycles += 4;
        c.g[A0][0] = 0x450000;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x41b0));
        let t = c.g[V0][0] != 0u64;
        c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x41b0));
        if t {
            break 'b_415b84;
        } else {
            c.cycles += 2;
            let t = c.g[A1][0] == 0u64;
            c.g[V1][0] = 0x4a0000;
            if t {
                break 'b_415b84;
            } else {
                c.cycles += 5;
                c.g[V0][0] = 0x4a0000;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x64c8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6458));
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.sw((c.g[A0][0] as u32).wrapping_add(0x41b0), c.g[V1][0]);
                break 'b_415b84;
            }
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x41b0));
    c.pc = t;
}

// ---------------------------------------------------------------------------
// __tful (0x00415b90)

/// `void __tfUl(void)` (0x0041_5b90, libgcc): install this fundamental
/// type's info into its registry slot when that slot is empty.
pub fn tful(c: &mut Cpu) {
    'b_415bbc: {
        c.cycles += 4;
        c.g[A0][0] = 0x450000;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x41b8));
        let t = c.g[V0][0] != 0u64;
        c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x41b8));
        if t {
            break 'b_415bbc;
        } else {
            c.cycles += 2;
            let t = c.g[A1][0] == 0u64;
            c.g[V1][0] = 0x4a0000;
            if t {
                break 'b_415bbc;
            } else {
                c.cycles += 5;
                c.g[V0][0] = 0x4a0000;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x64c0));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6458));
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.sw((c.g[A0][0] as u32).wrapping_add(0x41b8), c.g[V1][0]);
                break 'b_415bbc;
            }
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x41b8));
    c.pc = t;
}

// ---------------------------------------------------------------------------
// __tfux (0x00415bc8)

/// `void __tfUx(void)` (0x0041_5bc8, libgcc): install this fundamental
/// type's info into its registry slot when that slot is empty.
pub fn tfux(c: &mut Cpu) {
    'b_415bf4: {
        c.cycles += 4;
        c.g[A0][0] = 0x450000;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x41c0));
        let t = c.g[V0][0] != 0u64;
        c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x41c0));
        if t {
            break 'b_415bf4;
        } else {
            c.cycles += 2;
            let t = c.g[A1][0] == 0u64;
            c.g[V1][0] = 0x4a0000;
            if t {
                break 'b_415bf4;
            } else {
                c.cycles += 5;
                c.g[V0][0] = 0x4a0000;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x64b8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6458));
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.sw((c.g[A0][0] as u32).wrapping_add(0x41c0), c.g[V1][0]);
                break 'b_415bf4;
            }
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x41c0));
    c.pc = t;
}

// ---------------------------------------------------------------------------
// __tfus (0x00415c00)

/// `void __tfUs(void)` (0x0041_5c00, libgcc): install this fundamental
/// type's info into its registry slot when that slot is empty.
pub fn tfus(c: &mut Cpu) {
    'b_415c2c: {
        c.cycles += 4;
        c.g[A0][0] = 0x450000;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x41c8));
        let t = c.g[V0][0] != 0u64;
        c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x41c8));
        if t {
            break 'b_415c2c;
        } else {
            c.cycles += 2;
            let t = c.g[A1][0] == 0u64;
            c.g[V1][0] = 0x4a0000;
            if t {
                break 'b_415c2c;
            } else {
                c.cycles += 5;
                c.g[V0][0] = 0x4a0000;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x64b0));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6458));
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.sw((c.g[A0][0] as u32).wrapping_add(0x41c8), c.g[V1][0]);
                break 'b_415c2c;
            }
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x41c8));
    c.pc = t;
}

// ---------------------------------------------------------------------------
// __tfuc (0x00415c38)

/// `void __tfUc(void)` (0x0041_5c38, libgcc): install this fundamental
/// type's info into its registry slot when that slot is empty.
pub fn tfuc(c: &mut Cpu) {
    'b_415c64: {
        c.cycles += 4;
        c.g[A0][0] = 0x450000;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x41d0));
        let t = c.g[V0][0] != 0u64;
        c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x41d0));
        if t {
            break 'b_415c64;
        } else {
            c.cycles += 2;
            let t = c.g[A1][0] == 0u64;
            c.g[V1][0] = 0x4a0000;
            if t {
                break 'b_415c64;
            } else {
                c.cycles += 5;
                c.g[V0][0] = 0x4a0000;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x64a8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6458));
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.sw((c.g[A0][0] as u32).wrapping_add(0x41d0), c.g[V1][0]);
                break 'b_415c64;
            }
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x41d0));
    c.pc = t;
}

// ---------------------------------------------------------------------------
// __tfsc (0x00415c70)

/// `void __tfSc(void)` (0x0041_5c70, libgcc): install this fundamental
/// type's info into its registry slot when that slot is empty.
pub fn tfsc(c: &mut Cpu) {
    'b_415c9c: {
        c.cycles += 4;
        c.g[A0][0] = 0x450000;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x41d8));
        let t = c.g[V0][0] != 0u64;
        c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x41d8));
        if t {
            break 'b_415c9c;
        } else {
            c.cycles += 2;
            let t = c.g[A1][0] == 0u64;
            c.g[V1][0] = 0x4a0000;
            if t {
                break 'b_415c9c;
            } else {
                c.cycles += 5;
                c.g[V0][0] = 0x4a0000;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x64a0));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6458));
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.sw((c.g[A0][0] as u32).wrapping_add(0x41d8), c.g[V1][0]);
                break 'b_415c9c;
            }
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x41d8));
    c.pc = t;
}

// ---------------------------------------------------------------------------
// __ieee754_fmod (0x0041ef50)

/// /// `double __ieee754_fmod(double x, double y)` (0x41ef50): floating-point remainder. Kept in the generated dispatcher shape.
pub fn ieee754_fmod(c: &mut Cpu) {
    call::guest(c, |c| {
        let mut pc: u32 = c.pc;
        let frame = Frame { size: 0x40 };
        loop {
            match pc {
                0x0041_ef50 => {
                    c.cycles += 25;
                    Frame::open(c, 0x40);
                    c.sq(17, frame.at(c, 0x20));
                    c.sq(18, frame.at(c, 0x10));
                    c.g[S1][0] = c.g[A0][0];
                    c.g[S2][0] = c.g[A1][0];
                    c.sq(16, frame.at(c, 0x30));
                    c.sd(frame.at(c, 0), c.g[RA][0]);
                    c.g[V0][0] = c.g[S1][0];
                    c.g[T0][0] = c.g[V0][0] << 32;
                    c.g[T0][0] = ((c.g[T0][0] as i64) >> 32) as u64;
                    c.g[A3][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                    c.g[T6][0] = c.g[T0][0];
                    c.g[V0][0] = c.g[S2][0];
                    c.g[T2][0] = c.g[V0][0] << 32;
                    c.g[T2][0] = ((c.g[T2][0] as i64) >> 32) as u64;
                    c.g[T1][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                    c.g[T7][0] = c.g[T2][0];
                    c.g[V1][0] = 0xffffffff80000000;
                    c.g[V0][0] = 0x7fff0000;
                    c.g[T5][0] = c.g[A3][0] & c.g[V1][0];
                    c.g[V0][0] = c.g[V0][0] | 0xffff;
                    c.g[T1][0] = c.g[T1][0] & c.g[V0][0];
                    c.g[V1][0] = c.g[T1][0] | c.g[T2][0];
                    let t = c.g[V1][0] == 0u64;
                    c.g[A3][0] = c.g[A3][0] ^ c.g[T5][0];
                    if t {
                        pc = 0x0041_efe8;
                        continue;
                    }
                    pc = 0x0041_efb4;
                    continue;
                }
                0x0041_efb4 => {
                    c.cycles += 5;
                    c.g[V0][0] = 0x7fef0000;
                    c.g[V0][0] = c.g[V0][0] | 0xffff;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A3][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.g[S1][0];
                    if t {
                        pc = 0x0041_efec;
                        continue;
                    }
                    pc = 0x0041_efc8;
                    continue;
                }
                0x0041_efc8 => {
                    c.cycles += 8;
                    c.g[V0][0] = sx(0u32.wrapping_sub(c.g[T2][0] as u32));
                    c.g[V1][0] = 0x7ff00000;
                    c.g[V0][0] = c.g[T2][0] | c.g[V0][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 31);
                    c.g[V0][0] = c.g[T1][0] | c.g[V0][0];
                    c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[A3][0] as i64)) as u64;
                    if t {
                        pc = 0x0041_f018;
                        continue;
                    }
                    pc = 0x0041_efe8;
                    continue;
                }
                0x0041_efe8 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S1][0];
                    pc = 0x0041_efec;
                    continue;
                }
                0x0041_efec => {
                    c.cycles += 11;
                    c.g[RA][0] = sx(0x0041_eff4);
                    c.g[A1][0] = c.g[S2][0];
                    MULDF3.call(c, 0x0041_eff4)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x0041_f004);
                    c.g[A1][0] = c.g[S2][0];
                    MULDF3.call(c, 0x0041_f004)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0041_f010);
                    c.g[A1][0] = c.g[V0][0];
                    callee::IEEE_LOG_DIVDF3.call(c, 0x0041_f010)?;
                    let t = true;
                    c.lq(16, frame.at(c, 0x30));
                    if t {
                        pc = 0x0041_f390;
                        continue;
                    }
                    pc = 0x0041_f018;
                    continue;
                }
                0x0041_f018 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0xf0000;
                    if t {
                        pc = 0x0041_f044;
                        continue;
                    }
                    pc = 0x0041_f020;
                    continue;
                }
                0x0041_f020 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[T1][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.g[S1][0];
                    if t {
                        pc = 0x0041_f38c;
                        continue;
                    }
                    pc = 0x0041_f02c;
                    continue;
                }
                0x0041_f02c => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[T0][0] < c.g[T2][0]) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.g[S1][0];
                        pc = 0x0041_f38c;
                        continue;
                    }
                    pc = 0x0041_f038;
                    continue;
                }
                0x0041_f038 => {
                    c.cycles += 2;
                    let t = c.g[T0][0] == c.g[T2][0];
                    c.g[V1][0] = sx((c.g[T5][0] as u32) >> 31);
                    if t {
                        pc = 0x0041_f334;
                        continue;
                    }
                    pc = 0x0041_f040;
                    continue;
                }
                0x0041_f040 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0xf0000;
                    pc = 0x0041_f044;
                    continue;
                }
                0x0041_f044 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.g[V0][0] | 0xffff;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A3][0] as i64)) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = sx(((c.g[A3][0] as u32) as i32 >> 20) as u32);
                        pc = 0x0041_f0bc;
                        continue;
                    }
                    pc = 0x0041_f054;
                    continue;
                }
                0x0041_f054 => {
                    c.cycles += 2;
                    let t = c.g[A3][0] != 0u64;
                    c.g[V0][0] = sx((c.g[A3][0] as u32) << 11);
                    if t {
                        pc = 0x0041_f08c;
                        continue;
                    }
                    pc = 0x0041_f05c;
                    continue;
                }
                0x0041_f05c => {
                    c.cycles += 3;
                    c.g[A1][0] = 0xfffffffffffffbed;
                    let t = (c.g[T6][0] as i64) <= 0;
                    c.g[V0][0] = c.g[T6][0];
                    if t {
                        pc = 0x0041_f0c0;
                        continue;
                    }
                    pc = 0x0041_f068;
                    continue;
                }
                0x0041_f068 => {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                    let t = (c.g[V0][0] as i64) > 0;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x0041_f068) {
                            return Ok(());
                        }
                        pc = 0x0041f068;
                        continue;
                    }
                    pc = 0x0041_f084;
                    continue;
                }
                0x0041_f084 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0xf0000;
                    if t {
                        pc = 0x0041_f0c4;
                        continue;
                    }
                    pc = 0x0041_f08c;
                    continue;
                }
                0x0041_f08c => {
                    c.cycles += 2;
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[A1][0] = 0xfffffffffffffc02;
                    if t {
                        pc = 0x0041_f0c0;
                        continue;
                    }
                    pc = 0x0041_f094;
                    continue;
                }
                0x0041_f094 => {
                    c.cycles += 1;
                    pc = 0x0041_f098;
                    continue;
                }
                0x0041_f098 => {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                    let t = (c.g[V0][0] as i64) > 0;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x0041_f098) {
                            return Ok(());
                        }
                        pc = 0x0041f098;
                        continue;
                    }
                    pc = 0x0041_f0b4;
                    continue;
                }
                0x0041_f0b4 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0xf0000;
                    if t {
                        pc = 0x0041_f0c4;
                        continue;
                    }
                    pc = 0x0041_f0bc;
                    continue;
                }
                0x0041_f0bc => {
                    c.cycles += 1;
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3ff));
                    pc = 0x0041_f0c0;
                    continue;
                }
                0x0041_f0c0 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0xf0000;
                    pc = 0x0041_f0c4;
                    continue;
                }
                0x0041_f0c4 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.g[V0][0] | 0xffff;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[T1][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx(((c.g[T1][0] as u32) as i32 >> 20) as u32);
                    if t {
                        pc = 0x0041_f144;
                        continue;
                    }
                    pc = 0x0041_f0d4;
                    continue;
                }
                0x0041_f0d4 => {
                    c.cycles += 2;
                    let t = c.g[T1][0] != 0u64;
                    c.g[A0][0] = 0xfffffffffffffc02;
                    if t {
                        pc = 0x0041_f114;
                        continue;
                    }
                    pc = 0x0041_f0dc;
                    continue;
                }
                0x0041_f0dc => {
                    c.cycles += 4;
                    c.g[A0][0] = 0xfffffffffffffbed;
                    c.g[V0][0] = c.g[T7][0];
                    let t = (c.g[T7][0] as i64) <= 0;
                    c.g[V1][0] = ((c.g[A1][0] as i64) < -1022i64) as u64;
                    if t {
                        pc = 0x0041_f14c;
                        continue;
                    }
                    pc = 0x0041_f0ec;
                    continue;
                }
                0x0041_f0ec => {
                    c.cycles += 1;
                    pc = 0x0041_f0f0;
                    continue;
                }
                0x0041_f0f0 => {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                    let t = (c.g[V0][0] as i64) > 0;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x0041_f0f0) {
                            return Ok(());
                        }
                        pc = 0x0041f0f0;
                        continue;
                    }
                    pc = 0x0041_f10c;
                    continue;
                }
                0x0041_f10c => {
                    c.cycles += 2;
                    let t = true;
                    if t {
                        pc = 0x0041_f14c;
                        continue;
                    }
                    pc = 0x0041_f114;
                    continue;
                }
                0x0041_f114 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[T1][0] as u32) << 11);
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[V1][0] = ((c.g[A1][0] as i64) < -1022i64) as u64;
                    if t {
                        pc = 0x0041_f14c;
                        continue;
                    }
                    pc = 0x0041_f120;
                    continue;
                }
                0x0041_f120 => {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                    let t = (c.g[V0][0] as i64) > 0;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x0041_f120) {
                            return Ok(());
                        }
                        pc = 0x0041f120;
                        continue;
                    }
                    pc = 0x0041_f13c;
                    continue;
                }
                0x0041_f13c => {
                    c.cycles += 2;
                    let t = true;
                    if t {
                        pc = 0x0041_f14c;
                        continue;
                    }
                    pc = 0x0041_f144;
                    continue;
                }
                0x0041_f144 => {
                    c.cycles += 2;
                    c.g[V1][0] = ((c.g[A1][0] as i64) < -1022i64) as u64;
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3ff));
                    pc = 0x0041_f14c;
                    continue;
                }
                0x0041_f14c => {
                    c.cycles += 2;
                    let t = c.g[V1][0] != 0u64;
                    c.g[V0][0] = 0xfffffffffffffc02;
                    if t {
                        pc = 0x0041_f16c;
                        continue;
                    }
                    pc = 0x0041_f154;
                    continue;
                }
                0x0041_f154 => {
                    c.cycles += 6;
                    c.g[V0][0] = 0xf0000;
                    c.g[V1][0] = 0x100000;
                    c.g[V0][0] = c.g[V0][0] | 0xffff;
                    c.g[V0][0] = c.g[A3][0] & c.g[V0][0];
                    let t = true;
                    c.g[A3][0] = c.g[V0][0] | c.g[V1][0];
                    if t {
                        pc = 0x0041_f198;
                        continue;
                    }
                    pc = 0x0041_f16c;
                    continue;
                }
                0x0041_f16c => {
                    c.cycles += 4;
                    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A1][0] as u32));
                    c.g[V1][0] = ((c.g[A2][0] as i64) < 32i64) as u64;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = sx(0u32.wrapping_sub(c.g[A2][0] as u32));
                    if t {
                        pc = 0x0041_f190;
                        continue;
                    }
                    pc = 0x0041_f17c;
                    continue;
                }
                0x0041_f17c => {
                    c.cycles += 5;
                    c.g[V1][0] = sx((c.g[A3][0] as u32) << ((c.g[A2][0] as u32) & 31));
                    c.g[V0][0] = sx((c.g[T6][0] as u32) >> ((c.g[V0][0] as u32) & 31));
                    c.g[T0][0] = sx((c.g[T6][0] as u32) << ((c.g[A2][0] as u32) & 31));
                    let t = true;
                    c.g[A3][0] = c.g[V1][0] | c.g[V0][0];
                    if t {
                        pc = 0x0041_f198;
                        continue;
                    }
                    pc = 0x0041_f190;
                    continue;
                }
                0x0041_f190 => {
                    c.cycles += 2;
                    c.g[A3][0] = sx((c.g[T6][0] as u32) << ((c.g[A2][0] as u32) & 31));
                    c.g[T0][0] = 0u64;
                    pc = 0x0041_f198;
                    continue;
                }
                0x0041_f198 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < -1022i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0xfffffffffffffc02;
                    if t {
                        pc = 0x0041_f1bc;
                        continue;
                    }
                    pc = 0x0041_f1a4;
                    continue;
                }
                0x0041_f1a4 => {
                    c.cycles += 6;
                    c.g[V0][0] = 0xf0000;
                    c.g[V1][0] = 0x100000;
                    c.g[V0][0] = c.g[V0][0] | 0xffff;
                    c.g[V0][0] = c.g[T1][0] & c.g[V0][0];
                    let t = true;
                    c.g[T1][0] = c.g[V0][0] | c.g[V1][0];
                    if t {
                        pc = 0x0041_f1e8;
                        continue;
                    }
                    pc = 0x0041_f1bc;
                    continue;
                }
                0x0041_f1bc => {
                    c.cycles += 4;
                    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                    c.g[V1][0] = ((c.g[A2][0] as i64) < 32i64) as u64;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = sx(0u32.wrapping_sub(c.g[A2][0] as u32));
                    if t {
                        pc = 0x0041_f1e0;
                        continue;
                    }
                    pc = 0x0041_f1cc;
                    continue;
                }
                0x0041_f1cc => {
                    c.cycles += 5;
                    c.g[V1][0] = sx((c.g[T1][0] as u32) << ((c.g[A2][0] as u32) & 31));
                    c.g[V0][0] = sx((c.g[T7][0] as u32) >> ((c.g[V0][0] as u32) & 31));
                    c.g[T2][0] = sx((c.g[T7][0] as u32) << ((c.g[A2][0] as u32) & 31));
                    let t = true;
                    c.g[T1][0] = c.g[V1][0] | c.g[V0][0];
                    if t {
                        pc = 0x0041_f1e8;
                        continue;
                    }
                    pc = 0x0041_f1e0;
                    continue;
                }
                0x0041_f1e0 => {
                    c.cycles += 2;
                    c.g[T1][0] = sx((c.g[T7][0] as u32) << ((c.g[A2][0] as u32) & 31));
                    c.g[T2][0] = 0u64;
                    pc = 0x0041_f1e8;
                    continue;
                }
                0x0041_f1e8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                    if t {
                        pc = 0x0041_f230;
                        continue;
                    }
                    pc = 0x0041_f1f0;
                    continue;
                }
                0x0041_f1f0 => {
                    c.cycles += 5;
                    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_sub(c.g[T1][0] as u32));
                    c.g[V0][0] = (c.g[T0][0] < c.g[T2][0]) as u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    let t = (c.g[V1][0] as i64) >= 0;
                    c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_sub(c.g[T2][0] as u32));
                    if t {
                        pc = 0x0041_f218;
                        continue;
                    }
                    pc = 0x0041_f204;
                    continue;
                }
                0x0041_f204 => {
                    c.cycles += 5;
                    c.g[V0][0] = sx((c.g[T0][0] as u32) >> 31);
                    c.g[V1][0] = sx((c.g[A3][0] as u32) << 1);
                    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let t = true;
                    c.g[T0][0] = sx((c.g[T0][0] as u32) << 1);
                    if t {
                        pc = 0x0041_f230;
                        continue;
                    }
                    pc = 0x0041_f218;
                    continue;
                }
                0x0041_f218 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V1][0] | c.g[A1][0];
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
                    if t {
                        pc = 0x0041_f330;
                        continue;
                    }
                    pc = 0x0041_f224;
                    continue;
                }
                0x0041_f224 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[A1][0] as u32) >> 31);
                    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[T0][0] = sx((c.g[A1][0] as u32) << 1);
                    pc = 0x0041_f230;
                    continue;
                }
                0x0041_f230 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[A2][0];
                    let t = c.g[V0][0] != 0u64;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x0041_f1f0) {
                            return Ok(());
                        }
                        pc = 0x0041f1f0;
                        continue;
                    }
                    pc = 0x0041_f23c;
                    continue;
                }
                0x0041_f23c => {
                    c.cycles += 5;
                    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_sub(c.g[T1][0] as u32));
                    c.g[V0][0] = (c.g[T0][0] < c.g[T2][0]) as u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    let t = (c.g[V1][0] as i64) < 0;
                    c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_sub(c.g[T2][0] as u32));
                    if t {
                        pc = 0x0041_f258;
                        continue;
                    }
                    pc = 0x0041_f250;
                    continue;
                }
                0x0041_f250 => {
                    c.cycles += 2;
                    c.g[A3][0] = c.g[V1][0];
                    c.g[T0][0] = c.g[A1][0];
                    pc = 0x0041_f258;
                    continue;
                }
                0x0041_f258 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[A3][0] | c.g[T0][0];
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = sx((c.g[T5][0] as u32) >> 31);
                    if t {
                        pc = 0x0041_f334;
                        continue;
                    }
                    pc = 0x0041_f264;
                    continue;
                }
                0x0041_f264 => {
                    c.cycles += 5;
                    c.g[V0][0] = 0xf0000;
                    c.g[V0][0] = c.g[V0][0] | 0xffff;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A3][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < -1022i64) as u64;
                    if t {
                        pc = 0x0041_f2a0;
                        continue;
                    }
                    pc = 0x0041_f278;
                    continue;
                }
                0x0041_f278 => {
                    c.cycles += 2;
                    c.g[A1][0] = 0xf0000;
                    c.g[A1][0] = c.g[A1][0] | 0xffff;
                    pc = 0x0041_f280;
                    continue;
                }
                0x0041_f280 => {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[A3][0] as u32) << 1);
                    c.g[V1][0] = sx((c.g[T0][0] as u32) >> 31);
                    c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[T0][0] = sx((c.g[T0][0] as u32) << 1);
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A3][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x0041_f280) {
                            return Ok(());
                        }
                        pc = 0x0041f280;
                        continue;
                    }
                    pc = 0x0041_f29c;
                    continue;
                }
                0x0041_f29c => {
                    c.cycles += 1;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < -1022i64) as u64;
                    pc = 0x0041_f2a0;
                    continue;
                }
                0x0041_f2a0 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0xfffffffffffffc02;
                    if t {
                        pc = 0x0041_f2f0;
                        continue;
                    }
                    pc = 0x0041_f2a8;
                    continue;
                }
                0x0041_f2a8 => {
                    c.cycles += 18;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3ff));
                    c.g[V1][0] = 0xfffffffffff00000;
                    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 20);
                    c.g[A3][0] = c.g[V1][0] | c.g[A0][0];
                    c.g[V0][0] = c.g[A3][0] | c.g[T5][0];
                    c.g[A0][0] = 0xffffffffffffffff;
                    c.g[A0][0] = c.g[A0][0] >> 32;
                    c.g[V0][0] = c.g[V0][0] << 32;
                    c.g[T3][0] = c.g[T3][0] & c.g[A0][0];
                    c.g[V1][0] = c.g[T0][0] << 32;
                    c.g[T3][0] = c.g[T3][0] | c.g[V0][0];
                    c.g[A0][0] = 0xffffffffffffffff;
                    c.g[A0][0] = c.g[A0][0] << 32;
                    c.g[V1][0] = c.g[V1][0] >> 32;
                    c.g[T3][0] = c.g[T3][0] & c.g[A0][0];
                    let t = true;
                    c.g[S1][0] = c.g[T3][0] | c.g[V1][0];
                    if t {
                        pc = 0x0041_f388;
                        continue;
                    }
                    pc = 0x0041_f2f0;
                    continue;
                }
                0x0041_f2f0 => {
                    c.cycles += 4;
                    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                    c.g[V1][0] = ((c.g[A2][0] as i64) < 21i64) as u64;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = sx(0u32.wrapping_sub(c.g[A2][0] as u32));
                    if t {
                        pc = 0x0041_f314;
                        continue;
                    }
                    pc = 0x0041_f300;
                    continue;
                }
                0x0041_f300 => {
                    c.cycles += 5;
                    c.g[V1][0] = sx((c.g[T0][0] as u32) >> ((c.g[A2][0] as u32) & 31));
                    c.g[V0][0] = sx((c.g[A3][0] as u32) << ((c.g[V0][0] as u32) & 31));
                    c.g[T0][0] = c.g[V1][0] | c.g[V0][0];
                    let t = true;
                    c.g[A3][0] =
                        sx(((c.g[A3][0] as u32) as i32 >> ((c.g[A2][0] as u32) & 31)) as u32);
                    if t {
                        pc = 0x0041_f354;
                        continue;
                    }
                    pc = 0x0041_f314;
                    continue;
                }
                0x0041_f314 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A2][0] as i64) < 32i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx(0u32.wrapping_sub(c.g[A2][0] as u32));
                    if t {
                        pc = 0x0041_f34c;
                        continue;
                    }
                    pc = 0x0041_f320;
                    continue;
                }
                0x0041_f320 => {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[T0][0] as u32) >> ((c.g[A2][0] as u32) & 31));
                    c.g[V0][0] = sx((c.g[A3][0] as u32) << ((c.g[V0][0] as u32) & 31));
                    let t = true;
                    c.g[T0][0] = c.g[V0][0] | c.g[V1][0];
                    if t {
                        pc = 0x0041_f350;
                        continue;
                    }
                    pc = 0x0041_f330;
                    continue;
                }
                0x0041_f330 => {
                    c.cycles += 1;
                    c.g[V1][0] = sx((c.g[T5][0] as u32) >> 31);
                    pc = 0x0041_f334;
                    continue;
                }
                0x0041_f334 => {
                    c.cycles += 6;
                    c.g[V0][0] = 0x4a0000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x57f8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let t = true;
                    c.g[V0][0] = c.ld(c.g[V1][0] as u32);
                    if t {
                        pc = 0x0041_f38c;
                        continue;
                    }
                    pc = 0x0041_f34c;
                    continue;
                }
                0x0041_f34c => {
                    c.cycles += 1;
                    c.g[T0][0] =
                        sx(((c.g[A3][0] as u32) as i32 >> ((c.g[A2][0] as u32) & 31)) as u32);
                    pc = 0x0041_f350;
                    continue;
                }
                0x0041_f350 => {
                    c.cycles += 1;
                    c.g[A3][0] = c.g[T5][0];
                    pc = 0x0041_f354;
                    continue;
                }
                0x0041_f354 => {
                    c.cycles += 13;
                    c.g[V1][0] = 0xffffffffffffffff;
                    c.g[V1][0] = c.g[V1][0] >> 32;
                    c.g[V0][0] = c.g[A3][0] | c.g[T5][0];
                    c.g[V0][0] = c.g[V0][0] << 32;
                    c.g[T4][0] = c.g[T4][0] & c.g[V1][0];
                    c.g[T4][0] = c.g[T4][0] | c.g[V0][0];
                    c.g[V1][0] = c.g[T0][0] << 32;
                    c.g[V0][0] = 0xffffffffffffffff;
                    c.g[V0][0] = c.g[V0][0] << 32;
                    c.g[V1][0] = c.g[V1][0] >> 32;
                    c.g[T4][0] = c.g[T4][0] & c.g[V0][0];
                    c.g[S1][0] = c.g[T4][0] | c.g[V1][0];
                    pc = 0x0041_f388;
                    continue;
                }
                0x0041_f388 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S1][0];
                    pc = 0x0041_f38c;
                    continue;
                }
                0x0041_f38c => {
                    c.cycles += 1;
                    c.lq(16, frame.at(c, 0x30));
                    pc = 0x0041_f390;
                    continue;
                }
                0x0041_f390 => {
                    c.cycles += 5;
                    c.lq(17, frame.at(c, 0x20));
                    c.lq(18, frame.at(c, 0x10));
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0041ef50, 0x0041f3a4);
                    return Ok(());
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// __ieee754_log10 (0x0041f9d8)

/// `double __ieee754_log10(double x)` (0x41f9d8): base-10 logarithm via
/// fdlibm's decomposition: scale the natural-log helper output by
/// 1/ln(10) and add the exponent's `n * log10(2)`, split across hi/lo
/// constants. Kept in the generated block shape.
pub fn ieee754_log10(c: &mut Cpu) {
    call::guest(c, |c| {
        let frame = Frame::open(c, 0x40);
        c.cycles += 15;
        frame.save(c, 17, 0x20);
        frame.save(c, 16, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 18, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.g[S1][0];
        c.g[S0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
        c.g[V1][0] = c.g[V0][0] << 32;
        c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
        c.g[V0][0] = 0xf0000;
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
        let t = c.g[V0][0] != 0u64;
        c.g[S2][0] = 0u64;
        if !t {
            c.cycles += 6;
            c.g[V0][0] = 0x7fff0000;
            c.g[V0][0] = c.g[V0][0] | 0xffff;
            c.g[V0][0] = c.g[S0][0] & c.g[V0][0];
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            let t = c.g[V0][0] != 0u64;
            c.g[V1][0] = 0x4a0000;
            if !t {
                c.cycles += 2;
                c.g[V0][0] = c.ld((c.g[V1][0] as u32).wrapping_sub(0x5770));
                c.cycles += 1;
                frame.restore(c, 16, 0x30);
                c.cycles += 5;
                frame.restore(c, 17, 0x20);
                frame.restore(c, 18, 0x10);
                frame.restore_ra(c, 0);
                let t = c.g[RA][0] as u32;
                frame.close(c);
                c.pc = t;
                return Ok(());
            }
            c.cycles += 2;
            let t = (c.g[S0][0] as i64) >= 0;
            c.g[A0][0] = c.g[S1][0];
            if !t {
                c.cycles += 7;
                c.g[RA][0] = sx(0x0041_fa48);
                c.g[A1][0] = c.g[A0][0];
                SUBDF3.call(c, 0x0041_fa48)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0041_fa54);
                c.g[A1][0] = 0u64;
                callee::IEEE_LOG_DIVDF3.call(c, 0x0041_fa54)?;
                frame.restore(c, 16, 0x30);
                c.cycles += 5;
                frame.restore(c, 17, 0x20);
                frame.restore(c, 18, 0x10);
                frame.restore_ra(c, 0);
                let t = c.g[RA][0] as u32;
                frame.close(c);
                c.pc = t;
                return Ok(());
            }
            c.cycles += 7;
            c.g[A1][0] = 0x86a0;
            c.g[A1][0] = c.g[A1][0] << 47;
            c.g[RA][0] = sx(0x0041_fa6c);
            c.g[S2][0] = 0xffffffffffffffca;
            MULDF3.call(c, 0x0041_fa6c)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[S0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
        }
        c.cycles += 5;
        c.g[V0][0] = 0x7fef0000;
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
        if c.g[V0][0] != 0u64 {
            c.g[A0][0] = c.g[S1][0];
            c.cycles += 2;
            c.g[A1][0] = c.g[A0][0];
        } else {
            c.cycles += 57;
            c.g[V0][0] = sx(((c.g[S0][0] as u32) as i32 >> 20) as u32);
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x3ff));
            c.g[S2][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = 0xf0000;
            c.g[A0][0] = sx((c.g[S2][0] as u32) >> 31);
            c.g[V0][0] = 0x3ff;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A0][0] as u32));
            c.g[V1][0] = c.g[V1][0] | 0xffff;
            c.g[V1][0] = c.g[S0][0] & c.g[V1][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 20);
            c.g[S0][0] = c.g[V1][0] | c.g[V0][0];
            c.g[RA][0] = sx(0x0041_fac0);
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            FLOATSIDF.call(c, 0x0041_fac0)?;
            c.g[V1][0] = c.g[S1][0];
            c.g[A0][0] = 0xffffffffffffffff;
            c.g[A0][0] = c.g[A0][0] >> 32;
            c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
            c.g[A1][0] = c.g[S0][0] << 32;
            c.g[S1][0] = c.g[V1][0] | c.g[A1][0];
            c.g[S2][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0041_fae4);
            c.g[A0][0] = c.g[S1][0];
            callee::IEEE754_LOG_2.call(c, 0x0041_fae4)?;
            c.g[A1][0] = 0xf567;
            c.g[A1][0] = c.g[A1][0] << 16;
            c.g[A1][0] = c.g[A1][0] | 0xfbcc;
            c.g[A1][0] = c.g[A1][0] << 17;
            c.g[A1][0] = c.g[A1][0] | 0x8f89;
            c.g[A1][0] = c.g[A1][0] << 13;
            c.g[A1][0] = c.g[A1][0] | 0xb36;
            c.g[S1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0041_fb0c);
            c.g[A0][0] = c.g[S2][0];
            MULDF3.call(c, 0x0041_fb0c)?;
            c.g[A1][0] = 0xff6f;
            c.g[A1][0] = c.g[A1][0] << 18;
            c.g[A1][0] = c.g[A1][0] | 0xb7b1;
            c.g[A1][0] = c.g[A1][0] << 17;
            c.g[A1][0] = c.g[A1][0] | 0xa4dc;
            c.g[A1][0] = c.g[A1][0] << 11;
            c.g[A1][0] = c.g[A1][0] | 0x50e;
            c.g[S0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0041_fb34);
            c.g[A0][0] = c.g[S1][0];
            MULDF3.call(c, 0x0041_fb34)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0041_fb40);
            c.g[A1][0] = c.g[V0][0];
            ADDDF3.call(c, 0x0041_fb40)?;
            c.g[A1][0] = 0xff4d;
            c.g[A1][0] = c.g[A1][0] << 19;
            c.g[A1][0] = c.g[A1][0] | 0x826a;
            c.g[A1][0] = c.g[A1][0] << 19;
            c.g[A1][0] = c.g[A1][0] | 0x9f60;
            c.g[A1][0] = c.g[A1][0] << 8;
            c.g[S0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0041_fb64);
            c.g[A0][0] = c.g[S2][0];
            MULDF3.call(c, 0x0041_fb64)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
        }
        c.cycles += 2;
        c.g[RA][0] = sx(0x0041_fb80);
        ADDDF3.call(c, 0x0041_fb80)?;
        c.cycles += 1;
        frame.restore(c, 16, 0x30);
        c.cycles += 5;
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        return Ok(());
    })
}

/// `void *new_eh_context(void)` (0x410410): allocate an 0x18-byte exception
/// context and zero it with `memset`; `malloc` failure is fatal
/// (`__terminate`). Returns the block, whose word at +4 points 0x10 past it.
pub fn new_eh_context(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        frame.save_ra(c, 0x10);
        c.g[A0][0] = 0x18;
        callee::MALLOC.call(c, 0x0041_0424)?;
        c.g[S0][0] = c.g[V0][0];
        // The null-check delay slot runs on both arms: A0 = S0 before
        // the (possibly noreturn) `__terminate` call.
        c.g[A0][0] = c.g[S0][0];
        if c.g[S0][0] == 0 {
            c.cycles += 3;
            callee::TERMINATE.call(c, 0x0041_0438)?;
            c.g[A0][0] = c.g[S0][0];
        }
        c.cycles += 10;
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x18;
        callee::MEMSET.call(c, 0x0041_0448)?;
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.g[S0][0];
        Addr(c.g[S0][0] as u32).off(0x4).sw(c, c.g[V1][0]);
        c.g[RA][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        Ok(())
    })
}

/// `void *old_find_exception_handler(uint32 pc, void *table)` (0x410858):
/// scan the old-format handler table for the entry covering `pc` in `a0`.
/// The table at `a1` starts with a sentinel word; each 12-byte entry holds
/// a pc range and handler words. Returns the handler word (`v0 + 8`) of
/// the recorded entry, or 0. Null table returns 0.
pub fn old_find_exception_handler(c: &mut Cpu) {
    c.cycles += 3;
    c.g[T5][0] = c.g[A1][0];
    let t = c.g[T5][0] == 0u64;
    c.g[T0][0] = 0xffffffffffffffff;
    if t {
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[V0][0] = 0u64;
        c.pc = t;
        return;
    } else {
        c.cycles += 5;
        c.g[V0][0] = 0xffffffffffff0000;
        c.g[V1][0] = c.lw(c.g[T5][0] as u32);
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        let t = c.g[V1][0] == c.g[V0][0];
        c.g[T2][0] = 0u64;
        if !t {
            c.cycles += 6;
            c.g[T1][0] = 0xffffffffffff0000;
            c.g[T4][0] = 0xc;
            c.g[T3][0] = 0xffffffffffffffff;
            c.g[T1][0] = c.g[T1][0] | 0xffff;
            c.g[A2][0] = c.g[T5][0];
            let mut first = true;
            'l_410890: loop {
                if first {
                    first = false;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                    let t = c.g[V0][0] != c.g[T1][0];
                    c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x1));
                    if !t {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0041_0890);
                    }
                }
                c.cycles += 4;
                c.g[A3][0] = c.lw(c.g[A2][0] as u32);
                c.g[V0][0] = (c.g[A0][0] < c.g[A3][0]) as u64;
                let t = c.g[V0][0] != 0u64;

                if !t {
                    c.cycles += 4;
                    c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = (c.g[A0][0] < c.g[A1][0]) as u64;
                    let t = c.g[V0][0] == 0u64;

                    if !t {
                        c.cycles += 2;
                        let t = c.g[T0][0] == c.g[T3][0];
                        c.mult(0, 2, 8, 12);
                        if t {
                            c.cycles += 2;
                            c.g[T0][0] = c.g[T2][0];
                            c.cycles += 1;
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0xc));
                            continue 'l_410890;
                        } else {
                            c.cycles += 5;
                            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T5][0] as u32));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
                            if c.g[V0][0] != 0u64 {
                                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0xc));
                                continue 'l_410890;
                            } else {
                                c.cycles += 4;
                                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                c.g[V0][0] = (c.g[A3][0] < c.g[V0][0]) as u64;
                                if c.g[V0][0] != 0u64 {
                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0xc));
                                    continue 'l_410890;
                                } else {
                                    c.cycles += 2;
                                    c.g[T0][0] = c.g[T2][0];
                                    c.cycles += 1;
                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0xc));
                                    continue 'l_410890;
                                }
                            }
                        }
                    }
                }
                c.cycles += 2;
                if (c.g[T0][0] as i64) < 0 {
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0xc));
                    continue 'l_410890;
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                    c.g[V0][0] = (c.g[A0][0] < c.g[V0][0]) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0xffffffffffffffff;
                    if t {
                        c.cycles += 2;
                        let t = c.g[T0][0] == c.g[V0][0];
                        c.g[V1][0] = 0xc;
                        if t {
                            c.cycles += 2;
                            let t = c.g[RA][0] as u32;
                            c.g[V0][0] = 0u64;
                            c.pc = t;
                            return;
                        } else {
                            c.cycles += 4;
                            c.mult(0, 2, 8, 3);
                            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T5][0] as u32));
                            let t = c.g[RA][0] as u32;
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                            c.pc = t;
                            return;
                        }
                    } else {
                        c.cycles += 1;
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0xc));
                        continue 'l_410890;
                    }
                }
            }
        }
        c.cycles += 1;
        c.g[V0][0] = 0xffffffffffffffff;
        c.cycles += 2;
        let t = c.g[T0][0] == c.g[V0][0];
        c.g[V1][0] = 0xc;
        if t {
            c.cycles += 2;
            let t = c.g[RA][0] as u32;
            c.g[V0][0] = 0u64;
            c.pc = t;
            return;
        } else {
            c.cycles += 4;
            c.mult(0, 2, 8, 3);
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T5][0] as u32));
            let t = c.g[RA][0] as u32;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
            c.pc = t;
            return;
        }
    }
}

/// `void *find_exception_handler(uint32 pc, void *table, void *matcher)`
/// (0x410930): scan the new-format handler table for the entry covering
/// `pc` in `a0`. `a1` is the table (null returns 0), `a3` gates the range
/// narrowing, `a2` is the request passed to the `matcher` callback in
/// `a2`... (register flow as generated). Returns the first matching handler
/// record, or 0. The matcher at `*(a2)` runs through the table.
pub fn find_exception_handler(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = 0x1;
        c.sd(frame.at(c, 0x60), c.g[S6][0]);
        c.sd(frame.at(c, 0x50), c.g[S5][0]);
        c.g[S6][0] = c.g[T0][0];
        c.sd(frame.at(c, 0x40), c.g[S4][0]);
        c.g[S5][0] = c.g[A1][0];
        c.sd(frame.at(c, 0x30), c.g[S3][0]);
        c.g[S4][0] = c.g[A2][0];
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.g[S3][0] = 0;
        c.sd(frame.at(c, 0x70), c.g[RA][0]);
        c.g[S1][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x20), c.g[S2][0]);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        // The null-table test's delay slot: runs on both arms.
        c.sw(c.g[S6][0] as u32, c.g[V0][0]);
        if c.g[S5][0] == 0 {
            return found(c, frame);
        }
        'scan: {
            'narrow: {
                c.cycles += 3;
                c.g[A2][0] = 0;
                c.g[T0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x8));
                if c.g[A3][0] == 0 {
                    break 'narrow;
                }
                c.cycles += 11;
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[T0][0] as u32));
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[A2][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
                c.g[V1][0] = sx((c.g[A2][0] as u32) << 4);
                c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[S1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                c.g[V0][0] = !(0 | c.g[V0][0]);
                if c.g[V0][0] != 0 {
                    c.g[A2][0] = c.g[A1][0];
                }
                break 'scan;
            }
            c.cycles += 1;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
            break 'scan;
        }
        c.cycles += 7;
        c.g[V0][0] = sx((c.g[A2][0] as u32) << 4);
        c.g[V1][0] = 0xffff_ffff_ffff_0000;
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V1][0] |= 0xffff;
        c.g[V0][0] = c.lw(c.g[A1][0] as u32);
        c.g[S0][0] = c.g[A1][0];
        if c.g[V0][0] == c.g[V1][0] {
            return found(c, frame);
        }
        c.cycles += 5;
        c.g[S2][0] = 0xffff_ffff_ffff_0000;
        c.g[S2][0] |= 0xffff;
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
        'rows: loop {
            'row: {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                    break 'row;
                }
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                if c.g[V0][0] == 0 {
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                    break 'row;
                }
                'match_: {
                    'slot: {
                        c.cycles += 3;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                        if c.g[A1][0] == 0 {
                            break 'slot;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(c.g[S4][0] as u32);
                        c.g[A0][0] = c.g[S4][0];
                        if c.g[V0][0] == 0 {
                            break 'match_;
                        }
                        c.cycles += 4;
                        let target = c.g[V0][0] as u32;
                        c.g[A2][0] = c.g[S5][0];
                        call::call_ptr(c, target, 0x0041_0a18)?;
                        if c.g[V0][0] == 0 {
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                            break 'row;
                        }
                        // 'b_410a28 block: unconditional +2, then the
                        // s3 fill.
                        c.cycles += 2;
                        if c.g[S3][0] == 0 {
                            c.g[S3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        }
                        c.cycles += 2;
                        c.sw(c.g[S6][0] as u32, 0);
                        return found(c, frame);
                    }
                    c.cycles += 2;
                    if c.g[S3][0] == 0 {
                        c.g[S3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    }
                    break 'match_;
                }
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                break 'row;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
            if c.g[V0][0] != c.g[S2][0] {
                c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                if event_due(c) {
                    deliver_event_here(c, 0x0041_09e0);
                }
                continue 'rows;
            }
            return found(c, frame);
        }
    })
}

/// `void *get_reg_addr(uint32 reg, void *save_area)` (0x410a78): address of
/// register `reg` (`a0`) in the `setjmp` save area at `a1`, or 0 for the
/// zero register class. The area's +0x29c bytes tag each register by class;
/// class 2 walks the 8-byte register slots at +0x18 until the tag matches,
/// then aborts when the found tag is not class 1 (`abort`, noreturn on
/// that path); otherwise returns `base + slot + *(a1)`.
pub fn get_reg_addr(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x29c));
        c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[V0][0] = 0x2;
        c.g[A0][0] = c.lb(c.g[V1][0] as u32);
        let t = c.g[A0][0] != c.g[V0][0];
        c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        if t {
            // Wrong class: no address.
            return found_reg(c, frame);
        }
        c.cycles += 3;
        c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_add(0x18));
        c.g[V0][0] = sx((c.g[A2][0] as u32) << 3);
        c.g[A0][0] = 0x2;
        'slots: loop {
            c.cycles += 8;
            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = c.ld(c.g[V0][0] as u32);
            c.g[A2][0] = c.g[V1][0] << 32;
            c.g[A2][0] = ((c.g[A2][0] as i64) >> 32) as u64;
            c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.g[V1][0] = c.lb(c.g[V0][0] as u32);
            if c.g[V1][0] != c.g[A0][0] {
                return found_reg(c, frame);
            }
            c.g[V0][0] = sx((c.g[A2][0] as u32) << 3);
            if event_due(c) {
                deliver_event_here(c, 0x0041_0aa8);
            }
            continue 'slots;
        }
    })
}

/// `void copy_reg(uint32 reg, void *src_area, void *dst_area)`
/// (0x410b00): copy one register's saved value from the `src` slot of
/// `src_area` to the `dst` slot of `dst_area` (`memcpy`, 0x10 bytes for
/// general registers, 8 or 4 for the float/control classes).
pub fn copy_reg(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x40);
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.g[S1][0] = c.g[A0][0];
        c.g[S0][0] = c.g[A2][0];
        c.sd(frame.at(c, 0x20), c.g[S2][0]);
        frame.save_ra(c, 0x30);
        c.g[A2][0] = 0;
        callee::GET_REG_ADDR.call(c, 0x0041_0b24)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = 0;
        callee::GET_REG_ADDR.call(c, 0x0041_0b38)?;
        c.g[A0][0] = c.g[V0][0];
        // The size-class delay slot: V0 is set on both arms (the < 31
        // arm takes A2 = 0x10 without touching it again).
        c.g[V0][0] = ((c.g[S1][0] as i64) < 31) as u64;
        if c.g[V0][0] != 0 {
            c.g[A2][0] = 0x10;
        } else {
            c.cycles += 5;
            c.g[V0][0] = 0x3f;
            c.g[A2][0] = 0x8;
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S1][0] as i64)) as u64;
            c.g[V1][0] = 0x4;
            if c.g[V0][0] == 0 {
                c.g[A2][0] = c.g[V1][0];
            }
        }
        c.cycles += 8;
        c.g[A1][0] = c.g[S2][0];
        callee::MEMCPY.call(c, 0x0041_0b64)?;
        c.g[RA][0] = c.ld(frame.at(c, 0x30));
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        Ok(())
    })
}

/// `void *next_stack_level(void *level, void *save_area)` (0x410b80):
/// advance the unwind cursor `level` past one frame: `__frame_state_for`
/// resolves the frame, then register 0x298's class decides whether the
/// next address comes from `get_reg_addr` or the level words are shuffled
/// forward in place. Returns the advanced cursor, or 0 at the end.
pub fn next_stack_level(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.g[S1][0] = c.g[A1][0];
        c.sd(frame.at(c, 0), c.g[S0][0]);
        frame.save_ra(c, 0x20);
        c.g[A1][0] = c.g[A2][0];
        callee::FRAME_STATE_FOR.call(c, 0x0041_0b9c)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0 {
            // No frame state: join the epilogue directly (the generated
            // body breaks out of the whole nest here, skipping the
            // level-word shuffle below).
            c.cycles += 2;
            c.g[V0][0] = 0;
            return finish(c, frame);
        }
        c.g[A0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x298));
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x29c));
        c.g[A1][0] = c.g[S1][0];
        c.cycles += 4;
        if c.g[V1][0] != 0 {
            c.cycles += 7;
            c.g[A2][0] = 0;
            callee::GET_REG_ADDR.call(c, 0x0041_0bc8)?;
            c.g[V1][0] = c.ld(c.g[V0][0] as u32);
            c.g[V1][0] <<= 32;
            c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
            c.sw(c.g[S0][0] as u32, c.g[V1][0]);
            return level_done(c, frame);
        }
        c.cycles += 2;
        c.g[V0][0] = c.lw(c.g[S1][0] as u32);
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        level_done(c, frame)
    })
}

/// `void throw_helper(void)` (0x410c18): the landing-pad search of the
/// sjlj throw: walks the frame chain with `next_stack_level`, matches each
/// frame's handler table with `find_exception_handler` (twice) or
/// `old_find_exception_handler`, copies the register window with
/// `copy_reg`/`get_reg_addr`, and returns the landing context in the
/// frame slots. Falls through to `__terminate` when nothing matches.
/// Dispatcher shape: entry plus the five unwind heads, so an event unwind
/// resumes where generated does (the frame stays open).
pub fn throw_helper(c: &mut Cpu) {
    call::guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                // Entry prologue (0xc18): save everything, `fp = a2`.
                0x0041_0c18 => {
                    c.cycles += 26;
                    let frame = Frame::open(c, 0x6a0);
                    c.sd(frame.at(c, 0x680), c.g[FP][0]);
                    c.sd(frame.at(c, 0x660), c.g[S6][0]);
                    c.g[FP][0] = c.g[A2][0];
                    c.sd(frame.at(c, 0x650), c.g[S5][0]);
                    c.sd(frame.at(c, 0x640), c.g[S4][0]);
                    c.g[S5][0] = 0;
                    c.sd(frame.at(c, 0x630), c.g[S3][0]);
                    c.g[S4][0] = c.g[SP][0];
                    c.sd(frame.at(c, 0x600), c.g[S0][0]);
                    c.g[S3][0] = c.g[A0][0];
                    c.sd(frame.at(c, 0x690), c.g[RA][0]);
                    c.g[S0][0] = c.g[A1][0];
                    c.sd(frame.at(c, 0x670), c.g[S7][0]);
                    c.sd(frame.at(c, 0x620), c.g[S2][0]);
                    c.sd(frame.at(c, 0x610), c.g[S1][0]);
                    c.sw(frame.at(c, 0x5e4), c.g[A3][0]);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                    c.g[S7][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.sw(frame.at(c, 0x5e8), c.g[S0][0]);
                    c.g[S6][0] = (0 < c.g[V0][0]) as u64;
                    c.g[S1][0] = c.lw(c.g[A3][0] as u32);
                    c.sw(frame.at(c, 0x5f4), 0);
                    c.sw(frame.at(c, 0x5f8), 0);
                    c.sw(frame.at(c, 0x5e0), 0);
                    pc = 0x0041_0db0;
                    continue;
                }
                // The 0xdb0 scan head: advance one frame, stop at the end.
                0x0041_0db0 => {
                    c.cycles += 8;
                    c.g[S2][0] = c.g[S1][0];
                    c.g[A2][0] = c.g[S4][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S2][0];
                    callee::NEXT_STACK_LEVEL.call(c, 0x0041_0dc4)?;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[S4][0] = c.g[S2][0];
                    if c.g[S1][0] == 0 {
                        pc = 0x0041_0dd0;
                        continue;
                    }
                    if back_edge_unwinds(c, 0x0041_0c80) {
                        return Ok(());
                    }
                    pc = 0x0041_0c80;
                    continue;
                }
                // The 0xc80 matcher-kind dispatch.
                0x0041_0c80 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    if c.g[V1][0] != 0 {
                        c.g[V1][0] = c.lw(c.g[V1][0] as u32);
                        pc = 0x0041_0c94;
                        continue;
                    }
                    pc = 0x0041_0c8c;
                    continue;
                }
                // The 0xc8c null-kind path.
                0x0041_0c8c => {
                    c.cycles += 2;
                    c.g[V1][0] = 0;
                    pc = 0x0041_0ca4;
                    continue;
                }
                // The 0xc94 masked-kind path.
                0x0041_0c94 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0xffff_ffff_ffff_0000;
                    c.g[V0][0] |= 0xfffe;
                    c.g[V1][0] ^= c.g[V0][0];
                    c.g[V1][0] = (c.g[V1][0] < 1) as u64;
                    pc = 0x0041_0ca4;
                    continue;
                }
                // The 0xca4 table-kind select.
                0x0041_0ca4 => {
                    c.cycles += 2;
                    c.g[A2][0] = c.g[S7][0];
                    if c.g[S6][0] != 0 {
                        // The 0xcac new-table search. The `S6 = 0` runs in
                        // the call delay slot, so it clears before the call
                        // on this arm (the callee saves a different S6).
                        c.cycles += 9;
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                        c.g[A3][0] = 0x1;
                        c.g[T0][0] = sx(Frame { size: 0x6a0 }.at(c, 0x5e0));
                        c.g[S6][0] = 0;
                        callee::FIND_HANDLER.call(c, 0x0041_0cc4)?;
                        c.g[S5][0] = c.g[V0][0];
                        c.sw((c.g[S3][0] as u32).wrapping_add(0xc), 0);
                        pc = 0x0041_0d04;
                        continue;
                    }
                    pc = 0x0041_0cd0;
                    continue;
                }
                // The 0xcd0 search select. The `A0 = S0` runs in the test
                // delay slot, so it is set on both arms.
                0x0041_0cd0 => {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S0][0];
                    if c.g[V1][0] == 0 {
                        pc = 0x0041_0cf4;
                        continue;
                    }
                    pc = 0x0041_0cd8;
                    continue;
                }
                // The 0xcd8 second new-table search.
                0x0041_0cd8 => {
                    c.cycles += 7;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[A2][0] = c.g[S7][0];
                    c.g[A3][0] = 0;
                    c.g[T0][0] = sx(Frame { size: 0x6a0 }.at(c, 0x5e0));
                    callee::FIND_HANDLER.call(c, 0x0041_0cec)?;
                    c.g[S5][0] = c.g[V0][0];
                    pc = 0x0041_0d04;
                    continue;
                }
                // The 0xcf4 old-table search.
                0x0041_0cf4 => {
                    c.cycles += 4;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.g[S0][0];
                    callee::OLD_FIND_HANDLER.call(c, 0x0041_0d00)?;
                    c.g[S5][0] = c.g[V0][0];
                    pc = 0x0041_0d04;
                    continue;
                }
                // The 0xd04 handler check: S5 == 0 rejoins at 0xd90.
                0x0041_0d04 => {
                    let frame = Frame { size: 0x6a0 };
                    c.cycles += 2;
                    c.g[V0][0] = c.lw(frame.at(c, 0x5e0));
                    if c.g[S5][0] != 0 {
                        pc = 0x0041_0d0c;
                        continue;
                    }
                    pc = 0x0041_0d90;
                    continue;
                }
                // The 0xd0c unexpected-flag check: V0 still holds the
                // 0x5e0 word from 0xd04 (a set word drops to the 0xdd0
                // tail); the 0x5f8 load runs on both arms for 0xd14.
                0x0041_0d0c => {
                    let frame = Frame { size: 0x6a0 };
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0;
                    c.g[V0][0] = c.lw(frame.at(c, 0x5f8));
                    if t {
                        pc = 0x0041_0d88;
                        continue;
                    }
                    pc = 0x0041_0d14;
                    continue;
                }
                // The 0xd88 clear-and-leave.
                0x0041_0d88 => {
                    let frame = Frame { size: 0x6a0 };
                    c.cycles += 2;
                    c.sw(frame.at(c, 0x5f4), 0);
                    pc = 0x0041_0dd0;
                    continue;
                }
                // The 0xd14 register-window copy, unless already done
                // (its +2 runs on both arms of the test).
                0x0041_0d14 => {
                    c.cycles += 2;
                    if c.g[V0][0] != 0 {
                        c.g[A0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x29a));
                        pc = 0x0041_0d94;
                        continue;
                    }
                    pc = 0x0041_0d1c;
                    continue;
                }
                // The 0xd1c copy setup.
                0x0041_0d1c => {
                    c.cycles += 3;
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2f0));
                    c.g[V0][0] = c.g[S1][0];
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2e0));
                    pc = 0x0041_0d28;
                    continue;
                }
                // The 0xd28 copy loop.
                0x0041_0d28 => {
                    c.cycles += 13;
                    c.g[A1][0] = c.ld(c.g[V0][0] as u32);
                    c.g[A2][0] = c.ld((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[A3][0] = c.ld((c.g[V0][0] as u32).wrapping_add(0x10));
                    c.g[T0][0] = c.ld((c.g[V0][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[V1][0] as u32, c.g[A1][0]);
                    c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                    c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                    c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    if c.g[V0][0] != c.g[A0][0] {
                        if back_edge_unwinds(c, 0x0041_0d28) {
                            return Ok(());
                        }
                        pc = 0x0041_0d28;
                        continue;
                    }
                    pc = 0x0041_0d5c;
                    continue;
                }
                // The 0xd5c commit: stash the match, copy the tail.
                0x0041_0d5c => {
                    let frame = Frame { size: 0x6a0 };
                    c.cycles += 11;
                    c.g[A0][0] = 0x1;
                    c.g[A1][0] = 0x1;
                    c.sw(frame.at(c, 0x5f0), c.g[S0][0]);
                    c.sw(frame.at(c, 0x5ec), c.g[S5][0]);
                    c.sw(frame.at(c, 0x5f8), c.g[A0][0]);
                    c.sw(frame.at(c, 0x5f4), c.g[A1][0]);
                    c.g[A2][0] = c.ld(c.g[V0][0] as u32);
                    c.g[A3][0] = c.ld((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[V1][0] as u32, c.g[A2][0]);
                    c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    pc = 0x0041_0d90;
                    continue;
                }
                // The 0xd90 landing-pad search step.
                0x0041_0d90 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x29a));
                    pc = 0x0041_0d94;
                    continue;
                }
                // The 0xd94 frame-state step (each block charges its
                // cycles; the call takes the frame state in A1).
                0x0041_0d94 => {
                    c.cycles += 7;
                    c.g[A2][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S1][0];
                    callee::GET_REG_ADDR.call(c, 0x0041_0da0)?;
                    c.g[V1][0] = c.ld(c.g[V0][0] as u32);
                    c.g[V1][0] <<= 32;
                    c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
                    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                    pc = 0x0041_0db0;
                    continue;
                }
                // The 0xdd0 unexpected-handler tail: the flag test runs
                // first, and the reloads run whenever it is set — only the
                // cleanup call is gated on the use count.
                0x0041_0dd0 => {
                    let frame = Frame { size: 0x6a0 };
                    c.cycles += 3;
                    c.g[A1][0] = c.lw(frame.at(c, 0x5f8));
                    c.g[A2][0] = c.lw(frame.at(c, 0x5f4));
                    if c.g[A1][0] == 0 {
                        pc = 0x0041_0df4;
                        continue;
                    }
                    pc = 0x0041_0ddc;
                    continue;
                }
                // The 0xddc reloads.
                0x0041_0ddc => {
                    let frame = Frame { size: 0x6a0 };
                    c.cycles += 4;
                    c.g[S1][0] = sx(frame.at(c, throw_save::CUR));
                    c.g[S5][0] = c.lw(frame.at(c, 0x5ec));
                    c.g[S0][0] = c.lw(frame.at(c, 0x5f0));
                    if c.g[A2][0] == 0 {
                        pc = 0x0041_0df4;
                        continue;
                    }
                    pc = 0x0041_0dec;
                    continue;
                }
                // The 0xdec cleanup call.
                0x0041_0dec => {
                    c.cycles += 2;
                    callee::UNWINDING_CLEANUP.call(c, 0x0041_0df4)?;
                    pc = 0x0041_0df4;
                    continue;
                }
                // The 0xdf4 delivery: install the match or terminate.
                0x0041_0df4 => {
                    c.cycles += 2;
                    if c.g[S5][0] == 0 {
                        pc = 0x0041_0dfc;
                        continue;
                    }
                    c.sw(c.g[S3][0] as u32, c.g[S5][0]);
                    pc = 0x0041_0e08;
                    continue;
                }
                // The 0xdfc terminate path.
                0x0041_0dfc => {
                    c.cycles += 3;
                    callee::TERMINATE.call(c, 0x0041_0e04)?;
                    c.sw(c.g[S3][0] as u32, c.g[S5][0]);
                    pc = 0x0041_0e08;
                    continue;
                }
                // The 0xe08 context select: live fp or the next level.
                0x0041_0e08 => {
                    let frame = Frame { size: 0x6a0 };
                    c.cycles += 3;
                    c.g[A3][0] = c.lw(frame.at(c, 0x5e8));
                    c.g[S6][0] = c.g[S0][0];
                    if c.g[S0][0] != c.g[A3][0] {
                        pc = 0x0041_0e1c;
                        continue;
                    }
                    pc = 0x0041_0e14;
                    continue;
                }
                // The 0xe14 live-fp path.
                0x0041_0e14 => {
                    c.cycles += 2;
                    c.g[S1][0] = c.g[FP][0];
                    pc = 0x0041_0f0c;
                    continue;
                }
                // The 0xe1c context advance.
                0x0041_0e1c => {
                    let frame = Frame { size: 0x6a0 };
                    c.cycles += 4;
                    c.g[V1][0] = c.g[S1][0];
                    c.g[S0][0] = c.lw(frame.at(c, 0x5e8));
                    c.g[V0][0] = c.g[FP][0];
                    c.g[A0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x2e0));
                    pc = 0x0041_0e2c;
                    continue;
                }
                // The 0xe2c context-copy loop.
                0x0041_0e2c => {
                    c.cycles += 13;
                    c.g[T0][0] = c.ld(c.g[V0][0] as u32);
                    c.g[A1][0] = c.ld((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[A2][0] = c.ld((c.g[V0][0] as u32).wrapping_add(0x10));
                    c.g[A3][0] = c.ld((c.g[V0][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[V1][0] as u32, c.g[T0][0]);
                    c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                    c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                    c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[A3][0]);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    if c.g[V0][0] != c.g[A0][0] {
                        if back_edge_unwinds(c, 0x0041_0e2c) {
                            return Ok(());
                        }
                        pc = 0x0041_0e2c;
                        continue;
                    }
                    pc = 0x0041_0e60;
                    continue;
                }
                // The 0xe60 tail copy + tag check. A direct match takes the
                // 0xf00 edge with the tag already loaded (no 0xefc cycle);
                // otherwise the save loop below ends at 0xefc, which charges
                // its +1 for the same load.
                0x0041_0e60 => {
                    let frame = Frame { size: 0x6a0 };
                    c.cycles += 7;
                    c.g[T0][0] = c.ld(c.g[V0][0] as u32);
                    c.g[A0][0] = c.ld((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[V1][0] as u32, c.g[T0][0]);
                    c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                    c.g[A3][0] = c.lw(frame.at(c, 0x5e8));
                    if c.g[A3][0] == c.g[S6][0] {
                        c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x29a));
                        pc = 0x0041_0f00;
                        continue;
                    }
                    pc = 0x0041_0e7c;
                    continue;
                }
                // The 0xe7c save-loop entry.
                0x0041_0e7c => {
                    c.cycles += 1;
                    c.g[S2][0] = c.g[S1][0];
                    pc = 0x0041_0e80;
                    continue;
                }
                // The 0xe80 save-loop head.
                0x0041_0e80 => {
                    c.cycles += 10;
                    c.g[A2][0] = c.g[S4][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S2][0];
                    callee::NEXT_STACK_LEVEL.call(c, 0x0041_0e90)?;
                    c.g[S4][0] = c.g[S2][0];
                    c.g[S1][0] = c.g[V0][0];
                    c.g[S0][0] = 0;
                    c.g[A0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x29a));
                    c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x29c));
                    pc = 0x0041_0ea8;
                    continue;
                }
                // The 0xea8 tag-loop head.
                0x0041_0ea8 => {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    if c.g[S0][0] == c.g[A0][0] {
                        pc = 0x0041_0ecc;
                        continue;
                    }
                    pc = 0x0041_0eb0;
                    continue;
                }
                // The 0xeb0 tag-byte check.
                0x0041_0eb0 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lb(c.g[V0][0] as u32);
                    c.g[A1][0] = c.g[S1][0];
                    if c.g[V1][0] == 0 {
                        pc = 0x0041_0ecc;
                        continue;
                    }
                    pc = 0x0041_0ebc;
                    continue;
                }
                // The 0xebc tag install.
                0x0041_0ebc => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[FP][0];
                    callee::COPY_REG.call(c, 0x0041_0ec8)?;
                    c.g[A0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x29a));
                    pc = 0x0041_0ecc;
                    continue;
                }
                // The 0xecc round check.
                0x0041_0ecc => {
                    c.cycles += 4;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 79) as u64;
                    c.g[A2][0] = c.g[S2][0];
                    if c.g[V0][0] == 0 {
                        pc = 0x0041_0edc;
                        continue;
                    }
                    if back_edge_unwinds(c, 0x0041_0ea8) {
                        return Ok(());
                    }
                    pc = 0x0041_0ea8;
                    continue;
                }
                // The 0xedc register-address step.
                0x0041_0edc => {
                    c.cycles += 8;
                    c.g[A1][0] = c.g[S1][0];
                    callee::GET_REG_ADDR.call(c, 0x0041_0ee4)?;
                    c.g[V1][0] = c.ld(c.g[V0][0] as u32);
                    c.g[V1][0] <<= 32;
                    c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
                    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                    c.g[S2][0] = c.g[S1][0];
                    if c.g[S0][0] == c.g[S6][0] {
                        pc = 0x0041_0efc;
                        continue;
                    }
                    if back_edge_unwinds(c, 0x0041_0e80) {
                        return Ok(());
                    }
                    pc = 0x0041_0e80;
                    continue;
                }
                // The 0xefc tag load on the context-advance path.
                0x0041_0efc => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x29a));
                    pc = 0x0041_0f00;
                    continue;
                }
                // The 0xf00 register-tag check.
                0x0041_0f00 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x2;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x29c));
                    pc = 0x0041_0f0c;
                    continue;
                }
                // The 0xf0c return: restore everything and return.
                0x0041_0f0c => {
                    let frame = Frame { size: 0x6a0 };
                    c.cycles += 15;
                    c.g[T0][0] = c.lw(frame.at(c, 0x5e4));
                    c.g[V0][0] = c.g[S5][0];
                    c.g[RA][0] = c.ld(frame.at(c, 0x690));
                    c.sw(c.g[T0][0] as u32, c.g[S1][0]);
                    c.g[FP][0] = c.ld(frame.at(c, 0x680));
                    c.g[S7][0] = c.ld(frame.at(c, 0x670));
                    c.g[S6][0] = c.ld(frame.at(c, 0x660));
                    c.g[S5][0] = c.ld(frame.at(c, 0x650));
                    c.g[S4][0] = c.ld(frame.at(c, 0x640));
                    c.g[S3][0] = c.ld(frame.at(c, 0x630));
                    c.g[S2][0] = c.ld(frame.at(c, 0x620));
                    c.g[S1][0] = c.ld(frame.at(c, 0x610));
                    c.g[S0][0] = c.ld(frame.at(c, 0x600));
                    let t = c.g[RA][0] as u32;
                    frame.close(c);
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0041_0c18, 0x0041_0f48);
                    return Ok(());
                }
            }
        }
    })
}

/// `void eh_context_initialize(void)` (0x4104b8): publish
/// `eh_context_static` as the current thread's context getter, then run it
/// and return its context block (a plain call and return, not a tail
/// call: the frame stays open across it).
pub fn eh_context_initialize(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x410000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4e0));
        c.g[V1][0] = 0x450000;
        frame.save_ra(c, 0);
        let target = c.g[V0][0] as u32;
        Addr(0x0045_3444).sw(c, c.g[V0][0]);
        call::call_ptr(c, target, 0x0041_04d4)?;
        c.g[RA][0] = c.ld(frame.at(c, 0));
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        Ok(())
    })
}

/// `void *eh_context_static(void)` (0x4104e0): the static exception context
/// at `0x52ad...` (the `0x52adb0` cell region): on first use zero it with
/// `memset` and link the current-thread slot at +4. Returns the block.
pub fn eh_context_static(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = 0x530000;
        c.g[V0][0] = Addr(0x0052_adb0).lw(c);
        frame.save_ra(c, 0x20);
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        if c.g[V0][0] == 0 {
            c.cycles += 12;
            c.g[S1][0] = 0x530000;
            c.g[V0][0] = 0x1;
            c.g[S0][0] = sx(0x0053_0000u32.wrapping_sub(0x5260));
            Addr(0x0052_adb0).sw(c, c.g[V0][0]);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x10;
            callee::MEMSET.call(c, 0x0041_051c)?;
            c.g[V1][0] = sx(0x0053_0000u32.wrapping_sub(0x5248));
            Addr(c.g[S0][0] as u32).off(0x4).sw(c, c.g[V1][0]);
        } else {
            c.cycles += 1;
            c.g[S1][0] = 0x530000;
        }
        c.cycles += 6;
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x5260));
        c.g[RA][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        Ok(())
    })
}

/// `void terminate(void)` (0x412800, the C++ `terminate()`): call the
/// installed terminate handler (`*(0x450000 + 0x3440)`), then re-enter
/// `set_terminate` (which installs the default handler).
pub fn terminate_cpp(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 6;
        c.g[V0][0] = 0x450000;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3440));
        frame.save_ra(c, 0);
        let target = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0041_2818);
        call::call_ptr(c, target, 0x0041_2818)?;
        c.pc = callee::SET_TERMINATE.addr;
        (callee::SET_TERMINATE.f)(c);
        Ok(())
    })
}

/// `void set_unexpected(void (*fn)(void))` (0x412838): install `a0` as the
/// unexpected handler (`*(0x450000 + 0x3a38)`). Returns nothing.
pub fn set_unexpected(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = 0x450000;
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3a38));
    let t = c.g[RA][0] as u32;
    c.sw((c.g[V1][0] as u32).wrapping_add(0x3a38), c.g[A0][0]);
    c.pc = t;
}

/// `int uncaught_exception(void)` (0x412dd8): whether an exception is
/// active but uncaught: 1 when `*__get_eh_info()`'s handler count slot
/// (+0x14) is zero... (register flow as generated).
pub fn uncaught_exception(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        GET_EH_INFO.call(c, 0x0041_2de8)?;
        c.g[V0][0] = c.lw(c.g[V0][0] as u32);
        c.g[RA][0] = c.ld(frame.at(c, 0));
        if c.g[V0][0] == 0 {
            c.g[V1][0] = 0;
        } else {
            c.cycles += 2;
            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
            c.g[V1][0] = (c.g[V0][0] < 1) as u64;
        }
        c.cycles += 4;
        c.g[V0][0] = c.g[V1][0];
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        Ok(())
    })
}

/// `const char *exception_what(const void *ex)` (0x412e10): call the
/// exception's `what` slot (`*(*(a0) + 4)`) and return its string.
pub fn exception_what(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = c.lw(c.g[A0][0] as u32);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
        let target = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0041_2e28);
        call::call_ptr(c, target, 0x0041_2e28)?;
        c.g[V0][0] = c.lw(c.g[V0][0] as u32);
        c.g[RA][0] = c.ld(frame.at(c, 0));
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        Ok(())
    })
}

/// `bad_exception::~bad_exception(int)` (0x412e38).
pub fn bad_exception_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 7;
        c.g[V0][0] = 0x4a0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6748));
        frame.save_ra(c, 0);
        c.g[A1][0] &= 0x1;
        c.sw(c.g[A0][0] as u32, c.g[V0][0]);
        if c.g[A1][0] != 0 {
            c.cycles += 2;
            c.g[RA][0] = sx(0x0041_2e5c);
            call::call_ptr(c, 0x0031_7e50, 0x0041_2e5c)?;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(frame.at(c, 0));
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        Ok(())
    })
}

/// `bad_exception::bad_exception(void)` (0x412e68).
pub fn bad_exception_ctor(c: &mut Cpu) {
    exception_ctor_impl(c, 0x4a0000, 0x6760);
}

/// `exception::~exception(int)` (0x412ed0).
pub fn exception_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 7;
        c.g[V0][0] = 0x4a0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6748));
        frame.save_ra(c, 0);
        c.g[A1][0] &= 0x1;
        c.sw(c.g[A0][0] as u32, c.g[V0][0]);
        if c.g[A1][0] != 0 {
            c.cycles += 2;
            c.g[RA][0] = sx(0x0041_2ef4);
            call::call_ptr(c, 0x0031_7e50, 0x0041_2ef4)?;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(frame.at(c, 0));
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        Ok(())
    })
}

/// `exception::exception(void)` (0x412f00).
pub fn exception_ctor(c: &mut Cpu) {
    exception_ctor_impl(c, 0x4a0000, 0x6748);
}

/// `uint32_t decode_uleb128(const uint8_t **pp, uint32_t *out)` (0x413b38):
/// read an unsigned LEB128 at the byte pointer `a0`, advancing past its
/// bytes: the value goes to `*a1`, the advanced pointer returns in `v0`.
/// Kept in the generated block shape.
pub fn decode_uleb128(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = c.lbu(c.g[A0][0] as u32);
    c.g[A3][0] = 0u64;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
    c.g[V0][0] = c.g[V1][0] & 0x80;
    let t = c.g[V0][0] == 0u64;
    c.g[A2][0] = c.g[V1][0] & 0x7f;
    // A single-byte value returns the masked low 7 bits; longer
    // values accumulate shifted groups in the loop below.
    if !t {
        'l_413b50: loop {
            c.cycles += 8;
            c.g[V1][0] = c.lbu(c.g[A0][0] as u32);
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x7));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = c.g[V1][0] & 0x7f;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << ((c.g[A3][0] as u32) & 31));
            c.g[V1][0] = c.g[V1][0] & 0x80;
            let t = c.g[V1][0] != 0u64;
            c.g[A2][0] = c.g[A2][0] | c.g[V0][0];
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x0041_3b50);
                }
                continue 'l_413b50;
            }
            break;
        }
    }
    c.cycles += 3;
    c.sw(c.g[A1][0] as u32, c.g[A2][0]);
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = c.g[A0][0];
    c.pc = t;
    return;
}

/// `int32_t decode_sleb128(const uint8_t **pp, int32_t *out)` (0x413b80):
/// read a signed LEB128 at the byte pointer `a0` with sign extension,
/// advancing past its bytes: the value goes to `*a1`, the advanced
/// pointer returns in `v0`. Kept in the generated block shape.
pub fn decode_sleb128(c: &mut Cpu) {
    c.cycles += 2;
    c.g[A3][0] = 0u64;
    c.g[T0][0] = 0u64;
    'l_413b88: loop {
        c.cycles += 8;
        c.g[A2][0] = c.lbu(c.g[A0][0] as u32);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
        c.g[V0][0] = c.g[A2][0] & 0x7f;
        c.g[V1][0] = c.g[A2][0] & 0x80;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << ((c.g[A3][0] as u32) & 31));
        c.g[T0][0] = c.g[T0][0] | c.g[V0][0];
        let t = c.g[V1][0] != 0u64;
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x7));
        if t {
            if event_due(c) {
                deliver_event_here(c, 0x0041_3b88);
            }
            continue 'l_413b88;
        }
        // Sign-extend a short negative value: a set sign bit fills the
        // high bits from the shift count up.
        c.cycles += 3;
        c.g[V0][0] = (c.g[A3][0] < 0x20u64) as u64;
        {
            let short = c.g[V0][0] == 0u64;
            c.g[V0][0] = c.g[A2][0] & 0x40;
            if !short {
                c.cycles += 2;
                let positive = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0xffffffffffffffff;
                if !positive {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << ((c.g[A3][0] as u32) & 31));
                    c.g[T0][0] = c.g[T0][0] | c.g[V0][0];
                }
            }
        }
        c.cycles += 3;
        c.sw(c.g[A1][0] as u32, c.g[T0][0]);
        c.g[V0][0] = c.g[A0][0];
        c.pc = c.g[RA][0] as u32;
        return;
    }
}

/// `type_info::~type_info(int)` (0x414b68): store the `type_info` vtable,
/// then `operator delete` when `a1 & 1` is set. Kept in the generated
/// block shape.
pub fn type_info_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        let frame = Frame::open(c, 0x10);
        'b_414b8c: {
            c.cycles += 7;
            c.g[V0][0] = 0x4a0000;
            frame.save_ra(c, 0);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x65b8));
            c.g[A1][0] &= 0x1;
            c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            if c.g[A1][0] == 0 {
                break 'b_414b8c;
            }
            c.cycles += 2;
            c.g[RA][0] = sx(0x0041_4b8c);
            call::call_ptr(c, 0x0031_7e50, 0x0041_4b8c)?;
            break 'b_414b8c;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(frame.at(c, 0));
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        Ok(())
    })
}

/// `bool type_info::operator==(const type_info &) const` (0x414b98):
/// pointer identity, else compare the mangled names with `strcmp`.
/// Kept in the generated block shape.
pub fn type_info_eq(c: &mut Cpu) {
    call::guest(c, |c| {
        let frame = Frame::open(c, 0x20);
        // Equal when the pointers are identical or the type names
        // compare equal (a null table compares unequal).
        c.cycles += 5;
        c.sd(frame.at(c, 0), c.g[S0][0]);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = 0u64;
        if c.g[A1][0] == c.g[A0][0] {
            c.cycles += 2;
            c.g[S0][0] = 0x1;
            c.g[V0][0] = c.g[S0][0];
        } else {
            c.cycles += 5;
            c.g[A0][0] = c.lw(c.g[A0][0] as u32);
            c.g[RA][0] = sx(0x0041_4bb8);
            c.g[A1][0] = c.lw(c.g[A1][0] as u32);
            STR_CMP.call(c, 0x0041_4bb8)?;
            let mismatched = c.g[V0][0] != 0;
            c.g[V0][0] = c.g[S0][0];
            if !mismatched {
                c.cycles += 2;
                c.g[S0][0] = 0x1;
                c.g[V0][0] = c.g[S0][0];
            }
        }
        c.cycles += 4;
        c.g[RA][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        Ok(())
    })
}

/// `bool type_info::operator!=(const type_info &) const` (0x4151b8):
/// the negation of `type_info_eq`.
pub fn type_info_ne(c: &mut Cpu) {
    call::guest(c, |c| {
        let frame = Frame::open(c, 0x10);
        c.cycles += 8;
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0041_51c8);
        callee::TYPE_INFO_EQ.call(c, 0x0041_51c8)?;
        c.g[RA][0] = c.ld(frame.at(c, 0));
        c.g[V0][0] ^= 0x1;
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        Ok(())
    })
}

/// `size_t count_fdes(...) ` (0x413f38): count the frame description entries in
/// a chain. Kept in the generated dispatcher shape.
pub fn count_fdes(c: &mut Cpu) {
    call::guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0041_3f38 => {
                    c.cycles += 3;
                    c.g[A1][0] = c.lw(c.g[A0][0] as u32);
                    let t = c.g[A1][0] == 0u64;
                    c.g[A2][0] = 0u64;
                    if t {
                        pc = 0x0041_3f78;
                        continue;
                    }
                    pc = 0x0041_3f44;
                    continue;
                }
                0x0041_3f44 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x0041_3f58;
                        continue;
                    }
                    pc = 0x0041_3f4c;
                    continue;
                }
                0x0041_3f4c => {
                    c.cycles += 1;
                    pc = 0x0041_3f50;
                    continue;
                }
                0x0041_3f50 => {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[V1][0];
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                    pc = 0x0041_3f58;
                    continue;
                }
                0x0041_3f58 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x0041_3f68;
                        continue;
                    }
                    pc = 0x0041_3f60;
                    continue;
                }
                0x0041_3f60 => {
                    c.cycles += 2;
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                    if c.g[V1][0] != 0 {
                        c.g[A2][0] = c.g[V0][0];
                    }
                    pc = 0x0041_3f68;
                    continue;
                }
                0x0041_3f68 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    let t = c.g[V1][0] != 0u64;
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x0041_3f50) {
                            return Ok(());
                        }
                        pc = 0x0041_3f50;
                        continue;
                    }
                    pc = 0x0041_3f78;
                    continue;
                }
                0x0041_3f78 => {
                    c.cycles += 2;
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[A2][0];
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00413f38, 0x00413f80);
                    return Ok(());
                }
            }
        }
    })
}

/// `void frame_init(...)` (0x414008): register frame info, merging and sorting
/// FDEs. Kept in the generated block shape.
pub fn frame_init(c: &mut Cpu) {
    call::guest(c, |c| {
        let frame = Frame::open(c, 0x80);
        'b_414070: {
            c.cycles += 11;
            c.sd(frame.at(c, 0x50), c.g[S3][0]);
            frame.save_ra(c, 0x70);
            c.g[S3][0] = c.g[A0][0];
            c.sd(frame.at(c, 0x60), c.g[S4][0]);
            c.sd(frame.at(c, 0x40), c.g[S2][0]);
            c.sd(frame.at(c, 0x30), c.g[S1][0]);
            c.sd(frame.at(c, 0x20), c.g[S0][0]);
            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
            let t = c.g[V0][0] == 0u64;
            c.g[S0][0] = c.g[V0][0];
            if !t {
                c.cycles += 3;
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                let t = c.g[V0][0] == 0u64;
                c.g[S4][0] = 0u64;
                if t {
                    c.cycles += 1;
                    c.g[S0][0] = sx((c.g[S4][0] as u32) << 2);
                    break 'b_414070;
                } else {
                    'l_414040: loop {
                        c.cycles += 6;
                        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                        c.g[RA][0] = sx(0x0041_404c);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                        callee::COUNT_FDES.call(c, 0x0041_404c)?;
                        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                        let t = c.g[V1][0] != 0u64;
                        c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0041_4040);
                            }
                            continue 'l_414040;
                        } else {
                            c.cycles += 2;
                            c.g[S0][0] = sx((c.g[S4][0] as u32) << 2);
                            break 'b_414070;
                        }
                    }
                }
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x0041_4068);
            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
            callee::COUNT_FDES.call(c, 0x0041_4068)?;
            c.g[S4][0] = c.g[V0][0];
            c.cycles += 1;
            c.g[S0][0] = sx((c.g[S4][0] as u32) << 2);
            break 'b_414070;
        }
        'b_414108: {
            c.cycles += 15;
            c.sw((c.g[S3][0] as u32).wrapping_add(0x10), c.g[S4][0]);
            c.g[RA][0] = sx(0x0041_407c);
            c.g[A0][0] = c.g[S0][0];
            MALLOC.call(c, 0x0041_407c)?;
            c.sw(c.g[SP][0] as u32, c.g[V0][0]);
            c.g[RA][0] = sx(0x0041_4088);
            c.g[A0][0] = c.g[S0][0];
            MALLOC.call(c, 0x0041_4088)?;
            c.g[V1][0] = 0xffffffffffff0000;
            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
            c.g[V1][0] = c.g[V1][0] | 0xffff;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0u64);
            let t = c.g[A0][0] == 0u64;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x14), 0u64);
            if !t {
                c.cycles += 4;
                c.g[S0][0] = c.g[A0][0];
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                let t = c.g[V0][0] == 0u64;
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                if t {
                    c.cycles += 1;
                    c.g[A1][0] = c.g[S4][0];
                    break 'b_414108;
                } else {
                    c.cycles += 3;
                    c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x14));
                    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                    'l_4140c8: loop {
                        c.cycles += 8;
                        c.g[A1][0] = c.g[SP][0];
                        c.g[A2][0] = c.g[S2][0];
                        c.g[RA][0] = sx(0x0041_40d8);
                        c.g[A3][0] = c.g[S1][0];
                        callee::ADD_FDES.call(c, 0x0041_40d8)?;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                        if c.g[V0][0] != 0u64 {
                            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                            if event_due(c) {
                                deliver_event_here(c, 0x0041_40c8);
                            }
                            continue 'l_4140c8;
                        } else {
                            c.cycles += 2;
                            c.g[A1][0] = c.g[S4][0];
                            break 'b_414108;
                        }
                    }
                }
            }
            c.cycles += 5;
            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
            c.g[A1][0] = c.g[SP][0];
            c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[RA][0] = sx(0x0041_4104);
            c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x14));
            callee::ADD_FDES.call(c, 0x0041_4104)?;
            c.cycles += 1;
            c.g[A1][0] = c.g[S4][0];
            break 'b_414108;
        }
        c.cycles += 15;
        c.g[RA][0] = sx(0x0041_4110);
        c.g[A0][0] = c.g[SP][0];
        callee::END_FDE_SORT.call(c, 0x0041_4110)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
        c.sw((c.g[S3][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.sw((c.g[S3][0] as u32).wrapping_add(0x4), c.g[V1][0]);
        c.sw(c.g[S3][0] as u32, c.g[A0][0]);
        c.g[RA][0] = c.ld(frame.at(c, 0x70));
        c.g[S4][0] = c.ld(frame.at(c, 0x60));
        c.g[S3][0] = c.ld(frame.at(c, 0x50));
        c.g[S2][0] = c.ld(frame.at(c, 0x40));
        c.g[S1][0] = c.ld(frame.at(c, 0x30));
        c.g[S0][0] = c.ld(frame.at(c, 0x20));
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        return Ok(());
    })
}

/// `void fde_merge(...)` (0x413bd0): sorted-merge the source frame
/// description table into the destination in place, keyed on the entries'
/// PC ranges (an empty source merges to nothing). Kept in the generated
/// block shape.
pub fn fde_merge(c: &mut Cpu) {
    call::guest(c, |c| {
        let frame = Frame::open(c, 0x30);
        // Empty source table merges to nothing.
        c.cycles += 8;
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.sd(frame.at(c, 0x20), c.g[S2][0]);
        c.g[S1][0] = c.g[A0][0];
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.g[T8][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
        let t = c.g[T8][0] == 0u64;
        c.g[T5][0] = c.g[T8][0];
        if !t {
            c.cycles += 14;
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = sx((c.g[T8][0] as u32) << 2);
            c.g[T9][0] = c.lw(c.g[S1][0] as u32);
            c.g[V1][0] = c.lw(c.g[A1][0] as u32);
            c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
            c.g[T7][0] = c.g[T9][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
            c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[S2][0] = c.g[T7][0];
            c.g[T2][0] = c.g[V0][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T7][0] as u32));
            c.g[T1][0] = c.g[S0][0];
            c.g[T3][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T7][0] as u32));
            'l_413c28: loop {
                c.cycles += 5;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4));
                c.g[T6][0] = c.lw(c.g[A1][0] as u32);
                let t = c.g[T1][0] == 0u64;
                c.g[T5][0] = sx((c.g[T5][0] as u32).wrapping_sub(0x1));
                // Copy the run when words remain and this level
                // outranks the next; otherwise rejoin below.
                if !t {
                    c.cycles += 6;
                    c.g[V1][0] = c.lw(c.g[T3][0] as u32);
                    c.g[A2][0] = c.lw((c.g[T6][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A2][0] as u32));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[V0][0] = sx((c.g[T1][0] as u32) << 2);
                    if !t {
                        c.cycles += 5;
                        c.g[T4][0] = c.g[A2][0];
                        c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[A3][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[T9][0] as u32));
                        c.g[A2][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[S2][0] as u32));
                        c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                        'l_413c68: loop {
                            c.cycles += 8;
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x4));
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4));
                            c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x4));
                            c.sw(c.g[T0][0] as u32, c.g[V0][0]);
                            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x4));
                            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1));
                            let t = c.g[T1][0] == 0u64;
                            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x4));
                            if t {
                                break;
                            }
                            c.cycles += 5;
                            c.g[V1][0] = c.lw(c.g[A3][0] as u32);
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[T4][0] as u32));
                            if (c.g[V0][0] as i64) > 0 {
                                c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                                if event_due(c) {
                                    deliver_event_here(c, 0x0041_3c68);
                                }
                                continue 'l_413c68;
                            }
                            break;
                        }
                    }
                }
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[T5][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T7][0] as u32));
                let t = c.g[T5][0] != 0u64;
                c.sw(c.g[V0][0] as u32, c.g[T6][0]);
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0041_3c28);
                    }
                    continue 'l_413c28;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[T8][0] as u32));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    break;
                }
            }
        }
        c.cycles += 5;
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        return Ok(());
    })
}

/// `void end_fde_sort(...)` (0x413cd0): sort the frame description entries,
/// splitting and merging as needed. Kept in the generated dispatcher shape.
pub fn end_fde_sort(c: &mut Cpu) {
    call::guest(c, |c| {
        let mut pc: u32 = c.pc;
        let frame = Frame { size: 0x40 };
        loop {
            match pc {
                0x0041_3cd0 => {
                    c.cycles += 9;
                    Frame::open(c, 0x40);
                    c.sd(frame.at(c, 0x10), c.g[S1][0]);
                    c.sd(frame.at(c, 0x30), c.g[RA][0]);
                    c.g[S1][0] = c.g[A0][0];
                    c.sd(frame.at(c, 0x20), c.g[S2][0]);
                    c.sd(frame.at(c, 0), c.g[S0][0]);
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    let t = c.g[S0][0] == c.g[A1][0];
                    c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x0041_3cfc;
                        continue;
                    }
                    pc = 0x0041_3cf4;
                    continue;
                }
                0x0041_3cf4 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0041_3cfc);
                    ABORT.call(c, 0x0041_3cfc)?;
                    pc = 0x0041_3cfc;
                    continue;
                }
                0x0041_3cfc => {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x0041_3d08);
                    c.g[A1][0] = c.g[S2][0];
                    callee::FDE_SPLIT.call(c, 0x0041_3d08)?;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    if c.g[V0][0] == c.g[S0][0] {
                        c.g[T5][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                        pc = 0x0041_3d24;
                        continue;
                    }
                    pc = 0x0041_3d1c;
                    continue;
                }
                0x0041_3d1c => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0041_3d24);
                    ABORT.call(c, 0x0041_3d24)?;
                    pc = 0x0041_3d24;
                    continue;
                }
                0x0041_3d24 => {
                    c.cycles += 3;
                    c.g[T3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    let t = c.g[T5][0] == 0u64;
                    c.g[T6][0] = c.g[T5][0];
                    if t {
                        pc = 0x0041_3dfc;
                        continue;
                    }
                    pc = 0x0041_3d30;
                    continue;
                }
                0x0041_3d30 => {
                    c.cycles += 2;
                    c.g[T7][0] = (c.g[T5][0] < 0x2u64) as u64;
                    c.g[A0][0] = sx((c.g[T6][0] as u32).wrapping_sub(0x1));
                    pc = 0x0041_3d38;
                    continue;
                }
                0x0041_3d38 => {
                    c.cycles += 3;
                    c.g[T2][0] = sx((c.g[A0][0] as u32) << 1);
                    let t = true;
                    c.g[T6][0] = c.g[A0][0];
                    if t {
                        pc = 0x0041_3ddc;
                        continue;
                    }
                    pc = 0x0041_3d44;
                    continue;
                }
                0x0041_3d44 => {
                    c.cycles += 1;
                    pc = 0x0041_3d48;
                    continue;
                }
                0x0041_3d48 => {
                    c.cycles += 4;
                    c.g[T4][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
                    c.g[V0][0] = (c.g[T4][0] < c.g[T5][0]) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A3][0] = sx((c.g[A0][0] as u32) << 3);
                    if t {
                        pc = 0x0041_3da0;
                        continue;
                    }
                    pc = 0x0041_3d58;
                    continue;
                }
                0x0041_3d58 => {
                    c.cycles += 8;
                    c.g[T0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[T3][0] as u32));
                    c.g[T1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                    c.g[A2][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[A1][0] = sx((c.g[A0][0] as u32) << 2);
                    if t {
                        pc = 0x0041_3da4;
                        continue;
                    }
                    pc = 0x0041_3d78;
                    continue;
                }
                0x0041_3d78 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T3][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[T3][0] as u32));
                    if t {
                        pc = 0x0041_3da8;
                        continue;
                    }
                    pc = 0x0041_3d90;
                    continue;
                }
                0x0041_3d90 => {
                    c.cycles += 4;
                    c.sw(c.g[V1][0] as u32, c.g[T1][0]);
                    c.sw((c.g[T0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                    let t = true;
                    c.g[A0][0] = c.g[T4][0];
                    if t {
                        pc = 0x0041_3dd8;
                        continue;
                    }
                    pc = 0x0041_3da0;
                    continue;
                }
                0x0041_3da0 => {
                    c.cycles += 1;
                    c.g[A1][0] = sx((c.g[A0][0] as u32) << 2);
                    pc = 0x0041_3da4;
                    continue;
                }
                0x0041_3da4 => {
                    c.cycles += 1;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[T3][0] as u32));
                    pc = 0x0041_3da8;
                    continue;
                }
                0x0041_3da8 => {
                    c.cycles += 8;
                    c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T3][0] as u32));
                    c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                    c.g[A1][0] = c.lw(c.g[A2][0] as u32);
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                    let t = (c.g[V0][0] as i64) <= 0;
                    if t {
                        pc = 0x0041_3dec;
                        continue;
                    }
                    pc = 0x0041_3dc8;
                    continue;
                }
                0x0041_3dc8 => {
                    c.cycles += 4;
                    c.sw(c.g[A2][0] as u32, c.g[A0][0]);
                    c.sw((c.g[A3][0] as u32).wrapping_add(0x4), c.g[A1][0]);
                    c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(0x1));
                    pc = 0x0041_3dd8;
                    continue;
                }
                0x0041_3dd8 => {
                    c.cycles += 1;
                    c.g[T2][0] = sx((c.g[A0][0] as u32) << 1);
                    pc = 0x0041_3ddc;
                    continue;
                }
                0x0041_3ddc => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = (c.g[V0][0] < c.g[T5][0]) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = c.g[T2][0];
                    if t {
                        if back_edge_unwinds(c, 0x0041_3d48) {
                            return Ok(());
                        }
                        pc = 0x0041_3d48;
                        continue;
                    }
                    pc = 0x0041_3dec;
                    continue;
                }
                0x0041_3dec => {
                    c.cycles += 2;
                    let t = c.g[T6][0] != 0u64;
                    c.g[A0][0] = sx((c.g[T6][0] as u32).wrapping_sub(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x0041_3d38) {
                            return Ok(());
                        }
                        pc = 0x0041_3d38;
                        continue;
                    }
                    pc = 0x0041_3df4;
                    continue;
                }
                0x0041_3df4 => {
                    c.cycles += 2;
                    let t = true;
                    if t {
                        pc = 0x0041_3e00;
                        continue;
                    }
                    pc = 0x0041_3dfc;
                    continue;
                }
                0x0041_3dfc => {
                    c.cycles += 1;
                    c.g[T7][0] = 0x1;
                    pc = 0x0041_3e00;
                    continue;
                }
                0x0041_3e00 => {
                    c.cycles += 2;
                    let t = c.g[T7][0] != 0u64;
                    c.g[A1][0] = c.g[S2][0];
                    if t {
                        pc = 0x0041_3f08;
                        continue;
                    }
                    pc = 0x0041_3e08;
                    continue;
                }
                0x0041_3e08 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[T5][0] as u32) << 2);
                    c.g[S0][0] = 0x1;
                    c.g[T6][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T3][0] as u32));
                    c.g[T6][0] = sx((c.g[T6][0] as u32).wrapping_sub(0x4));
                    pc = 0x0041_3e18;
                    continue;
                }
                0x0041_3e18 => {
                    c.cycles += 8;
                    c.g[T5][0] = sx((c.g[T5][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = c.lw(c.g[T3][0] as u32);
                    c.g[V1][0] = c.lw(c.g[T6][0] as u32);
                    c.sw(c.g[T3][0] as u32, c.g[V1][0]);
                    c.sw(c.g[T6][0] as u32, c.g[V0][0]);
                    c.g[V0][0] = (c.g[S0][0] < c.g[T5][0]) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = 0u64;
                    if t {
                        pc = 0x0041_3ef8;
                        continue;
                    }
                    pc = 0x0041_3e38;
                    continue;
                }
                0x0041_3e38 => {
                    c.cycles += 4;
                    c.g[T2][0] = 0u64;
                    c.g[T7][0] = (c.g[T5][0] < 0x2u64) as u64;
                    c.g[T4][0] = sx((c.g[T2][0] as u32).wrapping_add(0x2));
                    pc = 0x0041_3e48;
                    continue;
                }
                0x0041_3e48 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[T4][0] < c.g[T5][0]) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A3][0] = sx((c.g[A0][0] as u32) << 3);
                    if t {
                        pc = 0x0041_3ea0;
                        continue;
                    }
                    pc = 0x0041_3e54;
                    continue;
                }
                0x0041_3e54 => {
                    c.cycles += 8;
                    c.g[T0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[T3][0] as u32));
                    c.g[T1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                    c.g[A2][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[A1][0] = sx((c.g[A0][0] as u32) << 2);
                    if t {
                        pc = 0x0041_3ea4;
                        continue;
                    }
                    pc = 0x0041_3e74;
                    continue;
                }
                0x0041_3e74 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T3][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[T3][0] as u32));
                    if t {
                        pc = 0x0041_3ea8;
                        continue;
                    }
                    pc = 0x0041_3e8c;
                    continue;
                }
                0x0041_3e8c => {
                    c.cycles += 4;
                    c.sw(c.g[V1][0] as u32, c.g[T1][0]);
                    c.sw((c.g[T0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                    let t = true;
                    c.g[A0][0] = c.g[T4][0];
                    if t {
                        pc = 0x0041_3ed8;
                        continue;
                    }
                    pc = 0x0041_3e9c;
                    continue;
                }
                0x0041_3e9c => {
                    c.cycles += 1;
                    pc = 0x0041_3ea0;
                    continue;
                }
                0x0041_3ea0 => {
                    c.cycles += 1;
                    c.g[A1][0] = sx((c.g[A0][0] as u32) << 2);
                    pc = 0x0041_3ea4;
                    continue;
                }
                0x0041_3ea4 => {
                    c.cycles += 1;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[T3][0] as u32));
                    pc = 0x0041_3ea8;
                    continue;
                }
                0x0041_3ea8 => {
                    c.cycles += 8;
                    c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T3][0] as u32));
                    c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                    c.g[A1][0] = c.lw(c.g[A2][0] as u32);
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                    let t = (c.g[V0][0] as i64) <= 0;
                    if t {
                        pc = 0x0041_3efc;
                        continue;
                    }
                    pc = 0x0041_3ec8;
                    continue;
                }
                0x0041_3ec8 => {
                    c.cycles += 4;
                    c.sw(c.g[A2][0] as u32, c.g[A0][0]);
                    c.sw((c.g[A3][0] as u32).wrapping_add(0x4), c.g[A1][0]);
                    c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(0x1));
                    pc = 0x0041_3ed8;
                    continue;
                }
                0x0041_3ed8 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 1);
                    c.g[T2][0] = c.g[V1][0];
                    c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = (c.g[V0][0] < c.g[T5][0]) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[T4][0] = sx((c.g[T2][0] as u32).wrapping_add(0x2));
                    if t {
                        if back_edge_unwinds(c, 0x0041_3e48) {
                            return Ok(());
                        }
                        pc = 0x0041_3e48;
                        continue;
                    }
                    pc = 0x0041_3ef0;
                    continue;
                }
                0x0041_3ef0 => {
                    c.cycles += 2;
                    let t = true;
                    if t {
                        pc = 0x0041_3efc;
                        continue;
                    }
                    pc = 0x0041_3ef8;
                    continue;
                }
                0x0041_3ef8 => {
                    c.cycles += 1;
                    c.g[T7][0] = (c.g[T5][0] < 0x2u64) as u64;
                    pc = 0x0041_3efc;
                    continue;
                }
                0x0041_3efc => {
                    c.cycles += 2;
                    let t = c.g[T7][0] == 0u64;
                    c.g[T6][0] = sx((c.g[T6][0] as u32).wrapping_sub(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x0041_3e18) {
                            return Ok(());
                        }
                        pc = 0x0041_3e18;
                        continue;
                    }
                    pc = 0x0041_3f04;
                    continue;
                }
                0x0041_3f04 => {
                    c.cycles += 1;
                    c.g[A1][0] = c.g[S2][0];
                    pc = 0x0041_3f08;
                    continue;
                }
                0x0041_3f08 => {
                    c.cycles += 11;
                    c.g[RA][0] = sx(0x0041_3f10);
                    c.g[A0][0] = c.g[S1][0];
                    callee::FDE_MERGE.call(c, 0x0041_3f10)?;
                    c.g[RA][0] = sx(0x0041_3f18);
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    FREE.call(c, 0x0041_3f18)?;
                    c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                    frame.restore_ra(c, 0x30);
                    c.g[S2][0] = c.ld(frame.at(c, 0x20));
                    c.g[S1][0] = c.ld(frame.at(c, 0x10));
                    c.g[S0][0] = c.ld(frame.at(c, 0));
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00413cd0, 0x00413f34);
                    return Ok(());
                }
            }
        }
    })
}

/// `const char *type_info::name() const` (0x4151d8): the mangled-name
/// pointer at `*(a0)`.
pub fn type_info_name(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
    c.pc = t;
}

/// `type_info::type_info(const char *)` (0x4151e0): install the vtable
/// word at `+4` and store the name pointer at `+0`. Straight line.
pub fn type_info_ctor(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = 0x4a0000;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x65b8));
    c.sw(c.g[V0][0] as u32, c.g[A1][0]);
    let t = c.g[RA][0] as u32;
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
    c.pc = t;
}

/// `bool type_info::before(const type_info &) const` (0x415238):
/// compare the mangled names with `strcmp`, order by its sign.
pub fn type_info_before(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(c.g[A0][0] as u32);
        c.g[RA][0] = sx(0x0041_524c);
        c.g[A1][0] = c.lw(c.g[A1][0] as u32);
        STR_CMP.call(c, 0x0041_524c)?;
        c.g[RA][0] = c.ld(frame.at(c, 0));
        c.g[V0][0] = sx(c.g[V0][0] as u32 >> 31);
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        Ok(())
    })
}

/// Tail of `get_reg_addr`: the class-1 check, abort path, and epilogue.
pub(crate) fn found_reg(c: &mut Cpu, frame: Frame) -> call::Flow {
    c.cycles += 4;
    c.g[A0][0] = 0x1;
    c.g[V1][0] = c.lb(c.g[V0][0] as u32);
    let t = c.g[V1][0] != c.g[A0][0];
    c.g[V1][0] = sx((c.g[A2][0] as u32) << 3);
    if !t {
        c.cycles += 5;
        c.g[A0][0] = c.lw(c.g[A1][0] as u32);
        c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    } else {
        c.cycles += 2;
        callee::ABORT.call(c, 0x0041_0af4)?;
    }
    c.cycles += 3;
    c.g[RA][0] = c.ld(frame.at(c, 0));
    let t = c.g[RA][0] as u32;
    frame.close(c);
    c.pc = t;
    Ok(())
}

/// Shared tail of `next_stack_level`: shuffle the level words and return.
pub(crate) fn level_done(c: &mut Cpu, frame: Frame) -> call::Flow {
    c.cycles += 5;
    c.g[V1][0] = c.lw(c.g[S0][0] as u32);
    c.g[V0][0] = c.g[S0][0];
    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.sw(c.g[S0][0] as u32, c.g[V1][0]);
    finish(c, frame)
}

/// Epilogue shared by `next_stack_level` and `level_done`: restore the
/// saved registers and return.
pub(crate) fn finish(c: &mut Cpu, frame: Frame) -> call::Flow {
    c.cycles += 5;
    c.g[RA][0] = c.ld(frame.at(c, 0x20));
    c.g[S1][0] = c.ld(frame.at(c, 0x10));
    c.g[S0][0] = c.ld(frame.at(c, 0));
    let t = c.g[RA][0] as u32;
    frame.close(c);
    c.pc = t;
    Ok(())
}

/// Shared epilogue of `find_exception_handler`: return `s3` and restore.
pub(crate) fn found(c: &mut Cpu, frame: Frame) -> call::Flow {
    c.cycles += 11;
    c.g[V0][0] = c.g[S3][0];
    c.g[RA][0] = c.ld(frame.at(c, 0x70));
    c.g[S6][0] = c.ld(frame.at(c, 0x60));
    c.g[S5][0] = c.ld(frame.at(c, 0x50));
    c.g[S4][0] = c.ld(frame.at(c, 0x40));
    c.g[S3][0] = c.ld(frame.at(c, 0x30));
    c.g[S2][0] = c.ld(frame.at(c, 0x20));
    c.g[S1][0] = c.ld(frame.at(c, 0x10));
    c.g[S0][0] = c.ld(frame.at(c, 0));
    let t = c.g[RA][0] as u32;
    frame.close(c);
    c.pc = t;
    Ok(())
}

/// Shared body of the four exception ctor/dtor one-liners: store the
/// vtable word `base - off` at `a0` (`dtor` conditionally runs the
/// `operator delete` at 0x317e50 first when `a1 & 1` is set).
pub(crate) fn exception_ctor_impl(c: &mut Cpu, base: u32, off: u32) {
    c.cycles += 5;
    c.g[V1][0] = base as u64;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(off));
    let t = c.g[RA][0] as u32;
    c.sw(c.g[A0][0] as u32, c.g[V1][0]);
    c.pc = t;
}

pub(crate) mod throw_save {

    /// Current save-area cursor (starts at `s1 + 0x2e0`).
    pub const CUR: u32 = 0x2f0;
}

// ---------------------------------------------------------------------------
// fde_split

/// `void fde_split(void *fdes, void *out)` (0x4149f0): split the frame's
/// FDE table at each overlapping row, compacting the pieces into `out`
/// (the inner loops walk the sorted rows; the outer one steps the table).
pub fn fde_split(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 18;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.g[T7][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[FP][0]);
        c.g[A0][0] = 0xffff_ffff_ffff_0000;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S0][0]);
        c.g[FP][0] = c.g[SP][0];
        c.g[T4][0] = c.g[A1][0];
        c.g[A0][0] |= 0xffff;
        c.g[T8][0] = c.lw((c.g[T7][0] as u32).wrapping_add(0x4));
        c.g[T1][0] = 0;
        c.g[V0][0] = sx((c.g[T8][0] as u32) << 5);
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf));
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 4);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        c.g[S0][0] = c.g[SP][0];
        if c.g[T8][0] == 0 {
            c.g[T0][0] = 0;
        } else {
            c.cycles += 2;
            c.g[T5][0] = 0xffff_ffff_ffff_0000;
            c.g[T5][0] |= 0xffff;
            'outer: loop {
                c.cycles += 3;
                c.g[T0][0] = c.g[A0][0];
                c.g[T3][0] = sx((c.g[T1][0] as u32) << 2);
                if c.g[T0][0] != c.g[T5][0] {
                    c.cycles += 11;
                    c.g[A3][0] = c.lw(c.g[T7][0] as u32);
                    c.g[A2][0] = sx((c.g[T0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                    c.g[A1][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                    c.g[T9][0] = c.g[T3][0];
                    if (c.g[V0][0] as i64) < 0 {
                        c.cycles += 2;
                        c.g[T6][0] = c.lw(c.g[T4][0] as u32);
                        c.g[T2][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                        'inner: loop {
                            c.cycles += 14;
                            c.g[A0][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A3][0] as u32));
                            c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                            c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
                            c.g[T0][0] = c.lw(c.g[V1][0] as u32);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T6][0] as u32));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                            c.sw(c.g[V0][0] as u32, c.g[A1][0]);
                            c.sw((c.g[T4][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                            c.g[A3][0] = c.lw(c.g[T7][0] as u32);
                            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A3][0] as u32));
                            c.sw(c.g[V0][0] as u32, 0);
                            if c.g[T0][0] == c.g[T5][0] {
                                // Table end: rejoin past the inner latch.
                                c.cycles += 2;
                                c.g[A0][0] = c.g[T1][0];
                                c.g[V1][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[T3][0] as u32));
                                break;
                            }
                            c.cycles += 11;
                            c.g[V0][0] = sx((c.g[T0][0] as u32) << 2);
                            c.g[V1][0] = sx((c.g[T9][0] as u32).wrapping_add(c.g[A3][0] as u32));
                            c.g[A2][0] = c.g[V0][0];
                            c.g[A1][0] = c.lw(c.g[V1][0] as u32);
                            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A3][0] as u32));
                            c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                            c.g[A0][0] = c.g[T1][0];
                            if (c.g[V1][0] as i64) < 0 {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0041_4a80);
                                }
                                continue 'inner;
                            }
                            c.cycles += 2;
                            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[T3][0] as u32));
                            break;
                        }
                    } else {
                        // Overlap check passed: rejoin past the inner latch.
                        c.cycles += 1;
                        c.g[T2][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                        c.cycles += 2;
                        c.g[A0][0] = c.g[T1][0];
                        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[T3][0] as u32));
                    }
                } else {
                    // Head sentinel: rejoin past the inner latch.
                    c.cycles += 1;
                    c.g[T2][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                    c.cycles += 2;
                    c.g[A0][0] = c.g[T1][0];
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[T3][0] as u32));
                }
                c.cycles += 4;
                c.g[T1][0] = c.g[T2][0];
                c.g[V0][0] = (c.g[T1][0] < c.g[T8][0]) as u64;
                c.sw(c.g[V1][0] as u32, c.g[T0][0]);
                if c.g[V0][0] != 0 {
                    if event_due(c) {
                        deliver_event_here(c, 0x0041_4a40);
                    }
                    continue 'outer;
                }
                c.cycles += 1;
                c.g[T1][0] = 0;
                break;
            }
        }
        if c.g[T8][0] != 0 {
            // (generated `'b_414b4c` zeroes T0 under a +2 on both arms).
            c.cycles += 2;
            c.g[T0][0] = 0;
            c.cycles += 3;
            c.g[A3][0] = c.lw(c.g[T7][0] as u32);
            c.g[V0][0] = sx((c.g[T1][0] as u32) << 2);
            'pack: loop {
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.g[V0][0] = sx((c.g[T0][0] as u32) << 2);
                if c.g[V1][0] == 0 {
                    // Slot empty: latch and keep packing.
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                    c.sw(c.g[V0][0] as u32, c.g[V1][0]);
                }
                c.cycles += 4;
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = (c.g[T1][0] < c.g[T8][0]) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = sx((c.g[T1][0] as u32) << 2);
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0041_4b20);
                    }
                    continue 'pack;
                }
                break;
            }
        } else {
            // T8 == 0: the +2 and the T0 zero still happen.
            c.cycles += 2;
            c.g[T0][0] = 0;
        }
        c.cycles += 6;
        c.sw((c.g[T7][0] as u32).wrapping_add(0x4), c.g[T0][0]);
        c.g[SP][0] = c.g[FP][0];
        c.g[FP][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// find_fde

/// `void *find_fde(void *pc)` (0x414148): the FDE covering `pc`, or null.
/// Walks the frame registry (initializing each table through `frame_init`
/// once); the scan and the binary search both check events, so an unwind
/// resumes at their heads.
pub fn find_fde(c: &mut Cpu) {
    call::guest(c, |c| {
        if c.pc != 0x0041_4148 && c.pc != 0x0041_4168 && c.pc != 0x0041_41d0 && c.pc != 0x0041_41b4
        {
            c.jump_out(c.pc, 0x0041_4148, 0x0041_422c);
            return Ok(());
        }
        if c.pc == 0x0041_4148 {
            c.cycles += 8;
            let frame = Frame::open(c, 0x30);
            c.g[V0][0] = 0x530000;
            frame_sd(c, frame, 0, c.g[S0][0]);
            frame_sd(c, frame, 0x10, c.g[S1][0]);
            c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x5218));
            c.g[S1][0] = c.g[A0][0];
            frame.save_ra(c, 0x20);
            c.pc = 0x0041_4168;
        }
        loop {
            match c.pc {
                0x0041_4168 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    c.pc = if c.g[S0][0] == 0 {
                        0x0041_4218
                    } else {
                        0x0041_4170
                    };
                }
                0x0041_4170 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                    let t = c.g[V0][0] != 0;
                    c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                    c.pc = if t { 0x0041_418c } else { 0x0041_417c };
                }
                0x0041_417c => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    callee::FRAME_INIT.call(c, 0x0041_4184)?;
                    c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                    c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                    c.pc = 0x0041_418c;
                }
                0x0041_418c => {
                    c.cycles += 2;
                    if c.g[V0][0] != 0 {
                        c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                        if back_edge_unwinds(c, 0x0041_4168) {
                            return Ok(());
                        }
                        c.pc = 0x0041_4168;
                    } else {
                        c.pc = 0x0041_4194;
                    }
                }
                0x0041_4194 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                        if back_edge_unwinds(c, 0x0041_4168) {
                            return Ok(());
                        }
                        c.pc = 0x0041_4168;
                    } else {
                        c.pc = 0x0041_41a4;
                    }
                }
                0x0041_41a4 => {
                    c.cycles += 2;
                    if c.g[S0][0] != 0 {
                        c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                        c.pc = 0x0041_41bc;
                    } else {
                        c.pc = 0x0041_41ac;
                    }
                }
                0x0041_41ac => {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    c.pc = 0x0041_4218;
                }
                0x0041_41b4 => {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[V1][0];
                    c.pc = 0x0041_4218;
                }
                0x0041_41bc => {
                    c.cycles += 2;
                    c.g[T0][0] = 0;
                    if c.g[A3][0] == 0 {
                        c.pc = 0x0041_4214;
                    } else {
                        c.pc = 0x0041_41c4;
                    }
                }
                0x0041_41c4 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.pc = 0x0041_41d0;
                }
                0x0041_41d0 => {
                    c.cycles += 8;
                    c.g[A2][0] = sx((c.g[V0][0] as u32) >> 1);
                    c.g[V1][0] = sx((c.g[A2][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V1][0] as u32);
                    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = (c.g[S1][0] < c.g[A1][0]) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                        c.pc = 0x0041_41f8;
                    } else {
                        c.pc = 0x0041_41f0;
                    }
                }
                0x0041_41f0 => {
                    c.cycles += 2;
                    c.g[A3][0] = c.g[A2][0];
                    c.pc = 0x0041_4208;
                }
                0x0041_41f8 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                    c.g[T0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    if c.g[V0][0] != 0 {
                        if back_edge_unwinds(c, 0x0041_41b4) {
                            return Ok(());
                        }
                        c.pc = 0x0041_41b4;
                    } else {
                        c.pc = 0x0041_4208;
                    }
                }
                0x0041_4208 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[T0][0] < c.g[A3][0]) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    if t {
                        if back_edge_unwinds(c, 0x0041_41d0) {
                            return Ok(());
                        }
                        c.pc = 0x0041_41d0;
                    } else {
                        c.pc = 0x0041_4214;
                    }
                }
                0x0041_4214 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    c.pc = 0x0041_4218;
                }
                0x0041_4218 => {
                    c.cycles += 5;
                    let frame = Frame { size: 0x30 };
                    frame.restore_ra(c, 0x20);
                    c.g[S1][0] = frame_ld(c, frame, 0x10);
                    c.g[S0][0] = frame_ld(c, frame, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => break,
            }
        }
        Ok(())
    })
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
            // The C++ `terminate()` (0x412800), not the local `TERMINATE`
            // (0x4103b0 `__terminate`): same call the generated body makes.
            callee::TERMINATE_CPP.call(c, 0x0041_2828)?;
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

// ---------------------------------------------------------------------------
// bad_typeid_dtor / bad_cast_dtor

/// `bad_typeid::~bad_typeid(int)` (0x415088): install the vtable, then free
/// the object when the low bit of `a1` says it owns it.
pub fn bad_typeid_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 7;
        c.g[V0][0] = 0x4a0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6748));
        frame.save_ra(c, 0);
        c.g[A1][0] &= 0x1;
        c.sw(c.g[A0][0] as u32, c.g[V0][0]);
        if c.g[A1][0] != 0 {
            c.cycles += 2;
            c.g[RA][0] = sx(0x0041_50ac);
            call::call_ptr(c, 0x0031_7e50, 0x0041_50ac)?;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(frame.at(c, 0));
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        Ok(())
    })
}

/// `bad_cast::~bad_cast(int)` (0x415120): install the vtable, then free
/// the object when the low bit of `a1` says it owns it.
pub fn bad_cast_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 7;
        c.g[V0][0] = 0x4a0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6748));
        frame.save_ra(c, 0);
        c.g[A1][0] &= 0x1;
        c.sw(c.g[A0][0] as u32, c.g[V0][0]);
        if c.g[A1][0] != 0 {
            c.cycles += 2;
            c.g[RA][0] = sx(0x0041_5144);
            call::call_ptr(c, 0x0031_7e50, 0x0041_5144)?;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(frame.at(c, 0));
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// bad_typeid_ctor / bad_cast_ctor

/// `bad_typeid::bad_typeid(void)` (0x4150b8): install the vtable.
pub fn bad_typeid_ctor(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V1][0] = 0x4a0000;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x65e8));
    let t = c.g[RA][0] as u32;
    c.sw(c.g[A0][0] as u32, c.g[V1][0]);
    c.pc = t;
}

/// `bad_cast::bad_cast(void)` (0x415150): install the vtable.
pub fn bad_cast_ctor(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V1][0] = 0x4a0000;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x65d0));
    let t = c.g[RA][0] as u32;
    c.sw(c.g[A0][0] as u32, c.g[V1][0]);
    c.pc = t;
}

// ---------------------------------------------------------------------------
// RCMP type_info getters

/// Shared body of the single-base RCMP type_info getters: return the
/// registry slot when something installed it, else build it with
/// `__rtti_user` (`slot` and `name` are the offsets from `0x4a0000` and
/// `0x490000`; `ret` is the `__rtti_user` return address).
pub(crate) fn tf_single_base(c: &mut Cpu, slot_off: u32, name_off: u32, ret: u32) -> Flow {
    c.cycles += 7;
    let frame = Frame::open(c, 0x20);
    c.g[V1][0] = 0x4a0000;
    frame.save(c, S0, 0x10);
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(slot_off));
    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(slot_off));
    frame.save_ra(c, 0);
    if c.g[V0][0] == 0 {
        c.cycles += 4;
        c.g[A1][0] = 0x490000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(name_off));
        callee::RTTI_USER.call(c, ret)?;
    }
    c.cycles += 5;
    c.g[V0][0] = c.g[S0][0];
    c.g[RA][0] = c.ld(frame.at(c, 0));
    frame.restore(c, S0, 0x10);
    let t = c.g[RA][0] as u32;
    frame.close(c);
    c.pc = t;
    Ok(())
}

/// `const type_info& __tfQ24RCMP5CODEC(void)` (0x42e2b0): `RCMP::CODEC`'s
/// type_info (built on first use).
pub fn tfq24rcmp5codec(c: &mut Cpu) {
    call::guest(c, |c| tf_single_base(c, 0x5ba0, 0x5410, 0x0042_e2dc))
}

/// `const type_info& __tfQ24RCMP7DECODER(void)` (0x42e458):
/// `RCMP::DECODER`'s type_info (built on first use).
pub fn tfq24rcmp7decoder(c: &mut Cpu) {
    call::guest(c, |c| tf_single_base(c, 0x5b90, 0x53d8, 0x0042_e484))
}

/// `const type_info& __tfQ24RCMP11RCMP_SYSTEM(void)` (0x42e4c8):
/// `RCMP::RCMP_SYSTEM`'s type_info (built on first use).
pub fn tfq24rcmp11rcmp_system(c: &mut Cpu) {
    call::guest(c, |c| tf_single_base(c, 0x5b98, 0x53c0, 0x0042_e4f4))
}

/// `const type_info& __tf23PS2_SONY_CODEC_INTERNAL(void)` (0x42e200):
/// `PS2_SONY_CODEC_INTERNAL`'s type_info (built on first use, on top of
/// `RCMP::CODEC`'s user type_info).
pub fn tf23ps2_sony_codec_internal(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x540000;
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1600));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x1600));
        c.g[S2][0] = c.g[V0][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        if c.g[V1][0] == 0 {
            c.cycles += 4;
            c.g[V1][0] = 0x4a0000;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5ba0));
            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5ba0));
            if c.g[V0][0] == 0 {
                c.cycles += 4;
                c.g[A1][0] = 0x490000;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5410));
                callee::RTTI_USER.call(c, 0x0042_e248)?;
            }
            c.cycles += 5;
            c.g[A1][0] = 0x490000;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5420));
            c.g[A2][0] = c.g[S0][0];
            callee::RTTI_SI.call(c, 0x0042_e25c)?;
        }
        c.cycles += 7;
        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1600));
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        c.g[RA][0] = c.ld(frame.at(c, 0));
        let t = c.g[RA][0] as u32;
        frame.close(c);
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x0040fcb0 __divdi3

/// Entry of `__divdi3`. Every other address inside it runs the generated
/// twin: the function makes no calls and has no event checks, so nothing
/// resumes there.
pub(crate) const DIVDI3_ENTRY: u32 = 0x0040_fcb0;

/// The low word of a doubleword (the body builds this mask as
/// `0xffff_ffff_ffff_0000 >> 32`).
pub(crate) const DIVDI3_LOW_WORD: u64 = 0xffff_ffff;

/// `W_TYPE_SIZE`: `count_leading_zeros` leaves it in a3.
pub(crate) const DIVDI3_WORD_BITS: u64 = 32;

/// The join a case leaves by, before the block that packs `q1:q0`: the
/// `q1 = 0` join (0x410328), the join after it (0x410330) or the packing
/// block itself (0x410334). Only the cycle charge differs.
#[derive(Clone, Copy)]
pub(crate) enum Divdi3Exit {
    ZeroHigh,
    Join,
    Pack,
}

impl Divdi3Exit {
    /// Cycles of the join blocks this exit runs.
    fn cycles(self) -> u64 {
        match self {
            Divdi3Exit::ZeroHigh => 2 + 1,
            Divdi3Exit::Join => 1,
            Divdi3Exit::Pack => 0,
        }
    }
}

/// The quotient of the magnitudes, as `q1:q0`.
pub(crate) struct Divdi3Quotient {
    high: u32,
    low: u32,
    exit: Divdi3Exit,
}

/// `long long __divdi3(long long n, long long d)` (0x40fcb0): libgcc2's
/// signed 64-bit division, truncating toward zero. It divides the
/// magnitudes with `__udivmoddi4` inlined (32-bit words, 16-bit
/// `udiv_qrnnd` digits, operands normalized by `count_leading_zeros`) and
/// negates the quotient when exactly one operand is negative. It runs in
/// registers only (no stack frame). Division by zero takes libgcc's
/// `d0 = 1 / d0`, which the EE's `divu` answers with 0xffffffff (no trap),
/// and runs on with that divisor (`5 / 0` gives 0x1_0000_0001).
///
/// Entered anywhere but its start, it runs the generated body.
pub fn divdi3(c: &mut Cpu) {
    if c.pc != DIVDI3_ENTRY {
        return ee::s15::f_0040fcb0__entries(c);
    }
    c.cycles += 7;
    let (n, d) = (c.g[A0][0], c.g[A1][0]);
    // The magnitudes; t7 says whether the quotient is negated. Each
    // negation leaves the low word of its result in a scratch register.
    let mut negate = false;
    let n = if (n as i64) < 0 {
        c.cycles += 21;
        negate = true;
        let n = n.wrapping_neg();
        c.g[A2][0] = n & DIVDI3_LOW_WORD;
        n
    } else {
        n
    };
    c.cycles += 3;
    let d = if (d as i64) < 0 {
        c.cycles += 20;
        negate = !negate;
        let d = d.wrapping_neg();
        c.g[A3][0] = d & DIVDI3_LOW_WORD;
        d
    } else {
        d
    };
    c.g[T7][0] = if negate { u64::MAX } else { 0 };
    c.g[T1][0] = d;

    // The words: n0 in t4, n1 in t2, d0 in t0 (sign-extended).
    c.cycles += 9;
    let (n1, n0, d1, d0) = ((n >> 32) as u32, n as u32, (d >> 32) as u32, d as u32);
    c.g[T0][0] = sx(d0);
    c.g[T2][0] = sx(n1);
    c.g[T4][0] = sx(n0);
    let q = if d1 == 0 {
        c.cycles += 3;
        if n1 < d0 {
            divdi3_by_word_short(c, n1, n0, d0)
        } else {
            divdi3_by_word_long(c, n1, n0, d0)
        }
    } else {
        c.cycles += 2;
        if d1 > n1 {
            // 00 = nn / DD
            c.cycles += 2;
            Divdi3Quotient {
                high: 0,
                low: 0,
                exit: Divdi3Exit::ZeroHigh,
            }
        } else {
            divdi3_by_doubleword(c, n1, n0, d1, d0)
        }
    };
    divdi3_return(c, q, negate);
}

/// `0q = nn / 0D` (d1 == 0, n1 < d0): one quotient word in two digits after
/// normalizing. Leaves a3 = the high digit, a2 = the low digit's `q * dl`,
/// t1 = dl and the normalized words in t0/t2/t4.
pub(crate) fn divdi3_by_word_short(c: &mut Cpu, n1: u32, n0: u32, d0: u32) -> Divdi3Quotient {
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
    let (high, r) = qrnnd_step(c, n1, n0 >> 16, d0, 11, 3);
    c.cycles += 2;
    let low = qrnnd_step_quotient(c, r, n0 & 0xffff, d0);
    // `mult a2, a1, t1` leaves the product's low word in a2 as well as LO.
    c.g[A2][0] = c.lo[0];
    c.cycles += 5;
    c.g[A3][0] = sx(high);
    c.g[T0][0] = sx(d0);
    c.g[T1][0] = (d0 & 0xffff) as u64;
    c.g[T2][0] = sx(n1);
    c.g[T4][0] = sx(n0);
    Divdi3Quotient {
        high: 0,
        low: high << 16 | low,
        exit: Divdi3Exit::ZeroHigh,
    }
}

/// `qq = NN / 0d` (d1 == 0, n1 >= d0): the high quotient word from `n1`
/// (just `1` with `n1 -= d0` when d0 is normalized, else two digits of
/// `n2:n1 / d0`), then the low word in two digits. Leaves a3 = the third
/// digit, a2 = the last digit's `q * dl`, t1 = t3 = dl, t2 = the remainder
/// of the high word and the normalized d0/n0 in t0/t4.
pub(crate) fn divdi3_by_word_long(c: &mut Cpu, n1: u32, n0: u32, d0: u32) -> Divdi3Quotient {
    c.cycles += 2;
    let mut d0 = d0;
    if d0 == 0 {
        // `d0 = 1 / d0`: the intentional division by zero.
        c.cycles += 3 + 6;
        d0 = abi::divu(c, 1, d0).0;
    }
    c.cycles += 2;
    let bm = clz_tab(c, d0);
    let (mut n1, mut n0) = (n1, n0);
    let high;
    if bm == 0 {
        c.cycles += 5;
        n1 = n1.wrapping_sub(d0);
        high = 1;
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
        high = hi << 16 | lo;
        n1 = r;
    }
    c.g[T2][0] = sx(n1);
    c.cycles += 6;
    let (hi, r) = qrnnd_step(c, n1, n0 >> 16, d0, 11, 2);
    c.cycles += 2;
    let lo = qrnnd_step_quotient(c, r, n0 & 0xffff, d0);
    // `mult a2, a1, t1` leaves the product's low word in a2 as well as LO.
    c.g[A2][0] = c.lo[0];
    c.cycles += 5;
    c.g[A3][0] = sx(hi);
    c.g[T0][0] = sx(d0);
    c.g[T1][0] = (d0 & 0xffff) as u64;
    c.g[T3][0] = (d0 & 0xffff) as u64;
    c.g[T4][0] = sx(n0);
    Divdi3Quotient {
        high,
        low: hi << 16 | lo,
        exit: Divdi3Exit::Join,
    }
}

/// `0q = NN / dd` (d1 != 0, d1 <= n1): a one-word quotient. With d1
/// normalized it is 1 unless `n < d`; otherwise two digits of `n2:n1 / d1`
/// after normalizing, then one less if `q0 * d0` (`umul_ppmm`) exceeds
/// the remainder. Leaves a2 = bm and a3 = 32 for the first; for the second
/// a2 = the high word of `q0 * d0`, a3 = the second digit's `q * dl`,
/// t1 = the first digit, t3 = dl of d1 and the normalized words in
/// t0/t2/t4, with the whole product in LO/HI.
pub(crate) fn divdi3_by_doubleword(
    c: &mut Cpu,
    n1: u32,
    n0: u32,
    d1: u32,
    d0: u32,
) -> Divdi3Quotient {
    c.cycles += 3;
    let bm = clz_tab(c, d1);
    if bm == 0 {
        c.g[A2][0] = 0;
        c.g[A3][0] = DIVDI3_WORD_BITS;
        c.cycles += 3;
        let low = if n1 > d1 {
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
        return Divdi3Quotient {
            high: 0,
            low,
            exit: Divdi3Exit::ZeroHigh,
        };
    }
    c.cycles += 15;
    let b = 32 - bm;
    let d1 = (d1 << bm) | (d0 >> b);
    let d0 = d0 << bm;
    let n2 = n1 >> b;
    let n1 = (n1 << bm) | (n0 >> b);
    let n0 = n0 << bm;
    let (hi, r) = qrnnd_step(c, n2, n1 >> 16, d1, 11, 3);
    c.cycles += 2;
    let (lo, rem) = qrnnd_step(c, r, n1 & 0xffff, d1, 13, 2);
    // `mult a3, a2, t3` leaves the product's low word in a3 as well as LO.
    c.g[A3][0] = c.lo[0];
    c.cycles += 10;
    let q0 = hi << 16 | lo;
    // umul_ppmm(m1, m0, q0, d0)
    abi::multu(c, q0, d0);
    let product = q0 as u64 * d0 as u64;
    let (m1, m0) = ((product >> 32) as u32, product as u32);
    c.g[A2][0] = sx(m1);
    c.g[T0][0] = sx(d0);
    c.g[T1][0] = sx(hi);
    c.g[T2][0] = sx(n1);
    c.g[T3][0] = (d1 & 0xffff) as u64;
    c.g[T4][0] = sx(n0);
    // q0 is one too big when m1:m0 > rem:n0.
    let (low, exit) = if m1 > rem {
        (q0.wrapping_sub(1), Divdi3Exit::ZeroHigh)
    } else {
        c.cycles += 2;
        if m1 != rem {
            (q0, Divdi3Exit::Join)
        } else {
            c.cycles += 3;
            if m0 > n0 {
                c.cycles += 1;
                (q0.wrapping_sub(1), Divdi3Exit::ZeroHigh)
            } else {
                (q0, Divdi3Exit::Pack)
            }
        }
    };
    Divdi3Quotient { high: 0, low, exit }
}

/// Packs `q1:q0` (t5 = q1, t8 = q0, a1 = the low-word mask), negates it
/// when the signs differed (t6 = its low word, v1 = the borrow) and
/// returns it in v0 (and a0).
pub(crate) fn divdi3_return(c: &mut Cpu, q: Divdi3Quotient, negate: bool) {
    c.cycles += q.exit.cycles() + 6;
    c.g[T5][0] = sx(q.high);
    c.g[T8][0] = q.low as u64;
    c.g[A1][0] = DIVDI3_LOW_WORD;
    c.g[V1][0] = (q.high as u64) << 32;
    let quotient = (q.high as u64) << 32 | q.low as u64;
    let result = if negate {
        c.cycles += 18;
        let result = quotient.wrapping_neg();
        c.g[T6][0] = result & DIVDI3_LOW_WORD;
        c.g[V1][0] = (q.low != 0) as u64;
        result
    } else {
        quotient
    };
    c.cycles += 2;
    c.g[A0][0] = result;
    c.g[V0][0] = result;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// 0x0042d470 __moddi3

/// Bytes of `__moddi3`'s stack frame.
pub(crate) const MODDI3_FRAME: u32 = 0x40;

/// `w`, the remainder `__udivmoddi4` stores through `rp` (`rp = &w`).
pub(crate) const MODDI3_SLOT_REMAINDER: u32 = 0x00;

/// The caller's s0 (`sd`/`ld`): s0 holds the sign flag `c`.
pub(crate) const MODDI3_SLOT_S0: u32 = 0x30;

/// The low word of a doubleword (the body builds this mask as
/// `0xffff_ffff_ffff_0000 >> 32`).
pub(crate) const MODDI3_LOW_WORD: u64 = 0xffff_ffff;

/// `W_TYPE_SIZE`: `count_leading_zeros` leaves it in a1 on the `d1 != 0`
/// path, and `32 - bm` in t6.
pub(crate) const MODDI3_WORD_BITS: u32 = 32;

/// The shift `count_leading_zeros` picks for a word above 0xffffff (the top
/// byte), which a normalized d1 always is.
pub(crate) const MODDI3_CLZ_TOP_BYTE: u64 = 24;

/// `long long __moddi3(long long n, long long d)` (0x42d470): libgcc2's
/// signed 64-bit remainder, truncating toward zero, so the result has the
/// dividend's sign. It takes the magnitudes, runs `__udivmoddi4` inlined
/// for the remainder only (32-bit words, 16-bit `udiv_qrnnd` digits,
/// operands normalized by `count_leading_zeros`), which stores it through
/// `rp = &w` (a stack slot), and negates `w` there when n was negative.
/// The compiler keeps `__udivmoddi4`'s `if (rp != 0)` tests (rp is never 0
/// with a real stack) except on the `d1 > n1` exit. Division by zero takes
/// libgcc's `d0 = 1 / d0`, which the EE's `divu` answers with 0xffffffff
/// (no trap), and runs on with that divisor (`5 % 0` gives 6).
pub fn moddi3(c: &mut Cpu) {
    c.cycles += 8;
    let n = c.g[A0][0];
    let frame = Frame::open(c, MODDI3_FRAME);
    let saved_s0 = c.g[S0][0];
    Addr(frame.at(c, MODDI3_SLOT_S0)).sd(c, saved_s0);
    let negative = (n as i64) < 0;
    c.g[S0][0] = if negative { u64::MAX } else { 0 };
    // The magnitudes. Each negation leaves the low word of its result in a
    // scratch register.
    let n = if negative {
        c.cycles += 21;
        let n = n.wrapping_neg();
        c.g[A2][0] = n & MODDI3_LOW_WORD;
        n
    } else {
        n
    };
    c.cycles += 3;
    let d = c.g[A1][0];
    // `negu a0, d1` sits in the branch delay slot: it runs for either sign.
    c.g[A0][0] = sx(((d >> 32) as u32).wrapping_neg());
    let d = if (d as i64) < 0 {
        c.cycles += 18;
        let d = d.wrapping_neg();
        c.g[A0][0] = d & !MODDI3_LOW_WORD;
        c.g[A1][0] = d;
        c.g[A3][0] = d & MODDI3_LOW_WORD;
        d
    } else {
        d
    };

    // The words: n0 in t4, n1 in t2, d0 in t0, d1 in t1 (sign-extended).
    c.cycles += 9;
    let (n1, n0, d1, d0) = ((n >> 32) as u32, n as u32, (d >> 32) as u32, d as u32);
    c.g[T0][0] = sx(d0);
    c.g[T1][0] = sx(d1);
    c.g[T2][0] = sx(n1);
    c.g[T4][0] = sx(n0);
    let rp = c.g[SP][0];
    c.g[T9][0] = rp;
    c.cycles += 3;
    if d1 == 0 {
        if n1 < d0 {
            moddi3_by_word_short(c, rp, n1, n0, d0);
        } else {
            moddi3_by_word_long(c, rp, n1, n0, d0);
        }
    } else if n1 < d1 {
        // 00 = nn / DD: the remainder is n, stored with no `rp` test.
        c.cycles += 5;
        c.g[T7][0] = n;
        Addr(frame.at(c, MODDI3_SLOT_REMAINDER)).sd(c, n);
    } else {
        moddi3_by_doubleword(c, rp, n1, n0, d1, d0);
    }
    moddi3_return(c, frame, negative);
}

/// `0q = nn / 0D` (d1 == 0, n1 < d0): normalizes, then the low word's two
/// digits. Leaves t5 = bm and the normalized words in t0/t2.
pub(crate) fn moddi3_by_word_short(c: &mut Cpu, rp: u64, n1: u32, n0: u32, d0: u32) {
    c.cycles += 3;
    let bm = clz_tab(c, d0);
    c.g[T5][0] = bm as u64;
    let (mut d0, mut n1, mut n0) = (d0, n1, n0);
    if bm != 0 {
        c.cycles += 5;
        n1 = (n1 << bm) | (n0 >> (MODDI3_WORD_BITS - bm));
        d0 <<= bm;
        n0 <<= bm;
        c.g[T0][0] = sx(d0);
        c.g[T2][0] = sx(n1);
        c.g[T4][0] = sx(n0);
    }
    c.cycles += 4;
    moddi3_low_word(c, rp, n1, n0, d0, bm);
}

/// `qq = NN / 0d` (d1 == 0, n1 >= d0): reduces n1 below d0 first (just
/// `n1 -= d0` when d0 is normalized, else the two digits of `n2:n1 / d0`),
/// then the low word's two digits. Leaves t5 = bm, t6 = `32 - bm`, t1 = dl,
/// t2 = the reduced n1 and, after normalizing, t3 = dl.
pub(crate) fn moddi3_by_word_long(c: &mut Cpu, rp: u64, n1: u32, n0: u32, d0: u32) {
    c.cycles += 2;
    let mut d0 = d0;
    if d0 == 0 {
        // `d0 = 1 / d0`: the intentional division by zero (the body divides
        // by d1, which is 0 too).
        c.cycles += 3 + 5;
        d0 = abi::divu(c, 1, 0).0;
        c.g[T0][0] = sx(d0);
    }
    c.cycles += 2;
    let bm = clz_tab(c, d0);
    c.g[T5][0] = bm as u64;
    c.g[T6][0] = (MODDI3_WORD_BITS - bm) as u64;
    let (mut n1, mut n0) = (n1, n0);
    if bm == 0 {
        c.cycles += 4;
        n1 = n1.wrapping_sub(d0);
    } else {
        c.cycles += 13;
        let b = MODDI3_WORD_BITS - bm;
        d0 <<= bm;
        let n2 = n1 >> b;
        n1 = (n1 << bm) | (n0 >> b);
        n0 <<= bm;
        c.g[T0][0] = sx(d0);
        c.g[T3][0] = (d0 & 0xffff) as u64;
        c.g[T4][0] = sx(n0);
        let r = mod_step_first(c, n2, n1 >> 16, d0);
        c.cycles += 4;
        n1 = mod_step(c, r, n1 & 0xffff, d0);
    }
    c.g[T1][0] = (d0 & 0xffff) as u64;
    c.g[T2][0] = sx(n1);
    c.cycles += 3;
    moddi3_low_word(c, rp, n1, n0, d0, bm);
}

/// The low word of a `d1 == 0` division: two digits of `n1:n0 / d0` (d0
/// normalized, n1 < d0), then `*rp = n0 >> bm` with a zero high word.
/// Leaves a1 = dh, a3 = dl, a0 = the last digit's part, a2 = its `q * dl`
/// and t4 = the normalized remainder.
pub(crate) fn moddi3_low_word(c: &mut Cpu, rp: u64, n1: u32, n0: u32, d0: u32, bm: u32) {
    c.g[A1][0] = (d0 >> 16) as u64;
    c.g[A3][0] = (d0 & 0xffff) as u64;
    c.cycles += 3;
    let r = mod_step(c, n1, n0 >> 16, d0);
    c.cycles += 4;
    c.g[A0][0] = (n0 & 0xffff) as u64;
    let (rem, corrected) = mod_step_last(c, r, n0 & 0xffff, d0);
    // `mult a2, v0, a3` left `m = q * dl` in a2.
    c.g[A2][0] = sx(corrected.wrapping_sub(rem));
    c.cycles += 2;
    c.g[T4][0] = sx(rem);
    if rp != 0 {
        c.cycles += 4;
        moddi3_store_remainder(c, rp, (rem >> bm) as u64);
    }
}

/// `0q = NN / dd` (d1 != 0, d1 <= n1): one quotient digit pair after
/// normalizing (with d1 normalized the quotient is 0 or 1 and n just loses
/// d once, which only `MIN % MIN` reaches), corrected by `umul_ppmm(q0,
/// d0)`, then `*rp = (n - m) >> bm`. Leaves t5 = bm, t6 = `32 - bm`, the
/// normalized words in t0/t1/t4, t3 = the first digit and in a2/a3 the
/// corrected product `m0`/`m1`.
pub(crate) fn moddi3_by_doubleword(c: &mut Cpu, rp: u64, n1: u32, n0: u32, d1: u32, d0: u32) {
    c.cycles += 4;
    let bm = clz_tab(c, d1);
    c.g[T5][0] = bm as u64;
    c.g[T6][0] = (MODDI3_WORD_BITS - bm) as u64;
    if bm == 0 {
        c.g[A1][0] = MODDI3_WORD_BITS as u64;
        c.g[A2][0] = MODDI3_CLZ_TOP_BYTE;
        c.cycles += 3;
        let low = n0.wrapping_sub(d0);
        c.g[A0][0] = sx(low);
        let subtract = if n1 > d1 {
            true
        } else {
            c.cycles += 3;
            n0 >= d0
        };
        let (mut n1, mut n0) = (n1, n0);
        if subtract {
            // sub_ddmmss(n1, n0, n1, n0, d1, d0)
            c.cycles += 4;
            n1 = n1.wrapping_sub(d1).wrapping_sub((n0 < low) as u32);
            n0 = low;
            c.g[T2][0] = sx(n1);
            c.g[T4][0] = sx(n0);
        }
        c.cycles += 2;
        if rp != 0 {
            c.cycles += 4;
            moddi3_store_remainder(c, rp, (n1 as u64) << 32 | n0 as u64);
        }
        return;
    }
    c.cycles += 15;
    let b = MODDI3_WORD_BITS - bm;
    let d1 = (d1 << bm) | (d0 >> b);
    let d0 = d0 << bm;
    let n2 = n1 >> b;
    let n1 = (n1 << bm) | (n0 >> b);
    let n0 = n0 << bm;
    c.g[T0][0] = sx(d0);
    c.g[T1][0] = sx(d1);
    c.g[T4][0] = sx(n0);
    let (hi, r) = qrnnd_step(c, n2, n1 >> 16, d1, 11, 3);
    c.g[T3][0] = sx(hi);
    c.cycles += 2;
    let (lo, n1) = qrnnd_step(c, r, n1 & 0xffff, d1, 13, 2);
    c.cycles += 11;
    c.g[A1][0] = sx(n1);
    c.g[T2][0] = sx(n1);
    let q0 = hi << 16 | lo;
    // umul_ppmm(m1, m0, q0, d0); q0 is one too big when m1:m0 > n1:n0.
    abi::multu(c, q0, d0);
    let product = q0 as u64 * d0 as u64;
    let (mut m1, mut m0) = ((product >> 32) as u32, product as u32);
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
    c.g[A2][0] = sx(m0);
    c.g[A3][0] = sx(m1);
    c.cycles += 2;
    let low = n0.wrapping_sub(m0);
    c.g[A0][0] = sx(low);
    if rp != 0 {
        // sub_ddmmss(n1, n0, n1, n0, m1, m0), then shifted back by bm
        c.cycles += 12;
        let high = n1.wrapping_sub(m1).wrapping_sub((n0 < low) as u32);
        c.g[T2][0] = sx(high);
        let rem_low = (high << b) | (low >> bm);
        c.g[A0][0] = (low >> bm) as u64;
        c.g[A1][0] = (rem_low as u64) << 32;
        moddi3_store_remainder(c, rp, ((high >> bm) as u64) << 32 | rem_low as u64);
    }
}

/// `*rp = rr.ll` (t7 = the remainder).
pub(crate) fn moddi3_store_remainder(c: &mut Cpu, rp: u64, rem: u64) {
    c.cycles += 1;
    c.g[T7][0] = rem;
    Addr(rp as u32).sd(c, rem);
}

/// Loads `w` back (v1), negates it in its slot when n was negative (a0 =
/// the borrow, v1 = its high word, t8 = `-w`), returns it in v0 and
/// restores s0.
pub(crate) fn moddi3_return(c: &mut Cpu, frame: Frame, negative: bool) {
    c.cycles += 2;
    let slot = Addr(frame.at(c, MODDI3_SLOT_REMAINDER));
    let w = slot.ld(c);
    c.g[V1][0] = w;
    if negative {
        c.cycles += 21;
        let w = w.wrapping_neg();
        c.g[A0][0] = (w as u32 != 0) as u64;
        c.g[V1][0] = w & !MODDI3_LOW_WORD;
        c.g[T8][0] = w;
        slot.sd(c, w);
    }
    c.cycles += 4;
    c.g[V0][0] = slot.ld(c);
    c.g[S0][0] = Addr(frame.at(c, MODDI3_SLOT_S0)).ld(c);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// __ieee754_log_2 (0x41f3a8): the game's second copy of fdlibm's e_log.c on
// the soft-float double ABI. Same algorithm and calls as `__ieee754_log`
// (0x40db58), but every constant except -inf is built in a1 with lui/ori/dsll
// instead of loaded from the .lit pool, the callee-saved registers are kept
// whole (`sq`), and the two `k != 0` general tails are not merged.

/// fdlibm e_log.c's double constants (bit patterns).
pub(crate) const IEEE_LOG_TWO54: u64 = 0x4350_0000_0000_0000;

pub(crate) const IEEE_LOG_ONE: u64 = 0x3ff0_0000_0000_0000;

pub(crate) const IEEE_LOG_TWO: u64 = 0x4000_0000_0000_0000;

pub(crate) const IEEE_LOG_HALF: u64 = 0x3fe0_0000_0000_0000;

pub(crate) const IEEE_LOG_THIRD: u64 = 0x3fd5_5555_5555_5555;

pub(crate) const IEEE_LOG_LN2_HI: u64 = 0x3fe6_2e42_fee0_0000;

pub(crate) const IEEE_LOG_LN2_LO: u64 = 0x3dea_39ef_3579_3c76;

pub(crate) const IEEE_LOG_LG1: u64 = 0x3fe5_5555_5555_5593;

pub(crate) const IEEE_LOG_LG2: u64 = 0x3fd9_9999_9997_fa04;

pub(crate) const IEEE_LOG_LG3: u64 = 0x3fd2_4924_9422_9359;

pub(crate) const IEEE_LOG_LG4: u64 = 0x3fcc_71c5_1d8e_78af;

pub(crate) const IEEE_LOG_LG5: u64 = 0x3fc7_4664_96cb_03de;

pub(crate) const IEEE_LOG_LG6: u64 = 0x3fc3_9a09_d078_c69f;

pub(crate) const IEEE_LOG_LG7: u64 = 0x3fc2_f112_df3e_5244;

/// The .lit word holding -inf (`-two54 / zero`, folded by the compiler).
pub(crate) const IEEE_LOG_2_NEG_INF: u32 = IEEE_LOG_POOL - 0x5790;

/// Frame size; `ra` is at 0 and the callee-saved registers, all 128 bits, above it.
pub(crate) const IEEE_LOG_2_FRAME: u32 = 0xa0;

pub(crate) const IEEE_LOG_2_SLOT_RA: u32 = 0;

/// s0 is restored on its own, in the delay slot of each early return.
pub(crate) const IEEE_LOG_2_SLOT_S0: u32 = 0x90;

pub(crate) const IEEE_LOG_2_SAVES: [(usize, u32); 9] = [
    (S0, IEEE_LOG_2_SLOT_S0),
    (S1, 0x80),
    (S2, 0x70),
    (S3, 0x60),
    (S4, 0x50),
    (S5, 0x40),
    (S6, 0x30),
    (S7, 0x20),
    (FP, 0x10),
];

/// The high word's mantissa bits; a high word above it (signed) is normal.
pub(crate) const IEEE_LOG_2_HI_MANTISSA: u32 = 0x000f_ffff;

/// The largest finite high word: above it x is inf or NaN.
pub(crate) const IEEE_LOG_2_HI_FINITE_MAX: u32 = 0x7fef_ffff;

pub(crate) const IEEE_LOG_2_HI_ABS: u32 = 0x7fff_ffff;

pub(crate) const IEEE_LOG_2_HI_EXP_LSB: u32 = 0x0010_0000;

pub(crate) const IEEE_LOG_2_HI_ONE: u32 = 0x3ff0_0000;

pub(crate) const IEEE_LOG_2_EXP_BIAS: u32 = 0x3ff;

pub(crate) const IEEE_LOG_2_LO_WORD: u64 = 0xffff_ffff;

/// `0x100000 - 0x6a09c`: carries into the exponent bit when the mantissa is
/// at least sqrt(2)'s, which then halves x and adds one to k.
pub(crate) const IEEE_LOG_2_SQRT2_CARRY: u32 = 0x95f64;

/// `k` after the two54 scaling of a subnormal.
pub(crate) const IEEE_LOG_2_SUBNORMAL_K: i64 = -54;

/// `(hx + 2) & 0xfffff < 3`: |f| < 2^-20.
pub(crate) const IEEE_LOG_2_TINY_SPAN: i64 = 3;

/// `i = hx - 0x6147a; j = 0x6b851 - hx`: `i | j > 0` picks the hfsq form.
pub(crate) const IEEE_LOG_2_SPLIT_LO: u32 = 0x6147a;

pub(crate) const IEEE_LOG_2_SPLIT_HI: u32 = 0x6b851;

/// Where a path of `__ieee754_log_2` joins the shared tail.
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub(crate) enum IeeeLog2Tail {
    /// `a1 = v0`, then `v0 = a0 - a1`.
    SubResult,
    /// `v0 = a0 - a1`.
    Sub,
    /// Return v0 (restore s0 first).
    Ret,
    /// Return v0 (s0 already restored).
    Epilogue,
}

/// `double __ieee754_log(double x)`, the game's second copy (0x41f3a8; x and
/// the result as 64-bit integers in a0 / v0): fdlibm's natural logarithm on
/// the soft-float helpers. log(+-0) = -inf, log(x < 0) = NaN, log(inf/NaN) =
/// x + x; otherwise x = 2^k * (1 + f) with sqrt(2)/2 < 1 + f < sqrt(2), the
/// short series for |f| < 2^-20 and `s = f / (2 + f)` with the Lg1..Lg7
/// polynomial for the rest.
pub fn ieee754_log_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, IEEE_LOG_2_FRAME);
        c.g[A2][0] = c.g[A0][0];
        for (reg, off) in IEEE_LOG_2_SAVES {
            frame.save(c, reg, off);
        }
        frame.save_ra(c, IEEE_LOG_2_SLOT_RA);
        c.g[V0][0] = c.g[A2][0];
        c.g[S3][0] = ((c.g[V0][0] as i64) >> 32) as u64; // hx
        c.g[V1][0] = ((c.g[V0][0] << 32) as i64 >> 32) as u64; // lx
        c.g[V0][0] = IEEE_LOG_2_HI_MANTISSA as u64;
        let normal = (c.g[V0][0] as i64) < (c.g[S3][0] as i64);
        c.g[V0][0] = normal as u64;
        c.g[FP][0] = 0; // k

        let tail = 'body: {
            if !normal {
                if let Some(tail) = ieee_log_2_below_normal(c, frame)? {
                    break 'body tail;
                }
            }

            c.cycles += 5;
            c.g[V0][0] = IEEE_LOG_2_HI_FINITE_MAX as u64;
            let inf_nan = (c.g[V0][0] as i64) < (c.g[S3][0] as i64);
            c.g[V0][0] = inf_nan as u64;
            c.g[S0][0] = 0xf0000; // lui half of the mantissa mask
            if inf_nan {
                // x + x
                c.cycles += 5;
                c.g[A0][0] = c.g[A2][0];
                c.g[A1][0] = c.g[A0][0];
                IEEE_LOG_ADDDF3.call(c, 0x0041_f480)?;
                frame.restore(c, S0, IEEE_LOG_2_SLOT_S0);
                break 'body IeeeLog2Tail::Epilogue;
            }

            if ieee_log_2_normalise(c)? {
                break 'body ieee_log_2_small_f(c, frame)?;
            }
            ieee_log_2_series(c)?;
            ieee_log_2_combine(c)?;
            IeeeLog2Tail::SubResult
        };

        if tail <= IeeeLog2Tail::SubResult {
            c.cycles += 1;
            c.g[A1][0] = c.g[V0][0];
        }
        if tail <= IeeeLog2Tail::Sub {
            c.cycles += 2;
            IEEE_LOG_SUBDF3.call(c, 0x0041_f9a4)?;
        }
        if tail <= IeeeLog2Tail::Ret {
            c.cycles += 1;
            frame.restore(c, S0, IEEE_LOG_2_SLOT_S0);
        }
        c.cycles += 11;
        for &(reg, off) in &IEEE_LOG_2_SAVES[1..] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, IEEE_LOG_2_SLOT_RA);
        frame.close(c);
        Ok(())
    })
}

/// `hx < 0x00100000`: +-0 gives -inf, x < 0 gives `(x - x) / zero`, a
/// subnormal is scaled by two54 (k = -54) and goes on (`None`).
pub(crate) fn ieee_log_2_below_normal(
    c: &mut Cpu,
    frame: Frame,
) -> Result<Option<IeeeLog2Tail>, Unwound> {
    c.cycles += 6;
    c.g[V0][0] = IEEE_LOG_2_HI_ABS as u64;
    c.g[V0][0] &= c.g[S3][0];
    c.g[V0][0] |= c.g[V1][0];
    let zero = c.g[V0][0] == 0;
    c.g[V1][0] = IEEE_LOG_POOL as u64;
    c.cycles += 2;
    if zero {
        c.g[V0][0] = c.ld(IEEE_LOG_2_NEG_INF);
        return Ok(Some(IeeeLog2Tail::Ret));
    }
    let negative = (c.g[S3][0] as i64) < 0;
    c.g[A0][0] = c.g[A2][0];
    c.cycles += 7;
    if negative {
        c.g[A1][0] = c.g[A0][0];
        IEEE_LOG_SUBDF3.call(c, 0x0041_f430)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        IEEE_LOG_DIVDF3.call(c, 0x0041_f43c)?;
        frame.restore(c, S0, IEEE_LOG_2_SLOT_S0);
        return Ok(Some(IeeeLog2Tail::Epilogue));
    }
    c.g[A1][0] = IEEE_LOG_TWO54;
    c.g[FP][0] = IEEE_LOG_2_SUBNORMAL_K as u64;
    IEEE_LOG_MULDF3.call(c, 0x0041_f454)?;
    c.g[A2][0] = c.g[V0][0];
    c.g[S3][0] = ((c.g[V0][0] as i64) >> 32) as u64;
    Ok(None)
}

/// `k += (hx >> 20) - 1023`, x (or x / 2 past sqrt(2)) put in
/// [sqrt(2)/2, sqrt(2)), `f = x - 1` in s5. Returns whether |f| < 2^-20.
pub(crate) fn ieee_log_2_normalise(c: &mut Cpu) -> Result<bool, Unwound> {
    c.cycles += 31;
    c.g[A0][0] = sx((r32(c, S3) as i32 >> 20) as u32);
    c.g[S0][0] |= 0xffff; // s0 = IEEE_LOG_2_HI_MANTISSA
    c.g[V0][0] = IEEE_LOG_2_SQRT2_CARRY as u64;
    c.g[S3][0] &= c.g[S0][0];
    c.g[V0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
    c.g[V1][0] = sx(r32(c, FP).wrapping_sub(IEEE_LOG_2_EXP_BIAS));
    c.g[A1][0] = IEEE_LOG_2_HI_EXP_LSB as u64;
    c.g[FP][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
    c.g[S4][0] = c.g[V0][0] & c.g[A1][0]; // i
    c.g[A0][0] = c.g[A2][0];
    c.g[V0][0] = IEEE_LOG_2_HI_ONE as u64;
    c.g[V1][0] = IEEE_LOG_2_LO_WORD;
    c.g[V0][0] ^= c.g[S4][0];
    c.g[A0][0] &= c.g[V1][0];
    c.g[V0][0] |= c.g[S3][0];
    c.g[V0][0] <<= 32;
    c.g[A2][0] = c.g[A0][0] | c.g[V0][0];
    c.g[V1][0] = sx((r32(c, S4) as i32 >> 20) as u32);
    c.g[A1][0] = IEEE_LOG_ONE;
    c.g[FP][0] = sx(r32(c, FP).wrapping_add(r32(c, V1)));
    c.g[A0][0] = c.g[A2][0];
    IEEE_LOG_SUBDF3.call(c, 0x0041_f4ec)?;
    c.g[S5][0] = c.g[V0][0]; // f
    c.g[V0][0] = sx(r32(c, S3).wrapping_add(2));
    c.g[V0][0] &= c.g[S0][0];
    let tiny = (c.g[V0][0] as i64) < IEEE_LOG_2_TINY_SPAN;
    c.g[V0][0] = tiny as u64;
    c.g[S0][0] = 0;
    Ok(tiny)
}

/// `s0 = dk * ln2_hi`, `v0 = dk * ln2_lo` with `s6 = dk = (double)k`
/// (the two `|f| < 2^-20, k != 0` arms; `rets` of the three calls).
pub(crate) fn ieee_log_2_dk_ln2(c: &mut Cpu, rets: [u32; 3]) -> Result<(), Unwound> {
    c.g[A0][0] = c.g[FP][0];
    IEEE_LOG_FLOATSIDF.call(c, rets[0])?;
    c.g[S6][0] = c.g[V0][0];
    c.g[A1][0] = IEEE_LOG_LN2_HI;
    c.g[A0][0] = c.g[S6][0];
    IEEE_LOG_MULDF3.call(c, rets[1])?;
    c.g[A1][0] = IEEE_LOG_LN2_LO;
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S6][0];
    IEEE_LOG_MULDF3.call(c, rets[2])
}

/// |f| < 2^-20: f == 0 gives 0 (k == 0) or `dk*ln2_hi + dk*ln2_lo`;
/// otherwise `R = f*f*(0.5 - f/3)` and `f - R` (k == 0) or
/// `dk*ln2_hi - ((R - dk*ln2_lo) - f)`.
pub(crate) fn ieee_log_2_small_f(c: &mut Cpu, frame: Frame) -> Result<IeeeLog2Tail, Unwound> {
    c.cycles += 5;
    c.g[A0][0] = c.g[S5][0];
    c.g[A1][0] = c.g[S0][0]; // 0.0
    IEEE_LOG_CMPDF2.call(c, 0x0041_f510)?;
    let f_nonzero = c.g[V0][0] != 0;
    c.g[A0][0] = c.g[S5][0];
    if !f_nonzero {
        c.cycles += 2;
        let k_zero = c.g[FP][0] == 0;
        c.g[V0][0] = c.g[S0][0];
        if k_zero {
            return Ok(IeeeLog2Tail::Ret); // log(1) = 0
        }
        c.cycles += 26;
        ieee_log_2_dk_ln2(c, [0x0041_f528, 0x0041_f54c, 0x0041_f574])?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        IEEE_LOG_ADDDF3.call(c, 0x0041_f580)?;
        frame.restore(c, S0, IEEE_LOG_2_SLOT_S0);
        return Ok(IeeeLog2Tail::Epilogue);
    }

    c.cycles += 21;
    c.g[A1][0] = c.g[S5][0];
    IEEE_LOG_MULDF3.call(c, 0x0041_f590)?;
    c.g[A1][0] = IEEE_LOG_THIRD;
    c.g[S0][0] = c.g[V0][0]; // f*f
    c.g[A0][0] = c.g[S5][0];
    IEEE_LOG_MULDF3.call(c, 0x0041_f5b8)?;
    c.g[A0][0] = IEEE_LOG_HALF;
    c.g[A1][0] = c.g[V0][0];
    IEEE_LOG_SUBDF3.call(c, 0x0041_f5c8)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[V0][0];
    IEEE_LOG_MULDF3.call(c, 0x0041_f5d4)?;
    let k_nonzero = c.g[FP][0] != 0;
    c.g[S2][0] = c.g[V0][0]; // R
    if !k_nonzero {
        c.cycles += 3;
        c.g[A0][0] = c.g[S5][0];
        c.g[A1][0] = c.g[S2][0];
        return Ok(IeeeLog2Tail::Sub);
    }
    c.cycles += 29;
    ieee_log_2_dk_ln2(c, [0x0041_f5f0, 0x0041_f614, 0x0041_f63c])?;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[V0][0];
    IEEE_LOG_SUBDF3.call(c, 0x0041_f648)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S5][0];
    IEEE_LOG_SUBDF3.call(c, 0x0041_f654)?;
    c.g[A0][0] = c.g[S0][0];
    Ok(IeeeLog2Tail::SubResult)
}

/// `v0 = w * v0` (w in s1).
pub(crate) fn ieee_log_2_times_w(c: &mut Cpu, ret: u32) -> Result<(), Unwound> {
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S1][0];
    IEEE_LOG_MULDF3.call(c, ret)
}

/// `v0 = v0 + coefficient`.
pub(crate) fn ieee_log_2_plus(c: &mut Cpu, coefficient: u64, ret: u32) -> Result<(), Unwound> {
    c.g[A1][0] = coefficient;
    c.g[A0][0] = c.g[V0][0];
    IEEE_LOG_ADDDF3.call(c, ret)
}

/// `s7 = s = f / (2 + f)`, `s6 = dk`, `s2 = z = s*s`, `s1 = w = z*z`,
/// `s4 = i | j`, then `s2 = R = t2 + t1` with
/// `t1 = w*(Lg2 + w*(Lg4 + w*Lg6))`, `t2 = z*(Lg1 + w*(Lg3 + w*(Lg5 + w*Lg7)))`.
pub(crate) fn ieee_log_2_series(c: &mut Cpu) -> Result<(), Unwound> {
    c.cycles += 110;
    c.g[A1][0] = IEEE_LOG_TWO;
    c.g[A0][0] = c.g[S5][0];
    IEEE_LOG_ADDDF3.call(c, 0x0041_f66c)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S5][0];
    IEEE_LOG_DIVDF3.call(c, 0x0041_f678)?;
    c.g[S7][0] = c.g[V0][0]; // s
    c.g[A0][0] = c.g[FP][0];
    IEEE_LOG_FLOATSIDF.call(c, 0x0041_f684)?;
    c.g[A0][0] = c.g[S7][0];
    c.g[A1][0] = c.g[S7][0];
    c.g[S6][0] = c.g[V0][0]; // dk
    IEEE_LOG_MULDF3.call(c, 0x0041_f694)?;
    c.g[V1][0] = sx(IEEE_LOG_2_SPLIT_LO.wrapping_neg());
    c.g[S2][0] = c.g[V0][0]; // z
    c.g[A0][0] = c.g[S2][0];
    c.g[S4][0] = sx(r32(c, S3).wrapping_add(r32(c, V1))); // i
    c.g[A1][0] = c.g[S2][0];
    IEEE_LOG_MULDF3.call(c, 0x0041_f6b0)?;
    c.g[S1][0] = c.g[V0][0]; // w
    c.g[S0][0] = sx(IEEE_LOG_2_SPLIT_HI.wrapping_sub(r32(c, S3))); // j

    // t1
    c.g[A1][0] = IEEE_LOG_LG6;
    c.g[A0][0] = c.g[S1][0];
    IEEE_LOG_MULDF3.call(c, 0x0041_f6e4)?;
    c.g[S4][0] |= c.g[S0][0];
    ieee_log_2_plus(c, IEEE_LOG_LG4, 0x0041_f70c)?;
    ieee_log_2_times_w(c, 0x0041_f718)?;
    ieee_log_2_plus(c, IEEE_LOG_LG2, 0x0041_f73c)?;
    ieee_log_2_times_w(c, 0x0041_f748)?;
    c.g[S0][0] = c.g[V0][0];

    // t2
    c.g[A1][0] = IEEE_LOG_LG7;
    c.g[A0][0] = c.g[S1][0];
    IEEE_LOG_MULDF3.call(c, 0x0041_f770)?;
    ieee_log_2_plus(c, IEEE_LOG_LG5, 0x0041_f794)?;
    ieee_log_2_times_w(c, 0x0041_f7a0)?;
    ieee_log_2_plus(c, IEEE_LOG_LG3, 0x0041_f7c4)?;
    ieee_log_2_times_w(c, 0x0041_f7d0)?;
    ieee_log_2_plus(c, IEEE_LOG_LG1, 0x0041_f7f4)?;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[V0][0];
    IEEE_LOG_MULDF3.call(c, 0x0041_f800)?;

    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S0][0];
    IEEE_LOG_ADDDF3.call(c, 0x0041_f80c)
}

/// The result from R (v0) for |f| >= 2^-20, left as `a0 - v0` for the tail:
/// i > 0 with `hfsq = 0.5*f*f`: `f - (hfsq - s*(hfsq + R))` or
/// `dk*ln2_hi - ((hfsq - (s*(hfsq + R) + dk*ln2_lo)) - f)`; else
/// `f - s*(f - R)` or `dk*ln2_hi - ((s*(f - R) - dk*ln2_lo) - f)`.
pub(crate) fn ieee_log_2_combine(c: &mut Cpu) -> Result<(), Unwound> {
    let hfsq_form = (c.g[S4][0] as i64) > 0;
    c.g[S2][0] = c.g[V0][0]; // R
    if hfsq_form {
        c.cycles += 9;
        c.g[A1][0] = IEEE_LOG_HALF;
        c.g[A0][0] = c.g[S5][0];
        IEEE_LOG_MULDF3.call(c, 0x0041_f824)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S5][0];
        IEEE_LOG_MULDF3.call(c, 0x0041_f830)?;
        let k_nonzero = c.g[FP][0] != 0;
        c.g[S3][0] = c.g[V0][0]; // hfsq
        if !k_nonzero {
            c.cycles += 11;
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.g[S3][0];
            IEEE_LOG_ADDDF3.call(c, 0x0041_f844)?;
            c.g[A0][0] = c.g[S7][0];
            c.g[A1][0] = c.g[V0][0];
            IEEE_LOG_MULDF3.call(c, 0x0041_f850)?;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[V0][0];
            IEEE_LOG_SUBDF3.call(c, 0x0041_f85c)?;
            c.g[A0][0] = c.g[S5][0];
            return Ok(());
        }
        c.cycles += 36;
        c.g[A1][0] = IEEE_LOG_LN2_HI;
        c.g[A0][0] = c.g[S6][0];
        IEEE_LOG_MULDF3.call(c, 0x0041_f884)?;
        c.g[S1][0] = c.g[V0][0]; // dk*ln2_hi
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S3][0];
        IEEE_LOG_ADDDF3.call(c, 0x0041_f894)?;
        c.g[A0][0] = c.g[S7][0];
        c.g[A1][0] = c.g[V0][0];
        IEEE_LOG_MULDF3.call(c, 0x0041_f8a0)?;
        c.g[A1][0] = IEEE_LOG_LN2_LO;
        c.g[S0][0] = c.g[V0][0]; // s*(hfsq + R)
        c.g[A0][0] = c.g[S6][0];
        IEEE_LOG_MULDF3.call(c, 0x0041_f8c8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        IEEE_LOG_ADDDF3.call(c, 0x0041_f8d4)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[V0][0];
        IEEE_LOG_SUBDF3.call(c, 0x0041_f8e0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S5][0];
        IEEE_LOG_SUBDF3.call(c, 0x0041_f8ec)?;
        c.g[A0][0] = c.g[S1][0];
        return Ok(());
    }

    c.cycles += 2;
    let k_zero = c.g[FP][0] == 0;
    c.g[A1][0] = c.g[S2][0];
    if k_zero {
        c.cycles += 6;
        c.g[A0][0] = c.g[S5][0];
        IEEE_LOG_SUBDF3.call(c, 0x0041_f988)?;
        c.g[A0][0] = c.g[S7][0];
        c.g[A1][0] = c.g[V0][0];
        IEEE_LOG_MULDF3.call(c, 0x0041_f994)?;
        c.g[A0][0] = c.g[S5][0];
        return Ok(());
    }
    c.cycles += 33;
    c.g[A1][0] = IEEE_LOG_LN2_HI;
    c.g[A0][0] = c.g[S6][0];
    IEEE_LOG_MULDF3.call(c, 0x0041_f91c)?;
    c.g[S1][0] = c.g[V0][0]; // dk*ln2_hi
    c.g[A1][0] = c.g[S2][0];
    c.g[A0][0] = c.g[S5][0];
    IEEE_LOG_SUBDF3.call(c, 0x0041_f92c)?;
    c.g[A0][0] = c.g[S7][0];
    c.g[A1][0] = c.g[V0][0];
    IEEE_LOG_MULDF3.call(c, 0x0041_f938)?;
    c.g[A1][0] = IEEE_LOG_LN2_LO;
    c.g[S0][0] = c.g[V0][0]; // s*(f - R)
    c.g[A0][0] = c.g[S6][0];
    IEEE_LOG_MULDF3.call(c, 0x0041_f960)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[V0][0];
    IEEE_LOG_SUBDF3.call(c, 0x0041_f96c)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S5][0];
    IEEE_LOG_SUBDF3.call(c, 0x0041_f978)?;
    c.g[A0][0] = c.g[S1][0];
    Ok(())
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

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use ssx3_ee as ee;

    pub(crate) const SEEDS: u32 = 200;

    /// Working base for fixtures: `DATA` with a nonzero low byte, so
    /// off-by-N addressing slips show as different data.
    /// Same 16-alignment and LOG clearance as `DATA` (`+0x10`).
    pub(crate) const DB: u32 = DATA + 0x10;

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

    use crate::difftest::elf_image;

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

    use crate::difftest::events::driven;

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

    pub(crate) use crate::game::object::debouncenode::crowd_man3d_save_state;

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

    /// __ieee754_log: +-0, negative, subnormal, inf/NaN, exact powers of
    /// two (k = 0 and not), |f| < 2^-20 (k = 0 and not) and general
    /// normal inputs on both sides of the `i > 0` split; real soft-float
    /// helpers, the constant pool from the ELF image.
    #[test]
    fn ieee754_log_matches() {
        check(
            ee::s15::f_0040db58__generated,
            ieee754_log,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x40_db58, 0);
                crate::difftest::elf_image(c);
                c.pc = 0x0040_db58;
                let lo = r.next_u32() as u64;
                let sign = ((r.next_u32() & 1) as u64) << 63;
                let exp =
                    |r: &mut Rng, span: u32| (0x3ff + r.next_u32() % (2 * span + 1) - span) as u64;
                let x: u64 = match seed % 10 {
                    0 => sign,
                    1 => 1 << 63 | (r.next_u32() as u64) << 32 | lo,
                    2 => ((r.next_u32() % 0x10_0000) as u64) << 32 | lo,
                    3 => {
                        let m = if r.next_u32() % 2 == 0 { 0 } else { lo };
                        sign | 0x7ff0_0000_0000_0000 | m
                    }
                    4 => exp(&mut r, 3) << 52,
                    5 => {
                        let hm = [0xf_fffeu64, 0xf_ffff, 0][(r.next_u32() % 3) as usize];
                        let low = if r.next_u32() % 3 == 0 { 0 } else { lo };
                        exp(&mut r, 3) << 52 | hm << 32 | low
                    }
                    6 | 7 => exp(&mut r, 2) << 52 | ((r.next_u32() % 0x10_0000) as u64) << 32 | lo,
                    _ => {
                        let e = 1 + (r.next_u32() % 0x7fe) as u64;
                        e << 52 | ((r.next_u32() % 0x10_0000) as u64) << 32 | lo
                    }
                };
                c.g[A0][0] = x;
            },
        );
    }
    use crate::difftest::check_paths_lohi;
    use crate::difftest::fakes::LOG_UNWIND_AT;

    #[test]
    fn empty_matches() {
        plain(ee::s15::f_00410408__generated, empty, 0x0041_0408);
    }

    #[test]
    fn unwinding_cleanup_matches() {
        plain(
            ee::s15::f_00410c10__generated,
            unwinding_cleanup,
            0x0041_0c10,
        );
    }

    #[test]
    fn get_eh_table_version_matches() {
        check(
            ee::s15::f_00410848__generated,
            get_eh_table_version,
            SEEDS_7,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0041_0848, 0);
                let p = DB + 0x200 + (r.next_u32() & 0xff0);
                c.bus.write32(p, r.next_u32());
                c.bus.write32(p + 4, r.next_u32());
                c.g[A0][0] = p as u64;
            },
        );
    }

    #[test]
    fn get_eh_table_language_matches() {
        check(
            ee::s15::f_00410850__generated,
            get_eh_table_language,
            SEEDS_7,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0041_0850, 0);
                let p = DB + 0x200 + (r.next_u32() & 0xff0);
                c.bus.write32(p, r.next_u32());
                c.bus.write32(p + 4, r.next_u32());
                c.g[A0][0] = p as u64;
            },
        );
    }

    #[test]
    fn pure_virtual_matches() {
        plain(ee::s15::f_00411530__generated, pure_virtual, 0x0041_1530);
    }

    crate::difftest::events::driven!(fmodf_gen_driven, ee::s15::f_0040e300__generated);

    crate::difftest::events::driven!(fmodf_hand_driven, ieee754_fmodf);

    /// `__ieee754_fmodf` operands the digit table never pairs: subnormal
    /// results (a subnormal y with a nonzero remainder, 2^149 mod 3 = 2 and
    /// mod 7 = 4, rebuild through the 0x40e538 shift-down arm), the largest
    /// subnormal as x and as y, x at exponent -126 against a normal and a
    /// subnormal y, remainders of exactly 0x800000 and 0x7fffff at the
    /// 0x40e4dc level test, and a remainder that renormalizes to exactly
    /// exponent -126.
    const FMODF_EDGES: [(u32, u32); 9] = [
        (0x3f80_0000, 0x0000_0003),
        (0xbf80_0000, 0x0000_0007),
        (0x007f_ffff, 0x0010_0000),
        (0x3f80_0000, 0x007f_ffff),
        (0x00c0_0000, 0x0090_0000),
        (0x00c0_0000, 0x0000_0003),
        (0x40a0_0000, 0x4040_0000),
        (0x3fff_ffff, 0x3f80_0000),
        (0x0160_0000, 0x0100_0000),
    ];

    #[test]
    fn ieee754_fmodf_matches() {
        /// Edge-case float bits steered by seed digit. Entries 12-15 are
        /// not powers of two (and one denormal), so some (x, y) pairs
        /// take the divide loop past 0x40e4c8 with a nonzero remainder
        /// (every power-of-two y divides evenly to the exact-zero exit).
        /// Denormals only scale the round count by the exponent
        /// difference (at most ~280 iterations): no hang on either side.
        fn bits(seed: u32, k: u32) -> u32 {
            match (seed / k) % 16 {
                0 => 0x00000000,
                1 => 0x80000000,
                2 => 0x3f800000,
                3 => 0xbf800000,
                4 => 0x7f800000,
                5 => 0xff800000,
                6 => 0x7fc00000,
                7 => 0x33800000,
                8 => 0x00800000,
                9 => 0x7f7fffff,
                10 => 0x4b000000,
                11 => 0xcb000000,
                12 => 0x40400000,
                13 => 0x40b00000,
                14 => 0x3f000001,
                _ => 0x00100000,
            }
        }
        check(
            ee::s15::f_0040e300__generated,
            ieee754_fmodf,
            SEEDS_7 + 9,
            &|c, seed| {
                let _ = machine(c, seed, 0x0040_e300, 0);
                c.f[12] = bits(seed, 1);
                c.f[13] = bits(seed, 4);
                if seed >= SEEDS_7 {
                    (c.f[12], c.f[13]) = FMODF_EDGES[(seed - SEEDS_7) as usize];
                }
                // Dispatcher shape: enter at the entry.
                c.pc = 0x0040_e300;
            },
        );
        // Events at every loop head and the exact-zero join: unwinds
        // resume at the head they interrupted. All three operand shapes
        // run loops (denormal x, denormal y, long divide), so every head
        // is reachable and the schedule spreads unwinds across them.
        check_events(fmodf_gen_driven, fmodf_hand_driven, SEEDS_7, &|c, seed| {
            let _ = machine(c, seed, 0x0040_e300, 0);
            // Every arm runs loops: denormal operands hit the align
            // loops, the long divide hits 0x4a8, the exact-zero divide
            // and equal magnitudes hit the 0x38c join, and the tiny
            // remainder hits the rebuild loop.
            let mut next = 0;
            match seed % 6 {
                0 => {
                    c.f[12] = 0x0010_0000;
                    c.f[13] = 0x0000_0001;
                }
                1 => {
                    c.f[12] = 5.5f32.to_bits();
                    c.f[13] = 0x0010_0000;
                }
                2 => {
                    // Exact zero through one divide round: bracket the
                    // join edge so an unwind lands on it.
                    c.f[12] = 6.0f32.to_bits();
                    c.f[13] = 3.0f32.to_bits();
                    next = 60 + (seed % 3) * 2;
                }
                3 => {
                    // Exact zero through a long divide: chained unwinds
                    // persist to the join edge.
                    c.f[12] = 0x4b00_0000;
                    c.f[13] = 2.0f32.to_bits();
                }
                4 => {
                    // Tiny remainder, no divide rounds: the first event
                    // check is the rebuild head itself.
                    c.f[12] = 0x3f80_0001;
                    c.f[13] = 1.0f32.to_bits();
                    next = 10;
                }
                _ => {
                    c.f[12] = ((1000 + seed * 37) as f32).to_bits();
                    c.f[13] = 3.0f32.to_bits();
                }
            }
            let mut r = Rng(0x0040_e300 ^ seed as u64);
            let mode = schedule(c, &mut r, seed, 0x0040_e300);
            c.next_event = c.cycles
                + if next != 0 {
                    next as u64
                } else {
                    10 + (r.next_u32() % 200) as u64
                };
            mode
        });
    }

    #[test]
    fn eh_rtime_match_matches() {
        // The zero-match value has the same cycles as a match, so no
        // `check_paths` total can show it; the zero seeds compare it.
        check(
            ee::s15::f_00410800__generated,
            eh_rtime_match,
            SEEDS_7,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0041_0800, 0);
                // The context getter (miss hook) returns our block;
                // +8 points at the matcher cell.
                let ctx = DB + 0x400;
                c.bus.write32(LOG_RESULTS, ctx);
                let inner = DB + 0x500 + (r.next_u32() & 0xf0);
                c.bus.write32(ctx + 8, inner);
                if seed % 4 == 3 {
                    // Zero match result: point the matcher cell at the real
                    // `__get_eh_table_language` (0x410850), whose halfword
                    // load at A0 + 4 reads the explicit zero below, so it
                    // returns 0 and `(0 < V0)` is false. A forced zero
                    // result is impossible (0 means "not forced").
                    c.bus.write32(inner, 0x0041_0850);
                    c.bus.write32(DB + 0x704, 0);
                } else {
                    match seed % 3 {
                        // No catcher at all.
                        0 => c.bus.write32(inner, 0),
                        _ => {
                            c.bus.write32(inner, DB + 0x600);
                            // Matcher result: match (nonzero) or not.
                            c.bus
                                .write32(LOG_RESULTS + 4, if seed % 3 == 1 { 1 } else { 0 });
                        }
                    }
                }
                c.g[A0][0] = (DB + 0x700) as u64;
            },
        );
    }

    /// Exception context for the sjlj tests: getter result (`LOG_RESULTS`)
    /// points at `ctx`; `*(ctx + 4)` is the handler head.
    pub(crate) fn sjlj_ctx(c: &mut Cpu, seed: u32) {
        let ctx = DB + 0x100;
        let h = DB + 0x200;
        c.bus.write32(LOG_RESULTS, ctx);
        c.bus.write32(ctx + 4, h);
        c.bus.write32(ctx + 8, 0);
        // The search tail reads through `a0`; keep it mapped.
        c.g[A0][0] = (DB + 0x900) as u64;
        // `fp` comes from `a2`.
        c.g[A2][0] = (DB + 0x800) as u64;
        if seed % 3 == 0 {
            // No handler: the search path with an empty context.
            c.bus.write32(h + 4, 0);
            return;
        }
        // One-link chain: the link at `M` carries data and a null
        // matcher (miss hook), whose forced result ends the walk.
        let m = DB + 0x300;
        c.bus.write32(h + 4, m);
        c.bus.write32(m, 0);
        c.bus.write32(m + 8, 0x2222);
        c.bus.write32(m + 4, 0);
        c.bus.write32(LOG_RESULTS + 4, 0);
    }

    #[test]
    fn sjthrow_matches() {
        check(ee::s15::f_00410570__generated, sjthrow, 32, &|c, seed| {
            let _ = machine(c, seed, 0x0041_0570, 0);
            sjlj_ctx(c, seed);
            if seed % 3 == 0 {
                // Live context: steer past the empty-context probe to
                // the landing-context check below (else it never runs).
                c.bus.write32(DB + 0x108, 0x2222);
            }
        });
    }

    #[test]
    fn sjpopnthrow_matches() {
        check(
            ee::s15::f_004106e8__generated,
            sjpopnthrow,
            32,
            &|c, seed| {
                let _ = machine(c, seed, 0x0041_06e8, 0);
                sjlj_ctx(c, seed);
            },
        );
    }

    #[test]
    fn throw_matches() {
        check(ee::s15::f_00410f48__generated, throw_, 32, &|c, seed| {
            let _ = machine(c, seed, 0x0041_0f48, 0);
            throw_save_area(c, seed);
            c.bus.write32(LOG_RESULTS, DB + 0x100);
            c.bus.write32(DB + 0x104, DB + 0x200);
            c.bus.write32(DB + 0x204, 0);
            c.bus.write32(DB + 0x208, 0);
            c.g[A2][0] = (DB + 0x800) as u64;
        });
    }

    #[test]
    fn rethrow_matches() {
        check(ee::s15::f_00411128__generated, rethrow, 32, &|c, seed| {
            let _ = machine(c, seed, 0x0041_1128, 0);
            throw_save_area(c, seed);
            c.bus.write32(LOG_RESULTS, DB + 0x100);
            c.bus.write32(DB + 0x104, DB + 0x200);
            c.bus.write32(DB + 0x204, 0);
            c.bus.write32(DB + 0x208, 0);
            c.g[A0][0] = (DB + 0x600) as u64;
            c.g[A2][0] = (DB + 0x800) as u64;
        });
    }

    pub(crate) const SEEDS_3: u32 = 64;

    /// Two edge-case float bits in `f12`/`f13`, GPRs scrambled.
    pub(crate) fn float_args(gen: GuestFn, hand: GuestFn, salt: u64) {
        check(gen, hand, SEEDS_3, &|c, seed| {
            let mut r = machine(c, seed, salt, 0);
            c.f[12] = r.edge_f32();
            c.f[13] = r.edge_f32();
        });
    }

    #[test]
    fn cplus_type_matcher_matches() {
        check(
            ee::s15::f_00412928__generated,
            cplus_type_matcher,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0041_2928, 0);
                let s0 = DB + 0x400;
                // Descriptor words the direct and indirect paths read.
                c.bus.write32(s0 + 8, r.next_u32());
                c.bus.write32(s0 + 0xc, r.next_u32());
                c.bus.write32(s0 + 0x28, r.next_u32());
                c.g[A0][0] = s0 as u64;
                match seed % 4 {
                    // Null table, sentinel matcher: direct descriptor read.
                    0 => {
                        c.g[A2][0] = 0;
                        c.g[A1][0] = 0xffff_ffff_ffff_ffff;
                    }
                    // Null table, indirect matcher (miss hook) + the real
                    // RTTI re-match on the fake's result.
                    1 => {
                        c.g[A2][0] = 0;
                        c.g[A1][0] = FAKE as u64;
                    }
                    // Class tag 4: the descriptor path with a table.
                    2 => {
                        c.bus.write16(DB + 0x504, 4);
                        c.g[A2][0] = (DB + 0x500) as u64;
                        c.g[A1][0] = 0xffff_ffff_ffff_ffff;
                    }
                    // Any other tag: the early 0.
                    _ => {
                        c.bus.write16(DB + 0x504, 0x1234);
                        c.g[A2][0] = (DB + 0x500) as u64;
                        c.g[A1][0] = 0;
                    }
                }
            },
        );
    }

    #[test]
    fn cp_push_exception_matches() {
        // Allocator, exception-info getter and chain splice are all real.
        plain(
            ee::s15::f_00412998__generated,
            cp_push_exception,
            0x0041_2998,
        );
    }

    /// Catch-handler record: use count steered (2: early return, 1: the
    /// unlink walk), destructor slot steered (absent or the miss-hook
    /// fake), the object/type words filled.
    pub(crate) fn pop_record(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let s0 = DB + 0x400;
        if seed % 2 == 0 {
            // Head record: steer the context getter's fake at our block
            // whose +8 points at s0, so the pop finds its own record and
            // takes the cleanup-pending check below (V0 both ways).
            // (Odd seeds keep the refcount early-return path.)
            c.bus.write32(DB + 0x608, s0);
            c.bus.write32(LOG_RESULTS, DB + 0x600);
        }
        c.bus.write64(s0 + 0x20, (seed % 2 + 1) as u64);
        c.bus
            .write32(s0 + 0x14, if seed / 2 % 2 == 0 { 0 } else { 1 });
        c.bus
            .write32(s0 + 0x10, if seed / 4 % 2 == 0 { 0 } else { FAKE });
        c.bus.write32(s0 + 0xc, r.next_u32());
        c.bus.write32(s0 + 0x8, r.next_u32());
        c.bus.write32(s0 + 0x18, r.next_u32());
        c.bus.write32(s0 + 0x28, r.next_u32());
        c.g[A0][0] = s0 as u64;
    }

    /// Two-link catch chain ending at the record: the walk mismatches
    /// once (taking the 0x12a80 back edge) then matches and splices.
    /// The info getter's fake is forced at our block whose +8 points at
    /// the head link.
    pub(crate) fn pop_chain_record(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let s0 = DB + 0x400;
        c.bus.write64(s0 + 0x20, 1);
        c.bus.write32(s0 + 0x14, 0);
        c.bus.write32(s0 + 0x10, 0);
        c.bus.write32(s0 + 0xc, r.next_u32());
        c.bus.write32(s0 + 0x8, r.next_u32());
        c.bus.write32(s0 + 0x18, r.next_u32());
        c.bus.write32(s0 + 0x28, r.next_u32());
        c.bus.write32(DB + 0x708, DB + 0x740);
        c.bus.write32(DB + 0x740 + 0x18, DB + 0x780);
        c.bus.write32(DB + 0x780 + 0x18, s0);
        c.bus.write32(LOG_RESULTS, DB + 0x700);
        c.g[A0][0] = s0 as u64;
        let _ = seed;
    }

    crate::difftest::events::driven!(pop_gen_driven, ee::s15::f_00412a28__generated);

    crate::difftest::events::driven!(pop_hand_driven, cp_pop_exception);

    #[test]
    fn cp_pop_exception_matches() {
        // Any terminate path would recurse forever with a returning
        // slot fake (see `terminate_matches`); unwind at the second
        // fake call instead. Both sides unwind identically.
        // Dispatcher shape: enter at the entry.
        crate::difftest::check_paths(
            ee::s15::f_00412a28__generated,
            cp_pop_exception,
            SEEDS_3,
            // Walking the whole chain takes 114 or 122 cycles (the destroy
            // tail varies with the record words).
            &[114, 122],
            &|c, seed| {
                let mut r = machine(c, seed, 0x0041_2a28, 0);
                c.bus.write32(LOG_UNWIND_AT, 1);
                c.pc = 0x0041_2a28;
                if seed % 4 == 3 {
                    // Two-link chain: the walk mismatches once (taking the
                    // 0x12a80 back edge) then matches and splices. The
                    // plain setup never walks past the head.
                    pop_chain_record(c, &mut r, seed);
                } else {
                    pop_record(c, &mut r, seed);
                }
            },
        );
    }

    #[test]
    fn cp_pop_exception_chain_events() {
        // Two-link chain: one walk iteration takes the 0x12a80 back
        // edge, an event unwind there resumes at the new entry, and the
        // next link matches. The resume count per head is in the log.
        check_events(pop_gen_driven, pop_hand_driven, SEEDS_3, &|c, seed| {
            let mut r = machine(c, seed, 0x0041_2a28, 0);
            pop_chain_record(c, &mut r, seed);
            let mut r = Rng(0x0041_2a28 ^ seed as u64);
            let mode = schedule(c, &mut r, seed, 0x0041_2a28);
            c.next_event = c.cycles + 10 + (r.next_u32() % 100) as u64;
            mode
        });
    }

    #[test]
    fn check_eh_spec_matches() {
        // Terminate paths would recurse forever with a returning slot
        // fake (see `terminate_matches`); unwind at the second fake
        // call instead. Both sides unwind identically.
        // Dispatcher shape: entries across seeds cover every arm (entry
        // plus each otherwise-dead resume arm); mid-entry arms run with
        // clamped counters (S3 = 2, S0 = 1..2) so the spec walks stay
        // bounded. Both sides run the same arms identically.
        crate::difftest::check_paths(
            ee::s15::f_00412b20__generated,
            check_eh_spec,
            SEEDS_3,
            // All-real-entry probe run with counts 1-2 takes 154 or 247 cycles
            // (one or two allowed-list walks); count 0 and mid-entries
            // differ.
            &[154, 247],
            &|c, seed| {
                let mut r = machine(c, seed, 0x0041_2b20, 0);
                c.bus.write32(LOG_UNWIND_AT, 1);
                const ENTRIES: [u32; 24] = [
                    0x0041_2b20,
                    0x0041_2b54,
                    0x0041_2b60,
                    0x0041_2b78,
                    0x0041_2b88,
                    0x0041_2b98,
                    0x0041_2ba0,
                    0x0041_2bac,
                    0x0041_2bb4,
                    0x0041_2bbc,
                    0x0041_2bc8,
                    0x0041_2be0,
                    0x0041_2bf0,
                    0x0041_2bf8,
                    0x0041_2c08,
                    0x0041_2c10,
                    0x0041_2c2c,
                    0x0041_2c44,
                    0x0041_2c4c,
                    0x0041_2c54,
                    0x0041_2c88,
                    0x0041_2c98,
                    0x0041_2ca0,
                    0x0041_2ca8,
                ];
                let entry = ENTRIES[seed as usize % ENTRIES.len()];
                c.pc = entry;
                if entry != 0x0041_2b20 {
                    c.g[S3][0] = 2;
                    c.g[S0][0] = 1 + (seed % 2) as u64;
                }
                let list = DB + 0x500;
                fill(c, &mut r, list, 8);
                // Spec count 0 (the unexpected path at once), 1 or 2 (the
                // allowed-list walk over filled words, then unexpected).
                // `seed / 24` for the real entry, so it walks too (`seed % 3`
                // there would always be 0).
                c.g[A0][0] = if entry == 0x0041_2b20 {
                    (seed / 24 % 3) as u64
                } else {
                    (seed % 3) as u64
                };
                c.g[A1][0] = list as u64;
            },
        );
    }

    #[test]
    fn throw_bad_cast_matches() {
        // The setup trampoline throws through the real `__throw`; unwind
        // at the second fake call so a terminate path can't recurse
        // forever on a returning slot fake. Both sides unwind identically.
        check(
            ee::s15::f_00412cd8__generated,
            throw_bad_cast,
            SEEDS_3,
            &|c, seed| {
                machine(c, seed, 0x0041_2cd8, 0);
                c.bus.write32(LOG_UNWIND_AT, 1);
            },
        );
    }

    #[test]
    fn throw_bad_typeid_matches() {
        check(
            ee::s15::f_00412d58__generated,
            throw_bad_typeid,
            SEEDS_3,
            &|c, seed| {
                machine(c, seed, 0x0041_2d58, 0);
                c.bus.write32(LOG_UNWIND_AT, 1);
            },
        );
    }

    /// Correlated second operand for add/compare differential tests: seeds
    /// 64 and up get a's class and an absolute exponent pair from a small
    /// table (equal, close and large diffs in both directions), so the
    /// alignment/add/compare cores run instead of only the class dispatch.
    /// Absolute pairs (not deltas off random bases) keep every exponent in
    /// the valid normal range by construction. On the equal-exp pairs the
    /// fractions are matched too (equal on the even half of seed/2,
    /// off-by-one on the odd half), so the fraction compares see equality
    /// and strict order. Seeds 0-63
    /// keep the independent pairing exactly as before (all 36 class pairs
    /// still occur there), so the earlier coverage still holds.
    pub(crate) fn correlate_b(c: &mut Cpu, seed: u32) {
        if seed >= 64 {
            const PAIRS: [(i32, i32); 15] = [
                (0, 0),
                (5, 5),
                (5, 4),
                (5, 6),
                (5, -26),
                (5, 36),
                (95, 127),
                (95, 63),
                (-95, -127),
                (-95, -63),
                (0, 32),
                (0, -32),
                (127, -126),
                (-126, 127),
                (100, 100),
            ];
            let (ea, eb) = PAIRS[(seed / 2 % 15) as usize];
            let ca = c.bus.read32(DB);
            let fa = c.bus.read32(DB + 0xc);
            c.bus.write32(DB + 0x20, ca);
            c.bus.write32(DB + 8, ea as u32);
            c.bus.write32(DB + 0x28, eb as u32);
            if ea == eb {
                // Equal fracs (even half) for the equality cases, off by
                // one (odd half) for strict order. Normal-class seeds are
                // always odd (class 3/5 ⟺ seed odd), and (seed/2) splits
                // each pair-index evenly, so both variants land on
                // core-reaching seeds whichever digit is normal.
                c.bus.write32(DB + 0x2c, fa.wrapping_add((seed / 2) & 1));
            }
        }
    }

    /// Unpacked single at the working base: class, sign, unbiased exponent and
    /// fraction steered by independent seed digits (NaNs, zero, infinity,
    /// denormals, rounding edges, overflow to infinity). Every digit cycles
    /// inside the 64-seed window, so fraction-nonzero and high-exponent
    /// cases actually occur (the 412fcc subnormal arm needs frac != 0).
    pub(crate) fn unpacked_f(c: &mut Cpu, r: &mut Rng, seed: u32, at: u32) {
        c.bus.write32(at, (seed % 6) as u32);
        c.bus.write32(at + 4, (seed / 6 % 2) as u32);
        // Exp/frac digits decorrelated from class: the pack shift block
        // needs (class >= 2, nonzero frac, exp -200) together (seed 32)
        // and the pack epilogue needs (class < 2, random frac) (seed 18);
        // plain seed/2, seed/3 digits can never express either pair.
        c.bus.write32(
            at + 8,
            match (seed / 7 + 5) % 9 {
                0 => 0xffff_ff38,
                1 => 0xffff_ff82,
                2 => 0xffff_ff83,
                3 => 0xffff_ff9c,
                4 => 0,
                5 => 100,
                6 => 127,
                7 => 128,
                _ => 200,
            },
        );
        c.bus.write32(
            at + 0xc,
            match seed / 5 % 4 {
                0 => 0,
                1 => 0x7f,
                2 => 0x80ff_ff,
                _ => r.next_u32(),
            },
        );
    }

    #[test]
    fn pack_f_matches() {
        crate::difftest::check_paths(
            ee::s15::f_00412f58__generated,
            pack_f,
            SEEDS_3 + 4,
            // Partial subnormal shift costs 42, both round-to-even ties 49
            // (tie up and tie stay differ by a data move only).
            &[42, 49],
            &|c, seed| {
                let mut r = machine(c, seed, 0x0041_2f58, 0);
                if seed >= SEEDS_3 {
                    // Explicit edge operands: the digit table never yields
                    // a partial-shift exponent (-152, -126] nor a tie
                    // fraction, so seeds 64-67 build them directly.
                    // Class 3 (number), sign 0.
                    c.bus.write32(DB, 3);
                    c.bus.write32(DB + 4, 0);
                    c.bus.write32(
                        DB + 0xc,
                        match seed - SEEDS_3 {
                            // Partial subnormal shift: -126 - -140 = 14 < 26.
                            0 => 0x2456_789a,
                            // Round-to-even tie, bit 0x80 set: rounds up.
                            1 => 0xc0,
                            // Round-to-even tie, bit 0x80 clear: stays.
                            2 => 0x40,
                            // Shift of exactly 26 flushes to zero.
                            _ => 0x7654_3210,
                        },
                    );
                    c.bus.write32(
                        DB + 8,
                        match seed - SEEDS_3 {
                            0 => 0xffff_ff74,
                            // -126 - -152 = 26: the first shift that flushes.
                            3 => 0xffff_ff68,
                            _ => 0,
                        },
                    );
                } else {
                    unpacked_f(c, &mut r, seed, DB);
                }
                c.g[A0][0] = DB as u64;
            },
        );
    }

    #[test]
    fn addsf3_matches() {
        float_args(ee::s15::f_00413330__generated, addsf3, 0x0041_3330);
    }

    #[test]
    fn subsf3_matches() {
        float_args(ee::s15::f_00413388__generated, subsf3, 0x0041_3388);
    }

    /// Deterministic edge-case float bits for the soft-float multiply and
    /// divide tests: all eight class pairs (signed zeros, denormal,
    /// normals, infinities, quiet NaN) over the 64-seed window, so the
    /// inf/NaN/zero classify arms actually execute.
    /// Edge-case floats for the multiply/divide differential tests: the
    /// original 8 class representatives first (indices 0-7, so every old
    /// class pair still occurs), then value variants that exercise the
    /// assembly and rounding tails — negative normals (sign overlap),
    /// fractional mantissas (nonzero lo-words, sticky bits), max normal,
    /// min subnormal and near-overflow products. x = seed/16, y = seed%16
    /// over 256 seeds covers all 256 pairs jointly.
    pub(crate) const SEEDS_8: u32 = 256;
    pub(crate) fn muldiv_bits(seed: u32, k: u32) -> u32 {
        match (seed / k) % 16 {
            0 => 0x00000000,  // +0
            1 => 0x80000000,  // -0
            2 => 0x00400000,  // denormal
            3 => 0x3f800000,  // 1.0
            4 => 0x42c80000,  // 100.0
            5 => 0x7f800000,  // +inf
            6 => 0xff800000,  // -inf
            7 => 0x7fc00000,  // qNaN
            8 => 0xbf800000,  // -1.0
            9 => 0xc2c80000,  // -100.0
            10 => 0x3fc00000, // 1.5
            11 => 0x3dcccccd, // 0.1
            12 => 0x7f7fffff, // max normal
            13 => 0x00000001, // min subnormal
            14 => 0x7f000000, // near-overflow
            _ => 0x40490fdb,  // pi
        }
    }

    #[test]
    fn mulsf3_matches() {
        // `multu`: LO/HI must match too.
        check_paths_lohi(
            ee::s15::f_004133f0__generated,
            mulsf3,
            SEEDS_8 + 6,
            // Round ties: sticky even 182, odd 183, exact even 185.
            &[182, 183, 185],
            &|c, seed| {
                machine(c, seed, 0x0041_33f0, 0);
                c.f[12] = muldiv_bits(seed, 16);
                c.f[13] = muldiv_bits(seed, 1);
                if seed >= SEEDS_8 {
                    (c.f[12], c.f[13]) = MULSF3_EDGES[(seed - SEEDS_8) as usize];
                }
            },
        );
    }

    /// `__mulsf3` operands the class table never yields: a negative NaN
    /// with a negative operand (both orders: the NaN's unpacked sign word
    /// is the xor of the signs), a product whose normalize loop sees exactly
    /// 0x3fffffff, and the three round ties (guard bits 0x40): even with
    /// no sticky bits (stays), even with sticky bits (rounds up), odd.
    const MULSF3_EDGES: [(u32, u32); 6] = [
        (0xffc0_0000, 0xbf80_0000),
        (0xbf80_0000, 0xffc0_0000),
        (0x3fff_fffe, 0x3f80_0001),
        (0x3f80_0003, 0x3fc0_0000),
        (0x3f80_00c0, 0x3f81_0001),
        (0x3f80_0001, 0x3fc0_0000),
    ];

    /// `__divsf3` setup: two edge-case floats; some seeds resume at the
    /// division-loop head with a live frame and plausible loop registers.
    pub(crate) fn divsf3_setup(c: &mut Cpu, seed: u32) -> u32 {
        let mut r = machine(c, seed, 0x0041_35e8, 0);
        c.f[12] = muldiv_bits(seed, 16);
        c.f[13] = muldiv_bits(seed, 1);
        if seed % 10 == 7 {
            // Resume at the 0x136e0 loop head: allocated frame, saved
            // registers, unpacked operands and running loop state.
            let s = crate::difftest::SP - 0x50;
            c.g[SP][0] = s as u64;
            fill(c, &mut r, s, 0x50 / 4);
            c.bus.write32(s, 3);
            c.bus.write32(s + 4, 0);
            c.bus.write32(s + 8, 0x3f80_0000);
            c.bus.write32(s + 0xc, 0x4000_0000);
            c.bus.write32(s + 0x10, 3);
            c.bus.write32(s + 0x14, 1);
            c.bus.write32(s + 0x18, 0x3f80_0000);
            c.bus.write32(s + 0x1c, 0x4000_0000);
            c.bus.write64(s + 0x40, crate::difftest::RA as u64);
            c.g[A1][0] = (s + 0x20) as u64;
            c.g[A0][0] = 1u64 << 60 | (r.next_u32() as u64) << 24;
            c.g[T0][0] = 1u64 << 60 | (r.next_u32() as u64) << 20;
            c.g[V0][0] = 1u64 << (r.next_u32() % 61);
            c.g[A3][0] = (r.next_u32() as u64) << 20;
            0x0041_36e0
        } else {
            0x0041_35e8
        }
    }

    #[test]
    fn divsf3_matches() {
        check(
            ee::s15::f_004135e8__generated,
            divsf3,
            SEEDS_8,
            &|c, seed| {
                c.pc = divsf3_setup(c, seed);
            },
        );
    }

    crate::difftest::events::driven!(divsf3_gen_driven, ee::s15::f_004135e8__generated);

    crate::difftest::events::driven!(divsf3_hand_driven, divsf3);

    /// `__divsf3` with events in the division loop: normal operands so
    /// the loop runs, first unwind at the loop head.
    #[test]
    fn divsf3_events_match() {
        check_events(divsf3_gen_driven, divsf3_hand_driven, 32, &|c, seed| {
            let mut r = machine(c, seed, 0x0041_35e8, 0);
            // Normal operands with close exponents: a long division loop.
            c.f[12] = 0x3f80_0000 + (r.next_u32() % 8 << 23);
            c.f[13] = 0x3f80_0000 + (r.next_u32() % 8 << 23);
            let mode = schedule(c, &mut r, seed, 0x0041_35e8);
            c.next_event = c.cycles + 40 + (r.next_u32() % 300) as u64;
            mode
        });
    }

    pub(crate) const SEEDS_4: u32 = 64;

    #[test]
    fn fpcmp_parts_f_matches() {
        crate::difftest::check_paths(
            ee::s15::f_00413748__generated,
            fpcmp_parts_f,
            // 0-63 independent pairs, 64+ correlated (see correlate_b).
            128 + 1,
            // a.frac > b.frac at equal sign and exponent costs 34.
            &[34],
            &|c, seed| {
                let mut r = machine(c, seed, 0x0041_3748, 0);
                unpacked_f(c, &mut r, seed, DB);
                unpacked_f(c, &mut r, seed / 6 + 1, DB + 0x20);
                correlate_b(c, seed);
                if seed == 128 {
                    // a.frac > b.frac at equal sign and exponent: the
                    // correlated digits only ever add to b's fraction,
                    // so this arm needs a direct build (class 3, sign 0,
                    // exp 5, fracs 0x100 vs 0x0ff).
                    c.bus.write32(DB, 3);
                    c.bus.write32(DB + 4, 0);
                    c.bus.write32(DB + 8, 5);
                    c.bus.write32(DB + 0xc, 0x100);
                    c.bus.write32(DB + 0x20, 3);
                    c.bus.write32(DB + 0x24, 0);
                    c.bus.write32(DB + 0x28, 5);
                    c.bus.write32(DB + 0x2c, 0xff);
                }
                c.g[A0][0] = DB as u64;
                c.g[A1][0] = (DB + 0x20) as u64;
            },
        );
    }

    #[test]
    fn floatsisf_matches() {
        crate::difftest::check_paths(
            ee::s15::f_004138b0__generated,
            floatsisf,
            SEEDS_4,
            // INT_MIN arm costs 19 cycles (7 + 3 + 3 + 4 + 2, no PACK_F call);
            // every other arm calls PACK_F and costs more.
            &[19],
            &|c, seed| {
                machine(c, seed, 0x0041_38b0, 0);
                c.g[A0][0] = match seed % 11 {
                    0 => 0,
                    1 => 1,
                    2 => 0xffff_ffff,
                    3 => 0x7fff_ffff,
                    4 => 0x8000_0000,
                    // Sign-extended INT_MIN: the only word equal to the
                    // 0xffffffff80000000 gate below (zero-extended takes
                    // the negate path instead).
                    10 => 0xffff_ffff_8000_0000,
                    5 => 0x4b00_0000,
                    6 => 0xcb00_0000,
                    7 => 0x3f80_0000,
                    // Boundary for the (0x3fffffff < A2) check below.
                    8 => 0x3fff_ffff,
                    _ => 0x4000_0000,
                };
            },
        );
    }

    /// Edge-case float bits for the float-to-int test: zeros, fractions,
    /// magnitudes around the 2^31 overflow boundary, infinities and NaN,
    /// so the exponent-boundary checks actually execute.
    pub(crate) fn fixsfsi_bits(seed: u32) -> u32 {
        match seed % 16 {
            0 => 0x00000000,
            1 => 0x80000000,
            2 => 0x3f800000,
            3 => 0x3f000000,
            4 => 0x42c80000,
            5 => 0x4e800000,
            6 => 0x4f000000,
            7 => 0x4f800000,
            8 => 0xcf000000,
            9 => 0xc2f6e666,
            10 => 0x7f800000,
            11 => 0xff800000,
            12 => 0x7fc00000,
            13 => 0x00400000,
            14 => 0x3f800001,
            _ => 0x7f7fffff,
        }
    }

    crate::difftest::events::driven!(fixsfsi_gen_driven, ee::s15::f_00413968__generated);

    crate::difftest::events::driven!(fixsfsi_hand_driven, fixsfsi);

    #[test]
    fn fixsfsi_matches() {
        check(
            ee::s15::f_00413968__generated,
            fixsfsi,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x0041_3968, 0);
                c.f[12] = fixsfsi_bits(seed);
                // Dispatcher shape: enter at the entry.
                c.pc = 0x0041_3968;
            },
        );
        // Events in the subnormal shift loop: unwinds resume at its head.
        check_events(
            fixsfsi_gen_driven,
            fixsfsi_hand_driven,
            SEEDS_4,
            &|c, seed| {
                let _ = machine(c, seed, 0x0041_3968, 0);
                // Small and denormal operands run the shift loop.
                c.f[12] = [0x0040_0000, 0x3f00_0000, 0x3380_0000][seed as usize % 3];
                let mut r = Rng(0x0041_3968 ^ seed as u64);
                let mode = schedule(c, &mut r, seed, 0x0041_3968);
                c.next_event = c.cycles + 10 + (r.next_u32() % 100) as u64;
                mode
            },
        );
    }

    /// An FDE chain: `n` nodes, each 8 bytes (`+0` next-ish, `+4` count-ish).
    pub(crate) fn fde_chain(c: &mut Cpu, r: &mut Rng, seed: u32, at: u32) {
        let n = (seed % 4) as u32;
        for i in 0..3 {
            c.bus
                .write32(at + i * 8, if i < n { at + (i + 1) * 8 } else { 0 });
            c.bus.write32(
                at + i * 8 + 4,
                if i == 0 && seed / 4 % 2 == 0 {
                    0
                } else {
                    r.next_u32() & 0xff
                },
            );
        }
        c.bus.write32(at + 3 * 8, 0);
        c.bus.write32(at + 3 * 8 + 4, 0);
    }

    #[test]
    fn deregister_frame_info_matches() {
        // The registry-empty path (global word zero) plus populated
        // chains; the abort tail returns through the miss-hook fake.
        // Seed 64 adds the no-free arm (see below).
        crate::difftest::check_paths(
            ee::s15::f_00414780__generated,
            deregister_frame_info,
            SEEDS_4 + 1,
            // All-no-free probe run takes 20 cycles; the match-free, walk and
            // abort arms cost more.
            &[20],
            &|c, seed| {
                let mut r = machine(c, seed, 0x0041_4780, 0);
                c.bus
                    .write32(0x0052_ade8, if seed % 3 == 0 { 0 } else { DB + 0x400 });
                fde_chain(c, &mut r, seed / 3, DB + 0x400);
                if seed == SEEDS_4 {
                    // The no-free arm: a populated registry whose head
                    // matches with no next link. The seed digits never
                    // align a match with an empty next (seed % 8 == 4
                    // with (seed / 3) % 4 == 0 always lands on seed % 3
                    // == 0, the abort tail), so this seed builds it
                    // directly.
                    c.bus.write32(0x0052_ade8, DB + 0x400);
                    c.bus.write32(DB + 0x400, 0);
                    c.bus.write32(DB + 0x408, DB + 0x600);
                }
                c.g[A0][0] = (DB + 0x600) as u64;
                fill(c, &mut r, DB + 0x600, 4);
                // Matching entry on a subset of populated chains: the walk
                // compares each node's +8 word against the query address
                // (A0 = DB+0x600) and the fixture never matches, so the
                // match arm never runs. On seed%8==4 with a non-empty
                // registry, point the first compared word (DB+0x408) at
                // the query so the first comparison hits. Other seeds
                // keep the mismatch walk, so this only adds coverage.
                if seed % 3 != 0 && seed % 8 == 4 {
                    c.bus.write32(DB + 0x408, DB + 0x600);
                }
            },
        );
    }

    #[test]
    fn rtti_class_matches() {
        check(
            ee::s15::f_00414bd8__generated,
            rtti_class,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x0041_4bd8, 0);
                c.g[A0][0] = if seed % 3 == 0 {
                    0
                } else {
                    (DB + 0x400) as u64
                };
                c.g[A1][0] = (DB + 0x500) as u64;
                c.g[A2][0] = 0x1234;
                c.g[A3][0] = 0x5678;
            },
        );
    }

    #[test]
    fn si_type_info_dcast_matches() {
        crate::difftest::check_paths(
            ee::s15::f_00414c70__generated,
            si_type_info_dcast,
            SEEDS_4,
            // All-base probe run takes only 93 cycles; early exit takes 83.
            &[83, 93],
            &|c, seed| {
                let mut r = machine(c, seed, 0x0041_4c70, 0);
                fill(c, &mut r, DB + 0x400, 16);
                c.bus.write32(DB + 0x400, DB + 0x500);
                c.bus.write32(DB + 0x408, 0);
                if seed % 4 == 3 {
                    // Distinct names, so the eq gate misses and the base
                    // call runs: T2 points at a base record whose +4 points
                    // at a slot table; the slot's +0x14 target is `__empty`
                    // (0x410408), which just returns, and +0x10 holds the
                    // 0x10 address delta the call adds to the base pointer.
                    for (i, b) in b"aaa\0".iter().enumerate() {
                        c.bus.write8(DB + 0x480 + i as u32, *b);
                    }
                    for (i, b) in b"bbb\0".iter().enumerate() {
                        c.bus.write8(DB + 0x580 + i as u32, *b);
                    }
                    c.bus.write32(DB + 0x400, DB + 0x480);
                    c.bus.write32(DB + 0x500, DB + 0x580);
                    c.bus.write32(DB + 0x408, DB + 0x900);
                    c.bus.write32(DB + 0x904, DB + 0x920);
                    c.bus.write32(DB + 0x930, 0x10);
                    c.bus.write32(DB + 0x934, 0x0041_0408);
                }
                c.g[A0][0] = (DB + 0x400) as u64;
                c.g[A1][0] = (DB + 0x500) as u64;
                c.g[A2][0] = (DB + 0x600) as u64;
                c.g[A3][0] = (DB + 0x700) as u64;
                c.g[T0][0] = r.next_u32() as u64;
                c.g[T1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn class_type_info_dcast_matches() {
        // The dcast walk is value-steered: the entry eq gate needs distinct
        // names (one seed keeps equal names for the early exit), then the
        // base count (0/1/2), the per-entry flags (mask/next via >>30,
        // direct/deref via >>29) and FP drive the skeleton. The three
        // virtual calls miss to the hook fake (unmapped targets); their
        // results steer the walk: A-seeds take the hash (nonzero, differing
        // per call, so two-iteration seeds reach the deep 414e88 region),
        // B-seeds (count=2 fixed) force LOG_RESULTS 1,2 so the iterations
        // differ, and force the S1/V0 pair through a 5-combo lattice against
        // S7. S5 = 0 on some seeds takes the 414e88 skip arm, nonzero the
        // deep calls. Both sides run the same walk identically. The S0 == 0
        // next-base arm is not reached: the fakes never return 0 (in the
        // game a null virtual return takes it).
        check(
            ee::s15::f_00414d08__generated,
            class_type_info_dcast,
            SEEDS_4 + 2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0041_4d08, 0);
                fill(c, &mut r, DB + 0x400, 32);
                fill(c, &mut r, DB + 0x700, 32);
                let dst_is_src = seed == 61;
                for (i, b) in (if dst_is_src {
                    &b"SrcT\0"[..]
                } else {
                    &b"DstT\0"[..]
                })
                .iter()
                .enumerate()
                {
                    c.bus.write8(DB + 0x500 + i as u32, *b);
                }
                for (i, b) in b"SrcT\0".iter().enumerate() {
                    c.bus.write8(DB + 0x600 + i as u32, *b);
                }
                // dst typeinfo: name, base table, count by seed (0/1/2,
                // B-seeds fix count=2 for the two-iteration lattice).
                let b_seed = seed >= 48;
                c.bus.write32(DB + 0x400, DB + 0x500);
                c.bus.write32(DB + 0x408, DB + 0x480);
                c.bus.write32(DB + 0x40c, if b_seed { 2 } else { seed % 3 });
                // Base entries: +4 flags steer mask (>>30==1) vs next and
                // direct (>>29&1==0) vs deref.
                let flags = if seed >= SEEDS_4 {
                    // Flags with the low 29 bits set (keeps the >>30==1
                    // skeleton path and the direct-deref bit).
                    // Seeds 0-63 untouched.
                    0x4000_0001
                } else if b_seed {
                    0x4000_0000
                } else {
                    match seed / 6 % 3 {
                        0 => 0,
                        1 => 0x4000_0000,
                        _ => 0x6000_0000,
                    }
                };
                c.bus.write32(DB + 0x484, flags);
                c.bus.write32(DB + 0x48c, flags);
                // src object: [0] name, [+4] vtable-ish chain head.
                c.bus.write32(DB + 0x640, DB + 0x600);
                c.bus.write32(DB + 0x644, DB + 0x760);
                // Offset cell + indirection chain. Every call target reads
                // as 0 (miss → fake): direct via [DB+0x774], deref via
                // unmapped reads.
                c.bus.write32(DB + 0x700, DB + 0x720);
                c.bus.write32(DB + 0x720, 0);
                c.bus.write32(DB + 0x724, DB + 0x760);
                c.bus.write32(DB + 0x770, 0);
                c.bus.write32(DB + 0x774, 0);
                if b_seed {
                    const LATTICE: [(u32, u32, u32); 5] =
                        [(5, 5, 5), (5, 5, 0), (5, 7, 5), (5, 7, 7), (5, 7, 9)];
                    let (s1, v0, s7) = LATTICE[(seed - 48) as usize % 5];
                    c.bus.write32(LOG_RESULTS, 1);
                    c.bus.write32(LOG_RESULTS + 4, 2);
                    c.bus.write32(LOG_RESULTS + 8, s1);
                    c.bus.write32(LOG_RESULTS + 12, v0);
                    c.g[T1][0] = s7 as u64;
                } else {
                    c.g[T1][0] = 0;
                }
                // S5 is read only by the 414e88 skip check (fake calls
                // ignore their args), so a bare value steers it.
                c.g[A0][0] = (DB + 0x400) as u64;
                c.g[A1][0] = (DB + 0x640) as u64;
                c.g[A2][0] = (if b_seed { 1 } else { seed / 3 % 2 }) as u64;
                c.g[A3][0] = (DB + 0x700) as u64;
                // S5 = 0 takes the 414e88 skip arm, nonzero the deep calls.
                c.g[T0][0] = (if seed / 12 % 2 == 0 { 0 } else { DB + 0x650 }) as u64;
                if seed == 65 {
                    // Seed 60's shape (B-seed lattice index 2) with the
                    // count raised from 2 to 3, so the entry-count test
                    // compares V0 = 2 against A1 = 3. The third entry is
                    // inert padding (null link, same flags); the walk
                    // exits on it.
                    c.bus.write32(DB + 0x40c, 3);
                    c.bus.write32(DB + 0x484, 0x4000_0000);
                    c.bus.write32(DB + 0x48c, 0x4000_0000);
                    c.bus.write32(DB + 0x490, 0);
                    c.bus.write32(DB + 0x494, 0x4000_0000);
                }
            },
        );
    }

    #[test]
    fn class_type_info_ctor_matches() {
        check(
            ee::s15::f_00414ee0__generated,
            class_type_info_ctor,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x0041_4ee0, 0);
                c.g[A0][0] = (DB + 0x400) as u64;
                c.g[A1][0] = (DB + 0x500) as u64;
                c.g[A2][0] = 0x1234;
                c.g[A3][0] = 0x5678;
            },
        );
    }

    #[test]
    fn si_type_info_ctor_matches() {
        check(
            ee::s15::f_00414f70__generated,
            si_type_info_ctor,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x0041_4f70, 0);
                c.g[A0][0] = (DB + 0x400) as u64;
                c.g[A1][0] = (DB + 0x500) as u64;
                c.g[A2][0] = 0x1234;
            },
        );
    }

    #[test]
    fn user_type_info_ctor_matches() {
        check(
            ee::s15::f_00415000__generated,
            user_type_info_ctor,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x0041_5000, 0);
                c.g[A0][0] = (DB + 0x400) as u64;
                c.g[A1][0] = (DB + 0x500) as u64;
                c.g[A2][0] = 0x1234;
                c.g[A3][0] = 0x5678;
            },
        );
    }

    #[test]
    fn frame_state_for_matches() {
        // find_fde walks the real registry at lw(0x52ade8): seed a one-node
        // table whose FDE covers the query pc, so the search hits instead of
        // returning 0. EXTRACT_CIE_INFO then runs over zeroed bytes and
        // returns nonzero (its only zero path needs a non-'z' augmentation
        // after two differing strcmp, which zero bytes don't produce). The
        // S1 window is steered small so the CFA loop stays bounded; the
        // tail copy loop runs a fixed 9 iterations. Both sides run the same
        // walk identically.
        check(
            ee::s15::f_00414818__generated,
            frame_state_for,
            SEEDS_4 + 10,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0041_4818, 0);
                fill(c, &mut r, DB + 0x400, 32);
                fill(c, &mut r, DB + 0x500, 8);
                fill(c, &mut r, DB + 0x600, 16);
                fill(c, &mut r, DB + 0x800, 64);
                // Registry node: nonzero (skip frame_init), pc_lo <= S1 <
                // pc_hi, table, count=1, end of chain.
                c.bus.write32(DB + 0x400, 1);
                c.bus.write32(DB + 0x404, 0x200000);
                c.bus.write32(DB + 0x40c, DB + 0x500);
                c.bus.write32(DB + 0x410, 1);
                c.bus.write32(DB + 0x414, 0);
                c.bus.write32(0x0052ade8, DB + 0x400);
                // Sort table: one FDE covering the query pc.
                c.bus.write32(DB + 0x500, DB + 0x600);
                // FDE: [0]=0, [+4] steers the S1 window small
                // (S1 = FDE-X+lw(FDE-X); X=FDE-0x100 → S1=0x104),
                // [+8]=query pc (span check), [+0xc]=span.
                c.bus.write32(DB + 0x600, 0);
                c.bus.write32(DB + 0x604, (DB + 0x600).wrapping_sub(0x100));
                c.bus.write32(DB + 0x608, 0x180000);
                c.bus.write32(DB + 0x60c, 0x100);
                c.g[A0][0] = 0x180000;
                c.g[A1][0] = (DB + 0x800) as u64;
                c.g[A2][0] = (seed % 2) as u64;
                c.g[A3][0] = (seed / 2 % 2) as u64;
                if seed >= SEEDS_4 {
                    // Steer S1 up to and past S0 so the augmentation/CFA
                    // loops run, including S1==S0 boundary inputs.
                    // Seeds 0-63 untouched.
                    let x = seed - SEEDS_4;
                    if x < 3 {
                        // Seeds 64-66: S1_r1 == S1_r2 boundary inputs (X=4
                        // shares one knob W=FDE[+0] for both loop ends).
                        // S1 = DB+0x604+W; the three W values probe
                        // different S0 magnitudes.
                        const WS: [u32; 3] = [0x10c, 0x10c + 0x100, 0xffc179ec];
                        c.bus.write32(DB + 0x604, 4);
                        c.bus.write8(DB + 0x609, 0x7a);
                        c.bus.write8(DB + 0x60a, 0x01);
                        c.bus.write8(DB + 0x60b, 0x00);
                        c.bus.write32(DB + 0x600, WS[x as usize]);
                    } else {
                        // Seeds 67+: augmentation bytes in controlled
                        // memory (X wraps so the augmentation pointer
                        // lands at DB+0x700): a 'z' header plus a 0x41
                        // advance-loc sled. Each call consumes at least
                        // one byte and the cursor advances monotonically,
                        // so the loops terminate at S1.
                        const XAUG: u32 = 0xffff_ff0d; // AUG = DB+0x700
                        c.bus.write32(DB + 0x604, XAUG);
                        c.bus.write8(DB + 0x700, 0x7a);
                        c.bus.write8(DB + 0x701, 0x01);
                        c.bus.write8(DB + 0x702, 0x00);
                        for i in 0..0x200u32 {
                            c.bus.write8(DB + 0x703 + i, 0x41);
                        }
                        // S1_r1 = (S2-X)+8+lw(S2-X+4); S2-X+4 = DB+0x6f7.
                        c.bus.write32(DB + 0x6f7, 0x100);
                        // S1_r2 = S2+FDE[+0]+4: 0 skips region 2
                        // (region-1 loop only); 0x200 enters it with the
                        // cursor inside the sled. Seed 69 also moves the
                        // span word off pc, and seed 71 moves the query
                        // pc off the span word instead (span untouched);
                        // both make A1 != S6. Seed 70 uses W2=0x201 for
                        // an even S1_r2 window.
                        if x == 3 {
                            c.bus.write32(DB + 0x600, 0);
                        } else {
                            c.bus
                                .write32(DB + 0x600, if x == 6 { 0x201 } else { 0x200 });
                            if x == 5 {
                                c.bus.write32(DB + 0x608, 0x17fff0);
                                c.bus.write32(DB + 0x60c, 0x20);
                            }
                            if x == 7 {
                                c.g[A0][0] = 0x180010;
                            }
                        }
                        if x == 8 {
                            // Both instruction windows empty (S0 == S1 at
                            // each loop gate): CIE length 0x4d ends region 1
                            // at the sled cursor DB+0x748; FDE augmentation
                            // length 0 and W2 = 0xd end region 2 at its
                            // first byte.
                            c.bus.write32(DB + 0x6f7, 0x4d);
                            c.bus.write8(DB + 0x610, 0);
                            c.bus.write32(DB + 0x600, 0xd);
                            c.bus.write8(DB + 0x611, 0x40);
                        }
                        if x == 9 {
                            // Region 2 at the query pc: four advance_loc(0)
                            // keep the running pc equal to the query pc and
                            // end exactly at S1 (W2 = 0x11); a fifth sits
                            // past S1 for an over-run to read.
                            c.bus.write8(DB + 0x610, 0);
                            for i in 0..5 {
                                c.bus.write8(DB + 0x611 + i, 0x40);
                            }
                            c.bus.write32(DB + 0x600, 0x11);
                        }
                    }
                }
            },
        );
    }

    pub(crate) const SEEDS_5: u32 = 64;

    #[test]
    fn throw_type_match_rtti_2_matches() {
        // Dispatcher shape: entries across seeds cover every arm (entry
        // plus each otherwise-dead resume arm). Mid-entry arms run with
        // scrambled registers; unmapped reads yield 0 on both sides, so
        // the typeinfo walks exit the same way. Both sides run the same
        // arms identically.
        check(
            ee::s15::f_00415260__generated,
            throw_type_match_rtti_2,
            134,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0041_5260, 0);
                fill(c, &mut r, DB, 16);
                c.bus.write32(DB, 0x4a0000);
                c.bus.write32(DATA + 4, DB + 0x100);
                c.bus.write32(DATA + 8, DB + 0x200);
                fill(c, &mut r, DB + 0x100, 16);
                fill(c, &mut r, DB + 0x200, 16);
                c.g[A0][0] = DB as u64;
                c.g[A1][0] = (DB + 0x100) as u64;
                c.g[A2][0] = (DB + 0x200) as u64;
                const ENTRIES: [u32; 67] = [
                    0x0041_5260,
                    0x0041_52a4,
                    0x0041_52ac,
                    0x0041_52b4,
                    0x0041_52e4,
                    0x0041_52ec,
                    0x0041_52f4,
                    0x0041_52fc,
                    0x0041_5328,
                    0x0041_5330,
                    0x0041_5338,
                    0x0041_5364,
                    0x0041_536c,
                    0x0041_537c,
                    0x0041_53a8,
                    0x0041_53b0,
                    0x0041_53b8,
                    0x0041_53c0,
                    0x0041_53ec,
                    0x0041_53f4,
                    0x0041_53fc,
                    0x0041_5410,
                    0x0041_5424,
                    0x0041_5438,
                    0x0041_5454,
                    0x0041_545c,
                    0x0041_548c,
                    0x0041_5494,
                    0x0041_549c,
                    0x0041_54c8,
                    0x0041_54d0,
                    0x0041_54d4,
                    0x0041_5500,
                    0x0041_5508,
                    0x0041_5510,
                    0x0041_553c,
                    0x0041_5544,
                    0x0041_554c,
                    0x0041_5554,
                    0x0041_5580,
                    0x0041_5588,
                    0x0041_559c,
                    0x0041_55a0,
                    0x0041_55a8,
                    0x0041_55b0,
                    0x0041_55d4,
                    0x0041_55dc,
                    0x0041_55e4,
                    0x0041_5608,
                    0x0041_5610,
                    0x0041_5618,
                    0x0041_5628,
                    0x0041_5630,
                    0x0041_5654,
                    0x0041_565c,
                    0x0041_5664,
                    0x0041_566c,
                    0x0041_5690,
                    0x0041_5698,
                    0x0041_56a0,
                    0x0041_56b4,
                    0x0041_56c4,
                    0x0041_56cc,
                    0x0041_56d8,
                    0x0041_56e0,
                    0x0041_56f0,
                    0x0041_56f4,
                ];
                c.pc = ENTRIES[seed as usize % ENTRIES.len()];
                let _ = r;
            },
        );
    }

    #[test]
    fn is_pointer_matches() {
        check(
            ee::s15::f_00415728__generated,
            is_pointer,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0041_5728, 0);
                if seed % 3 == 0 {
                    c.g[A0][0] = 0;
                } else {
                    let obj = DB + 0x400;
                    let vt = DB + 0x500;
                    c.bus
                        .write32(obj + 4, vt + if seed % 3 == 1 { 0 } else { 8 });
                    c.bus.write16(vt + 4, 0x42);
                    c.bus.write32(vt + 8, FAKE);
                    fill(c, &mut r, DB + 0x600, 8);
                    c.g[A0][0] = obj as u64;
                }
                let _ = r;
            },
        );
    }

    /// The six `__rtti_*` fillers share one shape; only the vtable word
    /// and stored fields differ.
    pub(crate) fn rtti_setup(c: &mut Cpu, seed: u32) {
        c.g[A0][0] = if seed % 3 == 0 {
            0
        } else {
            (DB + 0x400) as u64
        };
        c.g[A1][0] = 0x1111_1111;
        c.g[A2][0] = 0x2222_2222;
        c.g[A3][0] = 0x3333_3333;
    }

    #[test]
    fn rtti_ptr_matches() {
        check(
            ee::s15::f_00415778__generated,
            rtti_ptr,
            SEEDS_5,
            &|c, seed| {
                machine(c, seed, 0x0041_5778, 0);
                rtti_setup(c, seed);
            },
        );
    }

    #[test]
    fn rtti_attr_matches() {
        check(
            ee::s15::f_00415798__generated,
            rtti_attr,
            SEEDS_5,
            &|c, seed| {
                machine(c, seed, 0x0041_5798, 0);
                rtti_setup(c, seed);
            },
        );
    }

    #[test]
    fn rtti_func_matches() {
        check(
            ee::s15::f_004157c0__generated,
            rtti_func,
            SEEDS_5,
            &|c, seed| {
                machine(c, seed, 0x0041_57c0, 0);
                rtti_setup(c, seed);
            },
        );
    }

    #[test]
    fn rtti_ptmf_matches() {
        check(
            ee::s15::f_004157e0__generated,
            rtti_ptmf,
            SEEDS_5,
            &|c, seed| {
                machine(c, seed, 0x0041_57e0, 0);
                rtti_setup(c, seed);
            },
        );
    }

    #[test]
    fn rtti_ptmd_matches() {
        check(
            ee::s15::f_00415800__generated,
            rtti_ptmd,
            SEEDS_5,
            &|c, seed| {
                machine(c, seed, 0x0041_5800, 0);
                rtti_setup(c, seed);
            },
        );
    }

    #[test]
    fn rtti_array_matches() {
        check(
            ee::s15::f_00415820__generated,
            rtti_array,
            SEEDS_5,
            &|c, seed| {
                machine(c, seed, 0x0041_5820, 0);
                rtti_setup(c, seed);
            },
        );
    }

    #[test]
    fn dynamic_cast_matches() {
        // Indirect handler calls go through the miss-hook fake; unwind
        // at the second one so a terminate path can't recurse forever.
        // The last quarter of seeds never unwind: every call target is
        // controlled (FAKE or the laid-out chain below), so calls 3-4
        // and the epilogue run. Fake cycle counts vary with the call
        // hash, so no `check_paths` total can isolate the full walk;
        // these seeds compare it.
        check(
            ee::s15::f_00415840__generated,
            dynamic_cast,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0041_5840, 0);
                fill(c, &mut r, DB + 0x400, 16);
                if seed % 4 == 3 {
                    // Full walk: no unwind, call 1 returns our block, the
                    // chain words route call 4 at FAKE, calls 2-3 target
                    // FAKE directly (call 3's target passes through T0,
                    // which the first fake clobbers to a hash address that
                    // still lands in the miss hook).
                    c.bus.write32(LOG_UNWIND_AT, u32::MAX);
                    c.bus.write32(LOG_RESULTS, DB + 0x800);
                    c.bus.write32(DB + 0x804, DB + 0x820);
                    c.bus.write32(DB + 0x830, 0x20);
                    c.bus.write32(DB + 0x834, FAKE);
                    c.g[A0][0] = FAKE as u64;
                    c.g[A1][0] = FAKE as u64;
                    c.g[T0][0] = FAKE as u64;
                } else {
                    c.bus.write32(LOG_UNWIND_AT, 1);
                    c.g[A0][0] = FAKE as u64;
                    c.g[A1][0] = (DB + 0x400) as u64;
                    c.g[T0][0] = r.next_u32() as u64;
                }
                c.g[A2][0] = (DB + 0x500) as u64;
                c.g[A3][0] = (DB + 0x600) as u64;
                c.g[T1][0] = r.next_u32() as u64;
            },
        );
    }

    /// One `__tf*` singleton slot: zero (install path) or set (no-op).
    pub(crate) fn tf_singleton(c: &mut Cpu, r: &mut Rng, seed: u32, slot: u32) {
        c.bus
            .write32(slot, if seed % 3 == 0 { 0 } else { r.next_u32() | 1 });
    }

    #[test]
    fn tfx_matches() {
        check(ee::s15::f_00415928__generated, tfx, SEEDS_5, &|c, seed| {
            let mut r = machine(c, seed, 0x0041_5928, 0);
            tf_singleton(c, &mut r, seed, 0x0045_4160);
        });
    }

    #[test]
    fn tfl_matches() {
        check(ee::s15::f_00415960__generated, tfl, SEEDS_5, &|c, seed| {
            let mut r = machine(c, seed, 0x0041_5960, 0);
            tf_singleton(c, &mut r, seed, 0x0045_4168);
        });
    }

    #[test]
    fn tfi_matches() {
        check(ee::s15::f_00415998__generated, tfi, SEEDS_5, &|c, seed| {
            let mut r = machine(c, seed, 0x0041_5998, 0);
            tf_singleton(c, &mut r, seed, 0x0045_4170);
        });
    }

    #[test]
    fn tfs_matches() {
        check(ee::s15::f_004159d0__generated, tfs, SEEDS_5, &|c, seed| {
            let mut r = machine(c, seed, 0x0041_59d0, 0);
            tf_singleton(c, &mut r, seed, 0x0045_4178);
        });
    }

    #[test]
    fn tfb_matches() {
        check(ee::s15::f_00415a08__generated, tfb, SEEDS_5, &|c, seed| {
            let mut r = machine(c, seed, 0x0041_5a08, 0);
            tf_singleton(c, &mut r, seed, 0x0045_4180);
        });
    }

    #[test]
    fn tfc_matches() {
        check(ee::s15::f_00415a40__generated, tfc, SEEDS_5, &|c, seed| {
            let mut r = machine(c, seed, 0x0041_5a40, 0);
            tf_singleton(c, &mut r, seed, 0x0045_4188);
        });
    }

    #[test]
    fn tfw_matches() {
        check(ee::s15::f_00415a78__generated, tfw, SEEDS_5, &|c, seed| {
            let mut r = machine(c, seed, 0x0041_5a78, 0);
            tf_singleton(c, &mut r, seed, 0x0045_4190);
        });
    }

    #[test]
    fn tfr_matches() {
        check(ee::s15::f_00415ab0__generated, tfr, SEEDS_5, &|c, seed| {
            let mut r = machine(c, seed, 0x0041_5ab0, 0);
            tf_singleton(c, &mut r, seed, 0x0045_4198);
        });
    }

    pub(crate) const SEEDS_6: u32 = 64;

    /// Double bit pattern across the classes `__ieee754_fmod`/`log10`
    /// exercise: ±0, subnormals, inf/NaN, exact powers of two and
    /// general normals.
    pub(crate) fn edge_f64(r: &mut Rng, seed: u32, lo: u64) -> u64 {
        // Only the low word is the caller's operand fragment; the high
        // word is set per case below (or-ing the full word would bury
        // the subnormal and fixed-exponent patterns in random bits).
        let lo = lo & 0xffff_ffff;
        let sign = ((r.next_u32() & 1) as u64) << 63;
        match seed % 10 {
            0 => sign,
            1 => sign | lo,
            2 => ((r.next_u32() % 0x10_0000) as u64) << 32 | lo,
            3 => {
                let m = if r.next_u32() % 2 == 0 { 0 } else { lo };
                sign | 0x7ff0_0000_0000_0000 | m
            }
            4 => ((0x3ff + r.next_u32() % 7 - 3) as u64) << 52,
            5 => ((0x3ff + r.next_u32() % 7 - 3) as u64) << 52 | 0xf_ffff << 32 | lo,
            6 | 7 => {
                let e = 1 + (r.next_u32() % 0x7fe) as u64;
                e << 52 | ((r.next_u32() % 0x10_0000) as u64) << 32 | lo
            }
            _ => {
                let e = 1 + (r.next_u32() % 0x7fe) as u64;
                e << 52 | lo
            }
        }
    }

    #[test]
    fn tfd_matches() {
        check(ee::s15::f_00415ae8__generated, tfd, SEEDS_6, &|c, seed| {
            let mut r = machine(c, seed, 0x0041_5ae8, 0);
            tf_singleton(c, &mut r, seed, 0x0045_41a0);
        });
    }

    #[test]
    fn tff_matches() {
        check(ee::s15::f_00415b20__generated, tff, SEEDS_6, &|c, seed| {
            let mut r = machine(c, seed, 0x0041_5b20, 0);
            tf_singleton(c, &mut r, seed, 0x0045_41a8);
        });
    }

    #[test]
    fn tfui_matches() {
        check(ee::s15::f_00415b58__generated, tfui, SEEDS_6, &|c, seed| {
            let mut r = machine(c, seed, 0x0041_5b58, 0);
            tf_singleton(c, &mut r, seed, 0x0045_41b0);
        });
    }

    #[test]
    fn tful_matches() {
        check(ee::s15::f_00415b90__generated, tful, SEEDS_6, &|c, seed| {
            let mut r = machine(c, seed, 0x0041_5b90, 0);
            tf_singleton(c, &mut r, seed, 0x0045_41b8);
        });
    }

    #[test]
    fn tfux_matches() {
        check(ee::s15::f_00415bc8__generated, tfux, SEEDS_6, &|c, seed| {
            let mut r = machine(c, seed, 0x0041_5bc8, 0);
            tf_singleton(c, &mut r, seed, 0x0045_41c0);
        });
    }

    #[test]
    fn tfus_matches() {
        check(ee::s15::f_00415c00__generated, tfus, SEEDS_6, &|c, seed| {
            let mut r = machine(c, seed, 0x0041_5c00, 0);
            tf_singleton(c, &mut r, seed, 0x0045_41c8);
        });
    }

    #[test]
    fn tfuc_matches() {
        check(ee::s15::f_00415c38__generated, tfuc, SEEDS_6, &|c, seed| {
            let mut r = machine(c, seed, 0x0041_5c38, 0);
            tf_singleton(c, &mut r, seed, 0x0045_41d0);
        });
    }

    #[test]
    fn tfsc_matches() {
        check(ee::s15::f_00415c70__generated, tfsc, SEEDS_6, &|c, seed| {
            let mut r = machine(c, seed, 0x0041_5c70, 0);
            tf_singleton(c, &mut r, seed, 0x0045_41d8);
        });
    }

    crate::difftest::events::driven!(fmod_gen_driven, ee::s15::f_0041ef50__generated);

    crate::difftest::events::driven!(fmod_hand_driven, ieee754_fmod);

    /// fmod fixture: edge-case doubles plus targeted subnormal arms.
    pub(crate) fn fmod_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x0041_ef50, 0);
        elf_image(c);
        let lo0 = r.next_u32() as u64 | (r.next_u32() as u64) << 32;
        let lo1 = r.next_u32() as u64 | (r.next_u32() as u64) << 32;
        c.g[A0][0] = edge_f64(&mut r, seed, lo0);
        // Sometimes equal magnitudes (the fast path), else independent.
        c.g[A1][0] = if seed % 4 == 0 {
            c.g[A0][0] ^ ((r.next_u32() as u64 & 1) << 63)
        } else {
            edge_f64(&mut r, seed / 4 + 1, lo1)
        };
        if seed >= 128 {
            // Extra inputs for the subnormal reduction arms and the
            // f0dc/f320 regions.
            const EXTRA: [(u64, u64); 26] = [
                (0x0000000000000100, 0x0000000000000001), // f05c-chain (+f0dc)
                (0x0000000100000002, 0x0000000000000001), // f098 V0-test>0
                (0x3ff0000000000000, 0x0000000000000010), // f0dc via f044 (A1=0)
                (0x0010000000000001, 0x0000000000000001), // f0dc via f08c (A1=-1022)
                (0x000d557ffe3b890b, 0x000d557f4281815c), // f320 window
                (0x000fffff00000011, 0x0000000000000010), // f044 A3==0xFFFFF
                (0x3ff0000000000000, 0x000fffff00000000), // f0c4 T1==0xFFFFF
                (0x0640581d4aac726f, 0x0028dcb434e4e24b), // f29c A0==-1022
                (0x0000000100001000, 0x0000000000000100), // f16c A2=20
                (0x3ff0000000000000, 0x0000000100000001), // y high == low != 0
                (0x3ff0000000000000, 0x7ff0000000000001), // NaN y, low bits only
                (0x3ff0000000000000, 0x7ff0000080000000), // NaN y, low bit 31 only
                (0x3ff0000000000000, 0x7ff0000100000001), // NaN y, high bit 0
                (0xffefffffffffffff, 0x9ed00000000b1b84), // hx == 0x7fefffff
                (0x001000001028b332, 0x801000000000001c), // y exponent -1022
                (0x0000000000100000, 0x0000000000000003), // x shift of exactly 32
                (0x65900000ff620e16, 0x80000000001c32b6), // y shift of exactly 32
                (0x7fb0000000002000, 0x0367f66bbcccff9a), // overlapping result halves
                (0xe990000000000020, 0x803ce017356400ef), // negative x, normal result
                (0xc03fffffffffffff, 0xbfd0000000000000), // equal high words in the loop
                (0x6bdfffffffffffff, 0x0030000000002000), // equal low words in the loop
                (0x001fffffffffffff, 0x8000000000200000), // result shift of exactly 32
                (0xbadfffffffffffff, 0x0000000010000000), // negative x, subnormal result
                (0x4000000d0000000d, 0x3ff0000000000000), // loop difference high == low
                (0x3ff0000d0000000d, 0x3ff0000000000000), // final difference high == low
                (0xc008000000000000, 0x3ff0000000000000), // -3 mod 1: -0 after the loop
            ];
            let (x, y) = EXTRA[(seed - 128) as usize];
            c.g[A0][0] = x;
            c.g[A1][0] = y;
        } else if seed >= 64 {
            // Correlated class x exp table: x spans subnormal, tiny
            // exps (the normalization preamble needs exp < 16), mid
            // and max exps; y spans zero, subnormal, below-/-equal-/
            // above-x exps, inf and NaN, so the dead reduction arms
            // run instead of only the class dispatch. Absolute exps
            // keep every value in range by construction. Seeds 0-63
            // keep the independent pairing exactly as before, so the
            // earlier coverage still holds.
            const XS: [u64; 8] = [0, 1, 2, 5, 15, 0x3ff, 0x3fe, 0x7fe];
            let i = seed - 64;
            let ex = XS[(i / 8) as usize];
            let mut fx = lo0 & 0x000f_ffff_ffff_ffff;
            if i % 2 == 0 {
                fx |= 0x0008_0000_0000_0000;
            }
            if ex == 0 {
                fx |= 1;
            }
            let sx = (r.next_u32() as u64 & 1) << 63;
            let sy = (r.next_u32() as u64 & 1) << 63;
            let x = sx | ex << 52 | fx;
            let mut fy = lo1 & 0x000f_ffff_ffff_ffff;
            if i % 2 == 0 {
                fy |= 0x0008_0000_0000_0000;
            }
            c.g[A1][0] = sy
                | match i % 8 {
                    0 => 0,
                    1 => (fy | 1) & 0x000f_ffff_ffff_ffff,
                    2 => (if ex == 0 { 1 } else { ex - 1 }) << 52 | fy & 0x000f_ffff_ffff_ffff,
                    3 => {
                        // Equal magnitudes (the zero-result path).
                        (x ^ sx ^ sy) & 0x7fff_ffff_ffff_ffff | sy
                    }
                    4 => {
                        (if ex == 0 { 1 } else { ex + 1 }.min(0x7fe)) << 52
                            | fy & 0x000f_ffff_ffff_ffff
                    }
                    5 => {
                        // Same exp, last-bit-different frac (strict
                        // order on the low-word compares).
                        (ex << 52) | (fx ^ 1)
                    }
                    6 => 0x7ff0_0000_0000_0000,
                    _ => 0x7ff0_0000_0000_0000 | 0x0008_0000_0000_0000 | fy & 0x0007_ffff_ffff_ffff,
                };
            c.g[A0][0] = x;
        }
        // Dispatcher shape: enter at the entry.
    }

    #[test]
    fn ieee754_fmod_matches() {
        check(
            ee::s15::f_0041ef50__generated,
            ieee754_fmod,
            154,
            &|c, seed| {
                fmod_setup(c, seed);
                // Dispatcher shape: enter at the entry.
                c.pc = 0x0041_ef50;
            },
        );
        // Events in the normalize/divide loops: unwinds resume at the
        // head they interrupted (counts per head in the log).
        check_events(fmod_gen_driven, fmod_hand_driven, 154, &|c, seed| {
            fmod_setup(c, seed);
            let mut r = Rng(0x0041_ef50 ^ seed as u64);
            let mode = schedule(c, &mut r, seed, 0x0041_ef50);
            c.next_event = c.cycles + 10 + (r.next_u32() % 200) as u64;
            mode
        });
    }

    #[test]
    fn ieee754_log10_matches() {
        check(
            ee::s15::f_0041f9d8__generated,
            ieee754_log10,
            SEEDS_6 + 5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0041_f9d8, 0);
                elf_image(c);
                let lo = r.next_u32() as u64 | (r.next_u32() as u64) << 32;
                c.g[A0][0] = edge_f64(&mut r, seed, lo);
                if seed >= SEEDS_6 {
                    // Extra inputs for the small-x path and boundary S0
                    // values.
                    const EXTRA: [u64; 5] = [
                        0x0000000000000001, // +subnormal: fa38 t-flip + tail
                        0x0001000000000000, // S0==0xFFFFF straddle
                        0x000fffff00000001, // S0==0xFFFFF boundary
                        0x7fefffff00000000, // S0==0x7FEFFFFF boundary
                        0x0000000100000001, // hx & 0x7fffffff == lx != 0 (the |x| == 0 test)
                    ];
                    c.g[A0][0] = EXTRA[(seed - SEEDS_6) as usize];
                }
            },
        );
    }

    pub(crate) const SEEDS_7: u32 = 64;

    /// New-format handler table with a matcher slot the test drives
    /// through the miss hook (`LOG_RESULTS` cells).
    pub(crate) fn find_table(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let base = DB + 0x400;
        // Header: matcher at +0x4 only on matcher seeds.
        c.bus.write32(base + 4, 0);
        // Range rows: [lo, hi, _, matcher-gate] starting at +0x8,
        // stepped by 0x10. Row 0 is out of range for the probe
        // (covers the advance), row 1 matches and its matcher gate
        // is set so the walk calls the miss-hook fake and finds.
        // NOTE: a 0xffffffff word reads back as 0xffff_ffff_ffff
        // (`lw` sign-extends), so it closes the scan at the first
        // close test instead of walking on.
        let row = base + 8;
        c.bus.write32(row, 0x1000);
        c.bus.write32(row + 4, 0x1100);
        c.bus.write32(row + 8, 0);
        c.bus.write32(row + 12, 0);
        c.bus.write32(row + 16, 0x1000);
        c.bus.write32(row + 20, 0x2000);
        c.bus.write32(row + 24, FAKE);
        c.bus.write32(row + 28, 1);
        // Sentinel row (matches the 0xffff0000|0xffff close test).
        c.bus.write32(row + 32, 0xffff_0000);
        c.bus.write32(row + 32 + 4, 0xffff_ffff);
        // The matcher-target gate (`lw(s4)`) must be callable.
        c.bus.write32(DB + 0x800, FAKE);
        let _ = r;
        let _ = seed;
    }

    /// Old-format handler table at `DB + 0x400`: entries start at the
    /// table base (the scan reads 12-byte rows from `a1` itself): two
    /// covering ranges, then the -1 sentinel word that ends the scan.
    pub(crate) fn old_find_table(c: &mut Cpu, seed: u32) {
        let base = DB + 0x400;
        // Sentinel first: nothing to scan.
        c.bus
            .write32(base, if seed % 4 == 0 { 0xffff_ffff } else { 0x1000 });
        if seed % 4 != 0 {
            // Entry 0 covers [0x1000, 0x2000), handler 0xabcd.
            c.bus.write32(base + 4, 0x2000);
            c.bus.write32(base + 8, 0xabcd);
            // Entry 1 covers [0x3000, 0x4000), handler 0xdcba.
            c.bus.write32(base + 12, 0x3000);
            c.bus.write32(base + 16, 0x4000);
            c.bus.write32(base + 20, 0xdcba);
            // Sentinel row ends the scan.
            c.bus.write32(base + 24, 0xffff_ffff);
        }
    }

    /// Register-save area the `get_reg_addr` scan walks: entry tags at
    /// +0x29c admit regs 0-3 into the class-2 walk, slot chains at +0x18
    /// resolve every admitted reg to `base + 0x700` (reg 2 takes the loop
    /// back edge on the way), and the scratch buffers there are pre-filled
    /// so a missing or extra store shows.
    pub(crate) fn reg_save_area(c: &mut Cpu, seed: u32, base: u32) -> u32 {
        // Walk chain: slot 0 resolves at once (regs 0 and 1 land here),
        // slots 2 -> 3 resolve (reg 2 takes the back edge between them,
        // reg 3 joins at slot 3). Low words are the next tag index.
        for (slot, next) in [(0u32, 5u32), (2, 3), (3, 4)] {
            c.bus.write32(base + 0x18 + slot * 8, next);
            c.bus.write32(base + 0x18 + slot * 8 + 4, 0);
        }
        // Entry tags (reg == tag byte admits to the class-2 walk) and the
        // chain tags: 2 continues the walk, 1 resolves the address.
        for (i, tag) in [2u8, 2, 2, 2, 1, 1].iter().enumerate() {
            c.bus.write8(base + 0x29c + i as u32, *tag);
        }
        // Resolve words: every chain ends at `base + 0x700`.
        c.bus.write32(base, base + 0x700);
        c.bus.write32(base + 0x38, 0);
        c.bus.write32(base + 0x58, 0);
        // Scratch buffers, pre-filled so a missing or extra store shows.
        for i in 0..8u32 {
            c.bus
                .write32(base + 0x700 + i * 4, 0xA11C_0000 | (seed << 8) | i);
            c.bus
                .write32(base + 0x780 + i * 4, 0xBEAD_0000 | (seed << 8) | i);
        }
        base
    }

    /// Save area for `throw_helper`: register tags at +0x29a and a
    /// frame-state block the (generated) resolver walks.
    pub(crate) fn throw_save_area(c: &mut Cpu, seed: u32) -> u32 {
        let base = DB + 0x400;
        c.bus
            .write16(base + 0x29a, if seed % 2 == 0 { 0 } else { 1 });
        c.bus.write16(base + 0x29c, 0x0101);
        c.bus
            .write32(base + 0x298, if seed % 2 == 0 { 0 } else { 1 });
        // Frame-state block: zeroed (resolver returns 0 on seed 0).
        base
    }

    /// Exception object on the real heap (the delete path frees it).
    pub(crate) fn heap_obj(c: &mut Cpu) -> u32 {
        c.g[A0][0] = 0x20;
        ee::s15::malloc(c);
        c.g[V0][0] as u32
    }

    /// ULEB128 bytes at the working base: 1-3 byte sequences plus the 5-byte edge.
    pub(crate) fn uleb_bytes(c: &mut Cpu, r: &mut Rng, seed: u32) {
        match seed % 5 {
            0 => {
                c.bus.write8(DB, (r.next_u32() & 0x7f) as u8);
            }
            1 => {
                c.bus.write8(DB, 0x80 | (r.next_u32() & 0x7f) as u8);
                c.bus.write8(DB + 1, (r.next_u32() & 0x7f) as u8);
            }
            2 => {
                c.bus.write8(DB, 0x80);
                c.bus.write8(DB + 1, 0x80);
                c.bus.write8(DB + 2, (r.next_u32() & 0x7f) as u8);
            }
            3 => {
                c.bus.write8(DB, 0xff);
                c.bus.write8(DB + 1, 0xff);
                c.bus.write8(DB + 2, 0xff);
                c.bus.write8(DB + 3, 0xff);
                c.bus.write8(DB + 4, 0x07);
            }
            _ => {
                for i in 0..6 {
                    c.bus.write8(DB + i, r.next_u32() as u8);
                }
                c.bus.write8(DB + 5, (r.next_u32() & 0x7f) as u8);
            }
        }
    }

    /// Merge-table setup: entry counts steered small (0 skips the loop),
    /// tables filled with random words.
    pub(crate) fn fde_merge_setup(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let dst = DB + 0x100;
        let src = DB + 0x200;
        let t8 = (seed % 3) as u32;
        let s0 = (seed / 3 % 3) as u32;
        c.bus.write32(src + 4, t8);
        c.bus.write32(dst + 4, s0);
        c.bus.write32(src, DB + 0x300);
        c.bus.write32(dst, DB + 0x400);
        for i in 0..16 {
            c.bus.write32(DB + 0x300 + i * 4, r.next_u32());
            c.bus.write32(DB + 0x400 + i * 4, r.next_u32());
        }
        c.g[A0][0] = dst as u64;
        c.g[A1][0] = src as u64;
    }

    /// Two type-name strings (same or different by seed) with object
    /// headers pointing at them.
    pub(crate) fn type_info_objs(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let n0 = DB + 0x500;
        let n1 = DB + 0x600;
        for (i, b) in [b"TestType\0", b"OtherTyq\0"][seed as usize % 2]
            .iter()
            .enumerate()
        {
            c.bus.write8(n0 + i as u32, *b);
        }
        for (i, b) in b"TestType\0".iter().enumerate() {
            c.bus.write8(n1 + i as u32, *b);
        }
        c.bus.write32(DB + 0x400, n0);
        c.bus.write32(DB + 0x410, n1);
        let _ = r;
    }

    #[test]
    fn new_eh_context_matches() {
        // Small request: the real allocator serves it from the test heap.
        // The null arm (malloc returns 0) is not reachable by any
        // fixture: malloc has no override slot to fake, and in the
        // harness it serves 0x18 from zeroed bins without ever calling
        // `_sbrk_r` (an always-panicking hook on that slot never fires,
        // and emptying every bin head changes nothing).
        check(
            ee::s15::f_00410410__generated,
            new_eh_context,
            SEEDS_7,
            &|c, seed| {
                let _ = machine(c, seed, 0x0041_0410, 0);
                c.g[GP][0] = GAME_GP as u64;
            },
        );
    }

    #[test]
    fn old_find_exception_handler_matches() {
        check(
            ee::s15::f_00410858__generated,
            old_find_exception_handler,
            SEEDS_7 + 8,
            &|c, seed| {
                let _ = machine(c, seed, 0x0041_0858, 0);
                old_find_table(c, seed);
                c.g[A1][0] = if seed % 5 == 4 {
                    0
                } else {
                    (DB + 0x400) as u64
                };
                c.g[A0][0] = match seed % 4 {
                    1 => 0x1500,
                    2 => 0x3500,
                    _ => 0x2500,
                };
                if seed >= SEEDS_7 {
                    // Edge tables for pc 0x1500 (rows of start, end,
                    // handler): range bounds equal to the pc, and the
                    // choice between two covering rows.
                    let rows: &[[u32; 3]] = match seed - SEEDS_7 {
                        // Start == pc: covered.
                        0 => &[[0x1500, 0x2000, 0x1111]],
                        // End == pc: not covered.
                        1 => &[[0x1000, 0x1500, 0x2222]],
                        // A row starting past the pc ends the scan before
                        // the narrower last row.
                        2 => &[
                            [0x1000, 0x2000, 0xabcd],
                            [0x3000, 0x4000, 0xdcba],
                            [0x1400, 0x1800, 0x3333],
                        ],
                        // An empty row at the pc does not end it.
                        3 => &[
                            [0x1000, 0x2000, 0xabcd],
                            [0x1500, 0x1500, 0x4444],
                            [0x1400, 0x1800, 0x3333],
                        ],
                        // Narrower second row wins.
                        4 => &[[0x1000, 0x2000, 0xabcd], [0x1400, 0x1800, 0x3333]],
                        // Second row ends later: the first stays.
                        5 => &[[0x1000, 0x1800, 0xabcd], [0x1400, 0x2000, 0x3333]],
                        // Equal ends, later start: the second wins.
                        6 => &[[0x1000, 0x1800, 0xabcd], [0x1400, 0x1800, 0x3333]],
                        // Equal starts, earlier end: the second wins.
                        _ => &[[0x1400, 0x1800, 0xabcd], [0x1400, 0x1700, 0x3333]],
                    };
                    let base = DB + 0x400;
                    for (i, row) in rows.iter().enumerate() {
                        for (j, word) in row.iter().enumerate() {
                            c.bus.write32(base + 12 * i as u32 + 4 * j as u32, *word);
                        }
                    }
                    c.bus.write32(base + 12 * rows.len() as u32, 0xffff_ffff);
                    c.g[A1][0] = base as u64;
                    c.g[A0][0] = 0x1500;
                }
            },
        );
    }

    crate::difftest::events::driven!(old_find_gen_driven, ee::s15::f_00410858__generated);

    crate::difftest::events::driven!(old_find_hand_driven, old_find_exception_handler);

    /// `old_find_exception_handler` with events during a long scan: the
    /// 0x410890 back edge delivers a due event before the next row.
    #[test]
    fn old_find_exception_handler_events_match() {
        check_events(old_find_gen_driven, old_find_hand_driven, 48, &|c, seed| {
            let mut r = machine(c, seed, 0x0041_0858, 0);
            let mode = schedule(c, &mut r, seed, 0x0041_0858);
            // Rows [0x100 i, 0x100 i + 0x80) below the pc, so the scan
            // walks all of them; one wider row covers the pc.
            let base = DB + 0x400;
            let rows = 4 + r.next_u32() % 24;
            let hit = r.next_u32() % rows;
            for i in 0..rows {
                let (lo, hi) = if i == hit {
                    (0x100 * i, 0x8000)
                } else {
                    (0x100 * i, 0x100 * i + 0x80)
                };
                let row = base + 12 * i;
                c.bus.write32(row, lo);
                c.bus.write32(row + 4, hi);
                c.bus.write32(row + 8, 0x1000 + i);
            }
            c.bus.write32(base + 12 * rows, 0xffff_ffff);
            c.g[A1][0] = base as u64;
            c.g[A0][0] = 0x4000;
            c.next_event = c.cycles + 20 + (r.next_u32() % 200) as u64;
            mode
        });
    }

    #[test]
    fn find_exception_handler_matches() {
        check(
            ee::s15::f_00410930__generated,
            find_exception_handler,
            SEEDS_7 + 4,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0041_0930, 0);
                find_table(c, &mut r, seed);
                c.g[A1][0] = if seed % 5 == 4 {
                    0
                } else {
                    (DB + 0x400) as u64
                };
                if seed % 5 == 3 {
                    // Non-null table whose scan exits at the first
                    // close test with no matcher call and no later
                    // `sw(t0, 0)`: the entry `sw(t0, 1)` (a delay slot in
                    // generated, both arms) is the only store to `t0`, so
                    // a pre-filled buffer discriminates it.
                    c.bus.write32(DB + 0x400 + 8, 0xffff_ffff);
                    c.g[A3][0] = 0;
                    c.g[T0][0] = (DB + 0x900) as u64;
                    c.bus.write32(DB + 0x900, 0x2222_2222);
                }
                // Always in range of row 1, so the walk ends at a match
                // or the sentinel row. A3 is off here (A0 is a pc, not a
                // row pointer); seeds 66-67 below turn it on.
                c.g[A0][0] = 0x1500;
                c.g[A2][0] = (DB + 0x800) as u64;
                c.g[A3][0] = 0;
                if seed == 65 {
                    // S1 == row-bound edge (S1 is 0x14ff here): rows 0 and
                    // 1 both see lo equality, row 0 also sees hi equality,
                    // and the per-row range test sees equality too. (Seed
                    // 64 would not do: 64 % 5 == 4 zeroes A1 and the walk
                    // returns before the rows loop. Row 0's lo word can
                    // change: the unaligned read before the loop stays
                    // 32-bit and never equals the all-ones V1.)
                    c.bus.write32(DB + 0x408, 0x14ff);
                    c.bus.write32(DB + 0x40c, 0x14ff);
                    c.bus.write32(DB + 0x418, 0x14ff);
                }
                if seed >= 66 {
                    // Rethrow (A3 != 0): A0 points at a table row, the
                    // scan restarts at that row's end - 1, one row on
                    // unless the row is the -1 end-of-table entry. Rows
                    // 0 and 2 are catch-all covers with distinct handlers,
                    // row 3 closes the scan. Seed 66 rethrows from the
                    // end-of-table row 1 (generated returns 0, any skip
                    // past it finds 0xbbbb); seed 67 from a normal row 1
                    // (generated finds 0xbbbb in row 2; no skip finds row
                    // 1's 0xaaaa).
                    let t0 = DB + 0x408;
                    for (i, row) in [
                        [0x1000, 0x2000, 0xcccc, 0],
                        if seed == 66 {
                            [0xffff_ffff, 0x1600, 0, 0]
                        } else {
                            [0x1000, 0x1600, 0xaaaa, 0]
                        },
                        [0x1000, 0x2000, 0xbbbb, 0],
                        [0xffff_ffff, 0, 0, 0],
                    ]
                    .iter()
                    .enumerate()
                    {
                        for (j, w) in row.iter().enumerate() {
                            c.bus.write32(t0 + 16 * i as u32 + 4 * j as u32, *w);
                        }
                    }
                    c.g[A0][0] = (t0 + 0x10) as u64;
                    c.g[A3][0] = 1;
                }
            },
        );
    }

    #[test]
    fn get_reg_addr_matches() {
        check(
            ee::s15::f_00410a78__generated,
            get_reg_addr,
            SEEDS_7,
            &|c, seed| {
                let _ = machine(c, seed, 0x0041_0a78, 0);
                let base = reg_save_area(c, seed, DB + 0x400);
                c.g[A1][0] = base as u64;
                c.g[A0][0] = (seed % 5) as u64;
            },
        );
    }

    /// Gate word for the `memcpy` entry probe: `COPY_REG_MAGIC` records
    /// V0 at entry (then runs the slot body), anything else just runs it.
    /// The copy_reg size select leaves V0 behind for `memcpy` to clobber,
    /// so a wrong V0 there is only visible at that entry.
    pub(crate) const COPY_REG_PROBE: u32 = DB + 0xf00;
    pub(crate) const COPY_REG_MAGIC: u32 = 0xc09e_0001;
    pub(crate) const COPY_REG_V0: u32 = DB + 0xf04;

    static MEMCPY_PREV: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

    /// `memcpy` that snapshots the entry V0 for `copy_reg_matches`.
    fn fake_memcpy_probe(c: &mut Cpu) {
        if c.bus.read32(COPY_REG_PROBE) == COPY_REG_MAGIC {
            c.bus.write32(COPY_REG_V0, c.g[V0][0] as u32);
        }
        crate::difftest::fakes::fall_back(c, &MEMCPY_PREV, ee::s15::f_0041605c__generated);
    }

    pub(crate) fn install_memcpy_probe() {
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_0041605C),
            fake_memcpy_probe,
            &MEMCPY_PREV,
        );
    }

    #[test]
    fn copy_reg_matches() {
        // Both areas are working save areas: the two `get_reg_addr`
        // calls resolve into the pre-filled buffers, so the size
        // selection, the real `memcpy` and the epilogue all run. The
        // entry probe above snapshots V0 at `memcpy` (the size-select
        // compare, 1 for registers below 31).
        install_memcpy_probe();
        check(
            ee::s15::f_00410b00__generated,
            copy_reg,
            SEEDS_7 + 8,
            &|c, seed| {
                let _ = machine(c, seed, 0x0041_0b00, 0);
                let base = reg_save_area(c, seed, DB + 0x400);
                reg_save_area(c, seed, DB + 0x600);
                c.g[A1][0] = base as u64;
                c.g[A0][0] = (seed % 3) as u64;
                c.g[A2][0] = (DB + 0x600) as u64;
                c.bus.write32(COPY_REG_PROBE, COPY_REG_MAGIC);
                if seed >= SEEDS_7 {
                    // Registers 31 and up take the size select: 4 bytes
                    // for 31-63, 8 above 63. Areas far enough apart that
                    // a high register's slot word stays inside its own.
                    let reg = [31, 32, 47, 62, 63, 64, 70, 78][(seed - SEEDS_7) as usize];
                    let (src, dst) = (DB + 0x1000, DB + 0x2000);
                    for area in [src, dst] {
                        reg_save_area(c, seed, area);
                        // Class 1: resolves at once to `area + 0x700`.
                        c.bus.write8(area + 0x29c + reg, 1);
                        c.bus.write32(area + 0x18 + reg * 8, 0);
                    }
                    c.g[A0][0] = reg as u64;
                    c.g[A1][0] = src as u64;
                    c.g[A2][0] = dst as u64;
                }
            },
        );
    }

    #[test]
    fn next_stack_level_matches() {
        // Even seeds: no frame state (the 61-cycle null arm). Odd seeds:
        // the registry covers the query pc, so `frame_state_for` returns
        // the save block and the level advance runs — through
        // `get_reg_addr` (758 cycles) when the entry tag admits the
        // cursor, through the word shuffle (795 cycles) when the tag is
        // cleared.
        crate::difftest::check_paths(
            ee::s15::f_00410b80__generated,
            next_stack_level,
            SEEDS_7,
            &[61, 758, 795],
            &|c, seed| {
                let mut r = machine(c, seed, 0x0041_0b80, 0);
                let base = reg_save_area(c, seed, DB + 0x1400);
                if seed % 2 == 1 {
                    fill(c, &mut r, DB + 0x400, 32);
                    fill(c, &mut r, DB + 0x500, 8);
                    fill(c, &mut r, DB + 0x600, 16);
                    fill(c, &mut r, DB + 0x800, 64);
                    c.bus.write32(DB + 0x400, 1);
                    c.bus.write32(DB + 0x404, 0x200000);
                    c.bus.write32(DB + 0x40c, DB + 0x500);
                    c.bus.write32(DB + 0x410, 1);
                    c.bus.write32(DB + 0x414, 0);
                    c.bus.write32(0x0052ade8, DB + 0x400);
                    c.bus.write32(DB + 0x500, DB + 0x600);
                    c.bus.write32(DB + 0x600, 0);
                    c.bus.write32(DB + 0x604, (DB + 0x600).wrapping_sub(0x100));
                    c.bus.write32(DB + 0x608, 0x180000);
                    c.bus.write32(DB + 0x60c, 0x100);
                    // Cursor range the level advance reads (`S0 + 0x298`):
                    // zeroed, so the search arm takes reg 0. On shuffle
                    // seeds the entry tag is cleared instead, steering the
                    // word shuffle.
                    for i in 0..128u32 {
                        c.bus.write32(DB + 0xa00 + i * 4, 0);
                    }
                    if seed / 2 % 2 == 1 {
                        c.bus.write8(base + 0x29c, 0);
                    }
                    c.g[A0][0] = 0x180000;
                }
                c.g[A1][0] = base as u64;
                c.g[A2][0] = (DB + 0x800) as u64;
            },
        );
    }

    /// Gate + count words for the throw scan's frame-chain fake.
    pub(crate) const THROW_NSL_GATE: u32 = DB + 0xe00;
    pub(crate) const THROW_NSL_MAGIC: u32 = 0xc07e_0001;
    pub(crate) const THROW_NSL_COUNT: u32 = DB + 0xe04;
    /// Frame the fake scan returns on the first call; later calls return
    /// 0 (end of chain). Callee bodies downstream stay real, so the
    /// caller's argument setup still decides the result.
    pub(crate) const THROW_FRAME: u32 = DB + 0x600;

    static THROW_NSL_PREV: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

    /// Second frame of the scan chain (old-table hit on the second pass).
    pub(crate) const THROW_FRAME_B: u32 = DB + 0x1000;

    /// `next_stack_level` walking a one-frame test chain: the second
    /// call returns 0 (scan end); every other call returns the controlled
    /// frame (the save loop reuses it and terminates on its tags). Falls
    /// back to the generated body when the gate is off (every other test).
    fn fake_throw_nsl(c: &mut Cpu) {
        if c.bus.read32(THROW_NSL_GATE) != THROW_NSL_MAGIC {
            crate::difftest::fakes::fall_back(c, &THROW_NSL_PREV, ee::s15::f_00410b80__generated);
            return;
        }
        let n = c.bus.read32(THROW_NSL_COUNT);
        c.bus.write32(THROW_NSL_COUNT, n + 1);
        c.g[V0][0] = match n {
            0 => THROW_FRAME as u64,
            1 => THROW_FRAME_B as u64,
            _ => 0,
        };
        c.cycles += 5;
        c.pc = c.g[RA][0] as u32;
    }

    pub(crate) fn install_throw_nsl() {
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_00410B80),
            fake_throw_nsl,
            &THROW_NSL_PREV,
        );
    }

    /// New-format table the first pass misses: the probe lies outside
    /// both rows, so the scan reaches the sentinel with the 0x5e0 flag
    /// still set (no matcher runs to clear it). The base word doubles
    /// as the masked-kind `== mask` word that steers the dispatch into
    /// the second search (the scan itself starts at +8).
    pub(crate) fn throw_new_miss_table(c: &mut Cpu, seed: u32) {
        let mut dummy = Rng(seed as u64);
        find_table(c, &mut dummy, seed);
        c.bus.write32(DB + 0x400, 0xffff_fffe);
    }

    /// Old-format table the second pass hits: entry 0 covers the d94
    /// result, so the old-table search returns a handler with the 0x5e0
    /// flag still set from the first pass. Same layout as
    /// `old_find_table` (proven by its own test), at a base clear of
    /// the new table, both frames and the gate words.
    pub(crate) fn throw_old_hit_table(c: &mut Cpu) {
        let base = DB + 0x1800;
        c.bus.write32(base, 0x1000);
        c.bus.write32(base + 4, 0x2000);
        c.bus.write32(base + 8, 0xabcd);
        c.bus.write32(base + 12, 0x3000);
        c.bus.write32(base + 16, 0x4000);
        c.bus.write32(base + 20, 0xdcba);
        c.bus.write32(base + 24, 0xffff_ffff);
    }

    /// Two-frame scan chain. The first pass runs the second new-table
    /// search and misses (leaving the 0x5e0 flag set); the second pass
    /// runs the old-table search and hits, so the 0xd0c flag test takes
    /// the 0xd14 commit path (either search alone would miss the flag
    /// or the handler). `seed % 4` steers null/table and the second
    /// pass: misses unwind through `__terminate`, hits run commit,
    /// copy and the context advance to a normal return through the
    /// 0xe60 direct edge (the second `get_reg_addr` step reproduces
    /// the entry level, so the context is live; the save loop is not
    /// entered). `[a0 + 0xc]` is 0, so S6 = 0 and the 0xcac search is
    /// skipped; `throw_helper_matches` sets it on its extra seeds.
    pub(crate) fn throw_scan_chain(c: &mut Cpu, seed: u32) {
        // Exception object: zero matcher context (S7) and zero `+0xc`
        // (S6 = 0: the second-search and old-table arms).
        let ex = DB + 0x100;
        c.bus.write32(ex + 8, 0);
        c.bus.write32(ex + 12, 0);
        // First frame: masked path into the second new-table search,
        // which misses (the probe is outside its single row).
        reg_save_area(c, seed, THROW_FRAME);
        c.bus
            .write32(THROW_FRAME + 4, if seed % 4 == 0 { 0 } else { DB + 0x400 });
        throw_new_miss_table(c, seed);
        // Second frame: masked path (or null) into the old-table search,
        // which hits row 0 on hit seeds.
        reg_save_area(c, seed, THROW_FRAME_B);
        c.bus.write32(
            THROW_FRAME_B + 4,
            if seed % 4 >= 2 { DB + 0x1800 } else { 0 },
        );
        // Old-format table the second pass hits (same layout as
        // `old_find_table`, proven by its own test).
        throw_old_hit_table(c);
        // Scratch resolve words: the first step returns 0x1500 (the
        // second pass's probe), the second returns the entry level for
        // the direct edge below.
        c.bus.write32(THROW_FRAME + 0x700, 0x1501);
        c.bus.write32(THROW_FRAME_B + 0x700, 0x5001);
        // Out-of-range probe for the first pass; doubles as the
        // direct-edge level after the second pass resolves it.
        c.g[A1][0] = 0x5000;
        c.g[A0][0] = ex as u64;
        c.g[A2][0] = (DB + 0x800) as u64;
        c.g[A3][0] = (DB + 0x200) as u64;
        c.bus.write32(DB + 0x200, 0);
        c.bus.write32(THROW_NSL_GATE, THROW_NSL_MAGIC);
        c.bus.write32(THROW_NSL_COUNT, 0);
    }

    #[test]
    fn throw_helper_matches() {
        install_throw_nsl();
        check(
            ee::s15::f_00410c18__generated,
            throw_helper,
            SEEDS_3 + 12,
            &|c, seed| {
                let _ = machine(c, seed, 0x0041_0c18, 0);
                throw_scan_chain(c, seed);
                if seed >= SEEDS_3 {
                    // Each variant over the four chain shapes (`seed % 4`).
                    match (seed - SEEDS_3) / 4 {
                        // A nonzero `[a0 + 0xc]`: S6 = 1 takes the 0xcac
                        // new-table search, which clears the word.
                        0 => c.bus.write32(DB + 0x100 + 12, 0x1500),
                        1 => c.bus.write32(DB + 0x100 + 12, 0x5000),
                        // Kind word -1 (`lw` sign-extends): the masked
                        // test is false, so the first pass takes the
                        // old-table search.
                        _ => c.bus.write32(DB + 0x400, 0xffff_ffff),
                    }
                }
                // Dispatcher shape: enter at the entry.
                c.pc = 0x0041_0c18;
            },
        );
    }

    crate::difftest::events::driven!(throw_gen_driven, ee::s15::f_00410c18__generated);

    crate::difftest::events::driven!(throw_hand_driven, throw_helper);

    #[test]
    fn throw_helper_scan_events() {
        // Two-frame chain, hits only: misses unwind through
        // `__terminate`, which the default-profile drive cannot resume;
        // hits return normally through the direct edge. Unwinds land on
        // the scan head and, on later first events, inside the copy
        // loop. Resume counts per head below.
        install_throw_nsl();
        check_events(throw_gen_driven, throw_hand_driven, SEEDS_3, &|c, seed| {
            let _ = machine(c, seed, 0x0041_0c18, 0);
            throw_scan_chain(c, seed | 2);
            let mut r = Rng(0x0041_0c18 ^ seed as u64);
            let mode = schedule(c, &mut r, seed, 0x0041_0c18);
            // Direct hits run ~3300 cycles with the copy loop near the
            // end (the first search walks far); spread the first
            // deadline from the scan head into the copy loop by seed
            // (a deadline falling inside a callee loop is delivered
            // there without unwinding).
            c.next_event = c.cycles
                + [30, 3000, 3100, 3200][(seed / 4 % 4) as usize]
                + (r.next_u32() % 10) as u64;
            mode
        });
    }

    #[test]
    fn eh_context_initialize_matches() {
        check(
            ee::s15::f_004104b8__generated,
            eh_context_initialize,
            SEEDS_7,
            &|c, seed| {
                let _ = machine(c, seed, 0x0041_04b8, 0);
                c.g[GP][0] = GAME_GP as u64;
            },
        );
    }

    #[test]
    fn eh_context_static_matches() {
        check(
            ee::s15::f_004104e0__generated,
            eh_context_static,
            SEEDS_7,
            &|c, seed| {
                let _ = machine(c, seed, 0x0041_04e0, 0);
                // First-use zeroing vs the already-initialized block.
                c.bus
                    .write32(0x0052_adb0, if seed % 2 == 0 { 0 } else { DB + 0x300 });
            },
        );
    }

    #[test]
    fn terminate_cpp_matches() {
        // The slot handler + `set_terminate` are both real; the slot's
        // miss-hook fake returns on the first call and unwinds on the
        // re-entry from `set_terminate`'s inner `terminate` (the real
        // handler never returns, and a returning fake would recurse
        // forever). Both sides unwind identically every seed.
        covered(
            ee::s15::f_00412800__generated,
            terminate_cpp,
            SEEDS_7,
            &|c, seed| {
                machine(c, seed, 0x0041_2800, 0);
                c.bus.write32(LOG_UNWIND_AT, 1);
            },
        );
    }

    #[test]
    fn set_unexpected_matches() {
        check(
            ee::s15::f_00412838__generated,
            set_unexpected,
            SEEDS_7,
            &|c, seed| {
                let _ = machine(c, seed, 0x0041_2838, 0);
                c.g[GP][0] = GAME_GP as u64;
            },
        );
    }

    #[test]
    fn uncaught_exception_matches() {
        check(
            ee::s15::f_00412dd8__generated,
            uncaught_exception,
            SEEDS_7,
            &|c, seed| {
                let _ = machine(c, seed, 0x0041_2dd8, 0);
                c.g[GP][0] = GAME_GP as u64;
                if seed % 2 == 1 {
                    // Steer the context getter's fake at a block whose +8
                    // points at N, so the exception-count check below runs
                    // with both outcomes over seeds (N+0x14 is 0 or 1).
                    c.bus.write32(DB + 0x508, DB + 0x600);
                    c.bus.write32(DB + 0x614, (seed / 2 % 2) as u32);
                    c.bus.write32(LOG_RESULTS, DB + 0x500);
                }
            },
        );
    }

    #[test]
    fn exception_what_matches() {
        check(
            ee::s15::f_00412e10__generated,
            exception_what,
            SEEDS_7,
            &|c, seed| {
                machine(c, seed, 0x0041_2e10, 0);
                let obj = DB + 0x400;
                let vt = DB + 0x500;
                c.bus.write32(obj, vt);
                c.bus.write32(vt + 4, FAKE);
                if seed % 2 == 0 {
                    // The fake returns this pointer; `what` loads through it.
                    c.bus.write32(LOG_RESULTS, DB + 0x600);
                    c.bus.write32(DB + 0x600, 0x1234_5678);
                }
                c.g[A0][0] = obj as u64;
            },
        );
    }

    #[test]
    fn bad_exception_dtor_matches() {
        check(
            ee::s15::f_00412e38__generated,
            bad_exception_dtor,
            SEEDS_7,
            &|c, seed| {
                machine(c, seed, 0x0041_2e38, 0);
                c.g[A0][0] = heap_obj(c) as u64;
                // `a1 & 1`: the `operator delete` path or the plain store.
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn bad_exception_ctor_matches() {
        check(
            ee::s15::f_00412e68__generated,
            bad_exception_ctor,
            SEEDS_7,
            &|c, seed| {
                machine(c, seed, 0x0041_2e68, 0);
                c.g[A0][0] = heap_obj(c) as u64;
            },
        );
    }

    #[test]
    fn exception_dtor_matches() {
        check(
            ee::s15::f_00412ed0__generated,
            exception_dtor,
            SEEDS_7,
            &|c, seed| {
                machine(c, seed, 0x0041_2ed0, 0);
                c.g[A0][0] = heap_obj(c) as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn exception_ctor_matches() {
        check(
            ee::s15::f_00412f00__generated,
            exception_ctor,
            SEEDS_7,
            &|c, seed| {
                machine(c, seed, 0x0041_2f00, 0);
                c.g[A0][0] = heap_obj(c) as u64;
            },
        );
    }

    #[test]
    fn decode_uleb128_matches() {
        check(
            ee::s15::f_00413b38__generated,
            decode_uleb128,
            SEEDS_7,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0041_3b38, 0);
                uleb_bytes(c, &mut r, seed);
                c.bus.write32(DB + 0x40, 0xaaaa_aaaa);
                c.g[A0][0] = DB as u64;
                c.g[A1][0] = (DB + 0x40) as u64;
            },
        );
    }

    #[test]
    fn decode_sleb128_matches() {
        check(
            ee::s15::f_00413b80__generated,
            decode_sleb128,
            SEEDS_7,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0041_3b80, 0);
                uleb_bytes(c, &mut r, seed);
                c.bus.write32(DB + 0x40, 0xaaaa_aaaa);
                c.g[A0][0] = DB as u64;
                c.g[A1][0] = (DB + 0x40) as u64;
            },
        );
    }

    #[test]
    fn type_info_dtor_matches() {
        check(
            ee::s15::f_00414b68__generated,
            type_info_dtor,
            SEEDS_7,
            &|c, seed| {
                machine(c, seed, 0x0041_4b68, 0);
                c.g[A0][0] = heap_obj(c) as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn type_info_eq_matches() {
        check(
            ee::s15::f_00414b98__generated,
            type_info_eq,
            SEEDS_7,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0041_4b98, 0);
                type_info_objs(c, &mut r, seed);
                // Pointer-equal objects, or two headers (names same/different).
                c.g[A0][0] = (DB + 0x400) as u64;
                c.g[A1][0] = if seed % 3 == 0 {
                    (DB + 0x400) as u64
                } else {
                    (DB + 0x410) as u64
                };
            },
        );
    }

    #[test]
    fn type_info_ne_matches() {
        check(
            ee::s15::f_004151b8__generated,
            type_info_ne,
            SEEDS_7,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0041_51b8, 0);
                type_info_objs(c, &mut r, seed);
                c.g[A0][0] = (DB + 0x400) as u64;
                c.g[A1][0] = if seed % 3 == 0 {
                    (DB + 0x400) as u64
                } else {
                    (DB + 0x410) as u64
                };
            },
        );
    }

    crate::difftest::events::driven!(count_fdes_gen_driven, ee::s15::f_00413f38__generated);

    crate::difftest::events::driven!(count_fdes_hand_driven, count_fdes);

    #[test]
    fn count_fdes_matches() {
        check(
            ee::s15::f_00413f38__generated,
            count_fdes,
            SEEDS_7 + 1,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0041_3f38, 0);
                fde_chain(c, &mut r, seed, DB + 0x400);
                c.g[A0][0] = (DB + 0x400) as u64;
                if seed >= SEEDS_7 {
                    // A length-prefixed FDE array so the count loop takes
                    // its back edge. Seeds 0-63 untouched. First
                    // iteration uses L=0x10 with nonzero count/valid
                    // words and takes the back edge on [+0x14] != 0;
                    // the second iteration terminates on [+0x28] == 0.
                    c.bus.write32(DB + 0x400, 0x10);
                    c.bus.write32(DB + 0x404, 1);
                    c.bus.write32(DB + 0x408, 1);
                    c.bus.write32(DB + 0x410, 0x10);
                    c.bus.write32(DB + 0x414, 0x10);
                    c.bus.write32(DB + 0x418, 1);
                    c.bus.write32(DB + 0x41c, 1);
                    c.bus.write32(DB + 0x428, 0);
                }
                c.pc = 0x0041_3f38;
            },
        );
        // Events in the count loop: unwinds resume at its head.
        check_events(
            count_fdes_gen_driven,
            count_fdes_hand_driven,
            32,
            &|c, seed| {
                let _ = machine(c, seed, 0x0041_3f38, 0);
                // A looping FDE layout on every seed.
                c.bus.write32(DB + 0x400, 0x10);
                c.bus.write32(DB + 0x404, 1);
                c.bus.write32(DB + 0x408, 1);
                c.bus.write32(DB + 0x410, 0x10);
                c.bus.write32(DB + 0x414, 0x10);
                c.bus.write32(DB + 0x418, 1);
                c.bus.write32(DB + 0x41c, 1);
                c.bus.write32(DB + 0x428, 0);
                c.g[A0][0] = (DB + 0x400) as u64;
                let mut r = Rng(0x0041_3f38 ^ seed as u64);
                let mode = schedule(c, &mut r, seed, 0x0041_3f38);
                // Two-iteration loop: fire at once so the head unwinds.
                c.next_event = c.cycles + (r.next_u32() % 8) as u64;
                mode
            },
        );
    }

    #[test]
    fn frame_init_matches() {
        check(
            ee::s15::f_00414008__generated,
            frame_init,
            SEEDS_7 + 3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0041_4008, 0);
                fill(c, &mut r, DB + 0x400, 16);
                c.bus.write32(DB + 0x400, (DB + 0x500) as u32);
                c.bus.write32(DB + 0x40c, (seed % 3) as u32);
                c.g[A0][0] = (DB + 0x400) as u64;
                c.g[A3][0] = r.next_u32() as u64;
                if seed >= SEEDS_7 {
                    // A real table pointer in S3+12 so the COUNT_FDES /
                    // ADD_FDES loops run. Seeds 0-63 untouched. Seed 64:
                    // table [E1, E2, 0] with nonzero-count FDE entries.
                    // Seed 65: table [0] plus a 0x100-node chain at E1
                    // (length 8, count/valid nonzero, zero word after),
                    // so COUNT_FDES returns 0x100 and the scaled size
                    // (0x400 vs 0x40) straddles an allocator size class.
                    // The chain walk is bounded (back-edge unwind checks).
                    let e1 = DB + 0x600;
                    if (seed - SEEDS_7) == 0 {
                        let e2 = DB + 0x620;
                        for (e, term) in [(e1, DB + 0x614), (e2, DB + 0x634)] {
                            c.bus.write32(e, 0x10);
                            c.bus.write32(e + 4, 1);
                            c.bus.write32(e + 8, 1);
                            c.bus.write32(term, 0);
                        }
                        c.bus.write32(DB + 0x40c, (DB + 0x500) as u32);
                        c.bus.write32(DB + 0x500, e1);
                        c.bus.write32(DB + 0x504, e2);
                        c.bus.write32(DB + 0x508, 0);
                    } else {
                        for k in 0..0x100u32 {
                            c.bus.write32(e1 + k * 12, 8);
                            c.bus.write32(e1 + k * 12 + 4, 1);
                            c.bus.write32(e1 + k * 12 + 8, 1);
                        }
                        c.bus.write32(e1 + 0x100 * 12, 0);
                        c.bus.write32(DB + 0x40c, (DB + 0x500) as u32);
                        c.bus.write32(DB + 0x500, 0);
                        c.bus.write32(DB + 0x408, e1);
                    }
                    if seed == 66 {
                        // `S3+0xc == 0` skips the count loop straight to
                        // the COUNT_FDES scale, with a copy of the looping
                        // FDE layout of `count_fdes_events_match` at `S3+8`,
                        // so the shift that scales the count is visible.
                        // Runs after the
                        // seed>=64 block, which otherwise overwrites
                        // DB+0x40c. (Pointing COUNT_FDES at the stride-12
                        // e1 table misparses it and runs away; the
                        // stride-0x14 shape below is the bounded one.)
                        c.bus.write32(DB + 0x40c, 0);
                        c.bus.write32(DB + 0x408, DB + 0x600);
                        c.bus.write32(DB + 0x600, 0x10);
                        c.bus.write32(DB + 0x604, 1);
                        c.bus.write32(DB + 0x608, 1);
                        c.bus.write32(DB + 0x60c, 0);
                        c.bus.write32(DB + 0x610, 0x10);
                        c.bus.write32(DB + 0x614, 0x10);
                        c.bus.write32(DB + 0x618, 1);
                        c.bus.write32(DB + 0x61c, 1);
                        c.bus.write32(DB + 0x620, 0);
                        c.bus.write32(DB + 0x624, 0);
                        c.bus.write32(DB + 0x628, 0);
                    }
                }
            },
        );
    }

    #[test]
    fn fde_merge_matches() {
        check(
            ee::s15::f_00413bd0__generated,
            fde_merge,
            SEEDS_7 + 3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0041_3bd0, 0);
                fde_merge_setup(c, &mut r, seed);
                if seed >= SEEDS_7 {
                    // Three seeds share one merge-table shape with
                    // different inner-loop outcomes: T6 reads src_table[0]
                    // and the outer V1 reads dst_table[1] (T3 is not
                    // decremented in the outer body), while the inner V1
                    // reads dst_table[0]; s0=3, t8=1. The entry
                    // difference is 20-10 > 0, so the merge runs on all
                    // three seeds; the inner difference is 0 (seed 64),
                    // -5 (seed 65) or +10 (seed 66). Seeds 0-63
                    // untouched. Bounded: the inner loop exits on T1==0
                    // after at most 3 iterations.
                    let dst = DB + 0x100;
                    let src = DB + 0x200;
                    let p = DB + 0x500;
                    let q1 = DB + 0x600;
                    let q0 = DB + 0x700;
                    c.bus.write32(src + 4, 1);
                    c.bus.write32(dst + 4, 3);
                    for i in 0..16u32 {
                        c.bus.write32(DB + 0x300 + i * 4, p);
                        c.bus.write32(DB + 0x400 + i * 4, q1);
                    }
                    c.bus.write32(DB + 0x400, q0);
                    c.bus.write32(p + 8, 10);
                    c.bus.write32(q1 + 8, 20);
                    c.bus.write32(
                        q0 + 8,
                        match seed - SEEDS_7 {
                            0 => 10,
                            1 => 5,
                            _ => 20,
                        },
                    );
                    c.bus.write32(DB + 0x2fc, q0);
                }
            },
        );
    }

    /// end_fde_sort fixture: merge tables, heap block, and the
    /// 40-entry table (mid-entries run with clamped bounds).
    /// Returns the entry pc for `seed`.
    pub(crate) fn end_fde_sort_setup(c: &mut Cpu, seed: u32) -> u32 {
        let mut r = machine(c, seed, 0x0041_3cd0, 0);
        // Merge-table layout: dst/src headers plus two 16-word tables.
        fill(c, &mut r, DB + 0x100, 16);
        fill(c, &mut r, DB + 0x200, 16);
        for i in 0..16 {
            c.bus.write32(DB + 0x300 + i * 4, r.next_u32());
            c.bus.write32(DB + 0x400 + i * 4, r.next_u32());
        }
        c.bus.write32(DB + 0x100, DB + 0x400);
        c.bus.write32(DB + 0x104, 1);
        c.bus.write32(DB + 0x200, DB + 0x300);
        c.bus.write32(DB + 0x204, 1);
        // A real block for the 0x3f08 FREE of lw(S1+8): a small
        // request keeps the allocator on its fast path.
        c.g[A0][0] = 0x20;
        ee::s15::malloc(c);
        c.bus.write32(DB + 0x108, c.g[V0][0] as u32);
        const ENTRIES: [u32; 40] = [
            0x0041_3cd0,
            0x0041_3cf4,
            0x0041_3cfc,
            0x0041_3d1c,
            0x0041_3d24,
            0x0041_3d30,
            0x0041_3d38,
            0x0041_3d44,
            0x0041_3d48,
            0x0041_3d58,
            0x0041_3d78,
            0x0041_3d90,
            0x0041_3da0,
            0x0041_3da4,
            0x0041_3da8,
            0x0041_3dc8,
            0x0041_3dd8,
            0x0041_3ddc,
            0x0041_3dec,
            0x0041_3df4,
            0x0041_3dfc,
            0x0041_3e00,
            0x0041_3e08,
            0x0041_3e18,
            0x0041_3e38,
            0x0041_3e48,
            0x0041_3e54,
            0x0041_3e74,
            0x0041_3e8c,
            0x0041_3e9c,
            0x0041_3ea0,
            0x0041_3ea4,
            0x0041_3ea8,
            0x0041_3ec8,
            0x0041_3ed8,
            0x0041_3ef0,
            0x0041_3ef8,
            0x0041_3efc,
            0x0041_3f04,
            0x0041_3f08,
        ];
        let entry = ENTRIES[seed as usize % ENTRIES.len()];
        if entry == 0x0041_3cd0 {
            let base = DB + 0x400;
            c.g[A0][0] = base as u64;
            // Sometimes pointer-equal (the abort path), else the sort.
            c.g[A1][0] = if seed % 4 == 0 {
                base as u64
            } else {
                (DB + 0x500) as u64
            };
        } else {
            // Clamped bounds and indices (see comment above); T7 = 0
            // steers stray 0x3e00 flows into the second nest, whose
            // exit reaches 0x3f08 with the merge layout in place.
            c.g[S1][0] = (DB + 0x100) as u64;
            c.g[S2][0] = (DB + 0x200) as u64;
            c.g[T3][0] = (DB + 0x100) as u64;
            c.g[T5][0] = (2 + seed % 3) as u64;
            c.g[T6][0] = c.g[T5][0];
            c.g[S0][0] = 1;
            c.g[A0][0] = 1;
            c.g[T2][0] = 2;
            c.g[V1][0] = 2;
            c.g[T4][0] = 4;
            c.g[A1][0] = 0;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0;
            c.g[T1][0] = 0;
            c.g[T7][0] = 0;
        }
        entry
    }

    #[test]
    fn end_fde_sort_matches() {
        // Dispatcher shape: entries across seeds cover every arm (entry
        // plus each otherwise-dead sort/sift arm). Mid-entry arms run with
        // small clamped loop bounds (T5 = T6 = 2..4, S0 = A0 = 1) so the
        // heapify loops stay bounded, and with the merge-table layout for
        // S1/S2 (fde_merge's own shape) so the FDE_MERGE call and the FREE
        // of lw(S1+8) — a real malloc'd block (heap_obj playbook) — behave.
        // Both sides run the same arms identically.
        check(
            ee::s15::f_00413cd0__generated,
            end_fde_sort,
            SEEDS_7,
            &|c, seed| {
                c.pc = end_fde_sort_setup(c, seed);
            },
        );
    }

    #[test]
    fn type_info_name_matches() {
        check(
            ee::s15::f_004151d8__generated,
            type_info_name,
            SEEDS_7,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0041_51d8, 0);
                c.bus.write32(DB, r.next_u32() & 0xffff_fffc);
                c.g[A0][0] = DB as u64;
            },
        );
    }

    #[test]
    fn type_info_ctor_matches() {
        check(
            ee::s15::f_004151e0__generated,
            type_info_ctor,
            SEEDS_7,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0041_51e0, 0);
                c.g[A0][0] = (DB + 0x400) as u64;
                c.g[A1][0] = (DB + 0x500) as u64;
                fill(c, &mut r, DB + 0x400, 4);
                let _ = r;
            },
        );
    }

    #[test]
    fn type_info_before_matches() {
        check(
            ee::s15::f_00415238__generated,
            type_info_before,
            SEEDS_7,
            &|c, seed| {
                let r = machine(c, seed, 0x0041_5238, 0);
                c.bus.write32(DB + 0x400, DB + 0x500);
                c.bus.write32(DB + 0x410, DB + 0x600);
                for (i, b) in b"TestType\0".iter().enumerate() {
                    c.bus.write8(DB + 0x500 + i as u32, *b);
                }
                // "less" needs b greater than a at the first differing
                // byte ('U' > 'T'); the other seed digits never produce
                // that.
                let less = seed % 4 == 3;
                if less {
                    for (i, b) in b"UestType\0".iter().enumerate() {
                        c.bus.write8(DB + 0x600 + i as u32, *b);
                    }
                } else if seed % 3 == 0 {
                    for (i, b) in b"TestType\0".iter().enumerate() {
                        c.bus.write8(DB + 0x600 + i as u32, *b);
                    }
                } else {
                    for (i, b) in b"OtherTyq\0".iter().enumerate() {
                        c.bus.write8(DB + 0x600 + i as u32, *b);
                    }
                }
                c.g[A0][0] = (DB + 0x400) as u64;
                c.g[A1][0] = if seed % 3 == 1 && !less {
                    (DB + 0x400) as u64
                } else {
                    (DB + 0x410) as u64
                };
                let _ = r;
            },
        );
    }

    use crate::difftest::fakes;
    use crate::difftest::require_paths;
    use std::ptr::addr_of_mut;

    /// The `__clz_tab` copy `__divdi3` reads (the game image holds each
    /// byte's bit length there, which `clz_table` writes).
    pub(crate) const DIVDI3_CLZ_TAB: u32 = 0x0049_96a0;

    /// Magnitudes below 2^63, so the sign digits decide the signs.
    pub(crate) const DIVDI3_MAGNITUDE: u64 = u64::MAX >> 1;

    /// Edge dividends and divisors, crossed by `divdi3_operands` case 6.
    pub(crate) const DIVDI3_EDGE_N: [u64; 8] = [
        0,
        1,
        u64::MAX,
        1 << 63,
        u64::MAX >> 1,
        1 << 32,
        0xffff_ffff,
        0xffff_ffff_0000_0000,
    ];

    pub(crate) const DIVDI3_EDGE_D: [u64; 8] = [
        1,
        u64::MAX,
        1 << 63,
        u64::MAX >> 1,
        1 << 32,
        0xffff_ffff,
        0x8000_0000,
        0xffff_ffff_8000_0000,
    ];

    /// Arm totals `divdi3_operands` reaches, each taken by no other arm
    /// combination of the fixture (block charges of the generated body:
    /// signs 7 + 3 + 9, +21 for a negative n, +20 for a negative d, +18 to
    /// negate the quotient; joins 11 / 9 / 8):
    /// 34 / 72 / 73 / 75 `d1 > n1` with each sign pair; 89 `0q = nn/0D`,
    /// d0 normalized, no corrections; 92 the same with d0 <= 0xffff; 95 its
    /// low digit corrected twice; 96 its high digit corrected once; 111
    /// both digits corrected twice after normalizing; 104 `0q = NN/dd`
    /// normalized, `m1:m0 == n` exactly; 114 `q0 * d0` above n (one
    /// correction per digit); 117 the second digit corrected twice; 105
    /// division by zero (last digit corrected once); 168 `qq = NN/0d`
    /// normalized from d0 <= 0xffff; 178 / 181 / 187 / 193 its third digit
    /// corrected once / with carry / twice, and its second digit once.
    pub(crate) const DIVDI3_TOTALS: [u64; 18] = [
        34, 72, 73, 75, 89, 92, 95, 96, 111, 104, 114, 117, 105, 168, 178, 181, 187, 193,
    ];

    /// One operand pair per arm, `(n, d, generated cycle total)`, for the
    /// arms whose totals other arms share (a correction with carry, the two
    /// `umul_ppmm` exits that cost the same, the low digit corrected twice):
    /// the total pins the arm totals worked out from the block charges.
    pub(crate) const DIVDI3_ARMS: [(u64, u64, u64); 19] = [
        // qq, both negative: third digit twice, last once
        (0x801c_aa02_da0c_46ff, 0xffff_ffff_bfff_8004, 189),
        // 0q = NN/dd, m1 < n1, no corrections
        (0x4000_a769_7bad_915b, 0x0000_4000_7fff_5f63, 100),
        // 0q = nn/0D, n negative: high digit with carry, low digit once
        (0x9189_9abe_0bf1_2eaa, 0x0000_0000_d6e7_c7d1, 139),
        // 0q = nn/0D, d0 <= 0xffff, d negative
        (0x0000_2191_a95e_b2ad, 0xffff_ffff_ffff_cbeb, 130),
        // 0 / 0
        (0, 0, 102),
        // 0q = NN/dd, q0 * d0 above n, d negative
        (0x7f00_05f3_783b_42d0, 0xffff_feff_fe00_142c, 152),
        // qq: second digit with carry, third and last with carry
        (0x7fff_ff62_d92f_6ef9, 0x0000_0000_3fff_fffc, 145),
        // 0q = NN/dd, d1 > 0xffff, m1:m0 == n, n negative
        (0xb512_002e_b3c2_4331, 0x4aed_ffd1_4c3d_bcce, 143),
        // qq: third digit once, last twice
        (0x2b6d_c92d_487a_dc8d, 0x0000_0000_2000_3fff, 145),
        // 0q = NN/dd: both digits with carry
        (0x7fff_dc6e_67e7_600f, 0x0000_7fff_fffb_f405, 110),
        // 0q = nn/0D: both digits twice
        (0x0400_07fd_f058_20d2, 0x0000_0000_0400_07ff, 111),
        // 0q = nn/0D, d negative: high digit once, low digit with carry
        (0x4f42_4379_4418_253b, 0xffff_ffff_16e4_0df7, 137),
        // d1 > n1
        (0x2233_ff22_58b2_2fdd, 0x2233_ff23_d3f1_3c79, 34),
        // qq, d0 = 1
        (0x59fa_57f3_3670_78b4, 1, 130),
        // 0q = NN/dd, m1 == n1 and m0 > n0, d negative
        (0x5344_22bb_3f73_a135, 0xffff_5ead_2b30_8415, 144),
        // 0q = NN/dd, both negative: second digit twice
        (0xbfff_adea_f634_00fe, 0xffff_bfff_8001_5f48, 158),
        // MIN / MIN: d1 normalized, n == d
        (1 << 63, 1 << 63, 100),
        // qq: second digit once, third once
        (0x673e_14dd_02d5_6d8a, 0x0000_0000_0001_0001, 146),
        // MIN / 2^31: qq with d0 normalized (q1 = 1, n1 -= d0)
        (1 << 63, 0x8000_0000, 132),
    ];

    /// Operands for one seed: the case is `seed % 8`, the sign of n
    /// `seed / 8 % 2`, the sign of d `seed / 16 % 2`, the magnitude
    /// `pick = seed / 32 % 16`, the case-1 step `seed / 512 % 4` and the
    /// case-3 shift `seed / 64 % 32`, each independent of the others.
    /// Shared by the `__divdi3` and `__moddi3` tests.
    pub(crate) fn divdi3_operands(seed: u32) -> (u64, u64) {
        let mut r = Rng(0x0040_fcb0_0d1d_0000 ^ (seed as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15));
        let wide = |r: &mut Rng| (r.next_u32() as u64) << 32 | r.next_u32() as u64;
        let pick = seed / 32 % 16;
        let (n, d) = match seed % 8 {
            0 => {
                // 00 = nn / DD: d1 > n1
                let d1 = (r.next_u32() >> (1 + pick)) | 1;
                let n1 = match pick % 4 {
                    0 => d1 - 1,
                    1 => 0,
                    _ => r.next_u32() % d1,
                };
                let n0 = r.next_u32();
                let d0 = r.next_u32();
                ((n1 as u64) << 32 | n0 as u64, (d1 as u64) << 32 | d0 as u64)
            }
            1 => {
                // 0q = NN / dd with n a small step from a multiple of d:
                // the `umul_ppmm` compare arms
                let bm = 1 + pick + pick / 8 % 2 * 15;
                let d = (wide(&mut r) >> bm) | 1 << (63 - bm);
                let q = 1 + (r.next_u32() % (1 << bm.min(16))) as u64;
                let k = [0, 1, u64::MAX, 2][(seed / 512 % 4) as usize];
                (d.wrapping_mul(q).wrapping_add(k) & DIVDI3_MAGNITUDE, d)
            }
            2 => {
                // 0q = NN / dd with digit estimates one or two too big:
                // built normalized (shifted left by bm), then shifted back
                let bm = [17, 23, 28, 31][(pick % 4) as usize];
                let d1n = if pick / 4 % 2 == 0 {
                    0x8000_ffff - r.next_u32() % 16
                } else {
                    0xffff_ffff - r.next_u32() % 16
                };
                let dh = d1n >> 16;
                let top = (1u32 << (bm - 1)) - 1;
                let n2 = (top / dh * dh + r.next_u32() % 4).min(top);
                let n1n = if pick / 8 == 0 {
                    r.next_u32()
                } else {
                    d1n - 1 - r.next_u32() % 4
                };
                let low = u32::MAX >> bm << bm;
                let d = ((d1n as u64) << 32 | (r.next_u32() & low) as u64) >> bm;
                let n = (((n2 as u128) << 64 | (n1n as u128) << 32 | (r.next_u32() & low) as u128)
                    >> bm) as u64;
                (n, d)
            }
            3 => {
                // 0q = nn / 0D: d1 == 0, n1 < d0
                let (d0, n1) = match pick % 4 {
                    0 => {
                        let d0 = 0x8000_0000 | r.next_u32();
                        (d0, r.next_u32() % d0)
                    }
                    1 => {
                        let d0 = (r.next_u32() >> (16 + pick / 4 * 4)) | 1;
                        (d0, r.next_u32() % d0)
                    }
                    2 => {
                        // estimates one or two too big
                        let d0 = (0x8000_ffff - r.next_u32() % 16) >> (pick / 4 * 5);
                        (d0, d0 - 1 - r.next_u32() % 4)
                    }
                    _ => {
                        let d0 = (r.next_u32() >> (seed / 64 % 32)) | 1;
                        (d0, r.next_u32() % d0)
                    }
                };
                let n0 = r.next_u32();
                (
                    ((n1 as u64) << 32 | n0 as u64) & DIVDI3_MAGNITUDE,
                    d0 as u64,
                )
            }
            4 => {
                // qq = NN / 0d: d1 == 0, n1 >= d0 != 0 (estimates too big
                // with the large base and a dividend near the top)
                let shift = [1, 2, 5, 9, 15, 23, 30, 31][(pick / 2 % 8) as usize];
                let (d, n) = if pick % 2 == 0 {
                    (
                        (0x8000_ffff - r.next_u32() % 16) >> shift,
                        wide(&mut r) >> 1,
                    )
                } else {
                    (
                        (0xffff_ffff - r.next_u32() % 16) >> shift,
                        DIVDI3_MAGNITUDE - (wide(&mut r) >> 24),
                    )
                };
                let d = d as u64;
                (if n >> 32 < d { n | d << 32 } else { n }, d)
            }
            5 => {
                // division by zero
                let n = match pick % 8 {
                    0 => 0,
                    1 => 1,
                    2 => wide(&mut r) >> 1,
                    3 => DIVDI3_MAGNITUDE,
                    4 => ((r.next_u32() >> 1) as u64) << 32,
                    5 => r.next_u32() as u64,
                    6 => 1 << 63,
                    _ => 0x7fff_fffe << 32,
                };
                (n, 0)
            }
            6 => {
                // edge values, crossed; they carry their own signs
                return (
                    DIVDI3_EDGE_N[(seed / 8 % 8) as usize],
                    DIVDI3_EDGE_D[(seed / 64 % 8) as usize],
                );
            }
            _ => {
                // every divisor bit length (leading-zero counts 63..0)
                let k = seed / 8 % 64;
                let d = if k == 63 {
                    1 << 63
                } else {
                    1 << k | wide(&mut r) & ((1 << k) - 1)
                };
                (wide(&mut r) >> 1, d)
            }
        };
        let n = if seed / 8 % 2 == 1 {
            n.wrapping_neg()
        } else {
            n
        };
        let d = if seed / 16 % 2 == 1 {
            d.wrapping_neg()
        } else {
            d
        };
        (n, d)
    }

    /// A machine about to divide `n` by `d`: junk in every register and in
    /// LO/HI, the clz table where the generated body reads it.
    pub(crate) fn divdi3_machine(c: &mut Cpu, seed: u32, n: u64, d: u64) {
        let mut r = Rng(0x0040_fcb0 ^ seed as u64);
        scramble(c, &mut r);
        c.lo[0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
        c.hi[0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
        clz_table(c, DIVDI3_CLZ_TAB);
        c.g[A0][0] = n;
        c.g[A1][0] = d;
        c.pc = DIVDI3_ENTRY;
    }

    /// `__divdi3` against the generated body over every case of
    /// `__udivmoddi4` (each `count_leading_zeros` range, normalized or not,
    /// digit corrections, the `umul_ppmm` exits), signs crossed, edge
    /// values, every divisor bit length and division by zero; LO/HI too.
    #[test]
    fn divdi3_matches() {
        check_paths_lohi(
            ee::s15::f_0040fcb0__generated,
            divdi3,
            2048,
            &DIVDI3_TOTALS,
            &|c, seed| {
                let (n, d) = divdi3_operands(seed);
                divdi3_machine(c, seed, n, d);
            },
        );
    }

    /// The arms whose cycle totals other arms share, one operand pair each.
    #[test]
    fn divdi3_arms_match() {
        for (i, &(n, d, total)) in DIVDI3_ARMS.iter().enumerate() {
            check_paths_lohi(
                ee::s15::f_0040fcb0__generated,
                divdi3,
                1,
                &[total],
                &|c, _| divdi3_machine(c, i as u32, n, d),
            );
        }
    }

    /// Entry of `__moddi3` (it has no resume entries: no calls, no event
    /// checks).
    pub(crate) const MODDI3_ENTRY: u32 = 0x0042_d470;

    /// The `__clz_tab` copy `__moddi3` reads (the game image holds each
    /// byte's bit length there, which `clz_table` writes).
    pub(crate) const MODDI3_CLZ_TAB: u32 = 0x0049_ae18;

    /// An entry sp that puts the frame at address 0, so `rp = &w` is 0 and
    /// every `if (rp != 0)` store is skipped.
    pub(crate) const MODDI3_NULL_RP_SP: u64 = MODDI3_FRAME as u64;

    /// Arm totals `divdi3_operands` reaches, each taken by no other arm
    /// combination of the fixture (block charges of the generated body:
    /// signs 8 + 3 + 9 + 3, +21 for a negative n, +18 for a negative d, +21
    /// to negate the remainder, return 2 + 4; `0q`/`qq` digits 10/14/18 or
    /// 10/14/16 by correction (none / with carry / tested twice), the `qq`
    /// high word's digits 11/15/20/19 and 10/14/18, the `NN/dd` digits
    /// 11/16/18/22 and 13/18/20/23; the remainder store 4 + 1, 12 + 1 after
    /// `umul_ppmm`): 34 / 52 / 76 `d1 > n1` with n+ d+, n+ d-, n- d+; 86
    /// `0q = nn/0D`, d0 normalized, no corrections; 107 the same with d0 <=
    /// 0xffff, d negative; 108 d0 normalized, d negative, low digit with
    /// carry; 137 n negative, high digit with carry; 147 both digits twice;
    /// 159 both negative, high digit once; 98 division by zero; 106 the
    /// same with both digits with carry; 122 `qq = NN/0d` with d0 <=
    /// 0xffff; 174 n negative, the low word's high digit once; 196 both
    /// negative, the high word's low digit and both low-word digits with
    /// carry; 206 the same corrected once; 125 `0q = NN/dd`, high digit
    /// once, `m1 > n1`; 151 d negative, high digit once, low digit twice,
    /// `m1 != n1`; 162 d1 > 0xffff, n negative, `m1 == n1` and `m0 <= n0`;
    /// 172 n negative, both digits once; 186 both negative, both digits with
    /// carry.
    pub(crate) const MODDI3_TOTALS: [u64; 20] = [
        34, 52, 76, 86, 107, 108, 137, 147, 159, 98, 106, 122, 174, 196, 206, 125, 151, 162, 172,
        186,
    ];

    /// One operand pair per arm, `(n, d, generated cycle total)`, for the
    /// arms whose totals other arms share in the fixture: the total pins
    /// the arm worked out from the block charges.
    pub(crate) const MODDI3_ARMS: [(u64, u64, u64); 7] = [
        // qq, d negative: both low-word digits corrected twice
        (0x5232_27f3_a8cd_3c30, 0xffff_ffff_fbff_f801, 156),
        // -1 % 0
        (u64::MAX, 0, 140),
        // 0q, n negative, d0 normalized: high digit with carry, low once
        (0x9189_9abe_0bf1_2eaa, 0x0000_0000_d6e7_c7d1, 138),
        // 0q = NN/dd, d negative: m1 == n1 and m0 > n0
        (0x5344_22bb_3f73_a135, 0xffff_5ead_2b30_8415, 141),
        // d1 > n1, both negative
        (0xab77_d99f_5a22_194a, 0xab77_d99e_6609_0a55, 94),
        // MIN % MIN: d1 normalized, n == d
        (1 << 63, 1 << 63, 126),
        // MIN % 2^31: qq with d0 normalized (n1 -= d0)
        (1 << 63, 0x8000_0000, 132),
    ];

    /// Arm totals of the `rp == 0` run (no remainder store; the stale slot
    /// is returned, negated for a negative n): 81 / 129 `0q` with d0
    /// normalized, n+ / n-; 84 / 132 the same unnormalized; 93 / 139
    /// division by zero; 117 / 159 `qq`; 107 / 145 `0q = NN/dd`.
    pub(crate) const MODDI3_NULL_RP_TOTALS: [u64; 10] =
        [81, 129, 84, 132, 93, 139, 117, 159, 107, 145];

    /// `MIN % MIN` with `rp == 0`, `(n, d, generated cycle total)`.
    pub(crate) const MODDI3_NULL_RP_ARM: (u64, u64, u64) = (1 << 63, 1 << 63, 121);

    /// A machine about to take `n % d`: junk in every register, in LO/HI and
    /// in the frame `__moddi3` opens below `sp` (so a missing or extra store
    /// of `w` shows), the clz table where the generated body reads it.
    pub(crate) fn moddi3_machine(c: &mut Cpu, seed: u32, n: u64, d: u64) {
        let mut r = Rng(0x0042_d470 ^ seed as u64);
        scramble(c, &mut r);
        c.lo[0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
        c.hi[0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
        let frame = (c.g[SP][0] as u32).wrapping_sub(MODDI3_FRAME);
        for off in (0..MODDI3_FRAME).step_by(4) {
            c.bus.write32(frame + off, r.next_u32() | 1);
        }
        clz_table(c, MODDI3_CLZ_TAB);
        c.g[A0][0] = n;
        c.g[A1][0] = d;
        c.pc = MODDI3_ENTRY;
    }

    /// `__moddi3` against the generated body over every case of
    /// `__udivmoddi4` (each `count_leading_zeros` range, normalized or not,
    /// digit corrections, the `umul_ppmm` exits), signs crossed, edge
    /// values, every divisor bit length and division by zero; LO/HI too.
    #[test]
    fn moddi3_matches() {
        check_paths_lohi(
            ee::s15::f_0042d470__generated,
            moddi3,
            // The both-negative high-digit-once arm (159) first appears at
            // seed 3547.
            8192,
            &MODDI3_TOTALS,
            &|c, seed| {
                let (n, d) = divdi3_operands(seed);
                moddi3_machine(c, seed, n, d);
            },
        );
    }

    /// The arms whose cycle totals other arms share, one operand pair each.
    #[test]
    fn moddi3_arms_match() {
        for (i, &(n, d, total)) in MODDI3_ARMS.iter().enumerate() {
            check_paths_lohi(
                ee::s15::f_0042d470__generated,
                moddi3,
                1,
                &[total],
                &|c, _| moddi3_machine(c, i as u32, n, d),
            );
        }
    }

    /// `rp == 0` (the frame at address 0): every `if (rp != 0)` store is
    /// skipped and the stale slot comes back, negated for a negative n.
    #[test]
    fn moddi3_null_rp_matches() {
        let null_rp = |c: &mut Cpu, seed: u32, n: u64, d: u64| {
            c.g[SP][0] = MODDI3_NULL_RP_SP;
            moddi3_machine(c, seed, n, d);
        };
        check_paths_lohi(
            ee::s15::f_0042d470__generated,
            moddi3,
            256,
            &MODDI3_NULL_RP_TOTALS,
            &|c, seed| {
                let (n, d) = divdi3_operands(seed);
                null_rp(c, seed, n, d);
            },
        );
        let (n, d, total) = MODDI3_NULL_RP_ARM;
        check_paths_lohi(
            ee::s15::f_0042d470__generated,
            moddi3,
            1,
            &[total],
            &|c, _| null_rp(c, 0, n, d),
        );
    }

    pub(crate) const IEEE_LOG_2_ENTRY: u32 = 0x0041_f3a8;

    pub(crate) const IEEE_LOG_2_SEEDS: u32 = 400;

    /// Gate word of the path meters (outside the image, the stack and the fake log).
    pub(crate) const IEEE_LOG_2_TEST_METER: u32 = DATA + 0x20_0000;

    pub(crate) const IEEE_LOG_2_TEST_METER_MAGIC: u32 = 0x4c06_f3a8;

    /// The first high-word mantissa at or past sqrt(2)'s (x is halved).
    pub(crate) const IEEE_LOG_2_TEST_SQRT2_HM: u64 = 0x6a09c;

    /// High-word mantissas at the edges of the `i > 0` window and the sqrt(2) cut.
    pub(crate) const IEEE_LOG_2_TEST_EDGES: [u64; 10] = [
        0x6147a, 0x61479, 0x6b851, 0x6b852, 0x6a09b, 0x6a09c, 1, 2, 0xffffd, 0x6a09e,
    ];

    /// High-word mantissas with |f| < 2^-20.
    pub(crate) const IEEE_LOG_2_TEST_TINY: [u64; 3] = [0, 0xffffe, 0xfffff];

    /// Own cycles of every arm (soft-float helpers metered out), from the
    /// generated block charges.
    pub(crate) const IEEE_LOG_2_TEST_PATHS: [u64; 12] = [
        41,  // +-0: -inf
        47,  // x < 0: NaN
        42,  // inf / NaN: x + x
        76,  // x == 1: 0
        101, // |f| < 2^-20, f == 0, k != 0
        100, // |f| < 2^-20, f != 0, k == 0
        127, // |f| < 2^-20, f != 0, k != 0
        202, // i > 0, k == 0
        227, // i > 0, k != 0
        190, // i <= 0, k == 0
        217, // i <= 0, k != 0
        // Subnormal (+15): this build's soft-float reads a zero exponent as zero
        // (`gcc::unpack_d`), so `x * two54` is 0 and every subnormal takes the f == 0
        // arm; the subnormal tiny-f (142) and i > 0 / i <= 0 (242 / 232) arms cannot run.
        116,
    ];

    /// Path meters on the six soft-float helpers: with the gate word set a
    /// helper runs as before and then takes back the cycles it charged, so a
    /// run's total is the cost of `__ieee754_log_2`'s own blocks. Gate off:
    /// whatever the slot held before.
    macro_rules! ieee_log_2_meters {
        ($($meter:ident: $slot:ident, $generated:ident;)*) => {
            $(
                pub(crate) fn $meter(c: &mut Cpu) {
                    let prev = fakes::prev_cell($meter);
                    if c.bus.read32(IEEE_LOG_2_TEST_METER) != IEEE_LOG_2_TEST_METER_MAGIC {
                        return fakes::fall_back(c, prev, ee::s15::$generated);
                    }
                    let start = c.cycles;
                    fakes::fall_back(c, prev, ee::s15::$generated);
                    c.cycles = start;
                }
            )*

            pub(crate) fn ieee_log_2_meters_install() {
                $(fakes::hook(addr_of_mut!(ee::s15::$slot), $meter, fakes::prev_cell($meter));)*
            }
        };
    }

    ieee_log_2_meters! {
        ieee_log_2_meter_adddf3: OVERRIDE_00411F18, f_00411f18__generated;
        ieee_log_2_meter_subdf3: OVERRIDE_00411F70, f_00411f70__generated;
        ieee_log_2_meter_muldf3: OVERRIDE_00411FD8, f_00411fd8__generated;
        ieee_log_2_meter_divdf3: OVERRIDE_00412280, f_00412280__generated;
        ieee_log_2_meter_cmpdf2: OVERRIDE_00412500, f_00412500__generated;
        ieee_log_2_meter_floatsidf: OVERRIDE_00412550, f_00412550__generated;
    }

    pub(crate) fn ieee_log_2_test_bits(e: u64, hm: u64, lo: u32) -> u64 {
        e << 52 | hm << 32 | lo as u64
    }

    /// A high-word mantissa in one of the four regions: i > 0 below and above
    /// the sqrt(2) cut, i <= 0 below and above it (|f| < 2^-20 left out).
    pub(crate) fn ieee_log_2_test_region(r: &mut Rng, region: u32) -> u64 {
        let (lo, hi) = match region {
            0 => (0x6147a, IEEE_LOG_2_TEST_SQRT2_HM),
            1 => (IEEE_LOG_2_TEST_SQRT2_HM, 0x6b852),
            2 => (1, 0x6147a),
            _ => (0x6b852, 0xffffe),
        };
        lo + r.next_u32() as u64 % (hi - lo)
    }

    /// The exponent field for case 0 (k == 0), 1 (any other) or 2 (the
    /// smallest or largest normal, `side`).
    pub(crate) fn ieee_log_2_test_exp(r: &mut Rng, hm: u64, case: u32, side: u32) -> u64 {
        let k_zero = if hm >= IEEE_LOG_2_TEST_SQRT2_HM {
            0x3fe
        } else {
            0x3ff
        };
        match case {
            0 => k_zero,
            1 => {
                let e = 1 + (r.next_u32() % 0x7fe) as u64;
                if e == k_zero {
                    e + 1
                } else {
                    e
                }
            }
            _ => [1, 0x7fe][side as usize],
        }
    }

    /// A subnormal whose scaled high-word mantissa is `hm` (top bit at 21..51).
    pub(crate) fn ieee_log_2_test_subnormal(r: &mut Rng, hm: u64, lo: u32) -> u64 {
        let top = 21 + r.next_u32() % 31;
        let mantissa = hm << 32 | lo as u64;
        1u64 << top | mantissa >> (52 - top)
    }

    /// x for `seed`: the phase is `seed % 8`, its cases the digits of `seed / 8`.
    pub(crate) fn ieee_log_2_test_x(r: &mut Rng, seed: u32) -> u64 {
        let d = seed / 8;
        let lo = r.next_u32();
        match seed % 8 {
            // +-0
            0 => ((d % 2) as u64) << 63,
            // x < 0: normal, subnormal, -inf, -NaN, -1
            1 => {
                let magnitude = match d % 5 {
                    0 => {
                        let e = 1 + (r.next_u32() % 0x7fe) as u64;
                        ieee_log_2_test_bits(e, (r.next_u32() % 0x10_0000) as u64, lo)
                    }
                    1 => ieee_log_2_test_bits(0, (r.next_u32() % 0x10_0000) as u64, lo | 1),
                    2 => ieee_log_2_test_bits(0x7ff, 0, 0),
                    3 => ieee_log_2_test_bits(0x7ff, 0x8_0000, lo),
                    _ => IEEE_LOG_ONE,
                };
                1 << 63 | magnitude
            }
            // inf, NaNs (quiet, low word only, all ones)
            2 => match d % 4 {
                0 => ieee_log_2_test_bits(0x7ff, 0, 0),
                1 => ieee_log_2_test_bits(0x7ff, 0x8_0000, lo),
                2 => ieee_log_2_test_bits(0x7ff, 0, lo | 1),
                _ => 0x7fff_ffff_ffff_ffff,
            },
            // Subnormals: a power of two, |f| < 2^-20, both i arms, edges
            3 => match d % 5 {
                0 => 1 << (r.next_u32() % 52),
                1 => {
                    let hm = IEEE_LOG_2_TEST_TINY[(d / 5 % 3) as usize];
                    ieee_log_2_test_subnormal(r, hm, lo | 0x8000_0000)
                }
                2 => {
                    let hm = ieee_log_2_test_region(r, d / 5 % 2);
                    ieee_log_2_test_subnormal(r, hm, lo)
                }
                3 => {
                    let hm = ieee_log_2_test_region(r, 2 + d / 5 % 2);
                    ieee_log_2_test_subnormal(r, hm, lo)
                }
                _ => [0x000f_ffff_ffff_ffff, (lo | 1) as u64][(d / 5 % 2) as usize],
            },
            // Powers of two: 1 (k == 0), then k != 0
            4 => {
                let e = match d % 6 {
                    0 => 0x3ff,
                    1 => 0x3fe,
                    2 => 0x400,
                    3 => 1,
                    4 => 0x7fe,
                    _ => {
                        let e = 1 + (r.next_u32() % 0x7fe) as u64;
                        if e == 0x3ff {
                            0x3fd
                        } else {
                            e
                        }
                    }
                };
                e << 52
            }
            // |f| < 2^-20, f != 0, crossed with k
            5 => {
                let hm = IEEE_LOG_2_TEST_TINY[(d % 3) as usize];
                let e = ieee_log_2_test_exp(r, hm, d / 3 % 3, d / 9 % 2);
                ieee_log_2_test_bits(e, hm, lo | 1)
            }
            // The four mantissa regions crossed with k
            6 => {
                let hm = ieee_log_2_test_region(r, d % 4);
                let e = ieee_log_2_test_exp(r, hm, d / 4 % 3, d / 12 % 2);
                ieee_log_2_test_bits(e, hm, lo)
            }
            // The region edges crossed with k
            _ => {
                let hm = IEEE_LOG_2_TEST_EDGES[(d % 10) as usize];
                let e = ieee_log_2_test_exp(r, hm, d / 10 % 3, d / 30 % 2);
                ieee_log_2_test_bits(e, hm, lo)
            }
        }
    }

    /// Scrambled registers, the game image (soft-float's NaN constant), x in
    /// a0; the -inf word overwritten with a value only it holds (the pool has
    /// another -inf at 0x4a0000 - 0x6ef0). `metered` turns the path meters on.
    pub(crate) fn ieee_log_2_setup(c: &mut Cpu, seed: u32, metered: bool) {
        let mut r = machine(c, seed, 0x41_f3a8, 0);
        elf_image(c);
        c.bus.write64(
            IEEE_LOG_2_NEG_INF,
            0xfff0_0000_0000_0000 | (seed as u64) << 4 | 0xd,
        );
        c.pc = IEEE_LOG_2_ENTRY;
        c.g[A0][0] = ieee_log_2_test_x(&mut r, seed);
        if metered {
            ieee_log_2_meters_install();
            c.bus
                .write32(IEEE_LOG_2_TEST_METER, IEEE_LOG_2_TEST_METER_MAGIC);
        }
    }

    /// __ieee754_log_2 on every phase of e_log.c: +-0, negatives, inf/NaN,
    /// subnormals, powers of two, |f| < 2^-20 with k == 0 and not, and the
    /// four mantissa regions and their edges crossed with k; real soft-float
    /// helpers.
    #[test]
    fn ieee754_log_2_matches() {
        check(
            ee::s15::f_0041f3a8__generated,
            ieee754_log_2,
            IEEE_LOG_2_SEEDS,
            &|c, seed| ieee_log_2_setup(c, seed, false),
        );
    }

    /// The same seeds reach all fifteen arms (own cycles, helpers metered out).
    #[test]
    fn ieee754_log_2_paths() {
        require_paths(
            ee::s15::f_0041f3a8__generated,
            IEEE_LOG_2_SEEDS,
            &IEEE_LOG_2_TEST_PATHS,
            &|c, seed| ieee_log_2_setup(c, seed, true),
        );
    }
    use crate::difftest::check_paths;
    use crate::difftest::fakes::fall_back;
    use crate::difftest::fakes::hook;
    use crate::difftest::fakes::prev_cell;
    pub(crate) use crate::game::lib::libc::tests::glue_machine;
    pub(crate) use crate::game::lib::libc::tests::SEEDS_4 as SEEDS_9;
    pub(crate) use crate::game::misc::tests::unwind_machine;
    pub(crate) use crate::game::misc::tests::SEEDS_UNWIND as SEEDS_2;

    driven!(fde_split_gen_driven, ee::s15::f_004149f0__generated);

    driven!(fde_split_hand_driven, fde_split);

    #[test]
    fn fde_split_matches() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = unwind_machine(c, seed, 0x4149);
            // FDE table: entry count 0..4 (0 takes the empty-table arm);
            // rows point at scratch FDEs. Row starts steer overlap:
            // ascending (no overlap), a row-1 dip (overlapping pair,
            // multi-iteration inner loop), a row-1 touch (equal starts,
            // V0 == 0), a deep row-3 dip (non-sentinel inner latch), and
            // a row-2 dip with a negative or equal
            // current-vs-predecessor difference. The out descriptor has
            // a real prefilled base; its index starts at 0 or 1.
            let n = (seed / 5) % 5;
            c.bus.write32(DATA + 0x400, DATA + 0x500);
            c.bus.write32(DATA + 0x404, n);
            for i in 0..4u32 {
                let e = DATA + 0x600 + i * 16;
                let start = match (seed % 5, i) {
                    (0, _) => 0x1000 * (i + 1),
                    (1, 1) => 0x800,
                    (1, _) => 0x1000 * (i + 1),
                    (2, 1) => 0x1000,
                    (2, _) => 0x1000 * (i + 1),
                    (3, 3) => 0x500,
                    (3, _) => 0x1000 * (i + 1),
                    (_, 2) => {
                        if (seed / 5) % 2 == 0 {
                            0x500
                        } else {
                            0x1000
                        }
                    }
                    (_, _) => 0x1000 * (i + 1),
                };
                c.bus.write32(e, if i < n { 0x10 } else { 0 });
                c.bus
                    .write32(e + 4, if i < n { 1 + (seed + i) % 3 } else { 0 });
                c.bus.write32(e + 8, if i < n { start } else { 0 });
                c.bus.write32(e + 0xc, (seed * 37 + i * 11 + 1) % 0x100);
                c.bus.write32(DATA + 0x500 + i * 4, e);
            }
            c.bus.write32(DATA + 0x700, DATA + 0x900);
            c.bus.write32(DATA + 0x704, seed % 2);
            for i in 0..0x40u32 {
                c.bus.write32(DATA + 0x900 + i * 4, 0x5a5a_5a5a);
            }
            c.g[A0][0] = (DATA + 0x400) as u64;
            c.g[A1][0] = (DATA + 0x700) as u64;
            let mode = schedule(c, &mut r, seed, 0x0041_49f0);
            // Deliver-only (same harness reason as ds_timetosecs).
            ssx3_ps2::platform::machine_of(&mut c.bus.plat)
                .sched
                .next_vblank = 1 << 40;
            mode
        };
        check_paths(
            ee::s15::f_004149f0__generated,
            fde_split,
            SEEDS_2,
            &[53, 85, 117],
            &|c, seed| {
                setup(c, seed);
            },
        );
        check_events(fde_split_gen_driven, fde_split_hand_driven, SEEDS_2, &setup);
        // First-event placement per head: log every delivered event's
        // head through the same fixture on both sides with identical
        // logs, and require the outer (0x4a40) head to fire at least
        // once. (Structured loops never unwind, so placement — not
        // resumes — is the per-head evidence here; the inner head is
        // pinned by the focused run below.)
        let mut outer = false;
        for seed in 0..SEEDS_2 {
            let mut a = crate::difftest::fresh();
            setup(&mut a, seed);
            a.hooks.event = record_event_pc;
            crate::difftest::run_generated(&mut a, ee::s15::f_004149f0__generated);
            let mut b = crate::difftest::fresh();
            setup(&mut b, seed);
            b.hooks.event = record_event_pc;
            crate::difftest::run_generated(&mut b, fde_split);
            let n = a.bus.read32(FDE_EVENT_COUNT);
            assert_eq!(
                n,
                b.bus.read32(FDE_EVENT_COUNT),
                "seed {seed}: event count differs"
            );
            for i in 0..n.min(16) {
                let (pa, pb) = (
                    a.bus.read32(FDE_EVENT_PCS + i * 4),
                    b.bus.read32(FDE_EVENT_PCS + i * 4),
                );
                assert_eq!(pa, pb, "seed {seed} event {i}: head differs");
                if pa == 0x0041_4a40 {
                    outer = true;
                }
            }
        }
        assert!(outer, "no event fired at the outer head 0x414a40");
        // Inner-head placement: the deep row-3 dip (seed 23) spans a
        // multi-iteration inner trip after the outer edges have passed,
        // so an explicit deadline inside that window fires at the inner
        // head on both sides with identical logs.
        {
            let mut inner = false;
            for (f, tag) in [
                (ee::s15::f_004149f0__generated as fn(&mut Cpu), "gen"),
                (fde_split as fn(&mut Cpu), "hand"),
            ] {
                let mut c = crate::difftest::fresh();
                setup(&mut c, 23);
                c.hooks.event = record_event_pc;
                c.cycles = 1000;
                c.next_event = c.cycles + 110;
                crate::difftest::run_generated(&mut c, f);
                let n = c.bus.read32(FDE_EVENT_COUNT);
                assert_eq!(n, 1, "{tag}: expected exactly one event");
                let head = c.bus.read32(FDE_EVENT_PCS);
                assert_eq!(
                    head, 0x0041_4a80,
                    "{tag}: expected the inner head 0x414a80, got {head:#x}"
                );
                inner = true;
            }
            assert!(inner, "no event fired at the inner head 0x414a80");
        }
    }

    /// Event-PC log for the head-placement run below (count + heads).
    pub(crate) const FDE_EVENT_COUNT: u32 = DATA + 0x2000;

    pub(crate) const FDE_EVENT_PCS: u32 = DATA + 0x2004;

    /// Test event hook: log the head `pc` of every delivered event (up
    /// to 16), then run the machine's real event path with the same
    /// cycle budget as the default test hook.
    pub(crate) fn record_event_pc(c: &mut Cpu) -> bool {
        let n = c.bus.read32(FDE_EVENT_COUNT);
        if n < 16 {
            c.bus.write32(FDE_EVENT_PCS + n * 4, c.pc);
            c.bus.write32(FDE_EVENT_COUNT, n + 1);
        }
        if c.cycles >= crate::difftest::CYCLE_BUDGET {
            panic!(
                "cycle budget exceeded ({} cycles, pc {:#x}): runaway on test data",
                c.cycles, c.pc
            );
        }
        (ssx3_ps2::platform::HOOKS.event)(c)
    }

    driven!(find_fde_gen_driven, ee::s15::f_00414148__generated);

    driven!(find_fde_hand_driven, find_fde);

    /// Frame-registry head the unwinder walks (real game address, in RAM).
    pub(crate) const REG_HEAD: u32 = 0x0052_ade8;

    /// Registry objects: low, high, fde array, table, count, next.
    pub(crate) const OBJ0: u32 = DATA + 0x600;

    /// Registry/table/chain writes for `setup_paths` (pure in the seed:
    /// running it twice restores the pristine state).
    fn build_registry(c: &mut Cpu, seed: u32) -> u32 {
        // Up to two objects with sorted non-overlapping rows; row
        // starts stay inside [low, high). Seeds looking up pc == low
        // (seed % 8 == 6, always single-object) find the first object
        // with zero FDEs (the empty-table arm).
        let n_objs = 1 + (seed % 2);
        for i in 0..2u32 {
            let obj = OBJ0 + i * 0x40;
            let low = 0x1_0000 * (i + 1);
            let count = if seed % 8 == 6 && i == 0 {
                0
            } else {
                1 + ((seed + i) % 3)
            };
            let tbl = DATA + 0x700 + i * 0x40;
            for j in 0..4u32 {
                let row = DATA + 0x800 + (i * 4 + j) * 0x10;
                let on = i < n_objs && j < count;
                let start = low + 0x100 + j * 0x200;
                c.bus.write32(row + 8, if on { start } else { 0 });
                c.bus.write32(row + 0xc, if on { 0x80 } else { 0 });
                c.bus.write32(tbl + j * 4, if on { row } else { 0 });
            }
            c.bus.write32(obj, low);
            c.bus.write32(obj + 4, low + 0x1000);
            c.bus.write32(obj + 8, 0);
            c.bus.write32(obj + 0xc, tbl);
            c.bus.write32(obj + 0x10, count);
            c.bus
                .write32(obj + 0x14, if i + 1 < n_objs { obj + 0x40 } else { 0 });
        }
        // Seed 0 mod 7: empty registry (immediate null return).
        // Seed 6 mod 7: first object uninitialized (frame_init path,
        // with one valid chain node so init succeeds; odd quarters keep
        // the table word set so the counting loop runs too).
        if seed % 7 == 0 {
            c.bus.write32(REG_HEAD, 0);
        } else {
            c.bus.write32(REG_HEAD, OBJ0);
        }
        if seed % 7 == 6 {
            c.bus.write32(OBJ0, 0);
            // One valid chain node (length 8, count/valid nonzero) plus
            // the zero terminator, so frame_init succeeds.
            c.bus.write32(DATA + 0x900, 8);
            c.bus.write32(DATA + 0x904, 1);
            c.bus.write32(DATA + 0x908, 1);
            c.bus.write32(DATA + 0x90c, 0);
            c.bus.write32(DATA + 0x910, DATA + 0x900);
            c.bus.write32(DATA + 0x914, 0);
            c.bus.write32(OBJ0 + 8, DATA + 0x900);
            if (seed / 7) % 2 == 0 {
                c.bus.write32(OBJ0 + 0xc, 0);
            } else {
                c.bus.write32(OBJ0 + 0xc, DATA + 0x910);
            }
        }
        n_objs
    }

    fn setup_paths(c: &mut Cpu, seed: u32) -> Rng {
        let r = unwind_machine(c, seed, 0x4148);
        let n_objs = build_registry(c, seed);
        // Looked-up pc: below all, row starts, row interiors, the
        // high edge, the last row start, past the end, an exact low
        // (enters the search), and a gap address (binary search runs
        // dry). Two-object seeds steer at the second object, so the
        // scan walks further. Init seeds look up the initialized low
        // so the post-init compare is exact.
        let low0 = 0x1_0000u32;
        let low = if n_objs == 2 && matches!(seed % 8, 1 | 2 | 4) {
            0x2_0000u32
        } else {
            low0
        };
        let count = if low == low0 {
            1 + (seed % 3)
        } else {
            1 + ((seed + 1) % 3)
        };
        let mut pc = match seed % 8 {
            0 => low0 - 4,
            1 => low + 0x100,
            2 => low + 0x110,
            3 => low0 + 0x1000,
            4 if count > 1 => low + 0x100 + 0x200,
            4 => low + 0x180,
            5 => low0 + 0x2000,
            6 => low,
            _ => low + 0x50,
        };
        if seed % 7 == 6 {
            // Frame_init mutates the object while learning the low, so
            // prime once to learn it, then rebuild pristine: the test
            // itself runs the init path and lands on pc == low.
            c.g[A0][0] = OBJ0 as u64;
            c.pc = 0x0041_4008;
            ee::s15::frame_init(c);
            pc = c.bus.read32(OBJ0);
            build_registry(c, seed);
        }
        c.g[A0][0] = pc as u64;
        c.pc = 0x0041_4148;
        r
    }

    #[test]
    fn find_fde_matches() {
        check_paths(
            ee::s15::f_00414148__generated,
            find_fde,
            SEEDS_2,
            &[15, 22, 33, 45, 58, 835, 851],
            &|c, seed| {
                setup_paths(c, seed);
            },
        );
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = setup_paths(c, seed);
            schedule(c, &mut r, seed, 0x0041_4148)
        };
        check_events(find_fde_gen_driven, find_fde_hand_driven, SEEDS_2, &setup);
    }

    /// Find-FDE scan driven run: look up an address below every object
    /// so the scan walks the whole two-object registry with a
    /// frame-ready event due inside it, driven through the unwind and
    /// re-entered at the scan head (counted 0x4168 resume).
    #[test]
    fn find_fde_scan_driven_matches() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let _ = unwind_machine(c, seed, 0x4148);
            // Two-object registry, lookup below all lows: the scan
            // advances twice (counted resume on the second edge).
            build_registry(c, 1);
            let saved = (c.g[A0], c.g[A1], c.g[31]);
            ssx3_ps2::kernel::start_main_thread(c, 0x0041_4148, 0, 0);
            (c.g[A0], c.g[A1], c.g[31]) = saved;
            c.g[A0][0] = 0x5000;
            c.pc = 0x0041_4148;
            c.cycles = 1000;
            c.next_event = c.cycles + 15;
            Events::Unwind
        };
        check_events(find_fde_gen_driven, find_fde_hand_driven, 8, &setup);
        // Pin the resume itself: every seed must re-enter at the scan
        // head 0x4168 on both sides with identical per-seed sequences.
        for seed in 0..8 {
            let mut g = crate::difftest::fresh();
            setup(&mut g, seed);
            let mut h = crate::difftest::fresh();
            setup(&mut h, seed);
            let (gen_heads, hand_heads) = (
                crate::game::misc::tests::drive_resumes(&mut g, ee::s15::f_00414148__generated),
                crate::game::misc::tests::drive_resumes(&mut h, find_fde),
            );
            assert_eq!(
                gen_heads, hand_heads,
                "seed {seed}: resume sequences differ"
            );
            assert!(
                hand_heads.contains(&0x0041_4168),
                "seed {seed}: no resume at the scan head 0x4168 ({hand_heads:x?})"
            );
        }
    }

    /// Find-FDE resume heads: re-enter the scan and the binary search
    /// mid-walk with a crafted mid-search state (direct entry; the
    /// counted scan unwind lands in the driven test above).
    #[test]
    fn find_fde_resume_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let _ = setup_paths(c, seed);
            // Reuse the registry the fixture built; the table is small
            // and sorted, so every head below terminates.
            let obj = OBJ0;
            let tbl = c.bus.read32(obj + 0xc);
            let count = c.bus.read32(obj + 0x10);
            let row0 = c.bus.read32(tbl);
            let start0 = c.bus.read32(row0 + 8);
            match seed % 3 {
                // Scan head mid-registry.
                0 => {
                    c.g[S0][0] = obj as u64;
                    c.g[S1][0] = (start0 + 0x10) as u64;
                    c.pc = 0x0041_4168;
                }
                // Binary search mid-point: T0 = lo, A3 = count.
                1 => {
                    c.g[T0][0] = 0;
                    c.g[A3][0] = count as u64;
                    c.g[A0][0] = tbl as u64;
                    c.g[S0][0] = obj as u64;
                    c.g[S1][0] = (start0 + 0x10) as u64;
                    c.g[V0][0] = count as u64;
                    c.pc = 0x0041_41d0;
                }
                // Found row: V1 points at it.
                _ => {
                    c.g[V1][0] = row0 as u64;
                    c.g[S0][0] = obj as u64;
                    c.g[S1][0] = (start0 + 0x10) as u64;
                    c.pc = 0x0041_41b4;
                }
            }
        };
        check(ee::s15::f_00414148__generated, find_fde, SEEDS_2, &setup);
    }

    /// Gate word arming the `exit`/`set_terminate` callee fakes below.
    pub(crate) const EXIT_GATE: u32 = DATA + 0x5050;

    pub(crate) const EXIT_MAGIC: u32 = 0x474c_5545;

    /// Gated `__do_global_dtors` (slot 0x40fb88): return at once when the
    /// exit fixture arms the gate, else run the installed body.
    pub(crate) fn exit_fake_dtors(c: &mut Cpu) {
        if c.bus.read32(EXIT_GATE) != EXIT_MAGIC {
            return fall_back(c, prev_cell(exit_fake_dtors), ee::s15::__do_global_dtors);
        }
        c.cycles += 3;
        crate::abi::ret(c);
    }

    /// Gated `_cleanup` (slot 0x416fe0): return at once when armed.
    pub(crate) fn exit_fake_cleanup(c: &mut Cpu) {
        if c.bus.read32(EXIT_GATE) != EXIT_MAGIC {
            return fall_back(c, prev_cell(exit_fake_cleanup), ee::s15::_cleanup);
        }
        c.cycles += 3;
        crate::abi::ret(c);
    }

    /// Gated `terminate` (slot 0x412800): return at once when armed (the
    /// real one re-enters `set_terminate` through the handler, which has
    /// no bottom on a test machine).
    pub(crate) fn exit_fake_terminate(c: &mut Cpu) {
        if c.bus.read32(EXIT_GATE) != EXIT_MAGIC {
            return fall_back(c, prev_cell(exit_fake_terminate), ee::s15::terminate);
        }
        c.cycles += 3;
        crate::abi::ret(c);
    }

    /// Hook the three fakes (once per process; the gate word stays
    /// per-Cpu, so unarmed tests still run the installed bodies).
    #[allow(unused_unsafe)]
    pub(crate) fn exit_install_fakes() {
        unsafe {
            hook(
                std::ptr::addr_of_mut!(ee::s15::OVERRIDE_0040FB88),
                exit_fake_dtors,
                prev_cell(exit_fake_dtors),
            );
            hook(
                std::ptr::addr_of_mut!(ee::s15::OVERRIDE_00416FE0),
                exit_fake_cleanup,
                prev_cell(exit_fake_cleanup),
            );
            hook(
                std::ptr::addr_of_mut!(ee::s15::OVERRIDE_00412800),
                exit_fake_terminate,
                prev_cell(exit_fake_terminate),
            );
        }
    }

    /// Arm the gate on this machine.
    pub(crate) fn exit_arm_fakes(c: &mut Cpu) {
        exit_install_fakes();
        c.bus.write32(EXIT_GATE, EXIT_MAGIC);
    }

    #[test]
    fn set_terminate_matches() {
        check(
            ee::s15::f_00412818__generated,
            set_terminate,
            SEEDS_9,
            &|c, seed| {
                glue_machine(c, seed, 0);
                let mut r = Rng(0x2818 ^ seed as u64);
                c.g[A0][0] = (DATA + (r.next_u32() % 0x1000)) as u64;
                c.pc = 0x0041_2818;
                exit_arm_fakes(c);
            },
        );
    }

    #[test]
    fn set_terminate_resume_matches() {
        check(
            ee::s15::f_00412818__generated,
            set_terminate,
            SEEDS_9,
            &|c, seed| {
                glue_machine(c, seed, 0);
                let mut r = Rng(0x2828 ^ seed as u64);
                c.g[A0][0] = (DATA + (r.next_u32() % 0x1000)) as u64;
                c.g[RA][0] = 0x0012_3450 + seed as u64;
                c.pc = 0x0041_2828;
                exit_arm_fakes(c);
            },
        );
    }

    /// Seeds for the RTTI fixtures.
    pub(crate) const SEEDS_10: u32 = 64;

    /// Fixture base: scrambled machine; `a0`/`a1` from independent digits.
    pub(crate) fn rtti_machine(c: &mut Cpu, seed: u32) -> Rng {
        let mut r = machine(c, seed, 0x4343, 0);
        c.g[A0][0] = (crate::difftest::DATA + 0x400 + (r.next_u32() % 0x100)) as u64;
        c.g[A1][0] = (r.next_u32() % 4) as u64;
        // Non-zero high halves (checklist (a)): slot-vs-neighbor slips show.
        c.g[A0][1] = 0x5a5a_5a5a;
        r
    }

    #[test]
    fn bad_typeid_dtor_matches() {
        check_paths(
            ee::s15::f_00415088__generated,
            bad_typeid_dtor,
            SEEDS_10,
            &[10, 156, 159, 168],
            &|c, seed| {
                rtti_machine(c, seed);
                c.g[A0][0] = heap_obj(c) as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn bad_cast_dtor_matches() {
        check_paths(
            ee::s15::f_00415120__generated,
            bad_cast_dtor,
            SEEDS_10,
            &[10, 156, 159, 168],
            &|c, seed| {
                rtti_machine(c, seed);
                c.g[A0][0] = heap_obj(c) as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn bad_typeid_ctor_matches() {
        check_paths(
            ee::s15::f_004150b8__generated,
            bad_typeid_ctor,
            SEEDS_10,
            &[5],
            &|c, seed| {
                rtti_machine(c, seed);
            },
        );
    }

    #[test]
    fn bad_cast_ctor_matches() {
        check_paths(
            ee::s15::f_00415150__generated,
            bad_cast_ctor,
            SEEDS_10,
            &[5],
            &|c, seed| {
                rtti_machine(c, seed);
            },
        );
    }

    #[test]
    fn tfq24rcmp5codec_matches() {
        check_paths(
            ee::s15::f_0042e2b0__generated,
            tfq24rcmp5codec,
            SEEDS_10,
            &[12, 23],
            &|c, seed| {
                rtti_machine(c, seed);
                // Registry slot installed on even seeds (skip the build).
                c.bus.write32(0x004a_5ba0, (seed % 2) as u32);
            },
        );
    }

    #[test]
    fn tfq24rcmp7decoder_matches() {
        check_paths(
            ee::s15::f_0042e458__generated,
            tfq24rcmp7decoder,
            SEEDS_10,
            &[12, 23],
            &|c, seed| {
                rtti_machine(c, seed);
                c.bus.write32(0x004a_5b90, (seed % 2) as u32);
            },
        );
    }

    #[test]
    fn tfq24rcmp11rcmp_system_matches() {
        check_paths(
            ee::s15::f_0042e4c8__generated,
            tfq24rcmp11rcmp_system,
            SEEDS_10,
            &[12, 23],
            &|c, seed| {
                rtti_machine(c, seed);
                c.bus.write32(0x004a_5b98, (seed % 2) as u32);
            },
        );
    }

    #[test]
    fn tf23ps2_sony_codec_internal_matches() {
        check_paths(
            ee::s15::f_0042e200__generated,
            tf23ps2_sony_codec_internal,
            SEEDS_10,
            &[17, 34, 45],
            &|c, seed| {
                rtti_machine(c, seed);
                // Top slot installed every fourth seed (skip everything);
                // the user slot installed on the next (skip the user
                // build); both clear runs the full build.
                c.bus.write32(0x0053_ea00, (seed % 4 == 0) as u32);
                c.bus.write32(0x004a_5ba0, (seed % 4 == 1) as u32);
            },
        );
    }

    /// Gate word arming the `__rtti_user` upper-half probe below.
    pub(crate) const RTTI_GATE: u32 = DATA + 0x5070;

    pub(crate) const RTTI_MAGIC: u32 = 0x5254_4955;

    /// Gated `__rtti_user` probe (slot 0x414c20): with the gate armed,
    /// run the generated body (installing the record for real), then
    /// clobber the S0 upper half a 64-bit restore would drop. With the
    /// gate off, run the previously installed body.
    pub(crate) fn fake_rtti_user_upper(c: &mut Cpu) {
        if c.bus.read32(RTTI_GATE) != RTTI_MAGIC {
            return fall_back(c, prev_cell(fake_rtti_user_upper), ee::s15::__rtti_user);
        }
        ee::s15::f_00414c20__generated(c);
        c.g[S0][1] = 0xCA1F_0000_DEAD_BEEF;
    }

    /// Hook the probe once and arm the gate on this machine.
    pub(crate) fn arm_rtti_user_upper(c: &mut Cpu) {
        hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_00414C20),
            fake_rtti_user_upper,
            prev_cell(fake_rtti_user_upper),
        );
        c.bus.write32(RTTI_GATE, RTTI_MAGIC);
    }

    /// Saved-register upper halves (M1): the four RTTI getters must
    /// restore the full 128-bit callee-saved registers. The SYSTEM and
    /// Sony fixtures place the save slot over the type-info record the
    /// real callee fills, with a nonzero seeded upper half; the CODEC
    /// and DECODER fixtures run the real install through a gated probe
    /// that then clobbers the S0 upper half. A 64-bit restore keeps the
    /// clobbered half and visibly differs from the generated `lq`.
    #[test]
    fn tf_saved_upper_halves_matches() {
        // CODEC through the clobbering probe (slot clear: build runs).
        check(
            ee::s15::f_0042e2b0__generated,
            tfq24rcmp5codec,
            4,
            &|c, _seed| {
                let _ = machine(c, 0x4343, 0xe2b0, 0);
                c.g[GP][0] = GAME_GP as u64;
                c.g[S0][0] = 0x1234_5678;
                c.g[S0][1] = 0x9abc_def0_1357_2468;
                c.bus.write32(0x004a_5ba0, 0);
                arm_rtti_user_upper(c);
            },
        );
        // DECODER through the same probe.
        check(
            ee::s15::f_0042e458__generated,
            tfq24rcmp7decoder,
            4,
            &|c, _seed| {
                let _ = machine(c, 0x4343, 0xe458, 0);
                c.g[GP][0] = GAME_GP as u64;
                c.g[S0][0] = 0x1234_5678;
                c.g[S0][1] = 0x9abc_def0_1357_2468;
                c.bus.write32(0x004a_5b90, 0);
                arm_rtti_user_upper(c);
            },
        );
        // RCMP_SYSTEM with the save slot over its own record: entry SP
        // 0x4a5ba0 puts saved S0.high on the slot the real
        // `__rtti_user` fills; its low word reads 0 so the build runs.
        check(
            ee::s15::f_0042e4c8__generated,
            tfq24rcmp11rcmp_system,
            4,
            &|c, _seed| {
                let _ = machine(c, 0x4343, 0xe4c8, 0);
                c.g[GP][0] = GAME_GP as u64;
                c.g[SP][0] = 0x004a_5ba0;
                c.g[S0][0] = 0x1234_5678;
                c.g[S0][1] = 0xdead_0000_0000_0000;
            },
        );
        // Sony with the S2 save slot over its type-info record: entry
        // SP 0x53ea30, seeded S2 low 0 (build runs) and a nonzero high
        // half the real `__rtti_si` record write visibly changes.
        check(
            ee::s15::f_0042e200__generated,
            tf23ps2_sony_codec_internal,
            4,
            &|c, _seed| {
                let _ = machine(c, 0x4343, 0xe200, 0);
                c.g[GP][0] = GAME_GP as u64;
                c.g[SP][0] = 0x0053_ea30;
                c.g[S2][0] = 0;
                c.g[S2][1] = 0xbeef_0000_0000_0000;
                c.g[S0][1] = 0x5a5a_5a5a_5a5a_5a5a;
                c.g[S1][1] = 0xa5a5_a5a5_a5a5_a5a5;
            },
        );
    }
}
