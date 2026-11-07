//! `src/worldpainter/quadtree.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::be::bestate2::ea;
pub(crate) use crate::game::be::bestate2::gp_rel;
pub(crate) use crate::game::bx::bxstring::gp as gp_4;
pub(crate) use crate::game::bx::bxstring2::MESSAGE_MAP;
pub(crate) use crate::game::camera::camera::gp;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::p_8;
pub(crate) use crate::game::dirtysock::tags2::gp_9 as gp_3;
pub(crate) use crate::game::dirtysock::tags2::lw;
pub(crate) use crate::game::fe::feasyncfile::gp_11 as gp_5;
pub(crate) use crate::game::fe::feasyncfile::gpa;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::p as p_2;
pub(crate) use crate::game::fe::feasyncfile::p as p_3;
pub(crate) use crate::game::fe::feasyncfile::p as p_5;
pub(crate) use crate::game::fe::feasyncfile::p as p_6;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p;
pub(crate) use crate::game::fe::festateload::at;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p_4;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::festaterewards2::r;
pub(crate) use crate::game::main::gamemode::low as low_2;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::main::gamemode::sw_r;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2 as lw_at;
pub(crate) use crate::game::sound::bankmonitor::add32;
pub(crate) use crate::game::sound::soundsys::p as p_7;
pub(crate) use crate::game::visualfx::renderstateman::STRLEN_AND_STRCPY;
pub(crate) use crate::game::wscript::wscriptman::gp as gp_2;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002c_2088, quad_tree_flatten_node, "QuadTree_FlattenNode"),
    (
        0x002c_2210,
        mem_card_ascii_to_sjis_string,
        "MemCard_AsciiToSjisString",
    ),
    (
        0x002c_2268,
        memcard_sjis_to_ascii_string,
        "MemCard_SjisToAsciiString",
    ),
    (0x002c_22c8, c_ps2_mem_card_create, "cPS2MemCard_Create"),
    (0x002c_2300, c_ps2_mem_card_destroy, "cPS2MemCard_Destroy"),
    (
        0x002c_2338,
        memcard_ascii_char_to_sjis,
        "MemCard_AsciiCharToSjis",
    ),
    (
        0x002c_2430,
        memcard_sjis_char_to_ascii,
        "MemCard_SjisCharToAscii",
    ),
    (0x002c_24d0, ustr_len_2, "UStr_Len_2"),
    (0x002c_2540, ustr_from_ascii_2, "UStr_FromAscii_2"),
    (0x002c_25b8, ustr_copy_n, "UStr_CopyN"),
    (0x002c_2688, ustr_cat, "UStr_Cat"),
    (0x002c_2868, str_to_lower_in_place, "Str_ToLowerInPlace"),
    (0x002c_3fa8, c_ps2_mem_card_ctor, "cPS2MemCard_ctor"),
    (0x002c_3fe0, c_ps2_mem_card_dtor, "cPS2MemCard_dtor"),
    (
        0x002c_4198,
        c_ps2_mem_card_request_list_dir_w,
        "cPS2MemCard_RequestListDirW",
    ),
    (
        0x002c_41d8,
        c_ps2_mem_card_request_list_dir,
        "cPS2MemCard_RequestListDir",
    ),
    (
        0x002c_4210,
        c_ps2_mem_card_do_list_dir,
        "cPS2MemCard_DoListDir",
    ),
    (
        0x002c_42b0,
        cps2_mem_card_do_get_save_dir_entry,
        "cPS2MemCard_DoGetSaveDirEntry",
    ),
    (
        0x002c_4388,
        c_ps2_mem_card_do_format,
        "cPS2MemCard_DoFormat",
    ),
    (
        0x002c_4480,
        ps2_memcard_request_get_info,
        "cPS2MemCard_RequestGetInfo",
    ),
    (
        0x002c_44e0,
        c_ps2_mem_card_clear_port_changed,
        "cPS2MemCard_ClearPortChanged",
    ),
    (0x002c_4520, c_ps2_mem_card_v34, "cPS2MemCard_v34"),
    (0x002c_4578, c_ps2_mem_card_v35, "cPS2MemCard_v35"),
    (0x002c_45d0, cps2_mem_card_v37, "cPS2MemCard_v37"),
    (0x002c_4648, cps2_mem_card_v36, "cPS2MemCard_v36"),
    (0x002c_46c0, ps2_mem_card_v38, "cPS2MemCard_v38"),
    (
        0x002c_4748,
        c_ps2_mem_card_is_state34,
        "cPS2MemCard_IsState34",
    ),
    (
        0x002c_4788,
        cps2_mem_card_request_read_save,
        "cPS2MemCard_RequestReadSave",
    ),
    (0x002c_4800, c_ps2_mem_card_v29, "cPS2MemCard_v29"),
    (
        0x002c_4880,
        c_ps2_mem_card_is_state35,
        "cPS2MemCard_IsState35",
    ),
    (
        0x002c_50e0,
        ps2_memcard_do_get_info,
        "cPS2MemCard_DoGetInfo",
    ),
    (0x002c_5140, c_ps2_mem_card_is_busy, "cPS2MemCard_IsBusy"),
    (
        0x002c_5168,
        c_ps2_mem_card_is_port_card_error,
        "cPS2MemCard_IsPortCardError",
    ),
    (
        0x002c_51d0,
        cps2memcard_get_port_error,
        "cPS2MemCard_GetPortError",
    ),
    (
        0x002c_5250,
        cps2memcard_clear_port_info,
        "cPS2MemCard_ClearPortInfo",
    ),
    (
        0x002c_5278,
        ps2_mem_card_clear_dir_cache,
        "cPS2MemCard_ClearDirCache",
    ),
    (
        0x002c_52d8,
        c_ps2_mem_card_clear_dir_entry,
        "cPS2MemCard_ClearDirEntry",
    ),
    (
        0x002c_5300,
        c_ps2_mem_card_is_port_not_ps2_card,
        "cPS2MemCard_IsPortNotPS2Card",
    ),
    (
        0x002c_5320,
        cps2memcard_get_free_space_for_port,
        "cPS2MemCard_GetFreeSpaceForPort",
    ),
    (0x002c_5358, c_ps2_mem_card_v52, "cPS2MemCard_v52"),
    (
        0x002c_53b0,
        c_ps2_mem_card_is_port_formatted,
        "cPS2MemCard_IsPortFormatted",
    ),
    (
        0x002c_53c8,
        cps2_mem_card_get_port_changed_flag,
        "cPS2MemCard_GetPortChangedFlag",
    ),
    (
        0x002c_5528,
        c_ps2_mem_card_has_slot_name,
        "cPS2MemCard_HasSlotName",
    ),
    (
        0x002c_5558,
        c_ps2_mem_card_mark_slot_used,
        "cPS2MemCard_MarkSlotUsed",
    ),
    (
        0x002c_5570,
        cps2memcard_mark_save_invalid,
        "cPS2MemCard_MarkSaveInvalid",
    ),
    (
        0x002c_5588,
        c_ps2_mem_card_is_slot_empty,
        "cPS2MemCard_IsSlotEmpty",
    ),
    (
        0x002c_55c0,
        c_ps2_mem_card_slot_has_1ac,
        "cPS2MemCard_SlotHas1AC",
    ),
    (
        0x002c_6280,
        cps2memcard_mark_slots_without_space,
        "cPS2MemCard_MarkSlotsWithoutSpace",
    ),
    (0x002c_6360, c_ps2_mem_card_close, "cPS2MemCard_Close"),
    (
        0x002c_63e8,
        cps2memcard_is_mc_error,
        "cPS2MemCard_IsMcError",
    ),
    (
        0x002c_63f8,
        cps2_memcard_get_slot_name,
        "cPS2MemCard_GetSlotName",
    ),
    (0x002c_64a8, c_ps2_mem_card_v15, "cPS2MemCard_v15"),
    (0x002c_64d0, c_ps2_mem_card_v16, "cPS2MemCard_v16"),
    (0x002c_66b8, c_ps2_mem_card_v39, "cPS2MemCard_v39"),
    (
        0x002c_66d8,
        c_ps2_mem_card_is_port_result_m11,
        "cPS2MemCard_IsPortResultM11",
    ),
    (
        0x002c_6758,
        c_ps2_mem_card_calc_save_size_kb,
        "cPS2MemCard_CalcSaveSizeKB",
    ),
    (
        0x002c_6810,
        c_ps2_mem_card_request_list_dir_all,
        "cPS2MemCard_RequestListDirAll",
    ),
    (
        0x002c_6848,
        c_ps2_mem_card_do_list_dir_all,
        "cPS2MemCard_DoListDirAll",
    ),
    (
        0x002c_68c8,
        cps2_memcard_get_dir_entry_name,
        "cPS2MemCard_GetDirEntryName",
    ),
    (
        0x002c_68f0,
        cps2_mem_card_count_saves,
        "cPS2MemCard_CountSaves",
    ),
    (
        0x002c_6968,
        c_ps2_mem_card_request_open,
        "cPS2MemCard_RequestOpen",
    ),
    (0x002c_69a0, cps2_mem_card_do_open, "cPS2MemCard_DoOpen"),
    (0x002c_6a28, c_ps2_mem_card_do_close, "cPS2MemCard_DoClose"),
    (0x002c_6a88, c_ps2_mem_card_do_seek, "cPS2MemCard_DoSeek"),
    (0x002c_6b00, c_ps2_mem_card_do_flush, "cPS2MemCard_DoFlush"),
    (0x002c_6b78, c_ps2_mem_card_do_read, "cPS2MemCard_DoRead"),
    (0x002c_6bf8, c_ps2_mem_card_do_write, "cPS2MemCard_DoWrite"),
    (
        0x002c_6c50,
        c_ps2_mem_card_request_mkdir,
        "cPS2MemCard_RequestMkdir",
    ),
    (0x002c_6c88, c_ps2_mem_card_do_mkdir, "cPS2MemCard_DoMkdir"),
    (
        0x002c_6ce0,
        c_ps2_mem_card_request_delete,
        "cPS2MemCard_RequestDelete",
    ),
    (
        0x002c_6d18,
        c_ps2_mem_card_do_delete,
        "cPS2MemCard_DoDelete",
    ),
    (0x002c_6d70, loc_string_table_dtor, "LocStringTable_dtor"),
    (0x002c_6de0, loc_string_table_find, "LocStringTable_Find"),
];

/// Direct and cross-shard callees, by guest address.
pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::be::beintoption::callee::USTR_FROM_ASCII_2;

    pub(crate) use crate::game::camera::camera::callee::MEMCARD_SET_STATE;

    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_GET_SAVE_NAME;

    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_REQUEST_READ_SAVE;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_SET_STATE;

    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_CARD_ASCII_CHAR_TO_SJIS;

    pub(crate) use crate::game::dirtysock::tags2::callee::SCE_MC_FORMAT;

    pub(crate) use crate::game::dirtysock::tags2::callee::SPRINTF_2 as SPRINTF;

    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY as STRCPY_2;

    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_MEM_CARD_CLEAR_DIR_CACHE;

    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_COPY;

    pub(crate) use crate::game::fe::festateload::callee::STRLEN_COPY;

    pub(crate) use crate::game::intersect::worldsphtree::callee::MEMCARD_SJIS_CHAR_TO_ASCII;

    pub(crate) use crate::game::intersect::worldsphtree::callee::PS2_MEMCARD_SET_STATE;

    pub(crate) use crate::game::main::gamemode::callee::CPS2_MEM_CARD_SET_STATE as CPS2_MEM_CARD_SET_STATE_2;

    pub(crate) use crate::game::main::gamemode::callee::STRCPY as STRCPY_3;

    pub(crate) use crate::game::main::gamemode::callee::USTR_TO_ASCII as USTR_TO_ASCII_2;

    pub(crate) use crate::game::render::particle::callee::CPS2_MEM_CARD_SET_STATE;

    pub(crate) use crate::game::render::particle::callee::SCE_MC_DELETE;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_MKDIR;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_READ;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_SEEK;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_WRITE;

    pub(crate) use crate::game::sound::bankmonitor::callee::USTR_TO_ASCII;

    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STRLEN_AND_STRCPY;

    pub(crate) use crate::game::wscript::wscriptman::callee::CPS2_MEMCARD_SET_STATE;

    /// The engine's named-block allocator (`cMemMan_alloc`).
    pub const EA_MEM_ALLOC: Callee = Callee {
        addr: 0x0031_7d70,
        f: ee::s10::EA_MemAllocNamed,
    };

    pub const BUILTIN_DELETE_2: Callee = Callee {
        addr: 0x0031_7e50,
        f: ee::s10::__builtin_delete,
    };

    pub const PS2_SOUND_MANAGER_CTOR: Callee = Callee {
        addr: 0x002a_8c20,
        f: ee::s08::PS2SoundManager_ctor,
    };

    pub const STREAM_BUFFER_SET_FILE: Callee = Callee {
        addr: 0x002a_9e78,
        f: ee::s08::StreamBuffer_SetFile,
    };

    pub const STREAM_BUFFER_STOP: Callee = Callee {
        addr: 0x002a_a408,
        f: ee::s08::StreamBuffer_Stop,
    };

    pub const SPEECH_INSTANCE_STOP_STREAM: Callee = Callee {
        addr: 0x002b_0320,
        f: ee::s08::SpeechInstance_StopStream,
    };

    pub const SONG_GET_STREAM_STATUS: Callee = Callee {
        addr: 0x002b_2578,
        f: ee::s08::SongInstance_GetStreamStatus,
    };

    pub const SONG_POST_EVENT: Callee = Callee {
        addr: 0x002b_20c8,
        f: ee::s08::SongInstance_PostEvent,
    };

    pub const SONG_FADE_OUT: Callee = Callee {
        addr: 0x002b_2418,
        f: ee::s08::SongInstance_FadeOut,
    };

    /// `SNDSTRM_GetRequestStatus` (0x3b7c40, another shard).
    pub const SNDSTRM_GET_REQUEST_STATUS: u32 = 0x003b_7c40;

    pub const MEM_CARD_CTOR: Callee = Callee {
        addr: 0x002c_3fa8,
        f: ee::s08::cPS2MemCard_ctor,
    };

    pub const MEM_CARD_INIT: Callee = Callee {
        addr: 0x002c_4050,
        f: ee::s08::cPS2MemCard_Init,
    };

    pub const MEM_CARD_SET_STATE: Callee = Callee {
        addr: 0x002c_48c0,
        f: ee::s08::cPS2MemCard_SetState,
    };

    pub const CONSOLE_CONFIG_CTOR: Callee = Callee {
        addr: 0x002c_7478,
        f: ee::s08::cPS2ConsoleConfig_ctor,
    };

    /// `strcpy` (0x4166f4, s15).
    pub const STRCPY: Callee = Callee {
        addr: 0x0041_66f4,
        f: ee::s15::strcpy,
    };

    pub const MENU_ITEM_MEASURE_ROW: Callee = Callee {
        addr: 0x002c_a4c8,
        f: ee::s09::MenuSysItem_MeasureRow,
    };

    pub const STATS_MAN_CTOR: Callee = Callee {
        addr: 0x002d_21c0,
        f: ee::s09::SSXStatsMan_ctor,
    };

    pub const AI_VFX_MAN_CTOR: Callee = Callee {
        addr: 0x002f_03c8,
        f: ee::s09::PS2AIVisualEffectsMan_ctor,
    };

    pub const VFX_MAN_CTOR: Callee = Callee {
        addr: 0x002f_0548,
        f: ee::s09::PS2VisualEffectsMan_ctor,
    };

    /// `World_GetSectionScriptFunc` (0x3a6d18, s13).
    pub const WORLD_GET_SECTION_SCRIPT_FUNC: u32 = 0x003a_6d18;

    pub const WORLD_OBJ_CTOR: Callee = Callee {
        addr: 0x0034_fb90,
        f: ee::s11::WorldObj_ctor_2,
    };

    pub const MOD_OBJ_CTOR: Callee = Callee {
        addr: 0x0035_52d8,
        f: ee::s11::ModObj_ctor_2,
    };

    pub const GET_TRANSFORM_BOUNDS: Callee = Callee {
        addr: 0x0035_2b88,
        f: ee::s11::ModifierBlock_GetTransformBounds,
    };

    pub const INTC_REMOVE_HANDLER: Callee = Callee {
        addr: 0x0036_2008,
        f: ee::s12::IntcTable_RemoveHandler,
    };

    pub const EMITTER_RENDER: Callee = Callee {
        addr: 0x0037_08c0,
        f: ee::s12::Emitter_Render,
    };

    pub const GRAPHICS_ON_VIF1_DMA_END: Callee = Callee {
        addr: 0x0038_2688,
        f: ee::s12::PS2GraphicsMan_OnVif1DmaEnd,
    };

    /// `PS2GraphicsMan_ctor` (0x395288, s13).
    pub const GRAPHICS_MAN_CTOR: u32 = 0x0039_5288;
}

/// Cross-shard calls the generator emitted as `call_indirect(c, addr)`: they
/// resolve through the function table like the original's `jal`.
pub(crate) mod table_callee {
    pub(crate) use crate::game::be::bedbreward::table_callee::EA_MEMSET;
}

/// `gp`-relative globals used here.
pub(crate) mod g {}

/// Cross-shard direct calls the generator emitted as `call_indirect(c, addr)`.
pub(crate) mod table {
    pub(crate) use crate::game::wscript::wscriptman::table::BUILTIN_DELETE;

