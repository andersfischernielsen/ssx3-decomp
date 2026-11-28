//! `src/be/beintecon.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::be::beintmission::lw;
pub(crate) use crate::game::be::beintmission::BE_GET_RIDER_CHARACTER;
pub(crate) use crate::game::be::beintmission::BE_GET_RIDER_PROFILE;
pub(crate) use crate::game::be::beintmission::EA_MEM_ALLOC_NAMED;
pub(crate) use crate::game::be::beintmission::RIDER_STRIDE;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::input::inputparse::o406_lw;
pub(crate) use crate::game::seg::seg_1fbe38::oc417_add;
pub(crate) use crate::game::seg::seg_1fbe38::oc417_lh;
pub(crate) use crate::game::seg::seg_1fbe38::oc417_lw;
pub(crate) use crate::game::seg::seg_1fbe38::oc417_sw;
pub(crate) use crate::game::sound::bankmonitor::p;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0015_08d8,
        be_econ_int_get_instance,
        "BEEconInt_GetInstance",
    ),
    (0x0015_0928, be_econ_int_get_money, "BEEconInt_GetMoney"),
    (
        0x0015_0960,
        be_econ_int_get_rider_money,
        "BEEconInt_GetRiderMoney",
    ),
    (
        0x0015_09c0,
        be_econ_int_get_total_earned,
        "BEEconInt_GetTotalEarned",
    ),
    (
        0x0015_09f8,
        be_econ_int_get_rider_total_earned,
        "BEEconInt_GetRiderTotalEarned",
    ),
    (0x0015_0a90, be_econ_int_add_money, "BEEconInt_AddMoney"),
    (
        0x0015_0ae0,
        be_econ_int_add_rider_money,
        "BEEconInt_AddRiderMoney",
    ),
    (0x0015_0b48, be_econ_int_spend_money, "BEEconInt_SpendMoney"),
    (
        0x0015_0b88,
        be_set_char_state_flag_cheat,
        "BE_SetCharStateFlagCheat",
    ),
    (
        0x0015_0e50,
        be_econ_int_get_attribute_cost,
        "BEEconInt_GetAttributeCost",
    ),
    (
        0x0015_1178,
        reward_get_collect_item_points,
        "BERewardInt_GetCollectItemPoints",
    ),
    (
        0x0015_11b0,
        reward_get_mission_points,
        "BERewardInt_GetMissionPoints",
    ),
    (0x0015_1368, be_char_profile_dtor, "BE_CharProfile_dtor"),
    (
        0x0015_15b8,
        be_restore_char_profile_equip_flags,
        "BE_RestoreCharProfileEquipFlags",
    ),
    (
        0x0015_19e0,
        be_reset_char_rivalries,
        "BE_ResetCharRivalries",
    ),
    (
        0x0015_1c48,
        be_set_char_profile_bolt_owned,
        "BE_SetCharProfileBoltOwned",
    ),
    (
        0x0015_1c90,
        be_set_char_profile_bolt_equipped,
        "BE_SetCharProfileBoltEquipped",
    ),
    (
        0x0015_13b8,
        be_apply_default_char_profile_bolts,
        "BE_ApplyDefaultCharProfileBolts",
    ),
    (0x0015_1600, be_init_char_profile, "BE_InitCharProfile"),
    (
        0x0015_1a88,
        be_init_char_profile_bolt_map,
        "BE_InitCharProfileBoltMap",
    ),
    (
        0x0015_1ef0,
        be_apply_bolt_dependencies,
        "BE_ApplyBoltDependencies",
    ),
    (
        0x0015_2368,
        be_test_char_profile_bolt_flags,
        "BE_TestCharProfileBoltFlags",
    ),
    (
        0x0015_0c20,
        be_econ_int_buy_attribute,
        "BEEconInt_BuyAttribute",
    ),
    (
        0x0015_1040,
        be_reward_int_get_event_reward_points,
        "BERewardInt_GetEventRewardPoints",
    ),
    (
        0x0015_20e8,
        be_validate_exclusive_bolts,
        "BE_ValidateExclusiveBolts",
    ),
    (
        0x0015_21f0,
        be_ensure_default_bolts_equipped,
        "BE_EnsureDefaultBoltsEquipped",
    ),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {

    pub(crate) use crate::game::dirtysock::tagsunk::callee::EA_MEM_FREE_3 as EA_MEM_FREE;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::OPERATOR_DELETE;
}

/// The shared address arithmetic of the two economy getters: `db` (0x4a6ca8)
/// plus the rider slot (stride 0x9b50) and character slot (stride 0xf88).
pub(crate) fn econ_rider_base(c: &mut Cpu) -> u32 {
    c.cycles += 9;
    c.g[A0][0] = 0x9b50;
    let char_off = abi::mult(c, abi::arg(c, 2), 0xf88);
    c.g[V1][0] = sx(char_off);
    let rider_off = abi::mult(c, abi::arg(c, 1), 0x9b50);
    c.g[A0][0] = sx(rider_off);
    c.g[V0][0] = sx(0x4a_0000u32.wrapping_add(0x6ca8));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] as u32
}

/// `int BEEconInt_GetMoney(BEEconInt *self, int rider, int character)`
/// (0x150928): 0 for rider slots >= 2, else the money word at +0xac4.
pub fn be_econ_int_get_money(c: &mut Cpu) {
    c.cycles += 3;
    let rider = abi::arg64(c, 1);
    let under_two = (rider as i64) < 2;
    c.g[V0][0] = under_two as u64;
    c.g[V1][0] = 0xf88;
    if !under_two {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    let base = econ_rider_base(c);
    c.g[V0][0] = Addr(base).off(0xac4).lw(c);
    abi::ret(c);
}

/// `int BEEconInt_GetTotalEarned(BEEconInt *self, int rider, int character)`
/// (0x1509c0): as `GetMoney`, but the word at +0xac8.
pub fn be_econ_int_get_total_earned(c: &mut Cpu) {
    c.cycles += 3;
    let rider = abi::arg64(c, 1);
    let under_two = (rider as i64) < 2;
    c.g[V0][0] = under_two as u64;
    c.g[V1][0] = 0xf88;
    if !under_two {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    let base = econ_rider_base(c);
    c.g[V0][0] = Addr(base).off(0xac8).lw(c);
    abi::ret(c);
}

/// `int BEEconInt_AddMoney(BEEconInt *self, int rider, int character, int amount)`
/// (0x150a90): add `amount` to both money (+0xac4) and total earned (+0xac8);
/// returns the new money value.
pub fn be_econ_int_add_money(c: &mut Cpu) {
    c.cycles += 3;
    let rider = abi::arg64(c, 1);
    let under_two = (rider as i64) < 2;
    c.g[V0][0] = under_two as u64;
    c.g[V1][0] = 0xf88;
    if !under_two {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 15;
    c.g[A0][0] = 0x9b50;
    let rider_off = abi::mult(c, abi::arg(c, 1), 0x9b50);
    c.g[A0][0] = sx(rider_off);
    let char_off = abi::mult(c, abi::arg(c, 2), 0xf88);
    c.g[V1][0] = sx(char_off);
    c.g[V0][0] = sx(0x4a_0000u32.wrapping_add(0x6ca8));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let money = Addr(c.g[V1][0] as u32).off(0xac4).lw(c);
    let earned = Addr(c.g[V1][0] as u32).off(0xac8).lw(c);
    c.g[A1][0] = sx((money as u32).wrapping_add(abi::arg(c, 3)));
    c.g[A0][0] = sx((earned as u32).wrapping_add(abi::arg(c, 3)));
    c.g[V0][0] = c.g[A1][0];
    Addr(c.g[V1][0] as u32).off(0xac8).sw(c, c.g[A0][0]);
    Addr(c.g[V1][0] as u32).off(0xac4).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `int BEEconInt_SpendMoney(BEEconInt *self, int rider, int character, int amount)`
/// (0x150b48): subtract `amount` from money (+0xac4); returns the new value.
pub fn be_econ_int_spend_money(c: &mut Cpu) {
    c.cycles += 3;
    let rider = abi::arg64(c, 1);
    let under_two = (rider as i64) < 2;
    c.g[V0][0] = under_two as u64;
    c.g[V1][0] = 0xf88;
    if !under_two {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 11;
    c.g[A0][0] = 0x9b50;
    let char_off = abi::mult(c, abi::arg(c, 2), 0xf88);
    c.g[V1][0] = sx(char_off);
    let rider_off = abi::mult(c, abi::arg(c, 1), 0x9b50);
    c.g[A0][0] = sx(rider_off);
    c.g[V0][0] = sx(0x4a_0000u32.wrapping_add(0x6ca8));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0xac4).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(abi::arg(c, 3)));
    Addr(c.g[V1][0] as u32).off(0xac4).sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `int BEEconInt_GetAttributeCost(BEEconInt *self, int attribute)`
/// (0x150e50): the cost at `attribute*8 + 0x440554`.
pub fn be_econ_int_get_attribute_cost(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = 0x44_0000;
    c.g[A1][0] = sx(abi::arg(c, 1) << 3);
    c.g[V1][0] = sx(0x44_0000u32.wrapping_add(0x550));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x4).lw(c);
    abi::ret(c);
}

/// `BEEconInt *BEEconInt_GetInstance(void)` (0x1508d8).
pub fn be_econ_int_get_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x1edc));
        let have = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if !have {
            c.cycles += 11;
            c.g[A1][0] = 0x460000;
            c.g[A0][0] = 0x10;
            c.g[A1][0] = sx(0x460000u32.wrapping_sub(0x58e8));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, EA_MEM_ALLOC_NAMED, 0x0015_0900)?;
            c.g[V1][0] = 0x460000;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0);
            c.g[V1][0] = sx(0x460000u32.wrapping_sub(0x5388));
            c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
            c.sw((c.g[GP][0] as u32).wrapping_sub(0x1edc), c.g[V0][0]);
        }
        c.cycles += 4;
        c.g[V0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x1edc));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) const ECON_ADD_MONEY: Callee = Callee {
    addr: 0x0015_0a90,
    f: ssx3_ee::s01::f_00150a90,
};

/// `int BEEconInt_GetRiderMoney(BEEconInt *self, int rider)` (0x150960): the
/// rider's money at `0x4a6ca8 + char*0xf88 + profile*0x9b50 + 0xac4`.
pub fn be_econ_int_get_rider_money(c: &mut Cpu) {
    guest(c, |c| econ_rider_field(c, 0x0015_097c, 0x0015_0988, 0x0ac4))
}

/// `int BEEconInt_GetRiderTotalEarned(BEEconInt *self, int rider)` (0x1509f8):
/// the rider's total earned at +0xac8.
pub fn be_econ_int_get_rider_total_earned(c: &mut Cpu) {
    guest(c, |c| econ_rider_field(c, 0x0015_0a14, 0x0015_0a20, 0x0ac8))
}

/// Shared body of the economy getters: `<char,profile> -> field`.
pub(crate) fn econ_rider_field(c: &mut Cpu, r_prof: u32, r_char: u32, field: u32) -> Flow {
    c.cycles += 24;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S1, 0x10);
    c.g[S1][0] = c.g[A1][0];
    frame.save(c, S0, 0x20);
    frame.save_ra(c, 0);
    c.g[A0][0] = c.g[S1][0];
    BE_GET_RIDER_PROFILE.call(c, r_prof)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[S0][0] = c.g[V0][0];
    BE_GET_RIDER_CHARACTER.call(c, r_char)?;
    c.g[V1][0] = RIDER_STRIDE as u64;
    c.g[A0][0] = 0x9b50;
    c.mult(0, V0, V0, V1);
    c.mult(0, S0, S0, A0);
    c.g[V1][0] = 0x4a0000;
    frame.restore(c, S1, 0x10);
    c.g[V1][0] = sx(0x4a0000u32.wrapping_add(0x6ca8));
    frame.restore_ra(c, 0);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    frame.restore(c, S0, 0x20);
    c.g[V0][0] = lw(c, (c.g[V0][0] as u32).wrapping_add(field));
    frame.close(c);
    Ok(())
}

