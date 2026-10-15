//! OpenCode worker module (docs/investigation/phase2-worker-guide.md).
//!
//! Small leaf accessors from the front-end async manager, the section manager
//! and the SSX audio system / stream array. Each replacement is a literal,
//! typed transcription of the generated body (same loads/stores, same cycles,
//! same registers left behind, including scratch `v1`).

use crate::abi;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x001a_39c0, fe_async_manager_get_state, "FEAsyncManager_GetState"),
    (0x001a_39d8, fe_async_manager_set_state, "FEAsyncManager_SetState"),
    (0x0022_e288, section_man_push_free_node, "SectionMan_PushFreeNode"),
    (0x0028_b1b0, ssx_audio_system_is_in_game, "SSXAudioSystem_IsInGame"),
    (0x0028_b1e8, ssx_audio_system_get_be_library, "SSXAudioSystem_GetBELibrary"),
    (0x0029_17b8, ssx_audio_system_get_surface_bank, "SSXAudioSystem_GetSurfaceBank"),
    (
        0x0029_f088,
        ssx_audio_system_get_current_speech_category,
        "SSXAudioSystem_GetCurrentSpeechCategory",
    ),
    (
        0x002a_4550,
        ssx_audio_system_clear_pending_speech,
        "SSXAudioSystem_ClearPendingSpeech",
    ),
    (
        0x002a_b0d0,
        stream_array_set_volume_scale_ptr,
        "StreamArray_SetVolumeScalePtr",
    ),
    (0x002a_b188, stream_array_get_stream, "StreamArray_GetStream"),
];

/// Offset of the async manager's state word.
const FE_ASYNC_MANAGER_STATE: u32 = 0x114;

/// The `FEAsyncManager_GetState` / `_SetState` prologue scale: the index is
/// multiplied by this before being added to `this`.
const FE_ASYNC_MANAGER_STRIDE: u32 = 0x11c;

/// `u32 FEAsyncManager_GetState(FEAsyncManager* m, int index)` (0x001a39c0):
/// returns the state word for slot `index`. The original computes the element
/// address with a signed `mult` of `index` by 0x11c (leaving `v1` = LO).
pub fn fe_async_manager_get_state(c: &mut Cpu) {
    c.cycles += 5;
    let base = abi::arg(c, 0) as u64;
    let index = abi::arg(c, 1);
    c.g[V0][0] = FE_ASYNC_MANAGER_STRIDE as u64;
    let lo = abi::mult(c, index, FE_ASYNC_MANAGER_STRIDE);
    c.g[V1][0] = sx(lo);
    c.g[A0][0] = sx(lo.wrapping_add(base as u32));
    c.g[V0][0] = Addr(c.g[A0][0] as u32)
        .off(FE_ASYNC_MANAGER_STATE)
        .lw(c);
    abi::ret(c);
}

/// `void FEAsyncManager_SetState(FEAsyncManager* m, int index, u32 state)`
/// (0x001a39d8): stores `state` into slot `index`. Same stride as the getter;
/// `v0` is left holding 0x11c (the mult writes `v1`).
pub fn fe_async_manager_set_state(c: &mut Cpu) {
    c.cycles += 5;
    let base = abi::arg(c, 0) as u64;
    let index = abi::arg(c, 1);
    c.g[V0][0] = FE_ASYNC_MANAGER_STRIDE as u64;
    let lo = abi::mult(c, index, FE_ASYNC_MANAGER_STRIDE);
    c.g[V1][0] = sx(lo);
    c.g[A0][0] = sx(lo.wrapping_add(base as u32));
    Addr(c.g[A0][0] as u32)
        .off(FE_ASYNC_MANAGER_STATE)
        .sw(c, c.g[A2][0]);
    abi::ret(c);
}

/// Offset of the section manager's free-node list head.
const SECTION_MAN_FREE_LIST: u32 = 0x198;
/// Offset of a free node's `next` link.
const SECTION_FREE_NODE_NEXT: u32 = 0x4;

/// `void SectionMan_PushFreeNode(SectionMan* m, void* node)` (0x0022e288):
/// pushes `node` onto the manager's singly linked free list; `v0` is left
/// holding the previous head.
pub fn section_man_push_free_node(c: &mut Cpu) {
    c.cycles += 4;
    let man = Addr(abi::arg(c, 0));
    let node = abi::arg(c, 1);
    let head = man.off(SECTION_MAN_FREE_LIST).lw(c);
    c.g[V0][0] = head;
    Addr(node).off(SECTION_FREE_NODE_NEXT).sw(c, head);
    man.off(SECTION_MAN_FREE_LIST).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `bool SSXAudioSystem_IsInGame(void)` (0x0028b1b0): whether the game module
/// pointer (global at `gp-0x848`) has a non-zero app/module word at +0x84.
/// `v1` is left holding the game module pointer.
pub fn ssx_audio_system_is_in_game(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x84).lw(c);
    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
    abi::ret(c);
}

