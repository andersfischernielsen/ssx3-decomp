//! `src/render/ps2graphicsman.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::computer::sd;
pub(crate) use crate::game::fe::feasyncfile::at_2 as at;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::fe::fepopup::at as at_4;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::lq;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw_3;
pub(crate) use crate::game::fe::fepopup::sq;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw_2;
pub(crate) use crate::game::fe::festaterewards::lw_3 as lw_2;
pub(crate) use crate::game::fe::festatetrophyroom::r;
pub(crate) use crate::game::fe::festatetrophyroom::Ops;
pub(crate) use crate::game::fe::ovtemplatepausemenu::p;
pub(crate) use crate::game::fe::ovtemplatepausemenu::sw;
pub(crate) use crate::game::movie::movieplayer::add32;
pub(crate) use crate::game::movie::movieplayer::at as at_2;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::render::graphicsman::RegMem;
pub(crate) use crate::game::render::particle::deleting_dtor;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0036_7150,
        tex_man_alloc_nb_tex_info,
        "TexMan_AllocNBTexInfo",
    ),
    (
        0x0036_71c8,
        tex_man_free_nb_tex_info,
        "TexMan_FreeNBTexInfo",
    ),
    (0x0036_7260, tex_man_alloc_btex_info, "TexMan_AllocBTexInfo"),
    (0x0036_72c0, tex_man_free_btex_info, "TexMan_FreeBTexInfo"),
    (0x0036_7360, tex_man_shutdown, "TexMan_Shutdown"),
    (0x0036_73d0, tex_man_end_frame_stub, "TexMan_EndFrameStub"),
    (
        0x0036_73d8,
        tex_man_alloc_frame_buffer,
        "TexMan_AllocFrameBuffer",
    ),
    (0x0036_7b60, tex_man_unlock_vram, "TexMan_UnlockVram"),
    (0x0036_7cd0, tex_man_lock_texture, "TexMan_LockTexture"),
    (
        0x0036_8138,
        tex_man_emit_path3_window,
        "TexMan_EmitPath3Window",
    ),
    (
        0x0036_84f0,
        tex_man_emit_clut_upload,
        "TexMan_EmitClutUpload",
    ),
    (0x0036_8660, tex_man_bind_textures, "TexMan_BindTextures"),
    (0x0036_9130, tex_man_set_tex1_filter, "TexMan_SetTex1Filter"),
    (
        0x0036_91b0,
        tex_man_set_tex_function,
        "TexMan_SetTexFunction",
    ),
    (0x0036_95d8, graphics_man_dtor, "GraphicsMan_dtor"),
    (
        0x0036_9610,
        graphics_man_set_color_mask,
        "GraphicsMan_SetColorMask",
    ),
    (
        0x0036_9690,
        ps2_graphics_man_set_texture_name_cache,
        "PS2GraphicsMan_SetTextureNameCache",
    ),
    (
        0x0036_9890,
        ps2_graphics_man_set_shape_banks,
        "PS2GraphicsMan_SetShapeBanks",
    ),
    (
        0x0036_98a0,
        ps2_graphics_man_free_blended_matrix_history,
        "PS2GraphicsMan_FreeBlendedMatrixHistory",
    ),
    (0x0036_7bc0, tex_man_free_texture, "TexMan_FreeTexture"),
    (0x0036_7440, tex_man_load_texture, "TexMan_LoadTexture"),
    (
        0x0036_96c8,
        ps2_graphics_man_get_texture_by_name,
        "PS2GraphicsMan_GetTextureByName",
    ),
    (0x0036_7340, tex_man_init_temp_heap, "TexMan_InitTempHeap"),
    (0x0036_91f8, graphics_man_ctor, "GraphicsMan_ctor"),
    (
        0x0036_7230,
        tex_man_destroy_nb_tex_info_array,
        "TexMan_DestroyNBTexInfoArray",
    ),
    (
        0x0036_7310,
        texman_destroy_btexinfo_array,
        "TexMan_DestroyBTexInfoArray",
    ),
    (
        0x0036_7d20,
        texman_update_texture_data,
        "TexMan_UpdateTextureData",
    ),
    (
        0x0036_8170,
        tex_man_emit_image_upload,
        "TexMan_EmitImageUpload",
    ),
    (0x0036_8970, texman_emit_tex0, "TexMan_EmitTex0"),
    (0x0036_9098, texman_evict_texture, "TexMan_EvictTexture"),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {

    pub(crate) use crate::game::ai::rider::callee::TEX_MAN_EVICT_TEXTURE;
    pub(crate) use crate::game::ai::rider::callee::VRAM_HEAP_FREE_TEXTURE_BLOCKS;

    pub(crate) use crate::game::fe::fepopup::callee::C00366CE0;
    pub(crate) use crate::game::fe::fepopup::callee::C003684F0;
    pub(crate) use crate::game::fe::fepopup::callee::C0038F460;
    pub(crate) use crate::game::fe::fepopup::callee::C0038F668;

    pub(crate) use crate::game::fe::fepopup::callee::C00368170;
    pub(crate) use crate::game::fe::fepopup::callee::C00368970;

    pub(crate) use crate::game::fe::festatetrophyroom::callee::MEM_MAN_FREE;

    pub(crate) use crate::game::wscript::wscriptman::callee::TEX_MAN_LOAD_TEXTURE;
}

/// `GetHashValue32` (0x317670) and the UI object lookups, which the generator
/// emitted as cross-shard `call_indirect`s.
pub(crate) mod table_callee {

    pub(crate) use crate::game::fe::festaterewards::table_callee::UPLOAD_TEXTURE_PIXELS;
}

// ---------------------------------------------------------------------------
// Texture manager.

/// `void TexMan_UnlockVram(TexMan *man, int index)` (0x367b60): mark the
/// texture's vram lock released.
pub fn tex_man_unlock_vram(c: &mut Cpu) {
    c.cycles += 8;
    c.g[A3][0] = c.g[A0][0];
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V1][0] = u64::MAX;
    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x28));
    let t = c.g[V0][0] == c.g[V1][0];
    c.g[V1][0] = 0x1c;
    if t {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 13;
    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x30));
    c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
    c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1f60));
    c.g[A1][0] = sx(abi::mult(c, c.g[A1][0] as u32, 0x1c));
    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x4350));
    c.g[A0][0] ^= 0x9;
    c.g[A3][0] = 0xffff_ffff_ffff_fffd;
    if c.g[A0][0] != 0 {
        c.g[V0][0] = c.g[A2][0];
    }
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1ff0));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
    c.g[V0][0] &= c.g[A3][0];
    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
    c.cycles += 2;
    abi::ret(c);
}

/// `void TexMan_EmitPath3Window(TexMan *man, void **cursor)` (0x368138):
/// append a path3 window packet and advance the cursor.
pub fn tex_man_emit_path3_window(c: &mut Cpu) {
    c.cycles += 14;
    c.g[V0][0] = 0x450000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4e00));
    c.g[A0][0] = c.lw(c.g[A1][0] as u32);
    c.g[V1][0] = 0x3000_0000;
    c.g[V1][0] |= 0x22;
    c.g[V0][0] = c.g[V0][0] << 32;
    c.g[V0][0] |= c.g[V1][0];
    c.sd(c.g[A0][0] as u32, c.g[V0][0]);
    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
    c.sd((c.g[V1][0] as u32).wrapping_add(0x8), 0);
    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
    c.sw(c.g[A1][0] as u32, c.g[V0][0]);
    abi::ret(c);
}

/// `void TexMan_SetTex1Filter(TexMan *man, int index, uint a2, uint a3,
/// uint t0, uint t1)` (0x369130): rewrite the tex1 filter word.
pub fn tex_man_set_tex1_filter(c: &mut Cpu) {
    c.cycles += 31;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[V1][0] = 0xffff_ffff_ffe7_0000;
    c.g[V1][0] |= 0xffff;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[T0][0] &= 0x3;
    c.g[T2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
    c.g[T0][0] = c.g[T0][0] << 19;
    c.g[T1][0] &= 0xfff;
    c.g[A0][0] = 0xffff_ffff_ffff_f000;
    c.g[A0][0] = c.g[A0][0] << 16;
    c.g[A0][0] |= 0xffff;
    c.g[A0][0] = c.g[A0][0] << 16;
    c.g[A0][0] |= 0xffff;
    c.g[V0][0] = c.ld((c.g[T2][0] as u32).wrapping_add(0x40));
    c.g[T1][0] = c.g[T1][0] << 32;
    c.g[A2][0] &= 0x7;
    c.g[A1][0] = 0xffff_ffff_ffff_fe3f;
    c.g[V0][0] &= c.g[V1][0];
    c.g[A2][0] = c.g[A2][0] << 6;
    c.g[V0][0] |= c.g[T0][0];
    c.g[A3][0] &= 0x1;
    c.g[V0][0] &= c.g[A0][0];
    c.g[V1][0] = 0xffff_ffff_ffff_ffdf;
    c.g[V0][0] |= c.g[T1][0];
    c.g[A3][0] = c.g[A3][0] << 5;
    c.g[V0][0] &= c.g[A1][0];
    c.g[V0][0] |= c.g[A2][0];
    c.g[V0][0] &= c.g[V1][0];
    c.g[V0][0] |= c.g[A3][0];
    c.sd((c.g[T2][0] as u32).wrapping_add(0x40), c.g[V0][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Texture manager

/// `void TexMan_SetTexFunction(TexMan *man, int index, uint mode)` (0x3691b0):
/// if `index` is not -1, replace bits 32..34 of the function pointer stored at
/// `man + index*4 + 8`'s `+0x38` with `mode & 3`.
pub fn tex_man_set_tex_function(c: &mut Cpu) {
    c.cycles += 3;
    let index = c.g[A1][0];
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    let is_current = index == 0xffff_ffff_ffff_ffff;
    c.g[V0][0] = sx((index as u32) << 2);
    if !is_current {
        c.cycles += 13;
        c.g[V1][0] = c.g[A2][0] & 0x3;
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let mut mask = 0xffff_ffff_ffff_ffe7u64;
        mask <<= 16;
        mask |= 0xffff;
        mask <<= 16;
        mask |= 0xffff;
        c.g[A1][0] = mask;
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x8).lw(c);
        c.g[V1][0] = c.g[V1][0] << 35;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x38).ld(c);
        c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        Addr(c.g[A0][0] as u32).off(0x38).sd(c, c.g[V0][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// PS2 graphics manager

/// `void PS2GraphicsMan_SetShapeBanks(PS2GraphicsMan *man, uint a, uint b)`
/// (0x369890): store the two banks and clear the count.
pub fn ps2_graphics_man_set_shape_banks(c: &mut Cpu) {
    c.cycles += 4;
    let man = Addr(abi::arg(c, 0));
    man.off(0x27c).sw(c, c.g[A2][0]);
    man.off(0x278).sw(c, c.g[A1][0]);
    man.off(0xe80).sw(c, 0);
    abi::ret(c);
}

/// `GraphicsMan::~GraphicsMan()` (0x3695d8): the base graphics manager
/// destructor (`operator delete` when the low flag bit is set).
pub fn graphics_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        deleting_dtor(c, 0x0036_9600, 0x10d8, 0x0049_3950, 0x2a90)
    })
}

/// `EA_MemCopy` (0x3e6574).
pub(crate) const MEM_COPY: u32 = 0x003e_6574;

/// `void PS2GraphicsMan_SetTextureNameCache(this, count, const void* src)`
/// (0x369690): store `count` at `+0xe80` and copy `count` 12-byte entries
/// into the array at `+0x280`.
pub fn ps2_graphics_man_set_texture_name_cache(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[A3][0] = c.g[A1][0];
        c.g[V1][0] = 0xc;
        c.g[A1][0] = c.g[A2][0];
        let frame = Frame::open(c, 0x10);
        c.mult(0, 0, 7, 3);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[V0][0] = this;
        c.g[A0][0] = sx((this as u32).wrapping_add(0x280));
        c.g[A2][0] = c.lo[0];
        let count = c.g[A3][0];
        c.sw((this as u32).wrapping_add(0xe80), count);
        call_ptr(c, MEM_COPY, 0x0036_96bc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void PS2GraphicsMan_FreeBlendedMatrixHistory(PS2GraphicsMan* self)`
/// (0x3698A0): free the blended-matrix history buffer at +0xF48 and clear its
/// count.
pub fn ps2_graphics_man_free_blended_matrix_history(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let buffer = c.lw_r_2(A0, r(c, S0).wrapping_add(0xf48));
        if buffer == 0 {
            Addr(r(c, S0).wrapping_add(0xf4c)).sw(c, 0);
        } else {
            c.cycles += 3;
            callee::MEM_MAN_FREE.call(c, 0x0036_98c4)?;
            Addr(r(c, S0).wrapping_add(0xf4c)).sw(c, 0);
        }
        c.cycles += 5;
        Addr(r(c, S0).wrapping_add(0xf48)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void TexMan_FreeBTexInfo(TexMan* self, int slot)` (0x3672c0): free the
/// BTexInfo in `slots[slot]` and decrement the live count.
pub fn tex_man_free_btex_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = sx(r32(c, A1) << 2);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(r32(c, S1).wrapping_add(8));
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, A1)));
        c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        call_ptr(c, 0x0031_7e50, 0x0036_72e8)?;
        Addr(r32(c, S0)).sw(c, 0);
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
        Addr(r32(c, S1)).sw(c, c.g[V0][0]);
        frame.restore(c, S1, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void TexMan_LockTexture(int a0, int a1, int a2, int a3)` (0x367cd0): load
/// a texture with the renderer's default lock parameters.
pub fn tex_man_lock_texture(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        c.g[T0][0] = c.g[A2][0];
        c.g[T2][0] = c.g[A3][0];
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = u64::MAX;
        c.g[A3][0] = c.g[A1][0];
        c.g[V0][0] = 0x3;
        c.g[A2][0] = 0x4a_4068; // D_004A4068
        frame.save_ra(c, 0x20);
        Addr(frame.at(c, 8)).sw(c, c.g[V0][0]);
        c.g[A1][0] = 0;
        Addr(frame.at(c, 0x18)).sw(c, c.g[V1][0]);
        Addr(frame.at(c, 0)).sw(c, 0);
        c.g[T1][0] = 0;
        Addr(frame.at(c, 0x10)).sw(c, 0);
        c.g[RA][0] = sx(0x0036_7d14);
        c.g[T3][0] = 0;
        callee::TEX_MAN_LOAD_TEXTURE.call(c, 0x0036_7d14)?;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Textures and VRAM.

/// `void TexMan_AllocBTexInfo(TexMan* self, int id)` (0x367260): allocate one
/// 0x58-byte BTexInfo for slot `id` and bump the manager's counter.
pub fn tex_man_alloc_btex_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save(c, S1, 0x10);
        c.g[A1][0] = 0x0049_0000;
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(8));
        frame.save_ra(c, 0);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2078));
        c.g[A0][0] = 0x58;
        c.g[A2][0] = 0x2100_0000;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0036_7298)?;
        Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = lw(c, c.g[S1][0] as u32);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        Addr(c.g[S1][0] as u32).sw(c, c.g[V0][0]);
        frame.restore(c, S1, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `TexMan_FreeNBTexInfo(TexMan* self, int id)` (0x3671c8): free texture
/// slot `id` and push it onto the free list, then drop the live count.
pub fn tex_man_free_nb_tex_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        let this = c.g[A0][0];
        c.g[S2][0] = this;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        let id = c.g[A1][0];
        c.g[S0][0] = sx((id as u32) << 2);
        frame.save_ra(c, 0);
        c.g[S1][0] = sx((this as u32).wrapping_add(8));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[A0][0] = lw(c, c.g[S1][0] as u32);
        call_ptr(c, 0x0031_7e50, 0x0036_71f4)?;
        c.g[V0][0] = lw(c, p(this as u32, 0x1f48));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(8));
        c.g[S0][0] = sx((this as u32).wrapping_add(c.g[S0][0] as u32));
        frame.restore_ra(c, 0);
        sw(c, c.g[S1][0] as u32, c.g[V0][0]);
        sw(c, p(this as u32, 0x1f48), c.g[S0][0]);
        c.g[V0][0] = lw(c, p(this as u32, 4));
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        frame.restore(c, S1, 0x20);
        sw(c, p(this as u32, 4), c.g[V0][0]);
        frame.restore(c, S2, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int TexMan_AllocFrameBuffer(TexMan* self, int size)` (0x3673d8): allocate
/// a frame buffer from the VRAM heap, tag its 0x1c-byte entry and return the
/// entry index.
pub fn tex_man_alloc_frame_buffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        frame.save(c, S0, 0x10);
        c.g[A3][0] = 0xffff_ffff_ffff_ffff;
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x1f60));
        c.g[T0][0] = 0xffff_ffff_ffff_ffff;
        c.g[T1][0] = 1;
        call_ptr(c, 0x0036_6ce0, 0x0036_7400)?;
        c.g[V1][0] = 0x1c;
        c.g[A1][0] = lw(c, p(this as u32, 0x3f50));
        c.mult(0, V0, V0, V1);
        c.g[A0][0] = 0x7fff_ffff;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = lw(c, c.g[A1][0] as u32);
        sw(c, p(c.g[A1][0] as u32, 8), c.g[A0][0]);
        c.g[V1][0] = c.g[V1][0] | 0x22;
        c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32 >> 13) as u32);
        sw(c, c.g[A1][0] as u32, c.g[V1][0]);
        frame.close(c);
        Ok(())
    })
}

/// `void TexMan_EndFrameStub(void)` (0x3673d0): texture manager end-of-frame
/// hook, not installed.
pub fn tex_man_end_frame_stub(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

/// `void TexMan_Shutdown(TexMan* self)` (0x367360): destroy both texture-info
/// arrays, free the VRAM heaps and invalidate the resident-texture table.
pub fn tex_man_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0036_7230, 0x0036_7374)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0036_7310, 0x0036_737c)?;
        c.g[A0][0] = sx(at(c, S0, 0x1f60));
        call_ptr(c, 0x0036_6548, 0x0036_7384)?;
        c.g[A0][0] = sx(at(c, S0, 0x4350));
        call_ptr(c, 0x0036_6548, 0x0036_738c)?;
        c.g[A0][0] = u64::MAX;
        c.g[V1][0] = 1;
        c.g[V0][0] = sx(at(c, S0, 0x1f50));
        loop {
            c.cycles += 7;
            Addr(c.g[V0][0] as u32).sw(c, c.g[A0][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0036_7398);
            }
        }
        c.cycles += 6;
        c.g[V0][0] = u64::MAX;
        frame.restore_ra(c, 0);
        Addr(at(c, S0, 0x1f54)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `func_00367150` (0x367150): `TexMan_AllocNBTexInfo`. Pops a free texture
/// info node from the list at `+0x1f48`, works out its slot index
/// (`(node - 8 - this) / 4`), bumps the live count at `+4`, allocates a fresh
/// 0x58-byte node (`EA_MemAllocNamed(size, 0x492068, 0x21000000, 0)`) and
/// stores it in the slot. Returns the slot index. Signature:
/// `int alloc(TexMan* this)`.
pub fn tex_man_alloc_nb_tex_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = 0x49_0000;
        frame.save(c, S0, 0x20);
        c.g[A1][0] = sx(low(c, A1).wrapping_add(0x2068));
        frame.save(c, S1, 0x10);
        c.g[A3][0] = 0;
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(low(c, A0).wrapping_add(0x8));
        c.g[V0][0] = lw_r(c, A0, 0x1f48);
        c.g[V1][0] = lw_r(c, A0, 0x4);
        c.g[S0][0] = sx(low(c, V0).wrapping_sub(0x8));
        c.g[A2][0] = Addr(low(c, V0)).lw(c);
        c.g[S0][0] = sx(low(c, S0).wrapping_sub(low(c, A0)));
        c.g[V1][0] = sx(low(c, V1).wrapping_add(0x1));
        c.g[S0][0] = sx(((low(c, S0) as i32) >> 2) as u32);
        sw_r(c, A0, 0x1f48, c.g[A2][0]);
        c.g[V0][0] = sx(low(c, S0) << 2);
        sw_r(c, A0, 0x4, c.g[V1][0]);
        c.g[S1][0] = sx(low(c, S1).wrapping_add(low(c, V0)));
        c.g[A0][0] = 0x58;
        c.g[RA][0] = sx(0x0036_71a8);
        c.g[A2][0] = 0x2100_0000;
        call_ptr(c, 0x0031_7d70, 0x0036_71a8)?;
        sw_r(c, S1, 0, c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void GraphicsMan_SetColorMask(man, mask)` (0x00369610): invert the colour
/// mask into four words and set the two blend words.
pub fn graphics_man_set_color_mask(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(word(c, A3).wrapping_add(0xed8));
        c.g[V1][0] = c.g[A1][0];
        c.g[A2][0] = 0x3;
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(at_2(c, V1, 0)).lw(c);
            c.g[A2][0] = sx(word(c, A2).wrapping_sub(1));
            c.g[V1][0] = sx(word(c, V1).wrapping_add(4));
            c.g[V0][0] = !(0u64 | c.g[V0][0]);
            Addr(at_2(c, A0, 0)).sw(c, c.g[V0][0]);
            let more = (c.g[A2][0] as i64) >= 0;
            c.g[A0][0] = sx(word(c, A0).wrapping_add(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0036_9628);
                }
                continue;
            }
            c.cycles += 3;
            c.g[A0][0] = 0;
            c.g[A2][0] = 0x7fff;
            c.g[V1][0] = sx(word(c, A3).wrapping_add(0xee8));
            loop {
                c.cycles += 7;
                Addr(at_2(c, V1, 0)).sh(c, c.g[A2][0]);
                c.g[A0][0] = sx(word(c, A0).wrapping_add(1));
                c.g[V0][0] = (c.g[A0][0] < 2u64) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = sx(word(c, V1).wrapping_add(2));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0036_9650);
                    }
                    continue;
                }
                c.cycles += 8;
                c.g[V0][0] = Addr(at_2(c, A3, 0x10d8)).lw(c);
                c.g[A0][0] = Addr(at_2(c, V0, 0x1f0)).lh(c);
                c.g[V1][0] = Addr(at_2(c, V0, 0x1f4)).lw(c);
                c.g[A0][0] = add32(word(c, A3), word(c, A0));
                call_ptr(c, word(c, V1), 0x0036_9680)?;
                c.g[RA][0] = c.ld(frame.at(c, 0));
                frame.close(c);
                break;
            }
            break;
        }
        Ok(())
    })
}

/// `void TexMan_UpdateTextureData(self, index, data, clut, unused, upload)`
/// (0x367d20): upload the texture and/or CLUT pixels and then evict the slot.
pub fn tex_man_update_texture_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S1, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
        frame.save(c, S0, 0x40);
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = Addr(c.g[SP][0] as u32).off(0x58).lw(c);
        c.g[A1][0] = c.g[A2][0];
        c.g[S0][0] = Addr(c.g[V0][0] as u32).off(0x8).lw(c);
        let no_upload = c.g[V1][0] == 0;
        c.g[S3][0] = Addr(c.g[SP][0] as u32).off(0x50).lw(c);
        if !no_upload {
            c.cycles += 2;
            if c.g[A1][0] != 0 {
                c.cycles += 3;
                c.g[A0][0] = lw_2(c, S0, 0x1c);
                c.g[A2][0] = lw_2(c, S0, 0x10);
                call_ptr(c, table_callee::UPLOAD_TEXTURE_PIXELS, 0x0036_7d70)?;
            }
            c.cycles += 2;
            let no_clut = c.g[S3][0] == 0;
            c.g[A2][0] = 4;
            if !no_clut {
                c.cycles += 5;
                c.g[V0][0] = lw_2(c, S0, 0x18);
                c.g[A0][0] = lw_2(c, S0, 0x20);
                c.g[A1][0] = c.g[S3][0];
                c.g[A2][0] = sx((c.g[A2][0] as u32) << ((c.g[V0][0] as u32) & 31));
                call_ptr(c, table_callee::UPLOAD_TEXTURE_PIXELS, 0x0036_7d8c)?;
            }
        }
        c.cycles += 10;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        callee::TEX_MAN_EVICT_TEXTURE.call(c, 0x0036_7d98)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void TexMan_EvictTexture(self, index)` (0x369098): free the slot's VRAM
/// blocks and clear the three cached slot indices.
pub fn tex_man_evict_texture(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = u64::MAX;
        c.g[V0][0] = Addr(c.g[V0][0] as u32).off(0x8).lw(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x28).lw(c);
        let unused = c.g[V1][0] == c.g[A0][0];
        c.g[V1][0] = 1;
        if !unused {
            c.cycles += 4;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
            c.g[V0][0] = 9;
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 3;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4350));
                callee::VRAM_HEAP_FREE_TEXTURE_BLOCKS.call(c, 0x0036_90ec)?;
                c.g[A0][0] = u64::MAX;
            } else {
                c.cycles += 4;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1f60));
                callee::VRAM_HEAP_FREE_TEXTURE_BLOCKS.call(c, 0x0036_90dc)?;
                c.g[A0][0] = u64::MAX;
            }
            c.cycles += 1;
            c.g[V1][0] = 1;
        }
        c.cycles += 1;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1f50));
        loop {
            c.cycles += 7;
            Addr(c.g[V0][0] as u32).sw(c, c.g[A0][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0036_90f8);
            }
        }
        c.cycles += 6;
        c.g[V0][0] = u64::MAX;
        frame.restore_ra(c, 0);
        Addr(c.g[S0][0] as u32).off(0x1f54).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void TexMan_EmitClutUpload(Ps2UpMgr* self, int idx, ulong** dma, ulong**
/// gif)` (0x3684f0): append the 0xB0-byte texture CLUT-upload packet for
/// `self->texs[idx]` to the GIF stream (`w`/`h` 8x2 or 16x16 depending on
/// the texture PSM), then hand it to the DMA kick builder.
pub fn tex_man_emit_clut_upload(c: &mut Cpu) {
    'b_368554: {
        'b_368548: {
            c.cycles += 18;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.g[T5][0] = c.g[A0][0];
            c.sq(17, c.pa(SP, 0x30));
            c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
            c.sq(18, c.pa(SP, 0x20));
            c.g[A1][0] = sx((c.g[T5][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.sq(16, c.pa(SP, 0x40));
            c.g[S2][0] = c.g[A2][0];
            c.std(SP, 0x10, c.g[RA][0]);
            c.g[S1][0] = c.g[A3][0];
            c.g[T4][0] = c.ldw_2(A1, 0x8);
            c.g[V1][0] = c.ldw_2(S2, 0);
            c.g[V0][0] = c.ldw_2(T4, 0xc);
            c.stw_2(SP, 0, c.g[V1][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xb));
            c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[S0][0] = c.ldw_2(S1, 0);
            if t {
                break 'b_368548;
            } else {
                c.cycles += 4;
                c.g[A2][0] = 0x8;
                c.g[A0][0] = 0x2;
                let t = true;
                c.g[T2][0] = 0x10;
                if t {
                    break 'b_368554;
                } else {
                    break 'b_368548;
                }
            }
        }
        c.cycles += 3;
        c.g[A2][0] = 0x10;
        c.g[A0][0] = 0x10;
        c.g[T2][0] = 0x40;
        break 'b_368554;
    }
    c.cycles += 66;
    c.g[T3][0] = 0xe;
    c.g[V0][0] = 0x10000000;
    c.g[V0][0] = c.g[V0][0] | 0x6;
    c.g[V1][0] = 0x10000000;
    c.g[V1][0] = c.g[V1][0] << 32;
    c.g[V1][0] = c.g[V1][0] | 0x4;
    c.std(S0, 0x18, c.g[T3][0]);
    c.std(S0, 0x10, c.g[V1][0]);
    c.g[A0][0] = c.g[A0][0] << 32;
    c.std(S0, 0, c.g[V0][0]);
    c.g[A0][0] = c.g[A2][0] | c.g[A0][0];
    c.std(S0, 0x8, 0u64);
    c.g[A3][0] = 0x8000;
    c.g[A3][0] = c.g[A3][0] << 33;
    c.g[A1][0] = 0x8000;
    c.g[A1][0] = c.g[A1][0] << 44;
    c.g[T0][0] = 0x52;
    c.g[V0][0] = c.ldw_2(T4, 0x2c);
    c.g[A1][0] = c.g[T2][0] | c.g[A1][0];
    c.g[T1][0] = 0x53;
    c.g[V1][0] = 0x50;
    c.g[V0][0] = c.g[V0][0] << 32;
    c.g[A2][0] = 0x51;
    c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
    c.std(S0, 0x28, c.g[V1][0]);
    c.std(S0, 0x38, c.g[A2][0]);
    c.g[V1][0] = 0x30000000;
    c.std(S0, 0x40, c.g[A0][0]);
    c.g[V1][0] = c.g[V1][0] | c.g[T2][0];
    c.std(S0, 0x48, c.g[T0][0]);
    c.g[T2][0] = 0x10000000;
    c.g[T2][0] = c.g[T2][0] | 0x2;
    c.std(S0, 0x58, c.g[T1][0]);
    c.g[A2][0] = 0x10000000;
    c.g[A2][0] = c.g[A2][0] << 32;
    c.g[A2][0] = c.g[A2][0] | 0x8001;
    c.std(S0, 0x60, c.g[A1][0]);
    c.g[A3][0] = 0x3f;
    c.std(S0, 0x20, c.g[V0][0]);
    c.g[A0][0] = c.g[T5][0];
    c.std(S0, 0x30, 0u64);
    c.g[A1][0] = c.g[SP][0];
    c.std(S0, 0x50, 0u64);
    c.std(S0, 0x68, 0u64);
    c.g[V0][0] = c.ldw_2(T4, 0x20);
    c.std(S0, 0x78, 0u64);
    c.g[V0][0] = c.g[V0][0] << 32;
    c.std(S0, 0x88, 0u64);
    c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
    c.std(S0, 0xa0, 0u64);
    c.std(S0, 0x70, c.g[V1][0]);
    c.std(S0, 0x80, c.g[T2][0]);
    c.std(S0, 0x90, c.g[A2][0]);
    c.std(S0, 0x98, c.g[T3][0]);
    c.g[RA][0] = sx(0x00368634);
    c.std(S0, 0xa8, c.g[A3][0]);
    c.pc = 0x00368138;
    call_indirect(c, 0x00368138);
    if c.pc != 0x00368634 {
        return;
    }
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xb0));
    c.g[V0][0] = c.ldw_2(SP, 0);
    c.g[RA][0] = c.ldd(SP, 0x10);
    c.stw_2(S2, 0, c.g[V0][0]);
    c.stw_2(S1, 0, c.g[S0][0]);
    c.lq(16, c.pa(SP, 0x40));
    c.lq(17, c.pa(SP, 0x30));
    c.lq(18, c.pa(SP, 0x20));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
    c.pc = ra;
    return;
}

