//! `src/be/beintstat.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::festaterewards::r;
pub(crate) use crate::game::sound::soundsys::gp_4 as gp;
pub(crate) use crate::game::sound::soundsys::p;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0014_9778,
        bechar_int_is_trick_score_above_rating,
        "BECharInt_IsTrickScoreAboveRating",
    ),
    (0x0014_99a8, belibrary_dtor, "BELibrary_dtor"),
    (0x0014_a078, be_empty_stub_a078, "BE_EmptyStub_a078"),
    (
        0x0014_9a88,
        be_library_reset_player_profile,
        "BELibrary_ResetPlayerProfile",
    ),
    (0x0014_9860, be_library_ctor, "BELibrary_ctor"),
    (0x0014_9bb8, belibrary_load_char_db, "BELibrary_LoadCharDB"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {

    pub(crate) use crate::game::dirtysock::tagsunk::callee::BE_BAGT_INT_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::BE_CHAR_INT_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::BE_NEW_PLAYER_INT_GET_INSTANCE;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::FORMAT_TAGGED;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::MEMSET_3 as MEMSET;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::SPRINTF_6 as SPRINTF;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::STR_HASH_4 as STR_HASH;

    pub(crate) use crate::game::fe::festaterewards::callee::BECHAR_GET_OVERALL_RATING;

    pub(crate) use crate::game::fe::festaterewards::callee::BENEWPLAYER_IS_SHARING_CHAR_RECORD;
}

// ---------------------------------------------------------------------------
// BE library.

/// `BELibrary::~BELibrary(this, int flags)` (0x1499a8): free every singleton
/// the library owns and optionally the library itself.
pub fn belibrary_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 48;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        c.g[A0][0] = Addr(gp(c, -0x1f3c)).lw(c);
        c.sd(r32(c, SP), c.g[RA][0]);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, 0x0031_7e50, 0x0014_99c8)?;
        c.g[A0][0] = Addr(gp(c, -0x1f30)).lw(c);
        Addr(gp(c, -0x1f3c)).sw(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0014_99d4)?;
        c.g[A0][0] = Addr(gp(c, -0x1ee8)).lw(c);
        Addr(gp(c, -0x1f30)).sw(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0014_99e0)?;
        c.g[A0][0] = Addr(gp(c, -0x1f2c)).lw(c);
        Addr(gp(c, -0x1ee8)).sw(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0014_99ec)?;
        c.g[A0][0] = Addr(gp(c, -0x1ee4)).lw(c);
        Addr(gp(c, -0x1f2c)).sw(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0014_99f8)?;
        Addr(gp(c, -0x1ee4)).sw(c, 0);
        call_ptr(c, 0x0015_3050, 0x0014_9a00)?;
        c.g[A0][0] = Addr(gp(c, -0x1ee0)).lw(c);
        call_ptr(c, 0x0031_7e50, 0x0014_9a08)?;
        c.g[A0][0] = Addr(gp(c, -0x1eec)).lw(c);
        Addr(gp(c, -0x1ee0)).sw(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0014_9a14)?;
        c.g[A0][0] = Addr(gp(c, -0x1ea8)).lw(c);
        Addr(gp(c, -0x1eec)).sw(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0014_9a20)?;
        c.g[A0][0] = Addr(gp(c, -0x1f28)).lw(c);
        Addr(gp(c, -0x1ea8)).sw(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0014_9a2c)?;
        c.g[A0][0] = Addr(gp(c, -0x1eb8)).lw(c);
        Addr(gp(c, -0x1f28)).sw(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0014_9a38)?;
        c.g[A0][0] = Addr(gp(c, -0x1edc)).lw(c);
        Addr(gp(c, -0x1eb8)).sw(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0014_9a44)?;
        c.g[A0][0] = Addr(gp(c, -0x1ea4)).lw(c);
        Addr(gp(c, -0x1edc)).sw(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0014_9a50)?;
        c.g[A0][0] = Addr(gp(c, -0x1e8c)).lw(c);
        Addr(gp(c, -0x1ea4)).sw(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0014_9a5c)?;
        c.g[S0][0] = c.g[S0][0] & 1;
        let keep = c.g[S0][0] == 0;
        Addr(gp(c, -0x1e8c)).sw(c, 0);
        if !keep {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0014_9a70)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BE character queries.

/// `int BECharInt_IsTrickScoreAboveRating(char* self, void* rider, int value)`
/// (0x149778): true when `value` is at least the rating threshold of the
/// rider's overall rating level (clamped to the level table).
pub fn bechar_int_is_trick_score_above_rating(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        c.g[S2][0] = c.g[A2][0];
        c.g[A0][0] = c.g[S0][0];
        callee::BENEWPLAYER_IS_SHARING_CHAR_RECORD.call(c, 0x0014_97a0)?;

        let shared = r(c, V0) != 0;
        c.g[V0][0] = 0;
        if shared {
            return bechar_rating_tail(c, frame);
        }

        c.cycles += 30;
        c.g[V0][0] = 0x460000;
        c.g[A0][0] = c.g[S1][0];
        c.g[T0][0] = sx(r(c, V0).wrapping_sub(0x5d50));
        // Copy the 0x2c-byte rating table to the stack.
        c.ldl(3, p(r(c, T0), 0x7));
        c.ldr(3, r(c, T0));
        c.ldl(6, p(r(c, T0), 0xf));
        c.ldr(6, p(r(c, T0), 0x8));
        c.ldl(7, p(r(c, T0), 0x17));
        c.ldr(7, p(r(c, T0), 0x10));
        c.sdl(3, p(r(c, SP), 0x7));
        c.sdr(3, r(c, SP));
        c.sdl(6, p(r(c, SP), 0xf));
        c.sdr(6, p(r(c, SP), 0x8));
        c.sdl(7, p(r(c, SP), 0x17));
        c.sdr(7, p(r(c, SP), 0x10));
        c.ldl(3, p(r(c, T0), 0x1f));
        c.ldr(3, p(r(c, T0), 0x18));
        c.ldl(6, p(r(c, T0), 0x27));
        c.ldr(6, p(r(c, T0), 0x20));
        c.g[A3][0] = Addr(p(r(c, T0), 0x28)).lw(c);
        c.sdl(3, p(r(c, SP), 0x1f));
        c.sdr(3, p(r(c, SP), 0x18));
        c.sdl(6, p(r(c, SP), 0x27));
        c.sdr(6, p(r(c, SP), 0x20));
        c.sw(p(r(c, SP), 0x28), c.g[A3][0]);
        c.g[A1][0] = c.g[S0][0];
        callee::BECHAR_GET_OVERALL_RATING.call(c, 0x0014_9814)?;

        c.g[V0][0] = sx(r(c, V0).wrapping_sub(0x1));
        let negative = (c.g[V0][0] as i64) < 0;
        c.g[A0][0] = 0xa;
        let level;
        if negative {
            level = 0;
        } else {
            c.cycles += 4;
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = c.g[A0][0];
            }
            level = 1;
        }
        if level == 0 {
            c.cycles += 1;
            c.g[V1][0] = 0;
        }

        c.cycles += 5;
        c.g[V0][0] = sx(r(c, V1) << 2);
        c.g[V1][0] = sx(p(r(c, SP), r(c, V0)));
        c.g[V0][0] = Addr(r(c, V1)).lw(c);
        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        c.g[V0][0] = c.g[V0][0] ^ 1;
        bechar_rating_tail(c, frame)
    })
}

/// The `lq s0-s2; ld ra; addiu sp; jr ra` tail.
pub(crate) fn bechar_rating_tail(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 6;
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore_ra(c, 0x30);
    frame.close(c);
    Ok(())
}

