//! OpenCode worker module (docs/investigation/phase2-worker-guide.md).
//!
//! Trivial leaf replacements. Each copies the generated body's logic, cycle
//! charging and observable register (and FPU condition) results exactly, with
//! a differential test as its gate.

use crate::abi;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0014_c620, bolts_init_db, "Bolts_InitDB"),
    (0x0014_d988, bolts_get_char_bolt_count, "Bolts_GetCharBoltCount"),
    (0x0014_d998, bolts_get_char_bolts, "Bolts_GetCharBolts"),
    (0x0014_d9d0, bolts_get_char_bolts_2, "Bolts_GetCharBolts_2"),
    (0x0014_eec8, be_get_char_display_name, "BE_GetCharDisplayName"),
    (0x0015_0928, be_econ_int_get_money, "BEEconInt_GetMoney"),
    (0x0015_09c0, be_econ_int_get_total_earned, "BEEconInt_GetTotalEarned"),
    (0x0015_0a90, be_econ_int_add_money, "BEEconInt_AddMoney"),
    (0x0015_0b48, be_econ_int_spend_money, "BEEconInt_SpendMoney"),
    (0x0015_0e50, be_econ_int_get_attribute_cost, "BEEconInt_GetAttributeCost"),
    (0x0015_5270, be_score_int_get_player_result, "BEScoreInt_GetPlayerResult"),
    (0x0015_5288, be_score_int_get_best_records, "BEScoreInt_GetBestRecords"),
    (0x0015_5328, be_score_int_place_to_medal, "BEScoreInt_PlaceToMedal"),
    (0x0015_6a38, be_reward_int_is_rewards_file_loaded, "BERewardInt_IsRewardsFileLoaded"),
    (0x0015_6a60, be_reward_int_is_better_place, "BERewardInt_IsBetterPlace"),
    (0x0015_73f0, be_reward_int_set_bit, "BERewardInt_SetBit"),
    (0x0015_7430, be_reward_int_test_bit, "BERewardInt_TestBit"),
    (0x0015_8870, be_reward_int_get_collectible_threshold, "BERewardInt_GetCollectibleThreshold"),
    (0x0015_8890, be_reward_int_get_mission_threshold, "BERewardInt_GetMissionThreshold"),
    (0x0015_8f20, be_reward_int_get_new_rewards, "BERewardInt_GetNewRewards"),
];

// ---------------------------------------------------------------------------
// Bolt database.

/// `void Bolts_InitDB(BoltsDB *db)` (0x14c620): zero the eleven words of the
/// DB header (0x00..0x28); leaves `db` in v0, as the constructor stub does.
pub fn bolts_init_db(c: &mut Cpu) {
    c.cycles += 13;
    let db = Addr(c.g[A0][0] as u32);
    c.g[V0][0] = c.g[A0][0];
    for off in (0..0x2c).step_by(4) {
        db.off(off).sw(c, 0);
    }
    abi::ret(c);
}

/// `int Bolts_GetCharBoltCount(BoltsDB *db, int charIndex)` (0x14d988): read the
/// count at `db + charIndex*4 + 0x2c`.
pub fn bolts_get_char_bolt_count(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A1][0] = sx(abi::arg(c, 1) << 2);
    let record = abi::arg(c, 0).wrapping_add(c.g[A1][0] as u32);
    c.g[A0][0] = sx(record);
    c.g[V0][0] = Addr(record).off(0x2c).lw(c);
    abi::ret(c);
}

/// The shared body of `Bolts_GetCharBolts`/`_2`: if the character's bolt count
/// is positive, resolve the bolt array at `record + 0xa4` through its stride
/// (0x38) and add the base read at `db + 4`; otherwise return 0.
fn char_bolts_pointer(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A1][0] = sx(abi::arg(c, 1) << 2);
    let record = abi::arg(c, 0).wrapping_add(c.g[A1][0] as u32);
    c.g[V0][0] = sx(record);
    let count = Addr(record).off(0x2c).lw(c);
    c.g[V1][0] = c.g[V0][0];
    if (count as i64) <= 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 6;
    c.g[A1][0] = 0x38;
    c.g[V0][0] = Addr(record).off(0xa4).lw(c);
    c.g[A0][0] = Addr(abi::arg(c, 0)).off(0x4).lw(c);
    let index = abi::mult(c, c.g[V0][0] as u32, 0x38);
    c.g[V0][0] = sx(index);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    abi::ret(c);
}

