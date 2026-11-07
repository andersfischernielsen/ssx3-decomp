//! `src/be/beintbagt.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
pub(crate) use crate::game::be::beintmission::lw;
pub(crate) use crate::game::be::beintmission::BUILTIN_DELETE;
pub(crate) use crate::game::be::beintmission::EA_MEM_ALLOC_NAMED;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::scripter::ssxscriptengine::p;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0014_f900,
        be_bagt_int_get_instance,
        "BEBAGTInt_GetInstance",
    ),
    (
        0x0014_fd80,
        bebagt_int_set_uber_trick_button,
        "BEBAGTInt_SetUberTrickButton",
    ),
    (
        0x0014_fe08,
        be_bagt_int_clear_uber_trick_bit,
        "BEBAGTInt_ClearUberTrickBit",
    ),
    (
        0x0014_ff90,
        be_bagt_int_get_trick_table_entry,
        "BEBAGTInt_GetTrickTableEntry",
    ),
    (
        0x0014_ffb8,
        bebagt_int_test_uber_trick_bit_a,
        "BEBAGTInt_TestUberTrickBitA",
    ),
    (
        0x0015_0048,
        bebagt_int_test_uber_trick_bit_b,
        "BEBAGTInt_TestUberTrickBitB",
    ),
    (
        0x0015_00d8,
        bebagtint_get_trick_field0,
        "BEBAGTInt_GetTrickField0",
    ),
    (
        0x0015_00f8,
        bebagtint_get_trick_field2,
        "BEBAGTInt_GetTrickField2",
    ),
    (
        0x0015_0118,
        bebagtint_get_trick_field4,
        "BEBAGTInt_GetTrickField4",
    ),
    (
        0x0015_0138,
        bebagtint_get_trick_field6,
        "BEBAGTInt_GetTrickField6",
    ),
    (
        0x0015_0158,
        bebagtint_get_trick_field8,
        "BEBAGTInt_GetTrickField8",
    ),
    (
        0x0015_0178,
        bebagtint_get_trick_fielda,
        "BEBAGTInt_GetTrickFieldA",
    ),
    (
        0x0015_c6e8,
        be_bagt_int_destroy_instance,
        "BEBAGTInt_DestroyInstance",
    ),
    (
        0x0014_fea8,
        bebagt_int_get_uber_trick_slot_value,
        "BEBAGTInt_GetUberTrickSlotValue",
    ),
    (
        0x0015_0198,
        bebagt_int_get_trick_anim,
        "BEBAGTInt_GetTrickAnim",
    ),
];

// ---------------------------------------------------------------------------
// Callees the generator emitted as direct calls (the readable alias, which
// dispatches through the override slot) and as cross-shard `call_indirect`s.

pub(crate) mod callee {

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_PROFILE;

    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_CHARACTER_2 as BE_GET_RIDER_CHARACTER_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_3;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_CHARACTER as BE_GET_RIDER_CHARACTER_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_2;
}

/// `void BEBAGTInt_DestroyInstance(void)` (0x15c6e8).
pub fn be_bagt_int_destroy_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x1ee0));
        frame.save_ra(c, 0);
        call_ptr(c, BUILTIN_DELETE, 0x0015_c6fc)?;
        frame.restore_ra(c, 0);
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1ee0), 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BAGT singleton.

/// `BEBAGTInt *BEBAGTInt_GetInstance(void)` (0x14f900): lazily allocate the
/// singleton, install its vtable and run the vtable's constructor slot (+0x10).
pub fn be_bagt_int_get_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x1ee0));
        let have = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if !have {
            c.cycles += 16;
            c.g[A1][0] = 0x460000;
            c.g[A0][0] = 0x10;
            c.g[A1][0] = sx(0x460000u32.wrapping_sub(0x5928));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, EA_MEM_ALLOC_NAMED, 0x0014_f928)?;
            c.g[V1][0] = 0x460000;
            c.sw((c.g[GP][0] as u32).wrapping_sub(0x1ee0), c.g[V0][0]);
            c.g[V1][0] = sx(0x460000u32.wrapping_sub(0x5328));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[A1][0] = lw(c, (c.g[V1][0] as u32).wrapping_add(0x14));
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
            let target = c.g[A1][0] as u32;
            call_ptr(c, target, 0x0014_f948)?;
            c.g[V0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x1ee0));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0);
        }
        c.cycles += 4;
        c.g[V0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x1ee0));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `D_0045AEB8`: the BE BAGT trick table (0x14-byte entries).