/// `void BELibrary_ResetPlayerProfile(BELibrary *self, int idx)` (0x149a88):
/// clear and initialise the rider-table entry for `idx`, format its label,
/// clear the profile-string slot, then reset the player, character and BAGT
/// interfaces' per-rider caches.
pub fn belibrary_reset_player_profile(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 76;
        let frame = Frame::open(c, 0x90);
        frame.save(c, 19, 0x50);
        frame.save(c, 17, 0x70);
        frame.save(c, 16, 0x80);
        frame.save(c, 18, 0x60);
        frame.save_ra(c, 0x40);

        // idx = (signed char)arg1, entry = D_00535B20 + idx * 0x1c.
        let idx = (abi::arg(c, 1) as u32 as i8) as i32 as u32;
        c.g[V0][0] = 0x0053_0000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5b20));
        c.g[S3][0] = sx(idx);
        c.g[S1][0] = 0x1c;
        c.mult(0, 17, 19, 17);
        c.g[A2][0] = 0x1c;
        c.g[A1][0] = 0;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let entry = c.g[S1][0] as u32;
        c.g[A0][0] = c.g[S1][0];
        callee::MEMSET.call(c, 0x0014_9acc)?;

        // entry->flags |= 2; entry->+0x11 = 4.
        c.g[V1][0] = c.lw(entry.wrapping_add(0x10));
        c.g[V0][0] = 4;
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A0][0] = 0x0046_0000;
        c.g[V1][0] |= 2;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5d98));
        c.sw(entry.wrapping_add(0x10), c.g[V1][0]);
        c.sb(entry.wrapping_add(0x11), c.g[V0][0]);

        // o = *(world + 0x8c); vt = *(o + 4); label = o + vt->delta.
        let o = c.lw((c.g[A1][0] as u32).wrapping_add(0x8c)) as u32;
        let vt = c.lw(o.wrapping_add(4)) as u32;
        c.g[S2][0] = sx(vt);
        c.g[V0][0] = c.lh(vt.wrapping_add(0x20));
        c.g[S0][0] = sx(o);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[S2][0] = sx(vt);
        callee::STR_HASH.call(c, 0x0014_9b00)?;

        // label = FormatTagged((char*)sp, vt->slot(o)); then
        // sprintf(entry, label, idx + 1).
        c.g[V1][0] = c.lw(vt.wrapping_add(0x24));
        c.g[A0][0] = sx(o);
        let f = c.g[V1][0] as u32;
        c.g[A1][0] = c.g[V0][0];
        crate::call::call_ptr(c, f, 0x0014_9b10)?;
        let buf = c.g[SP][0] as u32;
        c.g[A0][0] = sx(buf);
        c.g[A1][0] = c.g[V0][0];
        callee::FORMAT_TAGGED.call(c, 0x0014_9b1c)?;

        c.g[A1][0] = sx(buf);
        c.g[A2][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
        c.g[A0][0] = c.g[S1][0];
        callee::SPRINTF.call(c, 0x0014_9b2c)?;

        // Clear the profile-string table's slot for this rider.
        c.g[V1][0] = 0x9b50;
        c.g[A0][0] = 0x004a_0000;
        c.mult(0, 19, 19, 3);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6ca8));
        c.g[A1][0] = 1;
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[RA][0] = sx(0x0014_9b48);
        c.pc = 0x0015_67b8;
        ee::s01::BE_ResetProfileCharacters(c);
        if c.pc != 0x0014_9b48 {
            return Err(crate::call::Unwound);
        }

        reset_interface(
            c,
            callee::BE_NEW_PLAYER_INT_GET_INSTANCE,
            0x0014_9b50,
            0x0014_9b64,
        )?;
        reset_interface(
            c,
            callee::BE_CHAR_INT_GET_INSTANCE,
            0x0014_9b6c,
            0x0014_9b80,
        )?;
        reset_interface(
            c,
            callee::BE_BAGT_INT_GET_INSTANCE,
            0x0014_9b88,
            0x0014_9b9c,
        )?;

        frame.restore(c, 16, 0x80);
        frame.restore(c, 17, 0x70);
        frame.restore(c, 18, 0x60);
        frame.restore(c, 19, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// The tail shared by the three interfaces: get the singleton, then call its
/// vtable slot `{int16 delta; fn}`(+0x10/+0x14) with `this + delta`.
pub(crate) fn reset_interface(
    c: &mut Cpu,
    get: Callee,
    get_ret: u32,
    vt_ret: u32,
) -> crate::call::Flow {
    get.call(c, get_ret)?;
    let obj = c.g[V0][0];
    c.g[V1][0] = c.lw((obj as u32).wrapping_add(0xc));
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
    let f = c.g[A1][0] as u32;
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    crate::call::call_ptr(c, f, vt_ret)?;
    Ok(())
}

/// `BELibrary* BELibrary_ctor(self)` (0x149860): loads the character DB
/// (`BELibrary_LoadCharDB`), copies the 0xa28-byte default block at 0x43fb28
/// to 0x535c18 (8-byte aligned: the aligned 32-byte copy loop always runs; 81
/// iterations plus 8 bytes) and creates every interface singleton, in order
/// (race, player, char DB, char, option, save, BAGT, network, score, bolt,
/// mission, econ, aggression, reward). Returns `self`. Frame 0x20 (s0).
///
/// NOT installed: no test. `BELibrary_LoadCharDB` is a direct callee with no
/// override slot, and its own callees (`RewardsFile_Load`, the async file
/// open) cannot be bounded on test data.
pub fn be_library_ctor(c: &mut Cpu) {
    const LOAD_CHAR_DB: Callee = Callee {
        addr: 0x0014_9bb8,
        f: ee::s01::f_00149bb8,
    };
    const INSTANCES: [(Callee, u32); 14] = [
        (
            Callee {
                addr: 0x0014_48d8,
                f: ee::s01::f_001448d8,
            },
            0x0014_9928,
        ),
        (
            Callee {
                addr: 0x0014_5970,
                f: ee::s01::f_00145970,
            },
            0x0014_9930,
        ),
        (
            Callee {
                addr: 0x0014_edd8,
                f: ee::s01::f_0014edd8,
            },
            0x0014_9938,
        ),
        (
            Callee {
                addr: 0x0014_7f78,
                f: ee::s01::f_00147f78,
            },
            0x0014_9940,
        ),
        (
            Callee {
                addr: 0x0014_f250,
                f: ee::s01::f_0014f250,
            },
            0x0014_9948,
        ),
        (
            Callee {
                addr: 0x0015_2638,
                f: ee::s01::f_00152638,
            },
            0x0014_9950,
        ),
        (
            Callee {
                addr: 0x0014_f900,
                f: ee::s01::f_0014f900,
            },
            0x0014_9958,
        ),
        (
            Callee {
                addr: 0x0014_e0e0,
                f: ee::s01::f_0014e0e0,
            },
            0x0014_9960,
        ),
        (
            Callee {
                addr: 0x0015_4948,
                f: ee::s01::f_00154948,
            },
            0x0014_9968,
        ),
        (
            Callee {
                addr: 0x0014_abe0,
                f: ee::s01::f_0014abe0,
            },
            0x0014_9970,
        ),
        (
            Callee {
                addr: 0x0015_32a0,
                f: ee::s01::f_001532a0,
            },
            0x0014_9978,
        ),
        (
            Callee {
                addr: 0x0015_08d8,
                f: ee::s01::f_001508d8,
            },
            0x0014_9980,
        ),
        (
            Callee {
                addr: 0x0015_5a50,
                f: ee::s01::f_00155a50,
            },
            0x0014_9988,
        ),
        (
            Callee {
                addr: 0x0015_69c0,
                f: ee::s01::f_001569c0,
            },
            0x0014_9990,
        ),
    ];
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        LOAD_CHAR_DB.call(c, 0x0014_9874)?;
        c.g[V0][0] = 0x53_0000;
        c.g[V1][0] = 0x44_0000;
        c.g[A0][0] = sx(r32(c, V0).wrapping_add(0x5c18));
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x4d8));
        c.g[V0][0] = c.g[V1][0] | c.g[A0][0];
        c.g[V0][0] &= 7;
        let unaligned = c.g[V0][0] != 0;
        c.g[V0][0] = sx(r32(c, V1).wrapping_add(0xa20));
        debug_assert!(!unaligned, "0x43fb28 and 0x535c18 are 8-byte aligned");
        loop {
            c.cycles += 11;
            let src = r32(c, V1);
            let dst = r32(c, A0);
            c.g[A1][0] = c.ld(src);
            c.g[A2][0] = c.ld(src.wrapping_add(8));
            c.g[A3][0] = c.ld(src.wrapping_add(0x10));
            c.g[T0][0] = c.ld(src.wrapping_add(0x18));
            c.sd(dst, c.g[A1][0]);
            c.sd(dst.wrapping_add(8), c.g[A2][0]);
            c.sd(dst.wrapping_add(0x10), c.g[A3][0]);
            c.sd(dst.wrapping_add(0x18), c.g[T0][0]);
            c.g[V1][0] = sx(src.wrapping_add(0x20));
            let more = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = sx(dst.wrapping_add(0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0014_98e8);
            }
        }
        c.cycles += 36;
        let src = r32(c, V1);
        let dst = r32(c, A0);
        c.ldl(V0, src.wrapping_add(7));
        c.ldr(V0, src);
        c.sdl(V0, dst.wrapping_add(7));
        c.sdr(V0, dst);
        for (callee, ret) in INSTANCES {
            callee.call(c, ret)?;
        }
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `BE_EmptyStub_a078` (0x14a078): does nothing, returns.
pub fn be_empty_stub_a078(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `cBEProfileLib_init` (0x1567b8): resets one rider's profile characters.
pub(crate) const PROFILE_LIB_INIT: Callee = Callee {
    addr: 0x0015_67b8,
    f: ee::s01::f_001567b8,
};

/// `BENewPlayerInt_GetInstance` (0x145970).
pub(crate) const PLAYER_INT_GET_INSTANCE: Callee = Callee {
    addr: 0x0014_5970,
    f: ee::s01::f_00145970,
};

/// `BECharInt_GetInstance` (0x147f78).
pub(crate) const CHAR_INT_GET_INSTANCE: Callee = Callee {
    addr: 0x0014_7f78,
    f: ee::s01::f_00147f78,
};

/// `BEBAGTInt_GetInstance` (0x14f900).
pub(crate) const BAGT_INT_GET_INSTANCE: Callee = Callee {
    addr: 0x0014_f900,
    f: ee::s01::f_0014f900,
};

/// The tail shared by the three interfaces: get the singleton, then call its
/// vtable method `{int16 delta; fn}` (at +0x10 / +0x14) on `this + delta`.
pub(crate) fn reset_profile_interface(
    c: &mut Cpu,
    get: Callee,
    get_ret: u32,
    method_ret: u32,
) -> Flow {
    get.call(c, get_ret)?;
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
    let method = c.g[A1][0] as u32;
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, method, method_ret)?;
    Ok(())
}