    pub(crate) use crate::game::wscript::wscriptman::table::MEM_FREE;
    pub(crate) use crate::game::wscript::wscriptman::table::SCE_MC_FLUSH;
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod addr {

    pub(crate) use crate::game::bx::bxstring2::addr::MC_MANAGER_GET_INFO;

    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRLEN;
}

// ---------------------------------------------------------------------------
// Memory card / character conversion.

/// `short MemCard_AsciiCharToSjis(uchar ch)` (0x2c2338): map an ASCII byte to
/// its SJIS code through two static tables, 0 for unmapped bytes.
pub fn memcard_ascii_char_to_sjis(c: &mut Cpu) {
    c.cycles += 6;
    let ch = abi::arg(c, 0) & 0xff;
    c.g[A2][0] = ch as u64;
    c.g[V1][0] = 0;
    c.g[A0][0] = 0;
    c.g[V0][0] = sx(ch.wrapping_sub(0x20));
    c.g[V0][0] = (c.g[V0][0] < 0x10) as u64;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[V1][0] = 0x1;
    } else {
        c.cycles += 4;
        c.g[V0][0] = sx(ch.wrapping_sub(0x30));
        let digits = (c.g[V0][0] < 0xa) as u64;
        c.g[V0][0] = sx(ch.wrapping_sub(0x3a));
        if digits == 0 {
            c.cycles += 3;
            c.g[V0][0] = (c.g[V0][0] < 0x7) as u64;
            let not_punct = c.g[V0][0] == 0;
            c.g[V0][0] = sx(ch.wrapping_sub(0x41));
            if !not_punct {
                c.cycles += 2;
                c.g[V1][0] = 0xb;
            } else {
                c.cycles += 3;
                c.g[V0][0] = (c.g[V0][0] < 0x1a) as u64;
                let not_upper = c.g[V0][0] == 0;
                c.g[V0][0] = sx(ch.wrapping_sub(0x5b));
                if !not_upper {
                    c.cycles += 2;
                    c.g[A0][0] = 0x1;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V0][0] < 0x6) as u64;
                    let not_lbracket = c.g[V0][0] == 0;
                    c.g[V0][0] = sx(ch.wrapping_sub(0x61));
                    if !not_lbracket {
                        c.cycles += 2;
                        c.g[V1][0] = 0x25;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = (c.g[V0][0] < 0x1a) as u64;
                        let not_lower = c.g[V0][0] == 0;
                        c.g[V0][0] = sx(ch.wrapping_sub(0x7b));
                        if !not_lower {
                            c.cycles += 2;
                            c.g[A0][0] = 0x2;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = (c.g[V0][0] < 0x4) as u64;
                            c.g[V1][0] = 0x3f;
                            if c.g[V0][0] == 0 {
                                c.cycles += 2;
                                c.g[V0][0] = 0;
                                abi::ret(c);
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
    // End of the mapping chain: byte-swap the mapped half-word.
    c.cycles += 2;
    if c.g[V1][0] == 0 {
        c.g[V1][0] = 0x440000;
        c.cycles += 9;
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[V1][0] = sx(0x440000u32.wrapping_add(0x5940));
        c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lhu(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x2)).lhu(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(ch));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A0][0] as u32));
        c.g[V0][0] &= 0xffff;
    } else {
        c.cycles += 8;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1f));
        c.g[V1][0] = 0x440000;
        c.g[V0][0] = sx(ch.wrapping_sub(c.g[V0][0] as u32));
        c.g[V1][0] = sx(0x440000u32.wrapping_add(0x5950));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = Addr(c.g[V0][0] as u32).lhu(c);
    }
    c.cycles += 6;
    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 8);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
    c.g[V0][0] |= c.g[V1][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
    abi::ret(c);
}

/// `uchar MemCard_SjisCharToAscii(ushort sjis)` (0x2c2430): map a two-byte SJIS
/// code back to ASCII.
pub fn memcard_sjis_char_to_ascii(c: &mut Cpu) {
    c.cycles += 6;
    let code = abi::arg(c, 0);
    let lead = (code >> 8) & 0xff;
    let trail = code & 0xff;
    c.g[V0][0] = sx(code >> 8);
    c.g[A2][0] = lead as u64;
    c.g[A0][0] = trail as u64;
    c.g[A1][0] = 0;
    if trail != 0x81 {
        c.cycles += 3;
        if trail != 0x82 {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
        c.cycles += 3;
        let mut a1: u32 = 0;
        if lead.wrapping_sub(0x4f) >= 0xa {
            c.cycles += 4;
            if lead.wrapping_sub(0x60) < 0x1a {
                a1 = 1;
            } else {
                c.cycles += 6;
                let v = lead.wrapping_add(0x7f) & 0xff;
                a1 = if 0x19 < v { 0 } else { 2 };
            }
            c.cycles += 1;
            a1 <<= 1;
        }
        c.g[A1][0] = a1 as u64;
        c.cycles += 10;
        let gp = c.g[GP][0] as u32;
        c.g[V0][0] = sx(gp.wrapping_add(0x7a0).wrapping_add(a1));
        c.g[A0][0] = sx(gp.wrapping_add(0x7a1));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lbu(c);
        let a1b = a1.wrapping_add(gp.wrapping_add(0x7a1));
        c.g[A1][0] = sx(a1b);
        c.g[V0][0] = Addr(a1b).lbu(c);
        c.g[V1][0] = sx(lead.wrapping_sub(c.g[V1][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32)) & 0xff;
        abi::ret(c);
        return;
    }
    c.cycles += 5;
    c.g[V1][0] = sx(0x440000u32.wrapping_add(0x5998).wrapping_add(lead));
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_sub(0x40)).lbu(c);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Memory-card port accessors.

/// `int cPS2MemCard_GetPortError(cPS2MemCard* mc, int port)` (0x2c51d0):
/// classify the port's status word into an error code (-10 when unknown).
pub fn cps2memcard_get_port_error(c: &mut Cpu) {
    c.cycles += 8;
    let base = c.g[A0][0] as u32;
    let port = c.g[A1][0] as u32;
    c.g[V0][0] = 0x14;
    c.g[A2][0] = c.g[A0][0];
    let prod = abi::mult(c, port, 0x14);
    c.g[V1][0] = sx(prod);
    c.g[V0][0] = sx(prod.wrapping_add(base));
    let e = Addr(c.g[V0][0] as u32).off(0x184).lw(c);
    c.g[A0][0] = e;
    c.g[V1][0] = ((e as i64) < -7) as u64;
    c.g[V0][0] = ((e as i64) < -10) as u64;
    let retry = if c.g[V1][0] == 0 {
        // status >= -7
        c.cycles += 2;
        c.g[V1][0] = 0x14;
        if (e as i64) > 0 {
            false
        } else {
            c.cycles += 1;
            let p = abi::mult(c, c.g[A1][0] as u32, 0x14);
            c.g[V0][0] = sx(p);
            true
        }
    } else {
        c.cycles += 2;
        let v0_zero = (e as i64) >= -10;
        let below10k = (e as i64) < -10000;
        if v0_zero {
            false
        } else {
            c.cycles += 2;
            if !below10k {
                false
            } else {
                c.cycles += 2;
                c.g[V1][0] = 0x14;
                if (e as i64) < -10002 {
                    false
                } else {
                    c.cycles += 2;
                    let p = abi::mult(c, c.g[A1][0] as u32, 0x14);
                    c.g[V0][0] = sx(p);
                    true
                }
            }
        }
    };
    if !retry {
        c.cycles += 2;
        c.g[V0][0] = 0xffff_ffff_ffff_fff6;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x184).lw(c);
    abi::ret(c);
}

/// `void cPS2MemCard_ClearPortInfo(cPS2MemCard* mc)` (0x2c5250): clear the two
/// words at the current port's +0x17c/+0x180.
pub fn cps2memcard_clear_port_info(c: &mut Cpu) {
    c.cycles += 10;
    let base = c.g[A0][0] as u32;
    let h = Addr(base).off(0xc).lw(c);
    c.g[V0][0] = h;
    c.g[A1][0] = 0x14;
    let p1 = abi::mult(c, h as u32, 0x14);
    c.g[V1][0] = sx(p1);
    c.g[V0][0] = sx(p1.wrapping_add(base));
    Addr(c.g[V0][0] as u32).off(0x17c).sw(c, 0);
    let h2 = Addr(base).off(0xc).lw(c);
    c.g[V1][0] = h2;
    let p2 = abi::mult(c, h2 as u32, 0x14);
    c.g[V0][0] = sx(p2);
    c.g[V1][0] = sx(p2.wrapping_add(base));
    Addr(c.g[V1][0] as u32).off(0x180).sw(c, 0);
    abi::ret(c);
}

/// `int cPS2MemCard_GetFreeSpaceForPort(cPS2MemCard* mc, int port)`
/// (0x2c5320): read the port's free-space word at +0x180.
pub fn cps2memcard_get_free_space_for_port(c: &mut Cpu) {
    c.cycles += 5;
    let base = c.g[A0][0] as u32;
    c.g[V0][0] = 0x14;
    let prod = abi::mult(c, c.g[A1][0] as u32, 0x14);
    c.g[V1][0] = sx(prod);
    c.g[A0][0] = sx(prod.wrapping_add(base));
    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x180).lw(c);
    abi::ret(c);
}

/// `void cPS2MemCard_MarkSaveInvalid(cPS2MemCard* mc, int port)`
/// (0x2c5570): set the port's invalid flag at +0x1a8.
pub fn cps2memcard_mark_save_invalid(c: &mut Cpu) {
    c.cycles += 6;
    let base = c.g[A0][0] as u32;
    c.g[V0][0] = 0x78;
    c.g[V1][0] = 0x1;
    let prod = abi::mult(c, c.g[A1][0] as u32, 0x78);
    c.g[A2][0] = sx(prod);
    c.g[A0][0] = sx(prod.wrapping_add(base));
    Addr(c.g[A0][0] as u32).off(0x1a8).sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `bool cPS2MemCard_IsMcError(int err)` (0x2c63e8): whether `err` is in the
/// MC error range `(-0x270f, -0x2709)`.
pub fn cps2memcard_is_mc_error(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x270f));
    c.g[V0][0] = (c.g[V0][0] < 0x2706) as u64;
    abi::ret(c);
}

/// `int cPS2MemCard_GetPortChangedFlag(cPS2MemCard* m, int port)` (0x2c53c8):
/// whether a card port (0x14-byte slots, flag at +0x18c) changed.
pub fn cps2_mem_card_get_port_changed_flag(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x14;
    c.mult(0, V1, A1, V0); // v1 = port * 0x14
    let selfp = c.g[A0][0] as u32;
    c.g[A0][0] = sx(selfp.wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18c));
    c.g[V0][0] = (0 < c.g[V0][0]) as u64;
    abi::ret(c);
}

/// `void cPS2MemCard_v39(this, const char *name, int flag)` (0x002c66b8):
/// cache `name` at `+0x14c0` when `flag == 0`.
pub fn c_ps2_mem_card_v39(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        let named = c.g[A2][0] != 0;
        frame.save_ra(c, 0);
        if !named {
            c.cycles += 2;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14c0));
            call_ptr(c, callee::STRLEN_AND_STRCPY, 0x002c_66cc)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_ClearDirEntry(cPS2MemCard *this)` (0x2c52d8): zero the
/// 0x40-byte directory entry at `this + 0x480`.
pub fn c_ps2_mem_card_clear_dir_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x480));
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x40;
        call_ptr(c, table_callee::EA_MEMSET, 0x002c_52f0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared body of the two `cPS2MemCard` string accessors: build the UTF-16
/// string for the ASCII text in `a2` (the `a0`/`a1` arguments are unused).
pub(crate) fn mem_card_string(c: &mut Cpu, ret: u32) -> Flow {
    c.cycles += 9;
    let frame = Frame::open(c, 0x10);
    c.g[A1][0] = MESSAGE_MAP as u64;
    frame.save_ra(c, 0);
    c.g[A0][0] = c.g[A2][0];
    c.g[A1][0] = sx(MESSAGE_MAP.wrapping_add(0x3968));
    callee::USTR_FROM_ASCII_2.call(c, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `UStr cPS2MemCard_v15(cPS2MemCard *this, int unused, char *ascii)`
/// (0x2c64a8): `UStr_FromAscii_2(ascii, D_004A3968)`.
pub fn c_ps2_mem_card_v15(c: &mut Cpu) {
    guest(c, |c| mem_card_string(c, 0x002c_64c0))
}

/// `UStr cPS2MemCard_v16(cPS2MemCard *this, int unused, char *ascii)`
/// (0x2c64d0): `UStr_FromAscii_2(ascii, D_004A3968)`.
pub fn c_ps2_mem_card_v16(c: &mut Cpu) {
    guest(c, |c| mem_card_string(c, 0x002c_64e8))
}

// ---------------------------------------------------------------------------
// MemCard and menus.

/// `void cPS2MemCard_GetDirEntryName(self, int i, char* dst)` (0x2c68c8):
/// copy the `i`-th 64-byte directory entry name (at `+0x4e0`) into `dst`.
pub fn cps2_memcard_get_dir_entry_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 6);
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(abi::arg(c, 0)));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A2][0];
        let src = c.g[A1][0] as u32;
        c.g[A1][0] = sx(at(src, 0x4e0));
        call_ptr(c, callee::STRLEN_COPY, 0x002c_68e4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Allocating constructors.

/// `new (size, name, flags)` then construct: the shape of every `Create` in
/// this module (`EA_MemAllocNamed` then the class constructor).
pub(crate) fn create_with<F>(
    c: &mut Cpu,
    size: u32,
    name: u32,
    flags: u32,
    ret: u32,
    ctor: F,
) -> Flow
where
    F: FnOnce(&mut Cpu, u32) -> Flow,
{
    c.cycles += 13;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[A1][0] = sx(name);
    c.g[A2][0] = sx(flags);
    c.g[A3][0] = 0;
    c.g[A0][0] = sx(size);
    callee::EA_MEM_ALLOC.call(c, ret)?;
    c.g[A0][0] = c.g[V0][0];
    ctor(c, ret.wrapping_add(8))?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// cPS2MemCard.

/// `void* cPS2MemCard_Create()` (0x2c22c8): allocate the 0x1500-byte memory
/// card object (`0x100` alignment) and construct it.
pub fn c_ps2_mem_card_create(c: &mut Cpu) {
    guest(c, |c| {
        create_with(c, 0x1500, 0x0048_5ff0, 0x100, 0x002c_22e8, |c, ret| {
            callee::MEM_CARD_CTOR.call(c, ret)
        })
    })
}

/// `void cPS2MemCard_Destroy(cPS2MemCard* card)` (0x2c2300): the deleting
/// destructor through the object's vtable (slot +8, flag 3).
pub fn c_ps2_mem_card_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let obj = c.g[A0][0];
        c.g[A2][0] = obj;
        frame.save_ra(c, 0);
        if obj != 0 {
            c.cycles += 6;
            let vt = Addr(obj as u32).lw(c);
            c.g[V0][0] = vt;
            c.g[A1][0] = 3;
            let m = VSlot::read(c, vt, 8);
            c.g[V1][0] = m.func;
            c.g[A0][0] = m.this(obj);
            call_ptr(c, m.func as u32, 0x002c_2328)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* cPS2MemCard_ctor(cPS2MemCard* card)` (0x2c3fa8): install the class
/// vtable and initialise the card state.
pub fn c_ps2_mem_card_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = sx(0x0048_6f78);
        frame.save(c, S0, 0x10);
        let obj = c.g[A0][0];
        c.g[S0][0] = obj;
        frame.save_ra(c, 0);
        Addr(obj as u32).sw(c, c.g[V0][0]);
        callee::MEM_CARD_INIT.call(c, 0x002c_3fc8)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_RequestListDir(cPS2MemCard* card, const char* name)`
/// (0x2c41d8): save the directory name and switch the card to state 5.
pub fn c_ps2_mem_card_request_list_dir(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let obj = c.g[A0][0];
        c.g[S0][0] = obj;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((obj as u32).wrapping_add(0x139));
        callee::STRCPY.call(c, 0x002c_41f0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 5;
        callee::MEM_CARD_SET_STATE.call(c, 0x002c_41fc)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_RequestDelete(cPS2MemCard* card, const char* name)`