/// `void func_00368660(void* self, int a, int b, ...)` (0x368660): bind (upload)
/// textures `a` and `b` of the upload manager: allocate heap nodes for the
/// image and its CLUT, emit the image/CLUT upload packets and finally the
/// texture setup for both; with nothing to send it writes an empty packet.
pub fn tex_man_bind_textures(c: &mut Cpu) {
    let frame = Frame::open(c, 0xe0);
    'b_368940: {
        'b_3686cc: {
            'b_3686c0: {
                c.cycles += 20;
                frame.save(c, S3, 0xa0);
                frame.save(c, S6, 0x70);
                c.g[S3][0] = c.g[A0][0];
                sq(c, 30, at_4(c, SP, 0x50));
                c.g[S6][0] = c.g[A2][0];
                frame.save(c, S0, 0xd0);
                c.g[FP][0] = c.g[A1][0];
                frame.save(c, S1, 0xc0);
                frame.save(c, S2, 0xb0);
                frame.save(c, S4, 0x90);
                frame.save(c, S5, 0x80);
                frame.save(c, S7, 0x60);
                frame.save_ra(c, 0x40);
                sw_2(c, at_4(c, SP, 0x1c), c.g[T0][0]);
                c.g[V0][0] = lw_3(c, r32(c, T0));
                sw_2(c, at_4(c, SP, 0x18), c.g[A3][0]);
                sw_2(c, at_4(c, SP, 0x20), c.g[T1][0]);
                let t = (c.g[FP][0] as i64) >= 0;
                sw_2(c, at_4(c, SP, 0x10), c.g[V0][0]);
                if t {
                    break 'b_3686c0;
                } else {
                    c.cycles += 2;
                    let t = (c.g[S6][0] as i64) < 0;
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_368940;
                    } else {
                        c.cycles += 2;
                        let t = true;

                        if t {
                            break 'b_3686cc;
                        } else {
                            break 'b_3686c0;
                        }
                    }
                }
            }
            c.cycles += 3;
            c.g[V0][0] = sx(r32(c, FP) << 2);
            c.g[V0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
            c.g[V0][0] = lw_3(c, at_4(c, V0, 0x8));
            break 'b_3686cc;
        }
        'b_3686e8: {
            'b_3686e4: {
                c.cycles += 2;
                let t = (c.g[S6][0] as i64) < 0;
                sw_2(c, r32(c, SP), c.g[V0][0]);
                if t {
                    break 'b_3686e4;
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = sx(r32(c, S6) << 2);
                    c.g[V0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
                    let t = true;
                    c.g[V0][0] = lw_3(c, at_4(c, V0, 0x8));
                    if t {
                        break 'b_3686e8;
                    } else {
                        break 'b_3686e4;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0u64;
            break 'b_3686e8;
        }
        c.cycles += 6;
        sw_2(c, at_4(c, SP, 0x4), c.g[V0][0]);
        c.g[A3][0] = 0u64;
        sw_2(c, at_4(c, SP, 0x14), 0u64);
        c.g[S7][0] = 0u64;
        c.g[S2][0] = c.g[FP][0];
        'l_368700: loop {
            'b_368894: {
                'b_368890: {
                    c.cycles += 3;
                    if c.g[S7][0] != 0 {
                        c.g[S2][0] = c.g[S6][0];
                    }
                    let t = (c.g[S2][0] as i64) < 0;
                    c.g[V0][0] = sx(r32(c, S2) << 2);
                    if t {
                        break 'b_368890;
                    } else {
                        'b_36873c: {
                            'b_368734: {
                                c.cycles += 7;
                                c.g[S4][0] = c.g[S6][0];
                                c.g[V0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
                                c.g[S1][0] = lw_3(c, at_4(c, V0, 0x8));
                                c.g[V1][0] = 0x9;
                                c.g[V0][0] = lw_3(c, at_4(c, S1, 0xc));
                                let t = c.g[V0][0] != c.g[V1][0];
                                if c.g[S7][0] != 0 {
                                    c.g[S4][0] = c.g[FP][0];
                                }
                                if t {
                                    break 'b_368734;
                                } else {
                                    c.cycles += 3;
                                    c.g[S0][0] = sx(at_4(c, S3, 0x4350));
                                    let t = true;
                                    c.g[S5][0] = 0x2;
                                    if t {
                                        break 'b_36873c;
                                    } else {
                                        break 'b_368734;
                                    }
                                }
                            }
                            c.cycles += 2;
                            c.g[S0][0] = sx(at_4(c, S3, 0x1f60));
                            c.g[S5][0] = 0u64;
                            break 'b_36873c;
                        }
                        'b_3687d8: {
                            c.cycles += 3;
                            c.g[V0][0] = lw_3(c, at_4(c, S1, 0x28));
                            if (c.g[V0][0] as i64) >= 0 {
                                c.g[V0][0] = lw_3(c, at_4(c, S1, 0x20));
                                break 'b_3687d8;
                            } else {
                                'b_3687bc: {
                                    c.cycles += 22;
                                    c.g[V0][0] = sx(r32(c, S7) << 2);
                                    c.g[V1][0] = 0xffffffffffffffff;
                                    c.g[V0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
                                    c.g[A0][0] = c.g[S0][0];
                                    sw_2(c, at_4(c, V0, 0x1f4c), c.g[V1][0]);
                                    c.g[A2][0] = c.g[S2][0];
                                    c.g[A3][0] = c.g[S4][0];
                                    c.g[T1][0] = 0x1;
                                    c.g[A1][0] = lw_3(c, at_4(c, S1, 0x14));
                                    c.g[RA][0] = sx(0x00368774);
                                    c.g[T0][0] = lw_3(c, at_4(c, SP, 0x18));
                                    if callee::C00366CE0.call(c, 0x00368774).is_err() {
                                        return;
                                    }
                                    c.g[V1][0] = 0x1c;
                                    c.g[A0][0] = lw_3(c, at_4(c, S0, 0x1ff0));
                                    c.mult(0, A1, V0, V1);
                                    c.g[A2][0] = lw_3(c, at_4(c, SP, 0x14));
                                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
                                    c.g[V1][0] = lw_3(c, r32(c, A0));
                                    sw_2(c, at_4(c, S1, 0x30), c.g[V0][0]);
                                    c.g[V1][0] = sx((r32(c, V1) as i32 >> 8) as u32);
                                    c.g[V1][0] = sx(r32(c, V1) << (r32(c, S5) & 31));
                                    let t = c.g[A2][0] != 0u64;
                                    sw_2(c, at_4(c, S1, 0x28), c.g[V1][0]);
                                    if t {
                                        break 'b_3687bc;
                                    } else {
                                        c.cycles += 7;
                                        c.g[V0][0] = lw_3(c, at_4(c, SP, 0x20));
                                        c.g[A2][0] = 0xffffffffffffffff;
                                        c.g[A0][0] = lw_3(c, at_4(c, GP, 0x1384));
                                        c.g[A3][0] = 0u64;
                                        c.g[RA][0] = sx(0x003687b8);
                                        c.g[A1][0] = lw_3(c, r32(c, V0));
                                        if callee::C0038F460.call(c, 0x003687b8).is_err() {
                                            return;
                                        }
                                        sw_2(c, at_4(c, SP, 0x14), c.g[V0][0]);
                                        break 'b_3687bc;
                                    }
                                }
                                c.cycles += 7;
                                c.g[A3][0] = sx(at_4(c, SP, 0x14));
                                c.g[A0][0] = c.g[S3][0];
                                c.g[A1][0] = c.g[S2][0];
                                c.g[RA][0] = sx(0x003687d0);
                                c.g[A2][0] = sx(at_4(c, SP, 0x10));
                                if callee::C00368170.call(c, 0x003687d0).is_err() {
                                    return;
                                }
                                c.g[A3][0] = 0x1;
                                c.g[V0][0] = lw_3(c, at_4(c, S1, 0x20));
                                break 'b_3687d8;
                            }
                        }
                        c.cycles += 2;
                        if c.g[V0][0] == 0u64 {
                            c.g[S7][0] = sx(at_4(c, S7, 0x1));
                            break 'b_368894;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = lw_3(c, at_4(c, S1, 0x2c));
                            if (c.g[V0][0] as i64) >= 0 {
                                c.g[S7][0] = sx(at_4(c, S7, 0x1));
                                break 'b_368894;
                            } else {
                                'b_36887c: {
                                    c.cycles += 28;
                                    c.g[V1][0] = 0xffffffffffffffff;
                                    c.g[A1][0] = 0x400;
                                    sw_2(c, at_4(c, S3, 0x1f54), c.g[V1][0]);
                                    c.g[A0][0] = sx(at_4(c, S3, 0x1f60));
                                    c.g[V1][0] = 0x1;
                                    c.g[A3][0] = c.g[S4][0];
                                    c.g[V0][0] = lw_3(c, at_4(c, S1, 0xc));
                                    c.g[A2][0] = c.g[S2][0];
                                    c.g[T0][0] = lw_3(c, at_4(c, SP, 0x18));
                                    c.g[T1][0] = 0u64;
                                    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xb));
                                    c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                                    c.g[V0][0] = 0x100;
                                    c.g[RA][0] = sx(0x00368828);
                                    if c.g[V1][0] == 0 {
                                        c.g[A1][0] = c.g[V0][0];
                                    }
                                    if callee::C00366CE0.call(c, 0x00368828).is_err() {
                                        return;
                                    }
                                    c.g[V1][0] = 0x1c;
                                    c.g[A1][0] = lw_3(c, at_4(c, S3, 0x3f50));
                                    c.mult(0, A0, V0, V1);
                                    c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                                    c.g[V1][0] = lw_3(c, r32(c, A1));
                                    c.g[V1][0] = c.g[V1][0] | 0x4;
                                    sw_2(c, r32(c, A1), c.g[V1][0]);
                                    c.g[V1][0] = sx((r32(c, V1) as i32 >> 8) as u32);
                                    sw_2(c, at_4(c, S1, 0x34), c.g[V0][0]);
                                    sw_2(c, at_4(c, S1, 0x2c), c.g[V1][0]);
                                    c.g[V0][0] = lw_3(c, at_4(c, SP, 0x14));
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A3][0] = sx(at_4(c, SP, 0x14));
                                    if t {
                                        break 'b_36887c;
                                    } else {
                                        c.cycles += 8;
                                        c.g[V0][0] = lw_3(c, at_4(c, SP, 0x20));
                                        c.g[A2][0] = 0xffffffffffffffff;
                                        c.g[A0][0] = lw_3(c, at_4(c, GP, 0x1384));
                                        c.g[A3][0] = 0u64;
                                        c.g[RA][0] = sx(0x00368874);
                                        c.g[A1][0] = lw_3(c, r32(c, V0));
                                        if callee::C0038F460.call(c, 0x00368874).is_err() {
                                            return;
                                        }
                                        sw_2(c, at_4(c, SP, 0x14), c.g[V0][0]);
                                        c.g[A3][0] = sx(at_4(c, SP, 0x14));
                                        break 'b_36887c;
                                    }
                                }
                                c.cycles += 5;
                                c.g[A1][0] = c.g[S2][0];
                                c.g[A0][0] = c.g[S3][0];
                                c.g[RA][0] = sx(0x0036888c);
                                c.g[A2][0] = sx(at_4(c, SP, 0x10));
                                if callee::C003684F0.call(c, 0x0036888c).is_err() {
                                    return;
                                }
                                c.g[A3][0] = 0x1;
                                break 'b_368890;
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[S7][0] = sx(at_4(c, S7, 0x1));
                break 'b_368894;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S7][0] as i64) < 2i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[S2][0] = c.g[FP][0];
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00368700);
                }
                continue 'l_368700;
            } else {
                'b_3688c4: {
                    c.cycles += 3;
                    c.g[A1][0] = lw_3(c, at_4(c, SP, 0x14));
                    let t = c.g[A1][0] == 0u64;
                    c.g[A0][0] = lw_3(c, at_4(c, GP, 0x1384));
                    if t {
                        break 'b_3688c4;
                    } else {
                        c.cycles += 6;
                        c.g[A2][0] = 0x4;
                        c.g[RA][0] = sx(0x003688b8);
                        sq(c, 7, at_4(c, SP, 0x30));
                        if callee::C0038F668.call(c, 0x003688b8).is_err() {
                            return;
                        }
                        c.g[V1][0] = lw_3(c, at_4(c, SP, 0x20));
                        sw_2(c, r32(c, V1), c.g[V0][0]);
                        lq(c, 7, at_4(c, SP, 0x30));
                        break 'b_3688c4;
                    }
                }
                'b_368904: {
                    c.cycles += 2;
                    let t = c.g[A3][0] != 0u64;
                    c.g[A0][0] = lw_3(c, at_4(c, SP, 0x10));
                    if t {
                        break 'b_368904;
                    } else {
                        c.cycles += 14;
                        c.g[V1][0] = 0x10000000;
                        c.g[V1][0] = c.g[V1][0] | 0x1;
                        c.g[A1][0] = 0x8800;
                        c.g[A1][0] = c.g[A1][0] << 45;
                        sd(c, r32(c, A0), c.g[V1][0]);
                        c.g[V0][0] = lw_3(c, at_4(c, SP, 0x10));
                        sd(c, at_4(c, V0, 0x8), 0u64);
                        c.g[V1][0] = lw_3(c, at_4(c, SP, 0x10));
                        sd(c, at_4(c, V1, 0x10), 0u64);
                        c.g[A0][0] = lw_3(c, at_4(c, SP, 0x10));
                        sd(c, at_4(c, A0, 0x18), c.g[A1][0]);
                        c.g[V0][0] = lw_3(c, at_4(c, SP, 0x10));
                        c.g[V0][0] = sx(at_4(c, V0, 0x20));
                        sw_2(c, at_4(c, SP, 0x10), c.g[V0][0]);
                        break 'b_368904;
                    }
                }
                'b_36891c: {
                    c.cycles += 2;
                    let t = (c.g[FP][0] as i64) < 0;
                    c.g[A1][0] = c.g[FP][0];
                    if t {
                        break 'b_36891c;
                    } else {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A2][0] = 0u64;
                        c.g[RA][0] = sx(0x0036891c);
                        c.g[A3][0] = sx(at_4(c, SP, 0x10));
                        if callee::C00368970.call(c, 0x0036891c).is_err() {
                            return;
                        }
                        break 'b_36891c;
                    }
                }
                'b_368934: {
                    c.cycles += 2;
                    let t = (c.g[S6][0] as i64) < 0;
                    c.g[A0][0] = c.g[S3][0];
                    if t {
                        break 'b_368934;
                    } else {
                        c.cycles += 4;
                        c.g[A1][0] = c.g[S6][0];
                        c.g[A2][0] = 0x1;
                        c.g[RA][0] = sx(0x00368934);
                        c.g[A3][0] = sx(at_4(c, SP, 0x10));
                        if callee::C00368970.call(c, 0x00368934).is_err() {
                            return;
                        }
                        break 'b_368934;
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = lw_3(c, at_4(c, SP, 0x10));
                c.g[V1][0] = lw_3(c, at_4(c, SP, 0x1c));
                sw_2(c, r32(c, V1), c.g[V0][0]);
                break 'b_368940;
            }
        }
    }
    c.cycles += 12;
    frame.restore(c, S0, 0xd0);
    frame.restore(c, S1, 0xc0);
    frame.restore(c, S2, 0xb0);
    frame.restore(c, S3, 0xa0);
    frame.restore(c, S4, 0x90);
    frame.restore(c, S5, 0x80);
    frame.restore(c, S6, 0x70);
    frame.restore(c, S7, 0x60);
    lq(c, 30, at_4(c, SP, 0x50));
    c.g[RA][0] = ld(c, at_4(c, SP, 0x40));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_4(c, SP, 0xe0));
    c.pc = t;
    return;
}

/// `TexMan` fields: texture pointers by id (+8), the cached lookups reset
/// on a free (+0x1f4c, +0x1f50, +0x1f54), the VRAM heaps (+0x1f60, and
/// +0x4350 for kind 9 textures).
pub(crate) const TEX_MAN_TEXTURES: u32 = 0x8;

pub(crate) const TEX_MAN_CACHE: u32 = 0x1f50;

pub(crate) const TEX_MAN_CACHE_LAST: u32 = 0x1f54;

pub(crate) const TEX_MAN_VRAM: u32 = 0x1f60;

pub(crate) const TEX_MAN_VRAM_KIND9: u32 = 0x4350;

/// Texture fields: owned flag, kind, the two CPU buffers, the two VRAM
/// blocks (-1: none) and their ids.
pub(crate) const TEX_OWNED: u32 = 0x0;

pub(crate) const TEX_KIND: u32 = 0xc;

pub(crate) const TEX_BUF_A: u32 = 0x1c;

pub(crate) const TEX_BUF_B: u32 = 0x20;

pub(crate) const TEX_SLOT_A: u32 = 0x28;

pub(crate) const TEX_SLOT_B: u32 = 0x2c;

pub(crate) const TEX_ID_A: u32 = 0x30;

pub(crate) const TEX_ID_B: u32 = 0x34;

/// Ids below this are B textures (`TexMan_FreeBTexInfo`), others NB.
pub(crate) const TEX_FIRST_NB: i64 = 1500;

/// `void TexMan_FreeTexture(TexMan* self, int id)` (0x367bc0): free a
/// texture's VRAM blocks (block A in the kind-9 heap for kind 9 textures),
/// its CPU buffers when it owns them, its info record (B or NB by id), and
/// reset the cached lookups (+0x1f4c..+0x1f54 = -1). The two-word reset
/// loop checks events at 0x367c90. Frame 0x50: ra at 0, s3..s0 at
/// 0x10..0x40.
pub fn tex_man_free_texture(c: &mut Cpu) {
    const VRAM_FREE: Callee = Callee {
        addr: 0x0036_63d8,
        f: ee::s12::VramHeap_FreeBlock,
    };
    const NONE: u64 = u64::MAX;
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S1, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let man = c.g[S1][0] as u32;
        let id = c.g[S2][0] as u32;
        c.g[V0][0] = sx(man.wrapping_add(id << 2));
        // (s0..s3 hold the texture, manager, id and -1 across the calls.)
        frame.save(c, S0, 0x40);
        c.g[S0][0] = Addr(c.g[V0][0] as u32).off(TEX_MAN_TEXTURES).lw(c);
        c.g[S3][0] = NONE;
        let tex = Addr(c.g[S0][0] as u32);
        let slot_a = tex.off(TEX_SLOT_A).lw(c);
        c.g[V1][0] = sx(man.wrapping_add(TEX_MAN_VRAM_KIND9));
        if slot_a != NONE {
            c.cycles += 6;
            let kind = tex.off(TEX_KIND).lw(c);
            c.g[A0][0] = sx(man.wrapping_add(TEX_MAN_VRAM));
            c.g[A1][0] = tex.off(TEX_ID_A).lw(c);
            c.g[V0][0] = kind ^ 9;
            if c.g[V0][0] == 0 {
                c.g[A0][0] = c.g[V1][0];
            }
            VRAM_FREE.call(c, 0x0036_7c14)?;
        }

        c.cycles += 3;
        let tex = Addr(c.g[S0][0] as u32);
        let man = c.g[S1][0] as u32;
        c.g[V0][0] = tex.off(TEX_SLOT_B).lw(c);
        if c.g[V0][0] != NONE {
            c.cycles += 4;
            c.g[A1][0] = tex.off(TEX_ID_B).lw(c);
            c.g[A0][0] = sx(man.wrapping_add(TEX_MAN_VRAM));
            VRAM_FREE.call(c, 0x0036_7c2c)?;
        }
        let tex = Addr(c.g[S0][0] as u32);
        c.g[V0][0] = tex.off(TEX_OWNED).lw(c);

        c.cycles += 2;
        let owned = c.g[V0][0] != 0;
        let is_b = (c.g[S2][0] as i64) < TEX_FIRST_NB;
        c.g[V0][0] = is_b as u64;
        if owned {
            c.cycles += 3;
            c.g[A0][0] = tex.off(TEX_BUF_A).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                call_ptr(c, 0x0031_7e98, 0x0036_7c4c)?; // EA_MemFree
            }
            c.g[A0][0] = Addr(c.g[S0][0] as u32).off(TEX_BUF_B).lw(c);
            c.cycles += 2;
            c.g[V0][0] = ((c.g[S2][0] as i64) < TEX_FIRST_NB) as u64;
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                call_ptr(c, 0x0031_7e98, 0x0036_7c60)?; // EA_MemFree
                c.g[V0][0] = ((c.g[S2][0] as i64) < TEX_FIRST_NB) as u64;
            }
        }

        c.cycles += 2;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S1][0];
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            Callee {
                addr: 0x0036_72c0,
                f: ee::s12::TexMan_FreeBTexInfo,
            }
            .call(c, 0x0036_7c74)?;
        } else {
            c.cycles += 3;
            Callee {
                addr: 0x0036_71c8,
                f: ee::s12::TexMan_FreeNBTexInfo,
            }
            .call(c, 0x0036_7c84)?;
        }
        c.g[A0][0] = NONE;

        // Reset the cached lookups (+0x1f50, +0x1f4c), then +0x1f54.
        c.cycles += 2;
        c.g[V1][0] = 1;
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(TEX_MAN_CACHE));
        loop {
            c.cycles += 7;
            let none = c.g[A0][0];
            Addr(c.g[V0][0] as u32).sw(c, none);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0036_7c90);
            }
        }
        c.cycles += 9;
        c.g[V0][0] = NONE;
        frame.restore(c, S0, 0x40);
        Addr((c.g[S1][0] as u32).wrapping_add(TEX_MAN_CACHE_LAST)).sw(c, NONE);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod tex_load_info {

    /// Non-zero: the manager owns (copied) the pixel and CLUT data.
    pub const OWNED: u32 = 0x00;

    /// Eight-character texture name.
    pub const NAME: u32 = 0x04;

    pub const FORMAT: u32 = 0x0c;

    pub const DATA_BYTES: u32 = 0x10;

    pub const VRAM_BYTES: u32 = 0x14;

    pub const BPP: u32 = 0x18;

    pub const PIXELS: u32 = 0x1c;

    pub const CLUT: u32 = 0x20;

    pub const MIPS: u32 = 0x24;

    pub const VRAM_BLOCK: u32 = 0x28;

    pub const VRAM_NODE: u32 = 0x30;

    pub const UNUSED_2C: u32 = 0x2c;

    pub const UNUSED_34: u32 = 0x34;

    /// GS TEX0, TEX1 and MIPTBP1/2 register images.
    pub const TEX0: u32 = 0x38;

    pub const TEX1: u32 = 0x40;

    pub const MIPTBP1: u32 = 0x48;

    pub const MIPTBP2: u32 = 0x50;
}

