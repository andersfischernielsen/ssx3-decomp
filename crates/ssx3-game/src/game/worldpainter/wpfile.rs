//! `src/worldpainter/wpfile.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
pub(crate) use crate::game::worldpainter::worldpaintermanquery::wp_field;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002b_b370, wpfile_ctor, "WPFile_ctor"),
    (0x002b_b390, wpfile_dtor, "WPFile_dtor"),
    (0x002b_b3f0, wpfile_poll_load, "WPFile_PollLoad"),
    (0x002b_b4d8, wp_file_begin_load, "WPFile_BeginLoad"),
    (0x002b_b570, wp_file_set_data, "WPFile_SetData"),
    (0x002b_b5a8, wp_file_unload, "WPFile_Unload"),
];

/// `WPFile* WPFile_ctor(WPFile* f)` (0x2bb370): clear the two half-word fields
/// at +0/+2, set +4 to -1 and +8 to 0.
pub fn wpfile_ctor(c: &mut Cpu) {
    c.cycles += 7;
    let f = Addr(abi::arg(c, 0));
    c.g[V0][0] = u64::MAX;
    f.off(0x8).sw(c, 0);
    f.off(0x4).sw(c, c.g[V0][0]);
    c.g[V0][0] = c.g[A0][0];
    f.sh(c, 0);
    f.off(0x2).sh(c, 0);
    abi::ret(c);
}

/// `void WPFile_SetData(WPFile* f, int, void* data)` (0x2bb570): writes the
/// file's type word (2 at `+0`, 1 at `+2`), stores `a2` as its data at `+8`
/// and relocates the file (`WPFile_Relocate`).
pub fn wp_file_set_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        c.g[V0][0] = c.g[A0][0];
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x2;
        c.g[V1][0] = 0x1;
        let file = Addr(c.g[V0][0] as u32);
        let kind = c.g[A0][0];
        file.sh(c, kind);
        let one = c.g[V1][0];
        file.off(0x2).sh(c, one);
        c.g[A0][0] = c.g[V0][0];
        let data = c.g[A2][0];
        file.off(0x8).sw(c, data);
        Callee {
            addr: 0x002b_b5f0,
            f: ee::s08::f_002bb5f0,
        }
        .call(c, 0x002b_b598)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `WPFile::~WPFile(int flags)` (0x2bb390): frees the buffer at +8 (0x317e98)
