//! `src/bx/bxstring.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::be::be::gp as gp_3;
pub(crate) use crate::game::be::beintscore::gp as gp_2;
pub(crate) use crate::game::fe::debugfe::r32_2;
pub(crate) use crate::game::fe::feasyncfile::at_2;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::feasyncfile::sb_r;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::fe::fereal::close;
pub(crate) use crate::game::fe::fereal::open;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::fe::ovtemplatepausemenu::sw;
pub(crate) use crate::game::main::gamemode::gpo;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::movie::movieplayer::at as at_3;
pub(crate) use crate::game::movie::movieplayer::word;
use crate::game::seg::seg_316fb8::CTYPE_TABLE;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw_2;
pub(crate) use crate::game::wscript::wscriptman::sw as sw_2;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0031_7fe8, c_bx_string_ctor_copy, "cBXString_ctor_copy"),
    (
        0x0031_8040,
        cbx_string_alloc_buffer,
        "cBXString_AllocBuffer",
    ),
    (0x0031_80b8, c_bx_string_release, "cBXString_Release"),
    (
        0x0031_8120,
        c_bx_string_copy_before_write,
        "cBXString_CopyBeforeWrite",
    ),
    (
        0x0031_8188,
        c_bx_string_alloc_before_write,
        "cBXString_AllocBeforeWrite",
    ),
    (0x0031_81e8, cbx_string_alloc_copy, "cBXString_AllocCopy"),
    (0x0031_8260, cbx_string_dtor, "cBXString_dtor"),
    (0x0031_82d8, cbx_string_ctor_cstr, "cBXString_ctor_cstr"),
    (
        0x0031_8350,
        c_bx_string_ctor_cstr_n,
        "cBXString_ctor_cstr_n",
    ),
    (0x0031_83d8, cbx_string_assign_copy, "cBXString_AssignCopy"),
    (0x0031_8448, cbxstring_assign, "cBXString_assign"),
    (0x0031_8540, c_bx_string_concat_copy, "cBXString_ConcatCopy"),
    (
        0x0031_85c8,
        c_bx_string_op_plus_str_str,
        "cBXString_OpPlusStrStr",
    ),
    (
        0x0031_8630,
        cbxstring_op_plus_str_cstr,
        "cBXString_OpPlusStrCStr",
    ),
    (
        0x0031_86d0,
        c_bx_string_op_plus_c_str_str,
        "cBXString_OpPlusCStrStr",
    ),
    (0x0031_8910, c_bx_string_append_cstr, "cBXString_AppendCStr"),
    (0x0031_89a0, cbxstring_append_str, "cBXString_AppendStr"),
    (0x0031_89d0, cbx_string_get_buffer, "cBXString_GetBuffer"),
    (
        0x0031_8a88,
        cbx_string_release_buffer,
        "cBXString_ReleaseBuffer",
    ),
    (0x0031_8d28, cbx_string_make_upper, "cBXString_MakeUpper"),
    (0x0031_8e68, c_bx_string_set_at, "cBXString_SetAt"),
    (0x0031_9028, c_bx_string_right, "cBXString_Right"),
    (0x0031_90a8, cbx_string_left, "cBXString_Left"),
    (
        0x0031_8760,
        cbx_string_op_plus_str_char,
        "cBXString_OpPlusStrChar",
    ),
    (
        0x0031_87c8,
        cbx_string_op_plus_char_str,
        "cBXString_OpPlusCharStr",
    ),
    (
        0x0031_8830,
        cbx_string_concat_in_place,
        "cBXString_ConcatInPlace",
    ),
    (0x0031_8968, cbx_string_append_char, "cBXString_AppendChar"),
    (
        0x0031_8af8,
        cbx_string_get_buffer_set_length,
        "cBXString_GetBufferSetLength",
    ),
    (0x0031_8b40, cbx_string_free_extra, "cBXString_FreeExtra"),
    (0x0031_8bc0, cbx_string_lock_buffer, "cBXString_LockBuffer"),
    (0x0031_8c18, cbx_string_find_char, "cBXString_FindChar"),
    (0x0031_8c60, cbx_string_find_one_of, "cBXString_FindOneOf"),
    (
        0x0031_8ca0,
        cbx_string_reverse_find,
        "cBXString_ReverseFind",
    ),
    (0x0031_8ce8, cbx_string_find_str, "cBXString_FindStr"),
    (0x0031_8d90, cbx_string_make_lower, "cBXString_MakeLower"),
    (
        0x0031_8df8,
        cbx_string_replace_char,
        "cBXString_ReplaceChar",
    ),
    (
        0x0031_8eb8,
        cbx_string_ctor_char_count,
        "cBXString_ctor_char_count",
    ),
    (0x0031_8f20, cbx_string_assign_char, "cBXString_assign_char"),
    (0x0031_8f58, cbx_string_mid_from, "cBXString_MidFrom"),
    (0x0031_8f90, cbx_string_mid, "cBXString_Mid"),
    (
        0x0031_9120,
        cbx_string_span_including,
        "cBXString_SpanIncluding",
    ),
    (
        0x0031_9170,
        cbx_string_span_excluding,
        "cBXString_SpanExcluding",
    ),
    (0x0031_94a8, cbx_string_format, "cBXString_Format"),
    (0x0031_94f0, cbx_string_trim_right, "cBXString_TrimRight"),
    (0x0031_9578, cbx_string_trim_left, "cBXString_TrimLeft"),
    (0x0031_91c0, cbx_string_format_v, "cBXString_FormatV"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use crate::call::Callee;
    use ssx3_ee as ee;

    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_ALLOC_BUFFER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_CTOR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BX_STRING_ALLOC_BUFFER;

    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_ALLOC_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_CTOR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_DTOR;

    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED;

    pub(crate) use crate::game::main::gamemode::callee::BUILTIN_DELETE as BUILTIN_DELETE_2;

    pub(crate) use crate::game::main::gamemode::callee::CBX_STRING_ALLOC_BUFFER;

    pub(crate) use crate::game::main::gamemode::callee::EA_MEM_FREE;

    pub(crate) use crate::game::main::gamemode::callee::MEMCPY;

    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_ALLOC_COPY;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_CTOR_COPY;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_DTOR;

    pub(crate) use crate::game::render::particle::callee::BXSTRING_ASSIGN_CSTR;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_CONCAT_IN_PLACE;

    pub(crate) use crate::game::render::particle::callee::STRLEN;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CONCAT_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CTOR_COPY as CBX_STRING_CTOR_COPY_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_DTOR as CBX_STRING_DTOR_2;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_BX_STRING_COPY_BEFORE_WRITE;

    pub(crate) const BXSTRING_ASSIGN_COPY: Callee = Callee {
        addr: 0x0031_83d8,
        f: ee::s10::cBXString_AssignCopy,
    };

    pub(crate) const CBX_STRING_GET_BUFFER: Callee = Callee {
        addr: 0x0031_89d0,
        f: ee::s10::cBXString_GetBuffer,
    };

    pub(crate) const CBX_STRING_MID: Callee = Callee {
        addr: 0x0031_8f90,
        f: ee::s10::cBXString_Mid,
    };

    pub(crate) const CBX_STRING_LEFT: Callee = Callee {
        addr: 0x0031_90a8,
        f: ee::s10::cBXString_Left,
    };

    pub(crate) const CBX_STRING_FORMAT_V: Callee = Callee {
        addr: 0x0031_91c0,
        f: ee::s10::cBXString_FormatV,
    };

    pub(crate) const MEMMOVE: Callee = Callee {
        addr: 0x0041_610c,
        f: ee::s15::memmove,
    };

    pub(crate) const MEMSET: Callee = Callee {
        addr: 0x0041_6210,
        f: ee::s15::memset,
    };

    pub(crate) const STRCHR: Callee = Callee {
        addr: 0x0041_6404,
        f: ee::s15::strchr,
    };

    pub(crate) const STRCSPN: Callee = Callee {
        addr: 0x0041_ab08,
        f: ee::s15::strcspn,
    };

    pub(crate) const STRPBRK: Callee = Callee {
        addr: 0x0041_ac40,
        f: ee::s15::strpbrk,
    };

    pub(crate) const STRRCHR: Callee = Callee {
        addr: 0x0041_acc0,
        f: ee::s15::strrchr,
    };

    pub(crate) const STRSPN: Callee = Callee {
        addr: 0x0041_ad10,
        f: ee::s15::strspn,
    };

    pub(crate) const STRSTR: Callee = Callee {
        addr: 0x0041_ad80,
        f: ee::s15::strstr,
    };
    pub(crate) use crate::game::fe::feasyncfile::callee::VSPRINTF;
    pub(crate) use crate::game::misc::callee::CBX_STRING_RELEASE_BUFFER;

    pub(crate) use crate::game::misc::callee::STRTOL;
}

// ---------------------------------------------------------------------------
// Strings.

/// `cBXString_AppendStr(cBXString* self, char** str)` (0x3189a0): append the
/// C string `*str` (its length comes from the cBXString header at `*str-8`),
/// returning `self`.
pub fn cbxstring_append_str(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A2][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A1][0] = Addr((c.g[A2][0] as u32).wrapping_sub(8)).lw(c);
        call_ptr(c, 0x0031_8830, 0x0031_89bc)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Bx strings and spheres.

/// `void cBXString_SetAt(cBXString* self, int idx, char ch)` (0x318e68): make
/// the buffer writable, then store `ch` at `idx`.
pub fn c_bx_string_set_at(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = sx(r32(c, A2) << 24);
        frame.save(c, S2, 0x10);
        let str = r32(c, A0);
        let idx = r32(c, A1);
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = sx((((c.g[S0][0] as u32) as i32) >> 24) as u32);
        callee::C_BX_STRING_COPY_BEFORE_WRITE.call(c, 0x0031_8e90)?;
        c.g[V0][0] = Addr(str).lw(c);
        frame.restore(c, S1, 0x20);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(idx));
        frame.restore_ra(c, 0);
        Addr(r32(c, V0)).sb(c, c.g[S0][0]);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S2, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// A `gp`-relative global.
#[inline(always)]
pub(crate) fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// `cBXString* cBXString_ctor_copy(cBXString* self, const cBXString* other)`
/// (0x317fe8): share `other`'s buffer when it is already shared, else copy.
pub fn c_bx_string_ctor_copy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(r32(c, A1)).lw(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0xffff_fff4).lw(c);
        let shared = (c.g[V0][0] as i64) < 0;
        c.g[S0][0] = c.g[A0][0];
        if shared {
            c.cycles += 5;
            c.g[V0][0] = Addr(gp(c, 0xda0)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            Addr(r32(c, S0)).sw(c, c.g[V0][0]);
            c.g[A1][0] = Addr(r32(c, A1)).lw(c);
            c.g[RA][0] = sx(0x0031_802c);
            callee::BXSTRING_ASSIGN_CSTR.call(c, 0x0031_802c)?;
        } else {
            c.cycles += 5;
            Addr(r32(c, S0)).sw(c, c.g[V1][0]);
            c.g[V0][0] = Addr(r32(c, V1)).off(0xffff_fff4).lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
            Addr(r32(c, V1)).off(0xffff_fff4).sw(c, c.g[V0][0]);
        }
        c.cycles += 5;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cBXString* cBXString_AppendCStr(cBXString* self, const char* str)`
/// (0x318910): append a C string in place.
pub fn c_bx_string_append_cstr(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let empty = c.g[S0][0] == 0;
        c.g[S1][0] = c.g[A0][0];
        let len;
        if empty {
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.g[A1][0] = c.g[V0][0];
            len = c.g[A1][0];
        } else {
            c.cycles += 4;
            c.g[RA][0] = sx(0x0031_8934);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::STRLEN, 0x0031_8934)?;
            c.g[A1][0] = c.g[V0][0];
            len = c.g[A1][0];
        }
        c.cycles += 9;
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = len;
        c.g[RA][0] = sx(0x0031_8950);
        callee::BXSTRING_CONCAT_IN_PLACE.call(c, 0x0031_8950)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cBXString_AllocBeforeWrite(cBXString* s, int needed)` (0x318188):
/// reallocate the string's buffer when the reference is not unique or the
/// buffer is too small for `needed` bytes.
pub fn c_bx_string_alloc_before_write(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let s = c.g[A0][0];
        let needed = c.g[A1][0];
        c.g[S0][0] = s;
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(r32_2(c, S0)).lw(c);
        c.g[V0][0] = Addr(r32_2(c, V1)).off(0xffff_fff4).lw(c);
        c.g[V0][0] = ((c.g[V0][0] as i64) < 2) as u64;
        c.g[S1][0] = needed;
        let rebuild;
        if c.g[V0][0] == 0 {
            // `refcount >= 2`: the buffer is shared, rebuild without a
            // capacity test (the generated `t = V0 == 0` arm).
            rebuild = true;
        } else {
            // Unique reference: rebuild only when the buffer is too small.
            c.cycles += 4;
            c.g[V0][0] = Addr(r32_2(c, V1)).off(0xffff_fffc).lw(c);
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S1][0] as i64)) as u64;
            rebuild = c.g[V0][0] != 0;
        }
        if rebuild {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_80b8, 0x0031_81c8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0031_8040, 0x0031_81d4)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[inline(always)]
pub(crate) fn ld(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).ld(c)
}

// ---------------------------------------------------------------------------
// cBXString refcounting.

/// `void cBXString_Release(cBXString* self)` (0x3180b8): drop the buffer's
/// reference count and free it at zero; a released string becomes the static
/// empty string (`D_004A3E90`).
pub fn c_bx_string_release(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = 0x0048_dcd4;
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        let s = lw(c, this as u32);
        c.g[V1][0] = s;
        if s != c.g[V0][0] {
            c.cycles += 4;
            c.g[V0][0] = lw(c, (s as u32).wrapping_sub(0xc));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            let live = c.g[V0][0] != 0;
            sw(c, (s as u32).wrapping_sub(0xc), c.g[V0][0]);
            let mut skip_reload = false;
            if !live {
                c.cycles += 4;
                c.g[V0][0] = lw(c, this as u32);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xc));
                let empty = c.g[A0][0] == 0;
                c.g[V0][0] = lw(c, gp_2(c, 0xda0));
                if empty {
                    skip_reload = true;
                } else {
                    c.cycles += 2;
                    call_ptr(c, 0x0031_7e98, 0x0031_8104)?;
                }
            }
            if !skip_reload {
                c.cycles += 1;
                c.g[V0][0] = lw(c, gp_2(c, 0xda0));
            }
            c.cycles += 2;
            sw(c, this as u32, c.g[V0][0]);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cBXString_CopyBeforeWrite(cBXString* self)` (0x318120): when the
/// buffer is shared, release it, allocate a private copy and memcpy the bytes
/// (length + 1) over.
pub fn c_bx_string_copy_before_write(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S1][0] = this;
        frame.save(c, S0, 0x20);
        let s = lw(c, this as u32);
        c.g[S0][0] = s;
        // The original leaves the `refcount < 2` test in v0; the copy path
        // overwrites it with the callees' results.
        let private = (lw(c, (s as u32).wrapping_sub(0xc)) as i64) < 2;
        c.g[V0][0] = private as u64;
        if !private {
            c.cycles += 11;
            call_ptr(c, 0x0031_80b8, 0x0031_8150)?;
            c.g[A1][0] = lw(c, (s as u32).wrapping_sub(8));
            c.g[A0][0] = this;
            call_ptr(c, 0x0031_8040, 0x0031_815c)?;
            c.g[A2][0] = lw(c, (s as u32).wrapping_sub(8));
            c.g[A1][0] = s;
            c.g[A0][0] = lw(c, this as u32);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            call_ptr(c, 0x0041_605c, 0x0031_8170)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cBXString* cBXString_OpPlusStrStr(cBXString* self, cBXString* a,
/// cBXString* b)` (0x3185c8): concatenate the two strings' buffers into a
/// temporary, copy-construct `self` from it and destroy the temporary.
pub fn c_bx_string_op_plus_str_str(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        c.g[V1][0] = lw(c, gp_2(c, 0xda0));
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        frame_save_16(c);
        c.g[S0][0] = c.g[A0][0];
        sw(c, c.g[SP][0] as u32, c.g[V1][0]);
        c.g[A0][0] = c.g[SP][0];
        c.g[V1][0] = lw(c, c.g[A1][0] as u32);
        c.g[V0][0] = lw(c, c.g[A2][0] as u32);
        c.g[A1][0] = lw(c, (c.g[V1][0] as u32).wrapping_sub(8));
        c.g[A2][0] = c.g[V1][0];
        c.g[A3][0] = lw(c, (c.g[V0][0] as u32).wrapping_sub(8));
        c.g[T0][0] = c.g[V0][0];
        call_ptr(c, 0x0031_8540, 0x0031_8600)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0031_7fe8, 0x0031_860c)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0031_8618)?;
        c.g[V0][0] = c.g[S0][0];
        c.g[RA][0] = ld(c, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

/// `sd ra, 0x10(sp); sq s0, 0x20(sp)` of [`c_bx_string_op_plus_str_str`].
#[inline(always)]
pub(crate) fn frame_save_16(c: &mut Cpu) {
    let a = c.g[SP][0] as u32;
    c.sq(16, a.wrapping_add(0x20));
    let ra = c.g[RA][0];
    c.sd(a.wrapping_add(0x10), ra);
}

// ---------------------------------------------------------------------------
// String, world-cache, replay and camera helpers.

/// `void cBXString_MakeUpper(cBXString* self)` (0x318d28): copy the buffer if
/// shared, then uppercase every character (`tolower`-table bit 1).
pub fn cbx_string_make_upper(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        open(c, 0x20);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x0031_8d3c);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0031_8120, 0x0031_8d3c)?;
        c.g[S0][0] = c.lw(c.g[S0][0] as u32);
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x8));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
        let empty = (c.g[A2][0] as i64) < 0;
        c.g[V0][0] = 0x4a0000;
        if !empty {
            c.cycles += 2;
            // 0x499c81: the character-class table (bit 1 = lowercase).
            c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x637f));
            loop {
                c.cycles += 10;
                c.g[A0][0] = c.lb(c.g[S0][0] as u32);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x20));
                c.g[V0][0] = c.lbu(c.g[A1][0] as u32);
                c.g[V0][0] = c.g[V0][0] & 0x2;
                if c.g[V0][0] == 0 {
                    c.g[V1][0] = c.g[A0][0];
                }
                c.sb(c.g[S0][0] as u32, c.g[V1][0]);
                let more = (c.g[A2][0] as i64) >= 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0031_8d58);
                }
            }
        }
        c.cycles += 4;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x20);
        Ok(())
    })
}