/// `BoltsBolt *Bolts_GetCharBolts(BoltsDB *db, int charIndex)` (0x14d998).
pub fn bolts_get_char_bolts(c: &mut Cpu) {
    char_bolts_pointer(c);
}

/// `BoltsBolt *Bolts_GetCharBolts_2(BoltsDB *db, int charIndex)` (0x14d9d0):
/// identical to `Bolts_GetCharBolts`.
pub fn bolts_get_char_bolts_2(c: &mut Cpu) {
    char_bolts_pointer(c);
}

// ---------------------------------------------------------------------------
// Front-end character/economy helpers.

/// `const char *BE_GetCharDisplayName(uint char, uint index)` (0x14eec8): for
/// character ids 0xa..0x1d uses the 8-byte name table based at 0x43fa38, else
/// the entry at `index*0x88 + 0x530990`.
pub fn be_get_char_display_name(c: &mut Cpu) {
    c.cycles += 4;
    let ch = abi::arg(c, 2);
    let in_name_table = sx(ch.wrapping_sub(0xa)) < 0x14;
    c.g[V0][0] = in_name_table as u64;
    if in_name_table {
        c.cycles += 4;
        c.g[V1][0] = sx(ch << 3);
        c.g[V0][0] = sx(0x44_0000u32.wrapping_sub(0x5c8));
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        abi::ret(c);
        return;
    }
    c.cycles += 6;
    c.g[V1][0] = 0x88;
    c.g[V0][0] = 0x53_0000;
    let entry = abi::mult(c, abi::arg(c, 1), 0x88);
    c.g[V1][0] = sx(entry);
    c.g[V0][0] = sx(0x53_0000u32.wrapping_add(0x990));
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    abi::ret(c);
}

/// The shared address arithmetic of the two economy getters: `db` (0x4a6ca8)
/// plus the rider slot (stride 0x9b50) and character slot (stride 0xf88).
fn econ_rider_base(c: &mut Cpu) -> u32 {
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

// ---------------------------------------------------------------------------
// Front-end scores.

/// `BEScoreResult *BEScoreInt_GetPlayerResult(BEScoreInt *self, int player, ...)`
/// (0x155270): `self + player*0xb0 + 0x20`.
pub fn be_score_int_get_player_result(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V1][0] = 0xb0;
    let slot = abi::mult(c, abi::arg(c, 1), 0xb0);
    c.g[V0][0] = sx(slot.wrapping_add(0x20));
    c.g[V0][0] = sx(abi::arg(c, 0).wrapping_add(c.g[V0][0] as u32));
    abi::ret(c);
}

/// `BEScoreRecord *BEScoreInt_GetBestRecords(BEScoreInt *self, int rider, int character)`
/// (0x155288): `rider*0x9b50 + character*0xf88 + 0x4a6ca8 + 0xde4`.
pub fn be_score_int_get_best_records(c: &mut Cpu) {
    c.cycles += 10;
    c.g[V1][0] = 0x9b50;
    c.g[A0][0] = 0xf88;
    let rider_off = abi::mult(c, abi::arg(c, 1), 0x9b50);
    c.g[V0][0] = sx(rider_off);
    let char_off = abi::mult(c, abi::arg(c, 2), 0xf88);
    c.g[A2][0] = sx(char_off);
    c.g[V1][0] = sx(0x4a_0000u32.wrapping_add(0x6ca8));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xde4));
    abi::ret(c);
}

