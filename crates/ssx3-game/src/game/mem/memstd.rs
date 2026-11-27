//! `src/mem/memstd.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw as lw_3;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::p_8 as p_5;
pub(crate) use crate::game::dirtysock::tagsunk::arg;
pub(crate) use crate::game::dirtysock::tagsunk::deleting_dtor;
pub(crate) use crate::game::dirtysock::tagsunk::vslot;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_2;
pub(crate) use crate::game::fe::feasyncfile::gpa;
pub(crate) use crate::game::fe::feasyncfile::gpn;
pub(crate) use crate::game::fe::feasyncfile::lh_3 as lh_2;
pub(crate) use crate::game::fe::feasyncfile::lw_9 as lw_4;
pub(crate) use crate::game::fe::feasyncfile::p as p_4;
pub(crate) use crate::game::fe::feasyncfile::sw_7 as sw_2;
pub(crate) use crate::game::fe::fepopupmisc::lh;
pub(crate) use crate::game::fe::fepopupmisc::lw_2 as lw;
pub(crate) use crate::game::fe::fepopupmisc::p_2;
pub(crate) use crate::game::fe::fepopupmisc::sw;
pub(crate) use crate::game::fe::fereal::close;
pub(crate) use crate::game::fe::fereal::gp;
pub(crate) use crate::game::fe::fereal::open;
pub(crate) use crate::game::intersect::riderspheretree::p_2 as p;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at_2;
pub(crate) use crate::game::main::gamestate::p_2 as p_3;
pub(crate) use crate::game::movie::movieplayer::lw_at;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0025_1f68, mem_debug_printf, "MEM_DebugPrintf"),
    (0x0025_2248, mem_is_valid_block_link, "MEM_IsValidBlockLink"),
    (0x0025_2660, mem_get_block_tag, "MEM_GetBlockTag"),
    (
        0x0025_26b8,
        mem_resize_block_locked,
        "MEM_ResizeBlockLocked",
    ),
    (0x0025_2fa0, mem_alloc_named128, "MEM_AllocNamed128"),
    (
        0x0025_3390,
        vid_engine_player_init_open_params,
        "VidEnginePlayer_InitOpenParams",
    ),
    (0x0025_33c8, vid_engine_player_ctor, "VidEnginePlayer_ctor"),
    (0x0025_3418, vid_engine_player_dtor, "VidEnginePlayer_dtor"),
    (
        0x0025_35f8,
        vid_engine_player_close,
        "VidEnginePlayer_Close",
    ),
    (
        0x0025_36c8,
        vid_engine_player_start_playback,
        "VidEnginePlayer_StartPlayback",
    ),
    (0x0025_3860, vid_engine_player_play, "VidEnginePlayer_Play"),
    (
        0x0025_3938,
        vid_engine_player_update,
        "VidEnginePlayer_Update",
    ),
    (
        0x0025_39e0,
        vid_engine_player_pause,
        "VidEnginePlayer_Pause",
    ),
    (
        0x0025_3a40,
        vid_engine_player_resume,
        "VidEnginePlayer_Resume",
    ),
    (
        0x0025_3aa0,
        vid_engine_install_mem_hooks,
        "VidEngine_InstallMemHooks",
    ),
    (0x0025_4350, vd_floor_log2, "VD_FloorLog2"),
    (0x0025_4368, vd_create_texture_sink, "VD_CreateTextureSink"),
    (
        0x0025_43f0,
        vd_texture_sink_set_frame_size,
        "VD_TextureSink_SetFrameSize",
    ),
    (0x0025_47b8, vd_texture_sink_dtor, "VD_TextureSink_dtor"),
    (0x0025_4dc0, vd_texture_sink_flush, "VD_TextureSink_Flush"),
    (0x0025_5638, vd_sink_base_dtor, "VD_SinkBase_dtor"),
    (
        0x0025_57e0,
        c_dsock_channel_base_ctor,
        "cDSockChannel_BaseCtor",
    ),
    (
        0x0025_5800,
        c_d_sock_channel_base_dtor,
        "cDSockChannel_BaseDtor",
    ),
    (
        0x0025_5830,
        c_dsock_channel_set_open,
        "cDSockChannel_SetOpen",
    ),
    (
        0x0025_5840,
        c_dsock_channel_reset_state,
        "cDSockChannel_ResetState",
    ),
    (0x0025_6698, c_dsock_channel_ctor, "cDSockChannel_ctor"),
    (0x0025_66e8, c_dsock_channel_dtor, "cDSockChannel_dtor"),
    (0x0025_6730, c_dsock_channel_recv, "cDSockChannel_Recv"),
    (0x0025_67e0, c_dsock_channel_send, "cDSockChannel_Send"),
    (0x0025_6860, c_dsock_channel_listen, "cDSockChannel_Listen"),
    (
        0x0025_68f8,
        c_dsock_channel_connect,
        "cDSockChannel_Connect",
    ),
    (
        0x0025_6998,
        c_dsock_channel_disconnect,
        "cDSockChannel_Disconnect",
    ),
    (0x0025_6a00, c_dsock_channel_update, "cDSockChannel_Update"),
    (
        0x0025_6a58,
        c_dsock_channel_update_connecting,
        "cDSockChannel_UpdateConnecting",
    ),
    (
        0x0025_6aa8,
        c_dsock_channel_update_connected,
        "cDSockChannel_UpdateConnected",
    ),
    (0x0025_6ba8, dsock_channel_v05, "cDSockChannel_v05"),
    (
        0x0026_7dc8,
        c_dsock_channel_base_update,
        "cDSockChannel_BaseUpdate",
    ),
    (
        0x0026_7dd0,
        c_dsock_channel_base_is_connected,
        "cDSockChannel_BaseIsConnected",
    ),
    (
        0x0026_7de8,
        c_dsock_channel_base_v06,
        "cDSockChannel_Base_v06",
    ),
    (
        0x0026_7df0,
        c_dsock_channel_base_recv,
        "cDSockChannel_BaseRecv",
    ),
    (
        0x0026_7df8,
        c_dsock_channel_base_send,
        "cDSockChannel_BaseSend",
    ),
    (0x0026_7e10, c_dsock_channel_v06, "cDSockChannel_v06"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::bx::bxstringctor::callee::VID_ENGINE_PLAYER_START_PLAYBACK;

    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::BUILTIN_DELETE;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::MEM_ALLOC_NAMED_ALIGNED;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::VID_ENGINE_PLAYER_CLOSE;

    pub(crate) use crate::game::main::gamestate::callee::VD_TEXTURE_SINK_CTOR;

    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VID_ENGINE_INSTALL_MEM_HOOKS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VID_ENGINE_PLAYER_INIT_OPEN_PARAMS;

    pub(crate) const C_DSOCK_CHANNEL_UPDATE_CONNECTING: Callee = Callee {
        addr: 0x0025_6a58,
        f: ee::s06::cDSockChannel_UpdateConnecting,
    };

    pub(crate) const C_DSOCK_CHANNEL_UPDATE_CONNECTED: Callee = Callee {
        addr: 0x0025_6aa8,
        f: ee::s06::cDSockChannel_UpdateConnected,
    };

    pub(crate) const C_NET_GAME_LINK_STATUS: Callee = Callee {
        addr: 0x003e_6958,
        f: ee::s14::NetGameLinkStatus,
    };

    pub(crate) const C_EA_MEM_COPY: Callee = Callee {
        addr: 0x003e_6574,
        f: ee::s14::EA_MemCopy,
    };

    pub(crate) const C_NET_GAME_LINK_RECV: Callee = Callee {
        addr: 0x003e_6b10,
        f: ee::s14::NetGameLinkRecv,
    };

    pub(crate) const C_NET_GAME_LINK_SEND: Callee = Callee {
        addr: 0x003e_6c80,
        f: ee::s14::NetGameLinkSend,
    };

    pub(crate) const C_NET_GAME_UTIL_CREATE: Callee = Callee {
        addr: 0x003e_6e08,
        f: ee::s14::NetGameUtilCreate,
    };

    pub(crate) const C_DSOCK_CHANNEL_SET_OPEN: Callee = Callee {
        addr: 0x0025_5830,
        f: ee::s06::cDSockChannel_SetOpen,
    };

    pub(crate) const C_NET_GAME_UTIL_CONNECT: Callee = Callee {
        addr: 0x003e_6f40,
        f: ee::s14::NetGameUtilConnect,
    };

    pub(crate) const C_DSOCK_CHANNEL_BASE_CTOR: Callee = Callee {
        addr: 0x0025_57e0,
        f: ee::s06::cDSockChannel_BaseCtor,
    };

    pub(crate) const C_DSOCK_CHANNEL_BASE_DTOR: Callee = Callee {
        addr: 0x0025_5800,
        f: ee::s06::cDSockChannel_BaseDtor,
    };

    pub(crate) const C_DSOCK_CHANNEL_RESET_STATE: Callee = Callee {
        addr: 0x0025_5840,
        f: ee::s06::cDSockChannel_ResetState,
    };

    pub(crate) const C_DSOCK_CHANNEL_DISCONNECT: Callee = Callee {
        addr: 0x0025_6998,
        f: ee::s06::cDSockChannel_Disconnect,
    };

    pub(crate) const C_NET_GAME_LINK_DESTROY: Callee = Callee {
        addr: 0x003e_67f8,
        f: ee::s14::NetGameLinkDestroy,
    };

    pub(crate) const C_NET_GAME_UTIL_DESTROY: Callee = Callee {
        addr: 0x003e_6e70,
        f: ee::s14::NetGameUtilDestroy,
    };

    pub(crate) const C_NET_GAME_UTIL_COMPLETE: Callee = Callee {
        addr: 0x003e_6fc8,
        f: ee::s14::NetGameUtilComplete,
    };

    pub(crate) const C_NET_GAME_LINK_CREATE: Callee = Callee {
        addr: 0x003e_66f0,
        f: ee::s14::NetGameLinkCreate,
    };

    pub(crate) const C_BUILTIN_DELETE: Callee = Callee {
        addr: 0x0031_7e50,
        f: ee::s10::__builtin_delete,
    };
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

/// `void VD_TextureSink_SetFrameSize(VD_TextureSink* s, float a, float b,
/// float c)` (0x2543f0): the three frame dimensions at +0x40..0x48.
pub fn vd_texture_sink_set_frame_size(c: &mut Cpu) {
    c.cycles += 4;
    let selfp = abi::arg(c, 0);
    c.swc1(14, selfp.wrapping_add(0x48));
    c.swc1(12, selfp.wrapping_add(0x40));
    c.swc1(13, selfp.wrapping_add(0x44));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Memory manager.

/// The tag of an allocated block whose size word has bit 0x800 set, at
/// `+4` of the tag record the block's `-0xc` word points at.
pub(crate) const BLOCK_HEADER_SIZE: u32 = 0xe;

pub(crate) const BLOCK_TAG_POINTER: u32 = 0xc;

pub(crate) const BLOCK_TAGGED: u32 = 0x800;

/// `int MEM_GetBlockTag(void* block)` (0x252660): the block's tag string, or
/// 0 when the block is not tagged.
pub fn mem_get_block_tag(c: &mut Cpu) {
    let p = arg(c, 0);
    c.cycles += 4;
    c.g[V0][0] = Addr(p.wrapping_sub(BLOCK_HEADER_SIZE)).lhu(c);
    c.g[V0][0] &= BLOCK_TAGGED as u64;
    let tagged = c.g[V0][0] != 0;
    c.g[V1][0] = 0;
    if tagged {
        c.cycles += 4;
        c.g[V0][0] = Addr(p.wrapping_sub(BLOCK_TAG_POINTER)).lw(c);
        c.g[V0][0] = sx(p.wrapping_add(c.g[V0][0] as u32));
        let rec = c.g[V0][0] as u32;
        c.lwl(V1, rec.wrapping_add(7));
        c.lwr(V1, rec.wrapping_add(4));
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[V1][0];
    abi::ret(c);
}

/// `void* MEM_AllocNamed128(const char* name, size_t size, uint32_t flags)`
/// (0x252fa0): named allocation with 128-byte alignment
/// (`MEM_AllocNamedAligned(name, size, 0x80, 0, flags, 1)`).
pub fn mem_alloc_named128(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[T0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = 0x80;
        c.g[A3][0] = 0;
        c.g[T1][0] = 1;
        callee::MEM_ALLOC_NAMED_ALIGNED.call(c, 0x0025_2fbc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Video, lobby and setup managers.

/// `VidEnginePlayer_Play(this)` 0x253860: start playback when a video file is
/// queued (+0x38); reports whether it started.
pub fn vid_engine_player_play(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        let file = lw_at(c, this as u64, 0x38);
        c.g[V0][0] = file;
        let none = file == 0;
        c.g[V0][0] = 0;
        if !none {
            c.cycles += 3;
            callee::VID_ENGINE_PLAYER_START_PLAYBACK.call(c, 0x0025_387c)?;
            c.g[V0][0] = 1;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void VD_SinkBase_dtor(VD_SinkBase *self, int flags)` (0x255638):
/// republishes `D_004802B0` at `self+4`, deletes on bit 0.
pub fn vd_sink_base_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0048_02b0, 4, 0x0025_565c))
}

/// `void cDSockChannel_BaseDtor(cDSockChannel *self, int flags)` (0x255800):
/// republishes `D_00481320` at `self+8`, deletes on bit 0.
pub fn c_d_sock_channel_base_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0048_1320, 8, 0x0025_5824))
}

/// `void VidEnginePlayer_dtor(VidEnginePlayer* self, int flags)` (0x253418):
/// close the player, then `operator delete` it when `flags & 1`.
pub fn vid_engine_player_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        callee::VID_ENGINE_PLAYER_CLOSE.call(c, 0x0025_3434)?;
        c.g[S0][0] = c.g[S0][0] & 1;
        let keep = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, callee::BUILTIN_DELETE, 0x0025_3448)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Video / buddies / login.

/// `VidEnginePlayer* VidEnginePlayer_ctor(VidEnginePlayer* self)`
/// (0x2533c8): install the memory hooks, init the open parameters and clear
/// the playback fields.
pub fn vid_engine_player_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        callee::VID_ENGINE_INSTALL_MEM_HOOKS.call(c, 0x0025_33dc)?;
        c.g[A0][0] = sx(this.wrapping_add(0x14));
        callee::VID_ENGINE_PLAYER_INIT_OPEN_PARAMS.call(c, 0x0025_33e4)?;
        c.lwc1(0, p(this, 0x34));
        c.g[V0][0] = c.g[S0][0];
        Addr(this).off(0x38).sw(c, 0);
        c.swc1(0, p(this, 0x10));
        Addr(this).off(0x3c).sw(c, 0);
        Addr(this).off(0xc).sw(c, 0);
        Addr(this).off(0x40).sw(c, 0);
        Addr(this).off(8).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `sd reg, off(sp)`: a register the original spills through the frame.
#[inline(always)]
pub(crate) fn save_reg(c: &mut Cpu, frame: Frame, reg: usize, off: u32) {
    let a = frame.at(c, off);
    let v = c.g[reg][0];
    c.sd(a, v);
}

// ---------------------------------------------------------------------------
// Debug printf and movie/audio helpers.

/// `void MEM_DebugPrintf(const char* fmt, ...)` (0x251f68): format the
/// varargs into a 0x200-byte stack buffer and hand it to the debug output.
pub fn mem_debug_printf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x290);
        save_reg(c, frame, A1, 0x258);
        save_reg(c, frame, A2, 0x260);
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0x200);
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x258));
        save_reg(c, frame, A3, 0x268);
        save_reg(c, frame, T0, 0x270);
        save_reg(c, frame, T1, 0x278);
        save_reg(c, frame, T2, 0x280);
        save_reg(c, frame, T3, 0x288);
        c.swc1(12, p_2(c.g[SP][0], 0x248));
        c.swc1(14, p_2(c.g[SP][0], 0x24c));
        c.swc1(16, p_2(c.g[SP][0], 0x250));
        c.swc1(18, p_2(c.g[SP][0], 0x254));
        call_ptr(c, 0x0041_86c8, 0x0025_1fac)?;
        c.g[A0][0] = 2;
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x003e_5d30, 0x0025_1fb8)?;
        frame.restore_ra(c, 0x200);
        frame.close(c);
        Ok(())
    })
}

/// `void VidEnginePlayer_Resume(VidEnginePlayer* self)` (0x253a40): resume
/// playback if the player has data, is enabled, is not paused and is stopped.
pub fn vid_engine_player_resume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = lw(c, p_2(c.g[S0][0], 0x38));
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = lw(c, p_2(c.g[S0][0], 0x3c));
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[V0][0] = lw(c, p_2(c.g[S0][0], 0));
                if c.g[V0][0] == 0 {
                    c.cycles += 3;
                    c.g[V0][0] = lw(c, p_2(c.g[S0][0], 8));
                    if c.g[V0][0] != 0 {
                        c.cycles += 4;
                        call_ptr(c, 0x003a_e958, 0x0025_3a88)?;
                        sw(c, p_2(c.g[S0][0], 8), 0);
                    }
                }
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void VD_TextureSink_Flush(VD_TextureSink* self)` (0x254dc0): spin until
/// the sink's device reports completion, then release the pending buffer.
pub fn vd_texture_sink_flush(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = lw(c, p_2(c.g[S0][0], 8));
        loop {
            c.cycles += 7;
            c.g[V1][0] = lw(c, p_2(c.g[A1][0], 0x10d8));
            c.g[A0][0] = lh(c, p_2(c.g[V1][0], 0x90));
            c.g[V0][0] = lw(c, p_2(c.g[V1][0], 0x94));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0025_4dec)?;
            if c.g[V0][0] != 0 {
                break;
            }
            c.g[A1][0] = lw(c, p_2(c.g[S0][0], 8));
            if event_due(c) {
                deliver_event_here(c, 0x0025_4dd8);
            }
        }
        c.cycles += 3;
        c.g[A1][0] = lw(c, p_2(c.g[S0][0], 0xc));
        if c.g[A1][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = lw(c, p_2(c.g[S0][0], 0));
            call_ptr(c, 0x003b_0680, 0x0025_4e08)?;
            sw(c, p_2(c.g[S0][0], 0xc), 0);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* MEM_ResizeBlockLocked(void* block, size_t size)` (0x2526b8):
/// resize a block under the allocator lock.
pub fn mem_resize_block_locked(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = sx(0x0053_8b00);
        frame.save(c, S1, 0x20);
        let block = c.g[A0][0];
        let size = c.g[A1][0];
        c.g[S2][0] = block;
        frame.save_ra(c, 0);
        c.g[S1][0] = size;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_5700, 0x0025_26e4)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0031_9bd8, 0x0025_26f0)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_5760, 0x0025_26fc)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void VidEnginePlayer_Pause(VidEnginePlayer* player)` (0x2539e0): pause
/// the player once its stream is ready and idle.
pub fn vid_engine_player_pause(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let player = c.g[A0][0];
        c.g[S0][0] = player;
        c.g[A0][0] = Addr(at(c, S0, 0x38)).lw(c);
        let mut done = c.g[A0][0] == 0;
        if !done {
            c.cycles += 3;
            c.g[V0][0] = Addr(at(c, S0, 0x3c)).lw(c);
            done = c.g[V0][0] == 0;
            if !done {
                c.cycles += 3;
                c.g[V0][0] = Addr(at(c, S0, 0x0)).lw(c);
                done = c.g[V0][0] != 0;
                if !done {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(at(c, S0, 0x8)).lw(c);
                    done = c.g[V0][0] != 0;
                    if !done {
                        c.cycles += 5;
                        call_ptr(c, 0x003a_e938, 0x0025_3a28)?;
                        c.g[V1][0] = 0x1;
                        Addr(at(c, S0, 0x8)).sw(c, c.g[V1][0]);
                    }
                }
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Small list / object helpers.

/// `void MEM_IsValidBlockLink(void* p)` (0x252248): true if `p` is a valid
/// allocator pointer whose block magic is `"BF"` or `"BS"`; otherwise record
/// the failure in `D_004A2E74` and return false.
pub fn mem_is_valid_block_link(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        open(c, 0x30);
        c.g[A1][0] = 0x18;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[A2][0] = 0u64;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x0025_226c);
        c.g[S1][0] = 0u64;
        call_ptr(c, 0x003e_6240, 0x0025_226c)?;
        let mut ok = 0u64;
        if c.g[V0][0] == 0 {
            // Not a pointer EA considers valid: flag the error.
            c.cycles += 1;
            c.cycles += 1;
            c.sw(gp(c, -0x27c), 1);
        } else {
            c.cycles += 3;
            let magic = Addr(c.g[S0][0] as u32).lhu(c);
            c.g[A0][0] = magic;
            c.g[V0][0] = 0x4253;
            if magic == 0x4246 {
                c.cycles += 2;
                ok = 1;
            } else {
                c.cycles += 2;
                if magic == 0x4253 {
                    c.cycles += 2;
                    ok = 1;
                } else {
                    c.cycles += 1;
                    c.sw(gp(c, -0x27c), 1);
                }
            }
        }
        c.cycles += 6;
        c.g[V0][0] = ok;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x30);
        Ok(())
    })
}

/// `void VidEnginePlayer_StartPlayback(VidEnginePlayer* p)` (0x002536c8):
/// restart the AV player on the player's current frame, pause it, create the
/// texture sink for the decoded frames and resume.
pub fn vid_engine_player_start_playback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
        c.g[A1][0] = 1;
        c.sq(S0, sp(c).wrapping_add(0x10));
        c.g[A2][0] = 0x21;
        c.sd(sp(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.sw(c.g[S0][0] as u32, 0);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
        c.sw((c.g[S0][0] as u32).wrapping_add(4), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0);
        call_ptr(c, 0x003a_e450, 0x0025_36f4)?; // AV_Player_Start
        c.g[V1][0] = 1;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x40), c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        call_ptr(c, 0x003a_e938, 0x0025_3708)?; // AV_Player_Pause
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
        c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
        c.g[A2][0] = c.lw(gpa(c, 0x2a90));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x64));
        call_ptr(c, 0x0025_4368, 0x0025_3720)?; // VD_CreateTextureSink
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x3c), c.g[V0][0]);
        call_ptr(c, 0x003a_e958, 0x0025_372c)?; // AV_Player_Resume
        c.g[V1][0] = c.lw(gpn(c, 0x254));
        c.lq(S0, sp(c).wrapping_add(0x10));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        c.g[RA][0] = c.ld(sp(c));
        c.sw(gpn(c, 0x254), c.g[V1][0]);
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        abi::ret(c);
        Ok(())
    })
}

/// `vd_CreateTextureSink(sink, w, h, fmt)` (0x254368): allocate a
/// `VD_TextureSink` through the render-device hook and construct it.
pub fn vd_create_texture_sink(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = 0x510000;
        frame.save(c, 16, 0x40);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6bd0));
        frame.save(c, 17, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, 18, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, 19, 0x10);
        c.g[S2][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A0][0];
        c.g[A0][0] = 0x4a0000;
        c.g[A1][0] = 0x54;
        c.g[T0][0] = lw_at_2(c, p_3(c.g[V0][0] as u32, 8));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2ea0));
        c.g[V0][0] = lw_at_2(c, (c.g[V1][0] as u32).wrapping_sub(0x6bd0));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        call_ptr(c, c.g[V0][0] as u32, 0x0025_43b8)?;
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[T0][0] = c.g[S2][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, callee::VD_TEXTURE_SINK_CTOR.addr, 0x0025_43d0)?;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void VidEnginePlayer_Render(self, int force)` (0x253890): drive the
/// player object's present or its frame-advance method.
pub fn vid_engine_player_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p_4(r32(c, S0), 0x38)).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A2][0] = Addr(p_4(r32(c, S0), 0x3c)).lw(c);
            let no_obj = c.g[A2][0] == 0;
            c.g[A3][0] = c.g[A2][0];
            if !no_obj {
                // 1: the object's vtable slot (a3 + delta); 2: the player's
                // own word field; 3: the object's other slot.
                let mut via: u32;
                c.cycles += 2;
                if c.g[A1][0] != 0 {
                    c.g[V0][0] = Addr(p_4(r32(c, A3), 4)).lw(c);
                    via = 1;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p_4(r32(c, S0), 8)).lw(c);
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = Addr(p_4(r32(c, A3), 4)).lw(c);
                        via = 1;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = Addr(p_4(r32(c, S0), 0x40)).lw(c);
                        if c.g[V0][0] != 0 {
                            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                            via = 2;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = Addr(p_4(r32(c, S0), 4)).lw(c);
                            if c.g[V0][0] != 0 {
                                c.g[V0][0] = Addr(p_4(r32(c, A3), 4)).lw(c);
                                via = 1;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                                via = 2;
                            }
                        }
                        if via == 2 {
                            c.cycles += 2;
                            if c.g[V0][0] == 0 {
                                c.g[V0][0] = Addr(p_4(r32(c, A2), 4)).lw(c);
                                via = 3;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = Addr(p_4(r32(c, A3), 4)).lw(c);
                                via = 1;
                            }
                        }
                    }
                }
                if via == 1 {
                    c.cycles += 6;
                    c.g[A0][0] = Addr(p_4(r32(c, V0), 0x28)).lh(c);
                    c.g[V1][0] = Addr(p_4(r32(c, V0), 0x2c)).lw(c);
                    let present = r32(c, V1);
                    c.g[A0][0] = sx(p_4(r32(c, A3), r32(c, A0)));
                    call_ptr(c, present, 0x00253904)?;
                    frame.restore(c, S0, 0x10);
                } else {
                    if via != 0 {
                        c.cycles += 6;
                        c.g[A1][0] = Addr(p_4(r32(c, S0), 0xc)).lw(c);
                        c.g[A0][0] = Addr(p_4(r32(c, V0), 0x20)).lh(c);
                        c.g[V1][0] = Addr(p_4(r32(c, V0), 0x24)).lw(c);
                        let advance = r32(c, V1);
                        c.g[A0][0] = sx(p_4(r32(c, A2), r32(c, A0)));
                        call_ptr(c, advance, 0x00253920)?;
                        Addr(p_4(r32(c, S0), 0x40)).sw(c, 0);
                    }
                    c.cycles += 1;
                    frame.restore(c, S0, 0x10);
                }
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void VidEnginePlayer_Update(player)` (0x253938): wait for the current
/// frame, decode it, and when both video and audio are done restart.
pub fn vid_engine_player_update(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x20);
        {
            'first: {
                'due: {
                    c.cycles += 7;
                    frame.save(c, S0, 0x10);
                    frame.save_ra(c, 0);
                    c.g[S0][0] = c.g[A0][0];
                    c.g[V0][0] = lw(c, at_2(c.g[S0][0], 0x40));
                    let t = c.g[V0][0] != 0;
                    c.g[V0][0] = 1;
                    if t {
                        break 'due;
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0025_395c);
                    c.g[A0][0] = lw(c, at_2(c.g[S0][0], 0x38));
                    call_ptr(c, 0x003a_e780, 0x0025_395c)?;
                    let t = c.g[V0][0] == 0;
                    c.g[V0][0] = 1;
                    if t {
                        break 'due;
                    }
                    c.cycles += 14;
                    c.g[A1][0] = lw(c, at_2(c.g[S0][0], 0x3c));
                    c.g[V0][0] = lw(c, at_2(c.g[A1][0], 4));
                    c.g[A0][0] = lh(c, at_2(c.g[V0][0], 0x30));
                    c.g[V1][0] = lw(c, at_2(c.g[V0][0], 0x34));
                    let target = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0025_397c);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0025_397c)?;
                    c.g[A0][0] = lw(c, at_2(c.g[S0][0], 0x38));
                    c.g[RA][0] = sx(0x0025_3988);
                    c.lwc1(12, at_2(c.g[A0][0], 0x48));
                    call_ptr(c, 0x003a_e6c8, 0x0025_3988)?;
                    c.g[V1][0] = 1;
                    Addr(at_2(c.g[S0][0], 0xc)).sw(c, c.g[V0][0]);
                    Addr(at_2(c.g[S0][0], 0x40)).sw(c, c.g[V1][0]);
                    Addr(at_2(c.g[S0][0], 4)).sw(c, 0);
                    break 'first;
                }
                c.cycles += 1;
                Addr(at_2(c.g[S0][0], 4)).sw(c, c.g[V0][0]);
                break 'first;
            }
            'sec: {
                c.cycles += 3;
                c.g[V0][0] = lw(c, at_2(c.g[S0][0], 0xc));
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x10);
                    break 'sec;
                }
                c.cycles += 3;
                c.g[V0][0] = lw(c, c.g[S0][0] as u32);
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x10);
                    break 'sec;
                }
                c.cycles += 7;
                c.g[V0][0] = 1;
                Addr(at_2(c.g[S0][0], 0x40)).sw(c, 0);
                c.g[A0][0] = lw(c, at_2(c.g[S0][0], 0x38));
                c.g[RA][0] = sx(0x0025_39cc);
                Addr(at_2(c.g[S0][0], 4)).sw(c, c.g[V0][0]);
                call_ptr(c, 0x003a_e860, 0x0025_39cc)?;
                Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
                frame.restore(c, S0, 0x10);
                break 'sec;
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void VidEnginePlayer_Close(VidEnginePlayer *self)` (0x2535f8): release the
/// player's texture/handle objects and clear its fields.
pub fn vid_engine_player_close(c: &mut Cpu) {
    guest(c, |c| {
        'b_253638: {
            c.cycles += 7;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
            if c.g[V0][0] == 0u64 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                break 'b_253638;
            } else {
                'b_253634: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3c));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
                    if t {
                        break 'b_253634;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
                        vslot(c, c.g[V0][0], 0x98, V1);
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x00253634);
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x00253634)?;
                        break 'b_253634;
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                break 'b_253638;
            }
        }
        'b_25365c: {
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3c));
                break 'b_25365c;
            } else {
                c.cycles += 3;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                if c.g[A1][0] == 0u64 {
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3c));
                    break 'b_25365c;
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x00253658);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x64));
                    call_ptr(c, 0x003b0680, 0x00253658)?;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3c));
                    break 'b_25365c;
                }
            }
        }
        'b_253688: {
            c.cycles += 2;
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = 0x3;
            if t {
                break 'b_253688;
            } else {
                c.cycles += 9;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                vslot(c, c.g[V0][0], 0x8, V1);
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00253678);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00253678)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x254));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x3c), 0u64);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                c.sw((c.g[GP][0] as u32).wrapping_sub(0x254), c.g[V0][0]);
                break 'b_253688;
            }
        }
        'b_2536a4: {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
            if c.g[A0][0] == 0u64 {
                c.sw((c.g[S0][0] as u32).wrapping_add(0x40), 0u64);
                break 'b_2536a4;
            } else {
                c.cycles += 4;
                c.g[RA][0] = sx(0x0025369c);
                c.g[A1][0] = 0x3;
                call_ptr(c, 0x003ae300, 0x0025369c)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x38), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x40), 0u64);
                break 'b_2536a4;
            }
        }
        c.cycles += 8;
        c.sw(c.g[S0][0] as u32, 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `MEM_ValidateBlock(block)` (0x2522b0): validate an allocated block's
/// magic (`BS`, `BF`, `BM`), its flag word and (for `BM`) the end marker,
/// recording the error code in `gp - 0x27c` and returning 1 when valid.
pub fn mem_validate_block(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        let _block = abi::arg(c, 0);
        frame.save(c, 16, 0x10);
        c.g[V0][0] = 0x4253;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V1][0] = c.lhu(c.g[S0][0] as u32);
        let t = c.g[V1][0] != c.g[V0][0];
        c.g[T0][0] = 0;
        // 0: done without the error store, 1: error code in V0, 2: error
        // code written to gp - 0x27c
        let mut state = 0u32;
        if !t {
            c.cycles += 4;
            c.g[V0][0] = c.lhu(p_5(c.g[S0][0] as u32, 2));
            c.g[V0][0] &= 0x8000;
            if c.g[V0][0] != 0 {
                c.g[T0][0] = 1;
                state = 1;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 2;
                state = 2;
            }
        }
        if state == 0 {
            // not `BS`: `BF` (free) and `BM` (mixed) blocks
            c.cycles += 3;
            c.g[V0][0] = 0x4246;
            let t = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = 0x424d;
            let mut direct = false;
            if !t {
                c.cycles += 4;
                c.g[V0][0] = c.lhu(p_5(c.g[S0][0] as u32, 2));
                c.g[V0][0] &= 0x4000;
                if c.g[V0][0] == 0 {
                    direct = true;
                } else {
                    c.cycles += 5;
                    c.g[A0][0] = lw_3(c, p_5(c.g[S0][0] as u32, 0x10));
                    call_ptr(c, 0x0025_2248, 0x0025_230c)?;
                    c.g[T0][0] = c.g[V0][0];
                    if c.g[T0][0] == 0 {
                        direct = true;
                    } else {
                        c.cycles += 4;
                        c.g[A0][0] = lw_3(c, p_5(c.g[S0][0] as u32, 0x14));
                        call_ptr(c, 0x0025_2248, 0x0025_2320)?;
                        c.g[T0][0] = c.g[V0][0];
                        state = 1;
                    }
                }
            } else {
                c.cycles += 2;
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = 4;
                if t {
                    state = 2;
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = c.lhu(p_5(c.g[S0][0] as u32, 2));
                    c.g[V0][0] &= 0x2000;
                    let t = c.g[V0][0] == 0;
                    c.g[A2][0] = 0x4245_0000;
                    if t {
                        c.cycles += 2;
                        c.g[T0][0] = 1;
                        state = 1;
                    } else {
                        c.cycles += 16;
                        c.g[V1][0] = lw_3(c, p_5(c.g[S0][0] as u32, 4));
                        c.g[A2][0] |= 0x4e44;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                        c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
                        c.g[A0][0] = c.lbu(p_5(c.g[V1][0] as u32, 1));
                        c.g[A1][0] = c.lbu(p_5(c.g[V1][0] as u32, 2));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                        c.g[A0][0] = sx((c.g[A0][0] as u32) << 16);
                        c.g[A3][0] = c.lbu(p_5(c.g[V1][0] as u32, 3));
                        c.g[V0][0] |= c.g[A0][0];
                        c.g[A1][0] = sx((c.g[A1][0] as u32) << 8);
                        c.g[V0][0] |= c.g[A1][0];
                        c.g[V0][0] |= c.g[A3][0];
                        let t = c.g[V0][0] != c.g[A2][0];
                        c.g[V0][0] = 3;
                        if t {
                            state = 2;
                        } else {
                            c.cycles += 2;
                            c.g[T0][0] = 1;
                            state = 1;
                        }
                    }
                }
            }
            if direct {
                state = 3;
            }
        }
        if state == 2 {
            c.cycles += 1;
            c.sw(gp(c, -0x27c), c.g[V0][0]);
        }
        if state != 3 {
            c.cycles += 1;
        }
        c.cycles += 4;
        c.g[V0][0] = c.g[T0][0];
        frame.restore_ra(c, 0);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `VD_TextureSink_dtor(self, flags)` (0x002547b8): release every texture of
/// the sink, then free the table, the name and the decoder.
pub fn vd_texture_sink_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        c.g[S1][0] = 0;
        frame.save(c, S0, 0x60);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[S2][0];
        c.g[S5][0] = c.g[A1][0];
        c.g[V0][0] = sx(0x0048_0270);
        sw_2(c, this, 4, c.g[V0][0]);

        'scan: {
            c.g[V1][0] = lw_4(c, this, 0x18);
            if (c.g[V1][0] as i64) <= 0 {
                break 'scan;
            }
            c.cycles += 4;
            c.g[S4][0] = 0xffff_ffff_ffff_ffff;
            c.g[S3][0] = sx(0x0050_9430);
            c.g[V1][0] = lw_4(c, this, 0x14);
            loop {
                // Release texture `i`.
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[S0][0] = lw_4(c, c.g[V0][0], 0);
                let empty = c.g[S0][0] == 0;
                c.g[A2][0] = Addr(gp(c, 0x2a90)).lw(c);
                if !empty {
                    c.cycles += 10;
                    c.g[A1][0] = lw_4(c, c.g[S0][0], 0);
                    c.g[V0][0] = lw_4(c, c.g[A2][0], 0x10d8);
                    c.g[A0][0] = lh_2(c, c.g[V0][0], 0x190);
                    c.g[V1][0] = lw_4(c, c.g[V0][0], 0x194);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, c.g[V1][0] as u32, 0x0025_4834)?;
                    sw_2(c, c.g[S0][0], 0, c.g[S4][0]);
                    c.g[V0][0] = lw_4(c, c.g[S3][0], 4);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, c.g[V0][0] as u32, 0x0025_4844)?;
                }
                c.cycles += 5;
                c.g[V0][0] = lw_4(c, this, 0x18);
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break 'scan;
                }
                c.g[V1][0] = lw_4(c, this, 0x14);
                if event_due(c) {
                    deliver_event_here(c, 0x0025_4808);
                }
            }
        }

        // Free the texture table.
        c.cycles += 3;
        c.g[A0][0] = lw_4(c, this, 0x14);
        let no_table = c.g[A0][0] == 0;
        let name;
        if no_table {
            name = lw_4(c, this, 0xc);
        } else {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0025_486c)?;
            name = lw_4(c, this, 0xc);
        }
        c.g[A1][0] = name;
        // Free the name.
        c.cycles += 2;
        let no_name = c.g[A1][0] == 0;
        sw_2(c, this, 0x14, 0);
        if !no_name {
            c.cycles += 3;
            c.g[A0][0] = lw_4(c, this, 0);
            call_ptr(c, 0x003b_0680, 0x0025_4880)?;
            sw_2(c, this, 0xc, 0);
        }
        // Virtual destructor tail: restore the base-object vtable and, for a
        // deleting destructor, call the operator delete.
        c.cycles += 5;
        c.g[V0][0] = sx(0x0048_02b0);
        c.g[V1][0] = c.g[S5][0] & 1;
        sw_2(c, this, 4, c.g[V0][0]);
        if c.g[V1][0] != 0 {
            c.cycles += 4;
            c.g[V1][0] = sx(0x0051_0000);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_sub(0x6bcc)).lw(c);
            c.g[A0][0] = this;
            call_ptr(c, c.g[V0][0] as u32, 0x0025_48a8)?;
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cDSockChannel_Update(cDSockChannel *self)` (0x256a00):
/// dispatches on the channel state: connecting (1) polls the pending
/// connect, connected (2) services the link; any other state is left
/// alone.
pub fn c_dsock_channel_update(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        if c.pc != 0x0025_6a00 {
            return Ok(ee::s06::f_00256a00__entries(c));
        }
        'l_256a00: loop {
            'b_256a50: {
                'b_256a4c: {
                    c.cycles += 7;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                    c.g[A1][0] = c.g[A0][0];
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = (c.g[V1][0] < 0x5u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        break 'b_256a4c;
                    } else {
                        'b_256a44: {
                            c.cycles += 6;
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x460));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                            let t = c.g[A0][0] as u32;
                            match t {
                                0x00256a00 => {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0025_6a00);
                                    }
                                    continue 'l_256a00;
                                }
                                0x00256a34 => {
                                    c.cycles += 4;
                                    c.g[A0][0] = c.g[A1][0];
                                    C_DSOCK_CHANNEL_UPDATE_CONNECTING.call(c, 0x0025_6a3c)?;
                                    let t = true;
                                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                                    if t {
                                        break 'b_256a50;
                                    } else {
                                        break 'b_256a44;
                                    }
                                }
                                0x00256a44 => {
                                    break 'b_256a44;
                                }
                                0x00256a4c => {
                                    break 'b_256a4c;
                                }
                                _ => {
                                    c.jump_out(t, 0x00256a00, 0x00256a58);
                                    return Ok(());
                                }
                            }
                        }
                        c.cycles += 2;
                        c.g[A0][0] = c.g[A1][0];
                        C_DSOCK_CHANNEL_UPDATE_CONNECTED.call(c, 0x0025_6a4c)?;
                        break 'b_256a4c;
                    }
                }
                c.cycles += 1;
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                break 'b_256a50;
            }
            c.cycles += 2;
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.pc = t;
            return Ok(());
        }
    })
}

