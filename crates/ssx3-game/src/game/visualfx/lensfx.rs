//! `src/visualfx/lensfx.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::rider::gp_at;
pub(crate) use crate::game::be::belibrary::char_bolts_pointer;
pub(crate) use crate::game::be::belibrary::BOLT_RECORDS;
pub(crate) use crate::game::be::belibrary::BOLT_RECORDS_OFF;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_2;
pub(crate) use crate::game::fe::feasyncfile::gp_16 as gp_3;
pub(crate) use crate::game::fe::feasyncfile::lh;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::ovstatehud::ld_d;
pub(crate) use crate::game::fe::ovstatehud::sd_d;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::render::particle::add32;
pub(crate) use crate::game::render::particle::at;
pub(crate) use crate::game::render::particle::fconst;
pub(crate) use crate::game::render::particle::fr;
pub(crate) use crate::game::render::particle::gp;
pub(crate) use crate::game::render::particle::ldf;
pub(crate) use crate::game::render::particle::le;
pub(crate) use crate::game::render::particle::lt;
pub(crate) use crate::game::render::particle::p;
pub(crate) use crate::game::render::particle::setf;
pub(crate) use crate::game::render::particle::shl32;
pub(crate) use crate::game::render::particle::srl32;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::forward;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::gp_2;
use crate::game::visualfx::boardwakefx as bw;
pub(crate) use crate::game::visualfx::boardwakefx::LENS_FX_SLOTS;
pub(crate) use crate::game::visualfx::renderstateman::weather_slot_float_getter;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0014_ad28,
        be_get_char_profile_data,
        "BE_GetCharProfileData",
    ),
    (0x0014_bdb8, bolts_get_db, "Bolts_GetDB"),
    (0x0014_d998, bolts_get_char_bolts, "Bolts_GetCharBolts"),
    (
        0x002e_c9e0,
        c_bolt_on_fx_interface_ctor,
        "cBoltOnFXInterface_ctor",
    ),
    (
        0x002e_ca68,
        c_bolt_on_fx_interface_ctor_2,
        "cBoltOnFXInterface_ctor_2",
    ),
    (
        0x002e_caf0,
        c_bolt_on_fx_interface_clear_emitters,
        "cBoltOnFXInterface_ClearEmitters",
    ),
    (
        0x002e_cb28,
        bolt_on_fx_interface_dtor,
        "cBoltOnFXInterface_dtor",
    ),
    (
        0x002e_cbc0,
        c_bolt_on_fx_interface_find_or_create_emitter,
        "cBoltOnFXInterface_FindOrCreateEmitter",
    ),
    (
        0x002e_cc28,
        c_bolt_on_fx_interface_create_emitter,
        "cBoltOnFXInterface_CreateEmitter",
    ),
    (
        0x002e_cf78,
        c_bolton_fx_interface_update,
        "cBoltOnFXInterface_Update",
    ),
    (
        0x002e_cfa0,
        world_painter_free_view_queries,
        "WorldPainter_FreeViewQueries",
    ),
    (
        0x002e_d338,
        world_painter_lerp_view_colours,
        "WorldPainter_LerpViewColours",
    ),
    (
        0x002e_d490,
        world_painter_update_view,
        "WorldPainter_UpdateView",
    ),
    (
        0x002e_dfb8,
        world_painter_get_view_irradiance,
        "WorldPainter_GetViewIrradiance",
    ),
    (0x002e_dfd0, irradiance_ref_resolve, "IrradianceRef_Resolve"),
    (
        0x002e_e268,
        world_painter_sun_flare_intensity,
        "WorldPainter_GetSunFlareIntensity",
    ),
    (
        0x002e_e2f8,
        irradiance_get_override,
        "Irradiance_GetOverride",
    ),
    (
        0x002e_e318,
        world_painter_get_irradiance0,
        "WorldPainter_GetIrradiance0",
    ),
    (
        0x002e_e340,
        world_painter_get_irradiance1,
        "WorldPainter_GetIrradiance1",
    ),
    (
        0x002e_e368,
        world_painter_get_irradiance2,
        "WorldPainter_GetIrradiance2",
    ),
    (
        0x002e_e6f0,
        world_painter_get_weather30,
        "WorldPainter_GetWeather30",
    ),
    (
        0x002e_e070,
        world_painter_get_sun_elevation,
        "WorldPainter_GetSunElevation",
    ),
    (
        0x002e_e0b8,
        world_painter_get_sun_azimuth,
        "WorldPainter_GetSunAzimuth",
    ),
    (
        0x002e_e100,
        world_painter_get_sun_red,
        "WorldPainter_GetSunRed",
    ),
    (
        0x002e_e148,
        world_painter_get_sun_green,
        "WorldPainter_GetSunGreen",
    ),
    (
        0x002e_e190,
        world_painter_get_sun_blue,
        "WorldPainter_GetSunBlue",
    ),
    (
        0x002e_e1d8,
        world_painter_get_sun_intensity,
        "WorldPainter_GetSunIntensity",
    ),
    (
        0x002e_e2b0,
        world_painter_get_sun_size,
        "WorldPainter_GetSunSize",
    ),
    (
        0x002e_db20,
        world_painter_sample_surface_colours,
        "WorldPainter_SampleSurfaceColours",
    ),
    (
        0x002e_df00,
        world_painter_compute_colour_factor,
        "WorldPainter_ComputeColourFactor",
    ),
    (
        0x002e_e010,
        irradiance_ref_get_entry,
        "IrradianceRef_GetEntry",
    ),
    (
        0x002e_e448,
        world_painter_get_weather08,
        "WorldPainter_GetWeather08",
    ),
    (
        0x002e_e4c0,
        world_painter_get_weather10,
        "WorldPainter_GetWeather10",
    ),
    (
        0x002e_e508,
        world_painter_get_weather18,
        "WorldPainter_GetWeather18",
    ),
    (
        0x002e_e5b8,
        world_painter_get_weather38,
        "WorldPainter_GetWeather38",
    ),
    (
        0x002e_e600,
        world_painter_get_weather48,
        "WorldPainter_GetWeather48",
    ),
    (
        0x002e_e660,
        world_painter_get_weather50,
        "WorldPainter_GetWeather50",
    ),
    (
        0x002e_e6a8,
        world_painter_get_weather40,
        "WorldPainter_GetWeather40",
    ),
    (
        0x002e_e738,
        world_painter_get_weather58,
        "WorldPainter_GetWeather58",
    ),
    (
        0x002e_e780,
        world_painter_get_weather60,
        "WorldPainter_GetWeather60",
    ),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {

    pub(crate) use crate::game::be::beintnetwork::callee::C_BOLTON_FX_REFRESH_EMITTERS;

    pub(crate) use crate::game::be::beintnetwork::callee::IRRADIANCE_REF_GET_ENTRY;

    pub(crate) use crate::game::be::beintnetwork::callee::WORLD_PAINTER_GET_IRRADIANCE_NAME0;
    pub(crate) use crate::game::be::beintnetwork::callee::WORLD_PAINTER_GET_IRRADIANCE_NAME1;

    pub(crate) use crate::game::be::beintoption::callee::WORLD_PAINTER_GET_IRRADIANCE_NAME2;

    pub(crate) use crate::game::fe::feasyncfile::callee::ATTACHED_EMITTER_FX_DESTROY_EMITTER;

    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_ON_FX_CLEAR_EMITTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_ON_FX_INIT_HANDLE;

    pub(crate) use crate::game::fe::feasyncfile::callee::BUILTIN_DELETE_6 as BUILTIN_DELETE_2;

    pub(crate) use crate::game::render::particle::callee::BLEND_VIEW_COLOURS;
    pub(crate) use crate::game::render::particle::callee::COMPUTE_COLOUR_FACTOR;

    pub(crate) use crate::game::render::particle::callee::GET_IRRADIANCE0;
    pub(crate) use crate::game::render::particle::callee::GET_IRRADIANCE1;
    pub(crate) use crate::game::render::particle::callee::GET_IRRADIANCE2;
    pub(crate) use crate::game::render::particle::callee::GET_LIGHTING28;

    pub(crate) use crate::game::render::particle::callee::IRRADIANCE_ADD;
    pub(crate) use crate::game::render::particle::callee::IRRADIANCE_SCALE;
    pub(crate) use crate::game::render::particle::callee::IRRADIANCE_SCALE_IN_PLACE;

    pub(crate) use crate::game::render::particle::callee::LERP_VIEW_COLOURS;

    pub(crate) use crate::game::render::particle::callee::RIDER_GET_MAIN_STATE;
    pub(crate) use crate::game::render::particle::callee::SAMPLE_SURFACE_COLOURS;
    pub(crate) use crate::game::render::particle::callee::SECTION_GET_EVENT_WORLD_SECTION;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ATTACHED_DYNAMIC_EMITTER_CREATE;

    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::IRRADIANCE_REF_GET_ENTRY as IRRADIANCE_REF_GET_ENTRY_2;
}

// ---------------------------------------------------------------------------
// World streamer.

/// Streamer view records: 0x50 bytes each at `wc + 0x250`.
pub(crate) mod view {}

// ---------------------------------------------------------------- world painter

/// `ViewIrradiance *WorldPainter_GetViewIrradiance(int view)` (0x2edfb8):
/// `&TABLE[view]`, entries of 0xf0 bytes starting at 0x4fa3c0. The index
/// multiply also sets `LO`/`HI`.
pub fn world_painter_get_view_irradiance(c: &mut Cpu) {
    c.cycles += 6;
    const VIEW_SIZE: u32 = 0xf0;
    const TABLE: u32 = 0x0050_0000 - 0x5c40; // 0x004f_a3c0
    c.g[V1][0] = VIEW_SIZE as u64;
    c.g[V0][0] = 0x0050_0000;
    let offset = abi::mult(c, abi::arg(c, 0), VIEW_SIZE);
    c.g[A0][0] = sx(offset);
    c.g[V0][0] = sx(TABLE);
    c.g[V0][0] = sx(TABLE.wrapping_add(offset));
    abi::ret(c);
}

/// `BoltsBolt *Bolts_GetCharBolts(BoltsDB *db, int charIndex)` (0x14d998).
pub fn bolts_get_char_bolts(c: &mut Cpu) {
    char_bolts_pointer(c);
}

/// `void* BE_GetCharProfileData(int char, int char2)` (0x14ad28): index into
/// the character-profile table (`char2*0xf88 + char*0x9b50`).
pub fn be_get_char_profile_data(c: &mut Cpu) {
    c.cycles += 9;
    c.g[V1][0] = 0xf88;
    c.g[A0][0] = 0x9b50;
    c.mult(0, V0, A2, V1);
    c.mult(0, A1, A1, A0);
    c.g[V1][0] = BOLT_RECORDS as u64;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(BOLT_RECORDS_OFF));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    abi::ret(c);
}

/// `void* Bolts_GetDB(void)` (0x14bdb8): the bolt database at `0x4a6750`.
pub fn bolts_get_db(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = BOLT_RECORDS as u64;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6750));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// World painter views.

/// The painter views (0xf0 bytes each).
pub(crate) const WP_VIEWS: u32 = 0x004f_a370;

pub(crate) const WP_VIEW_SIZE: u32 = 0xf0;

/// Default colour pair copied to sp+0/sp+0x10 for the rider's surface.
pub(crate) const WP_DEFAULT_COLOURS: u32 = 0x004f_ab20;

pub(crate) const WP_BLEND_COLOURS: u32 = 0x004f_ab40;

/// Loop heads (event checks).
pub(crate) const WP_NOTIFY_LOOP: u32 = 0x002e_d4e8;

pub(crate) const WP_COPY2_LOOP: u32 = 0x002e_d9ac;

pub(crate) const WP_COPY0_LOOP: u32 = 0x002e_d9ec;

pub(crate) const WP_COPY1_LOOP: u32 = 0x002e_da18;

pub(crate) const WP_COPY_BACK_LOOP: u32 = 0x002e_dad0;

/// `void WorldPainter_UpdateView(int view, float x /*f12*/, float y, float
/// z)` (0x2ed490): tell the view's nine painters about the position
/// (virtual 0x10); for the five player views also: the event world
/// section (view 0), the view's brightness, the rider's surface colours
/// (sampled, or lerped/blended by the rider's state) and the irradiance
/// mix (a cross-fade from the previous one, or the lighting-weighted blend
/// of two irradiance sets) into the view record.
pub fn world_painter_update_view(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x330);
        c.g[V0][0] = WP_VIEW_SIZE as u64;
        frame.save(c, S3, 0x2d0);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S5, 0x2b0);
        c.mult(0, V0, S3, V0);
        c.g[S5][0] = 0x0050_0000;
        frame.save(c, S0, 0x300);
        c.g[V1][0] = WP_VIEWS as u64;
        frame.save(c, S1, 0x2f0);
        frame.save_fpr(c, 22, 0x320);
        c.g[S1][0] = 8;
        frame.save_fpr(c, 21, 0x318);
        c.mov_s(22, 12);
        frame.save_fpr(c, 20, 0x310);
        c.mov_s(21, 13);
        frame.save(c, S2, 0x2e0);
        c.mov_s(20, 14);
        frame.save(c, S4, 0x2c0);
        c.g[S0][0] = add32(c.g[V0][0], c.g[V1][0]);
        frame.save_ra(c, 0x2a0);
        c.g[A1][0] = c.lw(r32(c, S0));
        // The nine painters: virtual 0x10 (position).
        loop {
            c.cycles += 12;
            c.mov_s(12, 22);
            c.mov_s(13, 21);
            c.mov_s(14, 20);
            c.g[V1][0] = c.lw(at(c, A1, 4));
            c.g[S0][0] = add32(c.g[S0][0], 4);
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            let vt = r32(c, V1);
            c.g[A0][0] = c.lh(p(vt, 0x10));
            c.g[V0][0] = c.lw(p(vt, 0x14));
            c.g[A0][0] = add32(c.g[A1][0], c.g[A0][0]);
            let target = r32(c, V0);
            call_ptr(c, target, 0x002e_d510)?;
            if (c.g[S1][0] as i64) < 0 {
                break;
            }
            c.g[A1][0] = c.lw(r32(c, S0));
            if event_due(c) {
                deliver_event_here(c, WP_NOTIFY_LOOP);
            }
        }
        c.cycles += 3;
        c.g[V0][0] = (c.g[S3][0] < 6) as u64;
        if c.g[V0][0] != 0 {
            wp_player_view(c, frame)?;
        }
        frame.restore(c, S0, 0x300);
        c.cycles += 11;
        frame.restore(c, S1, 0x2f0);
        frame.restore(c, S2, 0x2e0);
        frame.restore(c, S3, 0x2d0);
        frame.restore(c, S4, 0x2c0);
        frame.restore(c, S5, 0x2b0);
        frame.restore_ra(c, 0x2a0);
        frame.restore_fpr(c, 22, 0x320);
        frame.restore_fpr(c, 21, 0x318);
        frame.restore_fpr(c, 20, 0x310);
        frame.close(c);
        Ok(())
    })
}