pub(crate) const TRICK_TABLE: u32 = 0x0045_aeb8;

/// `u16 BEBAGTInt_GetTrickField0(int unused, int unused, int index)`
/// (0x1500d8): the first halfword of a BAGT trick entry (0x14-byte records in
/// `D_0045AEB8`).
pub fn bebagtint_get_trick_field0(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V1][0] = 0x14;
    c.g[V0][0] = 0x46_0000;
    c.mult(0, A2, A2, V1); // a2 = index * 0x14
    c.g[V0][0] = sx(TRICK_TABLE);
    let base = c.g[V0][0] as u32;
    c.g[A2][0] = sx(base.wrapping_add(c.g[A2][0] as u32));
    let entry = c.g[A2][0] as u32;
    c.g[V0][0] = c.lhu(entry);
    abi::ret(c);
}

/// `u16 BEBAGTInt_GetTrickField2(...)` (0x1500f8): halfword +2 of a trick
/// entry.
pub fn bebagtint_get_trick_field2(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V1][0] = 0x14;
    c.g[V0][0] = 0x46_0000;
    c.mult(0, A2, A2, V1);
    c.g[V0][0] = sx(TRICK_TABLE);
    let base = c.g[V0][0] as u32;
    c.g[A2][0] = sx(base.wrapping_add(c.g[A2][0] as u32));
    let entry = c.g[A2][0] as u32;
    c.g[V0][0] = c.lhu(entry.wrapping_add(2));
    abi::ret(c);
}

/// `u16 BEBAGTInt_GetTrickField4(...)` (0x150118): halfword +4 of a trick
/// entry.
pub fn bebagtint_get_trick_field4(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V1][0] = 0x14;
    c.g[V0][0] = 0x46_0000;
    c.mult(0, A2, A2, V1);
    c.g[V0][0] = sx(TRICK_TABLE);
    let base = c.g[V0][0] as u32;
    c.g[A2][0] = sx(base.wrapping_add(c.g[A2][0] as u32));
    let entry = c.g[A2][0] as u32;
    c.g[V0][0] = c.lhu(entry.wrapping_add(4));
    abi::ret(c);
}

/// `u16 BEBAGTInt_GetTrickField6(...)` (0x150138): halfword +6 of a trick
/// entry.
pub fn bebagtint_get_trick_field6(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V1][0] = 0x14;
    c.g[V0][0] = 0x46_0000;
    c.mult(0, A2, A2, V1);
    c.g[V0][0] = sx(TRICK_TABLE);
    let base = c.g[V0][0] as u32;
    c.g[A2][0] = sx(base.wrapping_add(c.g[A2][0] as u32));
    let entry = c.g[A2][0] as u32;
    c.g[V0][0] = c.lhu(entry.wrapping_add(6));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// BAGT trick records: 20-byte records at `D_0045AEB8`.

/// `D_0045AEB8`: the BAGT trick table.
pub(crate) const BAGT_TRICKS: u32 = 0x0045_aeb8;

/// `unsigned short BEBAGTInt_GetTrickField8(int idx)` (0x00150158): the u16
/// field at `+0x8` of trick record `idx`.
pub fn bebagtint_get_trick_field8(c: &mut Cpu) {
    bagt_trick_field(c, 0x8);
}

/// `unsigned short BEBAGTInt_GetTrickFieldA(int idx)` (0x00150178): the u16
/// field at `+0xa` of trick record `idx`.
pub fn bebagtint_get_trick_fielda(c: &mut Cpu) {
    bagt_trick_field(c, 0xa);
}

/// `p = &D_0045AEB8 + idx * 0x14; return *(u16 *)(p + off)`.
pub(crate) fn bagt_trick_field(c: &mut Cpu, off: u32) {
    c.cycles += 7;
    c.g[V1][0] = 0x14;
    c.g[V0][0] = 0x46_0000;
    // a2 = idx * 20 (the multiply leaves LO/HI set, as the original does).
    c.mult(0, A2, A2, V1);
    // `v0 = 0x460000 - 0x5148`: the low half of `&D_0045AEB8`.
    c.g[V0][0] = sx(BAGT_TRICKS);
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let ra = c.g[RA][0] as u32;
    c.g[V0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(off));
    c.pc = ra;
}

