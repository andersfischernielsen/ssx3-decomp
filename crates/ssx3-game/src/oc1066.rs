//! Staging module oc1066 (wave oc1066): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1066`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003f_5da8, usb_kb_load_modules, "UsbKb_LoadModules"),
    (0x003f_5e90, usb_kb_unload_module, "UsbKb_UnloadModule"),
    (0x003f_5f08, usb_kb_request_info, "UsbKb_RequestInfo"),
    (0x003f_5f88, usb_kb_sync_info, "UsbKb_SyncInfo"),
    (0x003f_6040, usb_kb_find_next_ready, "UsbKb_FindNextReady"),
    (0x003f_60b8, usb_kb_request_read, "UsbKb_RequestRead"),
    (0x003f_6138, usb_kb_sync_read, "UsbKb_SyncRead"),
    (0x003f_61d0, usb_kb_init, "UsbKb_Init"),
    (0x003f_62f8, usb_kb_shutdown, "UsbKb_Shutdown"),
    (0x003f_6348, usb_kb_get_max_keyboards, "UsbKb_GetMaxKeyboards"),
    (0x003f_6358, usb_kb_default_connect_cb, "UsbKb_DefaultConnectCb"),
    (0x003f_63f0, usb_kb_update, "UsbKb_Update"),
    (0x003f_64e0, usb_kb_get_data, "UsbKb_GetData"),
    (0x003f_6520, usb_kb_is_connected, "UsbKb_IsConnected"),
    (0x003f_6538, usb_kb_any_connected, "UsbKb_AnyConnected"),
    (0x003f_65a0, usb_kb_get_keypad_entry, "UsbKb_GetKeypadEntry"),
    (0x003f_65f8, usb_kb_lookup_key_char, "UsbKb_LookupKeyChar"),
    (0x003f_6670, usb_kb_convert_raw_code, "UsbKb_ConvertRawCode"),
    (0x003f_6700, usb_kb_check_modifier_key, "UsbKb_CheckModifierKey"),
    (0x003f_67e0, usb_kb_check_led_key, "UsbKb_CheckLedKey"),
    (0x003f_6838, usb_kb_is_key_down, "UsbKb_IsKeyDown"),
    (0x003f_68f0, usb_kb_translate_key, "UsbKb_TranslateKey"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::{ee, Callee};

    pub const STRCPY: u32 = 0x0041_66f4;

    pub const STRCAT: u32 = 0x0041_62d0;

    pub const STRLEN: u32 = 0x0041_6810;

    pub const SCE_SIF_LOAD_MODULE: u32 = 0x0042_ba68;

    pub const SCE_SIF_STOP_MODULE: u32 = 0x0042_b438;

    pub const SCE_SIF_UNLOAD_MODULE: u32 = 0x0042_b640;

    pub const SCE_USB_KB_INIT: u32 = 0x0040_c028;

    pub const SCE_USB_KB_END: u32 = 0x0040_c220;

    pub const SCE_USB_KB_GET_INFO: u32 = 0x0040_c2e8;

    pub const SCE_USB_KB_READ: u32 = 0x0040_c390;

    pub const SCE_USB_KB_SET_LED_STATUS: u32 = 0x0040_c550;

    pub const SCE_USB_KB_SET_LED_MODE: u32 = 0x0040_c628;

    pub const SCE_USB_KB_SET_REPEAT: u32 = 0x0040_c728;

    pub const SCE_USB_KB_SET_CODE_TYPE: u32 = 0x0040_c770;

    pub const SCE_USB_KB_SYNC: u32 = 0x0040_c810;

    pub const SCE_USB_KB_CNV_RAW_CODE: u32 = 0x0040_d268;

    pub const SCE_USB_KB_CLEAR_RBUF: u32 = 0x0040_d4a0;

    pub const USB_KB_LOAD_MODULES: Callee = Callee {
        addr: 0x003f_5da8,
        f: ee::s14::f_003f5da8,
    };

    pub const USB_KB_UNLOAD_MODULE: Callee = Callee {
        addr: 0x003f_5e90,
        f: ee::s14::f_003f5e90,
    };

    pub const USB_KB_REQUEST_INFO: Callee = Callee {
        addr: 0x003f_5f08,
        f: ee::s14::f_003f5f08,
    };

    pub const USB_KB_SYNC_INFO: Callee = Callee {
        addr: 0x003f_5f88,
        f: ee::s14::f_003f5f88,
    };

    pub const USB_KB_FIND_NEXT_READY: Callee = Callee {
        addr: 0x003f_6040,
        f: ee::s14::f_003f6040,
    };

    pub const USB_KB_REQUEST_READ: Callee = Callee {
        addr: 0x003f_60b8,
        f: ee::s14::f_003f60b8,
    };

    pub const USB_KB_SYNC_READ: Callee = Callee {
        addr: 0x003f_6138,
        f: ee::s14::f_003f6138,
    };

    pub const USB_KB_GET_MAX_KEYBOARDS: Callee = Callee {
        addr: 0x003f_6348,
        f: ee::s14::f_003f6348,
    };

    pub const USB_KB_UPDATE: Callee = Callee {
        addr: 0x003f_63f0,
        f: ee::s14::f_003f63f0,
    };

    pub const USB_KB_IS_CONNECTED: Callee = Callee {
        addr: 0x003f_6520,
        f: ee::s14::f_003f6520,
    };

    pub const USB_KB_GET_KEYPAD_ENTRY: Callee = Callee {
        addr: 0x003f_65a0,
        f: ee::s14::f_003f65a0,
    };

    pub const USB_KB_LOOKUP_KEY_CHAR: Callee = Callee {
        addr: 0x003f_65f8,
        f: ee::s14::f_003f65f8,
    };

    pub const USB_KB_CONVERT_RAW_CODE: Callee = Callee {
        addr: 0x003f_6670,
        f: ee::s14::f_003f6670,
    };

    pub const USB_KB_CHECK_MODIFIER_KEY: Callee = Callee {
        addr: 0x003f_6700,
        f: ee::s14::f_003f6700,
    };

    pub const USB_KB_CHECK_LED_KEY: Callee = Callee {
        addr: 0x003f_67e0,
        f: ee::s14::f_003f67e0,
    };
}

// ---------------------------------------------------------------------------
// The keyboard wrapper's state.

/// The keyboard wrapper's state block (one per game).
pub(crate) const USB_KB_STATE: u32 = 0x0052_5f80;
/// Its low half (`addiu` on the `lui` half the code keeps in a register).
const USB_KB_STATE_LO: u32 = 0x5f80;

// Fields of the state block.
/// Module id of `USBD.IRX` (-1: not loaded).
pub(crate) const USB_KB_USBD_ID: u32 = 0x0;
/// Module id of `USBKB.IRX`.
pub(crate) const USB_KB_USBKB_ID: u32 = 0x4;
/// `sceUsbKbGetInfo`'s buffer.
const USB_KB_INFO: u32 = 0x8;
/// Connection status per keyboard (bytes; inside the info buffer).
pub(crate) const USB_KB_CONNECTED: u32 = 0x10;
/// Keyboards the driver serves (`sceUsbKbInit` writes it; at most 8).
pub(crate) const USB_KB_MAX: u32 = 0x90;
/// Connection status last reported to the connect callback (bytes).
pub(crate) const USB_KB_SEEN: u32 = 0x94;
/// The pending request (`USB_KB_REQ_*`).
pub(crate) const USB_KB_REQUEST: u32 = 0x9c;
/// Byte: a request is in flight.
pub(crate) const USB_KB_BUSY: u32 = 0xa0;
/// Keyboard of the last read.
pub(crate) const USB_KB_READ_PORT: u32 = 0xa4;
/// The caller's keyboard data array (`USB_KB_DATA_SIZE` bytes per keyboard).
pub(crate) const USB_KB_DATA: u32 = 0xa8;
/// The read keyboard's LED and modifier words before the read.
pub(crate) const USB_KB_PREV_LED: u32 = 0xac;
pub(crate) const USB_KB_PREV_MKEY: u32 = 0xb0;
/// Frames since the last info request, and how many frames that may be.
pub(crate) const USB_KB_POLL_COUNT: u32 = 0xb4;
pub(crate) const USB_KB_POLL_PERIOD: u32 = 0xb8;
/// `void (*)(int keyboard, int status)`: told when a keyboard's status changes.
pub(crate) const USB_KB_CONNECT_CB: u32 = 0xbc;
/// `int (*)(int release)`: may hold the library (see [`usb_kb_update`]).
pub(crate) const USB_KB_HOLD_CB: u32 = 0xc0;

// Pending requests.
const USB_KB_REQ_INFO: u64 = 1;
const USB_KB_REQ_READ: u64 = 2;
/// The hold callback holds the library: nothing is requested.
const USB_KB_REQ_HELD: u64 = 3;

/// The info poll period `UsbKb_Init` sets (frames).
const USB_KB_POLL_FRAMES: u64 = 0x3c;
/// `UsbKb_Init` serves at most this many keyboards.
const USB_KB_MAX_KEYBOARDS: u64 = 8;

// One keyboard's data (`sceUsbKbRead`'s buffer).
pub(crate) const USB_KB_DATA_SIZE: u64 = 0x8c;
/// LED word (`USB_KB_LED_*`).
pub(crate) const USB_KB_DATA_LED: u32 = 0x0;
/// Modifier-key word (`USB_KB_MKEY_*`).
pub(crate) const USB_KB_DATA_MKEY: u32 = 0x4;
/// Number of key codes.
pub(crate) const USB_KB_DATA_LEN: u32 = 0x8;
/// Key codes (u16 each).
pub(crate) const USB_KB_DATA_KEYS: u32 = 0xc;
/// Byte: set when a read brought something new, cleared by `UsbKb_GetData`.
pub(crate) const USB_KB_DATA_FRESH: u32 = 0x88;

// LED bits.
const USB_KB_LED_NUM_LOCK: u64 = 0x1;
const USB_KB_LED_CAPS_LOCK: u64 = 0x2;
const USB_KB_LED_SCROLL_LOCK: u64 = 0x4;

// Modifier-key bits.
const USB_KB_MKEY_LCTRL: u64 = 0x01;
const USB_KB_MKEY_LSHIFT: u64 = 0x02;
const USB_KB_MKEY_LALT: u64 = 0x04;
const USB_KB_MKEY_LWIN: u64 = 0x08;
const USB_KB_MKEY_RCTRL: u64 = 0x10;
const USB_KB_MKEY_RSHIFT: u64 = 0x20;
const USB_KB_MKEY_RALT: u64 = 0x40;
const USB_KB_MKEY_RWIN: u64 = 0x80;

// The key codes the game asks about (they match Windows virtual-key codes).
pub(crate) const USB_KB_VK_SHIFT: u64 = 0x10;
pub(crate) const USB_KB_VK_CONTROL: u64 = 0x11;
pub(crate) const USB_KB_VK_MENU: u64 = 0x12;
pub(crate) const USB_KB_VK_CAPITAL: u64 = 0x14;
pub(crate) const USB_KB_VK_LWIN: u64 = 0x5b;
pub(crate) const USB_KB_VK_RWIN: u64 = 0x5c;
pub(crate) const USB_KB_VK_NUMLOCK: u64 = 0x90;
pub(crate) const USB_KB_VK_SCROLL: u64 = 0x91;
pub(crate) const USB_KB_VK_LSHIFT: u64 = 0xa0;
pub(crate) const USB_KB_VK_RSHIFT: u64 = 0xa1;
pub(crate) const USB_KB_VK_LCONTROL: u64 = 0xa2;
pub(crate) const USB_KB_VK_RCONTROL: u64 = 0xa3;
pub(crate) const USB_KB_VK_LMENU: u64 = 0xa4;
pub(crate) const USB_KB_VK_RMENU: u64 = 0xa5;

// USB key usages.
/// The keypad: `/ * - + Enter` (0x54-0x58), then `1`-`9 0 .` (0x59-0x63).
const USB_KB_USAGE_KEYPAD: u32 = 0x54;
const USB_KB_KEYPAD_KEYS: u64 = 0x10;
const USB_KB_USAGE_KEYPAD_1: u32 = 0x59;
const USB_KB_KEYPAD_DIGITS: u64 = 0xb;
/// The usages the other layout maps differently: 0x2e-0x35 and 0x87-0x8b.
const USB_KB_USAGE_ALT_A: u32 = 0x2e;
const USB_KB_USAGE_ALT_A_KEYS: u64 = 8;
const USB_KB_USAGE_ALT_B: u32 = 0x87;
const USB_KB_USAGE_ALT_B_KEYS: u64 = 5;

// Game tables (pairs `{usage, key code}`, read at +1).
/// Pairs indexed by usage (`+ 2 * usage`).
const USB_KB_KEY_TABLE: u32 = 0x0045_1350;
/// The other layout's pairs for 0x2e-0x35 and 0x87-0x8b (indexed by usage).
const USB_KB_KEY_TABLE_ALT_A: u32 = 0x0045_13dc;
const USB_KB_KEY_TABLE_ALT_B: u32 = 0x0045_133a;
/// Num-lock keypad pairs, indexed by `usage - 0x59`.
const USB_KB_KEY_TABLE_NUMPAD: u32 = 0x0045_1420;
/// Keypad characters, indexed by `usage - 0x54`.
const USB_KB_KEYPAD_CHARS: u32 = 0x0045_1458;

/// `UsbKb_TranslateKey`'s result (two bytes; gp-relative).
pub(crate) const USB_KB_KEY_PAIR_GP: u32 = 0x2a20;

/// Module file names (game text).
const USB_KB_USBD_IRX: u32 = 0x0049_7038; // "USBD.IRX"
const USB_KB_USBKB_IRX: u32 = 0x0049_7048; // "USBKB.IRX"
/// `UsbKb_UnloadModule`'s results for the two modules.
const USB_KB_UNLOAD_ERR_USBKB: u64 = 1;
const USB_KB_UNLOAD_ERR_USBD: u64 = 2;

/// The callbacks `UsbKb_Init` installs when given none.
const USB_KB_DEFAULT_CONNECT: u32 = 0x003f_6358; // UsbKb_DefaultConnectCb
const USB_KB_NULL_CALLBACK: u32 = 0x003f_61c8; // UsbKb_NullCallback