/// `void cDSockChannel_UpdateConnected(cDSockChannel *self)`
/// (0x256aa8): services the connected channel: polls link status,
/// ages the quality estimate, and drops to state 3 (stalled) or 4
/// (idle) when the link goes quiet.
pub fn c_dsock_channel_update_connected(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        'b_256b98: {
            'b_256b8c: {
                'b_256b84: {
                    c.cycles += 9;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[S0][0] = c.g[A0][0];
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                    C_NET_GAME_LINK_STATUS.call(c, 0x0025_6ac0)?;
                    c.g[A0][0] = c.g[V0][0];
                    if c.g[A0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                        break 'b_256b84;
                    } else {
                        'b_256aec: {
                            c.cycles += 6;
                            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x20));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                            c.g[V0][0] = ((c.g[V0][0] as i64) < 30001i64) as u64;
                            if c.g[V0][0] != 0u64 {
                                c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x45));
                                break 'b_256aec;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[V1][0] = 0x3;
                                if t {
                                    break 'b_256b8c;
                                } else {
                                    break 'b_256aec;
                                }
                            }
                        }
                        'b_256b14: {
                            c.cycles += 2;
                            if c.g[V0][0] != 0u64 {
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                                break 'b_256b14;
                            } else {
                                c.cycles += 8;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                                let t = c.g[V1][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x0025_6b08)?;
                                c.g[V0][0] = 0x1;
                                let t = true;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                if t {
                                    break 'b_256b98;
                                } else {
                                    break 'b_256b14;
                                }
                            }
                        }
                        'b_256b40: {
                            'b_256b2c: {
                                c.cycles += 2;
                                let t = (c.g[V0][0] as i64) < 0;
                                c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
                                if t {
                                    break 'b_256b2c;
                                } else {
                                    c.cycles += 4;
                                    c.mtc1(2, 1);
                                    c.cvt_s_w(1, 1);
                                    let t = true;
                                    if t {
                                        break 'b_256b40;
                                    } else {
                                        break 'b_256b2c;
                                    }
                                }
                            }
                            c.cycles += 5;
                            c.g[V0][0] = c.g[V0][0] & 0x1;
                            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                            c.mtc1(2, 1);
                            c.cvt_s_w(1, 1);
                            c.add_s(1, 1, 1);
                            break 'b_256b40;
                        }
                        'b_256b60: {
                            c.cycles += 8;
                            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x4b80));
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 2);
                            c.mul_s(3, 1, 0);
                            c.c_lt_s(2, 3);
                            if c.fcc() {
                                c.mov_s(3, 2);
                                break 'b_256b60;
                            } else {
                                break 'b_256b60;
                            }
                        }
                        c.cycles += 9;
                        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x10));
                        c.g[AT][0] = 0x40800000;
                        c.mtc1(1, 1);
                        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4b7c));
                        c.mul_s(0, 0, 1);
                        c.add_s(0, 0, 3);
                        c.mul_s(0, 0, 2);
                        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x10));
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                        break 'b_256b84;
                    }
                }
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = 0x4;
                if t {
                    break 'b_256b98;
                } else {
                    break 'b_256b8c;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = 0x1;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
            break 'b_256b98;
        }
        c.cycles += 4;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `int cDSockChannel_Recv(cDSockChannel *self, void *dst, int maxlen)`