/// Views 0..5. Returns with s0 about to be restored by the caller.
pub(crate) fn wp_player_view(c: &mut Cpu, frame: Frame) -> Flow {
    let _ = frame;
    // View 0 with event sections on: follow the rider's event section
    // (layer at gp+0x770/0xa7c) when it is a type-6 section.
    c.cycles += 3;
    c.g[V0][0] = c.lw(gp(c, 0xa6c));
    let events_on = c.g[V0][0] != 0;
    c.g[V0][0] = c.lw(gp(c, -0x848));
    let mut skip_v1 = false;
    if events_on {
        c.cycles += 2;
        let other_view = c.g[S3][0] != 0;
        c.g[V1][0] = WP_VIEW_SIZE as u64;
        if other_view {
            skip_v1 = true;
        } else {
            c.cycles += 16;
            c.g[V0][0] = c.lw(gp(c, -0x848));
            c.g[V1][0] = c.lw(at(c, V0, 0x84));
            c.g[A0][0] = c.lw(at(c, V1, 0x78));
            c.g[A1][0] = c.lw(at(c, A0, 0x1bc));
            call_ptr(c, callee::SECTION_GET_EVENT_WORLD_SECTION, 0x002e_d54c)?;
            c.g[A2][0] = c.g[V0][0];
            c.g[V0][0] = c.lw(gp(c, -0x848));
            c.g[A1][0] = shl32(c.g[A2][0], 3);
            c.g[V1][0] = c.lw(at(c, V0, 0x84));
            c.g[A0][0] = c.lw(at(c, V1, 0x10));
            c.g[V0][0] = c.lw(r32(c, A0));
            c.g[V0][0] = add32(c.g[V0][0], c.g[A1][0]);
            c.g[V1][0] = c.lw(at(c, V0, 0x24));
            c.g[V1][0] ^= 6;
            let not_event = c.g[V1][0] != 0;
            c.g[V0][0] = c.lw(gp(c, -0x848));
            if !not_event {
                c.cycles += 3;
                c.g[V0][0] = c.lw(gp(c, 0xa7c));
                let same = c.g[A2][0] == c.g[V0][0];
                c.g[V0][0] = c.lw(gp(c, -0x848));
                if !same {
                    c.cycles += 3;
                    let v = c.g[A2][0];
                    c.sw(gp(c, 0x770), v);
                    c.sw(gp(c, 0xa7c), v);
                    c.g[V0][0] = c.lw(gp(c, -0x848));
                }
            }
        }
    }
    if !skip_v1 {
        c.cycles += 1;
        c.g[V1][0] = WP_VIEW_SIZE as u64;
    }
    // View brightness (+0x24) and the rider's ground tile (s2).
    c.cycles += 14;
    c.mult(0, V1, S3, V1);
    c.g[A2][0] = shl32(c.g[S3][0], 2);
    c.g[A0][0] = c.lw(at(c, V0, 0x84));
    c.g[A1][0] = WP_VIEWS as u64;
    let k = ldf(c, 1, gp(c, -0x3974));
    c.g[V0][0] = c.lw(at(c, A0, 0xc));
    c.g[V1][0] = add32(c.g[V1][0], c.g[A1][0]);
    c.g[V0][0] = add32(c.g[V0][0], c.g[A2][0]);
    let level = ldf(c, 0, at(c, V1, 0x24));
    c.g[S1][0] = c.lw(at(c, V0, 0x28));
    c.g[A0][0] = add32(c.g[V1][0], 0x24);
    let rider = r32(c, S1);
    c.g[V0][0] = c.lw(p(rider, 0x3fc));
    let boosted = c.g[V0][0] != 0;
    let scaled = level * k;
    setf(c, 1, scaled);
    if boosted {
        c.cycles += 3;
        let add = ldf(c, 0, gp(c, -0x3970));
        setf(c, 0, scaled + add);
    } else {
        c.cycles += 1;
        setf(c, 0, scaled);
    }
    c.cycles += 5;
    c.swc1(0, r32(c, A0));
    c.g[A2][0] = c.lw(p(rider, 0x430));
    c.g[V0][0] = !c.g[A2][0];
    let none = c.g[V0][0] == 0;
    c.g[A1][0] = c.lw(gp(c, 0x16c8));
    if none {
        c.cycles += 1;
        c.g[S2][0] = 0;
    } else {
        // tile = world->layers[id & 0xff]->tiles[id >> 8] (>> 8, * 4).
        c.cycles += 8;
        c.g[V0][0] = shl32(c.g[A2][0] & 0xff, 2);
        c.g[V1][0] = c.lw(r32(c, A1));
        c.g[A0][0] = c.lw(at(c, V1, 8));
        c.g[V0][0] = add32(c.g[V0][0], c.g[A0][0]);
        c.g[V0][0] = c.lw(r32(c, V0));
        if c.g[V0][0] != 0 {
            c.g[V0][0] = c.lw(at(c, V0, 0xc));
            c.cycles += 7;
            c.g[V1][0] = shl32(srl32(c.g[A2][0], 8), 2);
            c.g[V1][0] = add32(c.g[V1][0], c.g[V0][0]);
            c.g[A0][0] = c.lw(r32(c, V1));
            c.g[V0][0] = srl32(c.g[A0][0], 8);
            let found = c.g[V0][0] != 0;
            c.g[V0][0] = shl32(c.g[V0][0], 2);
            if !found {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 1;
            c.g[A0][0] = c.g[V0][0];
        } else {
            c.cycles += 2;
            c.g[A0][0] = 0;
        }
        c.cycles += 2;
        let other_view = c.g[S3][0] != 0;
        c.g[S2][0] = c.g[A0][0];
        if !other_view {
            c.cycles += 4;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            let v = c.g[V0][0];
            c.sw(gp(c, 0x770), v);
            c.sw(gp(c, 0xa7c), v);
        }
    }
    wp_surface_colours(c)?;
    wp_irradiance(c)
}

/// The rider's surface colours into the view (`gp+0xa68`: blend pending).
/// Leaves v0 = 0xf0.
pub(crate) fn wp_surface_colours(c: &mut Cpu) -> Flow {
    c.cycles += 7;
    c.g[A0][0] = c.g[S1][0];
    c.g[S4][0] = add32(c.g[S1][0], 0xaac);
    call_ptr(c, callee::RIDER_GET_MAIN_STATE, 0x002e_d664)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[V0][0] = 1;
    let state1 = c.g[A0][0] == c.g[V0][0];
    c.g[V0][0] = 0x0050_0000;
    let sp = sp(c);
    let pending;
    if state1 {
        wp_lerp_colours(c, sp)?;
        c.g[V0][0] = 1;
        pending = Some(());
    } else {
        // Tile surface type 6 with a type-3 entry: blend toward fixed
        // colours; rider state 4: pending; else sample and blend.
        c.cycles += 2;
        let has_tile = c.g[S2][0] != 0;
        c.g[V0][0] = c.lw(gp(c, 0x16c8));
        let mut special = false;
        if has_tile {
            c.cycles += 4;
            let tile = r32(c, S2);
            c.g[A1][0] = c.lh(p(tile, 0x156));
            c.g[A0][0] = c.lw(r32(c, V0));
            let neg = (c.g[A1][0] as i64) < 0;
            c.g[V0][0] = 0;
            if !neg {
                c.cycles += 7;
                c.g[V0][0] = c.lbu(p(tile, 0x155));
                c.g[V0][0] = shl32(c.g[V0][0], 3);
                c.g[V0][0] = add32(c.g[A0][0], c.g[V0][0]);
                c.g[V1][0] = c.lw(at(c, V0, 0x24));
                c.g[V1][0] ^= 6;
                if c.g[V1][0] != 0 {
                    c.g[V0][0] = 0;
                } else {
                    c.cycles += 6;
                    c.g[V0][0] = 0x18;
                    c.mult(0, V1, A1, V0);
                    c.g[V0][0] = add32(c.g[V1][0], c.g[A0][0]);
                    c.g[V1][0] = c.lw(at(c, V0, 0x3f0));
                    c.g[V1][0] ^= 3;
                    c.g[V0][0] = (c.g[V1][0] < 1) as u64;
                }
            }
            c.cycles += 2;
            special = c.g[V0][0] != 0;
        }
        if !special {
            c.cycles += 19;
            c.g[A0][0] = WP_VIEW_SIZE as u64;
            let one = fconst(c, 0, 0x3f80_0000);
            c.mult(0, A0, S3, A0);
            c.g[V0][0] = 1;
            c.g[V1][0] = WP_VIEWS as u64;
            c.g[A2][0] = 0x0050_0000;
            let v = c.g[V0][0];
            c.sw(gp(c, 0xa68), v);
            c.g[A2][0] = WP_BLEND_COLOURS as u64;
            c.swc1(0, p(sp, 0xc));
            c.g[A1][0] = c.g[SP][0];
            c.g[A0][0] = add32(c.g[A0][0], c.g[V1][0]);
            c.sw(sp, 0);
            c.swc1(0, p(sp, 4));
            c.g[A3][0] = 0;
            c.swc1(0, p(sp, 8));
            let _ = one;
            callee::BLEND_VIEW_COLOURS.call(c, 0x002e_d8c8)?;
            c.g[V0][0] = WP_VIEW_SIZE as u64;
            return Ok(());
        }
        c.cycles += 5;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, callee::RIDER_GET_MAIN_STATE, 0x002e_d8d8)?;
        c.g[V1][0] = 4;
        let state4 = c.g[V0][0] == c.g[V1][0];
        c.g[S0][0] = sx(p(sp, 0x10));
        if state4 {
            c.cycles += 1;
            c.g[V0][0] = 1;
            pending = Some(());
        } else {
            c.cycles += 15;
            let pos = r32(c, S4);
            ldf(c, 13, p(pos, 4));
            ldf(c, 12, pos);
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[SP][0];
            c.g[A2][0] = c.g[S0][0];
            callee::SAMPLE_SURFACE_COLOURS.call(c, 0x002e_d908)?;
            c.g[A0][0] = WP_VIEW_SIZE as u64;
            c.g[V0][0] = WP_VIEWS as u64;
            c.mult(0, A0, S3, A0);
            c.g[A3][0] = c.lw(gp(c, 0xa68));
            c.g[A2][0] = c.g[S0][0];
            c.g[A1][0] = c.g[SP][0];
            c.g[A0][0] = add32(c.g[A0][0], c.g[V0][0]);
            callee::BLEND_VIEW_COLOURS.call(c, 0x002e_d928)?;
            c.sw(gp(c, 0xa68), 0);
            pending = None;
        }
    }
    if pending.is_some() {
        c.cycles += 2;
        let v = c.g[V0][0];
        c.sw(gp(c, 0xa68), v);
    }
    c.cycles += 1;
    c.g[V0][0] = WP_VIEW_SIZE as u64;
    Ok(())
}

/// Rider state 1: the default colour pair, then (on the rider's board
/// surface kinds 1 and 3) a lerp factor from the board's travel and the
/// sampled surface; `WorldPainter_LerpViewColours`.
pub(crate) fn wp_lerp_colours(c: &mut Cpu, sp: u32) -> Flow {
    c.cycles += 24;
    c.g[V1][0] = 0x0050_0000;
    c.g[T0][0] = WP_DEFAULT_COLOURS as u64;
    ld_d(c, A1, WP_DEFAULT_COLOURS);
    ld_d(c, A2, p(WP_DEFAULT_COLOURS, 8));
    sd_d(c, A1, sp);
    sd_d(c, A2, p(sp, 8));
    c.g[T0][0] = WP_BLEND_COLOURS as u64;
    ld_d(c, V0, WP_BLEND_COLOURS);
    ld_d(c, A2, p(WP_BLEND_COLOURS, 8));
    sd_d(c, V0, p(sp, 0x10));
    sd_d(c, A2, p(sp, 0x18));
    ldf(c, 1, gp(c, -0x396c));
    let rider = r32(c, S1);
    c.g[V0][0] = c.lw(p(rider, 0x788));
    c.g[V1][0] = c.lw(at(c, V0, 0xac));
    let kind1 = c.g[V1][0] == c.g[A0][0];
    c.g[A1][0] = 0;
    let lerp = if kind1 {
        true
    } else {
        c.cycles += 3;
        c.g[V0][0] = 3;
        c.g[V1][0] == c.g[V0][0]
    };
    if lerp {
        c.cycles += 1;
        c.g[A1][0] = 1;
    }
    c.cycles += 2;
    let on = c.g[A1][0] != 0;
    c.g[A2][0] = sx(p(sp, 0x10));
    if on {
        wp_lerp_factor(c, sp)?;
    }
    c.cycles += 9;
    c.g[A0][0] = WP_VIEW_SIZE as u64;
    c.g[V0][0] = WP_VIEWS as u64;
    c.mult(0, A0, S3, A0);
    c.mov_s(12, 1);
    c.g[A1][0] = c.g[SP][0];
    c.g[A0][0] = add32(c.g[A0][0], c.g[V0][0]);
    callee::LERP_VIEW_COLOURS.call(c, 0x002e_d828)
}

/// The lerp factor (f1) from the board record (rider +0x788): travel
/// `f21 = +0x98 - +0xa0`, a soft limit `f20 = clamp(+0x98 / 2, 2,
/// +0x98)`; below it a ratio, beyond it the sampled surface colours.
pub(crate) fn wp_lerp_factor(c: &mut Cpu, sp: u32) -> Flow {
    c.cycles += 9;
    let rider = r32(c, S1);
    c.g[V0][0] = c.lw(p(rider, 0x788));
    let board = r32(c, V0);
    c.g[A0][0] = c.lw(p(board, 0x30));
    c.g[T0][0] = c.g[V0][0];
    let span = ldf(c, 2, p(board, 0x98));
    let pos = ldf(c, 0, p(board, 0xa0));
    c.g[A3][0] = c.g[A0][0];
    c.g[V1][0] = !c.g[A0][0];
    let no_surface = c.g[V1][0] == 0;
    let travel = span - pos;
    setf(c, 21, travel);
    if no_surface {
        return Ok(());
    }
    c.cycles += 11;
    let half = fconst(c, 1, 0x3f00_0000);
    let two = fconst(c, 0, 0x4000_0000);
    let limit = (span * half).max(two).min(span);
    setf(c, 20, limit);
    let beyond = lt(c, limit, travel);
    setf(c, 2, span - limit);
    if beyond {
        c.cycles += 9;
        let rest = fr(c, 2);
        let d = travel - limit;
        setf(c, 0, d);
        let cap = ldf(c, 3, gp(c, -0x3968));
        setf(c, 1, FpuF32::ZERO);
        let r = (rest - d).div(rest);
        setf(c, 0, r);
        let pos = le(c, FpuF32::ZERO, r);
        setf(c, 12, FpuF32::ZERO);
        if pos {
            c.cycles += 1;
            setf(c, 12, r.min(cap));
        }
        c.cycles += 2;
        c.mov_s(1, 12);
        return Ok(());
    }
    // The tile under the surface id (a3).
    c.cycles += 9;
    c.g[A1][0] = c.lw(gp(c, 0x16c8));
    c.g[V0][0] = shl32(c.g[A0][0] & 0xff, 2);
    c.g[V1][0] = c.lw(r32(c, A1));
    c.g[A0][0] = c.lw(at(c, V1, 8));
    c.g[V0][0] = add32(c.g[V0][0], c.g[A0][0]);
    c.g[V1][0] = c.lw(r32(c, V0));
    let none = c.g[V1][0] == 0;
    c.g[A0][0] = 0;
    if !none {
        c.cycles += 8;
        c.g[A0][0] = c.lw(at(c, V1, 0xc));
        c.g[V0][0] = shl32(srl32(c.g[A3][0], 8), 2);
        c.g[V0][0] = add32(c.g[V0][0], c.g[A0][0]);
        c.g[V1][0] = c.lw(r32(c, V0));
        c.g[V0][0] = srl32(c.g[V1][0], 8);
        let none = c.g[V0][0] == 0;
        c.g[A0][0] = 0;
        if !none {
            c.cycles += 1;
            c.g[A0][0] = shl32(c.g[V0][0], 2);
        }
    }
    c.cycles += 11;
    c.g[S0][0] = sx(p(sp, 0x10));
    let board = r32(c, T0);
    ldf(c, 13, p(board, 0x38));
    ldf(c, 12, p(board, 0x34));
    c.g[A2][0] = c.g[S0][0];
    c.g[A1][0] = c.g[SP][0];
    callee::SAMPLE_SURFACE_COLOURS.call(c, 0x002e_d7cc)?;
    setf(c, 2, FpuF32::ZERO);
    let limit = fr(c, 20);
    let positive = lt(c, FpuF32::ZERO, limit);
    c.g[A2][0] = c.g[S0][0];
    if positive {
        c.cycles += 7;
        let d = limit - fr(c, 21);
        setf(c, 0, d);
        let cap = ldf(c, 3, gp(c, -0x3964));
        let r = d.div(limit);
        setf(c, 0, r);
        let pos = le(c, FpuF32::ZERO, r);
        c.mov_s(1, 2);
        if pos {
            c.cycles += 2;
            setf(c, 1, r.min(cap));
        }
        return Ok(());
    }
    c.cycles += 2;
    fconst(c, 1, 0x3f80_0000);
    Ok(())
}

