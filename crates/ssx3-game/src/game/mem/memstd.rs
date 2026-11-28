//! `src/mem/memstd.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::tail_wrapper;
use crate::call::Callee;
use crate::call::Frame;
use crate::call::Target;
use crate::events::back_edge_unwinds;
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
    (
        0x0025_3460,
        vid_engine_player_reopen,
        "VidEnginePlayer_Reopen",
    ),
    (0x0025_34a8, vid_engine_player_open, "VidEnginePlayer_Open"),
    (
        0x0025_3890,
        vid_engine_player_render,
        "VidEnginePlayer_Render",
    ),
    (0x0025_3af8, vid_engine_mem_free, "VidEngine_MemFree"),
    (
        0x0025_3b18,
        vid_engine_debug_printf_stub,
        "VidEngine_DebugPrintfStub",
    ),
    (
        0x0025_4330,
        vid_engine_player_global_ctors,
        "VidEnginePlayer_GlobalCtors",
    ),
    (
        0x0025_4400,
        vd_texture_sink_set_frame_rate,
        "VD_TextureSink_SetFrameRate",
    ),
    (0x0025_4410, vd_texture_sink_ctor, "VD_TextureSink_ctor"),
    (
        0x0025_48d0,
        vd_texture_sink_draw_frame,
        "VD_TextureSink_DrawFrame",
    ),
    (
        0x0025_4c48,
        vd_texture_sink_upload_frame,
        "VD_TextureSink_UploadFrame",
    ),
    (0x0025_4da0, vd_texture_sink_redraw, "VD_TextureSink_Redraw"),
    (0x0025_4e20, vd_debug_printf_stub, "VD_DebugPrintfStub"),
    (0x0025_5670, vd_alloc, "VD_Alloc"),
    (0x0025_56b0, vd_alloc_2, "VD_Alloc_2"),
    (0x0025_5750, vd_free, "VD_Free"),
    (0x0025_5778, vd_free_2, "VD_Free_2"),
    (0x0025_57c0, vd_global_ctors, "VD_GlobalCtors"),
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

    pub(crate) use crate::game::bx::cubicspline::callee::PS2_GRAPHICS_MAN_SET_TEXTURE_FILTER;

    pub(crate) use crate::game::bx::cubicspline::callee::VD_FLOOR_LOG2;
    pub(crate) use crate::game::bx::cubicspline::callee::VD_TEXTURE_SINK_SET_FRAME_RATE;
    pub(crate) use crate::game::bx::cubicspline::callee::VD_TEXTURE_SINK_SET_FRAME_SIZE;

    pub(crate) use crate::game::misc::callee::VIDENGINEPLAYER_INIT_OPEN_PARAMS;
    pub(crate) use crate::game::misc::callee::VIDENGINEPLAYER_OPEN;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VID_ENGINE_INSTALL_MEM_HOOKS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VID_ENGINE_PLAYER_INIT_OPEN_PARAMS;

    /// `Serialize_ReadU32_2` (`0x002d2558`).
    pub(crate) const SERIALIZE_READ_U32_2: Callee = Callee {
        addr: 0x002d_2558,
        f: ee::s09::Serialize_ReadU32_2,
    };

    /// `Serialize_ReadU8` (`0x002d2790`).
    pub(crate) const SERIALIZE_READ_U8: Callee = Callee {
        addr: 0x002d_2790,
        f: ee::s09::Serialize_ReadU8,
    };

    /// `MD5_Update` (`0x0031d3a8`).
    pub(crate) const MD5_UPDATE: Callee = Callee {
        addr: 0x0031_d3a8,
        f: ee::s10::MD5_Update,
    };

    /// `MD5_Transform` (`0x0031c350`).
    pub(crate) const MD5_TRANSFORM: Callee = Callee {
        addr: 0x0031_c350,
        f: ee::s10::MD5_Transform,
    };

    /// `DLHeap_MoreCore` (`0x0031ffd8`).
    pub(crate) const DLHEAP_MORE_CORE: Callee = Callee {
        addr: 0x0031_ffd8,
        f: ee::s10::DLHeap_MoreCore,
    };

    /// `VidEnginePlayer_Close` (`0x002535f8`).
    pub(crate) const VIDENGINEPLAYER_CLOSE: Callee = Callee {
        addr: 0x0025_35f8,
        f: ee::s06::VidEnginePlayer_Close,
    };

    /// `VidEnginePlayer_StartPlayback` (`0x002536c8`).
    pub(crate) const VIDENGINEPLAYER_START_PLAYBACK: Callee = Callee {
        addr: 0x0025_36c8,
        f: ee::s06::VidEnginePlayer_StartPlayback,
    };

    /// `VD_TextureSink_DrawFrame` (`0x002548d0`).
    pub(crate) const VD_TEXTURE_SINK_DRAW_FRAME: Callee = Callee {
        addr: 0x0025_48d0,
        f: ee::s06::VD_TextureSink_DrawFrame,
    };

    /// `PS2_SONY_CODEC_INTERNAL_FreeFrame` (`0x003b08e0`).
    pub(crate) const SONY_CODEC_FREE_FRAME: Callee = Callee {
        addr: 0x003b_08e0,
        f: ee::s13::f_003b08e0,
    };

    /// `MD5_HashString64` (`0x00317690`).
    pub(crate) const MD5_HASH_STRING64: Callee = Callee {
        addr: 0x0031_7690,
        f: ee::s10::f_00317690,
    };

    /// `MD5_HashString` (`0x00317710`).
    pub(crate) const MD5_HASH_STRING: Callee = Callee {
        addr: 0x0031_7710,
        f: ee::s10::f_00317710,
    };

    /// `DLHeap_IAlloc` (`0x0031f858`).
    pub(crate) const DLHEAP_IALLOC: Callee = Callee {
        addr: 0x0031_f858,
        f: ee::s10::f_0031f858,
    };

    /// `MEM_Free` (`0x002523a8`).
    pub(crate) const MEM_FREE: Callee = Callee {
        addr: 0x0025_23a8,
        f: ee::s06::f_002523a8,
    };

    /// `VidEnginePlayer_StaticInit` (`0x00253b58`).
    pub(crate) const VID_ENGINE_PLAYER_STATIC_INIT: Callee = Callee {
        addr: 0x0025_3b58,
        f: ee::s06::VidEnginePlayer_StaticInit,
    };

    /// `VD_StaticInit` (`0x00254e60`).
    pub(crate) const VD_STATIC_INIT: Callee = Callee {
        addr: 0x0025_4e60,
        f: ee::s06::VD_StaticInit,
    };

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

/// `int VidEnginePlayer_Render(...)` (0x253890): run the sink's render
/// method for the player, or nothing when there is no frame.
pub fn vid_engine_player_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
        if c.g[V0][0] != 0u64 {
            c.cycles += 3;
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3c));
            let t = c.g[A2][0] == 0u64;
            c.g[A3][0] = c.g[A2][0];
            if t {
                c.cycles += 1;
                frame.restore(c, 16, 0x10);
            } else {
                c.cycles += 2;
                // Selection: the A1!=0 and S0+8 arms break out past the
                // join straight to the render call; the deeper arms fall
                // into the join below.
                enum Sel {
                    Direct(u64),
                    Join(u64),
                }
                let sel = if c.g[A1][0] != 0u64 {
                    Sel::Direct(c.lw((c.g[A3][0] as u32).wrapping_add(0x4)))
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    if c.g[V0][0] != 0u64 {
                        Sel::Direct(c.lw((c.g[A3][0] as u32).wrapping_add(0x4)))
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                        if c.g[V0][0] != 0u64 {
                            Sel::Join(c.lw(c.g[S0][0] as u32))
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            if c.g[V0][0] != 0u64 {
                                Sel::Direct(c.lw((c.g[A3][0] as u32).wrapping_add(0x4)))
                            } else {
                                c.cycles += 1;
                                Sel::Join(c.lw(c.g[S0][0] as u32))
                            }
                        }
                    }
                };
                match sel {
                    Sel::Direct(v0) => {
                        c.g[V0][0] = v0;
                        c.cycles += 6;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x0025_3904)?;
                        frame.restore(c, 16, 0x10);
                    }
                    Sel::Join(v0) => {
                        c.g[V0][0] = v0;
                        c.cycles += 2;
                        if c.g[V0][0] == 0u64 {
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                            c.cycles += 6;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0025_3920)?;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x40), 0u64);
                            c.cycles += 1;
                            frame.restore(c, 16, 0x10);
                        } else {
                            c.cycles += 1;
                            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                            c.cycles += 6;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0025_3904)?;
                            frame.restore(c, 16, 0x10);
                        }
                    }
                }
            }
        } else {
            frame.restore(c, 16, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int VidEnginePlayer_Reopen(VidEnginePlayer *player, void *params)`
/// (0x253460): close the player, reset the open params, and open it again
/// on `params`.
pub fn vid_engine_player_reopen(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x60);
        frame.save(c, 16, 0x50);
        frame.save(c, 17, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        c.g[S1][0] = c.g[A1][0];
        VIDENGINEPLAYER_CLOSE.call(c, 0x0025_347c)?;
        c.g[A0][0] = c.g[SP][0];
        VIDENGINEPLAYER_INIT_OPEN_PARAMS.call(c, 0x0025_3484)?;
        c.sw(frame.at(c, 0), c.g[S1][0]);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SP][0];
        VIDENGINEPLAYER_OPEN.call(c, 0x0025_3494)?;
        frame.restore(c, 16, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void VidEngine_DebugPrintfStub(...)` (0x253b18): spill the argument
/// registers to stack scratch and return (the debug print never happens).
pub fn vid_engine_debug_printf_stub(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x48), c.g[A1][0]);
        c.sd(frame.at(c, 0x50), c.g[A2][0]);
        c.sd(frame.at(c, 0x58), c.g[A3][0]);
        c.sd(frame.at(c, 0x60), c.g[T0][0]);
        c.sd(frame.at(c, 0x68), c.g[T1][0]);
        c.sd(frame.at(c, 0x70), c.g[T2][0]);
        c.sd(frame.at(c, 0x78), c.g[T3][0]);
        c.swc1(12, frame.at(c, 0x38));
        c.swc1(14, frame.at(c, 0x3c));
        c.swc1(16, frame.at(c, 0x40));
        c.swc1(18, frame.at(c, 0x44));
        frame.close(c);
        Ok(())
    })
}

/// `void VD_DebugPrintfStub(...)` (0x254e20): spill the argument registers
/// to stack scratch and return (the debug print never happens).
pub fn vd_debug_printf_stub(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x48), c.g[A1][0]);
        c.sd(frame.at(c, 0x50), c.g[A2][0]);
        c.sd(frame.at(c, 0x58), c.g[A3][0]);
        c.sd(frame.at(c, 0x60), c.g[T0][0]);
        c.sd(frame.at(c, 0x68), c.g[T1][0]);
        c.sd(frame.at(c, 0x70), c.g[T2][0]);
        c.sd(frame.at(c, 0x78), c.g[T3][0]);
        c.swc1(12, frame.at(c, 0x38));
        c.swc1(14, frame.at(c, 0x3c));
        c.swc1(16, frame.at(c, 0x40));
        c.swc1(18, frame.at(c, 0x44));
        frame.close(c);
        Ok(())
    })
}

