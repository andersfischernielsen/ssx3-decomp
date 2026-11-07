//! `src/camera/cameraalgolist.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::be::bedbreward::BUILTIN_DELETE_ADDR;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::fe::feasyncfile::r;
use crate::guest::Addr;
#[cfg(test)]
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0015_c988, algorithm_list_push, "AlgorithmList_Push"),
    (0x0015_ca50, algorithm_list_remove, "AlgorithmList_Remove"),
    (0x0015_cb08, algorithm_list_clear, "AlgorithmList_Clear"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {

    pub(crate) use crate::game::fe::feasyncfile::callee::FREE;
}

pub(crate) const OPERATOR_NEW_NAMED_ADDR: u32 = 0x0031_7d70;

/// `void AlgorithmList_Push(AlgorithmList *list, void *payload, void *arg, float blend f12)`
/// (0x15c988): allocate a 0x1c-byte node, run the payload's prepare slot
/// (`payload+0x10 -> +0x20/+0x24`), link it at the head and remember the
/// per-node blend (1.0 or 0 when `blend == 1.0`).
pub fn algorithm_list_push(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        c.swc1(20, frame.at(c, 0x40));
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.mov_s(20, 12);
        c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x10)).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x20)).lh(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x24)).lw(c);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, c.g[V0][0] as u32, 0x0015_c9c4)?;
        c.g[A1][0] = 0x46_0000;
        c.g[A0][0] = 0x1c;
        c.g[A1][0] = sx(0x46_0000u32.wrapping_sub(0x4e10));
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, OPERATOR_NEW_NAMED_ADDR, 0x0015_c9dc)?;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.g[A0][0] = c.g[V0][0];
        c.sw((c.g[A0][0] as u32).wrapping_add(0x18), c.g[S1][0]);
        c.c_eq_s(20, 0);
        c.sw(c.g[A0][0] as u32, c.g[S0][0]);
        c.sw((c.g[A0][0] as u32).wrapping_add(0x10), 0);
        let blend_ne_one = !c.fcc();
        c.sw((c.g[A0][0] as u32).wrapping_add(0x14), 0);
        if blend_ne_one {
            c.cycles += 1;
            c.sw((c.g[A0][0] as u32).wrapping_add(4), 0);
        } else {
            c.cycles += 2;
            c.swc1(0, (c.g[A0][0] as u32).wrapping_add(4));
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
        let empty = c.g[V0][0] == 0;
        c.swc1(20, (c.g[A0][0] as u32).wrapping_add(8));
        if !empty {
            c.cycles += 1;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A0][0]);
        }
        c.cycles += 13;
        c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(4)).lw(c);
        c.g[V1][0] = Addr(c.g[S2][0] as u32).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        c.sw(c.g[S2][0] as u32, c.g[A0][0]);
        c.sw((c.g[S2][0] as u32).wrapping_add(4), c.g[V0][0]);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        c.lwc1(20, frame.at(c, 0x40));
        c.sw((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
        frame.close(c);
        Ok(())
    })
}

/// A node in an `AlgorithmList`: payload at +0, next at +0x14, flags at +0x18.
pub(crate) const ALG_NODE_PAYLOAD: u32 = 0x0;

pub(crate) const ALG_NODE_NEXT: u32 = 0x14;

pub(crate) const ALG_NODE_FLAGS: u32 = 0x18;