/// `void BEEconInt_AddRiderMoney(BEEconInt *self, int rider, int amount)`
/// (0x150ae0): add `amount` to the rider's money and total earned.
pub fn be_econ_int_add_rider_money(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        c.g[S3][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        BE_GET_RIDER_PROFILE.call(c, 0x0015_0b0c)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        BE_GET_RIDER_CHARACTER.call(c, 0x0015_0b18)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S3][0];
        c.g[A2][0] = c.g[V0][0];
        ECON_ADD_MONEY.call(c, 0x0015_0b2c)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BE rewards.

/// `int BERewardInt_GetCollectItemPoints(BERewardInt* self, int level)`
/// (0x151178): 1000 points for level 1, 500 below that, 2000 for level 2.
pub fn reward_get_collect_item_points(c: &mut Cpu) {
    c.cycles += 3;
    let level = c.g[A1][0];
    c.g[V0][0] = 1;
    let is_one = level == c.g[V0][0];
    c.g[V0][0] = ((level as i64) < 2) as u64;
    if is_one {
        c.cycles += 2;
        c.g[V0][0] = 0x3e8;
        crate::abi::ret(c);
        return;
    }
    c.cycles += 2;
    let below_two = c.g[V0][0] != 0;
    c.g[V0][0] = 2;
    if !below_two {
        c.cycles += 2;
        if level == c.g[V0][0] {
            c.cycles += 2;
            c.g[V0][0] = 0x7d0;
            crate::abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0x1f4;
    crate::abi::ret(c);
}

/// `int BERewardInt_GetMissionPoints(BERewardInt* self, int level)`
/// (0x1511b0): 4000 for level 2, 2000 below that, 6000 for level 3.
pub fn reward_get_mission_points(c: &mut Cpu) {
    c.cycles += 3;
    let level = c.g[A1][0];
    c.g[V0][0] = 2;
    let is_two = level == c.g[V0][0];
    c.g[V0][0] = ((level as i64) < 3) as u64;
    if is_two {
        c.cycles += 2;
        c.g[V0][0] = 0xfa0;
        crate::abi::ret(c);
        return;
    }
    c.cycles += 2;
    let below_three = c.g[V0][0] != 0;
    c.g[V0][0] = 3;
    if !below_three {
        c.cycles += 2;
        if level == c.g[V0][0] {
            c.cycles += 2;
            c.g[V0][0] = 0x1770;
            crate::abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0x7d0;
    crate::abi::ret(c);
}

/// `void BE_RestoreCharProfileEquipFlags(Profile* self)` (0x1515b8): for each
/// item (flags halfword at `+2`, `self+0x290`, stride 4), set bit 4 when bit
/// 2 is set, clear it otherwise.
pub fn be_restore_char_profile_equip_flags(c: &mut Cpu) {
    c.cycles += 3;
    let base = abi::arg(c, 0);
    c.g[V0][0] = Addr(base).off(0x28c).lw(c);
    let none = (c.g[V0][0] as i64) <= 0;
    c.g[A0][0] = sx(p(base, 0x290));
    if none {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[A1][0] = c.g[V0][0];
    c.g[A2][0] = 0xffff_ffff_ffff_ffef;
    loop {
        c.cycles += 4;
        c.g[V1][0] = Addr(r32(c, A0)).off(2).lhu(c);
        c.g[V0][0] = c.g[V1][0] & 4;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = c.g[V1][0] | 0x10;
        } else {
            c.cycles += 1;
            c.g[V0][0] = c.g[V1][0] & c.g[A2][0];
        }
        c.cycles += 4;
        c.sh(p(r32(c, A0), 2), c.g[V0][0]);
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
        let more = c.g[A1][0] != 0;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0015_15d0);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void BE_SetCharProfileBoltOwned(Profile* p, int id, bool on)` (0x151c48):
/// look `id` up in the u16 index table at `+0x288`; a non-negative index
/// selects the item at `+0x290 + idx*4`, whose flags word (`+2`) gets bit 1
/// set or cleared. A negative index falls back to address 2, as the original
/// does.
pub fn be_set_char_profile_bolt_owned(c: &mut Cpu) {
    c.cycles += 6;
    let prof = abi::arg(c, 0);
    let id = abi::arg(c, 1);
    let lookup = Addr(prof.wrapping_add(0x288)).lw(c) as u32;
    let idx = Addr(lookup.wrapping_add(id << 1)).lh(c);
    c.g[V0][0] = sx((idx as u32) << 2);
    let item = if (idx as i64) < 0 {
        c.cycles += 1;
        0
    } else {
        c.cycles += 3;
        let a = prof.wrapping_add((idx as u32) << 2).wrapping_add(0x290);
        c.g[A0][0] = sx(a);
        a
    };
    c.cycles += 2;
    let on = abi::arg(c, 2);
    let flags = Addr(item.wrapping_add(2)).lhu(c) as u32;
    if on == 0 {
        c.cycles += 3;
        c.g[V0][0] = sx(flags & 0xfffd);
    } else {
        c.cycles += 3;
        c.g[V0][0] = sx(flags | 2);
    }
    Addr(item.wrapping_add(2)).sh(c, c.g[V0][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// BE character profile.

/// `void BE_CharProfile_dtor(BE_CharProfile* p, int flags)` (0x151368):
/// free the owned block at +0x288 and, when bit 0 of `flags` is set, delete
/// the object. 8 (+2) + 3 (+2) + 5 cycles plus the callees.
pub fn be_char_profile_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let owned = c.lw(p(self_, 0x288));
        c.g[A0][0] = owned;
        c.g[S1][0] = c.g[A1][0];
        if owned != 0 {
            c.cycles += 2;
            call_ptr(c, callee::EA_MEM_FREE, 0x0015_1390)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        c.sw(p(self_, 0x288), 0);
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::OPERATOR_DELETE, 0x0015_13a4)?;
        }
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BE_SetCharStateFlagCheat` (0x150b88): set or clear character-state flag
/// 12/13 for player 0 or 1 (cheat helper), returning 1 when handled.
pub fn be_set_char_state_flag_cheat(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        c.g[A0][0] = c.g[A1][0];
        c.g[V0][0] = ((c.g[A0][0] as i64) < 2i64) as u64;
        let player_ok = c.g[V0][0] != 0;
        c.g[A1][0] = c.g[T0][0];
        if player_ok {
            c.cycles += 3;
            c.g[V0][0] = 1;
            let one = c.g[A3][0] != c.g[V0][0];
            c.g[A1][0] = c.g[A1][0] ^ 1;
            if !one {
                c.cycles += 10;
                c.g[V1][0] = 0xf88;
                c.g[V0][0] = 0x9b50;
                c.mult(0, 2, 4, 2);
                c.mult(0, 3, 6, 3);
                c.g[A0][0] = 0x004a_0000;
                c.g[A1][0] = c.g[A1][0] & 1;
                c.g[A0][0] = sx(0x004a_0000u32.wrapping_add(0x6ca8));
                c.g[A2][0] = 0xffff_ffff_ffff_efff;
                c.g[A1][0] = c.g[A1][0] << 12;
            } else {
                c.cycles += 3;
                c.g[V0][0] = 2;
                let two = c.g[A3][0] != c.g[V0][0];
                c.g[V1][0] = 0xf88;
                if two {
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    abi::ret(c);
                    return Ok(());
                }
                c.cycles += 8;
                c.g[V0][0] = 0x9b50;
                c.mult(0, 2, 4, 2);
                c.mult(0, 3, 6, 3);
                c.g[A0][0] = 0x004a_0000;
                c.g[A1][0] = c.g[A1][0] & 1;
                c.g[A0][0] = sx(0x004a_0000u32.wrapping_add(0x6ca8));
                c.g[A2][0] = 0xffff_ffff_ffff_dfff;
                c.g[A1][0] = c.g[A1][0] << 13;
            }
            c.cycles += 6;
            c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
            c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
            c.g[V0][0] = c.ld(p(r(c, V1), 0x278));
            c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
            c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
            c.sd(p(r(c, V1), 0x278), c.g[V0][0]);
            c.cycles += 2;
            c.g[V0][0] = 1;
            abi::ret(c);
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
        }
        Ok(())
    })
}

/// `void BE_ResetCharRivalries(p, index)` (0x1519e0): write the three record
/// bytes for `index` into the rivalry table (10 entries of 3 bytes).
pub fn be_reset_char_rivalries(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x10);
        c.cycles += 20;
        c.g[V1][0] = 0xc;
        c.mult(0, 5, 5, 3);
        c.g[V0][0] = sx(0x0044_0000);
        c.g[V1][0] = sx(0x004a_0000);
        c.g[V0][0] = sx(0x0044_06fc);
        c.g[T2][0] = sx(0x004a_1218);
        c.g[A3][0] = Addr(c.g[T2][0] as u32).lb(c);
        c.g[T0][0] = Addr(at(c.g[T2][0], 1)).lb(c);
        c.g[T1][0] = Addr(at(c.g[T2][0], 2)).lb(c);
        Addr(frame.at(c, 0)).sb(c, c.g[A3][0]);
        Addr(frame.at(c, 1)).sb(c, c.g[T0][0]);
        Addr(frame.at(c, 2)).sb(c, c.g[T1][0]);
        c.g[A2][0] = sx(at(c.g[V0][0], 4));
        c.g[A0][0] = sx(at(c.g[A0][0], 0xbc2));
        c.g[V1][0] = 0;
        c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[T0][0] = 3;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[A2][0] = 2;
        loop {
            'next: {
                'emit: {
                    'zero_or_match: {
                        c.cycles += 9;
                        c.g[T3][0] = Addr(frame.at(c, 0)).lb(c);
                        c.g[V0][0] = Addr(frame.at(c, 1)).lb(c);
                        c.g[T1][0] = Addr(frame.at(c, 2)).lb(c);
                        Addr(at(c.g[A0][0], 0xffff_ffff)).sb(c, c.g[T3][0]);
                        Addr(c.g[A0][0] as u32).sb(c, c.g[V0][0]);
                        Addr(at(c.g[A0][0], 1)).sb(c, c.g[T1][0]);
                        c.g[V0][0] = lw(c, c.g[A3][0] as u32);
                        if c.g[V1][0] != c.g[V0][0] {
                            c.g[V0][0] = lw(c, c.g[A1][0] as u32);
                            break 'zero_or_match;
                        }
                        c.cycles += 3;
                        Addr(at(c.g[A0][0], 0xffff_ffff)).sb(c, 0);
                        Addr(c.g[A0][0] as u32).sb(c, 0);
                        break 'emit;
                    }
                    c.cycles += 2;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V1][0] = sx(at(c.g[V1][0], 1));
                        break 'next;
                    }
                    c.cycles += 2;
                    Addr(at(c.g[A0][0], 0xffff_ffff)).sb(c, c.g[T0][0]);
                    Addr(c.g[A0][0] as u32).sb(c, c.g[A2][0]);
                    break 'emit;
                }
                c.cycles += 1;
                c.g[V1][0] = sx(at(c.g[V1][0], 1));
                break 'next;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 10i64) as u64;
            let t = c.g[V0][0] != 0;
            c.g[A0][0] = sx(at(c.g[A0][0], 3));
            if !t {
                c.cycles += 2;
                frame.close(c);
                return Ok(());
            }
            if crate::events::event_due(c) {
                crate::events::deliver_event_here(c, 0x0015_1a30);
            }
        }
    })
}

// ---------------------------------------------------------------------------
// BE_SetCharProfileBoltEquipped (0x151c90)

pub(crate) const BOLT_EQUIP_GET_DB: Callee = Callee {
    addr: 0x0014_bdb8,
    f: ee::s01::Bolts_GetDB,
};

pub(crate) const BOLT_EQUIP_FIND_INDEX: Callee = Callee {
    addr: 0x0014_d448,
    f: ee::s01::Bolts_FindBoltIndex,
};

pub(crate) const BOLT_EQUIP_FIRST_CHILD: Callee = Callee {
    addr: 0x0014_d558,
    f: ee::s01::Bolts_FindFirstChild,
};

pub(crate) const BOLT_EQUIP_COUNT_CHILDREN: Callee = Callee {
    addr: 0x0014_d4c8,
    f: ee::s01::Bolts_CountChildren,
};

pub(crate) const BOLT_EQUIP_APPLY_DEPS: Callee = Callee {
    addr: 0x0015_1ef0,
    f: ee::s01::BE_ApplyBoltDependencies,
};

/// The recursion goes through the generated alias (the override slot once
/// installed).
pub(crate) const BOLT_EQUIP_SELF: Callee = Callee {
    addr: 0x0015_1c90,
    f: ee::s01::BE_SetCharProfileBoltEquipped,
};

/// Profile fields: the id -> item index table (`short*`), the items (4 bytes
/// each, flags at +2: 0x10 = equipped), the bolt kind (byte).
pub(crate) const BOLT_EQUIP_LOOKUP: u32 = 0x288;

pub(crate) const BOLT_EQUIP_ITEMS: u32 = 0x290;

pub(crate) const BOLT_EQUIP_KIND: u32 = 0xbc0;

