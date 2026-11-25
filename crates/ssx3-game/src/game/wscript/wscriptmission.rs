//! `src/wscript/wscriptmission.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Frame;
pub(crate) use crate::game::luno::lunovm::oc640_field;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

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
