//! `src/fe/festateriderbiodetail.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Frame;
pub(crate) use crate::game::fe::feasyncfile::at;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_2;
pub(crate) use crate::game::fe::feasyncfile::lw_6 as lw;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2 as p;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0019_1228,
        cfe_state_rider_bio_fill_bio_info,
        "cFEStateRiderBio_fillBioInfo",
    ),
    (
        0x0019_12d0,
        c_fe_state_rider_bio_on_widget_action,
        "cFEStateRiderBio_OnWidgetAction",
    ),
    (
        0x0019_1360,
        cfe_state_rule_select_ctor,
        "cFEStateRuleSelect_ctor",
    ),
];

// ---------------------------------------------------------------------------
// Callees (addresses the generated body reaches through the function table).

pub(crate) mod callee {
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_ADD_CHILD;
}

/// `cFEStateRuleSelect* cFEStateRuleSelect_ctor(void* self)` (0x191360): the
/// rule-select state: `UIState_ctor`, vtable, the string-type byte from the
/// first UI layer, and the state id 0x14.
pub fn cfe_state_rule_select_ctor(c: &mut Cpu) {
    c.cycles += 22;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    if call_ptr(c, 0x0039_e2a0, 0x0019_1374).is_err() {
        return;
    }
    c.g[V1][0] = sx(0x0046_b090);
    c.sw(p(c, S0, 0x48), 0);
    c.sb(p(c, S0, 0x44), 0);
    c.sw(p(c, S0, 8), c.g[V1][0]);
    c.g[A1][0] = 0;
    c.g[V0][0] = Addr(p(c, S0, 0x10)).lw(c);
    c.g[A0][0] = Addr(p(c, V0, 0)).lw(c);
    if call_ptr(c, 0x001a_1cd0, 0x0019_1398).is_err() {
        return;
    }
    c.sb(p(c, S0, 0x15), c.g[V0][0]);
    c.g[V0][0] = 0x14;
    frame.restore_ra(c, 0);
    c.sw(p(c, S0, 0xc), c.g[V0][0]);
    c.g[V0][0] = c.g[S0][0];
    frame.restore(c, S0, 0x10);
    frame.close(c);
}