/// `int BE_SetCharProfileBoltEquipped(CharProfile* prof, int id, int on)`
/// (0x151c90): set or clear bolt `id`'s equipped flag. Equipping an exclusive
/// bolt (def flag 1) first unequips the other members of its group (owned
/// ones, or forced by their flag 8); unequipping a bolt (or one with flag 8)
/// unequips its children. A real change then applies the bolt's dependency
/// links. Returns 0 when a nested change failed, else 1.
pub fn be_set_char_profile_bolt_equipped(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S2, 0x60);
        frame.save(c, S5, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S7, 0x10);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S4, 0x40);
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        frame.save(c, S3, 0x50);
        frame.save(c, S6, 0x20);
        frame.save_ra(c, 0);
        c.g[S7][0] = c.g[A2][0];
        BOLT_EQUIP_GET_DB.call(c, 0x0015_1cc8)?;
        c.g[S4][0] = c.g[V0][0];
        c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(BOLT_EQUIP_KIND));
        c.g[A0][0] = c.g[S4][0];
        c.g[A2][0] = c.g[S5][0];
        BOLT_EQUIP_FIND_INDEX.call(c, 0x0015_1cdc)?;
        // t0 = the bolt's definition (0x38 bytes each), or null.
        c.g[A0][0] = c.g[V0][0];
        if (c.g[V0][0] as i64) >= 0 {
            c.cycles += 4;
            c.g[V0][0] = 0x38;
            c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
            c.mult(0, V0, A0, V0);
            c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        } else {
            c.cycles += 2;
            c.g[T0][0] = 0;
        }
        // a1 = the profile's item for the bolt, or null.
        c.cycles += 6;
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(BOLT_EQUIP_LOOKUP));
        c.g[V0][0] = sx((c.g[S5][0] as u32) << 1);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = c.lh(c.g[V0][0] as u32);
        let missing = (c.g[V0][0] as i64) < 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        if !missing {
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(BOLT_EQUIP_ITEMS));
            c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        } else {
            c.cycles += 1;
            c.g[A1][0] = 0;
        }
        // a3 = exclusive (def flag 1), a2 = forced (flag 8), s6 = was equipped.
        c.cycles += 8;
        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x34));
        c.g[A0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x2));
        c.g[A3][0] = c.g[V0][0] & 0x1;
        c.g[V0][0] = c.g[V0][0] & 0x8;
        c.g[V1][0] = c.g[A0][0] & 0x10;
        c.g[A2][0] = (c.g[V0][0] != 0) as u64;
        c.g[S6][0] = (c.g[V1][0] != 0) as u64;
        if c.g[S7][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = c.g[A0][0] | 0x10;
        } else {
            c.cycles += 1;
            c.g[V0][0] = c.g[A0][0] & 0xffef;
        }
        c.cycles += 2;
        c.sh((c.g[A1][0] as u32).wrapping_add(0x2), c.g[V0][0]);
        'exit: {
            'e84: {
                if c.g[S7][0] != 0 {
                    // Equip.
                    c.cycles += 2;
                    if c.g[S6][0] != 0 {
                        c.g[V0][0] = 0x1;
                        break 'exit;
                    }
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S4][0];
                    if c.g[A3][0] == 0 {
                        // e8c: (s6 is 0 here) apply the dependencies.
                        c.cycles += 2;
                        c.g[A0][0] = c.g[S2][0];
                        bolt_equip_apply_deps(c)?;
                        break 'exit;
                    }
                    // Unequip the rest of the exclusive group.
                    c.cycles += 14;
                    c.g[S0][0] = c.lh((c.g[T0][0] as u32).wrapping_add(0x6));
                    c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(BOLT_EQUIP_KIND));
                    c.g[S3][0] = 0;
                    c.g[A2][0] = c.g[S0][0];
                    BOLT_EQUIP_FIRST_CHILD.call(c, 0x0015_1d7c)?;
                    c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(BOLT_EQUIP_KIND));
                    c.g[A2][0] = c.g[S0][0];
                    c.g[S1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S4][0];
                    BOLT_EQUIP_COUNT_CHILDREN.call(c, 0x0015_1d90)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V0][0] = ((c.g[S6][0] as i64) < (c.g[S0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        break 'e84;
                    }
                    loop {
                        c.cycles += 3;
                        c.g[A1][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x4));
                        let is_self = c.g[A1][0] == c.g[S5][0];
                        c.g[V0][0] = sx((c.g[A1][0] as u32) << 1);
                        'counted: {
                            'skip: {
                                if is_self {
                                    break 'skip;
                                }
                                c.cycles += 5;
                                c.g[V1][0] =
                                    c.lw((c.g[S2][0] as u32).wrapping_add(BOLT_EQUIP_LOOKUP));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                c.g[V0][0] = c.lh(c.g[V0][0] as u32);
                                let missing = (c.g[V0][0] as i64) < 0;
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                if missing {
                                    break 'skip;
                                }
                                c.cycles += 5;
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                                c.g[V1][0] =
                                    c.lhu((c.g[V0][0] as u32).wrapping_add(BOLT_EQUIP_ITEMS + 2));
                                c.g[V1][0] = c.g[V1][0] & 0x10;
                                let equipped = c.g[V1][0] != 0;
                                c.g[A0][0] = c.g[S2][0];
                                if !equipped {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
                                    c.g[V0][0] = c.g[V0][0] & 0x8;
                                    if c.g[V0][0] == 0 {
                                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                        break 'counted;
                                    }
                                }
                                c.cycles += 4;
                                c.g[A2][0] = 0;
                                BOLT_EQUIP_SELF.call(c, 0x0015_1dec)?;
                                let failed = c.g[V0][0] == 0;
                                c.g[V0][0] = 0;
                                if failed {
                                    break 'exit;
                                }
                            }
                            c.cycles += 1;
                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S0][0] as i64)) as u64;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x38));
                        if c.g[V0][0] == 0 {
                            c.cycles += 2;
                            break 'e84;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0015_1da0);
                        }
                    }
                } else {
                    // Unequip: also unequip the children (if it was
                    // equipped, or is forced).
                    c.cycles += 2;
                    if c.g[S6][0] != 0 {
                        c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(BOLT_EQUIP_KIND));
                    } else {
                        c.cycles += 2;
                        if c.g[A2][0] == 0 {
                            break 'e84;
                        }
                        c.cycles += 1;
                        c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(BOLT_EQUIP_KIND));
                    }
                    c.cycles += 13;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A2][0] = c.g[S5][0];
                    c.g[S3][0] = 0;
                    BOLT_EQUIP_FIRST_CHILD.call(c, 0x0015_1e38)?;
                    c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(BOLT_EQUIP_KIND));
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A2][0] = c.g[S5][0];
                    BOLT_EQUIP_COUNT_CHILDREN.call(c, 0x0015_1e4c)?;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[S1][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        break 'e84;
                    }
                    c.cycles += 1;
                    loop {
                        c.cycles += 6;
                        c.g[A1][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A2][0] = 0;
                        BOLT_EQUIP_SELF.call(c, 0x0015_1e70)?;
                        let failed = c.g[V0][0] == 0;
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        if failed {
                            if event_due(c) {
                                deliver_event_here(c, 0x0015_1e0c);
                            }
                            c.cycles += 2;
                            c.g[V0][0] = 0;
                            break 'exit;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S1][0] as i64)) as u64;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x38));
                        if c.g[V0][0] == 0 {
                            break 'e84;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0015_1e60);
                        }
                    }
                }
            }
            // e84: apply the dependency links if the state really changed.
            c.cycles += 2;
            if c.g[S7][0] != 0 {
                c.cycles += 2;
                c.g[A0][0] = c.g[S2][0];
                if c.g[S6][0] == 0 {
                    bolt_equip_apply_deps(c)?;
                    break 'exit;
                }
                // (s7 is still set: e94 returns 1.)
                c.cycles += 2;
                c.g[V0][0] = 0x1;
                break 'exit;
            }
            c.cycles += 2;
            c.g[A0][0] = c.g[S2][0];
            if c.g[S6][0] == 0 {
                c.cycles += 1;
                c.g[V0][0] = 0x1;
                break 'exit;
            }
            bolt_equip_apply_deps(c)?;
        }
        c.cycles += 11;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore(c, S7, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// 0x151ea4: `v0 = BE_ApplyBoltDependencies(prof, id, on) ? 1 : 0` (a0 is set).
pub(crate) fn bolt_equip_apply_deps(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 6;
    c.g[A1][0] = c.g[S5][0];
    c.g[A2][0] = c.g[S7][0];
    BOLT_EQUIP_APPLY_DEPS.call(c, 0x0015_1eb0)?;
    c.g[V1][0] = c.g[V0][0];
    c.g[V0][0] = 0;
    if c.g[V1][0] != 0 {
        c.cycles += 1;
        c.g[V0][0] = 0x1;
    }
    Ok(())
}

// ----------------------------------------------- BE_ApplyBoltDependencies

/// `int BE_ApplyBoltDependencies(BE *self, int id, int on)` (0x151ef0):
/// switching bolt `id` on or off pulls its dependency links from the bolt
/// database. Each link (0xc bytes: condition id +6, condition value +5,
/// target +0xa, wanted state +9): skipped unless its condition item matches,
/// then the target item (item index table at `self+0x288`, items 4 words
/// from `self+0x290`) is either `id` itself (set/clear its 0x10 flag) or
/// another bolt that is switched through `BE_SetCharProfileBoltEquipped`.
/// Returns 0 when that call fails, else 1.
pub fn be_apply_bolt_dependencies(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S0, 0x90);
        frame.save(c, S2, 0x70);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S6, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S5, 0x40);
        frame.save(c, S1, 0x80);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S7, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S6][0] = c.g[A1][0];
        call_ptr(c, 0x0014_bdb8, 0x0015_1f28)?;
        c.g[S5][0] = c.g[V0][0];
        let a = (c.g[S2][0] as u32).wrapping_add(0xbc0);
        c.g[A1][0] = c.lb(a);
        c.g[A3][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S5][0];
        let a = frame.at(c, 0);
        c.sw(a, 0);
        c.g[A2][0] = c.g[S6][0];
        c.g[T0][0] = c.g[SP][0];
        call_ptr(c, 0x0014_db40, 0x0015_1f48)?;
        c.g[A0][0] = o406_lw(c, S2, 0x288);
        c.g[V1][0] = sx((c.g[S6][0] as u32) << 1);
        c.g[S0][0] = c.g[V0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = c.lh(c.g[V1][0] as u32);
        let missing = (c.g[V0][0] as i64) < 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        if missing {
            c.cycles += 1;
            c.g[S3][0] = 0;
        } else {
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
            c.g[S3][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        }
        'ret: {
            'done: {
                c.cycles += 3;
                c.g[V0][0] = c.lw(frame.at(c, 0));
                let none = (c.g[V0][0] as i64) <= 0;
                c.g[S4][0] = 0;
                if none {
                    break 'done;
                }
                c.cycles += 2;
                c.g[S7][0] = u64::MAX;
                loop {
                    'tail: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0xa));
                        let no_target = c.g[V0][0] == c.g[S7][0];
                        c.g[A0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xa));
                        if no_target {
                            c.cycles += 1;
                            c.g[V0][0] = c.lw(frame.at(c, 0));
                            break 'tail;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(6));
                        let no_cond = c.g[V0][0] == c.g[S7][0];
                        c.g[A1][0] = o406_lw(c, S2, 0x288);
                        if !no_cond {
                            // Is the condition item owned the way the link wants?
                            c.cycles += 5;
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                            c.g[V0][0] = c.lh(c.g[V0][0] as u32);
                            let absent = (c.g[V0][0] as i64) < 0;
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            if absent {
                                c.cycles += 1;
                                c.g[V0][0] = 0;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                                c.g[V0][0] =
                                    sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            }
                            c.cycles += 7;
                            c.g[V0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(2));
                            c.g[V1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(5));
                            c.g[V0][0] &= 0x10;
                            c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
                            c.g[V1][0] = (0u64 < c.g[V1][0]) as u64;
                            let differ = c.g[V0][0] != c.g[V1][0];
                            c.g[V0][0] = c.lw(frame.at(c, 0));
                            if differ {
                                break 'tail;
                            }
                        }
                        // The target item.
                        c.cycles += 15;
                        c.g[V0][0] = sx((c.g[A0][0] as u32) << 16);
                        c.g[A2][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
                        c.g[A0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(9));
                        c.g[V1][0] = sx((c.g[A2][0] as u32) << 1);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        c.g[A0][0] = (0u64 < c.g[A0][0]) as u64;
                        c.g[V0][0] = c.lh(c.g[V1][0] as u32);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[V1][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(2));
                        c.g[V1][0] &= 0x10;
                        c.g[V1][0] = (0u64 < c.g[V1][0]) as u64;
                        if c.g[V1][0] != c.g[A0][0] {
                            c.g[A1][0] = c.lh(c.g[S1][0] as u32);
                        } else {
                            c.cycles += 2;
                            let owned = c.g[V1][0] != 0;
                            c.g[V0][0] = c.lw(frame.at(c, 0));
                            if owned {
                                break 'tail;
                            }
                            // Not owned, link wants it off: only when the bolt
                            // definition has flag 8.
                            c.cycles += 5;
                            let a = (c.g[S2][0] as u32).wrapping_add(0xbc0);
                            c.g[A1][0] = c.lb(a);
                            c.g[A0][0] = c.g[S5][0];
                            call_ptr(c, 0x0014_d448, 0x0015_2030)?;
                            let found = (c.g[V0][0] as i64) >= 0;
                            c.g[A0][0] = c.g[V0][0];
                            if found {
                                c.cycles += 4;
                                c.g[V0][0] = 0x38;
                                c.g[V1][0] = o406_lw(c, S5, 4);
                                c.mult(0, V0, A0, V0);
                                c.g[V0][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            } else {
                                c.cycles += 2;
                                c.g[V0][0] = 0;
                            }
                            c.cycles += 4;
                            c.g[V0][0] = o406_lw(c, V0, 0x34);
                            c.g[V0][0] &= 8;
                            let no_flag = c.g[V0][0] == 0;
                            c.g[V0][0] = c.lw(frame.at(c, 0));
                            if no_flag {
                                break 'tail;
                            }
                            c.cycles += 1;
                            c.g[A1][0] = c.lh(c.g[S1][0] as u32);
                        }
                        // 0x152090: the target is `id` itself, or another bolt.
                        c.cycles += 2;
                        if c.g[A1][0] != c.g[S6][0] {
                            let a = (c.g[S0][0] as u32).wrapping_add(9);
                            c.g[A2][0] = c.lb(a);
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[A2][0] = (0u64 < c.g[A2][0]) as u64;
                            call_ptr(c, 0x0015_1c90, 0x0015_209c)?;
                            let failed = c.g[V0][0] == 0;
                            c.g[V0][0] = 0;
                            if failed {
                                break 'ret;
                            }
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(9));
                            let off = c.g[V0][0] == 0;
                            c.g[V0][0] = c.lhu((c.g[S3][0] as u32).wrapping_add(2));
                            if !off {
                                c.cycles += 3;
                                c.g[V0][0] |= 0x10;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] &= 0xffef;
                            }
                            let a = (c.g[S3][0] as u32).wrapping_add(2);
                            c.sh(a, c.g[V0][0]);
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw(frame.at(c, 0));
                    }
                    c.cycles += 4;
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(1));
                    c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                    if !more {
                        break 'done;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0015_1f88);
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 1;
        }
        c.cycles += 11;
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
    })
}

/// `void BE_InitCharProfile(CharProfile* self, int k, int reload)` (0x151600):
/// reset a character's economy profile: optionally reload the bolt map
/// (`BE_InitCharProfileBoltMap`), set the unlock bits, clear the slot and
/// entry tables, copy the 88 mission flags from the table at 0x43ee10, take
/// the colour from the table at 0x5308d9, then reset the rivalries and the
/// default tricks (kind at +0xbc0) and clear the counters.
pub fn be_init_char_profile(c: &mut Cpu) {
    const BEBAGT_INT_INIT_DEFAULT_TRICKS: Callee = Callee {
        addr: 0x00150558,
        f: ee::s01::f_00150558,
    };
    const BE_INIT_CHAR_PROFILE_BOLT_MAP: Callee = Callee {
        addr: 0x00151a88,
        f: ee::s01::f_00151a88,
    };
    const BE_RESET_CHAR_RIVALRIES: Callee = Callee {
        addr: 0x001519e0,
        f: ee::s01::f_001519e0,
    };
    guest(c, |c| {
        'b_151640: {
            c.cycles += 14;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xd0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S3][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.g[FP][0] = c.g[A1][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
            let t = c.g[A2][0] == 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
            if t {
                break 'b_151640;
            } else {
                c.cycles += 2;
                BE_INIT_CHAR_PROFILE_BOLT_MAP.call(c, 0x00151640)?;
                break 'b_151640;
            }
        }
        c.cycles += 78;
        c.g[V0][0] = c.ld((c.g[S3][0] as u32).wrapping_add(0x278));
        c.g[V1][0] = 0xfffffffffffffffe;
        c.g[A0][0] = 0xfffffffffffffffd;
        c.g[A1][0] = 0xfffffffffffffffb;
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[A2][0] = 0xfffffffffffffff7;
        c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
        c.g[V1][0] = 0xffffffffffffffef;
        c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
        c.g[A0][0] = 0xffffffffffffffdf;
        c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
        c.g[A1][0] = 0x1000;
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[A2][0] = 0x2000;
        c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
        c.g[V1][0] = 0x40;
        c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
        c.g[A0][0] = 0x80;
        c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
        c.g[A1][0] = 0x100;
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.g[A2][0] = 0x200;
        c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
        c.g[V1][0] = 0x400;
        c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
        c.g[A0][0] = 0x800;
        c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
        c.g[A1][0] = 0x4000;
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.g[A2][0] = 0x8000;
        c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
        c.g[V1][0] = 0x10000;
        c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
        c.g[A0][0] = 0x20000;
        c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
        c.g[A1][0] = 0x40000;
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.g[A2][0] = 0x80000;
        c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
        c.g[V1][0] = 0x1;
        c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
        c.sh((c.g[S3][0] as u32).wrapping_add(0x284), c.g[V1][0]);
        c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
        c.sw(c.g[S3][0] as u32, c.g[V1][0]);
        c.sd((c.g[S3][0] as u32).wrapping_add(0x278), c.g[V0][0]);
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0xba0));
        c.sh((c.g[S3][0] as u32).wrapping_add(0x280), c.g[V1][0]);
        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0xbdf));
        c.sh((c.g[S3][0] as u32).wrapping_add(0x282), 0u64);
        c.g[A1][0] = 0u64;
        c.sh((c.g[S3][0] as u32).wrapping_add(0x286), 0u64);
        c.g[A2][0] = 0x18;
        c.sw((c.g[S3][0] as u32).wrapping_add(0xacc), 0u64);
        c.g[S4][0] = 0u64;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
        call_ptr(c, 0x00416210, 0x00151720)?;
        c.g[V1][0] = sx((c.g[FP][0] as u32) << 4);
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 0u64;
        c.g[A2][0] = 0x8;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[V1][0]);
        call_ptr(c, 0x00416210, 0x00151738)?;
        c.g[V0][0] = 0xffffffffffffffff;
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0xbe6));
        c.sb((c.g[SP][0] as u32).wrapping_add(0x1), c.g[V0][0]);
        c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0xbb8));
        c.g[A2][0] = sx((c.g[S3][0] as u32).wrapping_add(0xde4));
        c.g[A3][0] = sx((c.g[S3][0] as u32).wrapping_add(0xf08));
        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0xf28));
        c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(0xf30));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x28), c.g[A0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), c.g[A2][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[A3][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V1][0]);
        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0xad0));
        'l_151778: loop {
            c.cycles += 2;
            c.g[V1][0] = 0x19;
            'l_151780: loop {
                c.cycles += 7;
                c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(4, c.g[SP][0] as u32);
                c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x7));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                c.sdr(4, c.g[V0][0] as u32);
                let t = (c.g[V1][0] as i64) >= 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00151780);
                    }
                    continue 'l_151780;
                } else {
                    c.cycles += 4;
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[S4][0] as i64) < 17i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0xad0));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00151778);
                        }
                        continue 'l_151778;
                    } else {
                        c.cycles += 5;
                        c.g[S4][0] = 0u64;
                        c.g[S7][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4));
                        c.g[S6][0] = 0x8;
                        c.g[S5][0] = 0u64;
                        'l_1517c0: loop {
                            c.cycles += 4;
                            c.g[S1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S7][0] as u32));
                            c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S6][0] as u32));
                            c.g[S2][0] = 0x16;
                            'l_1517d0: loop {
                                c.cycles += 9;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A1][0] = 0u64;
                                c.g[A2][0] = 0x8;
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                                call_ptr(c, 0x00416210, 0x001517e4)?;
                                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                c.sb(c.g[S1][0] as u32, 0u64);
                                let t = (c.g[S2][0] as i64) >= 0;
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x001517d0);
                                    }
                                    continue 'l_1517d0;
                                } else {
                                    c.cycles += 4;
                                    c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0xc));
                                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                                    let t = (c.g[S4][0] as i64) <= 0;
                                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0xc));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x001517c0);
                                        }
                                        continue 'l_1517c0;
                                    } else {
                                        c.cycles += 9;
                                        c.g[V0][0] = 0x440000;
                                        c.g[T1][0] = 0xfffffffffffffffe;
                                        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x11f0));
                                        c.g[T0][0] = 0xfffffffffffffffb;
                                        c.g[A3][0] = 0xfffffffffffffff7;
                                        c.g[A2][0] = 0xffffffffffffffef;
                                        c.g[A0][0] = c.g[S3][0];
                                        c.g[S4][0] = 0x57;
                                        'l_151828: loop {
                                            c.cycles += 19;
                                            c.g[V0][0] =
                                                c.lb((c.g[A1][0] as u32).wrapping_add(0x1a));
                                            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x1));
                                            c.g[V1][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x118));
                                            c.g[V0][0] = c.g[V0][0] & 0x1;
                                            c.g[V1][0] = c.g[V1][0] & c.g[T1][0];
                                            c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                                            c.g[V1][0] = c.g[V1][0] | 0x2;
                                            c.g[V1][0] = c.g[V1][0] & c.g[T0][0];
                                            c.g[V1][0] = c.g[V1][0] & c.g[A3][0];
                                            c.sw(
                                                (c.g[A0][0] as u32).wrapping_add(0x118),
                                                c.g[V1][0],
                                            );
                                            c.g[V1][0] = c.g[V1][0] & c.g[A2][0];
                                            c.g[V0][0] =
                                                c.lb((c.g[A1][0] as u32).wrapping_add(0x1b));
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x24));
                                            c.g[V0][0] = c.g[V0][0] & 0x1;
                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                                            c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                                            c.sw(
                                                (c.g[A0][0] as u32).wrapping_add(0x118),
                                                c.g[V1][0],
                                            );
                                            let t = (c.g[S4][0] as i64) >= 0;
                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x00151828);
                                                }
                                                continue 'l_151828;
                                            } else {
                                                c.cycles += 39;
                                                c.g[A0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                                                c.g[A2][0] = 0x7;
                                                c.g[A1][0] = 0x1;
                                                call_ptr(c, 0x00416210, 0x00151884)?;
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                                c.g[A0][0] = c.g[S3][0];
                                                c.g[V1][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_sub(c.g[FP][0] as u32));
                                                c.g[V0][0] = 0x530000;
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x8d9));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                c.lwl(5, (c.g[V1][0] as u32).wrapping_add(0x3));
                                                c.lwr(5, c.g[V1][0] as u32);
                                                c.g[A2][0] =
                                                    c.lb((c.g[V1][0] as u32).wrapping_add(0x4));
                                                c.g[A3][0] =
                                                    c.lb((c.g[V1][0] as u32).wrapping_add(0x5));
                                                c.g[V0][0] =
                                                    c.lb((c.g[V1][0] as u32).wrapping_add(0x6));
                                                c.swl(5, (c.g[S3][0] as u32).wrapping_add(0xbe2));
                                                c.swr(5, (c.g[S3][0] as u32).wrapping_add(0xbdf));
                                                c.sb(
                                                    (c.g[S3][0] as u32).wrapping_add(0xbe3),
                                                    c.g[A2][0],
                                                );
                                                c.sb(
                                                    (c.g[S3][0] as u32).wrapping_add(0xbe4),
                                                    c.g[A3][0],
                                                );
                                                c.sb(
                                                    (c.g[S3][0] as u32).wrapping_add(0xbe5),
                                                    c.g[V0][0],
                                                );
                                                c.sw((c.g[S3][0] as u32).wrapping_add(0xac4), 0u64);
                                                c.g[A1][0] =
                                                    c.lb((c.g[S3][0] as u32).wrapping_add(0xbc0));
                                                c.sw((c.g[S3][0] as u32).wrapping_add(0xac8), 0u64);
                                                BE_RESET_CHAR_RIVALRIES.call(c, 0x001518d4)?;
                                                c.g[A1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                                                c.g[A0][0] =
                                                    c.lb((c.g[S3][0] as u32).wrapping_add(0xbc0));
                                                BEBAGT_INT_INIT_DEFAULT_TRICKS
                                                    .call(c, 0x001518e0)?;
                                                c.g[A0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                                c.g[A1][0] = 0u64;
                                                c.g[A2][0] = 0x8;
                                                call_ptr(c, 0x00416210, 0x001518f0)?;
                                                c.g[A0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                                                c.g[A1][0] = 0u64;
                                                c.g[A2][0] = 0x54;
                                                call_ptr(c, 0x00416210, 0x00151900)?;
                                                c.sb((c.g[S3][0] as u32).wrapping_add(0xf04), 0u64);
                                                c.g[A3][0] = 0x1;
                                                c.g[A2][0] = 0u64;
                                                c.g[A1][0] =
                                                    sx((c.g[S3][0] as u32).wrapping_add(0xe38));
                                                'l_151910: loop {
                                                    c.cycles += 10;
                                                    c.sw(c.g[A1][0] as u32, 0u64);
                                                    c.g[V0][0] = sx((c.g[A3][0] as u32)
                                                        << ((c.g[A2][0] as u32) & 31));
                                                    c.g[V0][0] = !(0u64 | c.g[V0][0]);
                                                    c.g[A2][0] =
                                                        sx((c.g[A2][0] as u32).wrapping_add(0x1));
                                                    c.g[V1][0] =
                                                        c.lw(
                                                            (c.g[S3][0] as u32).wrapping_add(0xf00)
                                                        );
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32).wrapping_add(0x8));
                                                    c.g[A0][0] =
                                                        ((c.g[A2][0] as i64) < 25i64) as u64;
                                                    c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                                                    let t = c.g[A0][0] != 0u64;
                                                    c.sw(
                                                        (c.g[S3][0] as u32).wrapping_add(0xf00),
                                                        c.g[V1][0],
                                                    );
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x00151910);
                                                        }
                                                        continue 'l_151910;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[A3][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x14));
                                                        c.g[A2][0] = 0u64;
                                                        c.g[T1][0] = 0xffffffffffffffff;
                                                        c.g[T0][0] = 0x1;
                                                        'l_151948: loop {
                                                            c.cycles += 14;
                                                            c.g[V0][0] = ((c.g[T1][0] as i64)
                                                                < (c.g[A2][0] as i64))
                                                                as u64;
                                                            c.g[V1][0] = sx((c.g[A2][0] as u32)
                                                                .wrapping_add(0x1f));
                                                            if c.g[V0][0] != 0 {
                                                                c.g[V1][0] = c.g[A2][0];
                                                            }
                                                            c.g[V1][0] = sx(((c.g[V1][0] as u32)
                                                                as i32
                                                                >> 5)
                                                                as u32);
                                                            c.g[A0][0] = sx((c.g[T0][0] as u32)
                                                                << ((c.g[A2][0] as u32) & 31));
                                                            c.g[V1][0] =
                                                                sx((c.g[V1][0] as u32) << 2);
                                                            c.g[A0][0] = !(0u64 | c.g[A0][0]);
                                                            c.g[V1][0] = sx((c.g[A3][0] as u32)
                                                                .wrapping_add(c.g[V1][0] as u32));
                                                            c.g[A2][0] = sx((c.g[A2][0] as u32)
                                                                .wrapping_add(0x1));
                                                            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                                            c.g[A1][0] = ((c.g[A2][0] as i64)
                                                                < 61i64)
                                                                as u64;
                                                            c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
                                                            let t = c.g[A1][0] != 0u64;
                                                            c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                                                            if t {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c, 0x00151948,
                                                                    );
                                                                }
                                                                continue 'l_151948;
                                                            } else {
                                                                c.cycles += 23;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0x18));
                                                                c.g[A1][0] = 0u64;
                                                                c.g[A2][0] = 0x4;
                                                                call_ptr(
                                                                    c, 0x003e6448, 0x00151990,
                                                                )?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0x20));
                                                                c.g[A1][0] = 0u64;
                                                                c.g[A2][0] = 0x50;
                                                                call_ptr(
                                                                    c, 0x003e6448, 0x001519a0,
                                                                )?;
                                                                c.g[V0][0] = 0x11;
                                                                c.sw(
                                                                    (c.g[S3][0] as u32)
                                                                        .wrapping_add(0xf80),
                                                                    0u64,
                                                                );
                                                                c.sw(
                                                                    (c.g[S3][0] as u32)
                                                                        .wrapping_add(0x27c),
                                                                    c.g[V0][0],
                                                                );
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0xc0),
                                                                );
                                                                c.lq(
                                                                    17,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0xb0),
                                                                );
                                                                c.lq(
                                                                    18,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0xa0),
                                                                );
                                                                c.lq(
                                                                    19,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x90),
                                                                );
                                                                c.lq(
                                                                    20,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x80),
                                                                );
                                                                c.lq(
                                                                    21,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x70),
                                                                );
                                                                c.lq(
                                                                    22,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x60),
                                                                );
                                                                c.lq(
                                                                    23,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x50),
                                                                );
                                                                c.lq(
                                                                    30,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x40),
                                                                );
                                                                c.g[RA][0] = c
                                                                    .ld((c.g[SP][0] as u32)
                                                                        .wrapping_add(0x30));
                                                                let t = c.g[RA][0] as u32;
                                                                c.g[SP][0] = sx((c.g[SP][0]
                                                                    as u32)
                                                                    .wrapping_add(0xd0));
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
                            }
                        }
                    }
                }
            }
        }
    });
}