pub(crate) mod tex_load_man {

    /// Texture slot pointers (`TexInfo*`) from +8.
    pub const SLOTS: u32 = 0x8;

    /// Allocation flags OR-ed into every `operator new`.
    pub const ALLOC_FLAGS: u32 = 0x1f58;

    /// The embedded VRAM heap and its node array (`0x1c`-byte nodes).
    pub const VRAM_HEAP: u32 = 0x1f60;

    pub const VRAM_NODES: u32 = 0x3f50;

    /// VRAM byte budget above which owned PSMT8 textures are swizzled into
    /// a temporary copy instead of being uploaded as they are.
    pub const T8_LIMIT: u32 = 0x6344;
}

pub(crate) const TEX_LOAD_NB_ALLOC: Callee = Callee {
    addr: 0x0036_7150,
    f: ee::s12::TexMan_AllocNBTexInfo,
};

pub(crate) const TEX_LOAD_B_ALLOC: Callee = Callee {
    addr: 0x0036_7260,
    f: ee::s12::TexMan_AllocBTexInfo,
};

pub(crate) const TEX_LOAD_VRAM_ALLOC: Callee = Callee {
    addr: 0x0036_6ce0,
    f: ee::s12::VramHeap_Alloc,
};

pub(crate) const TEX_LOAD_COPY_TO_TEMP: Callee = Callee {
    addr: 0x0036_6e90,
    f: ee::s12::TexMan_CopyToTempTex,
};

pub(crate) const TEX_LOAD_STRNCPY: u32 = 0x0041_6ce0;

pub(crate) const TEX_LOAD_NEW: u32 = 0x0031_7e30;

pub(crate) const TEX_LOAD_MEMCOPY: u32 = 0x003e_6574;

/// The jump tables (`.rodata`) the original switches through.
pub(crate) const TEX_LOAD_FMT_TABLE: u32 = 0x0049_2090;

pub(crate) const TEX_LOAD_PSM_TABLE: u32 = 0x0049_20d0;

pub(crate) const TEX_LOAD_PAGE_TABLE: u32 = 0x0049_21c0;

pub(crate) const TEX_LOAD_PAGE_TABLE2: u32 = 0x0049_2240;

/// Texture format (`fmt < 13`) -> (case address, GS PSM). Formats outside
/// the table use PSMCT32 (0).
pub(crate) fn tex_load_fmt_case(fmt: u64) -> (u32, u64) {
    match fmt {
        2 => (0x0036_7524, 0x02),
        8 => (0x0036_751c, 0x01),
        9 | 10 => (0x0036_752c, 0x13),
        11 | 12 => (0x0036_7534, 0x14),
        _ => (0x0036_753c, 0x00),
    }
}

/// GS PSM (`< 0x3b`) -> (case address, bits per pixel); 0 bpp for PSMs
/// outside the switch.
pub(crate) fn tex_load_psm_case(psm: u64) -> (u32, u64) {
    match psm {
        0x00 | 0x01 | 0x30 | 0x31 => (0x0036_7564, 0x20),
        0x02 | 0x0a | 0x32 | 0x3a => (0x0036_756c, 0x10),
        0x13 | 0x1b => (0x0036_7574, 0x08),
        0x14 | 0x24 | 0x2c => (0x0036_757c, 0x04),
        _ => (0x0036_7584, 0),
    }
}

/// `bpp - 4` -> which GS page shape: 4 bpp 128x128, 8 bpp 128x64,
/// 15/16 bpp 64x64, else (32 bpp) 64x32.
pub(crate) fn tex_load_page_case(idx: u64) -> usize {
    match idx {
        0 => 0,
        4 => 1,
        11 | 12 => 2,
        _ => 3,
    }
}

/// Page (width, height) per case, the case addresses of the loop's switch
/// and of the no-data switch.
pub(crate) const TEX_LOAD_PAGE_DIMS: [(u64, u64); 4] =
    [(0x80, 0x80), (0x80, 0x40), (0x40, 0x40), (0x40, 0x20)];

pub(crate) const TEX_LOAD_PAGE_CASES: [u32; 4] =
    [0x0036_7640, 0x0036_764c, 0x0036_7658, 0x0036_7664];

pub(crate) const TEX_LOAD_PAGE_CASES2: [u32; 4] =
    [0x0036_7734, 0x0036_7740, 0x0036_774c, 0x0036_7758];

/// The page switch on `bpp - 4` held in `idx_reg` (`sltiu 29`, table at
/// `table`): leaves `v0`, `v1` (table slot) and `a0` (case address) as the
/// original does and returns the page (width, height). The jump costs 7
/// cycles, the case 3 (2 for the default).
pub(crate) fn tex_load_page_switch(
    c: &mut Cpu,
    idx: u64,
    table: u32,
    cases: &[u32; 4],
) -> (u64, u64) {
    let case = if idx < 0x1d {
        c.cycles += 7;
        c.g[V0][0] = sx(table);
        c.g[V1][0] = sx(((idx as u32) << 2).wrapping_add(table));
        let k = tex_load_page_case(idx);
        c.g[A0][0] = sx(cases[k]);
        k
    } else {
        3
    };
    c.cycles += if case == 3 { 2 } else { 3 };
    TEX_LOAD_PAGE_DIMS[case]
}

/// `int TexMan_LoadTexture(TexMan* man, void* pixels, const char* name,
/// int w, int h, int mips, int fmt, int clutFmt, void* clut, int vramMode,
/// int owned, int id)` (0x367440; the last four on the stack): takes a
/// texture slot (a fresh one for `id < 0`, else slot `id`), names it,
/// works out the GS PSM and bits per pixel from `fmt`, the data and VRAM
/// sizes of every mip level, copies (owned) or references the pixels and
/// the CLUT, builds the TEX0/TEX1/MIPTBP register images, reserves VRAM now
/// for `vramMode` 1 (or 3: 0x2000 extra, 32-block aligned), and for PSMT8
/// either swizzles into a temporary copy (owned, over budget, mode 0) or
/// retags the PSM as T8H. Returns the slot id (-1 for an unknown PSM).
pub fn tex_man_load_texture(c: &mut Cpu) {
    use tex_load_info as ti;
    use tex_load_man as tm;
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0xe0);
        frame.save(c, S0, 0xd0);
        frame.save(c, S1, 0xc0);
        frame.save(c, S2, 0xb0);
        c.g[S1][0] = c.g[T2][0]; // fmt
        frame.save(c, S3, 0xa0);
        c.g[S2][0] = c.g[T0][0]; // h
        frame.save(c, S4, 0x90);
        c.g[S3][0] = c.g[A3][0]; // w
        frame.save(c, S5, 0x80);
        c.g[S4][0] = c.g[T3][0]; // clutFmt
        frame.save(c, S6, 0x70);
        c.g[S5][0] = c.g[T1][0]; // mips
        frame.save(c, S7, 0x60);
        c.g[S6][0] = c.g[A0][0]; // man
        frame.save(c, FP, 0x50);
        c.g[S7][0] = c.g[A1][0]; // pixels
        c.g[S0][0] = Addr(frame.at(c, 0xf8)).lw(c); // id
        c.g[FP][0] = c.g[A2][0]; // name
        frame.save_ra(c, 0x40);

        // The slot: `id` (spilled at sp+0).
        c.cycles += 4;
        if (c.g[S0][0] as i64) >= 0 {
            c.g[A0][0] = c.g[S6][0];
            c.g[A1][0] = c.g[S0][0];
            TEX_LOAD_B_ALLOC.call(c, 0x0036_74b0)?;
            Addr(sp(c)).sw(c, c.g[S0][0]);
        } else {
            TEX_LOAD_NB_ALLOC.call(c, 0x0036_749c)?;
            Addr(sp(c)).sw(c, c.g[V0][0]);
        }

        // tex = man->slots[id]; strncpy(tex->name, name, 8).
        c.cycles += 19;
        c.g[V1][0] = Addr(sp(c)).lw(c);
        c.g[A1][0] = c.g[FP][0];
        c.g[A2][0] = 8;
        c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[S0][0] = Addr(c.g[V0][0] as u32).off(tm::SLOTS).lw(c);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(ti::NAME));
        call_ptr(c, TEX_LOAD_STRNCPY, 0x0036_74d4)?;
        let tex = Addr(c.g[S0][0] as u32);
        c.g[V0][0] = Addr(frame.at(c, 0xf0)).lw(c); // owned
        c.g[V1][0] = u64::MAX;
        tex.off(ti::UNUSED_34).sw(c, c.g[V1][0]);
        tex.off(ti::OWNED).sw(c, c.g[V0][0]);
        c.g[V0][0] = (c.g[S1][0] < 0xd) as u64;
        tex.off(ti::FORMAT).sw(c, c.g[S1][0]);
        tex.off(ti::VRAM_BLOCK).sw(c, c.g[V1][0]);
        tex.off(ti::VRAM_NODE).sw(c, c.g[V1][0]);
        tex.off(ti::UNUSED_2C).sw(c, c.g[V1][0]);
        tex.off(ti::MIPS).sw(c, c.g[S5][0]);

        // fmt -> PSM (a1).
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            c.g[A1][0] = 0;
        } else {
            c.cycles += 7;
            let fmt = c.g[S1][0];
            c.g[V0][0] = sx(TEX_LOAD_FMT_TABLE);
            c.g[V1][0] = sx(((fmt as u32) << 2).wrapping_add(TEX_LOAD_FMT_TABLE));
            let (case, psm) = tex_load_fmt_case(fmt);
            c.g[A0][0] = sx(case);
            c.cycles += if psm == 0 { 1 } else { 2 };
            c.g[A1][0] = psm;
        }

        // PSM -> bits per pixel (t3).
        c.cycles += 3;
        let psm = c.g[A1][0];
        c.g[V0][0] = sx(0x0049_0000);
        if psm < 0x3b {
            c.cycles += 6;
            c.g[V1][0] = sx(((psm as u32) << 2).wrapping_add(TEX_LOAD_PSM_TABLE));
            c.g[V0][0] = sx(TEX_LOAD_PSM_TABLE);
            let (case, bpp) = tex_load_psm_case(psm);
            c.g[A0][0] = sx(case);
            c.cycles += if bpp == 0 { 1 } else { 2 };
            c.g[T3][0] = bpp;
        } else {
            c.cycles += 1;
            c.g[T3][0] = 0;
        }
        c.cycles += 2;
        tex.off(ti::BPP).sw(c, c.g[T3][0]);
        if (c.g[T3][0] as i64) <= 0 {
            // Not reachable: every PSM the format switch yields has a size.
            c.cycles += 2;
            c.g[V0][0] = u64::MAX;
            tex_load_texture_exit(c, frame);
            return Ok(());
        }

        c.cycles += 2;
        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(1)); // w - 1
        if c.g[S7][0] != 0 {
            // Pixels given: sum the data and VRAM bytes of every mip level.
            c.cycles += 16;
            Addr(frame.at(c, 0x18)).sd(c, c.g[A1][0]); // psm
            c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
            c.g[A1][0] = sx(tex.off(ti::TEX0).0);
            Addr(frame.at(c, 0x20)).sw(c, c.g[V0][0]);
            Addr(frame.at(c, 0x8)).sw(c, c.g[A1][0]);
            c.g[V0][0] = ((c.g[T3][0] as i64) < 9) as u64;
            Addr(frame.at(c, 0x30)).sw(c, c.g[V1][0]);
            c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x3f));
            c.g[V1][0] = c.g[S5][0] << 32;
            c.g[S1][0] = 0; // data bytes
            c.g[T5][0] = 0; // VRAM bytes
            c.g[T4][0] = 0; // level
            Addr(frame.at(c, 0x4)).sw(c, c.g[A1][0]);
            Addr(frame.at(c, 0xc)).sw(c, c.g[V0][0]);
            Addr(frame.at(c, 0x10)).sd(c, c.g[V1][0]);
            if (c.g[S5][0] as i64) >= 0 {
                loop {
                    c.cycles += 17;
                    let lv = c.g[T4][0] as u32 & 31;
                    c.g[T1][0] = sx((c.g[S3][0] as u32) >> lv);
                    c.g[T2][0] = sx((c.g[S2][0] as u32) >> lv);
                    c.mult(0, V1, T1, T2);
                    c.g[A0][0] = u64::MAX;
                    c.g[A1][0] = 0xff;
                    c.g[A2][0] = 0x100;
                    c.g[T0][0] = sx((c.g[T3][0] as u32).wrapping_sub(4));
                    c.g[A3][0] = (c.g[T0][0] < 0x1d) as u64;
                    c.mult(0, V1, V1, T3);
                    c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(7));
                    if c.g[A0][0] != 0 {
                        c.g[V0][0] = c.g[V1][0];
                    }
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
                    c.g[A1][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[A1][0] == 0 {
                        c.g[V0][0] = c.g[A2][0];
                    }
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let idx = if c.g[A3][0] != 0 {
                        c.g[T0][0]
                    } else {
                        u64::MAX
                    };
                    let (pw, ph) =
                        tex_load_page_switch(c, idx, TEX_LOAD_PAGE_TABLE, &TEX_LOAD_PAGE_CASES);
                    c.g[A1][0] = pw;
                    c.g[V1][0] = ph;

                    c.cycles += 18;
                    c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    c.g[A0][0] = ((c.g[T2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[A1][0] = c.g[T1][0];
                    }
                    if c.g[A0][0] == 0 {
                        c.g[V1][0] = c.g[T2][0];
                    }
                    c.mult(0, V1, A1, V1);
                    c.g[A0][0] = u64::MAX;
                    c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(1));
                    c.g[A1][0] = ((c.g[S5][0] as i64) < (c.g[T4][0] as i64)) as u64;
                    c.mult(0, V1, V1, T3);
                    c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(7));
                    if c.g[A0][0] != 0 {
                        c.g[V0][0] = c.g[V1][0];
                    }
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
                    c.g[V1][0] = sx(0u32.wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] &= 0xff;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[T5][0] = sx((c.g[T5][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    if c.g[A1][0] != 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0036_75e0);
                    }
                }
            }
            c.cycles += 4;
            c.g[V0][0] = tex.off(ti::OWNED).lw(c);
            tex.off(ti::VRAM_BYTES).sw(c, c.g[T5][0]);
            tex.off(ti::DATA_BYTES).sw(c, c.g[S1][0]);
            if c.g[V0][0] != 0 {
                // Owned: copy the pixels into a fresh block.
                c.cycles += 14;
                c.g[V0][0] = Addr(c.g[S6][0] as u32).off(tm::ALLOC_FLAGS).lw(c);
                c.g[A2][0] = 0x0500_0000;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[FP][0];
                c.g[A2][0] |= c.g[V0][0];
                c.g[A3][0] = 0;
                call_ptr(c, TEX_LOAD_NEW, 0x0036_76e0)?;
                c.g[A1][0] = c.g[S7][0];
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S1][0];
                Addr(c.g[S0][0] as u32).off(ti::PIXELS).sw(c, c.g[V0][0]);
                call_ptr(c, TEX_LOAD_MEMCOPY, 0x0036_76f4)?;
                c.g[T3][0] = Addr(frame.at(c, 0xe0)).lw(c); // clut
            } else {
                c.cycles += 2;
                tex.off(ti::PIXELS).sw(c, c.g[S7][0]);
                c.cycles += 1;
                c.g[T3][0] = Addr(frame.at(c, 0xe0)).lw(c);
            }
        } else {
            // No pixels: only the top level's VRAM size.
            c.cycles += 5;
            c.g[V1][0] = sx((c.g[T3][0] as u32).wrapping_sub(4));
            tex.off(ti::PIXELS).sw(c, 0);
            c.g[V0][0] = (c.g[V1][0] < 0x1d) as u64;
            tex.off(ti::DATA_BYTES).sw(c, 0);
            let idx = if c.g[V0][0] != 0 {
                c.g[V1][0]
            } else {
                u64::MAX
            };
            let (pw, ph) =
                tex_load_page_switch(c, idx, TEX_LOAD_PAGE_TABLE2, &TEX_LOAD_PAGE_CASES2);
            c.g[A2][0] = pw;
            c.g[A0][0] = ph;

            c.cycles += 28;
            c.g[V1][0] = ((c.g[S2][0] as i64) < (c.g[A0][0] as i64)) as u64;
            c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[A2][0] as i64)) as u64;
            if c.g[V1][0] == 0 {
                c.g[A0][0] = c.g[S2][0];
            }
            if c.g[V0][0] == 0 {
                c.g[A2][0] = c.g[S3][0];
            }
            c.mult(0, A0, A2, A0);
            c.g[V0][0] = sx(tex.off(ti::TEX0).0);
            Addr(frame.at(c, 0x8)).sw(c, c.g[V0][0]);
            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x3f));
            Addr(frame.at(c, 0x18)).sd(c, c.g[A1][0]); // psm
            Addr(frame.at(c, 0x4)).sw(c, c.g[V0][0]);
            c.g[V1][0] = u64::MAX;
            c.mult(0, A0, A0, T3);
            c.g[A1][0] = ((c.g[T3][0] as i64) < 9) as u64;
            Addr(frame.at(c, 0xc)).sw(c, c.g[A1][0]);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
            c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_sub(1));
            Addr(frame.at(c, 0x30)).sw(c, c.g[S2][0]);
            Addr(frame.at(c, 0x20)).sw(c, c.g[A1][0]);
            c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(7));
            if c.g[V1][0] != 0 {
                c.g[V0][0] = c.g[A0][0];
            }
            c.g[A1][0] = c.g[S5][0] << 32;
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
            Addr(frame.at(c, 0x10)).sd(c, c.g[A1][0]);
            c.g[V1][0] = sx(0u32.wrapping_sub(c.g[V0][0] as u32));
            c.g[V1][0] &= 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            tex.off(ti::VRAM_BYTES).sw(c, c.g[V0][0]);
            c.cycles += 1;
            c.g[T3][0] = Addr(frame.at(c, 0xe0)).lw(c); // clut
        }

        // The CLUT: entry size from clutFmt (s2: 1 -> 4, 2 -> 2, else 0).
        let tex = Addr(c.g[S0][0] as u32);
        c.cycles += 2;
        c.g[V0][0] = 1;
        let mut has_clut = false;
        if c.g[T3][0] != 0 {
            c.cycles += 2;
            let is1 = c.g[S4][0] == 1;
            c.g[V0][0] = ((c.g[S4][0] as i64) < 2) as u64;
            if is1 {
                c.cycles += 1;
                c.g[S2][0] = 4;
            } else {
                c.cycles += 2;
                c.g[S2][0] = 0;
                if c.g[V0][0] == 0 {
                    c.cycles += 3;
                    c.g[V0][0] = 2;
                    if c.g[S4][0] == 2 {
                        c.g[S2][0] = 2;
                    } else {
                        c.cycles += 2;
                    }
                }
            }
            c.cycles += 2;
            c.g[V1][0] = 1;
            has_clut = c.g[S2][0] != 0;
        }
        if has_clut {
            // 16 entries for the 4-bit formats (11, 12), else 256.
            c.cycles += 8;
            c.g[V0][0] = tex.off(ti::FORMAT).lw(c);
            c.g[S1][0] = 0x100;
            c.g[A1][0] = 0x10;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xb));
            c.g[A0][0] = tex.off(ti::OWNED).lw(c);
            c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
            if c.g[V1][0] == 0 {
                c.g[S1][0] = c.g[A1][0];
            }
            if c.g[A0][0] != 0 {
                c.cycles += 15;
                c.g[V0][0] = Addr(c.g[S6][0] as u32).off(tm::ALLOC_FLAGS).lw(c);
                c.g[A2][0] = 0x0500_0000;
                c.g[A1][0] = c.g[FP][0];
                c.g[A0][0] = 0x400;
                c.g[A2][0] |= c.g[V0][0];
                c.g[A3][0] = 0;
                call_ptr(c, TEX_LOAD_NEW, 0x0036_7848)?;
                c.mult(0, 0, S1, S2);
                c.g[A1][0] = Addr(frame.at(c, 0xe0)).lw(c);
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.lo[0];
                Addr(c.g[S0][0] as u32).off(ti::CLUT).sw(c, c.g[V0][0]);
                call_ptr(c, TEX_LOAD_MEMCOPY, 0x0036_7860)?;
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(frame.at(c, 0xe0)).lw(c);
                tex.off(ti::CLUT).sw(c, c.g[V0][0]);
                c.cycles += 1;
            }
        } else {
            c.cycles += 1;
            tex.off(ti::CLUT).sw(c, 0);
            c.cycles += 1;
        }
        let tex = Addr(c.g[S0][0] as u32);
        c.g[T0][0] = 0;

        // TEX1 fields: LCM (t0), MXL, MMAG, MMIN (a3), L (t2), K (t1).
        c.cycles += 4;
        c.g[T2][0] = 0;
        c.g[T1][0] = sx(-0xb9i32 as u32);
        c.g[A3][0] = 5;
        if c.g[S5][0] == 0 {
            c.cycles += 4;
            c.g[T0][0] = 1;
            c.g[T2][0] = c.g[S5][0];
            c.g[T1][0] = 0;
            c.g[A3][0] = 1;
        }

        // log2(w - 1) and log2(h - 1) (plzcw).
        c.cycles += 19;
        c.g[V1][0] = Addr(frame.at(c, 0x20)).lw(c);
        c.g[V0][0] = (c.g[V1][0] << 32) >> 32;
        c.plzcw(V0, V0);
        c.g[V0][0] ^= 0x1f;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        c.g[A1][0] = Addr(frame.at(c, 0x30)).lw(c);
        c.g[A0][0] = ((c.g[V0][0] << 32) as i64 >> 32) as u64;
        c.g[V1][0] = (c.g[A1][0] << 32) >> 32;
        c.plzcw(V1, V1);
        c.g[V1][0] ^= 0x1f;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        c.g[V1][0] = ((c.g[V1][0] << 32) as i64 >> 32) as u64;
        c.g[V0][0] = 1;
        let is1 = c.g[S4][0] == 1;
        c.g[V0][0] = ((c.g[S4][0] as i64) < 2) as u64;
        // CLUT PSM (a2): PSMCT16 (2) for clutFmt 2, else PSMCT32.
        if is1 {
            c.cycles += 1;
            c.g[A2][0] = 0;
        } else {
            c.cycles += 2;
            c.g[A2][0] = 0;
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.g[V0][0] = 2;
                c.g[A2][0] = 2;
                if c.g[S4][0] != 2 {
                    c.cycles += 1;
                    c.g[A2][0] = 0;
                }
            }
        }

        // TEX0 = TBW | PSM | TW | TH | TCC | CPSM | (bpp <= 8: CLD 4).
        c.cycles += 21;
        c.g[T3][0] = Addr(frame.at(c, 0x4)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
        c.g[A0][0] <<= 26;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        c.g[V0][0] = sx((c.g[T3][0] as u32) >> 6);
        c.g[V1][0] <<= 30;
        c.g[T3][0] = Addr(frame.at(c, 0x18)).ld(c);
        c.g[V0][0] <<= 32;
        c.g[V0][0] >>= 18;
        c.g[A1][0] = c.g[T3][0] << 20;
        c.g[V0][0] |= c.g[A1][0];
        c.g[V0][0] |= c.g[A0][0];
        c.g[V0][0] |= c.g[V1][0];
        c.g[A0][0] = 0x8000 << 19;
        c.g[V0][0] |= c.g[A0][0];
        c.g[V1][0] = c.g[A2][0] << 51;
        c.g[A2][0] = c.g[V0][0] | c.g[V1][0];
        c.g[V0][0] = Addr(frame.at(c, 0xc)).lw(c);
        c.g[A1][0] = Addr(frame.at(c, 0x10)).ld(c);
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = 0x8000 << 46;
            c.g[A2][0] |= c.g[V0][0];
        }

        c.cycles += 15;
        c.g[A0][0] = c.g[A3][0] << 6;
        c.g[V0][0] = c.g[T2][0];
        c.g[T3][0] = Addr(frame.at(c, 0x8)).lw(c);
        c.g[V1][0] = c.g[A1][0] >> 30;
        c.g[V0][0] <<= 19;
        c.g[A1][0] = 0x20;
        c.g[V1][0] |= c.g[T0][0];
        c.g[A0][0] |= c.g[A1][0];
        Addr(c.g[T3][0] as u32).sd(c, c.g[A2][0]);
        c.g[V1][0] |= c.g[A0][0];
        c.g[V1][0] |= c.g[V0][0];
        c.g[A0][0] = c.g[T1][0] << 32;
        c.g[V1][0] |= c.g[A0][0];
        tex.off(ti::TEX1).sd(c, c.g[V1][0]);
        if c.g[S5][0] != 0 {
            // MIPTBP1/2: the buffer widths of levels 1..6.
            c.cycles += 26;
            let w = c.g[S3][0] as u32;
            let width = |add: u32, shift: u32| sx(w.wrapping_add(add) >> shift);
            c.g[V0][0] = width(0x7f, 7) << 32 >> 18;
            c.g[V1][0] = width(0x3ff, 10) << 32 >> 18;
            c.g[A2][0] = width(0xff, 8) << 34;
            c.g[A3][0] = width(0x7ff, 11) << 34;
            c.g[A0][0] = width(0x1ff, 9) << 54;
            c.g[A1][0] = width(0xfff, 12) << 54;
            c.g[V0][0] |= c.g[A2][0];
            c.g[V1][0] |= c.g[A3][0];
            c.g[V0][0] |= c.g[A0][0];
            c.g[V1][0] |= c.g[A1][0];
            tex.off(ti::MIPTBP1).sd(c, c.g[V0][0]);
            tex.off(ti::MIPTBP2).sd(c, c.g[V1][0]);
        }

        // VRAM now for modes 1 and 3.
        c.cycles += 4;
        c.g[V1][0] = Addr(frame.at(c, 0xe8)).lw(c);
        c.g[V0][0] = 1;
        c.g[A1][0] = Addr(frame.at(c, 0xe8)).lw(c);
        let mode = c.g[V1][0];
        let mut reserve = mode == 1;
        if !reserve {
            c.cycles += 3;
            c.g[V0][0] = 3;
            reserve = c.g[A1][0] == 3;
        }
        if reserve {
            let three = mode != 1;
            c.cycles += if three { 21 } else { 18 };
            c.g[A1][0] = tex.off(ti::VRAM_BYTES).lw(c);
            c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(tm::VRAM_HEAP));
            c.g[A2][0] = Addr(sp(c)).lw(c);
            c.g[A3][0] = u64::MAX;
            if three {
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2000));
            }
            c.g[T0][0] = u64::MAX;
            c.g[T1][0] = 1;
            TEX_LOAD_VRAM_ALLOC.call(c, if three { 0x0036_7a88 } else { 0x0036_7a30 })?;
            c.g[A0][0] = 0x1c;
            c.g[A1][0] = Addr(c.g[S6][0] as u32).off(tm::VRAM_NODES).lw(c);
            c.mult(0, A0, V0, A0);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            let tex = Addr(c.g[S0][0] as u32);
            if three {
                c.g[V1][0] |= 0x22;
                c.g[A2][0] = sx(((c.g[V1][0] as u32) as i32 >> 8) as u32);
                Addr(c.g[A1][0] as u32).sw(c, c.g[V1][0]);
                c.g[A0][0] = sx(0u32.wrapping_sub(c.g[A2][0] as u32));
                tex.off(ti::VRAM_NODE).sw(c, c.g[V0][0]);
                c.g[A0][0] &= 0x1f;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                tex.off(ti::VRAM_BLOCK).sw(c, c.g[A2][0]);
            } else {
                c.g[V1][0] |= 2;
                Addr(c.g[A1][0] as u32).sw(c, c.g[V1][0]);
                c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 8) as u32);
                tex.off(ti::VRAM_NODE).sw(c, c.g[V0][0]);
                tex.off(ti::VRAM_BLOCK).sw(c, c.g[V1][0]);
            }
            c.cycles += 1;
        }
        let tex = Addr(c.g[S0][0] as u32);
        c.g[V0][0] = tex.lw(c);

        // PSMT8 (format 9): swizzle an owned, over-budget texture loaded in
        // mode 0 into a temporary copy; otherwise retag TEX0's PSM as T8H.
        c.cycles += 2;
        let owned = c.g[V0][0] != 0;
        c.g[V0][0] = 9;
        c.cycles += 3;
        c.g[V1][0] = tex.off(ti::FORMAT).lw(c);
        let t8 = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = Addr(sp(c)).lw(c);
        if t8 {
            let mut retag = true;
            if owned {
                c.cycles += 5;
                c.g[V0][0] = Addr(c.g[S6][0] as u32).off(tm::T8_LIMIT).lw(c);
                c.g[V1][0] = tex.off(ti::VRAM_BYTES).lw(c);
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                c.g[T3][0] = Addr(frame.at(c, 0xe8)).lw(c);
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    if c.g[T3][0] == 0 {
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S6][0];
                        c.g[A1][0] = c.g[S0][0];
                        TEX_LOAD_COPY_TO_TEMP.call(c, 0x0036_7afc)?;
                        c.g[V0][0] = Addr(sp(c)).lw(c);
                        retag = false;
                    } else {
                        c.g[V0][0] = tex.off(ti::TEX0).ld(c);
                    }
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = tex.off(ti::TEX0).ld(c);
                }
            } else {
                c.cycles += 1;
                c.g[V0][0] = tex.off(ti::TEX0).ld(c);
            }
            if retag {
                c.cycles += 7;
                c.g[V1][0] = 0xffff_ffff_fc0f_ffff;
                c.g[A0][0] = 0x01b0_0000;
                c.g[V0][0] &= c.g[V1][0];
                c.g[V0][0] |= c.g[A0][0];
                tex.off(ti::TEX0).sd(c, c.g[V0][0]);
                c.g[V0][0] = Addr(sp(c)).lw(c);
            }
        }
        tex_load_texture_exit(c, frame);
        Ok(())
    });
}