/// `void cFEStateRiderBio_OnWidgetAction(self, item, msg)` (0x1912d0): on a
/// widget action (6) hand the event to the screen's virtual handler and swap
/// the screen; on a page change (9) remember the page and rebuild the bio.
pub fn c_fe_state_rider_bio_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        let self_ = abi::arg(c, 0);
        let item = abi::arg64(c, 1);
        let _msg = abi::arg64(c, 2);
        c.g[V1][0] = item;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let no_item = c.g[V1][0] == 0;
        c.g[S0][0] = sx(self_);
        if no_item {
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 6;
            let is_six = c.g[A2][0] == c.g[V0][0];
            c.g[V0][0] = 9;
            if is_six {
                c.cycles += 11;
                c.g[V0][0] = c.lw(at(c, S0, 0x10));
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = c.lw(at(c, V1, 0x18));
                c.g[A3][0] = c.lw(r32(c, V0));
                c.g[V1][0] = c.lw(at(c, A3, 4));
                c.g[A0][0] = c.lh(at(c, V1, 0x28));
                c.g[V0][0] = c.lw(at(c, V1, 0x2c));
                c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
                let target = r32(c, V0);
                call_ptr(c, target, 0x0019_1328)?;
                let nothing = c.g[V0][0] == 0;
                c.g[A1][0] = c.g[V0][0];
                if nothing {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x10);
                } else {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw(at(c, S0, 0x10));
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x18));
                    call_ptr(c, callee::UI_OBJECT_ADD_CHILD, 0x0019_133c)?;
                    frame.restore(c, S0, 0x10);
                }
            } else {
                c.cycles += 2;
                let is_nine = c.g[A2][0] == c.g[V0][0];
                if is_nine {
                    c.g[V0][0] = c.lbu(at(c, V1, 0x319));
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    c.sw(at(c, S0, 0x4c), c.g[V0][0]);
                    call_ptr(c, 0x0019_0a08, 0x0019_1350)?;
                    c.cycles += 1;
                    frame.restore(c, S0, 0x10);
                } else {
                    c.cycles += 2;
                    frame.restore(c, S0, 0x10);
                }
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider bio: format the character's display name into the bio's text object.

/// `void cFEStateRiderBio_fillBioInfo(cFEStateRiderBio* self)` (0x191228):
/// find the scrolling-text object on the screen, format the character's
/// display name into a stack buffer and set it as the text (by hash).
pub fn cfe_state_rider_bio_fill_bio_info(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x70);
        c.cycles += 14;
        frame.save(c, S0, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x50);
        c.g[A0][0] = sx(0x0046_0000);
        frame.save_ra(c, 0x40);
        c.g[RA][0] = sx(0x0019_1248);
        c.g[A0][0] = sx(0x0045_e438);
        call_ptr(c, 0x0031_7670, 0x0019_1248)?;
        c.g[A0][0] = lw(c, at_2(c.g[S0][0], 0x40));
        c.g[RA][0] = sx(0x0019_1254);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0019_1254)?;
        c.g[S1][0] = c.g[V0][0];
        if c.g[S1][0] == 0 {
            c.lq(16, frame.at(c, 0x60));
        } else {
            c.cycles += 23;
            c.g[RA][0] = sx(0x0019_1268);
            call_ptr(c, 0x0014_dc80, 0x0019_1268)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0019_1274);
            c.g[A1][0] = 2;
            call_ptr(c, 0x0014_dd58, 0x0019_1274)?;
            c.g[A1][0] = lw(c, at_2(c.g[S0][0], 0x48));
            c.g[A0][0] = c.g[V0][0];
            c.g[S0][0] = sx(0x0046_0000);
            c.g[A2][0] = 0;
            c.g[RA][0] = sx(0x0019_128c);
            c.g[S0][0] = sx(0x0045_f7d0);
            call_ptr(c, 0x0014_eec8, 0x0019_128c)?;
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = sx(frame.at(c, 0));
            c.g[RA][0] = sx(0x0019_129c);
            c.g[A2][0] = c.g[V0][0];
            call_ptr(c, 0x0041_7828, 0x0019_129c)?;
            c.g[RA][0] = sx(0x0019_12a4);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x003a_18c0, 0x0019_12a4)?;
            c.g[RA][0] = sx(0x0019_12ac);
            c.g[A0][0] = sx(frame.at(c, 0));
            call_ptr(c, 0x0031_7670, 0x0019_12ac)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x0019_12b8);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_1f18, 0x0019_12b8)?;
            c.lq(16, frame.at(c, 0x60));
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::w8;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::env;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_8 as run;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_6 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::w32;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::EMPTY;
    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_2;
    pub(crate) use crate::game::fe::festatecredits::tests::RET_ZERO;
    use ssx3_ee as ee;

    #[test]
    fn cfe_state_rule_select_ctor_matches() {
        check(
            ssx3_ee::s02::f_00191360__generated,
            cfe_state_rule_select_ctor,
            100,
            &|c, seed| {
                let _r = machine(c, seed, 0x191360, 2);
                let this = DATA + 0x400;
                c.bus.write32(this + 0x10, DATA + 0x500);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = seed as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_rider_bio_on_widget_action_matches() {
        check(
            ee::s02::f_001912d0__generated,
            c_fe_state_rider_bio_on_widget_action,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x1912, 0);
                env(c);
                let this = DATA + 0x400;
                let item = DATA + 0x800;
                let engine = DATA + 0x1000;
                let obj = DATA + 0x1100;
                let vt = DATA + 0x1200;
                slot(c, vt, 0x28, if seed % 2 == 0 { RET_ZERO } else { EMPTY });
                w32(c, this + 0x10, engine);
                w32(c, engine, obj);
                w32(c, obj + 4, vt);
                w8(c, item + 0x319, seed % 4);
                w32(c, item + 0x18, seed % 8);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 5 == 0 { 0 } else { sx(item) };
                c.g[A2][0] = [6u64, 9, 0, 3][(seed % 4) as usize];
            },
        );
    }

    #[test]
    fn cfe_state_rider_bio_fill_bio_info_matches() {
        check(
            ee::s02::f_00191228__generated,
            cfe_state_rider_bio_fill_bio_info,
            SEEDS_2,
            &|c, seed| {
                let _ = base(c, seed, 0x1912_28, 8);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                // The screen and (for the found branch) the text object.
                let screen = obj + 0x800;
                w(c, obj + 0x40, if seed % 3 == 0 { 0 } else { screen });
                w(c, obj + 0x48, seed % 8);
                // UIScreen_FindObject: count at_2 +0x3c, array at_2 +0x38,
                // entries whose +0x38 is the hash.
                w(c, screen + 0x3c, if seed % 2 == 0 { 0 } else { 1 });
                w(c, screen + 0x38, screen + 0x900);
                let entry = screen + 0xa00;
                w(c, screen + 0x900, entry);
                // The hash of the (zeroed) bio-title string.
                let hash = run(c, 0x0031_7670, ee::s10::f_00317670, &[0x0045_e438u64]);
                w(c, entry + 0x38, hash as u32);
                // The text-scroll object: no lines, no popup chain.
                w(c, entry + 0xbc, 0);
                wh(c, entry + 0xc8, 0);
            },
        );
    }
}
