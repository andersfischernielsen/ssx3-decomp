//! Staging module oc671 (wave wave236): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc671`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

use crate::game::input::inputmap::{
    COMPILE_BINDINGS as IM671_BINDINGS, STR_INPUT_MAP as IM671_STR_INPUT_MAP,
};

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0022_6b60,
    oc671_input_map_load_from_file,
    "InputMap_LoadFromFile",
)];

/// `void InputMap_LoadFromFile(void *self, const char *path)` (0x226b60),
/// `cSSXApp_loadInputMap` in the reference. Reads the map file at `path`
/// (its `#include`s expanded) into a new 0x18-byte `InputMap` initialised for
/// 205 bindings, frees the text, compiles the bindings and purges the file.
/// Returns the map in v0.
///
/// The first callee, `InputPreproc_Run` (0x325f48), has no slot; the test runs
/// it for real with its own callees (`InputPreproc_ProcessFile`, the
/// allocators) faked in their slots.
pub fn oc671_input_map_load_from_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        c.g[A0][0] = c.g[SP][0];
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0x20);
        // `size` out-parameter in the frame.
        c.g[A2][0] = sx(frame.at(c, 0x10));
        call_ptr(c, 0x0032_5f48, 0x0022_6b7c)?;
        c.g[S1][0] = c.g[V0][0];

        // `new InputMap` (0x18 bytes, tagged str_InputMap), then `init(205)`.
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A1][0] = IM671_STR_INPUT_MAP;
        c.g[A0][0] = 0x18;
        call_ptr(c, 0x0031_7d70, 0x0022_6b98)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 0xcd;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0032_0ca0, 0x0022_6ba8)?;

        // loadMapFile(map, data, size): the size is read back from the frame.
        c.g[A2][0] = Addr(frame.at(c, 0x10)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0032_0db0, 0x0022_6bb8)?;

        // Free the text when the loader returned one.
        c.g[A2][0] = 0x0048_0000;
        if c.g[S1][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e98, 0x0022_6bc8)?;
            c.g[A2][0] = 0x0048_0000;
        }
        c.cycles += 1032;

        // compileMap(map, index, name) for every binding; a1 and a2 are left
        // as the last call set them.
        for (index, &(ret, name, _)) in IM671_BINDINGS.iter().enumerate() {
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = index as u64;
            c.g[A2][0] = sx(name);
            call_ptr(c, 0x0032_0e18, ret)?;
        }

        // purgeMapFile(map); the result is the map.
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0032_0f10, 0x0022_7bd4)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x30);
        frame.restore(c, S0, 0x40);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate of the callee fakes (otherwise the body they replaced runs).
    const OC671_CTRL: u32 = DATA + 0x000e_4000;
    const OC671_MAGIC: u32 = 0x494d_5036;
    /// Call index at which the `operator_new_named` fake returns 0.
    const OC671_ZERO_AT: u32 = DATA + 0x000e_4004;
    const OC671_PATH: u32 = DATA + 0x2000;

    /// A gated fake for one callee slot: logs the call and returns.
    macro_rules! oc671_fake {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC671_CTRL) != OC671_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                let n = c.bus.read32(fakes::LOG_COUNT);
                fakes::fake_call(c, $addr);
                if $addr == 0x0031_7e30 && n == c.bus.read32(OC671_ZERO_AT) {
                    c.g[V0][0] = 0;
                }
            }
        };
    }
    oc671_fake!(
        oc671_f_new,
        OC671_PREV_NEW,
        0x0031_7e30,
        ee::s10::f_00317e30__generated
    );
    oc671_fake!(
        oc671_f_alloc,
        OC671_PREV_ALLOC,
        0x0031_7d70,
        ee::s10::f_00317d70__generated
    );
    oc671_fake!(
        oc671_f_free,
        OC671_PREV_FREE,
        0x0031_7e98,
        ee::s10::f_00317e98__generated
    );
    oc671_fake!(
        oc671_f_init,
        OC671_PREV_INIT,
        0x0032_0ca0,
        ee::s10::f_00320ca0__generated
    );
    oc671_fake!(
        oc671_f_parser,
        OC671_PREV_PARSER,
        0x0032_0db0,
        ee::s10::f_00320db0__generated
    );
    oc671_fake!(
        oc671_f_compile,
        OC671_PREV_COMPILE,
        0x0032_0e18,
        ee::s10::f_00320e18__generated
    );
    oc671_fake!(
        oc671_f_purge,
        OC671_PREV_PURGE,
        0x0032_0f10,
        ee::s10::f_00320f10__generated
    );
    oc671_fake!(
        oc671_f_process,
        OC671_PREV_PROCESS,
        0x0032_6078,
        ee::s10::f_00326078__generated
    );

    fn oc671_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc671_f_new,
            &OC671_PREV_NEW,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc671_f_alloc,
            &OC671_PREV_ALLOC,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc671_f_free,
            &OC671_PREV_FREE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00320CA0),
            oc671_f_init,
            &OC671_PREV_INIT,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00320DB0),
            oc671_f_parser,
            &OC671_PREV_PARSER,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00320E18),
            oc671_f_compile,
            &OC671_PREV_COMPILE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00320F10),
            oc671_f_purge,
            &OC671_PREV_PURGE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00326078),
            oc671_f_process,
            &OC671_PREV_PROCESS,
        );
    }

    /// `InputMap_LoadFromFile(self, path)`: loads the input map file (205
    /// bindings) through the preprocessor, allocates and compiles the map.
    /// `InputPreproc_Run` runs for real; the file reader and every allocator
    /// and map call is a gated fake. Fixture: a path with or without a
    /// directory part, a zero text pointer (no free) on some seeds, and
    /// unwinds at random calls.
    #[test]
    fn oc671_input_map_load_from_file_matches() {
        oc671_install_fakes();
        check(
            ee::s05::f_00226b60__generated,
            oc671_input_map_load_from_file,
            90,
            &|c, seed| {
                let _ = fakes::machine(c, seed, 0x0671_0000, 215);
                c.pc = 0x0022_6b60;
                c.bus.write32(OC671_CTRL, OC671_MAGIC);
                // The 3rd allocation call of the preprocessor is the text copy
                // (new #0, ProcessFile #1, new #2); zero it on every 3rd seed.
                let zero = if seed % 3 == 0 { 2 } else { u32::MAX };
                c.bus.write32(OC671_ZERO_AT, zero);
                let path: &[u8] = if seed % 2 == 0 {
                    b"data/input/ssx.map\0"
                } else {
                    b"ssx.map\0"
                };
                c.bus.write_bytes(OC671_PATH, path);
                c.g[A0][0] = sx(DATA + 0x3000);
                c.g[A1][0] = sx(OC671_PATH);
            },
        );
    }
}