/// The epilogue (12 cycles): restore s0..s7, fp and ra, pop the frame.
pub(crate) fn tex_load_texture_exit(c: &mut Cpu, frame: Frame) {
    c.cycles += 12;
    frame.restore(c, S0, 0xd0);
    frame.restore(c, S1, 0xc0);
    frame.restore(c, S2, 0xb0);
    frame.restore(c, S3, 0xa0);
    frame.restore(c, S4, 0x90);
    frame.restore(c, S5, 0x80);
    frame.restore(c, S6, 0x70);
    frame.restore(c, S7, 0x60);
    frame.restore(c, FP, 0x50);
    frame.restore_ra(c, 0x40);
    frame.close(c);
}

pub(crate) mod ps2gm {
    #[cfg(test)]
    pub(crate) use crate::game::render::particle::ps2gm::SHAPE_BANK_COUNT;
    #[cfg(test)]
    pub(crate) use crate::game::render::particle::ps2gm::TEX_CACHE_COUNT;
}

/// `int PS2GraphicsMan_GetTextureByName(PS2GraphicsMan* self, const char* name)`
/// (0x003696c8): texture id of a 4-character shape name. Copies at most four
/// characters of `name` into a local buffer, looks it up in the cache
/// (`self + 0x280 + 12 * i`: handle, name; count at `+0xe80`), otherwise
/// appends an entry, searches the shape banks (`+0x278` count, `+0x27c` array)
/// with `SHAPE_locatez` and creates the texture through vtable slot 46
/// (`self->+0x10d8`), or stores -1. Returns the entry's handle.
#[allow(unused_labels, unused_assignments, unused_variables, unreachable_code)]
pub fn ps2_graphics_man_get_texture_by_name(c: &mut Cpu) {
    guest(c, |c| {
        'b_369724: {
            c.cycles += 12;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[V0][0] = c.lb(c.g[A1][0] as u32);
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.g[A1][0];
            if t {
                break 'b_369724;
            } else {
                c.cycles += 2;
                c.g[A2][0] = c.g[SP][0];
                c.g[V0][0] = c.lbu(c.g[A0][0] as u32);
                'l_369700: loop {
                    c.cycles += 5;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.sb(c.g[A2][0] as u32, c.g[V0][0]);
                    c.g[V1][0] = c.lb(c.g[A0][0] as u32);
                    let t = c.g[V1][0] == 0u64;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    if t {
                        break 'b_369724;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[A1][0] as u32));
                        c.g[V0][0] = ((c.g[V0][0] as i64) < 4i64) as u64;
                        if c.g[V0][0] != 0u64 {
                            c.g[V0][0] = c.lbu(c.g[A0][0] as u32);
                            if event_due(c) {
                                deliver_event_here(c, 0x00369700);
                            }
                            continue 'l_369700;
                        } else {
                            break 'b_369724;
                        }
                    }
                }
            }
        }
        'b_36986c: {
            'b_369778: {
                c.cycles += 7;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[A1][0] as u32));
                c.g[S3][0] = 0u64;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe80));
                c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sb(c.g[V1][0] as u32, 0u64);
                let t = (c.g[A0][0] as i64) <= 0;
                c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x280));
                if t {
                    break 'b_369778;
                } else {
                    c.cycles += 2;
                    c.g[S2][0] = c.g[S4][0];
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x284));
                    'l_369748: loop {
                        'b_369764: {
                            c.cycles += 5;
                            c.g[A0][0] = c.g[SP][0];
                            c.g[RA][0] = sx(0x00369754);
                            c.g[A1][0] = c.g[S1][0];
                            call_ptr(c, 0x004165a8, 0x00369754)?;
                            if c.g[V0][0] != 0u64 {
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe80));
                                break 'b_369764;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                                if t {
                                    break 'b_36986c;
                                } else {
                                    break 'b_369764;
                                }
                            }
                        }
                        c.cycles += 5;
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
                        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00369748);
                            }
                            continue 'l_369748;
                        } else {
                            break 'b_369778;
                        }
                    }
                }
            }
            'b_36983c: {
                c.cycles += 11;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe80));
                c.g[V0][0] = 0xc;
                c.g[A1][0] = c.g[SP][0];
                c.g[S1][0] = 0u64;
                c.mult(0, 3, 4, 2);
                c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                c.g[RA][0] = sx(0x00369798);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x284));
                call_ptr(c, 0x004166f4, 0x00369798)?;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x278));
                let t = (c.g[V1][0] as i64) <= 0;
                c.g[S3][0] = 0xc;
                if t {
                    break 'b_36983c;
                } else {
                    c.cycles += 3;
                    c.g[S5][0] = 0x4a0000;
                    c.g[S2][0] = c.g[S4][0];
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe80));
                    'l_3697b0: loop {
                        'b_369828: {
                            c.cycles += 10;
                            c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x27c));
                            c.mult(0, 4, 5, 19);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                            c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                            c.g[RA][0] = sx(0x003697d0);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x284));
                            call_ptr(c, 0x003b3900, 0x003697d0)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = c.g[V0][0];
                            if t {
                                break 'b_369828;
                            } else {
                                c.cycles += 20;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10d8));
                                c.g[A2][0] = sx((c.g[S5][0] as u32).wrapping_add(0x41a0));
                                c.g[A3][0] = 0u64;
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x170));
                                c.g[T0][0] = 0x1;
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x174));
                                c.g[T1][0] = 0xffffffffffffffff;
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x003697fc);
                                c.g[A0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x003697fc)?;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe80));
                                c.mult(0, 4, 3, 19);
                                c.g[V1][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                                c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe80));
                                c.mult(0, 2, 4, 19);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                c.g[V1][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                let t = true;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xe80), c.g[A0][0]);
                                if t {
                                    break 'b_36986c;
                                } else {
                                    break 'b_369828;
                                }
                            }
                        }
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x278));
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        if c.g[V0][0] != 0u64 {
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe80));
                            if event_due(c) {
                                deliver_event_here(c, 0x003697b0);
                            }
                            continue 'l_3697b0;
                        } else {
                            break 'b_36983c;
                        }
                    }
                }
            }
            c.cycles += 12;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe80));
            c.g[A0][0] = 0xc;
            c.g[A1][0] = 0xffffffffffffffff;
            c.mult(0, 3, 2, 4);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S4][0] as u32));
            c.sw(c.g[V0][0] as u32, c.g[A1][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe80));
            c.mult(0, 2, 3, 4);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
            c.sw((c.g[S0][0] as u32).wrapping_add(0xe80), c.g[V1][0]);
            break 'b_36986c;
        }
        c.cycles += 9;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = t;
        return Ok(());
    });
}

/// `GraphicsMan::GraphicsMan(self)` (0x3691f8): initialises the graphics
/// manager object: its vtable (`+0x10d8`), the global instance pointer
/// (`gp+0x2a90`), the -1-filled 16-bit index tables (`+0xee8..`, `+0xed8..`,
/// `+0xf00..`), a 16-byte register image at `0x501420` (cleared with
/// `memset(.., 0, 16)`, then edited bit by bit and copied to `+0xe88..`),
/// and copies a 0x158-byte default block with `memcpy`. Returns `self`.
/// Straight-line register code, kept in the generated order (the nested
/// loops are the compiler's unrolled table fills).
#[allow(unreachable_code)]
pub fn graphics_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
        c.g[V0][0] = 0x490000;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3950));
        c.g[S2][0] = c.g[A0][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xf50));
        c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(0xf00));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S3][0] = 0xffffffffffffffff;
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S4][0] = 0xffffffffffffffff;
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S5][0] = sx((c.g[S2][0] as u32).wrapping_add(0xee8));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S6][0] = sx((c.g[S2][0] as u32).wrapping_add(0xed8));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S7][0] = sx((c.g[S2][0] as u32).wrapping_add(0xef0));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A0][0]);
        c.g[FP][0] = 0x500000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xe88));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.sw(c.g[SP][0] as u32, c.g[V1][0]);
        c.sw((c.g[S2][0] as u32).wrapping_add(0x10d8), c.g[V0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
        c.g[V0][0] = 0x3;
        'l_369268: loop {
            c.cycles += 4;
            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14));
            c.g[A1][0] = 0u64;
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
            'l_369278: loop {
                c.cycles += 7;
                c.sh(c.g[V1][0] as u32, c.g[S3][0]);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = (c.g[A1][0] < 0x2u64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00369278);
                    }
                    continue 'l_369278;
                } else {
                    c.cycles += 6;
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = 0x10;
                    call_ptr(c, 0x00416210, 0x003692a0)?;
                    c.g[V0][0] = c.g[S1][0];
                    let t = c.g[V0][0] != c.g[S4][0];
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00369268);
                        }
                        continue 'l_369268;
                    } else {
                        c.cycles += 3;
                        c.g[V1][0] = c.g[S5][0];
                        c.g[A0][0] = 0u64;
                        c.g[A1][0] = 0xffffffffffffffff;
                        'l_3692b8: loop {
                            c.cycles += 7;
                            c.sh(c.g[V1][0] as u32, c.g[A1][0]);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = (c.g[A0][0] < 0x2u64) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x003692b8);
                                }
                                continue 'l_3692b8;
                            } else {
                                c.cycles += 9;
                                c.g[A0][0] = c.g[S6][0];
                                c.g[A1][0] = 0u64;
                                c.g[A2][0] = 0x10;
                                call_ptr(c, 0x00416210, 0x003692e4)?;
                                c.g[A3][0] = 0xffffffffffffffff;
                                c.g[T0][0] = 0xffffffffffffffff;
                                c.g[V0][0] = c.g[S7][0];
                                c.g[V1][0] = 0x3;
                                'l_3692f8: loop {
                                    c.cycles += 4;
                                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                                    c.g[V1][0] = c.g[V0][0];
                                    c.g[A0][0] = 0u64;
                                    'l_369308: loop {
                                        c.cycles += 7;
                                        c.sh(c.g[V1][0] as u32, c.g[A3][0]);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                        c.g[V0][0] = (c.g[A0][0] < 0x2u64) as u64;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x00369308);
                                            }
                                            continue 'l_369308;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V1][0] = c.g[A2][0];
                                            let t = c.g[V1][0] != c.g[T0][0];
                                            c.g[V0][0] = c.g[A1][0];
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x003692f8);
                                                }
                                                continue 'l_3692f8;
                                            } else {
                                                c.cycles += 4;
                                                c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                                                c.g[A0][0] = 0u64;
                                                c.g[A1][0] = 0xffffffffffffffff;
                                                'l_369340: loop {
                                                    c.cycles += 7;
                                                    c.sh(c.g[V1][0] as u32, c.g[A1][0]);
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                                    c.g[V0][0] = (c.g[A0][0] < 0x2u64) as u64;
                                                    let t = c.g[V0][0] != 0u64;
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0x2));
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x00369340);
                                                        }
                                                        continue 'l_369340;
                                                    } else {
                                                        c.cycles += 11;
                                                        c.g[V0][0] = 0x1;
                                                        c.g[V1][0] = 0xffffffffffffffff;
                                                        c.g[V0][0] = 0x1;
                                                        c.g[V1][0] = 0xffffffffffffffff;
                                                        c.sw(
                                                            (c.g[S2][0] as u32).wrapping_add(0xf44),
                                                            0u64,
                                                        );
                                                        c.g[V0][0] = 0x4a0000;
                                                        c.sw(
                                                            (c.g[GP][0] as u32)
                                                                .wrapping_add(0x2a90),
                                                            c.g[S2][0],
                                                        );
                                                        c.g[A0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add(0x5a10));
                                                        c.g[V1][0] = 0u64;
                                                        c.g[A1][0] = 0xffffffffffffffff;
                                                        'l_369388: loop {
                                                            c.cycles += 7;
                                                            c.sh(c.g[A0][0] as u32, c.g[A1][0]);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x1));
                                                            c.g[V0][0] =
                                                                (c.g[V1][0] < 0x2u64) as u64;
                                                            let t = c.g[V0][0] != 0u64;
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x2));
                                                            if t {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c, 0x00369388,
                                                                    );
                                                                }
                                                                continue 'l_369388;
                                                            } else {
                                                                c.cycles += 51;
                                                                c.g[S0][0] = sx((c.g[FP][0]
                                                                    as u32)
                                                                    .wrapping_add(0x1420));
                                                                c.g[A1][0] = 0u64;
                                                                c.g[A0][0] = c.g[S0][0];
                                                                c.g[A2][0] = 0x10;
                                                                call_ptr(
                                                                    c, 0x00416210, 0x003693b8,
                                                                )?;
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x4));
                                                                c.g[V0][0] = 0xfffffffffe7f0000;
                                                                c.g[V0][0] = c.g[V0][0] | 0xffff;
                                                                c.g[A1][0] = 0x800000;
                                                                c.g[V1][0] =
                                                                    c.g[V1][0] & c.g[V0][0];
                                                                c.g[A0][0] = 0xffffffffffbf0000;
                                                                c.g[A0][0] = c.g[A0][0] | 0xffff;
                                                                c.g[V1][0] =
                                                                    c.g[V1][0] | c.g[A1][0];
                                                                c.g[V0][0] = 0xffffffffffcf0000;
                                                                c.g[V1][0] =
                                                                    c.g[V1][0] & c.g[A0][0];
                                                                c.g[V0][0] = c.g[V0][0] | 0xffff;
                                                                c.g[A1][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                c.g[V1][0] =
                                                                    c.g[V1][0] & c.g[V0][0];
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[FP][0] as u32)
                                                                        .wrapping_add(0x1420));
                                                                c.g[A3][0] = 0x300000;
                                                                c.g[A0][0] = 0xfffffffffffffc1f;
                                                                c.g[V0][0] = 0xfffffffffff00000;
                                                                c.g[T1][0] = 0xfffffffffffffffc;
                                                                c.g[V1][0] =
                                                                    c.g[V1][0] | c.g[A3][0];
                                                                c.g[A1][0] =
                                                                    c.g[A1][0] & c.g[A0][0];
                                                                c.g[V0][0] = c.g[V0][0] | 0xfff;
                                                                c.g[A2][0] =
                                                                    c.g[A2][0] & c.g[T1][0];
                                                                c.g[A0][0] = 0xfffffffffffff3ff;
                                                                c.g[V1][0] =
                                                                    c.g[V1][0] & c.g[V0][0];
                                                                c.g[A2][0] =
                                                                    c.g[A2][0] & c.g[A0][0];
                                                                c.g[A1][0] = c.g[A1][0] | 0x80;
                                                                c.g[V0][0] = 0xffffffffffffffe0;
                                                                c.g[T0][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0xc));
                                                                c.g[V1][0] =
                                                                    c.g[V1][0] & c.g[T1][0];
                                                                c.g[A0][0] = 0xfffffffffffffc3f;
                                                                c.g[A1][0] =
                                                                    c.g[A1][0] & c.g[V0][0];
                                                                c.g[A3][0] = 0xffffffffc0000000;
                                                                c.g[A2][0] =
                                                                    c.g[A2][0] & c.g[A0][0];
                                                                c.sw(
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0x8),
                                                                    c.g[A1][0],
                                                                );
                                                                c.sw(
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0x4),
                                                                    c.g[V1][0],
                                                                );
                                                                c.g[T0][0] =
                                                                    c.g[T0][0] & c.g[A3][0];
                                                                c.sw(
                                                                    (c.g[FP][0] as u32)
                                                                        .wrapping_add(0x1420),
                                                                    c.g[A2][0],
                                                                );
                                                                c.g[T4][0] = c.g[S0][0];
                                                                c.sw(
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0xc),
                                                                    c.g[T0][0],
                                                                );
                                                                c.g[T3][0] = c.g[FP][0];
                                                                c.g[T7][0] = 0x5;
                                                                c.g[T6][0] = 0x1f;
                                                                c.g[T5][0] = 0xff;
                                                                c.g[T2][0] = 0u64;
                                                                c.g[T1][0] = 0xc;
                                                                c.g[T0][0] = 0x2;
                                                                'l_369470: loop {
                                                                    c.cycles += 18;
                                                                    c.g[A3][0] = sx((c.g[T6][0]
                                                                        as u32)
                                                                        << ((c.g[T0][0] as u32)
                                                                            & 31));
                                                                    c.g[A1][0] = c
                                                                        .lw((c.g[T4][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    c.g[V1][0] = sx((c.g[T7][0]
                                                                        as u32)
                                                                        << ((c.g[T0][0] as u32)
                                                                            & 31));
                                                                    c.g[V0][0] =
                                                                        !(0u64 | c.g[A3][0]);
                                                                    c.g[A2][0] = c
                                                                        .lw((c.g[T3][0] as u32)
                                                                            .wrapping_add(0x1420));
                                                                    c.g[A0][0] = sx((c.g[T5][0]
                                                                        as u32)
                                                                        << ((c.g[T1][0] as u32)
                                                                            & 31));
                                                                    c.g[A1][0] =
                                                                        c.g[A1][0] & c.g[V0][0];
                                                                    c.g[V1][0] =
                                                                        c.g[V1][0] & c.g[A3][0];
                                                                    c.g[A0][0] =
                                                                        !(0u64 | c.g[A0][0]);
                                                                    c.g[A1][0] =
                                                                        c.g[A1][0] | c.g[V1][0];
                                                                    c.g[A2][0] =
                                                                        c.g[A2][0] & c.g[A0][0];
                                                                    c.g[T2][0] = sx((c.g[T2][0]
                                                                        as u32)
                                                                        .wrapping_add(0x1));
                                                                    c.sw(
                                                                        (c.g[T4][0] as u32)
                                                                            .wrapping_add(0x4),
                                                                        c.g[A1][0],
                                                                    );
                                                                    c.g[T1][0] = sx((c.g[T1][0]
                                                                        as u32)
                                                                        .wrapping_add(0x8));
                                                                    c.sw(
                                                                        (c.g[T3][0] as u32)
                                                                            .wrapping_add(0x1420),
                                                                        c.g[A2][0],
                                                                    );
                                                                    c.g[V0][0] = (c.g[T2][0]
                                                                        < 0x2u64)
                                                                        as u64;
                                                                    let t = c.g[V0][0] != 0u64;
                                                                    c.g[T0][0] = sx((c.g[T0][0]
                                                                        as u32)
                                                                        .wrapping_add(0x5));
                                                                    if t {
                                                                        if event_due(c) {
                                                                            deliver_event_here(
                                                                                c, 0x00369470,
                                                                            );
                                                                        }
                                                                        continue 'l_369470;
                                                                    } else {
                                                                        c.cycles += 4;
                                                                        c.g[A1][0] = 0u64;
                                                                        c.g[A2][0] = 0x500000;
                                                                        c.g[A3][0] = 0x3;
                                                                        c.g[A0][0] = 0x2;
                                                                        'l_3694c8: loop {
                                                                            c.cycles += 9;
                                                                            c.g[V0][0] =
                                                                                sx((c.g[A3][0]
                                                                                    as u32)
                                                                                    << ((c.g[A0][0]
                                                                                        as u32)
                                                                                        & 31));
                                                                            c.g[V1][0] = c.lw((c.g
                                                                                [A2][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x1420,
                                                                                ));
                                                                            c.g[V0][0] = !(0u64
                                                                                | c.g[V0][0]);
                                                                            c.g[A1][0] =
                                                                                sx((c.g[A1][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x1,
                                                                                    ));
                                                                            c.g[V1][0] = c.g[V1][0]
                                                                                & c.g[V0][0];
                                                                            c.g[A0][0] =
                                                                                sx((c.g[A0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x2,
                                                                                    ));
                                                                            c.g[V0][0] = (c.g[A1]
                                                                                [0]
                                                                                < 0x2u64)
                                                                                as u64;
                                                                            let t =
                                                                                c.g[V0][0] != 0u64;
                                                                            c.sw(
                                                                                (c.g[A2][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x1420,
                                                                                    ),
                                                                                c.g[V1][0],
                                                                            );
                                                                            if t {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x003694c8);
                                                                                }
                                                                                continue 'l_3694c8;
                                                                            } else {
                                                                                c.cycles += 11;
                                                                                c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x1420));
                                                                                c.g[V0][0] =
                                                                                    0x4a0000;
                                                                                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5a10));
                                                                                c.lwl(5, (c.g[A0][0] as u32).wrapping_add(0x3));
                                                                                c.lwr(
                                                                                    5,
                                                                                    c.g[A0][0]
                                                                                        as u32,
                                                                                );
                                                                                c.swl(5, (c.g[V1][0] as u32).wrapping_add(0x13));
                                                                                c.swr(5, (c.g[V1][0] as u32).wrapping_add(0x10));
                                                                                c.g[S0][0] =
                                                                                    c.g[FP][0];
                                                                                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xe88));
                                                                                c.g[V1][0] = 0x3;
                                                                                'l_369518: loop {
                                                                                    c.cycles += 14;
                                                                                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1420));
                                                                                    c.ldl(5, (c.g[A0][0] as u32).wrapping_add(0x7));
                                                                                    c.ldr(
                                                                                        5,
                                                                                        c.g[A0][0]
                                                                                            as u32,
                                                                                    );
                                                                                    c.ldl(6, (c.g[A0][0] as u32).wrapping_add(0xf));
                                                                                    c.ldr(6, (c.g[A0][0] as u32).wrapping_add(0x8));
                                                                                    c.g[A3][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
                                                                                    c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x7));
                                                                                    c.sdr(
                                                                                        5,
                                                                                        c.g[V0][0]
                                                                                            as u32,
                                                                                    );
                                                                                    c.sdl(6, (c.g[V0][0] as u32).wrapping_add(0xf));
                                                                                    c.sdr(6, (c.g[V0][0] as u32).wrapping_add(0x8));
                                                                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                                                                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                                                                    let t = (c.g[V1]
                                                                                        [0]
                                                                                        as i64)
                                                                                        >= 0;
                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
                                                                                    if t {
                                                                                        if event_due(
                                                                                            c,
                                                                                        ) {
                                                                                            deliver_event_here(c, 0x00369518);
                                                                                        }
                                                                                        continue 'l_369518;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            33;
                                                                                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                                                                        c.g[V0][0] = 0x4a0000;
                                                                                        c.sw((c.g[S2][0] as u32).wrapping_add(0xe84), c.g[A1][0]);
                                                                                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5960));
                                                                                        c.ldl(6, (c.g[A0][0] as u32).wrapping_add(0x7));
                                                                                        c.ldr(6, c.g[A0][0] as u32);
                                                                                        c.sdl(6, (c.g[S2][0] as u32).wrapping_add(0x7));
                                                                                        c.sdr(6, c.g[S2][0] as u32);
                                                                                        c.g[A1][0] = 0xffffffffffff0000;
                                                                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                                                                        c.g[A1][0] = c.g[A1][0] | 0xffff;
                                                                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), 0u64);
                                                                                        c.g[A2]
                                                                                            [0] =
                                                                                            0x158;
                                                                                        call_ptr(c, 0x003e6448, 0x00369588)?;
                                                                                        c.g[V1]
                                                                                            [0] =
                                                                                            0x1;
                                                                                        c.g[V0]
                                                                                            [0] = c
                                                                                            .g[S2]
                                                                                            [0];
                                                                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x270), c.g[V1][0]);
                                                                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x26c), 0u64);
                                                                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x274), 0u64);
                                                                                        c.sw((c.g[S2][0] as u32).wrapping_add(0xf48), 0u64);
                                                                                        c.sw((c.g[S2][0] as u32).wrapping_add(0xf4c), 0u64);
                                                                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                                                                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                                                                                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                                                                                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                                                                                        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                                                                                        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                                                                                        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                                                                                        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                                                        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                                                        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                                                                                        let t = c.g
                                                                                            [RA][0]
                                                                                            as u32;
                                                                                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                                                                                        c.pc = t;
                                                                                        return Ok(
                                                                                            (),
                                                                                        );
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    })
}