/// when there is one and the short at +2 is zero, and deletes the object
/// (0x317e50) when flags bit 0 is set.
pub fn wpfile_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x8)).lw(c);
        let no_buffer = c.g[A0][0] == 0u64;
        c.g[S1][0] = c.g[A1][0];
        'b3c8: {
            'b3c4: {
                if no_buffer {
                    break 'b3c4;
                }
                c.cycles += 3;
                let short_set = Addr((c.g[S0][0] as u32).wrapping_add(0x2)).lh(c) != 0u64;
                c.g[V0][0] = c.g[S1][0] & 0x1;
                if short_set {
                    break 'b3c8;
                }
                c.cycles += 2;
                call_ptr(c, 0x0031_7e98, 0x002b_b3c4)?;
                break 'b3c4;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S1][0] & 0x1;
        }
        'b3dc: {
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                frame.restore(c, S0, 0x20);
                break 'b3dc;
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x002b_b3d8)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `WPFile::PollLoad(this)` (0x2bb3f0): while the file's state (the short at
/// `this`) is 1 and its async load is done, takes the result (a buffer and its
/// size, through `AsyncFile_GetResult`), allocates a buffer for it and copies
/// the data in (`this+8` receives the buffer); the state is then set to 2 when
/// there is no data, or to 2 after the copy.
pub fn wpfile_poll_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x1;
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[S0][0] = Addr(c.g[S1][0] as u32).lh(c);
        'exit: {
            if c.g[S0][0] != c.g[V0][0] {
                break 'exit;
            }
            c.cycles += 4;
            c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x4)).lw(c);
            call_ptr(c, 0x003d_f980, 0x002b_b41c)?;
            if c.g[V0][0] != c.g[S0][0] {
                break 'exit;
            }
            'store: {
                'result: {
                    c.cycles += 9;
                    c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x4)).lw(c);
                    c.g[A1][0] = c.g[SP][0];
                    Addr(sp(c)).sw(c, 0);
                    c.g[A2][0] = c.g[SP][0] | 0x4;
                    Addr(sp(c).wrapping_add(0x4)).sw(c, 0);
                    call_ptr(c, 0x003d_f9d8, 0x002b_b43c)?;
                    c.g[A0][0] = Addr(sp(c)).lw(c);
                    let buffer_null = c.g[A0][0] == 0u64;
                    c.g[V0][0] = Addr(sp(c).wrapping_add(0x4)).lw(c);
                    if buffer_null {
                        break 'result;
                    }
                    c.cycles += 2;
                    let size_le_0 = (c.g[V0][0] as i64) <= 0;
                    c.g[V0][0] = 0x2;
                    if size_le_0 {
                        break 'store;
                    }
                    'copy: {
                        'size: {
                            c.cycles += 4;
                            call_ptr(c, 0x003b_4818, 0x002b_b458)?;
                            let unpacked_le_0 = (c.g[V0][0] as i64) <= 0;
                            c.g[A1][0] = 0x4a_0000;
                            if unpacked_le_0 {
                                break 'size;
                            }
                            c.cycles += 11;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3868));
                            c.g[A2][0] = 0x2000_0000;
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x0031_7e30, 0x002b_b474)?;
                            c.g[A0][0] = Addr(sp(c)).lw(c);
                            c.g[A1][0] = c.g[V0][0];
                            let buffer = c.g[V0][0];
                            Addr((c.g[S1][0] as u32).wrapping_add(0x8)).sw(c, buffer);
                            call_ptr(c, 0x003b_47f8, 0x002b_b484)?;
                            break 'copy;
                        }
                        c.cycles += 10;
                        c.g[A0][0] = Addr(sp(c).wrapping_add(0x4)).lw(c);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3868));
                        c.g[A2][0] = 0x2000_0000;
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x0031_7e30, 0x002b_b4a0)?;
                        c.g[A2][0] = Addr(sp(c).wrapping_add(0x4)).lw(c);
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = Addr(sp(c)).lw(c);
                        let buffer = c.g[V0][0];
                        Addr((c.g[S1][0] as u32).wrapping_add(0x8)).sw(c, buffer);
                        call_ptr(c, 0x003e_6574, 0x002b_b4b4)?;
                    }
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x002b_b4bc);
                    c.g[A0][0] = Addr(sp(c)).lw(c);
                    call_ptr(c, 0x0025_23a8, 0x002b_b4bc)?;
                }
                c.cycles += 1;
                c.g[V0][0] = 0x2;
            }
            c.cycles += 2;
            let state = c.g[V0][0];
            Addr(c.g[S1][0] as u32).sh(c, state);
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 4;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `void cWorldPainterSection_load(cWorldPainterSection* self, const char*
/// fileName)` (0x2bb4d8, reference `cWorldPainterSection_load`): clears the
/// streamed flag at +2, builds `fileName + ".swp"` in a temporary string
/// (`cBXString` concatenation), starts an async load of it, stores the handle
/// at +4 and sets the state at +0 to 1 for a positive handle, 2 otherwise.
/// Both temporary strings are destroyed (flag 2) before the state is written.
pub fn wp_file_begin_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 17, 0x30);
        c.g[V0][0] = 0x4a_0000;
        c.g[A2][0] = c.g[A1][0];
        frame.save(c, 16, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[S0][0] = sx(frame.at(c, 0x10));
        Addr(wp_field(c.g[S1][0], 2)).sh(c, 0);
        c.g[A1][0] = sx(wp_field(c.g[V0][0], 0x3870));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0031_86d0, 0x002b_b508)?;
        c.g[A2][0] = 0x4a_0000;
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = sx(wp_field(c.g[A2][0], 0x3878));
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x0031_8630, 0x002b_b51c)?;
        c.g[A0][0] = Addr(sp(c)).lw(c);
        c.g[A1][0] = 0;
        call_ptr(c, 0x003d_f690, 0x002b_b528)?;
        let handle = c.g[V0][0];
        Addr(wp_field(c.g[S1][0], 4)).sw(c, handle);
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x002b_b538)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x002b_b544)?;
        c.g[V0][0] = Addr(wp_field(c.g[S1][0], 4)).lw(c);
        let positive = (c.g[V0][0] as i64) > 0;
        c.g[V0][0] = 1;
        if !positive {
            c.cycles += 1;
            c.g[V0][0] = 2;
        }
        c.cycles += 6;
        let state = c.g[V0][0];
        Addr(wp_field(c.g[S1][0], 0)).sh(c, state);
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    });
}