/// `sceUsbKbSync` modes.
const USB_KB_SYNC_WAIT: u64 = 0;
const USB_KB_SYNC_POLL: u64 = 1;

/// What `UsbKb_DefaultConnectCb` sets up on a new keyboard: LED status
/// (num lock lit), LED mode 1, key repeat after 30 frames then every 2, code
/// type 0.
const USB_KB_CONNECT_LED_STATUS: u64 = 0x1;
const USB_KB_CONNECT_LED_MODE: u64 = 0x1;
const USB_KB_REPEAT_START: u64 = 0x1e;
const USB_KB_REPEAT_INTERVAL: u64 = 0x2;
const USB_KB_CODE_TYPE: u64 = 0;

/// The `lui` half of `addr` (its low half is below 0x8000 for every address
/// here).
#[inline]
fn usb_kb_lui(addr: u32) -> u64 {
    (addr & 0xffff_0000) as u64
}

/// The state block from the `lui` half in register `reg` (`addiu reg, 0x5f80`).
#[inline]
fn usb_kb_state_from(c: &Cpu, reg: usize) -> u64 {
    sx((c.g[reg][0] as u32).wrapping_add(USB_KB_STATE_LO))
}

// ---------------------------------------------------------------------------
// Modules.

/// `int UsbKb_LoadModules(const char *dir, const char *kb_args)` (0x3f5da8):
/// load `USBD.IRX` and then `USBKB.IRX` (with `kb_args`, its length plus the
/// NUL as the argument size) from the directory `dir` into the IOP, keeping
/// each `sceSifLoadModule` result as the module id (+0, +4). A null `dir`
/// records both ids as -1 and returns 0; a null `kb_args` or a failed load
/// returns -1.
pub fn usb_kb_load_modules(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x140);
        frame.save(c, S0, 0x130);
        frame.save(c, S1, 0x120);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x110);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x100);
        'done: {
            if c.g[S0][0] == 0 {
                c.cycles += 7;
                c.g[V0][0] = usb_kb_lui(USB_KB_STATE);
                c.g[V1][0] = u64::MAX;
                c.g[A0][0] = sx(USB_KB_STATE);
                Addr(USB_KB_STATE + USB_KB_USBD_ID).sw(c, c.g[V1][0]);
                Addr(USB_KB_STATE + USB_KB_USBKB_ID).sw(c, c.g[V1][0]);
                c.g[V0][0] = 0;
                break 'done;
            }
            'failed: {
                c.cycles += 2;
                c.g[A0][0] = c.g[SP][0];
                if c.g[S1][0] == 0 {
                    break 'failed;
                }
                // The path `dir` + "USBD.IRX" in the frame; no arguments.
                c.cycles += 14;
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, callee::STRCPY, 0x003f_5df4)?;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = sx(USB_KB_USBD_IRX);
                call_ptr(c, callee::STRCAT, 0x003f_5e04)?;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                call_ptr(c, callee::SCE_SIF_LOAD_MODULE, 0x003f_5e14)?;
                c.g[V1][0] = usb_kb_lui(USB_KB_STATE);
                c.g[S2][0] = sx(USB_KB_STATE);
                let failed = (c.g[V0][0] as i64) < 0;
                Addr(USB_KB_STATE + USB_KB_USBD_ID).sw(c, c.g[V0][0]);
                if failed {
                    break 'failed;
                }
                // `dir` + "USBKB.IRX" with the keyboard arguments.
                c.cycles += 19;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, callee::STRCPY, 0x003f_5e30)?;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = sx(USB_KB_USBKB_IRX);
                call_ptr(c, callee::STRCAT, 0x003f_5e40)?;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, callee::STRLEN, 0x003f_5e48)?;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                c.g[A2][0] = c.g[S1][0];
                call_ptr(c, callee::SCE_SIF_LOAD_MODULE, 0x003f_5e58)?;
                let failed = (c.g[V0][0] as i64) < 0;
                c.g[V1][0] = failed as u64;
                Addr((c.g[S2][0] as u32).wrapping_add(USB_KB_USBKB_ID)).sw(c, c.g[V0][0]);
                c.g[A0][0] = u64::MAX;
                c.g[V0][0] = if failed { c.g[A0][0] } else { 0 };
                break 'done;
            }
            c.cycles += 1;
            c.g[V0][0] = u64::MAX;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x130);
        frame.restore(c, S1, 0x120);
        frame.restore(c, S2, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `int UsbKb_UnloadModule(int id, int err)` (0x3f5e90): stop and unload
/// the IOP module `id` (nothing for a negative id). Returns 0, or `err` when
/// the stop or the unload failed.
pub fn usb_kb_unload_module(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        c.g[S3][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        if (c.g[S2][0] as i64) >= 0 {
            c.cycles += 11;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            c.g[A3][0] = c.g[SP][0];
            call_ptr(c, callee::SCE_SIF_STOP_MODULE, 0x003f_5ec8)?;
            c.g[S0][0] = u64::MAX;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            c.g[A0][0] = c.g[S2][0];
            if c.g[V0][0] == 0 {
                c.g[S1][0] = c.g[S3][0];
            }
            call_ptr(c, callee::SCE_SIF_UNLOAD_MODULE, 0x003f_5edc)?;
            c.g[S0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[S0][0] == 0 {
                c.g[S1][0] = c.g[S3][0];
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Requests to the driver.

/// `void UsbKb_RequestInfo(void)` (0x3f5f08): remember every keyboard's
/// connection status as reported (+0x94 from +0x10), then ask the driver for
/// new ones (`sceUsbKbGetInfo` into +8); when the request went out, mark it
/// pending (request 1, busy).
pub fn usb_kb_request_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = usb_kb_lui(USB_KB_STATE);
        frame.save(c, S0, 0x10);
        c.g[V1][0] = sx(USB_KB_STATE);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(USB_KB_STATE + USB_KB_MAX).lw(c);
        c.g[A1][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 2;
            c.g[A0][0] = sx(USB_KB_STATE + USB_KB_SEEN);
            loop {
                c.cycles += 7;
                let status_at = (c.g[A0][0] as u32).wrapping_sub(USB_KB_SEEN - USB_KB_CONNECTED);
                c.g[V0][0] = Addr(status_at).lbu(c);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                Addr(c.g[A0][0] as u32).sb(c, c.g[V0][0]);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(USB_KB_MAX)).lw(c);
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003f_5f30);
                }
            }
        }
        c.cycles += 6;
        c.g[V0][0] = usb_kb_lui(USB_KB_STATE);
        c.g[S0][0] = sx(USB_KB_STATE + USB_KB_INFO);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::SCE_USB_KB_GET_INFO, 0x003f_5f5c)?;
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(USB_KB_INFO));
            c.g[V0][0] = 1;
            Addr((c.g[V1][0] as u32).wrapping_add(USB_KB_BUSY)).sb(c, c.g[V0][0]);
            Addr((c.g[V1][0] as u32).wrapping_add(USB_KB_REQUEST)).sw(c, USB_KB_REQ_INFO);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int UsbKb_SyncInfo(void)` (0x3f5f88): poll the info request
/// (`sceUsbKbSync(1, &result)`); while it is not done return what the sync
/// said. Once done, call the connect callback (+0xbc) with `(keyboard,
/// status)` for every keyboard whose status differs from the one last
/// reported, clear the request and return 0.
pub fn usb_kb_sync_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x70);
        c.g[A0][0] = USB_KB_SYNC_POLL;
        frame.save(c, S3, 0x30);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S4, 0x20);
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, callee::SCE_USB_KB_SYNC, 0x003f_5fb0)?;
        c.g[S3][0] = c.g[V0][0];
        frame.restore(c, S0, 0x60);
        if c.g[S3][0] == 0 {
            c.cycles += 7;
            c.g[V1][0] = usb_kb_lui(USB_KB_STATE);
            c.g[S4][0] = c.g[V1][0];
            c.g[V1][0] = sx(USB_KB_STATE);
            c.g[V0][0] = Addr(USB_KB_STATE + USB_KB_MAX).lw(c);
            c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
            c.g[S1][0] = 0;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[S2][0] = c.g[V1][0];
                c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(USB_KB_CONNECTED));
                loop {
                    c.cycles += 4;
                    let seen_at = (c.g[S0][0] as u32).wrapping_add(USB_KB_SEEN - USB_KB_CONNECTED);
                    c.g[V1][0] = Addr(seen_at).lbu(c);
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).lbu(c);
                    c.g[A0][0] = c.g[S1][0];
                    if c.g[V1][0] != c.g[V0][0] {
                        c.cycles += 3;
                        let cb_at = (c.g[S2][0] as u32).wrapping_add(USB_KB_CONNECT_CB);
                        c.g[V0][0] = Addr(cb_at).lw(c);
                        let target = c.g[V0][0] as u32;
                        c.g[A1][0] = Addr(c.g[S0][0] as u32).lbu(c);
                        call_ptr(c, target, 0x003f_5ffc)?;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(USB_KB_MAX)).lw(c);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003f_5fe0);
                    }
                }
            }
            c.cycles += 5;
            c.g[V1][0] = usb_kb_state_from(c, S4);
            c.g[V0][0] = c.g[S3][0];
            Addr((c.g[V1][0] as u32).wrapping_add(USB_KB_REQUEST)).sw(c, 0);
            Addr((c.g[V1][0] as u32).wrapping_add(USB_KB_BUSY)).sb(c, 0);
            frame.restore(c, S0, 0x60);
        }
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

/// `int UsbKb_FindNextReady(int first)` (0x3f6040): the first keyboard,
/// from `first` on (wrapping to keyboard 0 past the last), that is connected
/// and whose last data was taken (fresh byte clear); -1 when none is.
pub fn usb_kb_find_next_ready(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = usb_kb_lui(USB_KB_STATE);
    c.g[A1][0] = c.g[A0][0];
    c.g[V0][0] = sx(USB_KB_STATE);
    c.g[V1][0] = Addr(USB_KB_STATE + USB_KB_MAX).lw(c);
    c.g[A2][0] = 0;
    if (c.g[V1][0] as i64) > 0 {
        c.cycles += 4;
        c.g[T0][0] = c.g[V0][0];
        c.g[A3][0] = c.g[V1][0];
        c.g[T2][0] = sx((c.g[T0][0] as u32).wrapping_add(USB_KB_CONNECTED));
        c.g[T1][0] = USB_KB_DATA_SIZE;
        loop {
            c.cycles += 6;
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A3][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.g[A1][0] = 0;
            }
            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T2][0] as u32));
            c.g[V0][0] = Addr(c.g[V1][0] as u32).lbu(c);
            let connected = c.g[V0][0] != 0;
            c.mult(0, V0, A1, T1);
            if connected {
                c.cycles += 5;
                c.g[V1][0] = Addr((c.g[T0][0] as u32).wrapping_add(USB_KB_DATA)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(USB_KB_DATA_FRESH)).lbu(c);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                if c.g[A0][0] == 0 {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[A1][0];
                    ret(c);
                    return;
                }
            } else {
                c.cycles += 1;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[A3][0] as i64)) as u64;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003f_6068);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = u64::MAX;
    ret(c);
}