/// `void BE_ApplyDefaultCharProfileBolts(CharProfile* self)` (0x1513b8):
/// equip the character's default bolts: mark the table-4 prizes, set
/// flags 0x26 on the unlocked items, mark and equip the free (and the
/// `-1` priced) bolts, then update the item flags.
pub fn be_apply_default_char_profile_bolts(c: &mut Cpu) {
    const BE_SET_CHAR_PROFILE_BOLT_EQUIPPED: Callee = Callee {
        addr: 0x00151c90,
        f: ee::s01::f_00151c90,
    };
    const BOLTS_GET_CHAR_BOLTS: Callee = Callee {
        addr: 0x0014d998,
        f: ee::s01::f_0014d998,
    };
    const BOLTS_GET_CHAR_BOLT_COUNT: Callee = Callee {
        addr: 0x0014d988,
        f: ee::s01::f_0014d988,
    };
    const BOLTS_GET_CHAR_TABLE4: Callee = Callee {
        addr: 0x0014dc50,
        f: ee::s01::f_0014dc50,
    };
    const BOLTS_GET_CHAR_TABLE4_COUNT: Callee = Callee {
        addr: 0x0014dc40,
        f: ee::s01::f_0014dc40,
    };
    guest(c, |c| {
        'b_151424: {
            c.cycles += 19;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.g[V0][0] = 0x4a0000;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S2][0] = c.g[A0][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6750));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0xbc0));
            BOLTS_GET_CHAR_TABLE4_COUNT.call(c, 0x001513ec)?;
            c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0xbc0));
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S3][0];
            BOLTS_GET_CHAR_TABLE4.call(c, 0x001513fc)?;
            let t = (c.g[S0][0] as i64) <= 0;
            c.g[S1][0] = c.g[V0][0];
            if t {
                break 'b_151424;
            } else {
                c.cycles += 1;
                'l_151408: loop {
                    c.cycles += 7;
                    c.g[A1][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x6));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A2][0] = 0x1;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    BE_SET_CHAR_PROFILE_BOLT_EQUIPPED.call(c, 0x0015141c)?;
                    let t = c.g[S0][0] != 0u64;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00151408);
                        }
                        continue 'l_151408;
                    } else {
                        break 'b_151424;
                    }
                }
            }
        }
        'b_151460: {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x28c));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[A1][0] = 0u64;
            if t {
                break 'b_151460;
            } else {
                c.cycles += 2;
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x290));
                'l_151438: loop {
                    'b_15144c: {
                        c.cycles += 4;
                        c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x2));
                        c.g[V0][0] = c.g[V1][0] & 0x10;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = c.g[V1][0] | 0x26;
                        if t {
                            break 'b_15144c;
                        } else {
                            c.cycles += 1;
                            c.sh((c.g[A0][0] as u32).wrapping_add(0x2), c.g[V0][0]);
                            break 'b_15144c;
                        }
                    }
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x28c));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00151438);
                        }
                        continue 'l_151438;
                    } else {
                        break 'b_151460;
                    }
                }
            }
        }
        'b_15154c: {
            c.cycles += 10;
            c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0xbc0));
            c.g[A0][0] = c.g[S3][0];
            BOLTS_GET_CHAR_BOLT_COUNT.call(c, 0x0015146c)?;
            c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0xbc0));
            c.g[A0][0] = c.g[S3][0];
            c.g[S1][0] = c.g[V0][0];
            BOLTS_GET_CHAR_BOLTS.call(c, 0x0015147c)?;
            c.g[S0][0] = c.g[V0][0];
            let t = (c.g[S1][0] as i64) <= 0;
            c.g[A0][0] = 0u64;
            if t {
                break 'b_15154c;
            } else {
                c.cycles += 2;
                c.g[S3][0] = 0xa;
                c.g[S4][0] = 0xffffffffffffffff;
                'l_151490: loop {
                    'b_1514a0: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0xe));
                        let t = (c.g[V0][0] as i64) <= 0;
                        c.g[V1][0] = c.g[V0][0];
                        if t {
                            break 'b_1514a0;
                        } else {
                            c.cycles += 1;
                            c.mult(0, 3, 2, 19);
                            break 'b_1514a0;
                        }
                    }
                    'b_151544: {
                        'b_151540: {
                            'b_1514e4: {
                                c.cycles += 2;
                                let t = c.g[V1][0] != 0u64;

                                if t {
                                    break 'b_1514e4;
                                } else {
                                    'b_1514d4: {
                                        'b_1514d0: {
                                            c.cycles += 7;
                                            c.g[V0][0] =
                                                c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                                            c.g[V1][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x288));
                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[V1][0] as u32));
                                            c.g[V0][0] = c.lh(c.g[V0][0] as u32);
                                            let t = (c.g[V0][0] as i64) < 0;
                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                            if t {
                                                break 'b_1514d0;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x290));
                                                let t = true;
                                                c.g[V1][0] = sx((c.g[S2][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                if t {
                                                    break 'b_1514d4;
                                                } else {
                                                    break 'b_1514d0;
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V1][0] = 0u64;
                                        break 'b_1514d4;
                                    }
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2));
                                    c.g[V0][0] = c.g[V0][0] | 0x2;
                                    let t = true;
                                    c.sh((c.g[V1][0] as u32).wrapping_add(0x2), c.g[V0][0]);
                                    if t {
                                        break 'b_151540;
                                    } else {
                                        break 'b_1514e4;
                                    }
                                }
                            }
                            'b_1514ec: {
                                c.cycles += 2;
                                if (c.g[V0][0] as i64) > 0 {
                                    c.mult(0, 2, 2, 19);
                                    break 'b_1514ec;
                                } else {
                                    break 'b_1514ec;
                                }
                            }
                            c.cycles += 2;
                            if c.g[V0][0] != c.g[S4][0] {
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                                break 'b_151544;
                            } else {
                                'b_151520: {
                                    'b_15151c: {
                                        c.cycles += 7;
                                        c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x288));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                        c.g[V0][0] = c.lh(c.g[V0][0] as u32);
                                        let t = (c.g[V0][0] as i64) < 0;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                        if t {
                                            break 'b_15151c;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_add(0x290));
                                            let t = true;
                                            c.g[V1][0] =
                                                sx((c.g[S2][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                            if t {
                                                break 'b_151520;
                                            } else {
                                                break 'b_15151c;
                                            }
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[V1][0] = 0u64;
                                    break 'b_151520;
                                }
                                c.cycles += 8;
                                c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2));
                                c.g[A0][0] = c.g[S2][0];
                                c.g[A2][0] = 0x1;
                                c.g[V0][0] = c.g[V0][0] | 0x2;
                                c.sh((c.g[V1][0] as u32).wrapping_add(0x2), c.g[V0][0]);
                                c.g[A1][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                                BE_SET_CHAR_PROFILE_BOLT_EQUIPPED.call(c, 0x0015153c)?;
                                c.g[A0][0] = 0x1;
                                break 'b_151540;
                            }
                        }
                        c.cycles += 1;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                        break 'b_151544;
                    }
                    c.cycles += 2;
                    let t = c.g[S1][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x38));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00151490);
                        }
                        continue 'l_151490;
                    } else {
                        break 'b_15154c;
                    }
                }
            }
        }
        'b_151598: {
            c.cycles += 2;
            if c.g[A0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                break 'b_151598;
            } else {
                'b_151594: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x28c));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[A1][0] = 0u64;
                    if t {
                        break 'b_151594;
                    } else {
                        c.cycles += 2;
                        c.g[A2][0] = 0xfffffffffffffffb;
                        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x290));
                        'l_151568: loop {
                            'b_15157c: {
                                c.cycles += 4;
                                c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x2));
                                c.g[V0][0] = c.g[V1][0] & 0x10;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = c.g[V1][0] | 0x6;
                                if t {
                                    break 'b_15157c;
                                } else {
                                    c.cycles += 1;
                                    c.g[V0][0] = c.g[V1][0] & c.g[A2][0];
                                    break 'b_15157c;
                                }
                            }
                            c.cycles += 6;
                            c.sh((c.g[A0][0] as u32).wrapping_add(0x2), c.g[V0][0]);
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x28c));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00151568);
                                }
                                continue 'l_151568;
                            } else {
                                break 'b_151594;
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                break 'b_151598;
            }
        }
        c.cycles += 7;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        Ok(())
    });
}

/// `sh src, off(reg)`.
#[inline]
pub(crate) fn oc417_sh(c: &mut Cpu, reg: usize, off: u32, src: usize) {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    let v = c.g[src][0];
    Addr(a).sh(c, v);
}

/// `sh zero, off(reg)`.
#[inline]
pub(crate) fn oc417_sh0(c: &mut Cpu, reg: usize, off: u32) {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    Addr(a).sh(c, 0);
}

