//! `src/object/animnode.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::abi::r32;
use crate::call::sp;
use crate::call::Flow;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee as ee;
pub(crate) use crate::game::visualfx::boardtrailfx::p;
pub(crate) use crate::game::be::bestate2::ea;
pub(crate) use crate::game::be::bestate2::gp_rel;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0034_e600, anim_object_get_node_position, "AnimObject_GetNodePosition"),
    (0x0034_eba0, anim_object_fire_end_script, "AnimObject_FireEndScript"),
    (0x0034_ebe0, anim_object_is_time_in_window, "AnimObject_IsTimeInWindow"),
    (0x0034_ee68, floating_v01, "Floating_v01"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::wscript::wscriptman::callee::SET_FADE_GAIN;
    pub(crate) use crate::game::wscript::wscriptman::callee::SND_BANK_UNLOAD;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_ADD_TIMER;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_AUDIO_INSTANCE_STOP_SOUND;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_INSTANCE_POLL_FREE;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_INSTANCE_SET_PITCH;
    pub(crate) use crate::game::wscript::wscriptman::callee::CPS2_MEMCARD_SET_STATE;
    pub(crate) use crate::game::wscript::wscriptman::callee::AVALANCHE_BAKE_NODE_HISTORY;
    pub(crate) use crate::game::wscript::wscriptman::callee::WSCRIPT_MAN_CALL_FUNC;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_MAN3D_GROUP_FREE_MESHES;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_MAN3D_GROUP_CREATE_MESHES_2 as CROWD_MAN3D_GROUP_CREATE_MESHES;
    pub(crate) use crate::game::wscript::wscriptman::callee::FLEX_RAIL_UPDATE_SPACE_NODE;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_FLAG_MANAGER_GROUP_LOAD;
    pub(crate) use crate::game::wscript::wscriptman::callee::FLOATING_V49;
    pub(crate) use crate::game::wscript::wscriptman::callee::MOD_OBJ_DTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::MODIFIER_LIST_UNLINK;
    pub(crate) use crate::game::wscript::wscriptman::callee::RAIL_MODIFIER_LIST_UNLINK;
    pub(crate) use crate::game::wscript::wscriptman::callee::DISABLE_INTC;
    pub(crate) use crate::game::wscript::wscriptman::callee::REMOVE_INTC_HANDLER;
    pub(crate) use crate::game::wscript::wscriptman::callee::TEX_MAN_LOAD_TEXTURE;
    pub(crate) use crate::game::wscript::wscriptman::callee::LIGHT_MAN_EMPTY_HOOK_A;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_GET_STATE_DATA;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::wscript::wscriptman::callee::CPS2_CHUNK_RESOLVE_CTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::AV_PLAYER_INIT;
    pub(crate) use crate::game::wscript::wscriptman::callee::FOCUS_MAN_ADD_FOCUS_BOX;
    pub(crate) use crate::game::wscript::wscriptman::callee::CAM_CONTROLLER_CLEAR_ALGORITHMS;
    pub(crate) use crate::game::wscript::wscriptman::callee::ALGORITHM_LIST_PUSH;
    pub(crate) use crate::game::wscript::wscriptman::callee::APP_MODULE_UPDATE_FADE;
    pub(crate) use crate::game::wscript::wscriptman::callee::DEBUG_MENU_MODULE_UPDATE_CURSOR_TRAIL;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_SETUP_MAN_SEND_QUIT;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_EMPTY_CHILD_HANDLER;
    pub(crate) use crate::game::wscript::wscriptman::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_SET_USER_DATA;
    pub(crate) use crate::game::wscript::wscriptman::callee::EA_FILE_MOUNT_BIG;
    pub(crate) use crate::game::wscript::wscriptman::callee::NET_FE_STATE_CTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::EA_MEM_ALLOC_NAMED;
}

/// Cross-shard direct calls the generator emitted as `call_indirect(c, addr)`.
pub(crate) mod table {
    pub(crate) use crate::game::wscript::wscriptman::table::IS_REPLAY_PLAYING;
    pub(crate) use crate::game::wscript::wscriptman::table::GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::wscript::wscriptman::table::SND_BANK_REMOVE;
    pub(crate) use crate::game::wscript::wscriptman::table::SND_MIDI_RESUME;
    pub(crate) use crate::game::wscript::wscriptman::table::SCE_MC_FLUSH;
    pub(crate) use crate::game::wscript::wscriptman::table::BUILTIN_DELETE;
    pub(crate) use crate::game::wscript::wscriptman::table::BUILTIN_VEC_DELETE;
    pub(crate) use crate::game::wscript::wscriptman::table::MEM_FREE;
    pub(crate) use crate::game::wscript::wscriptman::table::WORLD_GET_OBJECT_SCRIPT_FUNC;
    pub(crate) use crate::game::wscript::wscriptman::table::WSCRIPT_PROCESS_LIST_POP_FRONT;
    pub(crate) use crate::game::wscript::wscriptman::table::STR_HASH;
    pub(crate) use crate::game::wscript::wscriptman::table::SERIALIZE_READ_U32;
    pub(crate) use crate::game::wscript::wscriptman::table::SERIALIZE_READ_U32_2;
    pub(crate) use crate::game::wscript::wscriptman::table::BXFILE_EXISTS;
    pub(crate) use crate::game::wscript::wscriptman::table::STREAM_FILE_DESTROY;
    pub(crate) use crate::game::wscript::wscriptman::table::UI_SCREEN_FIND_OBJECT;
}