/// `void AlgorithmList_Clear(AlgorithmList *list)` (0x15cb08): delete every
/// node (running its payload destructor first) and null the head and count.
pub fn algorithm_list_clear(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = Addr(c.g[S2][0] as u32).lw(c);
        if c.g[S0][0] == 0 {
            c.sw(c.g[S2][0] as u32, 0);
        } else {
            c.cycles += 1;
            loop {
                c.cycles += 2;
                c.g[S1][0] = Addr((c.g[S0][0] as u32).wrapping_add(ALG_NODE_NEXT)).lw(c);
                c.cycles += 3;
                c.g[A2][0] = Addr((c.g[S0][0] as u32).wrapping_add(ALG_NODE_PAYLOAD)).lw(c);
                if c.g[A2][0] == 0 {
                    c.sw(c.g[S0][0] as u32, 0);
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(ALG_NODE_FLAGS)).lw(c);
                    let no_flags = c.g[V0][0] == 0;
                    c.g[A1][0] = 0x3;
                    if !no_flags {
                        c.cycles += 5;
                        c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x10)).lw(c);
                        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(8)).lh(c);
                        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, c.g[V1][0] as u32, 0x0015_cb64)?;
                    }
                    c.cycles += 1;
                    c.sw(c.g[S0][0] as u32, 0);
                }
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(ALG_NODE_FLAGS), 0);
                call_ptr(c, BUILTIN_DELETE_ADDR, 0x0015_cb74)?;
                c.cycles += 3;
                c.g[S0][0] = c.g[S1][0];
                let more = c.g[S0][0] != 0;
                if !more {
                    c.cycles += 1;
                    c.sw(c.g[S2][0] as u32, 0);
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0015_cb30);
                }
            }
        }
        c.cycles += 7;
        c.sw((c.g[S2][0] as u32).wrapping_add(4), 0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Algorithm list.

/// `void AlgorithmList_Remove(AlgorithmList* list, AlgorithmList::Node* n)`
/// (0x15ca50): unlink a list node, drop its held object when it is the head
/// and free the node; return the list's new length minus one.
pub fn algorithm_list_remove(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        'outer: {
            'prev: {
                frame.save(c, S0, 0x30);
                frame.save(c, S2, 0x10);
                c.g[S0][0] = c.g[A1][0];
                frame.save(c, S1, 0x20);
                let list = r(c, A0);
                c.g[S2][0] = sx(list);
                frame.save_ra(c, 0);
                c.g[V0][0] = Addr(p(r(c, S0), 0x10)).lw(c);
                let has_prev = c.g[V0][0] != 0;
                c.g[S1][0] = Addr(p(r(c, S0), 0x14)).lw(c);
                if has_prev {
                    break 'prev;
                }
                c.cycles += 2;
                Addr(r(c, S2)).sw(c, c.g[S1][0]);
                break 'outer;
            }
            c.cycles += 1;
            Addr(p(r(c, V0), 0x14)).sw(c, c.g[S1][0]);
        }
        'next: {
            c.cycles += 3;
            c.g[V1][0] = Addr(p(r(c, S0), 0x14)).lw(c);
            let last = c.g[V1][0] == 0;
            if last {
                break 'next;
            }
            c.cycles += 2;
            c.g[V0][0] = Addr(p(r(c, S0), 0x10)).lw(c);
            Addr(p(r(c, V1), 0x10)).sw(c, c.g[V0][0]);
        }
        'held: {
            c.cycles += 2;
            if c.g[S0][0] != 0 {
                'owned: {
                    c.cycles += 3;
                    c.g[A2][0] = Addr(r(c, S0)).lw(c);
                    if c.g[A2][0] == 0 {
                        Addr(r(c, S0)).sw(c, 0);
                        break 'owned;
                    }
                    'release: {
                        c.cycles += 3;
                        c.g[V0][0] = Addr(p(r(c, S0), 0x18)).lw(c);
                        let plain = c.g[V0][0] == 0;
                        c.g[A1][0] = 3;
                        if plain {
                            break 'release;
                        }
                        c.cycles += 5;
                        c.g[V0][0] = Addr(p(r(c, A2), 0x10)).lw(c);
                        c.g[A0][0] = Addr(p(r(c, V0), 8)).lh(c);
                        c.g[V1][0] = Addr(p(r(c, V0), 0xc)).lw(c);
                        let target = r(c, V1);
                        c.g[RA][0] = sx(0x0015_cacc);
                        c.g[A0][0] = sx(p(r(c, A2), r(c, A0)));
                        call_ptr(c, target, 0x0015_cacc)?;
                        break 'release;
                    }
                    c.cycles += 1;
                    Addr(r(c, S0)).sw(c, 0);
                    break 'owned;
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0015_cadc);
                Addr(p(r(c, S0), 0x18)).sw(c, 0);
                call_ptr(c, callee::FREE, 0x0015_cadc)?;
                c.g[V1][0] = Addr(p(r(c, S2), 4)).lw(c);
                break 'held;
            }
            c.g[V1][0] = Addr(p(r(c, S2), 4)).lw(c);
            break 'held;
        }
        c.cycles += 9;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, 16, 0x30);
        c.g[V1][0] = sx(r(c, V1).wrapping_sub(1));
        frame.restore(c, 17, 0x20);
        Addr(p(r(c, S2), 4)).sw(c, c.g[V1][0]);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::ee;
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::DATA;

    pub(crate) use crate::game::ai::aianimeventmap::tests::alloc_setup;

    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_4 as alloc_base;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot_11 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_15 as SEEDS_2;

    pub(crate) use crate::game::world::wscriptcache::tests::SEEDS;

    #[test]
    fn algorithm_list_push_matches() {
        check(
            ee::s01::f_0015c988,
            algorithm_list_push,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0xc988, 0);
                alloc_setup(c, &mut r);
                let list = DATA;
                let obj = DATA + 0x100;
                c.g[A0][0] = list as u64;
                c.g[A1][0] = obj as u64;
                c.g[A2][0] = sx(r.next_u32());
                c.bus
                    .write32(list, if seed % 3 == 0 { 0 } else { DATA + 0x300 });
                c.bus.write32(list + 4, r.next_u32());
                let vt = vtable(c, &mut r, &[0x20]);
                c.bus.write32(obj + 0x10, vt);
                c.f[12] = if seed % 2 == 0 {
                    0x3f80_0000
                } else {
                    r.edge_f32()
                };
            },
        );
    }

    #[test]
    fn algorithm_list_clear_matches() {
        check(
            ee::s01::f_0015cb08,
            algorithm_list_clear,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0xcb08, 0);
                alloc_setup(c, &mut r);
                let list = DATA;
                c.g[A0][0] = list as u64;
                if seed % 2 == 0 {
                    c.bus.write32(list, 0);
                    c.bus.write32(list + 4, 0);
                } else {
                    let node = DATA + 0x100;
                    c.bus.write32(list, node);
                    c.bus.write32(list + 4, 1);
                    c.bus.write32(node + 0x14, 0);
                    if seed % 4 == 0 {
                        c.bus.write32(node, 0);
                    } else {
                        let payload = DATA + 0x200;
                        c.bus.write32(node, payload);
                        let vt = vtable(c, &mut r, &[8]);
                        c.bus.write32(payload + 0x10, vt);
                    }
                    c.bus
                        .write32(node + 0x18, if seed % 6 == 0 { 0 } else { 1 });
                }
            },
        );
    }

    // -------------------------------------------------------- algorithm list

    #[test]
    fn algorithm_list_remove_matches() {
        check(
            ee::s01::f_0015ca50__generated,
            algorithm_list_remove,
            SEEDS_2,
            &|c, seed| {
                let mut r = alloc_base(c, seed, 0x15ca, 3);
                let list = DATA;
                let node = DATA + 0x100;
                w(c, list + 4, r.next_u32() % 4);
                w(c, node + 0x10, if seed % 2 == 0 { 0 } else { DATA + 0x200 });
                w(c, node + 0x14, if seed & 2 == 0 { 0 } else { DATA + 0x300 });
                w(c, node, if seed & 4 == 0 { 0 } else { DATA + 0x400 });
                w(c, node + 0x18, 0);
                let vt = DATA + 0x500;
                w(c, DATA + 0x400, vt);
                slot(c, vt, 0x10, FAKE);
                slot(c, vt, 0x14, FAKE);
                c.g[A0][0] = list as u64;
                c.g[A1][0] = node as u64;
            },
        );
    }
}
