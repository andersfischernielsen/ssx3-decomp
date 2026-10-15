//! OpenCode worker module (docs/investigation/phase2-worker-guide.md).
//!
//! Trivial leaf replacements. Each copies the generated body's logic, cycle
//! charging and observable register (and FPU condition) results exactly, with
//! a differential test as its gate.

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0023_d570, mc_manager_set_mode, "MCManager_SetMode"),
    (0x0024_2500, occlusion_man_ctor, "OcclusionMan_ctor"),
    (0x0024_2540, occlusion_man_add_occluder, "OcclusionMan_AddOccluder"),
    (0x0024_2570, occlusion_man_remove_occluder, "OcclusionMan_RemoveOccluder"),
    (0x0024_4240, pause_stack_set_base, "PauseStack_SetBase"),
    (0x0025_16d0, mem_init_block_header, "MEM_InitBlockHeader"),
    (0x0025_2960, mem_get_heap_block_overhead, "MEM_GetHeapBlockOverhead"),
    (0x0025_3390, vid_engine_player_init_open_params, "VidEnginePlayer_InitOpenParams"),
    (0x0025_3aa0, vid_engine_install_mem_hooks, "VidEngine_InstallMemHooks"),
    (0x0025_4350, vd_floor_log2, "VD_FloorLog2"),
    (0x0025_57e0, c_dsock_channel_base_ctor, "cDSockChannel_BaseCtor"),
    (0x0025_5830, c_dsock_channel_set_open, "cDSockChannel_SetOpen"),
    (0x0025_5840, c_dsock_channel_reset_state, "cDSockChannel_ResetState"),
    (0x0025_5a10, c_comm_system_get_version_tag, "cCommSystem_GetVersionTag"),
    (0x0025_5d38, c_comm_system_has_channel, "cCommSystem_HasChannel"),
    (0x0025_6228, dnasauth_init, "DNASAuth_Init"),
    (0x0025_8330, c_game_comm_update_max_latency, "cGameComm_UpdateMaxLatency"),
    (0x0025_9198, lobby_man_get_language, "LobbyMan_GetLanguage"),
    (0x0026_1058, buddy_man_clear_login, "BuddyMan_ClearLogin"),
    (0x0026_1460, buddy_man_is_connected, "BuddyMan_IsConnected"),
];

// ---------------------------------------------------------------------------
// Memory card manager.