/// (0x2c6ce0): save the name and switch the card to state 0x4d (delete).
pub fn c_ps2_mem_card_request_delete(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let obj = c.g[A0][0];
        c.g[S0][0] = obj;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((obj as u32).wrapping_add(0xf9));
        callee::STRCPY.call(c, 0x002c_6cf8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x4d;
        callee::MEM_CARD_SET_STATE.call(c, 0x002c_6d04)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Strings.

/// `int UStr_Len_2(const u16* s)` (0x2c24d0): the length in UTF-16 units
/// (`lhu`, two bytes per unit), counting the first unit and then while the
/// units are non-zero.
pub fn ustr_len_2(c: &mut Cpu) {
    const HEAD: u32 = 0x002c_24e0;
    c.cycles += 4;
    let mut p = abi::arg(c, 0);
    let mut ch = c.lhu(p);
    c.g[V0][0] = ch;
    let mut n = 0u32;
    c.g[V1][0] = 0;
    p = p.wrapping_add(2);
    c.g[A0][0] = sx(p);
    if ch != 0 {
        loop {
            c.cycles += 7;
            ch = c.lhu(p);
            c.g[V0][0] = ch;
            n = n.wrapping_add(1);
            c.g[V1][0] = sx(n);
            let more = ch != 0;
            p = p.wrapping_add(2);
            c.g[A0][0] = sx(p);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, HEAD);
                p = r32(c, A0);
                n = r32(c, V1);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[V1][0];
    abi::ret(c);
}

/// `void Str_ToLowerInPlace(char* s)` (0x2c2868): lower-case the ASCII
/// letters of a zero-terminated string.
///
/// The original is a dispatcher: it tests the byte, folds `A`-`Z`, stores it
/// back and loads the next one, and an unwind after the back-edge event check
/// re-enters it at the folding block with the current byte in `v1` and the
/// pointer in `a0`.
pub fn str_to_lower_in_place(c: &mut Cpu) {
    const HEAD: u32 = 0x002c_2870;
    // Re-entered at the folding block after an event asked the thread to
    // unwind (`pc` is the loop head, `a0`/`v1` hold the loop state).
    let mut resumed = c.pc == HEAD;
    if !resumed {
        c.cycles += 2;
        c.g[V0][0] = Addr(abi::arg(c, 0)).lb(c);
    }
    loop {
        if !resumed {
            // The zero test and the (unsigned) reload of the current byte.
            c.cycles += 2;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = Addr(r32(c, A0)).lbu(c);
            if !more {
                break;
            }
            if back_edge_unwinds(c, HEAD) {
                return;
            }
        }
        resumed = false;
        // Fold `A`-`Z` (`v1 - 'A' < 26`), leaving `v1 + 0x20` in v0.
        c.cycles += 4;
        c.g[V0][0] = sx((r32(c, V1).wrapping_sub(0x41) < 0x1a) as u32);
        let upper = c.g[V0][0] != 0;
        c.g[V0][0] = sx(r32(c, V1).wrapping_add(0x20));
        if upper {
            c.cycles += 1;
            Addr(r32(c, A0)).sb(c, c.g[V0][0]);
        }
        // Advance and load the next byte.
        c.cycles += 2;
        let p = r32(c, A0).wrapping_add(1);
        c.g[A0][0] = sx(p);
        c.g[V0][0] = Addr(p).lb(c);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `sceMcClose` (0x409e70).
pub(crate) const SCE_MC_CLOSE: u32 = 0x0040_9e70;

// ---------------------------------------------------------------------------
// cPS2MemCard request wrappers.

/// `void cPS2MemCard_RequestListDirAll(this, const char* name)` (0x2c6810):
/// clear the result field at `+0x54`, copy `name` to the work buffer at
/// `+0x139` and enter state 0x3c.
pub fn c_ps2_mem_card_request_list_dir_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.sw((this as u32).wrapping_add(0x54), 0);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x139));
        call_ptr(c, STRLEN_AND_STRCPY, 0x002c_682c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x3c;
        callee::MEMCARD_SET_STATE.call(c, 0x002c_6838)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_RequestOpen(this, const char* name, int mode)`
/// (0x2c6968): store `mode` at `+0x14e0`, copy the name to `+0xf9` and enter
/// state 0x3f.
pub fn c_ps2_mem_card_request_open(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        let mode = c.g[A2][0];
        c.sw((this as u32).wrapping_add(0x14e0), mode);
        c.g[A0][0] = sx((this as u32).wrapping_add(0xf9));
        call_ptr(c, STRLEN_AND_STRCPY, 0x002c_6984)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x3f;
        callee::MEMCARD_SET_STATE.call(c, 0x002c_6990)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_DoClose(this)` (0x2c6a28): close the file descriptor at
/// `+0x18`, keep the result in the `gp + 0x850` slot and enter state 0x4a.
pub fn c_ps2_mem_card_do_close(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[A0][0] = Addr((this as u32).wrapping_add(0x18)).lw(c);
        call_ptr(c, SCE_MC_CLOSE, 0x002c_6a40)?;
        c.g[A0][0] = c.g[S0][0];
        let result = c.g[V0][0];
        let slot = gp(c, 0x850);
        c.sw(slot, result);
        c.g[A1][0] = 0x4a;
        callee::MEMCARD_SET_STATE.call(c, 0x002c_6a50)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_RequestMkdir(this, const char* name, int mode)`
/// (0x2c6c50): store `mode` at `+0x20`, copy the name to `+0xf9` and enter
/// state 0x4b.
pub fn c_ps2_mem_card_request_mkdir(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        let mode = c.g[A2][0];
        c.sw((this as u32).wrapping_add(0x20), mode);
        c.g[A0][0] = sx((this as u32).wrapping_add(0xf9));
        call_ptr(c, STRLEN_AND_STRCPY, 0x002c_6c6c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x4b;
        callee::MEMCARD_SET_STATE.call(c, 0x002c_6c78)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `unsigned short* UStr_FromAscii_2(unsigned short* dst, const char* src)`
/// (0x2C2540): copy a NUL-terminated byte string into `dst` as 16-bit
/// characters (sign-extending each byte) and terminate it; returns `dst`.
pub fn ustr_from_ascii_2(c: &mut Cpu) {
    c.cycles += 4;
    let mut src = r(c, A1);
    let first = Addr(src).lb(c);
    c.g[V0][0] = first;
    c.g[A2][0] = c.g[A0][0];
    c.g[V1][0] = Addr(src).lbu(c);
    let mut dst = r(c, A0);
    if first != 0 {
        loop {
            c.cycles += 8;
            c.g[V0][0] = sx(r(c, V1) << 24);
            src = src.wrapping_add(1);
            c.g[A1][0] = sx(src);
            c.g[V0][0] = sx(((r(c, V0) as i32) >> 24) as u32);
            let ch = c.g[V0][0];
            Addr(dst).sh(c, ch);
            dst = dst.wrapping_add(2);
            c.g[A0][0] = sx(dst);
            c.g[V0][0] = Addr(src).lb(c);
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = Addr(src).lbu(c);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002c_2550);
            }
        }
    }
    c.cycles += 3;
    Addr(dst).sh(c, 0);
    c.g[V0][0] = c.g[A2][0];
    abi::ret(c);
}

/// `unsigned short* UStr_CopyN(unsigned short* dst, const unsigned short* src,
/// int n)` (0x2C25B8): copy at most `n` 16-bit characters and terminate.
pub fn ustr_copy_n(c: &mut Cpu) {
    c.cycles += 3;
    let mut src = r(c, A1);
    let first = Addr(src).lhu(c);
    c.g[V1][0] = first;
    let dst0 = c.g[A0][0];
    c.g[V0][0] = dst0;
    let mut dst = dst0 as u32;
    if first != 0 {
        c.cycles += 2;
        if (c.g[A2][0] as i64) > 0 {
            c.cycles += 1;
            let ch = c.g[V1][0];
            Addr(dst).sh(c, ch);
            loop {
                c.cycles += 5;
                src = src.wrapping_add(2);
                c.g[A1][0] = sx(src);
                c.g[V1][0] = Addr(src).lhu(c);
                dst = dst.wrapping_add(2);
                c.g[A0][0] = sx(dst);
                let at_end = c.g[V1][0] == 0;
                c.g[A2][0] = sx(r(c, A2).wrapping_sub(1));
                if at_end {
                    break;
                }
                c.cycles += 2;
                if (c.g[A2][0] as i64) > 0 {
                    let ch = c.g[V1][0];
                    Addr(dst).sh(c, ch);
                } else {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002c_25d0);
                }
            }
        }
    }
    c.cycles += 2;
    Addr(dst).sh(c, 0);
    abi::ret(c);
}

/// `void cPS2MemCard_RequestListDirW(cPS2MemCard* self, const u16* name)`
/// (0x2c4198): convert the name to ASCII and ask the virtual slot 6 of
/// `*self` for the directory listing.
pub fn c_ps2_mem_card_request_list_dir_w(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x410);
        frame.save(c, S0, 0x400);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x3f0);
        c.g[A0][0] = c.g[SP][0];
        callee::USTR_TO_ASCII.call(c, 0x002c_41b0)?;
        c.g[V1][0] = Addr(r32(c, S0)).lw(c);
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = Addr(r32(c, V1)).off(0x30).lh(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x34).lw(c);
        let func = r32(c, V0);
        c.g[RA][0] = sx(0x002c_41c8);
        c.g[A0][0] = add32(c.g[S0][0], c.g[A0][0]);
        c.pc = func;
        call_indirect(c, func);
        if c.pc != 0x002c_41c8 {
            return Err(crate::call::Unwound);
        }
        frame.restore(c, S0, 0x400);
        frame.restore_ra(c, 0x3f0);
        frame.close(c);
        Ok(())
    })
}

/// `void UStr_Cat(dst, src)` (0x2c2688): append the string `src` at the end of
/// the string `dst` (16-bit characters, walked by `lhu` in 2-byte steps).
pub fn ustr_cat(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(abi::r32(c, A0)).lhu(c);
        let empty = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        if !empty {
            c.cycles += 2;
            c.g[A0][0] = sx(abi::r32(c, A0).wrapping_add(2));
            loop {
                c.cycles += 7;
                c.g[V0][0] = Addr(abi::r32(c, A0)).lhu(c);
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A0][0] = sx(abi::r32(c, A0).wrapping_add(2));
                if event_due(c) {
                    deliver_event_here(c, 0x002c_26a0);
                }
            }
        }
        c.cycles += 5;
        callee::USTR_COPY.call(c, 0x002c_26c4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_DoFlush(void* self)` (0x2c6b00): flush the memcard and
/// move the object to the done (or error) state.
pub fn c_ps2_mem_card_do_flush(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(this).off(0x18).lw(c);
        call_ptr(c, table::SCE_MC_FLUSH, 0x002c_6b18)?;
        let ok = (c.g[V0][0] as i64) >= 0;
        Addr(gp_2(c, 0x850)).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        if ok {
            c.cycles += 4;
            c.g[A1][0] = 0x48;
            callee::CPS2_MEMCARD_SET_STATE.call(c, 0x002c_6b40)?;
        } else {
            c.cycles += 5;
            c.g[A1][0] = 0x1;
            callee::CPS2_MEMCARD_SET_STATE.call(c, 0x002c_6b2c)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LocStringTable_dtor(LocStringTable* self, int flags)`
/// (0x2c6d70): free the owned table and, when `flags & 1`, the object.
pub fn loc_string_table_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = Addr(this).off(4).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            call_ptr(c, table::MEM_FREE, 0x002c_6d98)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, table::BUILTIN_DELETE, 0x002c_6dac)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Memcards, menus and world.

/// `void MemCard_AsciiToSjisString(src, dst)` (0x2c2210): convert a
/// NUL-terminated ASCII string to Shift-JIS.
pub fn mem_card_ascii_to_sjis_string(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S0)).lb(c);
        let empty = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !empty {
            loop {
                c.cycles += 7;
                c.g[A0][0] = Addr(r32(c, S0)).lbu(c);
                c.g[RA][0] = sx(0x002c_223c);
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                callee::MEM_CARD_ASCII_CHAR_TO_SJIS.call(c, 0x002c_223c)?;
                Addr(r32(c, S1)).sh(c, c.g[V0][0]);
                c.g[V0][0] = Addr(r32(c, S0)).lb(c);
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(2));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002c_2230);
                }
            }
        }
        c.cycles += 6;
        Addr(r32(c, S1)).sh(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_DoFormat(self)` (0x2c4388): format the card and go to
/// the done/failed state.
pub fn c_ps2_mem_card_do_format(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[RA][0] = sx(0x002c_43a4);
        c.g[A0][0] = Addr(r32(c, S0)).off(0xc).lw(c);
        call_ptr(c, callee::SCE_MC_FORMAT, 0x002c_43a4)?;
        let ok = (c.g[V0][0] as i64) >= 0;
        Addr(gp_3(c, 0x850)).sw(c, c.g[V0][0]);
        if ok {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x002c_43cc);
            c.g[A1][0] = 0x12;
            callee::C_PS2_MEM_CARD_SET_STATE.call(c, 0x002c_43cc)?;
        } else {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x002c_43b8);
            c.g[A1][0] = 1;
            callee::C_PS2_MEM_CARD_SET_STATE.call(c, 0x002c_43b8)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_v34(self, name)` (0x2c4520): copy the save name, format
/// the full path and go to state 0x14.
pub fn c_ps2_mem_card_v34(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = sx(r32(c, S1).wrapping_add(0xb9));
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x002c_4540);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::STRCPY_2, 0x002c_4540)?;
        c.g[A1][0] = 0x4a_0000;
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(0xf9));
        c.g[RA][0] = sx(0x002c_4554);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3950));
        call_ptr(c, callee::SPRINTF, 0x002c_4554)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x002c_4560);
        c.g[A1][0] = 0x14;
        callee::C_PS2_MEM_CARD_SET_STATE.call(c, 0x002c_4560)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_v35(self, name)` (0x2c4578): convert the wide save name
/// to ASCII, format the full path and go to state 0x14.
pub fn c_ps2_mem_card_v35(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = sx(r32(c, S1).wrapping_add(0xb9));
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x002c_4598);
        c.g[A0][0] = c.g[S0][0];
        callee::USTR_TO_ASCII.call(c, 0x002c_4598)?;
        c.g[A1][0] = 0x4a_0000;
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(0xf9));
        c.g[RA][0] = sx(0x002c_45ac);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3950));
        call_ptr(c, callee::SPRINTF, 0x002c_45ac)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x002c_45b8);
        c.g[A1][0] = 0x14;
        callee::C_PS2_MEM_CARD_SET_STATE.call(c, 0x002c_45b8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_v29(self, a1, a2, a3)` (0x2c4800): build the save name
/// and request a read of the save.
pub fn c_ps2_mem_card_v29(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x50);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0x40);
        c.g[S2][0] = c.g[A3][0];
        c.g[RA][0] = sx(0x002c_4828);
        c.g[A2][0] = sx(c.g[SP][0] as u32);
        callee::C_PS2_MEM_CARD_GET_SAVE_NAME.call(c, 0x002c_4828)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x002c_483c);
        c.g[A1][0] = sx(c.g[SP][0] as u32);
        callee::C_PS2_MEM_CARD_REQUEST_READ_SAVE.call(c, 0x002c_483c)?;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_DoSeek(MemCard* self)` (0x2c6a88): start the seek phase,
/// then advance the state machine to the result.
pub fn c_ps2_mem_card_do_seek(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        c.g[A0][0] = Addr(this).off(0x18).lw(c);
        c.g[A1][0] = Addr(this).off(0x14e4).lw(c);
        c.g[A2][0] = Addr(this).off(0x14e8).lw(c);
        c.g[RA][0] = sx(0x002c_6aa8);
        call_ptr(c, callee::SCE_MC_SEEK, 0x002c_6aa8)?;
        let failed = (c.g[V0][0] as i64) < 0;
        Addr(gp_4(c, 0x850)).sw(c, c.g[V0][0]);
        if failed {
            c.cycles += 5;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = 1;
            c.g[RA][0] = sx(0x002c_6abc);
            callee::CPS2_MEM_CARD_SET_STATE.call(c, 0x002c_6abc)?;
            frame.restore(c, S0, 0x10);
        } else {
            c.g[A0][0] = sx(this);
            c.g[A1][0] = 0x42;
            c.g[RA][0] = sx(0x002c_6ad0);
            callee::CPS2_MEM_CARD_SET_STATE.call(c, 0x002c_6ad0)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_DoRead(MemCard* self)` (0x2c6b78): start the read phase.