/// `void *VD_Alloc(...)` (0x255670): allocate through the video alloc hook
/// (the function pointer at `0x509430`, reached as `*(0x510000 - 0x6bd0)`).
pub fn vd_alloc(c: &mut Cpu) {
    alloc_body(c, 0x0025_56a0);
}

/// `void *VD_Alloc_2(...)` (0x2556b0): allocate through the video alloc
/// hook, as `VD_Alloc` (a second call site).
pub fn vd_alloc_2(c: &mut Cpu) {
    alloc_body(c, 0x0025_56e0);
}

/// Shared body of the two `VD_Alloc` wrappers.
pub(crate) fn alloc_body(c: &mut Cpu, ret: u32) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 0x510000;
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6bd0));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x6bd0));
        c.g[T0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
        c.g[A2][0] = 0u64;
        c.g[A0][0] = 0x4a0000;
        c.g[A3][0] = 0u64;
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2ea0));
        call_ptr(c, t, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void *VD_Free(void *p)` (0x255750): free `p` through the video free hook
/// (the function pointer at `0x509434`, reached as `*(0x510000 - 0x6bcc)`).
pub fn vd_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[V1][0] = 0x510000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x6bcc));
        frame.save_ra(c, 0);
        let t = c.g[V0][0] as u32;
        call_ptr(c, t, 0x0025_5768)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void *VD_Free_2(void *p)` (0x255778): free `p` through the video free
/// hook, as `VD_Free` (a second call site).
pub fn vd_free_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[V1][0] = 0x510000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x6bcc));
        frame.save_ra(c, 0);
        let t = c.g[V0][0] as u32;
        call_ptr(c, t, 0x0025_5790)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void VidEnginePlayer_GlobalCtors(void)` (0x254330): run the player's
/// static initializer with `(1, 0xffff)`, publishing its default tables.
pub fn vid_engine_player_global_ctors(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x1;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        VID_ENGINE_PLAYER_STATIC_INIT.call(c, 0x0025_4344)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void VD_GlobalCtors(void)` (0x2557c0): run the video static initializer
/// with `(1, 0xffff)`, publishing its default tables.
pub fn vd_global_ctors(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x1;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        VD_STATIC_INIT.call(c, 0x0025_57d4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void VidEngine_MemFree(void *p)` (0x253af8): free `p` through `MEM_Free`.
pub fn vid_engine_mem_free(c: &mut Cpu) {
    use callee::*;
    tail_wrapper(c, None, Target::Direct(MEM_FREE), 0x0025_3b08);
}

/// `void VD_TextureSink_Redraw(VD_TextureSink *sink)` (0x254da0): redraw the
/// sink through `VD_TextureSink_DrawFrame`.
pub fn vd_texture_sink_redraw(c: &mut Cpu) {
    use callee::*;
    tail_wrapper(
        c,
        None,
        Target::Direct(VD_TEXTURE_SINK_DRAW_FRAME),
        0x0025_4db0,
    );
}

/// `void VD_TextureSink_SetFrameRate(VD_TextureSink *sink)` (0x254400):
/// store the incoming frame-rate floats (`f12`, `f13`) at `sink + 0x4c`
/// and `sink + 0x50`.
pub fn vd_texture_sink_set_frame_rate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        c.swc1(13, (c.g[A0][0] as u32).wrapping_add(0x50));
        let t = c.g[RA][0] as u32;
        c.swc1(12, (c.g[A0][0] as u32).wrapping_add(0x4c));
        c.pc = t;
        Ok(())
    });
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

/// A gcc 2.9x virtual call on the graphics manager held in register `obj`:
/// the vtable `lw(obj + 0x10d8)` into `vt`, the `this` delta `lh(vt + slot)`
/// into `delta`, the method `lw(vt + slot + 4)` into `func`, then
/// `a0 = obj + delta` and the call returning to `ret` (the generated order).
pub(crate) fn vdts_vcall(
    c: &mut Cpu,
    obj: usize,
    vt: usize,
    delta: usize,
    func: usize,
    slot: u32,
    ret: u32,
) -> crate::call::Flow {
    c.g[vt][0] = c.lw((c.g[obj][0] as u32).wrapping_add(0x10d8));
    c.g[delta][0] = c.lh((c.g[vt][0] as u32).wrapping_add(slot));
    c.g[func][0] = c.lw((c.g[vt][0] as u32).wrapping_add(slot + 4));
    let target = c.g[func][0] as u32;
    c.g[A0][0] = sx((c.g[obj][0] as u32).wrapping_add(c.g[delta][0] as u32));
    call_ptr(c, target, ret)
}

/// gcc's `fd = (float)(unsigned)v0` after a call: a non-negative word
/// converts directly (4 cycles); one with bit 31 set converts
/// `(v0 >> 1) | (v0 & 1)` and doubles it (6 cycles). Both arms leave
/// `v1 = v0 >> 1`.
pub(crate) fn vdts_word_to_float(c: &mut Cpu, fd: usize) {
    let negative = (c.g[V0][0] as i64) < 0;
    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
    if negative {
        c.cycles += 6;
        c.g[V0][0] = c.g[V0][0] & 0x1;
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.mtc1(2, fd);
        c.cvt_s_w(fd, fd);
        c.add_s(fd, fd, fd);
    } else {
        c.cycles += 4;
        c.mtc1(2, fd);
        c.cvt_s_w(fd, fd);
    }
}

/// `VD_TextureSink *VD_TextureSink_ctor(VD_TextureSink *sink, int id,
/// VD_ImageDesc *desc, PS2GraphicsMan *gm, int t0)` (0x254410): build a
/// video texture sink over the image of `desc`: the pixels follow the
/// header (or sit at its `+0x10` offset when flag `0x1000` is set), the
/// height is split into power-of-two strips (count at `+0x20`, heights from
/// `+0x24`), one texture cell per 16-pixel column and strip is created
/// through the graphics manager (`+0x178` slot) and given its filter; then
/// the frame is centred on the screen (`SetFrameSize`) at unit scale
/// (`SetFrameRate`) and `t0` lands at `+0x3c`. Returns `sink`.
///
/// Dispatcher-shaped: the strip loop (0x2544c8), the column loop
/// (0x254580) and the cell loop (0x2545a8) unwind at their back edges and
/// are re-entered at those heads.
pub fn vd_texture_sink_ctor(c: &mut Cpu) {
    use callee::*;
    let frame = Frame { size: 0x100 };
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0025_4410 => {
                    c.cycles += 27;
                    Frame::open(c, 0x100);
                    c.g[V0][0] = 0x480000;
                    frame.save(c, 19, 0xa0);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x270));
                    frame.save(c, 16, 0xd0);
                    c.g[S3][0] = c.g[A0][0];
                    frame.save(c, 17, 0xc0);
                    frame.save(c, 18, 0xb0);
                    frame.save(c, 20, 0x90);
                    frame.save(c, 21, 0x80);
                    frame.save(c, 22, 0x70);
                    frame.save(c, 23, 0x60);
                    frame.save(c, 30, 0x50);
                    frame.save_ra(c, 0x40);
                    frame.save_fpr(c, 22, 0xf0);
                    frame.save_fpr(c, 21, 0xe8);
                    frame.save_fpr(c, 20, 0xe0);
                    c.sw(c.g[S3][0] as u32, c.g[A1][0]);
                    // The vtable (0x480270).
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.sw(frame.at(c, 0x20), c.g[T0][0]);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = c.g[V0][0] & 0x1000;
                    if c.g[V0][0] == 0u64 {
                        // Pixels right after the 16-byte header (beql slot).
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
                    } else {
                        // 0x25447c: pixels at the header's own offset.
                        c.cycles += 2;
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    }
                    // 0x254484: width/16 columns, height into S0.
                    c.cycles += 17;
                    c.sw(frame.at(c, 0x24), c.g[A1][0]);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x6));
                    c.sw(frame.at(c, 0x28), c.g[A0][0]);
                    c.g[S1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x20), 0u64);
                    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[S1][0] as i64)) as u64;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xf));
                    if c.g[V1][0] != 0 {
                        c.g[V0][0] = c.g[S1][0];
                    }
                    c.g[S0][0] = c.lw(frame.at(c, 0x28));
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                    c.g[S6][0] = sx((c.g[S3][0] as u32).wrapping_add(0x24));
                    c.g[S2][0] = 0x510000;
                    c.g[S4][0] = 0x480000;
                    pc = 0x0025_44f0;
                    continue;
                }
                0x0025_44c8 => {
                    // Store the strip `1 << log2(rest)` and take it off.
                    c.cycles += 10;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw(c.g[V0][0] as u32, c.g[V1][0]);
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x20), c.g[V1][0]);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                    pc = 0x0025_44f0;
                    continue;
                }
                0x0025_44f0 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    VD_FLOOR_LOG2.call(c, 0x0025_44f8)?;
                    c.g[V1][0] = 0x1;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << ((c.g[V0][0] as u32) & 31));
                    let t = c.g[S0][0] != c.g[V1][0];
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x0025_44c8) {
                            return Ok(());
                        }
                        pc = 0x0025_44c8;
                        continue;
                    }
                    // 0x254508: the last strip (a power of two), then the
                    // cell-pointer array (cells * 12 bytes).
                    c.cycles += 28;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw(c.g[V0][0] as u32, c.g[S0][0]);
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x20));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x20), c.g[V1][0]);
                    c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = 0xc;
                    c.g[A2][0] = 0u64;
                    c.mult(0, 5, 5, 3);
                    c.lwc1(21, frame.at(c, 0x28));
                    c.cvt_s_w(21, 21);
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x6bd0));
                    c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x258));
                    c.g[A3][0] = 0u64;
                    c.mtc1(17, 22);
                    c.cvt_s_w(22, 22);
                    c.g[FP][0] = 0u64;
                    c.mult(0, 0, 5, 2);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x18), c.g[A1][0]);
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x6bd0));
                    c.g[A1][0] = c.lo[0];
                    let t = c.g[V0][0] as u32;
                    c.g[T0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    call_ptr(c, t, 0x0025_4568)?;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c));
                    c.g[A0][0] = 0u64;
                    let t = (c.g[V1][0] as i64) <= 0;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                    if t {
                        pc = 0x0025_46a8;
                        continue;
                    }
                    // 0x254578.
                    c.cycles += 2;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x20));
                    pc = 0x0025_4580;
                    continue;
                }
                0x0025_4580 => {
                    // One 16-pixel column (index in A0).
                    c.cycles += 5;
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.g[S7][0] = 0u64;
                    c.g[S5][0] = 0u64;
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.sw(frame.at(c, 0x30), c.g[V1][0]);
                    if t {
                        pc = 0x0025_4694;
                        continue;
                    }
                    // 0x254594.
                    c.cycles += 5;
                    c.g[V0][0] = sx((c.g[A0][0] as u32) << 6);
                    c.g[S4][0] = c.g[S6][0];
                    c.g[A0][0] = c.lw(frame.at(c, 0x28));
                    c.mult(0, 4, 4, 2);
                    c.sw(frame.at(c, 0x2c), c.g[A0][0]);
                    pc = 0x0025_45a8;
                    continue;
                }
                0x0025_45a8 => {
                    // One cell: a 12-byte record, its texture, its filter.
                    c.cycles += 59;
                    c.g[A1][0] = 0x510000;
                    c.g[V0][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6bd0));
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2ea0));
                    c.g[T0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[S2][0] = sx((c.g[FP][0] as u32) << 2);
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                    c.g[A1][0] = 0x510000;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0x6bd0));
                    c.g[A2][0] = 0u64;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A1][0] = 0xc;
                    c.g[A3][0] = 0u64;
                    let t = c.g[V0][0] as u32;
                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                    call_ptr(c, t, 0x0025_45e4)?;
                    c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x1));
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V1][0] = c.lw(c.g[S4][0] as u32);
                    c.g[V0][0] = 0x10;
                    c.g[A1][0] = sx((c.g[S7][0] as u32) << 6);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.g[T2][0] = 0u64;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                    c.g[V1][0] = c.lw(frame.at(c, 0x2c));
                    c.g[T3][0] = 0u64;
                    c.g[S1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
                    c.g[T1][0] = 0u64;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A0][0] = c.lw(frame.at(c, 0x24));
                    c.g[V1][0] = 0x4a0000;
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2ea8));
                    c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10d8));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x178));
                    c.sw(frame.at(c, 0x18), c.g[V0][0]);
                    c.sw(frame.at(c, 0), 0u64);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.sw(frame.at(c, 0x8), 0u64);
                    c.sw(frame.at(c, 0x10), 0u64);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x17c));
                    c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    let t = c.g[V0][0] as u32;
                    c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    call_ptr(c, t, 0x0025_4658)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = 0u64;
                    c.g[T0][0] = 0u64;
                    c.g[T1][0] = 0u64;
                    call_ptr(c, PS2_GRAPHICS_MAN_SET_TEXTURE_FILTER, 0x0025_4678)?;
                    c.sw(c.g[S2][0] as u32, c.g[S0][0]);
                    c.g[V1][0] = c.lw(c.g[S4][0] as u32);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x20));
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = ((c.g[S5][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    if t {
                        if back_edge_unwinds(c, 0x0025_45a8) {
                            return Ok(());
                        }
                        pc = 0x0025_45a8;
                        continue;
                    }
                    pc = 0x0025_4694;
                    continue;
                }
                0x0025_4694 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c));
                    c.g[A0][0] = c.lw(frame.at(c, 0x30));
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x20));
                        if back_edge_unwinds(c, 0x0025_4580) {
                            return Ok(());
                        }
                        pc = 0x0025_4580;
                        continue;
                    }
                    pc = 0x0025_46a8;
                    continue;
                }
                0x0025_46a8 => {
                    // Screen width (slot 0x50) and height (slot 0x58) as
                    // unsigned floats.
                    c.cycles += 9;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0xc), 0u64);
                    c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    vdts_vcall(c, A1, V1, A0, V0, 0x50, 0x0025_46c4)?;
                    vdts_word_to_float(c, 20);
                    c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.cycles += 7;
                    vdts_vcall(c, A1, V1, A0, V0, 0x58, 0x0025_4708)?;
                    vdts_word_to_float(c, 13);
                    c.sub_s(13, 13, 21);
                    // 0x254738: centre the frame, unit scale, keep `t0`.
                    c.cycles += 32;
                    c.g[AT][0] = 0x3f000000;
                    c.mtc1(1, 0);
                    c.sub_s(12, 20, 22);
                    c.g[A0][0] = 0x4a0000;
                    c.lwc1(14, (c.g[A0][0] as u32).wrapping_add(0x2eb0));
                    c.mul_s(13, 13, 0);
                    c.g[A0][0] = c.g[S3][0];
                    c.mul_s(12, 12, 0);
                    VD_TEXTURE_SINK_SET_FRAME_SIZE.call(c, 0x0025_475c)?;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 12);
                    c.g[A0][0] = c.g[S3][0];
                    c.mov_s(13, 12);
                    VD_TEXTURE_SINK_SET_FRAME_RATE.call(c, 0x0025_4770)?;
                    c.g[A1][0] = c.lw(frame.at(c, 0x20));
                    c.g[V0][0] = c.g[S3][0];
                    frame.restore(c, 16, 0xd0);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x3c), c.g[A1][0]);
                    frame.restore(c, 17, 0xc0);
                    frame.restore(c, 18, 0xb0);
                    frame.restore(c, 19, 0xa0);
                    frame.restore(c, 20, 0x90);
                    frame.restore(c, 21, 0x80);
                    frame.restore(c, 22, 0x70);
                    frame.restore(c, 23, 0x60);
                    frame.restore(c, 30, 0x50);
                    frame.restore_ra(c, 0x40);
                    frame.restore_fpr(c, 22, 0xf0);
                    frame.restore_fpr(c, 21, 0xe8);
                    frame.restore_fpr(c, 20, 0xe0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0025_4410, 0x0025_47b8);
                    return Ok(());
                }
            }
        }
    })
}

