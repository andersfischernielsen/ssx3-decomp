//! `src/ealib/seg_2b4578.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::call::Callee;
use ssx3_ee as ee;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x003b_47f8, eac_unpack, "EAC_Unpack")];

/// `RefPack_Unpack` (0x3b4740).
pub(crate) const OC327_REFPACK_UNPACK: Callee = Callee {
    addr: 0x003b_4740,
    f: ee::s13::RefPack_Unpack,
};

// ---------------------------------------------------------------------------
// Streaming / sound DSP.

/// `void* EAC_Unpack(...)` (0x3b47f8): a one-call wrapper over
/// `RefPack_Unpack` (the EA "refpack" decompressor).
pub fn eac_unpack(c: &mut Cpu) {
    crate::call::tail_wrapper(
        c,
        None,
        crate::call::Target::Direct(OC327_REFPACK_UNPACK),
        0x003b_4808,
    );
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    use ssx3_ee_rt::cpu::reg::*;
    use ssx3_ee_rt::cpu::sx;

    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::fake;
    use crate::difftest::fakes::machine;

    use crate::difftest::DATA;

    use std::sync::atomic::AtomicUsize;

    // ------------------------------------------------------------ fake callees

    /// Control word of this module's gated fakes (in the machine's RAM, so it
    /// never leaks into another test even when tests share a process).
    pub(crate) const OC327_CTRL: u32 = DATA + 0x3e_0000;

    pub(crate) const OC327_MAGIC: u32 = 0x0c32_7f00;

    /// One forced result word per fake.
    pub(crate) const OC327_RET: u32 = DATA + 0x3e_0010;

    pub(crate) const RET_REFPACK: u32 = 0;

    /// A logging fake when this module's control word is set, otherwise
    /// whatever was installed before (chained, as `oc318` does).
    pub(crate) fn oc327_fake_gated(
        c: &mut Cpu,
        addr: u32,
        ret_idx: u32,
        prev: &AtomicUsize,
        fallback: fn(&mut Cpu),
    ) {
        if c.bus.read32(OC327_CTRL) == OC327_MAGIC {
            let ra = c.g[RA][0] as u32;
            fake(c, addr, ra);
            if c.pc == ra {
                // Sign-extended, as `abi::set_ret` leaves it: `-1` is a
                // meaningful result for the script lookup.
                c.g[V0][0] = sx(c.bus.read32(OC327_RET + 4 * ret_idx));
            }
            return;
        }
        crate::difftest::fakes::fall_back(c, prev, fallback)
    }

    macro_rules! oc327_hook {
        ($fake:ident, $prev:ident, $addr:expr, $ret:expr, $fallback:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                oc327_fake_gated(c, $addr, $ret, &$prev, $fallback);
            }
        };
    }
    pub(crate) use oc327_hook;

    oc327_hook!(
        oc327_fake_refpack,
        OC327_PREV_REFPACK,
        0x003b_4740,
        RET_REFPACK,
        ee::s13::f_003b4740__generated
    );

    pub(crate) fn oc327_install_refpack() {
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_003B4740),
            oc327_fake_refpack,
            &OC327_PREV_REFPACK,
        );
    }

    /// `EAC_Unpack`: a one-call wrapper (the decompressor is a fake).
    #[test]
    fn eac_unpack_matches() {
        oc327_install_refpack();
        covered(
            ee::s13::f_003b47f8__generated,
            eac_unpack,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x003b_47f8, 1);
                c.pc = 0x003b_47f8;
                c.bus.write32(OC327_CTRL, OC327_MAGIC);
                c.bus.write32(OC327_RET + 4 * RET_REFPACK, 0x1000 + seed);
                c.g[A0][0] = sx(DATA + 0x200);
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = sx(DATA + 0x600);
                c.g[A3][0] = sx(DATA + 0x800);
            },
        );
    }
}