/// `void TexMan_InitTempHeap(TexMan* this)` (0x367340): initialises the temp
/// VRAM heap at `this + 0x4350` (`VramHeap_Init(heap, this + 8)`).
pub fn tex_man_init_temp_heap(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4350));
        Callee {
            addr: 0x0036_5e40,
            f: ee::s12::VramHeap_Init,
        }
        .call(c, 0x0036_7354)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Offsets in `cTextureMan` and `tTexInfo` used by `TexMan_EmitTex0`.
#[cfg(test)]
pub(crate) const TEXMAN_BOUND: u32 = 0x1f4c;

#[cfg(test)]
pub(crate) const TEXMAN_CLUT_TEX: u32 = 0x1f54;

/// Picks the GS page size (width, height in texels) of one mip level from the
/// 29-entry jump table at `0x490000 + tab`, indexed by the pixel format minus
/// 4 (`idx`); out of range (`in_range` false) takes the 64x32 default. The
/// four table targets (`base`, +0xc, +0x18, +0x24) set (128,128), (128,64),
/// (64,64) and (64,32) into `rx`/`ry`. Returns false when the table holds
/// anything else (the generated body leaves through `jump_out`).
pub(crate) fn tm_pick(
    c: &mut Cpu,
    in_range: bool,
    idx: usize,
    tab: u32,
    rx: usize,
    ry: usize,
    base: u32,
) -> bool {
    if !in_range {
        c.cycles += 2;
        c.g[rx][0] = 0x40;
        c.g[ry][0] = 0x20;
        return true;
    }
    c.cycles += 7;
    c.g[V0][0] = 0x490000;
    c.g[V1][0] = sx((c.g[idx][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(tab));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
    let target = c.g[A0][0] as u32;
    let (cycles, w, h) = if target == base {
        (3, 0x80, 0x80)
    } else if target == base + 0xc {
        (3, 0x80, 0x40)
    } else if target == base + 0x18 {
        (3, 0x40, 0x40)
    } else if target == base + 0x24 {
        (2, 0x40, 0x20)
    } else {
        c.jump_out(target, 0x0036_8970, 0x0036_9098);
        return false;
    };
    c.cycles += cycles;
    c.g[rx][0] = w;
    c.g[ry][0] = h;
    true
}

/// `void TexMan_EmitTex0(cTextureMan* self, int idx, int slot, ulong** dma)`
/// (0x368970): binds texture `idx` to TEX slot 0 or 1 and appends the GIF
/// packet to the DMA stream. If the slot already holds `idx` and its CLUT is
/// current nothing is written; if only the CLUT differs, a 2-register CLUT
/// load (TEX0_n with the CLUT address) is emitted. Otherwise a packet with
/// MIPTBP1/2 (mip level base pointers, each level's size from the page-size
/// jump table and the pixel format shift), TEX1 and TEX0 is written, and
/// `mCLUTTex` is updated when the texture has a CLUT.
pub fn texman_emit_tex0(c: &mut Cpu) {
    c.cycles += 16;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
    c.g[T8][0] = c.g[A0][0];
    c.g[T1][0] = c.g[A2][0];
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.g[V1][0] = sx((c.g[T1][0] as u32) << 2);
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[V0][0] = sx((c.g[T8][0] as u32).wrapping_add(0x1f4c));
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sq(20, c.g[SP][0] as u32);
    c.g[S1][0] = c.g[A3][0];
    c.g[T9][0] = c.g[A1][0];
    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
    let t = c.g[A0][0] == c.g[T9][0];
    c.g[T0][0] = c.lw(c.g[S1][0] as u32);
    'epi: {
        if t {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0x1f54));
            let t = c.g[V0][0] == c.g[A0][0];
            c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
            if t {
                break 'epi;
            }
            c.cycles += 5;
            c.g[V0][0] = sx((c.g[T8][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x2c));
            let t = (c.g[V1][0] as i64) < 0;
            c.g[A2][0] = 0xe;
            if t {
                break 'epi;
            }
            c.cycles += 25;
            c.sw((c.g[T8][0] as u32).wrapping_add(0x1f54), c.g[A0][0]);
            c.g[V0][0] = 0x10000000;
            c.g[V0][0] = c.g[V0][0] | 0x3;
            c.sd(c.g[T0][0] as u32, c.g[V0][0]);
            c.g[V1][0] = 0x50000000;
            c.g[V1][0] = c.g[V1][0] | 0x2;
            c.g[V1][0] = c.g[V1][0] << 32;
            c.g[A1][0] = 0x10000000;
            c.g[A1][0] = c.g[A1][0] << 32;
            c.g[A1][0] = c.g[A1][0] | 0x8001;
            c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
            c.g[V0][0] = 0x16;
            c.sd((c.g[T0][0] as u32).wrapping_add(0x20), c.g[A1][0]);
            c.g[A0][0] = 0x17;
            c.sd((c.g[T0][0] as u32).wrapping_add(0x28), c.g[A2][0]);
            if c.g[T1][0] == 0 {
                c.g[A0][0] = c.g[V0][0];
            }
            c.sd((c.g[T0][0] as u32).wrapping_add(0x8), 0u64);
            c.sd((c.g[T0][0] as u32).wrapping_add(0x10), 0u64);
            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x2c));
            c.g[V1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x38));
            c.g[V0][0] = c.g[V0][0] << 37;
            c.sd((c.g[T0][0] as u32).wrapping_add(0x38), c.g[A0][0]);
            c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
            c.sd((c.g[T0][0] as u32).wrapping_add(0x30), c.g[V1][0]);
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x40));
            break 'epi;
        }
        c.cycles += 5;
        c.sw(c.g[V0][0] as u32, c.g[T9][0]);
        c.g[V0][0] = sx((c.g[T9][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[T8][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let t = c.g[T1][0] != 0u64;
        c.g[T2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
        if t {
            c.cycles += 4;
            c.g[S4][0] = 0x7;
            c.g[S3][0] = 0x15;
            c.g[S0][0] = 0x35;
            c.g[S2][0] = 0x37;
        } else {
            c.cycles += 5;
            c.g[S4][0] = 0x6;
            c.g[S3][0] = 0x14;
            c.g[S0][0] = 0x34;
            c.g[S2][0] = 0x36;
        }
        c.cycles += 9;
        c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0xc));
        c.g[T7][0] = 0x8;
        c.g[A1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x24));
        c.g[A0][0] = 0x6;
        c.g[V0][0] = c.g[V0][0] ^ 0x9;
        c.g[A2][0] = 0x4;
        c.g[V1][0] = ((c.g[A1][0] as i64) < 4i64) as u64;
        let t = c.g[V1][0] == 0u64;
        if c.g[V0][0] == 0 {
            c.g[T7][0] = c.g[A0][0];
        }
        if !t {
            c.cycles += 4;
            c.g[V1][0] = ((c.g[A1][0] as i64) < 1i64) as u64;
            c.g[A2][0] = 0x2;
            c.g[V0][0] = 0x3;
            if c.g[V1][0] == 0 {
                c.g[A2][0] = c.g[V0][0];
            }
        }
        c.cycles += 21;
        c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
        c.g[V0][0] = 0x50000000;
        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
        c.g[A1][0] = c.g[A2][0] | 0x8000;
        c.g[A0][0] = 0x8000;
        c.g[A0][0] = c.g[A0][0] << 45;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2));
        c.g[A1][0] = c.g[A1][0] | c.g[A0][0];
        c.g[V0][0] = 0x10000000;
        c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
        c.g[V1][0] = c.g[V1][0] << 32;
        c.g[A0][0] = 0xe;
        c.sd(c.g[T0][0] as u32, c.g[V0][0]);
        c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
        c.sd((c.g[T0][0] as u32).wrapping_add(0x20), c.g[A1][0]);
        c.sd((c.g[T0][0] as u32).wrapping_add(0x28), c.g[A0][0]);
        c.sd((c.g[T0][0] as u32).wrapping_add(0x8), 0u64);
        c.sd((c.g[T0][0] as u32).wrapping_add(0x10), 0u64);
        c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x24));
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x30));
        if !t {
            c.cycles += 18;
            c.g[A0][0] = c.ld((c.g[T2][0] as u32).wrapping_add(0x38));
            c.g[A1][0] = 0x1;
            c.g[T1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x18));
            c.g[V0][0] = c.g[A0][0] >> 26;
            c.g[V1][0] = c.g[A0][0] >> 30;
            c.g[V0][0] = c.g[V0][0] & 0xf;
            c.g[V1][0] = c.g[V1][0] & 0xf;
            c.g[V0][0] = c.g[V0][0] << 32;
            c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
            c.g[A2][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x4));
            c.g[V1][0] = c.g[V1][0] << 32;
            c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
            c.g[T4][0] = sx((c.g[A1][0] as u32) << ((c.g[V0][0] as u32) & 31));
            c.g[T5][0] = sx((c.g[A1][0] as u32) << ((c.g[V1][0] as u32) & 31));
            c.g[A3][0] = c.g[A0][0];
            c.g[V0][0] = (c.g[A2][0] < 0x1du64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[T3][0] = c.g[T1][0];
            if !tm_pick(c, !t, A2, 0x2340, A0, A1, 0x00368ad4) {
                return;
            }
            c.cycles += 34;
            c.g[V0][0] = ((c.g[T4][0] as i64) < (c.g[A0][0] as i64)) as u64;
            c.g[V1][0] = ((c.g[T5][0] as i64) < (c.g[A1][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.g[A0][0] = c.g[T4][0];
            }
            if c.g[V1][0] == 0 {
                c.g[A1][0] = c.g[T5][0];
            }
            c.mult(0, 5, 4, 5);
            c.g[V0][0] = c.g[A3][0] >> 30;
            c.g[A0][0] = c.g[A3][0] >> 26;
            c.g[V0][0] = c.g[V0][0] & 0xf;
            c.g[V1][0] = 0x1;
            c.g[A0][0] = c.g[A0][0] & 0xf;
            c.g[V0][0] = c.g[V0][0] << 32;
            c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
            c.g[A0][0] = c.g[A0][0] << 32;
            c.g[A0][0] = ((c.g[A0][0] as i64) >> 32) as u64;
            c.mult(0, 5, 5, 9);
            c.g[V0][0] = sx((c.g[V1][0] as u32) << ((c.g[V0][0] as u32) & 31));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << ((c.g[A0][0] as u32) & 31));
            c.g[A2][0] = 0xffffffffffffffff;
            c.g[T4][0] = sx(((c.g[V1][0] as u32) as i32 >> 1) as u32);
            c.g[T5][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
            c.g[A3][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x28));
            c.g[T1][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x4));
            c.g[A2][0] = ((c.g[A2][0] as i64) < (c.g[A1][0] as i64)) as u64;
            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7));
            if c.g[A2][0] != 0 {
                c.g[V1][0] = c.g[A1][0];
            }
            c.g[T6][0] = c.g[T3][0];
            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 3) as u32);
            c.g[A0][0] = (c.g[T1][0] < 0x1du64) as u64;
            c.g[V0][0] = sx(0u32.wrapping_sub(c.g[V1][0] as u32));
            c.g[V0][0] = c.g[V0][0] & 0xff;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> ((c.g[T7][0] as u32) & 31)) as u32);
            let t = c.g[A0][0] == 0u64;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
            if !tm_pick(c, !t, T1, 0x23c0, A1, A0, 0x00368ba4) {
                return;
            }
            c.cycles += 34;
            c.g[V0][0] = ((c.g[T4][0] as i64) < (c.g[A1][0] as i64)) as u64;
            c.g[V1][0] = ((c.g[T5][0] as i64) < (c.g[A0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.g[A1][0] = c.g[T4][0];
            }
            if c.g[V1][0] == 0 {
                c.g[A0][0] = c.g[T5][0];
            }
            c.mult(0, 4, 5, 4);
            c.g[V0][0] = c.ld((c.g[T2][0] as u32).wrapping_add(0x38));
            c.g[A1][0] = 0x1;
            c.g[A2][0] = 0xffffffffffffffff;
            c.g[V1][0] = c.g[V0][0] >> 30;
            c.g[T5][0] = c.g[T3][0];
            c.g[V0][0] = c.g[V0][0] >> 26;
            c.g[V1][0] = c.g[V1][0] & 0xf;
            c.mult(0, 4, 4, 14);
            c.g[V0][0] = c.g[V0][0] & 0xf;
            c.g[V0][0] = c.g[V0][0] << 32;
            c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
            c.g[V1][0] = c.g[V1][0] << 32;
            c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
            c.g[V0][0] = sx((c.g[A1][0] as u32) << ((c.g[V0][0] as u32) & 31));
            c.g[T1][0] = sx((c.g[T5][0] as u32).wrapping_sub(0x4));
            c.g[A1][0] = sx((c.g[A1][0] as u32) << ((c.g[V1][0] as u32) & 31));
            c.g[T3][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
            c.g[A2][0] = ((c.g[A2][0] as i64) < (c.g[A0][0] as i64)) as u64;
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7));
            if c.g[A2][0] != 0 {
                c.g[V1][0] = c.g[A0][0];
            }
            c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 2) as u32);
            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 3) as u32);
            c.g[A0][0] = (c.g[T1][0] < 0x1du64) as u64;
            c.g[V0][0] = sx(0u32.wrapping_sub(c.g[V1][0] as u32));
            c.g[V0][0] = c.g[V0][0] & 0xff;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> ((c.g[T7][0] as u32) & 31)) as u32);
            let t = c.g[A0][0] == 0u64;
            c.g[T4][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
            if !tm_pick(c, !t, T1, 0x2440, A2, V1, 0x00368c74) {
                return;
            }
            c.cycles += 29;
            c.g[A0][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
            c.g[V0][0] = ((c.g[T3][0] as i64) < (c.g[A2][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.g[A2][0] = c.g[T3][0];
            }
            if c.g[A0][0] == 0 {
                c.g[V1][0] = c.g[A1][0];
            }
            c.mult(0, 3, 6, 3);
            c.g[A1][0] = 0xffffffffffffffff;
            c.g[A0][0] = c.g[T4][0];
            c.g[A2][0] = c.ld((c.g[T2][0] as u32).wrapping_add(0x48));
            c.g[A0][0] = c.g[A0][0] << 20;
            c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[S0][0]);
            c.g[A0][0] = c.g[A3][0] | c.g[A0][0];
            c.mult(0, 3, 3, 13);
            c.g[A1][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x7));
            if c.g[A1][0] != 0 {
                c.g[V0][0] = c.g[V1][0];
            }
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
            c.g[V1][0] = sx(0u32.wrapping_sub(c.g[V0][0] as u32));
            c.g[V1][0] = c.g[V1][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> ((c.g[T7][0] as u32) & 31)) as u32);
            c.g[T6][0] = sx((c.g[T4][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = c.g[T6][0] << 40;
            c.g[A0][0] = c.g[A0][0] | c.g[V1][0];
            c.g[A2][0] = c.g[A2][0] | c.g[A0][0];
            c.sd(c.g[T0][0] as u32, c.g[A2][0]);
            c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x24));
            c.g[V0][0] = ((c.g[V0][0] as i64) < 4i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x10));
            if !t {
                c.cycles += 20;
                c.g[A1][0] = c.ld((c.g[T2][0] as u32).wrapping_add(0x38));
                c.g[A0][0] = 0x1;
                c.g[T1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x18));
                c.g[V0][0] = c.g[A1][0] >> 26;
                c.g[V1][0] = c.g[A1][0] >> 30;
                c.g[V0][0] = c.g[V0][0] & 0xf;
                c.g[V1][0] = c.g[V1][0] & 0xf;
                c.g[V0][0] = c.g[V0][0] << 32;
                c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                c.g[V1][0] = c.g[V1][0] << 32;
                c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
                c.g[V0][0] = sx((c.g[A0][0] as u32) << ((c.g[V0][0] as u32) & 31));
                c.g[A2][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x4));
                c.g[A0][0] = sx((c.g[A0][0] as u32) << ((c.g[V1][0] as u32) & 31));
                c.g[T4][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
                c.g[T5][0] = sx(((c.g[A0][0] as u32) as i32 >> 3) as u32);
                c.g[A3][0] = c.g[A1][0];
                c.g[V0][0] = (c.g[A2][0] < 0x1du64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[T3][0] = c.g[T1][0];
                if !tm_pick(c, !t, A2, 0x24c0, A0, A1, 0x00368d80) {
                    return;
                }
                c.cycles += 33;
                c.g[V0][0] = ((c.g[T4][0] as i64) < (c.g[A0][0] as i64)) as u64;
                c.g[V1][0] = ((c.g[T5][0] as i64) < (c.g[A1][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.g[A0][0] = c.g[T4][0];
                }
                if c.g[V1][0] == 0 {
                    c.g[A1][0] = c.g[T5][0];
                }
                c.mult(0, 5, 4, 5);
                c.g[V1][0] = c.g[A3][0] >> 30;
                c.g[A0][0] = c.g[A3][0] >> 26;
                c.g[V1][0] = c.g[V1][0] & 0xf;
                c.g[V0][0] = 0x1;
                c.g[A0][0] = c.g[A0][0] & 0xf;
                c.g[V1][0] = c.g[V1][0] << 32;
                c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
                c.g[A0][0] = c.g[A0][0] << 32;
                c.g[A0][0] = ((c.g[A0][0] as i64) >> 32) as u64;
                c.mult(0, 5, 5, 9);
                c.g[V1][0] = sx((c.g[V0][0] as u32) << ((c.g[V1][0] as u32) & 31));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << ((c.g[A0][0] as u32) & 31));
                c.g[A2][0] = 0xffffffffffffffff;
                c.g[T1][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
                c.g[T4][0] = sx(((c.g[V1][0] as u32) as i32 >> 4) as u32);
                c.g[A3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x4));
                c.g[T5][0] = c.g[T3][0];
                c.g[A2][0] = ((c.g[A2][0] as i64) < (c.g[A1][0] as i64)) as u64;
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7));
                if c.g[A2][0] != 0 {
                    c.g[V0][0] = c.g[A1][0];
                }
                c.g[A0][0] = (c.g[A3][0] < 0x1du64) as u64;
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
                c.g[V1][0] = sx(0u32.wrapping_sub(c.g[V0][0] as u32));
                c.g[V1][0] = c.g[V1][0] & 0xff;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> ((c.g[T7][0] as u32) & 31)) as u32);
                let t = c.g[A0][0] == 0u64;
                c.g[T6][0] = sx((c.g[T6][0] as u32).wrapping_add(c.g[V0][0] as u32));
                if !tm_pick(c, !t, A3, 0x2540, A1, A0, 0x00368e4c) {
                    return;
                }
                c.cycles += 34;
                c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                c.g[V1][0] = ((c.g[T4][0] as i64) < (c.g[A0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.g[A1][0] = c.g[T1][0];
                }
                if c.g[V1][0] == 0 {
                    c.g[A0][0] = c.g[T4][0];
                }
                c.mult(0, 4, 5, 4);
                c.g[V0][0] = c.ld((c.g[T2][0] as u32).wrapping_add(0x38));
                c.g[A1][0] = 0x1;
                c.g[A2][0] = 0xffffffffffffffff;
                c.g[V1][0] = c.g[V0][0] >> 30;
                c.g[T4][0] = c.g[T3][0];
                c.g[V0][0] = c.g[V0][0] >> 26;
                c.g[V1][0] = c.g[V1][0] & 0xf;
                c.mult(0, 4, 4, 13);
                c.g[V0][0] = c.g[V0][0] & 0xf;
                c.g[V0][0] = c.g[V0][0] << 32;
                c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                c.g[V1][0] = c.g[V1][0] << 32;
                c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
                c.g[V0][0] = sx((c.g[A1][0] as u32) << ((c.g[V0][0] as u32) & 31));
                c.g[A3][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x4));
                c.g[A1][0] = sx((c.g[A1][0] as u32) << ((c.g[V1][0] as u32) & 31));
                c.g[T1][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
                c.g[A2][0] = ((c.g[A2][0] as i64) < (c.g[A0][0] as i64)) as u64;
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7));
                if c.g[A2][0] != 0 {
                    c.g[V1][0] = c.g[A0][0];
                }
                c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 5) as u32);
                c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 3) as u32);
                c.g[A0][0] = (c.g[A3][0] < 0x1du64) as u64;
                c.g[V0][0] = sx(0u32.wrapping_sub(c.g[V1][0] as u32));
                c.g[V0][0] = c.g[V0][0] & 0xff;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> ((c.g[T7][0] as u32) & 31)) as u32);
                let t = c.g[A0][0] == 0u64;
                c.g[T3][0] = sx((c.g[T6][0] as u32).wrapping_add(c.g[V1][0] as u32));
                if !tm_pick(c, !t, A3, 0x25c0, A2, V1, 0x00368f1c) {
                    return;
                }
                c.cycles += 26;
                c.g[A0][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.g[A2][0] = c.g[T1][0];
                }
                if c.g[A0][0] == 0 {
                    c.g[V1][0] = c.g[A1][0];
                }
                c.mult(0, 3, 6, 3);
                c.g[A1][0] = 0xffffffffffffffff;
                c.g[A0][0] = c.g[T3][0];
                c.g[A2][0] = c.ld((c.g[T2][0] as u32).wrapping_add(0x50));
                c.g[A0][0] = c.g[A0][0] << 20;
                c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[S2][0]);
                c.g[A0][0] = c.g[T6][0] | c.g[A0][0];
                c.mult(0, 3, 3, 12);
                c.g[A1][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x7));
                if c.g[A1][0] != 0 {
                    c.g[V0][0] = c.g[V1][0];
                }
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
                c.g[V1][0] = sx(0u32.wrapping_sub(c.g[V0][0] as u32));
                c.g[V1][0] = c.g[V1][0] & 0xff;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> ((c.g[T7][0] as u32) & 31)) as u32);
                c.g[V0][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V0][0] = c.g[V0][0] << 40;
                c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
                c.g[A2][0] = c.g[A2][0] | c.g[A0][0];
                c.sd(c.g[T0][0] as u32, c.g[A2][0]);
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x10));
            }
        }
        c.cycles += 6;
        c.g[V0][0] = c.ld((c.g[T2][0] as u32).wrapping_add(0x40));
        c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[S3][0]);
        c.sd(c.g[T0][0] as u32, c.g[V0][0]);
        c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x2c));
        let t = (c.g[V0][0] as i64) < 0;
        c.g[A0][0] = 0u64;
        if !t {
            c.cycles += 2;
            c.sw((c.g[T8][0] as u32).wrapping_add(0x1f54), c.g[T9][0]);
            c.g[A0][0] = c.g[V0][0];
        }
        c.cycles += 9;
        c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x28));
        c.g[A0][0] = c.g[A0][0] << 37;
        c.g[V1][0] = c.ld((c.g[T2][0] as u32).wrapping_add(0x38));
        c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
        c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[S4][0]);
        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
        c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
    }
    c.cycles += 8;
    c.sw(c.g[S1][0] as u32, c.g[T0][0]);
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lq(20, c.g[SP][0] as u32);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
    c.pc = t;
    return;
}