/// (0x256730): pulls one packet off the connected channel into `dst`,
/// returning its length (0 when nothing is ready).
pub fn c_dsock_channel_recv(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00256730 => {
                    c.cycles += 13;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x170));
                    c.g[V0][0] = 0x2;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x160));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x150));
                    c.g[S0][0] = c.g[A0][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x140));
                    c.g[S1][0] = c.g[A2][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x130));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x120));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x110), c.g[RA][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[S2][0] = c.g[A1][0];
                    if t {
                        pc = 0x002567bc;
                        continue;
                    }
                    pc = 0x00256764;
                    continue;
                }
                0x00256764 => {
                    c.cycles += 3;
                    c.g[S3][0] = 0x5;
                    let t = true;
                    c.g[S4][0] = 0x1;
                    if t {
                        pc = 0x002567a4;
                        continue;
                    }
                    pc = 0x00256770;
                    continue;
                }
                0x00256770 => {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[S3][0] {
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[S4][0]);
                        pc = 0x002567bc;
                        continue;
                    }
                    pc = 0x00256778;
                    continue;
                }
                0x00256778 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x3));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A2][0] = c.g[V0][0] & 0xff;
                    if t {
                        pc = 0x002567a4;
                        continue;
                    }
                    pc = 0x00256784;
                    continue;
                }
                0x00256784 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                        pc = 0x002567a8;
                        continue;
                    }
                    pc = 0x00256790;
                    continue;
                }
                0x00256790 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4));
                    C_EA_MEM_COPY.call(c, 0x0025_679c)?;
                    let t = true;
                    c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x3));
                    if t {
                        pc = 0x002567c0;
                        continue;
                    }
                    pc = 0x002567a4;
                    continue;
                }
                0x002567a4 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                    pc = 0x002567a8;
                    continue;
                }
                0x002567a8 => {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[A2][0] = 0x1;
                    C_NET_GAME_LINK_RECV.call(c, 0x0025_67b4)?;
                    let t = (c.g[V0][0] as i64) > 0;
                    c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x2));
                    if t {
                        if c.cycles >= c.next_event {
                            c.pc = 0x00256770;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x00256770;
                        continue;
                    }
                    pc = 0x002567bc;
                    continue;
                }
                0x002567bc => {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    pc = 0x002567c0;
                    continue;
                }
                0x002567c0 => {
                    c.cycles += 8;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x160));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x150));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x140));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x130));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x120));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x110));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x170));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00256730, 0x002567e0);
                    return Ok(());
                }
            }
        }
    })
}

