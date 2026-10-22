//! `src/wscript/wscriptcompile.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::float;
use ssx3_ee_rt::Cpu;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
pub(crate) use crate::game::wscript::wscriptman::p;
pub(crate) use crate::game::fe::feasyncfile::at;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0035_1120, instances_man_dtor, "InstancesMan_dtor"),
    (0x0035_1170, instances_man_alloc, "InstancesMan_Alloc"),
    (0x0035_11d0, instances_man_alloc_by_handle, "InstancesMan_AllocByHandle"),
    (0x0035_1260, instances_man_free, "InstancesMan_Free"),
];

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


// ---------------------------------------------------------------------------
// Instance manager.

/// `void* InstancesMan_Alloc(InstancesMan *man)` (0x351170): pop a free
/// instance from `man->free` (`+8`), marking it used and rebuilding its
/// handle.
pub fn instances_man_alloc(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = c.g[A0][0];
    c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
    if c.g[A3][0] == 0 {
        c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
    } else {
        c.cycles += 3;
        c.g[V0][0] = c.lw(c.g[A3][0] as u32);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
    }
    c.cycles += 16;
    c.g[A2][0] = 0xffff_ffff_ba2e_0000;
    c.g[A2][0] |= 0x8ba3;
    c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
    Addr(c.g[A3][0] as u32).off(0x78).sb(c, c.g[V0][0]);
    c.g[A0][0] |= 0x2000;
    c.g[V0][0] = c.g[A3][0];
    c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
    c.g[A1][0] = Addr(c.g[A3][0] as u32).off(0x78).lbu(c);
    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_sub(c.g[V1][0] as u32));
    Addr(c.g[A3][0] as u32).off(0x8).sw(c, c.g[A0][0]);
    c.g[V1][0] = sx(abi::mult(c, c.g[V1][0] as u32, c.g[A2][0] as u32));
    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 4) as u32);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 8);
    c.g[A1][0] |= c.g[V1][0];
    Addr(c.g[A3][0] as u32).off(0x78).sw(c, c.g[A1][0]);
    abi::ret(c);
}


/// `void InstancesMan_Free(InstancesMan *man, void *inst)` (0x351260): push
/// `inst` on the free list.
pub fn instances_man_free(c: &mut Cpu) {
    c.cycles += 4;
    let man = Addr(abi::arg(c, 0));
    let inst = c.g[A1][0];
    c.g[V0][0] = man.off(8).lw(c);
    Addr(inst as u32).sw(c, c.g[V0][0]);
    man.off(8).sw(c, inst);
    abi::ret(c);
}