/// Copy `n` 0x20-byte blocks with two `lq`/`sq` pairs through `r0`/`r1`
/// (`src`/`dst` registers advanced, loop until `src == end_reg`), event
/// checks at `head`.
pub(crate) fn wp_copy_blocks(
    c: &mut Cpu,
    src: usize,
    dst: usize,
    end_reg: usize,
    r0: usize,
    r1: usize,
    head: u32,
) {
    loop {
        c.cycles += 7;
        let s = r32(c, src);
        let d = r32(c, dst);
        c.lq(r0, s);
        c.lq(r1, p(s, 0x10));
        c.sq(r0, d);
        c.sq(r1, p(d, 0x10));
        c.g[src][0] = sx(p(s, 0x20));
        let more = c.g[src][0] != c.g[end_reg][0];
        c.g[dst][0] = sx(p(r32(c, dst), 0x20));
        if !more {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// The view's irradiance (+0x50, 0xa0 bytes): a cross-fade (fade >
/// threshold) or the lighting-weighted mix of sets 0 and 1.
pub(crate) fn wp_irradiance(c: &mut Cpu) -> Flow {
    let sp = sp(c);
    c.cycles += 9;
    c.g[V1][0] = WP_VIEWS as u64;
    c.mult(0, A2, S3, V0);
    let threshold = ldf(c, 1, gp(c, -0x3960));
    c.g[V0][0] = add32(c.g[A2][0], c.g[V1][0]);
    let fade = ldf(c, 0, at(c, V0, 0x24));
    let fading = lt(c, threshold, fade);
    c.g[A0][0] = 1;
    if !fading {
        c.cycles += 1;
        c.g[A0][0] = 0;
    }
    c.cycles += 2;
    let fading = c.g[A0][0] != 0;
    c.g[S0][0] = add32(c.g[V1][0], 0x50);
    if fading {
        c.cycles += 19;
        c.g[S2][0] = sx(p(sp, 0xc0));
        c.g[S0][0] = add32(c.g[A2][0], c.g[S0][0]);
        c.g[S1][0] = sx(p(sp, 0x20));
        c.g[A0][0] = c.g[S3][0];
        callee::GET_IRRADIANCE2.call(c, 0x002e_d974)?;
        ldf(c, 12, gp(c, 0xa80));
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, callee::IRRADIANCE_SCALE, 0x002e_d984)?;
        let t = ldf(c, 0, gp(c, 0xa80));
        c.g[A0][0] = c.g[S0][0];
        let one = fconst(c, 12, 0x3f80_0000);
        setf(c, 12, one - t);
        call_ptr(c, callee::IRRADIANCE_SCALE_IN_PLACE, 0x002e_d99c)?;
        c.g[A2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, callee::IRRADIANCE_ADD, 0x002e_d9ac)?;
        wp_copy_blocks(c, S1, S0, S2, A0, A1, WP_COPY2_LOOP);
        c.cycles += 2;
        return Ok(());
    }
    // Sets 0 and 1 to sp+0 and sp+0xa0.
    c.cycles += 7;
    c.g[A0][0] = c.g[S3][0];
    callee::GET_IRRADIANCE0.call(c, 0x002e_d9d8)?;
    c.g[S4][0] = sx(p(sp, 0xa0));
    c.g[S5][0] = sx(p(sp, 0x160));
    c.g[S2][0] = sx(p(sp, 0x200));
    c.g[A0][0] = add32(c.g[V0][0], 0xa0);
    c.g[V1][0] = c.g[SP][0];
    wp_copy_blocks(c, V0, V1, A0, T0, A1, WP_COPY0_LOOP);
    c.cycles += 4;
    c.g[A0][0] = c.g[S3][0];
    callee::GET_IRRADIANCE1.call(c, 0x002e_da10)?;
    c.g[V1][0] = c.g[S4][0];
    c.g[A0][0] = add32(c.g[V0][0], 0xa0);
    wp_copy_blocks(c, V0, V1, A0, T0, A1, WP_COPY1_LOOP);
    // w = factor * lighting; mix = set0 * w + set1 * (1 - w); then the
    // cross-fade with the view's previous irradiance (+0x68).
    c.cycles += 39;
    c.g[S1][0] = WP_VIEW_SIZE as u64;
    c.g[S0][0] = 0x0050_0000;
    c.mult(0, S1, S3, S1);
    c.g[S0][0] = sx(0x004f_a3a8);
    let one = fconst(c, 21, 0x3f80_0000);
    c.g[A0][0] = add32(c.g[S1][0], c.g[S0][0]);
    callee::COMPUTE_COLOUR_FACTOR.call(c, 0x002e_da54)?;
    c.mov_s(20, 0);
    c.g[A0][0] = c.g[S3][0];
    callee::GET_LIGHTING28.call(c, 0x002e_da60)?;
    let w = fr(c, 20) * fr(c, 0);
    setf(c, 20, w);
    c.g[S0][0] = add32(c.g[S0][0], 0x18);
    c.g[A0][0] = c.g[SP][0];
    c.g[S1][0] = add32(c.g[S1][0], c.g[S0][0]);
    setf(c, 12, w);
    call_ptr(c, callee::IRRADIANCE_SCALE_IN_PLACE, 0x002e_da78)?;
    setf(c, 12, fr(c, 21) - fr(c, 20));
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S4][0];
    call_ptr(c, callee::IRRADIANCE_SCALE_IN_PLACE, 0x002e_da88)?;
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, callee::IRRADIANCE_ADD, 0x002e_da98)?;
    ldf(c, 12, gp(c, 0xa80));
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, callee::IRRADIANCE_SCALE_IN_PLACE, 0x002e_daa4)?;
    let t = ldf(c, 12, gp(c, 0xa80));
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S1][0];
    setf(c, 12, fr(c, 21) - t);
    let _ = one;
    call_ptr(c, callee::IRRADIANCE_SCALE_IN_PLACE, 0x002e_dab8)?;
    c.g[A2][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S5][0];
    c.g[A1][0] = c.g[S0][0];
    call_ptr(c, callee::IRRADIANCE_ADD, 0x002e_dac8)?;
    c.g[V0][0] = c.g[S2][0];
    c.g[A0][0] = c.g[S5][0];
    wp_copy_blocks(c, A0, S1, V0, T0, V1, WP_COPY_BACK_LOOP);
    c.cycles += 1;
    Ok(())
}

// ---------------------------------------------------------------------------
// IrradianceRef_Resolve.

/// `void IrradianceRef_Resolve(IrradianceRef* r)` (0x2edfd0): store the
/// manager's handle for `r` (`func_0038AC50(&D_005047F8, r)`) in `r->handle`
/// (+0) and the type tag 0x123400 in `r->tag` (+4).
pub fn irradiance_ref_resolve(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x50_0000;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x47f8));
        call_ptr(c, 0x0038_ac50, 0x002e_dff0)?;
        c.g[V1][0] = 0x12_0000;
        let handle = c.g[V0][0];
        let this = r32(c, S0);
        Addr(this).sw(c, handle);
        c.g[V1][0] = c.g[V1][0] | 0x3400;
        frame.restore_ra(c, 0);
        let tag = c.g[V1][0];
        Addr(this.wrapping_add(0x4)).sw(c, tag);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void WorldPainter_GetIrradiance0()` (0x2ee318): look up entry 0 of the
/// irradiance table by name.
pub fn world_painter_get_irradiance0(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::WORLD_PAINTER_GET_IRRADIANCE_NAME0.call(c, 0x002e_e328)?;
        c.g[A0][0] = c.g[V0][0];
        callee::IRRADIANCE_REF_GET_ENTRY.call(c, 0x002e_e330)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WorldPainter_GetIrradiance1()` (0x2ee340): look up entry 1 of the
/// irradiance table by name.
pub fn world_painter_get_irradiance1(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::WORLD_PAINTER_GET_IRRADIANCE_NAME1.call(c, 0x002e_e350)?;
        c.g[A0][0] = c.g[V0][0];
        callee::IRRADIANCE_REF_GET_ENTRY.call(c, 0x002e_e358)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Bolt-on FX.

/// `void cBoltOnFXInterface_Update(cBoltOnFXInterface *self)` (0x2ecf78):
/// refresh the emitters when the interface is attached to a world.
pub fn c_bolton_fx_interface_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let selfp = Addr(abi::arg(c, 0));
        let attached = selfp.lw(c);
        c.g[V0][0] = attached;
        frame.save_ra(c, 0);
        if attached != 0 {
            c.cycles += 2;
            callee::C_BOLTON_FX_REFRESH_EMITTERS.call(c, 0x002e_cf90)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// World painter getters.

/// The lens-fx / render-state entry table `D_004FA370` (built as
/// `0x500000 - 0x5c90`) with stride 0xf0; the entry's object pointer is at
/// `+0x14` (lens fx) or `+0x1c` (render state).

/// `float WorldPainter_GetSunFlareIntensity(int i)` (0x2ee268): call virtual
/// slot 34 on the object of lens-fx entry `i` and return the float it
/// returns.
pub fn world_painter_sun_flare_intensity(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 18;
        c.g[V1][0] = 0xf0;
        c.mult(0, A0, A0, V1);
        c.g[V0][0] = 0x50_0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x5c90));
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
        c.g[V1][0] = c.lw(at(c, A0, 0x14));
        c.g[A2][0] = c.lw(r32(c, V1));
        c.g[A1][0] = c.lw(at(c, A2, 0x4));
        c.g[A0][0] = c.lh(at(c, A1, 0x110));
        c.g[V0][0] = c.lw(at(c, A1, 0x114));
        let target = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x002e_e2a0)?;
        c.lwc1(0, r32(c, V0));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// World painter view colours.

/// `void WorldPainter_LerpViewColours(WorldPainter* self, const Vec4* a,
/// const Vec4* b, float t)` (0x2ed338): `self->colour0/1 = old * (1 - t) +
/// new * t` (two 16-byte colour quads at self+0x28 and self+0x38).
pub fn world_painter_lerp_view_colours(c: &mut Cpu) {
    c.cycles += 86;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 1);
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
    let this = abi::arg(c, 0);
    let a = abi::arg(c, 1);
    let b = abi::arg(c, 2);
    // s = 1 - t, kept in f1.
    c.lwc1(5, p(this, 0x28));
    c.sub_s(1, 1, 12);
    c.lwc1(4, p(this, 0x2c));
    c.lwc1(3, p(this, 0x30));
    c.lwc1(2, p(this, 0x34));
    c.lwc1(8, p(a, 0x0c));
    c.mul_s(5, 5, 1);
    c.lwc1(0, a);
    c.mul_s(4, 4, 1);
    c.lwc1(7, p(a, 0x04));
    c.mul_s(3, 3, 1);
    c.lwc1(6, p(a, 0x08));
    c.mul_s(2, 2, 1);
    c.mul_s(8, 8, 12);
    c.swc1(5, p(sp(c), 0x10));
    c.mul_s(0, 0, 12);
    c.swc1(4, p(sp(c), 0x14));
    c.mul_s(7, 7, 12);
    c.swc1(3, p(sp(c), 0x18));
    c.mul_s(6, 6, 12);
    c.swc1(2, p(sp(c), 0x1c));
    c.add_s(5, 5, 0);
    c.swc1(0, p(sp(c), 0x20));
    c.add_s(4, 4, 7);
    c.swc1(7, p(sp(c), 0x24));
    c.add_s(3, 3, 6);
    c.swc1(6, p(sp(c), 0x28));
    c.add_s(2, 2, 8);
    c.swc1(5, sp(c));
    c.swc1(4, p(sp(c), 0x04));
    c.swc1(3, p(sp(c), 0x08));
    c.swc1(2, p(sp(c), 0x0c));
    c.swc1(8, p(sp(c), 0x2c));
    c.ldl(2, p(sp(c), 0x07));
    c.ldr(2, sp(c));
    c.ldl(3, p(sp(c), 0x0f));
    c.ldr(3, p(sp(c), 0x08));
    c.sdl(2, p(this, 0x2f));
    c.sdr(2, p(this, 0x28));
    c.sdl(3, p(this, 0x37));
    c.sdr(3, p(this, 0x30));
    // Second colour quad.
    c.lwc1(4, p(this, 0x38));
    c.lwc1(5, p(this, 0x3c));
    c.lwc1(3, p(this, 0x40));
    c.mul_s(4, 4, 1);
    c.lwc1(2, p(this, 0x44));
    c.mul_s(5, 5, 1);
    c.lwc1(8, p(b, 0x0c));
    c.mul_s(3, 3, 1);
    c.lwc1(0, b);
    c.mul_s(2, 2, 1);
    c.lwc1(7, p(b, 0x04));
    c.mul_s(8, 8, 12);
    c.lwc1(6, p(b, 0x08));
    c.mul_s(0, 0, 12);
    c.mul_s(7, 7, 12);
    c.swc1(4, p(sp(c), 0x10));
    c.mul_s(6, 6, 12);
    c.swc1(5, p(sp(c), 0x14));
    c.swc1(3, p(sp(c), 0x18));
    c.add_s(4, 4, 0);
    c.swc1(2, p(sp(c), 0x1c));
    c.add_s(5, 5, 7);
    c.add_s(3, 3, 6);
    c.swc1(0, p(sp(c), 0x20));
    c.add_s(2, 2, 8);
    c.swc1(4, sp(c));
    c.swc1(5, p(sp(c), 0x04));
    c.swc1(3, p(sp(c), 0x08));
    c.swc1(2, p(sp(c), 0x0c));
    c.swc1(7, p(sp(c), 0x24));
    c.swc1(6, p(sp(c), 0x28));
    c.swc1(8, p(sp(c), 0x2c));
    c.ldl(2, p(sp(c), 0x07));
    c.ldr(2, sp(c));
    c.ldl(3, p(sp(c), 0x0f));
    c.ldr(3, p(sp(c), 0x08));
    c.sdl(2, p(this, 0x3f));
    c.sdr(2, p(this, 0x38));
    c.sdl(3, p(this, 0x47));
    c.sdr(3, p(this, 0x40));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
    c.pc = ra;
}

/// `void Irradiance_GetOverride(void)` (0x002ee2f8): look up the global lens
/// FX irradiance entry at `gp_2 + 0x24c0`.
pub fn irradiance_get_override(c: &mut Cpu) {
    forward(
        c,
        8,
        0x10,
        |c| c.g[A0][0] = sx(gp_2(c, 0x24c0)),
        callee::IRRADIANCE_REF_GET_ENTRY_2,
        0x002e_e30c,
    )
}

/// `Irradiance *WorldPainter_GetIrradiance2(void)` (0x2ee368): resolve the
/// painter's irradiance name to a reference, then look the entry up.
pub fn world_painter_get_irradiance2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::WORLD_PAINTER_GET_IRRADIANCE_NAME2.call(c, 0x002e_e378)?;
        c.g[A0][0] = c.g[V0][0];
        callee::IRRADIANCE_REF_GET_ENTRY.call(c, 0x002e_e380)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cBoltOnFXInterface_ClearEmitters(cBoltOnFXInterface* fx)`
/// (0x2ecaf0): invalidate the 15 emitter slots (`id = -1`).
pub fn c_bolt_on_fx_interface_clear_emitters(c: &mut Cpu) {
    const HEAD: u32 = 0x002e_cb00;
    c.cycles += 4;
    c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(0xb8));
    c.g[V1][0] = u64::MAX;
    c.g[V0][0] = 0xe;
    loop {
        c.cycles += 7;
        Addr(r32(c, A0)).sw(c, c.g[V1][0]);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
        let more = (c.g[V0][0] as i64) >= 0;
        c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0xc));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, HEAD);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// WorldPainter / lens effects.

/// `float WorldPainter_GetWeather30(int i)` (0x2ee6f0): the lens-effect slot
/// `i` (`D_004FA370 + i * 0xf0`) asks its object's virtual weather accessor
/// (vtable slot 0x180/0x184) for the current weather blend.
pub fn world_painter_get_weather30(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[V1][0] = 0xf0; // sizeof(sLensFxSlot)
        let frame = Frame::open(c, 0x10);
        c.mult(0, A0, A0, V1); // a0 = i * 0xf0
        c.g[V0][0] = 0x0050_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(0x0050_0000u32.wrapping_sub(0x5c90)); // &D_004FA370
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let slot = c.g[A0][0] as u32;
        c.g[V1][0] = Addr(slot).off(0x20).lw(c); // s->p20
        let obj = Addr(c.g[V1][0] as u32).lw(c); // *s->p20
        c.g[A2][0] = obj;
        let vt = Addr((obj as u32).wrapping_add(4)).lw(c); // obj->vt
        c.g[A1][0] = vt;
        c.g[A0][0] = Addr((vt as u32).wrapping_add(0x180)).lh(c); // delta
        c.g[V0][0] = Addr((vt as u32).wrapping_add(0x184)).lw(c); // fn
        let target = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x002e_e728);
        c.g[A0][0] = sx((obj as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x002e_e728)?;
        c.lwc1(0, c.g[V0][0] as u32); // f0 = weather blend
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cBoltOnFXInterface_FindOrCreateEmitter(cBoltOnFXInterface* self,
/// int a1, int a2)` (0x2ecbc0): find the first of the 15 `{id, a, b}` slots
/// matching `(a2, a1)` (id != -1) and return its index, else create one.
pub fn c_bolt_on_fx_interface_find_or_create_emitter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = 0;
        frame.save_ra(c, 0);
        c.g[T0][0] = 0xffff_ffff_ffff_ffff;
        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        let mut i: u64;
        let mut slot = c.g[V1][0];
        loop {
            let mut found = false;
            c.cycles += 3;
            c.g[V0][0] = lw(c, (slot as u32).wrapping_sub(8));
            if c.g[V0][0] == c.g[T0][0] {
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 3;
                c.g[V0][0] = lw(c, (slot as u32).wrapping_sub(4));
                if c.g[V0][0] != c.g[A2][0] {
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = lw(c, slot as u32);
                    if c.g[V0][0] != c.g[A1][0] {
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                    } else {
                        c.cycles += 2;
                        found = true;
                    }
                }
            }
            i = c.g[A3][0];
            slot = c.g[V1][0];
            if found {
                c.g[V0][0] = i;
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = ((i as i64) < 15) as u64;
            let more = c.g[V0][0] != 0;
            slot = sx((slot as u32).wrapping_add(0xc));
            c.g[V1][0] = slot;
            if !more {
                c.cycles += 2;
                call_ptr(c, 0x002e_cc28, 0x002e_cc18)?;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002e_cbd8);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Bolt-on effects.

/// Shared body of the two `cBoltOnFXInterface` constructors: publish the
/// vtable, store the two owner words, invalidate the 15 emitter handles and
/// clear the emitters. `swapped` is true for the variant that takes its
/// primary argument in `a2`; `rets` are the original's three return addresses.
pub fn c_bolt_on_fx_interface_ctor_common(c: &mut Cpu, swapped: bool, rets: [u32; 3], head: u32) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x48_0000;
        frame.save(c, S0, 0x40);
        c.g[V0][0] = sx(0x0048_7ee8);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[V1][0] = 0x1;
        frame.save(c, S3, 0x10);
        c.g[S1][0] = sx(p(r32(c, S0), 0x10));
        frame.save_ra(c, 0);
        c.g[S2][0] = 0xe;
        Addr(p(r32(c, S0), 0xc4)).sw(c, c.g[V0][0]);
        c.g[S3][0] = u64::MAX;
        Addr(r32(c, S0)).sw(c, c.g[V1][0]);
        if swapped {
            Addr(p(r32(c, S0), 8)).sw(c, c.g[A1][0]);
            Addr(p(r32(c, S0), 0xc)).sw(c, c.g[A2][0]);
            Addr(p(r32(c, S0), 4)).sw(c, 0);
        } else {
            Addr(p(r32(c, S0), 4)).sw(c, c.g[A1][0]);
            Addr(p(r32(c, S0), 0xc)).sw(c, c.g[A2][0]);
            Addr(p(r32(c, S0), 8)).sw(c, 0);
        }
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(rets[0]);
            c.g[S2][0] = sx(r32(c, S2).wrapping_sub(1));
            callee::BOLT_ON_FX_INIT_HANDLE.call(c, rets[0])?;
            let more = c.g[S2][0] != c.g[S3][0];
            c.g[S1][0] = sx(p(r32(c, S1), 0xc));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, head);
            }
        }
        c.cycles += 10;
        c.g[RA][0] = sx(rets[2]);
        c.g[A0][0] = c.g[S0][0];
        callee::BOLT_ON_FX_CLEAR_EMITTERS.call(c, rets[2])?;
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