/// `TexMan_EmitImageUpload` (0x00368170): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn tex_man_emit_image_upload(c: &mut Cpu) {
    if c.pc != 0x00368170 {
        return tex_man_emit_image_upload_resume(c);
    }
    guest(c, |c| {
        'b_368268: {
            'b_368264: {
                'b_368260: {
                    'b_368228: {
                        c.cycles += 43;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
                        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
                        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.g[FP][0] = c.g[A0][0];
                        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A2][0]);
                        c.g[S7][0] = 0x8;
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.g[S1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                        c.g[A1][0] = 0x1;
                        c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                        c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                        c.g[V0][0] = c.ld((c.g[S1][0] as u32).wrapping_add(0x38));
                        c.g[A2][0] = 0xa;
                        c.sw(c.g[SP][0] as u32, c.g[V1][0]);
                        c.g[A0][0] = c.g[V0][0] >> 26;
                        c.g[V1][0] = c.g[V0][0] >> 30;
                        c.g[V0][0] = c.g[V0][0] >> 20;
                        c.g[A0][0] = c.g[A0][0] & 0xf;
                        c.g[V0][0] = c.g[V0][0] & 0x3f;
                        c.g[V1][0] = c.g[V1][0] & 0xf;
                        c.g[S6][0] = c.g[V0][0] << 32;
                        c.g[S6][0] = ((c.g[S6][0] as i64) >> 32) as u64;
                        c.g[A0][0] = c.g[A0][0] << 32;
                        c.g[A0][0] = ((c.g[A0][0] as i64) >> 32) as u64;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                        c.g[V1][0] = c.g[V1][0] << 32;
                        c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
                        c.g[T9][0] = sx((c.g[A1][0] as u32) << ((c.g[V1][0] as u32) & 31));
                        c.g[T8][0] = sx((c.g[A1][0] as u32) << ((c.g[A0][0] as u32) & 31));
                        c.g[S0][0] = c.lw(c.g[V0][0] as u32);
                        c.g[S5][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                        let t = c.g[A3][0] != c.g[A2][0];
                        c.g[S4][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                        if t {
                            break 'b_368228;
                        } else {
                            c.cycles += 3;
                            c.g[S6][0] = 0u64;
                            let t = true;
                            c.g[T8][0] = sx(((c.g[T8][0] as u32) as i32 >> 1) as u32);
                            if t {
                                break 'b_368260;
                            } else {
                                break 'b_368228;
                            }
                        }
                    }
                    'b_368240: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x9;
                        let t = c.g[A3][0] != c.g[V0][0];
                        c.g[V0][0] = 0xc;
                        if t {
                            break 'b_368240;
                        } else {
                            c.cycles += 3;
                            c.g[S6][0] = 0x1b;
                            let t = true;
                            c.g[S7][0] = 0x6;
                            if t {
                                break 'b_368264;
                            } else {
                                break 'b_368240;
                            }
                        }
                    }
                    c.cycles += 2;
                    if c.g[A3][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                        break 'b_368268;
                    } else {
                        'b_36825c: {
                            c.cycles += 2;
                            let t = c.g[T8][0] != c.g[T9][0];
                            c.g[S6][0] = 0u64;
                            if t {
                                break 'b_36825c;
                            } else {
                                c.cycles += 3;
                                c.g[T8][0] = sx(((c.g[T9][0] as u32) as i32 >> 1) as u32);
                                let t = true;
                                c.g[T9][0] = sx(((c.g[T9][0] as u32) as i32 >> 2) as u32);
                                if t {
                                    break 'b_368264;
                                } else {
                                    break 'b_36825c;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[T8][0] = sx(((c.g[T8][0] as u32) as i32 >> 2) as u32);
                        break 'b_368260;
                    }
                }
                c.cycles += 1;
                c.g[T9][0] = sx(((c.g[T9][0] as u32) as i32 >> 1) as u32);
                break 'b_368264;
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
            break 'b_368268;
        }
        'b_3684a0: {
            c.cycles += 2;
            let t = (c.g[V0][0] as i64) < 0;
            c.g[S3][0] = 0u64;
            if t {
                break 'b_3684a0;
            } else {
                'l_368270: loop {
                    'b_368330: {
                        'b_368328: {
                            c.cycles += 30;
                            c.g[V1][0] = c.ld((c.g[S1][0] as u32).wrapping_add(0x38));
                            c.g[A0][0] = 0x1;
                            c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                            c.g[A1][0] = 0xffffffffffffffff;
                            c.g[V0][0] = c.g[V1][0] >> 30;
                            c.g[S2][0] = 0x100;
                            c.g[V1][0] = c.g[V1][0] >> 26;
                            c.g[V0][0] = c.g[V0][0] & 0xf;
                            c.g[V1][0] = c.g[V1][0] & 0xf;
                            c.g[V0][0] = c.g[V0][0] << 32;
                            c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                            c.g[V0][0] = sx((c.g[A0][0] as u32) << ((c.g[V0][0] as u32) & 31));
                            c.g[V1][0] = c.g[V1][0] << 32;
                            c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
                            c.g[A0][0] = sx((c.g[A0][0] as u32) << ((c.g[V1][0] as u32) & 31));
                            c.g[T3][0] = sx(((c.g[V0][0] as u32) as i32
                                >> ((c.g[S3][0] as u32) & 31))
                                as u32);
                            c.g[T2][0] = sx(((c.g[A0][0] as u32) as i32
                                >> ((c.g[S3][0] as u32) & 31))
                                as u32);
                            c.g[T4][0] = c.g[A3][0];
                            c.mult(0, 3, 10, 11);
                            c.g[A0][0] = 0xff;
                            c.g[T0][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x4));
                            c.g[A2][0] = (c.g[T0][0] < 0x1du64) as u64;
                            c.mult(0, 3, 3, 7);
                            c.g[A1][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x7));
                            if c.g[A1][0] != 0 {
                                c.g[V0][0] = c.g[V1][0];
                            }
                            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
                            c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let t = c.g[A2][0] == 0u64;
                            if c.g[A0][0] != 0 {
                                c.g[S2][0] = c.g[V0][0];
                            }
                            if t {
                                break 'b_368328;
                            } else {
                                'b_36831c: {
                                    'b_368310: {
                                        c.cycles += 7;
                                        c.g[V0][0] = 0x490000;
                                        c.g[V1][0] = sx((c.g[T0][0] as u32) << 2);
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x22c0));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                        let t = c.g[A0][0] as u32;
                                        match t {
                                            0x00368304 => {
                                                c.cycles += 3;
                                                c.g[A0][0] = 0x80;
                                                let t = true;
                                                c.g[T1][0] = 0x80;
                                                if t {
                                                    break 'b_368330;
                                                } else {
                                                    break 'b_368310;
                                                }
                                            }
                                            0x00368310 => {
                                                break 'b_368310;
                                            }
                                            0x0036831c => {
                                                break 'b_36831c;
                                            }
                                            0x00368328 => {
                                                break 'b_368328;
                                            }
                                            _ => {
                                                c.jump_out(t, 0x00368170, 0x003684f0);
                                                return Ok(());
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[A0][0] = 0x80;
                                    let t = true;
                                    c.g[T1][0] = 0x40;
                                    if t {
                                        break 'b_368330;
                                    } else {
                                        break 'b_36831c;
                                    }
                                }
                                c.cycles += 3;
                                c.g[A0][0] = 0x40;
                                let t = true;
                                c.g[T1][0] = 0x40;
                                if t {
                                    break 'b_368330;
                                } else {
                                    break 'b_368328;
                                }
                            }
                        }
                        c.cycles += 2;
                        c.g[A0][0] = 0x40;
                        c.g[T1][0] = 0x20;
                        break 'b_368330;
                    }
                    c.cycles += 92;
                    c.g[V0][0] = ((c.g[T2][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    c.g[V1][0] = ((c.g[T3][0] as i64) < (c.g[T1][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[A0][0] = c.g[T2][0];
                    }
                    if c.g[V1][0] == 0 {
                        c.g[T1][0] = c.g[T3][0];
                    }
                    c.mult(0, 9, 4, 9);
                    c.g[A1][0] = sx(((c.g[S2][0] as u32) as i32 >> 4) as u32);
                    c.g[V1][0] = sx((c.g[T8][0] as u32).wrapping_add(0x3f));
                    c.g[T0][0] = 0x1;
                    c.g[A0][0] = 0u64;
                    c.g[A2][0] = c.g[T0][0];
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 6) as u32);
                    if c.g[V0][0] != 0 {
                        c.g[A2][0] = c.g[A1][0];
                    }
                    c.g[A0][0] = c.g[V0][0];
                    c.g[V1][0] = c.g[V1][0] << 48;
                    c.g[A3][0] = c.g[S4][0] << 32;
                    c.g[V0][0] = 0x30000000;
                    c.g[A3][0] = c.g[A3][0] | c.g[V1][0];
                    c.mult(0, 9, 9, 12);
                    c.g[A2][0] = c.g[A2][0] | c.g[V0][0];
                    if c.g[A0][0] != 0 {
                        c.g[T0][0] = c.g[A1][0];
                    }
                    c.g[V1][0] = 0x8000;
                    c.g[V1][0] = c.g[V1][0] << 44;
                    c.g[V0][0] = c.g[S6][0] << 56;
                    c.g[T0][0] = c.g[T0][0] | c.g[V1][0];
                    c.g[A3][0] = c.g[A3][0] | c.g[V0][0];
                    c.por(3, 0, 0);
                    c.g[V1][0] = 0x10000000;
                    c.g[V1][0] = c.g[V1][0] | 0x6;
                    c.sq(3, c.g[S0][0] as u32);
                    c.g[V0][0] = 0xffffffffffffffff;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[T1][0] as i64)) as u64;
                    c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x7));
                    c.g[T3][0] = c.g[S5][0] << 32;
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = c.g[T1][0];
                    }
                    c.g[T2][0] = c.g[T9][0] << 32;
                    c.g[T3][0] = c.g[A2][0] | c.g[T3][0];
                    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 3) as u32);
                    c.g[T2][0] = c.g[T8][0] | c.g[T2][0];
                    c.g[T7][0] = 0xe;
                    c.g[A0][0] = 0x10000000;
                    c.g[A0][0] = c.g[A0][0] << 32;
                    c.g[A0][0] = c.g[A0][0] | 0x4;
                    c.g[A1][0] = 0x50;
                    c.g[A2][0] = 0x51;
                    c.g[V0][0] = sx(0u32.wrapping_sub(c.g[V1][0] as u32));
                    c.g[T4][0] = 0x52;
                    c.g[T5][0] = 0x53;
                    c.g[T6][0] = 0x10000000;
                    c.g[T6][0] = c.g[T6][0] | 0x2;
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                    c.g[V0][0] = c.g[V0][0] & 0xff;
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x28), c.g[A1][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x38), c.g[A2][0]);
                    c.g[A0][0] = 0x10000000;
                    c.g[A0][0] = c.g[A0][0] << 32;
                    c.g[A0][0] = c.g[A0][0] | 0x1;
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x20), c.g[A3][0]);
                    c.g[A2][0] = 0x10000000;
                    c.g[A2][0] = c.g[A2][0] << 32;
                    c.g[A2][0] = c.g[A2][0] | 0x8001;
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x40), c.g[T2][0]);
                    c.g[A1][0] = 0x3f;
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x48), c.g[T4][0]);
                    c.g[V1][0] =
                        sx(((c.g[V1][0] as u32) as i32 >> ((c.g[S7][0] as u32) & 31)) as u32);
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x58), c.g[T5][0]);
                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x60), c.g[T0][0]);
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x70), c.g[T3][0]);
                    c.g[T8][0] = sx(((c.g[T8][0] as u32) as i32 >> 1) as u32);
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x80), c.g[T6][0]);
                    c.g[T9][0] = sx(((c.g[T9][0] as u32) as i32 >> 1) as u32);
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x18), c.g[T7][0]);
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x30), 0u64);
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x50), 0u64);
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x68), 0u64);
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x78), 0u64);
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x88), 0u64);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x98), c.g[T7][0]);
                    c.g[V0][0] = c.g[S3][0] ^ c.g[V0][0];
                    c.sd((c.g[S0][0] as u32).wrapping_add(0xa8), c.g[A1][0]);
                    if c.g[V0][0] == 0 {
                        c.g[A0][0] = c.g[A2][0];
                    }
                    c.sd((c.g[S0][0] as u32).wrapping_add(0xa0), 0u64);
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x90), c.g[A0][0]);
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S3][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xb0));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00368270);
                        }
                        continue 'l_368270;
                    } else {
                        break 'b_3684a0;
                    }
                }
            }
        }
        c.cycles += 20;
        c.g[A0][0] = c.g[FP][0];
        c.g[RA][0] = sx(0x003684ac);
        c.g[A1][0] = c.g[SP][0];
        Callee {
            addr: 0x00368138,
            f: ee::s12::TexMan_EmitPath3Window,
        }
        .call(c, 0x003684ac)?;
        c.g[V0][0] = c.lw(c.g[SP][0] as u32);
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sw(c.g[V1][0] as u32, c.g[V0][0]);
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sw(c.g[V0][0] as u32, c.g[S0][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        return Ok(());
    });
}

/// Native resume of `tex_man_emit_image_upload` (converted from the `entries` body): any entry
/// other than 0x00368170, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn tex_man_emit_image_upload_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00368170 => {
                    c.cycles += 43;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
                    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[FP][0] = c.g[A0][0];
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A2][0]);
                    c.g[S7][0] = 0x8;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[S1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.g[A1][0] = 0x1;
                    c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                    c.g[V0][0] = c.ld((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[A2][0] = 0xa;
                    c.sw(c.g[SP][0] as u32, c.g[V1][0]);
                    c.g[A0][0] = c.g[V0][0] >> 26;
                    c.g[V1][0] = c.g[V0][0] >> 30;
                    c.g[V0][0] = c.g[V0][0] >> 20;
                    c.g[A0][0] = c.g[A0][0] & 0xf;
                    c.g[V0][0] = c.g[V0][0] & 0x3f;
                    c.g[V1][0] = c.g[V1][0] & 0xf;
                    c.g[S6][0] = c.g[V0][0] << 32;
                    c.g[S6][0] = ((c.g[S6][0] as i64) >> 32) as u64;
                    c.g[A0][0] = c.g[A0][0] << 32;
                    c.g[A0][0] = ((c.g[A0][0] as i64) >> 32) as u64;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.g[V1][0] << 32;
                    c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
                    c.g[T9][0] = sx((c.g[A1][0] as u32) << ((c.g[V1][0] as u32) & 31));
                    c.g[T8][0] = sx((c.g[A1][0] as u32) << ((c.g[A0][0] as u32) & 31));
                    c.g[S0][0] = c.lw(c.g[V0][0] as u32);
                    c.g[S5][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                    let t = c.g[A3][0] != c.g[A2][0];
                    c.g[S4][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                    if t {
                        pc = 0x00368228;
                        continue;
                    }
                    pc = 0x0036821c;
                    continue;
                }
                0x0036821c => {
                    c.cycles += 3;
                    c.g[S6][0] = 0u64;
                    let t = true;
                    c.g[T8][0] = sx(((c.g[T8][0] as u32) as i32 >> 1) as u32);
                    if t {
                        pc = 0x00368260;
                        continue;
                    }
                    pc = 0x00368228;
                    continue;
                }
                0x00368228 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x9;
                    let t = c.g[A3][0] != c.g[V0][0];
                    c.g[V0][0] = 0xc;
                    if t {
                        pc = 0x00368240;
                        continue;
                    }
                    pc = 0x00368234;
                    continue;
                }
                0x00368234 => {
                    c.cycles += 3;
                    c.g[S6][0] = 0x1b;
                    let t = true;
                    c.g[S7][0] = 0x6;
                    if t {
                        pc = 0x00368264;
                        continue;
                    }
                    pc = 0x00368240;
                    continue;
                }
                0x00368240 => {
                    c.cycles += 2;

                    if c.g[A3][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                        pc = 0x00368268;
                        continue;
                    }
                    pc = 0x00368248;
                    continue;
                }
                0x00368248 => {
                    c.cycles += 2;
                    let t = c.g[T8][0] != c.g[T9][0];
                    c.g[S6][0] = 0u64;
                    if t {
                        pc = 0x0036825c;
                        continue;
                    }
                    pc = 0x00368250;
                    continue;
                }
                0x00368250 => {
                    c.cycles += 3;
                    c.g[T8][0] = sx(((c.g[T9][0] as u32) as i32 >> 1) as u32);
                    let t = true;
                    c.g[T9][0] = sx(((c.g[T9][0] as u32) as i32 >> 2) as u32);
                    if t {
                        pc = 0x00368264;
                        continue;
                    }
                    pc = 0x0036825c;
                    continue;
                }
                0x0036825c => {
                    c.cycles += 1;
                    c.g[T8][0] = sx(((c.g[T8][0] as u32) as i32 >> 2) as u32);
                    pc = 0x00368260;
                    continue;
                }
                0x00368260 => {
                    c.cycles += 1;
                    c.g[T9][0] = sx(((c.g[T9][0] as u32) as i32 >> 1) as u32);
                    pc = 0x00368264;
                    continue;
                }
                0x00368264 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                    pc = 0x00368268;
                    continue;
                }
                0x00368268 => {
                    c.cycles += 2;
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[S3][0] = 0u64;
                    if t {
                        pc = 0x003684a0;
                        continue;
                    }
                    pc = 0x00368270;
                    continue;
                }
                0x00368270 => {
                    c.cycles += 30;
                    c.g[V1][0] = c.ld((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[A0][0] = 0x1;
                    c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    c.g[A1][0] = 0xffffffffffffffff;
                    c.g[V0][0] = c.g[V1][0] >> 30;
                    c.g[S2][0] = 0x100;
                    c.g[V1][0] = c.g[V1][0] >> 26;
                    c.g[V0][0] = c.g[V0][0] & 0xf;
                    c.g[V1][0] = c.g[V1][0] & 0xf;
                    c.g[V0][0] = c.g[V0][0] << 32;
                    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                    c.g[V0][0] = sx((c.g[A0][0] as u32) << ((c.g[V0][0] as u32) & 31));
                    c.g[V1][0] = c.g[V1][0] << 32;
                    c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << ((c.g[V1][0] as u32) & 31));
                    c.g[T3][0] =
                        sx(((c.g[V0][0] as u32) as i32 >> ((c.g[S3][0] as u32) & 31)) as u32);
                    c.g[T2][0] =
                        sx(((c.g[A0][0] as u32) as i32 >> ((c.g[S3][0] as u32) & 31)) as u32);
                    c.g[T4][0] = c.g[A3][0];
                    c.mult(0, 3, 10, 11);
                    c.g[A0][0] = 0xff;
                    c.g[T0][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x4));
                    c.g[A2][0] = (c.g[T0][0] < 0x1du64) as u64;
                    c.mult(0, 3, 3, 7);
                    c.g[A1][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x7));
                    if c.g[A1][0] != 0 {
                        c.g[V0][0] = c.g[V1][0];
                    }
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
                    c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[A2][0] == 0u64;
                    if c.g[A0][0] != 0 {
                        c.g[S2][0] = c.g[V0][0];
                    }
                    if t {
                        pc = 0x00368328;
                        continue;
                    }
                    pc = 0x003682e8;
                    continue;
                }
                0x003682e8 => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x490000;
                    c.g[V1][0] = sx((c.g[T0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x22c0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x00368304 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x80;
                    let t = true;
                    c.g[T1][0] = 0x80;
                    if t {
                        pc = 0x00368330;
                        continue;
                    }
                    pc = 0x00368310;
                    continue;
                }
                0x00368310 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x80;
                    let t = true;
                    c.g[T1][0] = 0x40;
                    if t {
                        pc = 0x00368330;
                        continue;
                    }
                    pc = 0x0036831c;
                    continue;
                }
                0x0036831c => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x40;
                    let t = true;
                    c.g[T1][0] = 0x40;
                    if t {
                        pc = 0x00368330;
                        continue;
                    }
                    pc = 0x00368328;
                    continue;
                }
                0x00368328 => {
                    c.cycles += 2;
                    c.g[A0][0] = 0x40;
                    c.g[T1][0] = 0x20;
                    pc = 0x00368330;
                    continue;
                }
                0x00368330 => {
                    c.cycles += 92;
                    c.g[V0][0] = ((c.g[T2][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    c.g[V1][0] = ((c.g[T3][0] as i64) < (c.g[T1][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[A0][0] = c.g[T2][0];
                    }
                    if c.g[V1][0] == 0 {
                        c.g[T1][0] = c.g[T3][0];
                    }
                    c.mult(0, 9, 4, 9);
                    c.g[A1][0] = sx(((c.g[S2][0] as u32) as i32 >> 4) as u32);
                    c.g[V1][0] = sx((c.g[T8][0] as u32).wrapping_add(0x3f));
                    c.g[T0][0] = 0x1;
                    c.g[A0][0] = 0u64;
                    c.g[A2][0] = c.g[T0][0];
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 6) as u32);
                    if c.g[V0][0] != 0 {
                        c.g[A2][0] = c.g[A1][0];
                    }
                    c.g[A0][0] = c.g[V0][0];
                    c.g[V1][0] = c.g[V1][0] << 48;
                    c.g[A3][0] = c.g[S4][0] << 32;
                    c.g[V0][0] = 0x30000000;
                    c.g[A3][0] = c.g[A3][0] | c.g[V1][0];
                    c.mult(0, 9, 9, 12);
                    c.g[A2][0] = c.g[A2][0] | c.g[V0][0];
                    if c.g[A0][0] != 0 {
                        c.g[T0][0] = c.g[A1][0];
                    }
                    c.g[V1][0] = 0x8000;
                    c.g[V1][0] = c.g[V1][0] << 44;
                    c.g[V0][0] = c.g[S6][0] << 56;
                    c.g[T0][0] = c.g[T0][0] | c.g[V1][0];
                    c.g[A3][0] = c.g[A3][0] | c.g[V0][0];
                    c.por(3, 0, 0);
                    c.g[V1][0] = 0x10000000;
                    c.g[V1][0] = c.g[V1][0] | 0x6;
                    c.sq(3, c.g[S0][0] as u32);
                    c.g[V0][0] = 0xffffffffffffffff;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[T1][0] as i64)) as u64;
                    c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x7));
                    c.g[T3][0] = c.g[S5][0] << 32;
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = c.g[T1][0];
                    }
                    c.g[T2][0] = c.g[T9][0] << 32;
                    c.g[T3][0] = c.g[A2][0] | c.g[T3][0];
                    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 3) as u32);
                    c.g[T2][0] = c.g[T8][0] | c.g[T2][0];
                    c.g[T7][0] = 0xe;
                    c.g[A0][0] = 0x10000000;
                    c.g[A0][0] = c.g[A0][0] << 32;
                    c.g[A0][0] = c.g[A0][0] | 0x4;
                    c.g[A1][0] = 0x50;
                    c.g[A2][0] = 0x51;
                    c.g[V0][0] = sx(0u32.wrapping_sub(c.g[V1][0] as u32));
                    c.g[T4][0] = 0x52;
                    c.g[T5][0] = 0x53;
                    c.g[T6][0] = 0x10000000;
                    c.g[T6][0] = c.g[T6][0] | 0x2;
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                    c.g[V0][0] = c.g[V0][0] & 0xff;
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x28), c.g[A1][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x38), c.g[A2][0]);
                    c.g[A0][0] = 0x10000000;
                    c.g[A0][0] = c.g[A0][0] << 32;
                    c.g[A0][0] = c.g[A0][0] | 0x1;
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x20), c.g[A3][0]);
                    c.g[A2][0] = 0x10000000;
                    c.g[A2][0] = c.g[A2][0] << 32;
                    c.g[A2][0] = c.g[A2][0] | 0x8001;
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x40), c.g[T2][0]);
                    c.g[A1][0] = 0x3f;
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x48), c.g[T4][0]);
                    c.g[V1][0] =
                        sx(((c.g[V1][0] as u32) as i32 >> ((c.g[S7][0] as u32) & 31)) as u32);
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x58), c.g[T5][0]);
                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x60), c.g[T0][0]);
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x70), c.g[T3][0]);
                    c.g[T8][0] = sx(((c.g[T8][0] as u32) as i32 >> 1) as u32);
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x80), c.g[T6][0]);
                    c.g[T9][0] = sx(((c.g[T9][0] as u32) as i32 >> 1) as u32);
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x18), c.g[T7][0]);
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x30), 0u64);
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x50), 0u64);
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x68), 0u64);
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x78), 0u64);
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x88), 0u64);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x98), c.g[T7][0]);
                    c.g[V0][0] = c.g[S3][0] ^ c.g[V0][0];
                    c.sd((c.g[S0][0] as u32).wrapping_add(0xa8), c.g[A1][0]);
                    if c.g[V0][0] == 0 {
                        c.g[A0][0] = c.g[A2][0];
                    }
                    c.sd((c.g[S0][0] as u32).wrapping_add(0xa0), 0u64);
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x90), c.g[A0][0]);
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S3][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xb0));
                    if t {
                        if back_edge_unwinds(c, 0x00368270) {
                            return Ok(());
                        }
                        pc = 0x00368270;
                        continue;
                    }
                    pc = 0x003684a0;
                    continue;
                }
                0x003684a0 => {
                    c.cycles += 20;
                    c.g[A0][0] = c.g[FP][0];
                    c.g[RA][0] = sx(0x003684ac);
                    c.g[A1][0] = c.g[SP][0];
                    Callee {
                        addr: 0x00368138,
                        f: ee::s12::TexMan_EmitPath3Window,
                    }
                    .call(c, 0x003684ac)?;
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sw(c.g[V0][0] as u32, c.g[S0][0]);
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00368170, 0x003684f0);
                    return Ok(());
                }
            }
        }
    });
}