/// `void UsbKb_RequestRead(void)` (0x3f60b8): pick the next keyboard ready
/// for a read after the last one read (`UsbKb_FindNextReady`), remember it
/// and its LED and modifier words, and ask the driver for its data
/// (`sceUsbKbRead` into its data entry); when the request went out, mark it
/// pending (request 2, busy).
pub fn usb_kb_request_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = usb_kb_lui(USB_KB_STATE);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(USB_KB_STATE);
        c.g[A0][0] = Addr(USB_KB_STATE + USB_KB_READ_PORT).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
        callee::USB_KB_FIND_NEXT_READY.call(c, 0x003f_60d8)?;
        c.g[A0][0] = c.g[V0][0];
        let none = c.g[A0][0] == u64::MAX;
        c.g[V0][0] = USB_KB_DATA_SIZE;
        'sent: {
            if !none {
                c.cycles += 12;
                let state = c.g[S0][0] as u32;
                c.g[V1][0] = Addr(state.wrapping_add(USB_KB_DATA)).lw(c);
                c.mult(0, V0, A0, V0);
                Addr(state.wrapping_add(USB_KB_READ_PORT)).sw(c, c.g[A0][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                let data = c.g[V1][0] as u32;
                c.g[A2][0] = Addr(data.wrapping_add(USB_KB_DATA_LED)).lw(c);
                c.g[A1][0] = c.g[V1][0];
                Addr(state.wrapping_add(USB_KB_PREV_LED)).sw(c, c.g[A2][0]);
                c.g[V0][0] = Addr(data.wrapping_add(USB_KB_DATA_MKEY)).lw(c);
                Addr(state.wrapping_add(USB_KB_PREV_MKEY)).sw(c, c.g[V0][0]);
                call_ptr(c, callee::SCE_USB_KB_READ, 0x003f_6110)?;
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x10);
                    break 'sent;
                }
                c.cycles += 4;
                let state = c.g[S0][0] as u32;
                c.g[V1][0] = USB_KB_REQ_READ;
                c.g[V0][0] = 1;
                Addr(state.wrapping_add(USB_KB_BUSY)).sb(c, c.g[V0][0]);
                Addr(state.wrapping_add(USB_KB_REQUEST)).sw(c, c.g[V1][0]);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int UsbKb_SyncRead(void)` (0x3f6138): wait for the read request
/// (`sceUsbKbSync(0, &result)`); when that fails return its result. Else
/// mark the read keyboard's data fresh if it brought keys, or its modifier
/// or LED word changed, clear the request and return 0.
pub fn usb_kb_sync_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = USB_KB_SYNC_WAIT;
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, callee::SCE_USB_KB_SYNC, 0x003f_614c)?;
        c.g[A3][0] = c.g[V0][0];
        frame.restore_ra(c, 0x10);
        if c.g[A3][0] == 0 {
            c.cycles += 11;
            c.g[V1][0] = usb_kb_lui(USB_KB_STATE);
            c.g[A1][0] = USB_KB_DATA_SIZE;
            c.g[A2][0] = sx(USB_KB_STATE);
            c.g[T0][0] = c.g[V1][0];
            let state = c.g[A2][0] as u32;
            c.g[V0][0] = Addr(state.wrapping_add(USB_KB_READ_PORT)).lw(c);
            c.g[A0][0] = Addr(state.wrapping_add(USB_KB_DATA)).lw(c);
            c.mult(0, V0, V0, A1);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            let data = c.g[A0][0] as u32;
            c.g[V1][0] = Addr(data.wrapping_add(USB_KB_DATA_LEN)).lw(c);
            c.g[V0][0] = 1;
            let mut fresh = (c.g[V1][0] as i64) > 0;
            if !fresh {
                c.cycles += 4;
                c.g[V1][0] = Addr(data.wrapping_add(USB_KB_DATA_MKEY)).lw(c);
                c.g[V0][0] = Addr(state.wrapping_add(USB_KB_PREV_MKEY)).lw(c);
                fresh = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = 1;
                if !fresh {
                    c.cycles += 4;
                    c.g[V1][0] = Addr(state.wrapping_add(USB_KB_PREV_LED)).lw(c);
                    c.g[V0][0] = Addr(data.wrapping_add(USB_KB_DATA_LED)).lw(c);
                    fresh = c.g[V0][0] != c.g[V1][0];
                    c.g[V0][0] = 1;
                }
            }
            if fresh {
                c.cycles += 1;
                Addr(data.wrapping_add(USB_KB_DATA_FRESH)).sb(c, c.g[V0][0]);
            }
            c.cycles += 5;
            c.g[V1][0] = usb_kb_state_from(c, T0);
            c.g[V0][0] = c.g[A3][0];
            Addr((c.g[V1][0] as u32).wrapping_add(USB_KB_REQUEST)).sw(c, 0);
            Addr((c.g[V1][0] as u32).wrapping_add(USB_KB_BUSY)).sb(c, 0);
            frame.restore_ra(c, 0x10);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Start-up and shut-down.

/// `int UsbKb_Init(const char *dir, const char *kb_args, KbData *data,
/// void (*connect_cb)(int, int), int (*hold_cb)(int))` (0x3f61d0): load the
/// modules (`UsbKb_LoadModules(dir, kb_args)`) and start the driver
/// (`sceUsbKbInit`, which writes how many keyboards it serves; at most 8
/// are kept). Clear every keyboard's status, last reported status and fresh
/// byte, keep `data` and the callbacks (`UsbKb_DefaultConnectCb` and
/// `UsbKb_NullCallback` when null), set the info poll to 60 frames, then
/// request the keyboards' status and wait for it. Returns 0, or -1 when a
/// module or the driver failed.
pub fn usb_kb_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A3][0];
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[T0][0];
        callee::USB_KB_LOAD_MODULES.call(c, 0x003f_61f8)?;
        'done: {
            'failed: {
                let failed = (c.g[V0][0] as i64) < 0;
                c.g[V0][0] = usb_kb_lui(USB_KB_STATE);
                if failed {
                    break 'failed;
                }
                c.cycles += 6;
                c.g[S0][0] = sx(USB_KB_STATE + USB_KB_MAX);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, callee::SCE_USB_KB_INIT, 0x003f_620c)?;
                c.g[V1][0] = u64::MAX;
                let started = c.g[V0][0] != c.g[V1][0];
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_sub(USB_KB_MAX));
                if !started {
                    break 'failed;
                }
                usb_kb_init_state(c);
                callee::USB_KB_REQUEST_INFO.call(c, 0x003f_62c4)?;
                loop {
                    c.cycles += 4;
                    callee::USB_KB_SYNC_INFO.call(c, 0x003f_62d0)?;
                    let pending = c.g[V0][0] != 0;
                    c.g[V0][0] = 0;
                    if !pending {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003f_62c8);
                    }
                }
                break 'done;
            }
            c.cycles += 2;
            c.g[V0][0] = u64::MAX;
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

/// [`usb_kb_init`] once the driver runs (a0 = the state block, s1 = `data`,
/// s2/s3 = the callbacks): clamp the keyboard count, clear the per-keyboard
/// bytes, set up polling and the callbacks.
fn usb_kb_init_state(c: &mut Cpu) {
    c.cycles += 4;
    let max_at = (c.g[A0][0] as u32).wrapping_add(USB_KB_MAX);
    c.g[V0][0] = Addr(max_at).lw(c);
    c.g[V0][0] = ((c.g[V0][0] as i64) < (USB_KB_MAX_KEYBOARDS + 1) as i64) as u64;
    if c.g[V0][0] != 0 {
        c.g[V0][0] = Addr(max_at).lw(c);
    } else {
        c.cycles += 3;
        c.g[V0][0] = USB_KB_MAX_KEYBOARDS;
        Addr(max_at).sw(c, c.g[V0][0]);
        c.g[V0][0] = Addr(max_at).lw(c);
    }
    c.cycles += 3;
    c.g[V1][0] = 0;
    let none = (c.g[V0][0] as i64) <= 0;
    Addr((c.g[A0][0] as u32).wrapping_add(USB_KB_DATA)).sw(c, c.g[S1][0]);
    if !none {
        c.cycles += 4;
        c.g[A1][0] = c.g[A0][0];
        c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(USB_KB_DATA_FRESH));
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(USB_KB_SEEN - USB_KB_MAX));
        loop {
            c.cycles += 9;
            let status_at = (c.g[A0][0] as u32).wrapping_sub(USB_KB_SEEN - USB_KB_CONNECTED);
            Addr(status_at).sb(c, 0);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            Addr(c.g[A0][0] as u32).sb(c, 0);
            Addr(c.g[A2][0] as u32).sb(c, 0);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
            c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(USB_KB_MAX)).lw(c);
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(USB_KB_DATA_SIZE as u32));
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003f_6258);
            }
        }
    }
    c.cycles += 9;
    c.g[V0][0] = usb_kb_lui(USB_KB_STATE);
    c.g[V1][0] = USB_KB_POLL_FRAMES;
    c.g[A0][0] = sx(USB_KB_STATE);
    c.g[A3][0] = c.g[S2][0];
    let state = c.g[A0][0] as u32;
    Addr(state.wrapping_add(USB_KB_POLL_COUNT)).sw(c, c.g[V1][0]);
    Addr(state.wrapping_add(USB_KB_READ_PORT)).sw(c, 0);
    Addr(state.wrapping_add(USB_KB_POLL_PERIOD)).sw(c, c.g[V1][0]);
    Addr(state.wrapping_add(USB_KB_BUSY)).sb(c, 0);
    if c.g[A3][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = usb_kb_lui(USB_KB_DEFAULT_CONNECT);
        c.g[A3][0] = sx(USB_KB_DEFAULT_CONNECT);
    }
    c.cycles += 3;
    c.g[T0][0] = c.g[S3][0];
    Addr(state.wrapping_add(USB_KB_CONNECT_CB)).sw(c, c.g[A3][0]);
    if c.g[T0][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = usb_kb_lui(USB_KB_NULL_CALLBACK);
        c.g[T0][0] = sx(USB_KB_NULL_CALLBACK);
    }
    c.cycles += 3;
    Addr(state.wrapping_add(USB_KB_HOLD_CB)).sw(c, c.g[T0][0]);
}