/// `void* cBoltOnFXInterface_ctor(cBoltOnFXInterface* self, void* a1, void* a2)`
/// (0x2ec9e0).
pub fn c_bolt_on_fx_interface_ctor(c: &mut Cpu) {
    c_bolt_on_fx_interface_ctor_common(
        c,
        false,
        [0x002e_ca34, 0x002e_cab0, 0x002e_ca44],
        0x002e_ca28,
    );
}

/// `void* cBoltOnFXInterface_ctor_2(cBoltOnFXInterface* self, void* a1,
/// void* a2)` (0x2eca68): the variant that stores `a1` at +0x8.
pub fn c_bolt_on_fx_interface_ctor_2(c: &mut Cpu) {
    c_bolt_on_fx_interface_ctor_common(
        c,
        true,
        [0x002e_cabc, 0x002e_cab0, 0x002e_cacc],
        0x002e_cab0,
    );
}

// ---------------------------------------------------------------------------
// Lens FX emitters.

/// `int cBoltOnFXInterface_CreateEmitter(LensFX* self, void* x, void* y)`
/// (0x2ecc28): take the next free emitter slot (of 15) and start an attached
/// dynamic emitter for it.
pub fn c_bolt_on_fx_interface_create_emitter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = u64::MAX;
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = sx(p(r32(c, A0), 0x10));
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A2][0];
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
            if c.g[V0][0] != c.g[V1][0] {
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 15i64) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(0xc));
                if !more {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002e_cc58);
                }
                continue;
            }
            c.cycles += 9;
            c.g[A0][0] = Addr(p(r32(c, A0), 0xc)).lw(c);
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S3][0];
            callee::ATTACHED_DYNAMIC_EMITTER_CREATE.call(c, 0x002e_cc74)?;
            Addr(r32(c, S0)).sw(c, c.g[V0][0]);
            Addr(p(r32(c, S0), 8)).sw(c, c.g[S2][0]);
            c.g[V0][0] = c.g[S1][0];
            Addr(p(r32(c, S0), 4)).sw(c, c.g[S3][0]);
            break;
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

/// `cBoltOnFXInterface_dtor` (0x2ecb28): destroy the interface's fifteen
/// attached emitter slots and optionally free the object.
pub fn bolt_on_fx_interface_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x0048_0000;
        frame.save(c, S2, 0x30);
        c.g[V0][0] = sx(0x0048_0000u32.wrapping_add(0x7ee8));
        frame.save(c, S0, 0x50);
        let this = r(c, A0);
        c.g[S2][0] = sx(this);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = sx(this.wrapping_add(0x10));
        frame.save(c, S3, 0x20);
        c.g[S1][0] = 0xe;
        frame.save(c, S4, 0x10);
        c.g[S3][0] = 0xffff_ffff_ffff_ffff;
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A1][0];
        Addr(p(this, 0xc4)).sw(c, c.g[V0][0]);
        loop {
            c.cycles += 3;
            c.g[A1][0] = Addr(r(c, S0)).lw(c);
            if c.g[A1][0] != c.g[S3][0] {
                c.cycles += 4;
                c.g[RA][0] = sx(0x002e_cb7c);
                c.g[A0][0] = Addr(p(r(c, S2), 0xc)).lw(c);
                call_ptr(c, callee::ATTACHED_EMITTER_FX_DESTROY_EMITTER, 0x002e_cb7c)?;
                Addr(r(c, S0)).sw(c, c.g[S3][0]);
            }
            c.cycles += 2;
            c.g[S1][0] = sx(r(c, S1).wrapping_sub(1));
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx(r(c, S0).wrapping_add(0xc));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002e_cb68);
            }
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S4][0] & 1;
        let free_self = c.g[V0][0] != 0;
        frame.restore(c, S0, 0x50);
        if free_self {
            c.cycles += 3;
            c.g[RA][0] = sx(0x002e_cba0);
            c.g[A0][0] = c.g[S2][0];
            callee::BUILTIN_DELETE_2.call(c, 0x002e_cba0)?;
            frame.restore(c, S0, 0x50);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WorldPainter_FreeViewQueries()` (0x2ecfa0): release the 8 view
/// queries of every painter (the table is at_2 0x9fa370).
pub fn world_painter_free_view_queries(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x60);
        'end: {
            c.cycles += 9;
            c.g[V0][0] = lw(c, gp_3(c, 0xa64));
            frame.save(c, S0, 0x50);
            frame.save(c, S1, 0x40);
            frame.save(c, S2, 0x30);
            frame.save(c, S3, 0x20);
            frame.save(c, S4, 0x10);
            let t = c.g[V0][0] == 0;
            frame.save_ra(c, 0);
            if t {
                break 'end;
            }
            c.cycles += 5;
            c.g[V0][0] = sx(0x0050_0000);
            c.g[V1][0] = 0;
            c.g[S4][0] = sx(0x004f_a370);
            c.g[S3][0] = 0xf0;
            c.mult(0, 2, 3, 19);
            loop {
                c.cycles += 4;
                c.g[S2][0] = sx(at_2(c.g[V1][0], 1));
                c.g[S1][0] = 8;
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                loop {
                    {
                        c.cycles += 3;
                        c.g[A2][0] = lw(c, c.g[S0][0] as u32);
                        let t = c.g[A2][0] == 0;
                        c.g[A1][0] = 3;
                        if !t {
                            c.cycles += 5;
                            c.g[V0][0] = lw(c, at_2(c.g[A2][0], 4));
                            c.g[A0][0] = lh(c, at_2(c.g[V0][0], 8));
                            c.g[V1][0] = lw(c, at_2(c.g[V0][0], 0xc));
                            let target = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x002e_d008);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, target, 0x002e_d008)?;
                        }
                    }
                    c.cycles += 3;
                    c.g[S1][0] = sx(at_2(c.g[S1][0], 0xffff_ffff));
                    let t = (c.g[S1][0] as i64) >= 0;
                    c.g[S0][0] = sx(at_2(c.g[S0][0], 4));
                    if !t {
                        c.cycles += 4;
                        c.g[V1][0] = c.g[S2][0];
                        c.g[V0][0] = ((c.g[V1][0] as i64) < 8i64) as u64;
                        let t = c.g[V0][0] != 0;
                        c.mult(0, 2, 3, 19);
                        if !t {
                            break 'end;
                        }
                        if crate::events::event_due(c) {
                            crate::events::deliver_event_here(c, 0x002e_cfd8);
                        }
                        break;
                    }
                    if crate::events::event_due(c) {
                        crate::events::deliver_event_here(c, 0x002e_cfe8);
                    }
                }
            }
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        Addr(gp_3(c, 0xa64)).sw(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Helpers.

/// The world-painter view table `D_004FA370` (built as `0x500000 -
/// 0x5c90`), stride 0xf0; the sun interface holder at +0x14.
pub(crate) const PAINTER_VIEWS_HI: u64 = 0x50_0000;

pub(crate) const PAINTER_VIEWS_LO: u32 = 0x5c90;

pub(crate) const PAINTER_VIEW_SUN: u32 = 0x14;

/// The `WorldPainter_GetSun*` float getters (index in a0): `obj =
/// *views[index].sun`, call its virtual slot `vt_off` (`{delta, fn}` at
/// `obj->vtable + vt_off`) with `obj + delta`, and return the float the
/// method's result points at.
pub(crate) fn sun_float_getter(c: &mut Cpu, vt_off: u32, ret_addr: u32) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[V1][0] = 0xf0;
        let frame = Frame::open(c, 0x10);
        c.mult(0, A0, A0, V1); // a0 = index * 0xf0
        c.g[V0][0] = PAINTER_VIEWS_HI;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx((PAINTER_VIEWS_HI as u32).wrapping_sub(PAINTER_VIEWS_LO));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
        c.g[V1][0] = c.lw(at(c, A0, PAINTER_VIEW_SUN));
        c.g[A2][0] = c.lw(r32(c, V1));
        c.g[A1][0] = c.lw(at(c, A2, 4));
        c.g[A0][0] = c.lh(at(c, A1, vt_off));
        c.g[V0][0] = c.lw(at(c, A1, vt_off + 4));
        let method = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
        call_ptr(c, method, ret_addr)?;
        c.lwc1(0, r32(c, V0));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float WorldPainter_GetSunElevation(int view)` (0x2ee070): sun slot
/// 27 (`tWPIGD_Sun_GetElevation`).
pub fn world_painter_get_sun_elevation(c: &mut Cpu) {
    sun_float_getter(c, 0xd8, 0x002e_e0a8);
}

/// `float WorldPainter_GetSunAzimuth(int view)` (0x2ee0b8): sun slot 28
/// (`tWPIGD_Sun_GetAzimuth`).
pub fn world_painter_get_sun_azimuth(c: &mut Cpu) {
    sun_float_getter(c, 0xe0, 0x002e_e0f0);
}

/// `float WorldPainter_GetSunRed(int view)` (0x2ee100): sun slot 29.
pub fn world_painter_get_sun_red(c: &mut Cpu) {
    sun_float_getter(c, 0xe8, 0x002e_e138);
}

/// `float WorldPainter_GetSunGreen(int view)` (0x2ee148): sun slot 30.
pub fn world_painter_get_sun_green(c: &mut Cpu) {
    sun_float_getter(c, 0xf0, 0x002e_e180);
}

/// `float WorldPainter_GetSunBlue(int view)` (0x2ee190): sun slot 31.
pub fn world_painter_get_sun_blue(c: &mut Cpu) {
    sun_float_getter(c, 0xf8, 0x002e_e1c8);
}

/// `float WorldPainter_GetSunIntensity(int view)` (0x2ee1d8): sun slot 32.
pub fn world_painter_get_sun_intensity(c: &mut Cpu) {
    sun_float_getter(c, 0x100, 0x002e_e210);
}

/// `float WorldPainter_GetSunSize(int view)` (0x2ee2b0): sun slot 35.
pub fn world_painter_get_sun_size(c: &mut Cpu) {
    sun_float_getter(c, 0x118, 0x002e_e2e8);
}

pub(crate) mod world_painter {

    /// Layer kinds, three bits per layer (layer i at bit 3i).
    pub const LAYER_KINDS: u32 = 0xc;

    /// Rectangle sampled by kind-5 layers: origin x, y, size x, y.
    pub const RECT: u32 = 0x10;

    /// Quad corners (x, y) sampled by kind-1 layers: P0, P1, P2, P3.
    pub const QUAD: u32 = 0x20;

    /// Byte: index into the surface table (8-byte entries).
    pub const SURFACE_INDEX: u32 = 0x155;

    /// Short: index into the material table (0x18-byte entries); negative
    /// means none.
    pub const MATERIAL_INDEX: u32 = 0x156;

    /// One short sampler id per layer.
    pub const LAYER_IDS: u32 = 0x1a0;

    /// Layer kind: sample the rectangle at the clamped (u, v).
    pub const KIND_RECT: u64 = 5;

    /// Layer kind: sample the quad at (u, v), wrapped to the texel fraction.
    pub const KIND_QUAD: u64 = 1;

    /// Surface table entry: kind word (+0x24) that must be 6.
    pub const SURFACE_KIND: u32 = 0x24;

    pub const SURFACE_KIND_PAINTED: u64 = 6;

    /// Material table entry (relative to the table base): kind word that
    /// must be 3.
    pub const MATERIAL_KIND: u32 = 0x3f0;

    pub const MATERIAL_KIND_PAINTED: u64 = 3;

    /// `gp+0x16c8`: pointer to the painter tables (first word: the table base).
    pub const GP_TABLES: u32 = 0x16c8;

    /// `gp-0x854`: the sampler object; its interface at +0x10d8, method slot
    /// +0x1d0 (`{delta, fn}`) samples a colour.
    pub const GP_SAMPLER: i32 = -0x854;

    pub const SAMPLER_IFACE: u32 = 0x10d8;

    pub const SAMPLE_SLOT: u32 = 0x1d0;

    /// Per-channel colour weights (`0x500000 - 0x5500`).
    pub const WEIGHTS_HI: u32 = 0x0050_0000;

    pub const WEIGHTS: u32 = WEIGHTS_HI - 0x5500;

    /// Offset added before splitting the quad sample into texel and fraction.
    pub const TEXEL_BIAS: u32 = 0x447a_0000;
}

/// Unaligned 16-byte copy as gcc emits it (`ldl/ldr` into `ra`, `rb`, then
/// `sdl/sdr`); the registers keep the copied doublewords.
pub(crate) fn copy16_unaligned(c: &mut Cpu, src: u32, dst: u32, ra: usize, rb: usize) {
    c.ldl(ra, src.wrapping_add(7));
    c.ldr(ra, src);
    c.ldl(rb, src.wrapping_add(0xf));
    c.ldr(rb, src.wrapping_add(8));
    c.sdl(ra, dst.wrapping_add(7));
    c.sdr(ra, dst);
    c.sdl(rb, dst.wrapping_add(0xf));
    c.sdr(rb, dst.wrapping_add(8));
}

/// `void WorldPainter_SampleSurfaceColours(WorldPainter* this, Vec4* scale,
/// Vec4* ratio, float u /*f12*/, float v /*f13*/)` (0x002edb20).
///
/// When the painter's surface and material are both of the painted kind,
/// samples up to two colours through the sampler object's virtual method
/// (`gp-0x854`, interface +0x10d8, slot +0x1d0): colour A from the layer of
/// kind 5 (its rectangle at the clamped (u, v)), colour B from the layer of
/// kind 1 (its quad bilinearly at (u, v), passed as the fraction of
/// `x + 1000`). A layer of neither kind leaves the colour at zero; a later
/// layer of the same kind overrides an earlier one. Then writes
/// `scale = (B - A) * weights * 2 A.x` and `ratio = scale / B`, with the
/// x components of both set to 0.5.
pub fn world_painter_sample_surface_colours(c: &mut Cpu) {
    use crate::guest::Addr;
    use crate::ps2f::FpuF32;
    use world_painter as wp;
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x110);
        frame.save(c, S0, 0xe0);
        frame.save(c, S4, 0xa0);
        frame.save(c, S6, 0x80);
        frame.save_fpr(c, 21, 0xf8);
        frame.save_fpr(c, 20, 0xf0);
        frame.save(c, S1, 0xd0);
        frame.save(c, S2, 0xc0);
        frame.save(c, S3, 0xb0);
        frame.save(c, S5, 0x90);
        frame.save(c, S7, 0x70);
        frame.save(c, FP, 0x60);
        frame.save_ra(c, 0x50);
        frame.save_fpr(c, 22, 0x100);
        c.g[S0][0] = c.g[A0][0];
        c.g[S4][0] = c.g[A1][0];
        c.g[S6][0] = c.g[A2][0];
        c.f[21] = c.f[12];
        c.f[20] = c.f[13];
        let u = FpuF32(c.f[21]);
        let v = FpuF32(c.f[20]);
        let this = r32(c, S0);
        let scale_out = r32(c, S4);
        let ratio_out = r32(c, S6);
        // Colour A at sp+0, colour B at sp+0x10; both start at zero.
        for k in 0..8u32 {
            Addr(frame.at(c, 4 * k)).sw(c, 0);
        }
        'body: {
            if c.g[S0][0] == 0 {
                break 'body;
            }

            // Painted surface and painted material?
            c.cycles += 5;
            let tables = Addr(r32(c, GP).wrapping_add(wp::GP_TABLES)).lw(c);
            let material = Addr(this.wrapping_add(wp::MATERIAL_INDEX)).lh(c);
            let base = Addr(tables as u32).lw(c);
            c.g[A1][0] = material;
            c.g[A0][0] = base;
            c.g[V0][0] = 0;
            if (material as i64) >= 0 {
                c.cycles += 7;
                let surface = Addr(this.wrapping_add(wp::SURFACE_INDEX)).lbu(c);
                let entry = sx((base as u32).wrapping_add((surface as u32) << 3));
                let kind = Addr((entry as u32).wrapping_add(wp::SURFACE_KIND)).lw(c);
                c.g[V1][0] = kind ^ wp::SURFACE_KIND_PAINTED;
                if c.g[V1][0] == 0 {
                    c.cycles += 6;
                    c.g[V0][0] = 0x18;
                    let off = crate::abi::mult(c, material as u32, 0x18);
                    c.g[V1][0] = sx(off);
                    let mat = sx(off.wrapping_add(base as u32));
                    let kind = Addr((mat as u32).wrapping_add(wp::MATERIAL_KIND)).lw(c);
                    c.g[V1][0] = kind ^ wp::MATERIAL_KIND_PAINTED;
                    c.g[V0][0] = (c.g[V1][0] < 1) as u64;
                } else {
                    c.g[V0][0] = 0;
                }
            } else {
                c.g[V0][0] = 0;
            }
            c.cycles += 2;
            c.g[FP][0] = wp::WEIGHTS_HI as u64;
            if c.g[V0][0] == 0 {
                break 'body;
            }

            c.cycles += 8;
            c.g[AT][0] = wp::TEXEL_BIAS as u64;
            c.f[22] = wp::TEXEL_BIAS;
            let bias = FpuF32(wp::TEXEL_BIAS);
            c.g[S7][0] = wp::KIND_RECT;
            c.g[S1][0] = sx(this.wrapping_add(wp::LAYER_IDS));
            c.g[S5][0] = 0;
            c.g[S3][0] = 0;
            c.g[S2][0] = 2;
            let rd = |c: &mut Cpu, off: u32| Addr(this.wrapping_add(off)).read_fpu(c);
            loop {
                let shift = r32(c, S3) & 31;
                let layer_id = Addr(r32(c, S1));

                // Kind 5: colour A from the rectangle at the clamped (u, v).
                c.cycles += 5;
                let kinds = Addr(this.wrapping_add(wp::LAYER_KINDS)).lw(c);
                c.g[V0][0] = sx(((kinds as u32) as i32 >> shift) as u32) & 7;
                if c.g[V0][0] == wp::KIND_RECT {
                    c.cycles += 7;
                    c.g[AT][0] = 0x3f80_0000;
                    let one = FpuF32::ONE;
                    let u_ok = FpuF32::ZERO.le(u);
                    crate::abi::set_fcc(c, u_ok);
                    let mut cu = FpuF32::ZERO;
                    if u_ok {
                        c.cycles += 1;
                        cu = u.min(one);
                    }
                    c.cycles += 6;
                    let size_x = rd(c, wp::RECT + 8);
                    let v_ok = FpuF32::ZERO.le(v);
                    crate::abi::set_fcc(c, v_ok);
                    let origin_x = rd(c, wp::RECT);
                    let x = origin_x + size_x * cu;
                    let mut cv = FpuF32::ZERO;
                    if v_ok {
                        c.cycles += 1;
                        cv = v.min(one);
                    }
                    c.cycles += 22;
                    let size_y = rd(c, wp::RECT + 0xc);
                    let origin_y = rd(c, wp::RECT + 4);
                    let y = origin_y + size_y * cv;
                    // Registers at the call.
                    c.f[0] = origin_y.0;
                    c.f[1] = origin_x.0;
                    c.f[2] = cv.0;
                    c.f[3] = one.0;
                    c.f[4] = cu.0;
                    c.f[12] = x.0;
                    c.f[13] = y.0;
                    let sampler = Addr(r32(c, GP).wrapping_add(wp::GP_SAMPLER as u32)).lw(c);
                    let iface = Addr((sampler as u32).wrapping_add(wp::SAMPLER_IFACE)).lw(c);
                    let slot = crate::call::VSlot::read(c, iface, wp::SAMPLE_SLOT);
                    c.g[A0][0] = sx(frame.at(c, 0x20));
                    c.g[T0][0] = sampler;
                    c.g[A3][0] = 1;
                    c.g[V1][0] = iface;
                    c.g[A2][0] = layer_id.lh(c);
                    c.g[V0][0] = slot.func;
                    c.g[A1][0] = slot.this(sampler);
                    call_ptr(c, slot.func as u32, 0x002e_dc84)?;
                    copy16_unaligned(c, frame.at(c, 0x20), frame.at(c, 0), A0, A1);
                }
                c.g[V0][0] = Addr(this.wrapping_add(wp::LAYER_KINDS)).lw(c);

                // Kind 1: colour B from the quad at (u, v).
                c.cycles += 5;
                c.g[V1][0] = 1;
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> (r32(c, S5) & 31)) as u32) & 7;
                if c.g[V0][0] == wp::KIND_QUAD {
                    c.cycles += 56;
                    let p0x = rd(c, wp::QUAD);
                    let p0y = rd(c, wp::QUAD + 4);
                    let p1x = rd(c, wp::QUAD + 8);
                    let p1y = rd(c, wp::QUAD + 0xc);
                    let p2x = rd(c, wp::QUAD + 0x10);
                    let p2y = rd(c, wp::QUAD + 0x14);
                    let p3x = rd(c, wp::QUAD + 0x18);
                    let p3y = rd(c, wp::QUAD + 0x1c);
                    // x: P0->P2 and P1->P3 by u, then between them by v;
                    // y: P0->P1 and P2->P3 by v, then between them by u.
                    let dx02 = (p2x - p0x) * u;
                    let dy01 = (p1y - p0y) * v;
                    let dy23 = (p3y - p2y) * v;
                    let dx13 = (p3x - p1x) * u;
                    let x02 = p0x + dx02;
                    let x13 = p1x + dx13;
                    let y01 = p0y + dy01;
                    let y23 = p2y + dy23;
                    let dx = (x13 - x02) * v;
                    let dy = (y23 - y01) * u;
                    let x = x02 + dx + bias;
                    let y = y01 + dy + bias;
                    let tx = x.to_i32();
                    let ty = y.to_i32();
                    let fx = x - FpuF32::from_i32(tx);
                    let fy = y - FpuF32::from_i32(ty);
                    // Registers at the call.
                    c.f[0] = ty as u32;
                    c.f[1] = dx.0;
                    c.f[2] = y.0;
                    c.f[3] = x.0;
                    c.f[4] = dx02.0;
                    c.f[5] = dy01.0;
                    c.f[6] = dy23.0;
                    c.f[7] = dx13.0;
                    c.f[12] = fx.0;
                    c.f[13] = fy.0;
                    let sampler = Addr(r32(c, GP).wrapping_add(wp::GP_SAMPLER as u32)).lw(c);
                    let iface = Addr((sampler as u32).wrapping_add(wp::SAMPLER_IFACE)).lw(c);
                    let slot = crate::call::VSlot::read(c, iface, wp::SAMPLE_SLOT);
                    c.g[A0][0] = sx(frame.at(c, 0x20));
                    c.g[A3][0] = 1;
                    c.g[A2][0] = layer_id.lh(c);
                    c.g[A1][0] = slot.this(sampler);
                    c.g[T0][0] = slot.func;
                    c.g[V0][0] = sx(tx as u32);
                    c.g[V1][0] = sx(ty as u32);
                    call_ptr(c, slot.func as u32, 0x002e_dd78)?;
                    copy16_unaligned(c, frame.at(c, 0x20), frame.at(c, 0x10), V0, V1);
                }
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(2));

                c.cycles += 4;
                c.g[S5][0] = sx(r32(c, S5).wrapping_add(3));
                c.g[S2][0] = sx(r32(c, S2).wrapping_sub(1));
                c.g[S3][0] = sx(r32(c, S3).wrapping_add(3));
                if (c.g[S2][0] as i64) < 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002e_dc00);
                }
            }

            // scale = (B - A) * weights * 2 A.x; ratio = scale / B.
            c.cycles += 70;
            let sp_rd = |c: &mut Cpu, off: u32| Addr(frame.at(c, off)).read_fpu(c);
            let sp_wr = |c: &mut Cpu, off: u32, v: FpuF32| Addr(frame.at(c, off)).write_fpu(c, v);
            let a = [sp_rd(c, 0), sp_rd(c, 4), sp_rd(c, 8), sp_rd(c, 0xc)];
            let b = [
                sp_rd(c, 0x10),
                sp_rd(c, 0x14),
                sp_rd(c, 0x18),
                sp_rd(c, 0x1c),
            ];
            c.g[V0][0] = sx(wp::WEIGHTS);
            let weights = Addr(wp::WEIGHTS);
            let w = [
                weights.read_fpu(c),
                weights.off(4).read_fpu(c),
                weights.off(8).read_fpu(c),
                weights.off(0xc).read_fpu(c),
            ];
            let two_ax = a[0] + a[0];
            let d = [b[0] - a[0], b[1] - a[1], b[2] - a[2], b[3] - a[3]];
            let dw = [d[0] * w[0], d[1] * w[1], d[2] * w[2], d[3] * w[3]];
            let scale = [
                dw[0] * two_ax,
                dw[1] * two_ax,
                dw[2] * two_ax,
                dw[3] * two_ax,
            ];
            for k in 0..4u32 {
                sp_wr(c, 0x40 + 4 * k, d[k as usize]);
            }
            sp_wr(c, 0x30, dw[0]);
            sp_wr(c, 0x24, scale[1]);
            sp_wr(c, 0x20, scale[0]);
            sp_wr(c, 0x34, dw[1]);
            sp_wr(c, 0x28, scale[2]);
            sp_wr(c, 0x2c, scale[3]);
            sp_wr(c, 0x38, dw[2]);
            sp_wr(c, 0x3c, dw[3]);
            c.g[AT][0] = 0x3f00_0000;
            let half = FpuF32(0x3f00_0000);
            copy16_unaligned(c, frame.at(c, 0x20), scale_out, T1, V0);
            let s = [
                Addr(scale_out).read_fpu(c),
                Addr(scale_out.wrapping_add(4)).read_fpu(c),
                Addr(scale_out.wrapping_add(8)).read_fpu(c),
                Addr(scale_out.wrapping_add(0xc)).read_fpu(c),
            ];
            let ratio = [
                s[0].div(b[0]),
                s[1].div(b[1]),
                s[2].div(b[2]),
                s[3].div(b[3]),
            ];
            for k in 0..4u32 {
                sp_wr(c, 0x20 + 4 * k, ratio[k as usize]);
            }
            copy16_unaligned(c, frame.at(c, 0x20), ratio_out, A1, T1);
            Addr(scale_out).write_fpu(c, half);
            Addr(ratio_out).write_fpu(c, half);
            // FPU registers as the original leaves them.
            c.f[0] = ratio[3].0;
            c.f[1] = ratio[0].0;
            c.f[2] = ratio[1].0;
            c.f[3] = ratio[2].0;
            c.f[4] = half.0;
            c.f[5] = d[3].0;
            c.f[6] = dw[2].0;
            c.f[7] = dw[3].0;
            c.f[8] = scale[2].0;
            c.f[9] = scale[0].0;
            c.f[10] = b[3].0;
            c.f[11] = b[0].0;
            c.f[12] = b[2].0;
            c.f[13] = b[1].0;
        }

        c.cycles += 15;
        for (reg, off) in [
            (S0, 0xe0),
            (S1, 0xd0),
            (S2, 0xc0),
            (S3, 0xb0),
            (S4, 0xa0),
            (S5, 0x90),
            (S6, 0x80),
            (S7, 0x70),
            (FP, 0x60),
        ] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0x50);
        frame.restore_fpr(c, 22, 0x100);
        frame.restore_fpr(c, 21, 0xf8);
        frame.restore_fpr(c, 20, 0xf0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WorldPainter_ComputeColourFactor (0x2edf00).

/// `float WorldPainter_ComputeColourFactor(Colour* col)` (0x2edf00): the
/// luminance `0.299 r + 0.587 g + 0.114 b` of the colour at `+4..+0xc`,
/// mapped to a 0..1 factor: 0 at or below 0.1, 1 at or above 0.45, between
/// them `clamp(exp(log((lum - 0.1) / |0.35|) * 0.6001176), 0, 1)` (a gamma
/// curve).
pub fn world_painter_compute_colour_factor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        let col = r32(c, A0);
        c.lwc1(3, bw::gp(c, -0x395c)); // 0.299
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x10);
        c.lwc1(4, bw::gp(c, -0x3958)); // 0.587
        c.lwc1(1, bw::p(col, 0x4));
        c.lwc1(2, bw::p(col, 0x8));
        c.mul_s(1, 1, 3);
        c.lwc1(0, bw::p(col, 0xc));
        c.mul_s(2, 2, 4);
        c.lwc1(3, bw::gp(c, -0x3954)); // 0.114
        c.lwc1(4, bw::gp(c, -0x3950)); // 0.1
        c.mul_s(0, 0, 3);
        c.add_s(1, 1, 2);
        c.add_s(1, 1, 0);
        c.c_le_s(1, 4); // lum <= 0.1 ?
        c.mtc1(0, 0); // f0 = 0
        let dark = c.fcc();
        frame.restore_ra(c, 0);
        'restore: {
            if dark {
                break 'restore;
            }
            c.cycles += 6;
            c.lwc1(0, bw::gp(c, -0x394c)); // 0.45
            c.c_le_s(0, 1); // 0.45 <= lum ?
            bw::fconst(c, 0, 0x3f80_0000); // 1
            let bright = c.fcc();
            frame.restore_fpr(c, 20, 0x10);
            if bright {
                c.cycles += 2;
                frame.close(c);
                return Ok(());
            }
            // exp(log(t) * k), clamped to [0, 1].
            c.cycles += 17;
            c.sub_s(0, 1, 4);
            c.lwc1(12, bw::gp(c, -0x3948)); // 0.35
            c.abs_s(12, 12);
            c.lwc1(20, bw::gp(c, -0x3944)); // 0.6001176
            c.div_s(12, 0, 12);
            call_ptr(c, 0x0040_da10, 0x002e_df80)?; // logf
            c.g[RA][0] = sx(0x002e_df88);
            c.mul_s(12, 0, 20);
            call_ptr(c, 0x0040_d758, 0x002e_df88)?; // expf
            c.mov_s(1, 0);
            bw::fconst(c, 2, 0x3f80_0000); // 1
            c.mtc1(0, 0); // f0 = 0
            c.c_le_s(0, 1);
            if c.fcc() {
                c.min_s(0, 1, 2);
            }
            c.cycles += 1;
            frame.restore_ra(c, 0);
        }
        c.cycles += 1;
        frame.restore_fpr(c, 20, 0x10);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// IrradianceRef_GetEntry (0x2ee010)

