//! `src/wscript/wscriptmission.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::luno::lunovm::oc640_field;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0018_3de0,
    cfe_state_uber_trick_refresh_trick_list,
    "cFEStateUberTrick_RefreshTrickList",
)];

/// `cFEStateUberTrick::updateTrickMenuEntry(self)` (0x184520): refreshes the
/// uber-trick menu's cost text (the trick's cost from the table the category
/// lookup returns, as a dollar string) and its money text (the user's cash
/// for the character id). The trick and category are the two menus' rider
/// slot bytes.
///
/// Not in `OVERRIDES`: its callees `getDollarString` (0x198af0) and
/// `cUIText_setAsciiString` (0x3a0c50) have no override slot, so the
/// differential test cannot fake them, and running them on fixture data is
/// unbounded.
pub fn fe_uber_trick_update_cost(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S0, 0x60);
        frame.save(c, S4, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A0][0];
        c.g[V1][0] = oc640_field(c.g[S1][0], 0x50).lw(c);
        c.g[V0][0] = oc640_field(c.g[S1][0], 0x4c).lw(c);
        c.g[A1][0] = oc640_field(c.g[V1][0], 0xa0).lw(c);
        c.g[A0][0] = oc640_field(c.g[V0][0], 0xa0).lw(c);
        c.g[S4][0] = oc640_field(c.g[A1][0], 0x18).lb(c);
        c.g[S0][0] = oc640_field(c.g[A0][0], 0x18).lb(c);
        call_ptr(c, 0x0014_dc80, 0x0018_455c)?; // cBE_getBE()
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x6;
        call_ptr(c, 0x0014_dd58, 0x0018_4568)?; // cBE_getInterface_Fv(be, 6)
        c.g[A1][0] = oc640_field(c.g[S1][0], 0x44).lb(c);
        c.g[A2][0] = c.g[S0][0];
        c.g[A3][0] = c.g[SP][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_ff90, 0x0018_457c)?; // the cost table for the category
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x24));
        c.g[S2][0] = c.g[V0][0];
        call_ptr(c, 0x0039_7840, 0x0018_4588)?; // cList_first(&mScreens)
        c.g[A0][0] = 0x0046_0000;
        c.g[S3][0] = c.g[V0][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2648));
        call_ptr(c, 0x0031_7670, 0x0018_4598)?; // GetHashValue32(str_cost_num)
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, 0x0039_d860, 0x0018_45a4)?; // cUIScreen_getObjectByHashName
        c.g[S0][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[S4][0] as u32) << 3);
        if c.g[S0][0] != 0 {
            c.cycles += 6;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
            c.g[A0][0] = oc640_field(c.g[V0][0], 0x6).lhu(c);
            call_ptr(c, 0x0019_8af0, 0x0018_45bc)?; // getDollarString(cost)
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_0c50, 0x0018_45c8)?; // cUIText_setAsciiString
        }
        c.cycles += 9;
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2638));
        call_ptr(c, 0x0031_7670, 0x0018_45d4)?; // GetHashValue32(str_have_num)
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0018_45e0)?; // cUIScreen_getObjectByHashName
        c.g[S2][0] = c.g[V0][0];
        if c.g[S2][0] != 0 {
            c.cycles += 24;
            call_ptr(c, 0x0014_dc80, 0x0018_45f4)?; // cBE_getBE()
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0014_dd58, 0x0018_4600)?; // cBE_getInterface_Fv(be, 1)
            c.g[A1][0] = oc640_field(c.g[S1][0], 0x44).lb(c);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014_7488, 0x0018_460c)?; // getPlayerCharID(iface, user)
                                                    // The character id is a signed byte.
            c.g[S0][0] = sx((((c.g[V0][0] as u32) << 24) as i32 >> 24) as u32);
            call_ptr(c, 0x0014_dc80, 0x0018_4618)?; // cBE_getBE()
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0xb;
            call_ptr(c, 0x0014_dd58, 0x0018_4624)?; // cBE_getInterface_Fv(be, 11)
            c.g[A1][0] = oc640_field(c.g[S1][0], 0x44).lb(c);
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, 0x0015_0928, 0x0018_4634)?; // cBEEconInterface_getMoney
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0019_8af0, 0x0018_463c)?; // getDollarString(money)
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_0c50, 0x0018_4648)?; // cUIText_setAsciiString
        }
        frame.restore(c, S0, 0x60);
        c.cycles += 7;
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::fake;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fill;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use ssx3_ee_rt::funcs::register_functions;
    use ssx3_ee_rt::funcs::FnEntry;

    /// Set in the child process that runs the test with the stub table.
    pub(crate) const OC679_CHILD_ENV: &str = "SSX3_OC679_CHILD";

    pub(crate) const OC679_THIS: u32 = DATA + 0x8_6000;

    pub(crate) const OC679_MENU: u32 = DATA + 0x8_6400;

    pub(crate) const OC679_RIDER: u32 = DATA + 0x8_6800;

    pub(crate) const OC679_VT: u32 = DATA + 0x8_4000;

    pub(crate) const OC679_TABLE: u32 = DATA + 0x8_5000;

    pub(crate) const OC679_OBJS: u32 = DATA + 0x8_0000;

    /// Stub controls (`DATA + 0x200000` and up): counters, masks, results.
    pub(crate) const OC679_CTRL: u32 = DATA + 0x20_0000;

    /// The jump table the slot switch reads (`0x460000 - 0x2670`).
    pub(crate) const OC679_JUMP_TABLE: u32 = 0x0045_d990;

    pub(crate) fn oc679_tick(c: &mut Cpu, at: u32) -> u32 {
        let n = c.bus.read32(OC679_CTRL + at);
        c.bus.write32(OC679_CTRL + at, n + 1);
        n
    }

    /// A callee whose result is the fake's pointer.
    macro_rules! oc679_plain {
        ($name:ident, $addr:expr) => {
            fn $name(c: &mut Cpu) {
                let ret = c.g[RA][0] as u32;
                fake(c, $addr, ret);
            }
        };
    }

    oc679_plain!(oc679_stub_gget_grab_name, 0x0011_6938);

    oc679_plain!(oc679_stub_get_be, 0x0014_dc80);

    oc679_plain!(oc679_stub_get_interface, 0x0014_dd58);

    oc679_plain!(oc679_stub_hash, 0x0031_7670);

    oc679_plain!(oc679_stub_list_first, 0x0039_7840);

    oc679_plain!(oc679_stub_icon_lookup, 0x0039_8380);

    oc679_plain!(oc679_stub_abs_position, 0x003a_0290);

    oc679_plain!(oc679_stub_set_ascii, 0x003a_0c50);

    oc679_plain!(oc679_stub_text_extent, 0x003a_1030);

    oc679_plain!(oc679_stub_sprintf, 0x0041_7828);

    /// `chooseUberVar`: the chosen trick index (0..9).
    pub(crate) fn oc679_stub_choose(c: &mut Cpu) {
        let ret = c.g[RA][0] as u32;
        fake(c, 0x0014_fea8, ret);
        if c.pc == ret {
            c.g[V0][0] = (c.bus.read32(OC679_CTRL + 0x20) % 10) as u64;
        }
    }

    /// The trick table of a slot: stores the count through `a3` and returns
    /// the table.
    pub(crate) fn oc679_stub_table(c: &mut Cpu) {
        let ret = c.g[RA][0] as u32;
        let count_at = c.g[A3][0] as u32;
        fake(c, 0x0014_ff90, ret);
        if c.pc == ret {
            let n = c.bus.read32(OC679_CTRL + 0x1c);
            c.bus.write32(count_at, n);
            c.g[V0][0] = OC679_TABLE as u64;
        }
    }

    /// A per-call yes/no answer: bit `n` of the mask at `mask_at`.
    pub(crate) fn oc679_answer(c: &mut Cpu, target: u32, counter: u32, mask_at: u32) {
        let ret = c.g[RA][0] as u32;
        let n = oc679_tick(c, counter);
        fake(c, target, ret);
        if c.pc == ret {
            let mask = c.bus.read32(OC679_CTRL + mask_at);
            c.g[V0][0] = ((mask >> (n % 32)) & 1) as u64;
        }
    }

    pub(crate) fn oc679_stub_available(c: &mut Cpu) {
        oc679_answer(c, 0x0015_0048, 4, 0x14)
    }

    pub(crate) fn oc679_stub_owned(c: &mut Cpu) {
        oc679_answer(c, 0x0014_ffb8, 8, 0x18)
    }

    /// `cUIScreen_getObjectByHashName`: the first 18 lookups (the label and
    /// mark widgets) miss where the mask has a zero bit; the later ones
    /// always find a widget.
    pub(crate) fn oc679_stub_get_object(c: &mut Cpu) {
        let ret = c.g[RA][0] as u32;
        let k = oc679_tick(c, 0);
        fake(c, 0x0039_d860, ret);
        if c.pc == ret {
            let mask = c.bus.read32(OC679_CTRL + 0x10);
            c.g[V0][0] = if k < 18 && (mask >> k) & 1 == 0 {
                0
            } else {
                (OC679_OBJS + 0x200 * (k % 24)) as u64
            };
        }
    }

    macro_rules! oc679_entry {
        ($addr:expr, $f:ident) => {
            FnEntry {
                start: $addr,
                end: $addr + 4,
                f: $f,
            }
        };
    }

    /// The callees, sorted by address.
    pub(crate) static OC679_STUBS: [FnEntry; 15] = [
        oc679_entry!(0x0011_6938, oc679_stub_gget_grab_name),
        oc679_entry!(0x0014_dc80, oc679_stub_get_be),
        oc679_entry!(0x0014_dd58, oc679_stub_get_interface),
        oc679_entry!(0x0014_fea8, oc679_stub_choose),
        oc679_entry!(0x0014_ff90, oc679_stub_table),
        oc679_entry!(0x0014_ffb8, oc679_stub_owned),
        oc679_entry!(0x0015_0048, oc679_stub_available),
        oc679_entry!(0x0031_7670, oc679_stub_hash),
        oc679_entry!(0x0039_7840, oc679_stub_list_first),
        oc679_entry!(0x0039_8380, oc679_stub_icon_lookup),
        oc679_entry!(0x0039_d860, oc679_stub_get_object),
        oc679_entry!(0x003a_0290, oc679_stub_abs_position),
        oc679_entry!(0x003a_0c50, oc679_stub_set_ascii),
        oc679_entry!(0x003a_1030, oc679_stub_text_extent),
        oc679_entry!(0x0041_7828, oc679_stub_sprintf),
    ];

    /// The screen object with a rider (or none), a slot, the widgets the
    /// lookups find (all with one vtable of fake methods), and the stub
    /// answers; the branch choices come from the seed.
    pub(crate) fn oc679_setup(c: &mut Cpu, seed: u32) {
        // Only in the child process: the stubs replace the function table
        // (`fresh` registered the generated one just before this).
        if std::env::var_os(OC679_CHILD_ENV).is_some() {
            register_functions(&OC679_STUBS);
        }
        let mut r: Rng = machine(c, seed, 0x18_3de0, 100);
        c.pc = 0x0018_3de0;
        c.g[A0][0] = OC679_THIS as u64;
        // self: user number (+0x44), main menu (+0x4c), list head (+0x24),
        // owner (+0x10), icons (+0x54/+0x58/+0x5c).
        fill(c, &mut r, OC679_THIS, 0x40);
        c.bus.write8(OC679_THIS + 0x44, (seed % 4) as u8);
        c.bus.write32(OC679_THIS + 0x4c, OC679_MENU);
        // The rider (every fifth seed none) and its slot byte.
        let has_rider = seed % 5 != 0;
        c.bus
            .write32(OC679_MENU + 0xa0, if has_rider { OC679_RIDER } else { 0 });
        let slots: [u8; 13] = [0xff, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12];
        c.bus
            .write8(OC679_RIDER + 0x18, slots[(seed % 13) as usize]);
        // The slot switch: every arm, and a target outside the function.
        let arms = [
            0x0018_41b0u32,
            0x0018_41b8,
            0x0018_41c0,
            0x0018_41c8,
            0x0018_41d4,
            0x0018_41d8,
            0x0018_41dc,
        ];
        for slot in 0..10u32 {
            let target = if slot == 7 {
                0x00ba_d000
            } else {
                arms[(slot % 7) as usize]
            };
            c.bus.write32(OC679_JUMP_TABLE + 4 * slot, target);
        }
        // The widgets: random fields, a vtable of fake methods.
        for i in 0..24u32 {
            let obj = OC679_OBJS + 0x200 * i;
            fill(c, &mut r, obj, 0x80);
            c.bus.write32(obj + 8, OC679_VT);
        }
        for (i, off) in [0x40u32, 0x48, 0xa0, 0xa8].into_iter().enumerate() {
            c.bus.write16(OC679_VT + off, 0);
            c.bus.write32(OC679_VT + off + 4, FAKE + 0x10 * i as u32);
        }
        // Trick table: eight bytes per entry.
        fill(c, &mut r, OC679_TABLE, 0x40);
        // Stub answers.
        let pick = |r: &mut Rng, s: u32| match s % 3 {
            0 => u32::MAX,
            1 => 0,
            _ => r.next_u32(),
        };
        c.bus.write32(OC679_CTRL + 0x10, pick(&mut r, seed / 2));
        c.bus.write32(OC679_CTRL + 0x14, pick(&mut r, seed / 3));
        c.bus.write32(OC679_CTRL + 0x18, pick(&mut r, seed / 5));
        c.bus.write32(OC679_CTRL + 0x1c, r.next_u32() % 10);
        c.bus.write32(OC679_CTRL + 0x20, r.next_u32());
    }

    pub(crate) fn oc679_run_checked() {
        covered(
            ee::s02::f_00183de0__generated,
            cfe_state_uber_trick_refresh_trick_list,
            120,
            &oc679_setup,
        );
    }

    /// `cFEStateUberTrick_RefreshTrickList` against the generated body:
    /// with and without a rider, every slot (jump-table arm, a slot outside
    /// the table, a target outside the function), widgets missing or found,
    /// trick counts 0-9 with random availability and ownership, and unwinds
    /// at random call indices. The callees are fakes in a private function
    /// table (the test re-runs itself in a child process, as
    /// `game_module_load_matches` does).
    #[test]
    fn cfe_state_uber_trick_refresh_trick_list_matches() {
        if std::env::var_os(OC679_CHILD_ENV).is_some() {
            return oc679_run_checked();
        }
        let name = format!(
            "{}::cfe_state_uber_trick_refresh_trick_list_matches",
            module_path!().split_once("::").map_or("", |p| p.1)
        );
        let out = std::process::Command::new(std::env::current_exe().unwrap())
            .args([name.as_str(), "--exact", "--test-threads=1", "--nocapture"])
            .env(OC679_CHILD_ENV, "1")
            .output()
            .unwrap();
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(
            out.status.success() && text.contains("1 passed"),
            "child run of {name}:\n{text}"
        );
    }
}