// ---------------------------------------------------------------------------
// BE_InitCharProfileBoltMap (0x151a88)

/// `Bolts_GetCharBolts(db, kind)` (0x14d998).
pub(crate) const BOLT_MAP_GET_CHAR_BOLTS: Callee = Callee {
    addr: 0x0014_d998,
    f: ee::s01::Bolts_GetCharBolts,
};

/// `Bolts_GetCharBoltCount(db, kind)` (0x14d988).
pub(crate) const BOLT_MAP_GET_CHAR_BOLT_COUNT: Callee = Callee {
    addr: 0x0014_d988,
    f: ee::s01::Bolts_GetCharBoltCount,
};

/// `BE_ApplyDefaultCharProfileBolts(profile)` (0x1513b8).
pub(crate) const BOLT_MAP_APPLY_DEFAULT_BOLTS: Callee = Callee {
    addr: 0x0015_13b8,
    f: ee::s01::BE_ApplyDefaultCharProfileBolts,
};

/// The char-profile bolt table of a profile record: kind (byte at +0xbc0),
/// lookup (`short*` at +0x288), count (+0x28c), items (+0x290, 4 bytes: id,
/// flags); 525 item slots.
pub(crate) const BOLT_MAP_KIND: u32 = 0xbc0;

pub(crate) const BOLT_MAP_LOOKUP: u32 = 0x288;

pub(crate) const BOLT_MAP_COUNT: u32 = 0x28c;

pub(crate) const BOLT_MAP_ITEMS: u32 = 0x290;

pub(crate) const BOLT_MAP_SLOTS: i64 = 525;