/// `void MCManager_SetMode(MCManager *self, int mode)` (0x23d570): publish the
/// mode at +0xb8; when it is exactly 1, also store the mode at +0x108, 1.0f at
/// +0x110 and a global float (`gp-0x512c`) at +0x114.
pub fn mc_manager_set_mode(c: &mut Cpu) {
    c.cycles += 2;
    let this = Addr(abi::arg(c, 0));
    let mode = c.g[A1][0];
    this.off(0xb8).sw(c, mode);
    if mode != 0 {
        c.cycles += 3;
        c.g[V0][0] = 0x3c;
        if mode == 1 {
            c.cycles += 6;
            c.g[AT][0] = 0x3f80_0000;
            c.f[0] = c.g[AT][0] as u32; // mtc1 $at, $f0 (1.0f)
            c.f[1] = Addr((c.g[GP][0] as u32).wrapping_sub(0x512c)).read_u32(c);
            this.off(0x108).sw(c, c.g[V0][0]);
            this.off(0x110).write_fpu(c, FpuF32(c.f[0]));
            this.off(0x114).write_fpu(c, FpuF32(c.f[1]));
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Occlusion manager.

/// `OcclusionMan *OcclusionMan_ctor(OcclusionMan *self)` (0x242500): zero the
/// list head (+0), count (+4), visible count (+0x108) and occluder count
/// (+0x10c); returns `self`.
pub fn occlusion_man_ctor(c: &mut Cpu) {
    c.cycles += 6;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = c.g[A0][0];
    this.off(0x4).sw(c, 0);
    this.off(0x108).sw(c, 0);
    this.off(0x10c).sw(c, 0);
    this.sw(c, 0);
    abi::ret(c);
}

/// `void OcclusionMan_AddOccluder(OcclusionMan *man, Occluder *occ)`
/// (0x242540): push `occ` onto the doubly linked list at `man+0`, setting
/// `occ->next = 0` and `occ->prev = old head`, then increment the count.
pub fn occlusion_man_add_occluder(c: &mut Cpu) {
    c.cycles += 6;
    let man = Addr(abi::arg(c, 0));
    let occ = abi::arg(c, 1);
    let head = man.lw(c);
    c.g[V0][0] = head;
    Addr(occ).off(0xc0).sw(c, 0);
    Addr(occ).off(0xbc).sw(c, head);
    let head = man.lw(c);
    c.g[V0][0] = head;
    if head != 0 {
        Addr(head as u32).off(0xc0).sw(c, occ as u64);
    }
    c.cycles += 5;
    let count = man.off(0x4).lw(c);
    c.g[V0][0] = count;
    man.sw(c, occ as u64);
    c.g[V0][0] = sx((count as u32).wrapping_add(1));
    man.off(0x4).sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `void OcclusionMan_RemoveOccluder(OcclusionMan *man, Occluder *occ)`
/// (0x242570): unlink `occ` from the list (fixing neighbours and the head)
/// and decrement the count.
pub fn occlusion_man_remove_occluder(c: &mut Cpu) {
    c.cycles += 3;
    let man = Addr(abi::arg(c, 0));
    let occ = abi::arg(c, 1);
    let mut v1 = Addr(occ).off(0xc0).lw(c);
    if v1 == 0 {
        v1 = Addr(occ).off(0xbc).lw(c);
    } else {
        c.cycles += 3;
        let prev = Addr(occ).off(0xbc).lw(c);
        Addr(v1 as u32).off(0xbc).sw(c, prev);
        v1 = Addr(occ).off(0xbc).lw(c);
    }
    c.g[V1][0] = v1;
    c.cycles += 2;
    let mut v0;
    if v1 == 0 {
        v0 = man.lw(c);
    } else {
        c.cycles += 3;
        let next = Addr(occ).off(0xc0).lw(c);
        Addr(v1 as u32).off(0xc0).sw(c, next);
        v0 = man.lw(c);
    }
    c.cycles += 2;
    if c.g[A1][0] != v0 {
        v0 = man.off(0x4).lw(c);
    } else {
        c.cycles += 3;
        let prev = Addr(occ).off(0xbc).lw(c);
        man.sw(c, prev);
        v0 = man.off(0x4).lw(c);
    }
    c.cycles += 3;
    c.g[V0][0] = sx((v0 as u32).wrapping_sub(1));
    man.off(0x4).sw(c, c.g[V0][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Pause stack.

/// `void PauseStack_SetBase(int index)` (0x244240): look up a pointer in the
/// table at 0x4428f0 indexed by `index`, cache it at `gp-0x6a0` and store
/// `index` in the record reached through the table at `(gp-0x69c)`.
pub fn pause_stack_set_base(c: &mut Cpu) {
    c.cycles += 13;
    let index = abi::arg(c, 0);
    c.g[V1][0] = sx(index << 2);
    c.g[V0][0] = sx(0x44_0000u32.wrapping_add(0x28f0));
    c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x69c)).lw(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A1][0] = 0x53_0000;
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
    c.g[A1][0] = sx(0x53_0000u32.wrapping_add(0x66e8));
    c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
    Addr((c.g[GP][0] as u32).wrapping_sub(0x6a0)).sw(c, c.g[V0][0]);
    Addr(c.g[A2][0] as u32).sw(c, c.g[A0][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Memory heap.

/// `void MEM_InitBlockHeader(MEMBlock *block, uint16 magic, uint32 prev,
/// uint32 next, uint32 size)` (0x2516d0): fill the block header at `block`.
pub fn mem_init_block_header(c: &mut Cpu) {
    c.cycles += 8;
    let block = Addr(abi::arg(c, 0));
    c.g[V0][0] = 0x424d;
    block.off(0xc).sw(c, c.g[T1][0]);
    block.sh(c, c.g[V0][0]);
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
    block.off(0x2).sh(c, c.g[T0][0]);
    block.off(0x8).sw(c, c.g[T2][0]);
    block.off(0x4).sw(c, c.g[A2][0]);
    abi::ret(c);
}

/// `int MEM_GetHeapBlockOverhead(MEMHeap *heap, uint bucket)` (0x252960):
/// index the 64-entry table at 0x4d2438 and return the overhead at +0x30 of
/// the record.
pub fn mem_get_heap_block_overhead(c: &mut Cpu) {
    c.cycles += 8;
    c.g[A1][0] = c.g[A1][0] & 0x3f;
    c.g[V0][0] = 0x4d_0000;
    c.g[V0][0] = sx(0x4d_0000u32.wrapping_add(0x2438));
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x30).lw(c);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Video engine.

/// `void VidEnginePlayer_InitOpenParams(VidEnginePlayer *self)`
/// (0x253390): reset the player's open parameters.
pub fn vid_engine_player_init_open_params(c: &mut Cpu) {
    c.cycles += 13;
    let this = Addr(abi::arg(c, 0));
    c.g[AT][0] = 0x3f80_0000;
    c.f[0] = c.g[AT][0] as u32; // mtc1 $at, $f0 (1.0f)
    c.g[V0][0] = 0x1f;
    this.off(0x18).sw(c, 0);
    this.off(0x20).write_fpu(c, FpuF32(c.f[0]));
    this.off(0x1c).sw(c, c.g[V0][0]);
    this.sw(c, 0);
    this.off(0x4).sw(c, 0);
    this.off(0xc).sw(c, 0);
    this.off(0x8).sw(c, 0);
    this.off(0x10).sw(c, 0);
    this.off(0x14).sw(c, 0);
    abi::ret(c);
}

/// `void VidEngine_InstallMemHooks(void)` (0x253aa0): write the two heap
/// hooks into the global hook record at 0x509430.
pub fn vid_engine_install_mem_hooks(c: &mut Cpu) {
    c.cycles += 11;
    c.g[V0][0] = 0x25_0000;
    c.g[A2][0] = 0x51_0000;
    c.g[V1][0] = 0x25_0000;
    c.g[V0][0] = sx(0x25_0000u32.wrapping_add(0x3ad0));
    c.g[A1][0] = sx(0x51_0000u32.wrapping_sub(0x6bd0));
    Addr(c.g[A1][0] as u32).sw(c, c.g[V0][0]);
    c.g[V1][0] = sx(0x25_0000u32.wrapping_add(0x3af8));
    c.g[A0][0] = 0x400;
    Addr(c.g[A1][0] as u32).off(0x8).sw(c, c.g[A0][0]);
    Addr(c.g[A1][0] as u32).off(0x4).sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `int VD_FloorLog2(int x)` (0x254350): the classic float exponent trick —
/// bit-reinterpret `x`, convert it, extract the exponent and unbias it.
pub fn vd_floor_log2(c: &mut Cpu) {
    c.cycles += 6;
    // mtc1 $a0, $f0 then cvt.s.w $f0, $f0 (raw bits reinterpreted as an int).
    let conv = FpuF32::from_i32(c.g[A0][0] as u32 as i32);
    c.f[0] = conv.0;
    let exp = (conv.0 as i32) >> 23;
    c.g[V0][0] = sx((exp as u32).wrapping_sub(0x7f));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Networking (d-elf comm system).

/// `cDSockChannel *cDSockChannel_BaseCtor(cDSockChannel *self)` (0x2557e0):
/// clear the vtable-derived fields and install the base vtable at +0x8.
pub fn c_dsock_channel_base_ctor(c: &mut Cpu) {
    c.cycles += 7;
    let this = Addr(abi::arg(c, 0));
    c.g[V1][0] = 0x48_0000;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = sx(0x48_0000u32.wrapping_add(0x1320));
    this.sw(c, 0);
    this.off(0x8).sw(c, c.g[V1][0]);
    this.off(0x4).sw(c, 0);
    abi::ret(c);
}

/// `void cDSockChannel_SetOpen(cDSockChannel *self, int open)` (0x255830):
/// store `open` (only 1 is ever passed) at the state field and clear the
/// error field.
pub fn c_dsock_channel_set_open(c: &mut Cpu) {
    c.cycles += 4;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = 0x1;
    this.off(0x4).sw(c, 0);
    this.sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `void cDSockChannel_ResetState(cDSockChannel *self)` (0x255840): clear the
/// connection fields when the channel is open.
pub fn c_dsock_channel_reset_state(c: &mut Cpu) {
    c.cycles += 3;
    let this = Addr(abi::arg(c, 0));
    let state = this.lw(c);
    c.g[V0][0] = state;
    if state != 0 {
        c.cycles += 2;
        this.off(0x4).sw(c, 0);
        this.sw(c, 0);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `const char *cCommSystem_GetVersionTag(void)` (0x255a10): returns the
/// version-tag string at 0x481370.
pub fn c_comm_system_get_version_tag(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = sx(0x48_0000u32.wrapping_add(0x1370));
    abi::ret(c);
}

/// `bool cCommSystem_HasChannel(cCommSystem *self, int index)` (0x255d38):
/// whether `self->channels[index]` is non-null.
pub fn c_comm_system_has_channel(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A1][0] = sx(abi::arg(c, 1) << 2);
    c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
    abi::ret(c);
}

/// `void DNASAuth_Init(DNASAuth *self)` (0x256228): clear the auth state and
/// set the handle fields to -1.
pub fn dnasauth_init(c: &mut Cpu) {
    c.cycles += 9;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    this.sw(c, 0);
    this.off(0x18).sw(c, c.g[V0][0]);
    this.off(0x4).sw(c, 0);
    this.off(0x8).sw(c, 0);
    this.off(0xc).sw(c, 0);
    this.off(0x10).sw(c, 0);
    this.off(0x14).sw(c, 0);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Game comm.

/// `void cGameComm_UpdateMaxLatency(cGameComm *self)` (0x258330): keep the
/// running maximum of the two latency samples, clamped to the world's target
/// latency.
pub fn c_game_comm_update_max_latency(c: &mut Cpu) {
    c.cycles += 10;
    let this = Addr(abi::arg(c, 0));
    let a = this.off(0x114).read_fpu(c);
    let b = this.off(0x118).read_fpu(c);
    c.f[0] = a.0;
    c.f[1] = b.0;
    let world = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[V0][0] = world;
    let max = b.max(a);
    c.f[1] = max.0;
    this.off(0x11c).write_fpu(c, max);
    let limit = Addr(world as u32).off(0x14).read_fpu(c);
    c.f[0] = limit.0;
    if abi::c_lt_s(c, max, limit) {
        this.off(0x11c).write_fpu(c, limit);
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Lobby.

/// `int LobbyMan_GetLanguage(void)` (0x259198): returns the current language
/// from the globals at 0x4a2f18.
pub fn lobby_man_get_language(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = sx(0x4a_0000u32.wrapping_add(0x2f18));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Buddy list.

/// `void BuddyMan_ClearLogin(BuddyMan *self)` (0x261058): clear the login
/// state (a byte at +0, the handle at +0x40 to -1, a byte at +0x44 and the
/// flags byte at +0x64).
pub fn buddy_man_clear_login(c: &mut Cpu) {
    c.cycles += 6;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    this.off(0x64).sb(c, 0);
    this.off(0x40).sw(c, c.g[V0][0]);
    this.sb(c, 0);
    this.off(0x44).sb(c, 0);
    abi::ret(c);
}

/// `bool BuddyMan_IsConnected(BuddyMan *self)` (0x261460): true when the
/// session handle (+0xc8) and connection (+0xe0) exist and the state (+0xd8)
/// is not "connecting".
pub fn buddy_man_is_connected(c: &mut Cpu) {
    c.cycles += 3;
    let this = Addr(abi::arg(c, 0));
    let mut result = 0u64;
    let session = this.off(0xc8).lw(c);
    c.g[V0][0] = session;
    if session != 0 {
        c.cycles += 4;
        let state = this.off(0xd8).lw(c) ^ 1;
        c.g[V0][0] = state;
        if state != 0 {
            c.cycles += 3;
            let connected = this.off(0xe0).lw(c);
            c.g[V0][0] = connected;
            result = (connected != 0) as u64;
        }
    }
    c.cycles += 2;
    c.g[V1][0] = result;
    c.g[V0][0] = result;
    abi::ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

    #[test]
    fn mc_manager_set_mode_matches() {
        check(ssx3_ee::s06::f_0023d570, mc_manager_set_mode, 300, &|c, seed| {
            let mut r = Rng(0x23d5_7000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = match seed % 5 {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => ((r.next_u32() as u64) << 32) | (r.next_u32() as u64 | 2),
                _ => 0x1_0000_0000,
            };
            c.bus.write32(GAME_GP - 0x512c, r.edge_f32());
            c.bus.write32(DATA + 0x108, r.next_u32());
            c.bus.write32(DATA + 0x110, r.next_u32());
            c.bus.write32(DATA + 0x114, r.next_u32());
        });
    }

    #[test]
    fn occlusion_man_ctor_matches() {
        check(ssx3_ee::s06::f_00242500, occlusion_man_ctor, 100, &|c, seed| {
            let mut r = Rng(0x2425_0000 ^ seed as u64);
            scramble(c, &mut r);
            let base = DATA + (seed % 4) * 4;
            c.g[A0][0] = base as u64;
            for off in [0u32, 4, 0x108, 0x10c] {
                c.bus.write32(base + off, r.next_u32());
            }
        });
    }

    #[test]
    fn occlusion_man_add_occluder_matches() {
        check(ssx3_ee::s06::f_00242540, occlusion_man_add_occluder, 200, &|c, seed| {
            let mut r = Rng(0x2425_4000 ^ seed as u64);
            scramble(c, &mut r);
            let occ = DATA + 0x200;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = occ as u64;
            let head = if seed % 2 == 0 { 0 } else { DATA + 0x300 };
            c.bus.write32(DATA, head);
            c.bus.write32(DATA + 4, r.next_u32());
            c.bus.write32(occ + 0xbc, r.next_u32());
            c.bus.write32(occ + 0xc0, r.next_u32());
            if head != 0 {
                c.bus.write32(head + 0xc0, r.next_u32());
            }
        });
    }

    #[test]
    fn occlusion_man_remove_occluder_matches() {
        check(ssx3_ee::s06::f_00242570, occlusion_man_remove_occluder, 300, &|c, seed| {
            let mut r = Rng(0x2425_7000 ^ seed as u64);
            scramble(c, &mut r);
            let n = DATA + 0x100;
            let o1 = DATA + 0x200;
            let o2 = DATA + 0x300;
            // Cover next == 0, prev == 0, occ == head and every combination.
            let (next, prev, head) = match seed % 6 {
                0 => (0u32, 0u32, n),
                1 => (0, 0, o1),
                2 => (o1, 0, n),
                3 => (o1, o2, n),
                4 => (o1, o2, o1),
                _ => (o1, 0, o1),
            };
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = n as u64;
            c.bus.write32(DATA, head);
            c.bus.write32(DATA + 4, r.next_u32());
            c.bus.write32(n + 0xbc, prev);
            c.bus.write32(n + 0xc0, next);
            for node in [o1, o2] {
                c.bus.write32(node + 0xbc, r.next_u32());
                c.bus.write32(node + 0xc0, r.next_u32());
            }
        });
    }

    #[test]
    fn pause_stack_set_base_matches() {
        check(ssx3_ee::s06::f_00244240, pause_stack_set_base, 200, &|c, seed| {
            let mut r = Rng(0x2442_4000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let index = seed % 8;
            // Exercise the 32-bit `<< 2` with garbage in the upper half.
            c.g[A0][0] = if seed % 2 == 0 {
                index as u64
            } else {
                0xffff_ffff_0000_0000u64 | index as u64
            };
            let entry = (index << 2).wrapping_add(0x4428f0);
            c.bus.write32(entry, r.next_u32());
            let ptr = 0x100u32;
            c.bus.write32(GAME_GP - 0x69c, ptr);
            c.bus.write32((ptr << 2).wrapping_add(0x5366e8), r.next_u32());
        });
    }

    #[test]
    fn mem_init_block_header_matches() {
        check(ssx3_ee::s06::f_002516d0, mem_init_block_header, 100, &|c, seed| {
            let mut r = Rng(0x2516_d000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.g[T0][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.g[T1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.g[T2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            for off in [0u32, 2, 4, 8, 0xc] {
                c.bus.write32(DATA + off, r.next_u32());
            }
        });
    }

    #[test]
    fn mem_get_heap_block_overhead_matches() {
        check(ssx3_ee::s06::f_00252960, mem_get_heap_block_overhead, 200, &|c, seed| {
            let mut r = Rng(0x2529_6000 ^ seed as u64);
            scramble(c, &mut r);
            let idx = seed % 0x40;
            c.g[A1][0] = if seed % 2 == 0 {
                idx as u64
            } else {
                0xffff_ffff_0000_0000u64 | idx as u64
            };
            let node = DATA + 0x40;
            c.bus.write32(0x4d2438 + (idx << 2), node);
            c.bus.write32(node + 0x30, r.next_u32());
        });
    }

    #[test]
    fn vid_engine_player_init_open_params_matches() {
        check(
            ssx3_ee::s06::f_00253390,
            vid_engine_player_init_open_params,
            100,
            &|c, seed| {
                let mut r = Rng(0x2533_9000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                for off in [0u32, 4, 8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20] {
                    c.bus.write32(DATA + off, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn vid_engine_install_mem_hooks_matches() {
        check(ssx3_ee::s06::f_00253aa0, vid_engine_install_mem_hooks, 50, &|c, seed| {
            let mut r = Rng(0x253a_a000 ^ seed as u64);
            scramble(c, &mut r);
            for addr in [0x509430u32, 0x509434, 0x509438] {
                c.bus.write32(addr, r.next_u32());
            }
        });
    }

    #[test]
    fn vd_floor_log2_matches() {
        check(ssx3_ee::s06::f_00254350, vd_floor_log2, 300, &|c, seed| {
            let mut r = Rng(0x2543_5000 ^ seed as u64);
            scramble(c, &mut r);
            let bits = match seed % 3 {
                0 => r.edge_f32(),
                1 => r.next_u32(),
                _ => (seed as u32).wrapping_mul(0x9e37_79b9),
            };
            c.g[A0][0] = ((r.next_u32() as u64) << 32) | bits as u64;
        });
    }

    #[test]
    fn c_dsock_channel_base_ctor_matches() {
        check(ssx3_ee::s06::f_002557e0, c_dsock_channel_base_ctor, 100, &|c, seed| {
            let mut r = Rng(0x2557_e000 ^ seed as u64);
            scramble(c, &mut r);
            let base = DATA + (seed % 4) * 4;
            c.g[A0][0] = base as u64;
            for off in [0u32, 4, 8] {
                c.bus.write32(base + off, r.next_u32());
            }
        });
    }

    #[test]
    fn c_dsock_channel_set_open_matches() {
        check(ssx3_ee::s06::f_00255830, c_dsock_channel_set_open, 50, &|c, seed| {
            let mut r = Rng(0x2558_3000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, r.next_u32());
            c.bus.write32(DATA + 4, r.next_u32());
        });
    }

    #[test]
    fn c_dsock_channel_reset_state_matches() {
        check(ssx3_ee::s06::f_00255840, c_dsock_channel_reset_state, 200, &|c, seed| {
            let mut r = Rng(0x2558_4000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, if seed % 2 == 0 { 0 } else { r.next_u32() });
            c.bus.write32(DATA + 4, r.next_u32());
        });
    }

    #[test]
    fn c_comm_system_get_version_tag_matches() {
        check(ssx3_ee::s06::f_00255a10, c_comm_system_get_version_tag, 30, &|c, seed| {
            let mut r = Rng(0x255a_1000 ^ seed as u64);
            scramble(c, &mut r);
        });
    }

    #[test]
    fn c_comm_system_has_channel_matches() {
        check(ssx3_ee::s06::f_00255d38, c_comm_system_has_channel, 200, &|c, seed| {
            let mut r = Rng(0x255d_3800 ^ seed as u64);
            scramble(c, &mut r);
            let idx = (seed % 4) as u32;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = if seed % 2 == 0 {
                idx as u64
            } else {
                0x1_0000_0000u64 | idx as u64
            };
            c.bus.write32(
                DATA + idx * 4,
                match seed % 5 {
                    0 => 0,
                    1 => 1,
                    2 => 0x8000_0000,
                    3 => 0x7fff_ffff,
                    _ => r.next_u32(),
                },
            );
        });
    }

    #[test]
    fn dnasauth_init_matches() {
        check(ssx3_ee::s06::f_00256228, dnasauth_init, 100, &|c, seed| {
            let mut r = Rng(0x2562_2800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for off in [0u32, 4, 8, 0xc, 0x10, 0x14, 0x18] {
                c.bus.write32(DATA + off, r.next_u32());
            }
        });
    }

    #[test]
    fn c_game_comm_update_max_latency_matches() {
        check(ssx3_ee::s06::f_00258330, c_game_comm_update_max_latency, 300, &|c, seed| {
            let mut r = Rng(0x2583_3000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            let world = DATA + 0x400;
            c.bus.write32(GAME_GP - 0x848, world);
            // Seeds 0 and 1 force the FPU compare both ways; the rest fuzz.
            let (a, b, limit) = match seed % 3 {
                0 => (1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits()),
                1 => (5.0f32.to_bits(), 4.0f32.to_bits(), 1.0f32.to_bits()),
                _ => (r.edge_f32(), r.edge_f32(), r.edge_f32()),
            };
            c.bus.write32(DATA + 0x114, a);
            c.bus.write32(DATA + 0x118, b);
            c.bus.write32(world + 0x14, limit);
            c.bus.write32(DATA + 0x11c, r.next_u32());
        });
    }

    #[test]
    fn lobby_man_get_language_matches() {
        check(ssx3_ee::s06::f_00259198, lobby_man_get_language, 30, &|c, seed| {
            let mut r = Rng(0x2591_9800 ^ seed as u64);
            scramble(c, &mut r);
        });
    }

    #[test]
    fn buddy_man_clear_login_matches() {
        check(ssx3_ee::s06::f_00261058, buddy_man_clear_login, 100, &|c, seed| {
            let mut r = Rng(0x2610_5800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for off in [0u32, 0x40, 0x44, 0x64] {
                c.bus.write32(DATA + off, r.next_u32());
            }
        });
    }

    #[test]
    fn buddy_man_is_connected_matches() {
        check(ssx3_ee::s06::f_00261460, buddy_man_is_connected, 300, &|c, seed| {
            let mut r = Rng(0x2614_6000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            // Cover: no session, "connecting" state, and connected/disconnected.
            let (session, state, conn) = match seed % 6 {
                0 => (0, r.next_u32(), r.next_u32()),
                1 => (1, 1, r.next_u32()),
                2 => (1, 0, 0),
                3 => (1, 2, r.next_u32() | 1),
                4 => (1, 1, 0),
                _ => (r.next_u32() | 1, r.next_u32(), r.next_u32()),
            };
            c.bus.write32(DATA + 0xc8, session);
            c.bus.write32(DATA + 0xd8, state);
            c.bus.write32(DATA + 0xe0, conn);
        });
    }
}