/// `void VD_TextureSink_DrawFrame(VD_TextureSink *sink)` (0x2548d0): wait
/// until the graphics manager is idle (slot `0x88`), set a full-screen
/// viewport (slot `0xd0`, the screen size from slots `0x50`/`0x58` as
/// unsigned floats) and the render state (slot `0x110` with `0x4ff1a0`),
/// put the sink's 5-bit layer `+0x3c` into the top render state, then draw
/// one textured quad per cell (slot `0x238`), column strip by column strip
/// from `(x, y, z) = (+0x40, +0x44, +0x48)`, cell width `+0x4c * 16`, row
/// heights `+0x24[i] * +0x50`.
pub fn vd_texture_sink_draw_frame(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x170);
        frame.save(c, 16, 0x120);
        frame.save(c, 17, 0x110);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 18, 0x100);
        frame.save(c, 19, 0xf0);
        frame.save(c, 20, 0xe0);
        frame.save_ra(c, 0xd0);
        frame.save_fpr(c, 27, 0x168);
        frame.save_fpr(c, 26, 0x160);
        frame.save_fpr(c, 25, 0x158);
        frame.save_fpr(c, 24, 0x150);
        frame.save_fpr(c, 23, 0x148);
        frame.save_fpr(c, 22, 0x140);
        frame.save_fpr(c, 21, 0x138);
        frame.save_fpr(c, 20, 0x130);
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        // 0x254918: spin until the renderer is idle.
        loop {
            c.cycles += 7;
            vdts_vcall(c, A1, V1, A0, V0, 0x88, 0x0025_492c)?;
            let busy = c.g[V0][0] == 0u64;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
            if !busy {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0025_4918);
            }
        }
        // Screen width into f20, height into f15.
        c.cycles += 7;
        vdts_vcall(c, A1, V1, A0, V0, 0x50, 0x0025_4948)?;
        vdts_word_to_float(c, 20);
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.cycles += 7;
        vdts_vcall(c, A1, V1, A0, V0, 0x58, 0x0025_498c)?;
        vdts_word_to_float(c, 15);
        c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        // 0x2549bc: viewport (0, 0, 0.0, 0.0, w, h, 0.0, 1.0).
        c.cycles += 33;
        c.g[A1][0] = 0u64;
        c.mtc1(0, 12);
        c.g[A2][0] = 0u64;
        c.mov_s(14, 20);
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 17);
        c.mov_s(13, 12);
        c.mov_s(16, 12);
        vdts_vcall(c, A3, V1, A0, V0, 0xd0, 0x0025_49f0)?;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = 0x500000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xe60));
        vdts_vcall(c, A2, V0, A0, V1, 0x110, 0x0025_4a10)?;
        // The top render state's layer bits (`+0x8`, bits 5..9).
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = 0xffff_ffff_ffff_fc1f;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3c));
        c.g[A3][0] = 0xffff_ffff_ffff_ffff;
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xe84));
        c.g[A2][0] = 0x3;
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = c.g[V1][0] & 0x3e0;
        c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        // 0x254a40: the compiler's empty 4-step count-down.
        loop {
            c.cycles += 7;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
            let t = c.g[A2][0] != c.g[A3][0];
            c.g[A0][0] = 0x80;
            if !t {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0025_4a40);
            }
        }
        // Four vertices on the stack: colour 0x80 x4, q = 1.0.
        c.cycles += 5;
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.g[V1][0] = c.g[SP][0];
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
        loop {
            c.cycles += 8;
            c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A0][0]);
            c.sw((c.g[V1][0] as u32).wrapping_add(0x14), c.g[A0][0]);
            c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[A0][0]);
            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
            if c.g[V0][0] == 0u64 {
                break;
            }
            c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
            if event_due(c) {
                deliver_event_here(c, 0x0025_4a70);
            }
        }
        // 0x254a90: uv corners, x0 = x + 0.5, cell width = cw * 16.
        c.cycles += 23;
        c.lwc1(3, (c.g[S0][0] as u32).wrapping_add(0x4c));
        c.g[S3][0] = 0u64;
        c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(0x40));
        c.g[V1][0] = 0u64;
        c.g[AT][0] = 0x41800000;
        c.mtc1(1, 1);
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 0);
        c.mul_s(24, 3, 1);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.add_s(20, 2, 0);
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 1);
        c.sw(frame.at(c, 0), 0u64);
        c.swc1(1, frame.at(c, 0x94));
        c.sw(frame.at(c, 0x4), 0u64);
        c.add_s(21, 20, 24);
        c.swc1(1, frame.at(c, 0x30));
        c.sw(frame.at(c, 0x34), 0u64);
        c.sw(frame.at(c, 0x60), 0u64);
        c.swc1(1, frame.at(c, 0x64));
        let no_rows = (c.g[V0][0] as i64) <= 0;
        c.swc1(1, frame.at(c, 0x90));
        if !no_rows {
            c.cycles += 1;
            // 0x254af0: one column strip per `+0x1c` count.
            loop {
                'cells: {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                    c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.mtc1(0, 23);
                    c.g[S2][0] = 0u64;
                    c.add_s(25, 20, 24);
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.add_s(26, 21, 24);
                    if t {
                        break 'cells;
                    }
                    c.cycles += 7;
                    c.g[AT][0] = 0x3f000000;
                    c.mtc1(1, 27);
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24));
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 22);
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                    // 0x254b28: one quad per `+0x20` count.
                    loop {
                        c.cycles += 49;
                        c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
                        c.lwc1(2, c.g[S1][0] as u32);
                        c.cvt_s_w(2, 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x44));
                        c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                        c.g[A1][0] = 0x4;
                        c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
                        c.add_s(0, 0, 27);
                        c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                        c.g[A2][0] = c.g[SP][0];
                        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x50));
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        c.sh((c.g[A3][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                        c.add_s(0, 0, 23);
                        c.swc1(20, frame.at(c, 0xc0));
                        c.mul_s(2, 2, 1);
                        c.g[A3][0] = 0u64;
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x48));
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                        c.swc1(0, frame.at(c, 0xc4));
                        c.add_s(3, 0, 2);
                        c.swc1(1, frame.at(c, 0xc8));
                        c.add_s(23, 23, 2);
                        c.swc1(22, frame.at(c, 0xcc));
                        c.lq(2, frame.at(c, 0xc0));
                        c.swc1(21, frame.at(c, 0xc0));
                        c.sq(2, frame.at(c, 0x20));
                        c.lq(3, frame.at(c, 0xc0));
                        c.swc1(20, frame.at(c, 0xc0));
                        c.swc1(3, frame.at(c, 0xc4));
                        c.sq(3, frame.at(c, 0x50));
                        c.lq(4, frame.at(c, 0xc0));
                        c.swc1(21, frame.at(c, 0xc0));
                        c.sq(4, frame.at(c, 0x80));
                        c.lq(2, frame.at(c, 0xc0));
                        c.sq(2, frame.at(c, 0xb0));
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        vdts_vcall(c, A0, V0, V1, T0, 0x238, 0x0025_4bdc)?;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        if c.g[V0][0] == 0u64 {
                            break 'cells;
                        }
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                        if event_due(c) {
                            deliver_event_here(c, 0x0025_4b28);
                        }
                    }
                }
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                c.g[V1][0] = c.g[S4][0];
                c.mov_s(20, 25);
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0u64;
                c.mov_s(21, 26);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0025_4af0);
                }
            }
        }
        c.cycles += 16;
        frame.restore(c, 16, 0x120);
        frame.restore(c, 17, 0x110);
        frame.restore(c, 18, 0x100);
        frame.restore(c, 19, 0xf0);
        frame.restore(c, 20, 0xe0);
        frame.restore_ra(c, 0xd0);
        frame.restore_fpr(c, 27, 0x168);
        frame.restore_fpr(c, 26, 0x160);
        frame.restore_fpr(c, 25, 0x158);
        frame.restore_fpr(c, 24, 0x150);
        frame.restore_fpr(c, 23, 0x148);
        frame.restore_fpr(c, 22, 0x140);
        frame.restore_fpr(c, 21, 0x138);
        frame.restore_fpr(c, 20, 0x130);
        frame.close(c);
        Ok(())
    })
}