/// `BELibrary* SSXAudioSystem_GetBELibrary(void)` (0x0028b1e8): the library
/// pointer at game module +0x78. `v1` is left holding the game module pointer.
pub fn ssx_audio_system_get_be_library(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x78).lw(c);
    abi::ret(c);
}

/// `u32 SSXAudioSystem_GetSurfaceBank(int index)` (0x002917b8): 1 when
/// `index > 3` (signed 64-bit compare), else 0x10. `a0` is left as 1 and `v1`
/// holds the compare result.
pub fn ssx_audio_system_get_surface_bank(c: &mut Cpu) {
    c.cycles += 6;
    c.g[A0][0] = 1;
    let greater = 3i64 < c.g[A1][0] as i64;
    c.g[V1][0] = greater as u64;
    c.g[V0][0] = 0x10;
    if greater {
        c.g[V0][0] = c.g[A0][0];
    }
    abi::ret(c);
}

/// Offset of the current speech category word.
const SPEECH_CATEGORY: u32 = 0x5738;

/// `int SSXAudioSystem_GetCurrentSpeechCategory(this, bool clear)`
/// (0x0029f088): returns -1 when `clear` (`a1`) is non-zero, else the current
/// category at +0x5738.
pub fn ssx_audio_system_get_current_speech_category(c: &mut Cpu) {
    c.cycles += 2;
    if abi::arg64(c, 1) != 0 {
        c.cycles += 2;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = Addr(abi::arg(c, 0)).off(SPEECH_CATEGORY).lw(c);
    abi::ret(c);
}

/// Offsets of the pending-speech fields cleared by
/// `SSXAudioSystem_ClearPendingSpeech`.
const PENDING_SPEECH_FIELDS: [u32; 14] = [
    0x5788, 0x5740, 0x5744, 0x574c, 0x5750, 0x5754, 0x5758, 0x5760, 0x5768, 0x576c, 0x5770,
    0x5774, 0x577c, 0x5784,
];

/// `void SSXAudioSystem_ClearPendingSpeech(this)` (0x002a4550): zeroes the
/// fourteen pending-speech bookkeeping words.
pub fn ssx_audio_system_clear_pending_speech(c: &mut Cpu) {
    c.cycles += 15;
    let this = Addr(abi::arg(c, 0));
    for off in PENDING_SPEECH_FIELDS {
        this.off(off).sw(c, 0);
    }
    abi::ret(c);
}

/// Offset of the stream-array element table pointer.
const STREAM_ARRAY_TABLE: u32 = 0x8;
/// Offset of a stream's volume-scale pointer.
const STREAM_VOLUME_SCALE_PTR: u32 = 0x10;

/// `void StreamArray_SetVolumeScalePtr(StreamArray* a, int i, float* p)`
/// (0x002ab0d0): `streams[i]->volumeScalePtr = p` (no-op if slot `i` is null).
/// `a1` is left holding the stream pointer (or 0).
pub fn stream_array_set_volume_scale_ptr(c: &mut Cpu) {
    c.cycles += 6;
    let array = Addr(abi::arg(c, 0));
    c.g[V0][0] = array.off(STREAM_ARRAY_TABLE).lw(c);
    c.g[A1][0] = sx(abi::arg(c, 1) << 2);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A1][0] = Addr(c.g[A1][0] as u32).lw(c);
    if c.g[A1][0] != 0 {
        Addr(c.g[A1][0] as u32)
            .off(STREAM_VOLUME_SCALE_PTR)
            .sw(c, c.g[A2][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void* StreamArray_GetStream(StreamArray* a, int i)` (0x002ab188): returns
/// `streams[i]`. `v0` first holds the table pointer, then the element; `a1` is
/// left holding the element address.
pub fn stream_array_get_stream(c: &mut Cpu) {
    c.cycles += 5;
    let array = Addr(abi::arg(c, 0));
    c.g[V0][0] = array.off(STREAM_ARRAY_TABLE).lw(c);
    c.g[A1][0] = sx(abi::arg(c, 1) << 2);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
    abi::ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, check_lohi, fill, scramble, Rng, DATA, GAME_GP};

    #[test]
    fn fe_async_manager_get_state_matches() {
        check_lohi(
            ssx3_ee::s03::f_001a39c0__generated,
            fe_async_manager_get_state,
            300,
            &|c, seed| {
                let mut r = Rng(0x1234_5678 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (r.next_u32() % 16) as u64;
                fill(c, &mut r, DATA, 0x2000);
            },
        );
    }

    #[test]
    fn fe_async_manager_set_state_matches() {
        check_lohi(
            ssx3_ee::s03::f_001a39d8__generated,
            fe_async_manager_set_state,
            300,
            &|c, seed| {
                let mut r = Rng(0x2345_6789 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (r.next_u32() % 16) as u64;
                c.g[A2][0] = r.next_u32() as u64;
                fill(c, &mut r, DATA, 0x2000);
            },
        );
    }

    #[test]
    fn section_man_push_free_node_matches() {
        check(
            ssx3_ee::s05::f_0022e288__generated,
            section_man_push_free_node,
            300,
            &|c, seed| {
                let mut r = Rng(0x3456_789a ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                let head = if seed % 2 == 0 { 0 } else { (DATA + 0x200) as u64 };
                c.bus.write32(DATA + SECTION_MAN_FREE_LIST, head as u32);
                fill(c, &mut r, DATA + 0x100, 8);
            },
        );
    }

    #[test]
    fn ssx_audio_system_is_in_game_matches() {
        check(
            ssx3_ee::s07::f_0028b1b0__generated,
            ssx_audio_system_is_in_game,
            300,
            &|c, seed| {
                let mut r = Rng(0x4567_89ab ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP - 0x848, DATA);
                let v = match seed % 4 {
                    0 => 0,
                    1 => 1,
                    2 => 0x8000_0000,
                    _ => r.next_u32(),
                };
                c.bus.write32(DATA + 0x84, v);
            },
        );
    }

    #[test]
    fn ssx_audio_system_get_be_library_matches() {
        check(
            ssx3_ee::s07::f_0028b1e8__generated,
            ssx_audio_system_get_be_library,
            300,
            &|c, seed| {
                let mut r = Rng(0x5678_9abc ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP - 0x848, DATA);
                c.bus.write32(DATA + 0x78, if seed % 2 == 0 { 0 } else { r.next_u32() });
            },
        );
    }

    #[test]
    fn ssx_audio_system_get_surface_bank_matches() {
        check(
            ssx3_ee::s07::f_002917b8__generated,
            ssx_audio_system_get_surface_bank,
            300,
            &|c, seed| {
                let mut r = Rng(0x6789_abcd ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = match seed % 5 {
                    0 => 0,
                    1 => 3,
                    2 => 4,
                    3 => 0xffff_ffff,
                    _ => (r.next_u32() as u64) << 32 | r.next_u32() as u64,
                };
            },
        );
    }

    #[test]
    fn ssx_audio_system_get_current_speech_category_matches() {
        check(
            ssx3_ee::s08::f_0029f088__generated,
            ssx_audio_system_get_current_speech_category,
            300,
            &|c, seed| {
                let mut r = Rng(0x789a_bcde ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = if seed % 2 == 0 { 0 } else { r.next_u32() as u64 };
                c.bus.write32(DATA + SPEECH_CATEGORY, r.next_u32());
            },
        );
    }

    #[test]
    fn ssx_audio_system_clear_pending_speech_matches() {
        check(
            ssx3_ee::s08::f_002a4550__generated,
            ssx_audio_system_clear_pending_speech,
            300,
            &|c, seed| {
                let mut r = Rng(0x89ab_cdef ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for off in PENDING_SPEECH_FIELDS {
                    c.bus.write32(DATA + off, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn stream_array_set_volume_scale_ptr_matches() {
        check(
            ssx3_ee::s08::f_002ab0d0__generated,
            stream_array_set_volume_scale_ptr,
            300,
            &|c, seed| {
                let mut r = Rng(0x9abc_def0 ^ seed as u64);
                scramble(c, &mut r);
                let table = DATA + 0x100;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (r.next_u32() % 4) as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.bus.write32(DATA + STREAM_ARRAY_TABLE, table);
                fill(c, &mut r, table, 8);
                if seed % 2 == 0 {
                    for i in 0..4 {
                        c.bus.write32(table + i * 4, 0);
                    }
                } else {
                    for i in 0..4 {
                        c.bus.write32(table + i * 4, DATA + 0x300 + i * 0x20);
                    }
                }
                fill(c, &mut r, DATA + 0x300, 0x100);
            },
        );
    }

    #[test]
    fn stream_array_get_stream_matches() {
        check(
            ssx3_ee::s08::f_002ab188__generated,
            stream_array_get_stream,
            300,
            &|c, seed| {
                let mut r = Rng(0xabcd_ef01 ^ seed as u64);
                scramble(c, &mut r);
                let table = DATA + 0x100;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (r.next_u32() % 4) as u64;
                c.bus.write32(DATA + STREAM_ARRAY_TABLE, table);
                for i in 0..4 {
                    c.bus.write32(table + i * 4, if seed % 3 == 0 { 0 } else { r.next_u32() });
                }
            },
        );
    }
}