/// `void BELibrary_ResetPlayerProfile(BELibrary *self, int idx)` (0x149a88):
/// clears the rider slot `cBELibrary_playerDB[idx]` (0x1c bytes), marks it as
/// dual-shock character 4, formats its user name from the locale's label
/// (`sprintf(entry, label, idx + 1)`), clears that rider's profile characters,
/// then resets the player, character and BAGT interfaces' per-rider caches.
/// Frame 0x90: saves ra and s0-s3.
pub fn be_library_reset_player_profile(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 76;
        let frame = Frame::open(c, 0x90);
        frame.save(c, 19, 0x50);
        frame.save(c, 17, 0x70);
        frame.save(c, 16, 0x80);
        frame.save(c, 18, 0x60);
        frame.save_ra(c, 0x40);

        // idx = (signed char)a1; entry = 0x535b20 + idx * 0x1c.
        let idx = (c.g[A1][0] as u8 as i8) as i32;
        c.g[S3][0] = sx(idx as u32);
        c.g[S1][0] = 0x1c;
        c.mult(0, 17, 19, 17);
        c.g[V0][0] = 0x0053_5b20;
        c.g[A2][0] = 0x1c;
        c.g[A1][0] = 0;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let entry = c.g[S1][0] as u32;

        // memset(entry, 0, 0x1c).
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0041_6210, 0x0014_9acc)?;

        // entry->flags |= 2; entry->character = 4.
        c.g[V1][0] = c.lw(entry.wrapping_add(0x10));
        c.g[V0][0] = 4;
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A0][0] = 0x0046_0000;
        c.g[V1][0] |= 2;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5d98));
        c.sw(entry.wrapping_add(0x10), c.g[V1][0]);
        c.sb(entry.wrapping_add(0x11), c.g[V0][0]);

        // o = world->locale; vt = o->vtable; label method on this = o + vt->delta.
        c.g[S0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8c));
        c.g[S2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(4));
        c.g[V0][0] = c.lh((c.g[S2][0] as u32).wrapping_add(0x20));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        call_ptr(c, 0x0031_7670, 0x0014_9b00)?;

        // label = o->vt->slot(this, hash); FormatTagged(buf = sp, label);
        // sprintf(entry, buf, idx + 1).
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x24));
        c.g[A0][0] = c.g[S0][0];
        let label_method = c.g[V1][0] as u32;
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, label_method, 0x0014_9b10)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002c_2580, 0x0014_9b1c)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0041_7828, 0x0014_9b2c)?;

        // cBEProfileLib_init(cBELibrary_profileLib[idx], 1); the table is at
        // 0x4a6ca8 with 0x9b50-byte entries.
        c.g[V1][0] = 0x9b50;
        c.g[A0][0] = 0x004a_0000;
        c.mult(0, 19, 19, 3);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6ca8));
        c.g[A1][0] = 1;
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        PROFILE_LIB_INIT.call(c, 0x0014_9b48)?;

        reset_profile_interface(c, PLAYER_INT_GET_INSTANCE, 0x0014_9b50, 0x0014_9b64)?;
        reset_profile_interface(c, CHAR_INT_GET_INSTANCE, 0x0014_9b6c, 0x0014_9b80)?;
        reset_profile_interface(c, BAGT_INT_GET_INSTANCE, 0x0014_9b88, 0x0014_9b9c)?;

        frame.restore(c, 16, 0x80);
        frame.restore(c, 17, 0x70);
        frame.restore(c, 18, 0x60);
        frame.restore(c, 19, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `void BELibrary::LoadCharDB()` (0x149bb8, `cBELibrary::loadData`): reset
/// the back-end library: the bolts DB, the reward DB, the race DB (6 ids), the
/// six rider slots, load `data/be/charDB.dbl` into the character table, build
/// the ten rows of the equipment value table (+5 of the base table),
/// initialise the three profile libraries and fill the grab-point score table
/// with its constants.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut,
    clippy::all
)]
pub fn belibrary_load_char_db(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.g[A0][0] = 0x4a0000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6750));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S3][0] = 0x530000;
        Callee {
            addr: 0x0014c6a0,
            f: ee::s01::Bolts_LoadDB,
        }
        .call(c, 0x00149be0)?;
        c.g[A0][0] = 0x4c0000;
        c.g[A1][0] = 0u64;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3e98));
        Callee {
            addr: 0x0015ad58,
            f: ee::s01::RewardsFile_Load,
        }
        .call(c, 0x00149bf0)?;
        c.g[S0][0] = 0x530000;
        c.g[A1][0] = 0u64;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5b0));
        c.g[A2][0] = 0x4c;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x00416210, 0x00149c08)?;
        c.g[T4][0] = 0u64;
        'l_149c10: loop {
            c.cycles += 7;
            c.sw(c.g[S0][0] as u32, c.g[T4][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[T4][0]);
            c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = ((c.g[T4][0] as i64) < 6i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00149c10);
                }
                continue 'l_149c10;
            } else {
                c.cycles += 9;
                c.g[S0][0] = 0x530000;
                c.g[A1][0] = 0u64;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5b20));
                c.g[A2][0] = 0xa8;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00416210, 0x00149c44)?;
                c.g[A0][0] = 0x4;
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa8));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                'l_149c50: loop {
                    c.cycles += 8;
                    c.sb(c.g[S0][0] as u32, 0u64);
                    c.g[V0][0] = c.g[V0][0] | 0x2;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.sb((c.g[S0][0] as u32).wrapping_add(0x11), c.g[A0][0]);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                        if event_due(c) {
                            deliver_event_here(c, 0x00149c50);
                        }
                        continue 'l_149c50;
                    } else {
                        'b_149cac: {
                            c.cycles += 12;
                            c.g[A0][0] = 0x460000;
                            c.g[A1][0] = 0x100;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5d20));
                            call_ptr(c, 0x003e18c8, 0x00149c80)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A0][0] = 0x530000;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x970));
                            c.g[A1][0] = c.g[S0][0];
                            c.g[A2][0] = 0x550;
                            call_ptr(c, 0x0041605c, 0x00149c98)?;
                            let t = c.g[S0][0] == 0u64;
                            c.g[V0][0] = 0x530000;
                            if t {
                                break 'b_149cac;
                            } else {
                                c.cycles += 3;
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x00317e98, 0x00149ca8)?;
                                c.g[V0][0] = 0x530000;
                                break 'b_149cac;
                            }
                        }
                        c.cycles += 7;
                        c.g[V1][0] = 0x440000;
                        c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8d8));
                        c.g[T2][0] = sx((c.g[V1][0] as u32).wrapping_add(0xf00));
                        c.g[T4][0] = 0u64;
                        c.g[T5][0] = 0xb;
                        c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0xd));
                        'l_149cc8: loop {
                            c.cycles += 35;
                            c.g[A0][0] = c.lbu(c.g[T2][0] as u32);
                            c.g[A2][0] = c.lbu((c.g[T2][0] as u32).wrapping_add(0x1));
                            c.g[A3][0] = c.lbu((c.g[T2][0] as u32).wrapping_add(0x2));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5));
                            c.g[T0][0] = c.lbu((c.g[T2][0] as u32).wrapping_add(0x3));
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x5));
                            c.g[A1][0] = c.lbu((c.g[T2][0] as u32).wrapping_add(0x4));
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x5));
                            c.g[V1][0] = c.lbu((c.g[T2][0] as u32).wrapping_add(0x5));
                            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x5));
                            c.g[V0][0] = c.lbu((c.g[T2][0] as u32).wrapping_add(0x6));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5));
                            c.sb(c.g[T1][0] as u32, c.g[T4][0]);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5));
                            c.sb((c.g[T1][0] as u32).wrapping_add(0x1), c.g[A0][0]);
                            c.sb((c.g[T1][0] as u32).wrapping_add(0x7), c.g[V0][0]);
                            c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(0x1));
                            c.sb((c.g[T1][0] as u32).wrapping_add(0x2), c.g[A2][0]);
                            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x7));
                            c.sb((c.g[T1][0] as u32).wrapping_add(0x3), c.g[A3][0]);
                            c.g[V0][0] = ((c.g[T4][0] as i64) < 10i64) as u64;
                            c.sb((c.g[T1][0] as u32).wrapping_add(0x4), c.g[T0][0]);
                            c.sb((c.g[T1][0] as u32).wrapping_add(0x5), c.g[A1][0]);
                            c.sb((c.g[T1][0] as u32).wrapping_add(0x6), c.g[V1][0]);
                            c.sb((c.g[T3][0] as u32).wrapping_sub(0x4), c.g[T5][0]);
                            c.sb((c.g[T3][0] as u32).wrapping_sub(0x2), c.g[T5][0]);
                            c.sb((c.g[T3][0] as u32).wrapping_sub(0x5), c.g[T5][0]);
                            c.sb((c.g[T3][0] as u32).wrapping_sub(0x1), c.g[T5][0]);
                            c.sb((c.g[T3][0] as u32).wrapping_add(0x1), c.g[T5][0]);
                            c.sb(c.g[T3][0] as u32, c.g[T5][0]);
                            c.sb((c.g[T1][0] as u32).wrapping_add(0xa), c.g[T5][0]);
                            c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0xf));
                            let t = c.g[V0][0] != 0u64;
                            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0xf));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00149cc8);
                                }
                                continue 'l_149cc8;
                            } else {
                                c.cycles += 13;
                                c.g[A0][0] = 0x530000;
                                c.g[A1][0] = 0u64;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5898));
                                c.g[A2][0] = 0x288;
                                call_ptr(c, 0x00416210, 0x00149d68)?;
                                c.g[S2][0] = 0x9b50;
                                c.g[V1][0] = 0x4a0000;
                                c.g[V0][0] = 0x10000;
                                c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ca8));
                                c.g[V0][0] = c.g[V0][0] | 0xd1f0;
                                c.g[S1][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[A0][0] = c.g[S0][0];
                                'l_149d88: loop {
                                    c.cycles += 6;
                                    c.g[A1][0] = 0x1;
                                    Callee {
                                        addr: 0x001567b8,
                                        f: ee::s01::BE_ResetProfileCharacters,
                                    }
                                    .call(c, 0x00149d90)?;
                                    c.g[S0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A0][0] = c.g[S0][0];
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00149d88);
                                        }
                                        continue 'l_149d88;
                                    } else {
                                        c.cycles += 182;
                                        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x600));
                                        c.sw((c.g[S3][0] as u32).wrapping_add(0x600), 0u64);
                                        c.g[A1][0] = 0xc8;
                                        c.g[V1][0] = 0x190;
                                        c.g[A3][0] = 0x12c;
                                        c.g[T1][0] = 0xfa;
                                        c.g[T0][0] = 0x15e;
                                        c.g[A2][0] = 0x1f4;
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x70), c.g[T0][0]);
                                        c.g[A0][0] = 0xbb8;
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x48), c.g[T1][0]);
                                        c.g[T2][0] = 0x3e8;
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x88), c.g[A3][0]);
                                        c.g[T0][0] = 0x7d0;
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x8c), c.g[V1][0]);
                                        c.g[T3][0] = 0x9c4;
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
                                        c.g[T4][0] = 0xfa0;
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[A1][0]);
                                        c.g[T5][0] = 0x1388;
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x20), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x24), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x28), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x2c), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x30), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x34), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x38), c.g[T1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x3c), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x40), c.g[T1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x44), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x50), c.g[A3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x54), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x58), c.g[A3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x5c), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x60), c.g[A3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x64), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x68), c.g[A3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x6c), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x74), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x78), c.g[A3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x7c), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x80), c.g[A3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x84), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x98), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x9c), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x108), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x10c), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xa0), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xa4), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xa8), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xac), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xb0), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xb4), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xb8), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xbc), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xc0), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xc4), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xc8), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xcc), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xd0), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xd4), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xd8), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xdc), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xe0), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xe4), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xe8), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xec), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xf0), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xf4), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xf8), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xfc), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x100), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x104), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x148), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x14c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x158), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x15c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x168), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x16c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x150), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x154), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x160), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x164), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x170), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x174), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x178), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x17c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x188), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x18c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x198), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x19c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x180), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x184), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x190), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x194), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1a0), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1a4), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1a8), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1ac), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1b8), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1bc), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1c8), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1cc), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1b0), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1b4), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1c0), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1c4), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1d0), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1d4), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x118), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x11c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x128), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x12c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1e0), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1f0), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x200), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x138), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x13c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x120), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x124), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x130), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x134), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x140), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x144), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x208), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x20c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x218), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x21c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x228), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x22c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x210), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x214), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x220), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x224), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x230), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x234), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1d8), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1dc), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1e8), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1ec), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1f8), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1e4), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1f4), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x204), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x238), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x23c), c.g[T4][0]);
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                                        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x2a4), c.g[T5][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x280), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x284), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x240), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x244), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x248), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x24c), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x250), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x254), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x258), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x25c), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x260), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x264), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x268), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x26c), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x270), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x274), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x278), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x27c), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x288), c.g[T5][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x28c), c.g[T5][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x290), c.g[T5][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x294), c.g[T5][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x298), c.g[T5][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x29c), c.g[T5][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x2a0), c.g[T5][0]);
                                        let t = c.g[RA][0] as u32;
                                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
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
    })
}