/// `int BEScoreInt_PlaceToMedal(int place)` (0x155328): medal grade for a finish
/// place — 1st -> 2, 2nd -> 3, 3rd -> 1, anything else -1.
pub fn be_score_int_place_to_medal(c: &mut Cpu) {
    c.cycles += 3;
    let place = abi::arg64(c, 1);
    if place == 1 {
        c.cycles += 2;
        c.g[V0][0] = 2;
        abi::ret(c);
        return;
    }
    if (place as i64) >= 2 {
        c.cycles += 2;
        c.g[V0][0] = 2;
        c.cycles += 2;
        if place == 2 {
            c.cycles += 2;
            c.g[V0][0] = 3;
            abi::ret(c);
            return;
        }
        c.g[V0][0] = u64::MAX;
        c.cycles += 2;
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    // place < 2 and not 1: either 0, or a negative sign-extended value.
    c.cycles += 2;
    c.cycles += 2;
    c.g[V0][0] = u64::MAX;
    if place == 0 {
        c.cycles += 2;
        c.g[V0][0] = 1;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Front-end rewards.

/// `bool BERewardInt_IsRewardsFileLoaded(BERewardInt *self)` (0x156a38): false
/// while the loaded-file handle at 0x4c3e98 is -1, else whether its open count
/// at +4 is positive.
pub fn be_reward_int_is_rewards_file_loaded(c: &mut Cpu) {
    const FILE_HANDLE: u32 = 0x4c3e98;
    c.cycles += 6;
    c.g[V0][0] = 0x4c_0000;
    c.g[A1][0] = 0;
    let handle = Addr(FILE_HANDLE).lw(c);
    c.g[A0][0] = handle;
    c.g[V1][0] = u64::MAX;
    c.g[A2][0] = sx(FILE_HANDLE);
    if handle == u64::MAX {
        c.cycles += 2;
        let open = Addr(FILE_HANDLE + 4).lw(c);
        c.g[V0][0] = open;
        c.g[A1][0] = (open > 0) as u64;
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A1][0];
    abi::ret(c);
}

/// `bool BERewardInt_IsBetterPlace(BERewardInt *self, int old, int new)`
/// (0x156a60): false when `new` is -1, true when `old` is -1, else
/// `new < old`. All compared as whole 64-bit registers.
pub fn be_reward_int_is_better_place(c: &mut Cpu) {
    c.cycles += 3;
    let new_place = abi::arg64(c, 2);
    let old_place = abi::arg64(c, 1);
    c.g[V0][0] = u64::MAX;
    if new_place == u64::MAX {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    if old_place == u64::MAX {
        c.cycles += 2;
        c.g[V0][0] = 1;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = ((new_place as i64) < (old_place as i64)) as u64;
    abi::ret(c);
}

/// `void BERewardInt_SetBit(BERewardInt *self, int bit, u8 *bits)` (0x1573f0):
/// set `bit` (mod 8) in the byte at `bits + bit>>3`.
pub fn be_reward_int_set_bit(c: &mut Cpu) {
    c.cycles += 15;
    c.g[V0][0] = u64::MAX;
    c.g[V1][0] = sx(abi::arg(c, 1).wrapping_add(7));
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
    c.g[A0][0] = sx(abi::arg(c, 1) >> 3);
    if c.g[V0][0] != 0 {
        c.g[V1][0] = c.g[A1][0];
    }
    c.g[A2][0] = sx(abi::arg(c, 2).wrapping_add(c.g[A0][0] as u32));
    c.g[A3][0] = sx(((c.g[V1][0] as u32) as i32 >> 3) as u32);
    c.g[V0][0] = 0x1;
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 3);
    c.g[V1][0] = c.lbu(c.g[A2][0] as u32);
    c.g[A3][0] = sx(abi::arg(c, 1).wrapping_sub(c.g[A3][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << (c.g[A3][0] as u32 & 31));
    c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
    Addr(c.g[A2][0] as u32).sb(c, c.g[V1][0]);
    abi::ret(c);
}

/// `bool BERewardInt_TestBit(BERewardInt *self, int bit, u8 *bits)` (0x157430):
/// whether `bit` (mod 8) is set in the byte at `bits + bit>>3`.
pub fn be_reward_int_test_bit(c: &mut Cpu) {
    c.cycles += 13;
    c.g[V0][0] = u64::MAX;
    c.g[V1][0] = sx(abi::arg(c, 1).wrapping_add(7));
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
    c.g[A0][0] = sx(abi::arg(c, 1) >> 3);
    if c.g[V0][0] != 0 {
        c.g[V1][0] = c.g[A1][0];
    }
    c.g[A2][0] = sx(abi::arg(c, 2).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 3) as u32);
    c.g[V0][0] = Addr(c.g[A2][0] as u32).lb(c);
    c.g[A0][0] = sx((c.g[V1][0] as u32) << 3);
    c.g[V1][0] = sx(abi::arg(c, 1).wrapping_sub(c.g[A0][0] as u32));
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> (c.g[V1][0] as u32 & 31)) as u32);
    c.g[V0][0] = c.g[V0][0] & 0x1;
    abi::ret(c);
}

/// `int BERewardInt_GetCollectibleThreshold(BERewardInt *self, int a, int b)`
/// (0x158870): word at `a*16 + b*4 + 0x45afe8`.
pub fn be_reward_int_get_collectible_threshold(c: &mut Cpu) {
    c.cycles += 8;
    c.g[A2][0] = sx(abi::arg(c, 2) << 2);
    c.g[A1][0] = sx(abi::arg(c, 1) << 4);
    c.g[V0][0] = 0x46_0000;
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = sx(0x46_0000u32.wrapping_sub(0x5018));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
    abi::ret(c);
}

/// `int BERewardInt_GetMissionThreshold(BERewardInt *self, int a, int b)`
/// (0x158890): word at `a*16 + b*4 + 0x45b018`.
pub fn be_reward_int_get_mission_threshold(c: &mut Cpu) {
    c.cycles += 8;
    c.g[A2][0] = sx(abi::arg(c, 2) << 2);
    c.g[A1][0] = sx(abi::arg(c, 1) << 4);
    c.g[V0][0] = 0x46_0000;
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = sx(0x46_0000u32.wrapping_sub(0x4fe8));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
    abi::ret(c);
}

/// `NewRewards *BERewardInt_GetNewRewards(BERewardInt *self)` (0x158f20):
/// constant pointer 0x4c3ef0.
pub fn be_reward_int_get_new_rewards(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x4c_0000;
    c.g[V0][0] = sx(0x4c_0000u32.wrapping_add(0x3ef0));
    abi::ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, check_lohi, scramble, Rng, DATA};

    /// Address arithmetic shared by the economy tests.
    fn econ_base(rider: u32, ch: u32) -> u32 {
        0x4a6ca8u32
            .wrapping_add(rider.wrapping_mul(0x9b50))
            .wrapping_add(ch.wrapping_mul(0xf88))
    }

    #[test]
    fn bolts_init_db_matches() {
        check(ssx3_ee::s01::f_0014c620, bolts_init_db, 50, &|c, seed| {
            let mut r = Rng(0x1000_0001 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for off in (0..0x2c).step_by(4) {
                c.bus.write32(DATA + off, r.next_u32());
            }
        });
    }

    #[test]
    fn bolts_get_char_bolt_count_matches() {
        check(ssx3_ee::s01::f_0014d988, bolts_get_char_bolt_count, 100, &|c, seed| {
            let mut r = Rng(0x2000_0002 ^ seed as u64);
            scramble(c, &mut r);
            let idx = seed % 4;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = idx as u64;
            c.bus.write32(DATA + idx * 4 + 0x2c, r.next_u32());
        });
    }

    #[test]
    fn bolts_get_char_bolts_matches() {
        check_lohi(ssx3_ee::s01::f_0014d998, bolts_get_char_bolts, 300, &|c, seed| {
            let mut r = Rng(0x3000_0003 ^ seed as u64);
            scramble(c, &mut r);
            let idx = seed % 4;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = idx as u64;
            let record = DATA + idx * 4;
            let count = if seed % 2 == 0 {
                r.next_u32() | 0x8000_0000
            } else {
                (r.next_u32() & 0x7fff_ffff) | 1
            };
            c.bus.write32(record + 0x2c, count);
            c.bus.write32(record + 0xa4, r.next_u32());
            c.bus.write32(DATA + 4, r.next_u32());
        });
    }

    #[test]
    fn bolts_get_char_bolts_2_matches() {
        check_lohi(ssx3_ee::s01::f_0014d9d0, bolts_get_char_bolts_2, 300, &|c, seed| {
            let mut r = Rng(0x3000_0004 ^ seed as u64);
            scramble(c, &mut r);
            let idx = seed % 4;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = idx as u64;
            let record = DATA + idx * 4;
            let count = if seed % 2 == 0 {
                r.next_u32() | 0x8000_0000
            } else {
                (r.next_u32() & 0x7fff_ffff) | 1
            };
            c.bus.write32(record + 0x2c, count);
            c.bus.write32(record + 0xa4, r.next_u32());
            c.bus.write32(DATA + 4, r.next_u32());
        });
    }

    #[test]
    fn be_get_char_display_name_matches() {
        check_lohi(ssx3_ee::s01::f_0014eec8, be_get_char_display_name, 300, &|c, seed| {
            let mut r = Rng(0x4000_0005 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            let ch = match seed % 3 {
                0 => 0xa + (seed % 0x14),
                1 => seed % 0xa,
                _ => r.next_u32(),
            };
            c.g[A2][0] = ch as u64;
        });
    }

    #[test]
    fn be_econ_int_get_money_matches() {
        check_lohi(ssx3_ee::s01::f_00150928, be_econ_int_get_money, 300, &|c, seed| {
            let mut r = Rng(0x5000_0006 ^ seed as u64);
            scramble(c, &mut r);
            let rider = seed % 3;
            let ch = seed % 4;
            c.g[A1][0] = rider as u64;
            c.g[A2][0] = ch as u64;
            c.bus.write32(econ_base(rider, ch) + 0xac4, r.next_u32());
        });
    }

    #[test]
    fn be_econ_int_get_total_earned_matches() {
        check_lohi(ssx3_ee::s01::f_001509c0, be_econ_int_get_total_earned, 300, &|c, seed| {
            let mut r = Rng(0x6000_0007 ^ seed as u64);
            scramble(c, &mut r);
            let rider = seed % 3;
            let ch = seed % 4;
            c.g[A1][0] = rider as u64;
            c.g[A2][0] = ch as u64;
            c.bus.write32(econ_base(rider, ch) + 0xac8, r.next_u32());
        });
    }

    #[test]
    fn be_econ_int_add_money_matches() {
        check_lohi(ssx3_ee::s01::f_00150a90, be_econ_int_add_money, 300, &|c, seed| {
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
        });
    }

    #[test]
    fn be_econ_int_spend_money_matches() {
        check_lohi(ssx3_ee::s01::f_00150b48, be_econ_int_spend_money, 300, &|c, seed| {
            let mut r = Rng(0x8000_0009 ^ seed as u64);
            scramble(c, &mut r);
            let rider = seed % 3;
            let ch = seed % 4;
            c.g[A1][0] = rider as u64;
            c.g[A2][0] = ch as u64;
            c.g[A3][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.bus.write32(econ_base(rider, ch) + 0xac4, r.next_u32());
        });
    }

    #[test]
    fn be_econ_int_get_attribute_cost_matches() {
        check(ssx3_ee::s01::f_00150e50, be_econ_int_get_attribute_cost, 100, &|c, seed| {
            let mut r = Rng(0x9000_000a ^ seed as u64);
            scramble(c, &mut r);
            let idx = seed % 4;
            c.g[A1][0] = idx as u64;
            c.bus.write32(0x440550 + idx * 8 + 4, r.next_u32());
        });
    }

    #[test]
    fn be_score_int_get_player_result_matches() {
        check_lohi(ssx3_ee::s01::f_00155270, be_score_int_get_player_result, 200, &|c, seed| {
            let mut r = Rng(0xa000_000b ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.g[A1][0] = seed as u64;
        });
    }

    #[test]
    fn be_score_int_get_best_records_matches() {
        check_lohi(ssx3_ee::s01::f_00155288, be_score_int_get_best_records, 200, &|c, seed| {
            let mut r = Rng(0xb000_000c ^ seed as u64);
            scramble(c, &mut r);
            c.g[A1][0] = (seed % 8) as u64;
            c.g[A2][0] = ((seed / 8) % 8) as u64;
        });
    }

    #[test]
    fn be_score_int_place_to_medal_matches() {
        check(ssx3_ee::s01::f_00155328, be_score_int_place_to_medal, 400, &|c, seed| {
            let mut r = Rng(0xc000_000d ^ seed as u64);
            scramble(c, &mut r);
            c.g[A1][0] = match seed % 5 {
                0 => 1,
                1 => 0,
                2 => 2,
                3 => 3 + (seed % 8) as u64,
                _ => 0xffff_ffff_ffff_ff00u64 | (seed as u64 & 0xff),
            };
        });
    }

    #[test]
    fn be_reward_int_is_rewards_file_loaded_matches() {
        check(ssx3_ee::s01::f_00156a38, be_reward_int_is_rewards_file_loaded, 300, &|c, seed| {
            let mut r = Rng(0xd000_000e ^ seed as u64);
            scramble(c, &mut r);
            if seed % 2 == 0 {
                c.bus.write32(0x4c3e98, r.next_u32());
            } else {
                c.bus.write32(0x4c3e98, 0xffff_ffff);
            }
            let open = match seed % 3 {
                0 => 0,
                1 => r.next_u32() | 0x8000_0000,
                _ => r.next_u32() & 0x7fff_ffff,
            };
            c.bus.write32(0x4c3e9c, open);
        });
    }

    #[test]
    fn be_reward_int_is_better_place_matches() {
        check(ssx3_ee::s01::f_00156a60, be_reward_int_is_better_place, 300, &|c, seed| {
            let mut r = Rng(0xe000_000f ^ seed as u64);
            scramble(c, &mut r);
            c.g[A2][0] = match seed % 3 {
                0 => u64::MAX,
                _ => ((r.next_u32() as u64) << 32) | r.next_u32() as u64,
            };
            c.g[A1][0] = match seed % 4 {
                0 => u64::MAX,
                _ => ((r.next_u32() as u64) << 32) | r.next_u32() as u64,
            };
        });
    }

    #[test]
    fn be_reward_int_set_bit_matches() {
        check(ssx3_ee::s01::f_001573f0, be_reward_int_set_bit, 300, &|c, seed| {
            let mut r = Rng(0xf000_0010 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A2][0] = DATA as u64;
            let bit = (seed % 32) as u64;
            // Odd seeds are negative as a whole 64-bit value, so the
            // "index < 0" adjustment path is exercised too.
            c.g[A1][0] = if seed % 2 == 0 {
                bit
            } else {
                0x8000_0000_0000_0000u64 | bit
            };
            let byte_addr = DATA + ((c.g[A1][0] as u32) >> 3);
            c.bus.write8(byte_addr, r.next_u32() as u8);
        });
    }

    #[test]
    fn be_reward_int_test_bit_matches() {
        check(ssx3_ee::s01::f_00157430, be_reward_int_test_bit, 300, &|c, seed| {
            let mut r = Rng(0x1_0000_0011u64 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A2][0] = DATA as u64;
            let bit = (seed % 32) as u64;
            c.g[A1][0] = if seed % 2 == 0 {
                bit
            } else {
                0x8000_0000_0000_0000u64 | bit
            };
            let byte_addr = DATA + ((c.g[A1][0] as u32) >> 3);
            c.bus.write8(byte_addr, r.next_u32() as u8);
        });
    }

    #[test]
    fn be_reward_int_get_collectible_threshold_matches() {
        check(ssx3_ee::s01::f_00158870, be_reward_int_get_collectible_threshold, 200, &|c, seed| {
            let mut r = Rng(0x1_1000_0012u64 ^ seed as u64);
            scramble(c, &mut r);
            let a = seed % 4;
            let b = seed % 4;
            c.g[A1][0] = a as u64;
            c.g[A2][0] = b as u64;
            let addr = 0x45afe8u32.wrapping_add(b * 4).wrapping_add(a * 16);
            c.bus.write32(addr, r.next_u32());
        });
    }

    #[test]
    fn be_reward_int_get_mission_threshold_matches() {
        check(ssx3_ee::s01::f_00158890, be_reward_int_get_mission_threshold, 200, &|c, seed| {
            let mut r = Rng(0x1_2000_0013u64 ^ seed as u64);
            scramble(c, &mut r);
            let a = seed % 4;
            let b = seed % 4;
            c.g[A1][0] = a as u64;
            c.g[A2][0] = b as u64;
            let addr = 0x45b018u32.wrapping_add(b * 4).wrapping_add(a * 16);
            c.bus.write32(addr, r.next_u32());
        });
    }

    #[test]
    fn be_reward_int_get_new_rewards_matches() {
        check(ssx3_ee::s01::f_00158f20, be_reward_int_get_new_rewards, 20, &|c, seed| {
            let mut r = Rng(0x1_3000_0014u64 ^ seed as u64);
            scramble(c, &mut r);
        });
    }
}