/// `void InstancesMan_dtor(InstancesMan* self, int flags)` (0x351120): free
/// the instance table and, when `flags & 1`, the object.
pub fn instances_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = Addr(p(this, 0xc)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            call_ptr(c, table::BUILTIN_VEC_DELETE, 0x0035_1148)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, table::BUILTIN_DELETE, 0x0035_115c)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void* InstancesMan_AllocByHandle(InstancesMan* self, unsigned id)`
/// (0x3511d0): unlink the free node whose 16-byte slot index matches
/// `id >> 8` from the instance's free list, tag it with the instance's type
/// and return it (0 when there is none).
pub fn instances_man_alloc_by_handle(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A3][0] = c.g[A0][0];
    c.g[A2][0] = c.lw(at(c, A3, 8));
    let empty = c.g[A2][0] == 0;
    c.g[A0][0] = sx(p(r32(c, A3), 8));
    if empty {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 4;
    c.g[T0][0] = 0xffff_ffff_ba2e_0000;
    c.g[A1][0] = sx(r32(c, A1) >> 8);
    c.g[T0][0] = c.g[T0][0] | 0x8ba3;
    c.g[V0][0] = c.lw(at(c, A3, 0xc));
    loop {
        c.cycles += 5;
        c.g[V0][0] = sx(r32(c, A2).wrapping_sub(r32(c, V0)));
        c.mult(0, 2, 2, 8);
        c.g[V0][0] = sx(((r32(c, V0) as i32) >> 4) as u32);
        if c.g[V0][0] != c.g[A1][0] {
            c.g[A0][0] = c.g[A2][0];
            c.cycles += 3;
            c.g[A2][0] = c.lw(r32(c, A2));
            if c.g[A2][0] == 0 {
                c.cycles += 2;
                c.g[V0][0] = 0;
                abi::ret(c);
                return;
            }
            c.g[V0][0] = c.lw(at(c, A3, 0xc));
            if event_due(c) {
                deliver_event_here(c, 0x0035_11f0);
            }
            continue;
        }
        c.cycles += 17;
        c.g[V1][0] = c.lw(r32(c, A2));
        c.g[V0][0] = c.g[A2][0];
        c.sw(r32(c, A0), c.g[V1][0]);
        c.g[A0][0] = c.lbu(r32(c, A3));
        c.g[A1][0] = c.lw(at(c, A2, 8));
        c.sb(at(c, A2, 0x78), c.g[A0][0]);
        c.g[A1][0] = c.g[A1][0] | 0x2000;
        c.g[V1][0] = c.lw(at(c, A3, 0xc));
        c.g[A0][0] = c.g[A0][0] & 0xff;
        c.sw(at(c, A2, 8), c.g[A1][0]);
        c.g[V1][0] = sx(r32(c, A2).wrapping_sub(r32(c, V1)));
        c.mult(0, 3, 3, 8);
        c.g[V1][0] = sx(((r32(c, V1) as i32) >> 4) as u32);
        c.g[V1][0] = sx(r32(c, V1) << 8);
        c.g[A0][0] = c.g[A0][0] | c.g[V1][0];
        let ra = r32(c, RA);
        c.sw(at(c, A2, 0x78), c.g[A0][0]);
        c.pc = ra;
        return;
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::put_words;
    use crate::difftest::RA as TEST_RA;
    use ssx3_ee as ee;
    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS;
    pub(crate) use crate::game::wscript::wscriptman::tests::load;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::object::animnode::tests::heap_block;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS_2;
    pub(crate) use crate::game::dirtysock::tags2::tests::w8;
    pub(crate) use crate::game::fe::feasyncfile::tests::w32;
    pub(crate) use crate::game::fe::feasyncfile::tests::gpat;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_8 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_4 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_5 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::env;


    #[test]
    fn instances_man_alloc_matches() {
        check_lohi(ssx3_ee::s11::f_00351170, instances_man_alloc, 200, &|c, seed| {
            let mut r = Rng(0x1017 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write8(DATA, (r.next_u32() & 0xff) as u8);
            c.bus.write32(DATA + 0xc, r.next_u32());
            let slot = if seed % 3 == 0 { 0 } else { DATA + 0x100 };
            c.bus.write32(DATA + 8, slot);
            c.bus.write32(slot, r.next_u32());
            c.bus.write32(slot + 8, r.next_u32());
            c.bus.write8(slot + 0x78, (r.next_u32() & 0xff) as u8);
        });
    }


    #[test]
    fn instances_man_free_matches() {
        check(ssx3_ee::s11::f_00351260, instances_man_free, 200, &|c, seed| {
            let mut r = Rng(0x1018 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA as u64 + 0x100) | ((r.next_u32() as u64) << 32);
            c.bus.write32(DATA + 8, r.next_u32());
        });
    }


    #[test]
    fn instances_man_dtor_matches() {
        check(
            ee::s11::f_00351120__generated,
            instances_man_dtor,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x3511_20, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x40);
                let owned = if seed % 2 == 0 {
                    0
                } else {
                    heap_block(c, 0x40)
                };
                c.bus.write32(this + 0xc, owned);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 3 == 0) as u64;
            },
        );
    }


    #[test]
    fn instances_man_alloc_by_handle_matches() {
        check(
            ee::s11::f_003511d0__generated,
            instances_man_alloc_by_handle,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x3511, 0);
                env(c);
                let this = DATA + 0x400;
                w8(c, this, 5);
                match seed % 3 {
                    0 => w32(c, this + 8, 0),
                    1 => {
                        // `(lo(offset * 0xba2e8ba3) >> 4)` is `offset / 0xb0`
                        // for these offsets, so this node's handle matches.
                        let base = DATA + 0x2000;
                        let id = 3u32;
                        let node = base + 0xb0 * id;
                        w32(c, this + 8, node);
                        w32(c, this + 0xc, base);
                        w32(c, node, 0);
                        w32(c, node + 8, 0xff00);
                        c.g[A1][0] = (id << 8) as u64;
                    }
                    _ => {
                        // The free list holds one entry whose index does not
                        // match; `lw` of the address the multiplied handle
                        // lands on reads a clear word, so the loop ends.
                        let base = DATA + 0x3000;
                        let node = base + 0x10;
                        w32(c, this + 8, node);
                        w32(c, this + 0xc, base);
                        w32(c, node, 0);
                        c.g[A1][0] = 0x200;
                    }
                }
                c.g[A0][0] = sx(this);
            },
        );
    }
}