/// `void BE_InitCharProfileBoltMap(profile, kind)` (0x151a88): fetch the
/// character's bolt list (`Bolts_GetCharBolts`/`Count` of the global bolt
/// database at `0x4a6750`), record `kind` and the count, allocate the
/// id -> item lookup (`gp-0x1f08` shorts, named "Profile_Bolts_Map") on
/// first use and fill it with -1, enter each bolt (flags 0, its id at
/// `+4` of the 0x38-byte record) as item i with `lookup[id] = i`, mark the
/// remaining slots up to 525 unused (id -1, flags 0), then apply the default
/// bolts.
pub fn be_init_char_profile_bolt_map(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = 0x4a_0000;
        frame.save(c, S3, 0x10);
        c.g[S0][0] = oc417_add(c.g[S0][0], 0x6750);
        c.g[S3][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        BOLT_MAP_GET_CHAR_BOLTS.call(c, 0x0015_1ab8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[S2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        BOLT_MAP_GET_CHAR_BOLT_COUNT.call(c, 0x0015_1ac8)?;
        c.g[S0][0] = c.g[V0][0];
        let kind_addr = (c.g[S3][0] as u32).wrapping_add(BOLT_MAP_KIND);
        c.sb(kind_addr, c.g[S1][0]);
        c.g[V0][0] = oc417_lw(c, S3, BOLT_MAP_LOOKUP);
        let have_lookup = c.g[V0][0] != 0;
        oc417_sw(c, S3, BOLT_MAP_COUNT, S0);
        if !have_lookup {
            c.cycles += 8;
            c.g[A0][0] = oc417_lw(c, GP, 0u32.wrapping_sub(0x1f08));
            c.g[A1][0] = 0x46_0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x5858));
            c.g[A2][0] = 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7e30, 0x0015_1af8)?;
            oc417_sw(c, S3, BOLT_MAP_LOOKUP, V0);
        }
        // lookup[0..gp-0x1f08] = -1.
        c.cycles += 3;
        c.g[V0][0] = oc417_lw(c, GP, 0u32.wrapping_sub(0x1f08));
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[A0][0] = 0;
        if !empty {
            c.cycles += 2;
            c.g[A1][0] = u64::MAX;
            c.g[V0][0] = oc417_lw(c, S3, BOLT_MAP_LOOKUP);
            loop {
                c.cycles += 8;
                c.g[V1][0] = sx((c.g[A0][0] as u32) << 1);
                c.g[A0][0] = oc417_add(c.g[A0][0], 1);
                c.g[V1][0] = oc417_add(c.g[V1][0], c.g[V0][0]);
                let at = c.g[V1][0] as u32;
                c.sh(at, c.g[A1][0]);
                c.g[V0][0] = oc417_lw(c, GP, 0u32.wrapping_sub(0x1f08));
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = oc417_lw(c, S3, BOLT_MAP_LOOKUP);
                    if event_due(c) {
                        deliver_event_here(c, 0x0015_1b10);
                    }
                    continue;
                }
                break;
            }
        }
        // Items 0..count from the bolt list.
        c.cycles += 2;
        let no_bolts = (c.g[S0][0] as i64) <= 0;
        c.g[A0][0] = 0;
        if !no_bolts {
            c.cycles += 2;
            c.g[A1][0] = oc417_add(c.g[S3][0], BOLT_MAP_ITEMS as u64);
            loop {
                c.cycles += 13;
                oc417_sh0(c, A1, 2);
                c.g[V0][0] = oc417_lh(c, S2, 4);
                oc417_sh(c, A1, 0, V0);
                c.g[A1][0] = oc417_add(c.g[A1][0], 4);
                c.g[V0][0] = oc417_lh(c, S2, 4);
                c.g[V1][0] = oc417_lw(c, S3, BOLT_MAP_LOOKUP);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                c.g[V0][0] = oc417_add(c.g[V0][0], c.g[V1][0]);
                oc417_sh(c, V0, 0, A0);
                c.g[A0][0] = oc417_add(c.g[A0][0], 1);
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[S0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = oc417_add(c.g[S2][0], 0x38);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0015_1b40);
                    }
                    continue;
                }
                break;
            }
        }
        // Unused slots count..525.
        c.cycles += 4;
        c.g[A0][0] = c.g[S0][0];
        c.g[V0][0] = ((c.g[A0][0] as i64) < BOLT_MAP_SLOTS) as u64;
        let full = c.g[V0][0] == 0;
        c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
        if !full {
            c.cycles += 3;
            c.g[A1][0] = u64::MAX;
            c.g[V0][0] = oc417_add(c.g[V0][0], BOLT_MAP_ITEMS as u64);
            c.g[V1][0] = oc417_add(c.g[V0][0], c.g[S3][0]);
            loop {
                c.cycles += 7;
                oc417_sh(c, V1, 0, A1);
                c.g[A0][0] = oc417_add(c.g[A0][0], 1);
                oc417_sh0(c, V1, 2);
                c.g[V0][0] = ((c.g[A0][0] as i64) < BOLT_MAP_SLOTS) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = oc417_add(c.g[V1][0], 4);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0015_1b90);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 9;
        c.g[A0][0] = c.g[S3][0];
        BOLT_MAP_APPLY_DEFAULT_BOLTS.call(c, 0x0015_1bb4)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int BE_TestCharProfileBoltFlags(self, id, mask)` (0x152368): whether every
/// bit of `mask` is set in the 16-bit flags of bolt map entry `id`. The map
/// (at self+0x288) gives a halfword index per id, the flags sit at
/// `self + 4 * index + 0x292`. Returns 1 when `(~flags & mask) == 0`.
pub fn be_test_char_profile_bolt_flags(c: &mut Cpu) {
    c.cycles += 11;
    let this = c.g[A0][0] as u32;
    c.g[V0][0] = Addr(this).off(0x288).lw(c);
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 1);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = Addr(c.g[A1][0] as u32).lh(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(this));
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x292)).lhu(c);
    c.g[V0][0] = !c.g[V0][0];
    c.g[V0][0] &= c.g[A2][0];
    c.g[V0][0] = (c.g[V0][0] < 1) as u64;
    abi::ret(c);
}

/// `int BEEconInt_BuyAttribute(self, int human, int character, int attribute)`: buys one level
/// of a stat (speed, accel, trick, edging, spin, toughness, stability) if the character has
/// the money (`GetAttributeCost(level/5 - 1)`) and the stat is not at 55: deducts the money,
/// raises the stat byte and notifies the stat interface (virtual call). Returns 1 on a
/// purchase, else 0. Resumable (jump table): any other `pc` resumes in the generated body.
pub fn be_econ_int_buy_attribute(c: &mut Cpu) {
    if c.pc != 0x00150c20 {
        // resume after an unwound callee or event: the generated body (at install: `__generated`)
        return be_econ_int_buy_attribute_resume(c);
    }
    guest(c, |c| {
        'b_150e30: {
            'b_150de4: {
                'b_150ddc: {
                    c.cycles += 12;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                    c.g[T1][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S1][0] = c.g[A1][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S2][0] = c.g[A2][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
                    let t = c.g[V0][0] == 0u64;
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    if t {
                        break 'b_150ddc;
                    } else {
                        'b_150d98: {
                            c.cycles += 13;
                            c.g[V0][0] = 0xf88;
                            c.g[A0][0] = 0x9b50;
                            c.mult(0, 2, 18, 2);
                            c.mult(0, 4, 17, 4);
                            c.g[V1][0] = 0x4a0000;
                            c.g[S0][0] = 0u64;
                            c.g[S3][0] = c.g[V1][0];
                            c.g[A1][0] = (c.g[A3][0] < 0x7u64) as u64;
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ca8));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            let t = c.g[A1][0] == 0u64;
                            c.g[S4][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xac4));
                            if t {
                                break 'b_150d98;
                            } else {
                                'b_150d78: {
                                    'b_150d54: {
                                        'b_150d30: {
                                            'b_150d0c: {
                                                'b_150ce8: {
                                                    'b_150cc4: {
                                                        c.cycles += 7;
                                                        c.g[V0][0] = 0x460000;
                                                        c.g[V1][0] = sx((c.g[A3][0] as u32) << 2);
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_sub(0x58d0));
                                                        c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                            .wrapping_add(c.g[V0][0] as u32));
                                                        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                                        let t = c.g[A0][0] as u32;
                                                        match t {
                                                            0x00150ca0 => {
                                                                c.cycles += 9;
                                                                c.g[V0][0] = 0xf88;
                                                                c.g[V1][0] = 0x9b50;
                                                                c.mult(0, 2, 18, 2);
                                                                c.mult(0, 3, 17, 3);
                                                                c.g[A0][0] = sx((c.g[S3][0]
                                                                    as u32)
                                                                    .wrapping_add(0x6ca8));
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[A0][0] as u32,
                                                                    ));
                                                                c.g[V1][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[V0][0] as u32,
                                                                    ));
                                                                let t = true;
                                                                c.g[S0][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(0xbdf));
                                                                if t {
                                                                    break 'b_150d98;
                                                                } else {
                                                                    break 'b_150cc4;
                                                                }
                                                            }
                                                            0x00150cc4 => {
                                                                break 'b_150cc4;
                                                            }
                                                            0x00150ce8 => {
                                                                break 'b_150ce8;
                                                            }
                                                            0x00150d0c => {
                                                                break 'b_150d0c;
                                                            }
                                                            0x00150d30 => {
                                                                break 'b_150d30;
                                                            }
                                                            0x00150d54 => {
                                                                break 'b_150d54;
                                                            }
                                                            0x00150d78 => {
                                                                break 'b_150d78;
                                                            }
                                                            _ => {
                                                                c.jump_out(
                                                                    t, 0x00150c20, 0x00150e50,
                                                                );
                                                                return Ok(());
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 9;
                                                    c.g[V0][0] = 0xf88;
                                                    c.g[V1][0] = 0x9b50;
                                                    c.mult(0, 2, 18, 2);
                                                    c.mult(0, 3, 17, 3);
                                                    c.g[A0][0] =
                                                        sx((c.g[S3][0] as u32)
                                                            .wrapping_add(0x6ca8));
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add(c.g[V0][0] as u32));
                                                    let t = true;
                                                    c.g[S0][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0xbe0));
                                                    if t {
                                                        break 'b_150d98;
                                                    } else {
                                                        break 'b_150ce8;
                                                    }
                                                }
                                                c.cycles += 9;
                                                c.g[V0][0] = 0xf88;
                                                c.g[V1][0] = 0x9b50;
                                                c.mult(0, 2, 18, 2);
                                                c.mult(0, 3, 17, 3);
                                                c.g[A0][0] =
                                                    sx((c.g[S3][0] as u32).wrapping_add(0x6ca8));
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                let t = true;
                                                c.g[S0][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0xbe1));
                                                if t {
                                                    break 'b_150d98;
                                                } else {
                                                    break 'b_150d0c;
                                                }
                                            }
                                            c.cycles += 9;
                                            c.g[V0][0] = 0xf88;
                                            c.g[V1][0] = 0x9b50;
                                            c.mult(0, 2, 18, 2);
                                            c.mult(0, 3, 17, 3);
                                            c.g[A0][0] =
                                                sx((c.g[S3][0] as u32).wrapping_add(0x6ca8));
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            c.g[V1][0] =
                                                sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                            let t = true;
                                            c.g[S0][0] =
                                                sx((c.g[V1][0] as u32).wrapping_add(0xbe2));
                                            if t {
                                                break 'b_150d98;
                                            } else {
                                                break 'b_150d30;
                                            }
                                        }
                                        c.cycles += 9;
                                        c.g[V0][0] = 0xf88;
                                        c.g[V1][0] = 0x9b50;
                                        c.mult(0, 2, 18, 2);
                                        c.mult(0, 3, 17, 3);
                                        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6ca8));
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        let t = true;
                                        c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xbe3));
                                        if t {
                                            break 'b_150d98;
                                        } else {
                                            break 'b_150d54;
                                        }
                                    }
                                    c.cycles += 9;
                                    c.g[V0][0] = 0xf88;
                                    c.g[V1][0] = 0x9b50;
                                    c.mult(0, 2, 18, 2);
                                    c.mult(0, 3, 17, 3);
                                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6ca8));
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    let t = true;
                                    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xbe4));
                                    if t {
                                        break 'b_150d98;
                                    } else {
                                        break 'b_150d78;
                                    }
                                }
                                c.cycles += 8;
                                c.g[V0][0] = 0xf88;
                                c.g[V1][0] = 0x9b50;
                                c.mult(0, 2, 18, 2);
                                c.mult(0, 3, 17, 3);
                                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6ca8));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xbe5));
                                break 'b_150d98;
                            }
                        }
                        'b_150da8: {
                            c.cycles += 4;
                            c.g[A1][0] = c.lb(c.g[S0][0] as u32);
                            c.g[V0][0] = 0x5;
                            if c.g[V0][0] == 0u64 {
                                break 'b_150da8;
                            } else {
                                break 'b_150da8;
                            }
                        }
                        c.cycles += 9;
                        c.g[A0][0] = c.g[T1][0];
                        c.div(0, 5, 2);
                        c.g[A1][0] = c.lo[0];
                        c.g[RA][0] = sx(0x00150dbc);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                        Callee {
                            addr: 0x00150e50,
                            f: ee::s01::BEEconInt_GetAttributeCost,
                        }
                        .call(c, 0x00150dbc)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[A1][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = 0u64;
                        if t {
                            break 'b_150e30;
                        } else {
                            c.cycles += 4;
                            c.g[V1][0] = c.lb(c.g[S0][0] as u32);
                            c.g[V0][0] = 0x37;
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[A0][0] = 0xf88;
                            if t {
                                break 'b_150de4;
                            } else {
                                break 'b_150ddc;
                            }
                        }
                    }
                }
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0u64;
                if t {
                    break 'b_150e30;
                } else {
                    break 'b_150de4;
                }
            }
            c.cycles += 19;
            c.g[V0][0] = 0x9b50;
            c.mult(0, 2, 17, 2);
            c.mult(0, 4, 18, 4);
            c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6ca8));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xac4));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A1][0] as u32));
            c.sw((c.g[A0][0] as u32).wrapping_add(0xac4), c.g[V0][0]);
            c.g[V1][0] = c.lbu(c.g[S0][0] as u32);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
            c.g[RA][0] = sx(0x00150e18);
            c.sb(c.g[S0][0] as u32, c.g[V1][0]);
            Callee {
                addr: 0x00147f78,
                f: ee::s01::BECharInt_GetInstance,
            }
            .call(c, 0x00150e18)?;
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
            let t = c.g[A1][0] as u32;
            c.g[RA][0] = sx(0x00150e2c);
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x00150e2c)?;
            c.g[V0][0] = 0x1;
            break 'b_150e30;
        }
        c.cycles += 8;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    });
}

/// Native resume of `be_econ_int_buy_attribute` (converted from the `entries` body): any entry
/// other than 0x00150c20, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn be_econ_int_buy_attribute_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00150c20 => {
                    c.cycles += 12;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                    c.g[T1][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S1][0] = c.g[A1][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S2][0] = c.g[A2][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
                    let t = c.g[V0][0] == 0u64;
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    if t {
                        pc = 0x00150ddc;
                        continue;
                    }
                    pc = 0x00150c50;
                    continue;
                }
                0x00150c50 => {
                    c.cycles += 13;
                    c.g[V0][0] = 0xf88;
                    c.g[A0][0] = 0x9b50;
                    c.mult(0, 2, 18, 2);
                    c.mult(0, 4, 17, 4);
                    c.g[V1][0] = 0x4a0000;
                    c.g[S0][0] = 0u64;
                    c.g[S3][0] = c.g[V1][0];
                    c.g[A1][0] = (c.g[A3][0] < 0x7u64) as u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ca8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    let t = c.g[A1][0] == 0u64;
                    c.g[S4][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xac4));
                    if t {
                        pc = 0x00150d98;
                        continue;
                    }
                    pc = 0x00150c84;
                    continue;
                }
                0x00150c84 => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x460000;
                    c.g[V1][0] = sx((c.g[A3][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x58d0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x00150ca0 => {
                    c.cycles += 9;
                    c.g[V0][0] = 0xf88;
                    c.g[V1][0] = 0x9b50;
                    c.mult(0, 2, 18, 2);
                    c.mult(0, 3, 17, 3);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6ca8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    let t = true;
                    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xbdf));
                    if t {
                        pc = 0x00150d98;
                        continue;
                    }
                    pc = 0x00150cc4;
                    continue;
                }
                0x00150cc4 => {
                    c.cycles += 9;
                    c.g[V0][0] = 0xf88;
                    c.g[V1][0] = 0x9b50;
                    c.mult(0, 2, 18, 2);
                    c.mult(0, 3, 17, 3);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6ca8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    let t = true;
                    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xbe0));
                    if t {
                        pc = 0x00150d98;
                        continue;
                    }
                    pc = 0x00150ce8;
                    continue;
                }
                0x00150ce8 => {
                    c.cycles += 9;
                    c.g[V0][0] = 0xf88;
                    c.g[V1][0] = 0x9b50;
                    c.mult(0, 2, 18, 2);
                    c.mult(0, 3, 17, 3);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6ca8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    let t = true;
                    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xbe1));
                    if t {
                        pc = 0x00150d98;
                        continue;
                    }
                    pc = 0x00150d0c;
                    continue;
                }
                0x00150d0c => {
                    c.cycles += 9;
                    c.g[V0][0] = 0xf88;
                    c.g[V1][0] = 0x9b50;
                    c.mult(0, 2, 18, 2);
                    c.mult(0, 3, 17, 3);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6ca8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    let t = true;
                    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xbe2));
                    if t {
                        pc = 0x00150d98;
                        continue;
                    }
                    pc = 0x00150d30;
                    continue;
                }
                0x00150d30 => {
                    c.cycles += 9;
                    c.g[V0][0] = 0xf88;
                    c.g[V1][0] = 0x9b50;
                    c.mult(0, 2, 18, 2);
                    c.mult(0, 3, 17, 3);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6ca8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    let t = true;
                    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xbe3));
                    if t {
                        pc = 0x00150d98;
                        continue;
                    }
                    pc = 0x00150d54;
                    continue;
                }
                0x00150d54 => {
                    c.cycles += 9;
                    c.g[V0][0] = 0xf88;
                    c.g[V1][0] = 0x9b50;
                    c.mult(0, 2, 18, 2);
                    c.mult(0, 3, 17, 3);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6ca8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    let t = true;
                    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xbe4));
                    if t {
                        pc = 0x00150d98;
                        continue;
                    }
                    pc = 0x00150d78;
                    continue;
                }
                0x00150d78 => {
                    c.cycles += 8;
                    c.g[V0][0] = 0xf88;
                    c.g[V1][0] = 0x9b50;
                    c.mult(0, 2, 18, 2);
                    c.mult(0, 3, 17, 3);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6ca8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xbe5));
                    pc = 0x00150d98;
                    continue;
                }
                0x00150d98 => {
                    c.cycles += 4;
                    c.g[A1][0] = c.lb((c.g[S0][0] as u32));
                    c.g[V0][0] = 0x5;

                    if c.g[V0][0] == 0u64 {
                        pc = 0x00150da8;
                        continue;
                    }
                    pc = 0x00150da8;
                    continue;
                }
                0x00150da8 => {
                    c.cycles += 9;
                    c.g[A0][0] = c.g[T1][0];
                    c.div(0, 5, 2);
                    c.g[A1][0] = c.lo[0];
                    c.g[RA][0] = sx(0x00150dbc);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    Callee {
                        addr: 0x00150e50,
                        f: ee::s01::BEEconInt_GetAttributeCost,
                    }
                    .call(c, 0x00150dbc)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00150e30;
                        continue;
                    }
                    pc = 0x00150dcc;
                    continue;
                }
                0x00150dcc => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lb((c.g[S0][0] as u32));
                    c.g[V0][0] = 0x37;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = 0xf88;
                    if t {
                        pc = 0x00150de4;
                        continue;
                    }
                    pc = 0x00150ddc;
                    continue;
                }
                0x00150ddc => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00150e30;
                        continue;
                    }
                    pc = 0x00150de4;
                    continue;
                }
                0x00150de4 => {
                    c.cycles += 19;
                    c.g[V0][0] = 0x9b50;
                    c.mult(0, 2, 17, 2);
                    c.mult(0, 4, 18, 4);
                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6ca8));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xac4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[A1][0] as u32)));
                    c.sw((c.g[A0][0] as u32).wrapping_add(0xac4), c.g[V0][0]);
                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.g[RA][0] = sx(0x00150e18);
                    c.sb((c.g[S0][0] as u32), c.g[V1][0]);
                    Callee {
                        addr: 0x00147f78,
                        f: ee::s01::BECharInt_GetInstance,
                    }
                    .call(c, 0x00150e18)?;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                    let t = c.g[A1][0] as u32;
                    c.g[RA][0] = sx(0x00150e2c);
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x00150e2c)?;
                    c.g[V0][0] = 0x1;
                    pc = 0x00150e30;
                    continue;
                }
                0x00150e30 => {
                    c.cycles += 8;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00150c20, 0x00150e50);
                    return Ok(());
                }
            }
        }
    });
}

/// `int BERewardInt_GetEventRewardPoints(self, int medal, int key, int place)`: the prize
/// money of an event: for places 6..11 the race interface's special event score, else the
/// per-medal prize of the event `key` in the table at 0x4405a0 (times 100).
pub fn be_reward_int_get_event_reward_points(c: &mut Cpu) {
    guest(c, |c| {
        'b_151164: {
            'b_151160: {
                'b_15115c: {
                    'b_1510e0: {
                        c.cycles += 17;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[S0][0] = c.g[A1][0];
                        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[S1][0] = c.g[A3][0];
                        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                        c.g[RA][0] = sx(0x00151064);
                        c.g[S2][0] = c.g[A2][0];
                        Callee {
                            addr: 0x0014dc80,
                            f: ee::s01::BELibrary_GetInstance,
                        }
                        .call(c, 0x00151064)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x00151070);
                        c.g[A1][0] = 0u64;
                        Callee {
                            addr: 0x0014dd58,
                            f: ee::s01::BELibrary_GetInterface,
                        }
                        .call(c, 0x00151070)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x6));
                        c.g[V0][0] = (c.g[V0][0] < 0x6u64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A1][0] = 0u64;
                        if t {
                            break 'b_1510e0;
                        } else {
                            'b_1510d0: {
                                'b_1510c8: {
                                    'b_1510bc: {
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x2;
                                        let t = c.g[S0][0] == c.g[V0][0];
                                        c.g[V0][0] = ((c.g[S0][0] as i64) < 3i64) as u64;
                                        if t {
                                            break 'b_1510bc;
                                        } else {
                                            'b_1510a8: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[V0][0] = 0x3;
                                                if t {
                                                    break 'b_1510a8;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[S0][0] == c.g[V0][0];
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x30),
                                                    );
                                                    if t {
                                                        break 'b_1510c8;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.g[V0][0] = c.g[A1][0];
                                                        if t {
                                                            break 'b_151164;
                                                        } else {
                                                            break 'b_1510a8;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            let t = (c.g[S0][0] as i64) < 0;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                            if t {
                                                break 'b_151160;
                                            } else {
                                                c.cycles += 3;
                                                c.g[A1][0] = c.g[S1][0];
                                                let t = true;
                                                c.g[A2][0] = 0x2;
                                                if t {
                                                    break 'b_1510d0;
                                                } else {
                                                    break 'b_1510bc;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[A1][0] = c.g[S1][0];
                                    let t = true;
                                    c.g[A2][0] = 0x1;
                                    if t {
                                        break 'b_1510d0;
                                    } else {
                                        break 'b_1510c8;
                                    }
                                }
                                c.cycles += 2;
                                c.g[A1][0] = c.g[S1][0];
                                c.g[A2][0] = 0u64;
                                break 'b_1510d0;
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001510d8);
                            Callee {
                                addr: 0x00145630,
                                f: ee::s01::BENewRaceInt_GetSpecialEventScore,
                            }
                            .call(c, 0x001510d8)?;
                            let t = true;
                            c.g[A1][0] = c.g[V0][0];
                            if t {
                                break 'b_15115c;
                            } else {
                                break 'b_1510e0;
                            }
                        }
                    }
                    c.cycles += 10;
                    c.g[V0][0] = 0x440000;
                    c.g[T2][0] = 0x1;
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5a0));
                    c.g[A2][0] = ((c.g[S0][0] as i64) < 2i64) as u64;
                    c.g[T1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x154));
                    c.g[T0][0] = 0x2;
                    c.g[A3][0] = 0x3;
                    c.g[A0][0] = 0x64;
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    'l_151108: loop {
                        'b_151150: {
                            c.cycles += 2;
                            if c.g[V0][0] != c.g[S2][0] {
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x14));
                                break 'b_151150;
                            } else {
                                'b_151148: {
                                    c.cycles += 2;
                                    if c.g[S0][0] == c.g[T2][0] {
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                                        break 'b_151148;
                                    } else {
                                        'b_151130: {
                                            c.cycles += 2;
                                            let t = c.g[A2][0] == 0u64;

                                            if t {
                                                break 'b_151130;
                                            } else {
                                                c.cycles += 2;
                                                if c.g[S0][0] == 0u64 {
                                                    c.g[V0][0] =
                                                        c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                                                    break 'b_151148;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0x14));
                                                    if t {
                                                        break 'b_151150;
                                                    } else {
                                                        break 'b_151130;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        if c.g[S0][0] == c.g[T0][0] {
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                            break 'b_151148;
                                        } else {
                                            c.cycles += 2;
                                            if c.g[S0][0] == c.g[A3][0] {
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                                                break 'b_151148;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[V1][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x14));
                                                if t {
                                                    break 'b_151150;
                                                } else {
                                                    break 'b_151148;
                                                }
                                            }
                                        }
                                    }
                                }
                                c.cycles += 2;
                                c.mult(0, 5, 2, 4);
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x14));
                                break 'b_151150;
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[T1][0] as i64)) as u64;
                        if c.g[V0][0] != 0u64 {
                            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                            if event_due(c) {
                                deliver_event_here(c, 0x00151108);
                            }
                            continue 'l_151108;
                        } else {
                            break 'b_15115c;
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_151160;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[A1][0];
            break 'b_151164;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `int BE_ValidateExclusiveBolts(profile, int* changed)`: walks the character's bolt
/// definitions; every exclusive one (flag 0x1000) whose owned item has flag 0x10 is
/// re-applied through `BE_ApplyBoltDependencies`. Sets `*changed` and returns 0 as soon as
/// a re-apply fails, else 1.
pub fn be_validate_exclusive_bolts(c: &mut Cpu) {
    guest(c, |c| {
        'b_1521c8: {
            'b_1521c4: {
                c.cycles += 23;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S1][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S4][0] = c.g[A1][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[RA][0] = sx(0x00152118);
                c.g[S2][0] = 0u64;
                Callee {
                    addr: 0x0014bdb8,
                    f: ee::s01::Bolts_GetDB,
                }
                .call(c, 0x00152118)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0xbc0));
                c.g[RA][0] = sx(0x00152128);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x0014d988,
                    f: ee::s01::Bolts_GetCharBoltCount,
                }
                .call(c, 0x00152128)?;
                c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0xbc0));
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00152138);
                c.g[S3][0] = c.g[V0][0];
                Callee {
                    addr: 0x0014d998,
                    f: ee::s01::Bolts_GetCharBolts,
                }
                .call(c, 0x00152138)?;
                c.g[S0][0] = c.g[V0][0];
                let t = (c.g[S3][0] as i64) <= 0;
                c.sw(c.g[S4][0] as u32, 0u64);
                if t {
                    break 'b_1521c4;
                } else {
                    c.cycles += 1;
                    c.g[S5][0] = 0x1;
                    'l_152148: loop {
                        'b_1521b8: {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
                            c.g[V0][0] = c.g[V0][0] & 0x1000;
                            if c.g[V0][0] == 0u64 {
                                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                break 'b_1521b8;
                            } else {
                                'b_152184: {
                                    'b_152180: {
                                        c.cycles += 7;
                                        c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x288));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                        c.g[V0][0] = c.lh(c.g[V0][0] as u32);
                                        let t = (c.g[V0][0] as i64) < 0;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                        if t {
                                            break 'b_152180;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_add(0x290));
                                            let t = true;
                                            c.g[V0][0] =
                                                sx((c.g[S1][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                            if t {
                                                break 'b_152184;
                                            } else {
                                                break 'b_152180;
                                            }
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[V0][0] = 0u64;
                                    break 'b_152184;
                                }
                                'b_1521b4: {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x2));
                                    c.g[V0][0] = c.g[V0][0] & 0x10;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A0][0] = c.g[S1][0];
                                    if t {
                                        break 'b_1521b4;
                                    } else {
                                        c.cycles += 6;
                                        c.sw(c.g[S4][0] as u32, c.g[S5][0]);
                                        c.g[A2][0] = 0x1;
                                        c.g[RA][0] = sx(0x001521a4);
                                        c.g[A1][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                                        Callee {
                                            addr: 0x00151ef0,
                                            f: ee::s01::BE_ApplyBoltDependencies,
                                        }
                                        .call(c, 0x001521a4)?;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                        if t {
                                            break 'b_1521b8;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[V0][0] = 0u64;
                                            if t {
                                                break 'b_1521c8;
                                            } else {
                                                break 'b_1521b4;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 1;
                                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                break 'b_1521b8;
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S3][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x38));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00152148);
                            }
                            continue 'l_152148;
                        } else {
                            break 'b_1521c4;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0x1;
            break 'b_1521c8;
        }
        c.cycles += 9;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
    });
}

/// `int BE_EnsureDefaultBoltsEquipped(profile)`: for each equipment group of the character,
/// if none of its bolts is equipped, equips the group's default bolt. Returns 0 as soon as
/// one equip fails, else 1.
pub fn be_ensure_default_bolts_equipped(c: &mut Cpu) {
    guest(c, |c| {
        'b_152318: {
            c.cycles += 23;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x8b0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x8a0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x890));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x880));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x870));
            c.g[S2][0] = 0u64;
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x860));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x850));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x840), c.g[RA][0]);
            c.g[RA][0] = sx(0x00152220);
            c.g[S5][0] = 0x1;
            Callee {
                addr: 0x0014bdb8,
                f: ee::s01::Bolts_GetDB,
            }
            .call(c, 0x00152220)?;
            c.g[S3][0] = c.g[V0][0];
            c.g[A1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0xbc0));
            c.g[RA][0] = sx(0x00152230);
            c.g[A0][0] = c.g[S3][0];
            Callee {
                addr: 0x0014dc50,
                f: ee::s01::Bolts_GetCharTable4,
            }
            .call(c, 0x00152230)?;
            c.g[A1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0xbc0));
            c.g[S1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00152240);
            c.g[A0][0] = c.g[S3][0];
            Callee {
                addr: 0x0014dc40,
                f: ee::s01::Bolts_GetCharTable4Count,
            }
            .call(c, 0x00152240)?;
            c.g[S4][0] = c.g[V0][0];
            let t = (c.g[S4][0] as i64) <= 0;
            c.g[V0][0] = c.g[S5][0];
            if t {
                break 'b_152318;
            } else {
                c.cycles += 1;
                'l_152250: loop {
                    'b_1522e0: {
                        c.cycles += 10;
                        c.g[A1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0xbc0));
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A2][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x4));
                        c.g[A3][0] = c.g[SP][0];
                        c.g[T0][0] = 0xffffffffffffffff;
                        c.g[RA][0] = sx(0x0015226c);
                        c.g[T1][0] = 0x1;
                        Callee {
                            addr: 0x0014d608,
                            f: ee::s01::Bolts_CollectChildren,
                        }
                        .call(c, 0x0015226c)?;
                        c.g[A2][0] = c.g[V0][0];
                        let t = (c.g[A2][0] as i64) <= 0;
                        c.g[A1][0] = 0u64;
                        if t {
                            break 'b_1522e0;
                        } else {
                            c.cycles += 2;
                            c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x288));
                            c.g[A0][0] = c.g[SP][0];
                            'l_152280: loop {
                                'b_1522ac: {
                                    'b_1522a8: {
                                        c.cycles += 7;
                                        c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                                        c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x4));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                                        c.g[V0][0] = c.lh(c.g[V0][0] as u32);
                                        let t = (c.g[V0][0] as i64) < 0;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                        if t {
                                            break 'b_1522a8;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_add(0x290));
                                            let t = true;
                                            c.g[V0][0] =
                                                sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                            if t {
                                                break 'b_1522ac;
                                            } else {
                                                break 'b_1522a8;
                                            }
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[V0][0] = 0u64;
                                    break 'b_1522ac;
                                }
                                'b_1522d4: {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x2));
                                    c.g[V0][0] = c.g[V0][0] & 0x10;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                        break 'b_1522d4;
                                    } else {
                                        c.cycles += 5;
                                        c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x34));
                                        c.g[V0][0] = c.g[V0][0] & 0x10;
                                        let t = c.g[V0][0] != 0u64;

                                        if t {
                                            break 'b_1522e0;
                                        } else {
                                            c.cycles += 1;
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                            break 'b_1522d4;
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00152280);
                                    }
                                    continue 'l_152280;
                                } else {
                                    break 'b_1522e0;
                                }
                            }
                        }
                    }
                    'b_152314: {
                        'b_152308: {
                            c.cycles += 2;
                            if c.g[A1][0] != c.g[A2][0] {
                                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                break 'b_152308;
                            } else {
                                c.cycles += 6;
                                c.g[A1][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x6));
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x001522f8);
                                c.g[A2][0] = 0x1;
                                Callee {
                                    addr: 0x00151c90,
                                    f: ee::s01::BE_SetCharProfileBoltEquipped,
                                }
                                .call(c, 0x001522f8)?;
                                let t = c.g[V0][0] != 0u64;
                                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                if t {
                                    break 'b_152308;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[S5][0] = 0u64;
                                    if t {
                                        break 'b_152314;
                                    } else {
                                        break 'b_152308;
                                    }
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S4][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00152250);
                            }
                            continue 'l_152250;
                        } else {
                            break 'b_152314;
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S5][0];
                    break 'b_152318;
                }
            }
        }
        c.cycles += 9;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x8a0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x890));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x880));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x870));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x860));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x850));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x840));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x8b0));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;

    use crate::difftest::fakes::machine;

    use crate::difftest::fill;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;

    pub(crate) use crate::game::be::beintmission::tests::instance_setup;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::base_8 as base;

    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_9 as base_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::wb;

    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_2;

    pub(crate) use crate::game::main::gamemode::tests::SEEDS;

    /// Address arithmetic shared by the economy tests.
    pub(crate) fn econ_base(rider: u32, ch: u32) -> u32 {
        0x4a6ca8u32
            .wrapping_add(rider.wrapping_mul(0x9b50))
            .wrapping_add(ch.wrapping_mul(0xf88))
    }

    #[test]
    fn be_econ_int_get_money_matches() {
        check_lohi(
            ssx3_ee::s01::f_00150928,
            be_econ_int_get_money,
            300,
            &|c, seed| {
                let mut r = Rng(0x5000_0006 ^ seed as u64);
                scramble(c, &mut r);
                let rider = seed % 3;
                let ch = seed % 4;
                c.g[A1][0] = rider as u64;
                c.g[A2][0] = ch as u64;
                c.bus.write32(econ_base(rider, ch) + 0xac4, r.next_u32());
            },
        );
    }

    #[test]
    fn be_econ_int_get_total_earned_matches() {
        check_lohi(
            ssx3_ee::s01::f_001509c0,
            be_econ_int_get_total_earned,
            300,
            &|c, seed| {
                let mut r = Rng(0x6000_0007 ^ seed as u64);
                scramble(c, &mut r);
                let rider = seed % 3;
                let ch = seed % 4;
                c.g[A1][0] = rider as u64;
                c.g[A2][0] = ch as u64;
                c.bus.write32(econ_base(rider, ch) + 0xac8, r.next_u32());
            },
        );
    }

    #[test]
    fn be_econ_int_add_money_matches() {
        check_lohi(
            ssx3_ee::s01::f_00150a90,
            be_econ_int_add_money,
            300,
            &|c, seed| {
                let mut r = Rng(0x7000_0008 ^ seed as u64);
                scramble(c, &mut r);
                let rider = seed % 3;
                let ch = seed % 4;
                c.g[A1][0] = rider as u64;
                c.g[A2][0] = ch as u64;
                c.g[A3][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                let base = econ_base(rider, ch);
                c.bus.write32(base + 0xac4, r.next_u32());
                c.bus.write32(base + 0xac8, r.next_u32());
            },
        );
    }

    #[test]
    fn be_econ_int_spend_money_matches() {
        check_lohi(
            ssx3_ee::s01::f_00150b48,
            be_econ_int_spend_money,
            300,
            &|c, seed| {
                let mut r = Rng(0x8000_0009 ^ seed as u64);
                scramble(c, &mut r);
                let rider = seed % 3;
                let ch = seed % 4;
                c.g[A1][0] = rider as u64;
                c.g[A2][0] = ch as u64;
                c.g[A3][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.bus.write32(econ_base(rider, ch) + 0xac4, r.next_u32());
            },
        );
    }

    #[test]
    fn be_econ_int_get_attribute_cost_matches() {
        check(
            ssx3_ee::s01::f_00150e50,
            be_econ_int_get_attribute_cost,
            100,
            &|c, seed| {
                let mut r = Rng(0x9000_000a ^ seed as u64);
                scramble(c, &mut r);
                let idx = seed % 4;
                c.g[A1][0] = idx as u64;
                c.bus.write32(0x440550 + idx * 8 + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn be_econ_int_get_instance_matches() {
        check(
            ssx3_ee::s01::f_001508d8,
            be_econ_int_get_instance,
            90,
            &|c, seed| {
                instance_setup(c, seed, 0x1edc, 0xec0);
            },
        );
    }

    #[test]
    fn be_econ_int_getters_match() {
        for (gen, hand, salt) in [
            (
                ssx3_ee::s01::f_00150960 as fn(&mut Cpu),
                be_econ_int_get_rider_money as fn(&mut Cpu),
                0x0960,
            ),
            (
                ssx3_ee::s01::f_001509f8,
                be_econ_int_get_rider_total_earned,
                0x09f8,
            ),
        ] {
            check(gen, hand, 40, &|c, seed| {
                let mut r = machine(c, seed, salt, 0);
                c.g[A1][0] = (seed % 8) as u64;
                fill(c, &mut r, 0x530000, 0x800);
            });
        }
    }

    #[test]
    fn be_econ_int_add_rider_money_matches() {
        check(
            ssx3_ee::s01::f_00150ae0,
            be_econ_int_add_rider_money,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0ae0, 0);
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A0][0] = (DATA + 0x400) as u64;
                fill(c, &mut r, 0x530000, 0x800);
            },
        );
    }

    // ------------------------------------------------------------------
    // BE rewards / missions.

    #[test]
    fn reward_get_collect_item_points_matches() {
        check(
            ssx3_ee::s01::f_00151178__generated,
            reward_get_collect_item_points,
            SEEDS,
            &|c, seed| {
                let levels: [u64; 8] = [0, 1, 2, 3, 9, u64::MAX, 0xffff_ffff, 0x8000_0000];
                c.g[A1][0] = levels[seed as usize % levels.len()];
            },
        );
    }

    #[test]
    fn reward_get_mission_points_matches() {
        check(
            ssx3_ee::s01::f_001511b0__generated,
            reward_get_mission_points,
            SEEDS,
            &|c, seed| {
                let levels: [u64; 8] = [0, 1, 2, 3, 4, u64::MAX, 0xffff_ffff, 0x8000_0000];
                c.g[A1][0] = levels[seed as usize % levels.len()];
            },
        );
    }

    #[test]
    fn be_restore_char_profile_equip_flags_matches() {
        check(
            ee::s01::f_001515b8__generated,
            be_restore_char_profile_equip_flags,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x1515b8, 0);
                let self_at = DATA + 0x200;
                c.g[A0][0] = self_at as u64;
                c.bus.write32(self_at + 0x28c, (seed % 4) as u32);
                for k in 0..3u32 {
                    let flags = match (seed + k) % 4 {
                        0 => 0x0004u16,
                        1 => 0x0010,
                        2 => 0x0014,
                        _ => 0x00ff,
                    };
                    c.bus.write16(self_at + 0x290 + 4 * k + 2, flags);
                }
            },
        );
    }

    #[test]
    fn be_set_char_profile_bolt_owned_matches() {
        check(
            ee::s01::f_00151c48__generated,
            be_set_char_profile_bolt_owned,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15_1c48, 0);
                let prof = DATA + 0x1000;
                let table = DATA + 0x2000;
                c.bus.write32(prof + 0x288, table);
                let id = (r.next_u32() % 8) as u32;
                for i in 0..8u32 {
                    let idx: i32 = match (seed as usize + i as usize) % 4 {
                        0 => -1,
                        1 => 0,
                        2 => (i as i32) * 3,
                        _ => 0x7fff,
                    };
                    c.bus.write16(table + 2 * i, idx as u16);
                }
                c.g[A0][0] = sx(prof);
                c.g[A1][0] = sx(id);
                c.g[A2][0] = (seed % 3) as u64;
            },
        );
    }

    // ---------------------------------------------------------- BE profile

    #[test]
    fn be_char_profile_dtor_matches() {
        check(
            ee::s01::f_00151368__generated,
            be_char_profile_dtor,
            60,
            &|c, seed| {
                base(c, seed, 0x1513);
                let flags = if seed % 4 == 0 { 1u32 } else { 0 };
                let obj = if flags == 1 { 0x100 } else { DATA };
                let owned = if seed % 3 == 0 {
                    0
                } else {
                    0x40 + 8 * (seed % 8)
                };
                c.bus.write32(obj + 0x288, owned);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = sx(flags);
            },
        );
    }

    #[test]
    fn be_set_char_state_flag_cheat_matches() {
        check_lohi(
            ee::s01::f_00150b88__generated,
            be_set_char_state_flag_cheat,
            SEEDS,
            &|c, seed| {
                let _ = base_2(c, seed, 0x0b88, 1);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A3][0] = (seed / 4 % 4) as u64;
                c.g[T0][0] = (seed & 1) as u64;
            },
        );
    }

    #[test]
    fn be_reset_char_rivalries_matches() {
        check(
            ee::s01::f_001519e0__generated,
            be_reset_char_rivalries,
            SEEDS_2,
            &|c, seed| {
                let _ = base_3(c, seed, 0x1519_e0, 4);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 10) as u64;
                // The character record the loop compares against (`v0` at
                // `0x4406fc + i*0xc`) and the second pointer it reads
                // (`0x440700 + i*0xc`), which decides where the loop stops.
                for i in 0..10u32 {
                    w(c, 0x0044_06fc + 0xc * i, 0);
                    w(c, 0x0044_0700 + 0xc * i, (seed % 9) + 1);
                }
                for i in 0..0x40u32 {
                    w(c, obj + 0xbc2 + 4 * i, 0);
                }
                wb(c, 0x004a_1218, 0);
                wb(c, 0x004a_1219, 1);
                wb(c, 0x004a_121a, 2);
            },
        );
    }

    /// A small bolt database (eight bolts, child lists that only point at
    /// higher ids, so the unequip recursion ends; no dependency links) and a
    /// profile with some bolts equipped or missing.
    #[test]
    fn be_set_char_profile_bolt_equipped_matches() {
        check(
            ee::s01::f_00151c90__generated,
            be_set_char_profile_bolt_equipped,
            300,
            &|c, seed| {
                let mut r = Rng(0x0396_0004 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0015_1c90;
                let db = 0x004a_6750;
                let kind = seed % 2;
                let (defs, idx, first, cnt) = (
                    DATA + 0x1_0000,
                    DATA + 0x1_1000,
                    DATA + 0x1_1100,
                    DATA + 0x1_1200,
                );
                c.bus.write32(db + 4, defs);
                c.bus.write32(db + 0x4dc + 4 * kind, idx);
                c.bus.write32(db + 0x194 + 4 * kind, first);
                c.bus.write32(db + 0x11c + 4 * kind, cnt);
                for id in 0..8u32 {
                    let def = if id == 7 && r.next_u32() % 4 == 0 {
                        0xffff
                    } else {
                        id
                    };
                    c.bus.write16(idx + 2 * id, def as u16);
                    let d = defs + 0x38 * id;
                    c.bus.write16(d + 6, (r.next_u32() % 8) as u16);
                    c.bus.write32(d + 0x34, r.next_u32() & 0x9);
                    // Children of list `id`: entries 8 + 2*id, others > id.
                    let n = if id == 7 { 0 } else { r.next_u32() % 3 };
                    c.bus.write16(first + 2 * id, (8 + 2 * id) as u16);
                    c.bus.write16(cnt + 2 * id, n as u16);
                    for k in 0..2 {
                        let e = defs + 0x38 * (8 + 2 * id + k);
                        let other = id + 1 + r.next_u32() % (7 - id).max(1);
                        c.bus.write16(e + 4, other.min(7) as u16);
                        c.bus.write32(e + 0x34, r.next_u32() & 0x8);
                    }
                }
                let prof = DATA + 0x1_2000;
                let lookup = DATA + 0x1_3000;
                c.bus.write8(prof + BOLT_EQUIP_KIND, kind as u8);
                c.bus.write32(prof + BOLT_EQUIP_LOOKUP, lookup);
                for id in 0..8u32 {
                    let j = if r.next_u32() % 6 == 0 { 0xffff } else { id };
                    c.bus.write16(lookup + 2 * id, j as u16);
                    let flags = (r.next_u32() & 0xffff) as u16 & !0x10
                        | if r.next_u32() % 2 == 0 { 0x10 } else { 0 };
                    c.bus.write16(prof + BOLT_EQUIP_ITEMS + 4 * id + 2, flags);
                }
                c.g[A0][0] = sx(prof);
                c.g[A1][0] = (r.next_u32() % 8) as u64;
                c.g[A2][0] = (seed / 2 % 2) as u64;
            },
        );
    }

    use crate::difftest::elf_image;
    use crate::difftest::fakes;

    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::input::inputparse::tests::o406_w;
    pub(crate) use crate::game::input::inputparse::tests::O406_BOLT_DB;

    pub(crate) use crate::game::input::inputparse::tests::o406_base;
    pub(crate) use crate::game::input::inputparse::tests::o406_results;

    pub(crate) use crate::game::ai::ai::tests::oc414_env;

    pub(crate) use crate::game::seg::seg_1fbe38::tests::FK_BOLTS;
    pub(crate) use crate::game::seg::seg_1fbe38::tests::FK_BOLT_COUNT;
    pub(crate) use crate::game::seg::seg_1fbe38::tests::FK_NEW_NAMED;

    pub(crate) use crate::game::ai::rider::tests::oc417_force;
    pub(crate) use crate::game::seg::seg_1fbe38::tests::oc417_machine;

    // ------------------------------------------ BE_ApplyBoltDependencies

    /// Switching a bolt on or off pulls its links (real `FindTable2Entries`
    /// over a small database): conditions, owned/unowned targets, the target
    /// being the bolt itself (flag set/clear) or another bolt switched
    /// through a fake; the failure exit.
    #[test]
    fn be_apply_bolt_dependencies_matches() {
        fakes::covered(
            ee::s01::f_00151ef0__generated,
            be_apply_bolt_dependencies,
            240,
            &|c, seed| {
                let mut r = o406_base(c, seed, 0x151ef0, 4, 0x0015_1ef0);
                let this = DATA + 0x2000;
                let index = DATA + 0x3000;
                let links = DATA + 0x1100;
                let kind = (seed % 2) as u32;
                c.bus.write8(this + 0xbc0, kind as u8);
                o406_w(c, this + 0x288, index);
                for id in 0..8u32 {
                    let slot = if r.next_u32() % 6 == 0 {
                        0xffff
                    } else {
                        r.next_u32() % 6
                    };
                    c.bus.write16(index + 2 * id, slot as u16);
                }
                for i in 0..6u32 {
                    c.bus.write16(this + 0x290 + 4 * i, r.next_u32() as u16 % 8);
                    c.bus
                        .write16(this + 0x290 + 4 * i + 2, (r.next_u32() & 0x10) as u16);
                }
                // The database: link table, per-kind start and count.
                o406_w(c, O406_BOLT_DB + 8, links);
                let count = 1 + r.next_u32() % 3;
                o406_w(c, O406_BOLT_DB + 0x20c + 4 * kind, count);
                o406_w(c, O406_BOLT_DB + 0x284 + 4 * kind, r.next_u32() % 2);
                let id = 1 + r.next_u32() % 5;
                let on = r.next_u32() % 2;
                for i in 0..8u32 {
                    let l = links + 0xc * i;
                    c.bus.write8(l + 1, on as u8);
                    c.bus.write16(l + 2, id as u16);
                    c.bus.write8(l + 5, (r.next_u32() % 2) as u8);
                    c.bus.write16(
                        l + 6,
                        [0xffffu32, 1, 2, 3][(r.next_u32() % 4) as usize] as u16,
                    );
                    c.bus.write8(l + 9, (r.next_u32() % 2) as u8);
                    c.bus.write16(
                        l + 0xa,
                        [0xffffu32, 1, 2, 3, id][(r.next_u32() % 5) as usize] as u16,
                    );
                }
                o406_w(c, O406_BOLT_DB + 4, DATA + 0x1400);
                for i in 0..8u32 {
                    o406_w(c, DATA + 0x1400 + 0x38 * i + 0x34, r.next_u32() & 8);
                }
                let mut values = vec![];
                for _ in 0..10 {
                    values.push([u64::MAX, 0, 1, 2][(r.next_u32() % 4) as usize]);
                    values.push((r.next_u32() % 2) as u64);
                }
                o406_results(c, &values);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = id as u64;
                c.g[A2][0] = on as u64;
            },
        );
    }

    /// `BE_InitCharProfile`: with and without the bolt-map reload, every
    /// character kind (default tricks per kind), colour table row `k`.
    #[test]
    fn be_init_char_profile_matches() {
        check(
            ee::s01::f_00151600__generated,
            be_init_char_profile,
            36,
            &|c, seed| {
                let _ = machine(c, seed, 0x151600, 0);
                elf_image(c);
                oc414_env(c);
                c.pc = 0x0015_1600;
                let this = DATA + 0x100;
                c.bus.write8(this + 0xbc0, (seed % 11) as u8);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = (seed % 3 == 0) as u64;
            },
        );
    }

    /// `BE_ApplyDefaultCharProfileBolts`: every character kind, 0..6 items
    /// with random flags and a random item map; the bolt tables come from the
    /// game image.
    #[test]
    fn be_apply_default_char_profile_bolts_matches() {
        check(
            ee::s01::f_001513b8__generated,
            be_apply_default_char_profile_bolts,
            48,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1513b8, 0);
                elf_image(c);
                oc414_env(c);
                c.pc = 0x0015_13b8;
                let this = DATA + 0x100;
                c.bus.write8(this + 0xbc0, (seed % 12) as u8);
                let n = seed % 7;
                c.bus.write32(this + 0x28c, n);
                c.bus.write32(this + 0x288, DATA + 0x800);
                for i in 0..n {
                    c.bus.write16(this + 0x290 + 4 * i, i as u16);
                    c.bus.write16(this + 0x292 + 4 * i, r.next_u32() as u16);
                }
                for i in 0..64u32 {
                    c.bus.write16(
                        DATA + 0x800 + 2 * i,
                        ((r.next_u32() % (n + 2)) as u16).wrapping_sub(1),
                    );
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    // ----------------------------------------------------------------------
    // BE_InitCharProfileBoltMap

    /// Profile with or without an allocated lookup, the bolt list (faked
    /// `Bolts_GetCharBolts/Count`: a pointer to 0x38-byte records whose
    /// `+0xe` is set so the default-bolt pass takes its short path), item
    /// counts below and above the profile's capacity, empty lookups.
    #[test]
    fn be_init_char_profile_bolt_map_matches() {
        const PROFILE: u32 = DATA + 0x1000;
        const BOLTS: u32 = DATA + 0x6000;
        const LOOKUP: u32 = DATA + 0x8000;
        check(
            ee::s01::f_00151a88__generated,
            be_init_char_profile_bolt_map,
            200,
            &|c, seed| {
                let mut r = oc417_machine(c, seed, 0x4170_0005, 6);
                c.pc = 0x0015_1a88;
                let total = [0u32, 1, 12, 30, 40][(seed % 5) as usize];
                c.bus.write32(GAME_GP - 0x1f08, total);
                let count = [0u32, 1, 3, 6, 9][((seed / 5) % 5) as usize];
                oc417_force(c, FK_BOLT_COUNT, count);
                oc417_force(c, FK_BOLTS, BOLTS);
                oc417_force(c, FK_NEW_NAMED, LOOKUP);
                for i in 0..count {
                    let rec = BOLTS + 0x38 * i;
                    c.bus.write16(rec + 4, (r.next_u32() % 40) as u16);
                    c.bus.write16(rec + 0xe, 1);
                }
                c.bus.write32(
                    PROFILE + BOLT_MAP_LOOKUP,
                    if seed % 3 == 0 { 0 } else { LOOKUP },
                );
                c.g[A0][0] = PROFILE as u64;
                c.g[A1][0] = (seed % 4) as u64;
            },
        );
    }

    /// The map entry of a random id, its flags, and a mask that is either
    /// empty (true) or random (mostly false).
    #[test]
    fn be_test_char_profile_bolt_flags_matches() {
        check(
            ee::s01::f_00152368__generated,
            be_test_char_profile_bolt_flags,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0015_2368, 0);
                c.pc = 0x0015_2368;
                let this = DATA;
                let map = DATA + 0x400;
                let id = r.next_u32() % 32;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = id as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.bus.write32(this + 0x288, map);
                c.bus.write16(map + 2 * id, (r.next_u32() % 64) as u16);
                for k in 0..64 {
                    c.bus.write16(this + 4 * k + 0x292, r.next_u32() as u16);
                }
            },
        );
    }

    /// `Bolts_GetDB`: the bolt database in the game's data.
    pub(crate) const OC676_BOLTS_DB: u32 = 0x004a_6750;

    /// An interface singleton in the gp slot `gp - gp_off`: an object whose
    /// vtable (at +0xc) has `{delta, method}` at +0x10/+0x14, the method being
    /// a fake outside the function table.
    pub(crate) fn oc676_instance(c: &mut Cpu, gp_off: u32, k: u32) {
        let obj = DATA + 0x6_0000 + 0x100 * k;
        let vt = DATA + 0x6_1000 + 0x40 * k;
        c.bus.write32(GAME_GP - gp_off, obj);
        c.bus.write32(obj + 0xc, vt);
        c.bus.write16(vt + 0x10, (k * 4) as u16);
        c.bus.write32(vt + 0x14, fakes::FAKE + 0x10 * k);
    }

    /// `BEEconInt_BuyAttribute`: human 0..2 (2 is rejected), the seven stats
    /// and out-of-range ones, levels around the cost table, money below, at
    /// and above the cost; the stat interface's `discard` is a fake.
    #[test]
    fn be_econ_int_buy_attribute_matches() {
        check(
            ee::s01::f_00150c20__generated,
            be_econ_int_buy_attribute,
            324,
            &|c, seed| {
                let mut r = machine(c, seed, 0x150c20, 1);
                elf_image(c);
                oc676_instance(c, 0x1f2c, 0);
                let human = seed % 3;
                let ch = r.next_u32() % 6;
                let attr = r.next_u32() % 9;
                let base = 0x004a_6ca8 + human * 0x9b50 + ch * 0xf88;
                const LEVELS: [u8; 10] = [0, 3, 5, 10, 20, 30, 40, 50, 54, 55];
                for i in 0..7 {
                    let lvl = LEVELS[(r.next_u32() % 10) as usize];
                    c.bus.write8(base + 0xbdf + i, lvl);
                }
                let lvl = c.bus.read8(base + 0xbdf + attr.min(6)) as i32;
                let cost = c.bus.read32((0x0044_0550 + (lvl / 5 - 1) * 8 + 4) as u32);
                let money = match r.next_u32() % 4 {
                    0 => 0,
                    1 => cost,
                    2 => cost.wrapping_sub(1),
                    _ => cost.wrapping_add(5000),
                };
                c.bus.write32(base + 0xac4, money);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = human as u64;
                c.g[A2][0] = ch as u64;
                c.g[A3][0] = attr as u64;
                c.pc = 0x00150c20;
            },
        );
    }

    /// `BERewardInt_GetEventRewardPoints`: places 6..11 (the race interface's
    /// score through a fake vtable call) and other places (the prize table of
    /// the game image: a known event key or none), medals -1..5.
    #[test]
    fn be_reward_int_get_event_reward_points_matches() {
        check(
            ee::s01::f_00151040__generated,
            be_reward_int_get_event_reward_points,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x151040, 2);
                elf_image(c);
                c.bus.write32(GAME_GP - 0x1f04, DATA + 0x100);
                c.bus.write32(GAME_GP - 0x1efc, 0);
                let obj = DATA + 0x6_0000;
                let vt = DATA + 0x6_1000;
                c.bus.write32(0x0053_08a8, obj);
                c.bus.write32(obj + 4, vt);
                c.bus.write16(vt + 8, 0x20);
                c.bus.write32(vt + 0xc, fakes::FAKE);
                let medal = [0u32, 1, 2, 3, 0xffff_ffff, 5][(r.next_u32() % 6) as usize];
                let place = if seed % 2 == 0 {
                    6 + r.next_u32() % 6
                } else {
                    [0u32, 1, 3, 5, 12, 0xffff_ffff][(r.next_u32() % 6) as usize]
                };
                let key = if seed % 5 == 4 {
                    r.next_u32() % 100
                } else {
                    c.bus.read32(0x0044_05a0 + 0x14 * (r.next_u32() % 17))
                };
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = medal as i32 as i64 as u64;
                c.g[A2][0] = key as u64;
                c.g[A3][0] = place as i32 as i64 as u64;
                c.pc = 0x00151040;
            },
        );
    }

    /// Bolt database for character `ch`: `n` definitions from `off`, with ids
    /// 0..7, group 100/101 and flags without bits 1/8 (keeps the real bolt
    /// callees on their short paths), `n4` default-bolt groups from `t4off`.
    pub(crate) fn oc676_bolts_db(
        c: &mut Cpu,
        r: &mut Rng,
        ch: u32,
        n: u32,
        off: u32,
        n4: u32,
        t4off: u32,
    ) {
        let defs = DATA + 0x4_0000;
        let t4 = DATA + 0x5_0000;
        let db = OC676_BOLTS_DB;
        c.bus.write32(db + 4, defs);
        c.bus.write32(db + 0x10, t4);
        c.bus.write32(db + 0x2c + ch * 4, n);
        c.bus.write32(db + 0xa4 + ch * 4, off);
        c.bus.write32(db + 0x3ec + ch * 4, n4);
        c.bus.write32(db + 0x464 + ch * 4, t4off);
        for e in 0..(off + n) {
            let d = defs + e * 0x38;
            c.bus.write16(d + 4, (r.next_u32() % 8) as u16);
            c.bus.write16(d + 6, (100 + r.next_u32() % 2) as u16);
            let flags =
                (r.next_u32() & 0x7ff2) | ((r.next_u32() & 1) << 12) | (r.next_u32() & 0x20);
            c.bus.write32(d + 0x34, flags);
        }
        for e in 0..(t4off + n4) {
            let d = t4 + e * 8;
            c.bus.write16(d + 4, (100 + r.next_u32() % 2) as u16);
            c.bus.write16(d + 6, (r.next_u32() % 8) as u16);
        }
    }

    /// The profile object at `DATA`: character `ch`, bolt map (id -> item) and
    /// the items' flags at +0x290.
    pub(crate) fn oc676_bolt_profile(c: &mut Cpu, r: &mut Rng, ch: u32) {
        let tbl = DATA + 0x3_0000;
        c.bus.write8(DATA + 0xbc0, ch as u8);
        c.bus.write32(DATA + 0x288, tbl);
        for i in 0..16 {
            let v = if r.next_u32() % 4 == 0 {
                0xffff
            } else {
                r.next_u32() % 8
            };
            c.bus.write16(tbl + 2 * i, v as u16);
        }
        for i in 0..8 {
            c.bus.write16(DATA + 0x290 + 4 * i + 2, r.next_u32() as u16);
        }
    }

    /// `BE_ValidateExclusiveBolts`: 0..4 definitions per character, exclusive
    /// flags and owned items at random; `BE_ApplyBoltDependencies` is real
    /// (no dependency links: it succeeds). The failure return needs a failing
    /// recursive equip, which the fixture cannot build.
    #[test]
    fn be_validate_exclusive_bolts_matches() {
        check(
            ee::s01::f_001520e8__generated,
            be_validate_exclusive_bolts,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1520e8, 0);
                let ch = seed % 3;
                oc676_bolt_profile(c, &mut r, ch);
                let n = r.next_u32() % 5;
                let off = r.next_u32() % 3;
                oc676_bolts_db(c, &mut r, ch, n, off, 0, 0);
                c.bus.write32(DATA + 0x500, 0xdead_beef);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x500) as u64;
                c.pc = 0x001520e8;
            },
        );
    }

    /// `BE_EnsureDefaultBoltsEquipped`: 0..3 groups per character, 0..5 bolts
    /// per group, owned items at random; the real `BE_SetCharProfileBoltEquipped`
    /// runs on definitions without the bits that recurse. Its failure return
    /// (`ok = 0`) is not reachable from this fixture.
    #[test]
    fn be_ensure_default_bolts_equipped_matches() {
        check(
            ee::s01::f_001521f0__generated,
            be_ensure_default_bolts_equipped,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1521f0, 0);
                let ch = seed % 3;
                oc676_bolt_profile(c, &mut r, ch);
                let n = 1 + r.next_u32() % 5;
                let off = r.next_u32() % 3;
                let n4 = r.next_u32() % 4;
                let t4off = r.next_u32() % 3;
                oc676_bolts_db(c, &mut r, ch, n, off, n4, t4off);
                c.g[A0][0] = DATA as u64;
                c.pc = 0x001521f0;
            },
        );
    }
}