// ---------------------------------------------------------------------------
// TexMan_DestroyNBTexInfoArray (0x367230)

/// `TexMan_DestroyNBTexInfoArray` (0x367230): a countdown over 0x1f3 slots
/// with no memory effect and no callees. The loop variable starts at 498
/// and ends at -2 (the last test sees -1); each back edge takes an event
/// check, and the exit adds 2 cycles.
pub fn tex_man_destroy_nb_tex_info_array(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx(0x1f3u32.wrapping_sub(1));
    loop {
        c.cycles += 7;
        let more = (c.g[V0][0] as i64) >= 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0036_7238);
        }
    }
    c.cycles += 2;
    ret(c);
}

/// `s0 + off` for the texture manager functions (`s0` holds the table base in
/// `TexMan_EvictTexture` and the table entry's record in `TexMan_UpdateTextureData`).
pub(crate) fn texman_s0(c: &Cpu, off: u32) -> u32 {
    (c.g[S0][0] as u32).wrapping_add(off)
}

/// `void TexMan_DestroyBTexInfoArray(void)` (0x367310): counts v0 down from
/// 0x5da (one step per slot), checking events at each back edge, then returns.
pub fn texman_destroy_btexinfo_array(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx(0x5db_u32.wrapping_sub(1));
    loop {
        c.cycles += 7;
        let more = (c.g[V0][0] as i64) >= 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0036_7318);
        }
    }
    c.cycles += 2;
    ret(c);
}