/// `void cBXString_AssignCopy(cBXString* self, int len, const char* str)`
/// (0x3183d8): make room, copy `len` bytes and terminate, unless the string
/// is the shared empty buffer.
pub fn cbx_string_assign_copy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        call_ptr(c, 0x0031_8188, 0x0031_83fc)?;
        c.g[V0][0] = 0x48dc_d4;
        c.g[A0][0] = Addr(at_2(c, S0, 0)).lw(c);
        let shared = c.g[A0][0] == c.g[V0][0];
        c.g[A1][0] = c.g[S2][0];
        if !shared {
            c.cycles += 7;
            c.g[A2][0] = c.g[S1][0];
            call_ptr(c, 0x0041_605c, 0x0031_8418)?;
            c.g[V1][0] = Addr(at_2(c, S0, 0)).lw(c);
            Addr((c.g[V1][0] as u32).wrapping_sub(8)).sw(c, c.g[S1][0]);
            c.g[V0][0] = Addr(at_2(c, S0, 0)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
            Addr(c.g[V0][0] as u32).sb(c, 0);
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

/// `void cBXString_ReleaseBuffer(cBXString* self, int len)` (0x318a88):
/// detach the buffer, shrink it to `len` (`-1` uses the current length) and
/// terminate.
pub fn cbx_string_release_buffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, 0x0031_8120, 0x0031_8aa4)?;
        c.g[V0][0] = u64::MAX;
        let given = c.g[S0][0] != c.g[V0][0];
        c.g[V0][0] = 0x4900_00;
        if !given {
            c.cycles += 4;
            c.g[A0][0] = Addr(at_2(c, S1, 0)).lw(c);
            call_ptr(c, 0x0041_6810, 0x0031_8ab8)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[V0][0] = 0x4900_00;
        }
        c.cycles += 4;
        c.g[V1][0] = Addr(at_2(c, S1, 0)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x232c));
        if c.g[V1][0] != c.g[V0][0] {
            c.cycles += 5;
            Addr((c.g[V1][0] as u32).wrapping_sub(8)).sw(c, c.g[S0][0]);
            c.g[V0][0] = Addr(at_2(c, S1, 0)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
            Addr(c.g[V0][0] as u32).sb(c, 0);
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cBXString_Realloc` (0x318040): `cBXString_AllocBuffer`. A zero size just
/// points the string at the shared empty buffer (`D_004A3E90`); otherwise
/// allocate `size + 0xd` bytes with `operator new(size, D_0048D850,
/// 0x20000000, 0)`, write the three-word header (refcount 1, size, capacity)
/// in front of the characters and terminate the string.
pub fn cbx_string_alloc_buffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        if c.g[S0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = lw_r(c, GP, 0xda0);
            sw_r(c, S1, 0, c.g[V0][0]);
        } else {
            c.cycles += 14;
            c.g[A1][0] = 0x49_0000;
            c.g[A0][0] = sx(low(c, S0).wrapping_add(0xd));
            c.g[A1][0] = sx(low(c, A1).wrapping_sub(0x27b0));
            c.g[A2][0] = 0x2000_0000;
            c.g[RA][0] = sx(0x0031_8080);
            c.g[A3][0] = 0;
            callee::OPERATOR_NEW_NAMED.call(c, 0x0031_8080)?;
            c.g[V1][0] = 0x1;
            c.g[A0][0] = sx(low(c, V0).wrapping_add(0xc));
            sw_r(c, V0, 0, c.g[V1][0]);
            c.g[A1][0] = sx(low(c, A0).wrapping_add(low(c, S0)));
            sw_r(c, S1, 0, c.g[A0][0]);
            sb_r(c, A1, 0, 0);
            sw_r(c, V0, 0x8, c.g[S0][0]);
            sw_r(c, V0, 0x4, c.g[S0][0]);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cBXString_Left` (0x3190a8): the first `count` characters of `src` (a
/// negative count means "all"; a longer count is clamped to the source
/// length stored in front of its characters, `*(int*)(*(char**)src - 8)`).
/// The copy is built in a stack temporary and moved into `dst`.
/// Signature: `cBXString* left(cBXString* dst, cBXString* src, int count)`.
pub fn cbx_string_left(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        if (c.g[A2][0] as i64) >= 0 {
            c.cycles += 4;
            c.g[V0][0] = Addr(low(c, A1)).lw(c);
            c.g[A0][0] = Addr(low(c, V0).wrapping_sub(8)).lw(c);
            let shorter = (c.g[A0][0] as i64) < (c.g[A2][0] as i64);
            c.g[V1][0] = shorter as u64;
            if shorter {
                c.g[A2][0] = c.g[A0][0];
            }
        } else {
            c.cycles += 2;
            c.g[A2][0] = 0;
        }
        c.cycles += 18;
        c.g[V0][0] = lw_r(c, GP, 0xda0);
        c.g[A0][0] = c.g[A1][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[A3][0] = 0;
        sw_r(c, SP, 0, c.g[V0][0]);
        c.g[RA][0] = sx(0x0031_90f0);
        c.g[T0][0] = 0;
        callee::CBX_STRING_ALLOC_COPY.call(c, 0x0031_90f0)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x0031_90fc);
        c.g[A0][0] = c.g[S0][0];
        callee::CBX_STRING_CTOR_COPY.call(c, 0x0031_90fc)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x0031_9108);
        c.g[A1][0] = 0x2;
        callee::CBX_STRING_DTOR.call(c, 0x0031_9108)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0x10);
        frame.restore(c, S0, 0x20);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `void cBXString_AllocCopy(dst, src, count, start, extra)` (0x3181e8):
/// allocate `count + extra` characters and copy them from `src->chars +
/// start`; a zero total aliases the shared empty string.
pub fn cbx_string_alloc_copy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x50));
        c.sq(17, sp_2(c).wrapping_add(0x30));
        c.sq(16, sp_2(c).wrapping_add(0x40));
        c.g[S1][0] = c.g[A2][0];
        c.sq(18, sp_2(c).wrapping_add(0x20));
        c.g[V0][0] = sx(low(c, S1).wrapping_add(low(c, T0)));
        c.sq(19, sp_2(c).wrapping_add(0x10));
        c.g[S2][0] = c.g[A0][0];
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A1][0];
        let empty = c.g[V0][0] != 0;
        c.g[S3][0] = c.g[A3][0];
        if empty {
            c.cycles += 8;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            callee::CBX_STRING_ALLOC_BUFFER.call(c, 0x0031_8230)?;
            c.g[A1][0] = Addr(low(c, S2)).lw(c);
            c.g[A2][0] = c.g[S1][0];
            c.g[A0][0] = Addr(low(c, S0)).lw(c);
            c.g[A1][0] = sx(low(c, A1).wrapping_add(low(c, S3)));
            call_ptr(c, callee::MEMCPY, 0x0031_8244)?;
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(gpo(c, 0xda0)).lw(c);
            sw_r(c, S0, 0, c.g[V0][0]);
        }
        c.cycles += 7;
        c.lq(16, sp_2(c).wrapping_add(0x40));
        c.lq(17, sp_2(c).wrapping_add(0x30));
        c.lq(18, sp_2(c).wrapping_add(0x20));
        c.lq(19, sp_2(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// `void cBXString_dtor(self, flags)` (0x318260): drop the character buffer's
/// reference count, free it at zero and delete the object when `flags & 1`.
pub fn cbx_string_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x30));
        c.g[V0][0] = 0x49_0000;
        c.sq(16, sp_2(c).wrapping_add(0x20));
        c.g[V0][0] = sx(low(c, V0).wrapping_sub(0x232c));
        c.sq(17, sp_2(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[V1][0] = Addr(low(c, S0)).lw(c);
        let shared = c.g[V1][0] == c.g[V0][0];
        c.g[S1][0] = c.g[A1][0];
        let mut extra = true;
        let result;
        if shared {
            result = low(c, S1) & 1;
        } else {
            c.cycles += 4;
            c.g[V0][0] = Addr(low(c, V1).wrapping_sub(0xc)).lw(c);
            c.g[V0][0] = sx(low(c, V0).wrapping_sub(1));
            let alive = (c.g[V0][0] as i64) > 0;
            sw_r(c, V1, 0xffff_fff4, c.g[V0][0]);
            if alive {
                result = low(c, S1) & 1;
            } else {
                c.cycles += 4;
                c.g[V0][0] = Addr(low(c, S0)).lw(c);
                c.g[A0][0] = sx(low(c, V0).wrapping_sub(0xc));
                let null = c.g[A0][0] == 0;
                result = low(c, S1) & 1;
                c.g[V0][0] = c.g[S1][0] & 0x1;
                if null {
                    extra = false;
                } else {
                    c.cycles += 2;
                    callee::EA_MEM_FREE.call(c, 0x0031_82b0)?;
                }
            }
        }
        if extra {
            c.cycles += 1;
        }
        c.g[V0][0] = result as u64;
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.lq(16, sp_2(c).wrapping_add(0x20));
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            callee::BUILTIN_DELETE_2.call(c, 0x0031_82c4)?;
            c.lq(16, sp_2(c).wrapping_add(0x20));
        }
        c.cycles += 4;
        c.lq(17, sp_2(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `cBXString* cBXString_ctor_cstr(self, str)` (0x3182d8): point at the shared
/// empty string, then copy `strlen(str)` characters when `str` is non-empty.
pub fn cbx_string_ctor_cstr(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x40));
        c.g[V0][0] = Addr(gpo(c, 0xda0)).lw(c);
        c.sq(18, sp_2(c).wrapping_add(0x10));
        c.sq(17, sp_2(c).wrapping_add(0x20));
        c.g[S2][0] = c.g[A0][0];
        c.sq(16, sp_2(c).wrapping_add(0x30));
        c.g[S1][0] = c.g[A1][0];
        c.sd(sp_2(c), c.g[RA][0]);
        let null = c.g[S1][0] == 0;
        sw_r(c, S2, 0, c.g[V0][0]);
        if null {
            c.cycles += 1;
            c.g[S0][0] = 0;
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, callee::STRLEN, 0x0031_8308)?;
            c.g[S0][0] = c.g[V0][0];
        }
        c.cycles += 2;
        c.g[A0][0] = c.g[S2][0];
        if c.g[S0][0] != 0 {
            c.cycles += 6;
            c.g[A1][0] = c.g[S0][0];
            callee::CBX_STRING_ALLOC_BUFFER.call(c, 0x0031_8324)?;
            c.g[A0][0] = Addr(low(c, S2)).lw(c);
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, callee::MEMCPY, 0x0031_8334)?;
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S2][0];
        c.lq(16, sp_2(c).wrapping_add(0x30));
        c.lq(17, sp_2(c).wrapping_add(0x20));
        c.lq(18, sp_2(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `void cBXString_Right(dst, src, n)` (0x00319028): dst = the last `n`
/// characters of the BX string `src` (all of it when `n` is negative).
pub fn c_bx_string_right(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        if (c.g[A2][0] as i64) < 0 {
            c.cycles += 2;
            c.g[A2][0] = 0;
        } else {
            c.cycles += 4;
            c.g[V0][0] = Addr(at_3(c, A1, 0)).lw(c);
            c.g[A0][0] = Addr(word(c, V0).wrapping_sub(8)).lw(c);
            c.g[V1][0] = (c.g[A0][0] < c.g[A2][0]) as u64;
            if c.g[V1][0] != 0 {
                c.g[A2][0] = c.g[A0][0];
            }
        }
        c.cycles += 20;
        c.g[V0][0] = Addr(gp(c, 0xda0)).lw(c);
        c.g[A0][0] = c.g[A1][0];
        c.g[V1][0] = Addr(at_3(c, A1, 0)).lw(c);
        c.g[T0][0] = 0;
        Addr(at_3(c, SP, 0)).sw(c, c.g[V0][0]);
        c.g[A1][0] = c.g[SP][0];
        c.g[A3][0] = Addr(word(c, V1).wrapping_sub(8)).lw(c);
        c.g[A3][0] = sx(word(c, A3).wrapping_sub(word(c, A2)));
        callee::BX_STRING_ALLOC_COPY.call(c, 0x0031_9078)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        callee::BX_STRING_CTOR_COPY.call(c, 0x0031_9084)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 0x2;
        callee::BX_STRING_DTOR.call(c, 0x0031_9090)?;
        c.g[V0][0] = c.g[S0][0];
        c.g[RA][0] = c.ld(frame.at(c, 0x10));
        c.lq(16, frame.at(c, 0x20));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BX strings.

/// `cBXString* cBXString_ctor_cstr_n(cBXString* self, const char* str, int n)`
/// (0x318350): copy at most `n` bytes of `str` into a new BX string.
pub fn c_bx_string_ctor_cstr_n(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = Addr(gp(c, 0xda0)).lw(c);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        Addr(r32(c, S2)).sw(c, c.g[V0][0]);
        c.g[RA][0] = sx(0x0031_8380);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0041_6810, 0x0031_8380)?;
        c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0031_8394);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0041_6810, 0x0031_8394)?;
            c.g[S0][0] = c.g[V0][0];
        }
        c.cycles += 2;
        let empty = c.g[S0][0] == 0;
        c.g[A0][0] = c.g[S2][0];
        if !empty {
            c.cycles += 6;
            c.g[RA][0] = sx(0x0031_83a8);
            c.g[A1][0] = c.g[S0][0];
            callee::BX_STRING_ALLOC_BUFFER.call(c, 0x0031_83a8)?;
            c.g[A0][0] = Addr(r32(c, S2)).lw(c);
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x0031_83b8);
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, 0x0041_605c, 0x0031_83b8)?;
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cBXString* cBXString_ConcatCopy(cBXString* self, int len1, const char* s1,
/// int len2, const char* s2)` (0x318540): a new string of the two pieces (the
/// generated call order puts the lengths in `a1`/`a3`).
pub fn c_bx_string_concat_copy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x50);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S2, 0x30);
        c.g[A1][0] = sx(p(r32(c, S1), r32(c, S3)));
        frame.save(c, S4, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        let empty = c.g[A1][0] == 0;
        c.g[S4][0] = c.g[T0][0];
        if !empty {
            c.cycles += 11;
            c.g[RA][0] = sx(0x0031_8580);
            callee::BX_STRING_ALLOC_BUFFER.call(c, 0x0031_8580)?;
            c.g[A0][0] = Addr(r32(c, S0)).lw(c);
            c.g[A1][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x0031_8590);
            c.g[A2][0] = c.g[S1][0];
            call_ptr(c, 0x0041_605c, 0x0031_8590)?;
            c.g[A0][0] = Addr(r32(c, S0)).lw(c);
            c.g[A1][0] = c.g[S4][0];
            c.g[A2][0] = c.g[S3][0];
            c.g[RA][0] = sx(0x0031_85a4);
            c.g[A0][0] = sx(p(r32(c, A0), r32(c, S1)));
            call_ptr(c, 0x0041_605c, 0x0031_85a4)?;
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cBXString.

/// `cBXString* cBXString_OpPlusCStrStr(cBXString* self, char* str,
/// cBXString* b)` (0x3186d0): `*self = str + *b` through a temporary string.
pub fn c_bx_string_op_plus_c_str_str(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = Addr(gp_3(c, 0xda0)).lw(c);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[S2][0] = c.g[A0][0];
        Addr(r32(c, SP)).sw(c, c.g[V0][0]);
        let mut len = 0u64;
        if c.g[S0][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0041_6810, 0x0031_8704)?;
            len = c.g[V0][0];
        } else {
            c.cycles += 1;
        }
        c.g[A1][0] = len;
        c.cycles += 19;
        c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[A3][0] = Addr(r32(c, V0).wrapping_sub(8)).lw(c);
        c.g[T0][0] = c.g[V0][0];
        callee::CBX_STRING_CONCAT_COPY.call(c, 0x0031_8728)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S2][0];
        callee::CBX_STRING_CTOR_COPY_2.call(c, 0x0031_8734)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 2;
        callee::CBX_STRING_DTOR_2.call(c, 0x0031_8740)?;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Strings.

/// `cBXString& cBXString::operator=(const cBXString& other)` (0x318448):
/// share the other string when both headers are writable, otherwise copy it.
pub fn cbxstring_assign(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V1][0] = lw_2(c, c.g[S1][0] as u32);
        c.g[A0][0] = lw_2(c, c.g[S0][0] as u32);
        let same = c.g[V1][0] == c.g[A0][0];
        c.g[V0][0] = c.g[S1][0];
        if same {
            c.cycles += 5;
            frame.restore(c, S0, 0x20);
            frame.restore(c, S1, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = lw_2(c, p(c.g[V1][0] as u32, 0xffff_fff4));
        let mut copy = false;
        if (c.g[V0][0] as i64) >= 0 {
            c.g[V0][0] = lw_2(c, p(c.g[A0][0] as u32, 0xffff_fff4));
            c.cycles += 2;
            if (c.g[V0][0] as i64) < 0 {
                c.cycles += 1;
                c.g[A2][0] = lw_2(c, c.g[S0][0] as u32);
                copy = true;
            }
        } else {
            c.cycles += 4;
            c.g[V0][0] = 0x0048_dcd4;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[A2][0] = lw_2(c, c.g[S0][0] as u32);
                copy = true;
            } else {
                c.cycles += 1;
                c.g[V0][0] = lw_2(c, p(c.g[A0][0] as u32, 0xffff_fff4));
                c.cycles += 2;
                copy = (c.g[V0][0] as i64) < 0;
            }
        }
        if copy {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = lw_2(c, p(c.g[A2][0] as u32, 0xffff_fff8));
            call_ptr(c, 0x0031_83d8, 0x0031_84a8)?; // AssignCopy
            c.g[V0][0] = c.g[S1][0];
        } else {
            c.cycles += 8;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_80b8, 0x0031_84b8)?; // Release
            c.g[V1][0] = lw_2(c, c.g[S0][0] as u32);
            sw_2(c, c.g[S1][0] as u32, c.g[V1][0]);
            c.g[V0][0] = lw_2(c, p(c.g[V1][0] as u32, 0xffff_fff4));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            sw_2(c, p(c.g[V1][0] as u32, 0xffff_fff4), c.g[V0][0]);
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cBXString cBXString::operator+(const char* rhs)` (0x318630): concatenate
/// `rhs` after this string and return the result object.
pub fn cbxstring_op_plus_str_cstr(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = lw_2(c, gp(c, 0xda0));
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S3][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        c.g[V1][0] = lw_2(c, c.g[S1][0] as u32);
        sw_2(c, c.g[SP][0] as u32, c.g[V0][0]);
        let nil = c.g[S0][0] == 0;
        c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_sub(8));
        let len: u64;
        if !nil {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0041_6810, 0x0031_8670)?; // strlen
            len = c.g[V0][0];
        } else {
            c.cycles += 2;
            len = 0;
        }
        c.g[A2][0] = lw_2(c, c.g[S1][0] as u32);
        c.cycles += 19;
        c.g[T0][0] = c.g[S0][0];
        c.g[A1][0] = lw_2(c, c.g[S2][0] as u32);
        c.g[A0][0] = c.g[SP][0];
        c.g[A3][0] = len;
        call_ptr(c, 0x0031_8540, 0x0031_8694)?; // ConcatCopy
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, 0x0031_7fe8, 0x0031_86a0)?; // ctor_copy
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0031_86ac)?; // dtor
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// cBXString.

/// `char* cBXString_GetBuffer(cBXString* s, int min_len)` (0x3189d0): return a
/// writable buffer of at least `min_len` bytes, copying a shared buffer and
/// growing a short one first.
pub fn cbx_string_get_buffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        'outer: {
            'shared: {
                frame.save(c, S1, 0x30);
                frame.save(c, S2, 0x20);
                let s = r(c, A0);
                c.g[S1][0] = sx(s);
                frame.save(c, S0, 0x40);
                frame.save_ra(c, 0x10);
                c.g[V1][0] = Addr(s).lw(c);
                c.g[V0][0] = Addr(p(r(c, V1), (-0xc_i32) as u32)).lw(c);
                c.g[V0][0] = ((c.g[V0][0] as i64) < 2) as u64;
                let shared = c.g[V0][0] == 0;
                c.g[S2][0] = c.g[A1][0];
                if shared {
                    break 'shared;
                }
                c.cycles += 4;
                c.g[V0][0] = Addr(p(r(c, V1), (-4i32) as u32)).lw(c);
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = Addr(r(c, S1)).lw(c);
                    break 'outer;
                }
            }
            c.cycles += 25;
            c.g[A0][0] = sx(r(c, SP));
            c.g[RA][0] = sx(0x0031_8a18);
            c.g[A1][0] = c.g[S1][0];
            callee::BXSTRING_CTOR_COPY.call(c, 0x0031_8a18)?;
            c.g[V1][0] = Addr(r(c, SP)).lw(c);
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x0031_8a28);
            c.g[S0][0] = Addr(p(r(c, V1), (-8i32) as u32)).lw(c);
            callee::BXSTRING_RELEASE.call(c, 0x0031_8a28)?;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
            c.g[A1][0] = c.g[S2][0];
            if c.g[V0][0] == 0 {
                c.g[A1][0] = c.g[S0][0];
            }
            c.g[RA][0] = sx(0x0031_8a3c);
            c.g[A0][0] = c.g[S1][0];
            callee::BXSTRING_ALLOC_BUFFER.call(c, 0x0031_8a3c)?;
            c.g[A0][0] = Addr(r(c, S1)).lw(c);
            c.g[A2][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0031_8a4c);
            c.g[A1][0] = Addr(r(c, SP)).lw(c);
            call_ptr(c, 0x0041_605c, 0x0031_8a4c)?;
            c.g[V1][0] = Addr(r(c, S1)).lw(c);
            c.g[A0][0] = sx(r(c, SP));
            c.g[A1][0] = 0x2;
            Addr(p(r(c, V1), (-8i32) as u32)).sw(c, c.g[S0][0]);
            c.g[V0][0] = Addr(r(c, S1)).lw(c);
            c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, S0)));
            c.g[RA][0] = sx(0x0031_8a6c);
            Addr(r(c, V0)).sb(c, 0);
            callee::BXSTRING_DTOR.call(c, 0x0031_8a6c)?;
            c.g[V0][0] = Addr(r(c, S1)).lw(c);
        }
        c.cycles += 6;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// String concatenation operators.

/// gp-relative slot of the shared empty string a new `cBXString` starts
/// from.
pub(crate) const GP_EMPTY_STRING: u32 = 0xda0;

/// `cBXString *cBXString_OpPlusStrChar(cBXString *self, cBXString *a, char ch)`
/// (0x318760): `*self = *a + ch`, built in a stack temporary from the
/// concatenation of `a`'s bytes and the single character.
pub fn cbx_string_op_plus_str_char(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        c.g[V0][0] = Addr(r32(c, GP).wrapping_add(GP_EMPTY_STRING)).lw(c);
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[A3][0] = 1;
        frame.save_ra(c, 0x20);
        c.g[S0][0] = c.g[A0][0];
        Addr(sp(c).wrapping_add(0x10)).sb(c, c.g[A2][0]);
        c.g[A0][0] = c.g[SP][0];
        Addr(sp(c)).sw(c, c.g[V0][0]);
        c.g[T0][0] = sx(sp(c).wrapping_add(0x10));
        c.g[V0][0] = Addr(r32(c, A1)).lw(c);
        c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_sub(8)).lw(c);
        c.g[A2][0] = c.g[V0][0];
        callee::CBX_STRING_CONCAT_COPY.call(c, 0x0031_8798)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        callee::BXSTRING_CTOR_COPY.call(c, 0x0031_87a4)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 2;
        callee::BXSTRING_DTOR.call(c, 0x0031_87b0)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0x20);
        frame.restore(c, S0, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `cBXString *cBXString_OpPlusCharStr(cBXString *self, char ch, cBXString *b)`
/// (0x3187c8): `*self = ch + *b`, built in a stack temporary from the
/// concatenation of the single character and `b`'s bytes.
pub fn cbx_string_op_plus_char_str(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        c.g[V0][0] = Addr(r32(c, GP).wrapping_add(GP_EMPTY_STRING)).lw(c);
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0x20);
        c.g[S0][0] = c.g[A0][0];
        Addr(sp(c).wrapping_add(0x10)).sb(c, c.g[A1][0]);
        c.g[A0][0] = c.g[SP][0];
        Addr(sp(c)).sw(c, c.g[V0][0]);
        c.g[A1][0] = 1;
        c.g[V0][0] = Addr(r32(c, A2)).lw(c);
        c.g[A2][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A3][0] = Addr((c.g[V0][0] as u32).wrapping_sub(8)).lw(c);
        c.g[T0][0] = c.g[V0][0];
        callee::CBX_STRING_CONCAT_COPY.call(c, 0x0031_8800)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        callee::BXSTRING_CTOR_COPY.call(c, 0x0031_880c)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 2;
        callee::BXSTRING_DTOR.call(c, 0x0031_8818)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0x20);
        frame.restore(c, S0, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void cBXString_ConcatInPlace(cBXString *self, int len, const char *str)`
/// (0x318830): append `len` bytes of `str` to `self`, either by copying
/// into the spare capacity of a uniquely held buffer or by rebuilding
/// through a copy when the buffer is shared or too small.
pub fn cbx_string_concat_in_place(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x50);
        c.g[S3][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        // The buffer header below the text: refcount at -0xc, length at -8,
        // capacity at -4.
        let text = Addr(r32(c, S1)).lw(c);
        let shared = (Addr((text as u32).wrapping_sub(0xc)).lw(c) as i64) >= 2;
        c.g[A3][0] = text;
        c.g[V0][0] = ((Addr((text as u32).wrapping_sub(0xc)).lw(c) as i64) < 2) as u64;
        c.g[A0][0] = sx((text as u32).wrapping_sub(0xc));
        let mut slow = shared;
        if !slow {
            c.cycles += 6;
            c.g[V0][0] = Addr((text as u32).wrapping_sub(8)).lw(c);
            c.g[V1][0] = Addr((text as u32).wrapping_sub(4)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, S2)));
            c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V1][0] == 0 {
                c.g[A0][0] = Addr(r32(c, A0).wrapping_add(4)).lw(c);
            } else {
                slow = true;
            }
        }
        if slow {
            // Shared buffer, or no room: copy self aside, reset, concatenate
            // the old bytes with the new ones, drop the copy.
            c.cycles += 17;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = c.g[S1][0];
            callee::BXSTRING_CTOR_COPY.call(c, 0x0031_888c)?;
            c.g[S0][0] = Addr(r32(c, S1)).lw(c);
            c.g[A0][0] = c.g[S1][0];
            callee::BXSTRING_RELEASE.call(c, 0x0031_8898)?;
            c.g[A1][0] = Addr((c.g[S0][0] as u32).wrapping_sub(8)).lw(c);
            c.g[A0][0] = c.g[S1][0];
            c.g[A3][0] = c.g[S2][0];
            c.g[T0][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S0][0];
            callee::CBX_STRING_CONCAT_COPY.call(c, 0x0031_88b0)?;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = 2;
            callee::BXSTRING_DTOR.call(c, 0x0031_88bc)?;
            frame.restore(c, S0, 0x50);
        } else {
            // Unique buffer with room: memcpy the new bytes past the length,
            // bump the length and re-terminate.
            c.cycles += 13;
            c.g[A1][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S2][0];
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(r32(c, A0)));
            call_ptr(c, callee::MEMCPY, 0x0031_88d4)?;
            c.g[A0][0] = Addr(r32(c, S1)).lw(c);
            c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_sub(8)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, S2)));
            Addr((c.g[A0][0] as u32).wrapping_sub(8)).sw(c, c.g[V0][0]);
            c.g[V1][0] = Addr(r32(c, S1)).lw(c);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_sub(8)).lw(c);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sb(c.g[V1][0] as u32, 0);
            frame.restore(c, S0, 0x50);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cBXString *cBXString_AppendChar(cBXString *self, char ch)` (0x318968):
/// append the single character `ch` to `self` through the in-place
/// concatenation, returning `self`.
pub fn cbx_string_append_char(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[A2][0] = c.g[SP][0];
        c.g[S0][0] = c.g[A0][0];
        Addr(sp(c)).sb(c, c.g[A1][0]);
        frame.save_ra(c, 0x10);
        c.g[A1][0] = 1;
        callee::BXSTRING_CONCAT_IN_PLACE.call(c, 0x0031_8988)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0x10);
        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `char *cBXString_GetBufferSetLength(cBXString *self, int len)`
/// (0x318af8): detach the buffer for writing, record `len` as the new
/// length, re-terminate, and return the buffer.
pub fn cbx_string_get_buffer_set_length(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        callee::CBX_STRING_GET_BUFFER.call(c, 0x0031_8b14)?;
        c.g[V1][0] = Addr(r32(c, S1)).lw(c);
        frame.restore_ra(c, 0);
        Addr((c.g[V1][0] as u32).wrapping_sub(8)).sw(c, c.g[S0][0]);
        c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, S0)));
        c.sb(c.g[V0][0] as u32, 0);
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        frame.restore(c, S1, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void cBXString_FreeExtra(cBXString *self)` (0x318b40): when the buffer
/// holds spare capacity (length below capacity), rebuild it at exactly the
/// used length through a copy; otherwise leave it alone.
pub fn cbx_string_free_extra(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_sub(4)).lw(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_sub(8)).lw(c);
        let full = c.g[V1][0] == c.g[A0][0];
        c.g[A0][0] = c.g[SP][0];
        if !full {
            c.cycles += 16;
            c.g[A1][0] = c.g[S1][0];
            callee::BXSTRING_CTOR_COPY.call(c, 0x0031_8b70)?;
            c.g[V1][0] = Addr(sp(c)).lw(c);
            c.g[A0][0] = c.g[S1][0];
            c.g[S0][0] = Addr((c.g[V1][0] as u32).wrapping_sub(8)).lw(c);
            callee::BXSTRING_RELEASE.call(c, 0x0031_8b80)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            callee::BXSTRING_ALLOC_BUFFER.call(c, 0x0031_8b8c)?;
            c.g[A0][0] = Addr(r32(c, S1)).lw(c);
            c.g[A2][0] = c.g[S0][0];
            c.g[A1][0] = Addr(sp(c)).lw(c);
            call_ptr(c, callee::MEMCPY, 0x0031_8b9c)?;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = 2;
            callee::BXSTRING_DTOR.call(c, 0x0031_8ba8)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `char *cBXString_LockBuffer(cBXString *self)` (0x318bc0): detach the
/// buffer for writing and mark it unshareable (refcount -1), returning it.
pub fn cbx_string_lock_buffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::CBX_STRING_GET_BUFFER.call(c, 0x0031_8bd8)?;
        c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        Addr((c.g[A0][0] as u32).wrapping_sub(0xc)).sw(c, c.g[V1][0]);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// String search.

/// `int cBXString_FindChar(cBXString *self, char ch, int start)` (0x318c18):
/// the index of the first `ch` at or after `start`, or -1. Implemented
/// with the C library `strchr` over `text + start`.
pub fn cbx_string_find_char(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = sx(r32(c, A1) << 24);
        frame.save(c, S0, 0x10);
        c.g[A1][0] = sx(((r32(c, A1)) as i32 >> 24) as u32);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(r32(c, A2)));
        callee::STRCHR.call(c, 0x0031_8c3c)?;
        c.g[V1][0] = c.g[V0][0];
        if c.g[V1][0] == 0 {
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        } else {
            c.cycles += 2;
            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cBXString_FindOneOf(cBXString *self, const char *set)` (0x318c60):
/// the index of the first character from `set` in the string, or -1.
/// Implemented with the C library `strpbrk`.
pub fn cbx_string_find_one_of(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        callee::STRPBRK.call(c, 0x0031_8c78)?;
        c.g[V1][0] = c.g[V0][0];
        if c.g[V1][0] == 0 {
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        } else {
            c.cycles += 2;
            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cBXString_ReverseFind(cBXString *self, char ch)` (0x318ca0): the
/// index of the last `ch` in the string, or -1. Implemented with the C
/// library `strrchr`.
pub fn cbx_string_reverse_find(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = sx(r32(c, A1) << 24);
        frame.save(c, S0, 0x10);
        c.g[A1][0] = sx(((r32(c, A1)) as i32 >> 24) as u32);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        callee::STRRCHR.call(c, 0x0031_8cc0)?;
        c.g[V1][0] = c.g[V0][0];
        if c.g[V1][0] == 0 {
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        } else {
            c.cycles += 2;
            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cBXString_FindStr(cBXString *self, const char *needle, int start)`
/// (0x318ce8): the index of the needle's first match in the text at or
/// after `start` (via the C library `strstr` over `text + start`), or -1.
pub fn cbx_string_find_str(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(r32(c, A2)));
        callee::STRSTR.call(c, 0x0031_8d04)?;
        c.g[V1][0] = c.g[V0][0];
        if c.g[V1][0] == 0 {
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        } else {
            c.cycles += 2;
            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Case conversion and replacement (loops with event checks).

/// `void cBXString_MakeLower(cBXString *self)` (0x318d90): detach the
/// buffer for writing, then lowercase every character in place through the
/// character-class table.
pub fn cbx_string_make_lower(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::C_BX_STRING_COPY_BEFORE_WRITE.call(c, 0x0031_8da4)?;
        c.g[S0][0] = Addr(r32(c, S0)).lw(c);
        c.g[A2][0] = Addr((c.g[S0][0] as u32).wrapping_sub(8)).lw(c);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
        let empty = (c.g[A2][0] as i64) < 0;
        // The character-class table base the loop indexes with each byte.
        c.g[V0][0] = 0x4a0000;
        if !empty {
            c.cycles += 2;
            c.g[A3][0] = sx(CTYPE_TABLE);
            loop {
                c.cycles += 10;
                c.g[A0][0] = c.lb(c.g[S0][0] as u32);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
                c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = c.lbu(c.g[A1][0] as u32);
                c.g[V0][0] = c.g[V0][0] & 1;
                if c.g[V0][0] == 0 {
                    c.g[V1][0] = c.g[A0][0];
                }
                c.sb(c.g[S0][0] as u32, c.g[V1][0]);
                let more = (c.g[A2][0] as i64) >= 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_8dc0);
                    }
                } else {
                    break;
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cBXString_ReplaceChar(cBXString *self, char from, char to)`
/// (0x318df8): detach the buffer for writing, then replace every `from`
/// byte with `to` in place.
pub fn cbx_string_replace_char(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = sx(r32(c, A1) << 24);
        frame.save(c, S0, 0x30);
        c.g[A2][0] = sx(r32(c, A2) << 24);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = sx(((r32(c, A1)) as i32 >> 24) as u32);
        c.g[S1][0] = sx(((r32(c, A2)) as i32 >> 24) as u32);
        callee::C_BX_STRING_COPY_BEFORE_WRITE.call(c, 0x0031_8e24)?;
        c.g[S0][0] = Addr(r32(c, S0)).lw(c);
        c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_sub(8)).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        if (c.g[V1][0] as i64) >= 0 {
            loop {
                c.cycles += 3;
                c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                if c.g[V0][0] == c.g[S2][0] {
                    c.sb(c.g[S0][0] as u32, c.g[S1][0]);
                }
                c.cycles += 3;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                let more = (c.g[V1][0] as i64) >= 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_8e38);
                    }
                } else {
                    c.cycles += 1;
                    break;
                }
            }
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Small constructors.

/// `cBXString *cBXString_ctor_char_count(cBXString *self, char ch, int n)`
/// (0x318eb8): point `self` at the shared empty string, then when `n` is
/// positive allocate `n` bytes and fill them with `ch`.
pub fn cbx_string_ctor_char_count(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = Addr(r32(c, GP).wrapping_add(GP_EMPTY_STRING)).lw(c);
        frame.save(c, S1, 0x20);
        c.g[A1][0] = sx(r32(c, A1) << 24);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = sx(((r32(c, A1)) as i32 >> 24) as u32);
        let empty = (c.g[S0][0] as i64) <= 0;
        Addr(r32(c, S1)).sw(c, c.g[V0][0]);
        if !empty {
            c.cycles += 6;
            c.g[A1][0] = c.g[S0][0];
            callee::BXSTRING_ALLOC_BUFFER.call(c, 0x0031_8ef0)?;
            c.g[A0][0] = Addr(r32(c, S1)).lw(c);
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S0][0];
            callee::MEMSET.call(c, 0x0031_8f00)?;
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cBXString *cBXString_assign_char(cBXString *self, char ch)`
/// (0x318f20): assign the single character `ch` to `self` through the
/// copy assignment, returning `self`.
pub fn cbx_string_assign_char(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[A2][0] = c.g[SP][0];
        c.g[S0][0] = c.g[A0][0];
        Addr(sp(c)).sb(c, c.g[A1][0]);
        frame.save_ra(c, 0x10);
        c.g[A1][0] = 1;
        callee::BXSTRING_ASSIGN_COPY.call(c, 0x0031_8f40)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0x10);
        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `cBXString *cBXString_MidFrom(cBXString *self, cBXString *other,
/// int start)` (0x318f58): assign the substring of `other` from `start` to
/// its end into `self`, returning `self`.
pub fn cbx_string_mid_from(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = Addr(r32(c, A1)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A3][0] = Addr((c.g[V0][0] as u32).wrapping_sub(8)).lw(c);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(r32(c, A2)));
        callee::CBX_STRING_MID.call(c, 0x0031_8f78)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Short-string builders.

/// `cBXString* cBXString_Mid(cBXString* self, const cBXString* other,
/// int start, int count)` (0x318f90): rebuild `*self` from the slice of
/// `*other` at `start` of length `count`, clamping a negative start or count
/// to zero and the end to the string length, built in a stack temporary.
pub fn cbx_string_mid(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        // A negative count or start clamps to zero.
        let mut take = 0u64;
        if (c.g[A3][0] as i64) >= 0 {
            take = c.g[A3][0];
        }
        let mut start = c.g[A2][0];
        if (start as i64) < 0 {
            start = 0;
        }
        let this = c.g[A0][0];
        let other = c.g[A1][0];
        let len = Addr(Addr(other as u32).lw(c) as u32).off(0xffff_fff8).lw(c);
        // `end = start + take` is a 32-bit wrap; the end clamp compares it
        // signed, the start clamp compares the full 64-bit length.
        let end = (start as u32).wrapping_add(take as u32);
        if (len as i64) < (end as i32 as i64) {
            take = (len as u32).wrapping_sub(start as u32) as i32 as u64;
        }
        c.g[A3][0] = start;
        c.g[S0][0] = this;
        c.g[V0][0] = ((len as i64) < (end as i32 as i64)) as u64;
        c.g[V1][0] = ((len as i64) < (start as i64)) as u64;
        c.g[T2][0] = Addr(r32(c, GP).wrapping_add(GP_EMPTY_STRING)).lw(c);
        c.g[A0][0] = other;
        c.g[T1][0] = take;
        c.g[A2][0] = 0;
        c.g[A1][0] = c.g[SP][0];
        if c.g[V1][0] == 0 {
            c.g[A2][0] = take;
        }
        Addr(sp(c)).sw(c, c.g[T2][0]);
        c.g[T0][0] = 0;
        callee::CBX_STRING_ALLOC_COPY.call(c, 0x0031_8ff8)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        callee::BXSTRING_CTOR_COPY.call(c, 0x0031_9004)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 2;
        callee::BXSTRING_DTOR.call(c, 0x0031_9010)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0x10);
        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `cBXString* cBXString_SpanIncluding(cBXString* self, const cBXString* other,
/// const char* accept)` (0x319120): assign to `*self` the leading run of
/// `other`'s bytes that occur in `accept`.
pub fn cbx_string_span_including(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = c.g[A2][0];
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        callee::STRSPN.call(c, 0x0031_9144)?;
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S1][0];
        callee::CBX_STRING_LEFT.call(c, 0x0031_9154)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cBXString* cBXString_SpanExcluding(cBXString* self, const cBXString* other,
/// const char* reject)` (0x319170): assign to `*self` the leading run of
/// `other`'s bytes that do not occur in `reject`.
pub fn cbx_string_span_excluding(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = c.g[A2][0];
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        callee::STRCSPN.call(c, 0x0031_9194)?;
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S1][0];
        callee::CBX_STRING_LEFT.call(c, 0x0031_91a4)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cBXString_Format(cBXString* self, const char* fmt, ...)` (0x3194a8):
/// spill the register varargs (a2/a3/t0-t3 at `sp+0x50`, f12-f18 at
/// `sp+0x40`) into its own frame and pass that area to `cBXString_FormatV`
/// as the `va_list`.
pub fn cbx_string_format(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x80);
        Addr(frame.at(c, 0x50)).sd(c, c.g[A2][0]);
        frame.save_ra(c, 0);
        c.g[A2][0] = sx(frame.at(c, 0x50));
        Addr(frame.at(c, 0x58)).sd(c, c.g[A3][0]);
        Addr(frame.at(c, 0x60)).sd(c, c.g[T0][0]);
        Addr(frame.at(c, 0x68)).sd(c, c.g[T1][0]);
        Addr(frame.at(c, 0x70)).sd(c, c.g[T2][0]);
        Addr(frame.at(c, 0x78)).sd(c, c.g[T3][0]);
        frame.save_fpr(c, 12, 0x40);
        frame.save_fpr(c, 14, 0x44);
        frame.save_fpr(c, 16, 0x48);
        frame.save_fpr(c, 18, 0x4c);
        callee::CBX_STRING_FORMAT_V.call(c, 0x0031_94e0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cBXString_TrimRight(cBXString* self)` (0x3194f0): drop the trailing
/// whitespace run (byte-class table bit 3), truncating the string in place.
pub fn cbx_string_trim_right(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::C_BX_STRING_COPY_BEFORE_WRITE.call(c, 0x0031_9504)?;
        'done: {
            c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[A3][0] = 0;
            c.g[V0][0] = c.lb(c.g[A1][0] as u32);
            c.g[A2][0] = c.lbu(c.g[A1][0] as u32);
            if c.g[V0][0] == 0 {
                break 'done;
            }
            c.cycles += 2;
            // The byte-class table base the scan indexes with each byte.
            c.g[V0][0] = 0x4a0000;
            c.g[T0][0] = sx(CTYPE_TABLE);
            loop {
                c.cycles += 11;
                c.g[V0][0] = sx((c.g[A2][0] as u32) << 24);
                if c.g[A3][0] == 0 {
                    c.g[A3][0] = c.g[A1][0];
                }
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[A0][0] = c.lb(c.g[A1][0] as u32);
                c.g[V1][0] = c.lbu(c.g[V0][0] as u32);
                c.g[A2][0] = c.lbu(c.g[A1][0] as u32);
                c.g[V1][0] = c.g[V1][0] & 0x8;
                if c.g[V1][0] == 0 {
                    c.g[A3][0] = 0;
                }
                if c.g[A0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0031_9520);
                }
            }
        }
        c.cycles += 2;
        if c.g[A3][0] != 0 {
            c.cycles += 5;
            c.sb(c.g[A3][0] as u32, 0);
            c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(c.g[V1][0] as u32));
            Addr((c.g[V1][0] as u32).wrapping_sub(8)).sw(c, c.g[V0][0]);
        }
        frame.restore(c, S0, 0x10);
        // Epilogue charge before the `ld ra`.
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cBXString_TrimLeft(cBXString* self)` (0x319578): drop the leading
/// whitespace run (byte-class table bit 3), sliding the rest down in place.
pub fn cbx_string_trim_left(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        callee::C_BX_STRING_COPY_BEFORE_WRITE.call(c, 0x0031_9590)?;
        'done: {
            c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
            c.g[V0][0] = 0x4a0000;
            c.g[A0][0] = sx(CTYPE_TABLE);
            c.g[V1][0] = c.lb(c.g[A2][0] as u32);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
            c.g[V0][0] = c.g[V0][0] & 0x8;
            if c.g[V0][0] == 0 {
                c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
                break 'done;
            }
            c.cycles += 1;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            loop {
                c.cycles += 7;
                c.g[V0][0] = c.lb(c.g[A2][0] as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = c.lbu(c.g[V0][0] as u32);
                c.g[V1][0] = c.g[V1][0] & 0x8;
                if c.g[V1][0] != 0 {
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_95b8);
                    }
                } else {
                    c.cycles += 1;
                    c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
                    break;
                }
            }
        }
        c.cycles += 13;
        c.g[A1][0] = c.g[A2][0];
        c.g[S0][0] = Addr((c.g[A0][0] as u32).wrapping_sub(8)).lw(c);
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[A0][0] as u32));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
        callee::MEMMOVE.call(c, 0x0031_95f0)?;
        c.g[V1][0] = Addr(c.g[S1][0] as u32).lw(c);
        frame.restore(c, S1, 0x10);
        Addr((c.g[V1][0] as u32).wrapping_sub(8)).sw(c, c.g[S0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cBXString_FormatV.

/// Format-specifier classes of the first jump table (base 0x48d860, indexed
/// by the character minus `'F'`): only the length modifiers skip to the next
/// character, everything else enters the `c`/`C`/`s`/`S` chain.
pub(crate) fn format_v_table1_skip(spec: u32) -> bool {
    matches!(spec, 0 | 6 | 8 | 0x22 | 0x26)
}

/// Format-specifier classes of the second jump table (base 0x48d900, indexed
/// by the character minus `'G'`): float (`e`/`f`/`g`/`G`), integer
/// (`d`/`i`/`u`/`x`/`X`/`o`/`p`), count (`n`), or nothing.
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum FormatVItem {
    Float,
    Int,
    Count,
    None,
}

pub(crate) fn format_v_table2_class(spec: u32) -> FormatVItem {
    match spec {
        0 | 30 | 31 | 32 => FormatVItem::Float,
        17 | 29 | 34 | 40 | 41 | 46 | 49 => FormatVItem::Int,
        39 => FormatVItem::Count,
        _ => FormatVItem::None,
    }
}

/// `void cBXString_FormatV(cBXString* self, const char* fmt, va_list ap)`
/// (0x3191c0): scan `fmt`, sizing the result (`maxLen`, grown per conversion
/// and flag), then `vsprintf` into a buffer of that size. The two jump
/// tables above dispatch the conversion characters; length modifiers
/// (`h`/`l`/`F`/`N`/`L`) only skip a character. `'*'` takes the width (or
/// precision) from the argument list, digits from the format otherwise.
pub fn cbx_string_format_v(c: &mut Cpu) {
    guest(c, |c| {
        const ENTRY: u32 = 0x0031_91c0;
        const SCAN_HEAD: u32 = 0x0031_9210;
        const FLAG_HEAD: u32 = 0x0031_9258;
        const END: u32 = 0x0031_94a8;
        const FRAME: u32 = 0xa0;
        // The frame stays open across unwinds, so the resume heads reuse it
        // instead of opening it again; the loop state lives in the registers
        // (S0/S1/S4) and the open frame.
        let frame = Frame { size: FRAME };
        match c.pc {
            ENTRY => {
                c.cycles += 18;
                Frame::open(c, FRAME);
                frame.save(c, S0, 0x90);
                frame.save(c, S1, 0x80);
                frame.save(c, S4, 0x50);
                frame.save(c, S5, 0x40);
                frame.save(c, S6, 0x30);
                frame.save(c, S7, 0x20);
                frame.save(c, S2, 0x70);
                frame.save(c, S3, 0x60);
                c.g[S4][0] = 0x1;
                c.g[S5][0] = c.g[A0][0];
                c.g[S6][0] = c.g[A1][0];
                c.g[S7][0] = c.g[A2][0];
                c.g[S1][0] = c.g[S7][0];
                frame.save_ra(c, 0x10);
                c.g[V0][0] = Addr(r32(c, S6)).lb(c);
                let empty = c.g[V0][0] == 0;
                c.g[S0][0] = c.g[S6][0];
                if !empty {
                    // 0x319208 runs once on entry: its only incoming edge is
                    // the entry fall-through, and every loop-back re-enters
                    // at 0x319210.
                    c.cycles += 2;
                    c.g[V1][0] = Addr(r32(c, S0)).lb(c);
                    format_v_scan(c)?;
                }
                format_v_tail(c, &frame)
            }
            // 0x319210 resume: re-executes the +3 head test; V1 already holds
            // [S0], loaded (chargeless) by the back edge before the unwind.
            SCAN_HEAD => {
                format_v_scan(c)?;
                format_v_tail(c, &frame)
            }
            // 0x319258 resume: re-executes the flag test with the flag-run
            // registers intact, then rejoins the scan loop at its back edge.
            FLAG_HEAD => {
                format_v_flags(c)?;
                format_v_conversion(c)?;
                if !format_v_scan_back_edge(c)? {
                    format_v_scan(c)?;
                }
                format_v_tail(c, &frame)
            }
            pc => {
                c.jump_out(pc, ENTRY, END);
                Ok(())
            }
        }
    })
}

/// The scan loop of [`cbx_string_format_v`] from the +3 head test (0x319210,
/// post-hoist shape): V1 already holds `[S0]` on entry. Each iteration
/// dispatches one plain character or conversion; the back edge advances and
/// either exits or unwinds at the scan head.
pub(crate) fn format_v_scan(c: &mut Cpu) -> Flow {
    loop {
        // 0x319210: plain character or conversion.
        c.cycles += 3;
        c.g[V0][0] = 0x25;
        let plain = c.g[V1][0] != c.g[V0][0];
        if plain {
            c.g[S4][0] = sx(r32(c, S4).wrapping_add(1));
        } else {
            // 0x31921c: after '%': Verbatim '%' or a conversion.
            c.cycles += 4;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
            c.g[V0][0] = Addr(r32(c, S0)).lb(c);
            let escaped = c.g[V0][0] != c.g[V1][0];
            c.g[S3][0] = 0;
            if !escaped {
                // 0x31922c: '%%'.
                c.cycles += 2;
                c.g[S4][0] = sx(r32(c, S4).wrapping_add(1));
            } else {
                // 0x319234: a conversion starts a flag run.
                c.cycles += 2;
                let end = c.g[V0][0] == 0;
                c.g[A0][0] = 0;
                if end {
                    format_v_conversion(c)?;
                } else {
                    // 0x31923c: flag characters.
                    c.cycles += 7;
                    c.g[T1][0] = 0x23;
                    c.g[T0][0] = 0x2a;
                    c.g[A3][0] = 0x2d;
                    c.g[A2][0] = 0x2b;
                    c.g[A1][0] = 0x30;
                    c.g[V1][0] = 0x20;
                    c.g[V0][0] = Addr(r32(c, S0)).lb(c);
                    format_v_flags(c)?;
                    format_v_conversion(c)?;
                }
            }
        }
        // 0x319444: scan-loop back edge.
        if format_v_scan_back_edge(c)? {
            return Ok(());
        }
    }
}

/// The scan-loop back edge of [`cbx_string_format_v`] (0x319444): advance
/// S0, reload V0 (and V1 when continuing, chargeless, as the generated
/// block does). Returns true on NUL (scan done); otherwise unwinds at the
/// scan head (0x319210) when an event asks for it.
pub(crate) fn format_v_scan_back_edge(c: &mut Cpu) -> Result<bool, Unwound> {
    c.cycles += 4;
    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
    c.g[V0][0] = Addr(r32(c, S0)).lb(c);
    if c.g[V0][0] == 0 {
        return Ok(true);
    }
    c.g[V1][0] = Addr(r32(c, S0)).lb(c);
    if back_edge_unwinds(c, 0x0031_9210) {
        return Err(Unwound);
    }
    Ok(false)
}

/// The size/format/release tail of [`cbx_string_format_v`] (0x319454):
/// size the result, `vsprintf` into it, release the buffer, restore and
/// close the frame.
pub(crate) fn format_v_tail(c: &mut Cpu, frame: &Frame) -> Flow {
    const GET_BUFFER_RET: u32 = 0x0031_9460;
    const VSPRINTF_RET: u32 = 0x0031_9470;
    const RELEASE_BUFFER_RET: u32 = 0x0031_947c;
    // 0x319454: size, format, release.
    c.cycles += 21;
    c.g[A1][0] = c.g[S4][0];
    c.g[RA][0] = sx(GET_BUFFER_RET);
    c.g[A0][0] = c.g[S5][0];
    callee::CBX_STRING_GET_BUFFER.call(c, GET_BUFFER_RET)?;
    c.g[A0][0] = Addr(r32(c, S5)).lw(c);
    c.g[A1][0] = c.g[S6][0];
    c.g[RA][0] = sx(VSPRINTF_RET);
    c.g[A2][0] = c.g[S7][0];
    call_ptr(c, callee::VSPRINTF, VSPRINTF_RET)?;
    c.g[A0][0] = c.g[S5][0];
    c.g[RA][0] = sx(RELEASE_BUFFER_RET);
    c.g[A1][0] = 0xffff_ffff_ffff_ffff;
    callee::CBX_STRING_RELEASE_BUFFER.call(c, RELEASE_BUFFER_RET)?;
    frame.restore(c, S0, 0x90);
    frame.restore(c, S1, 0x80);
    frame.restore(c, S2, 0x70);
    frame.restore(c, S3, 0x60);
    frame.restore(c, S4, 0x50);
    frame.restore(c, S5, 0x40);
    frame.restore(c, S6, 0x30);
    frame.restore(c, S7, 0x20);
    frame.restore_ra(c, 0x10);
    frame.close(c);
    Ok(())
}

/// The flag run of [`cbx_string_format_v`] from the flag test (0x319258):
/// the 0x31923c setup left the flag characters in T1/T0/A3/A2/A1/V1 and the
/// current character in V0. `'#'` and `'*'` advance through 0x3192a0; the
/// `'-'`/`'+'`/`'0'` arms advance inline (as 0x3192a4 does) and rejoin at
/// the flag test. Breaks with V0 holding the first non-flag character. Like
/// the generated 0x3192a4, unwinds at the flag head (0x319258).
pub(crate) fn format_v_flags(c: &mut Cpu) -> Flow {
    'flags: loop {
        // 0x319258.
        c.cycles += 2;
        let not_hash = c.g[V0][0] != c.g[T1][0];
        if !not_hash {
            // 0x319260: '#'.
            c.cycles += 2;
            c.g[S4][0] = sx(r32(c, S4).wrapping_add(2));
        } else {
            // 0x319268.
            c.cycles += 2;
            let not_star = c.g[V0][0] != c.g[T0][0];
            if !not_star {
                // 0x319270: '*' takes the width.
                c.cycles += 4;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(8));
                c.g[A0][0] = 0x1;
                c.g[S3][0] = Addr(r32(c, S1).wrapping_sub(8)).lw(c);
            } else {
                // 0x319280: '-'. The generated arm advances
                // `S0` itself and rejoins at `0x3192a4`,
                // skipping the `0x3192a0` (`+1`, `S0+=1`)
                // that only `'#'`, `'*'` and `' '` take.
                c.cycles += 2;
                if c.g[V0][0] == c.g[A3][0] {
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                    c.cycles += 3;
                    c.g[V0][0] = Addr(r32(c, S0)).lb(c);
                    if c.g[V0][0] == 0 {
                        break 'flags;
                    }
                    if back_edge_unwinds(c, 0x0031_9258) {
                        return Err(Unwound);
                    }
                    continue 'flags;
                } else {
                    // 0x319288: '+' (same shape as '-').
                    c.cycles += 2;
                    if c.g[V0][0] == c.g[A2][0] {
                        c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                        c.cycles += 3;
                        c.g[V0][0] = Addr(r32(c, S0)).lb(c);
                        if c.g[V0][0] == 0 {
                            break 'flags;
                        }
                        if back_edge_unwinds(c, 0x0031_9258) {
                            return Err(Unwound);
                        }
                        continue 'flags;
                    } else {
                        // 0x319290: '0' (same shape).
                        c.cycles += 2;
                        if c.g[V0][0] == c.g[A1][0] {
                            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                            c.cycles += 3;
                            c.g[V0][0] = Addr(r32(c, S0)).lb(c);
                            if c.g[V0][0] == 0 {
                                break 'flags;
                            }
                            if back_edge_unwinds(c, 0x0031_9258) {
                                return Err(Unwound);
                            }
                            continue 'flags;
                        } else {
                            // 0x319298.
                            c.cycles += 2;
                            let other = c.g[V0][0] != c.g[V1][0];
                            if other {
                                break 'flags;
                            }
                        }
                    }
                }
            }
        }
        // 0x3192a0.
        c.cycles += 1;
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
        // 0x3192a4: flag-run back edge.
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, S0)).lb(c);
        if c.g[V0][0] == 0 {
            break 'flags;
        }
        if back_edge_unwinds(c, 0x0031_9258) {
            return Err(Unwound);
        }
    }
    Ok(())
}

/// The `%`-conversion tail of [`cbx_string_format_v`] (0x3192b0-0x31933c):
/// width (a `'*'` takes it from the argument list, digits from the format),
/// precision, and the first specifier-table dispatch.
pub(crate) fn format_v_conversion(c: &mut Cpu) -> Flow {
    // 0x3192b0: width, unless '*' already gave it.
    c.cycles += 2;
    if c.g[A0][0] != 0 {
        c.g[V1][0] = Addr(r32(c, S0)).lb(c);
    } else {
        // 0x3192b8: width digits.
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x0031_92c8);
        c.g[A2][0] = 0xa;
        callee::STRTOL.call(c, 0x0031_92c8)?;
        c.g[S0][0] = Addr(sp(c)).lw(c);
        c.g[S3][0] = sx(r32(c, V0));
        c.g[V1][0] = Addr(r32(c, S0)).lb(c);
    }
    // 0x3192d8: precision.
    c.cycles += 3;
    c.g[V0][0] = 0x2e;
    let no_precision = c.g[V1][0] != c.g[V0][0];
    c.g[S2][0] = 0;
    if !no_precision {
        // 0x3192e4.
        c.cycles += 5;
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
        c.g[V1][0] = 0x2a;
        c.g[V0][0] = Addr(r32(c, S0)).lb(c);
        let prec_digits = c.g[V0][0] != c.g[V1][0];
        c.g[A0][0] = c.g[S0][0];
        if prec_digits {
            // 0x319308: precision digits.
            c.cycles += 6;
            c.g[A1][0] = c.g[SP][0] | 0x4;
            c.g[RA][0] = sx(0x0031_9314);
            c.g[A2][0] = 0xa;
            callee::STRTOL.call(c, 0x0031_9314)?;
            c.g[S0][0] = Addr(sp(c).wrapping_add(4)).lw(c);
            c.g[S2][0] = sx(r32(c, V0));
        } else {
            // 0x3192f8: '*' takes the precision.
            c.cycles += 4;
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(8));
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
            c.g[S2][0] = Addr(r32(c, S1).wrapping_sub(8)).lw(c);
        }
    }
    // 0x319320: first specifier table (base 0x48d860, index char minus 'F').
    c.cycles += 7;
    c.g[V0][0] = Addr(r32(c, S0)).lbu(c);
    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x46));
    c.g[V0][0] = sx(r32(c, V0).wrapping_shl(24));
    c.g[A0][0] = sx(((r32(c, V0) as i32) >> 24) as u32);
    c.g[V1][0] = (c.g[A0][0] < 0x27u64) as u64;
    let in_range = c.g[V1][0] != 0;
    c.g[V0][0] = 0x490000;
    if !in_range {
        // Out-of-range first-table index: the generated `0x319320` falls
        // through to the `c`/`C`/`s`/`S` chain at `0x319358`, not to the
        // second table (table base `0x48d860`, `char - 'F' >= 0x27`).
        return format_v_chain(c);
    }
    // 0x31933c: through the table (its words are in the game image).
    c.cycles += 6;
    c.g[V1][0] = sx(r32(c, A0).wrapping_shl(2));
    c.g[V0][0] = sx(0x0049_0000u32.wrapping_sub(0x27a0));
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    let spec = r32(c, A0);
    c.g[A0][0] = Addr(r32(c, V1)).lw(c);
    if format_v_table1_skip(spec) {
        // 0x319354: length modifiers only skip a character.
        c.cycles += 1;
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
    }
    format_v_chain(c)
}

/// The `c`/`C`/`s`/`S` chain of [`cbx_string_format_v`] (0x319358-0x3193c0):
/// `c`/`C` cost a fixed 2, `s`/`S` cost `max(1, strlen(arg))`, anything else
/// costs 0 here and falls into the second table.
pub(crate) fn format_v_chain(c: &mut Cpu) -> Flow {
    // 0x319358.
    c.cycles += 4;
    c.g[V1][0] = Addr(r32(c, S0)).lb(c);
    c.g[V0][0] = 0x53;
    let upper_s = c.g[V1][0] == c.g[V0][0];
    c.g[A1][0] = 0;
    if upper_s {
        // 0x3193ac: 'S'.
        c.cycles += 1;
        c.g[S1][0] = sx(r32(c, S1).wrapping_add(8));
        return format_v_strlen(c);
    }
    // 0x319368.
    c.cycles += 3;
    c.g[V0][0] = ((c.g[V1][0] as i64) < 84) as u64;
    let high = c.g[V0][0] == 0;
    c.g[V0][0] = 0x43;
    if !high {
        // 0x319374.
        c.cycles += 2;
        if c.g[V1][0] == c.g[V0][0] {
            // 'C': fixed cost 2.
            c.g[A1][0] = 0x2;
            // 0x3193a4.
            c.cycles += 2;
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(8));
        } else {
            // 0x31937c.
            c.cycles += 2;
        }
        return format_v_clamp(c);
    }
    // 0x319384.
    c.cycles += 3;
    c.g[V0][0] = 0x63;
    let lower_c = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = 0x73;
    if lower_c {
        // 0x3193a0: fixed cost 2.
        c.cycles += 1;
        c.g[A1][0] = 0x2;
        // 0x3193a4.
        c.cycles += 2;
        c.g[S1][0] = sx(r32(c, S1).wrapping_add(8));
        return format_v_clamp(c);
    }
    // 0x319390.
    c.cycles += 2;
    if c.g[V1][0] == c.g[V0][0] {
        // 's'.
        c.g[S1][0] = sx(r32(c, S1).wrapping_add(8));
        return format_v_strlen(c);
    }
    // 0x319398.
    c.cycles += 2;
    format_v_clamp(c)
}

/// The `s`/`S` arm of [`cbx_string_format_v`] (0x3193b0): `max(1, strlen)`.
pub(crate) fn format_v_strlen(c: &mut Cpu) -> Flow {
    c.cycles += 5;
    c.g[RA][0] = sx(0x0031_93b8);
    c.g[A0][0] = Addr(r32(c, S1).wrapping_sub(8)).lw(c);
    call_ptr(c, callee::STRLEN, 0x0031_93b8)?;
    c.g[A1][0] = 0x1;
    c.g[V1][0] = ((0i64) < (c.g[V0][0] as i64)) as u64;
    if c.g[V1][0] != 0 {
        c.g[A1][0] = c.g[V0][0];
    }
    format_v_clamp(c)
}

/// The item-size clamp of [`cbx_string_format_v`] (0x3193c4-0x3193d4):
/// `max(item, width)`, then `min(item, precision)` when a precision ran.
pub(crate) fn format_v_clamp(c: &mut Cpu) -> Flow {
    // 0x3193c4.
    c.cycles += 2;
    let no_item = c.g[A1][0] == 0;
    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[A1][0] as i64)) as u64;
    if no_item {
        return format_v_second(c);
    }
    // 0x3193cc.
    c.cycles += 2;
    let no_precision = c.g[S2][0] == 0;
    if c.g[V0][0] == 0 {
        c.g[A1][0] = c.g[S3][0];
    }
    if no_precision {
        return format_v_advance(c);
    }
    // 0x3193d4.
    c.cycles += 3;
    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[S2][0] as i64)) as u64;
    if c.g[V0][0] == 0 {
        c.g[A1][0] = c.g[S2][0];
    }
    format_v_advance(c)
}

/// The second specifier table of [`cbx_string_format_v`] (0x3193e0-0x319440,
/// base 0x48d900, index char minus `'G'`): floats cost at least 128, integers
/// (and `%p`) at least 32, `%n` only advances the argument list.
pub(crate) fn format_v_second(c: &mut Cpu) -> Flow {
    // 0x3193e0.
    c.cycles += 7;
    c.g[V0][0] = Addr(r32(c, S0)).lbu(c);
    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x47));
    c.g[V0][0] = sx(r32(c, V0).wrapping_shl(24));
    c.g[A0][0] = sx(((r32(c, V0) as i32) >> 24) as u32);
    c.g[V1][0] = (c.g[A0][0] < 0x32u64) as u64;
    let in_range = c.g[V1][0] != 0;
    c.g[V0][0] = 0x490000;
    if !in_range {
        return format_v_advance(c);
    }
    // 0x3193fc: through the table (its words are in the game image).
    c.cycles += 6;
    c.g[V1][0] = sx(r32(c, A0).wrapping_shl(2));
    c.g[V0][0] = sx(0x0049_0000u32.wrapping_sub(0x2700));
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    let spec = r32(c, A0);
    c.g[A0][0] = Addr(r32(c, V1)).lw(c);
    match format_v_table2_class(spec) {
        FormatVItem::Float => {
            // 0x319414.
            c.cycles += 4;
            c.g[V1][0] = sx(r32(c, S3).wrapping_add(r32(c, S2)));
            c.g[A1][0] = 0x80;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 128) as u64;
        }
        FormatVItem::Int => {
            // 0x319424.
            c.cycles += 3;
            c.g[V1][0] = sx(r32(c, S3).wrapping_add(r32(c, S2)));
            c.g[A1][0] = 0x20;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 32) as u64;
        }
        FormatVItem::Count => {
            // 0x31943c: '%n'.
            c.cycles += 1;
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(8));
            return format_v_advance(c);
        }
        FormatVItem::None => return format_v_advance(c),
    }
    // 0x319430.
    c.cycles += 3;
    c.g[S1][0] = sx(r32(c, S1).wrapping_add(8));
    if c.g[V0][0] == 0 {
        c.g[A1][0] = c.g[V1][0];
    }
    format_v_advance(c)
}

/// The per-conversion tail of [`cbx_string_format_v`] (0x319440): grow the
/// result size by the conversion's item length.
pub(crate) fn format_v_advance(c: &mut Cpu) -> Flow {
    c.cycles += 1;
    c.g[S4][0] = sx(r32(c, S4).wrapping_add(r32(c, A1)));
    Ok(())
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_paths;

    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;

    use crate::difftest::fill;

    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as TEST_RA;

    pub(crate) use crate::game::ai::computer::tests::wb;
    pub(crate) use crate::game::be::bedbreward::tests::alloc_ready as alloc_ready_2;

    pub(crate) use crate::game::fe::debugfe::tests::pool_allocator;
    pub(crate) use crate::game::fe::debugfe::tests::w;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_4 as alloc_base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::b;

    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;

    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init;

    pub(crate) use crate::game::fe::feasyncfile::tests::string_2 as string;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_4;

    pub(crate) use crate::game::fe::fereal::tests::base;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY;

    pub(crate) use crate::game::main::gamemode::tests::w as w_3;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_5;
    pub(crate) use crate::game::movie::movieplayer::tests::alloc_base;

    pub(crate) use crate::game::movie::movieplayer::tests::w as w_4;

    pub(crate) use crate::game::render::particle::tests::common;
    pub(crate) use crate::game::render::particle::tests::gp_at;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base as base_4;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w as w_6;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS_2;

    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_5;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear_2;

    pub(crate) use crate::game::wscript::wscriptman::tests::w as w_7;

    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    pub(crate) const SEEDS: u32 = 200;

    // ------------------------------------------------------- strings

    #[test]
    fn cbxstring_append_str_matches() {
        check(
            ee::s10::f_003189a0__generated,
            cbxstring_append_str,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3189a0, 0);
                // Destination cBXString: `self->text` at DATA+0x200, header at
                // -0xc (>= 2 selects the in-place path), -8 (length), -4
                // (capacity, large enough for the append).
                let buffer = DATA + 0x200;
                let length = (seed % 5) as u32;
                c.bus.write32(DATA, buffer);
                c.bus.write32(buffer - 0xc, 2);
                c.bus.write32(buffer - 8, length);
                c.bus.write32(buffer - 4, 0x1000);
                let src = DATA + 0x300;
                c.bus.write32(DATA + 0x40, src);
                c.bus.write32(src - 8, 1 + (seed % 7));
                for i in 0..16u32 {
                    c.bus.write8(src + i, (r.next_u32() & 0x7f) as u8);
                    c.bus
                        .write8(buffer + length + i, (r.next_u32() & 0x7f) as u8);
                }
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x40) as u64;
            },
        );
    }

    /// Call a generated helper as a setup step (arguments in a0..t1).
    pub(crate) fn step(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = TEST_RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, TEST_RA, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    /// The main heap's arena over 256 KiB of test RAM.
    pub(crate) fn alloc_ready(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        c.g[GP][0] = GAME_GP as u64;
        step(
            c,
            0x0031_ff60,
            ssx3_ee::s10::f_0031ff60,
            &[MEM_HEAP as u64, (DATA + 0x1_0000) as u64, 0x4_0000],
        );
        c.bus.write32(GAME_GP + 0xdd8, 1); // size-class pools ready
    }

    // ------------------------------------------------------------------
    // Bx strings and spheres.

    #[test]
    fn c_bx_string_set_at_matches() {
        check(
            ee::s10::f_00318e68__generated,
            c_bx_string_set_at,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x318e_68, 1);
                alloc_ready(c);
                let str_ = DATA + 0x100;
                let buf = DATA + 0x1000;
                clear(c, str_, 0x10);
                c.bus.write32(str_, buf);
                for i in 0..0x20u32 {
                    c.bus.write8(buf + i, r.next_u32() as u8 | 1);
                }
                // The buffer's reference count sits 0xc bytes before it: 0/1
                // keeps the buffer (the writable path).
                c.bus.write32(buf - 0xc, (seed % 2) as u32);
                c.bus.write32(buf - 8, 0x10);
                c.g[A0][0] = sx(str_);
                c.g[A1][0] = (seed % 0x10) as u64;
                c.g[A2][0] = [0x41u64, 0x7f, 0x81, 0x100][(seed % 4) as usize];
            },
        );
    }

    #[test]
    fn c_bx_string_ctor_copy_matches() {
        check(
            ee::s10::f_00317fe8__generated,
            c_bx_string_ctor_copy,
            SEEDS_3,
            &|c, seed| {
                let _ = common(c, seed, 0x317fe8, 1);
                alloc_ready_2(c);
                let this = DATA + 0x100;
                let other = DATA + 0x200;
                let data = DATA + 0x300;
                clear(c, this, 0x40);
                clear(c, other, 0x40);
                clear(c, data - 0x10, 0x40);
                c.bus.write32(other, data);
                c.bus
                    .write32(data - 0xc, if seed % 3 == 0 { 1 } else { 0xffff_ffff });
                c.bus.write8(data, b'a');
                c.bus.write8(data + 1, b'b');
                c.bus.write32(gp_at(0xda0), data + 0x20);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(other);
            },
        );
    }

    #[test]
    fn c_bx_string_append_cstr_matches() {
        check(
            ee::s10::f_00318910__generated,
            c_bx_string_append_cstr,
            SEEDS_3,
            &|c, seed| {
                let _ = common(c, seed, 0x318910, 2);
                alloc_ready_2(c);
                let this = DATA + 0x100;
                let data = DATA + 0x300;
                let text = DATA + 0x400;
                clear(c, this, 0x40);
                clear(c, data - 0x10, 0x80);
                clear(c, text, 0x40);
                c.bus.write32(this, data);
                c.bus.write32(data - 0xc, 1);
                c.bus.write32(data - 4, 0x20);
                c.bus.write8(data, b'a');
                c.bus.write8(text, b'h');
                c.bus.write8(text + 1, b'i');
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(text) };
            },
        );
    }

    #[test]
    fn c_bx_string_alloc_before_write_matches() {
        check(
            ssx3_ee::s10::f_00318188__generated,
            c_bx_string_alloc_before_write,
            40,
            &|c, seed| {
                fakes::machine(c, seed, 0x318188, 0);
                pool_allocator(c);
                let buf = OBJ2;
                // Cover all three paths of the reference/capacity test:
                //  seed%4 == 0/1: unique reference, buffer big enough  -> early
                //                 return after the (charged) size test;
                //  seed%4 == 2:   shared reference (refcount >= 2)     -> rebuild
                //                 with no size test;
                //  seed%4 == 3:   refcount sign-extends negative, so the size
                //                 test runs and rebuilds.  (`refcount == 1`
                //                 would make `cBXString_Release` free the fake
                //                 buffer, so it is left to its own test.)
                let (refcount, capacity) = match seed % 4 {
                    0 => (1u32, 0x1000u32),
                    1 => (0, 0x1000),
                    2 => (5, 0x10),
                    _ => (0xffff_ffff, 0x10),
                };
                w(c, buf - 0xc, refcount);
                w(c, buf - 4, capacity);
                w(c, OBJ, buf);
                w(c, buf, 0x41);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = ((seed * 37) % 200) as u64;
            },
        );
    }

    #[test]
    fn c_bx_string_release_matches() {
        check(
            ssx3_ee::s10::f_003180b8__generated,
            c_bx_string_release,
            120,
            &|c, seed| {
                let _r = setup(c, seed, 0x3180b8);
                c.g[A0][0] = DATA as u64;
                match seed % 3 {
                    0 => c.bus.write32(DATA, EMPTY),
                    1 => {
                        c.bus.write32(DATA, DATA + 0x40);
                        c.bus.write32(DATA + 0x34, 2 + seed % 3);
                        c.bus.write32(DATA + 0x38, 0);
                    }
                    _ => {
                        // A buffer at 0xc: its header is at zero, so the release
                        // drops the count without calling the allocator.
                        c.bus.write32(DATA, 0xc);
                        c.bus.write32(0, 1);
                    }
                }
            },
        );
    }

    #[test]
    fn c_bx_string_copy_before_write_matches() {
        check(
            ssx3_ee::s10::f_00318120__generated,
            c_bx_string_copy_before_write,
            120,
            &|c, seed| {
                let _r = setup(c, seed, 0x318120);
                c.g[A0][0] = DATA as u64;
                if seed % 2 == 0 {
                    // Shared buffer (refcount 2): release, reallocate zero bytes
                    // (the static empty string) and copy one byte.
                    c.bus.write32(DATA, DATA + 0x40);
                    c.bus.write32(DATA + 0x34, 2);
                    c.bus.write32(DATA + 0x38, 0);
                } else {
                    // Private buffer (refcount 1, header at zero): the early out,
                    // which leaves the `refcount < 2` result in v0.
                    c.bus.write32(DATA, 0xc);
                    c.bus.write32(0, 1);
                    c.bus.write32(4, 0);
                }
            },
        );
    }

    #[test]
    fn c_bx_string_op_plus_str_str_matches() {
        check(
            ssx3_ee::s10::f_003185c8__generated,
            c_bx_string_op_plus_str_str,
            100,
            &|c, seed| {
                let _r = setup(c, seed, 0x3185c8);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.g[A2][0] = (DATA + 0x200) as u64;
                // Two empty strings: the concatenation stays the static empty one.
                c.bus.write32(DATA + 0x100, DATA + 0x300);
                c.bus.write32(DATA + 0x2f4, 0xffff_ffff);
                c.bus.write32(DATA + 0x2f8, 0);
                c.bus.write32(DATA + 0x300, 0);
                c.bus.write32(DATA + 0x200, DATA + 0x340);
                c.bus.write32(DATA + 0x334, 0xffff_ffff);
                c.bus.write32(DATA + 0x338, 0);
                c.bus.write32(DATA + 0x340, 0);
            },
        );
    }

    #[test]
    fn cbx_string_make_upper_matches() {
        check(
            ee::s10::f_00318d28__generated,
            cbx_string_make_upper,
            60,
            &|c, seed| {
                base(c, seed, 0x31_8d28);
                let buf = DATA + 0x400;
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA, buf);
                // Private buffer: `CopyBeforeWrite` takes its early out, so the
                // characters are uppercased in place.
                c.bus.write32(buf - 0xc, 1);
                let len = seed % 6;
                c.bus.write32(buf - 8, len);
                for i in 0..len {
                    c.bus.write8(buf + i, b"aZ0_~"[((seed + i) % 5) as usize]);
                }
                // The class table at 0x499c81: bit 1 marks lowercase letters.
                for ch in [b'a', b'z'] {
                    c.bus
                        .write8(0x0049_9c81 + ch as u32, if seed % 2 == 0 { 2 } else { 0 });
                }
                c.bus.write8(0x0049_9c81 + b'A' as u32, 0);
            },
        );
    }

    pub(crate) fn put_str(c: &mut Cpu, at_2: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(at_2 + i as u32, *b);
        }
    }

    #[test]
    fn cbx_string_assign_copy_matches() {
        check(
            ee::s10::f_003183d8__generated,
            cbx_string_assign_copy,
            80,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x3183_d8);
                let len = 4 + (seed % 9);
                let buf = DATA + 0x1000;
                let empty = 0x0048_dcd4;
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA, if seed % 2 == 0 { buf } else { empty });
                for b in [buf, empty] {
                    c.bus.write32(b - 0xc, 2);
                    c.bus.write32(b - 4, 0x100);
                }
                c.g[A1][0] = len as u64;
                c.g[A2][0] = sx(DATA + 0x400);
                fill(c, &mut r, DATA + 0x400, 8);
            },
        );
    }

    #[test]
    fn cbx_string_release_buffer_matches() {
        check(
            ee::s10::f_00318a88__generated,
            cbx_string_release_buffer,
            80,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x318a_88);
                let buf = DATA + 0x1000;
                let empty = 0x0048_dcd4;
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA, if seed % 2 == 0 { buf } else { empty });
                for b in [buf, empty] {
                    c.bus.write32(b - 0xc, 2);
                    c.bus.write32(b - 4, 0x100);
                }
                put_str(c, buf, b"hello\0");
                c.g[A1][0] = if seed % 3 == 0 {
                    u64::MAX
                } else {
                    (seed % 12) as u64
                };
                fill(c, &mut r, DATA + 0x400, 4);
            },
        );
    }

    #[test]
    fn cbx_string_alloc_buffer_matches() {
        check(
            ee::s10::f_00318040__generated,
            cbx_string_alloc_buffer,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x318040, 2);
                be_library(c);
                world(c);
                heap_init(c);
                c.g[A0][0] = (DATA + 0x100) as u64;
                let size = if seed % 4 == 0 { 0 } else { 1 + seed % 0x30 };
                c.g[A1][0] = size as u64;
            },
        );
    }

    #[test]
    fn cbx_string_left_matches() {
        check(
            ee::s10::f_003190a8__generated,
            cbx_string_left,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x3190a8, 4);
                be_library(c);
                world(c);
                heap_init(c);
                let src = DATA + 0x400;
                string(c, src, "hello");
                w_2(c, src - 8, 5);
                w_2(c, DATA + 0x200, src);
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x200) as u64;
                c.g[A2][0] = match seed % 4 {
                    0 => 0,
                    1 => 2,
                    2 => 0x10,
                    _ => u32::MAX, // -1: "all"
                } as u64;
            },
        );
    }

    #[test]
    fn cbx_string_alloc_copy_matches() {
        check(
            ee::s10::f_003181e8__generated,
            cbx_string_alloc_copy,
            SEEDS_5,
            &|c, seed| {
                machine(c, seed, 0x3181e8, 2);
                be_library(c);
                heap_init(c);
                let dst = DATA;
                let src = DATA + 0x800;
                c.g[A0][0] = dst as u64;
                c.g[A1][0] = src as u64;
                c.g[A2][0] = (seed % 5) as u64;
                c.g[A3][0] = (seed % 3) as u64;
                c.g[T0][0] = if seed % 4 == 0 { 0 } else { 1 };
                w_3(c, src, DATA + 0x1000);
                string(c, DATA + 0x1000, "abcdefgh");
            },
        );
    }

    #[test]
    fn cbx_string_dtor_matches() {
        check(
            ee::s10::f_00318260__generated,
            cbx_string_dtor,
            SEEDS_5,
            &|c, seed| {
                machine(c, seed, 0x318260, 2);
                be_library(c);
                let obj = DATA;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 2) as u64;
                if seed % 4 == 0 {
                    w_3(c, obj, 0x0048_dcd4);
                } else {
                    let buf = DATA + 0x1000;
                    w_3(c, obj, buf);
                    w_3(c, buf - 0xc, if seed % 4 == 1 { 1 } else { 2 + seed % 3 });
                }
            },
        );
    }

    #[test]
    fn cbx_string_ctor_cstr_matches() {
        check(
            ee::s10::f_003182d8__generated,
            cbx_string_ctor_cstr,
            SEEDS_5,
            &|c, seed| {
                machine(c, seed, 0x3182d8, 2);
                be_library(c);
                heap_init(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = if seed % 3 == 0 {
                    0
                } else {
                    (DATA + 0x1000) as u64
                };
                string(c, DATA + 0x1000, if seed % 2 == 0 { "hello" } else { "" });
            },
        );
    }

    #[test]
    fn c_bx_string_right_matches() {
        check(
            ee::s10::f_00319028__generated,
            c_bx_string_right,
            12,
            &|c, seed| {
                let mut r = alloc_base(c, seed, 0x3190_28, 4);
                let dst = DATA;
                let src = DATA + 0x1000;
                let buf = DATA + 0x2000;
                let len = 1 + r.next_u32() % 12;
                w_4(c, src, buf);
                w_4(c, buf - 8, len);
                for i in 0..len {
                    wb(c, buf + i, b'a' + (i % 26) as u8);
                }
                c.g[A0][0] = dst as u64;
                c.g[A1][0] = src as u64;
                c.g[A2][0] = match seed % 3 {
                    0 => (r.next_u32() % 16) as u64,
                    1 => 0,
                    _ => (-1i64) as u64,
                };
            },
        );
    }

    /// `DLHeap_InitArena(&MEM_HEAP, arena, 0x40000)`: the game heap over test
    /// memory, for the paths that allocate.
    pub(crate) fn heap_init_3(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        c.g[A0][0] = MEM_HEAP as u64;
        c.g[A1][0] = (DATA + 0x4_0000) as u64;
        c.g[A2][0] = 0x4_0000;
        c.g[RA][0] = TEST_RA as u64;
        c.pc = 0x0031_ff60;
        ssx3_ee::s10::DLHeap_InitArena(c);
        assert_eq!(c.pc, TEST_RA as u32, "DLHeap_InitArena did not return");
    }

    // ------------------------------------------------------------------
    // BX strings.

    #[test]
    fn c_bx_string_ctor_cstr_n_matches() {
        check(
            ssx3_ee::s10::f_00318350__generated,
            c_bx_string_ctor_cstr_n,
            SEEDS_6,
            &|c, seed| {
                let mut r = machine(c, seed, 0x318350, 3);
                be_library(c);
                world(c);
                heap_init_3(c);
                let this = DATA + 0x100;
                c.g[A0][0] = this as u64;
                let s = DATA + 0x200;
                let len = (r.next_u32() % 8) as usize;
                for i in 0..0x20 {
                    b(c, s + i as u32, b'a' + (i % 26) as u8);
                }
                b(c, s + len as u32, 0);
                c.g[A1][0] = s as u64;
                c.g[A2][0] = (r.next_u32() % 0x20) as u64;
            },
        );
    }

    #[test]
    fn c_bx_string_concat_copy_matches() {
        check(
            ssx3_ee::s10::f_00318540__generated,
            c_bx_string_concat_copy,
            SEEDS_6,
            &|c, seed| {
                let mut r = machine(c, seed, 0x318540, 3);
                be_library(c);
                world(c);
                heap_init_3(c);
                let this = DATA + 0x100;
                let s1 = DATA + 0x200;
                let s2 = DATA + 0x280;
                for i in 0..0x40 {
                    b(c, s1 + i, b'a' + (i % 26) as u8);
                    b(c, s2 + i, b'A' + (i % 26) as u8);
                }
                c.g[A0][0] = this as u64;
                // (len1, s1, len2, s2) as the generated call order shows.
                c.g[A1][0] = (r.next_u32() % 0x20) as u64;
                c.g[A2][0] = s1 as u64;
                c.g[A3][0] = (r.next_u32() % 0x20) as u64;
                c.g[T0][0] = s2 as u64;
            },
        );
    }

    /// `DLHeap_InitArena(&MEM_HEAP, arena, size)` over a test arena.
    pub(crate) fn heap_init_4(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        const RET: u32 = 0xdead_0100;
        c.g[A0][0] = MEM_HEAP as u64;
        c.g[A1][0] = (DATA + 0x4_0000) as u64;
        c.g[A2][0] = 0x4_0000;
        c.g[RA][0] = RET as u64;
        c.pc = 0x0031_ff60;
        ee::s10::DLHeap_InitArena(c);
        assert_eq!(c.pc, RET, "DLHeap_InitArena did not return");
    }

    /// The engine's size-class pool allocator: `gp_3 + 0xdd8` ready plus eight
    /// free nodes per size class.
    pub(crate) fn pool(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        w_6(c, GAME_GP.wrapping_add(0xdd8), 1);
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let pool = DATA + 0x40_0000 + 0x4_0000 * i;
            let base_4 = DATA + 0x80_0000 + 0x4000 * i;
            for n in 0..8u32 {
                let node = base_4 + 0x400 * n;
                w_6(c, node, if n == 7 { 0 } else { node + 0x400 });
            }
            w_6(c, pool + head, base_4);
            w_6(c, GAME_GP.wrapping_add(slot), pool);
        }
    }

    #[test]
    fn c_bx_string_op_plus_c_str_str_matches() {
        check(
            ee::s10::f_003186d0__generated,
            c_bx_string_op_plus_c_str_str,
            100,
            &|c, seed| {
                let _ = base_4(c, seed, 0x31_86d0);
                globals(c);
                pool(c);
                heap_init_4(c);
                // A cBXString at DATA+0x2000 whose buffer has a length word at -8.
                const BUF: u32 = DATA + 0x2100;
                w_6(c, DATA + 0x2000, BUF);
                w_6(c, BUF - 8, 4);
                for i in 0..8u32 {
                    c.bus.write8(BUF + i, b'x' + i as u8);
                }
                // The global stack temporary (a cBXString) starts out empty.
                w_6(c, 0x004a_3e90, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 2 == 0 { sx(DATA + 0x3000) } else { 0 };
                for i in 0..8u32 {
                    c.bus.write8(DATA + 0x3000 + i, b'a' + i as u8);
                }
                c.g[A2][0] = sx(DATA + 0x2000);
            },
        );
    }

    /// `DLHeap_InitArena(&MEM_HEAP, arena, size)` over a test arena, for the
    /// paths that allocate.
    pub(crate) fn heap(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        const RET: u32 = 0x0012_3450;
        for (i, a) in [MEM_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000]
            .into_iter()
            .enumerate()
        {
            c.g[[A0, A1, A2][i]][0] = a;
        }
        c.g[RA][0] = RET as u64;
        c.pc = 0x0031_ff60;
        ssx3_ee_rt::funcs::call_indirect(c, 0x0031_ff60);
        assert_eq!(c.pc, RET, "DLHeap_InitArena did not return");
    }

    pub(crate) fn put_str_2(c: &mut Cpu, addr: u32, s: &str) {
        for (i, b) in s.bytes().chain([0u8]).enumerate() {
            c.bus.write8(addr + i as u32, b);
        }
    }

    pub(crate) fn alloc(c: &mut Cpu, size: u32) -> u32 {
        const RET: u32 = 0x0012_3450;
        c.g[A0][0] = size as u64;
        c.g[A1][0] = 0x004a_0000;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[RA][0] = RET as u64;
        c.pc = 0x0031_7d70;
        ssx3_ee_rt::funcs::call_indirect(c, 0x0031_7d70);
        assert_eq!(c.pc, RET, "EA_MemAllocNamed did not return");
        c.g[V0][0] as u32
    }

    /// A cBXString block at `data`: header (refcount, length, capacity) and
    /// text.
    pub(crate) fn string_block(c: &mut Cpu, data: u32, refcount: i32, text: &str) {
        clear_2(c, data - 0x10, 0x10);
        w_7(c, data - 0xc, refcount as u32);
        w_7(c, data - 8, text.len() as u32);
        w_7(c, data - 4, 0x100);
        put_str_2(c, data, text);
    }

    // ------------------------------------------------------------- equipment

    // ---------------------------------------------------------------- strings

    #[test]
    fn cbxstring_assign_matches() {
        check(
            ssx3_ee::s10::f_00318448__generated,
            cbxstring_assign,
            12,
            &|c, seed| {
                let _ = base_5(c, seed, 0x0031_8448);
                heap(c);
                let a = DATA + 0x100;
                let b = DATA + 0x110;
                let da = alloc(c, 0x120);
                let da = if da == 0 { DATA + 0x2000 } else { da + 0x10 };
                string_block(c, da + 0x10, if seed % 3 == 2 { -1 } else { 2 }, "left");
                let db = DATA + 0x3000;
                string_block(c, db, 2, "right");
                w_7(c, a, if seed % 5 == 0 { db } else { da + 0x10 });
                w_7(c, b, db);
                c.g[A0][0] = sx(a);
                c.g[A1][0] = sx(b);
            },
        );
    }

    #[test]
    fn cbxstring_op_plus_str_cstr_matches() {
        check(
            ssx3_ee::s10::f_00318630__generated,
            cbxstring_op_plus_str_cstr,
            12,
            &|c, seed| {
                let _ = base_5(c, seed, 0x0031_8630);
                heap(c);
                let src_obj = DATA + 0x100;
                let dst_obj = DATA + 0x110;
                let da = alloc(c, 0x120);
                let data = if da == 0 { DATA + 0x2000 } else { da + 0x10 };
                string_block(c, data, 2, "hello");
                w_7(c, src_obj, data);
                w_7(c, dst_obj, 0);
                put_str_2(c, DATA + 0x3000, " world");
                w_7(c, GAME_GP.wrapping_add(0xda0), 0x0048_dcd4);
                c.g[A0][0] = sx(dst_obj);
                c.g[A1][0] = sx(src_obj);
                c.g[A2][0] = if seed % 4 == 0 { 0 } else { sx(DATA + 0x3000) };
            },
        );
    }

    // ---------------------------------------------------------- cBXString

    #[test]
    fn cbx_string_get_buffer_matches() {
        check(
            ee::s10::f_003189d0__generated,
            cbx_string_get_buffer,
            40,
            &|c, seed| {
                alloc_base_2(c, seed, 0x3189, 4);
                let s = DATA;
                let buf = DATA + 0x1_000;
                for i in 0..0x40u32 {
                    w_8(c, buf + 4 * i, 0x41);
                }
                w_8(c, s, buf);
                // 3 = shared buffer (copy path), 1 = unique with room (fast path).
                let shared = seed % 2 == 0;
                w_8(c, buf.wrapping_sub(0xc), if shared { 3 } else { 1 });
                w_8(c, buf.wrapping_sub(4), 0x100);
                w_8(c, buf.wrapping_sub(8), 4);
                c.g[A0][0] = s as u64;
                c.g[A1][0] = (if shared { 8 } else { 2 }) as u64;
            },
        );
    }

    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::Rng;

    pub(crate) use crate::game::bx::cubicspline::tests::heap_alloc_machine;
    use crate::game::misc::tests::put_cstr;

    /// A cBXString object at `obj` pointing at `buf`, with its header words
    /// (refcount, length, capacity).
    pub(crate) fn string_obj(c: &mut Cpu, obj: u32, buf: u32, refcount: u32, len: u32, cap: u32) {
        c.bus.write32(obj, buf);
        c.bus.write32(buf - 0xc, refcount);
        c.bus.write32(buf - 8, len);
        c.bus.write32(buf - 4, cap);
    }

    // ------------------------------------------------------- concatenation

    /// The `*self = *a + ch` (or `ch + *b`) fixture: `self` pre-filled
    /// non-zero, `a` a short string, the shared empty string at gp+0xda0.
    pub(crate) fn op_plus_fixture(
        c: &mut Cpu,
        seed: u32,
        salt: u64,
        entry: u32,
        other: u32,
    ) -> u32 {
        let mut r = heap_alloc_machine(c, seed, salt);
        let this = DATA + 0x100;
        let buf = DATA + 0x300;
        fill(c, &mut r, this, 4);
        let len = seed % 8;
        string_obj(c, other, buf, 1, len, 0x100);
        for i in 0..8u32 {
            c.bus.write8(
                buf + i,
                b"ab\x80d\xffg\x9ah"[(seed as usize + i as usize) % 8],
            );
        }
        c.bus.write8(buf + len, 0);
        c.pc = entry;
        this
    }

    /// A `char` argument with every register bit above the byte set: the
    /// callee sees only the low byte (`sb`, or `sll`/`sra` by 24).
    pub(crate) fn bxs_char_arg(ch: u8) -> u64 {
        0xa5c3_5a3c_7e5d_3a00 | ch as u64
    }

    #[test]
    fn cbx_string_op_plus_str_char_matches() {
        check(
            ee::s10::f_00318760__generated,
            cbx_string_op_plus_str_char,
            60,
            &|c, seed| {
                let this = op_plus_fixture(c, seed, 0x3187_60, 0x0031_8760, DATA + 0x200);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x200) as u64;
                c.g[A2][0] = bxs_char_arg([b'!', 0, b'~', 0x80, 0xff][seed as usize % 5]);
            },
        );
    }

    #[test]
    fn cbx_string_op_plus_char_str_matches() {
        check(
            ee::s10::f_003187c8__generated,
            cbx_string_op_plus_char_str,
            60,
            &|c, seed| {
                let this = op_plus_fixture(c, seed, 0x3187_c8, 0x0031_87c8, DATA + 0x200);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = bxs_char_arg([b'?', 0, b'#', 0x80, 0xe9][seed as usize % 5]);
                c.g[A2][0] = (DATA + 0x200) as u64;
            },
        );
    }

    /// The in-place concatenation fixture: `self` with a `len`/`cap` header
    /// steering the shared, full and room arms by `seed % 3`, the length by
    /// `seed / 3 % 5` and the appended count by `seed / 15 % 6`.
    pub(crate) fn concat_fixture(
        c: &mut Cpu,
        seed: u32,
        salt: u64,
        entry: u32,
        tail: u8,
    ) -> (u32, u32) {
        let mut r = heap_alloc_machine(c, seed, salt);
        let this = DATA + 0x100;
        let buf = DATA + 0x1000;
        let src = DATA + 0x2000;
        let len = 3 + seed / 3 % 5;
        let add = 1 + seed / 15 % 6;
        let (refcount, cap) = match seed % 3 {
            0 => (5, 0x100),
            1 => (1, len + add - 1),
            _ => (1, len + add + 8),
        };
        string_obj(c, this, buf, refcount, len, cap);
        for i in 0..0x40u32 {
            c.bus.write8(buf + i, tail);
        }
        for i in 0..len {
            c.bus
                .write8(buf + i, b"k\x80m\xe9o"[(seed + i) as usize % 5]);
        }
        for i in 0..add {
            c.bus
                .write8(src + i, b"0\xff2\x813"[(seed + i) as usize % 5]);
        }
        put_cstr(c, src + add, b"");
        fill(c, &mut r, this + 4, 4);
        c.pc = entry;
        (this, add)
    }

    /// Totals of the generated body: every total the 120 seeds give: the
    /// room arm (56-91, one per appended count) and the shared and full
    /// rebuild arms (484-775). Seeds 90 and up own a buffer with exactly
    /// enough room (the room arm's edge).
    #[test]
    fn cbx_string_concat_in_place_matches() {
        check_paths(
            ee::s10::f_00318830__generated,
            cbx_string_concat_in_place,
            120,
            &[
                56, 63, 70, 77, 84, 91, 484, 491, 498, 505, 512, 519, 526, 533, 540, 547, 704, 707,
                717, 734, 741, 749, 763, 775,
            ],
            &|c, seed| {
                let (this, add) = concat_fixture(c, seed, 0x3188_30, 0x0031_8830, 0xa5);
                if seed >= 90 {
                    let buf = DATA + 0x1000;
                    let len = c.bus.read32(buf - 8);
                    c.bus.write32(buf - 0xc, 1);
                    c.bus.write32(buf - 4, len + add);
                }
                c.g[A0][0] = this as u64;
                c.g[A1][0] = add as u64;
                c.g[A2][0] = (DATA + 0x2000) as u64;
            },
        );
    }

    #[test]
    fn cbx_string_append_char_matches() {
        check_paths(
            ee::s10::f_00318968__generated,
            cbx_string_append_char,
            60,
            &[69, 506, 513, 520, 527, 534, 695, 707, 713, 730, 734],
            &|c, seed| {
                let (this, _) = concat_fixture(c, seed, 0x3189_68, 0x0031_8968, 0x5a);
                // The append is always one byte: re-steer the header for it
                // (shared, exactly full, room) by `seed % 3`.
                let len = 3 + seed / 3 % 5;
                let (refcount, cap) = match seed % 3 {
                    0 => (5, 0x100),
                    1 => (1, len),
                    _ => (1, len + 8),
                };
                c.bus.write32(DATA + 0x1000 - 0xc, refcount);
                c.bus.write32(DATA + 0x1000 - 4, cap);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = bxs_char_arg([b'Q', 0, 0x7f, 0xff][seed as usize / 15 % 4]);
            },
        );
    }

    #[test]
    fn cbx_string_get_buffer_set_length_matches() {
        check_paths(
            ee::s10::f_00318af8__generated,
            cbx_string_get_buffer_set_length,
            60,
            &[39, 466, 480],
            &|c, seed| {
                let mut r = heap_alloc_machine(c, seed, 0x318a_f8);
                let this = DATA + 0x100;
                let buf = DATA + 0x1000;
                let len = 4 + seed % 4;
                string_obj(c, this, buf, if seed % 2 == 0 { 3 } else { 1 }, len, 0x40);
                put_cstr(c, buf, b"h\xe9l\x80o");
                fill(c, &mut r, buf + 8, 8);
                c.pc = 0x0031_8af8;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 20) as u64;
            },
        );
    }

    /// Totals of the generated body: every total the 60 seeds give: the full-buffer
    /// early out (15) and the rebuild through a copy (638-680).
    #[test]
    fn cbx_string_free_extra_matches() {
        check_paths(
            ee::s10::f_00318b40__generated,
            cbx_string_free_extra,
            60,
            &[15, 638, 646, 661, 666, 680],
            &|c, seed| {
                let mut r = heap_alloc_machine(c, seed, 0x318b_40);
                let this = DATA + 0x100;
                let buf = DATA + 0x1000;
                let len = 4 + seed / 2 % 5;
                // Even seeds: length equals capacity (the early out); odd
                // seeds: spare capacity (the rebuild through a copy).
                let cap = if seed % 2 == 0 {
                    len
                } else {
                    len + 1 + seed / 10 % 6
                };
                string_obj(c, this, buf, 1, len, cap);
                put_cstr(c, buf, b"h\x80llo w\xe9rld");
                fill(c, &mut r, buf + 12, 8);
                c.pc = 0x0031_8b40;
                c.g[A0][0] = this as u64;
            },
        );
    }

    #[test]
    fn cbx_string_lock_buffer_matches() {
        check_paths(
            ee::s10::f_00318bc0__generated,
            cbx_string_lock_buffer,
            40,
            &[34, 360, 372, 466, 473, 480, 487],
            &|c, seed| {
                let mut r = heap_alloc_machine(c, seed, 0x318b_c0);
                let this = DATA + 0x100;
                let buf = DATA + 0x1000;
                string_obj(
                    c,
                    this,
                    buf,
                    if seed % 2 == 0 { 3 } else { 1 },
                    2 + seed / 2 % 6,
                    0x20,
                );
                put_cstr(c, buf, b"h\x80llo w\xe9rld");
                fill(c, &mut r, buf + 12, 4);
                c.pc = 0x0031_8bc0;
                c.g[A0][0] = this as u64;
            },
        );
    }

    // ---------------------------------------------------------------- search

    /// The find fixture: `self` over "he\x80lo w\xe9rld" (length 11, two
    /// bytes >= 0x80) with a random tail, returning `(this, buf)`.
    pub(crate) fn find_fixture(c: &mut Cpu, seed: u32, salt: u64) -> (u32, u32) {
        let mut r = machine(c, seed, salt, 0);
        let this = DATA + 0x100;
        let buf = DATA + 0x1000;
        string_obj(c, this, buf, 1, 11, 0x20);
        put_cstr(c, buf, b"he\x80lo w\xe9rld");
        fill(c, &mut r, buf + 12, 8);
        (this, buf)
    }

    /// Totals of the generated body: every total the 72 seeds give (found and
    /// missing arms, the `strchr` scan length varying with the start).
    #[test]
    fn cbx_string_find_char_matches() {
        check_paths(
            ee::s10::f_00318c18__generated,
            cbx_string_find_char,
            72,
            &[
                28, 30, 31, 35, 37, 38, 42, 44, 45, 52, 56, 58, 59, 63, 64, 65, 66, 70, 72, 73, 77,
                78, 79, 84, 85, 86, 91, 93, 98, 100, 113, 138, 140,
            ],
            &|c, seed| {
                let (this, buf) = find_fixture(c, seed, 0x318c_18);
                let start = seed / 6 % 12;
                // Found ('l'), missing ('z'), the terminator, the byte at
                // the start itself, and the two bytes >= 0x80.
                let ch = match seed % 6 {
                    0 => b'l',
                    1 => b'z',
                    2 => 0,
                    3 => c.bus.read8(buf + start),
                    4 => 0x80,
                    _ => 0xe9,
                };
                c.pc = 0x0031_8c18;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = bxs_char_arg(ch);
                // An `int` start with the upper word set (the add is 32-bit).
                c.g[A2][0] = 0xffff_ffff_0000_0000 | start as u64;
            },
        );
    }

    /// Totals of the generated body: every total the 36 seeds give (found and
    /// missing arms, with the `strpbrk` scan cost).
    #[test]
    fn cbx_string_find_one_of_matches() {
        check_paths(
            ee::s10::f_00318c60__generated,
            cbx_string_find_one_of,
            36,
            &[81, 86, 328],
            &|c, seed| {
                let (this, _) = find_fixture(c, seed, 0x318c_60);
                // A set that hits ('aeiou'), one that misses ("xyz"), the
                // empty set, and sets hitting a byte >= 0x80 first.
                let sets: [&[u8]; 4] = [b"aeiou", b"xyz", b"", b"\xe9\x80"];
                put_cstr(c, DATA + 0x2000, sets[seed as usize % 4]);
                c.pc = 0x0031_8c60;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
            },
        );
    }

    /// Totals of the generated body: every total the 48 seeds give: missing (114)
    /// and found (116, the 2-cycle index arm).
    #[test]
    fn cbx_string_reverse_find_matches() {
        check_paths(
            ee::s10::f_00318ca0__generated,
            cbx_string_reverse_find,
            48,
            &[114, 116],
            &|c, seed| {
                let (this, _) = find_fixture(c, seed, 0x318c_a0);
                // Last 'l' (index 9, distinct from the first at 3), missing,
                // the terminator, and the two bytes >= 0x80.
                let ch = [b'l', b'z', 0, 0x80, 0xe9][seed as usize % 5];
                c.pc = 0x0031_8ca0;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = bxs_char_arg(ch);
            },
        );
    }

    /// Totals of the generated body: every total the 45 seeds give (found and
    /// missing arms, the `strstr` scan length varying with the start).
    #[test]
    fn cbx_string_find_str_matches() {
        check_paths(
            ee::s10::f_00318ce8__generated,
            cbx_string_find_str,
            45,
            &[
                26, 42, 49, 52, 53, 60, 63, 64, 71, 74, 82, 85, 93, 96, 107, 118, 129, 140,
            ],
            &|c, seed| {
                let (this, _) = find_fixture(c, seed, 0x318c_e8);
                let start = seed / 5 % 9;
                // Present ("w\xe9rld", found iff start <= 6), missing, empty
                // (matches at the start), overlapping ("o w") and a needle
                // starting with a byte >= 0x80 (found iff start <= 2).
                let needles: [&[u8]; 5] = [b"w\xe9rld", b"xyz", b"", b"o w", b"\x80l"];
                put_cstr(c, DATA + 0x2000, needles[seed as usize % 5]);
                c.pc = 0x0031_8ce8;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                // An `int` start with the upper word set (the add is 32-bit).
                c.g[A2][0] = 0xffff_ffff_0000_0000 | start as u64;
            },
        );
    }

    // ------------------------------------------------- loops with event checks

    driven!(gen_make_lower, ee::s10::f_00318d90__generated);

    driven!(hand_make_lower, cbx_string_make_lower);

    /// Load the game image (the class table with the game's own bytes);
    /// call before the rest of the set-up, which it would overwrite.
    pub(crate) fn bxs_ctype_image(c: &mut Cpu) {
        crate::difftest::elf_image(c);
        assert_eq!(
            c.bus.read8(CTYPE_TABLE + b'A' as u32),
            0x41,
            "class table missing"
        );
        assert_eq!(c.bus.read8(CTYPE_TABLE + b'\t' as u32), 0x28);
        assert_eq!(c.bus.read8(CTYPE_TABLE - 0x80), 0x6f);
    }

    /// The lowercasing fixture, steered by `key`: length `key % 13` (0 is
    /// the empty early-out), shared by `key / 13 % 2` (the detach then
    /// takes its realloc path), the pattern rotated by `key / 26`. The
    /// pattern holds upper, lower, space, digit and other bytes, and bytes
    /// >= 0x80 whose sign-extended class has the upper bit (0x80, 0x87) or
    /// not (0x88, 0xff).
    pub(crate) fn make_lower_fixture(c: &mut Cpu, seed: u32, key: u32) {
        bxs_ctype_image(c);
        let mut r = heap_alloc_machine(c, seed, 0x318d_90);
        let this = DATA + 0x100;
        let buf = DATA + 0x1000;
        let len = key % 13;
        let shared = key / 13 % 2 == 1;
        string_obj(c, this, buf, if shared { 3 } else { 1 }, len, 0x20);
        let pat = b"aB \x80Zq9!\x87\x88\xffMx";
        for i in 0..len {
            c.bus
                .write8(buf + i, pat[(key / 26 + i) as usize % pat.len()]);
        }
        fill(c, &mut r, buf + len + 1, 4);
        c.g[A0][0] = this as u64;
    }

    /// Totals of the generated body: every total the 78 seeds give: the empty early
    /// out and each loop length on a private buffer (28-150), and the shared
    /// buffer's detach with each length (325-596).
    #[test]
    fn cbx_string_make_lower_matches() {
        check_paths(
            ee::s10::f_00318d90__generated,
            cbx_string_make_lower,
            78,
            &[
                28, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 325, 342, 365, 460, 477,
                494, 511, 528, 545, 562, 579, 596,
            ],
            &|c, seed| {
                make_lower_fixture(c, seed, seed);
                c.pc = 0x0031_8d90;
            },
        );
        // The character loop's event check, delivered and unwinding: the
        // lengths vary, so the first unwind lands on different iterations.
        check_events(gen_make_lower, hand_make_lower, 78, &|c, seed| {
            make_lower_fixture(c, seed, seed / 3);
            let mut r = Rng(0x318d_9000 ^ seed as u64);
            let mode = schedule(c, &mut r, seed, 0x0031_8d90);
            if mode != Events::Off {
                c.next_event = c.cycles;
            }
            mode
        });
    }

    driven!(gen_replace_char, ee::s10::f_00318df8__generated);

    driven!(hand_replace_char, cbx_string_replace_char);

    /// The replace fixture, steered by `key`: length `key % 13` (0 is the
    /// empty early-out) of a 12-byte pattern holding bytes >= 0x80, `from`
    /// by `key / 13 % 4` (present, missing, and the two high bytes, which
    /// match only as sign-extended bytes), shared by `key / 52 % 2`. Both
    /// characters carry high register bits.
    pub(crate) fn replace_char_fixture(c: &mut Cpu, seed: u32, key: u32) {
        let mut r = heap_alloc_machine(c, seed, 0x318d_f8);
        let this = DATA + 0x100;
        let buf = DATA + 0x1000;
        let len = key % 13;
        let shared = key / 52 % 2 == 1;
        string_obj(c, this, buf, if shared { 3 } else { 1 }, len, 0x20);
        let base = b"hel\x80o w\xe9rl\x80d";
        for i in 0..len {
            c.bus.write8(buf + i, base[i as usize]);
        }
        fill(c, &mut r, buf + len + 1, 4);
        let from = [b'l', b'z', 0x80, 0xe9][key as usize / 13 % 4];
        c.g[A0][0] = this as u64;
        c.g[A1][0] = bxs_char_arg(from);
        c.g[A2][0] = bxs_char_arg(0xd8);
    }

    /// Totals of the generated body: every total the 104 seeds give: the empty early
    /// out and each loop length on a private buffer (35-108), and the shared
    /// buffer's detach with each length (328-557).
    #[test]
    fn cbx_string_replace_char_matches() {
        check_paths(
            ee::s10::f_00318df8__generated,
            cbx_string_replace_char,
            104,
            &[
                35, 42, 48, 54, 60, 66, 72, 78, 84, 90, 96, 97, 102, 108, 328, 341, 349, 453, 466,
                479, 492, 505, 518, 531, 544, 557,
            ],
            &|c, seed| {
                replace_char_fixture(c, seed, seed);
                c.pc = 0x0031_8df8;
            },
        );
        // The replacement loop's event check, delivered and unwinding.
        check_events(gen_replace_char, hand_replace_char, 312, &|c, seed| {
            replace_char_fixture(c, seed, seed / 3);
            let mut r = Rng(0x318d_f800 ^ seed as u64);
            let mode = schedule(c, &mut r, seed, 0x0031_8df8);
            if mode != Events::Off {
                c.next_event = c.cycles;
            }
            mode
        });
    }

    // ------------------------------------------------------- small constructors

    /// Totals of the generated body: every total the 48 seeds give: the zero and
    /// negative count early out (19) and the fill with each count (392-472).
    #[test]
    fn cbx_string_ctor_char_count_matches() {
        check_paths(
            ee::s10::f_00318eb8__generated,
            cbx_string_ctor_char_count,
            48,
            &[19, 392, 399, 430, 437, 463, 465, 470, 472],
            &|c, seed| {
                let mut r = heap_alloc_machine(c, seed, 0x318e_b8);
                let this = DATA + 0x100;
                fill(c, &mut r, this, 4);
                c.pc = 0x0031_8eb8;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = bxs_char_arg([b'A', 0x7f, 0, 0xff][seed as usize % 4]);
                // `seed / 4 % 6`: zero, negative (low word positive, the
                // compare is 64-bit) and four positive counts.
                c.g[A2][0] = match seed / 4 % 6 {
                    0 => 0,
                    1 => 0xffff_ffff_0000_0005,
                    k => (k as u64 - 1) * 5 + seed as u64 / 24,
                };
            },
        );
    }

    #[test]
    fn cbx_string_assign_char_matches() {
        check_paths(
            ee::s10::f_00318f20__generated,
            cbx_string_assign_char,
            40,
            &[75, 328],
            &|c, seed| {
                let mut r = heap_alloc_machine(c, seed, 0x318f_20);
                let this = DATA + 0x100;
                let buf = DATA + 0x3000;
                // A valid object: the callee reads `*self` before replacing
                // it, so a random pointer could walk off the RAM map. Even
                // seeds share the buffer (detach path), odd seeds own it.
                string_obj(c, this, buf, if seed % 2 == 0 { 3 } else { 1 }, 5, 0x20);
                put_cstr(c, buf, b"hello");
                fill(c, &mut r, buf + 8, 4);
                c.pc = 0x0031_8f20;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = bxs_char_arg([b'k', 0, 0x80, 0xfe][seed as usize / 2 % 4]);
            },
        );
    }

    #[test]
    fn cbx_string_mid_from_matches() {
        check(
            ee::s10::f_00318f58__generated,
            cbx_string_mid_from,
            48,
            &|c, seed| {
                let mut r = heap_alloc_machine(c, seed, 0x318f_58);
                let this = DATA + 0x100;
                let other = DATA + 0x200;
                let buf = DATA + 0x1000;
                fill(c, &mut r, this, 4);
                string_obj(c, other, buf, 1, 11, 0x20);
                put_cstr(c, buf, b"h\x80llo w\xe9rld");
                fill(c, &mut r, buf + 12, 4);
                c.pc = 0x0031_8f58;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = other as u64;
                // Starts past the end exercise the clamp in the callee;
                // `seed / 14 % 2` sets the upper word (the subtract is
                // 32-bit; the callee's 64-bit clamp then sees a negative).
                let hi = if seed / 14 % 2 == 1 {
                    0xffff_ffff_0000_0000
                } else {
                    0
                };
                c.g[A2][0] = hi | (seed % 14) as u64;
            },
        );
    }

    // ------------------------------------------------------------- mid/span

    #[test]
    fn cbx_string_mid_matches() {
        check_paths(
            ee::s10::f_00318f90__generated,
            cbx_string_mid,
            42,
            &[93, 358, 366, 493, 507],
            &|c, seed| {
                let mut r = heap_alloc_machine(c, seed, 0x318f_90);
                let this = DATA + 0x104;
                let other = DATA + 0x214;
                let buf = DATA + 0x1300;
                fill(c, &mut r, this, 4);
                string_obj(c, other, buf, 1, 11, 0x20);
                put_cstr(c, buf, b"h\x80llo w\xe9rld");
                fill(c, &mut r, buf + 12, 4);
                c.pc = 0x0031_8f90;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = other as u64;
                // Negative, zero, inside, at the end, past the end, and
                // negative as 64 bits with a positive low word (the clamps
                // compare all 64 bits).
                c.g[A2][0] = [
                    0xffff_ffff_ffff_fffd,
                    0,
                    2,
                    9,
                    11,
                    20,
                    0xffff_ffff_0000_0002,
                ][seed as usize % 7];
                // Negative, zero, short, longer than the string, and the
                // 64-bit negative with a positive low word.
                c.g[A3][0] = [0xffff_ffff_ffff_ffff, 0, 3, 11, 100, 0xffff_ffff_0000_0003]
                    [seed as usize / 7 % 6];
            },
        );
    }

    #[test]
    fn cbx_string_span_including_matches() {
        check_paths(
            ee::s10::f_00319120__generated,
            cbx_string_span_including,
            24,
            &[
                108, 112, 119, 126, 133, 387, 393, 401, 411, 431, 436, 542, 612,
            ],
            &|c, seed| {
                let mut r = heap_alloc_machine(c, seed, 0x3191_20);
                let this = DATA + 0x104;
                let other = DATA + 0x214;
                let buf = DATA + 0x1300;
                let accept = DATA + 0x2000;
                fill(c, &mut r, this, 4);
                let srcs: [&[u8]; 4] = [b"aa\x80abb", b"\x80\x80b", b"", b"ab\xe9abc"];
                let src = srcs[seed as usize % 4];
                string_obj(c, other, buf, 1, src.len() as u32, 0x20);
                put_cstr(c, buf, src);
                fill(c, &mut r, buf + 12, 4);
                // Full, partial, empty and missing spans, and sets with
                // bytes >= 0x80.
                let sets: [&[u8]; 6] = [b"a", b"ab", b"z", b"", b"a\x80", b"\xe9ab"];
                put_cstr(c, accept, sets[seed as usize / 4 % 6]);
                c.pc = 0x0031_9120;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = other as u64;
                c.g[A2][0] = accept as u64;
            },
        );
    }

    #[test]
    fn cbx_string_span_excluding_matches() {
        check_paths(
            ee::s10::f_00319170__generated,
            cbx_string_span_excluding,
            24,
            &[108, 117, 395, 401, 412, 414, 420, 442, 530, 579, 619],
            &|c, seed| {
                let mut r = heap_alloc_machine(c, seed, 0x3191_70);
                let this = DATA + 0x104;
                let other = DATA + 0x214;
                let buf = DATA + 0x1300;
                let reject = DATA + 0x2000;
                fill(c, &mut r, this, 4);
                let srcs: [&[u8]; 4] = [b"aa\x80abb", b"\x80\x80b", b"", b"ab\xe9abc"];
                let src = srcs[seed as usize % 4];
                string_obj(c, other, buf, 1, src.len() as u32, 0x20);
                put_cstr(c, buf, src);
                fill(c, &mut r, buf + 12, 4);
                // Empty, full and missing spans, and sets with bytes >= 0x80.
                let sets: [&[u8]; 6] = [b"a", b"z", b"", b"ab", b"\x80", b"\xe9z"];
                put_cstr(c, reject, sets[seed as usize / 4 % 6]);
                c.pc = 0x0031_9170;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = other as u64;
                c.g[A2][0] = reject as u64;
            },
        );
    }

    // ---------------------------------------------------------------- format

    #[test]
    fn cbx_string_format_matches() {
        check(
            ee::s10::f_003194a8__generated,
            cbx_string_format,
            56,
            &|c, seed| {
                let mut r = heap_alloc_machine(c, seed, 0x3194_a8);
                let this = DATA + 0x104;
                let buf = DATA + 0x1300;
                let fmt = DATA + 0x2100;
                let arg = DATA + 0x2200;
                string_obj(c, this, buf, 1, 5, 0x100);
                put_cstr(c, buf, b"hello");
                fill(c, &mut r, buf + 8, 4);
                put_cstr(c, arg, b"w\xe9rld");
                let fmts: [&[u8]; 7] = [
                    b"%d",
                    b"%s",
                    b"v=%d s=%s!",
                    b"%x-%c",
                    b"%d-%d",
                    b"100%%",
                    b"%d %d %d %d %d %d",
                ];
                let which = seed as usize % 7;
                put_cstr(c, fmt, fmts[which]);
                // Distinct non-zero float arguments: each spill slot holds
                // its own register.
                for (i, f) in [12, 14, 16, 18].into_iter().enumerate() {
                    c.f[f] = 0x3f80_0000 + 0x0011_0000 * i as u32 + seed;
                }
                c.pc = 0x0031_94a8;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = fmt as u64;
                // The `%s` form takes its string in a2; the rest take ints.
                c.g[A2][0] = if which == 1 {
                    arg as u64
                } else {
                    [0, 42, 0xffff_ffff, 123456][seed as usize % 4]
                };
                c.g[A3][0] = if which == 2 {
                    arg as u64
                } else {
                    (seed as u64).wrapping_mul(131).wrapping_add(7)
                };
                // Distinct non-zero register varargs, upper words set too.
                for (i, t) in [T0, T1, T2, T3].into_iter().enumerate() {
                    c.g[t][0] = 0x1357_0000_0000_0100 * (i as u64 + 1) + seed as u64;
                }
            },
        );
    }

    // ------------------------------------------------------------------ trim

    driven!(gen_trim_right, ee::s10::f_003194f0__generated);

    driven!(hand_trim_right, cbx_string_trim_right);

    /// Strings with no, partial and full trailing runs (the game's class
    /// table: tab, newline and space, and 0x80, whose sign-extended class
    /// has the space bit; 0x81's has not), picked by `key % 8`; shared by
    /// `key / 8 % 2` so the detach takes both paths.
    pub(crate) fn trim_right_fixture(c: &mut Cpu, seed: u32, key: u32) {
        bxs_ctype_image(c);
        let mut r = heap_alloc_machine(c, seed, 0x3194_f0);
        let this = DATA + 0x104;
        let buf = DATA + 0x1300;
        let bodies: [&[u8]; 8] = [
            b"",
            b"abc",
            b"ab  \t",
            b"   ",
            b"a\x80",
            b"\x81b\x80 \x80",
            b"x\x89 y",
            b" \n\x81",
        ];
        let body = bodies[key as usize % 8];
        string_obj(
            c,
            this,
            buf,
            if key / 8 % 2 == 1 { 3 } else { 1 },
            body.len() as u32,
            0x20,
        );
        put_cstr(c, buf, body);
        fill(c, &mut r, buf + 12, 4);
        c.pc = 0x0031_94f0;
        c.g[A0][0] = this as u64;
    }

    /// Totals of the generated body: every total the 32 seeds give: the empty early
    /// out, the scan with and without the truncation on a private buffer
    /// (29-91), and the shared buffer's detach (350-488).
    #[test]
    fn cbx_string_trim_right_matches() {
        check_paths(
            ee::s10::f_003194f0__generated,
            cbx_string_trim_right,
            32,
            &[29, 58, 64, 69, 75, 91, 350, 369, 374, 465, 488],
            &|c, seed| {
                trim_right_fixture(c, seed, seed);
            },
        );
        // The whitespace scan's event check, delivered and unwinding: the
        // lengths vary, so the first unwind lands on different iterations.
        check_events(gen_trim_right, hand_trim_right, 96, &|c, seed| {
            trim_right_fixture(c, seed, seed / 3);
            let mut r = Rng(0x3194_f000 ^ seed as u64);
            let mode = schedule(c, &mut r, seed, 0x0031_94f0);
            if mode != Events::Off {
                c.next_event = c.cycles;
            }
            mode
        });
    }

    driven!(gen_trim_left, ee::s10::f_00319578__generated);

    driven!(hand_trim_left, cbx_string_trim_left);

    /// Strings with no, partial and full leading runs (the game's class
    /// table: tab, newline and space, and 0x80/0x89/0x8a, whose
    /// sign-extended classes have the space bit; 0x81's and 0x87's have
    /// not), picked by `key % 8`; shared by `key / 8 % 2` so the detach
    /// takes both paths.
    pub(crate) fn trim_left_fixture(c: &mut Cpu, seed: u32, key: u32) {
        bxs_ctype_image(c);
        let mut r = heap_alloc_machine(c, seed, 0x3195_78);
        let this = DATA + 0x104;
        let buf = DATA + 0x1300;
        let bodies: [&[u8]; 8] = [
            b"",
            b"abc",
            b"  \tabc",
            b"   ",
            b"\x80\x80x",
            b"\x81abc",
            b" \x80 \x87z",
            b"\n\x89\x8a",
        ];
        let body = bodies[key as usize % 8];
        string_obj(
            c,
            this,
            buf,
            if key / 8 % 2 == 1 { 3 } else { 1 },
            body.len() as u32,
            0x20,
        );
        put_cstr(c, buf, body);
        fill(c, &mut r, buf + 12, 4);
        c.pc = 0x0031_9578;
        c.g[A0][0] = this as u64;
    }

    /// Totals of the generated body: every total the 32 seeds give: no leading run
    /// and runs of each length on a private buffer (61-123), and the shared
    /// buffer's detach (370-497).
    #[test]
    fn cbx_string_trim_left_matches() {
        check_paths(
            ee::s10::f_00319578__generated,
            cbx_string_trim_left,
            32,
            &[61, 82, 84, 89, 98, 105, 123, 370, 372, 467, 483, 497],
            &|c, seed| {
                trim_left_fixture(c, seed, seed);
            },
        );
        // The whitespace scan's event check, delivered and unwinding.
        check_events(gen_trim_left, hand_trim_left, 96, &|c, seed| {
            trim_left_fixture(c, seed, seed / 3);
            let mut r = Rng(0x3195_7800 ^ seed as u64);
            let mode = schedule(c, &mut r, seed, 0x0031_9578);
            if mode != Events::Off {
                c.next_event = c.cycles;
            }
            mode
        });
    }

    pub(crate) use crate::game::misc::tests::fv_fixture;

    /// One format per specifier class the two jump tables dispatch: ints,
    /// chars (`c`/`C`), strings (`s`/`S`), floats (`e`/`f`/`g`, `G` through
    /// the first table's default), `%X`, count, unknown, length modifiers,
    /// digit widths, digit precisions, `'*'` widths/precisions, every flag,
    /// mixed forms, escapes, a bare trailing `%` (which scans one past the
    /// NUL) and the empty format.
    pub(crate) const FV_FMTS: [&[u8]; 48] = [
        b"%d",
        b"%x",
        b"%o",
        b"%u",
        b"%i",
        b"%p",
        b"%c",
        b"%C",
        b"%s",
        b"%S",
        b"%e",
        b"%f",
        b"%g",
        b"%G",
        b"%X",
        b"%d%n",
        b"%q",
        b"%ld",
        b"%hd",
        b"%Fd",
        b"%Nd",
        b"%5d",
        b"%.3d",
        b"%8.3d",
        b"%.2s",
        b"%8.2s",
        b"%3s",
        b"%.0s",
        b"%*d",
        b"%.*d",
        b"%-d",
        b"%+d",
        b"% d",
        b"%#x",
        b"%05d",
        b"v=%d s=%s!",
        b"x%d%s",
        b"%d%s",
        b"%d-%d",
        b"100%%",
        b"plain",
        b"%d%",
        b"a%db",
        b"%#d",
        b"",
        b"%m",
        b"%T",
        b"%y",
    ];

    /// `cBXString_FormatV`: every jump-table class against the generated
    /// body, argument values crossed on `seed / FV_FMTS.len()` digits
    /// (the tail entries `%m`, `%T`, `%y` cover the table-boundary
    /// classes). No `check_paths`: the totals depend on the real
    /// callees (`strtol`, `strlen`, `vsprintf`), which the fixtures
    /// cannot run to pin.
    #[test]
    fn cbx_string_format_v_matches() {
        use crate::difftest::check;
        check(
            ee::s10::f_003191c0__generated,
            cbx_string_format_v,
            200,
            &|c, seed| {
                let which = seed as usize % FV_FMTS.len();
                fv_fixture(c, seed, FV_FMTS[which], which);
            },
        );
    }

    /// Tie/boundary formats for the clamp/second/strlen rows: float/int
    /// width+precision sums at exactly 128/32, width==item and
    /// item==precision pairs, a nonempty `strlen` probe, and `'*'`-driven
    /// widths. Layouts reuse existing `which` shapes; the `strlen` probe
    /// plants a string address at `[THIS-8]`.
    pub(crate) const FV_TIES: [(&[u8], usize); 10] = [
        (b"%120.8f", 10),
        (b"%112.8f", 10),
        (b"%24.8d", 28),
        (b"%22.8d", 28),
        (b"%6d", 28),
        (b"%.6d", 28),
        (b"%s", 8),
        (b"%5s", 8),
        (b"%*d", 28),
        (b"%.*d", 28),
    ];

    /// `cBXString_FormatV` tie/boundary formats.
    #[test]
    fn cbx_string_format_v_ties() {
        use crate::difftest::check;
        use crate::game::misc::tests::{FV_SARG, FV_THIS};
        check(
            ee::s10::f_003191c0__generated,
            cbx_string_format_v,
            120,
            &|c, seed| {
                let (fmt, which) = FV_TIES[seed as usize % FV_TIES.len()];
                fv_fixture(c, seed, fmt, which);
                c.bus.write32(FV_THIS - 8, FV_SARG);
                // `%s` strlen reads `[S1]` after the slot advance, i.e. the
                // string-object header: point it at the nonempty arg so the
                // `strlen == 0` row can die.
                c.bus.write32(FV_THIS, FV_SARG);
            },
        );
    }
    pub(crate) use crate::game::misc::tests::{FV_ARGS, FV_FMT, FV_THIS};

    driven!(gen_format_v, ee::s10::f_003191c0__generated);

    driven!(hand_format_v, cbx_string_format_v);

    /// Short event formats by `seed % 4`: a leading plain character (the
    /// first unwind lands on the scan head), a leading flag (the flag head),
    /// a call-heavy format (delivered but never unwound, so no mid-call
    /// unwind), and a call-free format.
    pub(crate) const FV_EVENT_FMTS: [&[u8]; 4] = [b"a%db", b"%#x", b"%d%s", b"ab"];

    /// Event setup for [`cbx_string_format_v_events`]: the shared fixture
    /// with a short format, then a main-thread event schedule. Unwind mode
    /// only runs where the first back edge precedes any call; the call-heavy
    /// format is forced to deliver-only. Returns the event mode.
    pub(crate) fn fv_events_setup(c: &mut Cpu, seed: u32) -> crate::difftest::events::Events {
        let variant = (seed % 4) as usize;
        let mut mode = seed / 4 % 3;
        if variant == 2 && mode == 1 {
            // "%d%s" would unwind inside `strtol`: deliver without unwinding.
            mode = 2;
        }
        // Slot layouts matching each short format's shape.
        let which = [42, 33, 37, 40][variant];
        fv_fixture(c, seed, FV_EVENT_FMTS[variant], which);
        // `schedule` starts the thread (clobbering the entry registers), so
        // the entry state goes in after it.
        let mut r = Rng(0x3191_c000 ^ seed as u64);
        let ev = schedule(c, &mut r, mode, 0x0031_91c0);
        c.pc = 0x0031_91c0;
        c.g[A0][0] = FV_THIS as u64;
        c.g[A1][0] = FV_FMT as u64;
        c.g[A2][0] = FV_ARGS as u64;
        c.cycles = 1000;
        if mode != 0 {
            // First back edge delivers at once, before any call runs.
            c.next_event = 1000;
        }
        ev
    }

    /// `cBXString_FormatV` with real events: both sides driven through
    /// unwinds, the first unwind landing on the scan head (seed 4) and the
    /// flag head (seed 5) in turn. `RESUMES` counts print with the test.
    #[test]
    fn cbx_string_format_v_events() {
        check_events(gen_format_v, hand_format_v, 48, &fv_events_setup);
        use crate::difftest::{fresh, run_generated};
        for (seed, head) in [(4u32, 0x0031_9210u32), (5, 0x0031_9258)] {
            let mut c = fresh();
            fv_events_setup(&mut c, seed);
            c.next_event = c.cycles;
            run_generated(&mut c, ee::s10::f_003191c0__generated);
            assert_eq!(c.pc, head | 1, "seed {seed}: first unwind");
        }
    }
}