/// `sTrick *BEBAGTInt_GetTrickTableEntry(void *this, int rider, int idx,
/// int *count)` (0x14ff90): entry `idx` of the 0x14-byte trick table
/// (`D_0045AEB8`), its `+0x10` value in v0 and its `+0xc` count stored in
/// `*count`.
pub fn be_bagt_int_get_trick_table_entry(c: &mut Cpu) {
    c.cycles += 10;
    c.g[V0][0] = 0x14; // entry stride
    c.g[V1][0] = 0x0046_0000;
    c.mult(0, A2, A2, V0);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5148)); // &D_0045AEB8
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lhu(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x10)).lw(c);
    let ret = c.g[RA][0] as u32;
    c.sw(c.g[A3][0] as u32, c.g[A0][0]);
    c.pc = ret;
}

/// `void BEBAGTInt_SetUberTrickButton(BEBAGTInt* self, int player, u32 id,
/// u8 button)` (0x14fd80): write the button byte into the per-character row
/// of the bag of tricks (`0x52eec0 + profile*0x13ec + player*6 + id`).
pub fn bebagt_int_set_uber_trick_button(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        open(c, 0x50);
        let sp0 = spv(c);
        c.sq(18, at(sp0, 0x20));
        c.g[S2][0] = c.g[A1][0];
        c.sq(16, at(sp0, 0x40));
        c.sq(19, at(sp0, 0x10));
        c.g[S0][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A3][0];
        c.sq(17, at(sp0, 0x30));
        c.sd(sp0, c.g[RA][0]);
        c.g[A0][0] = c.g[S2][0];
        callee::BE_GET_RIDER_PROFILE.call(c, 0x0014_fdac)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[S1][0] = c.g[V0][0];
        callee::BE_GET_RIDER_CHARACTER.call(c, 0x0014_fdb8)?;
        c.g[V1][0] = 0x1fe;
        c.g[A0][0] = 6;
        c.mult(0, 5, 2, 3);
        c.mult(0, 16, 16, 4);
        c.g[V0][0] = 0x13ec;
        c.g[V1][0] = 0x530000;
        c.mult(0, 17, 17, 2);
        c.g[V1][0] = sx(at(c.g[V1][0] as u32, 0xec0));
        c.g[V0][0] = 1;
        c.g[S0][0] = sx(at(c.g[S0][0] as u32, c.g[A1][0] as u32));
        c.g[RA][0] = c.ld(sp0);
        c.g[S0][0] = sx(at(c.g[S0][0] as u32, c.g[S1][0] as u32));
        c.g[S0][0] = sx(at(c.g[S0][0] as u32, c.g[V1][0] as u32));
        c.lq(18, at(sp0, 0x20));
        c.sb(at(c.g[S0][0] as u32, 1), c.g[S3][0]);
        c.lq(17, at(sp0, 0x30));
        c.lq(16, at(sp0, 0x40));
        c.lq(19, at(sp0, 0x10));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x50));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BE bag tricks.

/// `int BEBAGTInt_TestUberTrickBitA(void*, int rider, int idx, int bit)`
/// (0x14ffb8): bit `bit` of `flags` in the rider's character bag-trick table.
pub fn bebagt_int_test_uber_trick_bit_a(c: &mut Cpu) {
    bebagt_test_bit(c, 2);
}

/// `int BEBAGTInt_TestUberTrickBitB(void*, int rider, int idx, int bit)`
/// (0x150048): the same for the second flag word.
pub fn bebagt_int_test_uber_trick_bit_b(c: &mut Cpu) {
    bebagt_test_bit(c, 4);
}

/// The shared body of the two "test a bag-trick bit" wrappers.
pub(crate) fn bebagt_test_bit(c: &mut Cpu, flag_offset: u32) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S2][0];
        callee::BE_GET_RIDER_PROFILE_2.call(c, 0x0014_ffe4)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[S1][0] = c.g[V0][0];
        callee::BE_GET_RIDER_CHARACTER_2.call(c, 0x0014_fff0)?;
        c.g[A1][0] = 6;
        c.g[V1][0] = 0x1fe;
        c.mult(0, V0, V0, V1); // character * 510
        c.mult(0, S0, S0, A1); // index * 6
        c.g[A0][0] = 0x13ec;
        c.g[V1][0] = 0x0053_0000;
        c.mult(0, S1, S1, A0); // profile * 0x13ec
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(0xec0));
        frame.restore(c, S2, 0x20);
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
        frame.restore_ra(c, 0);
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, S1)));
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, S0)));
        frame.restore(c, S1, 0x30);
        c.g[V0][0] = Addr(p(r32(c, V1), flag_offset)).lhu(c);
        frame.restore(c, S0, 0x40);
        c.g[V0][0] = sx(((r32(c, V0) as i32) >> (r32(c, S3) & 31)) as u32);
        frame.restore(c, S3, 0x10);
        c.g[V0][0] = c.g[V0][0] & 1;
        frame.close(c);
        Ok(())
    })
}