/// `void WPFile_Unload(WPFile* self)` (0x2bb5a8): frees the data block at +8
/// when it is not borrowed (the flag at +2 is zero), clears the pointer, and
/// sets the state at +0 to 0.
pub fn wp_file_unload(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(wp_field(c.g[S0][0], 8)).lw(c);
        if c.g[A0][0] == 0 {
            Addr(wp_field(c.g[S0][0], 0)).sh(c, 0);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(wp_field(c.g[S0][0], 2)).lh(c);
            if c.g[V0][0] != 0 {
                Addr(wp_field(c.g[S0][0], 8)).sw(c, 0);
            } else {
                c.cycles += 3;
                call_ptr(c, 0x0031_7e98, 0x002b_b5d8)?;
                Addr(wp_field(c.g[S0][0], 8)).sw(c, 0);
            }
            c.cycles += 1;
            Addr(wp_field(c.g[S0][0], 0)).sh(c, 0);
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;

    #[test]
    fn wpfile_ctor_matches() {
        check(ssx3_ee::s08::f_002bb370, wpfile_ctor, 50, &|c, seed| {
            let mut r = Rng(0xb370 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
        });
    }

    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;

    use crate::difftest::fill;

    pub(crate) use crate::game::worldpainter::worldpainterman::tests::oc1549_gated_fake;

    pub(crate) use crate::game::worldpainter::worldpainterman::tests::oc1550_install;
    pub(crate) use crate::game::worldpainter::worldpainterman::tests::OC1549_MAGIC;
    pub(crate) use crate::game::worldpainter::worldpainterman::tests::OC1550_CTRL;
    pub(crate) use crate::game::worldpainter::worldpainterman::tests::OC1550_MAGIC;

    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// `WPFile_SetData` against the generated body. `a2 = 0` makes the
    /// relocation take its null short path (no pointer walk), so the
    /// callee stays bounded on test data.
    #[test]
    fn wp_file_set_data_matches() {
        check(
            ee::s08::f_002bb570__generated,
            wp_file_set_data,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002b_b570, 2);
                let file = DATA + 0x3100;
                fill(c, &mut r, file, 8);
                c.g[A0][0] = file as u64;
                c.g[A2][0] = 0;
            },
        );
    }

    pub(crate) const OC1549_DT_GATE: u32 = DATA + 0x2e_0200;

    pub(crate) const OC1549_PL_GATE: u32 = DATA + 0x2e_0500;

    // WPFile_dtor's callees: the buffer free and the delete.
    oc1549_gated_fake!(
        oc1549_dt_free,
        OC1549_DT_PREV_FREE,
        OC1549_DT_GATE,
        4,
        ee::s10::f_00317e98__generated,
        0x0031_7e98
    );

    oc1549_gated_fake!(
        oc1549_dt_delete,
        OC1549_DT_PREV_DELETE,
        OC1549_DT_GATE,
        8,
        ee::s10::f_00317e50__generated,
        0x0031_7e50
    );

    pub(crate) fn oc1549_dt_hook() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc1549_dt_free,
            &OC1549_DT_PREV_FREE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc1549_dt_delete,
            &OC1549_DT_PREV_DELETE,
        );
    }

    /// `WPFile_dtor` over the buffer and flag combinations.
    #[test]
    fn wpfile_dtor_matches() {
        check(
            ee::s08::f_002bb390__generated,
            wpfile_dtor,
            300,
            &|c, seed| {
                oc1549_dt_hook();
                machine(c, seed, 0x2b_b390, 3);
                c.bus.write32(OC1549_DT_GATE, OC1549_MAGIC);
                c.bus.write32(OC1549_DT_GATE + 4, 0);
                c.bus.write32(OC1549_DT_GATE + 8, 0);
                let file = DATA + 0x1000;
                let buffer = if seed % 3 == 0 { 0 } else { DATA + 0x3000 };
                c.bus.write32(file + 8, buffer);
                c.bus.write16(file + 2, if seed % 2 == 0 { 0 } else { 5 });
                c.g[A0][0] = sx(file);
                c.g[A1][0] = seed as u64;
                c.pc = 0x002b_b390;
            },
        );
    }

    // WPFile_PollLoad's callees. The async-file queries and the allocator are
    // gated fakes; the query results come from the gate's mailbox words.
    oc1549_gated_fake!(
        oc1549_pl_isdone,
        OC1549_PL_PREV_ISDONE,
        OC1549_PL_GATE,
        4,
        ee::s14::f_003df980__generated,
        0x003d_f980
    );

    oc1549_gated_fake!(
        oc1549_pl_size,
        OC1549_PL_PREV_SIZE,
        OC1549_PL_GATE,
        8,
        ee::s13::f_003b4818__generated,
        0x003b_4818
    );

    oc1549_gated_fake!(
        oc1549_pl_unpack,
        OC1549_PL_PREV_UNPACK,
        OC1549_PL_GATE,
        4,
        ee::s13::f_003b47f8__generated,
        0x003b_47f8
    );

    oc1549_gated_fake!(
        oc1549_pl_new,
        OC1549_PL_PREV_NEW,
        OC1549_PL_GATE,
        0xc,
        ee::s10::f_00317e30__generated,
        0x0031_7e30
    );

    oc1549_gated_fake!(
        oc1549_pl_memcpy,
        OC1549_PL_PREV_MEMCPY,
        OC1549_PL_GATE,
        4,
        ee::s14::f_003e6574__generated,
        0x003e_6574
    );

    oc1549_gated_fake!(
        oc1549_pl_free,
        OC1549_PL_PREV_FREE,
        OC1549_PL_GATE,
        4,
        ee::s06::f_002523a8__generated,
        0x0025_23a8
    );

    pub(crate) static OC1549_PL_PREV_RESULT: AtomicUsize = AtomicUsize::new(0);

    /// `AsyncFile_GetResult`: with the gate on, writes the two result words
    /// (gate + 0x10 and + 0x14) through its out pointers `a1` and `a2`.
    pub(crate) fn oc1549_pl_result(c: &mut Cpu) {
        if c.bus.read32(OC1549_PL_GATE) != OC1549_MAGIC {
            return fakes::fall_back(c, &OC1549_PL_PREV_RESULT, ee::s14::f_003df9d8__generated);
        }
        let out_a = c.g[A1][0] as u32;
        let out_b = c.g[A2][0] as u32;
        let a = c.bus.read32(OC1549_PL_GATE + 0x10);
        let b = c.bus.read32(OC1549_PL_GATE + 0x14);
        c.bus.write32(out_a, a);
        c.bus.write32(out_b, b);
        fakes::fake_call(c, 0x003d_f9d8);
    }

    pub(crate) fn oc1549_pl_hook() {
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DF980),
            oc1549_pl_isdone,
            &OC1549_PL_PREV_ISDONE,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DF9D8),
            oc1549_pl_result,
            &OC1549_PL_PREV_RESULT,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B4818),
            oc1549_pl_size,
            &OC1549_PL_PREV_SIZE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B47F8),
            oc1549_pl_unpack,
            &OC1549_PL_PREV_UNPACK,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc1549_pl_new,
            &OC1549_PL_PREV_NEW,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E6574),
            oc1549_pl_memcpy,
            &OC1549_PL_PREV_MEMCPY,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_002523A8),
            oc1549_pl_free,
            &OC1549_PL_PREV_FREE,
        );
    }

    /// `WPFile_PollLoad` over the state, the query results and the size:
    /// every exit (not loading, not done, no result, empty result, the
    /// allocate-and-copy path and the skipped copy).
    #[test]
    fn wpfile_poll_load_matches() {
        check(
            ee::s08::f_002bb3f0__generated,
            wpfile_poll_load,
            300,
            &|c, seed| {
                oc1549_pl_hook();
                machine(c, seed, 0x2b_b3f0, 3);
                c.bus.write32(OC1549_PL_GATE, OC1549_MAGIC);
                c.bus
                    .write32(OC1549_PL_GATE + 4, if seed % 5 == 0 { 0 } else { 1 });
                c.bus.write32(
                    OC1549_PL_GATE + 8,
                    if (seed / 3) % 2 == 0 { 0 } else { 0x40 },
                );
                c.bus.write32(OC1549_PL_GATE + 0xc, DATA + 0x5000);
                c.bus.write32(
                    OC1549_PL_GATE + 0x10,
                    if seed % 4 == 0 { 0 } else { 0x2_0000 },
                );
                c.bus.write32(
                    OC1549_PL_GATE + 0x14,
                    [0, 0xffff_ffff, 0x30][(seed % 3) as usize],
                );
                let file = DATA + 0x1000;
                c.bus.write16(file, if seed % 7 == 0 { 0 } else { 1 });
                c.bus.write32(file + 4, DATA + 0x3000);
                c.bus.write32(file + 8, 0);
                c.g[A0][0] = sx(file);
                c.pc = 0x002b_b3f0;
            },
        );
    }

    /// `WPFile_BeginLoad` fixture: a file object at DATA+0x100, a name at
    /// DATA+0x200, the string and loader fakes gated on. The mailbox picks
    /// a positive, zero or negative handle.
    pub(crate) fn oc1550_load_setup(c: &mut Cpu, seed: u32) {
        let _r: Rng = fakes::machine(c, seed, 0x1550_01, 5);
        c.bus.write32(OC1550_CTRL, OC1550_MAGIC);
        const MODES: [u32; 4] = [0, 1, 0, 2];
        c.bus.write32(OC1550_CTRL + 4, MODES[(seed % 4) as usize]);
        c.g[A0][0] = sx(DATA + 0x100);
        c.g[A1][0] = sx(DATA + 0x200);
    }

    #[test]
    fn wp_file_begin_load_matches() {
        oc1550_install();
        check(
            ee::s08::f_002bb4d8__generated,
            wp_file_begin_load,
            300,
            &|c, seed| oc1550_load_setup(c, seed),
        );
    }

    #[test]
    fn wp_file_unload_matches() {
        oc1550_install();
        check(
            ee::s08::f_002bb5a8__generated,
            wp_file_unload,
            200,
            &|c, seed| {
                let _r: Rng = fakes::machine(c, seed, 0x1550_02, 1);
                c.bus.write32(OC1550_CTRL, OC1550_MAGIC);
                let this = DATA + 0x400;
                c.g[A0][0] = sx(this);
                c.bus
                    .write32(this + 8, if seed % 4 == 0 { 0 } else { DATA + 0x500 });
                c.bus
                    .write16(this + 2, if seed % 3 == 0 { 0 } else { (seed as u16) | 1 });
            },
        );
    }
}