/// `void cFEStateUberTrick_RefreshTrickList(self)` (0x00183de0): refreshes the
/// trick menu of the uber-trick screen. Clears the nine `Trick%02d` label and
/// `ck%02d` mark widgets, then fills one label per unlocked trick of the
/// rider's slot (name, mark = dollar sign / checkmark / checkbox), puts the
/// shoulder-button icons beside the instruction texts (jump table on the
/// slot) and lays the instruction widgets out left to right.
///
/// The normal entry, converted from the generated body (every register, cycle
/// and call kept). Any other `pc` (a resume after an unwound callee or
/// event) runs the generated resume body; at install switch the guard to
/// `f_00183de0__generated`.
pub fn cfe_state_uber_trick_refresh_trick_list(c: &mut Cpu) {
    if c.pc != 0x00183de0 {
        return ee::s02::f_00183de0__generated(c);
    }
    guest(c, |c| {
        c.cycles += 42;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x220));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x1e0));
        c.g[S2][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x1f0));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x1d0));
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x24));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x1c0));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x1b0));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x1a0));
        c.g[S5][0] = 0u64;
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x190));
        c.g[S6][0] = 0xffffffffffffffff;
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x180));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x200));
        c.g[FP][0] = 0x4a0000;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x170), c.g[RA][0]);
        c.g[RA][0] = sx(0x00183e28);
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x210));
        call_ptr(c, 0x00397840, 0x00183e28)?;
        c.g[S7][0] = c.g[V0][0];
        c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x130));
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x158), c.g[V0][0]);
        c.g[S1][0] = c.g[SP][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x14c), c.g[V1][0]);
        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x154), c.g[A0][0]);
        c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x15c), c.g[A1][0]);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x160), c.g[A2][0]);
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x120));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x164), c.g[V0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x168), c.g[V1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x144), c.g[A0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x148), c.g[A1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x150), c.g[A2][0]);
        c.g[S4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
        c.g[S3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14c));
        'l_183e88: loop {
            'b_183ef8: {
                'b_183ef4: {
                    c.cycles += 13;
                    c.g[V0][0] = 0x460000;
                    c.g[A2][0] = c.g[S5][0];
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x26c0));
                    c.g[RA][0] = sx(0x00183e9c);
                    c.g[A0][0] = c.g[S4][0];
                    call_ptr(c, 0x00417828, 0x00183e9c)?;
                    c.g[RA][0] = sx(0x00183ea4);
                    c.g[A0][0] = c.g[S4][0];
                    call_ptr(c, 0x00317670, 0x00183ea4)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00183eb0);
                    c.g[A0][0] = c.g[S7][0];
                    call_ptr(c, 0x0039d860, 0x00183eb0)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;
                    c.g[A1][0] = 0x1;
                    if t {
                        break 'b_183ef4;
                    } else {
                        c.cycles += 14;
                        c.sw(c.g[S1][0] as u32, c.g[S0][0]);
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x00183ed4);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x00183ed4)?;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = 0u64;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x00183eec);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x00183eec)?;
                        let t = true;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[S6][0]);
                        if t {
                            break 'b_183ef8;
                        } else {
                            break 'b_183ef4;
                        }
                    }
                }
                c.cycles += 1;
                c.sw(c.g[S1][0] as u32, 0u64);
                break 'b_183ef8;
            }
            'b_183f4c: {
                'b_183f48: {
                    c.cycles += 12;
                    c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x1418));
                    c.g[A2][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x00183f08);
                    c.g[A0][0] = c.g[S4][0];
                    call_ptr(c, 0x00417828, 0x00183f08)?;
                    c.g[RA][0] = sx(0x00183f10);
                    c.g[A0][0] = c.g[S4][0];
                    call_ptr(c, 0x00317670, 0x00183f10)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00183f1c);
                    c.g[A0][0] = c.g[S7][0];
                    call_ptr(c, 0x0039d860, 0x00183f1c)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        break 'b_183f48;
                    } else {
                        c.cycles += 8;
                        c.sw(c.g[S3][0] as u32, c.g[S0][0]);
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x00183f40);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x00183f40)?;
                        let t = true;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[S6][0]);
                        if t {
                            break 'b_183f4c;
                        } else {
                            break 'b_183f48;
                        }
                    }
                }
                c.cycles += 1;
                c.sw(c.g[S3][0] as u32, 0u64);
                break 'b_183f4c;
            }
            c.cycles += 5;
            c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = ((c.g[S5][0] as i64) < 9i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00183e88);
                }
                continue 'l_183e88;
            } else {
                'b_183f74: {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa0));
                    let t = c.g[V0][0] == 0u64;
                    c.g[FP][0] = 0x1;
                    if t {
                        break 'b_183f74;
                    } else {
                        c.cycles += 1;
                        c.g[FP][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x18));
                        break 'b_183f74;
                    }
                }
                'b_1840e4: {
                    c.cycles += 24;
                    c.g[RA][0] = sx(0x00183f7c);
                    c.g[S1][0] = 0u64;
                    call_ptr(c, 0x0014dc80, 0x00183f7c)?;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x138), 0u64);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00183f8c);
                    c.g[A1][0] = 0x6;
                    call_ptr(c, 0x0014dd58, 0x00183f8c)?;
                    c.g[S6][0] = c.g[V0][0];
                    c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x154));
                    c.g[V1][0] = (c.g[FP][0] < 0xau64) as u64;
                    c.g[A0][0] = c.g[S6][0];
                    c.g[A2][0] = c.g[FP][0];
                    c.g[RA][0] = sx(0x00183fac);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x16c), c.g[V1][0]);
                    call_ptr(c, 0x0014ff90, 0x00183fac)?;
                    c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S6][0];
                    c.g[A2][0] = c.g[FP][0];
                    c.g[RA][0] = sx(0x00183fc4);
                    c.g[A3][0] = 0x1;
                    call_ptr(c, 0x0014fea8, 0x00183fc4)?;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x134), c.g[V0][0]);
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x130));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    if t {
                        break 'b_1840e4;
                    } else {
                        c.cycles += 1;
                        c.g[S4][0] = c.g[SP][0];
                        'l_183fd8: loop {
                            'b_1840d0: {
                                c.cycles += 7;
                                c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                                c.g[A0][0] = c.g[S6][0];
                                c.g[A2][0] = c.g[FP][0];
                                c.g[RA][0] = sx(0x00183fec);
                                c.g[A3][0] = c.g[S1][0];
                                call_ptr(c, 0x00150048, 0x00183fec)?;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x138));
                                if t {
                                    break 'b_1840d0;
                                } else {
                                    'b_184048: {
                                        c.cycles += 3;
                                        c.g[S0][0] = c.lw(c.g[S4][0] as u32);
                                        let t = c.g[S0][0] == 0u64;
                                        c.g[S3][0] = sx((c.g[A0][0] as u32) << 2);
                                        if t {
                                            break 'b_184048;
                                        } else {
                                            c.cycles += 18;
                                            c.g[RA][0] = sx(0x00184008);
                                            c.g[A0][0] = c.lhu(c.g[S5][0] as u32);
                                            call_ptr(c, 0x00116938, 0x00184008)?;
                                            c.g[A1][0] = c.g[V0][0];
                                            c.g[RA][0] = sx(0x00184014);
                                            c.g[A0][0] = c.g[S0][0];
                                            call_ptr(c, 0x003a0c50, 0x00184014)?;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            c.g[A1][0] = 0x1;
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x0018402c);
                                            c.g[A0][0] =
                                                sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x0018402c)?;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            c.g[A1][0] = 0u64;
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x00184044);
                                            c.g[A0][0] =
                                                sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x00184044)?;
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0x18),
                                                c.g[S1][0],
                                            );
                                            break 'b_184048;
                                        }
                                    }
                                    'b_1840c0: {
                                        c.cycles += 5;
                                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14c));
                                        c.g[V0][0] =
                                            sx((c.g[A1][0] as u32).wrapping_add(c.g[S3][0] as u32));
                                        c.g[S0][0] = c.lw(c.g[V0][0] as u32);
                                        let t = c.g[S0][0] == 0u64;
                                        c.g[A1][0] = 0x1;
                                        if t {
                                            break 'b_1840c0;
                                        } else {
                                            'b_1840bc: {
                                                'b_184098: {
                                                    c.cycles += 12;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                                    let t = c.g[V1][0] as u32;
                                                    c.g[RA][0] = sx(0x00184070);
                                                    c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    call_ptr(c, t, 0x00184070)?;
                                                    c.g[A1][0] = c
                                                        .lb((c.g[S2][0] as u32).wrapping_add(0x44));
                                                    c.g[A0][0] = c.g[S6][0];
                                                    c.g[A2][0] = c.g[FP][0];
                                                    c.g[RA][0] = sx(0x00184084);
                                                    c.g[A3][0] = c.g[S1][0];
                                                    call_ptr(c, 0x0014ffb8, 0x00184084)?;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[A2][0] = 0xffffffffffffffff;
                                                    if t {
                                                        break 'b_184098;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x5c));
                                                        let t = true;
                                                        c.sw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x78),
                                                            c.g[A2][0],
                                                        );
                                                        if t {
                                                            break 'b_1840bc;
                                                        } else {
                                                            break 'b_184098;
                                                        }
                                                    }
                                                }
                                                'b_1840b4: {
                                                    c.cycles += 3;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x134)
                                                        );
                                                    if c.g[V0][0] != c.g[S1][0] {
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x58));
                                                        break 'b_1840b4;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x54));
                                                        c.g[V1][0] = 0xffffffffffffffff;
                                                        let t = true;
                                                        c.sw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x78),
                                                            c.g[V1][0],
                                                        );
                                                        if t {
                                                            break 'b_1840bc;
                                                        } else {
                                                            break 'b_1840b4;
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                c.g[A0][0] = 0xffffffffffffffff;
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0x78),
                                                    c.g[A0][0],
                                                );
                                                break 'b_1840bc;
                                            }
                                            c.cycles += 1;
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0x7c),
                                                c.g[V0][0],
                                            );
                                            break 'b_1840c0;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x138));
                                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4));
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x138), c.g[A1][0]);
                                    break 'b_1840d0;
                                }
                            }
                            c.cycles += 5;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x130));
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x8));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00183fd8);
                                }
                                continue 'l_183fd8;
                            } else {
                                break 'b_1840e4;
                            }
                        }
                    }
                }
                'b_1841dc: {
                    c.cycles += 44;
                    c.g[A0][0] = 0x4a0000;
                    c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1420));
                    c.g[RA][0] = sx(0x001840f8);
                    c.g[S1][0] = 0u64;
                    call_ptr(c, 0x00317670, 0x001840f8)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                    c.g[RA][0] = sx(0x00184104);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x00398380, 0x00184104)?;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x13c), 0u64);
                    c.g[A0][0] = 0x4a0000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1428));
                    c.g[S6][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0018411c);
                    c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                    call_ptr(c, 0x00317670, 0x0018411c)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                    c.g[RA][0] = sx(0x00184128);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x00398380, 0x00184128)?;
                    c.g[A0][0] = 0x4a0000;
                    c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1430));
                    c.g[RA][0] = sx(0x0018413c);
                    c.g[S5][0] = c.g[V0][0];
                    call_ptr(c, 0x00317670, 0x0018413c)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                    c.g[RA][0] = sx(0x00184148);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x00398380, 0x00184148)?;
                    c.g[A0][0] = 0x4a0000;
                    c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1438));
                    c.g[RA][0] = sx(0x0018415c);
                    c.g[S4][0] = c.g[V0][0];
                    call_ptr(c, 0x00317670, 0x0018415c)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                    c.g[RA][0] = sx(0x00184168);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x00398380, 0x00184168)?;
                    c.g[A0][0] = 0x460000;
                    c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x26b0));
                    c.g[RA][0] = sx(0x0018417c);
                    c.g[S3][0] = c.g[V0][0];
                    call_ptr(c, 0x00317670, 0x0018417c)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                    c.g[RA][0] = sx(0x00184188);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x00398380, 0x00184188)?;
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x16c));
                    let t = c.g[A2][0] == 0u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x140), c.g[V0][0]);
                    if t {
                        break 'b_1841dc;
                    } else {
                        'b_1841d8: {
                            'b_1841d4: {
                                'b_1841c8: {
                                    'b_1841c0: {
                                        'b_1841b8: {
                                            c.cycles += 7;
                                            c.g[V0][0] = 0x460000;
                                            c.g[V1][0] = sx((c.g[FP][0] as u32) << 2);
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_sub(0x2670));
                                            c.g[V1][0] =
                                                sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                            let t = c.g[A0][0] as u32;
                                            match t {
                                                0x001841b0 => {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[S1][0] = c.g[S5][0];
                                                    if t {
                                                        break 'b_1841dc;
                                                    } else {
                                                        break 'b_1841b8;
                                                    }
                                                }
                                                0x001841b8 => {
                                                    break 'b_1841b8;
                                                }
                                                0x001841c0 => {
                                                    break 'b_1841c0;
                                                }
                                                0x001841c8 => {
                                                    break 'b_1841c8;
                                                }
                                                0x001841d4 => {
                                                    break 'b_1841d4;
                                                }
                                                0x001841d8 => {
                                                    break 'b_1841d8;
                                                }
                                                0x001841dc => {
                                                    break 'b_1841dc;
                                                }
                                                _ => {
                                                    c.jump_out(t, 0x00183de0, 0x00184520);
                                                    return Ok(());
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[S1][0] = c.g[S3][0];
                                        if t {
                                            break 'b_1841dc;
                                        } else {
                                            break 'b_1841c0;
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[S1][0] = c.g[S6][0];
                                    if t {
                                        break 'b_1841dc;
                                    } else {
                                        break 'b_1841c8;
                                    }
                                }
                                c.cycles += 3;
                                c.g[S1][0] = c.g[S6][0];
                                let t = true;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x13c), c.g[S5][0]);
                                if t {
                                    break 'b_1841dc;
                                } else {
                                    break 'b_1841d4;
                                }
                            }
                            c.cycles += 1;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x13c), c.g[S3][0]);
                            break 'b_1841d8;
                        }
                        c.cycles += 1;
                        c.g[S1][0] = c.g[S4][0];
                        break 'b_1841dc;
                    }
                }
                'b_184420: {
                    'b_184400: {
                        c.cycles += 130;
                        c.g[A0][0] = 0x460000;
                        c.g[S0][0] = 0xffffffffffffffff;
                        c.g[AT][0] = 0x41200000;
                        c.mtc1(1, 20);
                        c.g[RA][0] = sx(0x001841f4);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x26a0));
                        call_ptr(c, 0x00317670, 0x001841f4)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x00184200);
                        c.g[A0][0] = c.g[S7][0];
                        call_ptr(c, 0x0039d860, 0x00184200)?;
                        c.g[S3][0] = c.g[V0][0];
                        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x5a00));
                        c.g[A1][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x00184214);
                        c.g[A0][0] = c.g[S3][0];
                        Callee {
                            addr: 0x00183d08,
                            f: ee::s02::cFEStateUberTrick_SetButtonIcon,
                        }
                        .call(c, 0x00184214)?;
                        c.sw((c.g[S3][0] as u32).wrapping_add(0x7c), c.g[S1][0]);
                        c.g[A0][0] = 0x460000;
                        c.sw((c.g[S3][0] as u32).wrapping_add(0x78), c.g[S0][0]);
                        c.g[RA][0] = sx(0x00184228);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2690));
                        call_ptr(c, 0x00317670, 0x00184228)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x00184234);
                        c.g[A0][0] = c.g[S7][0];
                        call_ptr(c, 0x0039d860, 0x00184234)?;
                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x13c));
                        c.g[S1][0] = c.g[V0][0];
                        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x59fc));
                        c.g[RA][0] = sx(0x00184248);
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x00183d08,
                            f: ee::s02::cFEStateUberTrick_SetButtonIcon,
                        }
                        .call(c, 0x00184248)?;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x13c));
                        c.g[A0][0] = 0x460000;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x78), c.g[S0][0]);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2680));
                        c.g[RA][0] = sx(0x00184260);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x7c), c.g[V0][0]);
                        call_ptr(c, 0x00317670, 0x00184260)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x0018426c);
                        c.g[A0][0] = c.g[S7][0];
                        call_ptr(c, 0x0039d860, 0x0018426c)?;
                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x140));
                        c.g[S2][0] = c.g[V0][0];
                        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x59f8));
                        c.g[RA][0] = sx(0x00184280);
                        c.g[A0][0] = c.g[S2][0];
                        Callee {
                            addr: 0x00183d08,
                            f: ee::s02::cFEStateUberTrick_SetButtonIcon,
                        }
                        .call(c, 0x00184280)?;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x78), c.g[S0][0]);
                        c.g[A0][0] = 0x4a0000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1440));
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x140));
                        c.g[RA][0] = sx(0x00184298);
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x7c), c.g[V1][0]);
                        call_ptr(c, 0x00317670, 0x00184298)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001842a4);
                        c.g[A0][0] = c.g[S7][0];
                        call_ptr(c, 0x0039d860, 0x001842a4)?;
                        c.g[A0][0] = 0x4a0000;
                        c.g[S4][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001842b4);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1448));
                        call_ptr(c, 0x00317670, 0x001842b4)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001842c0);
                        c.g[A0][0] = c.g[S7][0];
                        call_ptr(c, 0x0039d860, 0x001842c0)?;
                        c.g[A0][0] = 0x4a0000;
                        c.g[S5][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001842d0);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1450));
                        call_ptr(c, 0x00317670, 0x001842d0)?;
                        c.g[A0][0] = c.g[S7][0];
                        c.g[RA][0] = sx(0x001842dc);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001842dc)?;
                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                        c.g[S0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001842ec);
                        c.g[A0][0] = c.g[S4][0];
                        call_ptr(c, 0x003a1030, 0x001842ec)?;
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.g[A0][0] = c.g[S5][0];
                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x15c));
                        c.add_s(0, 0, 20);
                        c.g[RA][0] = sx(0x00184304);
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x60));
                        call_ptr(c, 0x003a1030, 0x00184304)?;
                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x160));
                        c.g[RA][0] = sx(0x00184310);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x003a1030, 0x00184310)?;
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.g[A0][0] = c.g[S4][0];
                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x164));
                        c.add_s(0, 0, 20);
                        c.g[RA][0] = sx(0x00184328);
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x80));
                        call_ptr(c, 0x003a0290, 0x00184328)?;
                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x168));
                        c.g[RA][0] = sx(0x00184334);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x003a0290, 0x00184334)?;
                        c.ldl(4, (c.g[S3][0] as u32).wrapping_add(0x4b));
                        c.ldr(4, (c.g[S3][0] as u32).wrapping_add(0x44));
                        c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4c));
                        c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0xd7));
                        c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0xd0));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0xd8), c.g[A1][0]);
                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x144));
                        c.ldl(3, (c.g[S1][0] as u32).wrapping_add(0x4b));
                        c.ldr(3, (c.g[S1][0] as u32).wrapping_add(0x44));
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4c));
                        c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0xe7));
                        c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0xe0));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0xe8), c.g[A0][0]);
                        c.ldl(3, (c.g[S2][0] as u32).wrapping_add(0x4b));
                        c.ldr(3, (c.g[S2][0] as u32).wrapping_add(0x44));
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
                        c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0xf7));
                        c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0xf0));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0xf8), c.g[A0][0]);
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xa0));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa4));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x00184394);
                        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x00184394)?;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x148));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xa0));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa4));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001843ac);
                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001843ac)?;
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x150));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xa0));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa4));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001843c4);
                        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001843c4)?;
                        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.add_s(2, 1, 0);
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
                        let t = c.g[V0][0] == 0u64;
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x100));
                        if t {
                            break 'b_184400;
                        } else {
                            c.cycles += 7;
                            c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x110));
                            c.add_s(0, 0, 20);
                            c.add_s(1, 1, 20);
                            c.add_s(0, 2, 0);
                            c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x110));
                            let t = true;
                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xe0));
                            if t {
                                break 'b_184420;
                            } else {
                                break 'b_184400;
                            }
                        }
                    }
                    c.cycles += 8;
                    c.g[V0][0] = 0x500000;
                    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xf28));
                    c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
                    c.ldr(3, c.g[A2][0] as u32);
                    c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x117));
                    c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x110));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x118), c.g[A0][0]);
                    break 'b_184420;
                }
                c.cycles += 64;
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.g[AT][0] = 0x41200000;
                c.mtc1(1, 4);
                c.add_s(0, 0, 1);
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x200));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x190));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0xd7));
                c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd8));
                c.sdl(2, (c.g[S3][0] as u32).wrapping_add(0x4b));
                c.sdr(2, (c.g[S3][0] as u32).wrapping_add(0x44));
                c.sw((c.g[S3][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
                c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0xe7));
                c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0xe0));
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe8));
                c.sdl(6, (c.g[S1][0] as u32).wrapping_add(0x4b));
                c.sdr(6, (c.g[S1][0] as u32).wrapping_add(0x44));
                c.sw((c.g[S1][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
                c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0xf7));
                c.ldr(5, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf8));
                c.sdl(5, (c.g[S2][0] as u32).wrapping_add(0x4b));
                c.sdr(5, (c.g[S2][0] as u32).wrapping_add(0x44));
                c.sw((c.g[S2][0] as u32).wrapping_add(0x4c), c.g[A2][0]);
                c.ldl(4, (c.g[S4][0] as u32).wrapping_add(0x4b));
                c.ldr(4, (c.g[S4][0] as u32).wrapping_add(0x44));
                c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4c));
                c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0xa7));
                c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.sw((c.g[SP][0] as u32).wrapping_add(0xa8), c.g[A1][0]);
                c.ldl(3, (c.g[S5][0] as u32).wrapping_add(0x4b));
                c.ldr(3, (c.g[S5][0] as u32).wrapping_add(0x44));
                c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x4c));
                c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0xb7));
                c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.sw((c.g[SP][0] as u32).wrapping_add(0xb8), c.g[A0][0]);
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x1f0));
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.add_s(0, 0, 1);
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x1e0));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                c.add_s(0, 0, 4);
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x1c0));
                c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x180));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x170));
                c.add_s(0, 0, 3);
                c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x210));
                c.add_s(0, 0, 2);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0xb7));
                c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xb8));
                c.sdl(2, (c.g[S5][0] as u32).wrapping_add(0x4b));
                c.sdr(2, (c.g[S5][0] as u32).wrapping_add(0x44));
                c.sw((c.g[S5][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x1b0));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x220));
                c.pc = t;
                return Ok(());
            }
        }
    });
}