/// `void AnimObject_GetNodePosition(AnimObject *obj, int node, void *out,
/// void *arg)` (0x34e600): if `node` equals `obj->selected(+0x2c)`, copy the
/// node's 16-byte position (from `*(obj->model(+0x20))->+0x30` via a virtual
/// call) to `out` and to the stack; else copy the node's entry (at
/// `out = obj->model + node*0x40 + 0x40`, via a virtual call) to `out`.
pub fn anim_object_get_node_position(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x640);
        let obj = abi::arg(c, 0);
        let t0 = obj.wrapping_add(0x14);
        c.g[T0][0] = sx(t0);
        frame.save(c, S1, 0x620);
        frame.save(c, S0, 0x630);
        frame.save_ra(c, 0x610);
        let selected = Addr(obj).off(0x2c).lw(c);
        c.g[V0][0] = selected;
        let matches = abi::arg64(c, 1) == selected;
        c.g[S1][0] = abi::arg64(c, 3);
        if matches {
            c.cycles += 10;
            let vt = Addr(obj).off(0x20).lw(c);
            c.g[V1][0] = vt;
            c.g[A1][0] = abi::arg64(c, 2);
            let m = VSlot::read(c, vt, 0xe8);
            c.g[A0][0] = m.this(t0 as u64);
            c.g[V0][0] = m.func;
            call_ptr(c, m.func as u32, 0x0034_e638)?;
            c.lq(3, (c.g[V0][0] as u32).wrapping_add(0x30));
            c.sq(3, c.g[S1][0] as u32);
            c.sq(3, c.g[SP][0] as u32);
        } else {
            c.cycles += 14;
            let idx = abi::arg(c, 2);
            c.g[S0][0] = sx(idx << 6);
            let vt = Addr(obj).off(0x20).lw(c);
            c.g[A3][0] = vt;
            c.g[A1][0] = sx(abi::arg(c, 1).wrapping_add(0x10));
            c.g[V0][0] = 0x17;
            c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[V1][0] = u64::MAX;
            let m = VSlot::read(c, vt, 0x108);
            c.g[A0][0] = m.this(t0 as u64);
            c.g[V0][0] = m.func;
            call_ptr(c, m.func as u32, 0x0034_e670)?;
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[SP][0] as u32));
            c.lq(2, (c.g[V1][0] as u32).wrapping_add(0x40));
            c.sq(2, c.g[S1][0] as u32);
            c.sq(2, c.g[SP][0] as u32);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x630);
        frame.restore(c, S1, 0x620);
        frame.restore_ra(c, 0x610);
        frame.close(c);
        Ok(())
    })
}


/// `int AnimObject_FireEndScript(sAnimObject* obj)` (0x34eba0): fire the
/// animation end script of the link at `obj+0x14`, then report whether the
/// link's list head is the link itself (i.e. `list->head == link`).
pub fn anim_object_fire_end_script(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let obj = abi::arg(c, 0);
        c.g[S0][0] = sx(obj);
        c.g[A0][0] = Addr(p(obj, 0x2c)).lw(c);
        frame.save_ra(c, 0);
        let link = p(obj, 0x14);
        c.g[S0][0] = sx(link);
        call_ptr(c, 0x002d_1ac8, 0x0034_ebbc)?;
        c.g[V1][0] = Addr(p(link, 0x18)).lw(c);
        frame.restore_ra(c, 0);
        c.g[V0][0] = Addr(p(r32(c, V1), 0xc)).lw(c);
        c.g[V0][0] ^= c.g[S0][0];
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = (c.g[V0][0] < 1) as u64;
        frame.close(c);
        Ok(())
    })
}