/// `void BEBAGTInt_ClearUberTrickBit(rider, a1, slot, bit)` (0x14fe08): clear
/// the given bit in the rider's bag trick record.
pub fn be_bagt_int_clear_uber_trick_bit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        callee::BE_GET_RIDER_PROFILE_3.call(c, 0x0014_fe34)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[S2][0] = c.g[V0][0];
        callee::BE_GET_RIDER_CHARACTER_3.call(c, 0x0014_fe40)?;
        c.g[V1][0] = 0x1fe;
        c.g[A0][0] = 6;
        c.mult(0, V0, V0, V1);
        c.mult(0, S0, S0, A0);
        c.g[V1][0] = 0x13ec;
        c.g[A1][0] = 0x0053_0ec2;
        c.mult(0, S2, S2, V1);
        c.g[A0][0] = 1;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = sx((c.g[A0][0] as u32) << (c.g[S3][0] as u32 & 31));
        c.g[A0][0] = !(c.g[A0][0] | 0);
        frame.restore(c, S1, 0x30);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
        frame.restore(c, S3, 0x10);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        frame.restore(c, S2, 0x20);
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lhu(c);
        c.g[V0][0] = 1;
        c.g[RA][0] = Addr(c.g[SP][0] as u32).ld(c);
        c.g[V1][0] &= c.g[A0][0];
        Addr(c.g[S0][0] as u32).sh(c, c.g[V1][0]);
        frame.restore(c, S0, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BEBAGTInt_GetUberTrickSlotValue (0x14fea8, beintbagt.cpp)

/// Uber trick table (`D_0045AEB8`, 0x14-byte entries; +0xc: slot enabled).
pub(crate) const OC264_UBER_TRICKS: u32 = 0x0045_aeb8;

pub(crate) const OC264_UBER_TRICK_SIZE: u64 = 0x14;

pub(crate) const OC264_UBER_TRICK_ENABLED: u32 = 0xc;

/// Per profile/character/trick slot bytes (`D_00530EC0`): 6 bytes per
/// trick, 0x1fe per character, 0x13ec per profile.
pub(crate) const OC264_UBER_SLOTS: u32 = 0x0053_0ec0;

pub(crate) const OC264_BE_GET_RIDER_PROFILE: Callee = Callee {
    addr: 0x0014_a0e0,
    f: ee::s01::BE_GetRiderProfile,
};

pub(crate) const OC264_BE_GET_RIDER_CHARACTER: Callee = Callee {
    addr: 0x0014_a080,
    f: ee::s01::BE_GetRiderCharacter,
};

/// `int BEBAGTInt_GetUberTrickSlotValue(void* self, int rider, int trick,
/// int second)` (0x14fea8): -1 when uber trick `trick` has no slot,
/// else the first (or, with `second`, the second) slot byte of that trick
/// for the rider's profile and character.
pub fn bebagt_int_get_uber_trick_slot_value(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x50);
        c.g[A0][0] = OC264_UBER_TRICK_SIZE;
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S1, 0x30);
        c.mult(0, A0, S0, A0);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[V0][0] = sx(OC264_UBER_TRICKS);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        let entry = r32(c, A0).wrapping_add(OC264_UBER_TRICKS);
        c.g[A0][0] = sx(entry);
        c.g[V1][0] = Addr(entry.wrapping_add(OC264_UBER_TRICK_ENABLED)).lhu(c);
        c.g[S2][0] = c.g[A3][0];
        if c.g[V1][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = u64::MAX;
        } else {
            c.cycles += 7;
            c.g[A0][0] = c.g[S1][0];
            OC264_BE_GET_RIDER_PROFILE.call(c, 0x0014_fef8)?;
            c.g[S3][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            OC264_BE_GET_RIDER_CHARACTER.call(c, 0x0014_ff04)?;
            c.g[V1][0] = c.g[V0][0];
            let second = c.g[S2][0] != 0;
            c.cycles += if second { 13 } else { 12 };
            // slots + trick * 6 + character * 0x1fe + profile * 0x13ec
            c.g[A1][0] = 0x1fe;
            c.g[V0][0] = 6;
            c.mult(0, A1, V1, A1);
            c.mult(0, V0, S0, V0);
            c.g[V1][0] = 0x13ec;
            c.mult(0, V1, S3, V1);
            c.g[A0][0] = sx(OC264_UBER_SLOTS);
            let at = r32(c, V0)
                .wrapping_add(r32(c, A1))
                .wrapping_add(r32(c, V1))
                .wrapping_add(OC264_UBER_SLOTS);
            c.g[V0][0] = Addr(at.wrapping_add(second as u32)).lbu(c);
        }
        c.cycles += 7;
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
// BEBAGTInt_GetTrickAnim (0x150198)

/// `BE_GetRiderAltCharacter(rider)`: the rider's alternate character id.
pub(crate) const TRICK_ANIM_ALT_CHARACTER: Callee = Callee {
    addr: 0x0014_a0b0,
    f: ee::s01::BE_GetRiderAltCharacter,
};

/// `BEBAGTInt_GetUberTrickSlotValue(self, rider, trick, second)`: the slot
/// of the rider's animation for trick `trick` (-1 when the trick has none).
pub(crate) const TRICK_ANIM_SLOT_VALUE: Callee = Callee {
    addr: 0x0014_fea8,
    f: ee::s01::BEBAGTInt_GetUberTrickSlotValue,
};

/// The trick table (0x45aeb8): 0x14-byte entries, `anims` (8-byte entries,
/// the animation id a `u16` at +0) at +0x10.
pub(crate) const TRICK_ANIM_TABLE: u32 = 0x0045_aeb8;

pub(crate) const TRICK_ANIM_ENTRY_SIZE: u32 = 0x14;

pub(crate) const TRICK_ANIM_ANIMS: u32 = 0x10;

/// The animation returned when the trick has no slot.
pub(crate) const TRICK_ANIM_NONE: u64 = 0x1b6;

/// The alternate characters with their own animation for one trick:
/// 0x18 -> 0x9f, 0x14 -> 0xa0, 0x1c -> 0xa1 (trick 4), 0x19 -> 0xa2 (trick 9).
///
/// Sets the registers and cycles the original's compare chain leaves;
/// returns the special animation (already in v0) or `None` when the
/// generic lookup runs (a0 = self, a1 = rider set).
pub(crate) fn trick_anim_special(c: &mut Cpu, ch: u64) -> Option<u64> {
    c.g[V1][0] = ch;
    c.g[V0][0] = 0x18;
    // `trick == want ? special : generic` tail, with a0/a1 as the branch
    // leaves them (`a0_first`: a0 is set in this block, else a1).
    let pick = |c: &mut Cpu, want: u64, special: u64, a0_first: bool| -> Option<u64> {
        c.cycles += 3;
        c.g[V0][0] = want;
        if a0_first {
            c.g[A0][0] = c.g[S2][0];
        } else {
            c.g[A1][0] = c.g[S1][0];
        }
        if c.g[S0][0] != want {
            if a0_first {
                c.cycles += 1;
                c.g[A1][0] = c.g[S1][0];
            }
            None
        } else {
            c.cycles += 2;
            c.g[V0][0] = special;
            Some(special)
        }
    };
    if ch == 0 {
        c.cycles += 2;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S1][0];
        return None;
    }
    c.cycles += 2;
    c.g[V0][0] = ((ch as i64) < 25) as u64;
    if ch == 0x18 {
        return pick(c, 4, 0x9f, true);
    }
    c.cycles += 2;
    let below_25 = c.g[V0][0] != 0;
    c.g[V0][0] = 0x14;
    if below_25 {
        c.cycles += 2;
        c.g[A0][0] = c.g[S2][0];
        if ch == 0x14 {
            return pick(c, 4, 0xa0, false);
        }
    } else {
        c.cycles += 3;
        c.g[V0][0] = 0x1c;
        if ch == 0x19 {
            return pick(c, 9, 0xa2, true);
        }
        c.cycles += 2;
        c.g[A0][0] = c.g[S2][0];
        if ch == 0x1c {
            return pick(c, 4, 0xa1, false);
        }
    }
    c.cycles += 2;
    c.g[A1][0] = c.g[S1][0];
    None
}