/// `(game mode, game event, cycles)` that `SectionMan_OnEventTrigger` sets for
/// the section events 2..6 (the generated switch's cases).
#[allow(dead_code)]
pub(crate) fn oc673_section_event_modes(event: u32) -> Option<(u32, u32, u64)> {
    match event {
        2 => Some((0, 0, 8)),
        3 => Some((1, 1, 8)),
        4 => Some((2, 3, 8)),
        5 => Some((3, 2, 8)),
        6 => Some((5, 4, 6)),
        _ => None,
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::rider::tests::clear_2 as clear;
    pub(crate) use crate::game::fe::festaterewards::tests::base_9 as base_2;

    pub(crate) use crate::game::fe::festaterewards::tests::strict;

    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base;

    pub(crate) use crate::game::sound::soundsys::tests::w;

    use ssx3_ee as ee;

    #[test]
    fn belibrary_dtor_matches() {
        check(
            ee::s01::f_001499a8__generated,
            belibrary_dtor,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x1499a8, 2);
                let this = DATA;
                clear(c, this, 0x100);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = 0;
                for off in [
                    -0x1f3ci32, -0x1f30, -0x1ee8, -0x1f2c, -0x1ee4, -0x1ee0, -0x1eec, -0x1ea8,
                    -0x1f28, -0x1eb8, -0x1edc, -0x1ea4, -0x1e8c,
                ] {
                    w(
                        c,
                        GAME_GP.wrapping_sub(off as u32),
                        if seed % 2 == 0 { 0 } else { 0 },
                    );
                }
            },
        );
    }

    /// Call a setup helper (a generated function) with up to six arguments.
    pub(crate) fn call(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = 0x0012_3450;
        c.pc = addr;
        f(c);
        c.g[V0][0]
    }

    /// The engine's size-class pool allocator, enabled through `gp + 0xdd8`.
    pub(crate) fn pool_allocator(c: &mut Cpu) {
        w(c, GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32, u32); 7] = [
            (0x2a70, 0x800, 8),
            (0x2a60, 0x2000, 0x10),
            (0x2a6c, 0x32000, 0x20),
            (0x2a7c, 0x25800, 0x40),
            (0x2a84, 0x25800, 0x80),
            (0x2a78, 0x20000, 0x100),
            (0x2a68, 0x20000, 0x200),
        ];
        let mut nodes = DATA + 0x24_000;
        for (i, (slot, head, block)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base_2 = nodes;
            for n in 0..16u32 {
                let node = base_2 + block * n;
                w(c, node, if n == 15 { 0 } else { node + block });
            }
            w(c, pool + head, base_2);
            w(c, GAME_GP + slot, pool);
            nodes += block * 16;
        }
    }

    /// The dlmalloc arena the named allocators fall back to.
    pub(crate) const MEM_HEAP: u32 = 0x004f_f250;

    pub(crate) fn allocator(c: &mut Cpu) {
        pool_allocator(c);
        call(
            c,
            0x0031_ff60,
            ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000],
        );
    }

    // ------------------------------------------------------------ BE_2 char

    #[test]
    fn bechar_int_is_trick_score_above_rating_matches() {
        strict(
            ee::s01::f_00149778__generated,
            bechar_int_is_trick_score_above_rating,
            60,
            &|c, seed| {
                let _ = base_2(c, seed, 0x1497, 6);
                allocator(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                c.g[A2][0] = ((seed * 37) % 2000) as u64;
                // rider object for the BE_2 queries
                w(c, DATA + 0x800, (seed % 4) as u32);
                w(c, DATA + 0x804, ((seed / 4) % 5) as u32);
                w(c, DATA + 0x808, 1);
                w(c, DATA + 0x80c, 0);
                w(c, DATA + 0x810, (seed % 7) as u32);
            },
        );
    }

    use crate::difftest::scramble;
    use crate::difftest::Rng;

    #[test]
    fn be_empty_stub_a078_matches() {
        check(
            ssx3_ee::s01::f_0014a078__generated,
            be_empty_stub_a078,
            10,
            &|c, seed| {
                let mut r = Rng(0x14a0_7800 ^ seed as u64);
                scramble(c, &mut r);
            },
        );
    }

    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::fall_back;
    use crate::difftest::fakes::hook;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word for the singleton getter fakes (a magic word in the machine's RAM).
    pub(crate) const OC627_CTRL: u32 = DATA + 0x3f00;

    pub(crate) const OC627_MAGIC: u32 = 0x6267_0627;

    /// The three singletons and their shared vtable (`{delta 0, method FAKE}`
    /// at +0x10 / +0x14).
    pub(crate) const OC627_PLAYER_OBJ: u32 = DATA + 0x3900;

    pub(crate) const OC627_CHAR_OBJ: u32 = DATA + 0x3940;

    pub(crate) const OC627_BAGT_OBJ: u32 = DATA + 0x3980;

    pub(crate) const OC627_VTABLE: u32 = DATA + 0x39c0;

    pub(crate) static OC627_PREV_PLAYER_GET: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC627_PREV_CHAR_GET: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC627_PREV_BAGT_GET: AtomicUsize = AtomicUsize::new(0);

    /// A getter that returns `obj` while the gate is set, else the body it replaced.
    pub(crate) fn oc627_getter(c: &mut Cpu, prev: &AtomicUsize, generated: fn(&mut Cpu), obj: u32) {
        if c.bus.read32(OC627_CTRL) != OC627_MAGIC {
            return fall_back(c, prev, generated);
        }
        c.g[V0][0] = obj as u64;
        c.pc = c.g[RA][0] as u32;
    }

    pub(crate) fn oc627_player_get(c: &mut Cpu) {
        oc627_getter(
            c,
            &OC627_PREV_PLAYER_GET,
            ee::s01::f_00145970__generated,
            OC627_PLAYER_OBJ,
        )
    }

    pub(crate) fn oc627_char_get(c: &mut Cpu) {
        oc627_getter(
            c,
            &OC627_PREV_CHAR_GET,
            ee::s01::f_00147f78__generated,
            OC627_CHAR_OBJ,
        )
    }

    pub(crate) fn oc627_bagt_get(c: &mut Cpu) {
        oc627_getter(
            c,
            &OC627_PREV_BAGT_GET,
            ee::s01::f_0014f900__generated,
            OC627_BAGT_OBJ,
        )
    }

    #[test]
    fn be_library_reset_player_profile_matches() {
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00145970),
            oc627_player_get,
            &OC627_PREV_PLAYER_GET,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00147F78),
            oc627_char_get,
            &OC627_PREV_CHAR_GET,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014F900),
            oc627_bagt_get,
            &OC627_PREV_BAGT_GET,
        );
        covered(
            ee::s01::f_00149a88__generated,
            be_library_reset_player_profile,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0014_9a88, 3);
                let world = DATA + 0x3000;
                let locale = DATA + 0x3100;
                let vt = DATA + 0x3200;
                let text = DATA + 0x2000;
                // World at gp-0x848; world->+0x8c is the locale, locale->+4 its vtable.
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x8c, locale);
                c.bus.write32(locale + 4, vt);
                let delta = (r.next_u32() as i16) & !3;
                c.bus.write16(vt + 0x20, delta as u16);
                c.bus.write32(vt + 0x24, FAKE);
                // The label method (the first fake call) returns a format string.
                for (i, b) in b"Rider %d\0".iter().enumerate() {
                    c.bus.write8(text + i as u32, *b);
                }
                c.bus.write32(LOG_RESULTS, text);
                // The singletons (gated fakes) and their shared vtable.
                c.bus.write32(OC627_CTRL, OC627_MAGIC);
                c.bus.write32(OC627_VTABLE + 0x10, 0);
                c.bus.write32(OC627_VTABLE + 0x14, FAKE);
                for obj in [OC627_PLAYER_OBJ, OC627_CHAR_OBJ, OC627_BAGT_OBJ] {
                    c.bus.write32(obj + 0xc, OC627_VTABLE);
                }
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    use crate::difftest::fakes;

    /// Gate of this module's callee fakes: the magic word at `OC673_CTRL`.
    pub(crate) const OC673_CTRL: u32 = DATA + 0xf_e000;

    pub(crate) const OC673_MAGIC: u32 = 0x6730_73c1;

    /// `v0` of fake call `n` (0: first call) at `OC673_RESULTS + 4n`, or
    /// `OC673_NONE` for the fake's own pointer.
    pub(crate) const OC673_RESULTS: u32 = OC673_CTRL + 0x40;

    pub(crate) const OC673_NONE: u32 = 0xdead_beef;

    /// `(address, v0)` pairs: the result of every fake call to `address`.
    pub(crate) const OC673_BY_ADDR: u32 = OC673_CTRL + 0x100;

    /// A gated fake call: logs and clobbers like `fakes::fake`, then forces
    /// `v0` from the result table (unless the call unwound).
    pub(crate) fn oc673_fake_body(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let n = c.bus.read32(fakes::LOG_COUNT);
        fakes::fake(c, addr, ra);
        if c.pc == ra && n < 32 {
            let v = c.bus.read32(OC673_RESULTS + 4 * n);
            if v != OC673_NONE {
                c.g[V0][0] = sx(v);
            }
            for k in 0..16 {
                let slot = OC673_BY_ADDR + 8 * k;
                if c.bus.read32(slot) == addr {
                    c.g[V0][0] = sx(c.bus.read32(slot + 4));
                }
            }
        }
    }

    /// Gated fakes of the callees of this module's functions (each `hook`ed
    /// once per process; with the gate off they run the body they replaced).
    macro_rules! oc673_fakes {
        ($($fake:ident, $prev:ident, $addr:literal, $slot:path, $gen:path;)*) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC673_CTRL) != OC673_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc673_fake_body(c, $addr)
                }
            )*
            pub(crate) fn oc673_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc673_fakes! {
        oc673_fake_00113b10, OC673_PREV_00113B10, 0x0011_3b10, ee::s00::OVERRIDE_00113B10, ee::s00::f_00113b10__generated;
        oc673_fake_00113f38, OC673_PREV_00113F38, 0x0011_3f38, ee::s00::OVERRIDE_00113F38, ee::s00::f_00113f38__generated;
        oc673_fake_00113f88, OC673_PREV_00113F88, 0x0011_3f88, ee::s00::OVERRIDE_00113F88, ee::s00::f_00113f88__generated;
        oc673_fake_00114130, OC673_PREV_00114130, 0x0011_4130, ee::s00::OVERRIDE_00114130, ee::s00::f_00114130__generated;
        oc673_fake_00115b58, OC673_PREV_00115B58, 0x0011_5b58, ee::s00::OVERRIDE_00115B58, ee::s00::f_00115b58__generated;
        oc673_fake_00115d48, OC673_PREV_00115D48, 0x0011_5d48, ee::s00::OVERRIDE_00115D48, ee::s00::f_00115d48__generated;
        oc673_fake_00116120, OC673_PREV_00116120, 0x0011_6120, ee::s00::OVERRIDE_00116120, ee::s00::f_00116120__generated;
        oc673_fake_001161d0, OC673_PREV_001161D0, 0x0011_61d0, ee::s00::OVERRIDE_001161D0, ee::s00::f_001161d0__generated;
        oc673_fake_001162c8, OC673_PREV_001162C8, 0x0011_62c8, ee::s00::OVERRIDE_001162C8, ee::s00::f_001162c8__generated;
        oc673_fake_0012b180, OC673_PREV_0012B180, 0x0012_b180, ee::s00::OVERRIDE_0012B180, ee::s00::f_0012b180__generated;
        oc673_fake_00132060, OC673_PREV_00132060, 0x0013_2060, ee::s01::OVERRIDE_00132060, ee::s01::f_00132060__generated;
        oc673_fake_00132620, OC673_PREV_00132620, 0x0013_2620, ee::s01::OVERRIDE_00132620, ee::s01::f_00132620__generated;
        oc673_fake_00132770, OC673_PREV_00132770, 0x0013_2770, ee::s01::OVERRIDE_00132770, ee::s01::f_00132770__generated;
        oc673_fake_001446a0, OC673_PREV_001446A0, 0x0014_46a0, ee::s01::OVERRIDE_001446A0, ee::s01::f_001446a0__generated;
        oc673_fake_001448d8, OC673_PREV_001448D8, 0x0014_48d8, ee::s01::OVERRIDE_001448D8, ee::s01::f_001448d8__generated;
        oc673_fake_00144be0, OC673_PREV_00144BE0, 0x0014_4be0, ee::s01::OVERRIDE_00144BE0, ee::s01::f_00144be0__generated;
        oc673_fake_00144cc0, OC673_PREV_00144CC0, 0x0014_4cc0, ee::s01::OVERRIDE_00144CC0, ee::s01::f_00144cc0__generated;
        oc673_fake_00144df0, OC673_PREV_00144DF0, 0x0014_4df0, ee::s01::OVERRIDE_00144DF0, ee::s01::f_00144df0__generated;
        oc673_fake_001451e8, OC673_PREV_001451E8, 0x0014_51e8, ee::s01::OVERRIDE_001451E8, ee::s01::f_001451e8__generated;
        oc673_fake_00145970, OC673_PREV_00145970, 0x0014_5970, ee::s01::OVERRIDE_00145970, ee::s01::f_00145970__generated;
        oc673_fake_00146e10, OC673_PREV_00146E10, 0x0014_6e10, ee::s01::OVERRIDE_00146E10, ee::s01::f_00146e10__generated;
        oc673_fake_00147488, OC673_PREV_00147488, 0x0014_7488, ee::s01::OVERRIDE_00147488, ee::s01::f_00147488__generated;
        oc673_fake_00147f78, OC673_PREV_00147F78, 0x0014_7f78, ee::s01::OVERRIDE_00147F78, ee::s01::f_00147f78__generated;
        oc673_fake_0014abe0, OC673_PREV_0014ABE0, 0x0014_abe0, ee::s01::OVERRIDE_0014ABE0, ee::s01::f_0014abe0__generated;
        oc673_fake_0014c6a0, OC673_PREV_0014C6A0, 0x0014_c6a0, ee::s01::OVERRIDE_0014C6A0, ee::s01::f_0014c6a0__generated;
        oc673_fake_0014dc80, OC673_PREV_0014DC80, 0x0014_dc80, ee::s01::OVERRIDE_0014DC80, ee::s01::f_0014dc80__generated;
        oc673_fake_0014dd58, OC673_PREV_0014DD58, 0x0014_dd58, ee::s01::OVERRIDE_0014DD58, ee::s01::f_0014dd58__generated;
        oc673_fake_0014de68, OC673_PREV_0014DE68, 0x0014_de68, ee::s01::OVERRIDE_0014DE68, ee::s01::f_0014de68__generated;
        oc673_fake_0014e0e0, OC673_PREV_0014E0E0, 0x0014_e0e0, ee::s01::OVERRIDE_0014E0E0, ee::s01::f_0014e0e0__generated;
        oc673_fake_0014edd8, OC673_PREV_0014EDD8, 0x0014_edd8, ee::s01::OVERRIDE_0014EDD8, ee::s01::f_0014edd8__generated;
        oc673_fake_0014f250, OC673_PREV_0014F250, 0x0014_f250, ee::s01::OVERRIDE_0014F250, ee::s01::f_0014f250__generated;
        oc673_fake_0014f900, OC673_PREV_0014F900, 0x0014_f900, ee::s01::OVERRIDE_0014F900, ee::s01::f_0014f900__generated;
        oc673_fake_001508d8, OC673_PREV_001508D8, 0x0015_08d8, ee::s01::OVERRIDE_001508D8, ee::s01::f_001508d8__generated;
        oc673_fake_00152638, OC673_PREV_00152638, 0x0015_2638, ee::s01::OVERRIDE_00152638, ee::s01::f_00152638__generated;
        oc673_fake_001532a0, OC673_PREV_001532A0, 0x0015_32a0, ee::s01::OVERRIDE_001532A0, ee::s01::f_001532a0__generated;
        oc673_fake_00154948, OC673_PREV_00154948, 0x0015_4948, ee::s01::OVERRIDE_00154948, ee::s01::f_00154948__generated;
        oc673_fake_00155a50, OC673_PREV_00155A50, 0x0015_5a50, ee::s01::OVERRIDE_00155A50, ee::s01::f_00155a50__generated;
        oc673_fake_001567b8, OC673_PREV_001567B8, 0x0015_67b8, ee::s01::OVERRIDE_001567B8, ee::s01::f_001567b8__generated;
        oc673_fake_001569c0, OC673_PREV_001569C0, 0x0015_69c0, ee::s01::OVERRIDE_001569C0, ee::s01::f_001569c0__generated;
        oc673_fake_00158e30, OC673_PREV_00158E30, 0x0015_8e30, ee::s01::OVERRIDE_00158E30, ee::s01::f_00158e30__generated;
        oc673_fake_0015ad58, OC673_PREV_0015AD58, 0x0015_ad58, ee::s01::OVERRIDE_0015AD58, ee::s01::f_0015ad58__generated;
        oc673_fake_001a3590, OC673_PREV_001A3590, 0x001a_3590, ee::s03::OVERRIDE_001A3590, ee::s03::f_001a3590__generated;
        oc673_fake_001a3648, OC673_PREV_001A3648, 0x001a_3648, ee::s03::OVERRIDE_001A3648, ee::s03::f_001a3648__generated;
        oc673_fake_001f36d8, OC673_PREV_001F36D8, 0x001f_36d8, ee::s04::OVERRIDE_001F36D8, ee::s04::f_001f36d8__generated;
        oc673_fake_00227bf0, OC673_PREV_00227BF0, 0x0022_7bf0, ee::s05::OVERRIDE_00227BF0, ee::s05::f_00227bf0__generated;
        oc673_fake_00231250, OC673_PREV_00231250, 0x0023_1250, ee::s06::OVERRIDE_00231250, ee::s06::f_00231250__generated;
        oc673_fake_00232328, OC673_PREV_00232328, 0x0023_2328, ee::s06::OVERRIDE_00232328, ee::s06::f_00232328__generated;
        oc673_fake_00237cf8, OC673_PREV_00237CF8, 0x0023_7cf8, ee::s06::OVERRIDE_00237CF8, ee::s06::f_00237cf8__generated;
        oc673_fake_00238160, OC673_PREV_00238160, 0x0023_8160, ee::s06::OVERRIDE_00238160, ee::s06::f_00238160__generated;
        oc673_fake_00244400, OC673_PREV_00244400, 0x0024_4400, ee::s06::OVERRIDE_00244400, ee::s06::f_00244400__generated;
        oc673_fake_00244408, OC673_PREV_00244408, 0x0024_4408, ee::s06::OVERRIDE_00244408, ee::s06::f_00244408__generated;
        oc673_fake_0026d5e8, OC673_PREV_0026D5E8, 0x0026_d5e8, ee::s07::OVERRIDE_0026D5E8, ee::s07::f_0026d5e8__generated;
        oc673_fake_00278f68, OC673_PREV_00278F68, 0x0027_8f68, ee::s07::OVERRIDE_00278F68, ee::s07::f_00278f68__generated;
        oc673_fake_0027aa80, OC673_PREV_0027AA80, 0x0027_aa80, ee::s07::OVERRIDE_0027AA80, ee::s07::f_0027aa80__generated;
        oc673_fake_0028b180, OC673_PREV_0028B180, 0x0028_b180, ee::s07::OVERRIDE_0028B180, ee::s07::f_0028b180__generated;
        oc673_fake_002a4b68, OC673_PREV_002A4B68, 0x002a_4b68, ee::s08::OVERRIDE_002A4B68, ee::s08::f_002a4b68__generated;
        oc673_fake_002b3a98, OC673_PREV_002B3A98, 0x002b_3a98, ee::s08::OVERRIDE_002B3A98, ee::s08::f_002b3a98__generated;
        oc673_fake_002c7440, OC673_PREV_002C7440, 0x002c_7440, ee::s08::OVERRIDE_002C7440, ee::s08::f_002c7440__generated;
        oc673_fake_00309030, OC673_PREV_00309030, 0x0030_9030, ee::s10::OVERRIDE_00309030, ee::s10::f_00309030__generated;
        oc673_fake_00309f18, OC673_PREV_00309F18, 0x0030_9f18, ee::s10::OVERRIDE_00309F18, ee::s10::f_00309f18__generated;
        oc673_fake_00311ae8, OC673_PREV_00311AE8, 0x0031_1ae8, ee::s10::OVERRIDE_00311AE8, ee::s10::f_00311ae8__generated;
        oc673_fake_00311b20, OC673_PREV_00311B20, 0x0031_1b20, ee::s10::OVERRIDE_00311B20, ee::s10::f_00311b20__generated;
        oc673_fake_003128e8, OC673_PREV_003128E8, 0x0031_28e8, ee::s10::OVERRIDE_003128E8, ee::s10::f_003128e8__generated;
        oc673_fake_00312aa0, OC673_PREV_00312AA0, 0x0031_2aa0, ee::s10::OVERRIDE_00312AA0, ee::s10::f_00312aa0__generated;
        oc673_fake_00316898, OC673_PREV_00316898, 0x0031_6898, ee::s10::OVERRIDE_00316898, ee::s10::f_00316898__generated;
        oc673_fake_003168b0, OC673_PREV_003168B0, 0x0031_68b0, ee::s10::OVERRIDE_003168B0, ee::s10::f_003168b0__generated;
        oc673_fake_003168e8, OC673_PREV_003168E8, 0x0031_68e8, ee::s10::OVERRIDE_003168E8, ee::s10::f_003168e8__generated;
        oc673_fake_00316978, OC673_PREV_00316978, 0x0031_6978, ee::s10::OVERRIDE_00316978, ee::s10::f_00316978__generated;
        oc673_fake_00316ef8, OC673_PREV_00316EF8, 0x0031_6ef8, ee::s10::OVERRIDE_00316EF8, ee::s10::f_00316ef8__generated;
        oc673_fake_00317d70, OC673_PREV_00317D70, 0x0031_7d70, ee::s10::OVERRIDE_00317D70, ee::s10::f_00317d70__generated;
        oc673_fake_00317e30, OC673_PREV_00317E30, 0x0031_7e30, ee::s10::OVERRIDE_00317E30, ee::s10::f_00317e30__generated;
        oc673_fake_00317e98, OC673_PREV_00317E98, 0x0031_7e98, ee::s10::OVERRIDE_00317E98, ee::s10::f_00317e98__generated;
        oc673_fake_00326078, OC673_PREV_00326078, 0x0032_6078, ee::s10::OVERRIDE_00326078, ee::s10::f_00326078__generated;
        oc673_fake_00326a28, OC673_PREV_00326A28, 0x0032_6a28, ee::s10::OVERRIDE_00326A28, ee::s10::f_00326a28__generated;
        oc673_fake_00369ff8, OC673_PREV_00369FF8, 0x0036_9ff8, ee::s12::OVERRIDE_00369FF8, ee::s12::f_00369ff8__generated;
        oc673_fake_00375a08, OC673_PREV_00375A08, 0x0037_5a08, ee::s12::OVERRIDE_00375A08, ee::s12::f_00375a08__generated;
        oc673_fake_0038ae10, OC673_PREV_0038AE10, 0x0038_ae10, ee::s12::OVERRIDE_0038AE10, ee::s12::f_0038ae10__generated;
        oc673_fake_0038d638, OC673_PREV_0038D638, 0x0038_d638, ee::s12::OVERRIDE_0038D638, ee::s12::f_0038d638__generated;
        oc673_fake_003e18c8, OC673_PREV_003E18C8, 0x003e_18c8, ee::s14::OVERRIDE_003E18C8, ee::s14::f_003e18c8__generated;
        oc673_fake_003e2190, OC673_PREV_003E2190, 0x003e_2190, ee::s14::OVERRIDE_003E2190, ee::s14::f_003e2190__generated;
        oc673_fake_003e6448, OC673_PREV_003E6448, 0x003e_6448, ee::s14::OVERRIDE_003E6448, ee::s14::f_003e6448__generated;
        oc673_fake_0041605c, OC673_PREV_0041605C, 0x0041_605c, ee::s15::OVERRIDE_0041605C, ee::s15::f_0041605c__generated;
        oc673_fake_00416210, OC673_PREV_00416210, 0x0041_6210, ee::s15::OVERRIDE_00416210, ee::s15::f_00416210__generated;
        oc673_fake_004166f4, OC673_PREV_004166F4, 0x0041_66f4, ee::s15::OVERRIDE_004166F4, ee::s15::f_004166f4__generated;
        oc673_fake_00416ce0, OC673_PREV_00416CE0, 0x0041_6ce0, ee::s15::OVERRIDE_00416CE0, ee::s15::f_00416ce0__generated;
        oc673_fake_00417828, OC673_PREV_00417828, 0x0041_7828, ee::s15::OVERRIDE_00417828, ee::s15::f_00417828__generated;
        oc673_fake_0041acc0, OC673_PREV_0041ACC0, 0x0041_acc0, ee::s15::OVERRIDE_0041ACC0, ee::s15::f_0041acc0__generated;
    }

    /// A scrambled machine with the fakes on, the result table empty and
    /// `pc` at the function's entry; unwinds at one of `calls` fake calls.
    pub(crate) fn oc673_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32, entry: u32) -> Rng {
        oc673_install_fakes();
        let r = machine(c, seed, salt, calls);
        c.pc = entry;
        c.bus.write32(OC673_CTRL, OC673_MAGIC);
        for n in 0..32 {
            c.bus.write32(OC673_RESULTS + 4 * n, OC673_NONE);
        }
        r
    }

    /// Force the result of fake call `n`.
    pub(crate) fn oc673_result(c: &mut Cpu, n: u32, v: u32) {
        c.bus.write32(OC673_RESULTS + 4 * n, v);
    }

    /// Force the result of every fake call to the function at `addr`.
    pub(crate) fn oc673_result_at(c: &mut Cpu, addr: u32, v: u32) {
        for k in 0..16 {
            let slot = OC673_BY_ADDR + 8 * k;
            if c.bus.read32(slot) == 0 {
                c.bus.write32(slot, addr);
                c.bus.write32(slot + 4, v);
                return;
            }
        }
        panic!("oc673_result_at: table full");
    }

    /// `BELibrary_LoadCharDB` against the generated body: the reward DB load,
    /// the race DB and rider slot clearing, the grab-point record loop, the
    /// character DB load with and without a result (the free is skipped
    /// without), the equipment value table, the three profile resets and the
    /// score constants. Every callee is a fake; unwinds in each call.
    #[test]
    fn belibrary_load_char_db_matches() {
        fakes::covered(
            ee::s01::f_00149bb8__generated,
            belibrary_load_char_db,
            100,
            &|c, seed| {
                let mut r = oc673_machine(c, seed, 0x149bb8, 12, 0x0014_9bb8);
                // The 10 rows of 7 bytes of the equipment value table.
                for i in 0..0x46 {
                    c.bus.write8(0x0044_0f00 + i, r.next_u32() as u8);
                }
                // The slot records at 0x535b20 (0x1c bytes each).
                for i in 0..0x2a {
                    c.bus.write32(0x0053_5b20 + 4 * i, r.next_u32());
                }
                // Call 4 is the character DB load (0: nothing loaded).
                if seed % 3 == 0 {
                    oc673_result(c, 4, 0);
                }
            },
        );
    }

    /// `SectionMan_OnEventTrigger` against the generated body: skipped (game
    /// path, wrong mode, finale section, event 1) and taken; the AI state 4
    /// check; finale and the five table events and events outside the table;
    /// unwinds in each call. The race object's slot is a fake method.

    /// `BELibrary_ctor` against the generated body: the character DB load
    /// (its callees are fakes), the 0xa28-byte default block copy and the
    /// fourteen interface singleton creations, unwinds in each call.
    #[test]
    fn be_library_ctor_matches() {
        fakes::covered(
            ee::s01::f_00149860__generated,
            be_library_ctor,
            60,
            &|c, seed| {
                let mut r = oc673_machine(c, seed, 0x149860, 28, 0x0014_9860);
                c.g[A0][0] = sx(DATA + 0xc_0000);
                for i in 0..0x28a {
                    c.bus.write32(0x0043_fb28 + 4 * i, r.next_u32());
                }
                if seed % 3 == 0 {
                    oc673_result_at(c, 0x003e_18c8, 0);
                }
            },
        );
    }
}