/// `int VD_TextureSink_UploadFrame(...)` (0x254c48): upload the frame rows
/// through the sink's method table, then redraw through
/// `VD_TextureSink_DrawFrame`.
pub fn vd_texture_sink_upload_frame(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0025_4c48 => {
                    c.cycles += 13;
                    let frame = Frame::open(c, 0xb0);
                    frame.save(c, 16, 0xa0);
                    frame.save(c, 17, 0x90);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save(c, 18, 0x80);
                    frame.save(c, 19, 0x70);
                    frame.save(c, 20, 0x60);
                    frame.save(c, 21, 0x50);
                    frame.save(c, 22, 0x40);
                    frame.save(c, 23, 0x30);
                    frame.save(c, 30, 0x20);
                    let t = c.g[A1][0] == 0u64;
                    frame.save_ra(c, 0x10);
                    if !t {
                        c.cycles += 1;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[A1][0]);
                    }
                    pc = 0x0025_4ca0;
                    continue;
                }
                0x0025_4ca0 => {
                    c.cycles += 6;
                    c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = c.g[V0][0] & 0x1000;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S7][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
                    if !t {
                        c.cycles += 2;
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
                        c.g[S7][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    }
                    pc = 0x0025_4d68;
                    continue;
                }
                0x0025_4d68 => {
                    c.cycles += 6;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[S5][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                    c.g[A0][0] = 0u64;
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[FP][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x6));
                    if t {
                        pc = 0x0025_4d70;
                        continue;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                    pc = 0x0025_4cc0;
                    continue;
                }
                0x0025_4cc0 => {
                    c.cycles += 4;
                    c.g[S3][0] = 0u64;
                    c.g[S2][0] = 0u64;
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[S6][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x0025_4d54;
                        continue;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[A0][0] as u32) << 6);
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24));
                    c.mult(0, 20, 30, 2);
                    pc = 0x0025_4ce0;
                    continue;
                }
                0x0025_4ce0 => {
                    c.cycles += 29;
                    c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
                    c.g[V0][0] = sx((c.g[S5][0] as u32) << 2);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                    c.g[A2][0] = sx((c.g[S3][0] as u32) << 6);
                    c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10d8));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[S4][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A2][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[T1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x188));
                    c.g[T2][0] = 0u64;
                    c.sw(c.g[SP][0] as u32, 0u64);
                    c.g[T3][0] = 0u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18c));
                    c.g[T0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[V0][0] as u32;
                    c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    call_ptr(c, t, 0x0025_4d3c)?;
                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0025_4ce0);
                        }
                        pc = 0x0025_4ce0;
                        continue;
                    }
                    pc = 0x0025_4d54;
                    continue;
                }
                0x0025_4d54 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                    c.g[A0][0] = c.g[S6][0];
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                        if event_due(c) {
                            deliver_event_here(c, 0x0025_4cc0);
                        }
                        pc = 0x0025_4cc0;
                        continue;
                    }
                    pc = 0x0025_4d70;
                    continue;
                }
                0x0025_4d70 => {
                    c.cycles += 14;
                    c.g[A0][0] = c.g[S0][0];
                    VD_TEXTURE_SINK_DRAW_FRAME.call(c, 0x0025_4d70)?;
                    let frame = Frame { size: 0xb0 };
                    frame.restore(c, 16, 0xa0);
                    frame.restore(c, 17, 0x90);
                    frame.restore(c, 18, 0x80);
                    frame.restore(c, 19, 0x70);
                    frame.restore(c, 20, 0x60);
                    frame.restore(c, 21, 0x50);
                    frame.restore(c, 22, 0x40);
                    frame.restore(c, 23, 0x30);
                    frame.restore(c, 30, 0x20);
                    frame.restore_ra(c, 0x10);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0025_4c48, 0x0025_4da0);
                    return Ok(());
                }
            }
        }
    })
}

/// `int VidEnginePlayer_Open(VidEnginePlayer *player, void *params)`
/// (0x2534a8): close the player, copy 32 param bytes, and open again.
pub fn vid_engine_player_open(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 17, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 16, 0x40);
        frame.save(c, 18, 0x20);
        frame.save(c, 19, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = 0x1;
        VIDENGINEPLAYER_CLOSE.call(c, 0x0025_34d0)?;
        c.g[S2][0] = c.g[S3][0];
        c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x7));
        c.ldr(2, c.g[S0][0] as u32);
        c.ldl(3, (c.g[S0][0] as u32).wrapping_add(0xf));
        c.ldr(3, (c.g[S0][0] as u32).wrapping_add(0x8));
        c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x17));
        c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x10));
        c.ldl(5, (c.g[S0][0] as u32).wrapping_add(0x1f));
        c.ldr(5, (c.g[S0][0] as u32).wrapping_add(0x18));
        c.sdl(2, (c.g[S1][0] as u32).wrapping_add(0x1b));
        c.sdr(2, (c.g[S1][0] as u32).wrapping_add(0x14));
        c.sdl(3, (c.g[S1][0] as u32).wrapping_add(0x23));
        c.sdr(3, (c.g[S1][0] as u32).wrapping_add(0x1c));
        c.sdl(4, (c.g[S1][0] as u32).wrapping_add(0x2b));
        c.sdr(4, (c.g[S1][0] as u32).wrapping_add(0x24));
        c.sdl(5, (c.g[S1][0] as u32).wrapping_add(0x33));
        c.sdr(5, (c.g[S1][0] as u32).wrapping_add(0x2c));
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x34), c.g[V0][0]);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x34));
        if c.g[V0][0] == 0 {
            c.g[S2][0] = 0u64;
        }
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        if c.g[V1][0] != 0 {
            c.g[S3][0] = 0u64;
        }
        let t = c.g[A0][0] == 0u64;
        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x10));
        let opened = if t {
            false
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lb(c.g[A0][0] as u32);
            let t = c.g[V0][0] == 0u64;
            c.g[V1][0] = 0x510000;
            if t {
                false
            } else {
                c.cycles += 19;
                c.g[A0][0] = 0x4a0000;
                c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6bd0));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x6bd0));
                c.g[T0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2e98));
                c.g[A1][0] = 0x78;
                c.g[A2][0] = 0u64;
                let t = c.g[V0][0] as u32;
                c.g[A3][0] = 0u64;
                call_ptr(c, t, 0x0025_356c)?;
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = c.g[V0][0];
                c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                c.g[T1][0] = c.g[S2][0];
                c.g[T2][0] = c.g[S3][0];
                c.g[A2][0] = 0x60000;
                c.g[T0][0] = 0x20000;
                call_ptr(c, 0x003a_de08, 0x0025_358c)?;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x38), c.g[V0][0]);
                true
            }
        };
        if !opened {
            c.cycles += 17;
            c.g[V1][0] = 0x510000;
            c.g[A0][0] = 0x4a0000;
            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6bd0));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x6bd0));
            c.g[T0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2e98));
            c.g[A1][0] = 0x78;
            c.g[A2][0] = 0u64;
            let t = c.g[V0][0] as u32;
            c.g[A3][0] = 0u64;
            call_ptr(c, t, 0x0025_35bc)?;
            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
            c.g[A0][0] = c.g[V0][0];
            c.g[A3][0] = c.g[S2][0];
            c.g[T0][0] = c.g[S3][0];
            c.g[A2][0] = 0x80000;
            call_ptr(c, 0x003a_ddc0, 0x0025_35d4)?;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x38), c.g[V0][0]);
        }
        c.cycles += 8;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[V0][0] = 0x1;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
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
            return Ok(c_dsock_channel_update_resume(c));
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