pub fn c_ps2_mem_card_do_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        c.g[A0][0] = Addr(this).off(0x18).lw(c);
        c.g[A1][0] = Addr(this).off(0x2c).lw(c);
        c.g[A2][0] = Addr(this).off(0x24).lw(c);
        c.g[RA][0] = sx(0x002c_6b98);
        call_ptr(c, callee::SCE_MC_READ, 0x002c_6b98)?;
        let failed = (c.g[V0][0] as i64) < 0;
        Addr(gp_4(c, 0x850)).sw(c, c.g[V0][0]);
        if failed {
            c.cycles += 5;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = 1;
            c.g[RA][0] = sx(0x002c_6bac);
            callee::CPS2_MEM_CARD_SET_STATE.call(c, 0x002c_6bac)?;
            frame.restore(c, S0, 0x10);
        } else {
            c.g[A0][0] = sx(this);
            c.g[A1][0] = 0x44;
            c.g[RA][0] = sx(0x002c_6bc0);
            callee::CPS2_MEM_CARD_SET_STATE.call(c, 0x002c_6bc0)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_DoWrite(MemCard* self)` (0x2c6bf8): start the write phase.
pub fn c_ps2_mem_card_do_write(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        c.g[A0][0] = Addr(this).off(0x18).lw(c);
        c.g[A1][0] = Addr(this).off(0x2c).lw(c);
        c.g[A2][0] = Addr(this).off(0x24).lw(c);
        c.g[RA][0] = sx(0x002c_6c18);
        call_ptr(c, callee::SCE_MC_WRITE, 0x002c_6c18)?;
        let failed = (c.g[V0][0] as i64) < 0;
        Addr(gp_4(c, 0x850)).sw(c, c.g[V0][0]);
        if failed {
            c.cycles += 5;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = 1;
            c.g[RA][0] = sx(0x002c_6c2c);
            callee::CPS2_MEM_CARD_SET_STATE.call(c, 0x002c_6c2c)?;
            frame.restore(c, S0, 0x10);
        } else {
            c.g[A0][0] = sx(this);
            c.g[A1][0] = 0x46;
            c.g[RA][0] = sx(0x002c_6c40);
            callee::CPS2_MEM_CARD_SET_STATE.call(c, 0x002c_6c40)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_DoMkdir(MemCard* self)` (0x2c6c88): start creating the
/// save directory.
pub fn c_ps2_mem_card_do_mkdir(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        c.g[A2][0] = sx(this.wrapping_add(0xf9));
        c.g[A0][0] = Addr(this).off(0xc).lw(c);
        c.g[RA][0] = sx(0x002c_6ca8);
        call_ptr(c, callee::SCE_MC_MKDIR, 0x002c_6ca8)?;
        let failed = (c.g[V0][0] as i64) < 0;
        Addr(gp_4(c, 0x850)).sw(c, c.g[V0][0]);
        if failed {
            c.cycles += 5;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = 1;
            c.g[RA][0] = sx(0x002c_6cbc);
            callee::CPS2_MEM_CARD_SET_STATE.call(c, 0x002c_6cbc)?;
            frame.restore(c, S0, 0x10);
        } else {
            c.g[A0][0] = sx(this);
            c.g[A1][0] = 0x4c;
            c.g[RA][0] = sx(0x002c_6cd0);
            callee::CPS2_MEM_CARD_SET_STATE.call(c, 0x002c_6cd0)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_DoDelete(MemCard* self)` (0x2c6d18): start deleting the
/// save directory.
pub fn c_ps2_mem_card_do_delete(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        c.g[A2][0] = sx(this.wrapping_add(0xf9));
        c.g[A0][0] = Addr(this).off(0xc).lw(c);
        c.g[RA][0] = sx(0x002c_6d38);
        call_ptr(c, callee::SCE_MC_DELETE, 0x002c_6d38)?;
        let failed = (c.g[V0][0] as i64) < 0;
        Addr(gp_4(c, 0x850)).sw(c, c.g[V0][0]);
        if failed {
            c.cycles += 5;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = 1;
            c.g[RA][0] = sx(0x002c_6d4c);
            callee::CPS2_MEM_CARD_SET_STATE.call(c, 0x002c_6d4c)?;
            frame.restore(c, S0, 0x10);
        } else {
            c.g[A0][0] = sx(this);
            c.g[A1][0] = 0x4e;
            c.g[RA][0] = sx(0x002c_6d60);
            callee::CPS2_MEM_CARD_SET_STATE.call(c, 0x002c_6d60)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Memory-card port/slot state.

/// `void cPS2MemCard_ClearPortChanged(self)` (0x2c44e0): clear the port's
/// `+0x18c` field of the current level and, if it was -1, its `+0x184` field.
pub fn c_ps2_mem_card_clear_port_changed(c: &mut Cpu) {
    c.cycles += 13;
    let this = abi::arg(c, 0) as u32;
    c.g[V1][0] = Addr(this).off(0xc).lw(c);
    c.g[A1][0] = 0x14;
    c.g[A2][0] = sx(this.wrapping_add(0x184));
    c.g[A3][0] = 0xffff_ffff_ffff_ffff;
    c.mult(0, V0, V1, A1);
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(this));
    let level = c.g[V1][0] as u32;
    Addr(level).off(0x18c).sw(c, 0);
    c.g[V0][0] = Addr(this).off(0xc).lw(c);
    c.mult(0, V0, V0, A1);
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let word = c.g[A2][0] as u32;
    c.g[V1][0] = Addr(word).lw(c);
    if c.g[V1][0] == c.g[A3][0] {
        Addr(word).sw(c, 0);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `bool cPS2MemCard_IsState34(self)` (0x2c4748): state word `+4` equals 0x34.
pub fn c_ps2_mem_card_is_state34(c: &mut Cpu) {
    c.cycles += 4;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = this.off(0x4).lw(c);
    c.g[V0][0] ^= 0x34;
    c.g[V0][0] = (c.g[V0][0] < 0x1) as u64;
    abi::ret(c);
}

/// `bool cPS2MemCard_IsState35(self)` (0x2c4880): state word `+4` equals 0x35.
pub fn c_ps2_mem_card_is_state35(c: &mut Cpu) {
    c.cycles += 4;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = this.off(0x4).lw(c);
    c.g[V0][0] ^= 0x35;
    c.g[V0][0] = (c.g[V0][0] < 0x1) as u64;
    abi::ret(c);
}

/// `bool cPS2MemCard_IsBusy(self)` (0x2c5140): busy while the state word at
/// `+4` or the flag at `+0x40` is non-zero.
pub fn c_ps2_mem_card_is_busy(c: &mut Cpu) {
    'busy: {
        c.cycles += 3;
        let this = Addr(abi::arg(c, 0));
        c.g[V0][0] = this.off(0x4).lw(c);
        if c.g[V0][0] != 0 {
            break 'busy;
        }
        c.cycles += 3;
        c.g[V0][0] = this.off(0x40).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
        break 'busy;
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `bool cPS2MemCard_IsPortCardError(self)` (0x2c5168): whether the current
/// level's error word is one of -1, -3, -8..-10, -10001, -10002.
pub fn c_ps2_mem_card_is_port_card_error(c: &mut Cpu) {
    'outer: {
        'inner: {
            c.cycles += 8;
            let this = abi::arg(c, 0) as u32;
            c.g[V0][0] = Addr(this).off(0xc).lw(c);
            c.g[V1][0] = 0x14;
            c.mult(0, A1, V0, V1);
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(this));
            let level = c.g[V0][0] as u32;
            c.g[A0][0] = Addr(level).off(0x184).lw(c);
            c.g[V1][0] = ((c.g[A0][0] as i64) < -7) as u64;
            let ge_minus7 = c.g[V1][0] == 0;
            c.g[V0][0] = ((c.g[A0][0] as i64) < -10) as u64;
            if ge_minus7 {
                c.cycles += 3;
                c.g[V0][0] = 0xffff_ffff_ffff_fffd;
                let is_minus3 = c.g[A0][0] == c.g[V0][0];
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                if is_minus3 {
                    break 'outer;
                }
                c.cycles += 2;
                let not_minus1 = c.g[A0][0] != c.g[V0][0];
                c.g[V0][0] = 0;
                if not_minus1 {
                    break 'inner;
                }
                break 'outer;
            }
            c.cycles += 2;
            let ge_minus10 = c.g[V0][0] == 0;
            c.g[V0][0] = ((c.g[A0][0] as i64) < -10000) as u64;
            if ge_minus10 {
                break 'outer;
            }
            c.cycles += 2;
            let ge_minus10000 = c.g[V0][0] == 0;
            c.g[V0][0] = ((c.g[A0][0] as i64) < -10002) as u64;
            if ge_minus10000 {
                c.cycles += 1;
                c.g[V0][0] = 0;
                break 'inner;
            }
            c.cycles += 2;
            let lt_minus10002 = c.g[V0][0] != 0;
            c.g[V0][0] = 0;
            if lt_minus10002 {
                break 'inner;
            }
            c.cycles += 2;
            c.g[V0][0] = 1;
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `bool cPS2MemCard_IsPortNotPS2Card(self, port)` (0x2c5300): the port's
/// `+0x184` word equals -10002.
pub fn c_ps2_mem_card_is_port_not_ps2_card(c: &mut Cpu) {
    c.cycles += 8;
    let this = abi::arg(c, 0) as u32;
    c.g[V0][0] = 0x14;
    c.g[V1][0] = 0xffff_ffff_ffff_d8ee;
    c.mult(0, A2, A1, V0);
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(this));
    let level = c.g[A0][0] as u32;
    c.g[V0][0] = Addr(level).off(0x184).lw(c);
    c.g[V0][0] ^= c.g[V1][0];
    c.g[V0][0] = (c.g[V0][0] < 0x1) as u64;
    abi::ret(c);
}

/// `bool cPS2MemCard_v52(self, port)` (0x2c5358): the level error word is -7,
/// -10001 or in {-5..0}.
pub fn c_ps2_mem_card_v52(c: &mut Cpu) {
    'outer: {
        c.cycles += 7;
        let this = abi::arg(c, 0) as u32;
        c.g[V0][0] = 0x14;
        c.g[V1][0] = 0xffff_ffff_ffff_fff9;
        c.mult(0, A2, A1, V0);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(this));
        let level = c.g[A0][0] as u32;
        c.g[A0][0] = Addr(level).off(0x184).lw(c);
        let is_minus7 = c.g[A0][0] == c.g[V1][0];
        c.g[V0][0] = ((c.g[A0][0] as i64) < -6) as u64;
        if is_minus7 {
            break 'outer;
        }
        'inner: {
            'check: {
                c.cycles += 2;
                let ge_minus6 = c.g[V0][0] == 0;
                c.g[V0][0] = 0xffff_ffff_ffff_d8ef;
                if ge_minus6 {
                    break 'check;
                }
                c.cycles += 2;
                let is_minus10001 = c.g[A0][0] == c.g[V0][0];
                c.g[V0][0] = 0;
                if is_minus10001 {
                    break 'outer;
                }
                c.cycles += 2;
                break 'inner;
            }
            c.cycles += 2;
            let positive = (c.g[A0][0] as i64) > 0;
            c.g[V0][0] = 0;
            if positive {
                break 'inner;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A0][0] as i64) < -5) as u64;
            let lt_minus5 = c.g[V0][0] != 0;
            c.g[V0][0] = 0;
            if lt_minus5 {
                break 'inner;
            }
            break 'outer;
        }
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `bool cPS2MemCard_IsPortFormatted(self, port)` (0x2c53b0): the port's
/// `+0x188` word is non-zero.
pub fn c_ps2_mem_card_is_port_formatted(c: &mut Cpu) {
    c.cycles += 6;
    let this = abi::arg(c, 0) as u32;
    c.g[V0][0] = 0x14;
    c.mult(0, V1, A1, V0);
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(this));
    let level = c.g[A0][0] as u32;
    c.g[V0][0] = Addr(level).off(0x188).lw(c);
    c.g[V0][0] = (0 < c.g[V0][0]) as u64;
    abi::ret(c);
}

/// `bool cPS2MemCard_HasSlotName(self, slot)` (0x2c5528): whether the slot's
/// name at `+0x1b0` is non-empty.
pub fn c_ps2_mem_card_has_slot_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let this = abi::arg(c, 0) as u32;
        c.g[V0][0] = 0x78;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        // V1 = slot * 0x78 (the slot stride); V0 keeps 0x78 as the multiplier.
        c.mult(0, V1, A1, V0);
        let sp = c.g[SP][0] as u32;
        c.sd(sp, c.g[RA][0]);
        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(this));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1b0));
        call_ptr(c, 0x0041_6810, 0x002c_5544)?;
        c.g[RA][0] = c.ld(sp);
        c.g[V0][0] = (0 < c.g[V0][0]) as u64;
        c.g[SP][0] = sx(sp.wrapping_add(0x10));
        abi::ret(c);
        Ok(())
    });
}

/// `void cPS2MemCard_MarkSlotUsed(self, slot)` (0x2c5558): set the slot's
/// `+0x1a4` flag.
pub fn c_ps2_mem_card_mark_slot_used(c: &mut Cpu) {
    c.cycles += 6;
    let this = abi::arg(c, 0) as u32;
    c.g[V0][0] = 0x78;
    c.g[V1][0] = 0x1;
    c.mult(0, A2, A1, V0);
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(this));
    let slot = c.g[A0][0] as u32;
    c.g[V1][0] = 0x1;
    Addr(slot).off(0x1a4).sw(c, 1);
    abi::ret(c);
}

/// `bool cPS2MemCard_IsSlotEmpty(self, slot)` (0x2c5588): whether both the
/// `+0x1a8` and `+0x1a4` slot words are zero.
pub fn c_ps2_mem_card_is_slot_empty(c: &mut Cpu) {
    'taken: {
        c.cycles += 7;
        c.g[V1][0] = 0x78;
        c.g[A2][0] = c.g[A0][0];
        c.mult(0, A0, A1, V1);
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        let slot = c.g[V0][0] as u32;
        c.g[V1][0] = Addr(slot).off(0x1a8).lw(c);
        if c.g[V1][0] != 0 {
            break 'taken;
        }
        c.cycles += 3;
        c.g[V1][0] = Addr(slot).off(0x1a4).lw(c);
        if c.g[V1][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
        break 'taken;
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `bool cPS2MemCard_SlotHas1AC(self, slot)` (0x2c55c0): the slot's `+0x1ac`
/// word is non-zero.
pub fn c_ps2_mem_card_slot_has_1ac(c: &mut Cpu) {
    c.cycles += 6;
    let this = abi::arg(c, 0) as u32;
    c.g[V0][0] = 0x78;
    c.mult(0, V1, A1, V0);
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(this));
    let slot = c.g[A0][0] as u32;
    c.g[V0][0] = Addr(slot).off(0x1ac).lw(c);
    c.g[V0][0] = (0 < c.g[V0][0]) as u64;
    abi::ret(c);
}

/// `bool cPS2MemCard_IsPortResultM11(self)` (0x2c66d8): the current level's
/// error word equals -11.
pub fn c_ps2_mem_card_is_port_result_m11(c: &mut Cpu) {
    c.cycles += 9;
    let this = abi::arg(c, 0) as u32;
    c.g[V1][0] = Addr(this).off(0xc).lw(c);
    c.g[V0][0] = 0x14;
    c.g[A1][0] = 0xffff_ffff_ffff_fff5;
    c.mult(0, A2, V1, V0);
    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(this));
    let level = c.g[V1][0] as u32;
    c.g[V0][0] = Addr(level).off(0x184).lw(c);
    c.g[V0][0] ^= c.g[A1][0];
    c.g[V0][0] = (c.g[V0][0] < 0x1) as u64;
    abi::ret(c);
}

/// `int cPS2MemCard_CalcSaveSizeKB(size)` (0x2c6758): the save size in KiB
/// (1 KiB blocks, plus 77 KiB of header/overhead).
pub fn c_ps2_mem_card_calc_save_size_kb(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3ff));
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7fe));
    if c.g[V0][0] != 0 {
        c.g[A1][0] = c.g[V1][0];
    }
    c.g[A1][0] = sx((((c.g[A1][0] as u32) as i32) >> 10) as u32);
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4d));
    abi::ret(c);
}

/// `sw src, gp+off`.
#[inline(always)]
pub(crate) fn gp_sw(c: &mut Cpu, off: i32, v: u64) {
    c.sw(gp_rel(c, off), v);
}

/// `void MemCard_SjisToAsciiString(char* dst, char* src)` (0x2C2268): convert
/// the 16-bit SJIS string at `src` into single-byte characters at `dst`.
///
/// The original is a resumable loop (a delivered event may unwind with `pc`
/// at the conversion body), so the entry point also accepts that resume
/// address, as the generated `match pc` does.
pub fn memcard_sjis_to_ascii_string(c: &mut Cpu) {
    const ENTRY: u32 = 0x002c_2268;
    const CONVERT: u32 = 0x002c_2288;
    const TEST: u32 = 0x002c_22a0;
    const EXIT: u32 = 0x002c_22ac;

    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 7;
                    let frame = Frame::open(c, 0x30);
                    frame.save(c, S0, 0x20);
                    frame.save(c, S1, 0x10);
                    c.g[S0][0] = c.g[A2][0];
                    frame.save_ra(c, 0);
                    c.g[S1][0] = c.g[A1][0];
                    pc = TEST;
                }
                CONVERT => {
                    c.cycles += 6;
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 16);
                    c.g[S0][0] = sx(ea(c.g[S0][0], 2));
                    c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 16) as u32);
                    callee::MEMCARD_SJIS_CHAR_TO_ASCII.call(c, 0x002c_2298)?;
                    c.sb(c.g[S1][0] as u32, c.g[V0][0]);
                    c.g[S1][0] = sx(ea(c.g[S1][0], 1));
                    pc = TEST;
                }
                0x002c_2284 => {
                    // The generated block split (never branched to).
                    c.cycles += 1;
                    pc = CONVERT;
                }
                TEST => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lh(c.g[S0][0] as u32);
                    let more = c.g[V0][0] != 0;
                    c.g[A0][0] = c.lhu(c.g[S0][0] as u32);
                    if !more {
                        pc = EXIT;
                        continue;
                    }
                    if event_due(c) && crate::events::deliver_event(c, CONVERT) {
                        return Ok(());
                    }
                    pc = CONVERT;
                }
                EXIT => {
                    c.cycles += 6;
                    c.sb(c.g[S1][0] as u32, 0);
                    let frame = Frame { size: 0x30 };
                    frame.restore(c, S0, 0x20);
                    frame.restore(c, S1, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, ENTRY, 0x002c_22c4);
                    return Ok(());
                }
            }
        }
    })
}

/// `void cPS2MemCard_RequestGetInfo(cPS2MemCard* self, int op)` (0x2C4480):
/// remember the pending operation and, when the card's `v11()` finds nothing,
/// restart the state machine at state 3.
pub fn ps2_memcard_request_get_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.sw(ea(c.g[S0][0], 0xc), c.g[S1][0]);
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        c.g[A0][0] = c.lh(ea(c.g[V1][0], 0x58));
        c.g[V0][0] = c.lw(ea(c.g[V1][0], 0x5c));
        let method = c.g[V0][0] as u32;
        c.g[A0][0] = sx(ea(c.g[S0][0], c.g[A0][0] as u32));
        call_ptr(c, method, 0x002c_44b0)?;
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            c.sw(ea(c.g[S0][0], 0x10), c.g[S1][0]);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 3;
            callee::PS2_MEMCARD_SET_STATE.call(c, 0x002c_44c8)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cPS2MemCard_DoGetInfo(cPS2MemCard* self)` (0x2C50E0): ask the memory
/// card manager for the card's info and record success at `+0x40`.
pub fn ps2_memcard_do_get_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x20);
        gp_sw(c, 0x848, 0);
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.sw(ea(c.g[S0][0], 0x20), 0);
        c.g[A2][0] = sx(gp_rel(c, 0x848));
        c.g[A0][0] = c.lw(ea(c.g[S0][0], 0xc));
        c.g[A3][0] = sx(gp_rel(c, 0x84c));
        c.g[T0][0] = sx(gp_rel(c, 0x854));
        gp_sw(c, 0x854, 0);
        gp_sw(c, 0x84c, 0);
        call_ptr(c, addr::MC_MANAGER_GET_INFO, 0x002c_5118)?;
        c.g[V1][0] = (c.g[V0][0] < 1) as u64;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 5;
        c.sw(ea(c.g[S0][0], 0x40), c.g[V1][0]);
        c.sw(ea(c.g[S0][0], 0x4c), c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = c.g[V1][0];
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Memory card.

/// `void cPS2MemCard_v38(self, a1, a2, a3)` (0x2c46c0): build the save name
/// and call the virtual save slot.
pub fn ps2_mem_card_v38(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x50);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0x40);
        c.g[S2][0] = c.g[A3][0];
        c.g[A2][0] = c.g[SP][0];
        call_ptr(c, 0x002c_54d0, 0x002c_46e8)?;
        c.g[V1][0] = lw_2(c, c.g[S0][0] as u32);
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = Addr(p(c.g[V1][0], 0x120)).lh(c);
        c.g[V0][0] = lw_2(c, p(c.g[V1][0], 0x124));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x002c_4708)?;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    });
}

/// `void cPS2MemCard_ClearDirCache(cPS2MemCard* self)` (0x2c5278): wipe the
/// directory cache buffers and the current directory entry.
pub fn ps2_mem_card_clear_dir_cache(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0x40;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Addr(p(c.g[S0][0], 0x50)).sw(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x480));
        Addr(p(c.g[S0][0], 0x54)).sw(c, 0);
        call_ptr(c, 0x003e_6448, 0x002c_52a0)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1a4));
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x2d0;
        call_ptr(c, 0x003e_6448, 0x002c_52b0)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4c0));
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x1000;
        call_ptr(c, 0x003e_6448, 0x002c_52c0)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x002c_52d8, 0x002c_52c8)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Memory cards and cameras.

/// `void cPS2MemCard_DoOpen(cPS2MemCard* self)` (0x2c69a0): open the memory
/// card file and enter state 1 (failure) or 0x40 (ready).
pub fn cps2_mem_card_do_open(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A2][0] = sx(p_2(c.g[S0][0] as u32, 0xf9));
        Addr(p_2(c.g[S0][0] as u32, 0x18)).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(p_2(c.g[S0][0] as u32, 0x0c)).lw(c);
        c.g[A3][0] = Addr(p_2(c.g[S0][0] as u32, 0x14e0)).lw(c);
        call_ptr(c, 0x0040_9d10, 0x002c_69cc)?;
        let ok = (c.g[V0][0] as i64) >= 0;
        c.sw(gp(c, 0x850), c.g[V0][0]);
        if ok {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x40;
            call_ptr(c, 0x002c_48c0, 0x002c_69f4)?;
        } else {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x002c_48c0, 0x002c_69e0)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cPS2MemCard_dtor(self, flags)` (0x2c3fe0): close the card, free the buffer,
/// install the base vtable and free the object when asked.
pub fn c_ps2_mem_card_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x480000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6f78));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
        call_ptr(c, 0x002c_6360, 0x002c_4008)?;
        c.g[A0][0] = Addr(p_3(c.g[S0][0] as u32, 0x30)).lw(c);
        let no_buffer = c.g[A0][0] == 0;
        c.g[V0][0] = 0x480000;
        if !no_buffer {
            c.cycles += 3;
            call_ptr(c, 0x0025_23a8, 0x002c_401c)?;
            c.g[V0][0] = 0x480000;
        }
        c.cycles += 4;
        c.g[V1][0] = c.g[S1][0] & 0x1;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x71e8));
        let keep = c.g[V1][0] == 0;
        Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
        if !keep {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x002c_4038)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_DoDeleteSave(cPS2MemCard* m)` (0x2c4410): build the save