/// `void cDSockChannel_Connect(cDSockChannel *self, void *addr, int port)`
/// (0x2568f8): opens the channel and connects it to `addr:port`,
/// marking state 1 (connecting) once the link is up.
pub fn c_dsock_channel_connect(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        'b_25697c: {
            c.cycles += 20;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x140));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x110));
            c.g[S1][0] = c.g[A2][0] & 0xffff;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x100), c.g[RA][0]);
            c.g[S2][0] = c.g[A1][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), 0u64);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
            let t = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0025_6930)?;
            c.g[A0][0] = c.g[S0][0];
            C_DSOCK_CHANNEL_SET_OPEN.call(c, 0x0025_6938)?;
            C_NET_GAME_UTIL_CREATE.call(c, 0x0025_6940)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
            if t {
                break 'b_25697c;
            } else {
                c.cycles += 13;
                c.g[A3][0] = c.g[S1][0];
                c.g[A1][0] = 0x480000;
                c.g[A0][0] = c.g[SP][0];
                c.g[T0][0] = c.g[A3][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x448));
                c.g[A2][0] = c.g[S2][0];
                call_ptr(c, 0x0041_7828, 0x0025_6964)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                c.g[A1][0] = 0x102;
                c.g[A2][0] = c.g[SP][0];
                C_NET_GAME_UTIL_CONNECT.call(c, 0x0025_6974)?;
                c.g[V1][0] = 0x1;
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                break 'b_25697c;
            }
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x100));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
        c.pc = t;
        Ok(())
    })
}

/// `void cDSockChannel_Listen(cDSockChannel *self, void *addr, int port)`
/// (0x256860): opens the channel and listens on `addr:port`, marking
/// state 1 (connecting) once the link is up.
pub fn c_dsock_channel_listen(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        'b_2568e0: {
            c.cycles += 19;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x140));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x110));
            c.g[S1][0] = c.g[A2][0] & 0xffff;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x100), c.g[RA][0]);
            c.g[S2][0] = c.g[A1][0];
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
            let t = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0025_6894)?;
            c.g[A0][0] = c.g[S0][0];
            C_DSOCK_CHANNEL_SET_OPEN.call(c, 0x0025_689c)?;
            C_NET_GAME_UTIL_CREATE.call(c, 0x0025_68a4)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
            if t {
                break 'b_2568e0;
            } else {
                c.cycles += 13;
                c.g[A3][0] = c.g[S1][0];
                c.g[A1][0] = 0x480000;
                c.g[A0][0] = c.g[SP][0];
                c.g[T0][0] = c.g[A3][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x448));
                c.g[A2][0] = c.g[S2][0];
                call_ptr(c, 0x0041_7828, 0x0025_68c8)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                c.g[A1][0] = 0x101;
                c.g[A2][0] = c.g[SP][0];
                C_NET_GAME_UTIL_CONNECT.call(c, 0x0025_68d8)?;
                c.g[V1][0] = 0x1;
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                break 'b_2568e0;
            }
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x100));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
        c.pc = t;
        Ok(())
    })
}

/// `int cDSockChannel_Send(cDSockChannel *self, void *src, int len)`
/// (0x2567e0): queues `len` bytes from `src` on the connected channel;
/// nonzero only when the link accepted the whole packet.
pub fn c_dsock_channel_send(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        'b_256848: {
            'b_256820: {
                'b_256818: {
                    c.cycles += 9;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x140));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                    c.g[V0][0] = 0x2;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
                    c.g[S0][0] = c.g[A0][0];
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x110), c.g[RA][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A3][0] = c.g[A2][0];
                    if t {
                        break 'b_256818;
                    } else {
                        c.cycles += 3;
                        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        let t = c.g[S1][0] != 0u64;
                        c.g[V0][0] = 0u64;
                        if t {
                            break 'b_256848;
                        } else {
                            c.cycles += 2;
                            let t = (c.g[A3][0] as i64) > 0;
                            c.g[V0][0] = 0x5;
                            if t {
                                break 'b_256820;
                            } else {
                                break 'b_256818;
                            }
                        }
                    }
                }
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0u64;
                if t {
                    break 'b_256848;
                } else {
                    break 'b_256820;
                }
            }
            c.cycles += 10;
            c.g[A2][0] = c.g[A3][0];
            c.g[A0][0] = c.g[SP][0] | 0x4;
            c.sb((c.g[SP][0] as u32).wrapping_add(0x2), c.g[V0][0]);
            c.sb((c.g[SP][0] as u32).wrapping_add(0x3), c.g[A3][0]);
            C_EA_MEM_COPY.call(c, 0x0025_6834)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
            c.g[A1][0] = c.g[SP][0];
            c.g[A2][0] = 0x1;
            C_NET_GAME_LINK_SEND.call(c, 0x0025_6844)?;
            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            break 'b_256848;
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x110));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
        c.pc = t;
        Ok(())
    })
}

/// `void cDSockChannel_Disconnect(cDSockChannel *self)` (0x256998):
/// tears the channel down (reset, free the link and util handles) and
/// clears the state word.
pub fn c_dsock_channel_disconnect(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        'b_2569f4: {
            c.cycles += 7;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
            if c.g[V0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                break 'b_2569f4;
            } else {
                'b_2569d8: {
                    c.cycles += 5;
                    C_DSOCK_CHANNEL_RESET_STATE.call(c, 0x0025_69bc)?;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                    if c.g[A0][0] == 0u64 {
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                        break 'b_2569d8;
                    } else {
                        c.cycles += 4;
                        C_NET_GAME_LINK_DESTROY.call(c, 0x0025_69d0)?;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), 0u64);
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                        break 'b_2569d8;
                    }
                }
                'b_2569f0: {
                    c.cycles += 2;
                    if c.g[A0][0] == 0u64 {
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                        break 'b_2569f0;
                    } else {
                        c.cycles += 4;
                        C_NET_GAME_UTIL_DESTROY.call(c, 0x0025_69e8)?;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0u64);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                        break 'b_2569f0;
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                break 'b_2569f4;
            }
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `cDSockChannel *cDSockChannel_ctor(cDSockChannel *self)` (0x256698):
/// base-constructs the channel, installs the vtable and 0.1s poll
/// interval, and clears the state words.
pub fn c_dsock_channel_ctor(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 19;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        C_DSOCK_CHANNEL_BASE_CTOR.call(c, 0x0025_66ac)?;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x4b84));
        c.g[V1][0] = 0x480000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x12d0));
        c.g[V0][0] = c.g[S0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x10));
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0u64);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `void cDSockChannel_UpdateConnecting(cDSockChannel *self)`
/// (0x256a58): polls the pending connect; a completed link moves the
/// channel to the connected state (2).
pub fn c_dsock_channel_update_connecting(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        'b_256a94: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
            C_NET_GAME_UTIL_COMPLETE.call(c, 0x0025_6a70)?;
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.g[V0][0];
            if t {
                break 'b_256a94;
            } else {
                c.cycles += 5;
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0x237c;
                C_NET_GAME_LINK_CREATE.call(c, 0x0025_6a84)?;
                let t = c.g[V0][0] == 0u64;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                if t {
                    break 'b_256a94;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0x2;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    break 'b_256a94;
                }
            }
        }
        c.cycles += 4;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `void cDSockChannel_dtor(cDSockChannel *self, int free)` (0x2566e8):