/// The global irradiance table (`D_005047F8`).
pub(crate) const IRRADIANCE_TABLE: u32 = 0x0050_47f8;

/// `IrradianceRef::index` (+0) and `IrradianceRef::key` (+4); a reference is
/// resolved once its key is `RESOLVED`.
pub(crate) const REF_INDEX: u32 = 0x0;

pub(crate) const REF_KEY: u32 = 0x4;

pub(crate) const RESOLVED: u64 = 0x0012_3400;

/// The default irradiance entry index (gp+0x12d4, `D_004A43C4`).
pub(crate) const GP_DEFAULT_IRRADIANCE: u32 = 0x12d4;

pub(crate) const IRRADIANCE_REF_RESOLVE: Callee = Callee {
    addr: 0x002e_dfd0,
    f: ee::s09::IrradianceRef_Resolve,
};

/// `Irradiance_GetEntry(table, index)` (cross-shard: through the table).
pub(crate) const IRRADIANCE_GET_ENTRY: u32 = 0x0038_abf8;

/// `Irradiance* IrradianceRef_GetEntry(IrradianceRef* ref)` (0x2ee010): the
/// irradiance entry a reference names. A resolved reference (key 0x123400)
/// uses its index; an unnamed one (empty name) the default entry; any other
/// is resolved by name first (`IrradianceRef_Resolve`, which writes the
/// index and the key) and then uses its index.
pub fn irradiance_ref_get_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = RESOLVED;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = Addr(c.g[S0][0] as u32);
        c.g[V1][0] = this.off(REF_KEY).lw(c);
        if c.g[V1][0] == RESOLVED {
            c.g[A0][0] = 0x50_0000;
            c.cycles += 1;
            c.g[A1][0] = this.off(REF_INDEX).lw(c);
        } else {
            c.cycles += 3;
            c.g[V0][0] = this.lb(c);
            let gp = c.g[GP][0] as u32;
            c.g[A1][0] = Addr(gp).off(GP_DEFAULT_IRRADIANCE).lw(c);
            if c.g[V0][0] == 0 {
                // Unnamed: the default entry.
                c.cycles += 2;
                c.g[A0][0] = 0x50_0000;
            } else {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                IRRADIANCE_REF_RESOLVE.call(c, 0x002e_e050)?;
                c.g[A0][0] = 0x50_0000;
                c.cycles += 1;
                c.g[A1][0] = this.off(REF_INDEX).lw(c);
            }
        }
        c.cycles += 6;
        c.g[A0][0] = sx(IRRADIANCE_TABLE);
        call_ptr(c, IRRADIANCE_GET_ENTRY, 0x002e_e060)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod painter {

    /// gp+0xa70: weather effects disabled.
    pub const GP_DISABLED: i32 = 0xa70;

    /// The world painter's per-view slots (0xf0 bytes each) at 0x4fa370.
    pub const SLOTS: u32 = 0x0050_0000 - 0x5c90;

    pub const SLOT_SIZE: u64 = 0xf0;

    /// Slot: the weather object handle (`**obj`), the fade (1 = gone).
    pub const SLOT_OBJ: u32 = 0x20;

    pub const SLOT_FADE: u32 = 0x24;

    /// The object's virtual `float *GetWeather08()` (slot +0x158/+0x15c).
    pub const VT_GET: u32 = 0x158;
}