/// `void TexMan_UpdateTextureData(TexMan* gm, int idx, int a2, int a3..a7,
/// int a8, int a9)` (0x367d20). Takes the record `gm[idx]` (its pointer at
/// +8). When `a9` is set it calls the table function 0x3e6574 once with the
/// record's +0x10 pointer if `a2` is set, and once with `a8` (shifted by the
/// record's +0x18) if `a8` is set. Then it evicts the texture.
pub fn texman_update_texture_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        // a9 and a8 are the 9th and 10th arguments, at the caller's sp.
        c.g[V1][0] = Addr(frame.at(c, 0x58)).lw(c);
        c.g[A1][0] = c.g[A2][0];
        c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8)).lw(c);
        c.g[S3][0] = Addr(frame.at(c, 0x50)).lw(c);
        if c.g[V1][0] != 0 {
            c.cycles += 2;
            if c.g[A1][0] != 0 {
                c.cycles += 3;
                c.g[A0][0] = Addr(texman_s0(c, 0x1c)).lw(c);
                c.g[A2][0] = Addr(texman_s0(c, 0x10)).lw(c);
                call_ptr(c, 0x003e_6574, 0x0036_7d70)?;
            }
            c.cycles += 2;
            c.g[A2][0] = 0x4;
            if c.g[S3][0] != 0 {
                c.cycles += 5;
                c.g[V0][0] = Addr(texman_s0(c, 0x18)).lw(c);
                c.g[A0][0] = Addr(texman_s0(c, 0x20)).lw(c);
                c.g[A1][0] = c.g[S3][0];
                c.g[A2][0] = sx((c.g[A2][0] as u32) << ((c.g[V0][0] as u32) & 31));
                call_ptr(c, 0x003e_6574, 0x0036_7d8c)?;
            }
        }
        c.cycles += 10;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        Callee {
            addr: 0x0036_9098,
            f: ee::s12::f_00369098,
        }
        .call(c, 0x0036_7d98)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void TexMan_EvictTexture(TexMan* gm, int idx)` (0x369098). Looks at the
/// record `gm[idx]`: when its state word at +0x28 is -1 nothing is freed; else
/// the VRAM block list at `gm+0x1f60` is freed (tag 9 at +0xc: the list at
/// `gm+0x4350` instead). Then two words at `gm+0x1f50` and `gm+0x1f4c` are
/// set to -1, and `gm+0x1f54` gets the -1 as well.
pub fn texman_evict_texture(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = u64::MAX;
        c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8)).lw(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x28)).lw(c);
        let unused = c.g[V1][0] == c.g[A0][0];
        c.g[V1][0] = 0x1;
        if !unused {
            c.cycles += 4;
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
            c.g[V0][0] = 0x9;
            if c.g[V1][0] != c.g[V0][0] {
                c.cycles += 4;
                c.g[A0][0] = sx(texman_s0(c, 0x1f60));
                Callee {
                    addr: 0x0036_6e30,
                    f: ee::s12::f_00366e30,
                }
                .call(c, 0x0036_90dc)?;
                c.g[A0][0] = u64::MAX;
            } else {
                c.cycles += 3;
                c.g[A0][0] = sx(texman_s0(c, 0x4350));
                Callee {
                    addr: 0x0036_6e30,
                    f: ee::s12::f_00366e30,
                }
                .call(c, 0x0036_90ec)?;
                c.g[A0][0] = u64::MAX;
            }
            c.cycles += 1;
            c.g[V1][0] = 0x1;
        }
        c.cycles += 1;
        c.g[V0][0] = sx(texman_s0(c, 0x1f50));
        loop {
            c.cycles += 7;
            let fill = c.g[A0][0];
            Addr(c.g[V0][0] as u32).sw(c, fill);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0036_90f8);
            }
        }
        c.cycles += 6;
        c.g[V0][0] = u64::MAX;
        frame.restore_ra(c, 0);
        let minus_one = c.g[V0][0];
        Addr(texman_s0(c, 0x1f54)).sw(c, minus_one);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    pub(crate) use crate::game::fe::debugfe::tests::oc798_install_fakes;

    use crate::difftest::events::check_events;

    use crate::difftest::events::schedule;

    use crate::difftest::fakes::machine;

    use crate::difftest::fill;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::ai::tests::leaf;

    pub(crate) use crate::game::animation::animmodel::tests::SEEDS_4;
    pub(crate) use crate::game::bx::bxstring::tests::alloc_ready;

    pub(crate) use crate::game::fe::feasyncfile::tests::base_2 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;

    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_6;

    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_4;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;

    pub(crate) use crate::game::fe::festatetrophyroom::tests::belib;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::w;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::SEEDS;
    pub(crate) use crate::game::fe::ovstateenterlodge::tests::run as run_2;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;

    pub(crate) use crate::game::movie::movieplayer::tests::base_3;
    pub(crate) use crate::game::movie::movieplayer::tests::ret_slot;

    pub(crate) use crate::game::movie::movieplayer::tests::RET_ZERO;

    pub(crate) use crate::game::render::graphicsman::tests::same;
    pub(crate) use crate::game::render::particle::tests::heap_block;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS_2;
    pub(crate) use crate::game::worldpainter::quadtree::tests::heap_init;
    pub(crate) use crate::game::worldpainter::quadtree::tests::pool_allocator;

    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready as alloc_ready_2;

    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS_3;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn tex_man_unlock_vram_matches() {
        check_lohi(
            ssx3_ee::s12::f_00367b60,
            tex_man_unlock_vram,
            200,
            &|c, seed| {
                let mut r = Rng(0x102d ^ seed as u64);
                scramble(c, &mut r);
                let index = (seed % 4) as u32;
                let p = DATA + 0x300;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = index as u64;
                c.bus.write32(DATA + index * 4 + 8, p);
                c.bus.write32(
                    p + 0x28,
                    if seed % 3 == 0 {
                        0xffff_ffff
                    } else {
                        r.next_u32()
                    },
                );
                c.bus
                    .write32(DATA + 0xc, if seed % 2 == 0 { 9 } else { r.next_u32() });
                let idx30 = seed % 4;
                c.bus.write32(DATA + 0x30, idx30);
                let v0 = DATA + if seed % 2 == 0 { 0x1f60 } else { 0x4350 };
                let q = DATA + 0x400;
                c.bus.write32(v0 + 0x1ff0, q);
                c.bus.write32(q + idx30 * 0x1c, r.next_u32());
            },
        );
    }

    #[test]
    fn tex_man_emit_path3_window_matches() {
        check(
            ssx3_ee::s12::f_00368138,
            tex_man_emit_path3_window,
            200,
            &|c, seed| {
                let mut r = Rng(0x102e ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = DATA as u64;
                c.bus.write32(DATA, DATA + 0x100);
                c.bus.write64(
                    DATA + 0x100,
                    ((r.next_u32() as u64) << 32) | r.next_u32() as u64,
                );
            },
        );
    }

    #[test]
    fn tex_man_set_tex1_filter_matches() {
        check(
            ssx3_ee::s12::f_00369130,
            tex_man_set_tex1_filter,
            200,
            &|c, seed| {
                let mut r = Rng(0x102f ^ seed as u64);
                scramble(c, &mut r);
                let index = (seed % 4) as u32;
                let t2 = DATA + 0x200;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = index as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                c.g[T0][0] = r.next_u32() as u64;
                c.g[T1][0] = r.next_u32() as u64;
                c.bus.write32(DATA + index * 4 + 8, t2);
                c.bus.write64(
                    t2 + 0x40,
                    ((r.next_u32() as u64) << 32) | r.next_u32() as u64,
                );
            },
        );
    }

    #[test]
    fn tex_man_set_tex_function_matches() {
        check(
            ssx3_ee::s12::f_003691b0,
            tex_man_set_tex_function,
            200,
            &|c, seed| {
                let mut r = Rng(0x3691_b000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A2][0] = (r.next_u32() as u64) | (r.next_u32() as u64) << 32;
                if seed % 3 == 0 {
                    c.g[A1][0] = 0xffff_ffff_ffff_ffff;
                } else {
                    let idx = (seed % 8) as u32;
                    c.g[A1][0] = idx as u64;
                    c.bus.write32(DATA + idx * 4 + 8, DATA + 0x100);
                    c.bus.write32(DATA + 0x100 + 0x38, r.next_u32());
                    c.bus.write32(DATA + 0x100 + 0x3c, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn ps2_graphics_man_set_shape_banks_matches() {
        check(
            ssx3_ee::s12::f_00369890,
            ps2_graphics_man_set_shape_banks,
            200,
            &|c, seed| {
                let mut r = Rng(0x3698_9000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (r.next_u32() as u64) | (r.next_u32() as u64) << 32;
                c.g[A2][0] = (r.next_u32() as u64) | (r.next_u32() as u64) << 32;
            },
        );
    }

    // ------------------------------------------------------ destructors

    #[test]
    fn graphics_man_dtor_matches() {
        check(
            ee::s12::f_003695d8__generated,
            graphics_man_dtor,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3695_d8, 0);
                pool_allocator(c);
                heap_init(c);
                // A real block: with the low flag bit set the destructor frees it.
                let obj = heap_block(c, 0x1200);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = sx((seed & 3) as u32);
                c.bus.write32(GAME_GP + 0x2a90, r.next_u32());
            },
        );
    }

    #[test]
    fn ps2_graphics_man_set_texture_name_cache_matches() {
        check(
            ssx3_ee::s12::f_00369690__generated,
            ps2_graphics_man_set_texture_name_cache,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3696_90, 0);
                c.g[A0][0] = DATA as u64;
                let count = r.next_u32() % 8;
                c.g[A1][0] = count as u64;
                let src = DATA + 0x4000;
                c.g[A2][0] = src as u64;
                for i in 0..count * 3 {
                    c.bus.write32(src + i * 4, r.next_u32());
                }
                for i in 0..0x100u32 / 4 {
                    c.bus.write32(DATA + 0x280 + i * 4, r.next_u32());
                }
                c.bus.write32(DATA + 0xe80, r.next_u32());
            },
        );
    }

    #[test]
    fn ps2_graphics_man_free_blended_matrix_history_matches() {
        check(
            ssx3_ee::s12::f_003698a0__generated,
            ps2_graphics_man_free_blended_matrix_history,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x3698_a0, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
                let buffer = if seed % 2 == 0 { 0 } else { DATA + 0x1000 };
                w(c, DATA + 0xf48, buffer);
            },
        );
    }

    #[test]
    fn tex_man_free_btex_info_matches() {
        check(
            ee::s12::f_003672c0__generated,
            tex_man_free_btex_info,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3672_c0, 1);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x200);
                c.bus.write32(this, r.next_u32());
                for i in 0..0x10u32 {
                    c.bus.write32(
                        this + 8 + 4 * i,
                        if seed % 3 == 0 {
                            0
                        } else {
                            DATA + 0x1_1000 + 0x10 * i
                        },
                    );
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 0x10) as u64;
            },
        );
    }

    #[test]
    fn tex_man_lock_texture_matches() {
        check(
            ee::s12::f_00367cd0__generated,
            tex_man_lock_texture,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x367c_d0, 0);
                alloc_ready_2(c);
                let this = DATA + 0x100;
                clear(c, this, 0x100);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn tex_man_alloc_btex_info_matches() {
        check(
            ee::s12::f_00367260__generated,
            tex_man_alloc_btex_info,
            SEEDS_4,
            &|c, seed| {
                let mut r = base(c, seed, 0x3672_60);
                let s = DATA + 0x100;
                clear(c, s, 0x40);
                w_2(c, s, r.next_u32());
                c.g[A0][0] = sx(s);
                c.g[A1][0] = (seed % 8) as u64;
            },
        );
    }

    // -------------------------------------------------------------- allocators

    #[test]
    fn tex_man_free_nb_tex_info_matches() {
        check(
            ssx3_ee::s12::f_003671c8__generated,
            tex_man_free_nb_tex_info,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x3671c8);
                c.g[A0][0] = (DATA + 0x800) as u64;
                c.g[A1][0] = (seed % 4) as u64;
                // Slot `id` is null (a plain delete of null) and is pushed onto the
                // free list headed at +0x1f48.
                c.bus.write32(DATA + 0x808 + 4 * (seed % 4), 0);
                c.bus
                    .write32(DATA + 0x800 + 0x1f48, DATA + 0x500 + 8 * (seed % 4));
                c.bus.write32(DATA + 0x800 + 4, r.next_u32());
            },
        );
    }

    /// The TexMan at `DATA+0x10000` embeds its VRAM heap at `+0x1f60`; the
    /// node array pointer is the heap's `+0x1ff0` field (TexMan `+0x3f50`).
    pub(crate) const TEXMAN: u32 = DATA + 0x1_0000;

    pub(crate) const VHEAP: u32 = TEXMAN + 0x1f60;

    pub(crate) const VNODES: u32 = DATA + 0x2_0000;

    pub(crate) const VOWNER: u32 = DATA + 0x2_4000;

    /// A VRAM heap built by the game's own `VramHeap_Init`.
    pub(crate) fn vram_heap(c: &mut Cpu) {
        c.bus.write32(VHEAP + 0x1ff0, VNODES);
        for i in 0..16u32 {
            c.bus.write32(VOWNER + 4 * i, VOWNER + 0x100 + 0x40 * i);
        }
        run_2(
            c,
            0x0036_5e40,
            ssx3_ee::s12::VramHeap_Init,
            &[VHEAP as u64, 0, 0x10_0000, VOWNER as u64],
        );
    }

    #[test]
    fn tex_man_alloc_frame_buffer_matches() {
        check(
            ssx3_ee::s12::f_003673d8__generated,
            tex_man_alloc_frame_buffer,
            80,
            &|c, seed| {
                let _r = setup(c, seed, 0x3673d8);
                vram_heap(c);
                c.g[A0][0] = TEXMAN as u64;
                c.g[A1][0] = (0x100 * (1 + seed % 8)) as u64;
            },
        );
    }

    #[test]
    fn tex_man_end_frame_stub_matches() {
        leaf(
            ssx3_ee::s12::f_003673d0__generated,
            tex_man_end_frame_stub,
            5,
        );
    }

    #[test]
    fn tex_man_shutdown_matches() {
        check(
            ee::s12::f_00367360__generated,
            tex_man_shutdown,
            24,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x3673_60);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x4320, u32::MAX);
                c.bus.write32(DATA + 0x6710, u32::MAX);
                fill(c, &mut r, DATA + 0x1f50, 4);
            },
        );
    }

    #[test]
    fn tex_man_alloc_nb_tex_info_matches() {
        check(
            ee::s12::f_00367150__generated,
            tex_man_alloc_nb_tex_info,
            SEEDS_6,
            &|c, seed| {
                machine(c, seed, 0x367150, 2);
                be_library(c);
                world(c);
                heap_init_2(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let node = DATA + 0x2000;
                w(c, this + 0x1f48, node);
                w(c, node, 0);
                w(c, this + 4, seed % 4);
            },
        );
    }

    #[test]
    fn graphics_man_set_color_mask_matches() {
        check(
            ee::s12::f_00369610__generated,
            graphics_man_set_color_mask,
            12,
            &|c, seed| {
                let _ = base_3(c, seed, 0x3696_10, 4);
                let man = DATA;
                let mask = DATA + 0x1000;
                let vtable = DATA + 0x1200;
                for i in 0..4 {
                    w(c, mask + 4 * i, 0x0101_0101 * (i + 1));
                }
                w(c, man + 0x10d8, DATA + 0x1100);
                w(c, DATA + 0x1100, vtable);
                ret_slot(c, vtable, 0x1f0, RET_ZERO);
                c.g[A0][0] = man as u64;
                c.g[A1][0] = mask as u64;
            },
        );
    }

    #[test]
    fn tex_man_emit_clut_upload_matches() {
        same(
            ssx3_ee::s12::f_003684f0__generated,
            tex_man_emit_clut_upload,
            200,
            &|c, seed| {
                let mut r = Rng(0x3684f0 ^ seed as u64);
                scramble(c, &mut r);
                let mgr = DATA + 0x100;
                let tex = DATA + 0x800;
                c.g[A0][0] = mgr as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.g[A2][0] = (DATA + 0x300) as u64;
                c.g[A3][0] = (DATA + 0x340) as u64;
                c.bus.write32(mgr + 8 + 4 * (seed % 2), tex);
                c.bus
                    .write32(tex + 0xc, if seed % 3 == 0 { 0xc } else { 0x13 });
                c.bus.write32(tex + 0x20, r.next_u32() % 0x1000);
                c.bus.write32(tex + 0x2c, r.next_u32() % 0x1000);
                let dma = DATA + 0x400;
                let gif = DATA + 0x1000;
                c.bus.write32(DATA + 0x300, dma);
                c.bus.write32(DATA + 0x340, gif);
                for i in 0..0x30u32 {
                    c.bus.write8(gif + i, r.next_u32() as u8);
                }
                for i in 0..0x40u32 {
                    c.bus.write8(dma + i, r.next_u32() as u8);
                }
            },
        );
    }

    pub(crate) const UPMGR: u32 = DATA + 0x1_0000;

    pub(crate) const UPMGR_TEX: u32 = DATA + 0x1_2000;

    #[test]
    fn tex_man_bind_textures_matches() {
        check_all(
            ee::s12::f_00368660__generated,
            tex_man_bind_textures,
            20,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x3686_60, 10);
                c.g[A0][0] = sx(UPMGR);
                c.g[A1][0] = sx((seed % 3).wrapping_sub(1)); // -1, 0, 1
                c.g[A2][0] = sx(((seed / 3) % 3).wrapping_sub(1));
                c.g[A3][0] = sx(0);
                // `texs[]`: entries point at_4 small descriptors whose `addr` is
                // negative, so the bind path only allocates heap nodes.
                for i in 0..2u32 {
                    c.bus.write32(UPMGR + 4 * i, UPMGR_TEX + 0x40 * i);
                    c.bus.write32(UPMGR_TEX + 0x40 * i + 0x0, 0xffff_ffff); // addr
                    c.bus
                        .write32(UPMGR_TEX + 0x40 * i + 0x4, r.next_u32() % 0x100);
                    c.bus.write32(UPMGR_TEX + 0x40 * i + 0x8, (seed % 2) as u32); // clut
                    c.bus.write32(UPMGR_TEX + 0x40 * i + 0xc, (seed % 3) as u32);
                    // psm
                }
            },
        );
    }

    use crate::difftest::RA as SENTINEL;

    /// Call a generated function for a fixture (up to six arguments).
    pub(crate) fn oc287_setup_call(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        const BACK: u32 = 0x0000_7770;
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = BACK as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, BACK, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    /// `TexMan_FreeTexture`: a texture manager with both VRAM heaps built
    /// by the game's own `VramHeap_Init` / `VramHeap_Alloc`; textures of
    /// kind 9 or not, with VRAM blocks A / B or none (-1), owned or not,
    /// CPU buffers null or small pool pointers (the test machine's pools
    /// start at 0, so `EA_MemFree` takes the pool path), B ids (< 1500) and
    /// NB ids. The texture record itself is a small pool pointer too (its
    /// info is deleted).
    #[test]
    fn tex_man_free_texture_matches() {
        const MAN: u32 = DATA + 0x1_0000;
        const NODES: u32 = DATA + 0x2_0000;
        const OWNERS: u32 = DATA + 0x3_0000;
        check(
            ee::s12::f_00367bc0__generated,
            tex_man_free_texture,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0036_7bc0, 0);
                let pick = |r: &mut Rng, v: &[u32]| v[(r.next_u32() as usize) % v.len()];
                for i in 0..16 {
                    c.bus.write32(OWNERS + 4 * i, OWNERS + 0x100 + 0x40 * i);
                }
                // The two VRAM heaps, two blocks each.
                let mut blocks = Vec::new();
                for (h, heap) in [(0u32, MAN + TEX_MAN_VRAM), (1, MAN + TEX_MAN_VRAM_KIND9)] {
                    c.bus.write32(heap + 0x1ff0, NODES + 0x8000 * h);
                    oc287_setup_call(
                        c,
                        0x0036_5e40,
                        ee::s12::VramHeap_Init,
                        &[heap as u64, 0, 0x20_0000, OWNERS as u64],
                    );
                    let mut ids = Vec::new();
                    for _ in 0..2 {
                        let size = 0x100 * (1 + r.next_u32() % 16);
                        let owner = r.next_u32() % 7;
                        ids.push(oc287_setup_call(
                            c,
                            0x0036_6ce0,
                            ee::s12::VramHeap_Alloc,
                            &[heap as u64, size as u64, owner as u64, 7, 50, 0],
                        ) as u32);
                    }
                    blocks.push(ids);
                }
                c.g[GP][0] = GAME_GP as u64;
                // The texture (a small pool pointer, deleted by the info free).
                let tex = 0x100 + 0x40 * (r.next_u32() % 8);
                let kind9 = r.next_u32() % 3 == 0;
                c.bus.write32(
                    tex + TEX_KIND,
                    if kind9 {
                        9
                    } else {
                        pick(&mut r, &[0, 1, 8, 10])
                    },
                );
                let heap_a = if kind9 { 1 } else { 0 };
                let has_a = r.next_u32() % 3 != 0;
                c.bus.write32(
                    tex + TEX_SLOT_A,
                    if has_a { r.next_u32() % 4 } else { u32::MAX },
                );
                c.bus.write32(tex + TEX_ID_A, blocks[heap_a][0]);
                let has_b = r.next_u32() % 3 != 0;
                c.bus.write32(
                    tex + TEX_SLOT_B,
                    if has_b { r.next_u32() % 4 } else { u32::MAX },
                );
                c.bus.write32(tex + TEX_ID_B, blocks[0][1]);
                c.bus.write32(tex + TEX_OWNED, pick(&mut r, &[0, 1, 0x100]));
                let buf = |r: &mut Rng| {
                    if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        0x500 + 8 * (r.next_u32() % 0x40)
                    }
                };
                let (a, b) = (buf(&mut r), buf(&mut r));
                c.bus.write32(tex + TEX_BUF_A, a);
                c.bus.write32(tex + TEX_BUF_B, b);
                // The manager: the id's texture, counts, NB free-list head.
                let id = pick(&mut r, &[3, 100, 1499, 1500, 1501, 1700]);
                c.bus.write32(MAN + TEX_MAN_TEXTURES + 4 * id, tex);
                c.bus.write32(MAN, 1 + r.next_u32() % 50);
                c.bus.write32(MAN + 4, 1 + r.next_u32() % 50);
                c.bus
                    .write32(MAN + 0x1f48, MAN + 8 + 4 * (r.next_u32() % 1400));
                for off in [0x1f4cu32, 0x1f50, 0x1f54] {
                    c.bus.write32(MAN + off, r.next_u32());
                }
                c.g[A0][0] = sx(MAN);
                c.g[A1][0] = id as u64;
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x0036_7bc0;
            },
        );
    }

    /// Event delivery at the reset loop's head (0x367c90): a texture with
    /// nothing to free in VRAM or CPU memory.
    #[test]
    fn tex_man_free_texture_events_match() {
        const MAN: u32 = DATA + 0x1_0000;
        check_events(
            ee::s12::f_00367bc0__generated,
            tex_man_free_texture,
            90,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0036_7bc1, 0);
                let tex = 0x100 + 0x40 * (r.next_u32() % 8);
                c.bus.write32(tex + TEX_SLOT_A, u32::MAX);
                c.bus.write32(tex + TEX_SLOT_B, u32::MAX);
                c.bus.write32(tex + TEX_OWNED, 0);
                let id = if seed % 2 == 0 { 7 } else { 1600 };
                c.bus.write32(MAN + TEX_MAN_TEXTURES + 4 * id, tex);
                c.bus.write32(MAN + 0x1f48, MAN + 0x100);
                c.g[A0][0] = sx(MAN);
                c.g[A1][0] = id as u64;
                c.g[RA][0] = SENTINEL as u64;
                let mode = schedule(c, &mut r, seed, 0x0036_7bc0);
                mode
            },
        );
    }

    use crate::difftest::SP as STACK_TOP;

    /// Copy the loader's four jump tables (`0x492090..0x4922b4`) from the
    /// game image: the generated body jumps through them.
    pub(crate) fn tex_load_texture_tables(c: &mut Cpu) {
        static TABLES: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
        let bytes = TABLES.get_or_init(|| {
            let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../orig/SLUS_207.72");
            let d = std::fs::read(path).expect("orig/SLUS_207.72");
            let u32_at = |o: usize| u32::from_le_bytes(d[o..o + 4].try_into().unwrap());
            let ph = u32_at(0x1c) as usize;
            let (off, vaddr) = (u32_at(ph + 4) as usize, u32_at(ph + 8));
            let at = off + (TEX_LOAD_FMT_TABLE - vaddr) as usize;
            d[at..at + (TEX_LOAD_PAGE_TABLE2 + 29 * 4 - TEX_LOAD_FMT_TABLE) as usize].to_vec()
        });
        let at = TEX_LOAD_FMT_TABLE as usize;
        c.bus.ram[at..at + bytes.len()].copy_from_slice(bytes);
    }

    /// `TexMan_LoadTexture`: fresh and fixed slots, every format (and one
    /// past the table), with and without pixels, 0..3 mip levels (and none),
    /// owned and referenced data, every CLUT format with and without a CLUT,
    /// VRAM modes 0..3, and the PSMT8 swizzle budget on both sides. Real
    /// callees: the slot allocators, strncpy, operator new over the game
    /// heap, EA_MemCopy, VramHeap_Alloc over a heap built by VramHeap_Init
    /// and TexMan_CopyToTempTex.
    #[test]
    fn tex_man_load_texture_matches() {
        use crate::game::fe::feasyncfile::tests::{be_library_5, heap_init_5, world_3};
        use crate::game::render::ps2graphicsman::tests::{vram_heap, TEXMAN};
        check_lohi(
            ee::s12::f_00367440__generated,
            tex_man_load_texture,
            240,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x0036_7440, 0);
                tex_load_texture_tables(c);
                be_library_5(c);
                world_3(c);
                heap_init_5(c);
                vram_heap(c);
                let man = TEXMAN;
                let pixels = DATA + 0x20_0000;
                let clut = DATA + 0x21_0000;
                let name = DATA + 0x22_0000;
                for (i, b) in b"snowtex\0".iter().enumerate() {
                    c.bus.write8(name + i as u32, *b);
                }
                for i in 0..0x40 {
                    c.bus.write32(pixels + 4 * i, r.next_u32());
                    c.bus.write32(clut + 4 * i, r.next_u32());
                }
                // A free slot for the non-bound allocator, slot counters.
                let k = 1 + seed % 16;
                let node = man + tex_load_man::SLOTS + 4 * k;
                c.bus.write32(man + 0x1f48, node);
                c.bus.write32(node, 0);
                c.bus.write32(man, seed % 5);
                c.bus.write32(man + 4, seed % 7);
                c.bus.write32(man + tex_load_man::ALLOC_FLAGS, 0);
                let limit = match seed % 3 {
                    0 => 0,
                    1 => 0x7fff_ffff,
                    _ => r.next_u32() % 0x8000,
                };
                c.bus.write32(man + tex_load_man::T8_LIMIT, limit);

                let fmt: u64 = match (seed / 2) % 10 {
                    0 => 0,
                    1 => 2,
                    2 => 8,
                    3 | 4 => 9,
                    5 => 10,
                    6 => 11,
                    7 => 12,
                    8 => 13,
                    _ => r.next_u32() as u64,
                };
                let has_pixels = seed % 5 != 4;
                let mips: i32 = match (seed / 3) % 5 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => -1,
                    _ => (r.next_u32() % 3) as i32,
                };
                let sizes: &[u32] = if fmt == 9 {
                    &[16, 32, 64]
                } else {
                    &[8, 16, 32, 64]
                };
                let w = sizes[(r.next_u32() as usize) % sizes.len()];
                let h = sizes[(r.next_u32() as usize) % sizes.len()];
                let clut_fmt: u64 = match (seed / 7) % 5 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    _ => u64::MAX,
                };
                let mode = if mips < 0 {
                    2 * (seed % 2)
                } else {
                    (seed / 11) % 4
                };
                let owned = (seed / 13) % 2;
                let id: u32 = if seed % 4 == 0 {
                    0x20 + seed % 8
                } else {
                    u32::MAX
                };
                let has_clut = (seed / 17) % 3 != 0;

                c.g[A0][0] = sx(man);
                c.g[A1][0] = if has_pixels { sx(pixels) } else { 0 };
                c.g[A2][0] = sx(name);
                c.g[A3][0] = sx(w);
                c.g[T0][0] = sx(h);
                c.g[T1][0] = sx(mips as u32);
                c.g[T2][0] = fmt;
                c.g[T3][0] = clut_fmt;
                c.bus
                    .write64(STACK_TOP, if has_clut { clut as u64 } else { 0 });
                c.bus.write64(STACK_TOP + 0x8, mode as u64);
                c.bus.write64(STACK_TOP + 0x10, owned as u64);
                c.bus.write64(STACK_TOP + 0x18, sx(id));
                c.g[SP][0] = sx(STACK_TOP);
                c.g[RA][0] = sx(crate::difftest::RA);
                c.pc = 0x0036_7440;
            },
        );
    }

    use crate::game::dirtysock::buddyapi::tests::require_arm_values;

    pub(crate) use crate::game::render::particle::tests::PS2GM_RET;
    pub(crate) use crate::game::render::particle::tests::PS2GM_STACK;

    pub(crate) use crate::game::render::particle::tests::GTN_MAN;

    pub(crate) use crate::game::render::particle::tests::GTN_BEFORE;

    pub(crate) use crate::game::render::particle::tests::get_texture_by_name_setup;
    pub(crate) use crate::game::render::particle::tests::GTN_NAMES;

    /// GetTextureByName against the generated body: every key length, cache
    /// hits and misses, bank hits and misses (fake unwinds included).
    #[test]
    fn ps2_graphics_man_get_texture_by_name_matches_gfx1() {
        let setup = |c: &mut Cpu, seed: u32| {
            get_texture_by_name_setup(c, seed);
        };
        check_lohi(
            ee::s12::f_003696c8__generated,
            crate::game::render::ps2graphicsman::ps2_graphics_man_get_texture_by_name,
            216,
            &setup,
        );
        // The key copy: empty, ended by the NUL, cut at four characters.
        require_arm_values(
            ee::s12::f_003696c8__generated,
            216,
            &setup,
            &|c, seed| {
                let buf = PS2GM_STACK - 0x80;
                let len = (0..5).take_while(|&i| c.bus.read8(buf + i) != 0).count();
                match len {
                    0 => 0,
                    1..=3 => 1,
                    4 if GTN_NAMES[(seed % 4) as usize].len() == 4 => 2,
                    4 => 3,
                    _ => 9,
                }
            },
            &[0, 1, 2, 3],
        );
        // The outcome (0 cached, 1 loaded from a bank, 2 missing) with
        // whether the cache and the banks were searched.
        require_arm_values(
            ee::s12::f_003696c8__generated,
            216,
            &setup,
            &|c, _| {
                if c.pc != PS2GM_RET {
                    return (9, false, false);
                }
                let before = c.bus.read32(GTN_BEFORE);
                let banks = c.bus.read32(GTN_BEFORE + 4);
                let after = c.bus.read32(GTN_MAN + ps2gm::TEX_CACHE_COUNT);
                let outcome = if after == before {
                    0
                } else if c.g[V0][0] == u64::MAX {
                    2
                } else {
                    1
                };
                (outcome, before > 0, outcome != 0 && banks > 0)
            },
            &[
                (0, true, false),
                (1, false, true),
                (1, true, true),
                (2, false, false),
                (2, false, true),
                (2, true, false),
                (2, true, true),
            ],
        );
        // The returns and unwinds (`pc`, the copy flag at `GTN_UNWIND + 4`):
        // the renderer's texture load (fake call 0), strcmp before and after
        // the key was copied in, strcpy.
        require_arm_values(
            ee::s12::f_003696c8__generated,
            216,
            &setup,
            &|c, _| match c.pc {
                PS2GM_RET => (0, 0),
                pc => (pc, c.bus.read32(GTN_UNWIND + 4)),
            },
            &[
                (0, 0),
                (0xdead_0000, 1),
                (0xdead_0100, 0),
                (0xdead_0100, 1),
                (0xdead_0200, 1),
            ],
        );
    }

    use crate::difftest::events::driven;
    use crate::difftest::events::Events;
    use crate::difftest::fakes;
    use crate::game::render::particle::tests::ps2gm_deliver_only;
    use crate::game::render::particle::tests::ps2gm_put_str;
    use crate::game::render::particle::tests::GTN_BANKS;
    use crate::game::render::particle::tests::GTN_BANK_LIST;
    use crate::game::render::particle::tests::GTN_NAME;
    use crate::game::render::particle::tests::GTN_UNWIND;

    driven!(gen_get_texture_by_name, ee::s12::f_003696c8__generated);

    driven!(
        hand_get_texture_by_name_gfx1,
        crate::game::render::ps2graphicsman::ps2_graphics_man_get_texture_by_name
    );

    /// The bank loop's event fixture: key `"sky"`, an empty cache, and three
    /// banks of one entry whose shape's long name is empty (misses that make
    /// no event check: `strcmp` returns at the empty string's first byte); the
    /// event falls due when `strcpy` returns, so it lands on the bank loop's
    /// back edge.
    pub(crate) fn gtn_bank_head_fixture(c: &mut Cpu) {
        ps2gm_put_str(c, GTN_NAME, b"sky");
        c.bus.write32(GTN_MAN + ps2gm::TEX_CACHE_COUNT, 0);
        c.bus.write32(GTN_MAN + ps2gm::SHAPE_BANK_COUNT, 3);
        for k in 0..3 {
            let bank = GTN_BANKS + 0x1000 * k;
            c.bus.write32(GTN_BANK_LIST + 4 * k, bank);
            c.bus.write32(bank + 8, 1);
            c.bus.write32(bank + 0x14, 0x400);
            c.bus.write32(bank + 0x400, 0x70);
            ps2gm_put_str(c, bank + 0x404, b"");
        }
        c.bus.write32(GTN_UNWIND + 8, 1);
    }

    /// GetTextureByName with events at the copy, cache and bank loops' back
    /// edges (structured loops: delivered, never unwinding there): `seed / 3
    /// % 2` aims the event at the bank loop (0x3697b0).
    #[test]
    fn ps2_graphics_man_get_texture_by_name_events_match_gfx1() {
        check_events(
            gen_get_texture_by_name,
            hand_get_texture_by_name_gfx1,
            300,
            &|c, seed| {
                let mut r = get_texture_by_name_setup(c, seed / 6);
                c.bus.write32(fakes::LOG_UNWIND_AT, u32::MAX);
                c.bus.write32(GTN_UNWIND, 0);
                let mode = schedule(c, &mut r, seed, 0x0036_96c8);
                if seed / 3 % 2 == 1 && mode != Events::Off {
                    gtn_bank_head_fixture(c);
                }
                ps2gm_deliver_only(c, mode)
            },
        );
    }

    pub(crate) use crate::game::bx::memblockman::tests::oc689_put_cstr;
    pub(crate) use crate::game::bx::memblockman::tests::OC689_GFX;
    pub(crate) use crate::game::bx::memblockman::tests::OC689_SEEDS;

    pub(crate) const OC689_SRCS: u32 = DATA + 0x0003_0000;

    pub(crate) const OC689_VTABLE: u32 = DATA + 0x0004_0000;

    /// The graphics manager's texture cache (`+0x280`, 12-byte entries,
    /// count `+0xe80`) and shape banks (`+0x27c`, count `+0x278`): a bank
    /// holds `count` shapes whose names the real `SHAPE_locatez` compares;
    /// the texture is created through vtable slot 46 (a fake).
    pub(crate) fn oc689_texture_setup(c: &mut Cpu, seed: u32, long: bool) -> Rng {
        let mut r = fakes::machine(c, seed, 0x0689_0008, 3);
        c.pc = 0x0036_96c8;
        let names: [&[u8]; 5] = [b"abc", b"abcdefgh", b"wxyz", b"", b"q"];
        let name = names[(seed % 5) as usize];
        let want: Vec<u8> = name.iter().take(4).copied().collect();
        c.g[A0][0] = OC689_GFX as u64;
        c.g[A1][0] = (DATA + 0x1000) as u64;
        oc689_put_cstr(c, DATA + 0x1000, name);
        // Cache.
        let count = if long {
            30 + r.next_u32() % 8
        } else {
            r.next_u32() % 4
        };
        c.bus.write32(OC689_GFX + 0xe80, count);
        for i in 0..count {
            let e = OC689_GFX + 0x280 + 12 * i;
            c.bus.write32(e, r.next_u32());
            let pool: [&[u8]; 4] = [b"zzzz", b"q", b"wxyz", b"abc"];
            let other = pool[(r.next_u32() % 4) as usize];
            if !long && i + 1 == count && r.next_u32() % 3 == 0 {
                oc689_put_cstr(c, e + 4, &want);
            } else if long {
                oc689_put_cstr(c, e + 4, b"zzzz");
            } else {
                oc689_put_cstr(c, e + 4, other);
            }
        }
        // Shape banks.
        let banks = r.next_u32() % 3;
        c.bus.write32(OC689_GFX + 0x278, banks);
        c.bus.write32(OC689_GFX + 0x27c, OC689_SRCS);
        for j in 0..banks {
            let src = OC689_SRCS + 0x1000 * (j + 1);
            c.bus.write32(OC689_SRCS + 4 * j, src);
            let shapes = r.next_u32() % 3;
            c.bus.write32(src + 8, shapes);
            for k in 0..shapes {
                let off = 0x100 + 0x20 * k;
                c.bus.write32(src + 0x14 + 8 * k, off);
                c.bus.write32(src + off, 0x70);
                if r.next_u32() % 2 == 0 {
                    oc689_put_cstr(c, src + off + 4, &want);
                } else {
                    oc689_put_cstr(c, src + off + 4, b"nope");
                }
            }
        }
        // Vtable slot 46: `{delta, fn}` at +0x170, a fake.
        c.bus.write32(OC689_GFX + 0x10d8, OC689_VTABLE);
        c.bus
            .write16(OC689_VTABLE + 0x170, (r.next_u32() % 0x40 * 4) as u16);
        c.bus.write32(OC689_VTABLE + 0x174, fakes::FAKE);
        r
    }

    /// `PS2GraphicsMan_GetTextureByName`: cache hit, shape found (texture
    /// created by the vtable fake) and not found (-1), names of 0 to 8 characters.
    #[test]
    fn ps2_graphics_man_get_texture_by_name_matches() {
        check(
            ee::s12::f_003696c8__generated,
            ps2_graphics_man_get_texture_by_name,
            OC689_SEEDS,
            &|c, seed| {
                oc689_texture_setup(c, seed, false);
            },
        );
        check_events(
            ee::s12::f_003696c8__generated,
            ps2_graphics_man_get_texture_by_name,
            OC689_SEEDS,
            &|c, seed| {
                let mut r = oc689_texture_setup(c, seed, true);
                schedule(c, &mut r, seed, 0x0036_96c8)
            },
        );
    }

    pub(crate) use crate::game::fe::debugfe::tests::oc798_arm;

    pub(crate) use crate::game::fe::debugfe::tests::OC798_OBJ;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::oc770_machine;

    /// `GraphicsMan_ctor`: fake `memset`/`memcpy`, unwinds at every call.
    #[test]
    fn graphics_man_ctor_matches() {
        const THIS: u32 = DATA + 0x3_0000;
        check(
            ee::s12::f_003691f8__generated,
            graphics_man_ctor,
            200,
            &|c, seed| {
                let _r = oc770_machine(c, seed, 0x7706, 4);
                c.pc = 0x0036_91f8;
                c.g[A0][0] = THIS as u64;
            },
        );
    }

    /// `TexMan_InitTempHeap`: forwards to `VramHeap_Init`, including unwinds.
    #[test]
    fn tex_man_init_temp_heap_matches() {
        oc798_install_fakes();
        fakes::covered(
            ee::s12::f_00367340__generated,
            tex_man_init_temp_heap,
            60,
            &|c, seed| {
                machine(c, seed, 0x798b, 1);
                oc798_arm(c);
                c.pc = 0x0036_7340;
                c.g[RA][0] = sx(crate::difftest::RA);
                c.g[A0][0] = sx(OC798_OBJ + 0x10 * (seed & 3));
            },
        );
    }

    use crate::difftest;

    pub(crate) use crate::game::visualfx::boardwakefx::tests::oc1062_upload_setup;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// `TexMan_EmitTex0` against the generated body. Random texture infos
    /// (pixel format 0..40 so the jump table's default arm is hit as well,
    /// random TEX0 size bits, 0..6 mip levels, CLUT address negative or not,
    /// format 9 or not) bound to slot 0/1, with the slot already holding the
    /// texture (CLUT current or not) or not. The page-size jump tables come
    /// from the game image.
    #[test]
    fn texman_emit_tex0_matches() {
        difftest::check(
            ee::s12::f_00368970__generated,
            texman_emit_tex0,
            600,
            &|c, seed| {
                let mut r = Rng(0x368970 + seed as u64 * 7919);
                difftest::elf_image(c);
                let this = DATA;
                let dma_ptr = DATA + 0x1000;
                let packet = DATA + 0x4000;
                let idx = r.next_u32() % 8;
                let slot = r.next_u32() & 1;
                for i in 0..10u32 {
                    let tex = DATA + 0x2000 + i * 0x80;
                    c.bus.write32(this + 8 + 4 * i, tex);
                    let psm = if seed % 5 == 0 {
                        r.next_u32() % 40
                    } else {
                        4 + r.next_u32() % 29
                    };
                    c.bus.write32(
                        tex + 0xc,
                        if r.next_u32() & 1 == 0 {
                            9
                        } else {
                            r.next_u32() % 16
                        },
                    );
                    c.bus.write32(tex + 0x18, psm);
                    c.bus.write32(tex + 0x24, r.next_u32() % 7);
                    c.bus.write32(tex + 0x28, r.next_u32() & 0x3fff);
                    let clut = if r.next_u32() & 1 == 0 {
                        r.next_u32() & 0x3fff
                    } else {
                        0xffff_fff0
                    };
                    c.bus.write32(tex + 0x2c, clut);
                    let tex0 = ((r.next_u32() as u64) << 32 | r.next_u32() as u64)
                        & 0xffff_ffff_0000_0000
                        | ((r.next_u32() % 11) as u64) << 26
                        | ((r.next_u32() % 11) as u64) << 30;
                    c.bus.write64(tex + 0x38, tex0);
                    c.bus.write64(
                        tex + 0x40,
                        (r.next_u32() as u64) << 32 | r.next_u32() as u64,
                    );
                    c.bus.write64(
                        tex + 0x48,
                        (r.next_u32() as u64) << 32 | r.next_u32() as u64,
                    );
                    c.bus.write64(
                        tex + 0x50,
                        (r.next_u32() as u64) << 32 | r.next_u32() as u64,
                    );
                }
                let bound = match seed % 4 {
                    0 | 1 => idx,
                    _ => idx + 1,
                };
                c.bus.write32(this + TEXMAN_BOUND + 4 * slot, bound);
                c.bus
                    .write32(this + TEXMAN_BOUND + 4 * (slot ^ 1), r.next_u32() % 8);
                c.bus.write32(
                    this + TEXMAN_CLUT_TEX,
                    if seed % 8 == 1 || seed % 8 == 4 {
                        idx
                    } else {
                        idx + 2
                    },
                );
                c.bus.write32(dma_ptr, packet);
                c.g[SP][0] = sx(difftest::SP);
                c.g[GP][0] = sx(difftest::GAME_GP);
                c.g[RA][0] = sx(difftest::RA);
                c.pc = 0x0036_8970;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = idx as u64;
                c.g[A2][0] = slot as u64;
                c.g[A3][0] = sx(dma_ptr);
            },
        );
    }

    #[test]
    fn tex_man_emit_image_upload_matches() {
        check_events(
            ssx3_ee::s12::f_00368170__generated,
            tex_man_emit_image_upload,
            240,
            &oc1062_upload_setup,
        );
    }

    #[test]
    fn tex_man_destroy_nb_tex_info_array_matches() {
        check(
            ee::s12::f_00367230__generated,
            tex_man_destroy_nb_tex_info_array,
            20,
            &|c, seed| {
                let mut r = Rng(0x3672 ^ seed as u64);
                scramble(c, &mut r);
                // Events at a few points of the countdown (seed 0: at every back edge).
                c.next_event = (seed as u64 % 8) * 700;
            },
        );
    }

    /// Gate word of this module's callee fakes, in the machine's RAM.
    pub(crate) const OC1071_GATE: u32 = DATA + 0x0_f000;

    /// Written at `OC1071_GATE` by the fixtures that want the fakes.
    pub(crate) const OC1071_MAGIC: u32 = 0x1071_00de;

    /// A callee fake that answers only with the gate open, and otherwise runs
    /// the body it replaced.
    macro_rules! gated_fake {
        ($fake:ident, $prev:ident, $target:expr, $generated:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC1071_GATE) != OC1071_MAGIC {
                    return fakes::fall_back(c, &$prev, $generated);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $target, ra);
            }
        };
    }

    gated_fake!(
        oc1071_fake_3e6574,
        OC1071_PREV_3E6574,
        0x003e_6574,
        ee::s14::f_003e6574__generated
    );

    gated_fake!(
        oc1071_fake_366e30,
        OC1071_PREV_366E30,
        0x0036_6e30,
        ee::s12::f_00366e30__generated
    );

    gated_fake!(
        oc1071_fake_384dc0,
        OC1071_PREV_384DC0,
        0x0038_4dc0,
        ee::s12::f_00384dc0__generated
    );

    gated_fake!(
        oc1071_fake_384fd0,
        OC1071_PREV_384FD0,
        0x0038_4fd0,
        ee::s12::f_00384fd0__generated
    );

    gated_fake!(
        oc1071_fake_317e98,
        OC1071_PREV_317E98,
        0x0031_7e98,
        ee::s10::f_00317e98__generated
    );

    /// Install the fakes above (once per process; each one is gated).
    pub(crate) fn oc1071_hook() {
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E6574),
            oc1071_fake_3e6574,
            &OC1071_PREV_3E6574,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_00366E30),
            oc1071_fake_366e30,
            &OC1071_PREV_366E30,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_00384DC0),
            oc1071_fake_384dc0,
            &OC1071_PREV_384DC0,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_00384FD0),
            oc1071_fake_384fd0,
            &OC1071_PREV_384FD0,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc1071_fake_317e98,
            &OC1071_PREV_317E98,
        );
    }

    driven!(gen_destroy_btexinfo, ee::s12::f_00367310__generated);

    driven!(hand_destroy_btexinfo, texman_destroy_btexinfo_array);

    #[test]
    fn texman_destroy_btexinfo_array_matches() {
        check_events(
            gen_destroy_btexinfo,
            hand_destroy_btexinfo,
            64,
            &|c, seed| {
                let mut r = Rng(0x0036_7310 ^ seed as u64);
                scramble(c, &mut r);
                let mode = schedule(c, &mut r, seed, 0x0036_7310);
                // The loop is long: make the first back edges due, identically for both.
                c.next_event = c.cycles + 4 + (seed as u64 % 997);
                mode
            },
        );
    }

    #[test]
    fn texman_update_texture_data_matches() {
        oc1071_hook();
        check(
            ee::s12::f_00367d20__generated,
            texman_update_texture_data,
            200,
            &|c, seed| {
                let mut r = Rng(0x0036_7d20 ^ seed as u64);
                scramble(c, &mut r);
                c.bus.write32(OC1071_GATE, OC1071_MAGIC);
                let gm = DATA + 0x1000;
                let idx = seed % 4;
                c.g[A0][0] = sx(gm);
                c.g[A1][0] = idx as u64;
                // a2 zero or not: the first table call.
                c.g[A2][0] = if seed % 3 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                let rec = DATA + 0x2000;
                c.bus.write32(gm + idx * 4 + 8, rec);
                c.bus.write32(rec + 0x10, DATA + 0x3000);
                c.bus.write32(rec + 0x18, r.next_u32() % 32);
                c.bus.write32(rec + 0x1c, DATA + 0x3100);
                c.bus.write32(rec + 0x20, DATA + 0x3200);
                // Evict's record state: -1 (nothing freed), tag 9 or other.
                c.bus.write32(
                    rec + 0x28,
                    if seed % 4 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32()
                    },
                );
                c.bus
                    .write32(rec + 0xc, if seed % 2 == 0 { 9 } else { r.next_u32() % 8 });
                // a8 (sp+0) zero or not, a9 (sp+8) zero or not.
                let a8 = if seed % 2 == 0 { 0 } else { r.next_u32() };
                let a9 = if seed % 5 == 0 { 0 } else { r.next_u32() };
                c.bus.write32(crate::difftest::SP, a8);
                c.bus.write32(crate::difftest::SP + 8, a9);
            },
        );
    }

    #[test]
    fn texman_evict_texture_matches() {
        oc1071_hook();
        check(
            ee::s12::f_00369098__generated,
            texman_evict_texture,
            200,
            &|c, seed| {
                let mut r = Rng(0x0036_9098 ^ seed as u64);
                scramble(c, &mut r);
                c.bus.write32(OC1071_GATE, OC1071_MAGIC);
                let gm = DATA + 0x1000;
                let idx = seed % 4;
                c.g[A0][0] = sx(gm);
                c.g[A1][0] = idx as u64;
                let rec = DATA + 0x2000;
                c.bus.write32(gm + idx * 4 + 8, rec);
                // seed % 3: 0 nothing freed, 1 tag 9 (the second list), 2 the first list.
                let state = if seed % 3 == 0 {
                    u32::MAX
                } else {
                    r.next_u32()
                };
                c.bus.write32(rec + 0x28, state);
                c.bus
                    .write32(rec + 0xc, if seed % 3 == 1 { 9 } else { r.next_u32() % 8 });
            },
        );
    }
}