/// file name and prompt string, then move to state 0x36 (delete). The
/// generated body's call into the 1200-line memcard state machine cannot be
/// bounded cheaply, so this port has no differential test.
#[allow(dead_code)]
pub fn c_ps2_mem_card_do_delete_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x78;
        sqsp(c, 16, 0x20);
        sqsp(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xb9));
        Addr(p_4(r32(c, S0), 0x3c)).sw(c, 0);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = Addr(p_4(r32(c, S0), 0x1c)).lw(c);
        c.mult(0, V1, A1, V0);
        c.g[A1][0] = abi::addu(c.g[V1][0], c.g[S0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1b0));
        call::call_ptr(c, 0x0041_66f4, 0x002c_4448)?;
        c.g[A2][0] = c.g[S1][0];
        c.g[A1][0] = 0x004a_3948;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xf9));
        c.g[A3][0] = c.g[A2][0];
        call::call_ptr(c, 0x0041_7828, 0x002c_4460)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x36;
        call::call_ptr(c, 0x002c_48c0, 0x002c_446c)?;
        lqsp(c, 16, 0x20);
        lqsp(c, 17, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    });
}

/// `cPS2MemCard_CountSaves` (0x2C68F0): count the memory card's save slots whose
/// name is non-empty.
pub fn cps2_mem_card_count_saves(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_at(c, S2, 0x14);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S3][0] = 0;
        if !empty {
            c.cycles += 1;
            c.g[S1][0] = low(c, S2).wrapping_add(0x1b0);
            loop {
                c.cycles += 9;
                c.g[A0][0] = c.g[S1][0];
                c.g[S3][0] = low(c, S3).wrapping_add(1);
                call_ptr(c, addr::STRLEN, 0x002c_692c)?;
                c.g[S1][0] = low(c, S1).wrapping_add(0x78);
                c.g[A0][0] = lw_at(c, S2, 0x14);
                c.g[V1][0] = low(c, S0).wrapping_add(1);
                c.g[A0][0] = ((c.g[S3][0] as i64) < (c.g[A0][0] as i64)) as u64;
                let more = c.g[A0][0] != 0;
                if c.g[V0][0] != 0 {
                    c.g[S0][0] = c.g[V1][0];
                }
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002c_6920);
                }
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x30);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PS2 memcard save-name stubs (world-painter module).

/// Shared body of the three memcard stubs: build the ASCII copy of the name at
/// `+0xf9`, then record the two payload words and hand `state` to the memcard
/// state machine.
pub(crate) fn mem_card_save_name(
    c: &mut Cpu,
    a2: u64,
    a3: u64,
    state: u64,
    wide: bool,
    rets: [u32; 3],
) -> Flow {
    c.cycles += 30;
    c.g[SP][0] = sx(low_2(c, SP).wrapping_sub(0x50));
    c.sq(16, sp_2(c).wrapping_add(0x40));
    c.g[S0][0] = c.g[A0][0];
    c.sq(17, sp_2(c).wrapping_add(0x30));
    c.sq(18, sp_2(c).wrapping_add(0x20));
    c.g[S1][0] = sx(low_2(c, S0).wrapping_add(0xb9));
    c.sq(19, sp_2(c).wrapping_add(0x10));
    c.g[S2][0] = a2;
    c.sd(sp_2(c), c.g[RA][0]);
    c.g[S3][0] = a3;
    if wide {
        c.g[A0][0] = c.g[S1][0];
        callee::USTR_TO_ASCII_2.call(c, rets[0])?;
    } else {
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, callee::STRCPY_3, rets[0])?;
    }
    c.g[A2][0] = c.g[S1][0];
    c.g[A1][0] = 0x4a_0000;
    c.g[A0][0] = sx(low_2(c, S0).wrapping_add(0xf9));
    c.g[A3][0] = c.g[A2][0];
    c.g[A1][0] = sx(low_2(c, A1).wrapping_add(0x3948));
    call_ptr(c, callee::SPRINTF, rets[1])?;
    sw_r(c, S0, 0x2c, c.g[S2][0]);
    c.g[A0][0] = c.g[S0][0];
    sw_r(c, S0, 0x20, c.g[S3][0]);
    c.g[A1][0] = state;
    callee::CPS2_MEM_CARD_SET_STATE_2.call(c, rets[2])?;
    c.lq(16, sp_2(c).wrapping_add(0x40));
    c.lq(17, sp_2(c).wrapping_add(0x30));
    c.lq(18, sp_2(c).wrapping_add(0x20));
    c.lq(19, sp_2(c).wrapping_add(0x10));
    c.g[RA][0] = Addr(sp_2(c)).ld(c);
    let t = low_2(c, RA);
    c.g[SP][0] = sx(low_2(c, SP).wrapping_add(0x50));
    c.pc = t;
    Ok(())
}

/// `void cPS2MemCard_v37(self, wname, a2, a3)` (0x2c45d0): wide-character name.
pub fn cps2_mem_card_v37(c: &mut Cpu) {
    guest(c, |c| {
        let a2 = c.g[A2][0];
        let a3 = c.g[A3][0];
        mem_card_save_name(
            c,
            a2,
            a3,
            0x15,
            true,
            [0x002c_4600, 0x002c_4618, 0x002c_462c],
        )
    })
}

/// `void cPS2MemCard_v36(self, name, a2, a3)` (0x2c4648): narrow name.
pub fn cps2_mem_card_v36(c: &mut Cpu) {
    guest(c, |c| {
        let a2 = c.g[A2][0];
        let a3 = c.g[A3][0];
        mem_card_save_name(
            c,
            a2,
            a3,
            0x15,
            false,
            [0x002c_4678, 0x002c_4690, 0x002c_46a4],
        )
    })
}

/// `void cPS2MemCard_RequestReadSave(self, name, a2, a3)` (0x2c4788): same,
/// entering the `0x1b` (read) state.
pub fn cps2_mem_card_request_read_save(c: &mut Cpu) {
    guest(c, |c| {
        let a2 = c.g[A2][0];
        let a3 = c.g[A3][0];
        mem_card_save_name(
            c,
            a2,
            a3,
            0x1b,
            false,
            [0x002c_47b8, 0x002c_47d0, 0x002c_47e4],
        )
    })
}

/// `void cPS2MemCard_DoListDir(cPS2MemCard* m)` (0x002c4210): clear the
/// 0x1000-byte directory buffer and ask the IOP for the entries of the
/// current directory; state 1 on failure, state 6 on success.
pub fn c_ps2_mem_card_do_list_dir(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.g[A1][0] = 0;
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.g[A2][0] = 0x1000;
        c.g[S1][0] = c.g[A0][0];
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4c0));
        c.sd(sp(c), c.g[RA][0]);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_6448, 0x002c_4238)?; // EA_MemSet
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[T1][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x139));
        c.g[A3][0] = 0;
        c.g[T0][0] = 6;
        call_ptr(c, 0x0040_a688, 0x002c_4254)?; // sceMcGetDir
        let failed = (c.g[V0][0] as i64) < 0;
        c.sw(gpa(c, 0x850), c.g[V0][0]);
        if failed {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 1;
            call_ptr(c, 0x002c_48c0, 0x002c_4268)?; // cPS2MemCard_SetState
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 6;
            call_ptr(c, 0x002c_48c0, 0x002c_427c)?; // cPS2MemCard_SetState
        }
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.cycles += 4;
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

/// `void cPS2MemCard_DoListDirAll(cPS2MemCard* m)` (0x002c6848): as
/// [`c_ps2_mem_card_do_list_dir`] but listing every directory (`t0 = 0x40`)
/// and entering state 0x3d on success.
pub fn c_ps2_mem_card_do_list_dir_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.g[A1][0] = 0;
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.g[A2][0] = 0x1000;
        c.g[S1][0] = c.g[A0][0];
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4c0));
        c.sd(sp(c), c.g[RA][0]);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_6448, 0x002c_6870)?; // EA_MemSet
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[T1][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x139));
        c.g[A3][0] = 0;
        c.g[T0][0] = 0x40;
        call_ptr(c, 0x0040_a688, 0x002c_688c)?; // sceMcGetDir
        let failed = (c.g[V0][0] as i64) < 0;
        c.sw(gpa(c, 0x850), c.g[V0][0]);
        if failed {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 1;
            call_ptr(c, 0x002c_48c0, 0x002c_68a0)?; // cPS2MemCard_SetState
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0x3d;
            call_ptr(c, 0x002c_48c0, 0x002c_68b4)?; // cPS2MemCard_SetState
        }
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.cycles += 4;
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Memory card.

/// `void cPS2MemCard_Close(cPS2MemCard* self)` (0x2c6360): finish the pending
/// card request (if any) and clear the card state.
pub fn c_ps2_mem_card_close(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p_5(r32(c, S0), 0x40)).lw(c);
        let idle = c.g[V0][0] == 0;
        c.g[A0][0] = 0;
        let mut skip = false;
        if !idle {
            c.cycles += 8;
            Addr(gp_5(c, 0x850)).sw(c, 0);
            c.g[A2][0] = sx(gp_5(c, 0x850));
            c.g[RA][0] = sx(0x002c_638c);
            c.g[A1][0] = 0;
            call_ptr(c, 0x0040_a360, 0x002c_638c)?;
            c.g[V1][0] = Addr(gp_5(c, 0x850)).lw(c);
            c.g[V0][0] = u64::MAX;
            if c.g[V1][0] != c.g[V0][0] {
                Addr(p_5(r32(c, S0), 0x40)).sw(c, 0);
                skip = true;
            } else {
                c.cycles += 6;
                c.g[V0][0] = Addr(p_5(r32(c, S0), 0xc)).lw(c);
                c.g[V1][0] = 0x14;
                c.g[A0][0] = 0x1;
                c.mult(0, 5, 2, 3);
                c.g[V0][0] = sx(p_5(r32(c, A1), r32(c, S0)));
                Addr(p_5(r32(c, V0), 0x18c)).sw(c, c.g[A0][0]);
            }
        }
        if !skip {
            c.cycles += 1;
            Addr(p_5(r32(c, S0), 0x40)).sw(c, 0);
        }
        c.cycles += 12;
        c.g[A0][0] = c.g[S0][0];
        Addr(p_5(r32(c, S0), 0xc)).sw(c, 0);
        Addr(p_5(r32(c, S0), 0x17c)).sw(c, 0);
        Addr(p_5(r32(c, S0), 0x180)).sw(c, 0);
        Addr(p_5(r32(c, S0), 0x184)).sw(c, 0);
        Addr(p_5(r32(c, S0), 0x188)).sw(c, 0);
        c.g[RA][0] = sx(0x002c_63d8);
        Addr(p_5(r32(c, S0), 0x3c)).sw(c, 0);
        callee::PS2_MEM_CARD_CLEAR_DIR_CACHE.call(c, 0x002c_63d8)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cPS2MemCard_GetSlotName` (0x2c63f8): format the memory-card slot number
// (n+1) for the card icon, with the slot's free-space suffix when it is one
// of the first two slots.
pub fn cps2_memcard_get_slot_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        c.g[A0][0] = c.g[SP][0];
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        c.g[A1][0] = 0x4a0000;
        frame.save(c, S2, 0x40);
        frame.save(c, S1, 0x50);
        c.g[S3][0] = c.g[A2][0];
        frame.save_ra(c, 0x20);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3970));
        call_ptr(c, 0x002c_2540, 0x002c_6428)?;
        c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
        call_ptr(c, 0x002c_26d0, 0x002c_643c)?;
        c.g[V0][0] = ((c.g[S0][0] as i64) < 2i64) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            let negative = (c.g[S0][0] as i64) < 0;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            if !negative {
                c.cycles += 15;
                c.g[A0][0] = 0x480000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x64e8));
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                call_ptr(c, 0x0031_7670, 0x002c_646c)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                c.g[A0][0] = c.g[S0][0];
                let target = c.g[V1][0] as u32;
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, target, 0x002c_647c)?;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S2][0];
                call_ptr(c, 0x002c_26d0, 0x002c_648c)?;
            }
            c.cycles += 1;
            c.lq(16, frame.at(c, 0x60));
        } else {
            c.lq(16, frame.at(c, 0x60));
        }
        c.cycles += 6;
        c.lq(17, frame.at(c, 0x50));
        c.lq(18, frame.at(c, 0x40));
        c.lq(19, frame.at(c, 0x30));
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Localisation.

/// `char* LocStringTable_Find(LocStringTable* t, int key)` (0x2c6de0): binary
/// search the sorted (key, string) table; 0 when the key is absent or the
/// table is empty.
pub fn loc_string_table_find(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = Addr(p_6(c.g[A0][0] as u32, 0xc)).lw(c);
    c.g[T2][0] = 0;
    let empty = c.g[V0][0] == 0;
    c.g[T0][0] = Addr(p_6(c.g[A0][0] as u32, 8)).lw(c);
    if empty {
        c.cycles += 2;
        c.g[V0][0] = 0;
        c.pc = c.g[RA][0] as u32;
        return;
    }
    'miss: {
        c.cycles += 7;
        c.g[A0][0] = Addr(p_6(c.g[A0][0] as u32, 0x10)).lw(c);
        c.g[V1][0] = sx((c.g[T0][0] as u32) << 3);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
        let past_end = c.g[V0][0] != 0;
        c.g[T1][0] = c.g[A0][0];
        if past_end {
            break 'miss;
        }
        c.cycles += 4;
        c.g[V0][0] = Addr(c.g[T1][0] as u32).lw(c);
        c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
        let before_start = c.g[V0][0] == 0;
        c.g[A2][0] = sx(((c.g[T0][0] as u32) as i32 >> 1) as u32);
        if !before_start {
            break 'miss;
        }
        'found: {
            c.cycles += 5;
            c.g[A0][0] = sx((c.g[A2][0] as u32) << 3);
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T1][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            let hit = c.g[V1][0] == c.g[A1][0];
            c.g[A3][0] = c.g[T1][0];
            if hit {
                break 'found;
            }
            c.cycles += 2;
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A3][0] as u32));
            loop {
                c.cycles += 4;
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
                if c.g[V0][0] == 0 {
                    c.g[T2][0] = c.g[A2][0];
                } else {
                    c.cycles += 1;
                    c.g[T0][0] = c.g[A2][0];
                }
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_sub(c.g[T2][0] as u32));
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                let converged = c.g[V0][0] == c.g[A2][0];
                c.g[A2][0] = c.g[V0][0];
                if converged {
                    break 'miss;
                }
                c.cycles += 5;
                c.g[A0][0] = sx((c.g[A2][0] as u32) << 3);
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                let different = c.g[V1][0] != c.g[A1][0];
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                if !different {
                    break 'found;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002c_6e40);
                }
            }
        }
        c.cycles += 4;
        c.g[V1][0] = sx((c.g[A2][0] as u32) << 3);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T1][0] as u32));
        let ret = c.g[RA][0] as u32;
        c.g[V0][0] = Addr(p_6(c.g[V1][0] as u32, 4)).lw(c);
        c.pc = ret;
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    c.pc = c.g[RA][0] as u32;
}

/// `void cPS2MemCard_DoGetSaveDirEntry(self)` (0x2c42b0): issue the
/// sceMcGetDir for the save directory and update the state machine.
pub fn cps2_mem_card_do_get_save_dir_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x78;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S2][0] = this;
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xb9));
        c.sw(p_6(c.g[S2][0] as u32, 0x3c), 0);
        c.g[A1][0] = Addr(p_6(c.g[S2][0] as u32, 0x1c)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0xf9));
        c.mult(0, 3, 5, 2);
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S2][0] as u32));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1b0));
        call_ptr(c, 0x0041_66f4, 0x002c_42f0)?;
        c.g[A2][0] = c.g[S0][0];
        c.g[A1][0] = 0x4a0000;
        c.g[A3][0] = c.g[A2][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3948));
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0041_7828, 0x002c_4308)?;
        c.g[A0][0] = Addr(p_6(c.g[S2][0] as u32, 0xc)).lw(c);
        c.g[A2][0] = c.g[S1][0];
        c.g[A1][0] = 0;
        c.g[A3][0] = 0;
        c.g[T0][0] = 0x1;
        c.g[T1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x480));
        call_ptr(c, 0x0040_a688, 0x002c_4324)?;
        let ok = (c.g[V0][0] as i64) >= 0;
        c.sw(p_6(r32(c, GP), 0x850), c.g[V0][0]);
        if ok {
            c.cycles += 4;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 0x23;
            call_ptr(c, 0x002c_48c0, 0x002c_434c)?;
        } else {
            c.cycles += 5;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x002c_48c0, 0x002c_4338)?;
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Memory card: slot marking.

/// `void cPS2MemCard_MarkSlotsWithoutSpace(self)` (0x2c6280): mark every slot
/// that cannot hold another save, given the remaining space and save size.
pub fn cps2memcard_mark_slots_without_space(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = 0;
        frame.save(c, S4, 0x20);
        frame.save(c, S0, 0x60);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A0][0] = Addr(p_7(r32(c, V1), 0xa0)).lh(c);
        c.g[V0][0] = Addr(p_7(r32(c, V1), 0xa4)).lw(c);
        let f = r32(c, V0);
        c.g[A0][0] = sx(p_7(r32(c, S1), r32(c, A0)));
        call_ptr(c, f, 0x002c_62bc)?;
        c.g[V1][0] = Addr(r32(c, S1)).lw(c);
        c.g[S3][0] = c.g[V0][0];
        c.g[A1][0] = Addr(p_7(r32(c, S1), 0x24)).lw(c);
        c.g[A0][0] = Addr(p_7(r32(c, V1), 0xb0)).lh(c);
        c.g[V0][0] = Addr(p_7(r32(c, V1), 0xb4)).lw(c);
        let f = r32(c, V0);
        c.g[A0][0] = sx(p_7(r32(c, S1), r32(c, A0)));
        call_ptr(c, f, 0x002c_62d8)?;
        c.g[S4][0] = c.g[V0][0];
        c.g[V0][0] = Addr(p_7(r32(c, S1), 0x14)).lw(c);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[S5][0] = 1;
        if none {
            c.cycles += 9;
            frame.restore(c, S0, 0x60);
            frame.restore(c, S1, 0x50);
            frame.restore(c, S2, 0x40);
            frame.restore(c, S3, 0x30);
            frame.restore(c, S4, 0x20);
            frame.restore(c, S5, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 2;
        c.g[S0][0] = sx(p_7(r32(c, S1), 0x1ac));
        loop {
            c.cycles += 9;
            Addr(r32(c, S0)).sw(c, 0);
            c.g[A1][0] = c.g[S2][0];
            c.g[V1][0] = Addr(r32(c, S1)).lw(c);
            c.g[A0][0] = Addr(p_7(r32(c, V1), 0x218)).lh(c);
            c.g[V0][0] = Addr(p_7(r32(c, V1), 0x21c)).lw(c);
            let f = r32(c, V0);
            c.g[A0][0] = sx(p_7(r32(c, S1), r32(c, A0)));
            call_ptr(c, f, 0x002c_630c)?;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = Addr(p_7(r32(c, S1), 0x14)).lw(c);
            } else {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S4][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    Addr(r32(c, S0)).sw(c, c.g[S5][0]);
                } else {
                    c.cycles += 1;
                    c.g[S3][0] = sx(r32(c, S3).wrapping_sub(r32(c, S4)));
                }
                c.cycles += 1;
                c.g[V0][0] = Addr(p_7(r32(c, S1), 0x14)).lw(c);
            }
            c.cycles += 4;
            c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx(p_7(r32(c, S0), 0x78));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002c_62f0);
            }
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

/// `QuadTree_FlattenNode(self, out, count, map, node)` (0x2c2088): emit a
/// quadtree node into the flat array (`out[*count] = {child, payload}`),
/// record its index in `map` (`(node - self->base) >> 3`), then recurse into
/// the four children when the node's first child word has bit 0 set.
pub fn quad_tree_flatten_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x70);
        let this = abi::arg(c, 0);
        let _out = abi::arg(c, 1);
        let count = abi::arg(c, 2);
        let _map = abi::arg(c, 3);
        frame.save(c, 18, 0x40);
        frame.save(c, 19, 0x30);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, 20, 0x20);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, 21, 0x10);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, 16, 0x60);
        c.g[S5][0] = c.g[A3][0];
        frame.save(c, 17, 0x50);
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, count);
        c.g[A1][0] = lw(c, p_8(c.g[T0][0] as u32, 4));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
        c.sw(p_8(c.g[V0][0] as u32, 4), c.g[A1][0]);
        c.g[V1][0] = lw(c, count);
        c.g[A0][0] = lw(c, c.g[T0][0] as u32);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S4][0] as u32));
        c.sw(c.g[V1][0] as u32, c.g[A0][0]);
        c.g[V0][0] = lw(c, p_8(this, 0x20));
        c.g[V1][0] = lw(c, count);
        c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
        c.g[V0][0] &= 0xffff;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
        c.sw(c.g[V0][0] as u32, c.g[V1][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        c.sw(count, c.g[V1][0]);
        c.g[V0][0] = c.lhu(c.g[T0][0] as u32);
        c.g[V0][0] &= 1;
        c.g[S0][0] = c.g[T0][0];
        if c.g[V0][0] != 0 {
            // four children at base + (childWord >> 1)
            c.cycles += 2;
            c.g[S1][0] = 3;
            c.g[T0][0] = c.lhu(c.g[S0][0] as u32);
            loop {
                c.cycles += 13;
                c.g[A0][0] = c.g[S3][0];
                c.g[V0][0] = lw(c, p_8(c.g[S3][0] as u32, 0x20));
                c.g[A1][0] = c.g[S4][0];
                c.g[T0][0] = sx((c.g[T0][0] as u32) >> 1);
                c.g[A2][0] = c.g[S2][0];
                c.g[T0][0] = sx((c.g[T0][0] as u32) << 3);
                c.g[A3][0] = c.g[S5][0];
                c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[RA][0] = sx(0x002c_2148);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(2));
                c.pc = 0x002c_2088;
                quad_tree_flatten_node(c);
                if c.pc != 0x002c_2148 {
                    return Err(Unwound);
                }
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
                if (c.g[S1][0] as i64) < 0 {
                    break;
                }
                c.g[T0][0] = c.lhu(c.g[S0][0] as u32);
                if event_due(c) {
                    deliver_event_here(c, 0x002c_2120);
                }
            }
        }
        c.cycles += 9;
        frame.restore(c, 16, 0x60);
        frame.restore(c, 17, 0x50);
        frame.restore(c, 18, 0x40);
        frame.restore(c, 19, 0x30);
        frame.restore(c, 20, 0x20);
        frame.restore(c, 21, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cPS2MemCard_GetFreeSpaceString(self, idx, dst, port)` (0x2c53e0): format