/// `float WorldPainter_GetWeather08(int view)` (0x2ee448): the weather
/// object's value 0x08 for a view (its virtual getter), scaled by
/// `1 - fade`; 0 when weather is disabled.
pub fn world_painter_get_weather08(c: &mut Cpu) {
    use painter::*;
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = gp_at(c, GP_DISABLED).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.mtc1(0, 0);
        } else {
            c.cycles += 19;
            c.g[S0][0] = SLOT_SIZE;
            c.g[V0][0] = 0x0050_0000;
            c.mult(0, S0, A0, S0);
            c.g[V0][0] = sx(SLOTS);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(SLOTS));
            let slot = r32(c, S0);
            c.g[V1][0] = Addr(slot + SLOT_OBJ).lw(c);
            c.g[A2][0] = Addr(r32(c, V1)).lw(c);
            let obj = r32(c, A2);
            c.g[A1][0] = Addr(obj + 4).lw(c);
            let vt = r32(c, A1);
            c.g[A0][0] = Addr(vt + VT_GET).lh(c);
            c.g[V0][0] = Addr(vt + VT_GET + 4).lw(c);
            let f = r32(c, V0);
            c.g[A0][0] = sx(obj.wrapping_add(r32(c, A0)));
            call_ptr(c, f, 0x002e_e498)?;
            let slot = r32(c, S0);
            c.lwc1(2, slot + SLOT_FADE);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 0);
            c.lwc1(1, r32(c, V0));
            c.sub_s(0, 0, 2);
            c.mul_s(0, 1, 0);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float WorldPainter_GetWeather10(int index)` (0x2ee4c0): weather method
/// at v-table +0x160.
pub fn world_painter_get_weather10(c: &mut Cpu) {
    weather_slot_float_getter(c, 0x160, 0x002e_e4f8);
}

// ---------------------------------------------------------------------------
// World painter getters (lensfx.cpp / renderstateman.cpp).

/// The world painter's per-view table (`0x4fa370`, 0xf0 bytes per view).
pub(crate) const WP_TABLE_207: u32 = 0x004f_a370;

/// Stride of one view's entry in [`WP_TABLE_207`].
pub(crate) const WP_STRIDE_207: u64 = 0xf0;

/// The plain float getters of the 0x2ee*** family: `entry = table + index *
/// 0xf0`, `holder = entry->field`, `obj = *holder`, call the vtable slot
/// `vt_off` (`{delta, fn}`) with `obj + delta` and return the float its
/// result points at (in f0).
pub(crate) fn world_painter_float_getter_207(
    c: &mut Cpu,
    table_off: u32,
    vt_off: u32,
    ret_to: u32,
) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[V1][0] = WP_STRIDE_207;
        let frame = Frame::open(c, 0x10);
        c.mult(0, A0, A0, V1); // a0 = index * 0xf0
        c.g[V0][0] = 0x50_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(WP_TABLE_207);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
        c.g[V1][0] = Addr(r32(c, A0)).off(table_off).lw(c);
        c.g[A2][0] = Addr(r32(c, V1)).lw(c);
        c.g[A1][0] = Addr(r32(c, A2)).off(4).lw(c);
        c.g[A0][0] = Addr(r32(c, A1)).off(vt_off).lh(c);
        c.g[V0][0] = Addr(r32(c, A1)).off(vt_off + 4).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
        call_ptr(c, target, ret_to)?;
        let p = r32(c, V0);
        c.lwc1(0, p);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float WorldPainter_GetWeather18(int index)` (0x2ee508): the weather
/// block's value (vtable slot 0x168 of the weather holder at `entry+0x20`)
/// faded by the view's weather fade at `entry+0x24`:
/// `*get() * (1.0 - fade)`. The entry pointer lives in s0 across the call.
pub fn world_painter_get_weather18(c: &mut Cpu) {
    const RET: u32 = 0x002e_e544;
    /// The view's weather holder.
    const WEATHER: u32 = 0x20;
    /// The view's weather fade (0 = full weather).
    const FADE: u32 = 0x24;
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = WP_STRIDE_207;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = 0x50_0000;
        c.mult(0, S0, A0, V1); // s0 = index * 0xf0
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(WP_TABLE_207);
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
        c.g[V1][0] = Addr(r32(c, S0)).off(WEATHER).lw(c);
        c.g[A2][0] = Addr(r32(c, V1)).lw(c);
        c.g[A1][0] = Addr(r32(c, A2)).off(4).lw(c);
        c.g[A0][0] = Addr(r32(c, A1)).off(0x168).lh(c);
        c.g[V0][0] = Addr(r32(c, A1)).off(0x16c).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
        call_ptr(c, target, RET)?;
        let entry = r32(c, S0);
        c.lwc1(2, entry.wrapping_add(FADE));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0); // f0 = 1.0
        let p = r32(c, V0);
        c.lwc1(1, p);
        c.f[0] = (FpuF32(c.f[0]) - FpuF32(c.f[2])).0;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        c.f[0] = (FpuF32(c.f[1]) * FpuF32(c.f[0])).0;
        frame.close(c);
        Ok(())
    })
}

/// `float WorldPainter_GetWeather60(int index)` (0x2ee780): the weather
/// holder (`entry+0x20`), vtable slot 0x1b0.
pub fn world_painter_get_weather60(c: &mut Cpu) {
    world_painter_float_getter_207(c, 0x20, 0x1b0, 0x002e_e7b8);
}

// ------------------------------- WorldPainter_GetLightGlow18 / GetWeather58

/// The world painter's per-world slots (0xf0 bytes each).
pub(crate) const WP_SLOTS: u32 = 0x004f_a370;

pub(crate) const WP_SLOT_SIZE: u32 = 0xf0;

/// The shape both getters share: `float WorldPainter_GetX(int world)` =
/// `*(**(slots[world] + field))->vt[method]()`, where the method returns a
/// `float*`. `vslot` is the method's `{delta: i16, fn}` offset in the
/// v-table at +4; `ret` the call's return address.
pub(crate) fn world_painter_slot_float(c: &mut Cpu, field: u32, vslot: u32, ret_addr: u32) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[V1][0] = WP_SLOT_SIZE as u64;
        let frame = Frame::open(c, 0x10);
        let world = r32(c, A0);
        let offset = crate::abi::mult(c, world, WP_SLOT_SIZE);
        c.g[V0][0] = sx(WP_SLOTS);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(offset.wrapping_add(WP_SLOTS));
        c.g[V1][0] = Addr(offset.wrapping_add(WP_SLOTS).wrapping_add(field)).lw(c);
        c.g[A2][0] = Addr(r32(c, V1)).lw(c);
        c.g[A1][0] = Addr(r32(c, A2).wrapping_add(4)).lw(c);
        let delta = Addr(r32(c, A1).wrapping_add(vslot)).lh(c);
        c.g[V0][0] = Addr(r32(c, A1).wrapping_add(vslot + 4)).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A2).wrapping_add(delta as u32));
        call_ptr(c, target, ret_addr)?;
        c.lwc1(0, r32(c, V0));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float WorldPainter_GetWeather58(int world)` (0x2ee738): a weather value
/// of the world's lens-fx object (slot +0x20, method at v-table +0x1a8).
pub fn world_painter_get_weather58(c: &mut Cpu) {
    world_painter_slot_float(c, 0x20, 0x1a8, 0x002e_e770);
}

pub(crate) const LENS_FX_SLOT_SIZE: u64 = 0xf0;

/// `gp + 0xa70`: non-zero while the weather lens effects are off.
pub(crate) const LENS_FX_WEATHER_OFF: u32 = 0xa70;

/// `float WorldPainter_GetWeather48(int i)` (0x2ee600): 0.0 while the weather
/// effects are off (`gp + 0xa70`), else the value the lens-effect slot `i`'s
/// object returns through its virtual accessor (vtable slot 0x198/0x19c, a
/// pointer to the float).
pub fn world_painter_get_weather48(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(r32(c, GP).wrapping_add(LENS_FX_WEATHER_OFF)).lw(c);
        let off = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if off {
            c.cycles += 3;
            c.mtc1(0, 0); // f0 = 0.0
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 14;
            c.g[V1][0] = LENS_FX_SLOT_SIZE;
            c.g[V0][0] = 0x0050_0000;
            c.mult(0, V1, A0, V1); // v1 = i * 0xf0
            c.g[V0][0] = sx(LENS_FX_SLOTS);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(LENS_FX_SLOTS));
            c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x20)).lw(c); // s->p20
            c.g[A1][0] = Addr(r32(c, A0)).lw(c); // obj = *s->p20
            c.g[V1][0] = Addr(r32(c, A1).wrapping_add(4)).lw(c); // obj->vt
            c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x198)).lh(c); // delta
            c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x19c)).lw(c); // fn
            let target = r32(c, V0);
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            call_ptr(c, target, 0x002e_e64c)?;
            let p = r32(c, V0);
            c.lwc1(0, p);
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WorldPainter_GetLightGlow10 (0x2eedf8), WorldPainter_GetWeather40 (0x2ee6a8)

/// The shared body of the 18-cycle lens-slot accessors: `slot = &D_004FA370[i]`,
/// `obj = *slot->field`, return `*obj->vt[vt_off].fn(obj + delta)` in f0.
pub(crate) fn lens_slot_virtual_float(c: &mut Cpu, field: u32, vt_off: u32, ret_addr: u32) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[V1][0] = LENS_FX_SLOT_SIZE;
        let frame = Frame::open(c, 0x10);
        c.mult(0, A0, A0, V1); // a0 = i * 0xf0
        c.g[V0][0] = 0x0050_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(LENS_FX_SLOTS);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(LENS_FX_SLOTS));
        c.g[V1][0] = Addr(r32(c, A0).wrapping_add(field)).lw(c);
        c.g[A2][0] = Addr(r32(c, V1)).lw(c); // obj
        c.g[A1][0] = Addr(r32(c, A2).wrapping_add(4)).lw(c); // obj->vt
        c.g[A0][0] = Addr(r32(c, A1).wrapping_add(vt_off)).lh(c); // delta
        c.g[V0][0] = Addr(r32(c, A1).wrapping_add(vt_off + 4)).lw(c); // fn
        let target = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
        call_ptr(c, target, ret_addr)?;
        let p = r32(c, V0);
        c.lwc1(0, p);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float WorldPainter_GetWeather40(int i)` (0x2ee6a8): the weather value of
/// lens slot `i` (`*slot->p20`, vtable slot 0x190/0x194).
pub fn world_painter_get_weather40(c: &mut Cpu) {
    lens_slot_virtual_float(c, 0x20, 0x190, 0x002e_e6e0);
}

// ---------------------------------------------------------------------------
// WorldPainter_GetLightGlow08 (0x2eedb0), WorldPainter_GetWeather50 (0x2ee660)

/// The world painter view table `D_004FA370` (built as `0x500000 - 0x5c90`),
/// 0xf0 bytes per view; each view holds pointers to interface holders
/// (`holder -> object`, the object's vtable at +4).
pub(crate) const WP_GET_VIEWS: u32 = 0x0050_0000;

pub(crate) const WP_GET_VIEWS_NEG: u32 = 0x5c90;

pub(crate) const WP_GET_VIEW_SIZE: u64 = 0xf0;

pub(crate) const WP_VIEW_WEATHER: u32 = 0x20;

/// Object vtable pointer offset.
pub(crate) const WP_OBJ_VTABLE: u32 = 0x4;

/// The shared body of the world painter's float getters: call the virtual
/// method whose `{this delta, target}` pair is at `vslot` in the vtable of
/// the object held at `holder_off` in view `i` (a0), and return the float
/// at the pointer it returns. Frame 0x10 (ra at 0).
pub(crate) fn wp_view_get_float(c: &mut Cpu, holder_off: u32, vslot: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[V1][0] = WP_GET_VIEW_SIZE;
        let frame = Frame::open(c, 0x10);
        c.mult(0, A0, A0, V1);
        c.g[V0][0] = sx(WP_GET_VIEWS);
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(WP_GET_VIEWS.wrapping_sub(WP_GET_VIEWS_NEG));
        let view = Addr(r32(c, A0).wrapping_add(r32(c, V0)));
        c.g[A0][0] = sx(view.0);
        let holder = view.off(holder_off).lw(c);
        c.g[V1][0] = holder;
        let obj = Addr(holder as u32).lw(c);
        c.g[A2][0] = obj;
        let vtable = Addr(obj as u32).off(WP_OBJ_VTABLE).lw(c);
        c.g[A1][0] = vtable;
        let delta = Addr(vtable as u32).off(vslot).lh(c);
        let target = Addr(vtable as u32).off(vslot + 4).lw(c);
        c.g[V0][0] = target;
        c.g[A0][0] = sx((obj as u32).wrapping_add(delta as u32));
        call_ptr(c, target as u32, ret)?;
        let v = Addr(r32(c, V0)).read_fpu(c);
        c.f[0] = v.0;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float WorldPainter_GetWeather50(int view)` (0x2ee660): the weather
/// interface's float from virtual slot 0x1a0 of view `view`.
pub fn world_painter_get_weather50(c: &mut Cpu) {
    wp_view_get_float(c, WP_VIEW_WEATHER, 0x1a0, 0x002e_e698);
}

// ---------------------------------------------------------------------------
// 0x2ee978 WorldPainter_GetWeather98, 0x2ee5b8 WorldPainter_GetWeather38