/// `int UsbKb_Shutdown(void)` (0x3f62f8): stop the driver (`sceUsbKbEnd`)
/// and unload `USBKB.IRX`, then `USBD.IRX`. Returns the two
/// `UsbKb_UnloadModule` results or'ed (1: USBKB failed, 2: USBD failed).
pub fn usb_kb_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = usb_kb_lui(USB_KB_STATE);
        call_ptr(c, callee::SCE_USB_KB_END, 0x003f_6310)?;
        c.g[V0][0] = usb_kb_state_from(c, S0);
        c.g[A1][0] = USB_KB_UNLOAD_ERR_USBKB;
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(USB_KB_USBKB_ID)).lw(c);
        callee::USB_KB_UNLOAD_MODULE.call(c, 0x003f_6320)?;
        let state = usb_kb_state_from(c, S0) as u32;
        c.g[A0][0] = Addr(state.wrapping_add(USB_KB_USBD_ID)).lw(c);
        c.g[S1][0] = c.g[V0][0];
        c.g[A1][0] = USB_KB_UNLOAD_ERR_USBD;
        callee::USB_KB_UNLOAD_MODULE.call(c, 0x003f_6330)?;
        c.g[V0][0] = c.g[S1][0] | c.g[V0][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int UsbKb_GetMaxKeyboards(void)` (0x3f6348): how many keyboards the
/// driver serves (the count `UsbKb_Init` kept, at most 8).
pub fn usb_kb_get_max_keyboards(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = usb_kb_lui(USB_KB_STATE);
    c.g[V0][0] = Addr(USB_KB_STATE + USB_KB_MAX).lw(c);
    ret(c);
}

/// `void UsbKb_DefaultConnectCb(int keyboard, int status)` (0x3f6358): the
/// connect callback `UsbKb_Init` installs when given none. For a keyboard
/// that is now connected (`status` non-zero): light num lock
/// (`sceUsbKbSetLEDStatus`) and set LED mode 1 (`sceUsbKbSetLEDMode`), each
/// followed by `sceUsbKbSync(0, &result)` when its request went out; clear
/// the keyboard's buffer, repeat keys after 30 frames then every 2, code
/// type 0.
pub fn usb_kb_default_connect_cb(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        let connected = c.g[A1][0] != 0;
        c.g[S0][0] = c.g[A0][0];
        if connected {
            c.cycles += 4;
            c.g[A1][0] = USB_KB_CONNECT_LED_STATUS;
            call_ptr(c, callee::SCE_USB_KB_SET_LED_STATUS, 0x003f_6374)?;
            let sent = c.g[V0][0] == 0;
            Addr(sp(c)).sw(c, c.g[V0][0]);
            if sent {
                c.cycles += 3;
                c.g[A0][0] = USB_KB_SYNC_WAIT;
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, callee::SCE_USB_KB_SYNC, 0x003f_6388)?;
            }
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = USB_KB_CONNECT_LED_MODE;
            call_ptr(c, callee::SCE_USB_KB_SET_LED_MODE, 0x003f_6394)?;
            let sent = c.g[V0][0] == 0;
            Addr(sp(c)).sw(c, c.g[V0][0]);
            if sent {
                c.cycles += 3;
                c.g[A0][0] = USB_KB_SYNC_WAIT;
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, callee::SCE_USB_KB_SYNC, 0x003f_63a8)?;
            }
            c.cycles += 9;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::SCE_USB_KB_CLEAR_RBUF, 0x003f_63b0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = USB_KB_REPEAT_START;
            c.g[A2][0] = USB_KB_REPEAT_INTERVAL;
            call_ptr(c, callee::SCE_USB_KB_SET_REPEAT, 0x003f_63c0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = USB_KB_CODE_TYPE;
            call_ptr(c, callee::SCE_USB_KB_SET_CODE_TYPE, 0x003f_63cc)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Polling.

/// `void UsbKb_Update(void)` (0x3f63f0): the per-frame poll. Finish the
/// pending request: an info request (`UsbKb_SyncInfo`), a read
/// (`UsbKb_SyncRead`), or a hold, which ends when the hold callback (+0xc0)
/// called with 1 says so. With nothing pending, the hold callback called
/// with 0 may hold the library (request 3); else count the frame and request
/// the keyboards' status once the count passes the poll period
/// (`UsbKb_RequestInfo`, count reset), or the next keyboard's data
/// (`UsbKb_RequestRead`).
pub fn usb_kb_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = usb_kb_lui(USB_KB_STATE);
        frame.save(c, S0, 0x20);
        c.g[V1][0] = USB_KB_REQ_INFO;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(USB_KB_STATE);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(USB_KB_STATE + USB_KB_REQUEST).lw(c);
        c.g[S1][0] = c.g[V0][0];
        // v0: the request once the pending one is dealt with.
        'pending: {
            'reload: {
                if c.g[A0][0] == USB_KB_REQ_INFO {
                    c.cycles += 4;
                    c.g[S0][0] = usb_kb_state_from(c, S1);
                    callee::USB_KB_SYNC_INFO.call(c, 0x003f_6440)?;
                    let request_at = (c.g[S0][0] as u32).wrapping_add(USB_KB_REQUEST);
                    c.g[V0][0] = Addr(request_at).lw(c);
                    break 'pending;
                }
                c.cycles += 2;
                c.g[V0][0] = USB_KB_REQ_READ;
                if c.g[A0][0] == 0 {
                    break 'reload;
                }
                c.cycles += 2;
                c.g[V0][0] = USB_KB_REQ_HELD;
                if c.g[A0][0] == USB_KB_REQ_READ {
                    c.cycles += 4;
                    c.g[S0][0] = usb_kb_state_from(c, S1);
                    callee::USB_KB_SYNC_READ.call(c, 0x003f_6450)?;
                    let request_at = (c.g[S0][0] as u32).wrapping_add(USB_KB_REQUEST);
                    c.g[V0][0] = Addr(request_at).lw(c);
                    break 'pending;
                }
                c.cycles += 2;
                if c.g[A0][0] == USB_KB_REQ_HELD {
                    let hold_cb_at = (c.g[S0][0] as u32).wrapping_add(USB_KB_HOLD_CB);
                    c.g[V0][0] = Addr(hold_cb_at).lw(c);
                    c.cycles += 4;
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = 1;
                    call_ptr(c, target, 0x003f_6460)?;
                    if c.g[V0][0] != 0 {
                        let request_at = (c.g[S0][0] as u32).wrapping_add(USB_KB_REQUEST);
                        Addr(request_at).sw(c, 0);
                    }
                    break 'reload;
                }
                c.cycles += 2;
                c.g[S0][0] = usb_kb_state_from(c, S1);
                c.cycles += 1;
                let request_at = (c.g[S0][0] as u32).wrapping_add(USB_KB_REQUEST);
                c.g[V0][0] = Addr(request_at).lw(c);
                break 'pending;
            }
            c.cycles += 1;
            c.g[S0][0] = usb_kb_state_from(c, S1);
            c.cycles += 1;
            let request_at = (c.g[S0][0] as u32).wrapping_add(USB_KB_REQUEST);
            c.g[V0][0] = Addr(request_at).lw(c);
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            let hold_cb_at = (c.g[S0][0] as u32).wrapping_add(USB_KB_HOLD_CB);
            c.g[V0][0] = Addr(hold_cb_at).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = 0;
            call_ptr(c, target, 0x003f_6484)?;
            let held = c.g[V0][0] != 0;
            c.g[V0][0] = USB_KB_REQ_HELD;
            let state = c.g[S0][0] as u32;
            if held {
                c.cycles += 1;
                Addr(state.wrapping_add(USB_KB_REQUEST)).sw(c, c.g[V0][0]);
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(state.wrapping_add(USB_KB_REQUEST)).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 6;
                c.g[V0][0] = Addr(state.wrapping_add(USB_KB_POLL_COUNT)).lw(c);
                c.g[V1][0] = Addr(state.wrapping_add(USB_KB_POLL_PERIOD)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                Addr(state.wrapping_add(USB_KB_POLL_COUNT)).sw(c, c.g[V0][0]);
                if c.g[V1][0] != 0 {
                    c.cycles += 4;
                    Addr(state.wrapping_add(USB_KB_POLL_COUNT)).sw(c, 0);
                    callee::USB_KB_REQUEST_INFO.call(c, 0x003f_64bc)?;
                } else {
                    c.cycles += 3;
                    callee::USB_KB_REQUEST_READ.call(c, 0x003f_64cc)?;
                }
            }
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `KbData *UsbKb_GetData(int keyboard)` (0x3f64e0): the keyboard's data
/// entry when it is fresh (fresh byte 1, which this clears), else null.
pub fn usb_kb_get_data(c: &mut Cpu) {
    c.cycles += 10;
    c.g[V0][0] = USB_KB_DATA_SIZE;
    c.g[V1][0] = usb_kb_lui(USB_KB_STATE);
    c.mult(0, A1, A0, V0);
    c.g[A2][0] = sx(USB_KB_STATE);
    c.g[A0][0] = Addr(USB_KB_STATE + USB_KB_DATA).lw(c);
    c.g[V1][0] = 1;
    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    let fresh_at = (c.g[A0][0] as u32).wrapping_add(USB_KB_DATA_FRESH);
    c.g[V0][0] = Addr(fresh_at).lbu(c);
    if c.g[V0][0] == c.g[V1][0] {
        Addr(fresh_at).sb(c, 0);
        c.cycles += 3;
        c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(USB_KB_DATA)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0;
    }
    ret(c);
}

/// `bool UsbKb_IsConnected(int keyboard)` (0x3f6520): whether the
/// keyboard's status byte is non-zero.
pub fn usb_kb_is_connected(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = sx(USB_KB_STATE.wrapping_add(c.g[A0][0] as u32));
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(USB_KB_CONNECTED)).lbu(c);
    c.g[V0][0] = (c.g[V0][0] != 0) as u64;
    ret(c);
}

/// `UsbKb_AnyConnected`'s loop head (a resume entry after an event unwind).
const USB_KB_ANY_CONNECTED_HEAD: u32 = 0x003f_6560;

/// `bool UsbKb_AnyConnected(void)` (0x3f6538): poll (`UsbKb_Update`), then
/// whether any of the served keyboards is connected.
///
/// The original is a `match pc` dispatcher whose loop head (0x3f6560) is
/// also a resume entry after an event unwind; the loop state is in the
/// registers (s0 = the next keyboard, s1 = 1).
pub fn usb_kb_any_connected(c: &mut Cpu) {
    guest(c, |c| {
        let resumed = match c.pc {
            0x003f_6538 => false,
            USB_KB_ANY_CONNECTED_HEAD => true,
            pc => {
                c.jump_out(pc, 0x003f_6538, 0x003f_65a0);
                return Ok(());
            }
        };
        let frame = if resumed {
            Frame { size: 0x30 }
        } else {
            c.cycles += 9;
            let frame = Frame::open(c, 0x30);
            frame.save(c, S0, 0x20);
            frame.save(c, S1, 0x10);
            c.g[S0][0] = 0;
            frame.save_ra(c, 0);
            c.g[S1][0] = 1;
            callee::USB_KB_UPDATE.call(c, 0x003f_6554)?;
            frame
        };
        let mut at_head = resumed;
        loop {
            if at_head {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                callee::USB_KB_IS_CONNECTED.call(c, 0x003f_6568)?;
                let connected = c.g[V0][0] == c.g[S1][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                if connected {
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    break;
                }
            }
            c.cycles += 5;
            callee::USB_KB_GET_MAX_KEYBOARDS.call(c, 0x003f_6580)?;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = 0;
            if !more {
                break;
            }
            if back_edge_unwinds(c, USB_KB_ANY_CONNECTED_HEAD) {
                return Ok(());
            }
            at_head = true;
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
// Keys.

/// `const u8 *UsbKb_GetKeypadEntry(int usage)` (0x3f65a0): the
/// `{usage, key code}` pair of the alternate layout: its own pairs for
/// usages 0x2e-0x35 and 0x87-0x8b, the common table's for every other usage.
pub fn usb_kb_get_keypad_entry(c: &mut Cpu) {
    let usage = c.g[A0][0] as u32;
    let in_range = |first: u32, keys: u64| sx(usage.wrapping_sub(first)) < keys;
    let (table, cost) = if in_range(USB_KB_USAGE_ALT_A, USB_KB_USAGE_ALT_A_KEYS) {
        (USB_KB_KEY_TABLE_ALT_A, 9)
    } else if in_range(USB_KB_USAGE_ALT_B, USB_KB_USAGE_ALT_B_KEYS) {
        (USB_KB_KEY_TABLE_ALT_B, 12)
    } else {
        (USB_KB_KEY_TABLE, 11)
    };
    c.cycles += cost;
    c.g[V1][0] = sx(usage << 1);
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(table));
    ret(c);
}

/// `void UsbKb_LookupKeyChar(u8 *out, const KbData *data, int layout, int
/// usage)` (0x3f65f8): the key code of `usage` into `*out`: with num lock lit
/// a keypad digit's num-lock code; else the common table's code (layout 0)
/// or the alternate layout's (`UsbKb_GetKeypadEntry`).
pub fn usb_kb_lookup_key_char(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(USB_KB_DATA_LED)).lw(c);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] &= USB_KB_LED_NUM_LOCK;
        let num_lock = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[A3][0];
        // v0: the pair.
        'pair: {
            if num_lock {
                c.cycles += 4;
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(USB_KB_USAGE_KEYPAD_1));
                let digit = c.g[V1][0] < USB_KB_KEYPAD_DIGITS;
                c.g[V0][0] = usb_kb_lui(USB_KB_KEY_TABLE_NUMPAD);
                if digit {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(USB_KB_KEY_TABLE_NUMPAD));
                    break 'pair;
                }
            }
            c.cycles += 2;
            c.g[V0][0] = usb_kb_lui(USB_KB_KEY_TABLE);
            if c.g[A2][0] == 0 {
                c.cycles += 4;
                c.g[V1][0] = sx((c.g[A0][0] as u32) << 1);
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(USB_KB_KEY_TABLE));
            } else {
                c.cycles += 2;
                callee::USB_KB_GET_KEYPAD_ENTRY.call(c, 0x003f_6658)?;
            }
        }
        c.cycles += 6;
        c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(1)).lbu(c);
        frame.restore_ra(c, 0);
        Addr(c.g[S0][0] as u32).sb(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void UsbKb_ConvertRawCode(u8 *out, const KbData *data, int arrangement,
/// int usage)` (0x3f6670): the character of `usage` into `out[1]`: a keypad
/// operator's always, a keypad digit's only with num lock lit (else 0);
/// any other key's from `sceUsbKbCnvRawCode(arrangement, modifiers, LEDs,
/// usage)`, 0 when that is no character (256 or more).
pub fn usb_kb_convert_raw_code(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_sub(USB_KB_USAGE_KEYPAD));
        frame.save(c, S0, 0x10);
        let keypad = c.g[V1][0] < USB_KB_KEYPAD_KEYS;
        c.g[V0][0] = keypad as u64;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A2][0];
        if keypad {
            c.cycles += 3;
            let digit = c.g[A3][0] >= USB_KB_USAGE_KEYPAD_1 as u64;
            c.g[V0][0] = usb_kb_lui(USB_KB_KEYPAD_CHARS);
            let mut lit = true;
            if digit {
                c.cycles += 4;
                let led_at = (c.g[A1][0] as u32).wrapping_add(USB_KB_DATA_LED);
                c.g[V0][0] = Addr(led_at).lw(c) & USB_KB_LED_NUM_LOCK;
                lit = c.g[V0][0] != 0;
                c.g[V0][0] = usb_kb_lui(USB_KB_KEYPAD_CHARS);
            }
            let char_at = (c.g[S0][0] as u32).wrapping_add(1);
            if lit {
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(USB_KB_KEYPAD_CHARS));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lbu(c);
                Addr(char_at).sb(c, c.g[V1][0]);
            } else {
                c.cycles += 1;
                Addr(char_at).sb(c, 0);
            }
        } else {
            c.cycles += 8;
            let data = c.g[A1][0] as u32;
            c.g[A2][0] = Addr(data.wrapping_add(USB_KB_DATA_LED)).lw(c);
            c.g[A3][0] &= 0xffff;
            c.g[A1][0] = Addr(data.wrapping_add(USB_KB_DATA_MKEY)).lw(c);
            call_ptr(c, callee::SCE_USB_KB_CNV_RAW_CODE, 0x003f_66d0)?;
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < 0x100) as u64;
            let char_at = (c.g[S0][0] as u32).wrapping_add(1);
            if c.g[V0][0] == 0 {
                Addr(char_at).sb(c, 0);
            } else {
                c.cycles += 2;
                Addr(char_at).sb(c, c.g[V1][0]);
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool UsbKb_CheckModifierKey(u32 mkey, u32 key, u32 *down)` (0x3f6700):
/// whether `key` is a modifier key (shift, control, alt or Windows, either
/// side or both); if so `*down` = its bits of the modifier word `mkey`.
pub fn usb_kb_check_modifier_key(c: &mut Cpu) {
    // The key's modifier bits, and the cycles the compare chain takes to it.
    let (bits, cost) = match c.g[A1][0] {
        USB_KB_VK_LSHIFT => (Some(USB_KB_MKEY_LSHIFT), 6),
        USB_KB_VK_MENU => (Some(USB_KB_MKEY_LALT), 10),
        USB_KB_VK_SHIFT => (Some(USB_KB_MKEY_LSHIFT), 15),
        USB_KB_VK_CONTROL => (Some(USB_KB_MKEY_LCTRL | USB_KB_MKEY_RCTRL), 15),
        key if key < USB_KB_VK_MENU => (None, 17),
        USB_KB_VK_LWIN => (Some(USB_KB_MKEY_LWIN), 16),
        USB_KB_VK_RWIN => (Some(USB_KB_MKEY_RWIN), 16),
        key if key < USB_KB_VK_LSHIFT => (None, 18),
        USB_KB_VK_RCONTROL => (Some(USB_KB_MKEY_RCTRL), 12),
        USB_KB_VK_RSHIFT => (Some(USB_KB_MKEY_RSHIFT), 16),
        USB_KB_VK_LCONTROL => (Some(USB_KB_MKEY_LCTRL), 16),
        USB_KB_VK_LMENU => (Some(USB_KB_MKEY_LALT), 16),
        USB_KB_VK_RMENU => (Some(USB_KB_MKEY_RALT), 17),
        _ => (None, 19),
    };
    usb_kb_store_key_bits(c, bits, cost);
}

/// `bool UsbKb_CheckLedKey(u32 led, u32 key, u32 *down)` (0x3f67e0): whether
/// `key` is a lock key (num, caps or scroll lock); if so `*down` = its bit
/// of the LED word `led`.
pub fn usb_kb_check_led_key(c: &mut Cpu) {
    let (bits, cost) = match c.g[A1][0] {
        USB_KB_VK_NUMLOCK => (Some(USB_KB_LED_NUM_LOCK), 5),
        USB_KB_VK_CAPITAL => (Some(USB_KB_LED_CAPS_LOCK), 9),
        key if key < USB_KB_VK_NUMLOCK => (None, 11),
        USB_KB_VK_SCROLL => (Some(USB_KB_LED_SCROLL_LOCK), 10),
        _ => (None, 12),
    };
    usb_kb_store_key_bits(c, bits, cost);
}

/// The common tail of the two key checks: found (a3 = v0 = 1) with
/// v1 = `a0 & bits` stored at `a2`, or not (a3 = v0 = v1 = 0).
fn usb_kb_store_key_bits(c: &mut Cpu, bits: Option<u64>, cost: u64) {
    c.cycles += cost + 2;
    let word = c.g[A0][0];
    c.g[A3][0] = bits.is_some() as u64;
    c.g[V1][0] = bits.map_or(0, |b| word & b);
    if bits.is_some() {
        Addr(c.g[A2][0] as u32).sw(c, c.g[V1][0]);
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A3][0];
    ret(c);
}

/// The result word in `UsbKb_IsKeyDown`'s frame.
const USB_KB_IS_KEY_DOWN_RESULT: u32 = 0x10;

/// `bool UsbKb_IsKeyDown(const KbData *data, int layout, int index, u32
/// key)` (0x3f6838): whether `key` is down by key code `index` of `data`: a
/// modifier key by the modifier word, a lock key by the LED word, any other
/// key when the code's key code (`UsbKb_LookupKeyChar`) is `key`.
pub fn usb_kb_is_key_down(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x80);
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 1);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x50);
        c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        frame.save(c, S4, 0x30);
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, S3, 0x40);
        c.g[S4][0] = c.g[A1][0];
        frame.save_ra(c, 0x20);
        c.g[S2][0] = sx(frame.at(c, USB_KB_IS_KEY_DOWN_RESULT));
        c.g[A1][0] = c.g[S1][0];
        c.g[S3][0] = Addr((c.g[A2][0] as u32).wrapping_add(USB_KB_DATA_KEYS)).lhu(c);
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(USB_KB_DATA_MKEY)).lw(c);
        c.g[A2][0] = c.g[S2][0];
        Addr(frame.at(c, USB_KB_IS_KEY_DOWN_RESULT)).sw(c, 0);
        callee::USB_KB_CHECK_MODIFIER_KEY.call(c, 0x003f_6884)?;
        'done: {
            let handled = c.g[V0][0] != 0;
            c.g[V0][0] = Addr(frame.at(c, USB_KB_IS_KEY_DOWN_RESULT)).lw(c);
            if handled {
                break 'done;
            }
            c.cycles += 6;
            c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(USB_KB_DATA_LED)).lw(c);
            c.g[A2][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S1][0];
            callee::USB_KB_CHECK_LED_KEY.call(c, 0x003f_689c)?;
            let handled = c.g[V0][0] != 0;
            c.g[V0][0] = Addr(frame.at(c, USB_KB_IS_KEY_DOWN_RESULT)).lw(c);
            if handled {
                break 'done;
            }
            c.cycles += 8;
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S4][0];
            c.g[A3][0] = c.g[S3][0];
            c.g[A0][0] = c.g[SP][0];
            callee::USB_KB_LOOKUP_KEY_CHAR.call(c, 0x003f_68b8)?;
            c.g[V0][0] = Addr(sp(c)).lbu(c);
            let other = c.g[V0][0] != c.g[S1][0];
            c.g[V0][0] = Addr(frame.at(c, USB_KB_IS_KEY_DOWN_RESULT)).lw(c);
            if other {
                break 'done;
            }
            c.cycles += 3;
            c.g[V0][0] = 1;
            Addr(frame.at(c, USB_KB_IS_KEY_DOWN_RESULT)).sw(c, c.g[V0][0]);
            c.g[V0][0] = Addr(frame.at(c, USB_KB_IS_KEY_DOWN_RESULT)).lw(c);
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `const u8 *UsbKb_TranslateKey(const KbData *data, int arrangement, int
/// index)` (0x3f68f0): key code `index` of `data` as `{key code,
/// character}` (`UsbKb_LookupKeyChar`, `UsbKb_ConvertRawCode`) in a static
/// pair (gp+0x2a20), which it returns.
pub fn usb_kb_translate_key(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 1);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = Addr((c.g[A2][0] as u32).wrapping_add(USB_KB_DATA_KEYS)).lhu(c);
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = sx(r32(c, GP).wrapping_add(USB_KB_KEY_PAIR_GP));
        c.g[A3][0] = c.g[S2][0];
        callee::USB_KB_LOOKUP_KEY_CHAR.call(c, 0x003f_692c)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = sx(r32(c, GP).wrapping_add(USB_KB_KEY_PAIR_GP));
        c.g[A3][0] = c.g[S2][0];
        callee::USB_KB_CONVERT_RAW_CODE.call(c, 0x003f_6940)?;
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = sx(r32(c, GP).wrapping_add(USB_KB_KEY_PAIR_GP));
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes::{machine, FAKE, LOG_COUNT, LOG_UNWIND_AT};
    use crate::difftest::{
        check, check_lohi, check_paths, check_paths_lohi, elf_image, fill, fresh,
        require_paths, run_generated, Rng, CYCLE_BUDGET, DATA,
    };

    // The IOP side the fixtures stand in for (libusbkb and the SIF loader).
    /// libusbkb's semaphore (-1 until `sceUsbKbInit`: every `sceUsbKb*`
    /// call then fails at once).
    const USB_KB_TEST_LIB_SEMA: u32 = 0x0045_3438;
    /// libusbkb's state: +0 connectable keyboards, +4 the per-keyboard
    /// settings (0x38 bytes each), +0x90 the pending operation.
    const USB_KB_TEST_LIB: u32 = 0x0052_9000;
    const USB_KB_TEST_LIB_LIMIT: u32 = 0x0;
    const USB_KB_TEST_LIB_SETTINGS: u32 = 0x4;
    const USB_KB_TEST_LIB_OP: u32 = 0x90;
    /// libusbkb's RPC client (+0 its packet, +4 its request id).
    const USB_KB_TEST_LIB_RPC: u32 = 0x0052_90a0;
    /// libusbkb's result callbacks by pending operation.
    const USB_KB_TEST_LIB_RESULT_FNS: u32 = 0x0045_3420;
    /// The SIF RPC packet pool: +0 the request counter, +4 the packets (0x40
    /// bytes each, +0x10 flags with bit 0 in use, +0x18 request id), +8 their
    /// number.
    const USB_KB_TEST_SIF_POOL: u32 = 0x0052_d680;
    const USB_KB_TEST_PACKET_FLAGS: u32 = 0x10;
    const USB_KB_TEST_PACKET_ID: u32 = 0x18;
    /// The SIF command word `_sceSifSendCmd` reads.
    const USB_KB_TEST_SIF_CMD_WORD: u32 = 0x0052_bce0;
    /// sceSifLoadFileInit's bind word (>= 0: bound).
    const USB_KB_TEST_LOADFILE_BOUND: u32 = 0x0045_5228;
    /// The words `libkernl_LoadfileNeedsRebind` compares: the loader's id,
    /// the reference id, and a pointer to a third.
    const USB_KB_TEST_REBIND_ID: u32 = 0x0052_f2a8;
    const USB_KB_TEST_REBIND_REF: u32 = 0x0045_5164;
    const USB_KB_TEST_REBIND_PTR: u32 = 0x0045_522c;

    // Fixture data.
    /// The keyboard data array.
    const USB_KB_TEST_DATA: u32 = DATA + 0x2000;
    /// libusbkb's settings array.
    const USB_KB_TEST_SETTINGS: u32 = DATA + 0x7400;
    /// The SIF pool's packets (four) and the RPC client's last packet.
    const USB_KB_TEST_PACKETS: u32 = DATA + 0x6_0000;
    const USB_KB_TEST_CLIENT_PACKET: u32 = DATA + 0x6_0400;
    /// A word no code writes (the ids, request and saved words start as it).
    const USB_KB_TEST_MARK: u32 = 0x5555_5555;
    /// The fake connect and hold callbacks.
    const USB_KB_TEST_CONNECT_CB: u32 = FAKE + 0x20;
    const USB_KB_TEST_HOLD_CB: u32 = FAKE + 0x30;

    /// Event hook of the plain tests whose real callees reach `EIntr` (it
    /// makes the next check due): nothing is delivered; past the cycle
    /// budget the test fails.
    fn usb_kb_quiet_event(c: &mut Cpu) -> bool {
        assert!(c.cycles < CYCLE_BUDGET, "runaway at pc {:#x}", c.pc);
        false
    }

    /// Run the generated body for every seed (fakes never unwinding) and
    /// require that `arm`, read from the machine afterwards, takes every
    /// value in `want`: the fixture really reaches each arm, where the arms'
    /// cycle totals depend on fake callees.
    fn usb_kb_require_arms(
        generated: GuestFn,
        seeds: u32,
        want: &[u32],
        setup: &dyn Fn(&mut Cpu, u32),
        arm: &dyn Fn(&mut Cpu, u32) -> u32,
    ) {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..seeds {
            let mut c = fresh();
            setup(&mut c, seed);
            c.bus.write32(LOG_UNWIND_AT, u32::MAX);
            run_generated(&mut c, generated);
            seen.insert(arm(&mut c, seed));
        }
        for w in want {
            assert!(seen.contains(w), "arm {w} never taken (seen {seen:?})");
        }
    }

    /// Write `s` and its NUL at `at`.
    fn usb_kb_put_cstr(c: &mut Cpu, at: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(at + i as u32, *b);
        }
        c.bus.write8(at + s.len() as u32, 0);
    }

    /// Status bytes of eight keyboards from the base-4 digits of `pattern`:
    /// 0, 1, 2 or 0x80.
    fn usb_kb_status(pattern: u32) -> [u8; 8] {
        std::array::from_fn(|i| [0u8, 1, 2, 0x80][(pattern >> (2 * i) & 3) as usize])
    }

    /// The state block: junk in every word, then `max` keyboards, their
    /// status and last reported status bytes, `request`, the mark in the
    /// busy byte and the saved words, the data array (junk) and the fake
    /// callbacks.
    fn usb_kb_state_fixture(
        c: &mut Cpu,
        r: &mut Rng,
        max: u32,
        connected: [u8; 8],
        seen: [u8; 8],
        request: u32,
    ) {
        fill(c, r, USB_KB_STATE, 0x34);
        c.bus.write32(USB_KB_STATE + USB_KB_MAX, max);
        for i in 0..8u32 {
            c.bus
                .write8(USB_KB_STATE + USB_KB_CONNECTED + i, connected[i as usize]);
            c.bus.write8(USB_KB_STATE + USB_KB_SEEN + i, seen[i as usize]);
        }
        c.bus.write32(USB_KB_STATE + USB_KB_REQUEST, request);
        c.bus.write8(USB_KB_STATE + USB_KB_BUSY, 0x55);
        c.bus.write32(USB_KB_STATE + USB_KB_PREV_LED, USB_KB_TEST_MARK);
        c.bus.write32(USB_KB_STATE + USB_KB_PREV_MKEY, USB_KB_TEST_MARK);
        c.bus.write32(USB_KB_STATE + USB_KB_DATA, USB_KB_TEST_DATA);
        fill(c, r, USB_KB_TEST_DATA - 0x200, 0x80 + 0x8c * 10 / 4);
        c.bus
            .write32(USB_KB_STATE + USB_KB_CONNECT_CB, USB_KB_TEST_CONNECT_CB);
        c.bus.write32(USB_KB_STATE + USB_KB_HOLD_CB, USB_KB_TEST_HOLD_CB);
    }

    /// Keyboard `i` per `kinds[i]`: not connected (0), connected with its data
    /// not yet taken (1: fresh byte 1; 3: status 0x80, fresh 0xff), or
    /// connected and ready for a read (2: fresh 0).
    fn usb_kb_keyboards_fixture(c: &mut Cpu, kinds: [u32; 8]) {
        for (i, &kind) in kinds.iter().enumerate() {
            let (status, fresh) = [(0u8, 1u8), (1, 1), (2, 0), (0x80, 0xff)][kind as usize % 4];
            c.bus
                .write8(USB_KB_STATE + USB_KB_CONNECTED + i as u32, status);
            let entry = USB_KB_TEST_DATA + USB_KB_DATA_SIZE as u32 * i as u32;
            c.bus.write8(entry + USB_KB_DATA_FRESH, fresh);
        }
    }

    /// libusbkb: semaphore valid or -1, `limit` connectable keyboards, the
    /// settings array, nothing pending, fake result callbacks; the SIF pool
    /// with `packets` free packets (junk, in-use bit clear).
    fn usb_kb_lib_fixture(c: &mut Cpu, r: &mut Rng, sema_ok: bool, limit: u32, packets: u32) {
        c.bus
            .write32(USB_KB_TEST_LIB_SEMA, if sema_ok { 0x2a } else { u32::MAX });
        c.bus.write32(USB_KB_TEST_LIB + USB_KB_TEST_LIB_LIMIT, limit);
        c.bus
            .write32(USB_KB_TEST_LIB + USB_KB_TEST_LIB_SETTINGS, USB_KB_TEST_SETTINGS);
        c.bus.write32(USB_KB_TEST_LIB + USB_KB_TEST_LIB_OP, 0);
        fill(c, r, USB_KB_TEST_SETTINGS - 0x100, 0x140);
        for k in 0..6u32 {
            c.bus
                .write32(USB_KB_TEST_LIB_RESULT_FNS + 4 * k, FAKE + 0x100 + 0x10 * k);
        }
        fill(c, r, USB_KB_TEST_SIF_CMD_WORD, 1);
        let counter = r.next_u32() | 0x10;
        c.bus.write32(USB_KB_TEST_SIF_POOL, counter);
        c.bus.write32(USB_KB_TEST_SIF_POOL + 4, USB_KB_TEST_PACKETS);
        c.bus.write32(USB_KB_TEST_SIF_POOL + 8, packets);
        fill(c, r, USB_KB_TEST_PACKETS, 0x40);
        for i in 0..4u32 {
            let at = USB_KB_TEST_PACKETS + i * 0x40 + USB_KB_TEST_PACKET_FLAGS;
            let flags = c.bus.read32(at);
            c.bus.write32(at, flags & !1);
        }
        usb_kb_rpc_client(c, r, false);
    }

    /// libusbkb's RPC client: its last request still running (`busy`: the
    /// packet's in-use bit set, same request id) or none.
    fn usb_kb_rpc_client(c: &mut Cpu, r: &mut Rng, busy: bool) {
        let packet = USB_KB_TEST_CLIENT_PACKET;
        fill(c, r, packet, 0x10);
        let id = r.next_u32();
        c.bus.write32(packet + USB_KB_TEST_PACKET_ID, id);
        let flags = c.bus.read32(packet + USB_KB_TEST_PACKET_FLAGS);
        c.bus.write32(packet + USB_KB_TEST_PACKET_FLAGS, flags | 1);
        c.bus
            .write32(USB_KB_TEST_LIB_RPC, if busy { packet } else { 0 });
        c.bus.write32(USB_KB_TEST_LIB_RPC + 4, id);
    }

    /// The SIF loader bound, and `libkernl_LoadfileNeedsRebind`'s compares
    /// all equal (no rebind) or all different (`rebind`: every load, stop and
    /// unload fails at once).
    fn usb_kb_loadfile_fixture(c: &mut Cpu, rebind: bool) {
        c.bus.write32(USB_KB_TEST_LOADFILE_BOUND, 0);
        c.bus
            .write32(USB_KB_TEST_REBIND_ID, if rebind { 0x3131_3131 } else { 0 });
        c.bus
            .write32(USB_KB_TEST_REBIND_REF, if rebind { 0x3232_3232 } else { 0 });
        c.bus.write32(USB_KB_TEST_REBIND_PTR, 0);
    }

    // -----------------------------------------------------------------------
    // UsbKb_LoadModules

    /// `seed % 6`: no directory (0), no keyboard arguments (1), `USBD.IRX`
    /// failing for want of an RPC packet (2) or because the loader must
    /// rebind (3), `USBKB.IRX` failing (one packet: 4), both loading (5).
    /// `seed / 6 % 2`: keyboard arguments or an empty string. The loads run
    /// the real SIF loader; its four syscalls per load are fakes (some
    /// unwinding).
    fn usb_kb_load_modules_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3f_5da8, 8);
        c.hooks.event = usb_kb_quiet_event;
        elf_image(c);
        c.pc = 0x003f_5da8;
        let arm = seed % 6;
        let sp_now = c.g[SP][0] as u32;
        fill(c, &mut r, sp_now - 0x140, 0x50);
        fill(c, &mut r, USB_KB_STATE, 0x34);
        c.bus.write32(USB_KB_STATE + USB_KB_USBD_ID, USB_KB_TEST_MARK);
        c.bus.write32(USB_KB_STATE + USB_KB_USBKB_ID, USB_KB_TEST_MARK);
        let packets = [2, 2, 0, 2, 1, 2][arm as usize];
        usb_kb_lib_fixture(c, &mut r, true, 8, packets);
        usb_kb_loadfile_fixture(c, arm == 3);
        let dir = DATA + 0x100;
        usb_kb_put_cstr(c, dir, b"cdrom0:\\MODULES\\");
        let args = DATA + 0x200;
        let text: &[u8] = if seed / 6 % 2 == 0 { b"keybd=1" } else { b"" };
        usb_kb_put_cstr(c, args, text);
        c.g[A0][0] = if arm == 0 { 0 } else { sx(dir) };
        c.g[A1][0] = if arm == 1 { 0 } else { sx(args) };
    }

    /// The arm by the result and the two ids (the mark: never written).
    fn usb_kb_load_modules_arm(c: &mut Cpu, _seed: u32) -> u32 {
        let v0 = c.g[V0][0] as i64;
        let usbd = c.bus.read32(USB_KB_STATE + USB_KB_USBD_ID);
        let usbkb = c.bus.read32(USB_KB_STATE + USB_KB_USBKB_ID);
        let written = |id: u32| id != USB_KB_TEST_MARK;
        match (v0, usbd as i32, usbkb as i32) {
            (0, -1, -1) => 0,
            (-1, _, _) if !written(usbd) && !written(usbkb) => 1,
            (-1, d, _) if d < 0 && !written(usbkb) => 2,
            (-1, d, k) if d >= 0 && written(usbd) && k < 0 => 3,
            (0, d, k) if d >= 0 && k >= 0 && written(usbd) && written(usbkb) => 4,
            _ => 99,
        }
    }

    #[test]
    fn usb_kb_load_modules_matches() {
        check(
            ee::s14::f_003f5da8__generated,
            usb_kb_load_modules,
            72,
            &usb_kb_load_modules_setup,
        );
        usb_kb_require_arms(
            ee::s14::f_003f5da8__generated,
            72,
            &[0, 1, 2, 3, 4],
            &usb_kb_load_modules_setup,
            &usb_kb_load_modules_arm,
        );
    }

    // -----------------------------------------------------------------------
    // UsbKb_UnloadModule

    /// `seed % 4 == 0`: a negative module id (nothing to do); else id 1-3,
    /// stopped and unloaded by the real SIF loader with `seed / 4 % 3` free
    /// RPC packets (none: both fail; one: the stop works, the unload fails;
    /// two: both work), every call failing at once when `seed / 12 % 2` says
    /// the loader must rebind. Syscalls are fakes (some unwinding).
    fn usb_kb_unload_module_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3f_5e90, 8);
        c.hooks.event = usb_kb_quiet_event;
        c.pc = 0x003f_5e90;
        usb_kb_lib_fixture(c, &mut r, true, 8, seed / 4 % 3);
        usb_kb_loadfile_fixture(c, seed / 12 % 2 == 1);
        c.g[A0][0] = if seed % 4 == 0 {
            sx(0u32.wrapping_sub(1 + seed / 24 % 3))
        } else {
            (seed % 4) as u64
        };
        c.g[A1][0] = r.next_u32() as u64 | 1 << 40;
    }

    /// Skipped (0), both done (1: fakes ran, result 0), failed (2).
    fn usb_kb_unload_module_arm(c: &mut Cpu, _seed: u32) -> u32 {
        if c.g[V0][0] != 0 {
            2
        } else if c.bus.read32(LOG_COUNT) == 0 {
            0
        } else {
            1
        }
    }

    #[test]
    fn usb_kb_unload_module_matches() {
        check(
            ee::s14::f_003f5e90__generated,
            usb_kb_unload_module,
            72,
            &usb_kb_unload_module_setup,
        );
        usb_kb_require_arms(
            ee::s14::f_003f5e90__generated,
            72,
            &[0, 1, 2],
            &usb_kb_unload_module_setup,
            &usb_kb_unload_module_arm,
        );
    }

    // -----------------------------------------------------------------------
    // UsbKb_RequestInfo

    /// `seed % 5`: 0, 1, 3 or 8 keyboards or a negative count (no copy);
    /// `seed / 5 % 2`: libusbkb's semaphore -1 (the request fails at once)
    /// or valid (it goes out over the real `sceSifCallRpc`, two fake
    /// syscalls, some unwinding).
    fn usb_kb_request_info_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3f_5f08, 2);
        c.hooks.event = usb_kb_quiet_event;
        c.pc = 0x003f_5f08;
        let max = [0u32, 1, 3, 8, u32::MAX - 2][(seed % 5) as usize];
        let connected = usb_kb_status((seed / 10).wrapping_mul(0x9e37_79b9));
        usb_kb_state_fixture(c, &mut r, max, connected, [0x55; 8], USB_KB_TEST_MARK);
        usb_kb_lib_fixture(c, &mut r, seed / 5 % 2 == 1, 8, 2);
    }

    /// Bit 1: the request went out (request 1); bit 0: the statuses were
    /// copied (the first last-reported byte is no longer 0x55).
    fn usb_kb_request_info_arm(c: &mut Cpu, _seed: u32) -> u32 {
        let sent = c.bus.read32(USB_KB_STATE + USB_KB_REQUEST) == USB_KB_REQ_INFO as u32
            && c.bus.read8(USB_KB_STATE + USB_KB_BUSY) == 1;
        let copied = c.bus.read8(USB_KB_STATE + USB_KB_SEEN) != 0x55;
        (sent as u32) << 1 | copied as u32
    }

    #[test]
    fn usb_kb_request_info_matches() {
        // Semaphore -1: 29 cycles plus 2 + 7 per keyboard copied.
        check_paths(
            ee::s14::f_003f5f08__generated,
            usb_kb_request_info,
            60,
            &[29, 38, 52, 87],
            &usb_kb_request_info_setup,
        );
        usb_kb_require_arms(
            ee::s14::f_003f5f08__generated,
            60,
            &[0, 1, 2, 3],
            &usb_kb_request_info_setup,
            &usb_kb_request_info_arm,
        );
    }

    /// Eight keyboards: the copy loop's head 0x3f5f30 meets the due event
    /// (`schedule`: `seed % 3` none, an unwinding VBlank, a plain delivery).
    #[test]
    fn usb_kb_request_info_events() {
        driven!(gen, ee::s14::f_003f5f08__generated);
        driven!(hand, usb_kb_request_info);
        check_events(gen, hand, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x3f_5f09, 0);
            let connected = usb_kb_status((seed / 3).wrapping_mul(0x9e37_79b9));
            usb_kb_state_fixture(c, &mut r, 8, connected, [0x55; 8], USB_KB_TEST_MARK);
            usb_kb_lib_fixture(c, &mut r, false, 8, 0);
            schedule(c, &mut r, seed, 0x003f_5f08)
        });
    }

    // -----------------------------------------------------------------------
    // UsbKb_SyncInfo

    /// `seed % 3`: `sceUsbKbSync` fails (semaphore -1), says the info
    /// request still runs (1), or that it is done (0: the walk over the
    /// keyboards). `seed / 3 % 4`: 0, 2 or 8 keyboards or a negative count.
    /// `seed / 12` picks each keyboard's status and last reported status
    /// (0, 1, 2, 0x80), so some differ and call the connect callback (a
    /// fake, some unwinding).
    fn usb_kb_sync_info_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3f_5f88, 4);
        c.pc = 0x003f_5f88;
        let max = [0u32, 2, 8, u32::MAX][(seed / 3 % 4) as usize];
        let digit = seed / 12;
        let connected = usb_kb_status(digit.wrapping_mul(0x9e37_79b9));
        let seen = usb_kb_status(digit.wrapping_mul(0x85eb_ca6b));
        usb_kb_state_fixture(c, &mut r, max, connected, seen, USB_KB_TEST_MARK);
        usb_kb_lib_fixture(c, &mut r, seed % 3 != 0, 8, 0);
        usb_kb_rpc_client(c, &mut r, seed % 3 == 1);
    }

    /// Sync failed (0), still running (1), done with no change (2) or with
    /// the connect callback called (3).
    fn usb_kb_sync_info_arm(c: &mut Cpu, _seed: u32) -> u32 {
        match c.g[V0][0] {
            u64::MAX => 0,
            1 => 1,
            0 if c.bus.read32(LOG_COUNT) == 0 => 2,
            0 => 3,
            _ => 99,
        }
    }

    #[test]
    fn usb_kb_sync_info_matches() {
        check(
            ee::s14::f_003f5f88__generated,
            usb_kb_sync_info,
            144,
            &usb_kb_sync_info_setup,
        );
        usb_kb_require_arms(
            ee::s14::f_003f5f88__generated,
            144,
            &[0, 1, 2, 3],
            &usb_kb_sync_info_setup,
            &usb_kb_sync_info_arm,
        );
    }

    /// The sync done, eight keyboards: the walk's head 0x3f5fe0 meets the
    /// due event (first back edge already past it: the sync takes longer
    /// than the scheduled delay).
    #[test]
    fn usb_kb_sync_info_events() {
        driven!(gen, ee::s14::f_003f5f88__generated);
        driven!(hand, usb_kb_sync_info);
        check_events(gen, hand, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x3f_5f89, 0);
            let digit = seed / 3;
            let connected = usb_kb_status(digit.wrapping_mul(0x9e37_79b9));
            let seen = usb_kb_status(digit.wrapping_mul(0x85eb_ca6b));
            usb_kb_state_fixture(c, &mut r, 8, connected, seen, USB_KB_TEST_MARK);
            usb_kb_lib_fixture(c, &mut r, true, 8, 0);
            schedule(c, &mut r, seed, 0x003f_5f88)
        });
    }

    // -----------------------------------------------------------------------
    // UsbKb_FindNextReady

    /// `seed % 6`: 0, 1, 2, 5 or 8 keyboards or a negative count;
    /// `seed / 6 % 6`: the first keyboard to look at (0, 1, the last, one
    /// past it, far past it (both wrap), -1); `seed / 36 % 4`: keyboard 0's
    /// kind (see `usb_kb_keyboards_fixture`), the others' from that digit
    /// scrambled.
    fn usb_kb_find_next_ready_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3f_6040, 0);
        c.pc = 0x003f_6040;
        let max = [0u32, 1, 2, 5, 8, u32::MAX][(seed % 6) as usize];
        usb_kb_state_fixture(c, &mut r, max, [0; 8], [0; 8], USB_KB_TEST_MARK);
        let digit = seed / 36;
        let others = digit.wrapping_mul(0x2545_f491);
        let kinds: [u32; 8] =
            std::array::from_fn(|i| if i == 0 { digit % 4 } else { others >> (2 * i) & 3 });
        usb_kb_keyboards_fixture(c, kinds);
        let first = [0, 1, max.wrapping_sub(1), max, max.wrapping_add(5), u32::MAX]
            [(seed / 6 % 6) as usize];
        c.g[A0][0] = sx(first);
    }

    #[test]
    fn usb_kb_find_next_ready_matches() {
        // No keyboards (8); one keyboard: not connected (22), ready (23),
        // data not taken (26).
        check_paths_lohi(
            ee::s14::f_003f6040__generated,
            usb_kb_find_next_ready,
            216,
            &[8, 22, 23, 26],
            &usb_kb_find_next_ready_setup,
        );
    }

    /// Eight keyboards, none or only the seventh ready: the search's head
    /// 0x3f6068 meets the due event.
    #[test]
    fn usb_kb_find_next_ready_events() {
        driven!(gen, ee::s14::f_003f6040__generated);
        driven!(hand, usb_kb_find_next_ready);
        check_events(gen, hand, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x3f_6041, 0);
            usb_kb_state_fixture(c, &mut r, 8, [0; 8], [0; 8], USB_KB_TEST_MARK);
            let kinds = if seed / 3 % 2 == 0 {
                [0, 1, 0, 3, 1, 0, 1, 0]
            } else {
                [1, 0, 3, 1, 0, 1, 2, 0]
            };
            usb_kb_keyboards_fixture(c, kinds);
            c.g[A0][0] = (seed / 6 % 3) as u64;
            schedule(c, &mut r, seed, 0x003f_6040)
        });
    }

    // -----------------------------------------------------------------------
    // UsbKb_RequestRead

    /// `seed % 3`: no keyboard ready (0); four ready keyboards whose read
    /// fails (1: semaphore -1 or, `seed / 3 % 2`, the keyboard past
    /// libusbkb's limit) or goes out over the real `sceSifCallRpc` (2: two
    /// fake syscalls, some unwinding). `seed / 6 % 4`: the last keyboard
    /// read, so the search starts after it (and wraps).
    fn usb_kb_request_read_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3f_60b8, 2);
        c.hooks.event = usb_kb_quiet_event;
        c.pc = 0x003f_60b8;
        let arm = seed % 3;
        usb_kb_state_fixture(c, &mut r, 4, [0; 8], [0; 8], USB_KB_TEST_MARK);
        let kind = if arm == 0 { 1 } else { 2 };
        usb_kb_keyboards_fixture(c, [kind, kind, kind, kind, 0, 0, 0, 0]);
        c.bus
            .write32(USB_KB_STATE + USB_KB_READ_PORT, seed / 6 % 4);
        let (sema_ok, limit) = match arm {
            1 if seed / 3 % 2 == 0 => (false, 8),
            1 => (true, 0),
            _ => (true, 8),
        };
        usb_kb_lib_fixture(c, &mut r, sema_ok, limit, 2);
    }

    /// No read (0: the saved LED word still the mark), the read failed (1),
    /// the read went out (2: request 2).
    fn usb_kb_request_read_arm(c: &mut Cpu, _seed: u32) -> u32 {
        if c.bus.read32(USB_KB_STATE + USB_KB_PREV_LED) == USB_KB_TEST_MARK {
            0
        } else if c.bus.read32(USB_KB_STATE + USB_KB_REQUEST) == USB_KB_REQ_READ as u32 {
            2
        } else {
            1
        }
    }

    #[test]
    fn usb_kb_request_read_matches() {
        check_lohi(
            ee::s14::f_003f60b8__generated,
            usb_kb_request_read,
            48,
            &usb_kb_request_read_setup,
        );
        usb_kb_require_arms(
            ee::s14::f_003f60b8__generated,
            48,
            &[0, 1, 2],
            &usb_kb_request_read_setup,
            &usb_kb_request_read_arm,
        );
    }

    // -----------------------------------------------------------------------
    // UsbKb_SyncRead

    /// `seed % 2`: `sceUsbKbSync` fails (semaphore -1) or completes (two
    /// fake syscalls, some unwinding; `seed / 2 % 2`: with libusbkb's read
    /// result pending, its result callback, a fake, runs too). Then
    /// `seed / 4 % 4`: the read keyboard's data brought keys, a new modifier
    /// word, a new LED word (key count negative), or nothing new;
    /// `seed / 16 % 3`: which keyboard was read.
    fn usb_kb_sync_read_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3f_6138, 3);
        c.hooks.event = usb_kb_quiet_event;
        c.pc = 0x003f_6138;
        usb_kb_state_fixture(c, &mut r, 4, [1; 8], [1; 8], USB_KB_TEST_MARK);
        usb_kb_lib_fixture(c, &mut r, seed % 2 == 1, 8, 0);
        if seed / 2 % 2 == 1 {
            c.bus.write32(USB_KB_TEST_LIB + USB_KB_TEST_LIB_OP, 2);
        }
        let port = seed / 16 % 3;
        c.bus.write32(USB_KB_STATE + USB_KB_READ_PORT, port);
        let entry = USB_KB_TEST_DATA + USB_KB_DATA_SIZE as u32 * port;
        c.bus.write8(entry + USB_KB_DATA_FRESH, 0x55);
        let led = r.next_u32();
        let mkey = r.next_u32();
        c.bus.write32(USB_KB_STATE + USB_KB_PREV_LED, led);
        c.bus.write32(USB_KB_STATE + USB_KB_PREV_MKEY, mkey);
        let (len, new_mkey, new_led) = match seed / 4 % 4 {
            0 => (1 + seed / 48 % 5, false, false),
            1 => (0, true, false),
            2 => (u32::MAX, false, true),
            _ => (0, false, false),
        };
        c.bus.write32(entry + USB_KB_DATA_LEN, len);
        c.bus
            .write32(entry + USB_KB_DATA_MKEY, if new_mkey { mkey ^ 0x22 } else { mkey });
        c.bus
            .write32(entry + USB_KB_DATA_LED, if new_led { led ^ 0x1 } else { led });
    }

    /// Sync failed (0); marked fresh for keys (1), the modifier word (2),
    /// the LED word (3); nothing new (4).
    fn usb_kb_sync_read_arm(c: &mut Cpu, _seed: u32) -> u32 {
        if c.g[V0][0] != 0 {
            return 0;
        }
        let port = c.bus.read32(USB_KB_STATE + USB_KB_READ_PORT);
        let entry = USB_KB_TEST_DATA + USB_KB_DATA_SIZE as u32 * port;
        let fresh = c.bus.read8(entry + USB_KB_DATA_FRESH);
        let len = c.bus.read32(entry + USB_KB_DATA_LEN) as i32;
        let mkey_new = c.bus.read32(entry + USB_KB_DATA_MKEY)
            != c.bus.read32(USB_KB_STATE + USB_KB_PREV_MKEY);
        let led_new = c.bus.read32(entry + USB_KB_DATA_LED)
            != c.bus.read32(USB_KB_STATE + USB_KB_PREV_LED);
        match (fresh, len > 0, mkey_new, led_new) {
            (1, true, _, _) => 1,
            (1, false, true, _) => 2,
            (1, false, false, true) => 3,
            (0x55, false, false, false) => 4,
            _ => 99,
        }
    }

    #[test]
    fn usb_kb_sync_read_matches() {
        check_lohi(
            ee::s14::f_003f6138__generated,
            usb_kb_sync_read,
            96,
            &usb_kb_sync_read_setup,
        );
        usb_kb_require_arms(
            ee::s14::f_003f6138__generated,
            96,
            &[0, 1, 2, 3, 4],
            &usb_kb_sync_read_setup,
            &usb_kb_sync_read_arm,
        );
    }

    // -----------------------------------------------------------------------
    // UsbKb_Init

    /// The modules never get far enough in a fixture for `sceUsbKbInit` (a
    /// full IOP bind that retries until the IOP answers), so only the exit
    /// after a failed `UsbKb_LoadModules` runs: `seed % 4` no keyboard
    /// arguments, the loader must rebind, no RPC packet for `USBD.IRX`, none
    /// for `USBKB.IRX`. `seed / 4 % 2`: callbacks given or null.
    fn usb_kb_init_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3f_61d0, 4);
        c.hooks.event = usb_kb_quiet_event;
        elf_image(c);
        c.pc = 0x003f_61d0;
        let arm = seed % 4;
        usb_kb_state_fixture(c, &mut r, 3, [1; 8], [1; 8], USB_KB_TEST_MARK);
        usb_kb_lib_fixture(c, &mut r, true, 8, [2, 2, 0, 1][arm as usize]);
        usb_kb_loadfile_fixture(c, arm == 1);
        let dir = DATA + 0x100;
        usb_kb_put_cstr(c, dir, b"host0:");
        let args = DATA + 0x200;
        usb_kb_put_cstr(c, args, b"kb");
        c.g[A0][0] = sx(dir);
        c.g[A1][0] = if arm == 0 { 0 } else { sx(args) };
        c.g[A2][0] = sx(USB_KB_TEST_DATA);
        let given = seed / 4 % 2 == 0;
        c.g[A3][0] = if given { sx(USB_KB_TEST_CONNECT_CB) } else { 0 };
        c.g[T0][0] = if given { sx(USB_KB_TEST_HOLD_CB) } else { 0 };
    }

    #[test]
    fn usb_kb_init_matches() {
        check(ee::s14::f_003f61d0__generated, usb_kb_init, 32, &usb_kb_init_setup);
    }

    // -----------------------------------------------------------------------
    // UsbKb_Shutdown

    /// `sceUsbKbEnd` runs for real (its syscalls are fakes, some unwinding);
    /// `seed % 2`: libusbkb's semaphore valid (deleted) or -1. Both modules
    /// unload for real: `seed / 2 % 3` and `seed / 6 % 3` the `USBKB.IRX` and
    /// `USBD.IRX` ids (negative: skipped), `seed / 18 % 3` the loader (must
    /// rebind: every call fails; one free packet; four).
    fn usb_kb_shutdown_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3f_62f8, 12);
        c.hooks.event = usb_kb_quiet_event;
        c.pc = 0x003f_62f8;
        usb_kb_state_fixture(c, &mut r, 3, [1; 8], [1; 8], USB_KB_TEST_MARK);
        let id = |d: u32| [u32::MAX, 3, 5][d as usize];
        c.bus
            .write32(USB_KB_STATE + USB_KB_USBKB_ID, id(seed / 2 % 3));
        c.bus.write32(USB_KB_STATE + USB_KB_USBD_ID, id(seed / 6 % 3));
        let loader = seed / 18 % 3;
        usb_kb_lib_fixture(c, &mut r, seed % 2 == 1, 8, [0, 1, 4][loader as usize]);
        usb_kb_loadfile_fixture(c, loader == 0);
    }

    #[test]
    fn usb_kb_shutdown_matches() {
        check(
            ee::s14::f_003f62f8__generated,
            usb_kb_shutdown,
            54,
            &usb_kb_shutdown_setup,
        );
    }

    // -----------------------------------------------------------------------
    // UsbKb_GetMaxKeyboards

    #[test]
    fn usb_kb_get_max_keyboards_matches() {
        check(
            ee::s14::f_003f6348__generated,
            usb_kb_get_max_keyboards,
            30,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3f_6348, 0);
                c.pc = 0x003f_6348;
                fill(c, &mut r, USB_KB_STATE, 0x34);
            },
        );
    }

    // -----------------------------------------------------------------------
    // UsbKb_DefaultConnectCb

    /// `seed % 4 == 0`: the keyboard went away (nothing to do); else (status
    /// 1, 2 or only upper bits) its settings go out: `seed / 4 % 3` every
    /// `sceUsbKb*` call fails (semaphore -1, or the keyboard past libusbkb's
    /// limit) or the requests go out over the real `sceSifCallRpc`, each
    /// followed by a real `sceUsbKbSync` (fake syscalls and result callbacks,
    /// some unwinding). `seed / 12 % 4`: the keyboard.
    fn usb_kb_default_connect_cb_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3f_6358, 12);
        c.hooks.event = usb_kb_quiet_event;
        c.pc = 0x003f_6358;
        let keyboard = seed / 12 % 4;
        let calls = seed / 4 % 3;
        let limit = if calls == 1 { keyboard } else { 8 };
        usb_kb_lib_fixture(c, &mut r, calls != 0, limit, 4);
        c.g[A0][0] = keyboard as u64;
        c.g[A1][0] = [0u64, 1, 2, 1 << 33][(seed % 4) as usize];
    }

    /// Nothing done (0: the repeat setting untouched), every call failed
    /// (1: no fakes), the requests went out (2).
    fn usb_kb_default_connect_cb_arm(c: &mut Cpu, seed: u32) -> u32 {
        let keyboard = seed / 12 % 4;
        let repeat_at = USB_KB_TEST_SETTINGS + 0x38 * keyboard + 0x18;
        if c.bus.read32(repeat_at) != USB_KB_REPEAT_START as u32 {
            0
        } else if c.bus.read32(LOG_COUNT) == 0 {
            1
        } else {
            2
        }
    }

    #[test]
    fn usb_kb_default_connect_cb_matches() {
        check(
            ee::s14::f_003f6358__generated,
            usb_kb_default_connect_cb,
            48,
            &usb_kb_default_connect_cb_setup,
        );
        usb_kb_require_arms(
            ee::s14::f_003f6358__generated,
            48,
            &[0, 1, 2],
            &usb_kb_default_connect_cb_setup,
            &usb_kb_default_connect_cb_arm,
        );
    }

    // -----------------------------------------------------------------------
    // UsbKb_Update

    /// The pending requests `seed % 5` picks.
    const USB_KB_TEST_UPDATE_REQUESTS: [u32; 5] = [0, 1, 2, 3, 5];

    /// `UsbKb_Update` from each pending request (`seed % 5`: none, info,
    /// read, held, an unknown value). `seed / 5 % 2`: libusbkb's semaphore
    /// -1 (every library call fails at once) or valid (the syncs complete;
    /// new requests go out over the real `sceSifCallRpc`, fake syscalls,
    /// some unwinding). The hold callback is `UsbKb_IsConnected`, so
    /// keyboards 1 (`seed / 10 % 2`) and 0 (`seed / 20 % 2`) connected are
    /// its answers to "release" and "hold", or (`seed / 80 % 2`) a fake
    /// (always yes). `seed / 40 % 2`: the poll count reaches the period (an
    /// info request) or not (a read).
    fn usb_kb_update_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3f_63f0, 6);
        c.hooks.event = usb_kb_quiet_event;
        c.pc = 0x003f_63f0;
        let request = USB_KB_TEST_UPDATE_REQUESTS[(seed % 5) as usize];
        let mut connected = [0u8; 8];
        connected[1] = (seed / 10 % 2) as u8;
        connected[0] = (seed / 20 % 2) as u8 * 2;
        usb_kb_state_fixture(c, &mut r, 2, connected, [1, 1, 0, 0, 0, 0, 0, 0], request);
        let hold = if seed / 80 % 2 == 0 {
            0x003f_6520
        } else {
            USB_KB_TEST_HOLD_CB
        };
        c.bus.write32(USB_KB_STATE + USB_KB_HOLD_CB, hold);
        c.bus.write32(USB_KB_STATE + USB_KB_POLL_PERIOD, 60);
        let count = if seed / 40 % 2 == 0 { 60 } else { 7 };
        c.bus.write32(USB_KB_STATE + USB_KB_POLL_COUNT, count);
        c.bus.write32(USB_KB_STATE + USB_KB_READ_PORT, 1);
        for i in 0..2u32 {
            let entry = USB_KB_TEST_DATA + USB_KB_DATA_SIZE as u32 * i;
            c.bus.write8(entry + USB_KB_DATA_FRESH, 0);
        }
        usb_kb_lib_fixture(c, &mut r, seed / 5 % 2 == 1, 8, 4);
    }

    /// `request * 100 + poll * 10 + held`: the starting request; the poll
    /// count untouched (0), counted for a read (1) or reset for an info
    /// request (2); the library held afterwards (request 3).
    fn usb_kb_update_arm(c: &mut Cpu, seed: u32) -> u32 {
        let request = USB_KB_TEST_UPDATE_REQUESTS[(seed % 5) as usize];
        let start = if seed / 40 % 2 == 0 { 60 } else { 7 };
        let count = c.bus.read32(USB_KB_STATE + USB_KB_POLL_COUNT);
        let poll = match count {
            0 => 2,
            n if n == start + 1 => 1,
            n if n == start => 0,
            _ => 9,
        };
        let held = c.bus.read32(USB_KB_STATE + USB_KB_REQUEST) == USB_KB_REQ_HELD as u32;
        request * 100 + poll * 10 + held as u32
    }

    #[test]
    fn usb_kb_update_matches() {
        check(ee::s14::f_003f63f0__generated, usb_kb_update, 160, &usb_kb_update_setup);
        usb_kb_require_arms(
            ee::s14::f_003f63f0__generated,
            160,
            &[
                1, 10, 20, // idle: held, read, info
                100, 101, 110, 120, // info: still running, then idle
                200, 201, 210, 220, // read: failed, then idle
                301, 310, 320, // held: stays (or held again), released
                500, // unknown request
            ],
            &usb_kb_update_setup,
            &usb_kb_update_arm,
        );
    }

    // -----------------------------------------------------------------------
    // UsbKb_GetData

    #[test]
    fn usb_kb_get_data_matches() {
        // Fresh (13) or not (12).
        check_paths_lohi(
            ee::s14::f_003f64e0__generated,
            usb_kb_get_data,
            40,
            &[12, 13],
            &|c, seed| {
                let mut r = machine(c, seed, 0x3f_64e0, 0);
                c.pc = 0x003f_64e0;
                usb_kb_state_fixture(c, &mut r, 8, [1; 8], [1; 8], USB_KB_TEST_MARK);
                let keyboard = [0u32, 1, 7, 8, u32::MAX][(seed % 5) as usize];
                let entry = USB_KB_TEST_DATA.wrapping_add(
                    (USB_KB_DATA_SIZE as u32).wrapping_mul(keyboard),
                );
                let fresh = [0u8, 1, 2, 0xff][(seed / 5 % 4) as usize];
                c.bus.write8(entry + USB_KB_DATA_FRESH, fresh);
                c.g[A0][0] = sx(keyboard);
            },
        );
    }

    // -----------------------------------------------------------------------
    // UsbKb_IsConnected

    #[test]
    fn usb_kb_is_connected_matches() {
        check(
            ee::s14::f_003f6520__generated,
            usb_kb_is_connected,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3f_6520, 0);
                c.pc = 0x003f_6520;
                let connected = usb_kb_status((seed / 10).wrapping_mul(0x9e37_79b9));
                usb_kb_state_fixture(c, &mut r, 8, connected, [0; 8], USB_KB_TEST_MARK);
                c.g[A0][0] = [0u64, 1, 5, 7, 0xffff_ffff_ffff_fff0][(seed % 5) as usize]
                    | ((seed / 5 % 2) as u64) << 36;
            },
        );
    }

    // -----------------------------------------------------------------------
    // UsbKb_AnyConnected

    /// `seed % 4`: 0, 1, 3 or 8 keyboards; `seed / 4 % 5`: none connected,
    /// or the first connected one is 0, 1, 2 or 5 (status `seed / 20 % 3`:
    /// 1, 2 or 0x80). `UsbKb_Update` finds an unknown request (no calls).
    fn usb_kb_any_connected_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3f_6538, 0);
        c.pc = 0x003f_6538;
        let max = [0u32, 1, 3, 8][(seed % 4) as usize];
        let mut connected = [0u8; 8];
        if let Some(k) = [None, Some(0), Some(1), Some(2), Some(5)][(seed / 4 % 5) as usize] {
            connected[k] = [1u8, 2, 0x80][(seed / 20 % 3) as usize];
        }
        usb_kb_state_fixture(c, &mut r, max, connected, [0; 8], 5);
    }

    #[test]
    fn usb_kb_any_connected_matches() {
        // No keyboards (47); keyboard 0 connected of 1 (59); keyboard 2 of 3
        // (95); none of 3 (101).
        check_paths(
            ee::s14::f_003f6538__generated,
            usb_kb_any_connected,
            60,
            &[47, 59, 95, 101],
            &usb_kb_any_connected_setup,
        );
    }

    /// Eight keyboards, none or only the eighth connected: the loop head
    /// 0x3f6560 meets the due event (an unwinding VBlank returns there, and
    /// the function is re-entered at it).
    #[test]
    fn usb_kb_any_connected_events() {
        driven!(gen, ee::s14::f_003f6538__generated);
        driven!(hand, usb_kb_any_connected);
        check_events(gen, hand, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x3f_6539, 0);
            let mut connected = [0u8; 8];
            if seed / 3 % 2 == 1 {
                connected[7] = 1;
            }
            usb_kb_state_fixture(c, &mut r, 8, connected, [0; 8], 5);
            schedule(c, &mut r, seed, 0x003f_6538)
        });
    }

    // -----------------------------------------------------------------------
    // Keys

    #[test]
    fn usb_kb_get_keypad_entry_matches() {
        // Usages 0x2e-0x35 (9), 0x87-0x8b (12), any other (11).
        const USAGES: [u64; 14] = [
            0x00, 0x04, 0x2d, 0x2e, 0x31, 0x35, 0x36, 0x86, 0x87, 0x89, 0x8b, 0x8c, 0xffff,
            0x8000_002e,
        ];
        check_paths(
            ee::s14::f_003f65a0__generated,
            usb_kb_get_keypad_entry,
            56,
            &[9, 11, 12],
            &|c, seed| {
                let _r = machine(c, seed, 0x3f_65a0, 0);
                c.pc = 0x003f_65a0;
                c.g[A0][0] = USAGES[(seed % 14) as usize] | ((seed / 14 % 2) as u64) << 40;
            },
        );
    }

    #[test]
    fn usb_kb_lookup_key_char_matches() {
        const USAGES: [u64; 12] = [
            0x04, 0x1e, 0x2f, 0x35, 0x58, 0x59, 0x62, 0x63, 0x64, 0x87, 0x8a, 0xe1,
        ];
        // No num lock: layout 0 (20) or the alternate's three kinds (27, 29,
        // 30); num lock: a keypad digit (22), else layout 0 (24) or the
        // alternate's (31, 33, 34).
        check_paths(
            ee::s14::f_003f65f8__generated,
            usb_kb_lookup_key_char,
            144,
            &[20, 22, 24, 27, 29, 30, 31, 33, 34],
            &|c, seed| {
                let mut r = machine(c, seed, 0x3f_65f8, 0);
                elf_image(c);
                c.pc = 0x003f_65f8;
                let out = DATA + 0x40;
                let data = DATA + 0x80;
                fill(c, &mut r, out, 4);
                let led = r.next_u32() & !1 | seed % 2;
                c.bus.write32(data + USB_KB_DATA_LED, led);
                c.g[A0][0] = sx(out);
                c.g[A1][0] = sx(data);
                c.g[A2][0] = [0u64, 1, 1 << 32][(seed / 2 % 3) as usize];
                c.g[A3][0] = USAGES[(seed / 6 % 12) as usize] | ((seed / 72 % 2) as u64) << 36;
            },
        );
    }

    /// The usages `UsbKb_ConvertRawCode`'s test picks: keypad operators,
    /// digits, ordinary keys, a usage past 0xff, and keypad usages with upper
    /// bits (the 64-bit `< 0x59` compare takes them for digits).
    const USB_KB_TEST_CONVERT_USAGES: [u64; 12] = [
        0x54,
        0x58,
        0x59,
        0x63,
        0x04,
        0x1e,
        0x2c,
        0x3a,
        0x139,
        0x87,
        0x1_0000_0055,
        0x1_0000_0059,
    ];

    /// `seed % 3`: the arrangement (0-2: `sceUsbKbCnvRawCode` prints and
    /// fails on others); `seed / 3 % 4`: LED and modifier words (num lock
    /// off/on, caps lock, shifts); `seed / 12 % 12`: the usage.
    fn usb_kb_convert_raw_code_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3f_6670, 0);
        elf_image(c);
        c.pc = 0x003f_6670;
        let out = DATA + 0x40;
        let data = DATA + 0x80;
        fill(c, &mut r, out, 4);
        let (led, mkey) = [(0u32, 0u32), (1, 0), (2, 0x02), (3, 0x20)][(seed / 3 % 4) as usize];
        c.bus.write32(data + USB_KB_DATA_LED, led);
        c.bus.write32(data + USB_KB_DATA_MKEY, mkey);
        c.g[A0][0] = sx(out);
        c.g[A1][0] = sx(data);
        c.g[A2][0] = (seed % 3) as u64;
        c.g[A3][0] = USB_KB_TEST_CONVERT_USAGES[(seed / 12 % 12) as usize];
    }

    /// Keypad: no character (0: a digit without num lock) or one (1);
    /// other keys: `sceUsbKbCnvRawCode` gave a character (2) or 256 or more
    /// (3, left in v1).
    fn usb_kb_convert_raw_code_arm(c: &mut Cpu, seed: u32) -> u32 {
        let usage = USB_KB_TEST_CONVERT_USAGES[(seed / 12 % 12) as usize];
        let keypad = sx((usage as u32).wrapping_sub(0x54)) < 0x10;
        if keypad {
            (c.bus.read8(DATA + 0x41) != 0) as u32
        } else {
            2 + ((c.g[V1][0] as i64) >= 0x100) as u32
        }
    }

    #[test]
    fn usb_kb_convert_raw_code_matches() {
        check(
            ee::s14::f_003f6670__generated,
            usb_kb_convert_raw_code,
            144,
            &usb_kb_convert_raw_code_setup,
        );
        // Keypad: an operator or a digit without num lock (20), a digit with
        // num lock (24).
        require_paths(
            ee::s14::f_003f6670__generated,
            144,
            &[20, 24],
            &usb_kb_convert_raw_code_setup,
        );
        usb_kb_require_arms(
            ee::s14::f_003f6670__generated,
            144,
            &[0, 1, 2, 3],
            &usb_kb_convert_raw_code_setup,
            &usb_kb_convert_raw_code_arm,
        );
    }

    #[test]
    fn usb_kb_check_modifier_key_matches() {
        const KEYS: [u64; 16] = [
            0xa0,
            0x12,
            0x10,
            0x11,
            0x05,
            0x5b,
            0x5c,
            0x41,
            0xa3,
            0xa1,
            0xa2,
            0xa4,
            0xa5,
            0xff,
            0x1_0000_00a0,
            0x13,
        ];
        check_paths(
            ee::s14::f_003f6700__generated,
            usb_kb_check_modifier_key,
            64,
            &[10, 14, 16, 19, 20, 21, 22, 23],
            &|c, seed| {
                let mut r = machine(c, seed, 0x3f_6700, 0);
                c.pc = 0x003f_6700;
                let out = DATA + 0x10 + 4 * (seed / 16 % 4);
                fill(c, &mut r, DATA, 0x10);
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = KEYS[(seed % 16) as usize];
                c.g[A2][0] = sx(out);
            },
        );
    }

    #[test]
    fn usb_kb_check_led_key_matches() {
        const KEYS: [u64; 8] = [0x90, 0x14, 0x13, 0x91, 0x92, 0x05, 0x1_0000_0090, 0x8f];
        check_paths(
            ee::s14::f_003f67e0__generated,
            usb_kb_check_led_key,
            32,
            &[9, 13, 14, 15, 16],
            &|c, seed| {
                let mut r = machine(c, seed, 0x3f_67e0, 0);
                c.pc = 0x003f_67e0;
                let out = DATA + 0x10 + 4 * (seed / 8 % 4);
                fill(c, &mut r, DATA, 0x10);
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = KEYS[(seed % 8) as usize];
                c.g[A2][0] = sx(out);
            },
        );
    }

    /// `seed % 8`: the key asked about and the usage at the index: left
    /// shift, right Windows (modifiers), num and caps lock (lock keys), 'A'
    /// with usage 4 ('A': down) or 5 ('B'), keypad 1's num-lock code with
    /// usage 0x59, 'A' with upper bits (never down). `seed / 8 % 4`: LED and
    /// modifier words; `seed / 32 % 3`: the layout.
    #[test]
    fn usb_kb_is_key_down_matches() {
        const CASES: [(u64, u16); 8] = [
            (0xa0, 0x04),
            (0x5c, 0x04),
            (0x90, 0x04),
            (0x14, 0x04),
            (0x41, 0x04),
            (0x41, 0x05),
            (0x61, 0x59),
            (0x1_0000_0041, 0x04),
        ];
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x3f_6838, 0);
            elf_image(c);
            c.pc = 0x003f_6838;
            let data = DATA + 0x100;
            fill(c, &mut r, data, 0x24);
            let (key, usage) = CASES[(seed % 8) as usize];
            let (led, mkey) =
                [(0u32, 0u32), (1, 0x02), (2, 0x80), (7, 0xff)][(seed / 8 % 4) as usize];
            c.bus.write32(data + USB_KB_DATA_LED, led);
            c.bus.write32(data + USB_KB_DATA_MKEY, mkey);
            let index = seed / 8 % 6 * 9;
            c.bus.write16(data + USB_KB_DATA_KEYS + 2 * index, usage);
            c.g[A0][0] = sx(data);
            c.g[A1][0] = (seed / 32 % 3) as u64;
            c.g[A2][0] = index as u64;
            c.g[A3][0] = key;
        };
        check(ee::s14::f_003f6838__generated, usb_kb_is_key_down, 96, &setup);
        // Left shift (39), num lock (66), 'A' with LEDs off and layout 0:
        // other key (100), down (103).
        require_paths(ee::s14::f_003f6838__generated, 96, &[39, 66, 100, 103], &setup);
    }

    /// `seed % 3`: the arrangement; `seed / 3 % 4`: LED and modifier words;
    /// `seed / 12 % 10`: the usage at the index.
    #[test]
    fn usb_kb_translate_key_matches() {
        const USAGES: [u16; 10] = [0x04, 0x1e, 0x2c, 0x2f, 0x3a, 0x54, 0x59, 0x63, 0x87, 0x139];
        check(
            ee::s14::f_003f68f0__generated,
            usb_kb_translate_key,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3f_68f0, 0);
                elf_image(c);
                c.pc = 0x003f_68f0;
                let data = DATA + 0x100;
                fill(c, &mut r, data, 0x24);
                fill(c, &mut r, USB_KB_TEST_KEY_PAIR, 1);
                let (led, mkey) =
                    [(0u32, 0u32), (1, 0x02), (2, 0x20), (3, 0x11)][(seed / 3 % 4) as usize];
                c.bus.write32(data + USB_KB_DATA_LED, led);
                c.bus.write32(data + USB_KB_DATA_MKEY, mkey);
                let index = seed / 12 % 10 * 5;
                c.bus.write16(
                    data + USB_KB_DATA_KEYS + 2 * index,
                    USAGES[(seed / 12 % 10) as usize],
                );
                c.g[A0][0] = sx(data);
                c.g[A1][0] = (seed % 3) as u64;
                c.g[A2][0] = index as u64;
            },
        );
    }

    /// `UsbKb_TranslateKey`'s pair with the game's gp.
    const USB_KB_TEST_KEY_PAIR: u32 = crate::difftest::GAME_GP + USB_KB_KEY_PAIR_GP;
}