/// the free space of memory-card entry `idx` into the wide string `dst`.
/// No entry data, a status of -2 or an unknown status produce the "no card"
/// message; status 0 formats the free-space number through the file-system
/// object's size formatter.
pub fn cps2memcard_get_free_space_string(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x14;
        // `a3 = idx * sizeof(entry)` (imul rd=a3, rs=a1, rt=20)
        c.mult(0, A3, A1, V0);
        frame.save(c, 19, 0x10);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        frame.save(c, 18, 0x20);
        frame.save_ra(c, 0);
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A3][0] as u32));
        c.g[V1][0] = lw(c, p_8(c.g[V0][0] as u32, 0x188));
        let no_data = c.g[V1][0] == 0;
        c.g[S3][0] = c.g[A2][0];
        // 0: no data, 1: status -2, 2: status 0, 3: any other status
        let mut which = 0u32;
        if !no_data {
            c.cycles += 5;
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A3][0] as u32));
            c.g[V1][0] = sx(0xffff_fffe);
            c.g[V0][0] = lw(c, p_8(c.g[V0][0] as u32, 0x184));
            if c.g[V0][0] == c.g[V1][0] {
                c.g[A1][0] = 0x4a_0000;
                which = 1;
            } else {
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    which = 2;
                } else {
                    c.cycles += 2;
                    c.g[A1][0] = 0x4a_0000;
                    which = 3;
                }
            }
        }
        if which == 0 {
            c.cycles += 1;
            c.g[A1][0] = 0x4a_0000;
        }
        if which == 2 {
            // status 0: the free-space number, formatted by the file system
            c.cycles += 23;
            c.g[RA][0] = sx(0x002c_5454);
            c.g[S2][0] = 0;
            c.pc = 0x002c_5320;
            ee::s08::cPS2MemCard_GetFreeSpaceForPort(c);
            if c.pc != 0x002c_5454 {
                return Err(Unwound);
            }
            c.g[A1][0] = lw(c, gp(c, -0x848));
            c.g[V1][0] = sx(0xffff_ffff);
            c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            c.g[A0][0] = 0x48_0000;
            c.g[S0][0] = lw(c, p_8(c.g[A1][0] as u32, 0x8c));
            if c.g[V1][0] != 0 {
                c.g[S2][0] = c.g[V0][0];
            }
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x61f8));
            c.g[S1][0] = lw(c, p_8(c.g[S0][0] as u32, 4));
            c.g[V0][0] = c.lh(p_8(c.g[S1][0] as u32, 0x20));
            c.g[RA][0] = sx(0x002c_5480);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            call_ptr(c, 0x0031_7670, 0x002c_5480)?; // StrHash(D_004861F8)
            c.g[V1][0] = lw(c, p_8(c.g[S1][0] as u32, 0x24));
            c.g[A0][0] = c.g[S0][0];
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x002c_5490);
            c.g[A1][0] = c.g[V0][0];
            c.pc = t;
            ssx3_ee_rt::funcs::call_indirect(c, t);
            if c.pc != 0x002c_5490 {
                return Err(Unwound);
            }
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x002c_54a0);
            c.g[A2][0] = c.g[S2][0];
            c.pc = 0x002c_26d0;
            ee::s08::UStr_Sprintf(c);
            if c.pc != 0x002c_54a0 {
                return Err(Unwound);
            }
        } else {
            // "no card" (status -2, or no data) and unknown statuses
            let (cost, ret) = if which == 1 {
                (5, 0x002c_5444)
            } else {
                (4, 0x002c_54b4)
            };
            c.cycles += cost;
            c.g[A0][0] = c.g[S3][0];
            c.g[RA][0] = sx(ret);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3968));
            c.pc = 0x002c_2540;
            ee::s08::UStr_FromAscii_2(c);
            if c.pc != ret {
                return Err(Unwound);
            }
        }
        c.lq(16, p_8(sp(c), 0x40));
        c.cycles += 6;
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;

    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;

    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::fill;

    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL_RA;

    pub(crate) use crate::game::ai::ai::tests::strict;
    pub(crate) use crate::game::ai::ai::tests::SEEDS as SEEDS_10;
    pub(crate) use crate::game::ai::rider::tests::clear_2 as clear_6;
    pub(crate) use crate::game::be::beintnetwork::tests::put_str;
    pub(crate) use crate::game::be::bestate2::tests::SEEDS as SEEDS_6;
    pub(crate) use crate::game::bx::bxstring::tests::string as string_2;
    pub(crate) use crate::game::bx::bxstring2::tests::base_3 as base_4;
    pub(crate) use crate::game::bx::bxstring2::tests::game;

    pub(crate) use crate::game::bx::bxstring2::tests::slot_2;

    pub(crate) use crate::game::bx::bxstring2::tests::SCRATCH;
    pub(crate) use crate::game::bx::bxstring2::tests::VTS;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::base_6 as base_7;

    pub(crate) use crate::game::dirtysock::tags2::tests::clear as clear_3;
    pub(crate) use crate::game::dirtysock::tags2::tests::common;

    pub(crate) use crate::game::dirtysock::tags2::tests::string;
    pub(crate) use crate::game::dirtysock::tags2::tests::SEEDS_7 as SEEDS_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_3 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_4 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_7 as be_library_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::clear_4 as clear_5;

    pub(crate) use crate::game::fe::feasyncfile::tests::pool;

    pub(crate) use crate::game::fe::feasyncfile::tests::w_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::world;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_4 as world_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_9;

    pub(crate) use crate::game::fe::fepopup::tests::b;
    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::fe::festaterewards2::tests::belib;
    pub(crate) use crate::game::fe::festaterewards2::tests::w;
    pub(crate) use crate::game::fe::festaterewards2::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::main::gamemode::tests::be_library as be_library_2;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_8;
    pub(crate) use crate::game::object::animnode::tests::heap_block;
    pub(crate) use crate::game::object::debouncenode::tests::vtable_2;
    pub(crate) use crate::game::render::particle::tests::clear as clear_4;
    pub(crate) use crate::game::render::particle::tests::common as common_2;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_5;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_7;
    pub(crate) use crate::game::sound::bankmanager::tests::mixed;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS;
    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4 as SEEDS_3;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_6;

    pub(crate) use crate::game::sound::soundsys::tests::wf;
    pub(crate) use crate::game::ui::uitext::tests::h;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear_2;

    use ssx3_ee as ee;

    use ssx3_ee::s08;

    /// Fill the two static SJIS conversion tables around 0x445940.
    pub(crate) fn sjis_tables(c: &mut Cpu, r: &mut Rng) {
        for k in 0..0x60u32 {
            c.bus.write16(0x0044_5940 + 2 * k, r.next_u32() as u16);
        }
    }

    #[test]
    fn memcard_ascii_char_to_sjis_matches() {
        check(
            ssx3_ee::s08::f_002c2338,
            memcard_ascii_char_to_sjis,
            300,
            &|c, seed| {
                let mut r = Rng(0x2338 ^ seed as u64);
                scramble(c, &mut r);
                sjis_tables(c, &mut r);
                let ch = match seed % 9 {
                    0 => 0x20 + (r.next_u32() % 0x10),
                    1 => 0x30 + (r.next_u32() % 0xa),
                    2 => 0x3a + (r.next_u32() % 7),
                    3 => 0x41 + (r.next_u32() % 0x1a),
                    4 => 0x5b + (r.next_u32() % 6),
                    5 => 0x61 + (r.next_u32() % 0x1a),
                    6 => 0x7b + (r.next_u32() % 4),
                    7 => r.next_u32() % 0x20,
                    _ => r.next_u32() & 0xff,
                };
                c.g[A0][0] = ((r.next_u32() as u64) << 32) | ch as u64;
            },
        );
    }

    #[test]
    fn memcard_sjis_char_to_ascii_matches() {
        check(
            ssx3_ee::s08::f_002c2430,
            memcard_sjis_char_to_ascii,
            300,
            &|c, seed| {
                let mut r = Rng(0x2430 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                sjis_tables(c, &mut r);
                for k in 0..0x20u32 {
                    c.bus.write8(GAME_GP + 0x7a0 + k, r.next_u32() as u8);
                }
                let (lead, trail) = match seed % 5 {
                    0 => (r.next_u32() as u8, 0x81u8),
                    1 => (r.next_u32() as u8, 0x80u8),
                    _ => {
                        let lead = match seed % 6 {
                            0 => (0x4f + (r.next_u32() % 10)) as u8,
                            1 => (0x60 + (r.next_u32() % 0x1a)) as u8,
                            2 => (r.next_u32() % 0x4f) as u8,
                            3 => (0x59 + (r.next_u32() % 7)) as u8,
                            4 => (0x7a + (r.next_u32() % 6)) as u8,
                            _ => r.next_u32() as u8,
                        };
                        (lead, 0x82u8)
                    }
                };
                let code = ((lead as u32) << 8) | trail as u32;
                c.g[A0][0] = ((r.next_u32() as u64) << 32) | code as u64;
            },
        );
    }

    #[test]
    fn cps2memcard_get_port_error_matches() {
        check_lohi(
            ssx3_ee::s08::f_002c51d0,
            cps2memcard_get_port_error,
            300,
            &|c, seed| {
                let mut r = Rng(0x51d0 ^ seed as u64);
                scramble(c, &mut r);
                let port = seed % 4;
                c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
                c.g[A1][0] = port as u64;
                let e = match seed % 12 {
                    0 => 0,
                    1 => 1,
                    2 => (-7i32) as u32,
                    3 => (-8i32) as u32,
                    4 => (-10i32) as u32,
                    5 => (-11i32) as u32,
                    6 => (-10000i32) as u32,
                    7 => (-10001i32) as u32,
                    8 => (-10002i32) as u32,
                    9 => (-10003i32) as u32,
                    _ => r.next_u32(),
                };
                c.bus.write32(DATA + port * 0x14 + 0x184, e);
            },
        );
    }

    #[test]
    fn cps2memcard_clear_port_info_matches() {
        check_lohi(
            ssx3_ee::s08::f_002c5250,
            cps2memcard_clear_port_info,
            200,
            &|c, seed| {
                let mut r = Rng(0x5250 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0xc, r.next_u32() % 4);
            },
        );
    }

    #[test]
    fn cps2memcard_get_free_space_for_port_matches() {
        check_lohi(
            ssx3_ee::s08::f_002c5320,
            cps2memcard_get_free_space_for_port,
            200,
            &|c, seed| {
                let mut r = Rng(0x5320 ^ seed as u64);
                scramble(c, &mut r);
                let port = seed % 4;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = port as u64;
                c.bus.write32(DATA + port * 0x14 + 0x180, r.next_u32());
            },
        );
    }

    #[test]
    fn cps2memcard_mark_save_invalid_matches() {
        check_lohi(
            ssx3_ee::s08::f_002c5570,
            cps2memcard_mark_save_invalid,
            200,
            &|c, seed| {
                let mut r = Rng(0x5570 ^ seed as u64);
                scramble(c, &mut r);
                let port = seed % 4;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = port as u64;
                c.bus.write32(DATA + port * 0x78 + 0x1a8, r.next_u32());
            },
        );
    }

    #[test]
    fn cps2memcard_is_mc_error_matches() {
        check(
            ssx3_ee::s08::f_002c63e8,
            cps2memcard_is_mc_error,
            300,
            &|c, seed| {
                let mut r = Rng(0x63e8 ^ seed as u64);
                scramble(c, &mut r);
                let v = match seed % 4 {
                    0 => (r.next_u32() % 0x3000).wrapping_sub(0x2800),
                    1 => 0xffff_d8f1, // -9999
                    2 => 0x7fff_ffff,
                    _ => r.next_u32(),
                };
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | v as u64;
            },
        );
    }

    #[test]
    fn cps2_mem_card_get_port_changed_flag_matches() {
        check_lohi(
            ee::s08::f_002c53c8__generated,
            cps2_mem_card_get_port_changed_flag,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x002c_53c8, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                let port = (seed % 8) as u32;
                c.g[A1][0] = mixed(&mut r, seed, port);
                c.bus.write32(DATA + port * 0x14 + 0x18c, r.next_u32());
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_v39_matches() {
        check(
            ee::s08::f_002c66b8__generated,
            c_ps2_mem_card_v39,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c_66b8, 0);
                c.g[A0][0] = DATA as u64;
                let name = DATA + 0x800;
                put_str(c, name, if seed % 2 == 0 { "slot1" } else { "" });
                c.g[A1][0] = name as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_clear_dir_entry_matches() {
        check(
            ee::s08::f_002c52d8__generated,
            c_ps2_mem_card_clear_dir_entry,
            60,
            &|c, seed| {
                let _r = game(c, seed, 0x2c52d8, 1);
                c.g[A0][0] = SCRATCH as u64;
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_v15_matches() {
        check(
            ee::s08::f_002c64a8__generated,
            c_ps2_mem_card_v15,
            80,
            &|c, seed| {
                let mut r = game(c, seed, 0x2c64a8, 1);
                c.g[A2][0] = (SCRATCH + 0x900) as u64;
                c.g[A0][0] = ptr(&mut r) as u64;
                c.g[A1][0] = r.next_u32() as u64;
                // The ASCII source the second argument points at (D_004A3968).
                for (i, b) in b"memcard\0".iter().enumerate() {
                    c.bus.write8(0x004a_3968 + i as u32, *b);
                }
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_v16_matches() {
        check(
            ee::s08::f_002c64d0__generated,
            c_ps2_mem_card_v16,
            80,
            &|c, seed| {
                let mut r = game(c, seed, 0x2c64d0, 1);
                c.g[A2][0] = (SCRATCH + 0x900) as u64;
                c.g[A0][0] = ptr(&mut r) as u64;
                c.g[A1][0] = r.next_u32() as u64;
                for (i, b) in b"memcard\0".iter().enumerate() {
                    c.bus.write8(0x004a_3968 + i as u32, *b);
                }
            },
        );
    }

    // -----------------------------------------------------------------
    // MemCard and menus.

    #[test]
    fn cps2_memcard_get_dir_entry_name_matches() {
        check(
            s08::f_002c68c8__generated,
            cps2_memcard_get_dir_entry_name,
            150,
            &|c, seed| {
                machine(c, seed, 0x2c68c8, 1);
                c.g[A0][0] = sx(OBJ);
                c.g[A1][0] = sx(seed % 8);
                c.g[A2][0] = sx(DATA + 0x4000);
                clear(c, OBJ, 0x200);
                clear(c, DATA + 0x4000, 0x40);
                // A short NUL-terminated name in each of the eight entries.
                for i in 0..8u32 {
                    let name = b"mc0:dir\0";
                    for (k, b) in name.iter().enumerate() {
                        c.bus.write8(OBJ + (i << 6) + 0x4e0 + k as u32, *b);
                    }
                }
                // Scrambled destinations are fine, but keep strcpy on RAM: the
                // copy length comes from the source only.
                c.bus.write8(DATA + 0x4000 + 0x3f, 0);
            },
        );
    }

    /// The engine's size-class pools (`MEM_PoolAlloc`): allocations up to
    /// 0x200 bytes are popped from per-class free lists hanging off
    /// `gp + 0x2a60..`, but only when the "pools ready" flag at `gp + 0xdd8`
    /// is set; otherwise `MEM_PoolAlloc` forwards to `MEM_AllocInternal`, which
    /// asks the pools again — an endless recursion on a fresh test machine.
    pub(crate) fn pool_allocator(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0xdd8, 1); // pools ready
                                           // (pool pointer global, free-list head offset within the pool object,
                                           // block size of the class)
        const POOLS: [(u32, u32, u32); 7] = [
            (0x2a70, 0x800, 8),     // 8-byte blocks
            (0x2a60, 0x2000, 0x10), // 16
            (0x2a6c, 0x32000, 0x20),
            (0x2a7c, 0x25800, 0x40),
            (0x2a84, 0x25800, 0x80),
            (0x2a78, 0x20000, 0x100),
            (0x2a68, 0x20000, 0x200),
        ];
        let mut nodes = DATA + 0x24_000;
        for (i, (slot, head, block)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base = nodes;
            // 16 free nodes per class, spaced by the block size (the head is
            // updated on pop, so only an unpopped node's next pointer matters).
            for n in 0..16u32 {
                let node = base + block * n;
                c.bus.write32(node, if n == 15 { 0 } else { node + block });
            }
            c.bus.write32(pool + head, base);
            c.bus.write32(GAME_GP + slot, pool);
            nodes += block * 16;
        }
    }

    /// The main heap's dlmalloc arena over 256 KiB of test RAM, as
    /// `containers.rs`/`oc59` set up for the allocator tests: allocations
    /// larger than 0x200 bytes come from here.
    pub(crate) const MEM_HEAP: u32 = 0x004f_f250;

    pub(crate) fn run(c: &mut Cpu, addr: u32, f: fn(&mut Cpu), args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, SENTINEL_RA, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    pub(crate) fn heap_init(c: &mut Cpu) {
        c.g[GP][0] = GAME_GP as u64;
        run(
            c,
            0x0031_ff60,
            ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, (DATA + 0x40000) as u64, 0x40000],
        );
    }

    /// A machine for functions that allocate: fake callees for the misses,
    /// the pool free lists and a real heap arena.
    pub(crate) fn alloc_machine(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = machine(c, seed, salt, 0);
        pool_allocator(c);
        heap_init(c);
        r
    }

    /// A vtable at a fixed test address with `{delta, fake}` at each slot.
    pub(crate) fn put_vtable(c: &mut Cpu, r: &mut Rng, at: u32, slots: &[u32]) -> u32 {
        for (i, &off) in slots.iter().enumerate() {
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(at + off, delta as u16);
            c.bus.write32(at + off + 4, FAKE + 0x10 * i as u32);
        }
        at
    }

    // -------------------------------------------------------- memory card

    #[test]
    fn c_ps2_mem_card_create_matches() {
        check(
            ee::s08::f_002c22c8__generated,
            c_ps2_mem_card_create,
            40,
            &|c, seed| {
                alloc_machine(c, seed, 0x2c22_c8);
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_destroy_matches() {
        check(
            ee::s08::f_002c2300__generated,
            c_ps2_mem_card_destroy,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c23_00, 0);
                let obj = DATA + 0x100;
                let vt = put_vtable(c, &mut r, DATA + 0x800, &[8]);
                c.bus.write32(obj, vt);
                c.g[A0][0] = sx(if seed % 4 == 0 { 0 } else { obj });
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_ctor_matches() {
        check(
            ee::s08::f_002c3fa8__generated,
            c_ps2_mem_card_ctor,
            40,
            &|c, seed| {
                alloc_machine(c, seed, 0x2c3f_a8);
                c.g[A0][0] = sx(DATA + 0x1000);
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_request_list_dir_matches() {
        check(
            ee::s08::f_002c41d8__generated,
            c_ps2_mem_card_request_list_dir,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c41_d8, 0);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.bus.write8(DATA + 0x2000, 0);
                let len = (seed % 16) as u32;
                for i in 0..len {
                    c.bus
                        .write8(DATA + 0x2000 + i, b'a' + (r.next_u32() % 26) as u8);
                }
                c.bus.write8(DATA + 0x2000 + len, 0);
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_request_delete_matches() {
        check(
            ee::s08::f_002c6ce0__generated,
            c_ps2_mem_card_request_delete,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c6c_e0, 0);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx(DATA + 0x2000);
                let len = (seed % 16) as u32;
                for i in 0..len {
                    c.bus
                        .write8(DATA + 0x2000 + i, b'A' + (r.next_u32() % 26) as u8);
                }
                c.bus.write8(DATA + 0x2000 + len, 0);
            },
        );
    }

    // -------------------------------------------------------- strings

    driven!(gen_ustr_len_2, ee::s08::f_002c24d0__generated);

    driven!(hand_ustr_len_2, ustr_len_2);

    #[test]
    fn ustr_len_2_matches() {
        check_events(gen_ustr_len_2, hand_ustr_len_2, 200, &|c, seed| {
            let mut r = Rng(0x7573_7472_6c65_6e32 ^ (seed as u64) << 20);
            crate::difftest::scramble(c, &mut r);
            let len = (seed % 24) as u32;
            for i in 0..len {
                let b = match r.next_u32() % 5 {
                    0 => [1u16, 0x7f, 0x80, 0xff, 0xfe][(r.next_u32() % 5) as usize],
                    _ => (r.next_u32() as u16).max(1),
                };
                c.bus.write16(DATA + 2 * i, b);
            }
            c.bus.write16(DATA + 2 * len, 0);
            c.g[A0][0] = sx(DATA);
            schedule(c, &mut r, seed, 0x002c_24d0)
        });
    }

    driven!(gen_str_to_lower, ee::s08::f_002c2868__generated);

    driven!(hand_str_to_lower, str_to_lower_in_place);

    #[test]
    fn str_to_lower_in_place_matches() {
        check_events(gen_str_to_lower, hand_str_to_lower, 200, &|c, seed| {
            let mut r = Rng(0x6c6f_7765_72 ^ (seed as u64) << 20);
            crate::difftest::scramble(c, &mut r);
            let len = (seed % 24) as u32;
            for i in 0..len {
                let b = match r.next_u32() % 6 {
                    0 => b'A' + (r.next_u32() % 26) as u8,
                    1 => [b'Z', b'a', b'z', b'@', b'[', 0x80, 0xff][(r.next_u32() % 7) as usize],
                    _ => (r.next_u32() as u8).max(1),
                };
                c.bus.write8(DATA + i, b);
            }
            c.bus.write8(DATA + len, 0);
            c.g[A0][0] = sx(DATA);
            schedule(c, &mut r, seed, 0x002c_2868)
        });
    }

    // -----------------------------------------------------------------------
    // Memcard request wrappers.

    pub(crate) fn memcard_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        c.g[A0][0] = DATA as u64;
        let name = DATA + 0x800;
        match seed % 3 {
            0 => put_str(c, name, ""),
            1 => put_str(c, name, "BESLES-51234"),
            _ => put_str(c, name, "a much longer memory card file name"),
        }
        c.g[A1][0] = name as u64;
        c.g[A2][0] = (r.next_u32() % 4) as u64;
        // The state machine's dispatch table is zeroed: it lands in the
        // `jump_out` hook, which the test machine turns into a fake call.
        c.bus.write32(DATA + 0x18, 0x100);
    }

    #[test]
    fn c_ps2_mem_card_request_list_dir_all_matches() {
        check(
            ssx3_ee::s08::f_002c6810__generated,
            c_ps2_mem_card_request_list_dir_all,
            48,
            &|c, seed| {
                memcard_setup(c, seed, 0x2c68_10);
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_request_open_matches() {
        check(
            ssx3_ee::s08::f_002c6968__generated,
            c_ps2_mem_card_request_open,
            48,
            &|c, seed| {
                memcard_setup(c, seed, 0x2c69_68);
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_do_close_matches() {
        check(
            ssx3_ee::s08::f_002c6a28__generated,
            c_ps2_mem_card_do_close,
            48,
            &|c, seed| {
                // A zero `sceMcClose` global makes the firmware call take its
                // "not initialised" path and return at once.
                memcard_setup(c, seed, 0x2c6a_28);
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_request_mkdir_matches() {
        check(
            ssx3_ee::s08::f_002c6c50__generated,
            c_ps2_mem_card_request_mkdir,
            48,
            &|c, seed| {
                memcard_setup(c, seed, 0x2c6c_50);
            },
        );
    }

    #[test]
    fn ustr_from_ascii_2_matches() {
        check(
            ssx3_ee::s08::f_002c2540__generated,
            ustr_from_ascii_2,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x2c25_40, 0);
                belib(c);
                c.g[A0][0] = (DATA + 0x200) as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                let text: &[u8] = match seed % 4 {
                    0 => &b""[..],
                    1 => &b"Hello"[..],
                    2 => &[0x41, 0xc3, 0x80, 0x7f][..],
                    _ => &[0x80, 0x81, 0xff][..],
                };
                for (i, &ch) in text.iter().enumerate() {
                    b(c, DATA + 0x100 + i as u32, ch);
                }
                b(c, DATA + 0x100 + text.len() as u32, 0);
            },
        );
    }

    #[test]
    fn ustr_copy_n_matches() {
        check(
            ssx3_ee::s08::f_002c25b8__generated,
            ustr_copy_n,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x2c25_b8, 0);
                belib(c);
                c.g[A0][0] = (DATA + 0x200) as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.g[A2][0] = match seed % 3 {
                    0 => 3,
                    1 => 0,
                    _ => sx(0xffff_ffff),
                };
                let text: &[u16] = match seed % 4 {
                    0 => &[][..],
                    1 => &[0x41, 0x42, 0x43][..],
                    2 => &[0x1234, 0x8000][..],
                    _ => &[0x41][..],
                };
                for (i, &ch) in text.iter().enumerate() {
                    h(c, DATA + 0x100 + 2 * i as u32, ch);
                }
                h(c, DATA + 0x100 + 2 * text.len() as u32, 0);
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_request_list_dir_w_matches() {
        check(
            ee::s08::f_002c4198__generated,
            c_ps2_mem_card_request_list_dir_w,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c4198, 0);
                let self_at = DATA + 0x200;
                let vt = vtable(c, &mut r, &[0x30]);
                c.bus.write16(DATA + 0x300, b'd' as u16);
                c.bus.write16(DATA + 0x302, b'i' as u16);
                c.bus.write16(DATA + 0x304, b'r' as u16);
                c.bus.write16(DATA + 0x306, 0);
                c.g[A0][0] = self_at as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
                c.bus.write32(self_at, vt);
            },
        );
    }

    pub(crate) fn put_str16(c: &mut Cpu, at: u32, s: &[u16]) {
        for (i, h) in s.iter().enumerate() {
            c.bus.write16(at + 2 * i as u32, *h);
        }
    }

    #[test]
    fn ustr_cat_matches() {
        check(ee::s08::f_002c2688__generated, ustr_cat, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x2c26_88, 1);
            let dst = DATA + 0x200;
            let src = DATA + 0x300;
            c.g[A0][0] = sx(dst);
            c.g[A1][0] = sx(src);
            let empty = seed % 5 == 0;
            let empty_str = [0u16];
            let filled_str = [0x61, 0x62 + (seed as u16 % 3), 0];
            put_str16(
                c,
                dst,
                if empty {
                    &empty_str[..]
                } else {
                    &filled_str[..]
                },
            );
            put_str16(c, src, &[0x63 + (seed as u16 % 3), 0]);
            let _ = r.next_u32();
        });
    }

    #[test]
    fn c_ps2_mem_card_do_flush_matches() {
        check(
            ee::s08::f_002c6b00__generated,
            c_ps2_mem_card_do_flush,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2c6b_00, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let card = DATA + 0x2000;
                clear_2(c, this, 0x80);
                clear_2(c, card, 0x100);
                c.bus.write32(this + 0x18, card);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn loc_string_table_dtor_matches() {
        check(
            ee::s08::f_002c6d70__generated,
            loc_string_table_dtor,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2c6d_70, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear_2(c, this, 0x20);
                let owned = if seed % 2 == 0 {
                    0
                } else {
                    heap_block(c, 0x40)
                };
                c.bus.write32(this + 4, owned);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 3 == 0) as u64;
            },
        );
    }

    #[test]
    fn mem_card_ascii_to_sjis_string_matches() {
        check(
            ee::s08::f_002c2210__generated,
            mem_card_ascii_to_sjis_string,
            SEEDS_4,
            &|c, seed| {
                let _ = common(c, seed, 0x2c2210, 1);
                let src = DATA + 0x2000;
                let dst = DATA + 0x3000;
                clear_3(c, src, 0x40);
                clear_3(c, dst, 0x40);
                let s = match seed % 4 {
                    0 => "",
                    1 => "A",
                    2 => "SSX3",
                    _ => "name",
                };
                for (i, b) in s.bytes().enumerate() {
                    c.bus.write8(src + i as u32, b);
                }
                c.g[A0][0] = 0;
                c.g[A1][0] = sx(dst);
                c.g[A2][0] = sx(src);
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_do_format_matches() {
        check(
            ee::s08::f_002c4388__generated,
            c_ps2_mem_card_do_format,
            SEEDS_4,
            &|c, seed| {
                let _ = common(c, seed, 0x2c4388, 2);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear_3(c, this, 0x400);
                c.bus.write32(this + 0xc, seed % 4);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_v34_matches() {
        check(
            ee::s08::f_002c4520__generated,
            c_ps2_mem_card_v34,
            SEEDS_4,
            &|c, seed| {
                elf_image(c);
                let _ = common(c, seed, 0x2c4520, 2);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear_3(c, this, 0x400);
                let name = DATA + 0x2000;
                clear_3(c, name, 0x40);
                c.bus.write8(name, b'S');
                c.bus.write8(name + 1, b'S');
                c.bus.write8(name + 2, b'X');
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(name);
                let _ = seed;
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_v35_matches() {
        check(
            ee::s08::f_002c4578__generated,
            c_ps2_mem_card_v35,
            SEEDS_4,
            &|c, seed| {
                elf_image(c);
                let _ = common(c, seed, 0x2c4578, 2);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear_3(c, this, 0x400);
                let name = DATA + 0x2000;
                clear_3(c, name, 0x40);
                c.bus.write16(name, b'S' as u16);
                c.bus.write16(name + 2, b'X' as u16);
                c.bus.write16(name + 4, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(name);
                let _ = seed;
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_v29_matches() {
        check(
            ee::s08::f_002c4800__generated,
            c_ps2_mem_card_v29,
            SEEDS_4,
            &|c, seed| {
                elf_image(c);
                let _ = common(c, seed, 0x2c4800, 3);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear_3(c, this, 0x400);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = sx(DATA + 0x2000);
                c.g[A3][0] = sx(DATA + 0x2100);
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_do_seek_matches() {
        check(
            ee::s08::f_002c6a88__generated,
            c_ps2_mem_card_do_seek,
            SEEDS_5,
            &|c, seed| {
                let mut r = common_2(c, seed, 0x2c6a88, 1);
                let this = DATA + 0x100;
                clear_4(c, this, 0x400);
                c.bus.write32(this + 0x18, r.next_u32());
                c.bus.write32(this + 0x14e4, r.next_u32());
                c.bus.write32(this + 0x14e8, r.next_u32());
                c.g[A0][0] = sx(this);
                let _ = seed;
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_do_read_matches() {
        check(
            ee::s08::f_002c6b78__generated,
            c_ps2_mem_card_do_read,
            SEEDS_5,
            &|c, seed| {
                let mut r = common_2(c, seed, 0x2c6b78, 1);
                let this = DATA + 0x100;
                clear_4(c, this, 0x400);
                c.bus.write32(this + 0x18, r.next_u32());
                c.bus.write32(this + 0x2c, DATA + 0x3000);
                c.bus.write32(this + 0x24, 0x200);
                c.g[A0][0] = sx(this);
                let _ = seed;
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_do_write_matches() {
        check(
            ee::s08::f_002c6bf8__generated,
            c_ps2_mem_card_do_write,
            SEEDS_5,
            &|c, seed| {
                let mut r = common_2(c, seed, 0x2c6bf8, 1);
                let this = DATA + 0x100;
                clear_4(c, this, 0x400);
                c.bus.write32(this + 0x18, r.next_u32());
                c.bus.write32(this + 0x2c, DATA + 0x3000);
                c.bus.write32(this + 0x24, 0x200);
                c.g[A0][0] = sx(this);
                let _ = seed;
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_do_mkdir_matches() {
        check(
            ee::s08::f_002c6c88__generated,
            c_ps2_mem_card_do_mkdir,
            SEEDS_5,
            &|c, seed| {
                let mut r = common_2(c, seed, 0x2c6c88, 1);
                let this = DATA + 0x100;
                clear_4(c, this, 0x400);
                c.bus.write32(this + 0xc, r.next_u32());
                c.bus.write8(this + 0xf9, b'B');
                c.g[A0][0] = sx(this);
                let _ = seed;
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_do_delete_matches() {
        check(
            ee::s08::f_002c6d18__generated,
            c_ps2_mem_card_do_delete,
            SEEDS_5,
            &|c, seed| {
                let mut r = common_2(c, seed, 0x2c6d18, 1);
                let this = DATA + 0x100;
                clear_4(c, this, 0x400);
                c.bus.write32(this + 0xc, r.next_u32());
                c.bus.write8(this + 0xf9, b'B');
                c.g[A0][0] = sx(this);
                let _ = seed;
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_clear_port_changed_matches() {
        check(
            ssx3_ee::s08::f_002c44e0__generated,
            c_ps2_mem_card_clear_port_changed,
            160,
            &|c, seed| {
                let mut r = Rng(0x2c44_e000 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let level = seed % 4;
                c.bus.write32(this + 0xc, level);
                for i in 0..4u32 {
                    c.bus.write32(this + 0x184 + i * 0x14, r.next_u32());
                    c.bus.write32(this + 0x18c + i * 0x14, r.next_u32());
                }
                c.bus.write32(
                    this + 0x184 + level * 0x14,
                    if seed % 3 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32()
                    },
                );
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_is_state34_matches() {
        check(
            ssx3_ee::s08::f_002c4748__generated,
            c_ps2_mem_card_is_state34,
            160,
            &|c, seed| {
                let mut r = Rng(0x2c47_4800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus
                    .write32(DATA + 4, if seed % 3 == 0 { 0x34 } else { r.next_u32() });
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_is_state35_matches() {
        check(
            ssx3_ee::s08::f_002c4880__generated,
            c_ps2_mem_card_is_state35,
            160,
            &|c, seed| {
                let mut r = Rng(0x2c48_8000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus
                    .write32(DATA + 4, if seed % 3 == 0 { 0x35 } else { r.next_u32() });
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_is_busy_matches() {
        check(
            ssx3_ee::s08::f_002c5140__generated,
            c_ps2_mem_card_is_busy,
            160,
            &|c, seed| {
                let mut r = Rng(0x2c51_4000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus
                    .write32(DATA + 4, if seed % 3 == 0 { 0 } else { r.next_u32() | 1 });
                c.bus.write32(
                    DATA + 0x40,
                    if seed % 2 == 0 { 0 } else { r.next_u32() | 1 },
                );
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_is_port_card_error_matches() {
        check(
            ssx3_ee::s08::f_002c5168__generated,
            c_ps2_mem_card_is_port_card_error,
            220,
            &|c, seed| {
                let mut r = Rng(0x2c51_6800 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let level = seed % 3;
                c.bus.write32(this + 0xc, level);
                let values = [
                    0xffff_ffffu32,
                    0xffff_fffd,
                    0xffff_fff8,
                    0xffff_fff7,
                    0xffff_fff6,
                    0x0000_d8ef,
                    0x0000_d8ee,
                    0xffff_d8ef,
                    0xffff_d8ee,
                    0,
                    5,
                    0x8000_0000,
                ];
                c.bus.write32(
                    this + 0x184 + level * 0x14,
                    values[(seed as usize) % values.len()],
                );
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_is_port_not_ps2_card_matches() {
        check(
            ssx3_ee::s08::f_002c5300__generated,
            c_ps2_mem_card_is_port_not_ps2_card,
            160,
            &|c, seed| {
                let mut r = Rng(0x2c53_0000 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let port = seed % 3;
                c.g[A1][0] = port as u64;
                c.bus.write32(
                    this + 0x184 + port * 0x14,
                    if seed % 2 == 0 {
                        0xffff_d8ee
                    } else {
                        r.next_u32()
                    },
                );
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_v52_matches() {
        check(
            ssx3_ee::s08::f_002c5358__generated,
            c_ps2_mem_card_v52,
            220,
            &|c, seed| {
                let mut r = Rng(0x2c53_5800 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let port = seed % 3;
                c.g[A1][0] = port as u64;
                let values = [
                    0xffff_fff9u32, // -7
                    0xffff_d8ef,    // -10001
                    0xffff_fffb,    // -5
                    0xffff_fffc,    // -4
                    0xffff_fffd,    // -3
                    0xffff_fffe,    // -2
                    0xffff_ffff,    // -1
                    0,
                    0xffff_fff8, // -8
                    5,
                    0xffff_fffa, // -6
                    0x8000_0000,
                ];
                c.bus.write32(
                    this + 0x184 + port * 0x14,
                    values[(seed as usize) % values.len()],
                );
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_is_port_formatted_matches() {
        check(
            ssx3_ee::s08::f_002c53b0__generated,
            c_ps2_mem_card_is_port_formatted,
            160,
            &|c, seed| {
                let mut r = Rng(0x2c53_b000 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let port = seed % 3;
                c.g[A1][0] = port as u64;
                c.bus.write32(
                    this + 0x188 + port * 0x14,
                    if seed % 3 == 0 { 0 } else { r.next_u32() | 1 },
                );
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_has_slot_name_matches() {
        check(
            ssx3_ee::s08::f_002c5528__generated,
            c_ps2_mem_card_has_slot_name,
            120,
            &|c, seed| {
                let mut r = Rng(0x2c55_2800 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let slot = r.next_u32() % 3;
                c.g[A1][0] = slot as u64;
                let name = this + slot * 0x78 + 0x1b0;
                let len = match seed % 4 {
                    0 => 0,
                    1 => 1,
                    2 => 5,
                    _ => 12,
                };
                for i in 0..len {
                    c.bus.write8(name + i, b'A' + (seed % 26) as u8);
                }
                c.bus.write8(name + len, 0);
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_mark_slot_used_matches() {
        check(
            ssx3_ee::s08::f_002c5558__generated,
            c_ps2_mem_card_mark_slot_used,
            160,
            &|c, seed| {
                let mut r = Rng(0x2c55_5800 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let slot = r.next_u32() % 3;
                c.g[A1][0] = slot as u64;
                c.bus.write32(this + slot * 0x78 + 0x1a4, r.next_u32());
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_is_slot_empty_matches() {
        check(
            ssx3_ee::s08::f_002c5588__generated,
            c_ps2_mem_card_is_slot_empty,
            160,
            &|c, seed| {
                let mut r = Rng(0x2c55_8800 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let slot = r.next_u32() % 3;
                c.g[A1][0] = slot as u64;
                c.bus.write32(
                    this + slot * 0x78 + 0x1a8,
                    if seed % 2 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(
                    this + slot * 0x78 + 0x1a4,
                    if seed % 3 == 0 { 0 } else { r.next_u32() | 1 },
                );
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_slot_has_1ac_matches() {
        check(
            ssx3_ee::s08::f_002c55c0__generated,
            c_ps2_mem_card_slot_has_1ac,
            160,
            &|c, seed| {
                let mut r = Rng(0x2c55_c000 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let slot = r.next_u32() % 3;
                c.g[A1][0] = slot as u64;
                c.bus.write32(
                    this + slot * 0x78 + 0x1ac,
                    if seed % 3 == 0 { 0 } else { r.next_u32() | 1 },
                );
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_is_port_result_m11_matches() {
        check(
            ssx3_ee::s08::f_002c66d8__generated,
            c_ps2_mem_card_is_port_result_m11,
            160,
            &|c, seed| {
                let mut r = Rng(0x2c66_d800 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let level = seed % 3;
                c.bus.write32(this + 0xc, level);
                c.bus.write32(
                    this + 0x184 + level * 0x14,
                    if seed % 2 == 0 {
                        0xffff_fff5
                    } else {
                        r.next_u32()
                    },
                );
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_calc_save_size_kb_matches() {
        check(
            ssx3_ee::s08::f_002c6758__generated,
            c_ps2_mem_card_calc_save_size_kb,
            200,
            &|c, seed| {
                let mut r = Rng(0x2c67_5800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = sx(r.next_u32());
            },
        );
    }

    // ------------------------------------------------------- memcard

    #[test]
    fn memcard_sjis_to_ascii_string_matches() {
        check(
            ee::s08::f_002c2268__generated,
            memcard_sjis_to_ascii_string,
            SEEDS_6,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002c_2268, 1);
                belib(c);
                c.pc = 0x002c_2268;
                c.g[A1][0] = (DATA + 0x400) as u64;
                c.g[A2][0] = (DATA + 0x200) as u64;
                let n = 1 + seed % 6;
                for i in 0..n {
                    let ch = if seed % 5 == 0 {
                        0x8140 | (r.next_u32() & 0xff) as u16
                    } else if seed % 5 == 1 {
                        0x8260 | (r.next_u32() & 0x1f) as u16
                    } else {
                        (r.next_u32() & 0xff) as u16
                    };
                    h(c, DATA + 0x200 + 2 * i, ch);
                }
                h(c, DATA + 0x200 + 2 * n, 0);
            },
        );
    }

    #[test]
    fn ps2_memcard_request_get_info_matches() {
        check(
            ee::s08::f_002c4480__generated,
            ps2_memcard_request_get_info,
            SEEDS_6,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002c_4480, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 5) as u64;
                let vt = vtable(c, &mut r, &[0x58]);
                w(c, DATA, vt);
                if seed % 2 == 1 {
                    // A callee that answers 0 goes on to restart the state machine.
                    w(c, vt + 0x5c, 0x002a_41c8);
                    c.g[A1][0] = 0;
                }
            },
        );
    }

    #[test]
    fn ps2_memcard_do_get_info_matches() {
        check(
            ee::s08::f_002c50e0__generated,
            ps2_memcard_do_get_info,
            SEEDS_6,
            &|c, seed| {
                machine(c, seed, 0x002c_50e0, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
                w(c, DATA + 0xc, seed % 6);
            },
        );
    }

    #[test]
    fn ps2_mem_card_v38_matches() {
        check(
            ee::s08::f_002c46c0__generated,
            ps2_mem_card_v38,
            32,
            &|c, seed| {
                let mut r = base(c, seed, 0x2c46_c0);
                let s = DATA + 0x100;
                clear_5(c, s, 0x40);
                let vt = DATA + 0x400;
                vtable_2(c, vt, &[(0x120, FAKE + 0x10), (0x124, FAKE + 0x20)]);
                w_2(c, s, vt as u32);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn ps2_mem_card_clear_dir_cache_matches() {
        check(
            ee::s08::f_002c5278__generated,
            ps2_mem_card_clear_dir_cache,
            32,
            &|c, seed| {
                let _ = base(c, seed, 0x2c52_78);
                let s = DATA + 0x1000;
                clear_5(c, s, 0x1000);
                c.g[A0][0] = sx(s);
            },
        );
    }

    #[test]
    fn cps2_mem_card_do_open_matches() {
        check(
            ee::s08::f_002c69a0__generated,
            cps2_mem_card_do_open,
            8,
            &|c, seed| {
                machine(c, seed, 0x002c_69a0, 2);
                be_library(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0xc, 0);
                c.bus.write32(DATA + 0x14e0, 0);
                c.bus.write32(0x0052_71c0 + 0x24, 0);
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_dtor_matches() {
        check(
            ee::s08::f_002c3fe0__generated,
            c_ps2_mem_card_dtor,
            4,
            &|c, seed| {
                let _ = base_2(c, seed, 0x002c_3fe0, 2);
                c.g[A0][0] = OBJ as u64;
                // No buffer and no delete: a non-pool_2 pointer would walk the test
                // heap.
                c.g[A1][0] = 0;
                w(c, OBJ + 0x30, 0);
            },
        );
    }

    #[test]
    fn cps2_mem_card_count_saves_matches() {
        check(
            ee::s08::f_002c68f0__generated,
            cps2_mem_card_count_saves,
            SEEDS_7,
            &|c, seed| {
                machine(c, seed, 0x2c_68f0, 1);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                let n = (seed % 4) as u32;
                w(c, DATA + 0x2000 + 0x14, n);
                for i in 0..4u32 {
                    let name = DATA + 0x2000 + 0x1b0 + 0x78 * i;
                    if i % 2 == 0 && i < n {
                        string(c, name, "SAVE");
                    } else {
                        string(c, name, "");
                    }
                }
                c.pc = 0x002c_68f0;
            },
        );
    }

    /// A NUL-terminated UTF-16 string_2 at `a` (for the wide-char memcard name).
    pub(crate) fn wide_string(c: &mut Cpu, a: u32, s: &str) {
        for (i, ch) in s.encode_utf16().enumerate() {
            h(c, a + 2 * i as u32, ch);
        }
        h(c, a + 2 * s.encode_utf16().count() as u32, 0);
    }

    #[test]
    fn cps2_mem_card_v37_matches() {
        check(
            ee::s08::f_002c45d0__generated,
            cps2_mem_card_v37,
            SEEDS_8,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c45d0, 3);
                be_library_2(c);
                let obj = DATA;
                let name = DATA + 0x800;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = name as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                wide_string(c, name, if seed % 2 == 0 { "SAVE01" } else { "" });
            },
        );
    }

    #[test]
    fn cps2_mem_card_v36_matches() {
        check(
            ee::s08::f_002c4648__generated,
            cps2_mem_card_v36,
            SEEDS_8,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c4648, 3);
                be_library_2(c);
                let obj = DATA;
                let name = DATA + 0x800;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = name as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                string_2(c, name, if seed % 2 == 0 { "SAVE01" } else { "" });
            },
        );
    }

    #[test]
    fn cps2_mem_card_request_read_save_matches() {
        check(
            ee::s08::f_002c4788__generated,
            cps2_mem_card_request_read_save,
            SEEDS_8,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c4788, 3);
                be_library_2(c);
                let obj = DATA;
                let name = DATA + 0x800;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = name as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                string_2(c, name, if seed % 2 == 0 { "SLOT-A" } else { "" });
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_do_list_dir_matches() {
        check(
            ee::s08::f_002c4210__generated,
            c_ps2_mem_card_do_list_dir,
            40,
            &|c, seed| {
                let _ = base_3(c, seed, 0x2c42, 3);
                let m = DATA;
                w(c, m + 0xc, seed);
                w(c, m + 0x139, 0x4142);
                c.g[A0][0] = sx(m);
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_do_list_dir_all_matches() {
        check(
            ee::s08::f_002c6848__generated,
            c_ps2_mem_card_do_list_dir_all,
            40,
            &|c, seed| {
                let _ = base_3(c, seed, 0x2c68, 3);
                let m = DATA;
                w(c, m + 0xc, seed);
                w(c, m + 0x139, 0x4142);
                c.g[A0][0] = sx(m);
            },
        );
    }

    // ------------------------------------------------------------------
    // Memory card.

    #[test]
    fn c_ps2_mem_card_close_matches() {
        check(
            ssx3_ee::s08::f_002c6360__generated,
            c_ps2_mem_card_close,
            SEEDS_9,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c6360, 4);
                be_library_3(c);
                world_2(c);
                let card = DATA + 0x100;
                c.g[A0][0] = card as u64;
                w(c, card + 0x40, if seed % 2 == 0 { 0 } else { 1 });
                w(c, card + 0xc, r.next_u32() & 0x7);
                w(
                    c,
                    GAME_GP.wrapping_add(0x850),
                    if seed % 3 == 0 { 0xffff_ffff } else { 0 },
                );
            },
        );
    }

    #[test]
    fn cps2_memcard_get_slot_name_matches() {
        check(
            ee::s08::f_002c63f8__generated,
            cps2_memcard_get_slot_name,
            80,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x2c63);
                let dst = DATA + 0x2000;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = sx(dst);
                slot_2(c, VTS, 0x20, 0);
                fill(c, &mut r, dst, 0x100);
                put_words(c, 0x004a_3970, &[0]);
            },
        );
    }

    pub(crate) const TBL: u32 = DATA + 0x2000;

    // ------------------------------------------------------------- localisation

    #[test]
    fn loc_string_table_find_matches() {
        check(
            ee::s08::f_002c6de0__generated,
            loc_string_table_find,
            18,
            &|c, seed| {
                let _ = base_5(c, seed, 0x002c_6de0, 0);
                c.g[A0][0] = OBJ as u64;
                let count = if seed % 7 == 0 { 0 } else { 1 + (seed % 6) };
                w(c, OBJ + 0xc, count);
                w(c, OBJ + 8, count.saturating_sub(1));
                w(c, OBJ + 0x10, TBL);
                let mut keys = [0u32; 8];
                for i in 0..count {
                    keys[i as usize] = 0x10 + i * 4;
                    w(c, TBL + i * 8, keys[i as usize]);
                    w(c, TBL + i * 8 + 4, 0x1000 + i);
                }
                let i = (seed as usize) % count.max(1) as usize;
                let delta = (seed % 5) as i64 - 2;
                c.g[A1][0] = (keys[i] as i64 + delta) as u32 as u64;
            },
        );
    }

    #[test]
    fn cps2_mem_card_do_get_save_dir_entry_matches() {
        check(
            ee::s08::f_002c42b0__generated,
            cps2_mem_card_do_get_save_dir_entry,
            4,
            &|c, seed| {
                let _ = base_5(c, seed, 0x002c_42b0, 4);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 3) as u64;
                w(c, OBJ + 0x1c, 0);
                w(c, OBJ + 0xc, 0);
                w(c, OBJ + 0x3c, 0);
                w(c, GAME_GP.wrapping_add(0x850), 0);
            },
        );
    }

    #[test]
    fn cps2memcard_mark_slots_without_space_matches() {
        check(
            ee::s08::f_002c6280__generated,
            cps2memcard_mark_slots_without_space,
            12,
            &|c, seed| {
                let mut r = base_6(c, seed, 0x2c6280, 3);
                let this = DATA;
                clear_6(c, this, 0x2000);
                c.g[A0][0] = this as u64;
                let vt = vtable(c, &mut r, &[0xa0, 0xb0, 0x218]);
                w(c, this, vt);
                w(c, vt + 0x21c, 0x002c_c098);
                w(c, this + 0x24, (seed % 8) as u32);
                w(c, this + 0x14, 2);
                w(c, this + 0x150, (seed % 3) as u32);
                wf(c, this + 0x100, 2.0);
                wf(c, this + 0x104, 1.0);
                for i in 0..2u32 {
                    let e = this + 0x1ac + 0x78 * i;
                    w(c, e, 0);
                }
            },
        );
    }

    #[test]
    fn quad_tree_flatten_node_matches() {
        strict(
            ee::s08::f_002c2088__generated,
            quad_tree_flatten_node,
            SEEDS_10,
            &|c, seed| {
                let mut r = base_7(c, seed, 0x2088, 0);
                let this = DATA + 0x1000;
                let base_7 = DATA + 0x4000;
                let count = DATA + 0x100;
                let out = DATA + 0x2000;
                let map = DATA + 0x3000;
                w(c, count, 0);
                w(c, this + 0x20, base_7);
                // the root's four children are leaves at base_7 + 0x100 + 0x10*k
                for k in 0..4u32 {
                    let child = base_7 + 0x100 + 0x10 * k;
                    let val = 2 * (0x100 + 0x10 * k) + 1;
                    c.bus.write16(base_7 + 2 * k, val as u16);
                    w(c, child, 0);
                    w(c, child + 8, r.next_u32());
                }
                w(c, base_7 + 8, r.next_u32());
                for k in 0..4u32 {
                    c.bus.write16(
                        base_7 + 8 + 2 * k,
                        if seed % 2 == 0 {
                            0
                        } else {
                            (0x201 + 0x20 * k) as u16
                        },
                    );
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(out);
                c.g[A2][0] = sx(count);
                c.g[A3][0] = sx(map);
                c.g[T0][0] = sx(base_7);
            },
        );
    }
}
