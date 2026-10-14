// Register values are written as bit-field ORs (`0 | a << n`) for readability.
#![allow(clippy::precedence, clippy::identity_op)]

use super::swizzle::*;
use super::*;

// ------------------------------------------------------------------ helpers

fn giftag(nloop: u32, eop: bool, flg: u32, nreg: u32, regs: u64) -> Qword {
    let lo = nloop as u64 & 0x7FFF
        | (eop as u64) << 15
        | (flg as u64) << 58
        | ((nreg as u64) & 0xF) << 60;
    lo as u128 | (regs as u128) << 64
}

fn ad_q(addr: u8, value: u64) -> Qword {
    value as u128 | (addr as u128) << 64
}

/// Send a list of A+D writes as one PACKED packet on PATH3.
fn ad(gs: &mut Gs, list: &[(u8, u64)]) {
    let mut q = vec![giftag(list.len() as u32, true, 0, 1, 0xE)];
    q.extend(list.iter().map(|&(a, v)| ad_q(a, v)));
    gs.gif_write(3, &q);
}

fn bytes_to_qwords(b: &[u8]) -> Vec<Qword> {
    let mut v = b.to_vec();
    v.resize(v.len().next_multiple_of(16), 0);
    v.chunks_exact(16)
        .map(|c| Qword::from_le_bytes(c.try_into().unwrap()))
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn upload(gs: &mut Gs, dbp: u32, dbw: u32, dpsm: u32, x: u32, y: u32, w: u32, h: u32, data: &[u8]) {
    ad(
        gs,
        &[
            (
                0x50,
                (dbp as u64) << 32 | (dbw as u64) << 48 | (dpsm as u64) << 56,
            ),
            (0x51, (x as u64) << 32 | (y as u64) << 48),
            (0x52, w as u64 | (h as u64) << 32),
            (0x53, 0),
        ],
    );
    let q = bytes_to_qwords(data);
    let mut pkt = vec![giftag(q.len() as u32, true, 2, 0, 0)];
    pkt.extend(q);
    gs.gif_write(3, &pkt);
}

#[allow(clippy::too_many_arguments)]
fn download(gs: &mut Gs, sbp: u32, sbw: u32, spsm: u32, x: u32, y: u32, w: u32, h: u32) -> Vec<u8> {
    ad(
        gs,
        &[
            (0x50, sbp as u64 | (sbw as u64) << 16 | (spsm as u64) << 24),
            (0x51, x as u64 | (y as u64) << 16),
            (0x52, w as u64 | (h as u64) << 32),
            (0x53, 1),
        ],
    );
    let q = gs.read_local(usize::MAX);
    q.iter().flat_map(|q| q.to_le_bytes()).collect()
}

/// Standard 640x448 CT32 frame at FBP 0, full scissor, context 1.
fn setup_frame(gs: &mut Gs) {
    ad(
        gs,
        &[
            (0x4C, 10 << 16),              // FRAME_1: FBP 0, FBW 10, CT32
            (0x40, 639 << 16 | 447 << 48), // SCISSOR_1
            (0x18, 0),                     // XYOFFSET_1
            (0x47, 0),                     // TEST_1
        ],
    );
}

fn xyz(x: u32, y: u32, z: u32) -> u64 {
    (x * 16) as u64 | ((y * 16) as u64) << 16 | (z as u64) << 32
}

fn rgba(r: u8, g: u8, b: u8, a: u8) -> u64 {
    u32::from_le_bytes([r, g, b, a]) as u64 | (1.0f32.to_bits() as u64) << 32
}

fn fb(gs: &Gs, x: u32, y: u32) -> u32 {
    read_pixel(&gs.vram, PSMCT32, 0, 10, x, y)
}

fn sprite(gs: &mut Gs, x0: u32, y0: u32, x1: u32, y1: u32, z: u32, c: u64) {
    ad(
        gs,
        &[
            (0x00, 6),
            (0x01, c),
            (0x05, xyz(x0, y0, z)),
            (0x05, xyz(x1, y1, z)),
        ],
    );
}

// ------------------------------------------------------------------ swizzle

fn check_bijection(t: &[u16]) {
    let mut seen = vec![false; t.len()];
    for &e in t {
        assert!(!seen[e as usize], "duplicate element {e}");
        seen[e as usize] = true;
    }
}

#[test]
fn swizzle_tables_are_bijections() {
    let t = tables();
    for tab in [
        &t.t32, &t.z32, &t.t16, &t.t16s, &t.z16, &t.z16s, &t.t8, &t.t4,
    ] {
        check_bijection(tab);
    }
}

#[test]
fn swizzle_known_addresses_32() {
    let s = Surface::new(PSMCT32, 0, 2);
    assert_eq!(s.locate(0, 0), (0, 0));
    assert_eq!(s.locate(1, 0), (1, 0));
    assert_eq!(s.locate(0, 1), (2, 0));
    assert_eq!(s.locate(2, 0), (4, 0));
    assert_eq!(s.locate(0, 2), (16, 0));
    assert_eq!(s.locate(8, 0), (64, 0)); // block 1
    assert_eq!(s.locate(0, 8), (128, 0)); // block 2
    assert_eq!(s.locate(16, 0), (256, 0)); // block 4
    assert_eq!(s.locate(64, 0), (2048, 0)); // next page
    assert_eq!(s.locate(0, 32), (4096, 0)); // next page row (bw = 2 pages)
                                            // Block-granular base pointer.
    assert_eq!(Surface::new(PSMCT32, 3, 1).locate(0, 0), (192, 0));
    // Z32 uses its own block order.
    assert_eq!(Surface::new(PSMZ32, 0, 1).locate(0, 0), (24 * 64, 0));
    assert_eq!(Surface::new(PSMZ32, 0, 1).locate(32, 16), (0, 0));
    assert_eq!(Surface::new(PSMZ32, 0, 1).locate(32, 0), (8 * 64, 0));
}

#[test]
fn swizzle_known_addresses_16() {
    let s = Surface::new(PSMCT16, 0, 1);
    assert_eq!(s.elem(0, 0), 0);
    assert_eq!(s.elem(1, 0), 2);
    assert_eq!(s.elem(8, 0), 1);
    assert_eq!(s.locate(8, 0), (0, 16)); // upper halfword of word 0
    assert_eq!(s.elem(16, 0), 2 * 128); // block 2
    assert_eq!(s.elem(0, 8), 128); // block 1
    let ss = Surface::new(PSMCT16S, 0, 1);
    assert_eq!(ss.elem(32, 0), 16 * 128);
    assert_eq!(ss.elem(0, 16), 8 * 128);
    assert_eq!(Surface::new(PSMZ16, 0, 1).elem(0, 0), 24 * 128);
    assert_eq!(Surface::new(PSMZ16S, 0, 1).elem(32, 0), 8 * 128);
}

#[test]
fn swizzle_known_addresses_8_and_4() {
    // PCSX2 columnTable8 rows 0..3.
    let ct8: [[u32; 16]; 4] = [
        [0, 4, 16, 20, 32, 36, 48, 52, 2, 6, 18, 22, 34, 38, 50, 54],
        [
            8, 12, 24, 28, 40, 44, 56, 60, 10, 14, 26, 30, 42, 46, 58, 62,
        ],
        [33, 37, 49, 53, 1, 5, 17, 21, 35, 39, 51, 55, 3, 7, 19, 23],
        [
            41, 45, 57, 61, 9, 13, 25, 29, 43, 47, 59, 63, 11, 15, 27, 31,
        ],
    ];
    let s8 = Surface::new(PSMT8, 0, 2);
    for (y, row) in ct8.iter().enumerate() {
        for (x, &e) in row.iter().enumerate() {
            assert_eq!(s8.elem(x as u32, y as u32), e, "t8 ({x},{y})");
        }
    }
    assert_eq!(s8.elem(0, 4), 96);
    assert_eq!(s8.elem(16, 0), 256); // block 1
    assert_eq!(s8.elem(128, 0), 8192); // next page (bw 2 = one 128-wide page)
                                       // PCSX2 columnTable4 row 0 and part of row 2.
    let ct4_0: [u32; 32] = [
        0, 8, 32, 40, 64, 72, 96, 104, 2, 10, 34, 42, 66, 74, 98, 106, 4, 12, 36, 44, 68, 76, 100,
        108, 6, 14, 38, 46, 70, 78, 102, 110,
    ];
    let s4 = Surface::new(PSMT4, 0, 2);
    for (x, &e) in ct4_0.iter().enumerate() {
        assert_eq!(s4.elem(x as u32, 0), e, "t4 ({x},0)");
    }
    for (x, &e) in [65u32, 73, 97, 105, 1, 9, 33, 41].iter().enumerate() {
        assert_eq!(s4.elem(x as u32, 2), e, "t4 ({x},2)");
    }
    assert_eq!(s4.elem(32, 0), 2 * 512); // block 2 (BLOCK16[0][1])
}

#[test]
fn swizzle_formats_cover_pages_exactly() {
    // Writing every pixel of one page with a distinct value then reading back
    // must round-trip, and all formats must touch exactly 2048 words.
    for (psm, pw, ph, bits) in [
        (PSMCT32, 64, 32, 32),
        (PSMZ32, 64, 32, 32),
        (PSMCT16, 64, 64, 16),
        (PSMCT16S, 64, 64, 16),
        (PSMZ16, 64, 64, 16),
        (PSMZ16S, 64, 64, 16),
        (PSMT8, 128, 64, 8),
        (PSMT4, 128, 128, 4),
    ] {
        let mut vram = vec![0u32; VRAM_WORDS];
        let s = Surface::new(psm, 32, pw / 64);
        let mask = if bits == 32 {
            u32::MAX
        } else {
            (1 << bits) - 1
        };
        for y in 0..ph {
            for x in 0..pw {
                s.write(&mut vram, x, y, (x * 7 + y * 131 + 1) & mask);
            }
        }
        for y in 0..ph {
            for x in 0..pw {
                assert_eq!(
                    s.read(&vram, x, y),
                    (x * 7 + y * 131 + 1) & mask,
                    "psm {psm:#x} ({x},{y})"
                );
            }
        }
        // Only page 1 (words 2048..4096) was touched.
        assert!(vram[..2048].iter().all(|&w| w == 0));
        assert!(vram[4096..].iter().all(|&w| w == 0));
    }
}

#[test]
fn swizzle_high_formats_share_ct32_words() {
    let mut vram = vec![0u32; VRAM_WORDS];
    write_pixel(&mut vram, PSMCT24, 0, 1, 5, 3, 0x123456);
    write_pixel(&mut vram, PSMT8H, 0, 1, 5, 3, 0xAB);
    assert_eq!(read_pixel(&vram, PSMCT32, 0, 1, 5, 3), 0xAB12_3456);
    write_pixel(&mut vram, PSMT4HL, 0, 1, 5, 3, 0x5);
    write_pixel(&mut vram, PSMT4HH, 0, 1, 5, 3, 0xC);
    assert_eq!(read_pixel(&vram, PSMCT32, 0, 1, 5, 3), 0xC512_3456);
    assert_eq!(read_pixel(&vram, PSMT4HL, 0, 1, 5, 3), 0x5);
    assert_eq!(read_pixel(&vram, PSMT4HH, 0, 1, 5, 3), 0xC);
    assert_eq!(read_pixel(&vram, PSMT8H, 0, 1, 5, 3), 0xC5);
}

// ------------------------------------------------------------------ transfers

#[test]
fn image_upload_and_readback_ct32() {
    let mut gs = Gs::new();
    let w = 16;
    let h = 8;
    let data: Vec<u8> = (0..w * h)
        .flat_map(|i: u32| (i * 0x01010101 ^ 0xA5000000).to_le_bytes())
        .collect();
    upload(&mut gs, 64, 1, PSMCT32, 3, 2, w, h, &data);
    assert_eq!(
        read_pixel(&gs.vram, PSMCT32, 64, 1, 3, 2),
        u32::from_le_bytes(data[0..4].try_into().unwrap())
    );
    assert_eq!(
        read_pixel(&gs.vram, PSMCT32, 64, 1, 4, 2),
        u32::from_le_bytes(data[4..8].try_into().unwrap())
    );
    assert_eq!(
        read_pixel(&gs.vram, PSMCT32, 64, 1, 3, 3),
        u32::from_le_bytes(data[64..68].try_into().unwrap())
    );
    let back = download(&mut gs, 64, 1, PSMCT32, 3, 2, w, h);
    assert_eq!(back, data);
}

#[test]
fn image_upload_split_across_writes() {
    let mut gs = Gs::new();
    let data: Vec<u8> = (0..256u32).map(|i| (i * 13) as u8).collect();
    ad(
        &mut gs,
        &[
            (0x50, 0u64 << 32 | 2u64 << 48 | (PSMT8 as u64) << 56),
            (0x51, 0),
            (0x52, 16 | 16 << 32),
            (0x53, 0),
        ],
    );
    let q = bytes_to_qwords(&data);
    gs.gif_write(3, &[giftag(q.len() as u32, true, 2, 0, 0)]);
    for chunk in q.chunks(3) {
        gs.gif_write(3, chunk);
    }
    assert!(!gs.gif_path_busy(3));
    let back = download(&mut gs, 0, 2, PSMT8, 0, 0, 16, 16);
    assert_eq!(back, data);
}

#[test]
fn image_roundtrip_all_formats() {
    for (psm, bw) in [
        (PSMCT24, 1),
        (PSMCT16, 1),
        (PSMCT16S, 1),
        (PSMT8, 2),
        (PSMT4, 2),
        (PSMT8H, 1),
        (PSMT4HL, 1),
        (PSMT4HH, 1),
        (PSMZ32, 1),
        (PSMZ16, 1),
    ] {
        let mut gs = Gs::new();
        let (w, h) = (32u32, 8u32);
        let n = (w * h * bits_per_pixel(psm)).div_ceil(8) as usize;
        let data: Vec<u8> = (0..n).map(|i| (i * 37 + 11) as u8).collect();
        upload(&mut gs, 128, bw, psm, 8, 4, w, h, &data);
        let back = download(&mut gs, 128, bw, psm, 8, 4, w, h);
        assert_eq!(&back[..n], &data[..], "psm {psm:#x}");
    }
}

#[test]
fn local_to_local_copy() {
    let mut gs = Gs::new();
    let data: Vec<u8> = (0..16 * 16 * 4).map(|i| i as u8).collect();
    upload(&mut gs, 0, 1, PSMCT32, 0, 0, 16, 16, &data);
    ad(
        &mut gs,
        &[
            (0x50, 0 | 1 << 16 | 256u64 << 32 | 1 << 48),
            (0x51, 0 | (5u64 << 32) | (7u64 << 48)),
            (0x52, 16 | 16 << 32),
            (0x53, 2),
        ],
    );
    for y in 0..16 {
        for x in 0..16 {
            assert_eq!(
                read_pixel(&gs.vram, PSMCT32, 256, 1, x + 5, y + 7),
                read_pixel(&gs.vram, PSMCT32, 0, 1, x, y)
            );
        }
    }
}

// ------------------------------------------------------------------ drawing

#[test]
fn flat_sprite_fill() {
    let mut gs = Gs::new();
    setup_frame(&mut gs);
    sprite(&mut gs, 10, 20, 50, 60, 0, rgba(0x11, 0x22, 0x33, 0x80));
    assert_eq!(fb(&gs, 10, 20), 0x8033_2211);
    assert_eq!(fb(&gs, 49, 59), 0x8033_2211);
    assert_eq!(fb(&gs, 50, 30), 0);
    assert_eq!(fb(&gs, 30, 60), 0);
    assert_eq!(fb(&gs, 9, 30), 0);
    let count = (0..100)
        .flat_map(|y| (0..100).map(move |x| (x, y)))
        .filter(|&(x, y)| fb(&gs, x, y) != 0)
        .count();
    assert_eq!(count, 40 * 40);
}

#[test]
fn xyoffset_and_scissor() {
    let mut gs = Gs::new();
    setup_frame(&mut gs);
    ad(
        &mut gs,
        &[
            (0x18, (2048u64 * 16) | (2048u64 * 16) << 32),
            (0x40, 5 | 30 << 16 | 5 << 32 | 30 << 48),
        ],
    );
    sprite(
        &mut gs,
        2048,
        2048,
        2048 + 40,
        2048 + 40,
        0,
        rgba(255, 0, 0, 0),
    );
    assert_eq!(fb(&gs, 4, 10), 0);
    assert_eq!(fb(&gs, 5, 5), 0xFF);
    assert_eq!(fb(&gs, 30, 30), 0xFF);
    assert_eq!(fb(&gs, 31, 30), 0);
}

#[test]
fn gouraud_triangle() {
    let mut gs = Gs::new();
    setup_frame(&mut gs);
    ad(
        &mut gs,
        &[
            (0x00, 3 | 1 << 3), // triangle, IIP
            (0x01, rgba(255, 0, 0, 0)),
            (0x05, xyz(0, 0, 0)),
            (0x01, rgba(0, 255, 0, 0)),
            (0x05, xyz(64, 0, 0)),
            (0x01, rgba(0, 0, 255, 0)),
            (0x05, xyz(0, 64, 0)),
        ],
    );
    assert_eq!(fb(&gs, 0, 0), 0x0000_00FF);
    let mid = fb(&gs, 32, 0);
    let (r, g, b) = (mid & 0xFF, (mid >> 8) & 0xFF, (mid >> 16) & 0xFF);
    assert!(
        (126..=129).contains(&r) && (126..=129).contains(&g) && b == 0,
        "{mid:#x}"
    );
    let c = fb(&gs, 0, 32);
    assert!(
        (126..=129).contains(&(c & 0xFF)) && (126..=129).contains(&((c >> 16) & 0xFF)),
        "{c:#x}"
    );
    // Top-left fill rule: x + y < 64 exactly.
    let mut count = 0;
    for y in 0..70 {
        for x in 0..70 {
            let inside = fb(&gs, x, y) != 0 || (x == 0 && y == 0);
            if inside {
                count += 1;
                assert!(x + y < 64, "({x},{y}) drawn");
            }
        }
    }
    assert_eq!(count, 64 * 65 / 2);
}

#[test]
fn shared_edges_draw_once() {
    let mut gs = Gs::new();
    setup_frame(&mut gs);
    // Additive blend: Cv = (Cs - 0) * FIX(128) >> 7 + Cd.
    ad(
        &mut gs,
        &[(0x42, 0 | 2 << 2 | 2 << 4 | 1 << 6 | 128u64 << 32)],
    );
    let pts = [(3u32, 2u32), (61, 7), (9, 50), (57, 59)];
    ad(
        &mut gs,
        &[
            (0x00, 4 | 1 << 6), // tri strip, ABE
            (0x01, rgba(1, 1, 1, 0x80)),
            (0x05, xyz(pts[0].0, pts[0].1, 0)),
            (0x05, xyz(pts[1].0, pts[1].1, 0)),
            (0x05, xyz(pts[2].0, pts[2].1, 0)),
            (0x05, xyz(pts[3].0, pts[3].1, 0)),
        ],
    );
    for y in 0..64 {
        for x in 0..64 {
            let c = fb(&gs, x, y) & 0xFF;
            assert!(c <= 1, "pixel ({x},{y}) drawn {c} times");
        }
    }
    assert_eq!(fb(&gs, 30, 30) & 0xFF, 1);
}

#[test]
fn textured_sprite_psmt8_csm1() {
    let mut gs = Gs::new();
    setup_frame(&mut gs);
    let pal = |i: u32| 0x8000_0000 | (i * 0x010203 & 0xFF_FFFF);
    // CLUT at block 0x3000 in CSM1 order (index bits 3 and 4 swapped).
    let swap = |i: u32| (i & !0x18) | ((i & 0x08) << 1) | ((i & 0x10) >> 1);
    let clut: Vec<u8> = (0..256).flat_map(|p| pal(swap(p)).to_le_bytes()).collect();
    upload(&mut gs, 0x3000, 1, PSMCT32, 0, 0, 16, 16, &clut);
    // 16x16 PSMT8 texture at block 0x2000 (TBW 2).
    let tex: Vec<u8> = (0..256).map(|i| i as u8).collect();
    upload(&mut gs, 0x2000, 2, PSMT8, 0, 0, 16, 16, &tex);
    let tex0 = 0x2000u64
        | 2 << 14
        | (PSMT8 as u64) << 20
        | 4 << 26
        | 4 << 30
        | 1 << 34
        | 1 << 35
        | 0x3000u64 << 37
        | 1 << 61;
    ad(
        &mut gs,
        &[
            (0x06, tex0),
            (0x00, 6 | 1 << 4 | 1 << 8), // sprite, TME, FST
            (0x01, rgba(128, 128, 128, 128)),
            (0x03, 0),
            (0x05, xyz(0, 0, 0)),
            (0x03, 16 * 16 | (16 * 16) << 16),
            (0x05, xyz(16, 16, 0)),
        ],
    );
    for y in 0..16 {
        for x in 0..16 {
            assert_eq!(fb(&gs, x, y), pal(x + y * 16), "({x},{y})");
        }
    }
}

#[test]
fn textured_sprite_psmt4_ct16_clut_modulate() {
    let mut gs = Gs::new();
    setup_frame(&mut gs);
    // CT16 CLUT, 8x2 layout, CSA 1 (entries 16..31).
    let c16 = |i: u32| (i & 0x1F) | (31 - i) << 10 | 0x8000;
    let clut: Vec<u8> = (0..16)
        .flat_map(|i| (c16(i) as u16).to_le_bytes())
        .collect();
    upload(&mut gs, 0x3000, 1, PSMCT16, 0, 0, 8, 2, &clut);
    let tex: Vec<u8> = (0..32)
        .map(|i| ((i * 2) & 15 | ((i * 2 + 1) & 15) << 4) as u8)
        .collect();
    upload(&mut gs, 0x2000, 2, PSMT4, 0, 0, 16, 4, &tex);
    let tex0 = 0x2000u64
        | 2 << 14
        | (PSMT4 as u64) << 20
        | 4 << 26
        | 2 << 30
        | 1 << 34
        | 0x3000u64 << 37
        | 2 << 51
        | 1 << 56
        | 1 << 61;
    ad(
        &mut gs,
        &[
            (0x3B, 0x40 | 0x80u64 << 32), // TEXA: TA0 0x40, TA1 0x80
            (0x06, tex0),
            (0x00, 6 | 1 << 4 | 1 << 8),
            (0x01, rgba(128, 128, 128, 128)),
            (0x03, 0),
            (0x05, xyz(0, 0, 0)),
            (0x03, 16 * 16 | (4 * 16) << 16),
            (0x05, xyz(16, 4, 0)),
        ],
    );
    for x in 0..16 {
        let e = c16(x);
        let want = ((e & 0x1F) << 3) | ((e >> 10) & 0x1F) << 19 | 0x80 << 24;
        assert_eq!(fb(&gs, x, 0), want, "x {x}");
    }
}

#[test]
fn perspective_texture_stq() {
    let mut gs = Gs::new();
    setup_frame(&mut gs);
    let data: Vec<u8> = (0..16 * 16u32)
        .flat_map(|i| (i | 0xFF00_0000).to_le_bytes())
        .collect();
    upload(&mut gs, 0x2000, 1, PSMCT32, 0, 0, 16, 16, &data);
    let tex0 = 0x2000u64 | 1 << 14 | 4 << 26 | 4 << 30 | 1 << 34 | 1 << 35;
    let st = |s: f32, t: f32| s.to_bits() as u64 | (t.to_bits() as u64) << 32;
    let q = |q: f32| (q.to_bits() as u64) << 32 | 0x8080_8080;
    ad(
        &mut gs,
        &[
            (0x06, tex0),
            (0x00, 6 | 1 << 4), // sprite, TME, STQ
            (0x02, st(0.0, 0.0)),
            (0x01, q(2.0)),
            (0x05, xyz(0, 0, 0)),
            (0x02, st(2.0, 2.0)),
            (0x01, q(2.0)),
            (0x05, xyz(16, 16, 0)),
        ],
    );
    // s/q = 0..1 across 16 pixels -> texel x == pixel x.
    assert_eq!(fb(&gs, 5, 7), 5 + 7 * 16 | 0xFF00_0000);
}

#[test]
fn z_test_gequal() {
    let mut gs = Gs::new();
    setup_frame(&mut gs);
    // ZBUF at page 200, Z32; TEST: ZTE, ZTST GEQUAL.
    ad(&mut gs, &[(0x4E, 200), (0x47, 1 << 16 | 2 << 17)]);
    sprite(&mut gs, 0, 0, 8, 8, 100, rgba(10, 0, 0, 0));
    sprite(&mut gs, 0, 0, 8, 8, 50, rgba(20, 0, 0, 0));
    assert_eq!(fb(&gs, 3, 3) & 0xFF, 10);
    assert_eq!(read_pixel(&gs.vram, PSMZ32, 200 * 32, 10, 3, 3), 100);
    sprite(&mut gs, 0, 0, 8, 8, 100, rgba(30, 0, 0, 0));
    assert_eq!(fb(&gs, 3, 3) & 0xFF, 30);
    // ZMSK: test but do not write.
    ad(&mut gs, &[(0x4E, 200 | 1 << 32)]);
    sprite(&mut gs, 0, 0, 8, 8, 500, rgba(40, 0, 0, 0));
    assert_eq!(fb(&gs, 3, 3) & 0xFF, 40);
    assert_eq!(read_pixel(&gs.vram, PSMZ32, 200 * 32, 10, 3, 3), 100);
    // Z16: values clamp to 0xFFFF.
    ad(&mut gs, &[(0x4E, 200 | 2 << 24)]);
    sprite(&mut gs, 0, 0, 8, 8, 0x12345, rgba(50, 0, 0, 0));
    assert_eq!(read_pixel(&gs.vram, PSMZ16, 200 * 32, 10, 3, 3), 0xFFFF);
}

#[test]
fn alpha_test_and_blend() {
    let mut gs = Gs::new();
    setup_frame(&mut gs);
    sprite(&mut gs, 0, 0, 8, 8, 0, rgba(200, 100, 0, 0x80));
    // (Cs - Cd) * As >> 7 + Cd with As = 0x40.
    ad(
        &mut gs,
        &[
            (0x42, 0 | 1 << 2 | 0 << 4 | 1 << 6),
            (0x00, 6 | 1 << 6),
            (0x01, rgba(0, 0, 200, 0x40)),
        ],
    );
    ad(&mut gs, &[(0x05, xyz(0, 0, 0)), (0x05, xyz(8, 8, 0))]);
    let c = fb(&gs, 2, 2);
    assert_eq!(c & 0xFF_FFFF, 100 | 50 << 8 | 100 << 16, "{c:#x}");
    assert_eq!(c >> 24, 0x40);
    // Alpha test: ATST GREATER than 0x50, AFAIL KEEP -> rejected.
    ad(&mut gs, &[(0x47, 1 | 6 << 1 | 0x50 << 4)]);
    sprite(&mut gs, 0, 0, 8, 8, 0, rgba(1, 2, 3, 0x40));
    assert_eq!(fb(&gs, 2, 2), c);
    // AFAIL FB_ONLY: written even though the test fails.
    ad(&mut gs, &[(0x47, 1 | 6 << 1 | 0x50 << 4 | 1 << 12)]);
    sprite(&mut gs, 0, 0, 8, 8, 0, rgba(1, 2, 3, 0x40));
    assert_eq!(fb(&gs, 2, 2), 0x4003_0201);
}

#[test]
fn destination_alpha_test_and_fbmsk() {
    let mut gs = Gs::new();
    setup_frame(&mut gs);
    sprite(&mut gs, 0, 0, 4, 8, 0, rgba(1, 1, 1, 0x80));
    sprite(&mut gs, 4, 0, 8, 8, 0, rgba(2, 2, 2, 0x00));
    // DATE, DATM=0: only pixels with dst alpha bit clear pass.
    ad(&mut gs, &[(0x47, 1 << 14)]);
    sprite(&mut gs, 0, 0, 8, 8, 0, rgba(9, 9, 9, 0x00));
    assert_eq!(fb(&gs, 1, 1) & 0xFF, 1);
    assert_eq!(fb(&gs, 5, 1) & 0xFF, 9);
    // FBMSK protects the masked bits.
    ad(
        &mut gs,
        &[(0x47, 0), (0x4C, 10 << 16 | 0xFFFF_00FFu64 << 32)],
    );
    sprite(&mut gs, 0, 0, 8, 8, 0, rgba(0x77, 0x66, 0x55, 0x44));
    assert_eq!(fb(&gs, 1, 1), 0x8001_6601);
}

#[test]
fn frame_ct16_write() {
    let mut gs = Gs::new();
    ad(
        &mut gs,
        &[
            (0x4C, 10 << 16 | (PSMCT16 as u64) << 24),
            (0x40, 639 << 16 | 447 << 48),
        ],
    );
    sprite(&mut gs, 0, 0, 4, 4, 0, rgba(0xF8, 0x08, 0x80, 0x80));
    assert_eq!(
        read_pixel(&gs.vram, PSMCT16, 0, 10, 1, 1),
        0x1F | 1 << 5 | 0x10 << 10 | 0x8000
    );
}

#[test]
fn xyz3_does_not_kick_and_adc() {
    let mut gs = Gs::new();
    setup_frame(&mut gs);
    ad(
        &mut gs,
        &[
            (0x00, 6),
            (0x01, rgba(255, 255, 255, 0)),
            (0x0D, xyz(0, 0, 0)),
            (0x0D, xyz(8, 8, 0)),
        ],
    );
    assert_eq!(fb(&gs, 1, 1), 0);
    // PACKED XYZ2 with ADC set behaves as XYZ3.
    let tag = giftag(3, true, 0, 3, 0x1 | 0x5 << 4 | 0x5 << 8);
    let col = 255u128 | 255 << 32 | 255 << 64;
    let v0 = 0u128 | 1 << 111;
    let v1 = (8 * 16) as u128 | ((8 * 16) as u128) << 32 | 1 << 111;
    gs.gif_write(3, &[tag, col, v0, v1]);
    assert_eq!(fb(&gs, 1, 1), 0);
    gs.write_reg(0x05, xyz(8, 8, 0));
    assert_eq!(fb(&gs, 1, 1), 0); // vertex queue continued: (8,8)-(8,8) empty
}

#[test]
fn packed_rgbaq_uses_internal_q_and_pre() {
    let mut gs = Gs::new();
    setup_frame(&mut gs);
    // PRE=1 sets PRIM to sprite; regs: ST, RGBAQ, XYZ2, XYZ2.
    let lo: u64 = 1 | 1 << 15 | 1 << 46 | 6 << 47 | 4u64 << 60;
    let tag = lo as u128 | ((0x2u128 | 0x1 << 4 | 0x5 << 8 | 0x5 << 12) << 64);
    let st = 0u128 | (3.5f32.to_bits() as u128) << 64;
    let col = 9u128 | 8 << 32 | 7 << 64 | 6 << 96;
    let v0 = 0u128;
    let v1 = (4 * 16) as u128 | ((4 * 16) as u128) << 32;
    gs.gif_write(3, &[tag, st, col, v0, v1]);
    assert_eq!(fb(&gs, 1, 1), 0x0607_0809);
    assert_eq!(f32::from_bits((gs.regs.rgbaq >> 32) as u32), 3.5);
}

#[test]
fn reglist_mode() {
    let mut gs = Gs::new();
    setup_frame(&mut gs);
    // NREG 3 (PRIM, RGBAQ, XYZ2), NLOOP 1 + a second loop of XYZ2 via a new tag.
    let tag = giftag(1, false, 1, 3, 0x0 | 0x1 << 4 | 0x5 << 8);
    let q0 = 6u128 | (rgba(5, 6, 7, 8) as u128) << 64;
    let q1 = xyz(0, 0, 0) as u128; // upper half padding
    gs.gif_write(3, &[tag, q0, q1]);
    let tag2 = giftag(1, true, 1, 1, 0x5);
    gs.gif_write(3, &[tag2, xyz(2, 2, 0) as u128]);
    assert_eq!(fb(&gs, 1, 1), 0x0807_0605);
}

#[test]
fn fog() {
    let mut gs = Gs::new();
    setup_frame(&mut gs);
    ad(
        &mut gs,
        &[
            (0x3D, 0x0000FF),
            (0x00, 6 | 1 << 5),
            (0x01, rgba(0, 0, 0, 0)),
            (0x0A, 128u64 << 56),
        ],
    );
    ad(&mut gs, &[(0x05, xyz(0, 0, 0)), (0x05, xyz(4, 4, 0))]);
    // (128*0 + 127*255) >> 8 = 126
    assert_eq!(fb(&gs, 1, 1) & 0xFF, 126);
}

// ------------------------------------------------------------------ privileged / display

#[test]
fn csr_signal_finish_and_imr() {
    let mut gs = Gs::new();
    let csr = gs.read_priv(0x1000);
    assert_eq!(csr >> 16 & 0xFFFF, 0x551B);
    assert!(!gs.irq_pending());
    ad(
        &mut gs,
        &[
            (0x60, 0xFFFF_FFFF_0000_1234),
            (0x61, 0),
            (0x62, 0xFFFF_FFFF_0000_5678),
        ],
    );
    assert_eq!(gs.read_priv(0x1000) & 3, 3);
    assert_eq!(gs.read_priv(0x1080), 0x5678_0000_1234);
    assert!(!gs.irq_pending()); // masked by default IMR 0x7F00
    gs.write_priv(0x1010, 0x7F00 & !(1 << 9)); // unmask FINISH
    assert!(gs.irq_pending());
    gs.write_priv(0x1000, 2);
    assert!(!gs.irq_pending());
    assert_eq!(gs.read_priv(0x1000) & 3, 1);
    gs.write_priv(0x1010, 0x7F00 & !(1 << 11));
    gs.vblank(1);
    assert!(gs.irq_pending());
    assert_ne!(gs.read_priv(0x1000) & (1 << 13), 0);
    gs.write_priv(0x1000, 8);
    gs.vblank(0);
    assert_eq!(gs.read_priv(0x1000) & (1 << 13), 0);
}

#[test]
fn display_filled_framebuffer() {
    let mut gs = Gs::new();
    setup_frame(&mut gs);
    sprite(&mut gs, 0, 0, 640, 448, 0, rgba(10, 20, 30, 0));
    sprite(&mut gs, 100, 50, 101, 51, 0, rgba(1, 2, 3, 0));
    gs.write_priv(0x00, 0x1 | 0x20 | 0xFF << 8); // EN1, MMOD, ALP 0xFF
    gs.write_priv(0x70, 10 << 9); // DISPFB1: FBP 0, FBW 10, CT32
    gs.write_priv(0x80, 2559u64 << 32 | 447u64 << 44 | 3 << 23); // DISPLAY1
    let mut out = Vec::new();
    let (w, h) = gs.display(&mut out);
    assert_eq!((w, h), (640, 448));
    assert_eq!(out.len(), 640 * 448);
    assert_eq!(out[0], 0xFF1E_140A);
    assert_eq!(out[50 * 640 + 100], 0xFF03_0201);
    // Interlaced field mode keeps a 640x448 output (line doubled).
    gs.write_priv(0x20, 3);
    let (w, h) = gs.display(&mut out);
    assert_eq!((w, h), (640, 448));
    assert_eq!(out[100 * 640 + 100], 0xFF03_0201);
    assert_eq!(out[101 * 640 + 100], 0xFF03_0201);
}

#[test]
fn csr_reset_clears_gif_state() {
    let mut gs = Gs::new();
    gs.gif_write(3, &[giftag(5, true, 0, 1, 0xE)]);
    assert!(gs.gif_path_busy(3));
    gs.write_priv(0x1000, 1 << 9);
    assert!(!gs.gif_path_busy(3));
}

/// Rough throughput check: `cargo test -p ssx3-ps2 --release -- --ignored perf --nocapture`.
#[test]
#[ignore]
fn perf_fullscreen() {
    let mut gs = Gs::new();
    setup_frame(&mut gs);
    let data: Vec<u8> = (0..64 * 64u32)
        .flat_map(|i| (i * 0x9E37).to_le_bytes())
        .collect();
    upload(&mut gs, 0x2000, 1, PSMCT32, 0, 0, 64, 64, &data);
    let tex0 = 0x2000u64 | 1 << 14 | 6 << 26 | 6 << 30 | 1 << 34;
    ad(
        &mut gs,
        &[
            (0x06, tex0),
            (0x4E, 300),
            (0x47, 1 << 16 | 1 << 17),
            (0x42, 0 | 1 << 2 | 0 << 4 | 1 << 6),
        ],
    );
    let t = std::time::Instant::now();
    for _ in 0..10 {
        ad(
            &mut gs,
            &[
                (0x00, 4 | 1 << 3 | 1 << 4 | 1 << 6 | 1 << 8),
                (0x01, rgba(255, 0, 0, 0x40)),
                (0x03, 0),
                (0x05, xyz(0, 0, 10)),
                (0x01, rgba(0, 255, 0, 0x40)),
                (0x03, 640 * 16),
                (0x05, xyz(640, 0, 10)),
                (0x01, rgba(0, 0, 255, 0x40)),
                (0x03, (448 * 16) << 16),
                (0x05, xyz(0, 448, 10)),
                (0x03, 640 * 16 | (448 * 16) << 16),
                (0x05, xyz(640, 448, 10)),
            ],
        );
    }
    let dt = t.elapsed();
    println!(
        "10 full-screen textured/blended/z quads: {dt:?} ({:.1} Mpix/s)",
        10.0 * 640.0 * 448.0 / dt.as_secs_f64() / 1e6
    );
}

// ------------------------------------------------------------------ mipmapping

/// Solid-colour CT32 mip chain: 16x16 level 0 at block 0x2000 (TBW 1),
/// 8x8 level 1 at 0x2100, 4x4 level 2 at 0x2200 (MIPTBP1, TBW 1).
const MIP_COLORS: [u32; 3] = [0x8010_2030, 0x8040_5060, 0x8070_8090];

fn mip_setup(gs: &mut Gs) {
    setup_frame(gs);
    for (l, base) in [0x2000u32, 0x2100, 0x2200].into_iter().enumerate() {
        let n = 16 >> l;
        let data: Vec<u8> = (0..n * n).flat_map(|_| MIP_COLORS[l].to_le_bytes()).collect();
        upload(gs, base, 1, PSMCT32, 0, 0, n, n, &data);
    }
    let miptbp1 = 0x2100u64 | 1 << 14 | 0x2200u64 << 20 | 1 << 34;
    ad(gs, &[(0x34, miptbp1)]);
}

/// TEX1: LCM, MXL, MMAG, MMIN, L and K (in 1/16 units).
fn tex1(lcm: bool, mxl: u64, mmin: u64, l: u64, k16: i32) -> u64 {
    lcm as u64 | mxl << 2 | 1 << 5 | mmin << 6 | l << 19 | ((k16 as u64) & 0xFFF) << 32
}

/// Draw an 8x8 decal sprite with the mip chain; returns the pixel colour.
fn mip_sprite(gs: &mut Gs, tex1: u64, q: f32, fst: bool) -> u32 {
    let tex0 = 0x2000u64 | 1 << 14 | (PSMCT32 as u64) << 20 | 4 << 26 | 4 << 30 | 1 << 34 | 1 << 35;
    let rgbaq = 0x8080_8080u64 | (q.to_bits() as u64) << 32;
    let st = |s: f32, t: f32| (s * q).to_bits() as u64 | ((t * q).to_bits() as u64) << 32;
    ad(
        gs,
        &[
            (0x06, tex0),
            (0x14, tex1),
            (0x00, 6 | 1 << 4 | (fst as u64) << 8),
            (0x01, rgbaq),
            (0x02, st(0.0, 0.0)),
            (0x03, 0),
            (0x05, xyz(0, 0, 0)),
            (0x02, st(1.0, 1.0)),
            (0x03, 16 * 16 | (16 * 16) << 16),
            (0x05, xyz(8, 8, 0)),
        ],
    );
    fb(gs, 3, 3)
}

fn mix(a: u32, b: u32, w: u32) -> u32 {
    (0..4).fold(0, |o, i| {
        let (x, y) = ((a >> (i * 8)) & 0xFF, (b >> (i * 8)) & 0xFF);
        o | ((x * (256 - w) + y * w + 128) >> 8) << (i * 8)
    })
}

#[test]
fn mipmap_level_selection_lcm() {
    let mut gs = Gs::new();
    mip_setup(&mut gs);
    // NEAREST_MIPMAP_NEAREST, LOD = K.
    assert_eq!(mip_sprite(&mut gs, tex1(true, 2, 2, 0, 16), 1.0, true), MIP_COLORS[1]);
    assert_eq!(mip_sprite(&mut gs, tex1(true, 2, 2, 0, 24), 1.0, true), MIP_COLORS[2]);
    // LOD clamps to MXL.
    assert_eq!(mip_sprite(&mut gs, tex1(true, 1, 2, 0, 80), 1.0, true), MIP_COLORS[1]);
    // LOD <= 0 magnifies level 0.
    assert_eq!(mip_sprite(&mut gs, tex1(true, 2, 2, 0, -16), 1.0, true), MIP_COLORS[0]);
    // MIPMAP_LINEAR blends floor(LOD) and the next level by the fraction.
    let got = mip_sprite(&mut gs, tex1(true, 2, 3, 0, 8), 1.0, true);
    assert_eq!(got, mix(MIP_COLORS[0], MIP_COLORS[1], 128));
    let got = mip_sprite(&mut gs, tex1(true, 2, 5, 0, 20), 1.0, true);
    assert_eq!(got, mix(MIP_COLORS[1], MIP_COLORS[2], 64));
    // Without a mipmap filter (MMIN 1) level 0 is used whatever the LOD.
    assert_eq!(mip_sprite(&mut gs, tex1(true, 2, 1, 0, 32), 1.0, true), MIP_COLORS[0]);
}

#[test]
fn mipmap_lod_from_q() {
    let mut gs = Gs::new();
    mip_setup(&mut gs);
    // LOD = log2(1/Q) << L + K: Q = 1/4 -> 2; Q = 1/2, L = 1 -> 2; K shifts.
    assert_eq!(mip_sprite(&mut gs, tex1(false, 2, 2, 0, 0), 0.25, false), MIP_COLORS[2]);
    assert_eq!(mip_sprite(&mut gs, tex1(false, 2, 2, 1, 0), 0.5, false), MIP_COLORS[2]);
    assert_eq!(mip_sprite(&mut gs, tex1(false, 2, 2, 0, -16), 0.25, false), MIP_COLORS[1]);
    assert_eq!(mip_sprite(&mut gs, tex1(false, 2, 2, 0, 0), 1.0, false), MIP_COLORS[0]);
}

#[test]
fn mipmap_mtba_addresses() {
    let mut gs = Gs::new();
    mip_setup(&mut gs);
    // MTBA: level 1 follows level 0 (16x16x32 bits = 4 blocks), TBW 1.
    let data: Vec<u8> = (0..64).flat_map(|_| 0x80AA_BBCCu32.to_le_bytes()).collect();
    upload(&mut gs, 0x2004, 1, PSMCT32, 0, 0, 8, 8, &data);
    let t1 = tex1(true, 1, 2, 0, 16) | 1 << 9;
    assert_eq!(mip_sprite(&mut gs, t1, 1.0, true), 0x80AA_BBCC);
}

/// A host->local CT32 upload of an 8x8 block of `color` at `dbp`, as PATH3
/// DMA data: an A+D packet without EOP, then the IMAGE packet with EOP.
fn upload_packets(dbp: u32, color: u32) -> Vec<Qword> {
    let mut q = vec![giftag(4, false, 0, 1, 0xE)];
    q.push(ad_q(0x50, (dbp as u64) << 32 | 1u64 << 48));
    q.push(ad_q(0x51, 0));
    q.push(ad_q(0x52, 8 | 8u64 << 32));
    q.push(ad_q(0x53, 0));
    let data: Vec<u8> = (0..64).flat_map(|_| color.to_le_bytes()).collect();
    let img = bytes_to_qwords(&data);
    q.push(giftag(img.len() as u32, true, 2, 0, 0));
    q.extend(img);
    q
}

fn texel(gs: &Gs, dbp: u32) -> u32 {
    read_pixel(&gs.vram, PSMCT32, dbp, 1, 3, 3)
}

#[test]
fn path3_mask_holds_dma_until_unmasked() {
    // SSX 3 queues several uploads to the same block in one GIF DMA chain
    // and releases them one by one with MSKPATH3 from the VIF1 stream.
    let mut gs = Gs::new_cpu();
    gs.set_path3_mask(true);
    gs.path3_dma(&upload_packets(0x100, 0x1111_1111));
    assert_eq!(texel(&gs, 0x100), 0, "masked PATH3 must not reach VRAM");
    assert!(gs.path3_pending() > 0);
    assert_eq!(gs.gif_stat() & 2, 2, "M3P");
    gs.set_path3_mask(false);
    assert_eq!(texel(&gs, 0x100), 0x1111_1111);
    assert_eq!(gs.path3_pending(), 0);
    // Masking again holds the next upload while the first stays visible.
    gs.set_path3_mask(true);
    gs.path3_dma(&upload_packets(0x100, 0x2222_2222));
    assert_eq!(texel(&gs, 0x100), 0x1111_1111);
    gs.set_path3_mask(false);
    assert_eq!(texel(&gs, 0x100), 0x2222_2222);
    // GIF_MODE.M3R masks as well.
    gs.set_path3_mask_reg(true);
    gs.path3_dma(&upload_packets(0x100, 0x3333_3333));
    assert_eq!(texel(&gs, 0x100), 0x2222_2222);
    gs.set_path3_mask_reg(false);
    assert_eq!(texel(&gs, 0x100), 0x3333_3333);
}

#[test]
fn path3_mask_waits_for_end_of_packet() {
    // A mask set in the middle of a packet stops PATH3 only after the EOP
    // tag's data; a tag without EOP does not end the packet.
    let mut gs = Gs::new_cpu();
    let a = upload_packets(0x100, 0x4444_4444);
    let b = upload_packets(0x200, 0x5555_5555);
    gs.path3_dma(&a[..3]); // A+D tag and two of its registers
    gs.set_path3_mask(true);
    let mut rest = a[3..].to_vec();
    rest.extend(&b);
    gs.path3_dma(&rest);
    assert_eq!(texel(&gs, 0x100), 0x4444_4444, "the open packet completes");
    assert_eq!(texel(&gs, 0x200), 0, "the next packet waits");
    assert_eq!(gs.path3_pending(), b.len());
    gs.set_path3_mask(false);
    assert_eq!(texel(&gs, 0x200), 0x5555_5555);
}

#[test]
fn path3_unmask_pulse_releases_one_packet() {
    // `MSKPATH3 0` immediately followed by `MSKPATH3 1` (SSX 3's VIF1
    // stream before each textured batch) lets exactly one packet through.
    let mut gs = Gs::new_cpu();
    gs.set_path3_mask(true);
    let mut q = upload_packets(0x100, 0x6666_6666);
    q.extend(upload_packets(0x100, 0x7777_7777));
    gs.path3_dma(&q);
    assert_eq!(texel(&gs, 0x100), 0);
    gs.set_path3_mask(false);
    gs.set_path3_mask(true);
    assert_eq!(texel(&gs, 0x100), 0x6666_6666, "first texture for the first batch");
    gs.set_path3_mask(false);
    gs.set_path3_mask(true);
    assert_eq!(texel(&gs, 0x100), 0x7777_7777, "second texture for the next batch");
    assert_eq!(gs.path3_pending(), 0);
    // Left unmasked, the VIF moving on lets everything flow.
    gs.path3_dma(&upload_packets(0x100, 0x1234_5678));
    gs.path3_dma(&upload_packets(0x200, 0x1234_5678));
    gs.set_path3_mask(false);
    gs.path3_flow();
    assert_eq!(texel(&gs, 0x200), 0x1234_5678);
}
