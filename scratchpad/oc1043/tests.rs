#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine, FAKE};
    use crate::difftest::{fill, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's callee fakes (their own, unique to oc1043):
    /// a fake answers only when its machine holds `OC1043_MAGIC` here.
    pub(crate) const OC1043_CTRL: u32 = DATA + 0x27_1040;
    pub(crate) const OC1043_MAGIC: u32 = 0x1043_7e57;
    /// Mailbox words the setups fill and the fakes read.
    pub(crate) const OC1043_BOX: u32 = DATA + 0x27_1100;

    fn oc1043_box_w(c: &mut Cpu, k: u32, v: u32) {
        c.bus.write32(OC1043_BOX + 4 * k, v);
    }

    fn oc1043_box_r(c: &mut Cpu, k: u32) -> u32 {
        c.bus.read32(OC1043_BOX + 4 * k)
    }

    /// A gated callee fake: with the gate off it runs whatever the slot held
    /// before; with the gate on it logs the call like `fakes::fake` (every
    /// register the caller set up), then `$post` sets results from the
    /// arguments it saw (`[a0, a1, a2, a3, t0, t1]`).
    macro_rules! oc1043_fake {
        ($name:ident, $prev:ident, $gen:path, $post:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC1043_CTRL) != OC1043_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                let target = c.pc;
                let ret = c.g[RA][0] as u32;
                let args = [
                    c.g[A0][0] as u32,
                    c.g[A1][0] as u32,
                    c.g[A2][0] as u32,
                    c.g[A3][0] as u32,
                    c.g[T0][0] as u32,
                    c.g[T1][0] as u32,
                ];
                fakes::fake(c, target, ret);
                if c.pc == ret {
                    let post: fn(&mut Cpu, [u32; 6]) = $post;
                    post(c, args);
                }
            }
        };
    }

    // ----------------------------------------------------------------------
    // PATH_PlatGetTrackStatus.

    pub(crate) const OC1043_TRACK_TABLE: u32 = 0x004d_3e98;

    // 0x3b6670: stream position: header rate (u16) and sample count.
    oc1043_fake!(
        oc1043_fake_pos,
        OC1043_PREV_POS,
        ee::s13::f_003b6670__generated,
        |c, a| {
            let rate = oc1043_box_r(c, 1);
            let samples = oc1043_box_r(c, 2);
            let res = oc1043_box_r(c, 0);
            c.bus.write16(a[2], rate as u16);
            c.bus.write32(a[4], samples);
            c.g[V0][0] = sx(res);
        }
    );
    // 0x3b8ad8: time remaining of a sub-stream.
    oc1043_fake!(
        oc1043_fake_time,
        OC1043_PREV_TIME,
        ee::s13::f_003b8ad8__generated,
        |c, _a| {
            let v = oc1043_box_r(c, 4);
            c.g[V0][0] = sx(v);
        }
    );
    // 0x3b86c0: sub-stream volume.
    oc1043_fake!(
        oc1043_fake_vol,
        OC1043_PREV_VOL,
        ee::s13::f_003b86c0__generated,
        |c, _a| {
            let v = oc1043_box_r(c, 5);
            c.g[V0][0] = sx(v);
        }
    );
    // 0x3b7b48: stream info: the request count, status result.
    oc1043_fake!(
        oc1043_fake_info,
        OC1043_PREV_INFO,
        ee::s13::f_003b7b48__generated,
        |c, a| {
            let count = oc1043_box_r(c, 6);
            let res = oc1043_box_r(c, 7);
            c.bus.write32(a[1], count);
            c.g[V0][0] = sx(res);
        }
    );
    // 0x3b7c40: request status (state, a, b, c), alternating between two
    // prepared answers.
    oc1043_fake!(
        oc1043_fake_req,
        OC1043_PREV_REQ,
        ee::s13::f_003b7c40__generated,
        |c, a| {
            let n = oc1043_box_r(c, 0x20);
            oc1043_box_w(c, 0x20, n + 1);
            for k in 0..4 {
                let v = oc1043_box_r(c, 8 + 4 * (n % 2) + k);
                c.bus.write32(a[1] + 4 * k, v);
            }
        }
    );
    // 0x3b7f18: stream volume.
    oc1043_fake!(
        oc1043_fake_stvol,
        OC1043_PREV_STVOL,
        ee::s13::f_003b7f18__generated,
        |c, _a| {
            let v = oc1043_box_r(c, 0x10);
            c.g[V0][0] = sx(v);
        }
    );

    fn oc1043_install_track_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B6670),
            oc1043_fake_pos,
            &OC1043_PREV_POS,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B8AD8),
            oc1043_fake_time,
            &OC1043_PREV_TIME,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B86C0),
            oc1043_fake_vol,
            &OC1043_PREV_VOL,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B7B48),
            oc1043_fake_info,
            &OC1043_PREV_INFO,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B7C40),
            oc1043_fake_req,
            &OC1043_PREV_REQ,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B7F18),
            oc1043_fake_stvol,
            &OC1043_PREV_STVOL,
        );
    }

    /// A monitor slot of every shape: no slot, no handle, an in-memory stream
    /// (negative sub, gone sub, live sub with positive or negative sample
    /// counts, failing position or time) and streaming ones (no subs, subs
    /// that are gone, queued, playing, parsing or dead), with out pointers
    /// that are sometimes null.
    #[test]
    fn path_plat_get_track_status_matches() {
        oc1043_install_track_fakes();
        covered(
            ee::s08::f_002aec58__generated,
            path_plat_get_track_status,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_ec58, 3);
                c.bus.write32(OC1043_CTRL, OC1043_MAGIC);
                let idx = seed % 24;
                for i in 0..24 {
                    c.bus.write32(OC1043_TRACK_TABLE + 4 * i, 0);
                }
                let m = DATA + 0x28_0000;
                let subs = DATA + 0x28_1000;
                let kind = seed % 12;
                let streaming = kind >= 8;
                c.g[A0][0] = idx as u64;
                c.g[A1][0] = if streaming && seed % 5 == 0 { 0 } else { (DATA + 0x28_2000) as u64 };
                c.g[A2][0] = if streaming && seed % 7 == 0 { 0 } else { (DATA + 0x28_2010) as u64 };
                if kind != 0 {
                    c.bus.write32(OC1043_TRACK_TABLE + 4 * idx, m);
                }
                // handle, sub index
                c.bus.write32(m, if kind == 1 { u32::MAX } else { 1 + r.next_u32() % 100 });
                c.bus.write32(m + 0x4, if kind == 2 { u32::MAX } else { r.next_u32() % 3 });
                c.bus.write32(m + 0x50, r.next_u32() % 1000);
                c.bus.write32(m + 0x54, r.next_u32() % 1000);
                c.bus.write32(m + 0x6c, subs);
                c.bus.write8(m + 0x64, ((2..=7).contains(&kind)) as u8);
                let count = match kind {
                    8 => 0,
                    9 | 10 => 2,
                    11 => 1 + r.next_u32() % 3,
                    _ => 0,
                };
                c.bus.write8(m + 0x66, count as u8);
                for i in 0..3u32 {
                    let s = subs + 12 * i;
                    c.bus.write32(s, 1 + r.next_u32() % 1000);
                    let id = if (kind == 3 && i == 0) || (kind == 10 && i == 0) {
                        u32::MAX
                    } else {
                        5 + i
                    };
                    c.bus.write32(s + 4, id);
                    c.bus.write32(s + 8, r.next_u32() % 100);
                }
                // fakes' answers
                oc1043_box_w(c, 0, if kind == 6 { u32::MAX } else { r.next_u32() % 3 });
                oc1043_box_w(c, 1, r.next_u32() % 48000 + (seed % 9 != 0) as u32);
                oc1043_box_w(
                    c,
                    2,
                    if kind == 5 { 0x8000_0000 | r.next_u32() } else { r.next_u32() % 100_000 },
                );
                oc1043_box_w(c, 4, if kind == 7 { u32::MAX - 4 } else { r.next_u32() % 500 });
                oc1043_box_w(c, 5, r.next_u32());
                oc1043_box_w(c, 6, r.next_u32() % 4);
                oc1043_box_w(c, 7, if r.next_u32() % 4 == 0 { u32::MAX } else { r.next_u32() % 3 });
                oc1043_box_w(c, 0x10, r.next_u32());
                let states = [0u32, 1, 2, 3, u32::MAX, 5];
                for k in 0..2 {
                    oc1043_box_w(c, 8 + 4 * k, states[(r.next_u32() % 6) as usize]);
                    for j in 1..4 {
                        let v = if r.next_u32() % 4 == 0 { 0 } else { r.next_u32() % 100 };
                        oc1043_box_w(c, 8 + 4 * k + j, v);
                    }
                }
            },
        );
    }

    // ----------------------------------------------------------------------
    // MusicPlayer_CreateStream.

    oc1043_fake!(
        oc1043_fake_create_instance,
        OC1043_PREV_CREATE_INSTANCE,
        ee::s08::f_002aa870__generated,
        |_c, _a| {}
    );
    oc1043_fake!(
        oc1043_fake_get_handle,
        OC1043_PREV_GET_HANDLE,
        ee::s08::f_002ab128__generated,
        |_c, _a| {}
    );

    /// A new music-bank instance on a bank manager with `inst` instances so
    /// far; the play options are copied unaligned from `opts`.
    #[test]
    fn music_player_create_stream_matches() {
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002AA870),
            oc1043_fake_create_instance,
            &OC1043_PREV_CREATE_INSTANCE,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002AB128),
            oc1043_fake_get_handle,
            &OC1043_PREV_GET_HANDLE,
        );
        covered(
            ee::s08::f_002b1780__generated,
            music_player_create_stream,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2b_1780, 2);
                c.bus.write32(OC1043_CTRL, OC1043_MAGIC);
                let (pp, p1, bm) = (DATA + 0x2a_0000, DATA + 0x2a_0100, DATA + 0x2a_1000);
                c.bus.write32(GAME_GP + 0x6fc, pp);
                c.bus.write32(pp, p1);
                c.bus.write32(p1, bm);
                c.bus.write32(bm + 0x1f4, r.next_u32() % 3);
                for k in 0..6 {
                    c.bus.write32(bm + 0x1dc + 4 * k, r.next_u32());
                }
                let arr = DATA + 0x2a_4000;
                for (k, off) in [
                    0x20u32, 0x34, 0x38, 0x3c, 0x60, 0x64, 0x68, 0x6c, 0x70, 0x74, 0x78, 0x7c,
                    0x80, 0x84,
                ]
                .into_iter()
                .enumerate()
                {
                    let skew = if off == 0x20 { seed % 8 } else { 0 };
                    c.bus.write32(bm + 0x1d8 + off, arr + 0x400 * k as u32 + skew);
                    fill(c, &mut r, arr + 0x400 * k as u32, 0x20);
                }
                let opts = DATA + 0x2a_8000;
                fill(c, &mut r, opts, 8);
                c.g[A0][0] = (opts + (seed % 8)) as u64;
            },
        );
    }

    // ----------------------------------------------------------------------
    // MusicPlayer_dtor.

    oc1043_fake!(
        oc1043_fake_mem_free,
        OC1043_PREV_MEM_FREE,
        ee::s10::f_00317e98__generated,
        |_c, _a| {}
    );
    oc1043_fake!(
        oc1043_fake_operator_delete,
        OC1043_PREV_OPERATOR_DELETE,
        ee::s10::f_00317e50__generated,
        |_c, _a| {}
    );
    oc1043_fake!(
        oc1043_fake_song_instance_dtor,
        OC1043_PREV_SONG_INSTANCE_DTOR,
        ee::s08::f_002b1f78__generated,
        |_c, _a| {}
    );
    oc1043_fake!(
        oc1043_fake_stream_sys_dtor,
        OC1043_PREV_STREAM_SYS_DTOR,
        ee::s08::f_002aa648__generated,
        |_c, _a| {}
    );
    oc1043_fake!(
        oc1043_fake_bank_manager_dtor,
        OC1043_PREV_BANK_MANAGER_DTOR,
        ee::s07::f_0028bb10__generated,
        |_c, _a| {}
    );
    oc1043_fake!(
        oc1043_fake_sound_sys_dtor,
        OC1043_PREV_SOUND_SYS_DTOR,
        ee::s08::f_002adee8__generated,
        |_c, _a| {}
    );

    /// Every `flags` value (vtable rebasing, sub-object dtors, delete) with
    /// 0-3 song configurations (null or with a virtual dtor), an MPF file and
    /// a song instance that are present or not.
    #[test]
    fn music_player_dtor_matches() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc1043_fake_mem_free,
            &OC1043_PREV_MEM_FREE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc1043_fake_operator_delete,
            &OC1043_PREV_OPERATOR_DELETE,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B1F78),
            oc1043_fake_song_instance_dtor,
            &OC1043_PREV_SONG_INSTANCE_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002AA648),
            oc1043_fake_stream_sys_dtor,
            &OC1043_PREV_STREAM_SYS_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0028BB10),
            oc1043_fake_bank_manager_dtor,
            &OC1043_PREV_BANK_MANAGER_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002ADEE8),
            oc1043_fake_sound_sys_dtor,
            &OC1043_PREV_SOUND_SYS_DTOR,
        );
        covered(
            ee::s08::f_002b3398__generated,
            music_player_dtor,
            160,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2b_3398, 8);
                c.bus.write32(OC1043_CTRL, OC1043_MAGIC);
                let me = DATA + 0x2c_0000;
                let (p1, p2, p3) = (DATA + 0x2c_8000, DATA + 0x2c_9000, DATA + 0x2c_a000);
                c.bus.write32(me, p1);
                c.bus.write32(p1, p2);
                c.bus.write32(p2 + 0x1d8, p3);
                c.bus.write32(me + 0x3e4, r.next_u32() % 5);
                for i in 0..4u32 {
                    let song = if r.next_u32() % 3 == 0 { 0 } else { DATA + 0x2d_0000 + 0x100 * i };
                    c.bus.write32(me + 4 + 4 * i, song);
                    if song != 0 {
                        let vt = DATA + 0x2e_0000 + 0x20 * i;
                        c.bus.write32(song + 0xa8, vt);
                        c.bus.write16(vt + 8, r.next_u32() as u16);
                        c.bus.write32(vt + 0xc, FAKE + 0x10 * (i + 1));
                    }
                }
                let mpf = if r.next_u32() % 2 == 0 { 0 } else { DATA + 0x2c_c000 };
                c.bus.write32(me + 0x5434, mpf);
                let inst = if r.next_u32() % 2 == 0 { 0 } else { DATA + 0x2c_d000 };
                c.bus.write32(me + 0x408, inst);
                c.g[A0][0] = me as u64;
                c.g[A1][0] = (seed % 4) as u64;
            },
        );
    }

    // ----------------------------------------------------------------------
    // QuadTree_Compact.

    oc1043_fake!(
        oc1043_fake_qt_alloc,
        OC1043_PREV_QT_ALLOC,
        ee::s10::f_00317e30__generated,
        |c, a| {
            let p = oc1043_box_r(c, 0x30);
            oc1043_box_w(c, 0x30, (p + a[0] + 15) & !15);
            c.g[V0][0] = sx(p);
        }
    );
    oc1043_fake!(
        oc1043_fake_qt_flatten,
        OC1043_PREV_QT_FLATTEN,
        ee::s08::f_002c2088__generated,
        |c, a| {
            let n = oc1043_box_r(c, 0x40);
            c.bus.write32(a[2], n);
            for k in 0..n * 2 {
                let v = oc1043_box_r(c, 0x48 + k);
                c.bus.write32(a[1] + 4 * k, v);
            }
        }
    );
    oc1043_fake!(
        oc1043_fake_qt_set_children,
        OC1043_PREV_QT_SET_CHILDREN,
        ee::s08::f_002c16b8__generated,
        |_c, _a| {}
    );

    /// A tree of 0-4 nodes (leaf and inner records) exported through three
    /// callbacks (fixInt, fixShort, fixFloat at fake addresses).
    #[test]
    fn quad_tree_compact_matches() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc1043_fake_qt_alloc,
            &OC1043_PREV_QT_ALLOC,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C2088),
            oc1043_fake_qt_flatten,
            &OC1043_PREV_QT_FLATTEN,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C16B8),
            oc1043_fake_qt_set_children,
            &OC1043_PREV_QT_SET_CHILDREN,
        );
        covered(
            ee::s08::f_002c1da8__generated,
            quad_tree_compact,
            160,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c_1da8, 12);
                c.bus.write32(OC1043_CTRL, OC1043_MAGIC);
                let me = DATA + 0x2f_8000;
                fill(c, &mut r, me, 10);
                c.bus.write32(me + 0xc, r.next_u32() % 8);
                c.bus.write32(me + 0x10, r.next_u32() % 5);
                c.bus.write16(me + 0x14, (r.next_u32() % 4) as u16);
                c.bus.write32(me + 0x20, DATA + 0x2f_a000);
                oc1043_box_w(c, 0x30, DATA + 0x30_0000);
                let n = r.next_u32() % 5;
                oc1043_box_w(c, 0x40, n);
                for k in 0..8 {
                    // nodes alternate between inner (bit 0 set) and leaf records
                    let v = r.next_u32();
                    oc1043_box_w(c, 0x48 + k, if k % 2 == 0 && seed % 3 != 0 { v | 1 } else { v });
                }
                c.g[A0][0] = me as u64;
                c.g[A1][0] = (DATA + 0x2f_9000) as u64;
                c.g[A2][0] = (DATA + 0x2f_9010) as u64;
                c.g[A3][0] = r.next_u32() as u64;
                c.g[T0][0] = (FAKE + 0x100) as u64;
                c.g[T1][0] = (FAKE + 0x110) as u64;
                c.g[T2][0] = (FAKE + 0x120) as u64;
            },
        );
    }

    // ----------------------------------------------------------------------
    // Avalanche_WriteHistoryFile.

    oc1043_fake!(
        oc1043_fake_av_get_be,
        OC1043_PREV_AV_GET_BE,
        ee::s01::f_0014dc80__generated,
        |_c, _a| {}
    );
    oc1043_fake!(
        oc1043_fake_av_get_interface,
        OC1043_PREV_AV_GET_INTERFACE,
        ee::s01::f_0014dd58__generated,
        |_c, _a| {}
    );
    oc1043_fake!(
        oc1043_fake_av_location_id,
        OC1043_PREV_AV_LOCATION_ID,
        ee::s01::f_00144be0__generated,
        |_c, _a| {}
    );
    oc1043_fake!(
        oc1043_fake_av_location_file,
        OC1043_PREV_AV_LOCATION_FILE,
        ee::s01::f_00144c48__generated,
        |_c, _a| {}
    );
    oc1043_fake!(
        oc1043_fake_av_sprintf,
        OC1043_PREV_AV_SPRINTF,
        ee::s15::f_00417828__generated,
        |_c, _a| {}
    );
    // 0x3debf0 opensync: success flag in v0, the handle through a3.
    oc1043_fake!(
        oc1043_fake_av_open,
        OC1043_PREV_AV_OPEN,
        ee::s14::f_003debf0__generated,
        |c, a| {
            let ok = oc1043_box_r(c, 0);
            let fd = oc1043_box_r(c, 1);
            c.bus.write32(a[3], fd);
            c.g[V0][0] = ok as u64;
        }
    );
    oc1043_fake!(
        oc1043_fake_av_close,
        OC1043_PREV_AV_CLOSE,
        ee::s14::f_003deca0__generated,
        |_c, _a| {}
    );
    // 0x2d82a0 writeData: returns the bytes it wrote (size * count).
    oc1043_fake!(
        oc1043_fake_av_write,
        OC1043_PREV_AV_WRITE,
        ee::s09::f_002d82a0__generated,
        |c, a| {
            c.g[V0][0] = (a[4].wrapping_mul(a[5])) as u64;
        }
    );

    /// The avalanche history file: no list, a failing open, and 1-2
    /// avalanches with 0-2 nodes (life from 0 to 10, negative too) and 0-3
    /// collisions and ground collisions each.
    #[test]
    fn avalanche_write_history_file_matches() {
        fakes::hook(addr_of_mut!(ee::s01::OVERRIDE_0014DC80), oc1043_fake_av_get_be, &OC1043_PREV_AV_GET_BE);
        fakes::hook(addr_of_mut!(ee::s01::OVERRIDE_0014DD58), oc1043_fake_av_get_interface, &OC1043_PREV_AV_GET_INTERFACE);
        fakes::hook(addr_of_mut!(ee::s01::OVERRIDE_00144BE0), oc1043_fake_av_location_id, &OC1043_PREV_AV_LOCATION_ID);
        fakes::hook(addr_of_mut!(ee::s01::OVERRIDE_00144C48), oc1043_fake_av_location_file, &OC1043_PREV_AV_LOCATION_FILE);
        fakes::hook(addr_of_mut!(ee::s15::OVERRIDE_00417828), oc1043_fake_av_sprintf, &OC1043_PREV_AV_SPRINTF);
        fakes::hook(addr_of_mut!(ee::s14::OVERRIDE_003DEBF0), oc1043_fake_av_open, &OC1043_PREV_AV_OPEN);
        fakes::hook(addr_of_mut!(ee::s14::OVERRIDE_003DECA0), oc1043_fake_av_close, &OC1043_PREV_AV_CLOSE);
        fakes::hook(addr_of_mut!(ee::s09::OVERRIDE_002D82A0), oc1043_fake_av_write, &OC1043_PREV_AV_WRITE);
        covered(
            ee::s09::f_002d83b8__generated,
            avalanche_write_history_file,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2d_83b8, 12);
                c.bus.write32(OC1043_CTRL, OC1043_MAGIC);
                oc1043_box_w(c, 0, (seed % 6 != 0) as u32);
                oc1043_box_w(c, 1, r.next_u32());
                c.g[A0][0] = (seed % 2) as u64;
                c.g[A1][0] = (DATA + 0x2e_f000) as u64;
                let base = DATA + 0x31_0000;
                let n_av = 1 + r.next_u32() % 2;
                c.bus.write32(
                    GAME_GP + 0x9c0,
                    if seed % 8 == 0 { 0 } else { base },
                );
                let lives = [0u32, 0x40a0_0000, 0x4120_0000, 0xc000_0000, 0x3f80_0000];
                let mut node_at = DATA + 0x33_0000;
                for i in 0..n_av {
                    let av = base + 0x1000 * i;
                    c.bus.write32(av + 8, if i + 1 < n_av { av + 0x1000 } else { 0 });
                    // nodes
                    let nn = r.next_u32() % 3;
                    let mut head = 0;
                    for _ in 0..nn {
                        let node = node_at;
                        node_at += 0x200;
                        c.bus.write32(node + 0xf4, head);
                        c.bus.write32(node + 0xfc, DATA + 0x34_0000);
                        c.bus.write32(node + 0x100, lives[(r.next_u32() % 5) as usize]);
                        fill(c, &mut r, node + 0xe0, 3);
                        head = node;
                    }
                    c.bus.write32(av + 4, head);
                    let ncoll = r.next_u32() % 4;
                    let ngr = r.next_u32() % 4;
                    c.bus.write16(av + 0xc, ncoll as u16);
                    c.bus.write16(av + 0xe, ngr as u16);
                    for k in 0..ncoll {
                        c.bus.write32(av + 0x10 + 8 * k, r.next_u32());
                        c.bus.write32(av + 0x14 + 8 * k, DATA + 0x32_0000 + 0x100 * k);
                    }
                    for k in 0..ngr {
                        c.bus.write16(av + 0x110 + 4 * k, r.next_u32() as u16);
                        c.bus.write16(av + 0x112 + 4 * k, r.next_u32() as u16);
                    }
                }
                for k in 0..4 {
                    c.bus.write32(DATA + 0x32_0000 + 0x100 * k + 0x78, r.next_u32());
                }
            },
        );
    }
}