/// Native resume of `c_dsock_channel_update` (converted from the `entries` body): any entry
/// other than 0x00256a00, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn c_dsock_channel_update_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00256a00 => {
                    c.cycles += 7;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                    c.g[A1][0] = c.g[A0][0];
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = (c.g[V1][0] < 0x5u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x00256a4c;
                        continue;
                    }
                    pc = 0x00256a1c;
                    continue;
                }
                0x00256a1c => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x460));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x00256a34 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00256a3c);
                    c.g[A0][0] = c.g[A1][0];
                    Callee {
                        addr: 0x00256a58,
                        f: ee::s06::cDSockChannel_UpdateConnecting,
                    }
                    .call(c, 0x00256a3c)?;
                    let t = true;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    if t {
                        pc = 0x00256a50;
                        continue;
                    }
                    pc = 0x00256a44;
                    continue;
                }
                0x00256a44 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x00256a4c);
                    c.g[A0][0] = c.g[A1][0];
                    Callee {
                        addr: 0x00256aa8,
                        f: ee::s06::cDSockChannel_UpdateConnected,
                    }
                    .call(c, 0x00256a4c)?;
                    pc = 0x00256a4c;
                    continue;
                }
                0x00256a4c => {
                    c.cycles += 1;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    pc = 0x00256a50;
                    continue;
                }
                0x00256a50 => {
                    c.cycles += 2;
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00256a00, 0x00256a58);
                    return Ok(());
                }
            }
        }
    });
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

    use crate::containers::tests::free_target;
    use crate::containers::tests::mutex_state;
    use crate::containers::MEM_MUTEX;
    use crate::difftest::check_paths;
    use crate::difftest::elf_image;

    use crate::difftest::fakes::LOG_RESULTS;

    use crate::difftest::require_paths;

    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    // ---------- VD_TextureSink (ctor, DrawFrame) ----------

    /// The `miss` hook at a fixed 4-cycle cost (the fake's hash-based cost
    /// would make path totals seed-varying).
    pub(crate) fn vdts_norm_miss(c: &mut Cpu, target: u32) {
        let before = c.cycles;
        fakes::fake_call(c, target);
        c.cycles = before + 4;
    }

    pub(crate) const VDTS_SINK: u32 = DATA + 0x5_0000;

    pub(crate) const VDTS_DESC: u32 = DATA + 0x5_0200;

    pub(crate) const VDTS_HDR: u32 = DATA + 0x5_0300;

    pub(crate) const VDTS_GM: u32 = DATA + 0x5_1000;

    pub(crate) const VDTS_VT: u32 = DATA + 0x5_8000;

    pub(crate) const VDTS_MODES: u32 = DATA + 0x5_9000;

    pub(crate) const VDTS_TEXMAN: u32 = DATA + 0x5_a000;

    pub(crate) const VDTS_TEXREC: u32 = DATA + 0x5_b000;

    pub(crate) const VDTS_ITEMS: u32 = DATA + 0x5_c000;

    pub(crate) const VDTS_CELLS: u32 = DATA + 0x5_c100;

    pub(crate) const VDTS_SCRATCH: u32 = DATA + 0x5_d000;

    /// `this` delta of the real-method slots (0x50, 0x58, 0x88).
    pub(crate) const VDTS_GETTER_DELTA: u32 = 0x20;

    /// Gate of the `IsRenderIdle` fake (`DrawFrame`'s wait loop) and the
    /// number of "busy" answers it gives before "idle".
    pub(crate) const VDTS_DRAW_CTRL: u32 = DATA + 0x7_f000;

    pub(crate) const VDTS_DRAW_MAGIC: u32 = 0x1d1e_5eed;

    pub(crate) const VDTS_DRAW_BUSY: u32 = DATA + 0x7_f004;

    pub(crate) static VDTS_DRAW_PREV_IDLE: AtomicUsize = AtomicUsize::new(0);

    /// `PS2GraphicsMan_IsRenderIdle` (0x382730) with its gate on: logs the
    /// call, costs 1000 cycles (so the retry shows in the path total), and
    /// answers 0 while the busy count lasts, then 1.
    pub(crate) fn vdts_draw_fake_idle(c: &mut Cpu) {
        if c.bus.read32(VDTS_DRAW_CTRL) != VDTS_DRAW_MAGIC {
            return fakes::fall_back(c, &VDTS_DRAW_PREV_IDLE, ee::s12::f_00382730__generated);
        }
        let before = c.cycles;
        fakes::fake_call(c, 0x0038_2730);
        c.cycles = before + 1000;
        let busy = c.bus.read32(VDTS_DRAW_BUSY);
        if busy != 0 {
            c.bus.write32(VDTS_DRAW_BUSY, busy - 1);
        }
        c.g[V0][0] = (busy == 0) as u64;
    }

    pub(crate) fn vdts_draw_hook() {
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_00382730),
            vdts_draw_fake_idle,
            &VDTS_DRAW_PREV_IDLE,
        );
    }

    /// The graphics manager both sink functions talk to (also at
    /// `gp + 0x2a90`): every vtable slot is `FAKE` except the real screen
    /// getters (`GetScreenWidth`/`Height`, mode record 1 holding `width` /
    /// `height`) and `IsRenderIdle`, all with a `this` delta; a texture
    /// manager whose 16 slots point at distinctive records; the render-state
    /// scratch at `+0xe84`.
    pub(crate) fn vdts_graphics_man(c: &mut Cpu, seed: u32, width: u32, height: u32) {
        for i in 0..0x40u32 {
            c.bus.write32(VDTS_GM + 4 * i, 0x3333_0000 + i);
        }
        c.bus.write32(VDTS_GM + 0x10d8, VDTS_VT);
        c.bus.write32(VDTS_GM + 0x18f4, VDTS_TEXMAN);
        c.bus.write32(VDTS_GM + 0xe84, VDTS_SCRATCH);
        let this = VDTS_GM + VDTS_GETTER_DELTA;
        c.bus.write32(this + 0x59c8, 1);
        c.bus.write32(this + 0x59cc, VDTS_MODES);
        c.bus.write32(VDTS_MODES + 0xa4 + 0x88, width);
        c.bus.write32(VDTS_MODES + 0xa4 + 0x8c, height);
        for i in 0..0xa0u32 {
            c.bus.write32(VDTS_VT + 4 * i, FAKE);
        }
        for (slot, f) in [
            (0x50u32, 0x0039_5330u32),
            (0x58, 0x0039_5350),
            (0x88, 0x0038_2730),
        ] {
            c.bus.write16(VDTS_VT + slot, VDTS_GETTER_DELTA as u16);
            c.bus.write32(VDTS_VT + slot + 4, f);
        }
        for i in 0..16u32 {
            c.bus
                .write32(VDTS_TEXMAN + 8 + 4 * i, VDTS_TEXREC + 0x80 * i);
            c.bus.write64(
                VDTS_TEXREC + 0x80 * i + 0x40,
                0x1234_5678_9abc_def0 ^ ((i as u64) << 8) ^ seed as u64,
            );
        }
        for i in 0..8u32 {
            c.bus.write32(VDTS_SCRATCH + 4 * i, 0x6666_0000 + seed + i);
        }
        c.bus.write32(GAME_GP + 0x2a90, VDTS_GM);
    }

    /// The screen size word for a seed bit: positive, or with bit 31 set
    /// (gcc's unsigned-to-float arm).
    pub(crate) fn vdts_screen_word(base: u32, negative: bool) -> u32 {
        if negative {
            0x8000_0000 | base
        } else {
            base
        }
    }

    /// `VD_TextureSink_ctor` fixture. Seed bits: 0 = header flag 0x1000
    /// (0x25447c arm), 1 = screen width bit 31 (0x2546dc), 2 = screen height
    /// bit 31 (0x254720); `seed / 8 % 9` picks (width, height) of the image:
    /// strips p = popcount(height), columns = width / 16 (<= 0 skips to
    /// 0x2546a8; the last shape is a negative width, the `movn` rounding).
    /// At most 3 cells, so every fake result is forced: call 0 is the cell
    /// array, odd calls cell records, even calls texture slots 3/5/7.
    pub(crate) fn vdts_ctor_fixture(c: &mut Cpu, seed: u32) {
        const SHAPES: [(i16, i16); 9] = [
            (8, 4),
            (8, 6),
            (8, 7),
            (16, 4),
            (20, 6),
            (31, 7),
            (32, 2),
            (48, 1),
            (-20, 5),
        ];
        let flag = seed % 2 == 1;
        let (w, h) = SHAPES[(seed / 8) as usize % SHAPES.len()];
        vdts_graphics_man(
            c,
            seed,
            vdts_screen_word(0x280 + seed, (seed / 2) % 2 == 1),
            vdts_screen_word(0x1c0 + seed, (seed / 4) % 2 == 1),
        );
        for i in 0..0x20u32 {
            c.bus.write32(VDTS_SINK + 4 * i, 0x2222_0000 + i);
        }
        c.bus.write32(VDTS_DESC, 0x4242_0000 + seed);
        c.bus.write32(VDTS_DESC + 4, VDTS_HDR);
        for i in 0..8u32 {
            c.bus.write32(VDTS_HDR + 4 * i, 0x5555_0000 + i);
        }
        c.bus.write16(VDTS_HDR + 4, w as u16);
        c.bus.write16(VDTS_HDR + 6, h as u16);
        c.bus
            .write32(VDTS_HDR + 0xc, if flag { 0x0000_1003 } else { 0x0000_0e03 });
        c.bus.write32(VDTS_HDR + 0x10, 0x40 + 0x10 * (seed % 5));
        // The video allocator hook (alloc through the fake) and its flags.
        c.bus.write32(0x0050_9430, FAKE);
        c.bus.write32(0x0050_9438, 0x0bad_0000 | seed);
        c.bus.write32(LOG_RESULTS, VDTS_ITEMS);
        for k in 0..3u32 {
            c.bus
                .write32(LOG_RESULTS + 4 * (1 + 2 * k), VDTS_CELLS + 0x20 * k);
            c.bus.write32(LOG_RESULTS + 4 * (2 + 2 * k), 3 + 2 * k);
        }
        c.bus.write32(LOG_RESULTS + 4 * 7, VDTS_CELLS + 0x60);
        for i in 0..0x20u32 {
            c.bus.write32(VDTS_ITEMS + 4 * i, 0x7a7a_0000 + i);
            c.bus.write32(VDTS_CELLS + 4 * i, 0x7b7b_0000 + i);
        }
        c.g[A0][0] = VDTS_SINK as u64;
        c.g[A1][0] = 0x1111_0000 + seed as u64;
        c.g[A2][0] = VDTS_DESC as u64;
        c.g[A3][0] = VDTS_GM as u64;
        c.g[T0][0] = 0x7777_0000 + seed as u64;
        c.pc = 0x0025_4410;
    }

    /// `VD_TextureSink_ctor`: every strip / column / flag / sign arm, with
    /// the real `VD_FloorLog2`, `SetTextureFilter` (+ `TexMan_SetTex1Filter`),
    /// screen getters, `SetFrameSize` / `SetFrameRate`; allocations and
    /// texture creation through the fake. Path totals (f = flag, n = negative
    /// getter results, p = strips, k = columns):
    /// `143 + 22p + 2f + 2n + [k > 0](2 + k(15 + 105p))`.
    #[test]
    fn vd_texture_sink_ctor_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 0x0025_4410, 0);
            elf_image(c);
            c.hooks.miss = vdts_norm_miss;
            vdts_ctor_fixture(c, seed);
        };
        check_paths(
            ee::s06::f_00254410__generated,
            vd_texture_sink_ctor,
            72,
            // 165 p=1 k=0 (all non-negative, flag clear); 171 = f=1 n=2 (seed 7:
            // 0x25447c + 0x2546dc + 0x254720); 187 p=2; 209 p=3; 287 p=1 k=1;
            // 414 p=2 k=1; 541 p=3 k=1; 407 p=1 k=2; 527 p=1 k=3.
            &[165, 171, 187, 209, 287, 407, 414, 527, 541],
            &setup,
        );
        fakes::covered(
            ee::s06::f_00254410__generated,
            vd_texture_sink_ctor,
            72,
            &|c, seed| {
                machine(c, seed, 0x0025_4411, 8);
                elf_image(c);
                c.hooks.miss = vdts_norm_miss;
                vdts_ctor_fixture(c, seed);
            },
        );
    }

    driven!(vdts_ctor_gen, ee::s06::f_00254410__generated);

    driven!(vdts_ctor_hand, vd_texture_sink_ctor);

    /// The ctor's three back edges (0x2544c8 strips, 0x2545a8 cells,
    /// 0x254580 columns) under delivered and unwinding events: shapes with
    /// p >= 2 or more than one column only.
    #[test]
    fn vd_texture_sink_ctor_events() {
        check_events(vdts_ctor_gen, vdts_ctor_hand, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x0025_4412, 0);
            elf_image(c);
            c.hooks.miss = vdts_norm_miss;
            // Shapes 4..7 of the fixture (seed / 8 % 9 in 4..=7).
            let shape_seed = (4 + seed % 4) * 8 + (seed / 4) % 8;
            vdts_ctor_fixture(c, shape_seed);
            schedule(c, &mut r, seed, 0x0025_4410)
        });
    }

    /// `VD_TextureSink_DrawFrame` fixture. Seed bits: 0 = one "busy" answer
    /// before idle (the 0x254918 retry), 1 = screen width bit 31 (0x254960
    /// arm), 2 = screen height bit 31 (0x2549a4 arm); `seed / 8 % 4` rows
    /// (`+0x1c`, 0 skips 0x254af0), `seed / 32 % 3` cells per row (`+0x20`,
    /// 0 skips 0x254b28).
    pub(crate) fn vdts_draw_fixture(c: &mut Cpu, seed: u32) {
        let rows = (seed / 8) % 4;
        let cols = (seed / 32) % 3;
        vdts_graphics_man(
            c,
            seed,
            vdts_screen_word(0x280 + seed, (seed / 2) % 2 == 1),
            vdts_screen_word(0x1c0 + seed, (seed / 4) % 2 == 1),
        );
        for i in 0..0x20u32 {
            c.bus.write32(VDTS_SINK + 4 * i, 0x2222_0000 + i);
        }
        c.bus.write32(VDTS_SINK + 8, VDTS_GM);
        c.bus.write32(VDTS_SINK + 0x14, VDTS_ITEMS);
        c.bus.write32(VDTS_SINK + 0x1c, rows);
        c.bus.write32(VDTS_SINK + 0x20, cols);
        for i in 0..3u32 {
            c.bus.write32(VDTS_SINK + 0x24 + 4 * i, 2 + i + seed % 3);
        }
        c.bus.write32(VDTS_SINK + 0x3c, 0x4444_0000 + seed * 7);
        c.bus.write32(VDTS_SINK + 0x40, 0x4120_0000 + (seed << 8));
        c.bus.write32(VDTS_SINK + 0x44, 0x41a0_0000 + (seed << 8));
        c.bus.write32(VDTS_SINK + 0x48, 0x3f00_0000 + (seed << 4));
        c.bus.write32(VDTS_SINK + 0x4c, 0x4080_0000 + (seed << 6));
        c.bus.write32(VDTS_SINK + 0x50, 0x3f80_0000 + (seed << 8));
        for k in 0..8u32 {
            c.bus.write32(VDTS_ITEMS + 4 * k, VDTS_CELLS + 0x10 * k);
            c.bus
                .write32(VDTS_CELLS + 0x10 * k, 0x0abc_0000 + k + (seed << 4));
        }
        c.bus.write32(VDTS_DRAW_CTRL, VDTS_DRAW_MAGIC);
        c.bus.write32(VDTS_DRAW_BUSY, seed % 2);
        c.g[A0][0] = VDTS_SINK as u64;
        c.pc = 0x0025_48d0;
    }

    /// `VD_TextureSink_DrawFrame`: idle retry, both unsigned-float arms,
    /// rows x cells up to 3 x 2 through the fake draw slots. Path totals
    /// (R = idle calls, n = negative getter results, r rows, k cells):
    /// `163 + 1007R + 36 + 2n + [r > 0](1 + r(13 + [k > 0](7 + 53k)))`.
    #[test]
    fn vd_texture_sink_draw_frame_matches() {
        vdts_draw_hook();
        check_paths(
            ee::s06::f_002548d0__generated,
            vd_texture_sink_draw_frame,
            96,
            // 1206 r=0 (R=1, n=0); 1210 n=2 (seed 6); 2213 R=2 (seed 1);
            // 1220 r=1 k=0; 1246 r=3 k=0; 1280 r=1 k=1; 1353 r=2 k=1;
            // 1459 r=2 k=2; 1585 r=3 k=2.
            &[1206, 1210, 2213, 1220, 1246, 1280, 1353, 1459, 1585],
            &|c, seed| {
                machine(c, seed, 0x0025_48d0, 0);
                c.hooks.miss = vdts_norm_miss;
                vdts_draw_fixture(c, seed);
            },
        );
        fakes::covered(
            ee::s06::f_002548d0__generated,
            vd_texture_sink_draw_frame,
            96,
            &|c, seed| {
                machine(c, seed, 0x0025_48d1, 6);
                c.hooks.miss = vdts_norm_miss;
                vdts_draw_fixture(c, seed);
            },
        );
    }

    /// Manual Unwind-mode schedule past the prologue: the VBlank goes off
    /// `k` cycles after entry. `schedule` cannot do this: its k < 48
    /// fires before the draw loops, and `drive` cannot re-enter past a
    /// nested unwind.
    fn unwind_at(c: &mut Cpu, entry: u32, k: u64) {
        let saved = (c.g[A0], c.g[A1], c.g[RA]);
        ssx3_ps2::kernel::start_main_thread(c, entry, 0, 0);
        (c.g[A0], c.g[A1], c.g[RA]) = saved;
        c.pc = entry;
        c.cycles = 1000;
        c.next_event = 1000 + k;
        ssx3_ps2::platform::machine_of(&mut c.bus.plat)
            .sched
            .next_vblank = 1000 + k;
    }

    /// The five draw loops under a delivered VBlank. All heads are
    /// structured: each side must carry on past the delivery, never
    /// stopping there (compared by `check` on the raw bodies; `drive`
    /// cannot resume past a nested unwind). The per-seed VBlank time
    /// lands inside a loop for every head: 1140 hits 0x254a70, 120 the
    /// 0x254918 busy retry (via `vdts_draw_hook`), 1300/2200/2400 the
    /// row/cell loops, the rest 0x254a40; a reverted head unwinds there
    /// and `check` fails.
    #[test]
    fn vd_texture_sink_draw_frame_events() {
        use crate::difftest::events::UNWIND;
        let (gen, hand) = (
            ee::s06::f_002548d0__generated as fn(&mut Cpu),
            vd_texture_sink_draw_frame as fn(&mut Cpu),
        );
        let mut hit = std::collections::BTreeSet::new();
        vdts_draw_hook();
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 0x0025_48d0, 0);
            c.hooks.miss = vdts_norm_miss;
            vdts_draw_fixture(c, seed);
            unwind_at(
                c,
                0x0025_48d0,
                [1140, 120, 1300, 1150, 100, 2200, 140, 2400][seed as usize % 8] as u64,
            );
        };
        check(gen, hand, 96, &setup);
        for seed in 0..crate::difftest::seed_count(96) {
            let mut c = crate::difftest::fresh();
            setup(&mut c, seed);
            crate::difftest::run_generated(&mut c, gen);
            hit.insert(c.pc & !UNWIND);
        }
        let bad: Vec<u32> = [
            0x0025_4918,
            0x0025_4a40,
            0x0025_4a70,
            0x0025_4b28,
            0x0025_4af0,
        ]
        .into_iter()
        .filter(|h| hit.contains(h))
        .collect();
        assert!(
            bad.is_empty(),
            "unwound at structured heads {bad:#x?}: {hit:#x?}"
        );
    }

    /// The `miss` hook with a fixed 4-cycle cost.
    pub(crate) fn norm_miss(c: &mut Cpu, target: u32) {
        let before = c.cycles;
        crate::difftest::fakes::fake_call(c, target);
        c.cycles = before + 4;
    }

    /// A video sink with small row/column counts. `a1` is the frame (null
    /// one seed in four), whose `+4` descriptor has a row stride at `+6`
    /// and the 0x1000 flag at `+0xc` (data at `+0x10` or at `+[+0x10]`).
    /// The graphics manager at `gp+0x2a90` has the upload method (FAKE) at
    /// `[+0x10d8]+0x18c`; the row records the sink's `+0x14` table points
    /// at hold random words.
    pub(crate) fn upload_fixture(c: &mut Cpu, seed: u32) {
        const SINK: u32 = DATA + 0x100;
        const MID: u32 = DATA + 0x200;
        const VT: u32 = DATA + 0x300;
        const ROWS: u32 = DATA + 0x800;
        const SCRATCH: u32 = DATA + 0x900;
        const FRAME: u32 = DATA + 0xa00;
        const DESC: u32 = DATA + 0xa40;
        const RECORDS: u32 = DATA + 0xc00;
        let mut r = Rng(0x0025_4c48 ^ u64::from(seed) << 20);
        for i in 0..0x40u32 {
            c.bus.write32(SINK + 4 * i, 0x2222_0000 + i);
        }
        c.bus.write32(SINK + 8, MID);
        c.bus.write32(SINK + 0xc, 0x1111_0000 + seed);
        c.bus.write32(SINK + 0x10, SCRATCH);
        c.bus.write32(SINK + 0x14, ROWS);
        c.bus
            .write32(SINK + 0x1c, if seed % 3 == 0 { 0 } else { 1 + seed % 3 });
        c.bus
            .write32(SINK + 0x20, if seed % 2 == 0 { 0 } else { 1 + seed % 2 });
        c.bus.write32(SINK + 0x3c, 0x4444_0000 + seed * 7);
        crate::difftest::fill(c, &mut r, RECORDS, 0x40);
        for i in 0..0x10u32 {
            c.bus.write32(ROWS + 4 * i, RECORDS + 0x10 * i);
        }
        crate::difftest::fill(c, &mut r, DESC, 0x10);
        c.bus.write16(DESC + 6, 0x10 + (seed % 5) as u16 * 0x10);
        let flags = r.next_u32() & !0x1000 | if (seed / 6) % 2 == 0 { 0 } else { 0x1000 };
        c.bus.write32(DESC + 0xc, flags);
        c.bus.write32(DESC + 0x10, 0x40 + 4 * (seed % 7));
        c.bus.write32(FRAME + 4, DESC);
        c.bus.write32(crate::difftest::GAME_GP + 0x2a90, MID);
        c.bus.write32(MID + 0x10d8, VT);
        for i in 0..0x90u32 {
            c.bus.write32(VT + 4 * i, FAKE);
        }
        c.bus.write16(VT + 0x188, 0x20);
        c.bus.write32(MID + 0xe84, SCRATCH);
        c.bus.write32(SCRATCH + 8, 0x6666_0000 + seed);
        c.g[A0][0] = SINK as u64;
        c.g[A1][0] = if (seed / 12) % 4 == 3 {
            0
        } else {
            FRAME as u64
        };
        c.pc = 0x0025_4c48;
    }

    /// `VD_TextureSink_UploadFrame`: rows upload through the fake methods,
    /// then the real draw runs.
    #[test]
    fn vd_texture_sink_upload_frame_matches() {
        check(
            ee::s06::f_00254c48__generated,
            vd_texture_sink_upload_frame,
            60,
            &|c, seed| {
                machine(c, seed, 0x0025_4c48, 0);
                upload_fixture(c, seed);
            },
        );
        require_paths(
            ee::s06::f_00254c48__generated,
            60,
            // Rows {0, 2, 3} and columns {0, 2}, each with and without
            // the descriptor flag and the null frame.
            &[253, 298, 306, 319, 326, 680, 697, 903, 916, 932],
            &|c, seed| {
                machine(c, seed, 0x0025_4c48, 0);
                upload_fixture(c, seed);
            },
        );
    }

    driven!(upl_gen, ee::s06::f_00254c48__generated);

    driven!(upl_hand, vd_texture_sink_upload_frame);

    /// The upload loops under delivered events. Both heads (0x254cc0,
    /// 0x254ce0) are structured: the port must carry on, never resume
    /// there.
    #[test]
    fn vd_texture_sink_upload_frame_events() {
        let resumes = check_events(upl_gen, upl_hand, 60, &|c, seed| {
            machine(c, seed, 0x0025_4c48, 0);
            upload_fixture(c, seed);
            schedule(c, &mut Rng(seed as u64), seed, 0x0025_4c48)
        });
        for head in [0x0025_4cc0, 0x0025_4ce0] {
            assert!(
                resumes.get(&head).is_none(),
                "unwound at structured head {head:#x}: {resumes:x?}"
            );
        }
    }

    /// The video alloc hook `VidEnginePlayer_Open` calls (`0x510000 - 0x6bd0`).
    pub(crate) const OPEN_ALLOC_HOOK: u32 = 0x0050_9430;

    /// Gate of the two `AV_Player` constructor fakes (the by-name one loads
    /// the file for real): logged at a fixed 4 cycles under `OPEN_MAGIC`.
    pub(crate) const OPEN_CTRL: u32 = DATA + 0x1f80;

    pub(crate) const OPEN_MAGIC: u32 = 0x0a7e_2534;

    pub(crate) static OPEN_PREV_CTOR: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    pub(crate) static OPEN_PREV_CTOR_2: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    pub(crate) fn open_fake_ctor(c: &mut Cpu) {
        if c.bus.read32(OPEN_CTRL) != OPEN_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                &OPEN_PREV_CTOR,
                ee::s13::f_003addc0__generated,
            );
        }
        norm_miss(c, 0x003a_ddc0);
    }

    pub(crate) fn open_fake_ctor_2(c: &mut Cpu) {
        if c.bus.read32(OPEN_CTRL) != OPEN_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                &OPEN_PREV_CTOR_2,
                ee::s13::f_003ade08__generated,
            );
        }
        norm_miss(c, 0x003a_de08);
    }

    /// `P` is the player (Close runs for real: `+0x38` stream and `+0x3c`
    /// method table steered), `PARAMS` the 0x24-byte open parameters copied
    /// to `P + 0x14`: `+4` the file name (null, empty or a name), `+8` and
    /// `+0x10` the two flags that pick the `movz`/`movn` arguments.
    pub(crate) fn open_fixture(c: &mut Cpu, seed: u32) {
        const P: u32 = DATA + 0x100;
        const PARAMS: u32 = DATA + 0x400;
        const NAME: u32 = DATA + 0x600;
        for i in 0..0x20u32 {
            c.bus.write32(P + 4 * i, 0x8888_0000 + i);
        }
        for i in 0..0x10u32 {
            c.bus.write32(PARAMS + 4 * i, 0x9999_0000 + i);
        }
        c.bus.write8(NAME, 0);
        c.bus.write32(NAME + 4, 0x0076_6964);
        let name = [0, NAME, NAME + 4][seed as usize % 3];
        c.bus.write32(PARAMS + 4, name);
        c.bus.write32(PARAMS + 8, (seed / 3) % 2);
        c.bus.write32(PARAMS + 0x10, (seed / 6) % 2);
        c.bus.write32(P + 0x38, (seed / 12) % 2);
        for i in 0..8u32 {
            c.bus.write32(DATA + 0x500 + 4 * i, FAKE);
        }
        c.bus.write32(
            P + 0x3c,
            if (seed / 24) % 2 == 0 {
                0
            } else {
                DATA + 0x500
            },
        );
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_003ADDC0),
            open_fake_ctor,
            &OPEN_PREV_CTOR,
        );
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_003ADE08),
            open_fake_ctor_2,
            &OPEN_PREV_CTOR_2,
        );
        c.bus.write32(OPEN_CTRL, OPEN_MAGIC);
        c.bus.write32(OPEN_ALLOC_HOOK, FAKE);
        c.bus.write32(OPEN_ALLOC_HOOK + 4, FAKE);
        c.bus.write32(OPEN_ALLOC_HOOK + 8, 0x7777_0000 + seed);
        c.g[A0][0] = P as u64;
        c.g[A1][0] = PARAMS as u64;
        c.pc = 0x0025_34a8;
    }

    /// `VidEnginePlayer_Open`: close (quiet or loud), copy params, open by
    /// name or by stream with each flag mix; the alloc hook and the open
    /// callees are fakes.
    #[test]
    fn vid_engine_player_open_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 0x0025_34a8, 0);
            c.hooks.miss = norm_miss;
            open_fixture(c, seed);
        };
        check(
            ee::s06::f_002534a8__generated,
            vid_engine_player_open,
            48,
            &setup,
        );
        // Stream or by-name open (null/empty name vs a name), times the two
        // flags, times a quiet or loud Close.
        require_paths(
            ee::s06::f_002534a8__generated,
            48,
            &[114, 117, 119, 127, 130, 132, 185, 188, 190, 207, 210, 212],
            &setup,
        );
    }

    /// `VidEnginePlayer_Render`: null frames and null sinks skip the calls;
    /// live ones run the fake render method.
    #[test]
    fn vid_engine_player_render_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 0x0025_3890, 0);
            c.hooks.miss = norm_miss;
            const P: u32 = DATA + 0x100;
            const TAB: u32 = DATA + 0x300;
            const VT: u32 = DATA + 0x400;
            for i in 0..0x20u32 {
                c.bus.write32(P + 4 * i, 0x6666_0000 + i);
            }
            // The slot word rides bit 1 while `a1` rides bit 0: the
            // join needs `a1 == 0` with a null slot for one arm and a
            // nonzero slot for the other.
            c.bus
                .write32(P, if seed / 2 % 2 == 0 { 0 } else { 0x6666_0000 });
            c.bus.write32(P + 0x38, if seed % 3 == 0 { 0 } else { 1 });
            c.bus
                .write32(P + 0x3c, if (seed / 3) % 2 == 0 { 0 } else { TAB });
            c.bus.write32(
                P + 0x8,
                if (seed / 5) % 2 == 0 {
                    0
                } else {
                    0x7777_0000 + seed
                },
            );
            c.bus.write32(
                P + 0x40,
                if (seed / 7) % 2 == 0 {
                    0
                } else {
                    0x8888_0000 + seed
                },
            );
            c.bus.write32(
                P + 0x4,
                if (seed / 11) % 2 == 0 {
                    0
                } else {
                    0x9999_0000 + seed
                },
            );
            for i in 0..0x10u32 {
                c.bus.write32(TAB + 4 * i, FAKE);
            }
            c.bus.write32(TAB + 4, VT);
            for i in 0..0x20u32 {
                c.bus.write32(VT + 4 * i, FAKE);
            }
            c.g[A0][0] = P as u64;
            c.g[A1][0] = (seed % 2) as u64;
            c.pc = 0x0025_3890;
        };
        check(
            ee::s06::f_00253890__generated,
            vid_engine_player_render,
            100,
            &setup,
        );
        require_paths(
            ee::s06::f_00253890__generated,
            100,
            &[10, 14, 25, 28, 34, 38],
            &setup,
        );
        // The null-slot arm is the only writer of P+0x40 in
        // `Render`, clearing it after the 0x253920 call; the nonzero-slot
        // arm (0x253904 call) leaves it. The arms are cycle-identical, so
        // the clobber (and its absence) is what shows each ran.
        let mut clobbered = false;
        let mut left = false;
        for seed in 0..crate::difftest::seed_count(100) {
            let mut c = crate::difftest::fresh();
            setup(&mut c, seed);
            // The Join is reached with A1 == 0 (seed % 2 == 0), P+0x38
            // and P+0x3c (TAB) set, P+0x8 clear and P+0x40 set; the slot
            // word rides bit 1, so each arm is reachable.
            let slot_null = c.bus.read32(DATA + 0x100) == 0;
            let reaches_join = seed % 2 == 0
                && c.bus.read32(DATA + 0x138) != 0
                && c.bus.read32(DATA + 0x13c) == DATA + 0x300
                && c.bus.read32(DATA + 0x108) == 0
                && c.bus.read32(DATA + 0x140) != 0;
            crate::difftest::run_generated(&mut c, ee::s06::f_00253890__generated);
            if reaches_join && slot_null {
                assert_eq!(
                    c.bus.read32(DATA + 0x140),
                    0,
                    "seed {seed}: slot-0x20 arm did not clear P+0x40"
                );
                clobbered = true;
            }
            if reaches_join && !slot_null {
                assert_ne!(
                    c.bus.read32(DATA + 0x140),
                    0,
                    "seed {seed}: nonzero-slot arm cleared P+0x40"
                );
                left = true;
            }
        }
        assert!(clobbered, "slot-0x20 arm never ran with a nonzero word");
        assert!(left, "nonzero-slot arm never ran with a1 == 0");
    }

    /// `VidEnginePlayer_Reopen`: close (quiet or loud), reset the params,
    /// and open again on the probe struct.
    #[test]
    fn vid_engine_player_reopen_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 0x0025_3460, 0);
            c.hooks.miss = norm_miss;
            const P: u32 = DATA + 0x100;
            const PARAMS: u32 = DATA + 0x400;
            for i in 0..0x20u32 {
                c.bus.write32(P + 4 * i, 0x8888_0000 + i);
            }
            for i in 0..8u32 {
                c.bus.write32(PARAMS + 4 * i, 0x9999_0000 + i);
            }
            // Close arms: +0x38 quiet vs loud (+0x40/+0x3c/+0xc steer
            // the loud path); +0x24 steers Open's S2.
            c.bus.write32(P + 0x38, (seed % 2) as u32);
            c.bus.write32(P + 0x40, ((seed / 2) % 2) as u32);
            c.bus
                .write32(P + 0x3c, if (seed / 4) % 2 == 0 { 0 } else { DATA + 0x500 });
            c.bus.write32(P + 0xc, ((seed / 8) % 2) as u32);
            c.bus.write32(P + 0x24, ((seed / 16) % 2) as u32);
            for i in 0..8u32 {
                c.bus.write32(DATA + 0x500 + 4 * i, FAKE);
            }
            c.bus.write32(OPEN_ALLOC_HOOK, FAKE);
            c.g[A0][0] = P as u64;
            c.g[A1][0] = PARAMS as u64;
            c.pc = 0x0025_3460;
        };
        check(
            ee::s06::f_00253460__generated,
            vid_engine_player_reopen,
            60,
            &setup,
        );
        require_paths(
            ee::s06::f_00253460__generated,
            60,
            &[322, 325, 335, 338, 344, 357, 393, 396, 415, 418, 437],
            &setup,
        );
        // The Open hook site must be reached on every seed: a logged call
        // returning to the hook's return address (0x25356c opened,
        // 0x2535bc !opened) must target the planted FAKE, not address 0.
        // (A blanket no-null assertion cannot be used here: nested Close
        // calls log their own targets on some seeds.)
        for seed in 0..crate::difftest::seed_count(60) {
            let mut c = crate::difftest::fresh();
            setup(&mut c, seed);
            crate::difftest::run_generated(&mut c, ee::s06::f_00253460__generated);
            let targets = crate::difftest::fakes::logged_targets(&mut c);
            let rets = crate::difftest::fakes::logged_returns(&mut c);
            let hook_targets: Vec<u32> = rets
                .into_iter()
                .zip(targets)
                .filter(|(ret, _)| *ret == 0x0025_356c || *ret == 0x0025_35bc)
                .map(|(_, t)| t)
                .collect();
            assert!(
                !hook_targets.is_empty() && hook_targets.iter().all(|&t| t == FAKE),
                "seed {seed}: hook site not reached with FAKE: {hook_targets:#x?}"
            );
        }
    }

    /// The two debug-print stubs: every spilled register is distinctive, so
    /// a missing or extra store changes RAM.
    #[test]
    fn debug_printf_stubs_match() {
        for (gen, hand, addr) in [
            (
                ee::s06::f_00253b18__generated as fn(&mut Cpu),
                vid_engine_debug_printf_stub as fn(&mut Cpu),
                0x0025_3b18,
            ),
            (
                ee::s06::f_00254e20__generated as fn(&mut Cpu),
                vd_debug_printf_stub as fn(&mut Cpu),
                0x0025_4e20,
            ),
        ] {
            crate::difftest::check_paths(gen, hand, 100, &[15], &|c, seed| {
                machine(c, seed, addr as u64, 0);
                c.g[A0][0] = (DATA + 0x123) as u64;
                c.g[A1][0] = 0x1111_0000 + seed as u64;
                c.g[A2][0] = 0x2222_0000 + seed as u64 * 3;
                c.g[A3][0] = 0x3333_0000 + seed as u64 * 7;
                c.g[T0][0] = 0x4444_0000 + seed as u64;
                c.g[T1][0] = 0x5555_0000 + seed as u64;
                c.g[T2][0] = 0x6666_0000 + seed as u64;
                c.g[T3][0] = 0x7777_0000 + seed as u64;
                c.f[12] = 0x3f80_0000 + seed;
                c.f[14] = 0x4000_0000 + seed * 3;
                c.f[16] = 0x4040_0000 + seed * 5;
                c.f[18] = 0x4080_0000 + seed * 7;
                c.pc = addr;
            });
        }
    }

    /// The video hook words must actually be reached: run the setup
    /// through the generated body and require that no call fell through
    /// to address 0. (The pre-fix fixtures planted the hook at 0x4a9430 /
    /// 0x4a9434 instead of 0x509430 / 0x509434, so both sides called
    /// address 0 and agreed by accident.)
    fn assert_no_null_hook_calls(gen: fn(&mut Cpu), seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
        for seed in 0..crate::difftest::seed_count(seeds) {
            let mut c = crate::difftest::fresh();
            setup(&mut c, seed);
            crate::difftest::run_generated(&mut c, gen);
            let targets = crate::difftest::fakes::logged_targets(&mut c);
            assert!(
                targets.iter().all(|&t| t != 0),
                "seed {seed}: call to address 0 (hook word not planted): {targets:#x?}"
            );
        }
    }

    /// `VD_Alloc` and `VD_Alloc_2`: the hook word points at the fake, whose
    /// hash result varies with the object on both sides.
    #[test]
    fn vd_alloc_matches() {
        for (gen, hand, addr) in [
            (
                ee::s06::f_00255670__generated as fn(&mut Cpu),
                vd_alloc as fn(&mut Cpu),
                0x0025_5670,
            ),
            (
                ee::s06::f_002556b0__generated as fn(&mut Cpu),
                vd_alloc_2 as fn(&mut Cpu),
                0x0025_56b0,
            ),
        ] {
            let setup = |c: &mut Cpu, seed: u32| {
                let mut r = machine(c, seed, addr as u64, 0);
                c.hooks.miss = norm_miss;
                c.bus.write32(OPEN_ALLOC_HOOK, FAKE);
                c.g[A0][0] = (DATA + 0x123 + (r.next_u32() & 0xfff0)) as u64;
                c.pc = addr;
            };
            crate::difftest::check_paths(gen, hand, 300, &[19], &setup);
            assert_no_null_hook_calls(gen, 300, &setup);
        }
    }

    /// `VD_Free` and `VD_Free_2`: the hook word points at the fake, whose
    /// hash result varies with the freed pointer on both sides.
    #[test]
    fn vd_free_matches() {
        for (gen, hand, addr) in [
            (
                ee::s06::f_00255750__generated as fn(&mut Cpu),
                vd_free as fn(&mut Cpu),
                0x0025_5750,
            ),
            (
                ee::s06::f_00255778__generated as fn(&mut Cpu),
                vd_free_2 as fn(&mut Cpu),
                0x0025_5778,
            ),
        ] {
            let setup = |c: &mut Cpu, seed: u32| {
                let mut r = machine(c, seed, addr as u64, 0);
                c.bus.write32(OPEN_ALLOC_HOOK + 4, FAKE);
                c.g[A0][0] = (DATA + 0x100 + (r.next_u32() & 0xfff0)) as u64;
                c.pc = addr;
            };
            check(gen, hand, 300, &setup);
            assert_no_null_hook_calls(gen, 300, &setup);
        }
    }

    /// `VidEnginePlayer_GlobalCtors` and `VD_GlobalCtors`: the full
    /// `(1, 0xffff)` table init runs for real over the game image, so the
    /// float tables land identically on both sides.
    #[test]
    fn global_ctors_matches() {
        for (gen, hand, addr) in [
            (
                ee::s06::f_00254330__generated as fn(&mut Cpu),
                vid_engine_player_global_ctors as fn(&mut Cpu),
                0x0025_4330,
            ),
            (
                ee::s06::f_002557c0__generated as fn(&mut Cpu),
                vd_global_ctors as fn(&mut Cpu),
                0x0025_57c0,
            ),
        ] {
            check(gen, hand, 60, &|c, seed| {
                machine(c, seed, addr as u64, 0);
                elf_image(c);
                c.pc = addr;
            });
        }
    }

    /// `VidEngine_MemFree`: null (the early return, 14 cycles) or a live
    /// heap / pooled block through the real free path with the allocator
    /// mutex fixture.
    #[test]
    fn vid_engine_mem_free_matches() {
        check(
            ee::s06::f_00253af8__generated,
            vid_engine_mem_free,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0025_3af8, 0);
                mutex_state(c, &mut r, MEM_MUTEX);
                let p = free_target(c, &mut r);
                c.g[A0][0] = if seed % 3 == 2 { 0 } else { p };
                c.pc = 0x0025_3af8;
            },
        );
        crate::difftest::require_paths(ee::s06::f_00253af8__generated, 100, &[14], &|c, seed| {
            let mut r = machine(c, seed, 0x0025_3af8, 0);
            mutex_state(c, &mut r, MEM_MUTEX);
            let p = free_target(c, &mut r);
            c.g[A0][0] = if seed % 3 == 2 { 0 } else { p };
            c.pc = 0x0025_3af8;
        });
    }

    /// `VD_TextureSink_Redraw`: the sink's method table points every virtual
    /// call at the fake, and the row/column counts stay tiny (0 skips the
    /// loop, otherwise 1..3), so the draw runs its full dispatch on both
    /// sides without running away.
    #[test]
    fn vd_texture_sink_redraw_matches() {
        // Distinctive from FAKE: with FAKE slots the assert
        // passed with or without the gp plant, so it could not tell
        // the plants apart. VT_TARGET pins the fixture VT itself:
        // removing SINK+8 (the GM pointer this path reads) logs no
        // VT_TARGET and fails.
        const VT_TARGET: u32 = 0x0000_4002;
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 0x0025_4da0, 0);
            elf_image(c);
            const SINK: u32 = DATA + 0x100;
            const MID: u32 = DATA + 0x200;
            const VT: u32 = DATA + 0x300;
            const ROWS: u32 = DATA + 0x800;
            const SCRATCH: u32 = DATA + 0x900;
            for i in 0..0x40 {
                c.bus.write32(SINK + 4 * i, 0x2222_0000 + i);
            }
            c.bus.write32(SINK + 8, MID);
            // The graphics manager DrawFrame reads (without
            // this it reads the game image, not the fixture MID).
            c.bus.write32(crate::difftest::GAME_GP + 0x2a90, MID);
            // Row count: 0 skips the outer loop, else {2, 3} rows.
            c.bus
                .write32(SINK + 0x1c, if seed % 3 == 0 { 0 } else { 1 + seed % 3 });
            // Column count: 0 skips the inner loop, else {2}.
            c.bus
                .write32(SINK + 0x20, if seed % 2 == 0 { 0 } else { 1 + seed % 2 });
            c.bus.write32(SINK + 0x3c, 0x4444_0000 + seed * 7);
            // Row base: real RAM prefilled with distinctive words.
            c.bus.write32(SINK + 0x14, ROWS);
            for i in 0..0x10 {
                c.bus.write32(ROWS + 4 * i, 0x5555_0000 + i);
            }
            c.bus.write32(MID + 0x10d8, VT);
            for i in 0..0x90 {
                c.bus.write32(VT + 4 * i, VT_TARGET);
            }
            // The scratch pointer the +0xe84 chain loads, with a
            // distinctive word at the slot it reads and writes.
            c.bus.write32(MID + 0xe84, SCRATCH);
            c.bus.write32(SCRATCH + 8, 0x6666_0000 + seed);
            c.g[A0][0] = SINK as u64;
            c.pc = 0x0025_4da0;
        };
        check(
            ee::s06::f_00254da0__generated,
            vd_texture_sink_redraw,
            100,
            &setup,
        );
        // The drawn cells must run through the fixture VT (VT_TARGET):
        // some seed logs a call targeting VT_TARGET (SINK+8 removed
        // logs none).
        let mut saw_vt = false;
        for seed in 0..crate::difftest::seed_count(100) {
            let mut c = crate::difftest::fresh();
            setup(&mut c, seed);
            crate::difftest::run_generated(&mut c, ee::s06::f_00254da0__generated);
            if crate::difftest::fakes::logged_targets(&mut c).contains(&VT_TARGET) {
                saw_vt = true;
            }
        }
        assert!(saw_vt, "no seed drew through the fixture VT");
    }

    /// `VD_TextureSink_SetFrameRate`: distinctive nonzero floats land at
    /// `sink + 0x4c` / `sink + 0x50`.
    #[test]
    fn vd_texture_sink_set_frame_rate_matches() {
        check_paths(
            ee::s06::f_00254400__generated,
            vd_texture_sink_set_frame_rate,
            300,
            &[3],
            &|c, seed| {
                let mut r = machine(c, seed, 0x0025_4400, 0);
                let sink = DATA + 0x100 + (r.next_u32() & 0xf0);
                for i in 0..8 {
                    c.bus.write32(sink + 4 * i, 0x1111_1111 * (i + 1));
                }
                c.f[12] = 0x3f80_0000 + seed;
                c.f[13] = 0x4000_0000 + seed * 3;
                c.g[A0][0] = sink as u64;
                c.pc = 0x0025_4400;
            },
        );
    }
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
