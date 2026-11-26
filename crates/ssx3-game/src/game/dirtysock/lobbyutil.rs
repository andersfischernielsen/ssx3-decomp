//! DirtySDK lobby lists, hashes, text, ciphers, and connection utilities.

use crate::abi::addu;
use crate::abi::r32;
use crate::abi::ret;
use crate::abi::set_word;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::tail_wrapper;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Target;
use crate::call::Unwound;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::game::dirtysock::buddyapi::DIRTY_RPC_STATE;
use crate::game::lib::libc::callee::UDIVDI3;
use crate::guest::Addr;
use callee::*;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003e_e028, lobby_list_grow, "Lobby_ListGrow"),
    (
        0x003e_e1b0,
        lobby_list_compare_thunk,
        "Lobby_ListCompareThunk",
    ),
    (0x003e_e1e8, lobby_list_create, "Lobby_ListCreate"),
    (0x003e_e260, lobby_list_destroy, "Lobby_ListDestroy"),
    (0x003e_e320, lobby_list_add, "Lobby_ListAdd"),
    (0x003e_e3e0, lobby_list_update, "Lobby_ListUpdate"),
    (0x003e_e468, lobby_list_get_data, "Lobby_ListGetData"),
    (0x003e_e480, lobby_list_remove, "Lobby_ListRemove"),
    (0x003e_e550, lobby_list_get_count, "Lobby_ListGetCount"),
    (
        0x003e_e558,
        lobby_list_get_visible_count,
        "Lobby_ListGetVisibleCount",
    ),
    (0x003e_e560, lobby_list_get_at, "Lobby_ListGetAt"),
    (0x003e_e5a0, lobby_list_set_sort, "Lobby_ListSetSort"),
    (0x003e_e600, lobby_list_mark_dirty, "Lobby_ListMarkDirty"),
    (0x003e_e620, lobby_list_sort, "Lobby_ListSort"),
    (0x003e_e768, lobby_list_set_hash, "Lobby_ListSetHash"),
    (0x003e_e770, lobby_list_get_hash, "Lobby_ListGetHash"),
    (0x003e_e778, lobby_hash_create, "Lobby_HashCreate"),
    (0x003e_e820, lobby_hash_destroy, "Lobby_HashDestroy"),
    (0x003e_e900, lobby_hash_grow_pool, "Lobby_HashGrowPool"),
    (0x003e_e9a0, lobby_hash_pop_any, "Lobby_HashPopAny"),
    (0x003e_ea48, lobby_hash_string, "Lobby_HashString"),
    (0x003e_eb30, lobby_hash_add_str, "Lobby_HashAddStr"),
    (0x003e_ebf8, lobby_hash_replace_str, "Lobby_HashReplaceStr"),
    (0x003e_ec88, lobby_hash_find_str, "Lobby_HashFindStr"),
    (0x003e_ed08, lobby_hash_del_str, "Lobby_HashDelStr"),
    (0x003e_eda0, lobby_hash_add_int, "Lobby_HashAddInt"),
    (0x003e_eee8, lobby_hash_find_int, "Lobby_HashFindInt"),
    (0x003e_ef40, lobby_hash_del_int, "Lobby_HashDelInt"),
    (0x003e_f088, ds_timeinsecs, "ds_timeinsecs"),
    (0x003e_f090, ds_timezone, "ds_timezone"),
    (0x003e_f210, lobby_merge_sort, "Lobby_MergeSort"),
    (0x003e_f600, utf8_get_seq_len, "Utf8_GetSeqLen"),
    (0x003e_f640, utf8_decode_code_pt, "Utf8_DecodeCodePt"),
    (
        0x003e_f6f8,
        utf8_encode_from_ucs2_code_pt,
        "Utf8EncodeFromUCS2CodePt",
    ),
    (0x003e_f770, utf8_translate_code_pt, "Utf8_TranslateCodePt"),
    (0x003e_f808, utf8_strip, "Utf8Strip"),
    (0x003e_f828, utf8_replace, "Utf8Replace"),
    (0x003e_f940, utf8_str_len, "Utf8StrLen"),
    (0x003e_f9b8, utf8_encode_from_ucs2, "Utf8EncodeFromUCS2"),
    (0x003e_fa48, utf8_decode_to_ucs2, "Utf8DecodeToUCS2"),
    (0x003e_faf8, utf8_encode_from_8_bit, "Utf8EncodeFrom8Bit"),
    (0x003e_fba0, utf8_translate_to_8_bit, "Utf8TranslateTo8Bit"),
    (0x003e_fc68, lobby_parse_hex, "Lobby_ParseHex"),
    (0x003e_fcd8, lobby_parse_hex_range, "Lobby_ParseHexRange"),
    (
        0x003e_fd30,
        lobby_hash_memory_ranges,
        "Lobby_HashMemoryRanges",
    ),
    (0x003f_2220, crypt_md5_transform, "CryptMD5_Transform"),
    (0x003f_30f0, crypt_md5_init, "CryptMD5_Init"),
    (0x003f_3128, crypt_md5_update, "CryptMD5_Update"),
    (0x003f_31f8, crypt_md5_final, "CryptMD5_Final"),
    (0x003f_33b0, lobby_cipher_init, "Lobby_CipherInit"),
    (0x003f_3530, lobby_cipher_apply, "Lobby_CipherApply"),
    (0x003f_35d0, lobby_encrypt_string, "Lobby_EncryptString"),
    (0x003f_3748, lobby_decrypt_string, "Lobby_DecryptString"),
    (0x003f_3860, socket_lookup_done, "SocketLookup_Done"),
    (0x003f_3940, socket_lookup_free, "SocketLookup_Free"),
    (0x003f_39c8, socket_lookup_create, "SocketLookup_Create"),
    (0x003f_3a98, socket_lookup, "SocketLookup"),
    (0x003f_3ab8, socket_lookup_sync, "SocketLookupSync"),
    (0x003f_3b18, net_conn_startup, "NetConnStartup"),
    (0x003f_3bc8, net_conn_query, "NetConnQuery"),
    (0x003f_3c50, net_conn_connect, "NetConnConnect"),
    (0x003f_3cc0, net_conn_status, "NetConnStatus"),
    (0x003f_3f78, net_conn_mac, "NetConnMAC"),
    (0x003f_40b8, net_conn_disconnect, "NetConnDisconnect"),
    (0x003f_4118, net_conn_shutdown, "NetConnShutdown"),
    (0x003f_4198, net_tick_wrapper, "NetTick_Wrapper"),
    (0x003f_41b8, net_conn_sleep, "NetConnSleep"),
    (0x003f_42a0, net_conn_idle, "NetConnIdle"),
    (0x003f_4330, net_tick_wrapper2, "NetTick_Wrapper2"),
    (0x003f_43d0, net_conn_return_true, "NetConn_ReturnTrue"),
    (0x003f_4de8, net_tick, "NetTick"),
    (0x0025_91e8, lobby_mem_alloc, "Lobby_MemAlloc"),
    (0x0025_9210, lobby_mem_free, "Lobby_MemFree"),
    (
        0x003e_eab8,
        lobby_hash_str_cmp_no_case,
        "Lobby_HashStrCmpNoCase",
    ),
];

pub(crate) mod callee {
    use super::{ee, Callee};
    pub(super) use crate::game::dirtysock::buddyapi::callee::{
        DIRTY_RPC_CALL, DIRTY_RPC_INIT_REQUEST, DIRTY_RPC_PROCESS_REPLIES, LOBBY_MEM_ALLOC,
        LOBBY_MEM_FREE, NET_RPC_ALLOC, NET_RPC_FREE, NET_TICK,
    };
    pub(super) use crate::game::lib::libc::callee::MEMCPY;
    pub(super) use crate::game::misc::MEMSET;

    pub(crate) const NET_CONN_DISCONNECT: Callee = Callee {
        addr: 0x003f_40b8,
        f: ee::s14::NetConnDisconnect,
    };

    pub(crate) const NET_CONN_STATUS: Callee = Callee {
        addr: 0x003f_3cc0,
        f: ee::s14::NetConnStatus,
    };

    pub(crate) const DIRTY_RPC_IS_STARTED: Callee = Callee {
        addr: 0x003f_5470,
        f: ee::s14::DirtyRpc_IsStarted,
    };

    pub(crate) const DIRTY_RPC_STARTUP: Callee = Callee {
        addr: 0x003f_5000,
        f: ee::s14::DirtyRpc_Startup,
    };

    pub(crate) const DIRTY_RPC_INIT_IOP: Callee = Callee {
        addr: 0x003f_4d68,
        f: ee::s14::DirtyRpc_InitIop,
    };

    pub(crate) const SOCKET_LOOKUP_CREATE: Callee = Callee {
        addr: 0x003f_39c8,
        f: ee::s14::SocketLookup_Create,
    };

    pub(crate) const LOBBY_CIPHER_INIT: Callee = Callee {
        addr: 0x003f_33b0,
        f: ee::s14::Lobby_CipherInit,
    };

    pub(crate) const LOBBY_CIPHER_APPLY: Callee = Callee {
        addr: 0x003f_3530,
        f: ee::s14::Lobby_CipherApply,
    };

    pub(crate) const CRYPT_MD5_TRANSFORM: Callee = Callee {
        addr: 0x003f_2220,
        f: ee::s14::CryptMD5_Transform,
    };

    pub(crate) const CRYPT_MD5_INIT: Callee = Callee {
        addr: 0x003f_30f0,
        f: ee::s14::CryptMD5_Init,
    };

    pub(crate) const CRYPT_MD5_UPDATE: Callee = Callee {
        addr: 0x003f_3128,
        f: ee::s14::CryptMD5_Update,
    };

    pub(crate) const CRYPT_MD5_FINAL: Callee = Callee {
        addr: 0x003f_31f8,
        f: ee::s14::CryptMD5_Final,
    };

    pub(crate) const LOBBY_PARSE_HEX: Callee = Callee {
        addr: 0x003e_fc68,
        f: ee::s14::Lobby_ParseHex,
    };

    pub(crate) const LOBBY_PARSE_HEX_RANGE: Callee = Callee {
        addr: 0x003e_fcd8,
        f: ee::s14::Lobby_ParseHexRange,
    };

    pub(crate) const UTF8_DECODE_CODE_PT: Callee = Callee {
        addr: 0x003e_f640,
        f: ee::s14::Utf8_DecodeCodePt,
    };

    pub(crate) const UTF8_TRANSLATE_CODE_PT: Callee = Callee {
        addr: 0x003e_f770,
        f: ee::s14::Utf8_TranslateCodePt,
    };

    pub(crate) const UTF8_ENCODE_FROM_UCS2_CODE_PT: Callee = Callee {
        addr: 0x003e_f6f8,
        f: ee::s14::Utf8EncodeFromUCS2CodePt,
    };

    pub(crate) const UTF8_GET_SEQ_LEN: Callee = Callee {
        addr: 0x003e_f600,
        f: ee::s14::Utf8_GetSeqLen,
    };

    pub(crate) const UTF8_REPLACE: Callee = Callee {
        addr: 0x003e_f828,
        f: ee::s14::Utf8Replace,
    };

    pub(crate) const LOBBY_HASH_STRING: Callee = Callee {
        addr: 0x003e_ea48,
        f: ee::s14::Lobby_HashString,
    };

    pub(crate) const LOBBY_HASH_GROW_POOL: Callee = Callee {
        addr: 0x003e_e900,
        f: ee::s14::Lobby_HashGrowPool,
    };

    pub(crate) const LOBBY_MERGE_SORT: Callee = Callee {
        addr: 0x003e_f210,
        f: ee::s14::Lobby_MergeSort,
    };

    pub(crate) const SCE_SCF_GET_GMT_FROM_RTC: Callee = Callee {
        addr: 0x0040_bfd8,
        f: ee::s15::sceScfGetGMTfromRTC,
    };

    pub(crate) const SCE_SCF_GET_LOCAL_TIME_FROM_RTC: Callee = Callee {
        addr: 0x0040_bfe0,
        f: ee::s15::sceScfGetLocalTimefromRTC,
    };

    pub(crate) const LOBBY_LIST_GROW: Callee = Callee {
        addr: 0x003e_e028,
        f: ee::s14::Lobby_ListGrow,
    };
    pub(crate) use crate::game::dirtysock::tagsunk::callee::EA_MEM_FREE;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::OPERATOR_NEW_NAMED;
}

pub(crate) const DIRTY_RPC_STATE_HI: u64 = 0x0052_0000;

pub(crate) const NET_TICK_COUNT: u32 = 0x4df8;

pub(crate) const NET_TICK_EPOCH: u32 = 0x4e00;

pub(crate) const NET_TICK_MILLISECOND_CYCLES: u64 = 0x48000;

pub(crate) const NET_TICK_RETRY: u32 = 0x003f_4df8;

pub(crate) const TICK_FRAME: u32 = 0x30;

pub(crate) const TICK_SAVED_RA: u32 = 0x20;

pub(crate) const TICK_SAMPLE: u32 = 0;

pub(crate) const TICK_COUNT_SAMPLE: u32 = 4;

pub(crate) const TICK_EPOCH_SAMPLE: u32 = 8;

pub(crate) const TICK_ELAPSED: u32 = 0x10;

pub(crate) const NET_CONN_OPEN_GP: u32 = 0x1ab0;

pub(crate) const NET_CONN_HANDLE_GP: u32 = 0x1ab4;

pub(crate) const NET_CONN_NEXT_IDLE_GP: u32 = 0x1abc;

pub(crate) const NET_CONN_IDLE_CALLBACKS: u32 = 0x0051_f030;

pub(crate) const NET_CONN_IDLE_CALLBACK_COUNT: u64 = 16;

pub(crate) const NET_CONN_IDLE_CALLBACK_ARG: u32 = 4;

pub(crate) const NET_CONN_IDLE_CALLBACK_STRIDE: u32 = 8;

pub(crate) const NET_CONN_IDLE_HEAD: u32 = 0x003f_42f0;

pub(crate) const NET_CONN_IDLE_FRAME: u32 = 0x30;

pub(crate) const NET_CONN_IDLE_SAVED_RA: u32 = 0;

pub(crate) const NET_CONN_IDLE_SAVED_S1: u32 = 0x10;

pub(crate) const NET_CONN_IDLE_SAVED_S0: u32 = 0x20;

pub(crate) const NET_CONN_DELAY_MS: u32 = 5;

pub(crate) const NET_CONN_SLEEP_ENTRY: u32 = 0x003f_41b8;

pub(crate) const NET_CONN_SLEEP_HEAD: u32 = 0x003f_41e0;

pub(crate) const NET_CONN_SLEEP_END: u32 = 0x003f_420c;

pub(crate) const NET_CONN_SLEEP_FRAME: u32 = 0x20;

pub(crate) const NET_CONN_SLEEP_SAVED_S0: u32 = 0x10;

pub(crate) const NET_RPC_SERVICE: u64 = u32::from_be_bytes(*b"netw") as u64;

pub(crate) const NET_RPC_STOP: u64 = u32::from_be_bytes(*b"shut") as u64;

pub(crate) const NET_RPC_DESTROY: u64 = u32::from_be_bytes(*b"dest") as u64;

pub(crate) const NET_RPC_CREATE: u64 = u32::from_be_bytes(*b"crea") as u64;

pub(crate) const NET_RPC_START: u64 = u32::from_be_bytes(*b"star") as u64;

pub(crate) const NET_RPC_LIST: u64 = u32::from_be_bytes(*b"list") as u64;

pub(crate) const NET_RPC_STAT: u64 = u32::from_be_bytes(*b"stat") as u64;

pub(crate) const NET_RPC_TAG: u64 = u32::from_be_bytes(*b"1731") as u64;

pub(crate) const NET_RPC_REPLY_WORD: u32 = 0x20;

pub(crate) const NET_RPC_STATUS_WORD: u32 = NET_RPC_REPLY_WORD + 4;

pub(crate) const RPC_CALL_FLAGS_OUTPUT: u64 = 5;

pub(crate) const RPC_NUL_TERMINATED: u64 = u64::MAX;

pub(crate) const RPC_QUERY_ENTRY_SHIFT: u32 = 8;

pub(crate) const NET_CONN_STATUS_ENTRY: u32 = 0x003f_3cc0;

pub(crate) const NET_CONN_STATUS_BROADBAND_HEAD: u32 = 0x003f_3dc0;

pub(crate) const NET_CONN_STATUS_END: u32 = 0x003f_3f74;

pub(crate) const NET_CONN_STATUS_FRAME: u32 = 0x80;

pub(crate) const NET_CONN_STATUS_SAVED_RA: u32 = 0x30;

pub(crate) const NET_CONN_STATUS_SAVED_S0: u32 = 0x70;

pub(crate) const NET_CONN_STATUS_SAVED_S1: u32 = 0x60;

pub(crate) const NET_CONN_STATUS_SAVED_S2: u32 = 0x50;

pub(crate) const NET_CONN_STATUS_SAVED_S3: u32 = 0x40;

pub(crate) const NET_STATUS_OPEN: u64 = u32::from_be_bytes(*b"open") as u64;

pub(crate) const NET_STATUS_ONLINE: u64 = u32::from_be_bytes(*b"onln") as u64;

pub(crate) const NET_STATUS_CONNECTION: u64 = u32::from_be_bytes(*b"conn") as u64;

pub(crate) const NET_STATUS_HOST: u64 = u32::from_be_bytes(*b"host") as u64;

pub(crate) const NET_STATUS_ADDRESS: u64 = u32::from_be_bytes(*b"addr") as u64;

pub(crate) const NET_STATUS_MAC: u64 = u32::from_be_bytes(*b"macx") as u64;

pub(crate) const NET_STATUS_BROADBAND: u64 = u32::from_be_bytes(*b"bbnd") as u64;

pub(crate) const NET_STATUS_TYPE: u64 = u32::from_be_bytes(*b"type") as u64;

pub(crate) const NET_STATUS_BROADBAND_BIT: u32 = 2;

pub(crate) const HIGH_HALF_MASK: u32 = 0xffff_0000;

pub(crate) const SELECTOR_PREFIX_MASK: u64 = HIGH_HALF_MASK as u64;

pub(crate) const NET_MAC_CACHE_BASE: u64 = 0x0045_0000;

pub(crate) const NET_MAC_CACHE_OFFSET: u32 = 0x12e8;

pub(crate) const NET_MAC_REPLY_SIZE: u64 = 16;

pub(crate) const NET_MAC_HEX_BASE: u32 = 0x0049_7028;

pub(crate) const NET_MAC_NIBBLE_MASK: u64 = 0xf;

pub(crate) const NET_MAC_PREFIX: u64 = b'$' as u64;

pub(crate) const NET_MAC_FRAME: u32 = 0x50;

pub(crate) const NET_MAC_SAVED_RA: u32 = 0x10;

pub(crate) const NET_MAC_SAVED_S0: u32 = 0x40;

pub(crate) const NET_MAC_SAVED_S1: u32 = 0x30;

pub(crate) const NET_MAC_SAVED_S2: u32 = 0x20;

pub(crate) const LOOKUP_STATUS: u32 = 0;

pub(crate) const LOOKUP_ADDRESS: u32 = 4;

pub(crate) const LOOKUP_DONE_CALLBACK: u32 = 8;

pub(crate) const LOOKUP_FREE_CALLBACK: u32 = 0xc;

pub(crate) const LOOKUP_REMOTE_HANDLE: u32 = 0x58;

pub(crate) const LOOKUP_SIZE: u32 = 0x5c;

pub(crate) const LOOKUP_RPC_SERVICE: u64 = u32::from_be_bytes(*b"name") as u64;

pub(crate) const LOOKUP_RPC_CREATE: u64 = u32::from_be_bytes(*b"look") as u64;

pub(crate) const LOOKUP_RPC_FREE: u64 = u32::from_be_bytes(*b"free") as u64;

pub(crate) const LOOKUP_RPC_DONE: u64 = u32::from_be_bytes(*b"done") as u64;

pub(crate) const LOOKUP_RPC_ADDRESS: u64 = u32::from_be_bytes(*b"addr") as u64;

pub(crate) const LOOKUP_RPC_DONE_DESC: u32 = 0x0051_f000;

pub(crate) const LOOKUP_RPC_FREE_DESC: u32 = LOOKUP_RPC_DONE_DESC + 0x10;

pub(crate) const LOOKUP_RPC_CREATE_DESC: u32 = LOOKUP_RPC_DONE_DESC + 0x20;

pub(crate) const LOOKUP_SYNC_HEAD: u32 = 0x003f_3ae0;

pub(crate) const LOOKUP_DONE_ADDR: u32 = 0x003f_3860;

pub(crate) const LOOKUP_FREE_ADDR: u32 = 0x003f_3940;

pub(crate) const CIPHER_STRING_FRAME: u32 = 0x1b0;

pub(crate) const CIPHER_STRING_S0: u32 = 0x1a0;

pub(crate) const CIPHER_STRING_S1: u32 = 0x190;

pub(crate) const CIPHER_STRING_S2: u32 = 0x180;

pub(crate) const CIPHER_STRING_S3: u32 = 0x170;

pub(crate) const CIPHER_STRING_S4: u32 = 0x160;

pub(crate) const CIPHER_STRING_S5: u32 = 0x150;

pub(crate) const CIPHER_STRING_S6: u32 = 0x140;

pub(crate) const CIPHER_STRING_S7: u32 = 0x130;

pub(crate) const CIPHER_STRING_RA: u32 = 0x120;

pub(crate) const CIPHER_STRING_BYTE: u32 = 0x110;

pub(crate) const CIPHER_STRING_STREAM_BYTE: u32 = CIPHER_STRING_BYTE + 1;

pub(crate) const CIPHER_PADDING_STATE: u32 = 0x0051_eef8;

pub(crate) const CIPHER_PADDING_BANK: u64 = 0x0052_0000;

pub(crate) const CIPHER_PADDING_KEY: u32 = 0x0049_7018;

pub(crate) const CIPHER_PADDING_MASK: u64 = 0x3f;

pub(crate) const CIPHER_FIRST_PRINTABLE: u32 = b' ' as u32;

pub(crate) const CIPHER_PRINTABLE_COUNT: u64 = 95;

pub(crate) const CIPHER_STRING_ALPHABET: u64 = 96;

pub(crate) const CIPHER_STRING_BIAS: u32 = 0x40;

pub(crate) const CIPHER_STRING_TERMINATOR: u64 = 0x7f;

pub(crate) const CIPHER_ENCRYPT_HEAD: u32 = 0x003f_3670;

pub(crate) const CIPHER_DECRYPT_HEAD: u32 = 0x003f_37c0;

pub(crate) const NET_RPC_SHORT_FRAME: u32 = 0x30;

pub(crate) const NET_RPC_SHORT_SAVED_RA: u32 = 0x20;

pub(crate) const NET_RPC_REQUEST: u32 = 0x10;

pub(crate) const RPC_HAS_REPLY: u32 = 0;

pub(crate) const RPC_REPLY_OUT: u32 = 8;

pub(crate) const RPC_CALL_FLAGS: u64 = 1;

pub(crate) fn net_conn_gp(c: &Cpu, offset: u32) -> Addr {
    Addr(r32(c, GP)).off(offset)
}

/// Reads the EE clock against a consistent DirtyRPC epoch and returns milliseconds.
pub fn net_tick(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, TICK_FRAME);
        c.g[V0][0] = DIRTY_RPC_STATE_HI;
        frame.save_ra(c, TICK_SAVED_RA);
        c.g[A0][0] = sx(DIRTY_RPC_STATE);
        loop {
            c.cycles += 11;
            let state = Addr(r32(c, A0));
            c.g[V0][0] = state.off(NET_TICK_COUNT).lw(c);
            c.sw(frame.at(c, TICK_COUNT_SAMPLE), c.g[V0][0]);
            c.g[V1][0] = state.off(NET_TICK_EPOCH).ld(c);
            c.sd(frame.at(c, TICK_EPOCH_SAMPLE), c.g[V1][0]);
            c.mfc0(V0, 9);
            c.sw(frame.at(c, TICK_SAMPLE), c.g[V0][0]);
            c.g[V1][0] = c.lw(frame.at(c, TICK_SAMPLE));
            c.g[V1][0] = c.lw(frame.at(c, TICK_COUNT_SAMPLE));
            c.g[V0][0] = state.off(NET_TICK_COUNT).lw(c);
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 4;
                c.g[V1][0] = c.ld(frame.at(c, TICK_EPOCH_SAMPLE));
                c.g[V0][0] = state.off(NET_TICK_EPOCH).ld(c);
                if c.g[V1][0] == c.g[V0][0] {
                    break;
                }
            }
            if event_due(c) {
                deliver_event_here(c, NET_TICK_RETRY);
            }
        }
        c.cycles += 18;
        c.g[V0][0] = c.lw(frame.at(c, TICK_SAMPLE));
        c.g[A1][0] = NET_TICK_MILLISECOND_CYCLES;
        c.g[A0][0] = c.lw(frame.at(c, TICK_COUNT_SAMPLE));
        c.g[V1][0] = c.ld(frame.at(c, TICK_EPOCH_SAMPLE));
        let since_epoch = r32(c, V0).wrapping_sub(r32(c, A0));
        c.g[V0][0] = u64::from(since_epoch);
        c.g[V1][0] = c.g[V1][0].wrapping_add(c.g[V0][0]);
        c.sd(frame.at(c, TICK_ELAPSED), c.g[V1][0]);
        c.g[A0][0] = c.ld(frame.at(c, TICK_ELAPSED));
        UDIVDI3.call(c, 0x003f_4e68)?;
        frame.restore_ra(c, TICK_SAVED_RA);
        c.g[V0][0] = sx(r32(c, V0));
        frame.close(c);
        Ok(())
    });
}

/// Always returns true, preserving the variadic argument spills.
pub fn net_conn_return_true(c: &mut Cpu) {
    const VARIADIC_FRAME: u32 = 0x80;
    const FIRST_GPR_SPILL: u32 = 0x48;
    const FIRST_FPR_SPILL: u32 = 0x38;
    c.cycles += 15;
    let frame = Frame::open(c, VARIADIC_FRAME);
    c.g[V0][0] = 1;
    for (index, reg) in [A1, A2, A3, T0, T1, T2, T3].into_iter().enumerate() {
        c.sd(frame.at(c, FIRST_GPR_SPILL + index as u32 * 8), c.g[reg][0]);
    }
    for (index, reg) in [12, 14, 16, 18].into_iter().enumerate() {
        c.swc1(reg, frame.at(c, FIRST_FPR_SPILL + index as u32 * 4));
    }
    frame.close(c);
}

/// Forwards the second networking timer callback to NetTick.
pub fn net_tick_wrapper2(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(NET_TICK), 0x003f_4340);
}

/// Polls pending RPC replies and invokes registered network idle callbacks.
pub fn net_conn_idle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, NET_CONN_IDLE_FRAME);
        c.g[V0][0] = net_conn_gp(c, NET_CONN_OPEN_GP).lw(c);
        frame.save(c, S0, NET_CONN_IDLE_SAVED_S0);
        frame.save(c, S1, NET_CONN_IDLE_SAVED_S1);
        frame.save_ra(c, NET_CONN_IDLE_SAVED_RA);
        if c.g[V0][0] != 0 {
            c.cycles += 6;
            NET_TICK.call(c, 0x003f_42c0)?;
            c.g[V1][0] = net_conn_gp(c, NET_CONN_NEXT_IDLE_GP).lw(c);
            c.g[V0][0] = u64::from(c.g[V0][0] < c.g[V1][0]);
            let wait = c.g[V0][0] != 0;
            frame.restore(c, S0, NET_CONN_IDLE_SAVED_S0);
            if wait {
                c.cycles += 4;
                frame.restore(c, S1, NET_CONN_IDLE_SAVED_S1);
                frame.restore_ra(c, NET_CONN_IDLE_SAVED_RA);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 4;
            NET_TICK.call(c, 0x003f_42d8)?;
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(NET_CONN_DELAY_MS));
            c.sw(net_conn_gp(c, NET_CONN_NEXT_IDLE_GP).0, c.g[V0][0]);
        }

        c.cycles += 4;
        c.g[S1][0] = NET_CONN_IDLE_CALLBACK_COUNT - 1;
        DIRTY_RPC_PROCESS_REPLIES.call(c, 0x003f_42e8)?;
        c.g[V0][0] = DIRTY_RPC_STATE_HI;
        c.g[S0][0] = sx(NET_CONN_IDLE_CALLBACKS);
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
            let active = c.g[V0][0] != 0;
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            if active {
                c.cycles += 6;
                NET_TICK.call(c, 0x003f_4304)?;
                c.g[V1][0] = Addr(r32(c, S0)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                let callback = r32(c, V1);
                c.g[A0][0] = Addr(r32(c, S0)).off(NET_CONN_IDLE_CALLBACK_ARG).lw(c);
                call_ptr(c, callback, 0x003f_4314)?;
            }
            c.cycles += 2;
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(NET_CONN_IDLE_CALLBACK_STRIDE));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, NET_CONN_IDLE_HEAD);
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, NET_CONN_IDLE_SAVED_S0);
        c.cycles += 4;
        frame.restore(c, S1, NET_CONN_IDLE_SAVED_S1);
        frame.restore_ra(c, NET_CONN_IDLE_SAVED_RA);
        frame.close(c);
        Ok(())
    });
}

/// Busy-waits for the requested milliseconds, defaulting to five for zero.
pub fn net_conn_sleep(c: &mut Cpu) {
    guest(c, |c| {
        let frame = match c.pc {
            NET_CONN_SLEEP_ENTRY => {
                c.cycles += 6;
                let frame = Frame::open(c, NET_CONN_SLEEP_FRAME);
                frame.save(c, S0, NET_CONN_SLEEP_SAVED_S0);
                c.g[S0][0] = u64::from(NET_CONN_DELAY_MS);
                if c.g[A0][0] != 0 {
                    c.g[S0][0] = c.g[A0][0];
                }
                let wait = (c.g[S0][0] as i64) > 0;
                frame.save_ra(c, 0);
                if wait {
                    c.cycles += 4;
                    NET_TICK.call(c, 0x003f_41d8)?;
                    c.g[S0][0] = addu(c.g[V0][0], c.g[S0][0]);
                    net_conn_sleep_wait(c, false)?;
                }
                frame
            }
            NET_CONN_SLEEP_HEAD => {
                net_conn_sleep_wait(c, true)?;
                Frame {
                    size: NET_CONN_SLEEP_FRAME,
                }
            }
            pc => {
                c.jump_out(pc, NET_CONN_SLEEP_ENTRY, NET_CONN_SLEEP_END);
                return Ok(());
            }
        };
        c.cycles += 4;
        frame.restore(c, S0, NET_CONN_SLEEP_SAVED_S0);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

pub(crate) fn net_conn_sleep_wait(c: &mut Cpu, mut resumed: bool) -> crate::call::Flow {
    loop {
        if resumed {
            c.cycles += 2;
            NET_TICK.call(c, 0x003f_41e8)?;
        }
        c.cycles += 5;
        NET_TICK.call(c, 0x003f_41f0)?;
        c.g[V0][0] = u64::from(c.g[V0][0] < c.g[S0][0]);
        if c.g[V0][0] == 0 {
            return Ok(());
        }
        if back_edge_unwinds(c, NET_CONN_SLEEP_HEAD) {
            return Err(Unwound);
        }
        resumed = true;
    }
}

/// Forwards the networking timer callback to NetTick.
pub fn net_tick_wrapper(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(NET_TICK), 0x003f_41a8);
}

/// Shuts down the network connection and releases its RPC service.
pub fn net_conn_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, NET_RPC_SHORT_FRAME);
        c.g[V0][0] = net_conn_gp(c, NET_CONN_OPEN_GP).lw(c);
        frame.save_ra(c, NET_RPC_SHORT_SAVED_RA);
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            c.g[V0][0] = u64::MAX;
        } else {
            c.cycles += 23;
            NET_CONN_DISCONNECT.call(c, 0x003f_4130)?;
            net_conn_request(c, frame, NET_RPC_DESTROY, 0x003f_4148)?;
            c.g[V1][0] = RPC_CALL_FLAGS;
            c.g[A2][0] = net_conn_gp(c, NET_CONN_HANDLE_GP).lw(c);
            net_conn_no_result(c, frame, 0x003f_4178)?;
            net_conn_gp(c, NET_CONN_OPEN_GP).sw(c, 0);
            c.g[V0][0] = 0;
        }
        c.cycles += 3;
        frame.restore_ra(c, NET_RPC_SHORT_SAVED_RA);
        frame.close(c);
        Ok(())
    });
}

/// Requests that the current network connection be disconnected.
pub fn net_conn_disconnect(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, NET_RPC_SHORT_FRAME);
        frame.save_ra(c, NET_RPC_SHORT_SAVED_RA);
        net_conn_request(c, frame, NET_RPC_STOP, 0x003f_40d8)?;
        c.g[V1][0] = RPC_CALL_FLAGS;
        c.g[A2][0] = net_conn_gp(c, NET_CONN_HANDLE_GP).lw(c);
        net_conn_no_result(c, frame, 0x003f_4108)?;
        frame.restore_ra(c, NET_RPC_SHORT_SAVED_RA);
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    });
}

pub(crate) fn net_conn_request(
    c: &mut Cpu,
    frame: Frame,
    command: u64,
    return_pc: u32,
) -> crate::call::Flow {
    c.g[A1][0] = NET_RPC_SERVICE;
    c.g[A2][0] = command;
    c.g[A0][0] = sx(frame.at(c, NET_RPC_REQUEST));
    DIRTY_RPC_INIT_REQUEST.call(c, return_pc)
}

pub(crate) fn net_conn_no_result(c: &mut Cpu, frame: Frame, return_pc: u32) -> crate::call::Flow {
    c.sw(frame.at(c, RPC_HAS_REPLY), c.g[V1][0]);
    c.g[A0][0] = sx(frame.at(c, NET_RPC_REQUEST));
    c.sw(frame.at(c, RPC_REPLY_OUT), 0);
    c.g[A1][0] = RPC_CALL_FLAGS;
    for reg in [A3, T0, T1, T2, T3] {
        c.g[reg][0] = 0;
    }
    DIRTY_RPC_CALL.call(c, return_pc)
}

/// Returns the cached hexadecimal address of the network adapter.
pub fn net_conn_mac(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, NET_MAC_FRAME);
        frame.save(c, S2, NET_MAC_SAVED_S2);
        c.g[S2][0] = NET_MAC_CACHE_BASE;
        frame.save(c, S1, NET_MAC_SAVED_S1);
        let cached = Addr(r32(c, S2)).off(NET_MAC_CACHE_OFFSET);
        c.g[V0][0] = cached.lb(c);
        c.g[S1][0] = sx(cached.0);
        frame.save(c, S0, NET_MAC_SAVED_S0);
        frame.save_ra(c, NET_MAC_SAVED_RA);
        if c.g[V0][0] == 0 {
            c.cycles += 64;
            c.g[A0][0] = NET_STATUS_MAC;
            c.g[A1][0] = c.g[SP][0];
            c.g[A2][0] = NET_MAC_REPLY_SIZE;
            NET_CONN_STATUS.call(c, 0x003f_3fb0)?;
            net_conn_mac_format(c, frame);
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, NET_MAC_SAVED_S0);
        frame.restore(c, S1, NET_MAC_SAVED_S1);
        frame.restore(c, S2, NET_MAC_SAVED_S2);
        frame.restore_ra(c, NET_MAC_SAVED_RA);
        frame.close(c);
        Ok(())
    });
}

pub(crate) fn net_conn_mac_format(c: &mut Cpu, frame: Frame) {
    // The conversion leaves the low nibbles in temporaries and the high
    // nibbles in the argument registers after consulting the guest table.
    c.g[V1][0] = c.lbu(frame.at(c, 0));
    c.g[V0][0] = sx(NET_MAC_HEX_BASE);
    c.g[A0][0] = c.lbu(frame.at(c, 1));
    c.g[T1][0] = c.g[V1][0] & NET_MAC_NIBBLE_MASK;
    c.g[A1][0] = c.lbu(frame.at(c, 2));
    c.g[V1][0] = sx(r32(c, V1) >> 4);
    c.g[A2][0] = c.lbu(frame.at(c, 3));
    c.g[V1][0] = addu(c.g[V1][0], c.g[V0][0]);
    c.g[T2][0] = c.g[A0][0] & NET_MAC_NIBBLE_MASK;
    c.g[A3][0] = c.lbu(frame.at(c, 4));
    c.g[T1][0] = addu(c.g[T1][0], c.g[V0][0]);
    c.g[T3][0] = c.g[A1][0] & NET_MAC_NIBBLE_MASK;
    c.g[S0][0] = c.lbu(r32(c, V1));
    c.g[A0][0] = sx(r32(c, A0) >> 4);
    c.g[T0][0] = c.lbu(frame.at(c, 5));
    c.g[A0][0] = addu(c.g[A0][0], c.g[V0][0]);
    c.g[T2][0] = addu(c.g[T2][0], c.g[V0][0]);
    c.g[T4][0] = c.g[A2][0] & NET_MAC_NIBBLE_MASK;
    c.g[V1][0] = c.lbu(r32(c, T1));
    c.g[A1][0] = sx(r32(c, A1) >> 4);
    c.g[T3][0] = addu(c.g[T3][0], c.g[V0][0]);
    c.g[A1][0] = addu(c.g[A1][0], c.g[V0][0]);
    c.g[T5][0] = c.g[A3][0] & NET_MAC_NIBBLE_MASK;
    c.g[T7][0] = c.lbu(r32(c, A0));
    c.g[A2][0] = sx(r32(c, A2) >> 4);
    c.g[T1][0] = c.lbu(r32(c, T2));
    c.g[A2][0] = addu(c.g[A2][0], c.g[V0][0]);
    c.g[T4][0] = addu(c.g[T4][0], c.g[V0][0]);
    c.g[T6][0] = c.g[T0][0] & NET_MAC_NIBBLE_MASK;
    c.g[A0][0] = c.lbu(r32(c, A1));
    c.g[A3][0] = sx(r32(c, A3) >> 4);
    c.g[T2][0] = c.lbu(r32(c, T3));
    c.g[A3][0] = addu(c.g[A3][0], c.g[V0][0]);
    c.g[T5][0] = addu(c.g[T5][0], c.g[V0][0]);
    c.g[A1][0] = c.lbu(r32(c, A2));
    c.g[T3][0] = c.lbu(r32(c, T4));
    c.g[T0][0] = sx(r32(c, T0) >> 4);
    c.g[T6][0] = addu(c.g[T6][0], c.g[V0][0]);
    c.g[T0][0] = addu(c.g[T0][0], c.g[V0][0]);
    c.g[A2][0] = c.lbu(r32(c, A3));
    c.g[V0][0] = NET_MAC_PREFIX;
    c.g[T4][0] = c.lbu(r32(c, T5));
    c.g[A3][0] = c.lbu(r32(c, T0));
    c.g[T5][0] = c.lbu(r32(c, T6));
    c.sb(Addr(r32(c, S2)).off(NET_MAC_CACHE_OFFSET).0, c.g[V0][0]);
    let digits = [S0, V1, T7, T1, A0, T2, A1, T3, A2, T4, A3, T5];
    let cached = Addr(r32(c, S1));
    for (index, reg) in digits.into_iter().enumerate() {
        c.sb(cached.off(index as u32 + 1).0, c.g[reg][0]);
    }
    cached.off(digits.len() as u32 + 1).sb(c, 0);
}

/// Gets the connection state or issues a DirtyRPC status query for the selector.
pub fn net_conn_status(c: &mut Cpu) {
    guest(c, |c| {
        let frame = match c.pc {
            NET_CONN_STATUS_ENTRY => {
                c.cycles += 12;
                let frame = Frame::open(c, NET_CONN_STATUS_FRAME);
                frame.save(c, S0, NET_CONN_STATUS_SAVED_S0);
                c.g[V0][0] = NET_STATUS_OPEN;
                frame.save(c, S2, NET_CONN_STATUS_SAVED_S2);
                c.g[S0][0] = c.g[A0][0];
                frame.save(c, S3, NET_CONN_STATUS_SAVED_S3);
                c.g[S2][0] = c.g[A1][0];
                frame.save(c, S1, NET_CONN_STATUS_SAVED_S1);
                c.g[S3][0] = c.g[A2][0];
                frame.save_ra(c, NET_CONN_STATUS_SAVED_RA);
                net_conn_status_query(c, frame)?;
                frame
            }
            NET_CONN_STATUS_BROADBAND_HEAD => {
                let frame = Frame {
                    size: NET_CONN_STATUS_FRAME,
                };
                net_conn_status_reply(c, frame);
                frame
            }
            pc => {
                c.jump_out(pc, NET_CONN_STATUS_ENTRY, NET_CONN_STATUS_END);
                return Ok(());
            }
        };
        c.cycles += 7;
        frame.restore(c, S0, NET_CONN_STATUS_SAVED_S0);
        frame.restore(c, S1, NET_CONN_STATUS_SAVED_S1);
        frame.restore(c, S2, NET_CONN_STATUS_SAVED_S2);
        frame.restore(c, S3, NET_CONN_STATUS_SAVED_S3);
        frame.restore_ra(c, NET_CONN_STATUS_SAVED_RA);
        frame.close(c);
        Ok(())
    });
}

pub(crate) fn net_conn_status_reply(c: &mut Cpu, frame: Frame) {
    c.cycles += 2;
    c.g[V0][0] = c.lw(frame.at(c, NET_RPC_REPLY_WORD));
}

pub(crate) fn net_conn_status_query(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    if c.g[S0][0] == c.g[V0][0] {
        c.cycles += 2;
        c.g[V0][0] = net_conn_gp(c, NET_CONN_OPEN_GP).lw(c);
        return Ok(());
    }

    c.cycles += 4;
    c.g[V0][0] = NET_STATUS_ONLINE;
    if c.g[S0][0] == c.g[V0][0] {
        c.cycles += 23;
        net_conn_request(c, frame, NET_RPC_STAT, 0x003f_3d20)?;
        net_conn_status_word_call(c, frame, NET_RPC_REPLY_WORD, NET_STATUS_HOST);
        dirty_rpc_clear_extra_args(c);
        DIRTY_RPC_CALL.call(c, 0x003f_3d58)?;
        c.g[V0][0] = c.lw(frame.at(c, NET_RPC_REPLY_WORD));
        c.g[V0][0] = u64::from(c.g[V0][0] != 0);
        return Ok(());
    }

    c.cycles += 3;
    c.g[V0][0] = NET_STATUS_CONNECTION;
    if c.g[S0][0] == c.g[V0][0] {
        c.cycles += 20;
        net_conn_request(c, frame, NET_RPC_STAT, 0x003f_3d88)?;
        c.g[V1][0] = sx(frame.at(c, NET_RPC_REPLY_WORD));
        c.g[V0][0] = RPC_CALL_FLAGS;
        c.g[A2][0] = net_conn_gp(c, NET_CONN_HANDLE_GP).lw(c);
        c.g[A3][0] = NET_STATUS_CONNECTION & SELECTOR_PREFIX_MASK;
        c.sw(frame.at(c, RPC_HAS_REPLY), c.g[V0][0]);
        c.g[T2][0] = c.g[S2][0];
        c.sw(frame.at(c, RPC_REPLY_OUT), c.g[V1][0]);
        c.g[T3][0] = c.g[S3][0];
        c.g[A0][0] = sx(frame.at(c, NET_RPC_REQUEST));
        c.g[A1][0] = RPC_CALL_FLAGS_OUTPUT;
        c.g[A3][0] = NET_STATUS_CONNECTION;
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        DIRTY_RPC_CALL.call(c, 0x003f_3dc0)?;
        net_conn_status_reply(c, frame);
        return Ok(());
    }

    c.cycles += 3;
    let host = c.g[S0][0] == NET_STATUS_HOST;
    c.g[V0][0] = NET_STATUS_ADDRESS & SELECTOR_PREFIX_MASK;
    if host {
        c.cycles += 16;
        net_conn_request(c, frame, NET_RPC_STAT, 0x003f_3dec)?;
        net_conn_status_word_call(c, frame, NET_RPC_STATUS_WORD, NET_STATUS_HOST);
        return net_conn_status_host_or_address(c, frame);
    }

    c.cycles += 3;
    c.g[V0][0] = NET_STATUS_ADDRESS;
    if c.g[S0][0] == c.g[V0][0] {
        c.cycles += 15;
        net_conn_request(c, frame, NET_RPC_STAT, 0x003f_3e38)?;
        net_conn_status_word_call(c, frame, NET_RPC_STATUS_WORD, NET_STATUS_ADDRESS);
        return net_conn_status_host_or_address(c, frame);
    }

    c.cycles += 3;
    c.g[V0][0] = NET_STATUS_MAC;
    let mac = c.g[S0][0] == c.g[V0][0];
    c.g[S1][0] = NET_STATUS_BROADBAND & SELECTOR_PREFIX_MASK;
    if mac {
        c.cycles += 21;
        net_conn_request(c, frame, NET_RPC_STAT, 0x003f_3e9c)?;
        c.g[V1][0] = RPC_CALL_FLAGS;
        c.g[A2][0] = net_conn_gp(c, NET_CONN_HANDLE_GP).lw(c);
        c.g[A3][0] = NET_STATUS_MAC & SELECTOR_PREFIX_MASK;
        c.sw(frame.at(c, RPC_HAS_REPLY), c.g[V1][0]);
        c.g[T2][0] = c.g[S2][0];
        c.g[T3][0] = c.g[S3][0];
        c.sw(frame.at(c, RPC_REPLY_OUT), 0);
        c.g[A0][0] = sx(frame.at(c, NET_RPC_REQUEST));
        c.g[A1][0] = RPC_CALL_FLAGS_OUTPUT;
        c.g[A3][0] = NET_STATUS_MAC;
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        DIRTY_RPC_CALL.call(c, 0x003f_3ed0)?;
        c.g[V0][0] = 0;
        return Ok(());
    }

    c.cycles += 3;
    c.g[S1][0] = NET_STATUS_BROADBAND;
    c.g[V0][0] = NET_STATUS_TYPE & SELECTOR_PREFIX_MASK;
    if c.g[S0][0] != c.g[S1][0] {
        c.cycles += 3;
        c.g[V0][0] = u64::MAX;
        if c.g[S0][0] != NET_STATUS_TYPE {
            return Ok(());
        }
    }
    c.cycles += 22;
    net_conn_request(c, frame, NET_RPC_STAT, 0x003f_3f08)?;
    c.g[V1][0] = RPC_CALL_FLAGS;
    c.g[V0][0] = sx(frame.at(c, NET_RPC_REPLY_WORD));
    c.g[A3][0] = NET_STATUS_TYPE & SELECTOR_PREFIX_MASK;
    c.g[A2][0] = net_conn_gp(c, NET_CONN_HANDLE_GP).lw(c);
    c.sw(frame.at(c, RPC_HAS_REPLY), c.g[V1][0]);
    c.g[T2][0] = c.g[S2][0];
    c.sw(frame.at(c, RPC_REPLY_OUT), c.g[V0][0]);
    c.g[T3][0] = c.g[S3][0];
    c.g[A0][0] = sx(frame.at(c, NET_RPC_REQUEST));
    c.g[A1][0] = RPC_CALL_FLAGS;
    c.g[A3][0] = NET_STATUS_TYPE;
    c.g[T0][0] = 0;
    c.g[T1][0] = 0;
    DIRTY_RPC_CALL.call(c, 0x003f_3f40)?;
    let broadband = c.g[S0][0] == c.g[S1][0];
    c.g[V0][0] = c.lw(frame.at(c, NET_RPC_REPLY_WORD));
    if broadband {
        c.cycles += 4;
        c.g[V0][0] = u64::from((r32(c, V0) >> NET_STATUS_BROADBAND_BIT) & 1);
        c.sw(frame.at(c, NET_RPC_REPLY_WORD), c.g[V0][0]);
        if back_edge_unwinds(c, NET_CONN_STATUS_BROADBAND_HEAD) {
            return Err(Unwound);
        }
        net_conn_status_reply(c, frame);
    }
    Ok(())
}

pub(crate) fn net_conn_status_word_call(
    c: &mut Cpu,
    frame: Frame,
    reply_offset: u32,
    selector: u64,
) {
    c.g[V1][0] = sx(frame.at(c, reply_offset));
    c.g[V0][0] = RPC_CALL_FLAGS;
    c.g[A2][0] = net_conn_gp(c, NET_CONN_HANDLE_GP).lw(c);
    c.g[A3][0] = selector & SELECTOR_PREFIX_MASK;
    c.sw(frame.at(c, RPC_HAS_REPLY), c.g[V0][0]);
    c.g[A0][0] = sx(frame.at(c, NET_RPC_REQUEST));
    c.sw(frame.at(c, RPC_REPLY_OUT), c.g[V1][0]);
    c.g[A1][0] = RPC_CALL_FLAGS;
    c.g[A3][0] = selector;
}

/// Zero the extra argument registers `DirtyRpc_Call` reads (t0-t3).
pub(crate) fn dirty_rpc_clear_extra_args(c: &mut Cpu) {
    for reg in [T0, T1, T2, T3] {
        c.g[reg][0] = 0;
    }
}

pub(crate) fn net_conn_status_host_or_address(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 7;
    dirty_rpc_clear_extra_args(c);
    DIRTY_RPC_CALL.call(c, 0x003f_3e70)?;
    c.g[V0][0] = c.lw(frame.at(c, NET_RPC_STATUS_WORD));
    Ok(())
}

/// Passes a connection string to the network RPC start command.
pub fn net_conn_connect(c: &mut Cpu) {
    const CONNECT_FRAME: u32 = 0x50;
    const CONNECT_SAVED_S0: u32 = 0x40;
    const CONNECT_SAVED_RA: u32 = 0x30;
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, CONNECT_FRAME);
        frame.save(c, S0, CONNECT_SAVED_S0);
        frame.save_ra(c, CONNECT_SAVED_RA);
        c.g[S0][0] = c.g[A0][0];
        net_conn_request(c, frame, NET_RPC_START, 0x003f_3c78)?;
        c.g[V1][0] = sx(frame.at(c, NET_RPC_REPLY_WORD));
        c.g[V0][0] = RPC_CALL_FLAGS;
        c.g[A2][0] = net_conn_gp(c, NET_CONN_HANDLE_GP).lw(c);
        c.g[T0][0] = c.g[S0][0];
        c.sw(frame.at(c, RPC_HAS_REPLY), c.g[V0][0]);
        c.g[A0][0] = sx(frame.at(c, NET_RPC_REQUEST));
        c.sw(frame.at(c, RPC_REPLY_OUT), c.g[V1][0]);
        c.g[A1][0] = RPC_CALL_FLAGS;
        c.g[A3][0] = 0;
        c.g[T1][0] = RPC_NUL_TERMINATED;
        c.g[T2][0] = 0;
        c.g[T3][0] = 0;
        DIRTY_RPC_CALL.call(c, 0x003f_3cac)?;
        c.g[V0][0] = c.lw(frame.at(c, NET_RPC_REPLY_WORD));
        frame.restore(c, S0, CONNECT_SAVED_S0);
        frame.restore_ra(c, CONNECT_SAVED_RA);
        frame.close(c);
        Ok(())
    });
}

/// Requests the network connection list into the caller's entry buffer.
pub fn net_conn_query(c: &mut Cpu) {
    const QUERY_FRAME: u32 = 0x70;
    const QUERY_SAVED_S0: u32 = 0x60;
    const QUERY_SAVED_S1: u32 = 0x50;
    const QUERY_SAVED_S2: u32 = 0x40;
    const QUERY_SAVED_RA: u32 = 0x30;
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, QUERY_FRAME);
        frame.save(c, S1, QUERY_SAVED_S1);
        frame.save(c, S2, QUERY_SAVED_S2);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, QUERY_SAVED_S0);
        c.g[S2][0] = c.g[A2][0];
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, QUERY_SAVED_RA);
        net_conn_request(c, frame, NET_RPC_LIST, 0x003f_3c00)?;
        c.g[V1][0] = sx(frame.at(c, NET_RPC_REPLY_WORD));
        c.g[V0][0] = RPC_CALL_FLAGS;
        c.g[A2][0] = net_conn_gp(c, NET_CONN_HANDLE_GP).lw(c);
        c.g[T0][0] = c.g[S0][0];
        c.g[T2][0] = c.g[S1][0];
        c.g[T3][0] = sx(r32(c, S2) << RPC_QUERY_ENTRY_SHIFT);
        c.sw(frame.at(c, RPC_HAS_REPLY), c.g[V0][0]);
        c.g[A0][0] = sx(frame.at(c, NET_RPC_REQUEST));
        c.sw(frame.at(c, RPC_REPLY_OUT), c.g[V1][0]);
        c.g[A1][0] = RPC_CALL_FLAGS_OUTPUT;
        c.g[A3][0] = 0;
        c.g[T1][0] = RPC_NUL_TERMINATED;
        DIRTY_RPC_CALL.call(c, 0x003f_3c34)?;
        c.g[V0][0] = c.lw(frame.at(c, NET_RPC_REPLY_WORD));
        frame.restore(c, S0, QUERY_SAVED_S0);
        frame.restore(c, S1, QUERY_SAVED_S1);
        frame.restore(c, S2, QUERY_SAVED_S2);
        frame.restore_ra(c, QUERY_SAVED_RA);
        frame.close(c);
        Ok(())
    });
}

/// Starts DirtyRPC if needed and opens the network service.
pub fn net_conn_startup(c: &mut Cpu) {
    const STARTUP_FRAME: u32 = 0x50;
    const STARTUP_SAVED_S0: u32 = 0x40;
    const STARTUP_SAVED_S1: u32 = 0x30;
    const STARTUP_SAVED_RA: u32 = 0x20;
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, STARTUP_FRAME);
        c.g[V0][0] = net_conn_gp(c, NET_CONN_OPEN_GP).lw(c);
        frame.save(c, S1, STARTUP_SAVED_S1);
        frame.save(c, S0, STARTUP_SAVED_S0);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, STARTUP_SAVED_RA);
        if c.g[V0][0] == 0 {
            c.cycles += 4;
            DIRTY_RPC_IS_STARTED.call(c, 0x003f_3b44)?;
            let started = (c.g[V0][0] as i64) > 0;
            c.g[A1][0] = NET_RPC_SERVICE & SELECTOR_PREFIX_MASK;
            if !started {
                c.cycles += 5;
                c.g[A0][0] = 0;
                DIRTY_RPC_STARTUP.call(c, 0x003f_3b54)?;
                DIRTY_RPC_INIT_IOP.call(c, 0x003f_3b5c)?;
                c.g[A1][0] = NET_RPC_SERVICE & SELECTOR_PREFIX_MASK;
            }
            c.cycles += 21;
            // The same register value is stored below as the call's flags and as the open state.
            c.g[S0][0] = RPC_CALL_FLAGS;
            net_conn_request(c, frame, NET_RPC_CREATE, 0x003f_3b78)?;
            c.g[V1][0] = sx(net_conn_gp(c, NET_CONN_HANDLE_GP).0);
            c.g[A3][0] = NET_RPC_TAG & SELECTOR_PREFIX_MASK;
            c.sw(frame.at(c, RPC_REPLY_OUT), c.g[V1][0]);
            c.g[T0][0] = c.g[S1][0];
            c.sw(frame.at(c, RPC_HAS_REPLY), c.g[S0][0]);
            c.g[A0][0] = sx(frame.at(c, NET_RPC_REQUEST));
            c.g[A1][0] = RPC_CALL_FLAGS;
            c.g[A2][0] = 0;
            c.g[A3][0] = NET_RPC_TAG;
            c.g[T1][0] = RPC_NUL_TERMINATED;
            c.g[T2][0] = 0;
            c.g[T3][0] = 0;
            DIRTY_RPC_CALL.call(c, 0x003f_3bac)?;
            c.sw(net_conn_gp(c, NET_CONN_OPEN_GP).0, c.g[S0][0]);
            c.g[V0][0] = 0;
        } else {
            c.cycles += 2;
            c.g[V0][0] = u64::MAX;
        }
        c.cycles += 5;
        frame.restore(c, S0, STARTUP_SAVED_S0);
        frame.restore(c, S1, STARTUP_SAVED_S1);
        frame.restore_ra(c, STARTUP_SAVED_RA);
        frame.close(c);
        Ok(())
    });
}

/// Looks up a hostname and releases the request after completion.
pub fn socket_lookup_sync(c: &mut Cpu) {
    const SYNC_FRAME: u32 = 0x30;
    const SYNC_SAVED_S0: u32 = 0x20;
    const SYNC_SAVED_S1: u32 = 0x10;
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, SYNC_FRAME);
        frame.save(c, S0, SYNC_SAVED_S0);
        frame.save(c, S1, SYNC_SAVED_S1);
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;
        SOCKET_LOOKUP_CREATE.call(c, 0x003f_3ad0)?;
        c.g[S0][0] = c.g[V0][0];
        let allocated = c.g[S0][0] != 0;
        c.g[V0][0] = c.g[S1][0];
        if allocated {
            c.cycles += 1;
            c.g[V0][0] = Addr(r32(c, S0)).off(LOOKUP_DONE_CALLBACK).lw(c);
            loop {
                c.cycles += 4;
                let poll = r32(c, V0);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, poll, 0x003f_3ae8)?;
                if c.g[V0][0] != 0 {
                    break;
                }
                c.g[V0][0] = Addr(r32(c, S0)).off(LOOKUP_DONE_CALLBACK).lw(c);
                if event_due(c) {
                    deliver_event_here(c, LOOKUP_SYNC_HEAD);
                }
            }
            c.cycles += 5;
            c.g[V0][0] = Addr(r32(c, S0)).off(LOOKUP_FREE_CALLBACK).lw(c);
            c.g[A0][0] = c.g[S0][0];
            let free = r32(c, V0);
            c.g[S1][0] = Addr(r32(c, S0)).off(LOOKUP_ADDRESS).lw(c);
            call_ptr(c, free, 0x003f_3b00)?;
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 5;
        frame.restore(c, S0, SYNC_SAVED_S0);
        frame.restore(c, S1, SYNC_SAVED_S1);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Starts an asynchronous hostname lookup.
pub fn socket_lookup(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(SOCKET_LOOKUP_CREATE), 0x003f_3aa8);
}

/// Allocates a hostname lookup and submits its name-service request.
pub fn socket_lookup_create(c: &mut Cpu) {
    const CREATE_FRAME: u32 = 0x60;
    const CREATE_SAVED_S0: u32 = 0x50;
    const CREATE_SAVED_S1: u32 = 0x40;
    const CREATE_SAVED_S2: u32 = 0x30;
    const CREATE_SAVED_S3: u32 = 0x20;
    const CREATE_SAVED_RA: u32 = 0x10;
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, CREATE_FRAME);
        frame.save(c, S3, CREATE_SAVED_S3);
        frame.save(c, S2, CREATE_SAVED_S2);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, CREATE_SAVED_S1);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, CREATE_SAVED_S0);
        frame.save_ra(c, CREATE_SAVED_RA);
        c.g[A0][0] = u64::from(LOOKUP_SIZE);
        NET_RPC_ALLOC.call(c, 0x003f_39f0)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        if c.g[S1][0] != 0 {
            c.cycles += 30;
            c.g[A1][0] = 0;
            c.g[A2][0] = u64::from(LOOKUP_SIZE);
            MEMSET.call(c, 0x003f_3a08)?;
            c.g[V1][0] = sx(LOOKUP_DONE_ADDR);
            c.g[S0][0] = sx(LOOKUP_RPC_CREATE_DESC);
            c.g[V0][0] = sx(LOOKUP_FREE_ADDR);
            c.g[A1][0] = LOOKUP_RPC_SERVICE & SELECTOR_PREFIX_MASK;
            c.g[A2][0] = LOOKUP_RPC_CREATE & SELECTOR_PREFIX_MASK;
            c.sw(Addr(r32(c, S1)).off(LOOKUP_DONE_CALLBACK).0, c.g[V1][0]);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = LOOKUP_RPC_SERVICE;
            c.g[A2][0] = LOOKUP_RPC_CREATE;
            c.sw(Addr(r32(c, S1)).off(LOOKUP_FREE_CALLBACK).0, c.g[V0][0]);
            DIRTY_RPC_INIT_REQUEST.call(c, 0x003f_3a40)?;
            c.g[V1][0] = RPC_CALL_FLAGS;
            c.g[V0][0] = sx(Addr(r32(c, S1)).off(LOOKUP_REMOTE_HANDLE).0);
            c.sw(frame.at(c, RPC_HAS_REPLY), c.g[V1][0]);
            c.g[A0][0] = c.g[S0][0];
            c.sw(frame.at(c, RPC_REPLY_OUT), c.g[V0][0]);
            c.g[A3][0] = c.g[S2][0];
            c.g[T0][0] = c.g[S3][0];
            c.g[A1][0] = RPC_CALL_FLAGS;
            c.g[A2][0] = 0;
            c.g[T1][0] = RPC_NUL_TERMINATED;
            c.g[T2][0] = 0;
            c.g[T3][0] = 0;
            DIRTY_RPC_CALL.call(c, 0x003f_3a74)?;
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, CREATE_SAVED_S0);
        frame.restore(c, S1, CREATE_SAVED_S1);
        frame.restore(c, S2, CREATE_SAVED_S2);
        frame.restore(c, S3, CREATE_SAVED_S3);
        frame.restore_ra(c, CREATE_SAVED_RA);
        frame.close(c);
        Ok(())
    });
}

/// Releases the remote lookup handle and frees the local request.
pub fn socket_lookup_free(c: &mut Cpu) {
    const FREE_FRAME: u32 = 0x50;
    const FREE_SAVED_S0: u32 = 0x40;
    const FREE_SAVED_S1: u32 = 0x30;
    const FREE_SAVED_S2: u32 = 0x20;
    const FREE_SAVED_RA: u32 = 0x10;
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, FREE_FRAME);
        c.g[A1][0] = LOOKUP_RPC_SERVICE & SELECTOR_PREFIX_MASK;
        frame.save(c, S0, FREE_SAVED_S0);
        c.g[A2][0] = LOOKUP_RPC_FREE & SELECTOR_PREFIX_MASK;
        frame.save(c, S2, FREE_SAVED_S2);
        c.g[S0][0] = DIRTY_RPC_STATE_HI;
        frame.save_ra(c, FREE_SAVED_RA);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, FREE_SAVED_S1);
        c.g[S0][0] = sx(LOOKUP_RPC_FREE_DESC);
        c.g[S1][0] = Addr(r32(c, S2)).off(LOOKUP_REMOTE_HANDLE).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = LOOKUP_RPC_SERVICE;
        c.g[A2][0] = LOOKUP_RPC_FREE;
        DIRTY_RPC_INIT_REQUEST.call(c, 0x003f_397c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        dirty_rpc_clear_extra_args(c);
        c.sw(frame.at(c, RPC_HAS_REPLY), 0);
        c.sw(frame.at(c, RPC_REPLY_OUT), 0);
        c.g[A1][0] = RPC_CALL_FLAGS;
        DIRTY_RPC_CALL.call(c, 0x003f_39a8)?;
        c.g[A0][0] = c.g[S2][0];
        NET_RPC_FREE.call(c, 0x003f_39b0)?;
        frame.restore(c, S0, FREE_SAVED_S0);
        frame.restore(c, S1, FREE_SAVED_S1);
        frame.restore(c, S2, FREE_SAVED_S2);
        frame.restore_ra(c, FREE_SAVED_RA);
        frame.close(c);
        Ok(())
    });
}

/// Polls the name service and reads the address of a successful lookup.
pub fn socket_lookup_done(c: &mut Cpu) {
    const DONE_FRAME: u32 = 0x60;
    const DONE_SAVED_S0: u32 = 0x50;
    const DONE_SAVED_S1: u32 = 0x40;
    const DONE_SAVED_S2: u32 = 0x30;
    const DONE_SAVED_S3: u32 = 0x20;
    const DONE_SAVED_RA: u32 = 0x10;
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, DONE_FRAME);
        c.g[V0][0] = DIRTY_RPC_STATE_HI;
        frame.save(c, S0, DONE_SAVED_S0);
        c.g[A1][0] = LOOKUP_RPC_SERVICE & SELECTOR_PREFIX_MASK;
        frame.save(c, S1, DONE_SAVED_S1);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S3, DONE_SAVED_S3);
        c.g[S1][0] = sx(LOOKUP_RPC_DONE_DESC);
        frame.save_ra(c, DONE_SAVED_RA);
        c.g[A2][0] = LOOKUP_RPC_DONE & SELECTOR_PREFIX_MASK;
        frame.save(c, S2, DONE_SAVED_S2);
        c.g[A0][0] = c.g[S1][0];
        c.g[S2][0] = Addr(r32(c, S0)).off(LOOKUP_REMOTE_HANDLE).lw(c);
        c.g[A1][0] = LOOKUP_RPC_SERVICE;
        c.g[A2][0] = LOOKUP_RPC_DONE;
        c.g[S3][0] = RPC_CALL_FLAGS;
        DIRTY_RPC_INIT_REQUEST.call(c, 0x003f_38a4)?;
        c.sw(frame.at(c, RPC_HAS_REPLY), c.g[S3][0]);
        c.g[A0][0] = c.g[S1][0];
        c.sw(frame.at(c, RPC_REPLY_OUT), c.g[S0][0]);
        c.g[A1][0] = RPC_CALL_FLAGS;
        c.g[A2][0] = c.g[S2][0];
        c.g[A3][0] = 0;
        dirty_rpc_clear_extra_args(c);
        DIRTY_RPC_CALL.call(c, 0x003f_38d0)?;
        c.g[V1][0] = Addr(r32(c, S0)).off(LOOKUP_STATUS).lw(c);
        c.g[A1][0] = LOOKUP_RPC_SERVICE & SELECTOR_PREFIX_MASK;
        if (c.g[V1][0] as i64) > 0 {
            c.cycles += 17;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = LOOKUP_RPC_SERVICE;
            c.g[A2][0] = LOOKUP_RPC_ADDRESS;
            DIRTY_RPC_INIT_REQUEST.call(c, 0x003f_38f0)?;
            c.g[V1][0] = sx(Addr(r32(c, S0)).off(LOOKUP_ADDRESS).0);
            c.sw(frame.at(c, RPC_HAS_REPLY), c.g[S3][0]);
            c.sw(frame.at(c, RPC_REPLY_OUT), c.g[V1][0]);
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S2][0];
            c.g[A1][0] = RPC_CALL_FLAGS;
            c.g[A3][0] = 0;
            dirty_rpc_clear_extra_args(c);
            DIRTY_RPC_CALL.call(c, 0x003f_3920)?;
        }
        c.cycles += 8;
        c.g[V0][0] = Addr(r32(c, S0)).off(LOOKUP_STATUS).lw(c);
        frame.restore(c, S0, DONE_SAVED_S0);
        frame.restore(c, S1, DONE_SAVED_S1);
        frame.restore(c, S2, DONE_SAVED_S2);
        frame.restore(c, S3, DONE_SAVED_S3);
        frame.restore_ra(c, DONE_SAVED_RA);
        frame.close(c);
        Ok(())
    });
}

/// Decodes a lobby string up to its terminator or the destination limit.
pub fn lobby_decrypt_string(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, CIPHER_STRING_FRAME);
        frame.save(c, S6, CIPHER_STRING_S6);
        frame.save(c, S3, CIPHER_STRING_S3);
        c.g[S6][0] = c.g[A1][0];
        frame.save(c, S4, CIPHER_STRING_S4);
        c.g[A1][0] = c.g[A3][0];
        c.g[S3][0] = c.g[A0][0];
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, S1, CIPHER_STRING_S1);
        c.g[A2][0] = c.g[T0][0];
        frame.save(c, S0, CIPHER_STRING_S0);
        c.g[A3][0] = c.g[T1][0];
        frame.save(c, S2, CIPHER_STRING_S2);
        c.g[A0][0] = c.g[SP][0];
        frame.save(c, S5, CIPHER_STRING_S5);
        c.g[S1][0] = 0;
        frame.save(c, S7, CIPHER_STRING_S7);
        frame.save_ra(c, CIPHER_STRING_RA);
        c.sb(frame.at(c, CIPHER_STRING_BYTE), 0);
        LOBBY_CIPHER_INIT.call(c, 0x003f_3798)?;

        c.g[V0][0] = sx(r32(c, S6).wrapping_sub(1));
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 3;
            c.g[S0][0] = c.lbu(r32(c, S4));
            let empty = c.g[S0][0] == 0;
            c.g[S1][0] = 1;
            if !empty {
                c.cycles += 4;
                c.g[S5][0] = c.g[V0][0];
                c.g[S2][0] = CIPHER_STRING_ALPHABET;
                c.g[S7][0] = CIPHER_STRING_TERMINATOR;
                loop {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx(frame.at(c, CIPHER_STRING_BYTE));
                    c.g[A2][0] = 1;
                    LOBBY_CIPHER_APPLY.call(c, 0x003f_37d0)?;
                    c.g[V1][0] = c.lbu(frame.at(c, CIPHER_STRING_BYTE));
                    c.cycles += 11;
                    c.divu(0, V1, S2);
                    c.g[V0][0] = c.hi[0];
                    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(CIPHER_STRING_BIAS));
                    c.g[V0][0] = sx(r32(c, S0).wrapping_sub(r32(c, V0)));
                    c.div(0, V0, S2);
                    c.g[V1][0] = c.hi[0];
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(CIPHER_FIRST_PRINTABLE));
                    c.g[V0][0] = c.g[V1][0] as u8 as i8 as i64 as u64;
                    let terminator = c.g[V0][0] == c.g[S7][0];
                    c.sb(r32(c, S3), c.g[V1][0]);
                    if terminator {
                        break;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = u64::from((c.g[S1][0] as i64) < (c.g[S5][0] as i64));
                    let room = c.g[V0][0] != 0;
                    c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
                    if !room {
                        break;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = addu(c.g[S4][0], c.g[S1][0]);
                    c.g[S0][0] = c.lbu(r32(c, V0));
                    let more = c.g[S0][0] != 0;
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, CIPHER_DECRYPT_HEAD);
                    }
                }
            }
            c.cycles += 1;
        }
        c.g[V0][0] = u64::from((c.g[S1][0] as i64) < (c.g[S6][0] as i64));
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.sb(r32(c, S3), 0);
        }
        lobby_cipher_string_restore(c, frame);
        Ok(())
    });
}

/// Encodes a lobby string, padding the fixed-width destination after the input ends.
pub fn lobby_encrypt_string(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 39;
        let frame = Frame::open(c, CIPHER_STRING_FRAME);
        frame.save(c, S0, CIPHER_STRING_S0);
        frame.save(c, S1, CIPHER_STRING_S1);
        c.g[S0][0] = CIPHER_PADDING_BANK;
        frame.save(c, S2, CIPHER_STRING_S2);
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, S3, CIPHER_STRING_S3);
        c.g[S2][0] = c.g[T0][0];
        frame.save(c, S4, CIPHER_STRING_S4);
        c.g[S3][0] = c.g[T1][0];
        frame.save(c, S5, CIPHER_STRING_S5);
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, S6, CIPHER_STRING_S6);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S7, CIPHER_STRING_S7);
        c.g[S6][0] = c.g[A0][0];
        c.g[A0][0] = c.g[SP][0];
        frame.save_ra(c, CIPHER_STRING_RA);
        c.sb(frame.at(c, CIPHER_STRING_STREAM_BYTE), 0);
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A3][0] = c.g[S3][0];
        c.g[S7][0] = c.g[S0][0];
        LOBBY_CIPHER_INIT.call(c, 0x003f_3630)?;
        c.g[S0][0] =
            sx(r32(c, S0).wrapping_sub((CIPHER_PADDING_BANK as u32) - CIPHER_PADDING_STATE));
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A3][0] = sx(0u32.wrapping_sub(r32(c, S3)));
        c.g[A0][0] = c.g[S0][0];
        LOBBY_CIPHER_INIT.call(c, 0x003f_3648)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx(CIPHER_PADDING_KEY);
        c.g[A2][0] = u64::MAX;
        c.g[A3][0] = u64::MAX;
        LOBBY_CIPHER_INIT.call(c, 0x003f_3660)?;
        c.g[V0][0] = u64::from((c.g[S5][0] as i64) < 2);
        c.g[S1][0] = CIPHER_STRING_TERMINATOR;
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            c.g[S0][0] = CIPHER_STRING_ALPHABET;
            lobby_encrypt_string_bytes(c, frame)?;
        }
        c.cycles += 2;
        if (c.g[S5][0] as i64) > 0 {
            c.sb(r32(c, S6), 0);
        }
        lobby_cipher_string_restore(c, frame);
        Ok(())
    });
}

pub(crate) fn lobby_encrypt_string_bytes(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    loop {
        c.cycles += 2;
        if c.g[S4][0] == 0 {
            c.cycles += 9;
            c.g[A0][0] =
                sx(r32(c, S7).wrapping_sub((CIPHER_PADDING_BANK as u32) - CIPHER_PADDING_STATE));
            c.g[A1][0] = sx(frame.at(c, CIPHER_STRING_BYTE));
            c.g[A2][0] = 1;
            LOBBY_CIPHER_APPLY.call(c, 0x003f_3688)?;
            c.g[V0][0] = c.lbu(frame.at(c, CIPHER_STRING_BYTE));
            c.g[V0][0] &= CIPHER_PADDING_MASK;
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(CIPHER_FIRST_PRINTABLE));
            c.sb(frame.at(c, CIPHER_STRING_BYTE), c.g[V0][0]);
        } else {
            c.g[V0][0] = c.lbu(r32(c, S4));
            c.cycles += 4;
            c.g[S4][0] = sx(r32(c, S4).wrapping_add(1));
            c.g[V1][0] = c.g[V0][0];
            c.sb(frame.at(c, CIPHER_STRING_BYTE), c.g[V0][0]);
            if c.g[V1][0] == 0 {
                c.g[S4][0] = 0;
            }
        }
        c.cycles += 5;
        c.g[V0][0] = c.lbu(frame.at(c, CIPHER_STRING_BYTE));
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(CIPHER_FIRST_PRINTABLE));
        c.g[V0][0] = u64::from(c.g[V0][0] < CIPHER_PRINTABLE_COUNT);
        if c.g[V0][0] == 0 {
            c.sb(frame.at(c, CIPHER_STRING_BYTE), c.g[S1][0]);
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = sx(frame.at(c, CIPHER_STRING_STREAM_BYTE));
        c.g[A2][0] = 1;
        c.g[S5][0] = sx(r32(c, S5).wrapping_sub(1));
        LOBBY_CIPHER_APPLY.call(c, 0x003f_36d4)?;
        c.g[V0][0] = c.lbu(frame.at(c, CIPHER_STRING_STREAM_BYTE));
        c.cycles += 12;
        c.g[A0][0] = c.lbu(frame.at(c, CIPHER_STRING_BYTE));
        c.divu(0, V0, S0);
        c.g[A1][0] = u64::from((c.g[S5][0] as i64) < 2);
        c.g[V1][0] = c.hi[0];
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(CIPHER_STRING_BIAS));
        c.g[A0][0] = addu(c.g[A0][0], c.g[V1][0]);
        c.div(0, A0, S0);
        c.g[V0][0] = c.hi[0];
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(CIPHER_FIRST_PRINTABLE));
        c.sb(r32(c, S6), c.g[V0][0]);
        let more = c.g[A1][0] == 0;
        c.g[S6][0] = sx(r32(c, S6).wrapping_add(1));
        if !more {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, CIPHER_ENCRYPT_HEAD);
        }
    }
}

pub(crate) fn lobby_cipher_string_restore(c: &mut Cpu, frame: Frame) {
    c.cycles += 12;
    frame.restore(c, S0, CIPHER_STRING_S0);
    c.g[V0][0] = 0;
    frame.restore(c, S1, CIPHER_STRING_S1);
    frame.restore(c, S2, CIPHER_STRING_S2);
    frame.restore(c, S3, CIPHER_STRING_S3);
    frame.restore(c, S4, CIPHER_STRING_S4);
    frame.restore(c, S5, CIPHER_STRING_S5);
    frame.restore(c, S6, CIPHER_STRING_S6);
    frame.restore(c, S7, CIPHER_STRING_S7);
    frame.restore_ra(c, CIPHER_STRING_RA);
    frame.close(c);
}

pub(crate) const MD5_STATE_A: u32 = 0x04;

pub(crate) const MD5_STATE_B: u32 = 0x08;

pub(crate) const MD5_STATE_C: u32 = 0x0c;

pub(crate) const MD5_STATE_D: u32 = 0x10;

pub(crate) const MD5_BLOCK: u32 = 0x14;

pub(crate) const MD5_BLOCK_BYTES: u32 = 64;

pub(crate) const MD5_WORD_BYTES: u32 = size_of::<u32>() as u32;

pub(crate) const MD5_WORDS: u32 = MD5_BLOCK_BYTES / MD5_WORD_BYTES;

pub(crate) const MD5_DECODE_HEAD: u32 = 0x003f_2258;

// The first 64 bytes of the frame hold the decoded block. The next 19
// words hold constants and one pre-addition that the EE compiler spilled.
pub(crate) const MD5_K30_SLOT: u32 = 0x40;

pub(crate) const MD5_SUM39_SLOT: u32 = 0x44;

pub(crate) const MD5_K39_SLOT: u32 = 0x48;

pub(crate) const MD5_K41_SLOT: u32 = 0x4c;

pub(crate) const MD5_K42_SLOT: u32 = 0x50;

pub(crate) const MD5_K44_SLOT: u32 = 0x54;

pub(crate) const MD5_K46_SLOT: u32 = 0x58;

pub(crate) const MD5_K47_SLOT: u32 = 0x5c;

pub(crate) const MD5_K49_SLOT: u32 = 0x60;

pub(crate) const MD5_K50_SLOT: u32 = 0x64;

pub(crate) const MD5_K52_SLOT: u32 = 0x68;

pub(crate) const MD5_K53_SLOT: u32 = 0x6c;

pub(crate) const MD5_K55_SLOT: u32 = 0x70;

pub(crate) const MD5_K56_SLOT: u32 = 0x74;

pub(crate) const MD5_K58_SLOT: u32 = 0x78;

pub(crate) const MD5_K59_SLOT: u32 = 0x7c;

pub(crate) const MD5_K61_SLOT: u32 = 0x80;

pub(crate) const MD5_K62_SLOT: u32 = 0x84;

pub(crate) const MD5_K64_SLOT: u32 = 0x88;

pub(crate) const MD5_SAVE_FP: u32 = 0x90;

pub(crate) const MD5_SAVE_S7: u32 = 0xa0;

pub(crate) const MD5_SAVE_S6: u32 = 0xb0;

pub(crate) const MD5_SAVE_S5: u32 = 0xc0;

pub(crate) const MD5_SAVE_S4: u32 = 0xd0;

pub(crate) const MD5_SAVE_S3: u32 = 0xe0;

pub(crate) const MD5_SAVE_S2: u32 = 0xf0;

pub(crate) const MD5_SAVE_S1: u32 = 0x100;

pub(crate) const MD5_SAVE_S0: u32 = 0x110;

pub(crate) const MD5_FRAME_BYTES: u32 = 0x120;

#[rustfmt::skip]
pub(crate) const MD5_CONSTANTS: [u32; 64] = [
    0xd76a_a478, 0xe8c7_b756, 0x2420_70db, 0xc1bd_ceee,
    0xf57c_0faf, 0x4787_c62a, 0xa830_4613, 0xfd46_9501,
    0x6980_98d8, 0x8b44_f7af, 0xffff_5bb1, 0x895c_d7be,
    0x6b90_1122, 0xfd98_7193, 0xa679_438e, 0x49b4_0821,
    0xf61e_2562, 0xc040_b340, 0x265e_5a51, 0xe9b6_c7aa,
    0xd62f_105d, 0x0244_1453, 0xd8a1_e681, 0xe7d3_fbc8,
    0x21e1_cde6, 0xc337_07d6, 0xf4d5_0d87, 0x455a_14ed,
    0xa9e3_e905, 0xfcef_a3f8, 0x676f_02d9, 0x8d2a_4c8a,
    0xfffa_3942, 0x8771_f681, 0x6d9d_6122, 0xfde5_380c,
    0xa4be_ea44, 0x4bde_cfa9, 0xf6bb_4b60, 0xbebf_bc70,
    0x289b_7ec6, 0xeaa1_27fa, 0xd4ef_3085, 0x0488_1d05,
    0xd9d4_d039, 0xe6db_99e5, 0x1fa2_7cf8, 0xc4ac_5665,
    0xf429_2244, 0x432a_ff97, 0xab94_23a7, 0xfc93_a039,
    0x655b_59c3, 0x8f0c_cc92, 0xffef_f47d, 0x8584_5dd1,
    0x6fa8_7e4f, 0xfe2c_e6e0, 0xa301_4314, 0x4e08_11a1,
    0xf753_7e82, 0xbd3a_f235, 0x2ad7_d2bb, 0xeb86_d391,
];

pub(crate) const MD5_ROTATIONS: [[u32; 4]; 4] = [
    [7, 12, 17, 22],
    [5, 9, 14, 20],
    [4, 11, 16, 23],
    [6, 10, 15, 21],
];

pub(crate) struct Md5State {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

impl Md5State {
    fn step(&mut self, step: usize, word: u32) {
        self.step_with_sum(step, word, self.a.wrapping_add(MD5_CONSTANTS[step - 1]));
    }

    // Each round cycles the roles of the same four digest words. Some EE
    // steps add their constant early and pass that sum through a spill slot.
    fn step_with_sum(&mut self, step: usize, word: u32, a_plus_constant: u32) {
        let round = (step - 1) / MD5_WORDS as usize;
        let mixed = match round {
            0 => self.d ^ ((self.c ^ self.d) & self.b),
            1 => self.c ^ ((self.b ^ self.c) & self.d),
            2 => self.b ^ self.c ^ self.d,
            _ => self.last_round_mix(),
        };
        let rotation = MD5_ROTATIONS[round][(step - 1) % 4];
        self.add_rotated(
            a_plus_constant.wrapping_add(mixed.wrapping_add(word)),
            rotation,
        );
    }

    fn last_round_mix(&self) -> u32 {
        self.c ^ (self.b | !self.d)
    }

    fn add_rotated(&mut self, sum: u32, rotation: u32) {
        let next = self.b.wrapping_add(sum.rotate_left(rotation));
        (self.a, self.b, self.c, self.d) = (self.d, next, self.b, self.c);
    }
}

pub(crate) fn md5_block_word(c: &mut Cpu, stack: Addr, index: u32) -> u32 {
    stack.off(index * MD5_WORD_BYTES).lw(c) as u32
}

// These splits preserve the compiler's two stores and intervening reload
// when it assembles a round constant out of its two halfwords.
pub(crate) fn md5_finish_spill(c: &mut Cpu, slot: Addr, constant: u32) {
    let high_half = slot.lw(c) as u32;
    slot.sw(c, u64::from(high_half | (constant & u16::MAX as u32)));
}

pub(crate) fn md5_spill_constant(c: &mut Cpu, stack: Addr, offset: u32, step: usize) {
    let slot = stack.off(offset);
    let constant = MD5_CONSTANTS[step - 1];
    slot.sw(c, u64::from(constant & !(u16::MAX as u32)));
    md5_finish_spill(c, slot, constant);
}

pub(crate) fn md5_word_during_spill(
    c: &mut Cpu,
    stack: Addr,
    index: u32,
    offset: u32,
    step: usize,
) -> u32 {
    let slot = stack.off(offset);
    let constant = MD5_CONSTANTS[step - 1];
    slot.sw(c, u64::from(constant & !(u16::MAX as u32)));
    let word = md5_block_word(c, stack, index);
    md5_finish_spill(c, slot, constant);
    word
}

/// Compress one buffered 64-byte block into the lobby's MD5 digest state.
pub fn crypt_md5_transform(c: &mut Cpu) {
    c.cycles += 14;
    let frame = Frame::open(c, MD5_FRAME_BYTES);
    frame.save(c, S0, MD5_SAVE_S0);
    let decode_end = frame.at(c, MD5_BLOCK_BYTES);
    set_word(c, A1, decode_end);
    frame.save(c, S7, MD5_SAVE_S7);
    c.g[S0][0] = c.g[A0][0];
    frame.save(c, S1, MD5_SAVE_S1);
    set_word(c, S7, MD5_WORDS);
    frame.save(c, S2, MD5_SAVE_S2);
    let block_end = r32(c, S0).wrapping_add(MD5_BLOCK + MD5_BLOCK_BYTES);
    set_word(c, T2, block_end);
    frame.save(c, S3, MD5_SAVE_S3);
    frame.save(c, S4, MD5_SAVE_S4);
    frame.save(c, S5, MD5_SAVE_S5);
    frame.save(c, S6, MD5_SAVE_S6);
    frame.save(c, FP, MD5_SAVE_FP);

    // Decode from high addresses to low. The back edge exposes the pointers,
    // counter and partial-byte registers to the guest event handler.
    loop {
        c.cycles += 18;
        let source = Addr(r32(c, T2).wrapping_sub(MD5_WORD_BYTES));
        let destination = Addr(r32(c, A1).wrapping_sub(MD5_WORD_BYTES));
        let byte3 = source.off(3).lbu(c) as u8;
        let byte2 = source.off(2).lbu(c) as u8;
        let byte1 = source.off(1).lbu(c) as u8;
        let byte0 = source.lbu(c) as u8;
        let word = u32::from_le_bytes([byte0, byte1, byte2, byte3]);
        let remaining = r32(c, S7).wrapping_sub(1);
        set_word(c, T2, source.0);
        set_word(c, A1, destination.0);
        set_word(c, S7, remaining);
        set_word(c, A0, u32::from(byte1));
        set_word(c, V1, u32::from(byte0));
        set_word(c, V0, word & !(u8::MAX as u32));
        set_word(c, FP, word);
        destination.sw(c, u64::from(word));
        if remaining == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, MD5_DECODE_HEAD);
        }
    }

    c.cycles += 915;
    let stack = Addr(sp(c));
    let context = Addr(r32(c, S0));
    let digest_c = context.off(MD5_STATE_C).lw(c) as u32;
    let digest_d = context.off(MD5_STATE_D).lw(c) as u32;
    let digest_b = context.off(MD5_STATE_B).lw(c) as u32;
    let digest_a = context.off(MD5_STATE_A).lw(c) as u32;
    let mut state = Md5State {
        a: digest_a,
        b: digest_b,
        c: digest_c,
        d: digest_d,
    };

    for index in 0..MD5_WORDS {
        state.step(index as usize + 1, md5_block_word(c, stack, index));
    }

    // Round 2 constructs the stack constants used by later rounds.
    state.step(17, md5_block_word(c, stack, 1));
    let word18 = md5_block_word(c, stack, 6);
    md5_spill_constant(c, stack, MD5_K30_SLOT, 30);
    state.step(18, word18);
    state.step(19, md5_block_word(c, stack, 11));
    state.step(20, md5_block_word(c, stack, 0));
    let word21 = md5_block_word(c, stack, 5);
    md5_spill_constant(c, stack, MD5_K39_SLOT, 39);
    state.step(21, word21);
    let word22 = md5_word_during_spill(c, stack, 10, MD5_K41_SLOT, 41);
    stack
        .off(MD5_K42_SLOT)
        .sw(c, u64::from(MD5_CONSTANTS[42 - 1]));
    state.step(22, word22);
    state.step(23, md5_word_during_spill(c, stack, 15, MD5_K44_SLOT, 44));
    md5_spill_constant(c, stack, MD5_K46_SLOT, 46);
    state.step(24, md5_word_during_spill(c, stack, 4, MD5_K47_SLOT, 47));
    md5_spill_constant(c, stack, MD5_K49_SLOT, 49);
    state.step(25, md5_word_during_spill(c, stack, 9, MD5_K50_SLOT, 50));
    md5_spill_constant(c, stack, MD5_K52_SLOT, 52);
    let word26 = md5_word_during_spill(c, stack, 14, MD5_K53_SLOT, 53);
    let k30 = stack.off(MD5_K30_SLOT).lw(c) as u32;
    state.step(26, word26);
    md5_spill_constant(c, stack, MD5_K55_SLOT, 55);
    state.step(27, md5_word_during_spill(c, stack, 3, MD5_K56_SLOT, 56));
    md5_spill_constant(c, stack, MD5_K58_SLOT, 58);
    state.step(28, md5_word_during_spill(c, stack, 8, MD5_K59_SLOT, 59));
    md5_spill_constant(c, stack, MD5_K61_SLOT, 61);
    state.step(29, md5_word_during_spill(c, stack, 13, MD5_K62_SLOT, 62));
    md5_spill_constant(c, stack, MD5_K64_SLOT, 64);
    let word30 = md5_block_word(c, stack, 2);
    state.step_with_sum(30, word30, state.a.wrapping_add(k30));
    state.step(31, md5_block_word(c, stack, 7));
    state.step(32, md5_block_word(c, stack, 12));

    // Round 3 reloads several constants a few steps before consuming them.
    state.step(33, md5_block_word(c, stack, 5));
    state.step(34, md5_block_word(c, stack, 8));
    let word35 = md5_block_word(c, stack, 11);
    let k39 = stack.off(MD5_K39_SLOT).lw(c) as u32;
    state.step(35, word35);
    stack
        .off(MD5_SUM39_SLOT)
        .sw(c, u64::from(state.b.wrapping_add(k39)));
    state.step(36, md5_block_word(c, stack, 14));
    let word37 = md5_block_word(c, stack, 1);
    let k41 = stack.off(MD5_K41_SLOT).lw(c) as u32;
    state.step(37, word37);
    let sum41 = state.b.wrapping_add(k41);
    let word38 = md5_block_word(c, stack, 4);
    let k42 = stack.off(MD5_K42_SLOT).lw(c) as u32;
    state.step(38, word38);
    let sum42 = state.b.wrapping_add(k42);
    let word39 = md5_block_word(c, stack, 7);
    let sum39 = stack.off(MD5_SUM39_SLOT).lw(c) as u32;
    state.step_with_sum(39, word39, sum39);
    let word40 = md5_block_word(c, stack, 10);
    let k44 = stack.off(MD5_K44_SLOT).lw(c) as u32;
    state.step(40, word40);
    let sum44 = state.b.wrapping_add(k44);
    state.step_with_sum(41, md5_block_word(c, stack, 13), sum41);
    let word42 = md5_block_word(c, stack, 0);
    let k46 = stack.off(MD5_K46_SLOT).lw(c) as u32;
    state.step_with_sum(42, word42, sum42);
    let sum46 = state.b.wrapping_add(k46);
    let word43 = md5_block_word(c, stack, 3);
    let k47 = stack.off(MD5_K47_SLOT).lw(c) as u32;
    state.step(43, word43);
    let sum47 = state.b.wrapping_add(k47);
    state.step_with_sum(44, md5_block_word(c, stack, 6), sum44);
    let sum48 = state.b.wrapping_add(MD5_CONSTANTS[48 - 1]);
    let word45 = md5_block_word(c, stack, 9);
    let k49 = stack.off(MD5_K49_SLOT).lw(c) as u32;
    state.step(45, word45);
    let sum49 = state.b.wrapping_add(k49);
    let word46 = md5_block_word(c, stack, 12);
    let k50 = stack.off(MD5_K50_SLOT).lw(c) as u32;
    state.step_with_sum(46, word46, sum46);
    let sum50 = state.b.wrapping_add(k50);
    let word47 = md5_block_word(c, stack, 15);
    let word48 = md5_block_word(c, stack, 2);
    state.step_with_sum(47, word47, sum47);
    let k52 = stack.off(MD5_K52_SLOT).lw(c) as u32;
    state.step_with_sum(48, word48, sum48);
    let sum52 = state.b.wrapping_add(k52);

    // Round 4 keeps the compiler's late pre-additions for the EE registers
    // left live at return.
    let word49 = md5_block_word(c, stack, 0);
    let k53 = stack.off(MD5_K53_SLOT).lw(c) as u32;
    state.step_with_sum(49, word49, sum49);
    let sum53 = state.b.wrapping_add(k53);
    state.step_with_sum(50, md5_block_word(c, stack, 7), sum50);
    let sum54 = state.b.wrapping_add(MD5_CONSTANTS[54 - 1]);
    let word51 = md5_block_word(c, stack, 14);
    let k55 = stack.off(MD5_K55_SLOT).lw(c) as u32;
    state.step(51, word51);
    let sum55 = state.b.wrapping_add(k55);
    let word52 = md5_block_word(c, stack, 5);
    let k56 = stack.off(MD5_K56_SLOT).lw(c) as u32;
    state.step_with_sum(52, word52, sum52);
    let sum56 = state.b.wrapping_add(k56);
    state.step_with_sum(53, md5_block_word(c, stack, 12), sum53);
    let sum57 = state.b.wrapping_add(MD5_CONSTANTS[57 - 1]);
    let word54 = md5_block_word(c, stack, 3);
    let k58 = stack.off(MD5_K58_SLOT).lw(c) as u32;
    state.step_with_sum(54, word54, sum54);
    let sum58 = state.b.wrapping_add(k58);
    let word55 = md5_block_word(c, stack, 10);
    let k59 = stack.off(MD5_K59_SLOT).lw(c) as u32;
    state.step_with_sum(55, word55, sum55);
    let sum59 = state.b.wrapping_add(k59);
    state.step_with_sum(56, md5_block_word(c, stack, 1), sum56);
    let sum60 = state.b.wrapping_add(MD5_CONSTANTS[60 - 1]);
    let word57 = md5_block_word(c, stack, 8);
    let k61 = stack.off(MD5_K61_SLOT).lw(c) as u32;
    state.step_with_sum(57, word57, sum57);
    let sum61 = state.b.wrapping_add(k61);
    let word58 = md5_block_word(c, stack, 15);
    let k62 = stack.off(MD5_K62_SLOT).lw(c) as u32;
    state.step_with_sum(58, word58, sum58);
    let sum62 = state.b.wrapping_add(k62);
    let word59 = md5_block_word(c, stack, 6);
    let word60 = md5_block_word(c, stack, 13);
    state.step_with_sum(59, word59, sum59);
    let sum63 = state.b.wrapping_add(MD5_CONSTANTS[63 - 1]);
    state.step_with_sum(60, word60, sum60);

    // The final four additions read and update each digest word at its own
    // step. A context address can overlap the frame, including saved GPRs.
    let word61 = md5_block_word(c, stack, 4);
    let k64 = stack.off(MD5_K64_SLOT).lw(c) as u32;
    let sum64 = state.b.wrapping_add(k64);
    let digest_a = context.off(MD5_STATE_A).lw(c) as u32;
    state.step_with_sum(61, word61, sum61);
    let word62 = md5_block_word(c, stack, 11);
    let mix62_and_word = state.last_round_mix().wrapping_add(word62);
    context
        .off(MD5_STATE_A)
        .sw(c, u64::from(digest_a.wrapping_add(state.b)));
    frame.restore(c, S1, MD5_SAVE_S1);
    frame.restore(c, S2, MD5_SAVE_S2);
    frame.restore(c, S3, MD5_SAVE_S3);

    let digest_d = context.off(MD5_STATE_D).lw(c) as u32;
    state.add_rotated(sum62.wrapping_add(mix62_and_word), MD5_ROTATIONS[3][1]);
    let round_d = state.b;
    frame.restore(c, S4, MD5_SAVE_S4);
    context
        .off(MD5_STATE_D)
        .sw(c, u64::from(digest_d.wrapping_add(round_d)));
    frame.restore(c, S5, MD5_SAVE_S5);

    let word63 = md5_block_word(c, stack, 2);
    frame.restore(c, S6, MD5_SAVE_S6);
    frame.restore(c, S7, MD5_SAVE_S7);
    let digest_c = context.off(MD5_STATE_C).lw(c) as u32;
    state.step_with_sum(63, word63, sum63);
    let round_c = state.b;
    context
        .off(MD5_STATE_C)
        .sw(c, u64::from(digest_c.wrapping_add(round_c)));

    let word64 = md5_block_word(c, stack, 9);
    let mix64_and_word = state.last_round_mix().wrapping_add(word64);
    let digest_b = context.off(MD5_STATE_B).lw(c) as u32;
    state.add_rotated(sum64.wrapping_add(mix64_and_word), MD5_ROTATIONS[3][3]);
    let updated_b = digest_b.wrapping_add(state.b);
    context.off(MD5_STATE_B).sw(c, u64::from(updated_b));
    frame.restore(c, S0, MD5_SAVE_S0);
    frame.restore(c, FP, MD5_SAVE_FP);

    set_word(c, V0, updated_b);
    set_word(c, V1, digest_b);
    set_word(c, A0, mix64_and_word);
    set_word(c, A1, digest_c);
    set_word(c, A2, mix62_and_word);
    set_word(c, A3, sum64);
    set_word(c, T0, sum61);
    set_word(c, T1, sum60);
    set_word(c, T2, sum62);
    set_word(c, T3, sum53);
    set_word(c, T4, sum48);
    set_word(c, T5, sum54);
    set_word(c, T6, sum57);
    set_word(c, T7, sum63);
    set_word(c, T8, round_c);
    set_word(c, T9, round_d);
    frame.close(c);
}

pub(crate) const LOBBY_CIPHER_BYTES: u32 = 256;

pub(crate) const LOBBY_CIPHER_MASK: u64 = (LOBBY_CIPHER_BYTES - 1) as u64;

pub(crate) const LOBBY_CIPHER_CURSOR: u32 = LOBBY_CIPHER_BYTES;

pub(crate) const LOBBY_CIPHER_ACCUMULATOR: u32 = 0x104;

pub(crate) const LOBBY_CIPHER_CRC_TABLE: u32 = 0x0049_6c08;

pub(crate) const LOBBY_CIPHER_STATIC_STATE: u32 = 0x0051_eef8;

pub(crate) const LOBBY_CIPHER_STATIC_KEY: u32 = 0x0049_7008;

pub(crate) const LOBBY_CIPHER_STATIC_ROUNDS: u64 = 10;

pub(crate) const LOBBY_CIPHER_INITIALIZED_GP: u32 = 0x1aac;

pub(crate) const LOBBY_CIPHER_APPLY_BYTE: u32 = 0x003f_3550;

pub(crate) const LOBBY_CIPHER_INIT_FILL: u32 = 0x003f_3418;

pub(crate) const LOBBY_CIPHER_INIT_KEY_LENGTH: u32 = 0x003f_3458;

pub(crate) const LOBBY_CIPHER_INIT_SHUFFLE: u32 = 0x003f_3498;

/// XOR a lobby buffer with the cipher stream and save the updated cursor and accumulator.
pub fn lobby_cipher_apply(c: &mut Cpu) {
    c.cycles += 6;
    c.g[T0][0] = c.g[A0][0];
    c.g[T2][0] = c.g[A1][0];
    c.g[T3][0] = c.g[A2][0];
    let state = Addr(r32(c, T0));
    c.g[T1][0] = state.off(LOBBY_CIPHER_CURSOR).lbu(c);
    c.g[A3][0] = state.off(LOBBY_CIPHER_ACCUMULATOR).lw(c);
    if c.g[T3][0] as i64 > 0 {
        c.cycles += 2;
        c.g[V0][0] = sx(LOBBY_CIPHER_CRC_TABLE & HIGH_HALF_MASK);
        c.g[T4][0] = sx(LOBBY_CIPHER_CRC_TABLE);
        loop {
            c.cycles += 29;
            c.g[V0][0] = c.g[T1][0];
            c.g[V1][0] = c.g[A3][0] & LOBBY_CIPHER_MASK;
            c.g[A0][0] = addu(c.g[T0][0], c.g[V0][0]);
            c.g[A3][0] = sx(r32(c, A3) >> 8);
            c.g[A1][0] = Addr(r32(c, A0)).lbu(c);
            c.g[V0][0] = addu(c.g[V0][0], 1);
            c.g[T1][0] = c.g[V0][0] & LOBBY_CIPHER_MASK;
            c.g[T3][0] = sx(r32(c, T3).wrapping_sub(1));
            c.g[V1][0] ^= c.g[A1][0];
            c.g[A2][0] = addu(c.g[T0][0], c.g[T1][0]);
            c.g[V1][0] = sx(r32(c, V1) << 2);
            c.g[A1][0] = Addr(r32(c, A2)).lbu(c);
            c.g[V1][0] = addu(c.g[V1][0], c.g[T4][0]);
            c.g[V0][0] = Addr(r32(c, V1)).lw(c);
            c.g[A3][0] ^= c.g[V0][0];
            c.g[A0][0] = c.g[A3][0] & LOBBY_CIPHER_MASK;
            c.g[A0][0] = addu(c.g[T0][0], c.g[A0][0]);
            c.g[V0][0] = Addr(r32(c, A0)).lbu(c);
            Addr(r32(c, A2)).sb(c, c.g[V0][0]);
            c.g[V1][0] = sx(r32(c, A1).wrapping_sub(r32(c, V0)));
            Addr(r32(c, A0)).sb(c, c.g[A1][0]);
            c.g[V1][0] &= LOBBY_CIPHER_MASK;
            c.g[V1][0] = addu(c.g[T0][0], c.g[V1][0]);
            c.g[V0][0] = Addr(r32(c, T2)).lbu(c);
            c.g[A0][0] = Addr(r32(c, V1)).lbu(c);
            c.g[V0][0] ^= c.g[A0][0];
            Addr(r32(c, T2)).sb(c, c.g[V0][0]);
            let more = c.g[T3][0] as i64 > 0;
            c.g[T2][0] = addu(c.g[T2][0], 1);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, LOBBY_CIPHER_APPLY_BYTE);
            }
        }
    }
    c.cycles += 3;
    let state = Addr(r32(c, T0));
    state.off(LOBBY_CIPHER_ACCUMULATOR).sw(c, c.g[A3][0]);
    state.off(LOBBY_CIPHER_CURSOR).sb(c, c.g[T1][0]);
    ret(c);
}

pub(crate) const CIPHER_INIT_FRAME: u32 = 0x50;

pub(crate) const CIPHER_INIT_SAVE_S0: u32 = 0x40;

pub(crate) const CIPHER_INIT_SAVE_S1: u32 = 0x30;

pub(crate) const CIPHER_INIT_SAVE_S2: u32 = 0x20;

pub(crate) const CIPHER_INIT_SAVE_S3: u32 = 0x10;

pub(crate) const CIPHER_INIT_SAVE_RA: u32 = 0;

/// Initialize or reshuffle a lobby cipher permutation from a key.
pub fn lobby_cipher_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, CIPHER_INIT_FRAME);
        c.g[V0][0] = Addr(r32(c, GP)).off(LOBBY_CIPHER_INITIALIZED_GP).lw(c);
        frame.save(c, S0, CIPHER_INIT_SAVE_S0);
        frame.save(c, S1, CIPHER_INIT_SAVE_S1);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, CIPHER_INIT_SAVE_S2);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, CIPHER_INIT_SAVE_S3);
        c.g[S2][0] = c.g[A3][0];
        frame.save_ra(c, CIPHER_INIT_SAVE_RA);
        let initialized = c.g[V0][0] != 0;
        c.g[S3][0] = c.g[A1][0];
        if !initialized {
            c.cycles += 9;
            c.g[V0][0] = 1;
            Addr(r32(c, GP))
                .off(LOBBY_CIPHER_INITIALIZED_GP)
                .sw(c, c.g[V0][0]);
            c.g[A0][0] = sx(LOBBY_CIPHER_STATIC_STATE);
            c.g[A1][0] = sx(LOBBY_CIPHER_STATIC_KEY);
            c.g[A2][0] = u64::MAX;
            c.g[A3][0] = LOBBY_CIPHER_STATIC_ROUNDS;
            callee::LOBBY_CIPHER_INIT.call(c, 0x003f_3404)?;
        }

        c.cycles += 2;
        let reset = c.g[S2][0] as i64 >= 0;
        c.g[T1][0] = LOBBY_CIPHER_MASK;
        if reset {
            c.cycles += 1;
            c.g[V0][0] = addu(c.g[S1][0], LOBBY_CIPHER_MASK);
            loop {
                c.cycles += 7;
                Addr(r32(c, V0)).sb(c, c.g[T1][0]);
                c.g[T1][0] = sx(r32(c, T1).wrapping_sub(1));
                let more = c.g[T1][0] as i64 >= 0;
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, LOBBY_CIPHER_INIT_FILL);
                }
            }
            c.cycles += 2;
            let state = Addr(r32(c, S1));
            state.off(LOBBY_CIPHER_CURSOR).sb(c, 0);
            state.off(LOBBY_CIPHER_ACCUMULATOR).sw(c, 0);
        } else {
            c.cycles += 2;
            c.g[S2][0] = sx(r32(c, S2).wrapping_neg());
        }

        c.cycles += 2;
        if (c.g[S0][0] as i64) < 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S3)).lbu(c);
            c.g[S0][0] = 0;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[S0][0] = 1;
                loop {
                    c.cycles += 7;
                    c.g[V0][0] = addu(c.g[S3][0], c.g[S0][0]);
                    c.g[V1][0] = Addr(r32(c, V0)).lbu(c);
                    if c.g[V1][0] == 0 {
                        break;
                    }
                    c.g[S0][0] = addu(c.g[S0][0], 1);
                    if event_due(c) {
                        deliver_event_here(c, LOBBY_CIPHER_INIT_KEY_LENGTH);
                    }
                }
            }
        }

        c.cycles += 2;
        if c.g[S0][0] as i64 > 0 {
            c.cycles += 4;
            c.g[T2][0] = sx(r32(c, S2).wrapping_mul(LOBBY_CIPHER_BYTES));
            c.g[A1][0] = 0;
            c.g[T1][0] = 0;
            if c.g[T2][0] as i64 > 0 {
                c.cycles += 3;
                c.g[V0][0] = sx(LOBBY_CIPHER_CRC_TABLE & HIGH_HALF_MASK);
                c.g[T4][0] = 0;
                c.g[T3][0] = sx(LOBBY_CIPHER_CRC_TABLE);
                loop {
                    c.cycles += 3;
                    c.div(0, T1, S0);
                    c.cycles += 28;
                    c.g[A0][0] = c.g[A1][0] & LOBBY_CIPHER_MASK;
                    c.g[A2][0] = c.g[T1][0] & LOBBY_CIPHER_MASK;
                    c.g[A2][0] = addu(c.g[S1][0], c.g[A2][0]);
                    c.g[A1][0] = sx(r32(c, A1) >> 8);
                    c.g[A3][0] = Addr(r32(c, A2)).lbu(c);
                    c.g[T1][0] = addu(c.g[T1][0], 1);
                    c.g[T0][0] = ((c.g[T1][0] as i64) < (c.g[T2][0] as i64)) as u64;
                    c.g[V1][0] = c.hi[0];
                    c.g[V1][0] = addu(c.g[S3][0], c.g[V1][0]);
                    c.g[V0][0] = Addr(r32(c, V1)).lbu(c);
                    c.g[A0][0] ^= c.g[V0][0];
                    c.g[A0][0] = sx(r32(c, A0) << 2);
                    c.g[A0][0] = addu(c.g[A0][0], c.g[T3][0]);
                    c.g[V1][0] = Addr(r32(c, A0)).lw(c);
                    c.g[A1][0] ^= c.g[V1][0];
                    c.g[V0][0] = c.g[A1][0] & LOBBY_CIPHER_MASK;
                    c.g[V0][0] ^= c.g[A3][0];
                    c.g[A1][0] = sx(r32(c, A1) >> 8);
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    c.g[V0][0] = addu(c.g[V0][0], c.g[T3][0]);
                    c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                    c.g[A1][0] ^= c.g[V1][0];
                    c.g[A0][0] = c.g[A1][0] & LOBBY_CIPHER_MASK;
                    c.g[A0][0] = addu(c.g[S1][0], c.g[A0][0]);
                    c.g[V0][0] = Addr(r32(c, A0)).lbu(c);
                    Addr(r32(c, A2)).sb(c, c.g[V0][0]);
                    let more = c.g[T0][0] != 0;
                    Addr(r32(c, A0)).sb(c, c.g[A3][0]);
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, LOBBY_CIPHER_INIT_SHUFFLE);
                    }
                }
            }
            c.cycles += 1;
        }
        frame.restore(c, S0, CIPHER_INIT_SAVE_S0);
        c.cycles += 6;
        frame.restore(c, S1, CIPHER_INIT_SAVE_S1);
        frame.restore(c, S2, CIPHER_INIT_SAVE_S2);
        frame.restore(c, S3, CIPHER_INIT_SAVE_S3);
        frame.restore_ra(c, CIPHER_INIT_SAVE_RA);
        frame.close(c);
        Ok(())
    });
}

pub(crate) const MD5_COUNT: u32 = 0;

pub(crate) const MD5_DIGEST_BYTES: u64 = 16;

pub(crate) const MD5_LENGTH_FIELD: u32 = MD5_BLOCK + MD5_BLOCK_BYTES - 8;

pub(crate) const MD5_LENGTH_INDEX: u64 = MD5_BLOCK_BYTES as u64 - 8;

pub(crate) const MD5_INITIAL_STATE: [u32; 4] = [0x6745_2301, 0xefcd_ab89, 0x98ba_dcfe, 0x1032_5476];

pub(crate) const MD5_HEX_TABLE_GP: u32 = 0x1aa8;

pub(crate) const MD5_FINAL_PAD: u32 = 0x003f_3250;

pub(crate) const MD5_FINAL_DIGEST: u32 = 0x003f_32d0;

pub(crate) const MD5_FINAL_CLEAR: u32 = 0x003f_3368;

pub(crate) const MD5_UPDATE_STRING: u32 = 0x003f_3168;

pub(crate) const MD5_UPDATE_COPY: u32 = 0x003f_31a0;

pub(crate) const MD5_FINAL_FRAME: u32 = 0x90;

pub(crate) const MD5_FINAL_SAVE_S0: u32 = 0x80;

pub(crate) const MD5_FINAL_SAVE_S1: u32 = 0x70;

pub(crate) const MD5_FINAL_SAVE_S2: u32 = 0x60;

pub(crate) const MD5_FINAL_SAVE_S3: u32 = 0x50;

pub(crate) const MD5_FINAL_SAVE_S4: u32 = 0x40;

pub(crate) const MD5_FINAL_SAVE_S5: u32 = 0x30;

pub(crate) const MD5_FINAL_SAVE_S6: u32 = 0x20;

pub(crate) const MD5_FINAL_SAVE_S7: u32 = 0x10;

pub(crate) const MD5_FINAL_SAVE_RA: u32 = 0;

/// Finalize the digest and write 16 binary bytes or a NUL-terminated hexadecimal string.
pub fn crypt_md5_final(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, MD5_FINAL_FRAME);
        frame.save(c, S0, MD5_FINAL_SAVE_S0);
        frame.save(c, S1, MD5_FINAL_SAVE_S1);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, MD5_FINAL_SAVE_S2);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, MD5_FINAL_SAVE_S3);
        c.g[S2][0] = 0x80;
        frame.save(c, S4, MD5_FINAL_SAVE_S4);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S5, MD5_FINAL_SAVE_S5);
        c.g[S4][0] = 0;
        frame.save(c, S6, MD5_FINAL_SAVE_S6);
        c.g[S5][0] = addu(c.g[S0][0], MD5_BLOCK as u64);
        frame.save(c, S7, MD5_FINAL_SAVE_S7);
        c.g[S6][0] = MD5_LENGTH_INDEX;
        frame.save_ra(c, MD5_FINAL_SAVE_RA);
        c.g[S7][0] = u64::from(MD5_BLOCK_BYTES);
        c.g[V0][0] = Addr(r32(c, S0)).off(MD5_COUNT).lw(c);
        c.g[A2][0] = c.g[V0][0] & (u64::from(MD5_BLOCK_BYTES) - 1);
        c.g[V0][0] = addu(c.g[S5][0], c.g[A2][0]);
        loop {
            c.cycles += 4;
            Addr(r32(c, V0)).sb(c, c.g[S2][0]);
            c.g[A2][0] = addu(c.g[A2][0], 1);
            let full = c.g[A2][0] == c.g[S7][0];
            c.g[S2][0] = 0;
            if full {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                callee::CRYPT_MD5_TRANSFORM.call(c, 0x003f_3268)?;
                c.g[A2][0] = 0;
            }
            c.cycles += 2;
            let padding_done = c.g[A2][0] == c.g[S6][0];
            c.g[V0][0] = addu(c.g[S5][0], c.g[A2][0]);
            if padding_done {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, MD5_FINAL_PAD);
            }
        }

        c.cycles += 23;
        let context = Addr(r32(c, S0));
        c.g[V0][0] = context.off(MD5_COUNT).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[V1][0] = context.off(MD5_COUNT).lbu(c);
        c.g[A3][0] = sx(r32(c, V0) >> 29);
        c.g[A2][0] = sx(r32(c, V0) >> 5);
        c.g[A1][0] = sx(r32(c, V0) >> 13);
        c.g[V1][0] = sx(r32(c, V1) << 3);
        c.g[V0][0] = sx(r32(c, V0) >> 21);
        let bit_length = context.off(MD5_LENGTH_FIELD);
        bit_length.off(5).sb(c, 0);
        bit_length.off(6).sb(c, 0);
        bit_length.off(7).sb(c, 0);
        bit_length.off(1).sb(c, c.g[A2][0]);
        bit_length.off(4).sb(c, c.g[A3][0]);
        bit_length.sb(c, c.g[V1][0]);
        bit_length.off(2).sb(c, c.g[A1][0]);
        bit_length.off(3).sb(c, c.g[V0][0]);
        callee::CRYPT_MD5_TRANSFORM.call(c, 0x003f_32b8)?;
        c.g[T1][0] = ((c.g[S3][0] as i64) < (MD5_DIGEST_BYTES * 2 + 1) as i64) as u64;
        c.g[T0][0] = addu(c.g[S0][0], MD5_STATE_A as u64);
        c.g[T2][0] = MD5_DIGEST_BYTES;
        c.g[A3][0] = c.g[T1][0];
        c.g[A2][0] = 0;

        loop {
            c.cycles += 3;
            c.g[V0][0] = c.g[A2][0] & 3;
            let first_byte = c.g[V0][0] == 0;
            c.g[V0][0] = sx((r32(c, A2) as i32 >> 2) as u32);
            if first_byte {
                c.cycles += 3;
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[V0][0] = addu(c.g[T0][0], c.g[V0][0]);
                c.g[S4][0] = Addr(r32(c, V0)).lw(c);
            }

            c.cycles += 2;
            if c.g[S3][0] == c.g[T2][0] {
                c.cycles += 2;
                Addr(r32(c, S1)).sb(c, c.g[S4][0]);
                c.cycles += 2;
                c.g[S1][0] = addu(c.g[S1][0], 1);
                c.g[A2][0] = addu(c.g[A2][0], 1);
            } else {
                c.cycles += 2;
                if c.g[A3][0] != 0 {
                    c.g[A2][0] = addu(c.g[A2][0], 1);
                } else {
                    c.cycles += 12;
                    c.g[A0][0] = Addr(r32(c, GP)).off(MD5_HEX_TABLE_GP).lw(c);
                    c.g[V0][0] = sx(r32(c, S4) >> 4);
                    c.g[V0][0] &= 0xf;
                    c.g[A1][0] = c.g[S4][0] & 0xf;
                    c.g[A0][0] = addu(c.g[A0][0], c.g[V0][0]);
                    c.g[V1][0] = Addr(r32(c, A0)).lbu(c);
                    Addr(r32(c, S1)).sb(c, c.g[V1][0]);
                    c.g[S1][0] = addu(c.g[S1][0], 1);
                    c.g[V0][0] = Addr(r32(c, GP)).off(MD5_HEX_TABLE_GP).lw(c);
                    c.g[V0][0] = addu(c.g[V0][0], c.g[A1][0]);
                    c.g[V1][0] = Addr(r32(c, V0)).lbu(c);
                    Addr(r32(c, S1)).sb(c, c.g[V1][0]);
                    c.cycles += 2;
                    c.g[S1][0] = addu(c.g[S1][0], 1);
                    c.g[A2][0] = addu(c.g[A2][0], 1);
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A2][0] as i64) < MD5_DIGEST_BYTES as i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S4][0] = sx(r32(c, S4) >> 8);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, MD5_FINAL_DIGEST);
            }
        }

        c.cycles += 2;
        let hex_output = c.g[T1][0] == 0;
        c.g[V0][0] = MD5_DIGEST_BYTES;
        if hex_output {
            c.cycles += 2;
            Addr(r32(c, S1)).sb(c, 0);
            c.cycles += 1;
            frame.restore(c, S0, MD5_FINAL_SAVE_S0);
            c.cycles += 1;
            frame.restore(c, S1, MD5_FINAL_SAVE_S1);
        } else {
            c.cycles += 2;
            let binary_output = c.g[S3][0] == c.g[V0][0];
            frame.restore(c, S0, MD5_FINAL_SAVE_S0);
            if binary_output {
                c.cycles += 1;
                frame.restore(c, S1, MD5_FINAL_SAVE_S1);
            } else {
                c.cycles += 2;
                if c.g[S3][0] as i64 <= 0 {
                    frame.restore(c, S1, MD5_FINAL_SAVE_S1);
                } else {
                    // The guest's short-buffer loop keeps the requested size in S3.
                    c.cycles += 1;
                    loop {
                        c.cycles += 7;
                        Addr(r32(c, S1)).sb(c, 0);
                        let more = c.g[S3][0] as i64 > 0;
                        c.g[S1][0] = addu(c.g[S1][0], 1);
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, MD5_FINAL_CLEAR);
                        }
                    }
                    c.cycles += 1;
                    frame.restore(c, S0, MD5_FINAL_SAVE_S0);
                    c.cycles += 1;
                    frame.restore(c, S1, MD5_FINAL_SAVE_S1);
                }
            }
        }
        c.cycles += 9;
        frame.restore(c, S2, MD5_FINAL_SAVE_S2);
        frame.restore(c, S3, MD5_FINAL_SAVE_S3);
        frame.restore(c, S4, MD5_FINAL_SAVE_S4);
        frame.restore(c, S5, MD5_FINAL_SAVE_S5);
        frame.restore(c, S6, MD5_FINAL_SAVE_S6);
        frame.restore(c, S7, MD5_FINAL_SAVE_S7);
        frame.restore_ra(c, MD5_FINAL_SAVE_RA);
        frame.close(c);
        Ok(())
    });
}

/// Feed bytes or a NUL-terminated string into the lobby's MD5 context.
pub fn crypt_md5_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S4, 0x10);
        let scan = (c.g[S0][0] as i64) < 0;
        frame.save_ra(c, 0);
        if scan {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S2)).lbu(c);
            c.g[S0][0] = 0;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[S0][0] = 1;
                loop {
                    c.cycles += 7;
                    c.g[V0][0] = addu(c.g[A1][0], c.g[S0][0]);
                    c.g[V1][0] = Addr(r32(c, V0)).lbu(c);
                    if c.g[V1][0] == 0 {
                        break;
                    }
                    c.g[S0][0] = addu(c.g[S0][0], 1);
                    if event_due(c) {
                        deliver_event_here(c, MD5_UPDATE_STRING);
                    }
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, S1)).off(MD5_COUNT).lw(c);
        c.g[A1][0] = c.g[V0][0] & (u64::from(MD5_BLOCK_BYTES) - 1);
        if c.g[S0][0] as i64 > 0 {
            c.cycles += 4;
            c.g[S3][0] = addu(c.g[S1][0], MD5_BLOCK as u64);
            c.g[S4][0] = u64::from(MD5_BLOCK_BYTES);
            c.g[A0][0] = Addr(r32(c, S2)).lbu(c);
            loop {
                c.cycles += 8;
                c.g[V1][0] = addu(c.g[S3][0], c.g[A1][0]);
                c.g[S2][0] = addu(c.g[S2][0], 1);
                c.g[A1][0] = addu(c.g[A1][0], 1);
                Addr(r32(c, V1)).sb(c, c.g[A0][0]);
                let count = Addr(r32(c, S1)).off(MD5_COUNT);
                c.g[V0][0] = count.lw(c);
                c.g[V0][0] = addu(c.g[V0][0], 1);
                let full = c.g[A1][0] == c.g[S4][0];
                count.sw(c, c.g[V0][0]);
                if full {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S1][0];
                    callee::CRYPT_MD5_TRANSFORM.call(c, 0x003f_31c8)?;
                    c.g[A1][0] = 0;
                }
                c.cycles += 3;
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(1));
                if c.g[S0][0] as i64 <= 0 {
                    break;
                }
                c.g[A0][0] = Addr(r32(c, S2)).lbu(c);
                if event_due(c) {
                    deliver_event_here(c, MD5_UPDATE_COPY);
                }
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Seed the digest state and clear the byte count.
pub fn crypt_md5_init(c: &mut Cpu) {
    c.cycles += 14;
    c.g[V1][0] = sx(MD5_INITIAL_STATE[0]);
    c.g[A1][0] = sx(MD5_INITIAL_STATE[1]);
    c.g[A2][0] = sx(MD5_INITIAL_STATE[2]);
    c.g[V0][0] = sx(MD5_INITIAL_STATE[3]);
    let context = Addr(r32(c, A0));
    context.off(MD5_STATE_A).sw(c, c.g[V1][0]);
    context.off(MD5_STATE_D).sw(c, c.g[V0][0]);
    context.off(MD5_STATE_B).sw(c, c.g[A1][0]);
    context.off(MD5_STATE_C).sw(c, c.g[A2][0]);
    context.off(MD5_COUNT).sw(c, 0);
    ret(c);
}

pub(crate) const LOBBY_HASH_MINIMUM_GP: u32 = 0x19a8;

pub(crate) const LOBBY_HASH_LIMIT_GP: u32 = 0x19ac;

pub(crate) const LOBBY_HASH_START: u32 = 0x60;

pub(crate) const LOBBY_HASH_END: u32 = 0x64;

pub(crate) const LOBBY_HASH_NEXT_RANGE: u32 = 0x003e_fd88;

/// Hash the range description around each parsed span that falls within the game's permitted memory.
pub fn lobby_hash_memory_ranges(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S0, 0xc0);
        frame.save(c, S2, 0xa0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S3, 0x90);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S4, 0x80);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S1, 0xb0);
        c.g[A0][0] = c.g[SP][0];
        frame.save_ra(c, 0x70);
        c.g[S4][0] = c.g[A3][0];
        callee::CRYPT_MD5_INIT.call(c, 0x003e_fd64)?;
        c.g[S1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        callee::CRYPT_MD5_UPDATE.call(c, 0x003e_fd78)?;
        let has_ranges = c.g[S1][0] != 0;
        c.g[A1][0] = c.g[S1][0];
        if has_ranges {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            loop {
                c.cycles += 6;
                c.g[A1][0] = sx(frame.at(c, LOBBY_HASH_START));
                c.g[A2][0] = sx(frame.at(c, LOBBY_HASH_END));
                callee::LOBBY_PARSE_HEX_RANGE.call(c, 0x003e_fd94)?;
                c.g[A1][0] = Addr(frame.at(c, LOBBY_HASH_START)).lw(c);
                let missing_start = c.g[A1][0] == 0;
                c.g[S0][0] = c.g[V0][0];
                if missing_start {
                    break;
                }
                c.cycles += 3;
                c.g[A2][0] = Addr(frame.at(c, LOBBY_HASH_END)).lw(c);
                let missing_end = c.g[A2][0] == 0;
                c.g[V0][0] = Addr(r32(c, GP)).off(LOBBY_HASH_MINIMUM_GP).lw(c);
                if missing_end {
                    break;
                }
                c.cycles += 3;
                c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
                let below_minimum = c.g[V0][0] != 0;
                c.g[V0][0] = Addr(r32(c, GP)).off(LOBBY_HASH_LIMIT_GP).lw(c);
                if !below_minimum {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[A2][0] < c.g[V0][0]) as u64;
                    let below_limit = c.g[V0][0] != 0;
                    c.g[V0][0] = (c.g[A1][0] < c.g[A2][0]) as u64;
                    if below_limit {
                        c.cycles += 2;
                        let ascending = c.g[V0][0] != 0;
                        c.g[A0][0] = c.g[SP][0];
                        if ascending {
                            c.cycles += 2;
                            c.g[A2][0] = sx(r32(c, A2).wrapping_sub(r32(c, A1)));
                            callee::CRYPT_MD5_UPDATE.call(c, 0x003e_fdd4)?;
                        }
                    }
                }
                c.cycles += 2;
                let more = c.g[S0][0] != 0;
                c.g[A0][0] = c.g[S0][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, LOBBY_HASH_NEXT_RANGE);
                }
            }
            c.cycles += 1;
            c.g[A1][0] = c.g[S1][0];
        }
        c.cycles += 15;
        c.g[A2][0] = c.g[S2][0];
        c.g[A0][0] = c.g[SP][0];
        callee::CRYPT_MD5_UPDATE.call(c, 0x003e_fdec)?;
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = c.g[S4][0];
        c.g[A0][0] = c.g[SP][0];
        callee::CRYPT_MD5_FINAL.call(c, 0x003e_fdfc)?;
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S1, 0xb0);
        frame.restore(c, S2, 0xa0);
        frame.restore(c, S3, 0x90);
        frame.restore(c, S4, 0x80);
        frame.restore_ra(c, 0x70);
        frame.close(c);
        Ok(())
    });
}

pub(crate) const LOBBY_HEX_RANGE_SEPARATOR: u8 = b':';

pub(crate) const LOBBY_HEX_LIST_SEPARATOR: u8 = b',';

pub(crate) const LOBBY_HEX_NEXT_BYTE: u32 = 0x003e_fc70;

/// Decode a hexadecimal start and end and return the next range's text position.
pub fn lobby_parse_hex_range(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        callee::LOBBY_PARSE_HEX.call(c, 0x003e_fcec)?;
        c.g[V1][0] = Addr(r32(c, V0)).lb(c);
        c.g[A0][0] = LOBBY_HEX_RANGE_SEPARATOR as u64;
        let has_end = c.g[V1][0] == c.g[A0][0];
        c.g[A1][0] = addu(c.g[V0][0], 1);
        if has_end {
            c.cycles += 8;
            c.g[A0][0] = c.g[A1][0];
            c.g[A1][0] = c.g[S0][0];
            callee::LOBBY_PARSE_HEX.call(c, 0x003e_fd08)?;
            c.g[V1][0] = Addr(r32(c, V0)).lb(c);
            c.g[A0][0] = addu(c.g[V0][0], 1);
            c.g[V1][0] ^= LOBBY_HEX_LIST_SEPARATOR as u64;
            if c.g[V1][0] == 0 {
                c.g[V0][0] = c.g[A0][0];
            }
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Read hexadecimal digits into a word and return the first unread character.
pub fn lobby_parse_hex(c: &mut Cpu) {
    match c.pc {
        0x003e_fc68 => {
            c.cycles += 2;
            c.g[V1][0] = 0;
        }
        LOBBY_HEX_NEXT_BYTE => {
            c.cycles += 1;
            c.g[A0][0] = addu(c.g[A0][0], 1);
        }
        pc => {
            c.jump_out(pc, 0x003e_fc68, 0x003e_fcd8);
            return;
        }
    }
    loop {
        c.cycles += 5;
        let cursor = Addr(r32(c, A0));
        c.g[A2][0] = cursor.lbu(c);
        c.g[V0][0] = (sx(r32(c, A2).wrapping_sub(b'0' as u32)) < 10) as u64;
        if c.g[V0][0] == 0 {
            c.cycles += 4;
            c.g[V0][0] = (sx(r32(c, A2).wrapping_sub(b'a' as u32)) < 6) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 4;
                c.g[V0][0] = (sx(r32(c, A2).wrapping_sub(b'A' as u32)) < 6) as u64;
                if c.g[V0][0] == 0 {
                    Addr(r32(c, A1)).sw(c, c.g[V1][0]);
                    break;
                }
                c.cycles += 1;
            }
        }
        c.g[A2][0] = cursor.lb(c);
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A2][0] as i64) < LOBBY_HEX_RANGE_SEPARATOR as i64) as u64;
        let decimal = c.g[V0][0] != 0;
        c.g[V1][0] = sx(r32(c, V1) << 4);
        if !decimal {
            c.cycles += 3;
            c.g[V0][0] = addu(c.g[A2][0], 9);
            c.g[V0][0] = sx(r32(c, V0) << 24);
            c.g[A2][0] = sx((r32(c, V0) as i32 >> 24) as u32);
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[A2][0] & 0xf;
        c.g[V1][0] |= c.g[V0][0];
        if back_edge_unwinds(c, LOBBY_HEX_NEXT_BYTE) {
            return;
        }
        c.cycles += 1;
        c.g[A0][0] = addu(c.g[A0][0], 1);
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

pub(crate) const UTF8_HIGH_BIT: u64 = 0x80;

pub(crate) const UTF8_TWO_BYTE_PREFIX: u64 = 0xc0;

pub(crate) const UTF8_THREE_BYTE_PREFIX: u64 = 0xe0;

pub(crate) const UTF8_CONTINUATION_MASK: u64 = 0x3f;

pub(crate) const UTF8_CONTINUATION_BITS: u32 = 6;

pub(crate) const UTF8_TRANSLATION_END: u32 = 4;

pub(crate) const UTF8_TRANSLATION_BYTES: u32 = 8;

pub(crate) const UTF8_TRANSLATION_ROW: u64 = 12;

pub(crate) const UTF8_UNMAPPED_BYTE: u64 = 0xff;

pub(crate) const UCS2_BYTES: u64 = 2;

pub(crate) const UTF8_TERMINATOR_BYTES: u32 = 1;

pub(crate) const UTF8_MAX_UCS2_BYTES: u64 = 3;

pub(crate) const UTF8_TRANSLATE_TO_8_BIT_HEAD: u32 = 0x003e_fbf8;

pub(crate) const UTF8_ENCODE_FROM_8_BIT_HEAD: u32 = 0x003e_fb40;

pub(crate) const UTF8_DECODE_TO_UCS2_HEAD: u32 = 0x003e_fa90;

pub(crate) const UTF8_ENCODE_FROM_UCS2_HEAD: u32 = 0x003e_f9f8;

pub(crate) const UTF8_STR_LEN_HEAD: u32 = 0x003e_f970;

pub(crate) const UTF8_REPLACE_PREFIX_HEAD: u32 = 0x003e_f880;

pub(crate) const UTF8_REPLACE_SEQUENCE_HEAD: u32 = 0x003e_f8b8;

pub(crate) const UTF8_TRANSLATE_CODE_PT_HEAD: u32 = 0x003e_f788;

pub(crate) const UTF8_GET_SEQ_LEN_HEAD: u32 = 0x003e_f618;

/// Convert UTF-8 into the lobby's single-byte encoding, with an optional replacement byte.
pub fn utf8_translate_to_8_bit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x90);
        c.g[A3][0] = sx(r32(c, A3) << 24);
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x60);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S4, 0x40);
        c.g[S3][0] = sx(r32(c, A1).wrapping_sub(UTF8_TERMINATOR_BYTES));
        frame.save(c, S5, 0x30);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S6, 0x20);
        c.g[S5][0] = c.g[T0][0];
        frame.save_ra(c, 0x10);
        c.g[V0][0] = Addr(r32(c, S2)).lb(c);
        let empty = c.g[V0][0] == 0;
        c.g[S6][0] = sx((r32(c, A3) as i32 >> 24) as u32);
        let mut need_terminator_pointer = true;
        if !empty {
            c.cycles += 2;
            let no_room = c.g[S3][0] as i64 <= 0;
            c.g[V1][0] = addu(c.g[S4][0], c.g[S0][0]);
            if no_room {
                need_terminator_pointer = false;
            } else {
                c.cycles += 1;
                c.g[A0][0] = c.g[SP][0];
                loop {
                    c.cycles += 13;
                    c.g[A1][0] = addu(c.g[S2][0], c.g[S1][0]);
                    callee::UTF8_DECODE_CODE_PT.call(c, 0x003e_fc00)?;
                    c.g[A2][0] = Addr(r32(c, SP)).lhu(c);
                    c.g[S1][0] = addu(c.g[S1][0], c.g[V0][0]);
                    c.g[A0][0] = addu(c.g[S4][0], c.g[S0][0]);
                    c.g[A1][0] = c.g[S5][0];
                    c.g[A3][0] = c.g[S6][0];
                    callee::UTF8_TRANSLATE_CODE_PT.call(c, 0x003e_fc18)?;
                    c.g[V1][0] = addu(c.g[S2][0], c.g[S1][0]);
                    c.g[S0][0] = addu(c.g[S0][0], c.g[V0][0]);
                    c.g[V0][0] = Addr(r32(c, V1)).lb(c);
                    let end = c.g[V0][0] == 0;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < c.g[S3][0] as i64) as u64;
                    if end {
                        break;
                    }
                    c.cycles += 2;
                    let room = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[SP][0];
                    if !room {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, UTF8_TRANSLATE_TO_8_BIT_HEAD);
                    }
                }
            }
        }
        if need_terminator_pointer {
            c.cycles += 1;
            c.g[V1][0] = addu(c.g[S4][0], c.g[S0][0]);
        }
        c.cycles += 12;
        c.g[V0][0] = addu(c.g[S0][0], 1);
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore_ra(c, 0x10);
        Addr(r32(c, V1)).sb(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Convert a single-byte string to UTF-8 through a byte-to-UCS-2 table.
pub fn utf8_encode_from_8_bit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x30);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x20);
        c.g[S2][0] = sx(r32(c, A1).wrapping_sub(UTF8_MAX_UCS2_BYTES as u32 - 1));
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S0)).lb(c);
        let empty = c.g[V0][0] == 0;
        c.g[S4][0] = c.g[A3][0];
        let mut need_terminator_pointer = true;
        if !empty {
            c.cycles += 2;
            let no_room = c.g[S2][0] as i64 <= 0;
            c.g[V1][0] = addu(c.g[S3][0], c.g[S1][0]);
            if no_room {
                need_terminator_pointer = false;
            } else {
                c.cycles += 2;
                c.g[V0][0] = Addr(r32(c, S0)).lbu(c);
                loop {
                    c.cycles += 9;
                    c.g[A0][0] = addu(c.g[S3][0], c.g[S1][0]);
                    c.g[S0][0] = addu(c.g[S0][0], 1);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_mul(UCS2_BYTES as u32));
                    c.g[V0][0] = addu(c.g[S4][0], c.g[V0][0]);
                    c.g[A1][0] = Addr(r32(c, V0)).lhu(c);
                    callee::UTF8_ENCODE_FROM_UCS2_CODE_PT.call(c, 0x003e_fb58)?;
                    c.g[V1][0] = Addr(r32(c, S0)).lb(c);
                    let end = c.g[V1][0] == 0;
                    c.g[S1][0] = addu(c.g[S1][0], c.g[V0][0]);
                    if end {
                        break;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < c.g[S2][0] as i64) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.g[V0][0] = Addr(r32(c, S0)).lbu(c);
                    if event_due(c) {
                        deliver_event_here(c, UTF8_ENCODE_FROM_8_BIT_HEAD);
                    }
                }
            }
        }
        if need_terminator_pointer {
            c.cycles += 1;
            c.g[V1][0] = addu(c.g[S3][0], c.g[S1][0]);
        }
        c.cycles += 10;
        c.g[V0][0] = addu(c.g[S1][0], 1);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        Addr(r32(c, V1)).sb(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Decode a UTF-8 string into a terminated UCS-2 buffer.
pub fn utf8_decode_to_ucs2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x40);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S4, 0x20);
        c.g[S3][0] = sx(r32(c, A1).wrapping_sub(1));
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S2)).lb(c);
        let empty = c.g[V0][0] == 0;
        c.g[S4][0] = c.g[A0][0];
        if !empty {
            c.cycles += 2;
            c.g[S5][0] = UTF8_MAX_UCS2_BYTES;
            if c.g[S3][0] as i64 > 0 {
                c.cycles += 1;
                c.g[A0][0] = sx(r32(c, S0).wrapping_mul(UCS2_BYTES as u32));
                loop {
                    c.cycles += 11;
                    c.g[A1][0] = addu(c.g[S2][0], c.g[S1][0]);
                    c.g[A0][0] = addu(c.g[S4][0], c.g[A0][0]);
                    callee::UTF8_DECODE_CODE_PT.call(c, 0x003e_fa9c)?;
                    c.g[S1][0] = addu(c.g[S1][0], c.g[V0][0]);
                    c.g[A0][0] = addu(c.g[S0][0], 1);
                    c.g[V0][0] = ((c.g[S5][0] as i64) < c.g[V0][0] as i64) as u64;
                    c.g[V1][0] = addu(c.g[S2][0], c.g[S1][0]);
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = c.g[A0][0];
                    }
                    c.g[V0][0] = Addr(r32(c, V1)).lb(c);
                    let end = c.g[V0][0] == 0;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < c.g[S3][0] as i64) as u64;
                    if end {
                        break;
                    }
                    c.cycles += 2;
                    let room = c.g[V0][0] != 0;
                    c.g[A0][0] = sx(r32(c, S0).wrapping_mul(UCS2_BYTES as u32));
                    if !room {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, UTF8_DECODE_TO_UCS2_HEAD);
                    }
                }
            }
        }
        c.cycles += 13;
        c.g[V1][0] = sx(r32(c, S0).wrapping_mul(UCS2_BYTES as u32));
        c.g[V0][0] = addu(c.g[S0][0], 1);
        c.g[V1][0] = addu(c.g[V1][0], c.g[S4][0]);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        Addr(r32(c, V1)).sh(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Encode a terminated UCS-2 string into a bounded UTF-8 buffer.
pub fn utf8_encode_from_ucs2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = sx(r32(c, A1).wrapping_sub(UTF8_MAX_UCS2_BYTES as u32 - 1));
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S0)).lhu(c);
        let empty = c.g[V0][0] == 0;
        c.g[S3][0] = c.g[A0][0];
        let mut need_terminator_pointer = true;
        if !empty {
            c.cycles += 2;
            let no_room = c.g[S2][0] as i64 <= 0;
            c.g[V1][0] = addu(c.g[S3][0], c.g[S1][0]);
            if no_room {
                need_terminator_pointer = false;
            } else {
                c.cycles += 2;
                c.g[A1][0] = Addr(r32(c, S0)).lhu(c);
                loop {
                    c.cycles += 6;
                    c.g[A0][0] = addu(c.g[S3][0], c.g[S1][0]);
                    c.g[S0][0] = addu(c.g[S0][0], UCS2_BYTES);
                    callee::UTF8_ENCODE_FROM_UCS2_CODE_PT.call(c, 0x003e_fa04)?;
                    c.g[V1][0] = Addr(r32(c, S0)).lhu(c);
                    let end = c.g[V1][0] == 0;
                    c.g[S1][0] = addu(c.g[S1][0], c.g[V0][0]);
                    if end {
                        break;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < c.g[S2][0] as i64) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.g[A1][0] = Addr(r32(c, S0)).lhu(c);
                    if event_due(c) {
                        deliver_event_here(c, UTF8_ENCODE_FROM_UCS2_HEAD);
                    }
                }
            }
        }
        if need_terminator_pointer {
            c.cycles += 1;
            c.g[V1][0] = addu(c.g[S3][0], c.g[S1][0]);
        }
        c.cycles += 9;
        c.g[V0][0] = addu(c.g[S1][0], 1);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        Addr(r32(c, V1)).sb(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Count sequences in a NUL-terminated UTF-8 string. The SDK's width helper counts one past each
/// multibyte sequence, so the byte after each one is skipped.
pub fn utf8_str_len(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S1)).lb(c);
        let empty = c.g[V0][0] == 0;
        c.g[S2][0] = 0;
        if !empty {
            c.cycles += 2;
            c.g[V1][0] = addu(c.g[S1][0], c.g[S0][0]);
            c.g[V0][0] = Addr(r32(c, V1)).lb(c);
            loop {
                c.cycles += 3;
                c.g[V0][0] &= UTF8_HIGH_BIT;
                if c.g[V0][0] == 0 {
                    c.g[S0][0] = addu(c.g[S0][0], 1);
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = Addr(r32(c, V1)).lbu(c);
                    callee::UTF8_GET_SEQ_LEN.call(c, 0x003e_f984)?;
                    c.g[S0][0] = addu(c.g[S0][0], c.g[V0][0]);
                }
                c.cycles += 4;
                c.g[V1][0] = addu(c.g[S1][0], c.g[S0][0]);
                c.g[V0][0] = Addr(r32(c, V1)).lb(c);
                let end = c.g[V0][0] == 0;
                c.g[S2][0] = addu(c.g[S2][0], 1);
                if end {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, UTF8_STR_LEN_HEAD);
                }
            }
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Replace non-ASCII sequences after the leading ASCII prefix. Like the length count it skips the
/// byte after each multibyte sequence, as the SDK does.
pub fn utf8_replace(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x70);
        c.g[A3][0] = sx(r32(c, A3) << 24);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x40);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x20);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S5, 0x10);
        c.g[S4][0] = sx((r32(c, A3) as i32 >> 24) as u32);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(r32(c, S3)).lb(c);
        c.g[V0][0] = c.g[V1][0] & UTF8_HIGH_BIT;
        let initial_ascii = c.g[V0][0] == 0;
        c.g[S5][0] = c.g[A1][0];

        let mut nonempty = true;
        if initial_ascii {
            c.cycles += 2;
            let empty = c.g[V1][0] == 0;
            c.g[A0][0] = c.g[S3][0];
            if empty {
                nonempty = false;
            } else {
                c.cycles += 2;
                c.g[A0][0] = addu(c.g[A0][0], 1);
                loop {
                    c.cycles += 4;
                    c.g[V1][0] = Addr(r32(c, A0)).lb(c);
                    c.g[V0][0] = c.g[V1][0] & UTF8_HIGH_BIT;
                    let non_ascii = c.g[V0][0] != 0;
                    c.g[S1][0] = addu(c.g[S1][0], 1);
                    if non_ascii {
                        break;
                    }
                    c.cycles += 2;
                    if c.g[V1][0] == 0 {
                        break;
                    }
                    c.g[A0][0] = addu(c.g[A0][0], 1);
                    if event_due(c) {
                        deliver_event_here(c, UTF8_REPLACE_PREFIX_HEAD);
                    }
                }
            }
        }

        if nonempty {
            c.cycles += 4;
            c.g[V1][0] = addu(c.g[S3][0], c.g[S1][0]);
            c.g[V0][0] = Addr(r32(c, V1)).lb(c);
            let end = c.g[V0][0] == 0;
            c.g[V0][0] = ((c.g[S1][0] as i64) < c.g[S5][0] as i64) as u64;
            if !end {
                c.cycles += 3;
                c.g[S0][0] = c.g[S1][0];
                let room = c.g[V0][0] != 0;
                c.g[V0][0] = addu(c.g[S2][0], c.g[S0][0]);
                if room {
                    c.cycles += 1;
                    c.g[V0][0] = Addr(r32(c, V1)).lb(c);
                    loop {
                        c.cycles += 3;
                        c.g[V0][0] &= UTF8_HIGH_BIT;
                        let ascii = c.g[V0][0] == 0;
                        c.g[A0][0] = Addr(r32(c, V1)).lbu(c);
                        let wrote_byte;
                        if ascii {
                            c.cycles += 3;
                            c.g[V0][0] = addu(c.g[S2][0], c.g[S0][0]);
                            c.g[S1][0] = addu(c.g[S1][0], 1);
                            Addr(r32(c, V0)).sb(c, c.g[A0][0]);
                            wrote_byte = true;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] &= 0xff;
                            callee::UTF8_GET_SEQ_LEN.call(c, 0x003e_f8cc)?;
                            wrote_byte = c.g[S4][0] != 0;
                            c.g[S1][0] = addu(c.g[S1][0], c.g[V0][0]);
                            if wrote_byte {
                                c.cycles += 3;
                                c.g[V0][0] = addu(c.g[S2][0], c.g[S0][0]);
                                Addr(r32(c, V0)).sb(c, c.g[S4][0]);
                            }
                        }
                        if wrote_byte {
                            c.cycles += 1;
                            c.g[S0][0] = addu(c.g[S0][0], 1);
                        }
                        c.cycles += 4;
                        c.g[V1][0] = addu(c.g[S3][0], c.g[S1][0]);
                        c.g[V0][0] = Addr(r32(c, V1)).lb(c);
                        let end = c.g[V0][0] == 0;
                        c.g[V0][0] = ((c.g[S0][0] as i64) < c.g[S5][0] as i64) as u64;
                        if end {
                            break;
                        }
                        c.cycles += 2;
                        if c.g[V0][0] == 0 {
                            break;
                        }
                        c.g[V0][0] = Addr(r32(c, V1)).lb(c);
                        if event_due(c) {
                            deliver_event_here(c, UTF8_REPLACE_SEQUENCE_HEAD);
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = addu(c.g[S2][0], c.g[S0][0]);
                }
                c.cycles += 2;
                Addr(r32(c, V0)).sb(c, 0);
                c.g[S0][0] = addu(c.g[S0][0], 1);
            }
        }
        c.cycles += 10;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x50);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Remove non-ASCII sequences using the replacement routine's zero-byte mode.
pub fn utf8_strip(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A3][0] = 0;
        callee::UTF8_REPLACE.call(c, 0x003e_f818)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Map one UCS-2 code point through the translation ranges.
pub fn utf8_translate_code_pt(c: &mut Cpu) {
    c.cycles += 6;
    c.g[A3][0] = sx(r32(c, A3) << 24);
    c.g[T2][0] = c.g[A0][0];
    c.g[A3][0] = sx((r32(c, A3) as i32 >> 24) as u32);
    c.g[A2][0] &= 0xffff;
    c.g[T0][0] = 0;
    c.g[T1][0] = UTF8_UNMAPPED_BYTE;
    loop {
        c.cycles += 3;
        c.g[V1][0] = Addr(r32(c, A1)).off(UTF8_TRANSLATION_END).lw(c);
        if c.g[V1][0] == 0 {
            break;
        }
        c.cycles += 4;
        c.g[V0][0] = Addr(r32(c, A1)).lw(c);
        c.g[V0][0] = (c.g[A2][0] < c.g[V0][0]) as u64;
        let below = c.g[V0][0] != 0;
        if !below {
            c.cycles += 3;
            c.g[V0][0] = (c.g[V1][0] < c.g[A2][0]) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 8;
                c.g[V0][0] = Addr(r32(c, A1)).lhu(c);
                c.g[V1][0] = Addr(r32(c, A1)).off(UTF8_TRANSLATION_BYTES).lw(c);
                c.g[V0][0] = sx(r32(c, A2).wrapping_sub(r32(c, V0)));
                c.g[A2][0] = c.g[V0][0] & 0xffff;
                c.g[V1][0] = addu(c.g[V1][0], c.g[A2][0]);
                c.g[V0][0] = Addr(r32(c, V1)).lbu(c);
                let mut byte = c.g[V0][0];
                if byte == c.g[T1][0] {
                    c.cycles += 2;
                    if c.g[A3][0] != 0 {
                        c.cycles += 1;
                        byte = c.g[A3][0];
                    }
                }
                Addr(r32(c, A0)).sb(c, byte);
                c.cycles += 2;
                c.g[A0][0] = addu(c.g[A0][0], 1);
                c.g[T0][0] = 1;
                break;
            }
        }
        c.g[A1][0] = addu(c.g[A1][0], UTF8_TRANSLATION_ROW);
        if event_due(c) {
            deliver_event_here(c, UTF8_TRANSLATE_CODE_PT_HEAD);
        }
    }
    c.cycles += 2;
    if c.g[T0][0] == 0 {
        c.cycles += 2;
        if c.g[A3][0] != 0 {
            c.cycles += 2;
            Addr(r32(c, A0)).sb(c, c.g[A3][0]);
            c.g[A0][0] = addu(c.g[A0][0], 1);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = sx(r32(c, A0).wrapping_sub(r32(c, T2)));
    ret(c);
}

/// Encode one UCS-2 code point and return the number of UTF-8 bytes written.
pub fn utf8_encode_from_ucs2_code_pt(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = c.g[A1][0] & 0xffff;
    c.g[V0][0] = (c.g[V1][0] < UTF8_HIGH_BIT) as u64;
    let ascii = c.g[V0][0] != 0;
    c.g[A2][0] = c.g[A0][0];
    let out = Addr(r32(c, A2));
    if ascii {
        c.cycles += 3;
        out.sb(c, c.g[V1][0]);
        c.g[A3][0] = 1;
    } else {
        c.cycles += 3;
        c.g[V0][0] = (c.g[V1][0] < 0x800) as u64;
        let two_bytes = c.g[V0][0] != 0;
        c.g[A3][0] = 2;
        if two_bytes {
            c.cycles += 7;
            c.g[V0][0] = c.g[V1][0] & UTF8_CONTINUATION_MASK;
            c.g[V1][0] = sx(r32(c, V1) >> UTF8_CONTINUATION_BITS);
            c.g[V0][0] |= UTF8_HIGH_BIT;
            c.g[V1][0] |= UTF8_TWO_BYTE_PREFIX;
            out.off(1).sb(c, c.g[V0][0]);
            out.sb(c, c.g[V1][0]);
        } else {
            c.cycles += 11;
            c.g[V0][0] = sx(r32(c, V1) >> UTF8_CONTINUATION_BITS);
            c.g[A0][0] = c.g[V1][0] & UTF8_CONTINUATION_MASK;
            c.g[V0][0] &= UTF8_CONTINUATION_MASK;
            c.g[V1][0] = sx(r32(c, V1) >> (UTF8_CONTINUATION_BITS * 2));
            c.g[V1][0] |= UTF8_THREE_BYTE_PREFIX;
            c.g[V0][0] |= UTF8_HIGH_BIT;
            c.g[A0][0] |= UTF8_HIGH_BIT;
            out.sb(c, c.g[V1][0]);
            out.off(2).sb(c, c.g[A0][0]);
            c.g[A3][0] = UTF8_MAX_UCS2_BYTES;
            out.off(1).sb(c, c.g[V0][0]);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A3][0];
    ret(c);
}

/// Decode one UTF-8 sequence into UCS-2, returning its input width.
pub fn utf8_decode_code_pt(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        c.g[A2][0] = c.g[A0][0];
        let frame = Frame::open(c, 0x10);
        let input = Addr(r32(c, A1));
        c.g[A0][0] = input.lbu(c);
        c.g[V0][0] = c.g[A0][0] & UTF8_HIGH_BIT;
        let ascii = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        if ascii {
            c.cycles += 4;
            c.g[V0][0] = c.g[A0][0] & 0xff;
            c.g[A3][0] = 1;
            Addr(r32(c, A2)).sh(c, c.g[V0][0]);
        } else {
            c.cycles += 4;
            c.g[V1][0] = c.g[A0][0] & 0xe0;
            c.g[V0][0] = UTF8_TWO_BYTE_PREFIX;
            let two_bytes = c.g[V1][0] == c.g[V0][0];
            c.g[V1][0] = c.g[A0][0] & 0xf0;
            if two_bytes {
                c.cycles += 9;
                c.g[V0][0] = input.lbu(c);
                c.g[A3][0] = 2;
                c.g[V1][0] = input.off(1).lbu(c);
                c.g[V0][0] &= 0x1f;
                c.g[V0][0] = sx(r32(c, V0) << UTF8_CONTINUATION_BITS);
                c.g[V1][0] &= UTF8_CONTINUATION_MASK;
                c.g[V0][0] |= c.g[V1][0];
                Addr(r32(c, A2)).sh(c, c.g[V0][0]);
            } else {
                c.cycles += 3;
                c.g[V0][0] = UTF8_THREE_BYTE_PREFIX;
                let three_bytes = c.g[V1][0] == c.g[V0][0];
                c.g[A3][0] = 3;
                if three_bytes {
                    c.cycles += 12;
                    c.g[V0][0] = input.lbu(c);
                    c.g[V1][0] = input.off(1).lbu(c);
                    c.g[A0][0] = input.off(2).lbu(c);
                    c.g[V0][0] &= 0x0f;
                    c.g[V1][0] &= UTF8_CONTINUATION_MASK;
                    c.g[V0][0] = sx(r32(c, V0) << (UTF8_CONTINUATION_BITS * 2));
                    c.g[V1][0] = sx(r32(c, V1) << UTF8_CONTINUATION_BITS);
                    c.g[A0][0] &= UTF8_CONTINUATION_MASK;
                    c.g[V0][0] |= c.g[V1][0];
                    c.g[V0][0] |= c.g[A0][0];
                    Addr(r32(c, A2)).sh(c, c.g[V0][0]);
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = input.lbu(c);
                    callee::UTF8_GET_SEQ_LEN.call(c, 0x003e_f6e0)?;
                    c.g[A3][0] = c.g[V0][0];
                }
            }
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        c.g[V0][0] = c.g[A3][0];
        frame.close(c);
        Ok(())
    });
}

/// Count a UTF-8 lead byte's sequence width as the SDK does: 1 for ASCII, otherwise one more than
/// the lead byte's leading one bits, at most 7, so a two-byte lead gives 3.
pub fn utf8_get_seq_len(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = c.g[A0][0] & 0xff;
    c.g[V0][0] = c.g[V1][0] & UTF8_HIGH_BIT;
    let ascii = c.g[V0][0] == 0;
    c.g[A0][0] = 1;
    if !ascii {
        c.cycles += 2;
        c.g[V1][0] = sx(r32(c, V1) << 1);
        loop {
            c.cycles += 4;
            c.g[A0][0] = addu(c.g[A0][0], 1);
            c.g[V0][0] = ((c.g[A0][0] as i64) < 7) as u64;
            let at_limit = c.g[V0][0] == 0;
            c.g[V1][0] &= 0xff;
            if at_limit {
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = c.g[V1][0] & UTF8_HIGH_BIT;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx(r32(c, V1) << 1);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, UTF8_GET_SEQ_LEN_HEAD);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

pub(crate) const LOBBY_SORT_FRAME: u32 = 0x1d00;

pub(crate) const LOBBY_SORT_STACK_COUNT_LIMIT: i64 = 512;

pub(crate) const LOBBY_SORT_STACK_ITEM_BYTES: i64 = 1024;

pub(crate) const LOBBY_SORT_INDEX_BYTES: u64 = 4;

pub(crate) const LOBBY_SORT_POINTERS: u32 = 0x800;

pub(crate) const LOBBY_SORT_POINTERS_ALT: u32 = LOBBY_SORT_POINTERS + LOBBY_SORT_INDEX_BYTES as u32;

pub(crate) const LOBBY_SORT_STACK_POINTERS: u32 = 0x810;

pub(crate) const LOBBY_SORT_STACK_POINTERS_ALT: u32 = 0x1010;

pub(crate) const LOBBY_SORT_STACK_ITEM: u32 = 0x1810;

pub(crate) const LOBBY_SORT_USER: u32 = 0x1c10;

pub(crate) const LOBBY_SORT_ITEMS: u32 = 0x1c14;

pub(crate) const LOBBY_SORT_COUNT: u32 = 0x1c18;

pub(crate) const LOBBY_SORT_COMPARE: u32 = 0x1c1c;

pub(crate) const LOBBY_SORT_RUN_LENGTHS: u32 = 0x1c20;

pub(crate) const LOBBY_SORT_ITEM_SCRATCH: u32 = 0x1c24;

pub(crate) const LOBBY_SORT_PASS: u32 = 0x1c28;

pub(crate) const LOBBY_SORT_STACK_SCRATCH: u32 = 0x1c2c;

pub(crate) const LOBBY_SORT_SAVE_RUN_SUM: u32 = 0x1c30;

pub(crate) const LOBBY_SORT_SAVE_NEXT_PAIR: u32 = 0x1c40;

pub(crate) const LOBBY_SORT_SAVE_POINTERS: u32 = 0x1c50;

pub(crate) const LOBBY_SORT_SAVE_RA: u32 = 0x1c60;

pub(crate) const LOBBY_SORT_SAVE_FP: u32 = 0x1c70;

pub(crate) const LOBBY_SORT_SAVE_S7: u32 = 0x1c80;

pub(crate) const LOBBY_SORT_SAVE_S6: u32 = 0x1c90;

pub(crate) const LOBBY_SORT_SAVE_S5: u32 = 0x1ca0;

pub(crate) const LOBBY_SORT_SAVE_S4: u32 = 0x1cb0;

pub(crate) const LOBBY_SORT_SAVE_S3: u32 = 0x1cc0;

pub(crate) const LOBBY_SORT_SAVE_S2: u32 = 0x1cd0;

pub(crate) const LOBBY_SORT_SAVE_S1: u32 = 0x1ce0;

pub(crate) const LOBBY_SORT_SAVE_S0: u32 = 0x1cf0;

pub(crate) const LOBBY_SORT_SCAN_RUN: u32 = 0x003e_f308;

pub(crate) const LOBBY_SORT_MERGE_PASS: u32 = 0x003e_f370;

pub(crate) const LOBBY_SORT_MERGE_PAIR: u32 = 0x003e_f3c8;

pub(crate) const LOBBY_SORT_PICK_ENTRY: u32 = 0x003e_f3e8;

pub(crate) const LOBBY_SORT_ADVANCE_RIGHT: u32 = 0x003e_f438;

pub(crate) const LOBBY_SORT_MOVE_ENTRY: u32 = 0x003e_f4d0;

pub(crate) const LOBBY_SORT_ROTATE_ENTRY: u32 = 0x003e_f518;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum LobbySortMergeStage {
    Pass,
    Pair,
    PickEntry,
    AdvanceRight,
}

/// Stably merge the lobby list's sorted runs, then move each record into its new slot.
pub fn lobby_merge_sort(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame {
            size: LOBBY_SORT_FRAME,
        };
        match c.pc {
            0x003e_f210 => lobby_sort_start(c),
            LOBBY_SORT_SCAN_RUN => lobby_sort_scan(c, frame),
            LOBBY_SORT_MERGE_PASS => lobby_sort_merge(c, frame, LobbySortMergeStage::Pass),
            LOBBY_SORT_MERGE_PAIR => lobby_sort_merge(c, frame, LobbySortMergeStage::Pair),
            LOBBY_SORT_PICK_ENTRY => lobby_sort_merge(c, frame, LobbySortMergeStage::PickEntry),
            LOBBY_SORT_ADVANCE_RIGHT => {
                lobby_sort_merge(c, frame, LobbySortMergeStage::AdvanceRight)
            }
            LOBBY_SORT_MOVE_ENTRY => lobby_sort_permute(c, frame, false),
            LOBBY_SORT_ROTATE_ENTRY => lobby_sort_permute(c, frame, true),
            pc => {
                c.jump_out(pc, 0x003e_f210, 0x003e_f5fc);
                Ok(())
            }
        }
    });
}

pub(crate) fn lobby_sort_start(c: &mut Cpu) -> Flow {
    c.cycles += 18;
    let frame = Frame::open(c, LOBBY_SORT_FRAME);
    Addr(frame.at(c, LOBBY_SORT_COUNT)).sw(c, c.g[A2][0]);
    frame.save(c, FP, LOBBY_SORT_SAVE_FP);
    frame.save(c, S0, LOBBY_SORT_SAVE_S0);
    c.g[FP][0] = c.g[A3][0];
    frame.save(c, S1, LOBBY_SORT_SAVE_S1);
    c.g[V0][0] = ((c.g[A2][0] as i64) < 2) as u64;
    frame.save(c, S2, LOBBY_SORT_SAVE_S2);
    frame.save(c, S3, LOBBY_SORT_SAVE_S3);
    frame.save(c, S4, LOBBY_SORT_SAVE_S4);
    frame.save(c, S5, LOBBY_SORT_SAVE_S5);
    frame.save(c, S6, LOBBY_SORT_SAVE_S6);
    frame.save(c, S7, LOBBY_SORT_SAVE_S7);
    frame.save_ra(c, LOBBY_SORT_SAVE_RA);
    Addr(frame.at(c, LOBBY_SORT_USER)).sw(c, c.g[A0][0]);
    Addr(frame.at(c, LOBBY_SORT_ITEMS)).sw(c, c.g[A1][0]);
    let trivial = c.g[V0][0] != 0;
    Addr(frame.at(c, LOBBY_SORT_COMPARE)).sw(c, c.g[T0][0]);
    if trivial {
        c.cycles += 1;
        frame.restore(c, S0, LOBBY_SORT_SAVE_S0);
        lobby_sort_return(c, frame);
        return Ok(());
    }

    c.cycles += 3;
    c.g[V0][0] = ((c.g[A2][0] as i64) < LOBBY_SORT_STACK_COUNT_LIMIT) as u64;
    let stack_runs = c.g[V0][0] != 0;
    c.g[V0][0] = sx(frame.at(c, LOBBY_SORT_STACK_POINTERS));
    if stack_runs {
        c.cycles += 5;
        c.g[V1][0] = sx(frame.at(c, LOBBY_SORT_STACK_POINTERS_ALT));
        Addr(frame.at(c, LOBBY_SORT_POINTERS)).sw(c, c.g[V0][0]);
        Addr(frame.at(c, LOBBY_SORT_RUN_LENGTHS)).sw(c, c.g[SP][0]);
        Addr(frame.at(c, LOBBY_SORT_POINTERS_ALT)).sw(c, c.g[V1][0]);
    } else {
        c.cycles += 11;
        c.g[V0][0] = Addr(frame.at(c, LOBBY_SORT_COUNT)).lw(c);
        c.g[S0][0] = sx(r32(c, V0).wrapping_mul(LOBBY_SORT_INDEX_BYTES as u32));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::LOBBY_MEM_ALLOC.addr, 0x003e_f288)?;
        Addr(frame.at(c, LOBBY_SORT_RUN_LENGTHS)).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::LOBBY_MEM_ALLOC.addr, 0x003e_f294)?;
        Addr(frame.at(c, LOBBY_SORT_POINTERS)).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::LOBBY_MEM_ALLOC.addr, 0x003e_f2a0)?;
        Addr(frame.at(c, LOBBY_SORT_POINTERS_ALT)).sw(c, c.g[V0][0]);
    }

    c.cycles += 3;
    c.g[V0][0] = ((c.g[FP][0] as i64) <= LOBBY_SORT_STACK_ITEM_BYTES) as u64;
    let stack_item = c.g[V0][0] != 0;
    c.g[V1][0] = sx(frame.at(c, LOBBY_SORT_STACK_ITEM));
    if stack_item {
        c.cycles += 3;
        Addr(frame.at(c, LOBBY_SORT_ITEM_SCRATCH)).sw(c, c.g[V1][0]);
        Addr(frame.at(c, LOBBY_SORT_STACK_SCRATCH)).sw(c, c.g[V1][0]);
    } else {
        c.cycles += 5;
        c.g[A0][0] = c.g[FP][0];
        call_ptr(c, callee::LOBBY_MEM_ALLOC.addr, 0x003e_f2c4)?;
        c.g[A0][0] = sx(frame.at(c, LOBBY_SORT_STACK_ITEM));
        Addr(frame.at(c, LOBBY_SORT_ITEM_SCRATCH)).sw(c, c.g[V0][0]);
        Addr(frame.at(c, LOBBY_SORT_STACK_SCRATCH)).sw(c, c.g[A0][0]);
    }

    c.cycles += 12;
    c.g[V0][0] = Addr(frame.at(c, LOBBY_SORT_ITEMS)).lw(c);
    c.g[V1][0] = Addr(frame.at(c, LOBBY_SORT_ITEMS)).lw(c);
    c.g[S3][0] = Addr(frame.at(c, LOBBY_SORT_POINTERS)).lw(c);
    c.lo[0] = c.g[V0][0];
    c.g[V0][0] = Addr(frame.at(c, LOBBY_SORT_COUNT)).lw(c);
    c.g[S0][0] = addu(c.g[V1][0], c.g[FP][0]);
    Addr(r32(c, S3)).sw(c, c.g[V1][0]);
    c.madd(0, S1, V0, FP);
    c.g[S6][0] = Addr(frame.at(c, LOBBY_SORT_RUN_LENGTHS)).lw(c);
    c.g[V0][0] = 1;
    let has_next_item = c.g[S0][0] != c.g[S1][0];
    Addr(r32(c, S6)).sw(c, c.g[V0][0]);
    if has_next_item {
        c.cycles += 2;
        c.g[S2][0] = 1;
        c.g[A1][0] = Addr(r32(c, S3)).lw(c);
        lobby_sort_scan(c, frame)
    } else {
        lobby_sort_after_scan(c, frame)
    }
}

pub(crate) fn lobby_sort_scan(c: &mut Cpu, frame: Frame) -> Flow {
    loop {
        c.cycles += 6;
        c.g[A2][0] = c.g[S0][0];
        c.g[T1][0] = Addr(frame.at(c, LOBBY_SORT_COMPARE)).lw(c);
        let compare = r32(c, T1);
        c.g[A0][0] = Addr(frame.at(c, LOBBY_SORT_USER)).lw(c);
        call_ptr(c, compare, 0x003e_f318)?;
        if c.g[V0][0] as i64 > 0 {
            c.g[S6][0] = addu(c.g[S6][0], LOBBY_SORT_INDEX_BYTES);
            c.cycles += 1;
            Addr(r32(c, S6)).sw(c, c.g[S2][0]);
        } else {
            c.cycles += 4;
            c.g[V0][0] = Addr(r32(c, S6)).lw(c);
            c.g[V0][0] = addu(c.g[V0][0], 1);
            Addr(r32(c, S6)).sw(c, c.g[V0][0]);
        }
        c.cycles += 5;
        c.g[S3][0] = addu(c.g[S3][0], LOBBY_SORT_INDEX_BYTES);
        Addr(r32(c, S3)).sw(c, c.g[S0][0]);
        c.g[S0][0] = addu(c.g[S0][0], c.g[FP][0]);
        if c.g[S0][0] == c.g[S1][0] {
            break;
        }
        c.g[A1][0] = Addr(r32(c, S3)).lw(c);
        if back_edge_unwinds(c, LOBBY_SORT_SCAN_RUN) {
            return Err(Unwound);
        }
    }
    lobby_sort_after_scan(c, frame)
}

pub(crate) fn lobby_sort_after_scan(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 7;
    c.g[S6][0] = addu(c.g[S6][0], LOBBY_SORT_INDEX_BYTES);
    Addr(r32(c, S6)).off(LOBBY_SORT_INDEX_BYTES as u32).sw(c, 0);
    Addr(r32(c, S6)).sw(c, 0);
    c.g[V1][0] = Addr(frame.at(c, LOBBY_SORT_RUN_LENGTHS)).lw(c);
    c.g[V0][0] = Addr(r32(c, V1)).off(LOBBY_SORT_INDEX_BYTES as u32).lw(c);
    let multiple_runs = c.g[V0][0] != 0;
    c.g[A3][0] = 0;
    if multiple_runs {
        c.cycles += 3;
        c.g[T0][0] = sx(frame.at(c, LOBBY_SORT_POINTERS));
        c.g[V0][0] = c.g[A3][0] & 1;
        lobby_sort_merge(c, frame, LobbySortMergeStage::Pass)
    } else {
        lobby_sort_begin_permutation(c, frame)
    }
}

pub(crate) fn lobby_sort_merge(c: &mut Cpu, frame: Frame, mut stage: LobbySortMergeStage) -> Flow {
    loop {
        let mut has_pair = true;
        if stage == LobbySortMergeStage::Pass {
            c.cycles += 21;
            c.g[A0][0] = Addr(frame.at(c, LOBBY_SORT_RUN_LENGTHS)).lw(c);
            c.g[V1][0] = sx(r32(c, V0) << 2);
            c.g[S6][0] = Addr(frame.at(c, LOBBY_SORT_RUN_LENGTHS)).lw(c);
            c.g[A1][0] = Addr(r32(c, A0)).lw(c);
            c.g[V1][0] = addu(c.g[T0][0], c.g[V1][0]);
            c.g[V0][0] ^= 1;
            c.g[S3][0] = Addr(r32(c, V1)).lw(c);
            c.g[A2][0] = Addr(r32(c, A0)).off(LOBBY_SORT_INDEX_BYTES as u32).lw(c);
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V1][0] = sx(r32(c, A1) << 2);
            c.g[V0][0] = addu(c.g[T0][0], c.g[V0][0]);
            c.g[S0][0] = addu(c.g[S3][0], c.g[V1][0]);
            c.g[A0][0] = sx(r32(c, A2) << 2);
            c.g[S1][0] = Addr(r32(c, V0)).lw(c);
            c.g[A3][0] = addu(c.g[A3][0], 1);
            c.g[S4][0] = addu(c.g[S0][0], c.g[A0][0]);
            Addr(frame.at(c, LOBBY_SORT_PASS)).sw(c, c.g[A3][0]);
            c.g[S5][0] = addu(c.g[S6][0], 2 * LOBBY_SORT_INDEX_BYTES);
            c.g[S2][0] = c.g[S3][0];
            has_pair = c.g[A1][0] != 0;
            c.g[S7][0] = c.g[S0][0];
            if has_pair {
                c.cycles += 1;
                stage = LobbySortMergeStage::Pair;
            }
        }

        while has_pair {
            lobby_sort_merge_pair(c, frame, stage)?;
            c.cycles += 13;
            c.g[S3][0] = c.g[S4][0];
            c.g[S6][0] = addu(c.g[S6][0], LOBBY_SORT_INDEX_BYTES);
            c.g[S2][0] = c.g[S3][0];
            c.g[A1][0] = Addr(r32(c, S5)).lw(c);
            c.g[S5][0] = c.g[A3][0];
            c.g[A2][0] = Addr(r32(c, S5)).lw(c);
            c.g[V0][0] = sx(r32(c, A1) << 2);
            c.g[S0][0] = addu(c.g[S3][0], c.g[V0][0]);
            c.g[S5][0] = addu(c.g[S5][0], LOBBY_SORT_INDEX_BYTES);
            c.g[V0][0] = sx(r32(c, A2) << 2);
            c.g[S7][0] = c.g[S0][0];
            has_pair = c.g[A1][0] != 0;
            c.g[S4][0] = addu(c.g[S0][0], c.g[V0][0]);
            if has_pair && back_edge_unwinds(c, LOBBY_SORT_MERGE_PAIR) {
                return Err(Unwound);
            }
            stage = LobbySortMergeStage::Pair;
        }

        c.cycles += 7;
        Addr(r32(c, S6)).off(LOBBY_SORT_INDEX_BYTES as u32).sw(c, 0);
        c.g[A3][0] = Addr(frame.at(c, LOBBY_SORT_PASS)).lw(c);
        Addr(r32(c, S6)).sw(c, 0);
        c.g[V1][0] = Addr(frame.at(c, LOBBY_SORT_RUN_LENGTHS)).lw(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(LOBBY_SORT_INDEX_BYTES as u32).lw(c);
        let more_passes = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[A3][0] & 1;
        if !more_passes {
            break;
        }
        if back_edge_unwinds(c, LOBBY_SORT_MERGE_PASS) {
            return Err(Unwound);
        }
        stage = LobbySortMergeStage::Pass;
    }
    lobby_sort_begin_permutation(c, frame)
}

pub(crate) fn lobby_sort_merge_pair(c: &mut Cpu, frame: Frame, stage: LobbySortMergeStage) -> Flow {
    let mut advance_right = stage == LobbySortMergeStage::AdvanceRight;
    if stage == LobbySortMergeStage::Pair {
        c.cycles += 3;
        c.g[V1][0] = addu(c.g[A1][0], c.g[A2][0]);
        let left_empty = c.g[S3][0] == c.g[S0][0];
        c.g[A3][0] = addu(c.g[S5][0], LOBBY_SORT_INDEX_BYTES);
        if left_empty {
            c.cycles += 2;
            if c.g[S3][0] == c.g[S4][0] {
                Addr(r32(c, S6)).sw(c, c.g[V1][0]);
                return Ok(());
            }
            c.cycles += 2;
            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
            advance_right = true;
        }
    }

    loop {
        if !advance_right {
            c.cycles += 2;
            let right_nonempty = c.g[S0][0] != c.g[S4][0];
            if right_nonempty {
                c.g[A1][0] = Addr(r32(c, S2)).lw(c);
                c.cycles += 11;
                c.g[A2][0] = Addr(r32(c, S0)).lw(c);
                c.g[A0][0] = Addr(frame.at(c, LOBBY_SORT_USER)).lw(c);
                c.g[T1][0] = Addr(frame.at(c, LOBBY_SORT_COMPARE)).lw(c);
                frame.save(c, V1, LOBBY_SORT_SAVE_RUN_SUM);
                frame.save(c, A3, LOBBY_SORT_SAVE_NEXT_PAIR);
                let compare = r32(c, T1);
                frame.save(c, T0, LOBBY_SORT_SAVE_POINTERS);
                call_ptr(c, compare, 0x003e_f418)?;
                frame.restore(c, V1, LOBBY_SORT_SAVE_RUN_SUM);
                frame.restore(c, A3, LOBBY_SORT_SAVE_NEXT_PAIR);
                advance_right = c.g[V0][0] as i64 > 0;
                frame.restore(c, T0, LOBBY_SORT_SAVE_POINTERS);
            }
            if advance_right {
                c.cycles += 1;
                c.g[V0][0] = Addr(r32(c, S0)).lw(c);
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, S2)).lw(c);
                c.g[S2][0] = addu(c.g[S2][0], LOBBY_SORT_INDEX_BYTES);
            }
        }
        if advance_right {
            c.cycles += 1;
            c.g[S0][0] = addu(c.g[S0][0], LOBBY_SORT_INDEX_BYTES);
        }

        c.cycles += 3;
        Addr(r32(c, S1)).sw(c, c.g[V0][0]);
        let left_nonempty = c.g[S2][0] != c.g[S7][0];
        c.g[S1][0] = addu(c.g[S1][0], LOBBY_SORT_INDEX_BYTES);
        if left_nonempty {
            advance_right = false;
            if back_edge_unwinds(c, LOBBY_SORT_PICK_ENTRY) {
                return Err(Unwound);
            }
        } else {
            c.cycles += 2;
            if c.g[S0][0] == c.g[S4][0] {
                break;
            }
            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
            advance_right = true;
            if back_edge_unwinds(c, LOBBY_SORT_ADVANCE_RIGHT) {
                return Err(Unwound);
            }
        }
    }
    c.cycles += 1;
    Addr(r32(c, S6)).sw(c, c.g[V1][0]);
    Ok(())
}

pub(crate) fn lobby_sort_begin_permutation(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 2;
    let already_ordered = c.g[A3][0] == 0;
    c.g[V0][0] = c.g[A3][0] & 1;
    if !already_ordered {
        c.cycles += 7;
        c.g[V1][0] = sx(frame.at(c, LOBBY_SORT_POINTERS));
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[A0][0] = Addr(frame.at(c, LOBBY_SORT_COUNT)).lw(c);
        c.g[V1][0] = addu(c.g[V1][0], c.g[V0][0]);
        c.g[S4][0] = 0;
        let has_items = c.g[A0][0] as i64 > 0;
        c.g[S3][0] = Addr(r32(c, V1)).lw(c);
        if has_items {
            c.cycles += 2;
            c.g[V0][0] = sx(r32(c, S4) << 2);
            return lobby_sort_permute(c, frame, false);
        }
    }
    lobby_sort_free(c, frame)
}

pub(crate) fn lobby_sort_permute(c: &mut Cpu, frame: Frame, mut resume_rotation: bool) -> Flow {
    loop {
        let mut has_entry = true;
        let mut rotate = resume_rotation;
        if !resume_rotation {
            c.cycles += 4;
            c.g[S0][0] = addu(c.g[V0][0], c.g[S3][0]);
            c.g[A1][0] = Addr(r32(c, S0)).lw(c);
            has_entry = c.g[A1][0] != 0;
            c.g[S5][0] = addu(c.g[S4][0], 1);
            if has_entry {
                c.cycles += 5;
                c.g[A0][0] = Addr(frame.at(c, LOBBY_SORT_ITEM_SCRATCH)).lw(c);
                c.g[A2][0] = c.g[FP][0];
                call_ptr(c, MEMCPY.addr, 0x003e_f4ec)?;

                c.cycles += 7;
                c.g[V1][0] = Addr(r32(c, S0)).lw(c);
                c.g[T1][0] = Addr(frame.at(c, LOBBY_SORT_ITEMS)).lw(c);
                c.g[V1][0] = sx(r32(c, V1).wrapping_sub(r32(c, T1)));
                c.div(0, V1, FP);
                c.g[S2][0] = c.lo[0];
                rotate = c.g[S2][0] != c.g[S4][0];
                c.g[S0][0] = c.g[S4][0];
                if rotate {
                    c.cycles += 2;
                    c.g[S0][0] = sx(r32(c, S0) << 2);
                }
            }
        }

        if has_entry {
            while rotate {
                c.cycles += 10;
                c.g[S1][0] = sx(r32(c, S2) << 2);
                c.g[S0][0] = addu(c.g[S0][0], c.g[S3][0]);
                c.g[S1][0] = addu(c.g[S1][0], c.g[S3][0]);
                c.g[A0][0] = Addr(r32(c, S0)).lw(c);
                c.g[A2][0] = c.g[FP][0];
                c.g[A1][0] = Addr(r32(c, S1)).lw(c);
                call_ptr(c, MEMCPY.addr, 0x003e_f534)?;
                Addr(r32(c, S0)).sw(c, 0);

                c.cycles += 8;
                c.g[S0][0] = c.g[S2][0];
                c.g[V0][0] = Addr(r32(c, S1)).lw(c);
                c.g[V1][0] = Addr(frame.at(c, LOBBY_SORT_ITEMS)).lw(c);
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, V1)));
                c.div(0, V0, FP);
                c.g[S2][0] = c.lo[0];
                rotate = c.g[S2][0] != c.g[S4][0];
                if rotate {
                    c.g[S0][0] = sx(r32(c, S0) << 2);
                    if back_edge_unwinds(c, LOBBY_SORT_ROTATE_ENTRY) {
                        return Err(Unwound);
                    }
                }
            }

            c.cycles += 7;
            c.g[S0][0] = sx(r32(c, S0) << 2);
            c.g[A1][0] = Addr(frame.at(c, LOBBY_SORT_ITEM_SCRATCH)).lw(c);
            c.g[S0][0] = addu(c.g[S0][0], c.g[S3][0]);
            c.g[A2][0] = c.g[FP][0];
            c.g[A0][0] = Addr(r32(c, S0)).lw(c);
            call_ptr(c, MEMCPY.addr, 0x003e_f578)?;
            Addr(r32(c, S0)).sw(c, 0);
        }

        c.cycles += 5;
        c.g[V1][0] = Addr(frame.at(c, LOBBY_SORT_COUNT)).lw(c);
        c.g[S4][0] = c.g[S5][0];
        c.g[V0][0] = ((c.g[S4][0] as i64) < c.g[V1][0] as i64) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V0][0] = sx(r32(c, S4) << 2);
        if !more {
            break;
        }
        if back_edge_unwinds(c, LOBBY_SORT_MOVE_ENTRY) {
            return Err(Unwound);
        }
        resume_rotation = false;
    }
    lobby_sort_free(c, frame)
}

pub(crate) fn lobby_sort_free(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 3;
    c.g[A0][0] = Addr(frame.at(c, LOBBY_SORT_RUN_LENGTHS)).lw(c);
    let heap_runs = c.g[A0][0] != c.g[SP][0];
    c.g[T1][0] = Addr(frame.at(c, LOBBY_SORT_ITEM_SCRATCH)).lw(c);
    if heap_runs {
        c.cycles += 7;
        call_ptr(c, callee::LOBBY_MEM_FREE.addr, 0x003e_f5a4)?;
        c.g[A0][0] = Addr(frame.at(c, LOBBY_SORT_POINTERS)).lw(c);
        call_ptr(c, callee::LOBBY_MEM_FREE.addr, 0x003e_f5ac)?;
        c.g[A0][0] = Addr(frame.at(c, LOBBY_SORT_POINTERS_ALT)).lw(c);
        call_ptr(c, callee::LOBBY_MEM_FREE.addr, 0x003e_f5b4)?;
        c.g[T1][0] = Addr(frame.at(c, LOBBY_SORT_ITEM_SCRATCH)).lw(c);
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(frame.at(c, LOBBY_SORT_STACK_SCRATCH)).lw(c);
    let heap_item = c.g[T1][0] != c.g[V0][0];
    frame.restore(c, S0, LOBBY_SORT_SAVE_S0);
    if heap_item {
        c.cycles += 2;
        c.g[A0][0] = c.g[T1][0];
        call_ptr(c, callee::LOBBY_MEM_FREE.addr, 0x003e_f5cc)?;
        c.cycles += 1;
        frame.restore(c, S0, LOBBY_SORT_SAVE_S0);
    }
    lobby_sort_return(c, frame);
    Ok(())
}

pub(crate) fn lobby_sort_return(c: &mut Cpu, frame: Frame) {
    c.cycles += 11;
    frame.restore(c, S1, LOBBY_SORT_SAVE_S1);
    frame.restore(c, S2, LOBBY_SORT_SAVE_S2);
    frame.restore(c, S3, LOBBY_SORT_SAVE_S3);
    frame.restore(c, S4, LOBBY_SORT_SAVE_S4);
    frame.restore(c, S5, LOBBY_SORT_SAVE_S5);
    frame.restore(c, S6, LOBBY_SORT_SAVE_S6);
    frame.restore(c, S7, LOBBY_SORT_SAVE_S7);
    frame.restore(c, FP, LOBBY_SORT_SAVE_FP);
    frame.restore_ra(c, LOBBY_SORT_SAVE_RA);
    frame.close(c);
}

pub(crate) const DS_TIMEZONE_GP: u32 = 0x19a4;

pub(crate) const DS_TIMEZONE_UNSET: u64 = u64::MAX;

pub(crate) const DS_RTC_SECOND: u32 = 1;

pub(crate) const DS_RTC_MINUTE: u32 = 2;

pub(crate) const DS_RTC_HOUR: u32 = 3;

pub(crate) const DS_RTC_DAY: u32 = 5;

pub(crate) const DS_RTC_MONTH: u32 = 6;

pub(crate) const DS_RTC_YEAR: u32 = 7;

pub(crate) const DS_RTC_DAY_OF_TEST_DATE: u64 = 2;

pub(crate) const DS_RTC_MONTH_YEAR_OF_TEST_DATE: u64 = 1;

pub(crate) const DECIMAL_BASE: u64 = 10;

pub(crate) const BCD_NIBBLE_BITS: u32 = 4;

pub(crate) const BCD_NIBBLE_MASK: u64 = 0xf;

pub(crate) const DS_SECONDS_PER_MINUTE: u64 = 60;

pub(crate) const DS_SECONDS_PER_HOUR: u64 = 60 * DS_SECONDS_PER_MINUTE;

pub(crate) const DS_SECONDS_PER_DAY: u64 = 24 * DS_SECONDS_PER_HOUR;

pub(crate) const DS_TIMEZONE_FRAME_BYTES: u32 = 0x90;

pub(crate) const DS_TIMEZONE_SAVED_RA: u32 = 0x10;

pub(crate) const DS_TIMEZONE_SAVED_REGISTERS: [(usize, u32); 7] = [
    (S0, 0x80),
    (S1, 0x70),
    (S2, 0x60),
    (S3, 0x50),
    (S4, 0x40),
    (S5, 0x30),
    (S6, 0x20),
];

/// Returns the cached DirtySDK time-zone offset in seconds, deriving it from
/// the difference between the local and GMT conversion of a fixed RTC date.
pub fn ds_timezone(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, DS_TIMEZONE_FRAME_BYTES);
        c.g[V1][0] = Addr(r32(c, GP)).off(DS_TIMEZONE_GP).lw(c);
        for (reg, offset) in DS_TIMEZONE_SAVED_REGISTERS {
            frame.save(c, reg, offset);
        }
        c.g[V0][0] = DS_TIMEZONE_UNSET;
        let cold = c.g[V1][0] == c.g[V0][0];
        frame.save_ra(c, DS_TIMEZONE_SAVED_RA);
        if cold {
            c.cycles += 73;
            let rtc = Addr(r32(c, SP));
            rtc.sd(c, 0);
            c.g[S2][0] = DS_RTC_MONTH_YEAR_OF_TEST_DATE;
            c.g[S4][0] = DS_RTC_DAY_OF_TEST_DATE;
            c.g[A0][0] = c.g[SP][0];
            rtc.off(DS_RTC_DAY).sb(c, c.g[S4][0]);
            c.g[S1][0] = DECIMAL_BASE;
            rtc.off(DS_RTC_MONTH).sb(c, c.g[S2][0]);
            c.g[S3][0] = DS_SECONDS_PER_DAY;
            rtc.off(DS_RTC_YEAR).sb(c, c.g[S2][0]);
            callee::SCE_SCF_GET_GMT_FROM_RTC.call(c, 0x003e_f0ec)?;

            let rtc = Addr(r32(c, SP));
            c.g[S5][0] = DS_SECONDS_PER_HOUR;
            c.g[A1][0] = rtc.off(DS_RTC_DAY).lbu(c);
            c.g[S6][0] = DS_SECONDS_PER_MINUTE;
            c.g[A2][0] = rtc.off(DS_RTC_HOUR).lbu(c);
            c.g[A0][0] = c.g[SP][0];
            c.g[A3][0] = rtc.off(DS_RTC_MINUTE).lbu(c);
            set_word(c, S0, r32(c, A1) >> BCD_NIBBLE_BITS);
            set_word(c, V1, r32(c, A2) >> BCD_NIBBLE_BITS);
            c.mult(0, S0, S0, S1);
            set_word(c, V0, r32(c, A3) >> BCD_NIBBLE_BITS);
            c.mult(0, V1, V1, S1);
            c.mult(0, V0, V0, S1);
            c.g[T0][0] = rtc.off(DS_RTC_SECOND).lbu(c);
            c.g[A1][0] &= BCD_NIBBLE_MASK;
            c.g[A2][0] &= BCD_NIBBLE_MASK;
            c.g[S0][0] = addu(c.g[S0][0], c.g[A1][0]);
            c.g[A3][0] &= BCD_NIBBLE_MASK;
            c.g[V1][0] = addu(c.g[V1][0], c.g[A2][0]);
            set_word(c, A1, r32(c, T0) >> BCD_NIBBLE_BITS);
            c.g[V0][0] = addu(c.g[V0][0], c.g[A3][0]);
            c.mult(0, V1, V1, S5);
            c.mult(0, S0, S0, S3);
            c.g[T0][0] &= BCD_NIBBLE_MASK;
            c.mult(0, V0, V0, S6);
            rtc.sd(c, 0);
            c.mult(0, A1, A1, S1);
            rtc.off(DS_RTC_DAY).sb(c, c.g[S4][0]);
            rtc.off(DS_RTC_YEAR).sb(c, c.g[S2][0]);
            c.g[S0][0] = addu(c.g[S0][0], c.g[V1][0]);
            rtc.off(DS_RTC_MONTH).sb(c, c.g[S2][0]);
            c.g[S0][0] = addu(c.g[S0][0], c.g[V0][0]);
            c.g[A1][0] = addu(c.g[A1][0], c.g[T0][0]);
            c.g[S0][0] = addu(c.g[S0][0], c.g[A1][0]);
            callee::SCE_SCF_GET_LOCAL_TIME_FROM_RTC.call(c, 0x003e_f174)?;

            let rtc = Addr(r32(c, SP));
            c.g[V1][0] = rtc.off(DS_RTC_DAY).lbu(c);
            c.g[A2][0] = rtc.off(DS_RTC_HOUR).lbu(c);
            c.g[T0][0] = rtc.off(DS_RTC_MINUTE).lbu(c);
            set_word(c, V0, r32(c, V1) >> BCD_NIBBLE_BITS);
            set_word(c, A1, r32(c, A2) >> BCD_NIBBLE_BITS);
            c.mult(0, V0, V0, S1);
            set_word(c, A0, r32(c, T0) >> BCD_NIBBLE_BITS);
            c.mult(0, A1, A1, S1);
            c.mult(0, A0, A0, S1);
            c.g[V1][0] &= BCD_NIBBLE_MASK;
            c.g[A3][0] = rtc.off(DS_RTC_SECOND).lbu(c);
            c.g[A2][0] &= BCD_NIBBLE_MASK;
            c.g[V0][0] = addu(c.g[V0][0], c.g[V1][0]);
            c.g[T0][0] &= BCD_NIBBLE_MASK;
            c.g[A1][0] = addu(c.g[A1][0], c.g[A2][0]);
            set_word(c, V1, r32(c, A3) >> BCD_NIBBLE_BITS);
            c.mult(0, V0, V0, S3);
            c.mult(0, A1, A1, S5);
            c.g[A0][0] = addu(c.g[A0][0], c.g[T0][0]);
            c.g[A3][0] &= BCD_NIBBLE_MASK;
            c.mult(0, A0, A0, S6);
            c.mult(0, V1, V1, S1);
            c.g[V0][0] = addu(c.g[V0][0], c.g[A1][0]);
            c.g[V0][0] = addu(c.g[V0][0], c.g[A0][0]);
            c.g[V1][0] = addu(c.g[V1][0], c.g[A3][0]);
            c.g[V0][0] = addu(c.g[V0][0], c.g[V1][0]);
            set_word(c, V0, r32(c, V0).wrapping_sub(r32(c, S0)));
            Addr(r32(c, GP)).off(DS_TIMEZONE_GP).sw(c, c.g[V0][0]);
        }
        c.cycles += 11;
        c.g[V0][0] = Addr(r32(c, GP)).off(DS_TIMEZONE_GP).lw(c);
        for (reg, offset) in DS_TIMEZONE_SAVED_REGISTERS {
            frame.restore(c, reg, offset);
        }
        frame.restore_ra(c, DS_TIMEZONE_SAVED_RA);
        frame.close(c);
        Ok(())
    });
}

/// Returns the DirtySDK clock stub's zero wall-clock time.
pub fn ds_timeinsecs(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0;
    ret(c);
}

pub(crate) const HASH_ENTRY_COUNT: u32 = 0;

pub(crate) const HASH_BUCKET_COUNT: u32 = 0x10;

pub(crate) const HASH_BUCKETS: u32 = 0x14;

pub(crate) const HASH_FREE_HEAD: u32 = 0x1c;

pub(crate) const HASH_ENTRY_VALUE: u32 = 4;

pub(crate) const HASH_ENTRY_KEY: u32 = 8;

pub(crate) const HASH_DEL_INT_LOOP: u32 = 0x003e_ef70;

pub(crate) const HASH_FIND_INT_LOOP: u32 = 0x003e_ef1c;

pub(crate) fn hash_entry_recycle(c: &mut Cpu, table: usize, entry: usize, link: usize) {
    c.g[V0][0] = Addr(r32(c, entry)).lw(c);
    Addr(r32(c, link)).sw(c, c.g[V0][0]);
    c.g[V1][0] = Addr(r32(c, table)).off(HASH_FREE_HEAD).lw(c);
    Addr(r32(c, entry)).sw(c, c.g[V1][0]);
    Addr(r32(c, table)).off(HASH_FREE_HEAD).sw(c, c.g[entry][0]);
    c.g[V0][0] = Addr(r32(c, table)).off(HASH_ENTRY_COUNT).lw(c);
    set_word(c, V0, r32(c, V0).wrapping_sub(1));
    Addr(r32(c, table)).off(HASH_ENTRY_COUNT).sw(c, c.g[V0][0]);
    c.g[V0][0] = Addr(r32(c, entry)).off(HASH_ENTRY_VALUE).lw(c);
}

/// Removes an integer key, returns its value, and recycles its entry.
pub fn lobby_hash_del_int(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A2][0] = c.g[A0][0];
    c.g[V0][0] = c.g[A1][0];
    if (c.g[A1][0] as i64) < 0 {
        c.cycles += 1;
        set_word(c, V0, r32(c, V0).wrapping_neg());
    }
    c.cycles += 4;
    c.g[V1][0] = Addr(r32(c, A2)).off(HASH_BUCKET_COUNT).lw(c);
    c.div(0, V0, V1);
    c.cycles += 4;
    c.g[V0][0] = Addr(r32(c, A2)).off(HASH_BUCKETS).lw(c);
    c.g[A0][0] = sx((c.hi[0] as u32) << 2);
    c.g[V1][0] = addu(c.g[V0][0], c.g[A0][0]);

    loop {
        c.cycles += 3;
        c.g[A0][0] = Addr(r32(c, V1)).lw(c);
        if c.g[A0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
            break;
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, A0)).off(HASH_ENTRY_KEY).lw(c);
        if c.g[V0][0] == c.g[A1][0] {
            c.cycles += 10;
            hash_entry_recycle(c, A2, A0, V1);
            break;
        }
        c.g[V1][0] = c.g[A0][0];
        if event_due(c) {
            deliver_event_here(c, HASH_DEL_INT_LOOP);
        }
    }
    ret(c);
}

/// Returns the value held for an integer key, or zero if it is missing.
pub fn lobby_hash_find_int(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A2][0] = c.g[A1][0];
    c.g[V1][0] = Addr(r32(c, A0)).off(HASH_BUCKET_COUNT).lw(c);
    c.g[V0][0] = c.g[A2][0];
    if (c.g[A2][0] as i64) < 0 {
        c.cycles += 1;
        set_word(c, V0, r32(c, V0).wrapping_neg());
    }
    c.cycles += 3;
    c.div(0, V0, V1);
    c.cycles += 5;
    c.g[V0][0] = Addr(r32(c, A0)).off(HASH_BUCKETS).lw(c);
    c.g[A1][0] = sx((c.hi[0] as u32) << 2);
    c.g[A1][0] = addu(c.g[A1][0], c.g[V0][0]);
    c.g[A1][0] = Addr(r32(c, A1)).lw(c);

    loop {
        c.cycles += 2;
        if c.g[A1][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
            break;
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, A1)).off(HASH_ENTRY_KEY).lw(c);
        if c.g[V0][0] == c.g[A2][0] {
            c.cycles += 2;
            c.g[V0][0] = Addr(r32(c, A1)).off(HASH_ENTRY_VALUE).lw(c);
            break;
        }
        c.g[A1][0] = Addr(r32(c, A1)).lw(c);
        if event_due(c) {
            deliver_event_here(c, HASH_FIND_INT_LOOP);
        }
    }
    ret(c);
}

pub(crate) const HASH_COMPARE: u32 = 0x20;

pub(crate) const HASH_DEL_STR_LOOP: u32 = 0x003e_ed38;

pub(crate) const HASH_FIND_STR_LOOP: u32 = 0x003e_ecc8;

pub(crate) const HASH_REPLACE_STR_LOOP: u32 = 0x003e_ec40;

pub(crate) const HASH_STRING_LOOP: u32 = 0x003e_ea60;

pub(crate) const HASH_STRING_MULTIPLIER: u64 = 131;

pub(crate) const HASH_STRING_CASE_MASK: u64 = 0x5f;

pub(crate) const HASH_STRING_FIRST_SIGNIFICANT: i64 = b'!' as i64;

pub(crate) const HASH_STRING_NONNEGATIVE: u64 = i32::MAX as u64;

pub(crate) const HASH_GROW_BASE: u32 = 10;

pub(crate) const HASH_SAVED_RA: u32 = 0;

pub(crate) const HASH_FRAME3_BYTES: u32 = 0x40;

pub(crate) const HASH_FRAME4_BYTES: u32 = 0x50;

pub(crate) const HASH_FRAME3_S0: u32 = 0x30;

pub(crate) const HASH_FRAME3_S1: u32 = 0x20;

pub(crate) const HASH_FRAME3_S2: u32 = 0x10;

pub(crate) const HASH_FRAME4_S0: u32 = 0x40;

pub(crate) const HASH_FRAME4_S1: u32 = 0x30;

pub(crate) const HASH_FRAME4_S2: u32 = 0x20;

pub(crate) const HASH_FRAME4_S3: u32 = 0x10;

pub(crate) fn hash_close_frame4(c: &mut Cpu, frame: Frame) {
    frame.restore(c, S0, HASH_FRAME4_S0);
    frame.restore(c, S1, HASH_FRAME4_S1);
    frame.restore(c, S2, HASH_FRAME4_S2);
    frame.restore(c, S3, HASH_FRAME4_S3);
    frame.restore_ra(c, HASH_SAVED_RA);
    frame.close(c);
}

pub(crate) fn hash_close_frame3(c: &mut Cpu, frame: Frame) {
    frame.restore(c, S0, HASH_FRAME3_S0);
    frame.restore(c, S1, HASH_FRAME3_S1);
    frame.restore(c, S2, HASH_FRAME3_S2);
    frame.restore_ra(c, HASH_SAVED_RA);
    frame.close(c);
}

pub(crate) fn hash_grow_for_insert(c: &mut Cpu, return_pc: u32) -> Flow {
    c.cycles += 11;
    c.g[V0][0] = Addr(r32(c, S0)).off(HASH_ENTRY_COUNT).lw(c);
    c.g[A0][0] = c.g[S0][0];
    let count = r32(c, V0);
    let scaled = count.wrapping_mul(3).wrapping_add(HASH_GROW_BASE) as i32;
    let requested = scaled / 2;
    set_word(c, V0, (scaled as u32) >> 31);
    set_word(c, A1, requested as u32);
    callee::LOBBY_HASH_GROW_POOL.call(c, return_pc)
}

pub(crate) fn hash_insert_from_free_head(c: &mut Cpu) {
    c.cycles += 16;
    c.g[A2][0] = sx(r32(c, S1) << 2);
    c.g[V0][0] = 1;
    c.g[V1][0] = Addr(r32(c, A1)).lw(c);
    Addr(r32(c, S0)).off(HASH_FREE_HEAD).sw(c, c.g[V1][0]);
    Addr(r32(c, A1)).off(HASH_ENTRY_KEY).sw(c, c.g[S2][0]);
    Addr(r32(c, A1)).off(HASH_ENTRY_VALUE).sw(c, c.g[S3][0]);
    c.g[V1][0] = Addr(r32(c, S0)).off(HASH_BUCKETS).lw(c);
    c.g[V1][0] = addu(c.g[A2][0], c.g[V1][0]);
    c.g[A0][0] = Addr(r32(c, V1)).lw(c);
    Addr(r32(c, A1)).sw(c, c.g[A0][0]);
    c.g[V1][0] = Addr(r32(c, S0)).off(HASH_BUCKETS).lw(c);
    c.g[A2][0] = addu(c.g[A2][0], c.g[V1][0]);
    Addr(r32(c, A2)).sw(c, c.g[A1][0]);
    c.g[V1][0] = Addr(r32(c, S0)).off(HASH_ENTRY_COUNT).lw(c);
    set_word(c, V1, r32(c, V1).wrapping_add(1));
    Addr(r32(c, S0)).off(HASH_ENTRY_COUNT).sw(c, c.g[V1][0]);
}

/// Inserts an integer key and value at the head of its hash bucket.
pub fn lobby_hash_add_int(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, HASH_FRAME4_BYTES);
        frame.save(c, S0, HASH_FRAME4_S0);
        frame.save(c, S1, HASH_FRAME4_S1);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, HASH_FRAME4_S2);
        frame.save(c, S3, HASH_FRAME4_S3);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, HASH_SAVED_RA);
        c.g[A0][0] = c.g[S2][0];
        if (c.g[S2][0] as i64) < 0 {
            c.cycles += 1;
            set_word(c, A0, r32(c, A0).wrapping_neg());
        }
        c.cycles += 5;
        c.g[S3][0] = c.g[A2][0];
        c.g[V0][0] = Addr(r32(c, S0)).off(HASH_BUCKET_COUNT).lw(c);
        c.div(0, A0, V0);
        c.cycles += 4;
        c.g[V1][0] = Addr(r32(c, S0)).off(HASH_FREE_HEAD).lw(c);
        c.g[S1][0] = c.hi[0];
        if c.g[V1][0] == 0 {
            hash_grow_for_insert(c, 0x003e_ee14)?;
            if (c.g[V0][0] as i64) <= 0 {
                c.cycles += 9;
                c.g[V0][0] = u64::MAX;
                hash_close_frame4(c, frame);
                return Ok(());
            }
        }
        c.g[A1][0] = Addr(r32(c, S0)).off(HASH_FREE_HEAD).lw(c);
        hash_insert_from_free_head(c);
        c.cycles += 7;
        hash_close_frame4(c, frame);
        Ok(())
    });
}

/// Removes the matching string entry, returning its value or zero.
pub fn lobby_hash_del_str(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, HASH_FRAME4_BYTES);
        frame.save(c, S2, HASH_FRAME4_S2);
        frame.save(c, S3, HASH_FRAME4_S3);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, HASH_FRAME4_S1);
        frame.save(c, S0, HASH_FRAME4_S0);
        frame.save_ra(c, HASH_SAVED_RA);
        c.g[S3][0] = c.g[A1][0];
        callee::LOBBY_HASH_STRING.call(c, 0x003e_ed2c)?;
        c.g[V1][0] = Addr(r32(c, S2)).off(HASH_BUCKETS).lw(c);
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[S1][0] = addu(c.g[V1][0], c.g[V0][0]);

        loop {
            c.cycles += 3;
            c.g[S0][0] = Addr(r32(c, S1)).lw(c);
            c.g[A1][0] = c.g[S3][0];
            if c.g[S0][0] == 0 {
                c.cycles += 1;
                c.g[V0][0] = 0;
                break;
            }
            c.cycles += 5;
            c.g[V0][0] = Addr(r32(c, S2)).off(HASH_COMPARE).lw(c);
            let compare = r32(c, V0);
            c.g[A0][0] = Addr(r32(c, S0)).off(HASH_ENTRY_KEY).lw(c);
            call_ptr(c, compare, 0x003e_ed50)?;
            if c.g[V0][0] == 0 {
                c.cycles += 10;
                hash_entry_recycle(c, S2, S0, S1);
                break;
            }
            c.g[S1][0] = Addr(r32(c, S1)).lw(c);
            if event_due(c) {
                deliver_event_here(c, HASH_DEL_STR_LOOP);
            }
        }

        c.cycles += 7;
        hash_close_frame4(c, frame);
        Ok(())
    });
}

pub(crate) fn hash_find_string_entry(
    c: &mut Cpu,
    compare_return: u32,
    loop_head: u32,
    replace: bool,
) -> Flow {
    c.g[V1][0] = Addr(r32(c, S1)).off(HASH_BUCKETS).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = addu(c.g[V0][0], c.g[V1][0]);
    c.g[S0][0] = Addr(r32(c, V0)).lw(c);
    c.g[V0][0] = 0;
    if c.g[S0][0] == 0 {
        return Ok(());
    }

    c.cycles += 2;
    c.g[V0][0] = Addr(r32(c, S1)).off(HASH_COMPARE).lw(c);
    loop {
        c.cycles += 5;
        c.g[A1][0] = c.g[S2][0];
        let compare = r32(c, V0);
        c.g[A0][0] = Addr(r32(c, S0)).off(HASH_ENTRY_KEY).lw(c);
        call_ptr(c, compare, compare_return)?;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = Addr(r32(c, S0)).off(HASH_ENTRY_VALUE).lw(c);
            if replace {
                c.cycles += 1;
                Addr(r32(c, S0)).off(HASH_ENTRY_VALUE).sw(c, c.g[S3][0]);
            }
            return Ok(());
        }
        c.g[S0][0] = Addr(r32(c, S0)).lw(c);
        c.cycles += 2;
        if c.g[S0][0] == 0 {
            c.cycles += 1;
            c.g[V0][0] = 0;
            return Ok(());
        }
        c.g[V0][0] = Addr(r32(c, S1)).off(HASH_COMPARE).lw(c);
        if event_due(c) {
            deliver_event_here(c, loop_head);
        }
    }
}

/// Returns the value held for a string key, or zero if it is missing.
pub fn lobby_hash_find_str(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, HASH_FRAME3_BYTES);
        frame.save(c, S1, HASH_FRAME3_S1);
        frame.save(c, S2, HASH_FRAME3_S2);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, HASH_FRAME3_S0);
        frame.save_ra(c, HASH_SAVED_RA);
        c.g[S2][0] = c.g[A1][0];
        callee::LOBBY_HASH_STRING.call(c, 0x003e_eca8)?;
        hash_find_string_entry(c, 0x003e_ecd4, HASH_FIND_STR_LOOP, false)?;
        c.cycles += 6;
        hash_close_frame3(c, frame);
        Ok(())
    });
}

/// Changes the value held for an existing string key and returns its old value.
pub fn lobby_hash_replace_str(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, HASH_FRAME4_BYTES);
        frame.save(c, S1, HASH_FRAME4_S1);
        frame.save(c, S2, HASH_FRAME4_S2);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, HASH_FRAME4_S3);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, HASH_FRAME4_S0);
        frame.save_ra(c, HASH_SAVED_RA);
        c.g[S3][0] = c.g[A2][0];
        callee::LOBBY_HASH_STRING.call(c, 0x003e_ec20)?;
        hash_find_string_entry(c, 0x003e_ec4c, HASH_REPLACE_STR_LOOP, true)?;
        c.cycles += 7;
        hash_close_frame4(c, frame);
        Ok(())
    });
}

/// Inserts a string key and value at the head of its hash bucket.
pub fn lobby_hash_add_str(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, HASH_FRAME4_BYTES);
        frame.save(c, S0, HASH_FRAME4_S0);
        frame.save(c, S2, HASH_FRAME4_S2);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S3, HASH_FRAME4_S3);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S1, HASH_FRAME4_S1);
        frame.save_ra(c, HASH_SAVED_RA);
        c.g[S3][0] = c.g[A2][0];
        callee::LOBBY_HASH_STRING.call(c, 0x003e_eb58)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V0][0] = Addr(r32(c, S0)).off(HASH_FREE_HEAD).lw(c);
        if c.g[V0][0] == 0 {
            hash_grow_for_insert(c, 0x003e_eb8c)?;
            if (c.g[V0][0] as i64) <= 0 {
                c.cycles += 9;
                c.g[V0][0] = u64::MAX;
                hash_close_frame4(c, frame);
                return Ok(());
            }
        }
        c.g[A1][0] = Addr(r32(c, S0)).off(HASH_FREE_HEAD).lw(c);
        hash_insert_from_free_head(c);
        c.cycles += 7;
        hash_close_frame4(c, frame);
        Ok(())
    });
}

/// Hashes significant single-byte characters into a lobby hash bucket.
pub fn lobby_hash_string(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = Addr(r32(c, A1)).lb(c);
    c.g[A3][0] = 0;
    let empty = c.g[V0][0] == 0;
    c.g[A2][0] = Addr(r32(c, A1)).lbu(c);
    if !empty {
        c.cycles += 2;
        c.g[T0][0] = HASH_STRING_MULTIPLIER;
        loop {
            c.cycles += 5;
            c.g[V0][0] = u64::from(i64::from(r32(c, A2) as i8) < HASH_STRING_FIRST_SIGNIFICANT);
            set_word(c, A1, r32(c, A1).wrapping_add(1));
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.mult(0, V1, A3, T0);
                c.g[V0][0] = c.g[A2][0] & HASH_STRING_CASE_MASK;
                c.g[A3][0] = addu(c.g[V1][0], c.g[V0][0]);
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, A1)).lb(c);
            let more = c.g[V0][0] != 0;
            c.g[A2][0] = Addr(r32(c, A1)).lbu(c);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, HASH_STRING_LOOP);
            }
        }
    }
    c.cycles += 6;
    c.g[V1][0] = Addr(r32(c, A0)).off(HASH_BUCKET_COUNT).lw(c);
    c.g[V0][0] = HASH_STRING_NONNEGATIVE;
    c.g[A3][0] &= c.g[V0][0];
    c.cycles += 4;
    c.div(0, A3, V1);
    c.g[V0][0] = c.hi[0];
    ret(c);
}

pub(crate) const HASH_POP_BUCKET: u32 = 4;

pub(crate) const HASH_POOL_BLOCKS: u32 = 0x18;

pub(crate) const HASH_ENTRY_BYTES: u64 = 12;

pub(crate) const HASH_HEADER_BYTES: u64 = 0x24;

pub(crate) const HASH_MIN_BUCKETS: u64 = 16;

pub(crate) const HASH_STR_CMP_NO_CASE: u32 = 0x003e_eab8;

pub(crate) const HASH_POP_LOOP: u32 = 0x003e_e9e0;

pub(crate) const HASH_GROW_POOL_LOOP: u32 = 0x003e_e968;

pub(crate) const HASH_DESTROY_LOOP: u32 = 0x003e_e840;

pub(crate) const HASH_FRAME1_BYTES: u32 = 0x20;

pub(crate) const HASH_FRAME1_S0: u32 = 0x10;

/// Removes an entry from the first occupied bucket at or after the cursor and returns its value.
pub fn lobby_hash_pop_any(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A2][0] = c.g[A0][0];
    c.g[V0][0] = Addr(r32(c, A2)).off(HASH_ENTRY_COUNT).lw(c);
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        ret(c);
        return;
    }
    c.g[V0][0] = Addr(r32(c, A2)).off(HASH_POP_BUCKET).lw(c);
    c.cycles += 6;
    c.g[A0][0] = Addr(r32(c, A2)).off(HASH_BUCKETS).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = addu(c.g[V0][0], c.g[A0][0]);
    c.g[V1][0] = Addr(r32(c, V0)).lw(c);
    let empty = c.g[V1][0] == 0;
    c.g[V0][0] = Addr(r32(c, A2)).off(HASH_POP_BUCKET).lw(c);
    if empty {
        c.cycles += 4;
        c.g[A1][0] = Addr(r32(c, A2)).off(HASH_BUCKET_COUNT).lw(c);
        c.g[A3][0] = c.g[A0][0];
        c.g[T0][0] = 0;
        loop {
            c.cycles += 11;
            set_word(c, V0, r32(c, V0).wrapping_add(1));
            c.div(0, V0, A1);
            c.g[A0][0] = c.hi[0];
            c.g[V1][0] = sx(r32(c, A0) << 2);
            Addr(r32(c, A2)).off(HASH_POP_BUCKET).sw(c, c.g[A0][0]);
            c.g[V1][0] = addu(c.g[V1][0], c.g[A3][0]);
            c.g[V0][0] = Addr(r32(c, V1)).lw(c);
            let occupied = c.g[V0][0] != 0;
            c.g[V0][0] = Addr(r32(c, A2)).off(HASH_POP_BUCKET).lw(c);
            if occupied {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, HASH_POP_LOOP);
            }
        }
    }
    c.cycles += 14;
    c.g[V1][0] = Addr(r32(c, A2)).off(HASH_BUCKETS).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = addu(c.g[V0][0], c.g[V1][0]);
    c.g[A1][0] = Addr(r32(c, V0)).lw(c);
    c.g[V1][0] = Addr(r32(c, A1)).lw(c);
    Addr(r32(c, V0)).sw(c, c.g[V1][0]);
    c.g[A0][0] = Addr(r32(c, A2)).off(HASH_FREE_HEAD).lw(c);
    Addr(r32(c, A1)).sw(c, c.g[A0][0]);
    Addr(r32(c, A2)).off(HASH_FREE_HEAD).sw(c, c.g[A1][0]);
    c.g[V0][0] = Addr(r32(c, A2)).off(HASH_ENTRY_COUNT).lw(c);
    set_word(c, V0, r32(c, V0).wrapping_sub(1));
    Addr(r32(c, A2)).off(HASH_ENTRY_COUNT).sw(c, c.g[V0][0]);
    c.g[V0][0] = Addr(r32(c, A1)).off(HASH_ENTRY_VALUE).lw(c);
    ret(c);
}

/// Allocates another block of hash entries and links its entries into the free pool.
pub fn lobby_hash_grow_pool(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, HASH_FRAME3_BYTES);
        let positive_count = (c.g[A1][0] as i64) > 0;
        c.g[V0][0] = u64::from(positive_count);
        frame.save(c, S1, HASH_FRAME3_S1);
        c.g[V1][0] = HASH_ENTRY_BYTES;
        c.g[S1][0] = 1;
        frame.save(c, S2, HASH_FRAME3_S2);
        if positive_count {
            c.g[S1][0] = c.g[A1][0];
        }
        frame.save(c, S0, HASH_FRAME3_S0);
        set_word(c, S1, r32(c, S1).wrapping_add(1));
        c.g[S0][0] = c.g[A0][0];
        c.mult(0, S2, S1, V1);
        frame.save_ra(c, HASH_SAVED_RA);
        c.g[A0][0] = c.g[S2][0];
        callee::LOBBY_MEM_ALLOC.call(c, 0x003e_e938)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = u64::MAX;
        if c.g[A1][0] != 0 {
            c.cycles += 7;
            c.g[V0][0] = Addr(r32(c, S0)).off(HASH_POOL_BLOCKS).lw(c);
            c.g[A0][0] = addu(c.g[A1][0], c.g[S2][0]);
            set_word(c, A2, r32(c, S1).wrapping_sub(1));
            set_word(c, V1, r32(c, A1).wrapping_add(HASH_ENTRY_BYTES as u32));
            Addr(r32(c, A1)).sw(c, c.g[V0][0]);
            let have_entries = c.g[V1][0] != c.g[A0][0];
            Addr(r32(c, S0)).off(HASH_POOL_BLOCKS).sw(c, c.g[A1][0]);
            if have_entries {
                c.cycles += 2;
                c.g[V0][0] = Addr(r32(c, S0)).off(HASH_FREE_HEAD).lw(c);
                loop {
                    c.cycles += 7;
                    Addr(r32(c, V1)).sw(c, c.g[V0][0]);
                    Addr(r32(c, S0)).off(HASH_FREE_HEAD).sw(c, c.g[V1][0]);
                    set_word(c, V1, r32(c, V1).wrapping_add(HASH_ENTRY_BYTES as u32));
                    if c.g[V1][0] == c.g[A0][0] {
                        break;
                    }
                    c.g[V0][0] = Addr(r32(c, S0)).off(HASH_FREE_HEAD).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, HASH_GROW_POOL_LOOP);
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[A2][0];
        }
        c.cycles += 6;
        hash_close_frame3(c, frame);
        Ok(())
    });
}

/// Frees the pool blocks, bucket array, and lobby hash table.
pub fn lobby_hash_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, HASH_FRAME1_BYTES);
        frame.save(c, S0, HASH_FRAME1_S0);
        frame.save_ra(c, HASH_SAVED_RA);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, S0)).off(HASH_POOL_BLOCKS).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            c.g[A0][0] = Addr(r32(c, S0)).off(HASH_POOL_BLOCKS).lw(c);
            loop {
                c.cycles += 6;
                c.g[V0][0] = Addr(r32(c, A0)).lw(c);
                Addr(r32(c, S0)).off(HASH_POOL_BLOCKS).sw(c, c.g[V0][0]);
                callee::LOBBY_MEM_FREE.call(c, 0x003e_e84c)?;
                c.g[V0][0] = Addr(r32(c, S0)).off(HASH_POOL_BLOCKS).lw(c);
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A0][0] = Addr(r32(c, S0)).off(HASH_POOL_BLOCKS).lw(c);
                if event_due(c) {
                    deliver_event_here(c, HASH_DESTROY_LOOP);
                }
            }
        }
        c.cycles += 8;
        c.g[A0][0] = Addr(r32(c, S0)).off(HASH_BUCKETS).lw(c);
        callee::LOBBY_MEM_FREE.call(c, 0x003e_e860)?;
        c.g[A0][0] = c.g[S0][0];
        callee::LOBBY_MEM_FREE.call(c, 0x003e_e868)?;
        frame.restore(c, S0, HASH_FRAME1_S0);
        frame.restore_ra(c, HASH_SAVED_RA);
        frame.close(c);
        Ok(())
    });
}

/// Creates a lobby hash table with a bucket array and an initial entry pool.
pub fn lobby_hash_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, HASH_FRAME3_BYTES);
        let positive_entries = (c.g[A0][0] as i64) > 0;
        c.g[V1][0] = u64::from(positive_entries);
        frame.save(c, S2, HASH_FRAME3_S2);
        c.g[V0][0] = HASH_MIN_BUCKETS - 1;
        frame.save(c, S0, HASH_FRAME3_S0);
        c.g[S2][0] = 1;
        if positive_entries {
            c.g[S2][0] = c.g[A0][0];
        }
        let enough_buckets = (c.g[A1][0] as i64) > (c.g[V0][0] as i64);
        c.g[V0][0] = u64::from(enough_buckets);
        c.g[S0][0] = HASH_MIN_BUCKETS;
        frame.save(c, S1, HASH_FRAME3_S1);
        frame.save_ra(c, HASH_SAVED_RA);
        if enough_buckets {
            c.g[S0][0] = c.g[A1][0];
        }
        c.g[A0][0] = HASH_HEADER_BYTES;
        callee::LOBBY_MEM_ALLOC.call(c, 0x003e_e7b0)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A2][0] = HASH_HEADER_BYTES;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[S1][0];
        callee::MEMSET.call(c, 0x003e_e7c4)?;
        Addr(r32(c, S1)).off(HASH_BUCKET_COUNT).sw(c, c.g[S0][0]);
        c.g[A0][0] = sx(r32(c, S0) << 2);
        callee::LOBBY_MEM_ALLOC.call(c, 0x003e_e7d0)?;
        c.g[A2][0] = Addr(r32(c, S1)).off(HASH_BUCKET_COUNT).lw(c);
        c.g[A0][0] = c.g[V0][0];
        Addr(r32(c, S1)).off(HASH_BUCKETS).sw(c, c.g[V0][0]);
        c.g[A1][0] = 0;
        c.g[A2][0] = sx(r32(c, A2) << 2);
        callee::MEMSET.call(c, 0x003e_e7e8)?;
        c.g[V1][0] = sx(HASH_STR_CMP_NO_CASE);
        let have_entries = (c.g[S2][0] as i64) > 0;
        Addr(r32(c, S1)).off(HASH_COMPARE).sw(c, c.g[V1][0]);
        if have_entries {
            c.cycles += 3;
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.g[S1][0];
            callee::LOBBY_HASH_GROW_POOL.call(c, 0x003e_e804)?;
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S1][0];
        hash_close_frame3(c, frame);
        Ok(())
    });
}

pub(crate) const LIST_SORT_DIRTY: u32 = 0x08;

pub(crate) const LIST_NOTIFY_DIRTY: u32 = 0x0c;

pub(crate) const LIST_VISIBLE_COUNT: u32 = 0x14;

pub(crate) const LIST_ENTRY_COUNT: u32 = 0x18;

pub(crate) const LIST_FILTER_ARG1: u32 = 0x1c;

pub(crate) const LIST_FILTER_ARG0: u32 = 0x20;

pub(crate) const LIST_FILTER: u32 = 0x24;

pub(crate) const LIST_SORT_ARG1: u32 = 0x28;

pub(crate) const LIST_SORT_ARG0: u32 = 0x2c;

pub(crate) const LIST_COMPARE: u32 = 0x30;

pub(crate) const LIST_HASH: u32 = 0x34;

pub(crate) const LIST_ENTRIES: u32 = 0x40;

pub(crate) const LIST_ENTRY_POINTER_BYTES: u32 = 4;

pub(crate) const LIST_COMPARE_THUNK: u32 = 0x003e_e1b0;

pub(crate) const LIST_FILTER_LOOP: u32 = 0x003e_e680;

pub(crate) const LIST_APPEND_HIDDEN_LOOP: u32 = 0x003e_e6f0;

pub(crate) fn list_sort_partition(c: &mut Cpu) -> Flow {
    c.cycles += 6;
    c.g[S1][0] = Addr(r32(c, S2)).off(LIST_ENTRIES).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[S3][0] = addu(c.g[S1][0], c.g[V0][0]);
    c.g[S0][0] = c.g[S1][0];
    c.g[A0][0] = c.g[S1][0];
    if c.g[S1][0] == c.g[S3][0] {
        return Ok(());
    }

    c.cycles += 1;
    loop {
        c.cycles += 3;
        c.g[V1][0] = Addr(r32(c, S2)).off(LIST_FILTER).lw(c);
        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        let mut visible = true;
        if c.g[V1][0] != 0 {
            c.cycles += 6;
            c.g[A0][0] = Addr(r32(c, S2)).off(LIST_FILTER_ARG0).lw(c);
            c.g[A2][0] = Addr(r32(c, V0)).lw(c);
            let filter = r32(c, V1);
            c.g[A1][0] = Addr(r32(c, S2)).off(LIST_FILTER_ARG1).lw(c);
            call_ptr(c, filter, 0x003e_e69c)?;
            visible = (c.g[V0][0] as i64) > 0;
            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        }
        if visible {
            c.cycles += 3;
            Addr(r32(c, S1)).sw(c, c.g[V0][0]);
            set_word(c, S1, r32(c, S1).wrapping_add(LIST_ENTRY_POINTER_BYTES));
        } else {
            c.cycles += 2;
            Addr(r32(c, S3)).sw(c, c.g[V0][0]);
            set_word(c, S3, r32(c, S3).wrapping_add(LIST_ENTRY_POINTER_BYTES));
        }
        c.cycles += 7;
        c.g[V0][0] = Addr(r32(c, S2)).off(LIST_ENTRY_COUNT).lw(c);
        set_word(c, S0, r32(c, S0).wrapping_add(LIST_ENTRY_POINTER_BYTES));
        c.g[V1][0] = Addr(r32(c, S2)).off(LIST_ENTRIES).lw(c);
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V0][0] = addu(c.g[V1][0], c.g[V0][0]);
        let more = c.g[S0][0] != c.g[V0][0];
        c.g[A0][0] = c.g[V1][0];
        if !more {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, LIST_FILTER_LOOP);
        }
    }
}

pub(crate) fn list_sort_append_hidden(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V1][0] = Addr(r32(c, S2)).off(LIST_ENTRY_COUNT).lw(c);
    let visible_bytes = r32(c, S1).wrapping_sub(r32(c, A0)) as i32;
    set_word(c, V0, (visible_bytes >> 2) as u32);
    c.g[V1][0] = sx(r32(c, V1) << 2);
    c.g[S0][0] = addu(c.g[A0][0], c.g[V1][0]);
    let have_hidden = c.g[S0][0] != c.g[S3][0];
    Addr(r32(c, S2)).off(LIST_VISIBLE_COUNT).sw(c, c.g[V0][0]);
    if have_hidden {
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
            set_word(c, S0, r32(c, S0).wrapping_add(LIST_ENTRY_POINTER_BYTES));
            Addr(r32(c, S1)).sw(c, c.g[V0][0]);
            let more = c.g[S0][0] != c.g[S3][0];
            set_word(c, S1, r32(c, S1).wrapping_add(LIST_ENTRY_POINTER_BYTES));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, LIST_APPEND_HIDDEN_LOOP);
            }
        }
    }
}

/// Returns the hash-table pointer attached to the lobby list.
pub fn lobby_list_get_hash(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = Addr(r32(c, A0)).off(LIST_HASH).lw(c);
    ret(c);
}

/// Attaches a hash-table pointer to the lobby list.
pub fn lobby_list_set_hash(c: &mut Cpu) {
    c.cycles += 2;
    Addr(r32(c, A0)).off(LIST_HASH).sw(c, c.g[A1][0]);
    ret(c);
}

/// Repartitions a changed lobby list by visibility, then optionally sorts visible entries.
pub fn lobby_list_sort(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, LIST_STORAGE_FRAME);
        frame.save(c, S2, LIST_STORAGE_SAVE_S2);
        frame.save(c, S0, LIST_STORAGE_SAVE_S0);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, LIST_STORAGE_SAVE_S1);
        frame.save(c, S3, LIST_STORAGE_SAVE_S3);
        frame.save_ra(c, LIST_SAVE_RA);
        c.g[V1][0] = Addr(r32(c, S2)).off(LIST_SORT_DIRTY).lw(c);
        c.g[V0][0] = Addr(r32(c, S2)).off(LIST_NOTIFY_DIRTY).lw(c);
        let dirty = addu(c.g[V1][0], c.g[V0][0]) != 0;
        c.g[V0][0] = 0;
        if dirty {
            c.cycles += 2;
            if c.g[V1][0] == 0 {
                c.cycles += 3;
                Addr(r32(c, S2)).off(LIST_NOTIFY_DIRTY).sw(c, 0);
                c.g[V0][0] = 1;
            } else {
                c.g[V0][0] = Addr(r32(c, S2)).off(LIST_ENTRY_COUNT).lw(c);
                list_sort_partition(c)?;
                list_sort_append_hidden(c);
                c.cycles += 6;
                c.g[A1][0] = Addr(r32(c, S2)).off(LIST_ENTRIES).lw(c);
                c.g[V1][0] = Addr(r32(c, S2)).off(LIST_COMPARE).lw(c);
                let list_bytes = r32(c, S1).wrapping_sub(r32(c, A1)) as i32;
                set_word(c, V0, (list_bytes >> 2) as u32);
                let needs_sort = c.g[V1][0] != 0;
                Addr(r32(c, S2)).off(LIST_ENTRY_COUNT).sw(c, c.g[V0][0]);
                if needs_sort {
                    c.cycles += 6;
                    c.g[A2][0] = Addr(r32(c, S2)).off(LIST_VISIBLE_COUNT).lw(c);
                    c.g[T0][0] = sx(LIST_COMPARE_THUNK);
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A3][0] = u64::from(LIST_ENTRY_POINTER_BYTES);
                    callee::LOBBY_MERGE_SORT.call(c, 0x003e_e73c)?;
                }
                c.cycles += 3;
                Addr(r32(c, S2)).off(LIST_NOTIFY_DIRTY).sw(c, 0);
                c.g[V0][0] = 2;
                Addr(r32(c, S2)).off(LIST_SORT_DIRTY).sw(c, 0);
            }
        }
        c.cycles += 7;
        hash_close_frame4(c, frame);
        Ok(())
    });
}

/// Optionally increments the lobby list's notification counter and returns whether it is positive.
pub fn lobby_list_mark_dirty(c: &mut Cpu) {
    c.cycles += 2;
    let list = Addr(r32(c, A0));
    let mark = c.g[A1][0] != 0;
    c.g[V0][0] = list.off(LIST_NOTIFY_DIRTY).lw(c);
    if mark {
        c.cycles += 3;
        set_word(c, V0, r32(c, V0).wrapping_add(1));
        list.off(LIST_NOTIFY_DIRTY).sw(c, c.g[V0][0]);
        c.g[V0][0] = list.off(LIST_NOTIFY_DIRTY).lw(c);
    }
    c.cycles += 2;
    c.g[V0][0] = u64::from((c.g[V0][0] as i64) > 0);
    ret(c);
}

/// Sets the list's comparison callback and arguments, requesting a sort when entries exist.
pub fn lobby_list_set_sort(c: &mut Cpu) {
    c.cycles += 3;
    let list = Addr(r32(c, A0));
    c.g[V0][0] = list.off(LIST_ENTRY_COUNT).lw(c);
    let has_entries = (c.g[V0][0] as i64) > 0;
    c.g[V0][0] = 1;
    if has_entries {
        c.cycles += 1;
        list.off(LIST_SORT_DIRTY).sw(c, c.g[V0][0]);
    }
    c.cycles += 4;
    list.off(LIST_COMPARE).sw(c, c.g[A3][0]);
    list.off(LIST_SORT_ARG0).sw(c, c.g[A1][0]);
    list.off(LIST_SORT_ARG1).sw(c, c.g[A2][0]);
    ret(c);
}

/// Returns the item at a valid lobby list index, otherwise zero.
pub fn lobby_list_get_at(c: &mut Cpu) {
    c.cycles += 2;
    if c.g[A0][0] != 0 {
        c.cycles += 2;
        if (c.g[A1][0] as i64) >= 0 {
            c.cycles += 4;
            let list = Addr(r32(c, A0));
            c.g[V0][0] = list.off(LIST_ENTRY_COUNT).lw(c);
            let in_bounds = (c.g[A1][0] as i64) < (c.g[V0][0] as i64);
            c.g[V0][0] = u64::from(in_bounds);
            if in_bounds {
                c.g[V0][0] = list.off(LIST_ENTRIES).lw(c);
                c.cycles += 5;
                c.g[V1][0] = sx(r32(c, A1) << 2);
                c.g[V1][0] = addu(c.g[V1][0], c.g[V0][0]);
                c.g[A0][0] = Addr(r32(c, V1)).lw(c);
                c.g[V0][0] = Addr(r32(c, A0)).lw(c);
                ret(c);
                return;
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    ret(c);
}

pub(crate) const LIST_INITIAL_SIZE: u32 = 0x00;

pub(crate) const LIST_GROW_PERCENT: u32 = 0x04;

pub(crate) const LIST_FREE_COUNT: u32 = 0x10;

pub(crate) const LIST_FREE_HEAD: u32 = 0x38;

pub(crate) const LIST_CHUNKS: u32 = 0x3c;

pub(crate) const LIST_HEADER_BYTES: u64 = 0x44;

pub(crate) const LIST_ENTRY_FRAME: u32 = 0x40;

pub(crate) const LIST_STORAGE_FRAME: u32 = 0x50;

pub(crate) const LIST_SAVE_RA: u32 = 0;

pub(crate) const LIST_ENTRY_SAVE_S0: u32 = 0x30;

pub(crate) const LIST_ENTRY_SAVE_S1: u32 = 0x20;

pub(crate) const LIST_ENTRY_SAVE_S2: u32 = 0x10;

pub(crate) const LIST_STORAGE_SAVE_S0: u32 = 0x40;

pub(crate) const LIST_STORAGE_SAVE_S1: u32 = 0x30;

pub(crate) const LIST_STORAGE_SAVE_S2: u32 = 0x20;

pub(crate) const LIST_STORAGE_SAVE_S3: u32 = 0x10;

pub(crate) const LIST_ENTRY_SHIFT: u32 = LIST_ENTRY_POINTER_BYTES.trailing_zeros();

/// Returns the number of visible entries in the lobby list.
pub fn lobby_list_get_visible_count(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = Addr(r32(c, A0)).off(LIST_VISIBLE_COUNT).lw(c);
    ret(c);
}

/// Returns the total number of entries in the lobby list.
pub fn lobby_list_get_count(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = Addr(r32(c, A0)).off(LIST_ENTRY_COUNT).lw(c);
    ret(c);
}

/// Unlinks the requested lobby entry and returns its data.
pub fn lobby_list_remove(c: &mut Cpu) {
    const SEARCH_HEAD: u32 = 0x003e_e4a8;
    const SHIFT_HEAD: u32 = 0x003e_e508;
    c.cycles += 4;
    c.g[A3][0] = c.g[A0][0];
    c.g[A0][0] = Addr(r32(c, A3)).off(LIST_ENTRY_COUNT).lw(c);
    c.g[A2][0] = 0;
    let hit_first = if (c.g[A0][0] as i64) > 0 {
        c.cycles += 4;
        c.g[V1][0] = Addr(r32(c, A3)).off(LIST_ENTRIES).lw(c);
        c.g[V0][0] = Addr(r32(c, V1)).lw(c);
        if c.g[V0][0] == c.g[A1][0] {
            true
        } else {
            c.cycles += 2;
            set_word(c, A2, r32(c, A2).wrapping_add(1));
            loop {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[A0][0] as i64)) as u64;
                set_word(c, V1, r32(c, V1).wrapping_add(LIST_ENTRY_POINTER_BYTES));
                if c.g[V0][0] == 0 {
                    break;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, V1)).lw(c);
                if c.g[V0][0] == c.g[A1][0] {
                    break;
                }
                set_word(c, A2, r32(c, A2).wrapping_add(1));
                if event_due(c) {
                    deliver_event_here(c, SEARCH_HEAD);
                }
            }
            false
        }
    } else {
        false
    };
    if !hit_first {
        c.cycles += 1;
    }
    c.g[V0][0] = Addr(r32(c, A3)).off(LIST_ENTRY_COUNT).lw(c);
    c.cycles += 2;
    if c.g[A2][0] == c.g[V0][0] {
        c.cycles += 2;
        c.g[V0][0] = 0;
        ret(c);
        return;
    }

    c.g[V1][0] = Addr(r32(c, A3)).off(LIST_VISIBLE_COUNT).lw(c);
    c.cycles += 3;
    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V1][0] as i64)) as u64;
    set_word(c, V1, r32(c, V1).wrapping_sub(1));
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        c.g[V0][0] = Addr(r32(c, A3)).off(LIST_SORT_DIRTY).lw(c);
        Addr(r32(c, A3)).off(LIST_VISIBLE_COUNT).sw(c, c.g[V1][0]);
        set_word(c, V0, r32(c, V0).wrapping_add(1));
        Addr(r32(c, A3)).off(LIST_SORT_DIRTY).sw(c, c.g[V0][0]);
    }
    c.cycles += 5;
    c.g[V0][0] = Addr(r32(c, A3)).off(LIST_ENTRY_COUNT).lw(c);
    set_word(c, V0, r32(c, V0).wrapping_sub(1));
    c.g[V1][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
    Addr(r32(c, A3)).off(LIST_ENTRY_COUNT).sw(c, c.g[V0][0]);
    if c.g[V1][0] != 0 {
        c.cycles += 1;
        c.g[V0][0] = Addr(r32(c, A3)).off(LIST_ENTRIES).lw(c);
        loop {
            c.cycles += 9;
            set_word(c, A0, r32(c, A2) << LIST_ENTRY_SHIFT);
            set_word(c, A2, r32(c, A2).wrapping_add(1));
            set_word(c, A0, r32(c, A0).wrapping_add(r32(c, V0)));
            c.g[V1][0] = Addr(r32(c, A0)).off(LIST_ENTRY_POINTER_BYTES).lw(c);
            Addr(r32(c, A0)).sw(c, c.g[V1][0]);
            c.g[V0][0] = Addr(r32(c, A3)).off(LIST_ENTRY_COUNT).lw(c);
            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = Addr(r32(c, A3)).off(LIST_ENTRIES).lw(c);
            if event_due(c) {
                deliver_event_here(c, SHIFT_HEAD);
            }
        }
    }
    c.cycles += 8;
    c.g[V1][0] = Addr(r32(c, A3)).off(LIST_FREE_HEAD).lw(c);
    c.g[V0][0] = Addr(r32(c, A1)).lw(c);
    Addr(r32(c, A1)).sw(c, c.g[V1][0]);
    Addr(r32(c, A3)).off(LIST_FREE_HEAD).sw(c, c.g[A1][0]);
    c.g[V1][0] = Addr(r32(c, A3)).off(LIST_FREE_COUNT).lw(c);
    set_word(c, V1, r32(c, V1).wrapping_add(1));
    Addr(r32(c, A3)).off(LIST_FREE_COUNT).sw(c, c.g[V1][0]);
    ret(c);
}

/// Returns the data carried by a lobby entry, or zero for a null entry.
pub fn lobby_list_get_data(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0;
    if c.g[A1][0] != 0 {
        c.cycles += 1;
        c.g[V0][0] = Addr(r32(c, A1)).lw(c);
    }
    c.cycles += 2;
    ret(c);
}

pub(crate) fn lobby_list_close_entry_frame(c: &mut Cpu, frame: Frame) {
    frame.restore(c, S0, LIST_ENTRY_SAVE_S0);
    frame.restore(c, S1, LIST_ENTRY_SAVE_S1);
    frame.restore(c, S2, LIST_ENTRY_SAVE_S2);
    frame.restore_ra(c, LIST_SAVE_RA);
    frame.close(c);
}

pub(crate) fn lobby_list_close_storage_frame(c: &mut Cpu, frame: Frame) {
    frame.restore(c, S0, LIST_STORAGE_SAVE_S0);
    frame.restore(c, S1, LIST_STORAGE_SAVE_S1);
    frame.restore(c, S2, LIST_STORAGE_SAVE_S2);
    frame.restore(c, S3, LIST_STORAGE_SAVE_S3);
    frame.restore_ra(c, LIST_SAVE_RA);
    frame.close(c);
}

/// Replaces the entry data and marks the visible list dirty if either value passes the filter.
pub fn lobby_list_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, LIST_ENTRY_FRAME);
        frame.save(c, S0, LIST_ENTRY_SAVE_S0);
        frame.save(c, S1, LIST_ENTRY_SAVE_S1);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, LIST_ENTRY_SAVE_S2);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, LIST_SAVE_RA);
        c.g[V0][0] = Addr(r32(c, S0)).off(LIST_FILTER).lw(c);
        c.g[S2][0] = c.g[A1][0];

        let mark = if c.g[V0][0] == 0 {
            c.cycles += 1;
            c.g[V0][0] = Addr(r32(c, S0)).off(LIST_SORT_DIRTY).lw(c);
            true
        } else {
            c.cycles += 6;
            c.g[A0][0] = Addr(r32(c, S0)).off(LIST_FILTER_ARG0).lw(c);
            c.g[A1][0] = Addr(r32(c, S0)).off(LIST_FILTER_ARG1).lw(c);
            let filter = r32(c, V0);
            c.g[A2][0] = Addr(r32(c, S2)).lw(c);
            call_ptr(c, filter, 0x003e_e418)?;
            if (c.g[V0][0] as i64) > 0 {
                c.g[V0][0] = Addr(r32(c, S0)).off(LIST_SORT_DIRTY).lw(c);
                true
            } else {
                c.cycles += 7;
                c.g[V0][0] = Addr(r32(c, S0)).off(LIST_FILTER).lw(c);
                c.g[A2][0] = c.g[S1][0];
                c.g[A0][0] = Addr(r32(c, S0)).off(LIST_FILTER_ARG0).lw(c);
                let filter = r32(c, V0);
                c.g[A1][0] = Addr(r32(c, S0)).off(LIST_FILTER_ARG1).lw(c);
                call_ptr(c, filter, 0x003e_e434)?;
                if (c.g[V0][0] as i64) > 0 {
                    c.cycles += 1;
                    c.g[V0][0] = Addr(r32(c, S0)).off(LIST_SORT_DIRTY).lw(c);
                    true
                } else {
                    false
                }
            }
        };
        if mark {
            c.cycles += 3;
            set_word(c, V0, r32(c, V0).wrapping_add(1));
            Addr(r32(c, S0)).off(LIST_SORT_DIRTY).sw(c, c.g[V0][0]);
        }
        Addr(r32(c, S2)).sw(c, c.g[S1][0]);

        c.cycles += 6;
        lobby_list_close_entry_frame(c, frame);
        Ok(())
    });
}

/// Adds an entry, growing the free list if necessary, and returns its handle.
pub fn lobby_list_add(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, LIST_ENTRY_FRAME);
        frame.save(c, S0, LIST_ENTRY_SAVE_S0);
        frame.save(c, S2, LIST_ENTRY_SAVE_S2);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, LIST_ENTRY_SAVE_S1);
        frame.save_ra(c, LIST_SAVE_RA);
        c.g[V0][0] = Addr(r32(c, S0)).off(LIST_FREE_HEAD).lw(c);
        c.g[S2][0] = c.g[A1][0];

        if c.g[V0][0] == 0 {
            c.cycles += 4;
            callee::LOBBY_LIST_GROW.call(c, 0x003e_e34c)?;
            if (c.g[V0][0] as i64) <= 0 {
                c.cycles += 2;
                c.g[V0][0] = 0;
                c.cycles += 6;
                lobby_list_close_entry_frame(c, frame);
                return Ok(());
            }
        } else {
            c.cycles += 1;
        }
        c.g[S1][0] = Addr(r32(c, S0)).off(LIST_FREE_HEAD).lw(c);

        c.cycles += 16;
        c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        Addr(r32(c, S0)).off(LIST_FREE_HEAD).sw(c, c.g[V0][0]);
        Addr(r32(c, S1)).sw(c, c.g[S2][0]);
        c.g[V0][0] = Addr(r32(c, S0)).off(LIST_ENTRY_COUNT).lw(c);
        c.g[V1][0] = Addr(r32(c, S0)).off(LIST_ENTRIES).lw(c);
        set_word(c, V0, r32(c, V0) << LIST_ENTRY_SHIFT);
        c.g[V0][0] = addu(c.g[V0][0], c.g[V1][0]);
        Addr(r32(c, V0)).sw(c, c.g[S1][0]);
        c.g[V0][0] = Addr(r32(c, S0)).off(LIST_ENTRY_COUNT).lw(c);
        c.g[V1][0] = Addr(r32(c, S0)).off(LIST_FREE_COUNT).lw(c);
        set_word(c, V0, r32(c, V0).wrapping_add(1));
        c.g[A3][0] = Addr(r32(c, S0)).off(LIST_FILTER).lw(c);
        set_word(c, V1, r32(c, V1).wrapping_sub(1));
        Addr(r32(c, S0)).off(LIST_ENTRY_COUNT).sw(c, c.g[V0][0]);
        Addr(r32(c, S0)).off(LIST_FREE_COUNT).sw(c, c.g[V1][0]);
        let mark = if c.g[A3][0] == 0 {
            true
        } else {
            c.cycles += 6;
            c.g[A0][0] = Addr(r32(c, S0)).off(LIST_FILTER_ARG0).lw(c);
            c.g[A1][0] = Addr(r32(c, S0)).off(LIST_FILTER_ARG1).lw(c);
            let filter = r32(c, A3);
            c.g[A2][0] = Addr(r32(c, S1)).lw(c);
            call_ptr(c, filter, 0x003e_e3b0)?;
            let visible = (c.g[V0][0] as i64) > 0;
            c.g[V0][0] = c.g[S1][0];
            visible
        };
        if mark {
            c.cycles += 4;
            c.g[V0][0] = Addr(r32(c, S0)).off(LIST_SORT_DIRTY).lw(c);
            set_word(c, V0, r32(c, V0).wrapping_add(1));
            Addr(r32(c, S0)).off(LIST_SORT_DIRTY).sw(c, c.g[V0][0]);
            c.g[V0][0] = c.g[S1][0];
        }

        c.cycles += 6;
        lobby_list_close_entry_frame(c, frame);
        Ok(())
    });
}

/// Releases the list's entry chunks, index array, and header.
pub fn lobby_list_destroy(c: &mut Cpu) {
    const DESTROY_HEAD: u32 = 0x003e_e280;
    const DESTROY_FRAME: u32 = 0x20;
    const DESTROY_SAVE_S0: u32 = 0x10;
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, DESTROY_FRAME);
        frame.save(c, S0, DESTROY_SAVE_S0);
        frame.save_ra(c, LIST_SAVE_RA);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, S0)).off(LIST_CHUNKS).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            c.g[A0][0] = Addr(r32(c, S0)).off(LIST_CHUNKS).lw(c);
            loop {
                c.cycles += 6;
                c.g[V0][0] = Addr(r32(c, A0)).lw(c);
                Addr(r32(c, S0)).off(LIST_CHUNKS).sw(c, c.g[V0][0]);
                callee::LOBBY_MEM_FREE.call(c, 0x003e_e28c)?;
                c.g[V0][0] = Addr(r32(c, S0)).off(LIST_CHUNKS).lw(c);
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A0][0] = Addr(r32(c, S0)).off(LIST_CHUNKS).lw(c);
                if event_due(c) {
                    deliver_event_here(c, DESTROY_HEAD);
                }
            }
        }

        c.cycles += 8;
        c.g[A0][0] = Addr(r32(c, S0)).off(LIST_ENTRIES).lw(c);
        callee::LOBBY_MEM_FREE.call(c, 0x003e_e2a0)?;
        c.g[A0][0] = c.g[S0][0];
        callee::LOBBY_MEM_FREE.call(c, 0x003e_e2a8)?;
        frame.restore(c, S0, DESTROY_SAVE_S0);
        frame.restore_ra(c, LIST_SAVE_RA);
        frame.close(c);
        Ok(())
    });
}

/// Allocates a lobby list and seeds its first chunk of reusable entries.
pub fn lobby_list_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, LIST_STORAGE_FRAME);
        frame.save(c, S1, LIST_STORAGE_SAVE_S1);
        frame.save(c, S2, LIST_STORAGE_SAVE_S2);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, LIST_STORAGE_SAVE_S3);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, LIST_STORAGE_SAVE_S0);
        c.g[S3][0] = c.g[A2][0];
        frame.save_ra(c, LIST_SAVE_RA);
        c.g[A0][0] = LIST_HEADER_BYTES;
        callee::LOBBY_MEM_ALLOC.call(c, 0x003e_e214)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = LIST_HEADER_BYTES;
        c.g[A0][0] = c.g[S0][0];
        callee::MEMSET.call(c, 0x003e_e228)?;
        Addr(r32(c, S0)).off(LIST_INITIAL_SIZE).sw(c, c.g[S1][0]);
        c.g[A0][0] = c.g[S0][0];
        callee::LOBBY_LIST_GROW.call(c, 0x003e_e234)?;
        Addr(r32(c, S0)).off(LIST_INITIAL_SIZE).sw(c, c.g[S2][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(r32(c, S0)).off(LIST_GROW_PERCENT).sw(c, c.g[S3][0]);
        lobby_list_close_storage_frame(c, frame);
        Ok(())
    });
}

/// Passes entry data and the stored sort arguments to the list's comparator.
pub fn lobby_list_compare_thunk(c: &mut Cpu) {
    const COMPARE_FRAME: u32 = 0x10;
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, COMPARE_FRAME);
        frame.save_ra(c, LIST_SAVE_RA);
        c.g[V0][0] = Addr(r32(c, A2)).lw(c);
        c.g[V1][0] = Addr(r32(c, A1)).lw(c);
        c.g[A3][0] = Addr(r32(c, V0)).lw(c);
        c.g[A1][0] = Addr(r32(c, A0)).off(LIST_SORT_ARG1).lw(c);
        c.g[V0][0] = Addr(r32(c, A0)).off(LIST_COMPARE).lw(c);
        c.g[A2][0] = Addr(r32(c, V1)).lw(c);
        let comparator = r32(c, V0);
        c.g[A0][0] = Addr(r32(c, A0)).off(LIST_SORT_ARG0).lw(c);
        call_ptr(c, comparator, 0x003e_e1d8)?;
        frame.restore_ra(c, LIST_SAVE_RA);
        frame.close(c);
        Ok(())
    });
}

pub(crate) const LIST_GROW_PERCENT_SCALE: u64 = 100;

pub(crate) const LIST_GROW_CHUNK_OVERHEAD: u32 = 2;

pub(crate) const LIST_GROW_INDEX_SHIFT: u32 = 3;

pub(crate) const LIST_GROW_HEAD: u32 = 0x003e_e128;

/// Allocates and links a chunk of free entries, then grows the list's index array.
pub fn lobby_list_grow(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, LIST_STORAGE_FRAME);
        frame.save(c, S2, LIST_STORAGE_SAVE_S2);
        frame.save(c, S0, LIST_STORAGE_SAVE_S0);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, LIST_STORAGE_SAVE_S1);
        frame.save(c, S3, LIST_STORAGE_SAVE_S3);
        frame.save_ra(c, LIST_SAVE_RA);
        c.g[A0][0] = Addr(r32(c, S2)).off(LIST_GROW_PERCENT).lw(c);
        if (c.g[A0][0] as i64) > 0 {
            c.g[A1][0] = Addr(r32(c, S2)).off(LIST_INITIAL_SIZE).lw(c);
            c.cycles += 2;
            c.g[V0][0] = Addr(r32(c, S2)).off(LIST_ENTRY_COUNT).lw(c);
            c.cycles += 3;
            c.g[V1][0] = LIST_GROW_PERCENT_SCALE;
            if c.g[A1][0] == 0 {
                c.cycles += 5;
                c.mult(0, V0, V0, V1);
                c.div(0, V0, A0);
                c.g[V0][0] = c.lo[0];
                c.g[S1][0] = c.g[V0][0];
            } else {
                c.cycles += 7;
                c.mult(0, V0, V0, V1);
                c.div(0, V0, A0);
                c.g[V0][0] = c.lo[0];
                c.g[V0][0] = addu(c.g[A1][0], c.g[V0][0]);
                set_word(c, V1, r32(c, V0) >> 31);
                c.g[V0][0] = addu(c.g[V0][0], c.g[V1][0]);
                set_word(c, S1, ((r32(c, V0) as i32) >> 1) as u32);
            }
        } else {
            c.cycles += 5;
            c.g[S1][0] = Addr(r32(c, S2)).off(LIST_INITIAL_SIZE).lw(c);
            c.g[V1][0] = 1;
            c.g[V0][0] = ((c.g[S1][0] as i64) > 0) as u64;
            if c.g[V0][0] == 0 {
                c.g[S1][0] = c.g[V1][0];
            }
        }

        c.cycles += 11;
        c.g[V0][0] = 1;
        c.g[V1][0] = 2;
        c.g[V0][0] = ((c.g[S1][0] as i64) > (c.g[V0][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            c.g[S1][0] = c.g[V1][0];
        }
        set_word(c, S1, r32(c, S1).wrapping_add(LIST_GROW_CHUNK_OVERHEAD));
        set_word(c, S3, r32(c, S1) << LIST_ENTRY_SHIFT);
        c.g[A0][0] = c.g[S3][0];
        callee::LOBBY_MEM_ALLOC.call(c, 0x003e_e0d4)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.cycles += 7;
            lobby_list_close_storage_frame(c, frame);
            return Ok(());
        }
        c.g[A0][0] = c.g[S0][0];

        c.cycles += 14;
        c.g[A1][0] = 0;
        c.g[A2][0] = c.g[S3][0];
        c.g[S1][0] = 1;
        callee::MEMSET.call(c, 0x003e_e0f8)?;
        c.g[A0][0] = Addr(r32(c, S2)).off(LIST_CHUNKS).lw(c);
        c.g[V0][0] = u64::MAX;
        c.g[V1][0] = addu(c.g[S3][0], c.g[S0][0]);
        Addr(r32(c, S0)).sw(c, c.g[A0][0]);
        Addr(r32(c, V1).wrapping_sub(LIST_ENTRY_POINTER_BYTES)).sw(c, c.g[V0][0]);
        Addr(r32(c, S2)).off(LIST_CHUNKS).sw(c, c.g[S0][0]);
        c.g[V0][0] = Addr(r32(c, S0)).off(LIST_ENTRY_POINTER_BYTES).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            set_word(c, A0, r32(c, S0).wrapping_add(LIST_ENTRY_POINTER_BYTES));
            loop {
                c.cycles += 7;
                c.g[V0][0] = Addr(r32(c, S2)).off(LIST_FREE_HEAD).lw(c);
                Addr(r32(c, A0)).sw(c, c.g[V0][0]);
                Addr(r32(c, S2)).off(LIST_FREE_HEAD).sw(c, c.g[A0][0]);
                set_word(c, A0, r32(c, A0).wrapping_add(LIST_ENTRY_POINTER_BYTES));
                c.g[V0][0] = Addr(r32(c, A0)).lw(c);
                set_word(c, S1, r32(c, S1).wrapping_add(1));
                if c.g[V0][0] != 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, LIST_GROW_HEAD);
                }
            }
            c.cycles += 1;
        }
        c.g[V0][0] = Addr(r32(c, S2)).off(LIST_FREE_COUNT).lw(c);

        c.cycles += 10;
        c.g[A0][0] = Addr(r32(c, S2)).off(LIST_ENTRY_COUNT).lw(c);
        set_word(c, V0, r32(c, V0).wrapping_sub(1));
        c.g[V0][0] = addu(c.g[V0][0], c.g[S1][0]);
        c.g[A0][0] = addu(c.g[A0][0], c.g[V0][0]);
        Addr(r32(c, S2)).off(LIST_FREE_COUNT).sw(c, c.g[V0][0]);
        set_word(c, A0, r32(c, A0) << LIST_GROW_INDEX_SHIFT);
        callee::LOBBY_MEM_ALLOC.call(c, 0x003e_e164)?;
        c.g[A1][0] = Addr(r32(c, S2)).off(LIST_ENTRIES).lw(c);
        c.g[S0][0] = c.g[V0][0];
        if c.g[A1][0] != 0 {
            c.cycles += 6;
            c.g[A2][0] = Addr(r32(c, S2)).off(LIST_ENTRY_COUNT).lw(c);
            c.g[A0][0] = c.g[S0][0];
            set_word(c, A2, r32(c, A2) << LIST_ENTRY_SHIFT);
            callee::MEMCPY.call(c, 0x003e_e180)?;
            c.g[A0][0] = Addr(r32(c, S2)).off(LIST_ENTRIES).lw(c);
            callee::LOBBY_MEM_FREE.call(c, 0x003e_e188)?;
        }
        c.cycles += 2;
        Addr(r32(c, S2)).off(LIST_ENTRIES).sw(c, c.g[S0][0]);
        set_word(c, V0, r32(c, S1).wrapping_sub(1));
        c.cycles += 7;
        lobby_list_close_storage_frame(c, frame);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Small lobby allocators and event forwarding.

/// `void *Lobby_MemAlloc(unsigned int size)` (0x2591e8): allocate `size`
/// bytes from the lobby heap (`operator_new` with the lobby heap tag).
pub fn lobby_mem_alloc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx(0x004a_2f28);
        frame.save_ra(c, 0);
        c.g[A2][0] = 0x20000000;
        c.g[A3][0] = 0u64;
        // A0 (size) passes through untouched.
        callee::OPERATOR_NEW_NAMED.call(c, 0x0025_9204)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Lobby_MemFree(void *ptr)` (0x259210): release a lobby-heap block,
/// ignoring null.
pub fn lobby_mem_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        let live = c.g[A0][0] != 0u64;
        frame.save_ra(c, 0);
        if live {
            c.cycles += 2;
            callee::EA_MEM_FREE.call(c, 0x0025_9224)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int Lobby_HashStrCmpNoCase(const char *a, const char *b)` (0x3eeab8):
/// compare two lobby strings case-insensitively through the fold table,
/// skipping characters that fold to 1; returns the first differing fold
/// difference, or 0 when both terminate together.
pub fn lobby_hash_str_cmp_no_case(c: &mut Cpu) {
    c.g[V0][0] = 0x490000;
    c.g[T0][0] = 0x1;
    c.g[T2][0] = c.g[V0][0];
    c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6a58));
    c.cycles += 4;
    'l_3eeac8: loop {
        c.cycles += 1;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).lbu(c);
        'l_3eeacc: loop {
            c.cycles += 6;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T1][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lbu(c);
            let end_a = c.g[V1][0] == c.g[T0][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            if end_a {
                if event_due(c) {
                    deliver_event_here(c, 0x003e_eac8);
                }
                continue 'l_3eeac8;
            }
            c.cycles += 3;
            c.g[A3][0] = sx((c.g[T2][0] as u32).wrapping_add(0x6a58));
            c.g[A2][0] = 0x1;
            'l_3eeaf0: loop {
                c.cycles += 7;
                c.g[V0][0] = Addr(c.g[A1][0] as u32).lbu(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lbu(c);
                let end_b = c.g[V0][0] == c.g[A2][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                if end_b {
                    if event_due(c) {
                        deliver_event_here(c, 0x003e_eaf0);
                    }
                    continue 'l_3eeaf0;
                }
                c.cycles += 3;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                if c.g[V0][0] != 0u64 {
                    c.cycles += 2;
                    ret(c);
                    return;
                }
                c.cycles += 2;
                if c.g[V1][0] != 0u64 {
                    c.g[V0][0] = Addr(c.g[A0][0] as u32).lbu(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x003e_eacc);
                    }
                    continue 'l_3eeacc;
                }
                c.cycles += 2;
                c.g[V0][0] = 0u64;
                ret(c);
                return;
            }
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_paths;
    use crate::difftest::check_paths_lohi;
    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes;
    use crate::difftest::fakes::fall_back;
    use crate::difftest::fakes::hook;
    use crate::difftest::fakes::prev_cell;
    use crate::difftest::fill;
    use crate::difftest::fresh;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::game::dirtysock::tagsunk::tests::mem_fakes_hook_with_new;
    use crate::game::dirtysock::tagsunk::tests::own_check_delivers;
    use crate::game::dirtysock::tagsunk::tests::strcmp_fake_on;
    use crate::game::dirtysock::tagsunk::tests::MEM_FAKES_CTRL;
    use crate::game::dirtysock::tagsunk::tests::MEM_FAKES_MAGIC;
    use core::ptr::addr_of_mut;
    use ssx3_ee as ee;

    pub(crate) const NET_CLOCK_TEST_GATE: u32 = DATA + 0xf7340;

    pub(crate) const NET_CLOCK_TEST_ORIGIN: u32 = NET_CLOCK_TEST_GATE + 4;

    pub(crate) const NET_CLOCK_TEST_MAGIC: u32 = u32::from_be_bytes(*b"TICK");

    pub(crate) const NET_CLOCK_TEST_QUANTUM: u64 = 64;

    pub(crate) const NET_CLOCK_CALL_CYCLES: u64 = 37;

    pub(crate) fn net_clock_fixture(c: &mut Cpu, seed: u32, entry: u32) -> Rng {
        let mut rng = Rng(0x1fbed47c ^ u64::from(seed));
        scramble(c, &mut rng);
        c.pc = entry;
        c.g[GP][0] = sx(GAME_GP);
        rng
    }

    pub(crate) fn net_tick_fake_division(c: &mut Cpu) {
        if c.bus.read32(NET_CLOCK_TEST_GATE) != NET_CLOCK_TEST_MAGIC {
            return fall_back(
                c,
                prev_cell(net_tick_fake_division),
                ee::s15::f_00411538__generated,
            );
        }
        let origin = u64::from(c.bus.read32(NET_CLOCK_TEST_ORIGIN));
        c.g[V0][0] = (c.cycles - origin) / NET_CLOCK_TEST_QUANTUM;
        ret(c);
    }

    pub(crate) fn net_tick_install_fake_clock() {
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_00411538),
            net_tick_fake_division,
            prev_cell(net_tick_fake_division),
        );
    }

    pub(crate) fn net_tick_enable_fake_clock(c: &mut Cpu) {
        c.bus.write32(NET_CLOCK_TEST_GATE, NET_CLOCK_TEST_MAGIC);
        c.bus.write32(NET_CLOCK_TEST_ORIGIN, c.cycles as u32);
    }

    pub(crate) fn net_tick_snapshot(c: &mut Cpu, seed: u32) {
        c.bus.write32(
            DIRTY_RPC_STATE + NET_TICK_COUNT,
            [0, 537, u32::MAX - 31][seed as usize % 3],
        );
        c.bus.write64(
            DIRTY_RPC_STATE + NET_TICK_EPOCH,
            [0, 0x0000_0001_f000_0000, u64::MAX - 127][seed as usize / 3 % 3],
        );
        c.cycles = u64::from(seed % 3) * 10_000;
    }

    #[test]
    fn net_tick_matches() {
        check(ee::s14::f_003f4de8__generated, net_tick, 12, &|c, seed| {
            net_clock_fixture(c, seed, 0x003f_4de8);
            net_tick_snapshot(c, seed);
        });
    }

    #[test]
    fn net_conn_return_true_matches() {
        check(
            ee::s14::f_003f43d0__generated,
            net_conn_return_true,
            12,
            &|c, seed| {
                net_clock_fixture(c, seed, 0x003f_43d0);
            },
        );
    }

    #[test]
    fn net_tick_wrapper2_matches() {
        check(
            ee::s14::f_003f4330__generated,
            net_tick_wrapper2,
            12,
            &|c, seed| {
                net_clock_fixture(c, seed, 0x003f_4330);
                net_tick_snapshot(c, seed);
            },
        );
    }

    pub(crate) fn net_conn_idle_fixture(c: &mut Cpu, seed: u32) {
        net_clock_fixture(c, seed, 0x003f_42a0);
        net_tick_enable_fake_clock(c);
        let next_idle = if seed % 3 == 2 { 1 } else { 0 };
        c.bus
            .write32(GAME_GP + NET_CONN_OPEN_GP, u32::from(seed % 3 != 0));
        c.bus.write32(GAME_GP + NET_CONN_NEXT_IDLE_GP, next_idle);
        if seed / 3 % 2 != 0 {
            c.bus.write32(NET_CONN_IDLE_CALLBACKS, 0x003f_43d0);
            c.bus.write32(
                NET_CONN_IDLE_CALLBACKS + NET_CONN_IDLE_CALLBACK_ARG,
                DATA + seed * 4,
            );
        }
    }

    #[test]
    fn net_conn_idle_matches() {
        net_tick_install_fake_clock();
        // Throttled, closed/open with no callbacks, closed/open with one callback.
        check_paths(
            ee::s14::f_003f42a0__generated,
            net_conn_idle,
            6,
            &[53, 115, 199, 173, 257],
            &net_conn_idle_fixture,
        );
    }

    driven!(generated_net_conn_idle, ee::s14::f_003f42a0__generated);

    driven!(hand_net_conn_idle, net_conn_idle);

    #[test]
    fn net_conn_idle_events() {
        net_tick_install_fake_clock();
        check_events(
            generated_net_conn_idle,
            hand_net_conn_idle,
            9,
            &|c, seed| {
                net_conn_idle_fixture(c, 3);
                let mut rng = Rng(0x0ac9_98ae ^ u64::from(seed));
                let mode = schedule(c, &mut rng, seed, 0x003f_42a0);
                net_tick_enable_fake_clock(c);
                if mode != Events::Off {
                    // During the idle table scan, after the callback has returned.
                    c.next_event = c.cycles + 95;
                }
                mode
            },
        );
    }

    pub(crate) fn net_conn_sleep_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let rng = net_clock_fixture(c, seed, NET_CONN_SLEEP_ENTRY);
        net_tick_enable_fake_clock(c);
        c.g[A0][0] = [u64::MAX, 1, 0][seed as usize % 3];
        rng
    }

    #[test]
    fn net_conn_sleep_matches() {
        net_tick_install_fake_clock();
        let once = 19 + 2 * NET_CLOCK_CALL_CYCLES;
        let loop_cost = 7 + 2 * NET_CLOCK_CALL_CYCLES;
        check_paths(
            ee::s14::f_003f41b8__generated,
            net_conn_sleep,
            9,
            &[10, once, once + 3 * loop_cost],
            &|c, seed| {
                net_conn_sleep_fixture(c, seed);
            },
        );
    }

    driven!(generated_net_conn_sleep, ee::s14::f_003f41b8__generated);

    driven!(hand_net_conn_sleep, net_conn_sleep);

    #[test]
    fn net_conn_sleep_events() {
        net_tick_install_fake_clock();
        let setup = |c: &mut Cpu, seed| {
            let mut rng = net_conn_sleep_fixture(c, 2 + 3 * seed);
            let mode = schedule(c, &mut rng, seed, NET_CONN_SLEEP_ENTRY);
            net_tick_enable_fake_clock(c);
            if mode != Events::Off {
                c.next_event = c.cycles + 85;
            }
            mode
        };
        require_lobby_resume(
            ee::s14::f_003f41b8__generated,
            net_conn_sleep,
            NET_CONN_SLEEP_HEAD,
            &|c| setup(c, 1),
        );
        check_events(generated_net_conn_sleep, hand_net_conn_sleep, 9, &setup);
    }

    #[test]
    fn net_tick_wrapper_matches() {
        check(
            ee::s14::f_003f4198__generated,
            net_tick_wrapper,
            12,
            &|c, seed| {
                net_clock_fixture(c, seed, 0x003f_4198);
                net_tick_snapshot(c, seed);
            },
        );
    }

    #[test]
    fn net_conn_shutdown_matches() {
        check_paths(
            ee::s14::f_003f4118__generated,
            net_conn_shutdown,
            8,
            &[8, 188],
            &|c, seed| {
                net_clock_fixture(c, seed, 0x003f_4118);
                c.bus.write32(GAME_GP + NET_CONN_OPEN_GP, seed % 2);
                c.bus.write32(GAME_GP + NET_CONN_HANDLE_GP, seed / 2);
            },
        );
    }

    #[test]
    fn net_conn_disconnect_matches() {
        check(
            ee::s14::f_003f40b8__generated,
            net_conn_disconnect,
            8,
            &|c, seed| {
                net_clock_fixture(c, seed, 0x003f_40b8);
                c.bus.write32(GAME_GP + NET_CONN_HANDLE_GP, seed);
            },
        );
    }

    pub(crate) const NET_STATUS_RPC_STARTED_GP: u32 = 0x1ac0;

    pub(crate) const NET_STATUS_TEST_BUFFER: u32 = DATA + 0x100;

    pub(crate) const DIRTY_RPC_COMMANDS_BEGIN: u32 = 0x4d88;

    pub(crate) const DIRTY_RPC_COMMANDS_END: u32 = 0x4d8c;

    pub(crate) fn net_status_no_rpc_commands(c: &mut Cpu) {
        c.bus.write32(DIRTY_RPC_STATE + DIRTY_RPC_COMMANDS_BEGIN, 0);
        c.bus.write32(DIRTY_RPC_STATE + DIRTY_RPC_COMMANDS_END, 0);
    }

    #[test]
    fn net_conn_mac_matches() {
        check_paths(
            ee::s14::f_003f3f78__generated,
            net_conn_mac,
            8,
            &[16, 203],
            &|c, seed| {
                let mut rng = net_clock_fixture(c, seed, 0x003f_3f78);
                elf_image(c);
                net_status_no_rpc_commands(c);
                let cache = NET_MAC_CACHE_BASE as u32 + NET_MAC_CACHE_OFFSET;
                fill(c, &mut rng, cache, 4);
                c.bus.write8(cache, if seed % 2 == 0 { 0 } else { b'$' });
                let bytes = frame_addr_for_fixture(c, NET_MAC_FRAME, 0);
                for index in 0..6 {
                    c.bus.write8(bytes + index, (seed * 31 + index * 37) as u8);
                }
            },
        );
    }

    pub(crate) fn frame_addr_for_fixture(c: &Cpu, frame_size: u32, offset: u32) -> u32 {
        r32(c, SP) - frame_size + offset
    }

    pub(crate) fn net_conn_status_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let mut rng = net_clock_fixture(c, seed, NET_CONN_STATUS_ENTRY);
        let selector = [
            NET_STATUS_OPEN,
            NET_STATUS_ONLINE,
            NET_STATUS_CONNECTION,
            NET_STATUS_HOST,
            NET_STATUS_ADDRESS,
            NET_STATUS_MAC,
            NET_STATUS_BROADBAND,
            NET_STATUS_TYPE,
            0,
        ][seed as usize % 9];
        c.g[A0][0] = selector;
        c.g[A1][0] = sx(NET_STATUS_TEST_BUFFER);
        c.g[A2][0] = 16;
        c.bus.write32(GAME_GP + NET_CONN_OPEN_GP, seed / 9);
        c.bus.write32(GAME_GP + NET_CONN_HANDLE_GP, seed / 9 + 1);
        fill(c, &mut rng, NET_STATUS_TEST_BUFFER, 4);
        let result = [0, 4, 0xffff_ffff][seed as usize / 9 % 3];
        let word = frame_addr_for_fixture(c, NET_CONN_STATUS_FRAME, NET_RPC_REPLY_WORD);
        let other = frame_addr_for_fixture(c, NET_CONN_STATUS_FRAME, NET_RPC_STATUS_WORD);
        c.bus.write32(word, result);
        c.bus.write32(other, result.wrapping_add(37));
        rng
    }

    #[test]
    fn net_conn_status_matches() {
        check_paths(
            ee::s14::f_003f3cc0__generated,
            net_conn_status,
            27,
            &[21, 113, 115, 119, 121, 123, 133, 130, 41],
            &|c, seed| {
                net_conn_status_fixture(c, seed);
            },
        );
    }

    driven!(generated_net_conn_status, ee::s14::f_003f3cc0__generated);

    driven!(hand_net_conn_status, net_conn_status);

    #[test]
    fn net_conn_status_events() {
        let setup = |c: &mut Cpu, seed| {
            let mut rng = net_conn_status_fixture(c, 6 + 9 * seed);
            let mode = schedule(c, &mut rng, seed, NET_CONN_STATUS_ENTRY);
            if mode != Events::Off {
                // After the request, during the broadband result rewrite.
                c.next_event = c.cycles + 121;
            }
            mode
        };
        require_lobby_resume(
            ee::s14::f_003f3cc0__generated,
            net_conn_status,
            NET_CONN_STATUS_BROADBAND_HEAD,
            &|c| setup(c, 1),
        );
        check_events(generated_net_conn_status, hand_net_conn_status, 9, &setup);
    }

    #[test]
    fn net_conn_connect_matches() {
        check(
            ee::s14::f_003f3c50__generated,
            net_conn_connect,
            8,
            &|c, seed| {
                net_clock_fixture(c, seed, 0x003f_3c50);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(GAME_GP + NET_CONN_HANDLE_GP, seed);
                let word = frame_addr_for_fixture(c, 0x50, NET_RPC_REPLY_WORD);
                c.bus.write32(word, seed.wrapping_sub(2));
            },
        );
    }

    #[test]
    fn net_conn_query_matches() {
        socket_lookup_install_fakes();
        check(
            ee::s14::f_003f3bc8__generated,
            net_conn_query,
            8,
            &|c, seed| {
                let mut rng = net_clock_fixture(c, seed, 0x003f_3bc8);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(NET_STATUS_TEST_BUFFER);
                c.g[A2][0] = u64::from(seed % 3);
                fill(c, &mut rng, NET_STATUS_TEST_BUFFER, 8);
                c.bus.write32(GAME_GP + NET_CONN_HANDLE_GP, seed);
                let word = frame_addr_for_fixture(c, 0x70, NET_RPC_REPLY_WORD);
                c.bus.write32(word, seed.wrapping_sub(2));
                // `DirtyRpc_Call` only reaches its send path with a registered command, so the
                // call is logged instead, with every register it receives.
                c.bus.write32(RPC_CALL_LOG_GATE, LOOKUP_TEST_MAGIC);
                c.bus.write32(fakes::LOG_UNWIND_AT, u32::MAX);
            },
        );
    }

    pub(crate) const STARTUP_FAKE_GATE: u32 = DATA + 0xf7380;

    pub(crate) const STARTUP_FAKE_MAGIC: u32 = 0x003f_3b18;

    /// Charges the lookup fakes' cycle cost on purpose: both stand in for a short guest call.
    pub(crate) fn startup_fake_call(c: &mut Cpu, address: u32) {
        let cycles = c.cycles;
        let return_pc = r32(c, RA);
        fakes::fake(c, address, return_pc);
        c.cycles = cycles + LOOKUP_FAKE_CYCLES;
    }

    pub(crate) fn startup_fake_dirty_rpc_startup(c: &mut Cpu) {
        if c.bus.read32(STARTUP_FAKE_GATE) != STARTUP_FAKE_MAGIC {
            return fall_back(
                c,
                prev_cell(startup_fake_dirty_rpc_startup),
                ee::s14::f_003f5000__generated,
            );
        }
        startup_fake_call(c, 0x003f_5000);
    }

    pub(crate) fn startup_fake_dirty_rpc_init_iop(c: &mut Cpu) {
        if c.bus.read32(STARTUP_FAKE_GATE) != STARTUP_FAKE_MAGIC {
            return fall_back(
                c,
                prev_cell(startup_fake_dirty_rpc_init_iop),
                ee::s14::f_003f4d68__generated,
            );
        }
        startup_fake_call(c, 0x003f_4d68);
    }

    #[test]
    fn net_conn_startup_matches() {
        hook(
            addr_of_mut!(ee::s14::OVERRIDE_003F5000),
            startup_fake_dirty_rpc_startup,
            prev_cell(startup_fake_dirty_rpc_startup),
        );
        hook(
            addr_of_mut!(ee::s14::OVERRIDE_003F4D68),
            startup_fake_dirty_rpc_init_iop,
            prev_cell(startup_fake_dirty_rpc_init_iop),
        );
        check_paths(
            ee::s14::f_003f3b18__generated,
            net_conn_startup,
            16,
            &[14, 106, 119],
            &|c, seed| {
                net_clock_fixture(c, seed, 0x003f_3b18);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(GAME_GP + NET_CONN_OPEN_GP, seed % 2);
                // Half of the seeds find the RPC service not yet started, which brings it up
                // through `DirtyRpc_Startup` and `DirtyRpc_InitIop`, both faked.
                let started = seed / 8 % 2;
                c.bus.write32(GAME_GP + NET_STATUS_RPC_STARTED_GP, started);
                c.bus.write32(STARTUP_FAKE_GATE, STARTUP_FAKE_MAGIC);
                c.bus.write32(fakes::LOG_UNWIND_AT, u32::MAX);
            },
        );
    }

    pub(crate) const LOOKUP_TEST_GATE: u32 = DATA + 0xf7350;

    pub(crate) const LOOKUP_TEST_ALLOC: u32 = LOOKUP_TEST_GATE + 4;

    pub(crate) const LOOKUP_TEST_STATUS: u32 = LOOKUP_TEST_GATE + 8;

    pub(crate) const LOOKUP_TEST_MAGIC: u32 = u32::from_be_bytes(*b"LOOK");

    pub(crate) const LOOKUP_TEST_OBJECT: u32 = DATA + 0x200;

    pub(crate) const LOOKUP_FAKE_CYCLES: u64 = 4;

    pub(crate) fn lookup_log_call(c: &mut Cpu, address: u32) {
        let cycles = c.cycles;
        let return_pc = r32(c, RA);
        fakes::fake(c, address, return_pc);
        c.cycles = cycles + LOOKUP_FAKE_CYCLES;
    }

    pub(crate) fn lookup_fake_allocate(c: &mut Cpu) {
        if c.bus.read32(LOOKUP_TEST_GATE) != LOOKUP_TEST_MAGIC {
            return fall_back(
                c,
                prev_cell(lookup_fake_allocate),
                ee::s10::f_00317e30__generated,
            );
        }
        lookup_log_call(c, 0x0031_7e30);
        c.g[V0][0] = sx(c.bus.read32(LOOKUP_TEST_ALLOC));
    }

    pub(crate) fn lookup_fake_free(c: &mut Cpu) {
        if c.bus.read32(LOOKUP_TEST_GATE) != LOOKUP_TEST_MAGIC {
            return fall_back(
                c,
                prev_cell(lookup_fake_free),
                ee::s10::f_00317e98__generated,
            );
        }
        lookup_log_call(c, 0x0031_7e98);
    }

    pub(crate) fn lookup_finish_after_memset(c: &mut Cpu) {
        let object = r32(c, A0);
        let called_for_lookup = c.bus.read32(LOOKUP_TEST_GATE) == LOOKUP_TEST_MAGIC
            && object == LOOKUP_TEST_OBJECT
            && r32(c, A2) == LOOKUP_SIZE;
        let return_pc = r32(c, RA);
        fall_back(
            c,
            prev_cell(lookup_finish_after_memset),
            ee::s15::f_00416210__generated,
        );
        if called_for_lookup && c.pc == return_pc {
            let status = c.bus.read32(LOOKUP_TEST_STATUS);
            if status != 0 {
                // Supply the reply an IOP lookup would leave in the object.
                c.bus.write32(object + LOOKUP_STATUS, status);
                c.bus.write32(object + LOOKUP_ADDRESS, 0x7f00_0001);
            }
        }
    }

    pub(crate) const LOOKUP_TEST_PENDING: u32 = LOOKUP_TEST_GATE + 12;

    pub(crate) const LOOKUP_TEST_POLLS: u32 = LOOKUP_TEST_GATE + 16;

    pub(crate) const RPC_CALL_LOG_GATE: u32 = LOOKUP_TEST_GATE + 20;

    pub(crate) const LOOKUP_TEST_EVENT: u32 = LOOKUP_TEST_GATE + 24;

    /// The return address of the status request in `SocketLookup_Done`, the completion poll.
    pub(crate) const LOOKUP_DONE_POLL_RA: u32 = 0x003f_38d0;

    /// Stands in for `DirtyRpc_Call`: with `RPC_CALL_LOG_GATE` set it only logs the call's
    /// registers, so a test can see every argument; with `LOOKUP_TEST_PENDING` set it also answers
    /// the first completion poll with "not yet" (status 0) and every later one with a finished
    /// lookup.
    pub(crate) fn lookup_fake_rpc_call(c: &mut Cpu) {
        let pending = c.bus.read32(LOOKUP_TEST_GATE) == LOOKUP_TEST_MAGIC
            && c.bus.read32(LOOKUP_TEST_PENDING) == 1;
        if !pending && c.bus.read32(RPC_CALL_LOG_GATE) != LOOKUP_TEST_MAGIC {
            return fall_back(
                c,
                prev_cell(lookup_fake_rpc_call),
                ee::s14::f_003f4c68__generated,
            );
        }
        let from_poll = r32(c, RA) == LOOKUP_DONE_POLL_RA;
        lookup_log_call(c, 0x003f_4c68);
        if !pending || !from_poll {
            return;
        }
        let polls = c.bus.read32(LOOKUP_TEST_POLLS);
        c.bus.write32(LOOKUP_TEST_POLLS, polls + 1);
        if polls == 0 {
            c.bus.write32(LOOKUP_TEST_OBJECT + LOOKUP_STATUS, 0);
            if c.bus.read32(LOOKUP_TEST_EVENT) == 1 {
                // Make the event due as the first poll returns, so the retry's own check takes it.
                c.next_event = c.cycles;
            }
        } else {
            c.bus.write32(LOOKUP_TEST_OBJECT + LOOKUP_STATUS, 1);
            c.bus
                .write32(LOOKUP_TEST_OBJECT + LOOKUP_ADDRESS, 0x7f00_0001);
        }
    }

    pub(crate) fn socket_lookup_install_fakes() {
        hook(
            addr_of_mut!(ee::s14::OVERRIDE_003F4C68),
            lookup_fake_rpc_call,
            prev_cell(lookup_fake_rpc_call),
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            lookup_fake_allocate,
            prev_cell(lookup_fake_allocate),
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            lookup_fake_free,
            prev_cell(lookup_fake_free),
        );
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_00416210),
            lookup_finish_after_memset,
            prev_cell(lookup_finish_after_memset),
        );
    }

    pub(crate) fn socket_lookup_fixture(c: &mut Cpu, seed: u32, entry: u32, allocate: bool) {
        let mut rng = net_clock_fixture(c, seed, entry);
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = u64::from(seed % 3);
        c.bus.write8(DATA, b'x');
        c.bus.write8(DATA + 1, 0);
        c.bus.write32(LOOKUP_TEST_GATE, LOOKUP_TEST_MAGIC);
        c.bus.write32(
            LOOKUP_TEST_ALLOC,
            if allocate { LOOKUP_TEST_OBJECT } else { 0 },
        );
        c.bus.write32(fakes::LOG_UNWIND_AT, u32::MAX);
        fill(c, &mut rng, LOOKUP_TEST_OBJECT, LOOKUP_SIZE / 4);
    }

    #[test]
    fn socket_lookup_sync_matches() {
        socket_lookup_install_fakes();
        check_paths(
            ee::s14::f_003f3ab8__generated,
            socket_lookup_sync,
            12,
            &[49, 469, 555, 446],
            &|c, seed| {
                let mode = seed % 3;
                socket_lookup_fixture(c, seed, 0x003f_3ab8, mode != 0);
                c.bus
                    .write32(LOOKUP_TEST_STATUS, [0, u32::MAX, 1][mode as usize]);
                // The later seeds' first completion poll answers "not yet", so the lookup is
                // polled a second time.
                if seed >= 6 && mode != 0 {
                    c.bus.write32(LOOKUP_TEST_PENDING, 1);
                    c.bus.write32(LOOKUP_TEST_POLLS, 0);
                }
            },
        );
    }

    driven!(generated_socket_lookup_sync, ee::s14::f_003f3ab8__generated);

    driven!(hand_socket_lookup_sync, socket_lookup_sync);

    #[test]
    fn socket_lookup_sync_events() {
        socket_lookup_install_fakes();
        check_events(
            generated_socket_lookup_sync,
            hand_socket_lookup_sync,
            6,
            &|c, seed| {
                socket_lookup_fixture(c, seed, 0x003f_3ab8, true);
                c.bus.write32(LOOKUP_TEST_STATUS, 1);
                c.bus.write32(LOOKUP_TEST_PENDING, 1);
                c.bus.write32(LOOKUP_TEST_POLLS, 0);
                let mut rng = Rng(0x5a1c_0001 ^ u64::from(seed));
                let mode = schedule(c, &mut rng, seed, 0x003f_3ab8);
                if mode != Events::Off {
                    // The event comes due only when the first poll returns (see the fake), not
                    // inside the lookup's creation, which cannot be resumed.
                    c.next_event = c.cycles + 1_000_000;
                    c.bus.write32(LOOKUP_TEST_EVENT, 1);
                }
                mode
            },
        );
    }

    #[test]
    fn socket_lookup_matches() {
        socket_lookup_install_fakes();
        check_paths(
            ee::s14::f_003f3a98__generated,
            socket_lookup,
            6,
            &[42, 233],
            &|c, seed| {
                socket_lookup_fixture(c, seed, 0x003f_3a98, seed % 2 == 1);
            },
        );
    }

    #[test]
    fn socket_lookup_create_matches() {
        socket_lookup_install_fakes();
        check_paths(
            ee::s14::f_003f39c8__generated,
            socket_lookup_create,
            6,
            &[35, 226],
            &|c, seed| {
                socket_lookup_fixture(c, seed, 0x003f_39c8, seed % 2 == 1);
            },
        );
    }

    #[test]
    fn socket_lookup_free_matches() {
        socket_lookup_install_fakes();
        check(
            ee::s14::f_003f3940__generated,
            socket_lookup_free,
            6,
            &|c, seed| {
                socket_lookup_fixture(c, seed, 0x003f_3940, false);
                c.g[A0][0] = sx(LOOKUP_TEST_OBJECT);
                c.bus
                    .write32(LOOKUP_TEST_OBJECT + LOOKUP_REMOTE_HANDLE, seed);
            },
        );
    }

    #[test]
    fn socket_lookup_done_matches() {
        check_paths(
            ee::s14::f_003f3860__generated,
            socket_lookup_done,
            6,
            &[106, 192],
            &|c, seed| {
                let mut rng = net_clock_fixture(c, seed, 0x003f_3860);
                c.g[A0][0] = sx(LOOKUP_TEST_OBJECT);
                fill(c, &mut rng, LOOKUP_TEST_OBJECT, LOOKUP_SIZE / 4);
                c.bus
                    .write32(LOOKUP_TEST_OBJECT + LOOKUP_REMOTE_HANDLE, seed / 3);
                c.bus.write32(
                    LOOKUP_TEST_OBJECT + LOOKUP_STATUS,
                    [0, u32::MAX, 1][seed as usize % 3],
                );
            },
        );
    }

    /// STATUS 0 takes the `V1 <= 0` path, where `A1` (service tag masked
    /// with the prefix mask) survives to return. `check` under
    /// args-strictness ignores caller-saved regs, so this asserts `A1`
    /// directly (`triaged G rows`: `&`/`^` on the mask).
    #[test]
    fn socket_lookup_done_masked_service_matches() {
        let mut c = fresh();
        let mut rng = net_clock_fixture(&mut c, 1, 0x003f_3860);
        c.pc = 0x003f_3860;
        c.g[A0][0] = sx(LOOKUP_TEST_OBJECT);
        fill(&mut c, &mut rng, LOOKUP_TEST_OBJECT, LOOKUP_SIZE / 4);
        c.bus.write32(LOOKUP_TEST_OBJECT + LOOKUP_REMOTE_HANDLE, 1);
        c.bus.write32(LOOKUP_TEST_OBJECT + LOOKUP_STATUS, 0);
        socket_lookup_done(&mut c);
        assert_eq!(
            c.g[A1][0],
            LOOKUP_RPC_SERVICE & SELECTOR_PREFIX_MASK,
            "a1 must carry the masked lookup service tag"
        );
    }

    pub(crate) const CIPHER_TEST_INPUT: u32 = DATA + 0x300;

    pub(crate) const CIPHER_TEST_OUTPUT: u32 = DATA + 0x400;

    pub(crate) const CIPHER_DECRYPT_ENTRY: u32 = 0x003f_3748;

    pub(crate) const CIPHER_ENCRYPT_ENTRY: u32 = 0x003f_35d0;

    pub(crate) fn lobby_cipher_string_fixture(c: &mut Cpu, seed: u32, entry: u32) -> Rng {
        let rng = net_clock_fixture(c, seed, entry);
        elf_image(c);
        // Each call initializes its own permutation. Keep the one-time
        // process-wide padding initializer out of the per-string fixtures.
        c.bus.write32(GAME_GP + LOBBY_CIPHER_INITIALIZED_GP, 1);
        c.g[A0][0] = sx(CIPHER_TEST_OUTPUT);
        c.g[A1][0] = u64::from(seed % 5);
        c.g[A2][0] = sx(CIPHER_TEST_INPUT);
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        for offset in 0..8 {
            c.bus.write8(CIPHER_TEST_OUTPUT + offset, 0xa5);
        }
        rng
    }

    pub(crate) fn lobby_decrypt_string_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let rng = lobby_cipher_string_fixture(c, seed, CIPHER_DECRYPT_ENTRY);
        // With the identity permutation the first keystream byte is zero.
        // A DEL input therefore reaches the decoded end-marker branch.
        let input = [b"\0\0", b"\x7f\0", b"A\0", b"AB"][seed as usize / 5 % 4];
        for (offset, &byte) in input.iter().enumerate() {
            c.bus.write8(CIPHER_TEST_INPUT + offset as u32, byte);
        }
        c.bus.write8(CIPHER_TEST_INPUT + 2, 0);
        rng
    }

    #[test]
    fn lobby_decrypt_string_matches() {
        check_paths_lohi(
            ee::s14::f_003f3748__generated,
            lobby_decrypt_string,
            20,
            &[1856, 1860, 1922, 1925, 1929, 1990, 1994],
            &|c, seed| {
                lobby_decrypt_string_fixture(c, seed);
            },
        );
    }

    driven!(
        generated_lobby_decrypt_string,
        ee::s14::f_003f3748__generated
    );

    driven!(hand_lobby_decrypt_string, lobby_decrypt_string);

    #[test]
    fn lobby_decrypt_string_events() {
        check_events(
            generated_lobby_decrypt_string,
            hand_lobby_decrypt_string,
            9,
            &|c, seed| {
                let mut rng = lobby_decrypt_string_fixture(c, 19);
                let mode = schedule(c, &mut rng, seed, CIPHER_DECRYPT_ENTRY);
                if mode != Events::Off {
                    c.next_event = c.cycles + 1910;
                }
                mode
            },
        );
    }

    pub(crate) fn lobby_encrypt_string_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let rng = lobby_cipher_string_fixture(c, seed, CIPHER_ENCRYPT_ENTRY);
        match seed / 5 % 4 {
            0 => c.g[A2][0] = 0,
            1 => c.bus.write8(CIPHER_TEST_INPUT, 0),
            2 => {
                c.bus.write8(CIPHER_TEST_INPUT, b'A');
                c.bus.write8(CIPHER_TEST_INPUT + 1, 0xff);
            }
            _ => {
                c.bus.write8(CIPHER_TEST_INPUT, b'A');
                c.bus.write8(CIPHER_TEST_INPUT + 1, b'B');
            }
        }
        c.bus.write8(CIPHER_TEST_INPUT + 2, 0);
        rng
    }

    #[test]
    fn lobby_encrypt_string_matches() {
        // A byte from input costs 71 cycles and a padding byte costs 116.
        // All runs start with the same three cipher initializations.
        const NO_OUTPUT: u64 = 11750;
        const INPUT_BYTE: u64 = 71;
        const PAD_BYTE: u64 = 116;
        const LOOP_ENTRY: u64 = 1;
        check_paths_lohi(
            ee::s14::f_003f35d0__generated,
            lobby_encrypt_string,
            20,
            &[
                NO_OUTPUT,
                NO_OUTPUT + LOOP_ENTRY + INPUT_BYTE,
                NO_OUTPUT + LOOP_ENTRY + PAD_BYTE,
                NO_OUTPUT + LOOP_ENTRY + 2 * INPUT_BYTE,
                NO_OUTPUT + LOOP_ENTRY + INPUT_BYTE + PAD_BYTE,
                NO_OUTPUT + LOOP_ENTRY + 2 * PAD_BYTE,
            ],
            &|c, seed| {
                lobby_encrypt_string_fixture(c, seed);
            },
        );
    }

    driven!(
        generated_lobby_encrypt_string,
        ee::s14::f_003f35d0__generated
    );

    driven!(hand_lobby_encrypt_string, lobby_encrypt_string);

    #[test]
    fn lobby_encrypt_string_events() {
        check_events(
            generated_lobby_encrypt_string,
            hand_lobby_encrypt_string,
            9,
            &|c, seed| {
                let mut rng = lobby_encrypt_string_fixture(c, 19);
                let mode = schedule(c, &mut rng, seed, CIPHER_ENCRYPT_ENTRY);
                if mode != Events::Off {
                    c.next_event = c.cycles + 11800;
                }
                mode
            },
        );
    }

    pub(crate) const MD5_TRANSFORM_ENTRY: u32 = 0x003f_2220;

    pub(crate) fn md5_transform_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let mut rng = Rng(0x6d64_3500 ^ u64::from(seed));
        scramble(c, &mut rng);
        let frame_start = crate::difftest::SP - MD5_FRAME_BYTES;
        fill(c, &mut rng, frame_start, MD5_FRAME_BYTES / MD5_WORD_BYTES);
        let context = match seed % 5 {
            0 => DATA,
            // Feed-forward writes alias words that a later step still reads.
            1 => frame_start + 2 * MD5_WORD_BYTES - MD5_STATE_A,
            2 => frame_start + 9 * MD5_WORD_BYTES - MD5_STATE_C,
            // The context aliases a register save or the constant spill area.
            3 => frame_start + MD5_SAVE_S3 - MD5_STATE_A,
            _ => frame_start + MD5_K44_SLOT - MD5_STATE_B,
        };
        fill(
            c,
            &mut rng,
            context,
            (MD5_BLOCK + MD5_BLOCK_BYTES) / MD5_WORD_BYTES,
        );
        set_word(c, A0, context);
        c.pc = MD5_TRANSFORM_ENTRY;
        rng
    }

    #[test]
    fn crypt_md5_transform_matches() {
        check_paths(
            ssx3_ee::s14::f_003f2220__generated,
            crypt_md5_transform,
            80,
            &[1217],
            &|c, seed| {
                md5_transform_fixture(c, seed);
            },
        );
    }

    pub(crate) fn md5_transform_with_event_check(c: &mut Cpu) {
        let due = c.next_event;
        crypt_md5_transform(c);
        if due <= c.cycles {
            assert_ne!(c.next_event, due, "the block copy should deliver the event");
        }
    }

    #[test]
    fn crypt_md5_transform_events() {
        check_events(
            ssx3_ee::s14::f_003f2220__generated,
            md5_transform_with_event_check,
            45,
            &|c, seed| {
                let mut rng = md5_transform_fixture(c, seed / 3);
                let mode = schedule(c, &mut rng, seed, MD5_TRANSFORM_ENTRY);
                if mode != Events::Off {
                    // The first event falls inside one of the 15 back-edge intervals.
                    let word = 1 + u64::from(seed / 3);
                    c.next_event = c.cycles + 14 + word * 18 - 1;
                }
                mode
            },
        );
    }

    #[test]
    fn crypt_md5_transform_known_answers() {
        let vectors: [(&[u8], [u32; 4]); 3] = [
            (b"", [0xd98c_1dd4, 0x04b2_008f, 0x9809_80e9, 0x7e42_f8ec]),
            (b"abc", [0x9850_0190, 0xb04f_d23c, 0x7d3f_96d6, 0x727f_e128]),
            (
                b"12345678901234567890123456789012345678901234567890123456789012345678901234567890",
                [0xa2f4_ed57, 0x55c9_e32b, 0x2eda_49ac, 0x7ab6_0721],
            ),
        ];
        for (message, expected) in vectors {
            let mut c = fresh();
            put_words(&mut c, DATA + MD5_STATE_A, &MD5_INITIAL_STATE);

            // Pad with a one bit, then zeros, then the original bit length.
            let block_bytes = MD5_BLOCK_BYTES as usize;
            let bit_length = ((message.len() as u64) * 8).to_le_bytes();
            let mut padded = message.to_vec();
            padded.push(0x80);
            let length_offset = (padded.len() + bit_length.len()).div_ceil(block_bytes)
                * block_bytes
                - bit_length.len();
            padded.resize(length_offset, 0);
            padded.extend_from_slice(&bit_length);

            for block in padded.chunks_exact(block_bytes) {
                for (index, &byte) in block.iter().enumerate() {
                    c.bus.write8(DATA + MD5_BLOCK + index as u32, byte);
                }
                set_word(&mut c, A0, DATA);
                c.pc = MD5_TRANSFORM_ENTRY;
                crypt_md5_transform(&mut c);
            }
            let digest = [MD5_STATE_A, MD5_STATE_B, MD5_STATE_C, MD5_STATE_D]
                .map(|offset| c.bus.read32(DATA + offset));
            assert_eq!(digest, expected, "message {message:?}");
        }
    }

    pub(crate) const TEXT_OUTPUT: u32 = DATA + 0x2000;

    pub(crate) const TEXT_INPUT: u32 = DATA + 0x3000;

    pub(crate) const TEXT_TABLE: u32 = DATA + 0x4000;

    pub(crate) fn fill_bytes(c: &mut Cpu, address: u32, bytes: &[u8]) {
        for (offset, &byte) in bytes.iter().enumerate() {
            c.bus.write8(address + offset as u32, byte);
        }
    }

    pub(crate) fn prepare_text(c: &mut Cpu, pc: u32, seed: u32) -> Rng {
        let mut rng = Rng(0x4221_4ab8_c95d_cb03 ^ seed as u64);
        scramble(c, &mut rng);
        c.g[GP][0] = sx(GAME_GP);
        c.pc = pc;
        fill_bytes(c, TEXT_OUTPUT, &[0xcd; 96]);
        rng
    }

    pub(crate) fn prepare_cipher(c: &mut Cpu, pc: u32, seed: u32) -> Rng {
        let rng = prepare_text(c, pc, seed);
        for index in 0..LOBBY_CIPHER_BYTES {
            // This is the standard reflected CRC-32 lookup used by the cipher.
            let crc = (0..8).fold(index, |crc, _| {
                (crc >> 1) ^ if crc & 1 == 0 { 0 } else { 0xedb8_8320 }
            });
            c.bus.write32(LOBBY_CIPHER_CRC_TABLE + index * 4, crc);
            // A scrambled permutation, so that XOR and OR of its entries differ.
            c.bus.write8(
                DATA + index,
                (index as u8).wrapping_mul(37).wrapping_add(11),
            );
        }
        c.bus.write8(DATA + LOBBY_CIPHER_CURSOR, 0xff);
        c.bus.write32(DATA + LOBBY_CIPHER_ACCUMULATOR, 0x846f_ba31);
        c.g[A0][0] = sx(DATA);
        rng
    }

    pub(crate) fn lobby_cipher_apply_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let rng = prepare_cipher(c, 0x003f_3530, seed);
        c.g[A1][0] = sx(if seed / 4 % 2 == 0 {
            TEXT_OUTPUT
        } else {
            DATA + 4
        });
        c.g[A2][0] = sx([u32::MAX, 0, 1, 4][(seed % 4) as usize]);
        rng
    }

    #[test]
    fn lobby_cipher_apply_matches() {
        check_paths(
            ee::s14::f_003f3530__generated,
            lobby_cipher_apply,
            8,
            &[9, 40, 127],
            &|c, seed| {
                lobby_cipher_apply_fixture(c, seed);
            },
        );
    }

    driven!(
        lobby_cipher_apply_generated_driven,
        ee::s14::f_003f3530__generated
    );

    driven!(lobby_cipher_apply_hand_driven, lobby_cipher_apply);

    #[test]
    fn lobby_cipher_apply_events() {
        check_events(
            lobby_cipher_apply_generated_driven,
            lobby_cipher_apply_hand_driven,
            9,
            &|c, seed| {
                let mut rng = lobby_cipher_apply_fixture(c, 3);
                let mode = schedule(c, &mut rng, seed, 0x003f_3530);
                if mode != Events::Off {
                    c.next_event = c.cycles + 33;
                }
                mode
            },
        );
    }

    pub(crate) fn lobby_cipher_init_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let rng = prepare_cipher(c, 0x003f_33b0, seed);
        let (key, length, rounds) = match seed % 7 {
            0 | 6 => (b"".as_slice(), 0, 0),
            1 => (b"abc".as_slice(), 3, 0),
            2 => (b"".as_slice(), -1, 0),
            3 => (b"abc".as_slice(), -1, 0),
            4 => (b"abc".as_slice(), 3, -1),
            _ => (b"abc".as_slice(), 3, 1),
        };
        c.bus.write32(
            GAME_GP + LOBBY_CIPHER_INITIALIZED_GP,
            (seed % 7 != 6) as u32,
        );
        fill_bytes(c, TEXT_INPUT, key);
        // Synthetic non-empty key for the cold-init recursion.
        fill_bytes(c, LOBBY_CIPHER_STATIC_KEY, b"x\0");
        c.g[A1][0] = sx(TEXT_INPUT);
        c.g[A2][0] = sx(length as u32);
        c.g[A3][0] = sx(rounds as u32);
        rng
    }

    #[test]
    fn lobby_cipher_init_matches() {
        check_paths_lohi(
            ee::s14::f_003f33b0__generated,
            lobby_cipher_init,
            7,
            &[1819, 1824, 1822, 1850, 7970, 9763, 83027],
            &|c, seed| {
                lobby_cipher_init_fixture(c, seed);
            },
        );
    }

    driven!(
        lobby_cipher_init_generated_driven,
        ee::s14::f_003f33b0__generated
    );

    driven!(lobby_cipher_init_hand_driven, lobby_cipher_init);

    #[test]
    fn lobby_cipher_init_events() {
        check_events(
            lobby_cipher_init_generated_driven,
            lobby_cipher_init_hand_driven,
            9,
            &|c, seed| {
                let (fixture, delay) = [(0, 18), (3, 1818), (4, 54)][(seed / 3) as usize];
                let mut rng = lobby_cipher_init_fixture(c, fixture);
                let mode = schedule(c, &mut rng, seed, 0x003f_33b0);
                if mode != Events::Off {
                    c.next_event = c.cycles + delay;
                }
                mode
            },
        );
    }

    pub(crate) fn prepare_md5(c: &mut Cpu, pc: u32, seed: u32, count: u32) -> Rng {
        let mut rng = prepare_text(c, pc, seed);
        c.g[A0][0] = sx(DATA);
        c.bus.write32(DATA + MD5_COUNT, count);
        for offset in (MD5_STATE_A..MD5_BLOCK + MD5_BLOCK_BYTES).step_by(4) {
            c.bus.write32(DATA + offset, rng.next_u32() | 0x101);
        }
        rng
    }

    pub(crate) fn crypt_md5_final_fixture(c: &mut Cpu, seed: u32) -> Rng {
        // The high bits of the byte count land in the bit-length bytes, so half of the seeds
        // add bits above 21 without changing the count within the block.
        let high = if seed / 12 % 2 == 1 {
            0x5a5u32 << 21
        } else {
            0
        };
        let count = [0, 55, 56, 63][(seed % 4) as usize] | high;
        let rng = prepare_md5(c, 0x003f_31f8, seed, count);
        c.g[A1][0] = sx(TEXT_OUTPUT);
        c.g[A2][0] = sx([0, 16, 33][(seed / 4 % 3) as usize]);
        c.bus.write32(GAME_GP + MD5_HEX_TABLE_GP, TEXT_TABLE);
        fill_bytes(c, TEXT_TABLE, b"0123456789abcdef");
        rng
    }

    #[test]
    fn crypt_md5_final_matches() {
        check_paths(
            ee::s14::f_003f31f8__generated,
            crypt_md5_final,
            24,
            &[1785, 1816, 2009, 1486, 3084, 3042],
            &|c, seed| {
                crypt_md5_final_fixture(c, seed);
            },
        );
    }

    driven!(
        crypt_md5_final_generated_driven,
        ee::s14::f_003f31f8__generated
    );

    driven!(crypt_md5_final_hand_driven, crypt_md5_final);

    #[test]
    fn crypt_md5_final_events() {
        check_events(
            crypt_md5_final_generated_driven,
            crypt_md5_final_hand_driven,
            6,
            &|c, seed| {
                let mut rng = crypt_md5_final_fixture(c, 4);
                let mode = schedule(c, &mut rng, seed, 0x003f_31f8);
                if mode != Events::Off {
                    c.next_event = c.cycles + [25, 1600][(seed / 3) as usize];
                }
                mode
            },
        );
    }

    pub(crate) fn crypt_md5_update_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let count = [0, 61][(seed / 4 % 2) as usize];
        let rng = prepare_md5(c, 0x003f_3128, seed, count);
        c.g[A1][0] = sx(TEXT_INPUT);
        c.g[A2][0] = sx([0, u32::MAX, 3, u32::MAX][(seed % 4) as usize]);
        if seed % 4 != 1 {
            fill_bytes(c, TEXT_INPUT, b"abc\0");
        }
        rng
    }

    #[test]
    fn crypt_md5_update_matches() {
        check_paths(
            ee::s14::f_003f3128__generated,
            crypt_md5_update,
            8,
            &[22, 25, 59, 85, 1279, 1305],
            &|c, seed| {
                crypt_md5_update_fixture(c, seed);
            },
        );
    }

    driven!(
        crypt_md5_update_generated_driven,
        ee::s14::f_003f3128__generated
    );

    driven!(crypt_md5_update_hand_driven, crypt_md5_update);

    #[test]
    fn crypt_md5_update_events() {
        check_events(
            crypt_md5_update_generated_driven,
            crypt_md5_update_hand_driven,
            6,
            &|c, seed| {
                let (fixture, delay) = [(3, 19), (2, 22)][(seed / 3) as usize];
                let mut rng = crypt_md5_update_fixture(c, fixture);
                let mode = schedule(c, &mut rng, seed, 0x003f_3128);
                if mode != Events::Off {
                    c.next_event = c.cycles + delay;
                }
                mode
            },
        );
    }

    #[test]
    fn crypt_md5_init_matches() {
        check_paths(
            ee::s14::f_003f30f0__generated,
            crypt_md5_init,
            8,
            &[14],
            &|c, seed| {
                prepare_md5(c, 0x003f_30f0, seed, u32::MAX);
            },
        );
    }

    #[test]
    fn crypt_md5_stream_known_answers() {
        let vectors = [
            (
                b"".as_slice(),
                [0xd9_8c_1d_d4, 0x04_b2_00_8f, 0x98_09_80_e9, 0x7e_42_f8_ec],
            ),
            (
                b"abc".as_slice(),
                [0x98_50_01_90, 0xb0_4f_d2_3c, 0x7d_3f_96_d6, 0x72_7f_e1_28],
            ),
            (
                b"12345678901234567890123456789012345678901234567890123456789012345678901234567890"
                    .as_slice(),
                [0xa2_f4_ed_57, 0x55_c9_e3_2b, 0x2e_da_49_ac, 0x7a_b6_07_21],
            ),
        ];
        for (message, expected) in vectors {
            let mut c = fresh();
            c.g[A0][0] = sx(DATA);
            crypt_md5_init(&mut c);
            fill_bytes(&mut c, TEXT_INPUT, message);
            for (part, slice) in message.chunks(17).enumerate() {
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(TEXT_INPUT + (part * 17) as u32);
                c.g[A2][0] = slice.len() as u64;
                crypt_md5_update(&mut c);
            }
            fill_bytes(&mut c, TEXT_OUTPUT, &[0xcd; 17]);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(TEXT_OUTPUT);
            c.g[A2][0] = MD5_DIGEST_BYTES;
            crypt_md5_final(&mut c);
            let digest = std::array::from_fn(|i| c.bus.read32(TEXT_OUTPUT + (i as u32) * 4));
            assert_eq!(digest, expected);
            assert_eq!(c.bus.read8(TEXT_OUTPUT + MD5_DIGEST_BYTES as u32), 0xcd);
        }
    }

    pub(crate) fn lobby_hash_memory_ranges_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let rng = prepare_text(c, 0x003e_fd30, seed);
        let input: &[u8] = match seed % 11 {
            0 => b"",
            1 => b"0:800103\0",
            2 => b"800101:0\0",
            3 => b"8000ff:800102\0",
            4 => b"800103:800110\0",
            5 => b"800103:800102\0",
            6 => b"800101:800104\0",
            7 => b"800101:800104,800105:800107\0",
            // A range starting exactly at the allowed minimum, and an empty one.
            9 => b"800100:800104\0",
            10 => b"800103:800103\0",
            _ => b"800102\0",
        };
        fill_bytes(c, TEXT_INPUT, input);
        fill_bytes(c, DATA + 0x100, b"memory span");
        if seed % 11 == 8 {
            // A missing colon leaves the caller's end slot unchanged.
            c.bus
                .write32(crate::difftest::SP - 0xd0 + LOBBY_HASH_END, DATA + 0x10b);
        }
        c.bus.write32(GAME_GP + LOBBY_HASH_MINIMUM_GP, DATA + 0x100);
        c.bus.write32(GAME_GP + LOBBY_HASH_LIMIT_GP, DATA + 0x110);
        c.g[A0][0] = sx(if input.is_empty() { 0 } else { TEXT_INPUT });
        c.g[A1][0] = (!input.is_empty()) as u64;
        c.g[A2][0] = sx(TEXT_OUTPUT);
        c.g[A3][0] = MD5_DIGEST_BYTES;
        rng
    }

    #[test]
    fn lobby_hash_memory_ranges_matches() {
        check_paths(
            ee::s14::f_003efd30__generated,
            lobby_hash_memory_ranges,
            11,
            &[1909, 2075, 2078, 2194, 2183, 2185, 2228, 2484, 2125],
            &|c, seed| {
                lobby_hash_memory_ranges_fixture(c, seed);
            },
        );
    }

    driven!(
        lobby_hash_memory_ranges_generated_driven,
        ee::s14::f_003efd30__generated
    );

    driven!(
        lobby_hash_memory_ranges_hand_driven,
        lobby_hash_memory_ranges
    );

    #[test]
    fn lobby_hash_memory_ranges_events() {
        check_events(
            lobby_hash_memory_ranges_generated_driven,
            lobby_hash_memory_ranges_hand_driven,
            6,
            &|c, seed| {
                let mut rng = lobby_hash_memory_ranges_fixture(c, 7);
                let mode = schedule(c, &mut rng, seed, 0x003e_fd30);
                if mode != Events::Off {
                    c.next_event = c.cycles + 350;
                }
                mode
            },
        );
    }

    #[test]
    fn lobby_parse_hex_range_matches() {
        check_paths(
            ee::s14::f_003efcd8__generated,
            lobby_parse_hex_range,
            6,
            &[43, 98],
            &|c, seed| {
                prepare_text(c, 0x003e_fcd8, seed);
                let input: &[u8] = match seed % 3 {
                    0 => b"1\0",
                    1 => b"a:B\0",
                    _ => b"a:B,9\0",
                };
                fill_bytes(c, TEXT_INPUT, input);
                c.g[A0][0] = sx(TEXT_INPUT);
                c.g[A1][0] = sx(TEXT_OUTPUT);
                c.g[A2][0] = sx(TEXT_OUTPUT + 4);
            },
        );
    }

    pub(crate) fn lobby_parse_hex_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let rng = prepare_text(c, 0x003e_fc68, seed);
        let input: &[u8] = match seed % 8 {
            0 => b"\0",
            1 => b"9\0",
            2 => b"f\0",
            3 => b"F\0",
            4 => b"1aB\xff\0",
            5 => b"ffffffff9:\0",
            // The letters just past the hex range.
            6 => b"g\0",
            _ => b"G\0",
        };
        fill_bytes(c, TEXT_INPUT, input);
        c.g[A0][0] = sx(TEXT_INPUT);
        c.g[A1][0] = sx(TEXT_OUTPUT);
        rng
    }

    #[test]
    fn lobby_parse_hex_matches() {
        check_paths(
            ee::s14::f_003efc68__generated,
            lobby_parse_hex,
            8,
            &[17, 29, 36, 41, 72, 181],
            &|c, seed| {
                lobby_parse_hex_fixture(c, seed);
            },
        );
    }

    driven!(
        lobby_parse_hex_generated_driven,
        ee::s14::f_003efc68__generated
    );

    driven!(lobby_parse_hex_hand_driven, lobby_parse_hex);

    pub(crate) fn require_lobby_resume(
        generated: GuestFn,
        hand: GuestFn,
        head: u32,
        setup: &dyn Fn(&mut Cpu) -> Events,
    ) {
        for test in [generated, hand] {
            let mut c = fresh();
            assert_eq!(setup(&mut c), Events::Unwind);
            test(&mut c);
            assert_eq!(c.pc, head | crate::difftest::events::UNWIND);
        }
    }

    #[test]
    fn lobby_parse_hex_events() {
        let setup = |c: &mut Cpu, seed| {
            let mut rng = lobby_parse_hex_fixture(c, 4);
            let mode = schedule(c, &mut rng, seed, 0x003e_fc68);
            if mode != Events::Off {
                c.next_event = c.cycles + 10;
            }
            mode
        };
        require_lobby_resume(
            ee::s14::f_003efc68__generated,
            lobby_parse_hex,
            LOBBY_HEX_NEXT_BYTE,
            &|c| setup(c, 1),
        );
        check_events(
            lobby_parse_hex_generated_driven,
            lobby_parse_hex_hand_driven,
            6,
            &setup,
        );
    }

    pub(crate) const UTF8_TEST_MAP_BYTES: u32 = TEXT_TABLE + 0x100;

    pub(crate) fn utf8_translation_table(c: &mut Cpu) {
        c.bus.write32(TEXT_TABLE, b'A' as u32);
        c.bus
            .write32(TEXT_TABLE + UTF8_TRANSLATION_END, b'B' as u32);
        c.bus
            .write32(TEXT_TABLE + UTF8_TRANSLATION_BYTES, UTF8_TEST_MAP_BYTES);
        let row = TEXT_TABLE + UTF8_TRANSLATION_ROW as u32;
        c.bus.write32(row, 0xa2);
        c.bus.write32(row + UTF8_TRANSLATION_END, 0xa2);
        c.bus
            .write32(row + UTF8_TRANSLATION_BYTES, UTF8_TEST_MAP_BYTES + 2);
        c.bus
            .write32(row + UTF8_TRANSLATION_ROW as u32 + UTF8_TRANSLATION_END, 0);
        fill_bytes(c, UTF8_TEST_MAP_BYTES, &[0x31, 0xff, 0xf3]);
    }

    pub(crate) fn utf8_byte_string_fixture(c: &mut Cpu, entry: u32, seed: u32) -> Rng {
        let rng = prepare_text(c, entry, seed);
        let input: &[u8] = match seed % 8 {
            0 => b"\0",
            1 => b"A\0",
            2 => b"\xc2\xa2\0",
            3 => b"\xe2\x82\xac\0",
            4 => b"\xf0\x9f\x99\x82A\0",
            5 => b"A\xc2\xa2z\0",
            6 => b"Z\0",
            _ => b"AB\0",
        };
        fill_bytes(c, TEXT_INPUT, input);
        c.g[A0][0] = sx(TEXT_OUTPUT);
        c.g[A1][0] = sx([96, 1, 2, 3][(seed / 8 % 4) as usize]);
        c.g[A2][0] = sx(TEXT_INPUT);
        c.g[A3][0] = [0, 0xff][(seed / 32 % 2) as usize];
        c.g[T0][0] = sx(TEXT_TABLE);
        rng
    }

    pub(crate) fn utf8_translate_to_8_bit_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let rng = utf8_byte_string_fixture(c, 0x003e_fba0, seed);
        utf8_translation_table(c);
        rng
    }

    #[test]
    fn utf8_translate_to_8_bit_matches() {
        check_paths(
            ee::s14::f_003efba0__generated,
            utf8_translate_to_8_bit,
            64,
            &[32, 33, 92, 111, 112, 114, 94],
            &|c, seed| {
                utf8_translate_to_8_bit_fixture(c, seed);
            },
        );
    }

    driven!(
        utf8_translate_to_8_bit_generated_driven,
        ee::s14::f_003efba0__generated
    );

    driven!(utf8_translate_to_8_bit_hand_driven, utf8_translate_to_8_bit);

    #[test]
    fn utf8_translate_to_8_bit_events() {
        check_events(
            utf8_translate_to_8_bit_generated_driven,
            utf8_translate_to_8_bit_hand_driven,
            6,
            &|c, seed| {
                let mut rng = utf8_translate_to_8_bit_fixture(c, 7);
                let mode = schedule(c, &mut rng, seed, 0x003e_fba0);
                if mode != Events::Off {
                    c.next_event = c.cycles + 80;
                }
                mode
            },
        );
    }

    pub(crate) fn utf8_encode_from_8_bit_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let rng = utf8_byte_string_fixture(c, 0x003e_faf8, seed);
        let input: &[u8] = match seed % 5 {
            0 => b"\0",
            1 => b"A\0",
            2 => b"\x90\0",
            3 => b"\xf0\0",
            _ => b"A\x90\xf0\0",
        };
        fill_bytes(c, TEXT_INPUT, input);
        c.g[A3][0] = sx(TEXT_TABLE);
        c.g[A1][0] = [96, 1, 3][(seed / 5 % 3) as usize];
        for (byte, code_point) in [(b'A', 0x7f), (0x90, 0x80), (0xf0, 0xffff)] {
            c.bus
                .write16(TEXT_TABLE + byte as u32 * UCS2_BYTES as u32, code_point);
        }
        rng
    }

    #[test]
    fn utf8_encode_from_8_bit_matches() {
        check_paths(
            ee::s14::f_003efaf8__generated,
            utf8_encode_from_8_bit,
            15,
            &[25, 26, 47, 54, 58, 50],
            &|c, seed| {
                utf8_encode_from_8_bit_fixture(c, seed);
            },
        );
    }

    driven!(
        utf8_encode_from_8_bit_generated_driven,
        ee::s14::f_003efaf8__generated
    );

    driven!(utf8_encode_from_8_bit_hand_driven, utf8_encode_from_8_bit);

    #[test]
    fn utf8_encode_from_8_bit_events() {
        check_events(
            utf8_encode_from_8_bit_generated_driven,
            utf8_encode_from_8_bit_hand_driven,
            6,
            &|c, seed| {
                let mut rng = utf8_encode_from_8_bit_fixture(c, 4);
                let mode = schedule(c, &mut rng, seed, 0x003e_faf8);
                if mode != Events::Off {
                    c.next_event = c.cycles + 37;
                }
                mode
            },
        );
    }

    #[test]
    fn utf8_decode_to_ucs2_matches() {
        const BYTE_STRING_CASES: u32 = 32;
        check_paths(
            ee::s14::f_003efa48__generated,
            utf8_decode_to_ucs2,
            BYTE_STRING_CASES + 1,
            &[28, 30, 56, 58, 65, 71, 83, 94, 98, 119, 125],
            &|c, seed| {
                utf8_byte_string_fixture(c, 0x003e_fa48, seed);
                if seed == BYTE_STRING_CASES {
                    // The guest skips five bytes for this lead byte, then decodes B.
                    fill_bytes(c, TEXT_INPUT, b"\xf0\x9f\x99\x82AB\0");
                }
            },
        );
    }

    driven!(
        utf8_decode_to_ucs2_generated_driven,
        ee::s14::f_003efa48__generated
    );

    driven!(utf8_decode_to_ucs2_hand_driven, utf8_decode_to_ucs2);

    #[test]
    fn utf8_decode_to_ucs2_events() {
        check_events(
            utf8_decode_to_ucs2_generated_driven,
            utf8_decode_to_ucs2_hand_driven,
            6,
            &|c, seed| {
                let mut rng = utf8_byte_string_fixture(c, 0x003e_fa48, 7);
                let mode = schedule(c, &mut rng, seed, 0x003e_fa48);
                if mode != Events::Off {
                    c.next_event = c.cycles + 44;
                }
                mode
            },
        );
    }

    pub(crate) fn utf8_encode_from_ucs2_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let rng = prepare_text(c, 0x003e_f9b8, seed);
        let input: &[u16] = match seed % 5 {
            0 => &[0],
            1 => &[0x7f, 0],
            2 => &[0x80, 0],
            3 => &[0xffff, 0],
            _ => &[0x7f, 0x80, 0xffff, 0],
        };
        for (index, &code_point) in input.iter().enumerate() {
            c.bus
                .write16(TEXT_INPUT + index as u32 * UCS2_BYTES as u32, code_point);
        }
        c.g[A0][0] = sx(TEXT_OUTPUT);
        c.g[A1][0] = [96, 1, 3][(seed / 5 % 3) as usize];
        c.g[A2][0] = sx(TEXT_INPUT);
        rng
    }

    #[test]
    fn utf8_encode_from_ucs2_matches() {
        check_paths(
            ee::s14::f_003ef9b8__generated,
            utf8_encode_from_ucs2,
            15,
            &[22, 23, 41, 48, 52, 44],
            &|c, seed| {
                utf8_encode_from_ucs2_fixture(c, seed);
            },
        );
    }

    driven!(
        utf8_encode_from_ucs2_generated_driven,
        ee::s14::f_003ef9b8__generated
    );

    driven!(utf8_encode_from_ucs2_hand_driven, utf8_encode_from_ucs2);

    #[test]
    fn utf8_encode_from_ucs2_events() {
        check_events(
            utf8_encode_from_ucs2_generated_driven,
            utf8_encode_from_ucs2_hand_driven,
            6,
            &|c, seed| {
                let mut rng = utf8_encode_from_ucs2_fixture(c, 4);
                let mode = schedule(c, &mut rng, seed, 0x003e_f9b8);
                if mode != Events::Off {
                    c.next_event = c.cycles + 33;
                }
                mode
            },
        );
    }

    pub(crate) fn utf8_str_len_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let rng = utf8_byte_string_fixture(c, 0x003e_f940, seed);
        c.g[A0][0] = sx(TEXT_INPUT);
        rng
    }

    #[test]
    fn utf8_str_len_matches() {
        check_paths(
            ee::s14::f_003ef940__generated,
            utf8_str_len,
            8,
            &[17, 26, 51, 58, 65],
            &|c, seed| {
                utf8_str_len_fixture(c, seed);
            },
        );
    }

    driven!(
        utf8_str_len_generated_driven,
        ee::s14::f_003ef940__generated
    );

    driven!(utf8_str_len_hand_driven, utf8_str_len);

    #[test]
    fn utf8_str_len_events() {
        check_events(
            utf8_str_len_generated_driven,
            utf8_str_len_hand_driven,
            6,
            &|c, seed| {
                let mut rng = utf8_str_len_fixture(c, 7);
                let mode = schedule(c, &mut rng, seed, 0x003e_f940);
                if mode != Events::Off {
                    c.next_event = c.cycles + 17;
                }
                mode
            },
        );
    }

    pub(crate) fn utf8_replace_fixture(c: &mut Cpu, entry: u32, seed: u32) -> Rng {
        let rng = utf8_byte_string_fixture(c, entry, seed);
        c.g[A1][0] = [96, 0, 1][(seed / 8 % 3) as usize];
        c.g[A3][0] = [0, 0xff][(seed / 24 % 2) as usize];
        rng
    }

    #[test]
    fn utf8_replace_matches() {
        check_paths(
            ee::s14::f_003ef828__generated,
            utf8_replace,
            48,
            &[30, 42, 72, 76, 37, 45],
            &|c, seed| {
                utf8_replace_fixture(c, 0x003e_f828, seed);
            },
        );
    }

    driven!(
        utf8_replace_generated_driven,
        ee::s14::f_003ef828__generated
    );

    driven!(utf8_replace_hand_driven, utf8_replace);

    #[test]
    fn utf8_replace_events() {
        check_events(
            utf8_replace_generated_driven,
            utf8_replace_hand_driven,
            6,
            &|c, seed| {
                let mut rng = utf8_replace_fixture(c, 0x003e_f828, 0);
                let (input, delay): (&[u8], u64) = if seed / 3 == 0 {
                    (b"aa\xc2\xa2aa\0", 25)
                } else {
                    (b"\xc2\xa2az\0", 59)
                };
                fill_bytes(c, TEXT_INPUT, input);
                let mode = schedule(c, &mut rng, seed, 0x003e_f828);
                if mode != Events::Off {
                    c.next_event = c.cycles + delay;
                }
                mode
            },
        );
    }

    #[test]
    fn utf8_replace_output_full_matches() {
        // The sequence-width helper counts one byte past each multibyte sequence, so a long
        // input only runs out of output room when the cap is small: the NUL is never reached.
        check_paths(
            ee::s14::f_003ef828__generated,
            utf8_replace,
            4,
            &[78, 91],
            &|c, seed| {
                utf8_replace_fixture(c, 0x003e_f828, 0);
                fill_bytes(c, TEXT_INPUT, b"\xc2\xa2abc\0");
                c.g[A1][0] = u64::from(1 + seed % 2);
                c.g[A3][0] = [0xff, u64::from(b'?')][(seed / 2 % 2) as usize];
            },
        );
    }

    #[test]
    fn utf8_strip_matches() {
        check_paths(
            ee::s14::f_003ef808__generated,
            utf8_strip,
            8,
            &[37, 79, 49],
            &|c, seed| {
                utf8_replace_fixture(c, 0x003e_f808, seed);
                c.g[A3][0] = 0xff;
            },
        );
    }

    pub(crate) fn utf8_translate_code_pt_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let rng = prepare_text(c, 0x003e_f770, seed);
        utf8_translation_table(c);
        c.g[A0][0] = sx(TEXT_OUTPUT);
        c.g[A1][0] = sx(TEXT_TABLE);
        c.g[A2][0] = [0, 0x43, 0xa3, 0x41, 0x42, 0xa2][(seed % 6) as usize];
        c.g[A3][0] = [0, 0xff][(seed / 6 % 2) as usize];
        rng
    }

    #[test]
    fn utf8_translate_code_pt_matches() {
        check_paths(
            ee::s14::f_003ef770__generated,
            utf8_translate_code_pt,
            12,
            &[29, 31, 32, 34, 35, 37, 30, 33, 40],
            &|c, seed| {
                utf8_translate_code_pt_fixture(c, seed);
            },
        );
    }

    driven!(
        utf8_translate_code_pt_generated_driven,
        ee::s14::f_003ef770__generated
    );

    driven!(utf8_translate_code_pt_hand_driven, utf8_translate_code_pt);

    #[test]
    fn utf8_translate_code_pt_events() {
        check_events(
            utf8_translate_code_pt_generated_driven,
            utf8_translate_code_pt_hand_driven,
            6,
            &|c, seed| {
                let mut rng = utf8_translate_code_pt_fixture(c, 7);
                let mode = schedule(c, &mut rng, seed, 0x003e_f770);
                if mode != Events::Off {
                    c.next_event = c.cycles + 13;
                }
                mode
            },
        );
    }

    #[test]
    fn utf8_encode_from_ucs2_code_pt_matches() {
        check_paths(
            ee::s14::f_003ef6f8__generated,
            utf8_encode_from_ucs2_code_pt,
            8,
            &[9, 16, 20],
            &|c, seed| {
                prepare_text(c, 0x003e_f6f8, seed);
                c.g[A0][0] = sx(TEXT_OUTPUT);
                c.g[A1][0] = 0x1234_5678_ffff_0000
                    | [0, 0x7f, 0x80, 0x7ff, 0x800, 0xd800, 0xffff, 0x20ac][seed as usize];
            },
        );
    }

    #[test]
    fn utf8_decode_code_pt_matches() {
        check_paths(
            ee::s14::f_003ef640__generated,
            utf8_decode_code_pt,
            8,
            &[14, 23, 29, 35, 56, 67],
            &|c, seed| {
                prepare_text(c, 0x003e_f640, seed);
                let bytes: &[u8] = match seed % 8 {
                    0 => b"A",
                    1 => b"\xc2\xa2",
                    2 => b"\xe2\x82\xac",
                    3 => b"\x80\xb0",
                    4 => b"\xf0\x9f\x99\x82",
                    5 => b"\xfe\x80\x80\x80\x80\x80\x80",
                    6 => b"\xc0\x80",
                    _ => b"\xe0\x80\x80",
                };
                fill_bytes(c, TEXT_INPUT, bytes);
                c.g[A0][0] = sx(TEXT_OUTPUT);
                c.g[A1][0] = sx(TEXT_INPUT);
            },
        );
    }

    #[test]
    fn utf8_get_seq_len_matches() {
        check_paths(
            ee::s14::f_003ef600__generated,
            utf8_get_seq_len,
            8,
            &[6, 15, 22, 29, 36, 43, 47],
            &|c, seed| {
                prepare_text(c, 0x003e_f600, seed);
                c.g[A0][0] = 0x1234_5678_ffff_0000
                    | [0x7f, 0x80, 0xc0, 0xe0, 0xf0, 0xf8, 0xfc, 0xff][seed as usize];
            },
        );
    }

    driven!(
        utf8_get_seq_len_generated_driven,
        ee::s14::f_003ef600__generated
    );

    driven!(utf8_get_seq_len_hand_driven, utf8_get_seq_len);

    #[test]
    fn utf8_get_seq_len_events() {
        check_events(
            utf8_get_seq_len_generated_driven,
            utf8_get_seq_len_hand_driven,
            6,
            &|c, seed| {
                let mut rng = prepare_text(c, 0x003e_f600, seed);
                c.g[A0][0] = 0xf8;
                let mode = schedule(c, &mut rng, seed, 0x003e_f600);
                if mode != Events::Off {
                    c.next_event = c.cycles + 10;
                }
                mode
            },
        );
    }

    pub(crate) const LOBBY_SORT_TEST_GATE: u32 = DATA + 0x33000;

    pub(crate) const LOBBY_SORT_TEST_ALLOCATIONS: u32 = LOBBY_SORT_TEST_GATE + 4;

    pub(crate) const LOBBY_SORT_TEST_FREES: u32 = LOBBY_SORT_TEST_GATE + 8;

    pub(crate) const LOBBY_SORT_TEST_MAGIC: u32 = 0xb86d_2e91;

    pub(crate) const LOBBY_SORT_TEST_ITEMS: u32 = DATA + 0x34000;

    pub(crate) const LOBBY_SORT_TEST_HEAP: u32 = DATA + 0x38000;

    pub(crate) const LOBBY_SORT_TEST_ALLOCATION_BYTES: u32 = 0x1000;

    pub(crate) fn lobby_sort_compare(c: &mut Cpu) {
        use crate::difftest::fakes;
        if c.bus.read32(LOBBY_SORT_TEST_GATE) != LOBBY_SORT_TEST_MAGIC {
            fakes::fall_back(
                c,
                fakes::prev_cell(lobby_sort_compare),
                ee::s15::f_004165a8__generated,
            );
            return;
        }
        c.cycles += 1;
        let left = c.bus.read8(r32(c, A1)) as i32;
        let right = c.bus.read8(r32(c, A2)) as i32;
        c.g[V0][0] = sx(left.wrapping_sub(right) as u32);
        ret(c);
    }

    pub(crate) fn lobby_sort_alloc(c: &mut Cpu) {
        use crate::difftest::fakes;
        if c.bus.read32(LOBBY_SORT_TEST_GATE) != LOBBY_SORT_TEST_MAGIC {
            fakes::fall_back(
                c,
                fakes::prev_cell(lobby_sort_alloc),
                ee::s10::f_00317e30__generated,
            );
            return;
        }
        let index = c.bus.read32(LOBBY_SORT_TEST_ALLOCATIONS);
        assert!(index < 4);
        assert!(r32(c, A0) <= LOBBY_SORT_TEST_ALLOCATION_BYTES);
        c.bus.write32(LOBBY_SORT_TEST_ALLOCATIONS, index + 1);
        c.g[V0][0] = sx(LOBBY_SORT_TEST_HEAP + index * LOBBY_SORT_TEST_ALLOCATION_BYTES);
        c.cycles += 1;
        ret(c);
    }

    pub(crate) fn lobby_sort_free_test(c: &mut Cpu) {
        use crate::difftest::fakes;
        if c.bus.read32(LOBBY_SORT_TEST_GATE) != LOBBY_SORT_TEST_MAGIC {
            fakes::fall_back(
                c,
                fakes::prev_cell(lobby_sort_free_test),
                ee::s10::f_00317e98__generated,
            );
            return;
        }
        let index = c.bus.read32(LOBBY_SORT_TEST_FREES);
        assert_eq!(
            r32(c, A0),
            LOBBY_SORT_TEST_HEAP + index * LOBBY_SORT_TEST_ALLOCATION_BYTES
        );
        c.bus.write32(LOBBY_SORT_TEST_FREES, index + 1);
        c.cycles += 1;
        ret(c);
    }

    pub(crate) fn lobby_sort_test_hooks() {
        use crate::difftest::fakes::{hook, prev_cell};
        use std::ptr::addr_of_mut;
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_004165A8),
            lobby_sort_compare,
            prev_cell(lobby_sort_compare),
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            lobby_sort_alloc,
            prev_cell(lobby_sort_alloc),
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            lobby_sort_free_test,
            prev_cell(lobby_sort_free_test),
        );
    }

    pub(crate) fn lobby_merge_sort_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let mut rng = prepare_text(c, 0x003e_f210, seed);
        let (count, width, keys): (u32, u32, &[u8]) = match seed % 12 {
            0 => (0, 4, &[]),
            1 => (1, 4, &[5]),
            2 => (2, 0, &[3, 1]),
            3 => (2, 4, &[1, 2]),
            4 => (4, 4, &[4, 3, 2, 1]),
            5 => (5, 4, &[1, 4, 2, 5, 6]),
            6 => (3, 4, &[3, 1, 2]),
            7 => (6, 4, &[2, 4, 1, 2, 3, 4]),
            8 => (512, 4, &[2]),
            9 => (2, 1028, &[2, 1]),
            10 => (3, 4, &[3, 2, 1]),
            _ => (6, 4, &[1, 6, 5, 4, 3, 2]),
        };
        for index in 0..count {
            let item = LOBBY_SORT_TEST_ITEMS + index * width;
            let key = keys[index as usize % keys.len()];
            c.bus
                .write32(item, (rng.next_u32() & !0xff) | u32::from(key));
            if width > 4 {
                c.bus.write32(item + width - 4, 0xbad0_0000 | index);
            }
        }
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(LOBBY_SORT_TEST_ITEMS);
        c.g[A2][0] = sx(count);
        c.g[A3][0] = sx(width);
        c.g[T0][0] = sx(crate::game::lib::libc::callee::STRCMP.addr);
        c.bus.write32(LOBBY_SORT_TEST_GATE, LOBBY_SORT_TEST_MAGIC);
        rng
    }

    #[test]
    fn lobby_merge_sort_matches() {
        lobby_sort_test_hooks();
        check_paths_lohi(
            ee::s14::f_003ef210__generated,
            lobby_merge_sort,
            12,
            &[30, 70, 88, 443],
            &|c, seed| {
                lobby_merge_sort_fixture(c, seed);
            },
        );
    }

    #[test]
    fn lobby_merge_sort_preserves_equal_item_order() {
        lobby_sort_test_hooks();
        let mut c = fresh();
        lobby_merge_sort_fixture(&mut c, 7);
        let original: [u32; 6] =
            std::array::from_fn(|index| c.bus.read32(LOBBY_SORT_TEST_ITEMS + (index as u32) * 4));
        lobby_merge_sort(&mut c);
        let reordered: [u32; 6] =
            std::array::from_fn(|index| c.bus.read32(LOBBY_SORT_TEST_ITEMS + (index as u32) * 4));
        assert_eq!(
            reordered,
            [
                original[2],
                original[0],
                original[3],
                original[4],
                original[1],
                original[5]
            ]
        );
    }

    driven!(
        lobby_merge_sort_generated_driven,
        ee::s14::f_003ef210__generated
    );

    driven!(lobby_merge_sort_hand_driven, lobby_merge_sort);

    #[test]
    fn lobby_merge_sort_events() {
        lobby_sort_test_hooks();
        let cases = [
            (4, 58, LOBBY_SORT_SCAN_RUN),
            (4, 138, LOBBY_SORT_PICK_ENTRY),
            (5, 202, LOBBY_SORT_ADVANCE_RIGHT),
            (4, 162, LOBBY_SORT_MERGE_PAIR),
            (4, 215, LOBBY_SORT_MERGE_PASS),
            (6, 301, LOBBY_SORT_ROTATE_ENTRY),
            (6, 405, LOBBY_SORT_MOVE_ENTRY),
        ];
        let setup = |c: &mut Cpu, seed| {
            let (fixture, delay, _) = cases[(seed / 3) as usize];
            let mut rng = lobby_merge_sort_fixture(c, fixture);
            let mode = schedule(c, &mut rng, seed, 0x003e_f210);
            if mode != Events::Off {
                c.next_event = c.cycles + delay;
            }
            mode
        };
        for (index, &(_, _, head)) in cases.iter().enumerate() {
            require_lobby_resume(
                ee::s14::f_003ef210__generated,
                lobby_merge_sort,
                head,
                &|c| setup(c, index as u32 * 3 + 1),
            );
        }
        check_events(
            lobby_merge_sort_generated_driven,
            lobby_merge_sort_hand_driven,
            3 * cases.len() as u32,
            &setup,
        );
    }

    pub(crate) const TABLE: u32 = DATA + 0x100;

    pub(crate) const BUCKETS: u32 = DATA + 0x200;

    pub(crate) const FIRST_ENTRY: u32 = DATA + 0x300;

    pub(crate) const SECOND_ENTRY: u32 = DATA + 0x310;

    pub(crate) const FREE_ENTRY: u32 = DATA + 0x320;

    pub(crate) const POOL: u32 = DATA + 0x800;

    pub(crate) const STRING_KEY: u32 = DATA + 0x400;

    pub(crate) const OTHER_KEY: u32 = DATA + 0x410;

    pub(crate) const STRCMP_CALLBACK: u32 = 0x0041_65a8;

    pub(crate) const ALLOC_RESULT: u32 = MEM_FAKES_CTRL + 0x18;

    pub(crate) fn check_lobby_loop_events(
        generated: GuestFn,
        hand: GuestFn,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32) -> Events,
    ) {
        check_events(generated, hand, seeds, setup);
        own_check_delivers("lobby utility loop", generated, hand, seeds, setup);
    }

    pub(crate) fn hash_fixture(c: &mut Cpu, seed: u32, entry: u32, count: u32) -> Rng {
        let mut r = Rng(u64::from(entry) ^ u64::from(seed));
        scramble(c, &mut r);
        c.pc = entry;
        c.g[A0][0] = sx(TABLE);
        c.bus.write32(TABLE + HASH_BUCKET_COUNT, count);
        c.bus.write32(TABLE + HASH_BUCKETS, BUCKETS);
        c.bus.write32(TABLE + HASH_FREE_HEAD, FREE_ENTRY);
        c.bus.write32(TABLE + HASH_ENTRY_COUNT, 2);
        c.bus.write32(BUCKETS, 0);
        c.bus.write32(BUCKETS + 4, 0);
        c.bus.write32(FIRST_ENTRY, SECOND_ENTRY);
        c.bus.write32(SECOND_ENTRY, 0);
        c.bus
            .write32(FIRST_ENTRY + HASH_ENTRY_VALUE, r.next_u32() | 1);
        c.bus
            .write32(SECOND_ENTRY + HASH_ENTRY_VALUE, r.next_u32() | 1);
        r
    }

    #[test]
    fn ds_timezone_matches() {
        check_paths_lohi(
            ee::s14::f_003ef090__generated,
            ds_timezone,
            8,
            &[23, 762],
            &|c, seed| {
                scramble(c, &mut Rng(u64::from(seed)));
                c.pc = 0x003e_f090;
                let cached = [u32::MAX, 0, 3_600, (-18_000_i32) as u32][(seed % 4) as usize];
                c.bus.write32(r32(c, GP) + DS_TIMEZONE_GP, cached);
                // Select the SDK's precached development-kit settings path.
                c.bus.write8(0x0045_3400, b'0');
                c.bus.write8(0x0045_3404, b'T');
                c.bus.write8(0x0045_33fe, 0);
            },
        );
    }

    #[test]
    fn ds_timeinsecs_matches() {
        check_paths(
            ee::s14::f_003ef088__generated,
            ds_timeinsecs,
            8,
            &[2],
            &|c, seed| {
                scramble(c, &mut Rng(u64::from(seed)));
                c.pc = 0x003e_f088;
            },
        );
    }

    pub(crate) fn hash_int_lookup_fixture(c: &mut Cpu, seed: u32, entry: u32) -> Rng {
        let r = hash_fixture(c, seed, entry, 2);
        // The sign test is "below zero", so the key also takes 0.
        let needle: i32 = match (seed % 2, seed / 8 % 2) {
            (_, 1) => 0,
            (0, _) => 7,
            _ => -7,
        };
        c.g[A1][0] = sx(needle as u32);
        let bucket = BUCKETS + 4;
        let (head, first_key, second_key) = match seed / 2 % 4 {
            0 => (0, 1, 3),
            1 => (FIRST_ENTRY, needle, 3),
            2 => (FIRST_ENTRY, 1, needle),
            _ => (FIRST_ENTRY, 1, 3),
        };
        c.bus.write32(bucket, head);
        c.bus
            .write32(FIRST_ENTRY + HASH_ENTRY_KEY, first_key as u32);
        c.bus
            .write32(SECOND_ENTRY + HASH_ENTRY_KEY, second_key as u32);
        r
    }

    #[test]
    fn lobby_hash_del_int_matches() {
        check_paths_lohi(
            ee::s14::f_003eef40__generated,
            lobby_hash_del_int,
            16,
            &[16, 17, 27, 28, 33, 34, 29],
            &|c, seed| {
                hash_int_lookup_fixture(c, seed, 0x003e_ef40);
            },
        );
    }

    #[test]
    fn lobby_hash_find_int_matches() {
        check_paths_lohi(
            ee::s14::f_003eeee8__generated,
            lobby_hash_find_int,
            16,
            &[16, 17, 19, 20, 24, 25, 26, 27],
            &|c, seed| {
                hash_int_lookup_fixture(c, seed, 0x003e_eee8);
            },
        );
    }

    #[test]
    fn lobby_hash_int_lookup_events() {
        for (entry, original, hand, delay) in [
            (
                0x003e_ef40,
                ee::s14::f_003eef40__generated as GuestFn,
                lobby_hash_del_int as GuestFn,
                16,
            ),
            (
                0x003e_eee8,
                ee::s14::f_003eeee8__generated as GuestFn,
                lobby_hash_find_int as GuestFn,
                16,
            ),
        ] {
            check_lobby_loop_events(original, hand, 6, &|c, seed| {
                let mut r = hash_int_lookup_fixture(c, (seed / 3) + 4, entry);
                let mode = schedule(c, &mut r, seed, entry);
                if mode != Events::Off {
                    c.next_event = c.cycles + delay;
                }
                mode
            });
        }
    }

    pub(crate) fn hash_insert_fixture(c: &mut Cpu, seed: u32, entry: u32, string_key: bool) {
        let mut r = hash_fixture(c, seed, entry, 2);
        let mode = seed / 2 % 3;
        c.bus.write32(
            TABLE + HASH_FREE_HEAD,
            if mode == 0 { FREE_ENTRY } else { 0 },
        );
        c.bus.write32(FREE_ENTRY, 0);
        c.bus.write32(TABLE + HASH_POOL_BLOCKS, 0);
        c.bus.write32(BUCKETS, FIRST_ENTRY);
        c.bus.write32(BUCKETS + 4, FIRST_ENTRY);
        c.bus.write32(MEM_FAKES_CTRL, MEM_FAKES_MAGIC);
        c.bus
            .write32(ALLOC_RESULT, if mode == 2 { POOL } else { 0 });
        c.bus.write8(STRING_KEY, 0);
        let key: i32 = match (seed % 2, seed / 6 % 2) {
            (_, 1) => 0,
            (0, _) => 7,
            _ => -7,
        };
        c.g[A1][0] = if string_key {
            sx(STRING_KEY)
        } else {
            sx(key as u32)
        };
        c.g[A2][0] = sx(r.next_u32() | 1);
    }

    #[test]
    fn lobby_hash_add_int_matches() {
        mem_fakes_hook_with_new();
        check_paths_lohi(
            ee::s14::f_003eeda0__generated,
            lobby_hash_add_int,
            12,
            &[42, 43, 79, 80, 159, 160],
            &|c, seed| hash_insert_fixture(c, seed, 0x003e_eda0, false),
        );
    }

    #[test]
    fn lobby_hash_add_str_matches() {
        mem_fakes_hook_with_new();
        check_paths_lohi(
            ee::s14::f_003eeb30__generated,
            lobby_hash_add_str,
            6,
            &[51, 88, 168],
            &|c, seed| hash_insert_fixture(c, seed, 0x003e_eb30, true),
        );
    }

    pub(crate) fn hash_str_lookup_fixture(c: &mut Cpu, seed: u32, entry: u32) -> Rng {
        let mut r = hash_fixture(c, seed, entry, 2);
        strcmp_fake_on(c);
        c.bus.write32(TABLE + HASH_COMPARE, STRCMP_CALLBACK);
        c.g[A1][0] = sx(STRING_KEY);
        c.g[A2][0] = sx(r.next_u32() | 1);
        // The empty key hashes to bucket 0; "A" is 65, odd, so it hashes to bucket 1 of 2.
        let second_bucket = seed / 4 % 2 == 1;
        c.bus
            .write8(STRING_KEY, if second_bucket { b'A' } else { 0 });
        c.bus.write8(STRING_KEY + 1, 0);
        c.bus.write8(OTHER_KEY, b'x');
        c.bus.write8(OTHER_KEY + 1, 0);
        let (head, first_key, second_key) = match seed % 4 {
            0 => (0, OTHER_KEY, OTHER_KEY),
            1 => (FIRST_ENTRY, STRING_KEY, OTHER_KEY),
            2 => (FIRST_ENTRY, OTHER_KEY, STRING_KEY),
            _ => (FIRST_ENTRY, OTHER_KEY, OTHER_KEY),
        };
        c.bus
            .write32(if second_bucket { BUCKETS + 4 } else { BUCKETS }, head);
        c.bus.write32(FIRST_ENTRY + HASH_ENTRY_KEY, first_key);
        c.bus.write32(SECOND_ENTRY + HASH_ENTRY_KEY, second_key);
        r
    }

    #[test]
    fn lobby_hash_del_str_matches() {
        check_paths(
            ee::s14::f_003eed08__generated,
            lobby_hash_del_str,
            8,
            &[37, 61, 79, 73],
            &|c, seed| {
                hash_str_lookup_fixture(c, seed, 0x003e_ed08);
            },
        );
    }

    #[test]
    fn lobby_hash_find_str_matches() {
        check_paths(
            ee::s14::f_003eec88__generated,
            lobby_hash_find_str,
            8,
            &[34, 53, 70, 71],
            &|c, seed| {
                hash_str_lookup_fixture(c, seed, 0x003e_ec88);
            },
        );
    }

    #[test]
    fn lobby_hash_replace_str_matches() {
        check_paths(
            ee::s14::f_003eebf8__generated,
            lobby_hash_replace_str,
            8,
            &[37, 57, 74],
            &|c, seed| {
                hash_str_lookup_fixture(c, seed, 0x003e_ebf8);
            },
        );
    }

    #[test]
    fn lobby_hash_str_lookup_events() {
        for (entry, original, hand, delay) in [
            (
                0x003e_ed08,
                ee::s14::f_003eed08__generated as GuestFn,
                lobby_hash_del_str as GuestFn,
                43,
            ),
            (
                0x003e_ec88,
                ee::s14::f_003eec88__generated as GuestFn,
                lobby_hash_find_str as GuestFn,
                46,
            ),
            (
                0x003e_ebf8,
                ee::s14::f_003eebf8__generated as GuestFn,
                lobby_hash_replace_str as GuestFn,
                48,
            ),
        ] {
            check_lobby_loop_events(original, hand, 6, &|c, seed| {
                let mut r = hash_str_lookup_fixture(c, 2 + seed / 3, entry);
                let mode = schedule(c, &mut r, seed, entry);
                if mode != Events::Off {
                    c.next_event = c.cycles + delay;
                }
                mode
            });
        }
    }

    pub(crate) fn hash_string_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let r = hash_fixture(c, seed, 0x003e_ea48, 13);
        c.g[A1][0] = sx(STRING_KEY);
        let input: &[u8] = match seed % 4 {
            0 => b"\0",
            1 => b" A\0",
            2 => b"abC!\0",
            _ => b"\xff \0",
        };
        for (offset, &byte) in input.iter().enumerate() {
            c.bus.write8(STRING_KEY + offset as u32, byte);
        }
        r
    }

    #[test]
    fn lobby_hash_string_matches() {
        check_paths_lohi(
            ee::s14::f_003eea48__generated,
            lobby_hash_string,
            4,
            &[14, 35, 60, 32],
            &|c, seed| {
                hash_string_fixture(c, seed);
            },
        );
    }

    #[test]
    fn lobby_hash_string_events() {
        check_lobby_loop_events(
            ee::s14::f_003eea48__generated,
            lobby_hash_string,
            9,
            &|c, seed| {
                let mut r = hash_string_fixture(c, 2);
                let mode = schedule(c, &mut r, seed, 0x003e_ea48);
                if mode != Events::Off {
                    c.next_event = c.cycles + 16 + 11 * u64::from(seed / 3);
                }
                mode
            },
        );
    }

    pub(crate) fn hash_pop_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let r = hash_fixture(c, seed, 0x003e_e9a0, 3);
        let (count, cursor, occupied) =
            [(0, 0, 0), (2, 0, 0), (2, 0, 1), (2, 1, 0)][(seed % 4) as usize];
        c.bus.write32(TABLE + HASH_ENTRY_COUNT, count);
        c.bus.write32(TABLE + HASH_POP_BUCKET, cursor);
        c.bus.write32(BUCKETS + 8, 0);
        c.bus.write32(BUCKETS + 4 * occupied, FIRST_ENTRY);
        r
    }

    #[test]
    fn lobby_hash_pop_any_matches() {
        check_paths_lohi(
            ee::s14::f_003ee9a0__generated,
            lobby_hash_pop_any,
            4,
            &[6, 24, 39, 50],
            &|c, seed| {
                hash_pop_fixture(c, seed);
            },
        );
    }

    #[test]
    fn lobby_hash_pop_any_events() {
        check_lobby_loop_events(
            ee::s14::f_003ee9a0__generated,
            lobby_hash_pop_any,
            3,
            &|c, seed| {
                let mut r = hash_pop_fixture(c, 3);
                let mode = schedule(c, &mut r, seed, 0x003e_e9a0);
                if mode != Events::Off {
                    c.next_event = c.cycles + 24;
                }
                mode
            },
        );
    }

    pub(crate) fn hash_grow_pool_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let mut r = hash_fixture(c, seed, 0x003e_e900, 2);
        let count = [0, 1, 3, 1 << 30][(seed % 4) as usize];
        c.g[A1][0] = sx(count);
        c.bus.write32(TABLE + HASH_POOL_BLOCKS, DATA + 0x900);
        c.bus.write32(MEM_FAKES_CTRL, MEM_FAKES_MAGIC);
        c.bus
            .write32(ALLOC_RESULT, if seed / 4 % 2 == 0 { POOL } else { 0 });
        crate::difftest::fill(c, &mut r, POOL, 12);
        r
    }

    #[test]
    fn lobby_hash_grow_pool_matches() {
        mem_fakes_hook_with_new();
        check_paths_lohi(
            ee::s14::f_003ee900__generated,
            lobby_hash_grow_pool,
            8,
            &[40, 48, 57, 71],
            &|c, seed| {
                hash_grow_pool_fixture(c, seed);
            },
        );
    }

    #[test]
    fn lobby_hash_grow_pool_events() {
        mem_fakes_hook_with_new();
        check_lobby_loop_events(
            ee::s14::f_003ee900__generated,
            lobby_hash_grow_pool,
            6,
            &|c, seed| {
                let mut r = hash_grow_pool_fixture(c, 2);
                let mode = schedule(c, &mut r, seed, 0x003e_e900);
                if mode != Events::Off {
                    c.next_event = c.cycles + 49 + 7 * u64::from(seed / 3);
                }
                mode
            },
        );
    }

    pub(crate) fn hash_destroy_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let r = hash_fixture(c, seed, 0x003e_e820, 2);
        let blocks = seed % 3;
        c.bus.write32(
            TABLE + HASH_POOL_BLOCKS,
            if blocks == 0 { 0 } else { FIRST_ENTRY },
        );
        c.bus
            .write32(FIRST_ENTRY, if blocks == 2 { SECOND_ENTRY } else { 0 });
        c.bus.write32(MEM_FAKES_CTRL, MEM_FAKES_MAGIC);
        r
    }

    #[test]
    fn lobby_hash_destroy_matches() {
        mem_fakes_hook_with_new();
        check_paths(
            ee::s14::f_003ee820__generated,
            lobby_hash_destroy,
            3,
            &[37, 55, 72],
            &|c, seed| {
                hash_destroy_fixture(c, seed);
            },
        );
    }

    #[test]
    fn lobby_hash_destroy_events() {
        mem_fakes_hook_with_new();
        check_lobby_loop_events(
            ee::s14::f_003ee820__generated,
            lobby_hash_destroy,
            3,
            &|c, seed| {
                let mut r = hash_destroy_fixture(c, 2);
                let mode = schedule(c, &mut r, seed, 0x003e_e820);
                if mode != Events::Off {
                    c.next_event = c.cycles + 24;
                }
                mode
            },
        );
    }

    pub(crate) const HASH_CREATE_ALLOC_CTRL: u32 = DATA + 0xc_4000;

    pub(crate) const HASH_CREATE_ALLOC_MAGIC: u32 = 0x003e_e778;

    pub(crate) const HASH_CREATE_ALLOC_INDEX: u32 = HASH_CREATE_ALLOC_CTRL + 4;

    pub(crate) fn hash_create_alloc(c: &mut Cpu) {
        use crate::difftest::fakes::{fall_back, prev_cell};

        if c.bus.read32(HASH_CREATE_ALLOC_CTRL) != HASH_CREATE_ALLOC_MAGIC {
            return fall_back(
                c,
                prev_cell(hash_create_alloc),
                ee::s10::f_00317e30__generated,
            );
        }
        let index = c.bus.read32(HASH_CREATE_ALLOC_INDEX);
        c.bus.write32(HASH_CREATE_ALLOC_INDEX, index + 1);
        c.g[V0][0] = sx([TABLE, BUCKETS, POOL][index as usize]);
        c.cycles += 7;
        ret(c);
    }

    #[test]
    fn lobby_hash_create_matches() {
        use crate::difftest::fakes::{hook, prev_cell};

        hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            hash_create_alloc,
            prev_cell(hash_create_alloc),
        );
        check_paths_lohi(
            ee::s14::f_003ee778__generated,
            lobby_hash_create,
            6,
            &[242, 256, 272, 286],
            &|c, seed| {
                let mut r = hash_fixture(c, seed, 0x003e_e778, 2);
                c.bus
                    .write32(HASH_CREATE_ALLOC_CTRL, HASH_CREATE_ALLOC_MAGIC);
                c.g[A0][0] = u64::from((seed % 2) * 3);
                // Fewer than the minimum of 16 buckets is raised to 16, 15 being the largest such
                // request; 17 is kept as asked.
                c.g[A1][0] = [0, 17, 15][(seed / 2 % 3) as usize];
                crate::difftest::fill(c, &mut r, TABLE, 9);
                crate::difftest::fill(c, &mut r, BUCKETS, 17);
                crate::difftest::fill(c, &mut r, POOL, 12);
            },
        );
    }

    pub(crate) fn list_fixture(c: &mut Cpu, seed: u32, entry: u32) -> Rng {
        let mut r = Rng(u64::from(entry) ^ u64::from(seed));
        scramble(c, &mut r);
        c.pc = entry;
        c.g[A0][0] = sx(TABLE);
        crate::difftest::fill(c, &mut r, TABLE, 17);
        c.bus.write32(TABLE + LIST_ENTRIES, BUCKETS);
        c.bus.write32(BUCKETS, FIRST_ENTRY);
        c.bus.write32(BUCKETS + 4, SECOND_ENTRY);
        c.bus.write32(FIRST_ENTRY, r.next_u32() | 1);
        c.bus.write32(SECOND_ENTRY, r.next_u32() | 1);
        r
    }

    #[test]
    fn lobby_list_get_hash_matches() {
        check_paths(
            ee::s14::f_003ee770__generated,
            lobby_list_get_hash,
            6,
            &[2],
            &|c, seed| {
                list_fixture(c, seed, 0x003e_e770);
            },
        );
    }

    #[test]
    fn lobby_list_set_hash_matches() {
        check_paths(
            ee::s14::f_003ee768__generated,
            lobby_list_set_hash,
            6,
            &[2],
            &|c, seed| {
                list_fixture(c, seed, 0x003e_e768);
            },
        );
    }

    pub(crate) fn list_sort_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let r = list_fixture(c, seed, 0x003e_e620);
        let (sort_dirty, notify_dirty, count, filter, comparator, filter_sign) = match seed % 8 {
            0 => (0, 0, 0, 0, 0, false),
            1 => (0, 1, 0, 0, 0, false),
            2 => (1, 0, 0, 0, 0, false),
            3 => (1, 0, 1, 0, 0, false),
            4 => (1, 0, 2, 0, 0, false),
            5 => (1, 0, 2, STRCMP_CALLBACK, 0, true),
            6 => (1, 0, 2, STRCMP_CALLBACK, 0, false),
            _ => (1, 0, 1, 0, STRCMP_CALLBACK, false),
        };
        strcmp_fake_on(c);
        c.bus.write32(TABLE + LIST_SORT_DIRTY, sort_dirty);
        c.bus.write32(TABLE + LIST_NOTIFY_DIRTY, notify_dirty);
        c.bus.write32(TABLE + LIST_ENTRY_COUNT, count);
        c.bus.write32(TABLE + LIST_FILTER_ARG0, STRING_KEY);
        c.bus.write32(TABLE + LIST_FILTER_ARG1, OTHER_KEY);
        c.bus.write32(TABLE + LIST_FILTER, filter);
        c.bus.write32(TABLE + LIST_COMPARE, comparator);
        c.bus
            .write8(STRING_KEY, if filter_sign { b'z' } else { b'a' });
        c.bus.write8(STRING_KEY + 1, 0);
        c.bus.write8(OTHER_KEY, b'm');
        c.bus.write8(OTHER_KEY + 1, 0);
        c.bus.write32(BUCKETS + 8, FREE_ENTRY);
        c.bus.write32(BUCKETS + 12, FREE_ENTRY);
        r
    }

    #[test]
    fn lobby_list_sort_matches() {
        check_paths(
            ee::s14::f_003ee620__generated,
            lobby_list_sort,
            8,
            &[19, 24, 43, 57, 70, 102, 114, 93],
            &|c, seed| {
                list_sort_fixture(c, seed);
            },
        );
    }

    /// The two filter keys read equal, so the real `strcmp` filter returns
    /// exactly 0: `visible` flips and the partition routes the entry to the
    /// hidden half (`triaged G row`).
    #[test]
    fn lobby_list_sort_hidden_matches() {
        lobby_list_test_hooks();
        check(
            ee::s14::f_003ee620__generated,
            lobby_list_sort,
            8,
            &|c, seed| {
                list_sort_fixture(c, seed);
                c.bus.write8(STRING_KEY, b'm');
                c.bus.write8(STRING_KEY + 1, 0);
                c.bus.write8(OTHER_KEY, b'm');
                c.bus.write8(OTHER_KEY + 1, 0);
                c.bus.write32(TABLE + LIST_FILTER, STRCMP_CALLBACK);
            },
        );
    }

    #[test]
    fn lobby_list_sort_events() {
        check_lobby_loop_events(
            ee::s14::f_003ee620__generated,
            lobby_list_sort,
            6,
            &|c, seed| {
                let mut r = list_sort_fixture(c, 6);
                let mode = schedule(c, &mut r, seed, 0x003e_e620);
                if mode != Events::Off {
                    let first_due = [48, 90][(seed / 3) as usize];
                    c.next_event = c.cycles + first_due;
                }
                mode
            },
        );
    }

    #[test]
    fn lobby_list_mark_dirty_matches() {
        check_paths(
            ee::s14::f_003ee600__generated,
            lobby_list_mark_dirty,
            8,
            &[4, 7],
            &|c, seed| {
                list_fixture(c, seed, 0x003e_e600);
                c.g[A1][0] = u64::from(seed % 2);
                c.bus.write32(
                    TABLE + LIST_NOTIFY_DIRTY,
                    [0, 2, u32::MAX, i32::MAX as u32][(seed / 2 % 4) as usize],
                );
            },
        );
    }

    #[test]
    fn lobby_list_set_sort_matches() {
        check_paths(
            ee::s14::f_003ee5a0__generated,
            lobby_list_set_sort,
            6,
            &[7, 8],
            &|c, seed| {
                list_fixture(c, seed, 0x003e_e5a0);
                c.bus.write32(
                    TABLE + LIST_ENTRY_COUNT,
                    [0, 1, u32::MAX][(seed % 3) as usize],
                );
            },
        );
    }

    #[test]
    fn lobby_list_get_at_matches() {
        check_paths(
            ee::s14::f_003ee560__generated,
            lobby_list_get_at,
            8,
            &[4, 6, 10, 13],
            &|c, seed| {
                list_fixture(c, seed, 0x003e_e560);
                c.g[A0][0] = if seed % 2 == 0 { 0 } else { sx(TABLE) };
                c.g[A1][0] = sx([u32::MAX, 0, 1, 2][(seed / 2) as usize]);
                c.bus.write32(TABLE + LIST_ENTRY_COUNT, 2);
            },
        );
    }

    pub(crate) const LIST_TEST_ENTRY: u32 = DATA + 0x100;

    pub(crate) const LIST_TEST_SLOTS: u32 = DATA + 0x200;

    pub(crate) const LIST_TEST_REMOVE: u32 = 0x003e_e480;

    pub(crate) fn lobby_list_fixture(c: &mut Cpu, seed: u32, entry: u32) -> Rng {
        let mut rng = Rng(0x4c69_7374 ^ u64::from(seed));
        scramble(c, &mut rng);
        fill(c, &mut rng, DATA, 0x100);
        c.g[A0][0] = sx(DATA);
        c.pc = entry;
        rng
    }

    #[test]
    fn lobby_list_get_visible_count_matches() {
        check_paths(
            ee::s14::f_003ee558__generated,
            lobby_list_get_visible_count,
            24,
            &[2],
            &|c, seed| {
                lobby_list_fixture(c, seed, 0x003e_e558);
                c.bus.write32(DATA + LIST_VISIBLE_COUNT, seed);
                c.bus.write32(DATA + LIST_ENTRY_COUNT, seed + 1);
            },
        );
    }

    #[test]
    fn lobby_list_get_count_matches() {
        check_paths(
            ee::s14::f_003ee550__generated,
            lobby_list_get_count,
            24,
            &[2],
            &|c, seed| {
                lobby_list_fixture(c, seed, 0x003e_e550);
                c.bus.write32(DATA + LIST_ENTRY_COUNT, seed);
                c.bus.write32(DATA + LIST_VISIBLE_COUNT, seed + 1);
            },
        );
    }

    pub(crate) fn lobby_list_remove_fixture(
        c: &mut Cpu,
        seed: u32,
        count: u32,
        index: u32,
        visible: u32,
    ) -> Rng {
        let rng = lobby_list_fixture(c, seed, LIST_TEST_REMOVE);
        c.bus.write32(DATA + LIST_ENTRY_COUNT, count);
        c.bus.write32(DATA + LIST_VISIBLE_COUNT, visible);
        c.bus.write32(DATA + LIST_ENTRIES, LIST_TEST_SLOTS);
        c.bus.write32(DATA + LIST_FREE_HEAD, LIST_TEST_ENTRY + 0x40);
        for i in 0..4 {
            c.bus.write32(
                LIST_TEST_SLOTS + i * LIST_ENTRY_POINTER_BYTES,
                LIST_TEST_ENTRY + i * 0x10,
            );
        }
        c.g[A1][0] = sx(LIST_TEST_ENTRY + index * 0x10);
        rng
    }

    #[test]
    fn lobby_list_remove_matches() {
        check_paths(
            ee::s14::f_003ee480__generated,
            lobby_list_remove,
            24,
            &[9, 18, 36, 26, 30, 54, 58, 47, 51],
            &|c, seed| {
                let (count, index) = match seed % 6 {
                    0 => (0, 0),
                    1 => (1, 4),
                    2 => (4, 4),
                    3 => (1, 0),
                    4 => (4, 1),
                    _ => (4, 3),
                };
                let visible = if seed / 6 % 2 == 0 { 0 } else { count };
                lobby_list_remove_fixture(c, seed, count, index, visible);
            },
        );
    }

    driven!(lobby_list_remove_driven, lobby_list_remove);

    #[test]
    fn lobby_list_remove_events() {
        check_events(
            ee::s14::f_003ee480__generated,
            lobby_list_remove_driven,
            12,
            &|c, seed| {
                let (index, delay) = if seed / 3 % 2 == 0 { (3, 14) } else { (0, 30) };
                let mut rng = lobby_list_remove_fixture(c, seed, 4, index, 4);
                let mode = schedule(c, &mut rng, seed, LIST_TEST_REMOVE);
                if mode != Events::Off {
                    c.next_event = c.cycles + delay;
                }
                mode
            },
        );
    }

    #[test]
    fn lobby_list_get_data_matches() {
        check_paths(
            ee::s14::f_003ee468__generated,
            lobby_list_get_data,
            24,
            &[4, 5],
            &|c, seed| {
                lobby_list_fixture(c, seed, 0x003e_e468);
                c.g[A1][0] = if seed % 2 == 0 {
                    0
                } else {
                    sx(LIST_TEST_ENTRY)
                };
            },
        );
    }

    pub(crate) const LIST_TEST_UPDATE: u32 = 0x003e_e3e0;

    pub(crate) const LIST_TEST_ADD: u32 = 0x003e_e320;

    pub(crate) const LIST_TEST_DESTROY: u32 = 0x003e_e260;

    pub(crate) const LIST_TEST_CREATE: u32 = 0x003e_e1e8;

    pub(crate) const LIST_TEST_COMPARE_THUNK: u32 = 0x003e_e1b0;

    pub(crate) const LIST_TEST_GROW: u32 = 0x003e_e028;

    pub(crate) const LIST_TEST_FILTER: u32 = 0x0041_65a8;

    pub(crate) const LIST_TEST_GATE: u32 = DATA + 0x5_4000;

    pub(crate) const LIST_TEST_MAGIC: u32 = 0x1e82_65d1;

    pub(crate) const LIST_TEST_CALLBACKS: u32 = LIST_TEST_GATE + 4;

    pub(crate) const LIST_TEST_UNWIND_AT: u32 = LIST_TEST_GATE + 8;

    pub(crate) const LIST_TEST_ALLOCS: u32 = LIST_TEST_GATE + 12;

    pub(crate) const LIST_TEST_ALLOC_RESULTS: u32 = LIST_TEST_GATE + 16;

    pub(crate) const LIST_TEST_FREED: u32 = LIST_TEST_GATE + 28;

    pub(crate) const LIST_TEST_CHUNK: u32 = DATA + 0x180;

    pub(crate) const LIST_TEST_NEW_CHUNK: u32 = DATA + 0x5_4100;

    pub(crate) const LIST_TEST_NEW_SLOTS: u32 = DATA + 0x5_4200;

    pub(crate) const LIST_TEST_NEW_LIST: u32 = DATA + 0x5_4300;

    pub(crate) fn lobby_list_test_callback(c: &mut Cpu) {
        if c.bus.read32(LIST_TEST_GATE) != LIST_TEST_MAGIC {
            return fall_back(
                c,
                prev_cell(lobby_list_test_callback),
                ee::s15::f_004165a8__generated,
            );
        }
        let index = c.bus.read32(LIST_TEST_CALLBACKS);
        c.bus.write32(LIST_TEST_CALLBACKS, index + 1);
        c.cycles += 3;
        c.g[V0][0] = c.g[A2][0];
        if index == c.bus.read32(LIST_TEST_UNWIND_AT) {
            c.pc = 0xdead_0040;
        } else {
            ret(c);
        }
    }

    pub(crate) fn lobby_list_test_alloc(c: &mut Cpu) {
        if c.bus.read32(LIST_TEST_GATE) != LIST_TEST_MAGIC {
            return fall_back(
                c,
                prev_cell(lobby_list_test_alloc),
                ee::s10::f_00317e30__generated,
            );
        }
        let index = c.bus.read32(LIST_TEST_ALLOCS);
        c.bus.write32(LIST_TEST_ALLOCS, index + 1);
        c.g[V0][0] = sx(c
            .bus
            .read32(LIST_TEST_ALLOC_RESULTS + index * LIST_ENTRY_POINTER_BYTES));
        c.cycles += 1;
        ret(c);
    }

    pub(crate) fn lobby_list_test_free(c: &mut Cpu) {
        if c.bus.read32(LIST_TEST_GATE) != LIST_TEST_MAGIC {
            return fall_back(
                c,
                prev_cell(lobby_list_test_free),
                ee::s10::f_00317e98__generated,
            );
        }
        let ptr = r32(c, A0);
        c.bus.write32(LIST_TEST_FREED, ptr);
        c.cycles += 1;
        ret(c);
    }

    pub(crate) fn lobby_list_test_hooks() {
        use std::ptr::addr_of_mut;
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_004165A8),
            lobby_list_test_callback,
            prev_cell(lobby_list_test_callback),
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            lobby_list_test_alloc,
            prev_cell(lobby_list_test_alloc),
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            lobby_list_test_free,
            prev_cell(lobby_list_test_free),
        );
    }

    pub(crate) fn lobby_list_call_fixture(c: &mut Cpu, seed: u32, entry: u32) -> Rng {
        let mut rng = lobby_list_fixture(c, seed, entry);
        c.bus.write32(LIST_TEST_GATE, LIST_TEST_MAGIC);
        c.bus.write32(LIST_TEST_UNWIND_AT, u32::MAX);
        c.bus.write32(DATA + LIST_FILTER_ARG0, 0x1234_5678);
        c.bus.write32(DATA + LIST_FILTER_ARG1, 0x8765_4321);
        fill(c, &mut rng, LIST_TEST_NEW_CHUNK, 0x100);
        rng
    }

    pub(crate) fn lobby_list_alloc_results(c: &mut Cpu, results: &[u32]) {
        for (index, &result) in results.iter().enumerate() {
            c.bus.write32(
                LIST_TEST_ALLOC_RESULTS + index as u32 * LIST_ENTRY_POINTER_BYTES,
                result,
            );
        }
    }

    #[test]
    fn lobby_list_update_matches() {
        lobby_list_test_hooks();
        check_paths(
            ee::s14::f_003ee3e0__generated,
            lobby_list_update,
            24,
            &[20, 28, 39, 35, 19, 29],
            &|c, seed| {
                lobby_list_call_fixture(c, seed, LIST_TEST_UPDATE);
                let filter = seed % 4 != 0;
                // The filter test is "greater than zero", so the stored data also takes 0.
                let old = match seed % 4 {
                    1 => 1,
                    _ if seed / 12 % 2 == 1 => 0,
                    _ => u32::MAX,
                };
                let new = if seed % 4 == 2 { 1 } else { 0 };
                c.bus.write32(
                    DATA + LIST_FILTER,
                    if filter { LIST_TEST_FILTER } else { 0 },
                );
                c.bus.write32(LIST_TEST_ENTRY, old);
                c.g[A1][0] = sx(LIST_TEST_ENTRY);
                c.g[A2][0] = sx(new);
                match seed / 4 % 3 {
                    1 => c.bus.write32(LIST_TEST_UNWIND_AT, 0),
                    2 => c.bus.write32(LIST_TEST_UNWIND_AT, 1),
                    _ => {}
                }
            },
        );
    }

    #[test]
    fn lobby_list_add_matches() {
        lobby_list_test_hooks();
        check_paths(
            ee::s14::f_003ee320__generated,
            lobby_list_add,
            24,
            &[36, 45, 41, 35, 177, 67],
            &|c, seed| {
                lobby_list_call_fixture(c, seed, LIST_TEST_ADD);
                c.bus.write32(DATA + LIST_FREE_HEAD, LIST_TEST_ENTRY);
                c.bus.write32(DATA + LIST_FREE_COUNT, 1);
                // The new entry's slot is entries + count * 4, so the plain adds start from a
                // nonzero count; the grow arms start empty because a grow copies the old slots.
                let count = if seed % 6 < 4 { seed / 6 % 3 } else { 0 };
                c.bus.write32(DATA + LIST_ENTRY_COUNT, count);
                c.bus.write32(DATA + LIST_ENTRIES, LIST_TEST_SLOTS);
                c.bus.write32(DATA + LIST_CHUNKS, 0);
                c.bus.write32(DATA + LIST_INITIAL_SIZE, 0);
                c.bus.write32(DATA + LIST_GROW_PERCENT, 0);
                c.bus.write32(LIST_TEST_ENTRY, 0);
                c.bus.write32(
                    DATA + LIST_FILTER,
                    if matches!(seed % 6, 1..=3) {
                        LIST_TEST_FILTER
                    } else {
                        0
                    },
                );
                c.g[A1][0] = sx(match (seed % 6, seed / 6 % 3) {
                    (2, _) => u32::MAX,
                    (_, 2) => 0,
                    _ => seed + 1,
                });
                if seed % 6 == 3 {
                    c.bus.write32(LIST_TEST_UNWIND_AT, 0);
                }
                if seed % 6 >= 4 {
                    c.bus.write32(DATA + LIST_FREE_HEAD, 0);
                    c.bus.write32(DATA + LIST_FREE_COUNT, 0);
                    c.bus.write32(DATA + LIST_ENTRIES, 0);
                    let chunk = if seed % 6 == 4 {
                        LIST_TEST_NEW_CHUNK
                    } else {
                        0
                    };
                    lobby_list_alloc_results(c, &[chunk, LIST_TEST_NEW_SLOTS]);
                }
            },
        );
    }

    pub(crate) fn lobby_list_destroy_fixture(
        c: &mut Cpu,
        seed: u32,
        chunks: u32,
        slots: bool,
    ) -> Rng {
        let rng = lobby_list_call_fixture(c, seed, LIST_TEST_DESTROY);
        c.bus.write32(
            DATA + LIST_CHUNKS,
            if chunks == 0 { 0 } else { LIST_TEST_CHUNK },
        );
        c.bus.write32(
            LIST_TEST_CHUNK,
            if chunks > 1 {
                LIST_TEST_CHUNK + 0x10
            } else {
                0
            },
        );
        c.bus.write32(LIST_TEST_CHUNK + 0x10, 0);
        c.bus
            .write32(DATA + LIST_ENTRIES, if slots { LIST_TEST_SLOTS } else { 0 });
        rng
    }

    #[test]
    fn lobby_list_destroy_matches() {
        lobby_list_test_hooks();
        check_paths(
            ee::s14::f_003ee260__generated,
            lobby_list_destroy,
            12,
            &[30, 33, 46, 49, 61, 64],
            &|c, seed| {
                lobby_list_destroy_fixture(c, seed, seed % 3, seed / 3 % 2 != 0);
            },
        );
    }

    driven!(lobby_list_destroy_driven, lobby_list_destroy);

    #[test]
    fn lobby_list_destroy_events() {
        lobby_list_test_hooks();
        check_events(
            ee::s14::f_003ee260__generated,
            lobby_list_destroy_driven,
            6,
            &|c, seed| {
                let mut rng = lobby_list_destroy_fixture(c, seed, 2, true);
                let mode = schedule(c, &mut rng, seed, LIST_TEST_DESTROY);
                if mode != Events::Off {
                    c.next_event = c.cycles + 22;
                }
                mode
            },
        );
    }

    #[test]
    fn lobby_list_create_matches() {
        lobby_list_test_hooks();
        check_paths(
            ee::s14::f_003ee1e8__generated,
            lobby_list_create,
            12,
            &[159, 251, 272],
            &|c, seed| {
                lobby_list_call_fixture(c, seed, LIST_TEST_CREATE);
                c.g[A0][0] = if seed % 2 == 0 { 0 } else { 4 };
                c.g[A1][0] = u64::from(seed + 2);
                c.g[A2][0] = u64::from(seed + 10);
                let chunk = if seed / 2 % 2 == 0 {
                    0
                } else {
                    LIST_TEST_NEW_CHUNK
                };
                lobby_list_alloc_results(c, &[LIST_TEST_NEW_LIST, chunk, LIST_TEST_NEW_SLOTS]);
            },
        );
    }

    #[test]
    fn lobby_list_compare_thunk_matches() {
        lobby_list_test_hooks();
        check_paths(
            ee::s14::f_003ee1b0__generated,
            lobby_list_compare_thunk,
            12,
            &[16],
            &|c, seed| {
                lobby_list_call_fixture(c, seed, LIST_TEST_COMPARE_THUNK);
                c.bus.write32(DATA + LIST_SORT_ARG0, 0x1122_3344);
                c.bus.write32(DATA + LIST_SORT_ARG1, 0xaabb_ccdd);
                c.bus.write32(DATA + LIST_COMPARE, LIST_TEST_FILTER);
                c.bus.write32(LIST_TEST_SLOTS, LIST_TEST_ENTRY);
                c.bus.write32(
                    LIST_TEST_SLOTS + LIST_ENTRY_POINTER_BYTES,
                    LIST_TEST_ENTRY + LIST_ENTRY_POINTER_BYTES,
                );
                c.bus.write32(LIST_TEST_ENTRY, seed.wrapping_sub(6));
                c.bus
                    .write32(LIST_TEST_ENTRY + LIST_ENTRY_POINTER_BYTES, seed + 3);
                c.g[A1][0] = sx(LIST_TEST_SLOTS);
                c.g[A2][0] = sx(LIST_TEST_SLOTS + LIST_ENTRY_POINTER_BYTES);
                if seed % 2 != 0 {
                    c.bus.write32(LIST_TEST_UNWIND_AT, 0);
                }
            },
        );
    }

    pub(crate) fn lobby_list_grow_fixture(c: &mut Cpu, seed: u32, kind: u32, fail: bool) -> Rng {
        let rng = lobby_list_call_fixture(c, seed, LIST_TEST_GROW);
        // The percent arm halves initial + growth with a rounding step that depends on the sign
        // and the low bit of the sum, so it takes an even, an odd and a negative initial size.
        let percent_initial = match seed / 8 % 3 {
            0 => 4,
            1 => 5,
            _ => -5i32 as u32,
        };
        let (initial, percent, count) = match kind {
            0 => (0, 0, 0),
            1 => (4, 0, 0),
            2 => (0, 100, 1),
            _ => (percent_initial, 50, 1),
        };
        c.bus.write32(DATA + LIST_INITIAL_SIZE, initial);
        c.bus.write32(DATA + LIST_GROW_PERCENT, percent);
        c.bus.write32(DATA + LIST_ENTRY_COUNT, count);
        c.bus.write32(DATA + LIST_FREE_COUNT, 0);
        c.bus.write32(DATA + LIST_FREE_HEAD, 0);
        c.bus.write32(
            DATA + LIST_CHUNKS,
            if count == 0 { 0 } else { LIST_TEST_CHUNK },
        );
        c.bus.write32(
            DATA + LIST_ENTRIES,
            if count == 0 { 0 } else { LIST_TEST_SLOTS },
        );
        c.bus.write32(LIST_TEST_SLOTS, LIST_TEST_ENTRY);
        let chunk = if fail { 0 } else { LIST_TEST_NEW_CHUNK };
        lobby_list_alloc_results(c, &[chunk, LIST_TEST_NEW_SLOTS]);
        rng
    }

    #[test]
    fn lobby_list_grow_matches() {
        lobby_list_test_hooks();
        check_paths_lohi(
            ee::s14::f_003ee028__generated,
            lobby_list_grow,
            24,
            &[46, 51, 53, 138, 159, 196, 235],
            &|c, seed| {
                lobby_list_grow_fixture(c, seed, seed % 4, seed / 4 % 2 != 0);
            },
        );
    }

    driven!(lobby_list_grow_driven, lobby_list_grow);

    #[test]
    fn lobby_list_grow_events() {
        lobby_list_test_hooks();
        check_events(
            ee::s14::f_003ee028__generated,
            lobby_list_grow_driven,
            6,
            &|c, seed| {
                let mut rng = lobby_list_grow_fixture(c, seed, 0, false);
                let mode = schedule(c, &mut rng, seed, LIST_TEST_GROW);
                if mode != Events::Off {
                    c.next_event = c.cycles + 98;
                }
                mode
            },
        );
    }
    use crate::difftest::fakes::machine;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::lobbyrig_arm;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::lobbyrig_mail;

    // ------------------------------------------------------------------
    // `Lobby_MemAlloc` (0x2591e8), `Lobby_MemFree` (0x259210).

    /// Allocating goes through the named operator new; freeing a null
    /// pointer returns at once, freeing a block calls the heap free.
    #[test]
    fn lobby_mem_alloc_matches() {
        check(
            ee::s06::f_002591e8__generated,
            lobby_mem_alloc,
            200,
            &|c, seed| {
                machine(c, seed, 0x2591_e8, 0);
                lobbyrig_arm(c);
                c.g[A0][0] = [0, 1, 7, 16, 64, 0x1000, 0x8000_0000u32.cast_signed()]
                    [seed as usize % 7] as u64;
                c.bus.write32(
                    lobbyrig_mail(0x0031_7e30),
                    if seed % 3 == 0 { 0 } else { DATA + 0x6000 },
                );
            },
        );
    }

    #[test]
    fn lobby_mem_free_matches() {
        check(
            ee::s06::f_00259210__generated,
            lobby_mem_free,
            200,
            &|c, seed| {
                machine(c, seed, 0x2592_10, 0);
                lobbyrig_arm(c);
                c.g[A0][0] = if seed % 4 == 0 {
                    0
                } else {
                    (DATA + 0x6000 + seed % 0x40) as u64
                };
                c.bus.write32(lobbyrig_mail(0x0031_7e98), 0);
            },
        );
    }

    // ------------------------------------------------------------------
    // `Lobby_HashStrCmpNoCase` (0x3eeab8).

    pub(crate) const HASH_CMP_A: u32 = DATA + 0x1000;

    pub(crate) const HASH_CMP_B: u32 = DATA + 0x1100;

    driven!(gen_hash_str_cmp_no_case, ee::s14::f_003eeab8__generated);

    driven!(hand_hash_str_cmp_no_case, lobby_hash_str_cmp_no_case);

    pub(crate) fn hash_cmp_put(c: &mut Cpu, at: u32, s: &[u8]) {
        for (i, &b) in s.iter().enumerate() {
            c.bus.write8(at + i as u32, b);
        }
        c.bus.write8(at + s.len() as u32, 0);
    }

    /// Case-insensitive lobby string compare: equal up to case and
    /// skippable blanks, ordered otherwise, empty against anything.
    #[test]
    fn lobby_hash_str_cmp_no_case_matches() {
        check_events(
            gen_hash_str_cmp_no_case,
            hand_hash_str_cmp_no_case,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3eea_b8, 0);
                // Fold table: blanks skip (1), upper case folds down,
                // everything else keys on itself, NUL terminates (0).
                for i in 0..256u32 {
                    let fold = if i == 0 {
                        0
                    } else if i == b' ' as u32 {
                        1
                    } else if (b'A' as u32..=b'Z' as u32).contains(&i) {
                        i + 32
                    } else {
                        i
                    };
                    c.bus.write8(
                        crate::game::dirtysock::tagsunk::HASH_CMP_FOLD_TABLE + i,
                        fold as u8,
                    );
                }
                match seed % 6 {
                    0 => {
                        hash_cmp_put(c, HASH_CMP_A, b"a b-C07");
                        hash_cmp_put(c, HASH_CMP_B, b"A  B-c07");
                    }
                    1 => {
                        hash_cmp_put(c, HASH_CMP_A, b"abc");
                        hash_cmp_put(c, HASH_CMP_B, b"abd");
                    }
                    2 => {
                        hash_cmp_put(c, HASH_CMP_A, b"ab");
                        hash_cmp_put(c, HASH_CMP_B, b"abc");
                    }
                    3 => {
                        hash_cmp_put(c, HASH_CMP_A, b"");
                        hash_cmp_put(c, HASH_CMP_B, b"");
                    }
                    4 => {
                        hash_cmp_put(c, HASH_CMP_A, b"Quick Match 12");
                        hash_cmp_put(c, HASH_CMP_B, b"quick match 13");
                    }
                    _ => {
                        hash_cmp_put(c, HASH_CMP_A, b" - ");
                        hash_cmp_put(c, HASH_CMP_B, b"");
                    }
                }
                let mode = schedule(c, &mut r, seed, 0x003e_eab8);
                c.g[A0][0] = sx(HASH_CMP_A);
                c.g[A1][0] = sx(HASH_CMP_B);
                if mode != Events::Off {
                    c.next_event = c.cycles + 4;
                }
                mode
            },
        );
    }
}