/// `void Floating_v01(Floating* self, int flags)` (0x34ee68): the anim
/// node's destructor: install the vtable, run the subclass teardown, free
/// the owned block and the base object.
pub fn floating_v01(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x48_f858; // D_0048F858
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        Addr(p(this, 0xc)).sw(c, c.g[V0][0]);
        callee::FLOATING_V49.call(c, 0x0034_ee90)?;
        c.g[A0][0] = Addr(p(this, 0x74)).lw(c);
        call_ptr(c, table::BUILTIN_DELETE, 0x0034_ee98)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        callee::MOD_OBJ_DTOR.call(c, 0x0034_eea4)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int AnimObject_IsTimeInWindow(AnimObject* self, float t)` (0x34EBE0): is
/// `t` inside the object's [start, end) play window? A start of 1e10 means
/// "no window"; a window that ends before it starts wraps.
pub fn anim_object_is_time_in_window(c: &mut Cpu) {
    c.cycles += 6;
    c.lwc1(0, gp_rel(c, -0x2aa0)); // the "no window" sentinel (1e10)
    c.lwc1(1, ea(c.g[A0][0], 8)); // start
    c.c_eq_s(1, 0);
    let no_window = c.fcc();
    c.g[V0][0] = 0;
    if no_window {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 4;
    c.lwc1(0, ea(c.g[A0][0], 0xc)); // end
    c.c_lt_s(1, 0);
    if !c.fcc() {
        // start >= end: the window wraps, so [end, start) is outside.
        c.cycles += 4;
        c.c_le_s(0, 12);
        c.g[V0][0] = 0;
        if c.fcc() {
            c.cycles += 4;
            c.c_lt_s(12, 1);
            c.g[V0][0] = 1;
            if c.fcc() {
                c.cycles += 2;
                abi::ret(c);
                return;
            }
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    // start < end: start <= t < end.
    c.cycles += 3;
    c.c_le_s(1, 12);
    if c.fcc() {
        c.cycles += 3;
        c.c_lt_s(12, 0);
        if c.fcc() {
            c.cycles += 2;
            c.g[V0][0] = 1;
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::check;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::fakes::*;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::put_words;
    use crate::difftest::GAME_GP;
    use ssx3_ee as ee;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::RA as TEST_RA;
    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS;
    pub(crate) use crate::game::wscript::wscriptman::tests::load;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::be::bestate2::tests::w;
    pub(crate) use crate::game::be::bestate2::tests::belib;


    #[test]
    fn anim_object_get_node_position_matches() {
        covered(
            ssx3_ee::s11::f_0034e600,
            anim_object_get_node_position,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 22, 2);
                let obj = ptr(&mut r);
                c.g[A0][0] = obj as u64;
                let node = r.next_u32();
                let selected = if r.next_u32() % 2 == 0 { node } else { r.next_u32() };
                c.bus.write32(obj + 0x2c, selected);
                c.g[A1][0] = node as u64;
                c.g[A2][0] = ptr(&mut r) as u64;
                c.g[A3][0] = ptr(&mut r) as u64;
                let vt = vtable(c, &mut r, &[0xe8, 0x108]);
                c.bus.write32(obj + 0x20, vt);
            },
        );
    }


    #[test]
    fn anim_object_fire_end_script_matches() {
        check(ee::s11::f_0034eba0__generated, anim_object_fire_end_script, 200, &|c, seed| {
            machine(c, seed, 0x34eba0, 3);
            let obj = DATA;
            let list = DATA + 0x1000;
            let man = DATA + 0x2000;
            c.bus.write32(GAME_GP + 0xce8, man);
            c.bus.write32(man + 0x28c, 0);
            c.bus.write32(list + 0x78, 0);
            c.bus.write32(list + 0x88, 0);
            c.bus.write32(obj + 0x2c, list);
            // `list->head == link` decides the result.
            c.bus.write32(list + 0xc, if seed % 2 == 0 { obj + 0x14 } else { obj + 0x40 });
            c.g[A0][0] = sx(obj);
        });
    }


    /// A block from the test heap (`alloc_ready` must have run first).
    pub(crate) fn heap_block(c: &mut Cpu, size: u32) -> u32 {
        const MEM_HEAP: u32 = 0x004f_f250;
        load(
            c,
            0x0032_0058,
            ssx3_ee::s10::f_00320058,
            &[MEM_HEAP as u64, 16, size as u64],
        ) as u32
    }


    #[test]
    fn floating_v01_matches() {
        check(ee::s11::f_0034ee68__generated, floating_v01, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x34ee_68, 0);
            alloc_ready(c);
            let this = DATA + 0x100;
            clear(c, this, 0x200);
            let owned = if seed % 2 == 0 {
                0
            } else {
                heap_block(c, 0x30)
            };
            c.bus.write32(this + 0x74, owned);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = (seed % 3 == 0) as u64;
        });
    }


    pub(crate) fn fb(x: f32) -> u32 {
        x.to_bits()
    }


    #[test]
    fn anim_object_is_time_in_window_matches() {
        check(ee::s11::f_0034ebe0__generated, anim_object_is_time_in_window, 150, &|c, seed| {
            let mut r = machine(c, seed, 0x0034_ebe0, 0);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.f[12] = r.edge_f32();
            w(c, GAME_GP - 0x2aa0, fb(1.0e10));
            let (start, end) = match seed % 4 {
                0 => (1.0e10, 2.0),
                1 => (1.0, 2.0),
                2 => (2.0, 1.0),
                _ => (-1.0, 1.0),
            };
            w(c, DATA + 8, fb(start));
            w(c, DATA + 0xc, fb(end));
        });
    }
}