/// disconnects, then base-destroys (which releases the channel block
/// when `free` is set).
pub fn c_dsock_channel_dtor(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 18;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.g[V0][0] = 0x480000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12d0));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        c.g[RA][0] = sx(0x00256710);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        C_DSOCK_CHANNEL_DISCONNECT.call(c, 0x0025_6710)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        C_DSOCK_CHANNEL_BASE_DTOR.call(c, 0x0025_671c)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `float cDSockChannel_v05(cDSockChannel *self)` (0x256ba8): returns the
/// smoothed link-quality float at `self+0x10`.
pub fn dsock_channel_v05(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x10));
    c.pc = t;
    return;
}

/// `void cDSockChannel_BaseUpdate(cDSockChannel *self)` (0x267dc8):
/// base tick slot; the connected channel overrides it, so the base
/// version returns without touching state.
pub fn c_dsock_channel_base_update(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.pc = t;
    return;
}

/// `int cDSockChannel_BaseIsConnected(cDSockChannel *self)` (0x267dd0):
/// base connected-query slot, always 0.
pub fn c_dsock_channel_base_is_connected(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = 0u64;
    c.pc = t;
    return;
}

/// `int cDSockChannel_Base_v06(cDSockChannel *self)` (0x267de8): base
/// vtable slot returning 0.
pub fn c_dsock_channel_base_v06(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = 0u64;
    c.pc = t;
    return;
}

/// `int cDSockChannel_BaseRecv(...)` (0x267df0): base receive slot,
/// always 0.
pub fn c_dsock_channel_base_recv(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = 0u64;
    c.pc = t;
    return;
}

/// `int cDSockChannel_BaseSend(...)` (0x267df8): base send slot, always
/// 0.
pub fn c_dsock_channel_base_send(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = 0u64;
    c.pc = t;
    return;
}