/// `int BEBAGTInt_GetTrickAnim(BEBAGTInt* self, int rider, int trick, int
/// second)` (0x150198): the animation a rider plays for a trick. Some
/// alternate characters have a fixed animation for one trick (0x14, 0x18 and
/// 0x1c for trick 4, 0x19 for trick 9); otherwise look up the rider's slot
/// for the trick and return `trick_table[trick].anims[slot]`, or 0x1b6 when
/// the trick has no slot.
pub fn bebagt_int_get_trick_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A3][0];
        c.g[A0][0] = c.g[S1][0];
        TRICK_ANIM_ALT_CHARACTER.call(c, 0x0015_01c8)?;
        let ch = c.g[V0][0];
        if trick_anim_special(c, ch).is_none() {
            c.cycles += 6;
            c.g[A3][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S0][0];
            TRICK_ANIM_SLOT_VALUE.call(c, 0x0015_0274)?;
            let slot = c.g[V0][0];
            c.g[V1][0] = slot;
            c.g[A1][0] = TRICK_ANIM_ENTRY_SIZE as u64;
            if (slot as i64) < 0 {
                c.cycles += 1;
                c.g[V0][0] = TRICK_ANIM_NONE;
            } else {
                c.cycles += 9;
                let trick = r32(c, S0);
                let entry_off = abi::mult(c, trick, TRICK_ANIM_ENTRY_SIZE);
                c.g[A1][0] = sx(entry_off);
                let entry = TRICK_ANIM_TABLE.wrapping_add(entry_off);
                c.g[V0][0] = sx(entry);
                let anims = Addr(entry.wrapping_add(TRICK_ANIM_ANIMS)).lw(c);
                c.g[A0][0] = anims;
                let at = (slot as u32).wrapping_shl(3).wrapping_add(anims as u32);
                c.g[V1][0] = sx(at);
                c.g[V0][0] = Addr(at).lhu(c);
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;

    use crate::difftest::fakes::machine;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::bx::bxstring2::tests::SCRATCH;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_2;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w;

    pub(crate) use crate::game::sound::bankmanager::tests::mixed;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS;
    use ssx3_ee as ee;

    #[test]
    fn be_bagt_int_get_instance_matches() {
        check(
            ssx3_ee::s01::f_0014f900,
            be_bagt_int_get_instance,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0xf900, 0);
                c.bus.write32(
                    GAME_GP.wrapping_sub(0x1ee0),
                    if seed % 3 == 0 { 0 } else { DATA + 0x100 },
                );
                let _ = &mut r;
            },
        );
    }

    #[test]
    fn bebagtint_get_trick_field0_matches() {
        check_lohi(
            ee::s01::f_001500d8__generated,
            bebagtint_get_trick_field0,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0015_00d8, seed);
                scramble(c, &mut r);
                let index = (seed % 8) as u32;
                c.g[A2][0] = mixed(&mut r, seed, index);
                c.bus
                    .write16(TRICK_TABLE + index * 0x14, r.next_u32() as u16);
            },
        );
    }

    #[test]
    fn bebagtint_get_trick_field2_matches() {
        check_lohi(
            ee::s01::f_001500f8__generated,
            bebagtint_get_trick_field2,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0015_00f8, seed);
                scramble(c, &mut r);
                let index = (seed % 8) as u32;
                c.g[A2][0] = mixed(&mut r, seed, index);
                c.bus
                    .write16(TRICK_TABLE + index * 0x14 + 2, r.next_u32() as u16);
            },
        );
    }

    #[test]
    fn bebagtint_get_trick_field4_matches() {
        check_lohi(
            ee::s01::f_00150118__generated,
            bebagtint_get_trick_field4,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0015_0118, seed);
                scramble(c, &mut r);
                let index = (seed % 8) as u32;
                c.g[A2][0] = mixed(&mut r, seed, index);
                c.bus
                    .write16(TRICK_TABLE + index * 0x14 + 4, r.next_u32() as u16);
            },
        );
    }

    #[test]
    fn bebagtint_get_trick_field6_matches() {
        check_lohi(
            ee::s01::f_00150138__generated,
            bebagtint_get_trick_field6,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0015_0138, seed);
                scramble(c, &mut r);
                let index = (seed % 8) as u32;
                c.g[A2][0] = mixed(&mut r, seed, index);
                c.bus
                    .write16(TRICK_TABLE + index * 0x14 + 6, r.next_u32() as u16);
            },
        );
    }

    // -----------------------------------------------------------------------
    // BAGT trick records.

    pub(crate) fn trick_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x1501_58 ^ seed as u64);
        scramble(c, &mut r);
        let idx = (r.next_u32() % 40) as i32 - 4;
        c.g[A2][0] = sx(idx as u32);
        let base = 0x0045_aeb8u32.wrapping_add((idx as u32).wrapping_mul(0x14));
        for k in 0..10u32 {
            c.bus.write16(base.wrapping_add(2 * k), r.next_u32() as u16);
        }
    }

    #[test]
    fn bebagtint_get_trick_field8_matches() {
        check_lohi(
            ee::s01::f_00150158__generated,
            bebagtint_get_trick_field8,
            200,
            &trick_setup,
        );
    }

    #[test]
    fn bebagtint_get_trick_fielda_matches() {
        check_lohi(
            ee::s01::f_00150178__generated,
            bebagtint_get_trick_fielda,
            200,
            &trick_setup,
        );
    }

    #[test]
    fn be_bagt_int_get_trick_table_entry_matches() {
        check(
            ee::s01::f_0014ff90__generated,
            be_bagt_int_get_trick_table_entry,
            200,
            &|c, seed| {
                let mut r = Rng(0x14ff_9000 ^ seed as u64);
                crate::difftest::scramble(c, &mut r);
                let index = seed % 8;
                c.g[A2][0] = index as u64;
                c.g[A3][0] = (SCRATCH + 0x1200) as u64;
                let table = 0x0045_aeb8u32;
                for i in 0..8u32 {
                    let e = table + 0x14 * i;
                    c.bus.write16(e + 0xc, r.next_u32() as u16);
                    c.bus.write32(e + 0x10, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn bebagt_int_set_uber_trick_button_matches() {
        check(
            ee::s01::f_0014fd80__generated,
            bebagt_int_set_uber_trick_button,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x14fd80, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 20) as u64;
                c.g[A2][0] = sx(DATA);
                c.g[A3][0] = (seed % 256) as u64;
                // The race setup table: player 0 in slot 0, whose record has a
                // profile word of 0 and character 0.
                c.bus.write32(0x0053_05b0, 0);
                c.bus.write32(0x0053_5b20 + 0xc, 0);
                c.bus.write8(0x0053_5b20 + 0x11, 0);
            },
        );
    }

    #[test]
    fn bebagt_int_test_uber_trick_bit_a_matches() {
        check(
            ee::s01::f_0014ffb8__generated,
            bebagt_int_test_uber_trick_bit_a,
            100,
            &|c, seed| {
                let _ = base(c, seed, 0x14_ffb8);
                globals(c);
                w(c, DATA, if seed % 2 == 0 { 0xffff } else { 0xaaaa });
                c.g[A0][0] = 0;
                c.g[A1][0] = sx(seed % 8);
                c.g[A2][0] = sx(seed % 4);
                c.g[A3][0] = sx(seed % 16);
            },
        );
    }

    #[test]
    fn bebagt_int_test_uber_trick_bit_b_matches() {
        check(
            ee::s01::f_00150048__generated,
            bebagt_int_test_uber_trick_bit_b,
            100,
            &|c, seed| {
                let _ = base(c, seed, 0x15_0048);
                globals(c);
                w(c, DATA, if seed % 2 == 0 { 0xffff } else { 0x5555 });
                c.g[A0][0] = 0;
                c.g[A1][0] = sx(seed % 8);
                c.g[A2][0] = sx(seed % 4);
                c.g[A3][0] = sx(seed % 16);
            },
        );
    }

    #[test]
    fn be_bagt_int_clear_uber_trick_bit_matches() {
        check_lohi(
            ssx3_ee::s01::f_0014fe08__generated,
            be_bagt_int_clear_uber_trick_bit,
            10,
            &|c, seed| {
                let _r = base_2(c, seed, 0x14fe08, 3);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = (seed % 4) as u64;
                c.g[A3][0] = (seed % 16) as u64;
                for i in 0..0x2000u32 {
                    c.bus.write8(0x0053_0ec2 + i, seed.wrapping_add(i) as u8);
                }
            },
        );
    }

    /// BEBAGTInt_GetUberTrickSlotValue: tricks with and without a slot,
    /// first and second byte; the real BE_GetRiderProfile/Character read a
    /// rider table whose profile entries are -1 (profile 2) or flag words.
    #[test]
    fn bebagt_int_get_uber_trick_slot_value_matches() {
        const RIDER_SLOTS: u32 = 0x0053_05b0;
        const RIDERS: u32 = 0x0053_5b20;
        check(
            ee::s01::f_0014fea8__generated,
            bebagt_int_get_uber_trick_slot_value,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x14_fea8, 0);
                c.pc = 0x0014_fea8;
                let rider = seed % 4;
                let trick = r.next_u32() % 24;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(rider);
                c.g[A2][0] = sx(trick);
                c.g[A3][0] = if seed % 3 == 0 {
                    0
                } else {
                    sx(r.next_u32() | 1)
                };
                let enabled = if seed % 5 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 0xffff
                };
                c.bus.write16(
                    OC264_UBER_TRICKS + 0x14 * trick + OC264_UBER_TRICK_ENABLED,
                    enabled as u16,
                );
                let slot = r.next_u32() % 4;
                c.bus.write32(RIDER_SLOTS + 4 * rider, slot);
                let rec = RIDERS + 0x1c * slot;
                let profile = if seed % 2 == 0 {
                    u32::MAX
                } else {
                    r.next_u32()
                };
                c.bus.write32(rec + 0xc, profile);
                c.bus.write32(rec + 0x10, r.next_u32());
                c.bus.write8(rec + 0x11, (r.next_u32() % 16) as u8);
                // The slot bytes this trick/character/profile can reach.
                for p in 0..3u32 {
                    for ch in 0..16u32 {
                        let at = OC264_UBER_SLOTS + 6 * trick + 0x1fe * ch + 0x13ec * p;
                        let v = r.next_u32();
                        c.bus.write8(at, v as u8);
                        c.bus.write8(at + 1, (v >> 8) as u8);
                    }
                }
            },
        );
    }

    /// BEBAGTInt_GetTrickAnim: the real character/profile getters over a
    /// test rider table (alternate character 0, each special one, and
    /// others), the real slot lookup over a test trick table (tricks with
    /// and without animations) and slot bytes.
    #[test]
    fn bebagt_int_get_trick_anim_matches() {
        use crate::difftest::{check_lohi, elf_image, fill, scramble, Rng, DATA, GAME_GP};
        const RIDER_PROFILES: u32 = 0x0053_05b0;
        const PROFILES: u32 = 0x0053_5b20;
        const SLOTS: u32 = 0x0053_0ec0;
        check_lohi(
            ee::s01::f_00150198__generated,
            bebagt_int_get_trick_anim,
            300,
            &|c, seed| {
                let mut r = Rng(0x0015_0198 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                elf_image(c);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x0015_0198;
                let rider = r.next_u32() % 4;
                let profile = r.next_u32() % 3;
                c.bus.write32(RIDER_PROFILES + 4 * rider, profile);
                let entry = PROFILES + 0x1c * profile;
                fill(c, &mut r, entry, 7);
                c.bus.write32(
                    entry + 0xc,
                    if seed % 5 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32()
                    },
                );
                c.bus.write8(entry + 0x11, (r.next_u32() % 8) as u8);
                let chars = [0u8, 0x14, 0x18, 0x19, 0x1c, 5, 30, 0xf0];
                c.bus.write8(entry + 0x12, chars[(seed % 8) as usize]);
                for i in 0..0x4000u32 {
                    c.bus.write8(SLOTS + i, (mix_small(seed, i) % 8) as u8);
                }
                for t in 0..12u32 {
                    let e = TRICK_ANIM_TABLE + TRICK_ANIM_ENTRY_SIZE * t;
                    c.bus.write16(e + 0xc, if t % 3 == 0 { 0 } else { 8 });
                    let anims = DATA + 0x1000 + 0x100 * t;
                    c.bus.write32(e + TRICK_ANIM_ANIMS, anims);
                    fill(c, &mut r, anims, 0x40);
                }
                let trick = match seed % 4 {
                    0 => 4,
                    1 => 9,
                    _ => r.next_u32() % 12,
                };
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = rider as u64;
                c.g[A2][0] = trick as u64;
                c.g[A3][0] = (r.next_u32() % 2) as u64;
            },
        );
        fn mix_small(seed: u32, i: u32) -> u32 {
            (seed.wrapping_mul(0x9e37_79b9) ^ i.wrapping_mul(0x85eb_ca6b)).rotate_left(13)
        }
    }
}