/// The world painters' weather getters: `entry = &D_004FA370[index]`
/// (0xf0 bytes each), `obj = *entry->weather` (+0x20), call the weather
/// object's virtual method at v-table `vt_off` (`{delta: i16, fn}`, `this +
/// delta`) through the function table and return the float it points at.
pub(crate) fn world_painter_weather_float_getter(c: &mut Cpu, vt_off: u32, ret_addr: u32) {
    const WEATHER_SLOT: u32 = 0x20;
    guest(c, |c| {
        c.cycles += 18;
        c.g[V1][0] = 0xf0;
        let frame = Frame::open(c, 0x10);
        c.mult(0, A0, A0, V1); // a0 = index * 0xf0
        c.g[V0][0] = 0x50_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(0x50_0000u32.wrapping_sub(0x5c90));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
        c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(WEATHER_SLOT));
        c.g[A2][0] = c.lw(r32(c, V1));
        c.g[A1][0] = c.lw(r32(c, A2).wrapping_add(4));
        c.g[A0][0] = c.lh(r32(c, A1).wrapping_add(vt_off));
        c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(vt_off + 4));
        let method = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
        call_ptr(c, method, ret_addr)?;
        c.lwc1(0, r32(c, V0));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float WorldPainter_GetWeather38(int index)` (0x2ee5b8): weather
/// method at v-table +0x188.
pub fn world_painter_get_weather38(c: &mut Cpu) {
    world_painter_weather_float_getter(c, 0x188, 0x002e_e5f0);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;

    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::fill;

    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::control::handplantcontrol::tests::SEEDS_2;
    pub(crate) use crate::game::bx::appman::tests::rng;
    pub(crate) use crate::game::bx::appman::tests::SEEDS;
    pub(crate) use crate::game::bx::bxstring::tests::heap_init_4 as heap_init;
    pub(crate) use crate::game::bx::bxstring::tests::pool;
    pub(crate) use crate::game::bx::bxstring2::tests::game;

    pub(crate) use crate::game::bx::bxstring2::tests::SCRATCH;

    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_3;

    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_7 as be_library;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot_9 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;

    pub(crate) use crate::game::fe::feasyncfile::tests::world_4 as world;

    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH as SCRATCH_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13 as SEEDS_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_5;

    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;

    pub(crate) use crate::game::input::inputmap::tests::alloc;
    pub(crate) use crate::game::render::particle::tests::f;
    pub(crate) use crate::game::render::particle::tests::gp_at;
    pub(crate) use crate::game::render::particle::tests::put_vec3;
    pub(crate) use crate::game::render::particle::tests::uf;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals;

    pub(crate) use crate::game::sound::bankmonitor::tests::difftest_gp;
    use ssx3_ee::s09;

    #[test]
    fn world_painter_get_view_irradiance_matches() {
        check(
            s09::f_002edfb8__generated,
            world_painter_get_view_irradiance,
            SEEDS,
            &|c, seed| {
                let mut r = rng(seed, 3);
                scramble(c, &mut r);
                c.g[A0][0] = (match seed % 4 {
                    0 => 0,
                    1 => 1,
                    2 => u32::MAX,
                    _ => r.next_u32(),
                }) as u64;
            },
        );
    }

    #[test]
    fn bolts_get_char_bolts_matches() {
        check_lohi(
            ssx3_ee::s01::f_0014d998,
            bolts_get_char_bolts,
            300,
            &|c, seed| {
                let mut r = Rng(0x3000_0003 ^ seed as u64);
                scramble(c, &mut r);
                let idx = seed % 4;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = idx as u64;
                let record = DATA + idx * 4;
                let count = if seed % 2 == 0 {
                    r.next_u32() | 0x8000_0000
                } else {
                    (r.next_u32() & 0x7fff_ffff) | 1
                };
                c.bus.write32(record + 0x2c, count);
                c.bus.write32(record + 0xa4, r.next_u32());
                c.bus.write32(DATA + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn be_get_char_profile_data_matches() {
        check_lohi(
            ssx3_ee::s01::f_0014ad28,
            be_get_char_profile_data,
            200,
            &|c, seed| {
                let mut r = Rng(0x14ad ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = (r.next_u32() % 16) as u64;
                c.g[A2][0] = (r.next_u32() % 16) as u64;
            },
        );
    }

    #[test]
    fn bolts_get_db_matches() {
        check(ssx3_ee::s01::f_0014bdb8, bolts_get_db, 32, &|c, seed| {
            let mut r = Rng(0x14bd ^ seed as u64);
            scramble(c, &mut r);
        });
    }

    #[test]
    fn world_painter_update_view_matches() {
        check(
            ee::s09::f_002ed490__generated,
            world_painter_update_view,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ed4, 12);
                let view = seed % 7;
                // The nine painters of every view: virtual 0x10 is a fake.
                let vt = DATA + 0x500;
                c.bus.write16(vt + 0x10, 0);
                c.bus.write32(vt + 0x14, FAKE);
                for v in 0..6u32 {
                    let rec = WP_VIEWS + WP_VIEW_SIZE * v;
                    for i in 0..9u32 {
                        let painter = DATA + 0x600 + 0x10 * i;
                        c.bus.write32(painter + 4, vt);
                        c.bus.write32(rec + 4 * i, painter);
                    }
                    c.bus.write32(rec + 0x24, f(uf(&mut r, 0.0, 1.0)));
                    fill(c, &mut r, rec + 0x50, 0x28);
                }
                c.f[12] = f(uf(&mut r, -100.0, 100.0));
                c.f[13] = f(uf(&mut r, -100.0, 100.0));
                c.f[14] = f(uf(&mut r, -100.0, 100.0));
                // World, sections, riders.
                let (world, x, y, z, sections, riders, rider, board) = (
                    DATA + 0x1_0000,
                    DATA + 0x1_0100,
                    DATA + 0x1_0200,
                    DATA + 0x1_0300,
                    DATA + 0x1_0400,
                    DATA + 0x1_0500,
                    DATA + 0x1_1000,
                    DATA + 0x1_2000,
                );
                c.bus.write32(gp_at(0xa6c), seed / 7 % 2);
                c.bus.write32(gp_at(0xa74), 0);
                c.bus.write32(gp_at(0xa7c), r.next_u32() % 3);
                c.bus.write32(gp_at(-0x848), world);
                c.bus.write32(world + 0x84, x);
                c.bus.write32(x + 0x78, y);
                c.bus.write32(y + 0x1bc, r.next_u32() % 4);
                c.bus.write32(x + 0x10, z);
                c.bus.write32(z, sections);
                for i in 0..16 {
                    c.bus.write32(
                        sections + 8 * i + 0x24,
                        if r.next_u32() % 2 == 0 { 6 } else { 2 },
                    );
                }
                c.bus.write32(x + 0xc, riders);
                for v in 0..6u32 {
                    c.bus.write32(riders + 4 * v + 0x28, rider);
                }
                for off in [-0x3974, -0x3970, -0x396c, -0x3968, -0x3964, -0x3960] {
                    c.bus.write32(gp_at(off), f(uf(&mut r, 0.0, 1.0)));
                }
                c.bus.write32(gp_at(0xa80), f(uf(&mut r, 0.0, 1.0)));
                c.bus.write32(rider + 0x3fc, r.next_u32() % 2);
                let surface = if r.next_u32() % 4 == 0 {
                    u32::MAX
                } else {
                    (r.next_u32() % 2) << 8 | r.next_u32() % 2
                };
                c.bus.write32(rider + 0x430, surface);
                c.bus.write32(rider + 0x788, board);
                c.bus.write32(rider + 0x77c, DATA + 0x1_1800);
                c.bus.write32(
                    DATA + 0x1_1800 + 0xde0,
                    [1u32, 4, 2][(seed / 2 % 3) as usize],
                );
                put_vec3(c, &mut r, rider + 0xaac, -50.0, 50.0);
                c.bus
                    .write32(board + 0xac, [1u32, 3, 2][(seed / 3 % 3) as usize]);
                c.bus.write32(
                    board + 0x30,
                    if seed % 5 == 0 {
                        u32::MAX
                    } else {
                        surface & 0x1ff
                    },
                );
                let span = uf(&mut r, 0.0, 12.0);
                c.bus.write32(board + 0x98, f(span));
                c.bus.write32(board + 0xa0, f(uf(&mut r, -2.0, span)));
                c.bus.write32(board + 0x34, f(uf(&mut r, -50.0, 50.0)));
                c.bus.write32(board + 0x38, f(uf(&mut r, -50.0, 50.0)));
                // Painter layers (gp+0x16c8): layer table, tiles, tile types.
                let (w2, layer_list, table, layer, tiles, tile) = (
                    DATA + 0x1_3000,
                    DATA + 0x1_3100,
                    DATA + 0x1_3800,
                    DATA + 0x1_3900,
                    DATA + 0x1_3a00,
                    DATA + 0x1_4000,
                );
                c.bus.write32(gp_at(0x16c8), w2);
                c.bus.write32(w2, layer_list);
                c.bus.write32(layer_list + 8, table);
                for i in 0..2 {
                    c.bus
                        .write32(table + 4 * i, if r.next_u32() % 4 == 0 { 0 } else { layer });
                    c.bus.write32(
                        tiles + 4 * i,
                        if r.next_u32() % 4 == 0 {
                            0
                        } else {
                            (tile >> 2) << 8
                        },
                    );
                }
                c.bus.write32(layer + 0xc, tiles);
                c.bus.write16(
                    tile + 0x156,
                    if r.next_u32() % 4 == 0 {
                        0xffff
                    } else {
                        (r.next_u32() % 3) as u16
                    },
                );
                c.bus.write8(tile + 0x155, (r.next_u32() % 3) as u8);
                for i in 0..3 {
                    c.bus.write32(
                        layer_list + 8 * i + 0x24,
                        if r.next_u32() % 2 == 0 { 6 } else { 1 },
                    );
                    c.bus.write32(
                        layer_list + 0x18 * i + 0x3f0,
                        if r.next_u32() % 2 == 0 { 3 } else { 0 },
                    );
                }
                c.g[A0][0] = view as u64;
            },
        );
    }

    #[test]
    fn irradiance_ref_resolve_matches() {
        check(
            ssx3_ee::s09::f_002edfd0__generated,
            irradiance_ref_resolve,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2edf_d0, 0);
                // The manager's entry count at D_005047F8+4 is <= 0, so the lookup
                // callee returns 0 without walking its table.
                c.bus.write32(0x0050_47f8 + 4, 0);
                c.g[A0][0] = sx(DATA + (r.next_u32() & 0xfff0));
            },
        );
    }

    /// `WorldPainter_GetIrradianceNameN` returns the static name buffer while
    /// its cache word is set, which keeps its vtable path out of the test.
    pub(crate) fn irradiance_fixture(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_add(0xa74), 1);
    }

    #[test]
    fn world_painter_get_irradiance0_matches() {
        check(
            ee::s09::f_002ee318__generated,
            world_painter_get_irradiance0,
            SEEDS_2,
            &|c, seed| {
                let mut r = Rng(0x002e_e318 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                irradiance_fixture(c);
            },
        );
    }

    #[test]
    fn world_painter_get_irradiance1_matches() {
        check(
            ee::s09::f_002ee340__generated,
            world_painter_get_irradiance1,
            SEEDS_2,
            &|c, seed| {
                let mut r = Rng(0x002e_e340 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                irradiance_fixture(c);
            },
        );
    }

    // --------------------------------------------------------------- bolt-on FX

    #[test]
    fn c_bolton_fx_interface_update_matches() {
        check(
            ee::s09::f_002ecf78__generated,
            c_bolton_fx_interface_update,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0xcf78, 0);
                let selfp = DATA + 0x1000;
                c.g[A0][0] = selfp as u64;
                // The refresh walks the interface fields; a detached interface
                // (all zero past `+0`) keeps it on its short path.
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                // `BEBoltInt_GetInstance` returns this cached object, so the
                // refresh does not allocate.
                c.bus.write32(GAME_GP.wrapping_sub(0x1f28), DATA + 0x200);
                c.bus.write32(selfp, if seed % 3 == 0 { 0 } else { 1 });
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn world_painter_sun_flare_intensity_matches() {
        covered(
            ssx3_ee::s09::f_002ee268__generated,
            world_painter_sun_flare_intensity,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 47, 1);
                let i = r.next_u32() % 3;
                let slot = 0x004f_a370 + i * 0xf0;
                let cell = DATA + 0x400 + i * 0x100;
                let obj = DATA + 0x800 + i * 0x100;
                let vt = vtable(c, &mut r, &[0x110]);
                c.bus.write32(obj, vt);
                c.bus.write32(cell, obj);
                c.bus.write32(slot + 0x14, cell);
                c.g[A0][0] = sx(i);
            },
        );
    }

    #[test]
    fn world_painter_lerp_view_colours_matches() {
        check(
            ee::s09::f_002ed338__generated,
            world_painter_lerp_view_colours,
            120,
            &|c, seed| {
                let mut r = Rng(0x2f_ed33 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                c.g[A2][0] = sx(DATA + 0x140);
                for i in 0..8 {
                    c.bus.write32(DATA + 0x28 + i * 4, r.edge_f32());
                    c.bus.write32(DATA + 0x100 + i * 4, r.edge_f32());
                    c.bus.write32(DATA + 0x140 + i * 4, r.edge_f32());
                }
                c.f[12] = r.edge_f32();
            },
        );
    }

    #[test]
    fn irradiance_get_override_matches() {
        check(
            ee::s09::f_002ee2f8__generated,
            irradiance_get_override,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2e_e2f8, 0);
                let entry = GAME_GP.wrapping_add(0x24c0);
                match seed % 3 {
                    0 => {}
                    1 => c.bus.write32(entry + 4, 0x0012_3400),
                    _ => {
                        c.bus.write32(entry + 4, 0x0012_3400);
                        c.bus.write8(entry, 1);
                        c.bus.write32(entry, 0x0100_0001);
                    }
                }
                c.bus.write32(entry + 8, r.next_u32() % 0x100);
            },
        );
    }

    #[test]
    fn world_painter_get_irradiance2_matches() {
        check(
            ee::s09::f_002ee368__generated,
            world_painter_get_irradiance2,
            80,
            &|c, seed| {
                let _r = game(c, seed, 0x2ee368, 2);
                // A non-null painter singleton makes the name lookup return the
                // static name at `gp + 0x24c0`; the reference lookup then resolves
                // that name.
                c.bus.write32(difftest_gp() + 0xa74, SCRATCH);
                c.g[A0][0] = (seed * 4) as u64;
            },
        );
    }

    #[test]
    fn c_bolt_on_fx_interface_clear_emitters_matches() {
        check(
            ee::s09::f_002ecaf0__generated,
            c_bolt_on_fx_interface_clear_emitters,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2eca_f0, 0);
                let fx = DATA + 0x100;
                c.g[A0][0] = sx(fx);
                fill(c, &mut r, fx, 0x100);
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x2ee6f0 WorldPainter_GetWeather30.

    #[test]
    fn world_painter_get_weather30_matches() {
        check(
            ee::s09::f_002ee6f0__generated,
            world_painter_get_weather30,
            32,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ee6_f0, 1);
                let i = seed % 8;
                c.g[A0][0] = i as u64;
                let slot = 0x004f_a370 + 0xf0 * i;
                let obj = DATA + 0x200;
                let vt = vtable(c, &mut r, &[0x180]);
                c.bus.write32(slot + 0x20, DATA + 0x100);
                c.bus.write32(DATA + 0x100, obj);
                c.bus.write32(obj + 4, vt);
            },
        );
    }

    // ------------------------------------------------------------- FX / emitters

    #[test]
    fn c_bolt_on_fx_interface_find_or_create_emitter_matches() {
        check(
            ssx3_ee::s09::f_002ecbc0__generated,
            c_bolt_on_fx_interface_find_or_create_emitter,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x2ecbc0);
                c.g[A0][0] = DATA as u64;
                let a1 = r.next_u32();
                let a2 = r.next_u32();
                c.g[A1][0] = (if seed % 2 == 0 { a1 } else { r.next_u32() }) as u64;
                c.g[A2][0] = a2 as u64;
                // Slot 0 always matches (id = a2, kind = a1); the others do not,
                // so both the found and the create paths are visited.
                c.bus
                    .write32(DATA + 0x10, if seed % 3 == 0 { 0xffff_ffff } else { 5 });
                c.bus.write32(DATA + 0x14, a2);
                c.bus.write32(DATA + 0x18, a1);
                for i in 1..15u32 {
                    c.bus.write32(DATA + 0x18 + 0xc * i - 8, 0xffff_ffff);
                }
            },
        );
    }

    // ------------------------------------------------------------------
    // Bolt-on effects.

    #[test]
    fn c_bolt_on_fx_interface_ctor_matches() {
        check(
            ssx3_ee::s09::f_002ec9e0__generated,
            c_bolt_on_fx_interface_ctor,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ec9e0, 0);
                be_library(c);
                world(c);
                let obj = DATA + 0x100;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn c_bolt_on_fx_interface_ctor_2_matches() {
        check(
            ssx3_ee::s09::f_002eca68__generated,
            c_bolt_on_fx_interface_ctor_2,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2eca68, 0);
                be_library(c);
                world(c);
                let obj = DATA + 0x100;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn c_bolt_on_fx_interface_create_emitter_matches() {
        check(
            ee::s09::f_002ecc28__generated,
            c_bolt_on_fx_interface_create_emitter,
            90,
            &|c, seed| {
                let _ = base(c, seed, 0x2e_cc28);
                globals(c);
                pool(c);
                heap_init(c);
                let mut free_first = seed % 3 == 0;
                for i in 0..15u32 {
                    let id = if free_first && i == 0 { u64::MAX } else { 0 };
                    if i == 3 {
                        free_first = seed % 5 == 0;
                    }
                    w(c, DATA + 0x10 + 0xc * i, id as u32);
                }
                w(c, DATA + 0xc, DATA + 0x8000);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.g[A2][0] = sx(DATA + 0x3000);
            },
        );
    }

    #[test]
    fn bolt_on_fx_interface_dtor_matches() {
        check(
            ee::s09::f_002ecb28__generated,
            bolt_on_fx_interface_dtor,
            SEEDS_4,
            &alloc_base_setup(|c, seed, _r| {
                let free_self = seed & 1;
                let this = if free_self != 0 {
                    alloc(c, 0x200)
                } else {
                    DATA
                };
                w(c, this + 0xc, DATA + 0x800);
                for i in 0..15u32 {
                    w(
                        c,
                        this + 0x10 + 0xc * i,
                        if (seed >> (i % 8)) & 1 == 0 {
                            0xffff_ffff
                        } else {
                            0x1
                        },
                    );
                }
                c.g[A0][0] = this as u64;
                c.g[A1][0] = free_self as u64;
            }),
        );
    }

    // ------------------------------------------------------- world painter

    #[test]
    fn world_painter_free_view_queries_matches() {
        check(
            ee::s09::f_002ecfa0__generated,
            world_painter_free_view_queries,
            SEEDS_5,
            &|c, seed| {
                let _ = base_3(c, seed, 0x2ecf_a0, 4);
                w(c, GAME_GP + 0xa64, if seed % 3 == 0 { 0 } else { 1 });
                // The view-query table: 8 painters x (1 + 8) entries.
                for i in 0..0x60u32 {
                    w(c, 0x009f_a370 + 4 * i, 0);
                }
                if seed % 2 == 1 {
                    let obj = SCRATCH_2 + 0x1000;
                    w(c, 0x009f_a370, obj);
                    w(c, obj + 4, obj + 0x1100);
                    slot_2(c, obj + 0x1100, 8, 0);
                    slot_2(c, obj + 0x1100, 0xc, 1);
                }
            },
        );
    }

    use ssx3_ee_rt::cpu::sx;
    use ssx3_ee_rt::Cpu;

    /// The painter view table entry `view`: its sun holder -> object ->
    /// vtable whose slots `slots` are fakes (FAKE + 0x10 i).
    pub(crate) fn painter_sun(
        c: &mut Cpu,
        r: &mut Rng,
        view: u32,
        holder: u32,
        vt: u32,
        slots: &[u32],
    ) {
        let entry = 0x004f_a370u32.wrapping_add(view.wrapping_mul(0xf0));
        let obj = holder + 0x40;
        c.bus.write32(entry + 0x14, holder);
        c.bus.write32(holder, obj);
        c.bus.write32(obj + 4, vt);
        for (i, &off) in slots.iter().enumerate() {
            let delta = (r.next_u32() as i16 as i32 % 64) & !3;
            c.bus.write16(vt + off, delta as u16);
            c.bus.write32(vt + off + 4, FAKE + 0x10 * i as u32);
        }
    }

    pub(crate) fn sun_getter_setup(c: &mut Cpu, seed: u32, vt_off: u32) {
        let mut r = machine(c, seed, 0x5375_6e00 ^ vt_off as u64, 1);
        let view = seed % 8;
        c.g[A0][0] = view as u64 | (r.next_u32() as u64) << 32;
        painter_sun(c, &mut r, view, DATA + 0x200, DATA + 0x6000, &[vt_off]);
    }

    macro_rules! sun_getter_test {
        ($name:ident, $gen:path, $hand:path, $off:expr) => {
            #[test]
            fn $name() {
                covered($gen, $hand, 160, &|c, seed| sun_getter_setup(c, seed, $off));
            }
        };
    }

    sun_getter_test!(
        world_painter_get_sun_elevation_matches,
        ssx3_ee::s09::f_002ee070__generated,
        super::world_painter_get_sun_elevation,
        0xd8
    );

    sun_getter_test!(
        world_painter_get_sun_azimuth_matches,
        ssx3_ee::s09::f_002ee0b8__generated,
        super::world_painter_get_sun_azimuth,
        0xe0
    );

    sun_getter_test!(
        world_painter_get_sun_red_matches,
        ssx3_ee::s09::f_002ee100__generated,
        super::world_painter_get_sun_red,
        0xe8
    );

    sun_getter_test!(
        world_painter_get_sun_green_matches,
        ssx3_ee::s09::f_002ee148__generated,
        super::world_painter_get_sun_green,
        0xf0
    );

    sun_getter_test!(
        world_painter_get_sun_blue_matches,
        ssx3_ee::s09::f_002ee190__generated,
        super::world_painter_get_sun_blue,
        0xf8
    );

    sun_getter_test!(
        world_painter_get_sun_intensity_matches,
        ssx3_ee::s09::f_002ee1d8__generated,
        super::world_painter_get_sun_intensity,
        0x100
    );

    sun_getter_test!(
        world_painter_get_sun_size_matches,
        ssx3_ee::s09::f_002ee2b0__generated,
        super::world_painter_get_sun_size,
        0x118
    );

    /// `WorldPainter_SampleSurfaceColours` against the generated body. The
    /// painter tables (`gp+0x16c8`) make the surface/material gate pass or
    /// fail (null painter, negative material, wrong surface or material
    /// kind); the three layer kinds are random among rect (5), quad (1) and
    /// neither, so each seed samples zero to three colours through the
    /// sampler's virtual method (`gp-0x854`, interface +0x10d8, slot +0x1d0,
    /// a fake that sometimes unwinds). (u, v) cover below 0, inside and
    /// above 1 (the rectangle clamps); the output vectors are sometimes only
    /// word-aligned (the `sdl/sdr` copies).
    #[test]
    fn world_painter_sample_surface_colours_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        use crate::difftest::{DATA, GAME_GP};
        let gp_at = |off: i32| (GAME_GP as i32 + off) as u32;
        covered(
            ee::s09::f_002edb20__generated,
            world_painter_sample_surface_colours,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2edb20, 6);
                c.pc = 0x002e_db20;
                let this = DATA + 0x1000;
                let tables = DATA + 0x2000;
                let base = DATA + 0x3000;
                let sampler = DATA + 0x6000;
                // The gate: surface entry kind 6, material entry kind 3.
                c.bus
                    .write32(gp_at(world_painter::GP_TABLES as i32), tables);
                c.bus.write32(tables, base);
                let surface = r.next_u32() % 4;
                let material = r.next_u32() % 4;
                c.bus
                    .write8(this + world_painter::SURFACE_INDEX, surface as u8);
                let gate = r.next_u32() % 8;
                let mat_index = if gate == 0 {
                    0x8000 | material
                } else {
                    material
                };
                c.bus
                    .write16(this + world_painter::MATERIAL_INDEX, mat_index as u16);
                for k in 0..4 {
                    c.bus.write32(base + 8 * k + world_painter::SURFACE_KIND, 6);
                    c.bus
                        .write32(base + 0x18 * k + world_painter::MATERIAL_KIND, 3);
                }
                if gate == 1 {
                    c.bus
                        .write32(base + 8 * surface + world_painter::SURFACE_KIND, 7);
                }
                if gate == 2 {
                    c.bus
                        .write32(base + 0x18 * material + world_painter::MATERIAL_KIND, 2);
                }
                // Layer kinds (3 bits each; upper bits random) and ids.
                let mut kinds = r.next_u32() & !0x1ff;
                for i in 0..3 {
                    let k = [5u32, 1, 5, 1, 0, 7][(r.next_u32() % 6) as usize];
                    kinds |= k << (3 * i);
                }
                c.bus.write32(this + world_painter::LAYER_KINDS, kinds);
                for i in 0..3 {
                    c.bus
                        .write16(this + world_painter::LAYER_IDS + 2 * i, r.next_u32() as u16);
                }
                // Rectangle and quad.
                for k in 0..12 {
                    c.bus
                        .write32(this + world_painter::RECT + 4 * k, r.f32(64.0));
                }
                // The sampler object and its interface's slot.
                c.bus.write32(gp_at(world_painter::GP_SAMPLER), sampler);
                let iface = vtable(c, &mut r, &[world_painter::SAMPLE_SLOT]);
                c.bus.write32(sampler + world_painter::SAMPLER_IFACE, iface);
                // Colour weights.
                for k in 0..4 {
                    c.bus.write32(world_painter::WEIGHTS + 4 * k, r.f32(2.0));
                }
                // Sampled colours: whatever the stack held (random fill) or
                // the forced fake results do not reach memory, so seed the
                // result slot area with random floats (frame at SP-0x110).
                for k in 0..8 {
                    c.bus
                        .write32(crate::difftest::SP - 0x110 + 0x20 + 4 * k, r.f32(4.0));
                }
                let scale = DATA + 0x8000 + 4 * (r.next_u32() % 3);
                let ratio = DATA + 0x8100 + 4 * (r.next_u32() % 3);
                c.g[A0][0] = if seed % 16 == 0 { 0 } else { sx(this) };
                c.g[A1][0] = sx(scale);
                c.g[A2][0] = sx(ratio);
                let uv = |r: &mut crate::difftest::Rng| match r.next_u32() % 4 {
                    0 => r.edge_f32(),
                    _ => r.f32(1.5),
                };
                c.f[12] = uv(&mut r);
                c.f[13] = uv(&mut r);
            },
        );
    }

    use crate::difftest::elf_image;

    /// The colour factor over the three luminance bands (dark, bright, and
    /// the gamma curve through the real `logf`/`expf`), with random colours
    /// around the thresholds.
    #[test]
    fn world_painter_compute_colour_factor_matches() {
        check(
            ee::s09::f_002edf00__generated,
            world_painter_compute_colour_factor,
            250,
            &|c, seed| {
                let mut r = Rng(0x2edf_0000 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                elf_image(c);
                let col = DATA;
                // Grey levels in the three bands, plus random colours.
                let grey = [0.05f32, 0.1, 0.2, 0.3, 0.44, 0.45, 0.8][(seed % 7) as usize];
                for k in 0..3u32 {
                    let v = if seed % 3 == 0 {
                        f32::from_bits(r.f32(1.0)).abs()
                    } else {
                        grey
                    };
                    c.bus.write32(col + 4 + 4 * k, v.to_bits());
                }
                c.g[A0][0] = sx(col);
                c.pc = 0x002e_df00;
            },
        );
    }

    /// IrradianceRef_GetEntry: resolved, unnamed and named references (the
    /// real `IrradianceRef_Resolve` searches a small name table; the empty
    /// name or a miss decides the index it writes).
    #[test]
    fn irradiance_ref_get_entry_matches() {
        check(
            ee::s09::f_002ee010__generated,
            irradiance_ref_get_entry,
            240,
            &|c, seed| {
                let mut r = Rng(0x2ee0_1000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                // Irradiance table: {?, count, entries (0xa0 each), names (8 each)}.
                let names = DATA + 0x2000;
                let count = 1 + r.next_u32() % 3;
                put_words(c, IRRADIANCE_TABLE, &[0, count, DATA + 0x4000, names]);
                fill(c, &mut r, names, 8);
                if seed % 5 == 0 {
                    c.bus.write8(names, 0); // first name empty
                }
                c.bus
                    .write32(GAME_GP + GP_DEFAULT_IRRADIANCE, r.next_u32() % 4);
                c.bus.write32(this + REF_INDEX, r.next_u32() % 5);
                match seed % 3 {
                    0 => c.bus.write32(this + REF_KEY, RESOLVED as u32),
                    1 => {
                        c.bus.write32(this + REF_KEY, r.next_u32());
                        c.bus.write8(this, 0);
                    }
                    _ => {
                        c.bus.write32(this + REF_KEY, r.next_u32());
                        c.bus.write8(this, 1 + (r.next_u32() % 0x7f) as u8);
                        c.bus.write8(this + 1, 0);
                    }
                }
            },
        );
    }

    pub(crate) use crate::game::bx::memblockman::tests::put_gp;

    /// Disabled (0) or a view's slot whose weather object's getter is a fake
    /// (returning a pointer to a random float; some seeds unwind there).
    #[test]
    fn world_painter_get_weather08_matches() {
        use painter::*;
        crate::difftest::fakes::covered(
            ssx3_ee::s09::f_002ee448__generated,
            world_painter_get_weather08,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ee448, 1);
                c.pc = 0x002e_e448;
                put_gp(
                    c,
                    GP_DISABLED,
                    if seed % 5 == 0 { r.next_u32() | 1 } else { 0 },
                );
                let view = r.next_u32() % 8;
                let slot = SLOTS + view * SLOT_SIZE as u32;
                let handle = DATA + 0x100;
                let obj = DATA + 0x200;
                let vt = DATA + 0x400;
                let value = DATA + 0x800;
                c.bus.write32(slot + SLOT_OBJ, handle);
                c.bus.write32(slot + SLOT_FADE, r.f32(1.0));
                c.bus.write32(handle, obj);
                c.bus.write32(obj + 4, vt);
                c.bus
                    .write16(vt + VT_GET, ((r.next_u32() as i16) & !3) as u16);
                c.bus.write32(vt + VT_GET + 4, crate::difftest::fakes::FAKE);
                c.bus.write32(value, r.f32(100.0));
                c.bus.write32(crate::difftest::fakes::LOG_RESULTS, value);
                c.g[A0][0] = view as u64;
            },
        );
    }

    // (fakes imported above)

    pub(crate) use crate::game::ai::ai::tests::oc258_w32;
    pub(crate) use crate::game::visualfx::renderstateman::tests::weather_getter_253_setup;

    /// `WorldPainter_GetWeather10`.
    #[test]
    fn world_painter_get_weather10_matches() {
        crate::difftest::fakes::covered(
            ee::s09::f_002ee4c0__generated,
            world_painter_get_weather10,
            200,
            &|c, seed| weather_getter_253_setup(c, seed, 0x002e_e4c0, 0x160),
        );
    }

    // ------------------------------------------------------- world painter

    /// A world-painter getter call for view `seed % 4`: the holder at
    /// `entry + table_off` → object → vtable whose slot `vt_off` is a fake
    /// (logs, sometimes unwinds); random floats at every 16-byte slot the
    /// fake's result pointer can land on.
    pub(crate) fn wp_getter_setup_207(
        c: &mut Cpu,
        seed: u32,
        entry_pc: u32,
        table_off: u32,
        vt_off: u32,
    ) -> Rng {
        let mut r = fakes::machine(c, seed, entry_pc as u64, 1);
        c.pc = entry_pc;
        for i in 0..0x4000u32 {
            let v = r.edge_f32();
            c.bus.write32(DATA + i * 0x10, v);
        }
        let index = seed % 4;
        c.g[A0][0] = index as u64;
        let entry = WP_TABLE_207.wrapping_add(index * 0xf0);
        let holder = fakes::ptr(&mut r);
        let obj = fakes::ptr(&mut r);
        let vt = fakes::ptr(&mut r);
        c.bus.write32(entry.wrapping_add(table_off), holder);
        c.bus.write32(holder, obj);
        c.bus.write32(obj.wrapping_add(4), vt);
        let delta = (r.next_u32() as i16 as i32 & !3) as u32;
        c.bus.write16(vt.wrapping_add(vt_off), delta as u16);
        c.bus.write32(vt.wrapping_add(vt_off).wrapping_add(4), FAKE);
        r
    }

    /// WorldPainter_GetWeather18: also a random (edge-case) fade at
    /// `entry+0x24`.
    #[test]
    fn world_painter_get_weather18_matches() {
        covered(
            ee::s09::f_002ee508__generated,
            world_painter_get_weather18,
            200,
            &|c, seed| {
                let mut r = wp_getter_setup_207(c, seed, 0x002e_e508, 0x20, 0x168);
                let entry = WP_TABLE_207 + (seed % 4) * 0xf0;
                let fade = r.edge_f32();
                c.bus.write32(entry + 0x24, fade);
            },
        );
    }

    #[test]
    fn world_painter_get_weather60_matches() {
        covered(
            ee::s09::f_002ee780__generated,
            world_painter_get_weather60,
            200,
            &|c, seed| {
                wp_getter_setup_207(c, seed, 0x002e_e780, 0x20, 0x1b0);
            },
        );
    }

    /// The world painter getters' fixture: world index 0..4, its slot's
    /// field -> object pointer -> object with a fake v-table method (which
    /// returns a float pointer, or unwinds).
    pub(crate) fn world_painter_slot_setup(
        c: &mut Cpu,
        seed: u32,
        entry: u32,
        field: u32,
        vslot: u32,
    ) {
        let mut r = machine(c, seed, entry as u64, 1);
        c.pc = entry;
        let world = seed % 5;
        let slot = WP_SLOTS + WP_SLOT_SIZE * world;
        let (holder, obj) = (DATA + 0x20_0000, DATA + 0x20_0100);
        // The floats the returned pointer points at (fake results land in
        // DATA..DATA+0x40000; the v-table is written over them).
        fill(c, &mut r, DATA, 0x1_0000);
        c.bus.write32(slot + field, holder);
        c.bus.write32(holder, obj);
        let vt = crate::difftest::fakes::vtable(c, &mut r, &[vslot]);
        c.bus.write32(obj + 4, vt);
        c.g[A0][0] = (world as u64) | (r.next_u32() as u64) << 32;
    }

    /// `WorldPainter_GetWeather58`.
    #[test]
    fn world_painter_get_weather58_matches() {
        crate::difftest::fakes::covered(
            ssx3_ee::s09::f_002ee738__generated,
            world_painter_get_weather58,
            300,
            &|c, seed| world_painter_slot_setup(c, seed, 0x002e_e738, 0x20, 0x1a8),
        );
    }

    /// Weather48: weather off (f0 = 0) or the slot object's virtual accessor.
    #[test]
    fn world_painter_get_weather48_matches() {
        check(
            ee::s09::f_002ee600__generated,
            world_painter_get_weather48,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ee600, 1);
                c.pc = 0x002e_e600;
                oc258_w32(
                    c,
                    GAME_GP + LENS_FX_WEATHER_OFF,
                    if seed % 4 == 0 { r.next_u32() | 1 } else { 0 },
                );
                let i = seed % 8;
                c.g[A0][0] = i as u64;
                let slot = LENS_FX_SLOTS + 0xf0 * i;
                let obj = DATA + 0x200;
                let vt = vtable(c, &mut r, &[0x198]);
                oc258_w32(c, slot + 0x20, DATA + 0x100);
                oc258_w32(c, DATA + 0x100, obj);
                oc258_w32(c, obj + 4, vt);
            },
        );
    }

    /// A lens slot `i` whose `field` points at an object with a fake
    /// virtual accessor at vtable `vt_off`.
    pub(crate) fn oc258_lens_slot(c: &mut Cpu, seed: u32, salt: u64, field: u32, vt_off: u32) {
        let mut r = machine(c, seed, salt, 1);
        let i = seed % 8;
        c.g[A0][0] = i as u64;
        let slot = LENS_FX_SLOTS + 0xf0 * i;
        let obj = DATA + 0x200;
        let vt = vtable(c, &mut r, &[vt_off]);
        oc258_w32(c, slot + field, DATA + 0x100);
        oc258_w32(c, DATA + 0x100, obj);
        oc258_w32(c, obj + 4, vt);
    }

    #[test]
    fn world_painter_get_weather40_matches() {
        check(
            ee::s09::f_002ee6a8__generated,
            world_painter_get_weather40,
            32,
            &|c, seed| {
                oc258_lens_slot(c, seed, 0x2ee6a8, 0x20, 0x190);
                c.pc = 0x002e_e6a8;
            },
        );
    }

    /// A world painter view `i` (0..3) whose interface holder at
    /// `holder_off` points at an object with a fake virtual method at
    /// `vslot` (it returns a pointer to the float; some calls unwind).
    pub(crate) fn wp_view_fixture(c: &mut Cpu, seed: u32, salt: u64, holder_off: u32, vslot: u32) {
        let mut r = machine(c, seed, salt, 1);
        let i = r.next_u32() % 3;
        let view = 0x004f_a370 + i * 0xf0;
        let holder = DATA + 0x400 + i * 0x100;
        let obj = DATA + 0x800 + i * 0x100;
        let vt = crate::difftest::fakes::vtable(c, &mut r, &[vslot]);
        c.bus.write32(obj + WP_OBJ_VTABLE, vt);
        c.bus.write32(holder, obj);
        c.bus.write32(view + holder_off, holder);
        c.g[A0][0] = sx(i);
    }

    /// WorldPainter_GetWeather50: views 0..2 through a fake method.
    #[test]
    fn world_painter_get_weather50_matches() {
        crate::difftest::fakes::covered(
            ee::s09::f_002ee660__generated,
            world_painter_get_weather50,
            120,
            &|c, seed| {
                wp_view_fixture(c, seed, 0x2e_e660, WP_VIEW_WEATHER, 0x1a0);
                c.pc = 0x002e_e660;
            },
        );
    }

    /// The weather-getter fixture: painter index 0..3, its weather holder,
    /// object and v-table at random pointers, the slot `{delta, FAKE}`;
    /// the fake (which may unwind) returns a pointer to random data.
    pub(crate) fn world_painter_weather_setup(c: &mut Cpu, seed: u32, entry_pc: u32, vt_off: u32) {
        use crate::difftest::fakes::{machine, ptr, FAKE};
        let mut r = machine(c, seed, entry_pc as u64, 1);
        c.pc = entry_pc;
        let index = seed % 4;
        c.g[A0][0] = index as u64;
        let entry = 0x004f_a370u32.wrapping_add(index * 0xf0);
        let holder = ptr(&mut r);
        let obj = ptr(&mut r);
        let vt = ptr(&mut r);
        c.bus.write32(entry + 0x20, holder);
        c.bus.write32(holder, obj);
        c.bus.write32(obj + 4, vt);
        let delta = (r.next_u32() as i16 as i32 & !3) as u32;
        c.bus.write16(vt + vt_off, delta as u16);
        c.bus.write32(vt + vt_off + 4, FAKE);
    }

    /// WorldPainter_GetWeather38.
    #[test]
    fn world_painter_get_weather38_matches() {
        crate::difftest::fakes::covered(
            ee::s09::f_002ee5b8__generated,
            world_painter_get_weather38,
            200,
            &|c, seed| world_painter_weather_setup(c, seed, 0x002e_e5b8, 0x188),
        );
    }
}