/// `int cDSockChannel_v06(cDSockChannel *self)` (0x267e10): vtable slot
/// returning 1.
pub fn c_dsock_channel_v06(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = 0x1;
    c.pc = t;
    return;
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::fill;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::be::bedbprofile::tests::strict;
    pub(crate) use crate::game::bx::bxstring::tests::alloc_ready;

    pub(crate) use crate::game::camera::camera::tests::alloc_base;

    pub(crate) use crate::game::camera::camera::tests::run as run_4;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::be_library_2 as be_library;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::dtor_setup;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::heap_init;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::rng_2;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::debugfe::tests::w;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::check_all_2 as check_all;

    pub(crate) use crate::game::fe::feasyncfile::tests::gp_at;

    pub(crate) use crate::game::fe::feasyncfile::tests::put_str_2 as put_str;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot_9 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_3;

    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_4;

    pub(crate) use crate::game::fe::fepopupmisc::tests::setup;
    pub(crate) use crate::game::fe::fereal::tests::base;
    pub(crate) use crate::game::main::gamestate::tests::base_3;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_3;
    pub(crate) use crate::game::movie::movieplayer::tests::globals;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS_2;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS;
    use ssx3_ee as ee;

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
        check(
            ssx3_ee::s06::f_00253aa0,
            vid_engine_install_mem_hooks,
            50,
            &|c, seed| {
                let mut r = Rng(0x253a_a000 ^ seed as u64);
                scramble(c, &mut r);
                for addr in [0x509430u32, 0x509434, 0x509438] {
                    c.bus.write32(addr, r.next_u32());
                }
            },
        );
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
        check(
            ssx3_ee::s06::f_002557e0,
            c_dsock_channel_base_ctor,
            100,
            &|c, seed| {
                let mut r = Rng(0x2557_e000 ^ seed as u64);
                scramble(c, &mut r);
                let base = DATA + (seed % 4) * 4;
                c.g[A0][0] = base as u64;
                for off in [0u32, 4, 8] {
                    c.bus.write32(base + off, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn c_dsock_channel_set_open_matches() {
        check(
            ssx3_ee::s06::f_00255830,
            c_dsock_channel_set_open,
            50,
            &|c, seed| {
                let mut r = Rng(0x2558_3000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA, r.next_u32());
                c.bus.write32(DATA + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn c_dsock_channel_reset_state_matches() {
        check(
            ssx3_ee::s06::f_00255840,
            c_dsock_channel_reset_state,
            200,
            &|c, seed| {
                let mut r = Rng(0x2558_4000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus
                    .write32(DATA, if seed % 2 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(DATA + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn vd_texture_sink_set_frame_size_matches() {
        check(
            ee::s06::f_002543f0__generated,
            vd_texture_sink_set_frame_size,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0025_43f0, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.f[12] = r.next_u32();
                c.f[13] = r.edge_f32();
                c.f[14] = r.next_u32();
            },
        );
    }

    // -------------------------------------------------------------- memory

    #[test]
    fn mem_get_block_tag_matches() {
        check(
            ssx3_ee::s06::f_00252660__generated,
            mem_get_block_tag,
            250,
            &|c, seed| {
                let mut r = rng_2(seed, 0x25);
                scramble(c, &mut r);
                let block = DATA + 0x8000;
                let tagged = seed % 2 == 0;
                let size_word = if tagged {
                    0x800u16 | (r.next_u32() as u16 & 0x7fff)
                } else {
                    r.next_u32() as u16 & 0x7ff
                };
                c.bus.write16(block - 0xe, size_word);
                // The word at -0xc is the signed offset from the block to the tag
                // record; the value is the unaligned word at record + 4.
                let tag = DATA + 0x9000;
                let extra = match seed % 4 {
                    0 => 0u32,
                    1 => 0x10,
                    2 => 0x1000,
                    _ => (-0x20i32) as u32,
                };
                c.bus.write32(
                    block - 0xc,
                    (tag as u32).wrapping_sub(block as u32).wrapping_add(extra),
                );
                for i in 0..8u32 {
                    c.bus.write32(tag + 4 * i, r.next_u32());
                }
                c.g[A0][0] = sx(block);
            },
        );
    }

    #[test]
    fn mem_alloc_named128_matches() {
        check(
            ssx3_ee::s06::f_00252fa0__generated,
            mem_alloc_named128,
            60,
            &|c, seed| {
                let mut r = rng_2(seed, 0x26);
                scramble(c, &mut r);
                heap_init(c);
                let name = DATA + 0x9000;
                let len = 1 + (seed % 8) as u32;
                for i in 0..len {
                    c.bus.write8(name + i, b'A' + (r.next_u32() % 26) as u8);
                }
                c.bus.write8(name + len, 0);
                c.g[A0][0] = sx(name);
                c.g[A1][0] = (8 + (seed as u64 % 0x400)) as u64;
                c.g[A2][0] = (r.next_u32() & 0x0f00_0000) as u64;
            },
        );
    }

    #[test]
    fn vid_engine_player_play_matches() {
        check(
            ssx3_ee::s06::f_00253860__generated,
            vid_engine_player_play,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0xbf, 1);
                globals(c);
                c.g[A0][0] = sx(DATA);
                c.bus
                    .write32(DATA + 0x38, if seed % 2 == 0 { 0 } else { DATA + 0x3000 });
            },
        );
    }

    #[test]
    fn vd_sink_base_dtor_matches() {
        check(
            ee::s06::f_00255638__generated,
            vd_sink_base_dtor,
            60,
            &|c, seed| dtor_setup(c, seed, 0x255638),
        );
    }

    #[test]
    fn c_d_sock_channel_base_dtor_matches() {
        check(
            ee::s06::f_00255800__generated,
            c_d_sock_channel_base_dtor,
            60,
            &|c, seed| dtor_setup(c, seed, 0x255800),
        );
    }

    #[test]
    fn vid_engine_player_dtor_matches() {
        check(
            ssx3_ee::s06::f_00253418__generated,
            vid_engine_player_dtor,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x253418, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = (seed & 1) as u64;
            },
        );
    }

    // ------------------------------------------------------------------
    // Video / buddies / login.

    #[test]
    fn vid_engine_player_ctor_matches() {
        check(
            ee::s06::f_002533c8__generated,
            vid_engine_player_ctor,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2533_c8, 1);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x20);
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x34, r.edge_f32());
                c.bus.write32(GAME_GP + 0x2898, 0);
            },
        );
    }

    // -------------------------------------------------- debug and movie

    #[test]
    fn mem_debug_printf_matches() {
        check(
            ssx3_ee::s06::f_00251f68__generated,
            mem_debug_printf,
            100,
            &|c, seed| {
                let mut r = setup(c, seed, 0x251f68);
                put_str(
                    c,
                    DATA + 0x100,
                    if seed % 3 == 0 { "hello" } else { "hi %d" },
                );
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                c.g[T0][0] = r.next_u32() as u64;
                c.g[T1][0] = r.next_u32() as u64;
                c.g[T2][0] = r.next_u32() as u64;
                c.g[T3][0] = r.next_u32() as u64;
                for i in 0..4 {
                    c.f[12 + i] = r.edge_f32();
                }
            },
        );
    }

    #[test]
    fn vid_engine_player_resume_matches() {
        check(
            ssx3_ee::s06::f_00253a40__generated,
            vid_engine_player_resume,
            150,
            &|c, seed| {
                let _r = setup(c, seed, 0x253a40);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let stage = seed % 5;
                c.bus.write32(this + 0x38, if stage == 0 { 0 } else { 1 });
                c.bus.write32(this + 0x3c, if stage <= 1 { 0 } else { 1 });
                c.bus.write32(this + 0x00, if stage <= 2 { 0 } else { 1 });
                c.bus.write32(this + 0x08, if stage <= 3 { 0 } else { 1 });
            },
        );
    }

    #[test]
    fn vd_texture_sink_flush_matches() {
        check(
            ssx3_ee::s06::f_00254dc0__generated,
            vd_texture_sink_flush,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x254dc0);
                let this = DATA;
                c.g[A0][0] = this as u64;
                // A device whose completion method is a fake (non-zero result, so
                // the spin loop is left at once).
                c.bus.write32(this + 8, DATA + 0x900);
                let vt = vtable(c, &mut r, &[0x90]);
                c.bus.write32(DATA + 0x900 + 0x10d8, vt);
                c.bus.write32(this + 0xc, if seed % 3 == 0 { 0 } else { 1 });
                c.bus.write32(this + 0, DATA + 0xa00);
                let vt = vtable(c, &mut r, &[0x30]);
                c.bus.write32(DATA + 0xa00 + 0x14, vt);
            },
        );
    }

    /// Run one generated function as a setup step, with `args` in a0..t1.
    pub(crate) fn run_2(c: &mut Cpu, addr: u32, f: fn(&mut Cpu), args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(
            c.pc,
            crate::difftest::RA,
            "setup call {addr:#x} did not return"
        );
        c.g[V0][0]
    }

    pub(crate) fn heap_block(c: &mut Cpu, size: u32) -> u32 {
        heap_init(c);
        run_2(
            c,
            0x0032_0058,
            ssx3_ee::s10::MEM_HeapMemalign,
            &[MEM_HEAP as u64, 16, size as u64],
        ) as u32
    }

    #[test]
    fn mem_resize_block_locked_matches() {
        check(
            ssx3_ee::s06::f_002526b8__generated,
            mem_resize_block_locked,
            10,
            &|c, seed| {
                fakes::machine(c, seed, 0x2526b8, 0);
                let block = heap_block(c, 0x40);
                c.g[A0][0] = block as u64;
                c.g[A1][0] = 0x20 + (seed as u64 % 0x40);
            },
        );
    }

    #[test]
    fn vid_engine_player_pause_matches() {
        check(
            ssx3_ee::s06::f_002539e0__generated,
            vid_engine_player_pause,
            80,
            &|c, seed| {
                fakes::machine(c, seed, 0x2539e0, 0);
                w(c, OBJ + 0x38, if seed % 2 == 0 { 0 } else { seed });
                w(c, OBJ + 0x3c, if seed % 3 == 0 { 0 } else { seed });
                w(c, OBJ, (seed / 2) % 2);
                w(c, OBJ + 0x8, (seed / 4) % 2);
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    #[test]
    fn mem_is_valid_block_link_matches() {
        check(
            ee::s06::f_00252248__generated,
            mem_is_valid_block_link,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x25_2248);
                // `EA_IsValidPointer` accepts the low EE window (< 0x100000) and
                // the high kernel windows; main RAM (0x800000) is rejected.
                let valid_ptr = seed % 4 != 0;
                c.g[A0][0] = if valid_ptr { 0x1000 } else { sx(DATA + 0x400) };
                let magic = match seed % 3 {
                    0 => 0x4246u32,
                    1 => 0x4253,
                    _ => r.next_u32() & 0xffff,
                };
                c.bus.write16(0x1000, magic as u16);
                c.bus.write32(GAME_GP.wrapping_sub(0x27c), 0xbeef);
            },
        );
    }

    #[test]
    fn vid_engine_player_start_playback_matches() {
        check(
            ee::s06::f_002536c8__generated,
            vid_engine_player_start_playback,
            40,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x2536, 3);
                let p = DATA;
                w_2(c, p + 0x38, DATA + 0x400);
                w_2(c, p + 0x30, seed);
                fill(c, &mut r, DATA + 0x400, 0x40);
                c.g[A0][0] = sx(p);
            },
        );
    }

    #[test]
    fn vd_create_texture_sink_matches() {
        covered(
            ee::s06::f_00254368__generated,
            vd_create_texture_sink,
            SEEDS_3,
            &|c, seed| {
                base_3(c, seed, 0x54368, 1);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = (seed % 0x40) as u64;
                c.g[A2][0] = ((seed / 0x40) % 0x40) as u64;
                c.g[A3][0] = (seed % 4) as u64;
                c.bus.write32(0x0050_9430, FAKE);
                c.bus.write32(0x0050_9438, 0x1234);
            },
        );
    }

    // ----------------------------------------------------------- video player

    #[test]
    fn vid_engine_player_update_matches() {
        check(
            ee::s06::f_00253938__generated,
            vid_engine_player_update,
            SEEDS_4,
            &|c, seed| {
                let _ = base_4(c, seed, 0x2539_38, 6);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                let player = obj + 0x2000;
                w_3(c, obj + 0x38, player);
                w_3(c, obj + 0x3c, obj + 0x2100);
                w_3(c, obj + 0x40, (seed % 2) as u32);
                w_3(c, obj + 0xc, (seed % 3 == 0) as u32);
                w_3(c, obj, (seed % 4 == 0) as u32);
                // The player's audio track object and its vtables.
                w_3(c, obj + 0x2100 + 4, obj + 0x2200);
                slot_2(c, obj + 0x2200, 0x30, 0);
                w_3(c, player + 0x48, 0);
                w_3(c, player + 0x470, 0);
            },
        );
    }

    #[test]
    fn vid_engine_player_close_matches() {
        strict(
            ee::s06::f_002535f8__generated,
            vid_engine_player_close,
            60,
            &|c, seed| {
                let _r = base_5(c, seed, 0x2535);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(gp_at(0x2a90), DATA + 0x2000);
                c.bus.write32(DATA + 0x38, 0);
                c.bus
                    .write32(DATA + 0x3c, if seed % 3 == 0 { 0 } else { DATA + 0x3000 });
                c.bus
                    .write32(DATA + 0x40, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(DATA + 0xc, (seed % 2) as u32);
                c.bus.write32(DATA + 0x3004, DATA + 0x3100);
                c.bus.write16(DATA + 0x3100 + 8, 0);
                c.bus.write32(DATA + 0x3100 + 0xc, FAKE);
            },
        );
    }

    /// A real heap block of `size` bytes.
    pub(crate) fn alloc(c: &mut Cpu, size: u32) -> u32 {
        run_4(
            c,
            0x0031_7d70,
            ee::s10::f_00317d70,
            &[size as u64, 0x0048_0000, 0, 0],
        ) as u32
    }

    #[test]
    fn vd_texture_sink_dtor_matches() {
        check_all(
            ee::s06::f_002547b8__generated,
            vd_texture_sink_dtor,
            60,
            &|c, seed| {
                let _r = alloc_base(c, seed, 0x2547);
                let this_ = DATA + 0x1000;
                let n = seed % 4;
                let table = alloc(c, 4 * (n + 1));
                for i in 0..n {
                    if seed % 2 == 0 || i == 0 {
                        let tex = alloc(c, 0x10);
                        c.bus.write32(table + 4 * i, tex);
                    } else {
                        c.bus.write32(table + 4 * i, 0);
                    }
                }
                c.bus.write32(this_ + 0x18, n);
                c.bus.write32(this_ + 0x14, table);
                c.bus.write32(this_ + 0xc, 0);
                c.bus.write32(this_, 0);
                c.g[A0][0] = this_ as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }
    use crate::difftest::check_paths;
    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::fall_back;
    use crate::difftest::fakes::hook;
    use crate::difftest::fakes::prev_cell;
    use crate::difftest::RA as SENTINEL_RA;
    pub(crate) const UPDATE_FAKE_BASE: u32 = DATA + 0x6_2000;

    pub(crate) const UPDATE_STATUS_GATE: u32 = UPDATE_FAKE_BASE;

    pub(crate) const UPDATE_STATUS_RESULT: u32 = UPDATE_FAKE_BASE + 4;

    pub(crate) const UPDATE_COMPLETE_GATE: u32 = UPDATE_FAKE_BASE + 8;

    pub(crate) const UPDATE_COMPLETE_RESULT: u32 = UPDATE_FAKE_BASE + 12;

    pub(crate) const UPDATE_LINKCREATE_GATE: u32 = UPDATE_FAKE_BASE + 16;

    pub(crate) const UPDATE_LINKCREATE_RESULT: u32 = UPDATE_FAKE_BASE + 20;

    pub(crate) const UPDATE_FAKE_MAGIC: u32 = 0x5eed_0001;

    pub(crate) fn gated(c: &mut Cpu, gate: u32, magic: u32) -> bool {
        c.bus.read32(gate) == magic
    }

    /// `NetGameLinkStatus`: fixed status block (or null).
    pub(crate) fn update_status_fake(c: &mut Cpu) {
        if !gated(c, UPDATE_STATUS_GATE, UPDATE_FAKE_MAGIC) {
            return fall_back(
                c,
                prev_cell(update_status_fake),
                ee::s14::f_003e6958__generated,
            );
        }
        c.g[V0][0] = c.bus.read32(UPDATE_STATUS_RESULT) as u64;
        c.cycles += 4;
        crate::abi::ret(c);
    }

    /// `NetGameUtilComplete`: forced completion.
    pub(crate) fn update_complete_fake(c: &mut Cpu) {
        if !gated(c, UPDATE_COMPLETE_GATE, UPDATE_FAKE_MAGIC) {
            return fall_back(
                c,
                prev_cell(update_complete_fake),
                ee::s14::f_003e6fc8__generated,
            );
        }
        c.g[V0][0] = c.bus.read32(UPDATE_COMPLETE_RESULT) as u64;
        c.cycles += 4;
        crate::abi::ret(c);
    }

    /// `NetGameLinkCreate`: forced handle.
    pub(crate) fn update_linkcreate_fake(c: &mut Cpu) {
        if !gated(c, UPDATE_LINKCREATE_GATE, UPDATE_FAKE_MAGIC) {
            return fall_back(
                c,
                prev_cell(update_linkcreate_fake),
                ee::s14::f_003e66f0__generated,
            );
        }
        c.g[V0][0] = c.bus.read32(UPDATE_LINKCREATE_RESULT) as u64;
        c.cycles += 4;
        crate::abi::ret(c);
    }

    pub(crate) fn install_update_fakes() {
        hook(
            std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003E6958),
            update_status_fake,
            prev_cell(update_status_fake),
        );
        hook(
            std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003E6FC8),
            update_complete_fake,
            prev_cell(update_complete_fake),
        );
        hook(
            std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003E66F0),
            update_linkcreate_fake,
            prev_cell(update_linkcreate_fake),
        );
    }

    /// Update fixture: state word steers the dispatch; the sibling
    /// arms run their real bodies against the scripted fakes.
    pub(crate) fn update_fixture(c: &mut Cpu, seed: u32) {
        install_update_fakes();
        machine(c, seed, 0x0025_6a00, 0);
        elf_image(c);
        let this = DATA + 0x400;
        let st = DATA + 0x800;
        let variant = seed % 6;
        // States: 0, 1, 2, 3, 4 and out-of-range (early exit).
        c.bus
            .write32(this + 0xc, [0, 1, 2, 3, 4, 9][variant as usize]);
        c.bus.write32(this + 0x1c, DATA + 0x900);
        c.bus.write32(this + 0x8, DATA + 0x600);
        c.bus.write32(DATA + 0x600 + 0x10, 0x20);
        c.bus.write32(DATA + 0x600 + 0x14, 0x003e_6574);
        c.bus.write32(this + 0x18, DATA + 0x950);
        c.bus.write32(this + 0x4, 0);
        c.bus.write32(this + 0x14, 0);
        c.bus.write32(this + 0x10, 0x3f80_0000);
        c.bus.write32(st, 0x0000_1000);
        c.bus.write32(st + 0x20, 0);
        c.bus.write32(st + 0xc, 0x41f0_0000);
        c.bus.write8(st + 0x45, 1);
        c.bus.write32(UPDATE_STATUS_GATE, UPDATE_FAKE_MAGIC);
        c.bus
            .write32(UPDATE_STATUS_RESULT, [0, 0, st, 0, 0, 0][variant as usize]);
        c.bus.write32(UPDATE_COMPLETE_GATE, UPDATE_FAKE_MAGIC);
        c.bus.write32(
            UPDATE_COMPLETE_RESULT,
            [0, DATA + 0x700, 0, 0, 0, 0][variant as usize],
        );
        c.bus.write32(UPDATE_LINKCREATE_GATE, UPDATE_FAKE_MAGIC);
        c.bus.write32(
            UPDATE_LINKCREATE_RESULT,
            [0, DATA + 0x800, 0, 0, 0, 0][variant as usize],
        );
        c.g[A0][0] = this as u64;
        c.g[A1][0] = (DATA + 0x700) as u64;
        c.g[A2][0] = 8;
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = 0x0025_6a00;
    }

    #[test]
    fn c_dsock_channel_update_matches() {
        // idle/connecting/connected/other/out-of-range dispatch arms.
        const PATHS: &[u64] = &[10, 16, 46, 68];
        check_paths(
            ee::s06::f_00256a00__generated,
            c_dsock_channel_update,
            72,
            PATHS,
            &update_fixture,
        );
    }

    /// Resume heads: entering at either call-resume address runs the
    /// same tail as the generated body.
    #[test]
    fn c_dsock_channel_update_resume_heads_matches() {
        for entry in [0x0025_6a34u32, 0x0025_6a44u32] {
            check(
                ee::s06::f_00256a00__generated,
                c_dsock_channel_update,
                8,
                &|c, seed| {
                    update_fixture(c, seed);
                    c.pc = entry;
                },
            );
        }
    }

    pub(crate) const CHANNEL_FAKE_BASE: u32 = DATA + 0x6_1000;

    pub(crate) const CHANNEL_STATUS_GATE: u32 = CHANNEL_FAKE_BASE;

    pub(crate) const CHANNEL_STATUS_RESULT: u32 = CHANNEL_FAKE_BASE + 4;

    pub(crate) const CHANNEL_RECV_GATE: u32 = CHANNEL_FAKE_BASE + 8;

    pub(crate) const CHANNEL_RECV_IDX: u32 = CHANNEL_FAKE_BASE + 12;

    pub(crate) const CHANNEL_RECV_SEQ: u32 = CHANNEL_FAKE_BASE + 16;

    pub(crate) const CHANNEL_UTILCREATE_GATE: u32 = CHANNEL_FAKE_BASE + 40;

    pub(crate) const CHANNEL_UTILCREATE_RESULT: u32 = CHANNEL_FAKE_BASE + 44;

    pub(crate) const CHANNEL_UTILCONNECT_GATE: u32 = CHANNEL_FAKE_BASE + 48;

    pub(crate) const CHANNEL_LINKSEND_GATE: u32 = CHANNEL_FAKE_BASE + 52;

    pub(crate) const CHANNEL_LINKSEND_RESULT: u32 = CHANNEL_FAKE_BASE + 56;

    pub(crate) const CHANNEL_SPRINTF_GATE: u32 = CHANNEL_FAKE_BASE + 60;

    pub(crate) const CHANNEL_FAKE_MAGIC: u32 = 0x5eed_0002;

    /// `NetGameLinkStatus`: connected socket's status block (planted)
    /// or null (quiet link).
    pub(crate) fn channel_status_fake(c: &mut Cpu) {
        if !gated(c, CHANNEL_STATUS_GATE, CHANNEL_FAKE_MAGIC) {
            return fall_back(
                c,
                prev_cell(channel_status_fake),
                ee::s14::f_003e6958__generated,
            );
        }
        c.g[V0][0] = c.bus.read32(CHANNEL_STATUS_RESULT) as u64;
        c.cycles += 4;
        crate::abi::ret(c);
    }

    /// `NetGameLinkRecv`: one scripted datagram per call (length and
    /// tag bytes), from the per-fixture sequence table.
    pub(crate) fn channel_recv_fake(c: &mut Cpu) {
        if !gated(c, CHANNEL_RECV_GATE, CHANNEL_FAKE_MAGIC) {
            return fall_back(
                c,
                prev_cell(channel_recv_fake),
                ee::s14::f_003e6b10__generated,
            );
        }
        let idx = c.bus.read32(CHANNEL_RECV_IDX).min(1);
        let b = c.g[A1][0] as u32;
        let b2 = c.bus.read32(CHANNEL_RECV_SEQ + 12 * idx + 4) as u8;
        let b3 = c.bus.read32(CHANNEL_RECV_SEQ + 12 * idx + 8) as u8;
        c.bus.write8(b + 2, b2);
        c.bus.write8(b + 3, b3);
        c.g[V0][0] = c.bus.read32(CHANNEL_RECV_SEQ + 12 * idx) as i32 as i64 as u64;
        c.bus.write32(CHANNEL_RECV_IDX, idx + 1);
        c.cycles += 4;
        crate::abi::ret(c);
    }

    /// `NetGameUtilCreate`: handle or null (steers the open path).
    pub(crate) fn channel_utilcreate_fake(c: &mut Cpu) {
        if !gated(c, CHANNEL_UTILCREATE_GATE, CHANNEL_FAKE_MAGIC) {
            return fall_back(
                c,
                prev_cell(channel_utilcreate_fake),
                ee::s14::f_003e6e08__generated,
            );
        }
        c.g[V0][0] = c.bus.read32(CHANNEL_UTILCREATE_RESULT) as u64;
        c.cycles += 4;
        crate::abi::ret(c);
    }

    /// `NetGameUtilConnect`: result ignored by the caller; fixed cost.
    pub(crate) fn channel_utilconnect_fake(c: &mut Cpu) {
        if !gated(c, CHANNEL_UTILCONNECT_GATE, CHANNEL_FAKE_MAGIC) {
            return fall_back(
                c,
                prev_cell(channel_utilconnect_fake),
                ee::s14::f_003e6f40__generated,
            );
        }
        c.g[V0][0] = 0;
        c.cycles += 4;
        crate::abi::ret(c);
    }

    /// `NetGameLinkSend`: accepted byte count (steers the nonzero arm).
    pub(crate) fn channel_linksend_fake(c: &mut Cpu) {
        if !gated(c, CHANNEL_LINKSEND_GATE, CHANNEL_FAKE_MAGIC) {
            return fall_back(
                c,
                prev_cell(channel_linksend_fake),
                ee::s14::f_003e6c80__generated,
            );
        }
        c.g[V0][0] = c.bus.read32(CHANNEL_LINKSEND_RESULT) as u64;
        c.cycles += 4;
        crate::abi::ret(c);
    }

    /// `sprintf`: the format comes from the game image and the length
    /// is unchecked; the fake leaves the stack buffer alone at a fixed
    /// cost.
    pub(crate) fn channel_sprintf_fake(c: &mut Cpu) {
        if !gated(c, CHANNEL_SPRINTF_GATE, CHANNEL_FAKE_MAGIC) {
            return fall_back(
                c,
                prev_cell(channel_sprintf_fake),
                ee::s15::f_00417828__generated,
            );
        }
        c.g[V0][0] = 8;
        c.cycles += 4;
        crate::abi::ret(c);
    }

    pub(crate) fn install_channel_fakes() {
        hook(
            std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003E6958),
            channel_status_fake,
            prev_cell(channel_status_fake),
        );
        hook(
            std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003E6B10),
            channel_recv_fake,
            prev_cell(channel_recv_fake),
        );
        hook(
            std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003E6E08),
            channel_utilcreate_fake,
            prev_cell(channel_utilcreate_fake),
        );
        hook(
            std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003E6F40),
            channel_utilconnect_fake,
            prev_cell(channel_utilconnect_fake),
        );
        hook(
            std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003E6C80),
            channel_linksend_fake,
            prev_cell(channel_linksend_fake),
        );
        hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_00417828),
            channel_sprintf_fake,
            prev_cell(channel_sprintf_fake),
        );
    }

    /// Channel fixture: state word, idle word, link handle, vtable
    /// routing through `EA_MemCopy`, and seed-steered registers.
    pub(crate) fn channel_fixture(c: &mut Cpu, seed: u32, entry: u32) {
        machine(c, seed, entry as u64, 0);
        elf_image(c);
        let this = DATA + 0x400;
        let vt = DATA + 0x600;
        c.bus.write32(this + 0x8, vt);
        c.bus.write32(vt + 0x10, 0x20);
        c.bus.write32(vt + 0x14, 0x003e_6574);
        c.bus.write32(this + 0x10, 0x3f80_0000 + seed);
        let dst = DATA + 0x700;
        for i in 0..4u32 {
            c.bus.write32(dst + 4 * i, 0x1111_1111 * (i + 1) + seed);
        }
    }

    #[test]
    fn c_dsock_channel_update_connected_matches() {
        // status/idle/flag arms (the stalled, idle, smoothing and
        // vtable paths; combos sharing a total are split by the RAM
        // comparison).
        const PATHS: &[u64] = &[19, 22, 28, 50, 53, 99];
        check_paths(
            ee::s06::f_00256aa8__generated,
            c_dsock_channel_update_connected,
            96,
            PATHS,
            &|c, seed| {
                install_channel_fakes();
                channel_fixture(c, seed, 0x0025_6aa8);
                let this = DATA + 0x400;
                let st = DATA + 0x800;
                let variant = seed % 4;
                c.bus.write32(this + 0x1c, DATA + 0x900);
                c.bus.write32(this + 0x4, [0, 1][seed as usize / 4 % 2]);
                c.bus.write32(this + 0x14, [0, 7][seed as usize / 8 % 2]);
                c.bus.write32(CHANNEL_STATUS_GATE, CHANNEL_FAKE_MAGIC);
                c.bus
                    .write32(CHANNEL_STATUS_RESULT, [0, st, st, 0][variant as usize]);
                // Status block: counter words (small or big spread),
                // flag byte, quality word.
                let big = variant == 2;
                c.bus.write32(
                    st,
                    0x0000_1000 + if big { 0x1_0000 } else { 1000 * (seed % 5) },
                );
                c.bus.write32(st + 0x20, 0x0000_1000);
                c.bus.write32(st + 0xc, 0x0000_00c0 + seed);
                c.bus.write8(st + 0x45, [0, 1][seed as usize / 16 % 2]);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x700) as u64;
                c.g[A2][0] = 8;
                c.g[RA][0] = SENTINEL_RA as u64;
                c.pc = 0x0025_6aa8;
            },
        );
    }

    /// Recv fixture: state word, link handle, destination buffer and
    /// max length, plus the two-call datagram script (length, tag,
    /// extra bytes per call).
    pub(crate) fn channel_recv_fixture(c: &mut Cpu, seed: u32) {
        install_channel_fakes();
        channel_fixture(c, seed, 0x0025_6730);
        let this = DATA + 0x400;
        let dst = DATA + 0xa00;
        let connected = seed % 2;
        c.bus.write32(this + 0xc, [9, 2][connected as usize]);
        c.bus.write32(this + 0x1c, DATA + 0x900);
        c.bus.write32(this + 0x14, 0);
        for i in 0..8u32 {
            c.bus.write32(dst + 4 * i, 0xaaaaaaaa + i);
        }
        // Call 0: steer the first datagram; call 1 always ends the
        // loop so the loop-back arm is covered exactly once per seed.
        let (v0, b2, b3, ml) = match seed / 2 % 6 {
            0 => (0, 0, 0, 4),
            1 => (3, 4, 0, 4),
            2 => (3, 5, 0, 4),
            3 => (3, 5, 6, 4),
            4 => (3, 5, 6, 8),
            _ => (3, 5, 2, 8),
        };
        c.bus.write32(CHANNEL_RECV_SEQ, v0);
        c.bus.write32(CHANNEL_RECV_SEQ + 4, b2);
        c.bus.write32(CHANNEL_RECV_SEQ + 8, b3);
        c.bus.write32(CHANNEL_RECV_SEQ + 12, 0);
        c.bus.write32(CHANNEL_RECV_SEQ + 16, 0);
        c.bus.write32(CHANNEL_RECV_SEQ + 20, 0);
        c.bus.write32(CHANNEL_RECV_IDX, 0);
        c.bus.write32(CHANNEL_RECV_GATE, CHANNEL_FAKE_MAGIC);
        c.g[A0][0] = this as u64;
        c.g[A1][0] = dst as u64;
        // Max length crosses the too-long loop-back arm (b3 = 6).
        c.g[A2][0] = ml as u64;
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = 0x0025_6730;
    }

    driven!(recv_generated, ee::s06::f_00256730__generated);

    driven!(recv_hand, c_dsock_channel_recv);

    #[test]
    fn c_dsock_channel_recv_matches() {
        // not-connected/empty/wrong-tag/empty-tag/too-long/copy arms.
        const PATHS: &[u64] = &[22, 35, 37, 50, 52, 71, 99];
        check_paths(
            ee::s06::f_00256730__generated,
            c_dsock_channel_recv,
            72,
            PATHS,
            &channel_recv_fixture,
        );
    }

    /// Recv event hook: service only the datagram loop head; the copy
    /// call's checks report no event without servicing.
    pub(crate) fn recv_rerun_event(c: &mut Cpu) -> bool {
        if c.pc != 0x0025_6770 {
            return false;
        }
        if c.cycles >= crate::difftest::CYCLE_BUDGET {
            panic!(
                "cycle budget exceeded ({} cycles, pc {:#x}): runaway on test data",
                c.cycles, c.pc
            );
        }
        (ssx3_ps2::platform::HOOKS.event)(c)
    }

    #[test]
    fn c_dsock_channel_recv_events_matches() {
        let resumes = check_events(recv_generated, recv_hand, 48, &|c, seed| {
            // Unwind seeds force the looping variant (connected,
            // empty tag); the rest keep the seed's own arms.
            channel_recv_fixture(c, if seed % 3 == 1 { 5 } else { seed });
            let mut r = Rng(0x9e37_79b9_7f4a_7c15 ^ (seed as u64) << 8 ^ 0x0025_6730);
            let mode = schedule(c, &mut r, seed, 0x0025_6730);
            if mode == Events::Unwind {
                c.next_event = c.cycles;
                ssx3_ps2::platform::machine_of(&mut c.bus.plat)
                    .sched
                    .next_vblank = c.cycles;
                c.hooks.event = recv_rerun_event;
            }
            mode
        });
        assert!(
            resumes.get(&0x0025_6770).copied().unwrap_or(0) > 0,
            "no unwind at head 0x256770 (resumes {resumes:x?})",
        );
    }

    /// Connect/Listen fixture: channel with a vtable routing through
    /// the real `EA_MemCopy`, a short hostname, and a steered port.
    pub(crate) fn channel_open_fixture(c: &mut Cpu, seed: u32, entry: u32) {
        install_channel_fakes();
        channel_fixture(c, seed, entry);
        let this = DATA + 0x400;
        let host = DATA + 0xb00;
        for (i, b) in b"peer\0".iter().enumerate() {
            c.bus.write8(host + i as u32, *b);
        }
        c.bus.write32(CHANNEL_UTILCREATE_GATE, CHANNEL_FAKE_MAGIC);
        c.bus.write32(CHANNEL_UTILCONNECT_GATE, CHANNEL_FAKE_MAGIC);
        c.bus.write32(CHANNEL_SPRINTF_GATE, CHANNEL_FAKE_MAGIC);
        c.bus.write32(
            CHANNEL_UTILCREATE_RESULT,
            [0, DATA + 0xc00][seed as usize % 2],
        );
        c.g[A0][0] = this as u64;
        c.g[A1][0] = host as u64;
        c.g[A2][0] = [0, 80, 443][seed as usize / 2 % 3] as u64;
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = entry;
    }

    #[test]
    fn c_dsock_channel_connect_matches() {
        // create-fails/open-only arms (prologue accounts for the
        // one-cycle split vs Listen).
        const PATHS: &[u64] = &[42, 63, 126, 147, 188, 209];
        check_paths(
            ee::s06::f_002568f8__generated,
            c_dsock_channel_connect,
            64,
            PATHS,
            &|c, seed| channel_open_fixture(c, seed, 0x0025_68f8),
        );
    }

    #[test]
    fn c_dsock_channel_listen_matches() {
        // create-fails/open-only arms.
        const PATHS: &[u64] = &[41, 62, 125, 146, 187, 208];
        check_paths(
            ee::s06::f_00256860__generated,
            c_dsock_channel_listen,
            64,
            PATHS,
            &|c, seed| channel_open_fixture(c, seed, 0x0025_6860),
        );
    }

    #[test]
    fn c_dsock_channel_send_matches() {
        // not-connected/busy/empty/sent arms.
        const PATHS: &[u64] = &[16, 17, 50, 99];
        check_paths(
            ee::s06::f_002567e0__generated,
            c_dsock_channel_send,
            72,
            PATHS,
            &|c, seed| {
                install_channel_fakes();
                channel_fixture(c, seed, 0x0025_67e0);
                let this = DATA + 0x400;
                let src = DATA + 0xd00;
                for i in 0..4u32 {
                    c.bus.write32(src + 4 * i, 0x5152_5354 + i + seed);
                }
                let variant = seed % 4;
                c.bus.write32(this + 0xc, [9, 2, 2, 2][variant as usize]);
                c.bus.write32(this + 0x4, [0, 0, 7, 0][variant as usize]);
                c.bus.write32(CHANNEL_LINKSEND_GATE, CHANNEL_FAKE_MAGIC);
                c.bus
                    .write32(CHANNEL_LINKSEND_RESULT, [0, 1][seed as usize / 4 % 2]);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = src as u64;
                c.g[A2][0] = [0, 1, 8, 8][variant as usize] as u64;
                c.g[RA][0] = SENTINEL_RA as u64;
                c.pc = 0x0025_67e0;
            },
        );
    }

    /// Fake-control block for this module's DirtySock fakes.
    pub(crate) const LIFECYCLE_FAKE_BASE: u32 = DATA + 0x6_0000;

    pub(crate) const LIFECYCLE_DESTROY_GATE: u32 = LIFECYCLE_FAKE_BASE;

    pub(crate) const LIFECYCLE_UTILDESTROY_GATE: u32 = LIFECYCLE_FAKE_BASE + 4;

    pub(crate) const LIFECYCLE_COMPLETE_GATE: u32 = LIFECYCLE_FAKE_BASE + 8;

    pub(crate) const LIFECYCLE_COMPLETE_RESULT: u32 = LIFECYCLE_FAKE_BASE + 12;

    pub(crate) const LIFECYCLE_LINKCREATE_GATE: u32 = LIFECYCLE_FAKE_BASE + 16;

    pub(crate) const LIFECYCLE_LINKCREATE_RESULT: u32 = LIFECYCLE_FAKE_BASE + 20;

    pub(crate) const LIFECYCLE_DELETE_GATE: u32 = LIFECYCLE_FAKE_BASE + 24;

    pub(crate) const LIFECYCLE_FAKE_MAGIC: u32 = 0x5eed_0003;

    /// `__builtin_delete`: fixed-cost stand-in (the real allocator walk
    /// needs a heap block the fixture does not build).
    pub(crate) fn lifecycle_delete_fake(c: &mut Cpu) {
        if !gated(c, LIFECYCLE_DELETE_GATE, LIFECYCLE_FAKE_MAGIC) {
            return fall_back(
                c,
                prev_cell(lifecycle_delete_fake),
                ee::s10::f_00317e50__generated,
            );
        }
        c.g[V0][0] = 0;
        c.cycles += 4;
        crate::abi::ret(c);
    }

    /// `NetGameLinkDestroy`: result unused; fixed cost.
    pub(crate) fn lifecycle_destroy_fake(c: &mut Cpu) {
        if !gated(c, LIFECYCLE_DESTROY_GATE, LIFECYCLE_FAKE_MAGIC) {
            return fall_back(
                c,
                prev_cell(lifecycle_destroy_fake),
                ee::s14::f_003e67f8__generated,
            );
        }
        c.g[V0][0] = 0;
        c.cycles += 4;
        crate::abi::ret(c);
    }

    /// `NetGameUtilDestroy`: result unused; fixed cost.
    pub(crate) fn lifecycle_utildestroy_fake(c: &mut Cpu) {
        if !gated(c, LIFECYCLE_UTILDESTROY_GATE, LIFECYCLE_FAKE_MAGIC) {
            return fall_back(
                c,
                prev_cell(lifecycle_utildestroy_fake),
                ee::s14::f_003e6e70__generated,
            );
        }
        c.g[V0][0] = 0;
        c.cycles += 4;
        crate::abi::ret(c);
    }

    /// `NetGameUtilComplete`: pending-connect poll; forced completion.
    pub(crate) fn lifecycle_complete_fake(c: &mut Cpu) {
        if !gated(c, LIFECYCLE_COMPLETE_GATE, LIFECYCLE_FAKE_MAGIC) {
            return fall_back(
                c,
                prev_cell(lifecycle_complete_fake),
                ee::s14::f_003e6fc8__generated,
            );
        }
        c.g[V0][0] = c.bus.read32(LIFECYCLE_COMPLETE_RESULT) as u64;
        c.cycles += 4;
        crate::abi::ret(c);
    }

    /// `NetGameLinkCreate`: link creation; forced handle.
    pub(crate) fn lifecycle_linkcreate_fake(c: &mut Cpu) {
        if !gated(c, LIFECYCLE_LINKCREATE_GATE, LIFECYCLE_FAKE_MAGIC) {
            return fall_back(
                c,
                prev_cell(lifecycle_linkcreate_fake),
                ee::s14::f_003e66f0__generated,
            );
        }
        c.g[V0][0] = c.bus.read32(LIFECYCLE_LINKCREATE_RESULT) as u64;
        c.cycles += 4;
        crate::abi::ret(c);
    }

    pub(crate) fn install_lifecycle_fakes() {
        hook(
            std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003E67F8),
            lifecycle_destroy_fake,
            prev_cell(lifecycle_destroy_fake),
        );
        hook(
            std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003E6E70),
            lifecycle_utildestroy_fake,
            prev_cell(lifecycle_utildestroy_fake),
        );
        hook(
            std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003E6FC8),
            lifecycle_complete_fake,
            prev_cell(lifecycle_complete_fake),
        );
        hook(
            std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003E66F0),
            lifecycle_linkcreate_fake,
            prev_cell(lifecycle_linkcreate_fake),
        );
        hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            lifecycle_delete_fake,
            prev_cell(lifecycle_delete_fake),
        );
    }

    #[test]
    fn c_dsock_channel_disconnect_matches() {
        // state x link-handle x util-handle arms. (link,util)=(1,0)
        // and (0,1) share a total; their stores differ and both are
        // covered by the RAM comparison.
        const PATHS: &[u64] = &[10, 23, 31, 39];
        check_paths(
            ee::s06::f_00256998__generated,
            c_dsock_channel_disconnect,
            64,
            PATHS,
            &|c, seed| {
                install_lifecycle_fakes();
                machine(c, seed, 0x0025_6998, 0);
                let this = DATA + 0x211;
                let state = [0, 1][seed as usize % 2];
                let link = [0, (DATA + 0x400) as u32][seed as usize / 2 % 2];
                let util = [0, (DATA + 0x500) as u32][seed as usize / 4 % 2];
                c.bus.write32(this + 0xc, state);
                c.bus.write32(this + 0x1c, link);
                c.bus.write32(this + 0x18, util);
                c.bus.write32(LIFECYCLE_DESTROY_GATE, LIFECYCLE_FAKE_MAGIC);
                c.bus
                    .write32(LIFECYCLE_UTILDESTROY_GATE, LIFECYCLE_FAKE_MAGIC);
                c.g[A0][0] = this as u64;
                c.g[RA][0] = SENTINEL_RA as u64;
                c.pc = 0x0025_6998;
            },
        );
    }

    #[test]
    fn c_dsock_channel_ctor_matches() {
        check_paths(
            ee::s06::f_00256698__generated,
            c_dsock_channel_ctor,
            16,
            &[26],
            &|c, seed| {
                machine(c, seed, 0x0025_6698, 0);
                elf_image(c);
                let this = DATA + 0x221;
                c.g[A0][0] = this as u64;
                c.g[RA][0] = SENTINEL_RA as u64;
                c.pc = 0x0025_6698;
            },
        );
    }

    #[test]
    fn c_dsock_channel_update_connecting_matches() {
        // complete? x created?: 2 x 2 arms.
        const PATHS: &[u64] = &[16, 25, 27];
        check_paths(
            ee::s06::f_00256a58__generated,
            c_dsock_channel_update_connecting,
            64,
            PATHS,
            &|c, seed| {
                install_lifecycle_fakes();
                machine(c, seed, 0x0025_6a58, 0);
                let this = DATA + 0x231;
                c.bus.write32(this + 0x18, DATA + 0x600);
                c.bus.write32(LIFECYCLE_COMPLETE_GATE, LIFECYCLE_FAKE_MAGIC);
                c.bus
                    .write32(LIFECYCLE_LINKCREATE_GATE, LIFECYCLE_FAKE_MAGIC);
                c.bus.write32(
                    LIFECYCLE_COMPLETE_RESULT,
                    [0, DATA + 0x700][seed as usize % 2],
                );
                c.bus.write32(
                    LIFECYCLE_LINKCREATE_RESULT,
                    [0, DATA + 0x800][seed as usize / 2 % 2],
                );
                c.g[A0][0] = this as u64;
                c.g[RA][0] = SENTINEL_RA as u64;
                c.pc = 0x0025_6a58;
            },
        );
    }

    #[test]
    fn c_dsock_channel_dtor_matches() {
        // free? x live-state: 2 x 2 arms (delete runs the fixed-cost
        // fake on scratch).
        const PATHS: &[u64] = &[38, 44, 51, 57];
        check_paths(
            ee::s06::f_002566e8__generated,
            c_dsock_channel_dtor,
            64,
            PATHS,
            &|c, seed| {
                install_lifecycle_fakes();
                machine(c, seed, 0x0025_66e8, 0);
                let this = DATA + 0x241;
                let live = seed % 2;
                c.bus.write32(this + 0xc, live);
                c.bus.write32(this + 0x1c, 0);
                c.bus.write32(this + 0x18, 0);
                c.bus.write32(LIFECYCLE_DESTROY_GATE, LIFECYCLE_FAKE_MAGIC);
                c.bus
                    .write32(LIFECYCLE_UTILDESTROY_GATE, LIFECYCLE_FAKE_MAGIC);
                c.bus.write32(LIFECYCLE_DELETE_GATE, LIFECYCLE_FAKE_MAGIC);
                c.g[A0][0] = this as u64;
                // `A1 & 1` steers the `__builtin_delete` arm (fixed-cost
                // fake); the state word steers Disconnect's arms.
                c.g[A1][0] = (seed / 2 % 2) as u64;
                c.g[RA][0] = SENTINEL_RA as u64;
                c.pc = 0x0025_66e8;
            },
        );
    }

    pub(crate) fn fixture_const(c: &mut Cpu, seed: u32, entry: u32, salt: u64) {
        machine(c, seed, salt, 0);
        c.g[A0][0] = (DATA + 0x181) as u64;
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = entry;
    }

    #[test]
    fn dsock_channel_v05_matches() {
        check_paths(
            ee::s06::f_00256ba8__generated,
            dsock_channel_v05,
            40,
            &[2],
            &|c, seed| {
                machine(c, seed, 0x0025_6ba8, 0);
                let this = DATA + 0x191;
                // Distinctive float bits incl. a negative and a denormal.
                let bits = [0x3f80_0001, 0xbf80_0001, 0x0000_0001, 0x7f80_0000];
                c.bus.write32(this + 0x10, bits[seed as usize % 4]);
                c.g[A0][0] = this as u64;
                c.g[RA][0] = SENTINEL_RA as u64;
                c.pc = 0x0025_6ba8;
            },
        );
    }

    #[test]
    fn c_dsock_channel_base_update_matches() {
        check_paths(
            ee::s07::f_00267dc8__generated,
            c_dsock_channel_base_update,
            16,
            &[2],
            &|c, seed| fixture_const(c, seed, 0x0026_7dc8, 0x0026_7dc8),
        );
    }

    #[test]
    fn c_dsock_channel_base_is_connected_matches() {
        check_paths(
            ee::s07::f_00267dd0__generated,
            c_dsock_channel_base_is_connected,
            16,
            &[2],
            &|c, seed| fixture_const(c, seed, 0x0026_7dd0, 0x0026_7dd0),
        );
    }

    #[test]
    fn c_dsock_channel_base_v06_matches() {
        check_paths(
            ee::s07::f_00267de8__generated,
            c_dsock_channel_base_v06,
            16,
            &[2],
            &|c, seed| fixture_const(c, seed, 0x0026_7de8, 0x0026_7de8),
        );
    }

    #[test]
    fn c_dsock_channel_base_recv_matches() {
        check_paths(
            ee::s07::f_00267df0__generated,
            c_dsock_channel_base_recv,
            16,
            &[2],
            &|c, seed| fixture_const(c, seed, 0x0026_7df0, 0x0026_7df0),
        );
    }

    #[test]
    fn c_dsock_channel_base_send_matches() {
        check_paths(
            ee::s07::f_00267df8__generated,
            c_dsock_channel_base_send,
            16,
            &[2],
            &|c, seed| fixture_const(c, seed, 0x0026_7df8, 0x0026_7df8),
        );
    }

    #[test]
    fn c_dsock_channel_v06_matches() {
        check_paths(
            ee::s07::f_00267e10__generated,
            c_dsock_channel_v06,
            16,
            &[2],
            &|c, seed| fixture_const(c, seed, 0x0026_7e10, 0x0026_7e10),
        );
    }
}
