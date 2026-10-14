use super::*;
use crate::vu::reference;

fn pack(words: &[u32]) -> Vec<Qword> {
    let mut w = words.to_vec();
    while !w.len().is_multiple_of(4) {
        w.push(0); // NOP padding
    }
    w.chunks_exact(4)
        .map(|c| c[0] as u128 | (c[1] as u128) << 32 | (c[2] as u128) << 64 | (c[3] as u128) << 96)
        .collect()
}

fn qw(vu: &Vu, addr: usize) -> [u32; 4] {
    let o = addr * 16;
    std::array::from_fn(|i| {
        u32::from_le_bytes(vu.data[o + i * 4..o + i * 4 + 4].try_into().unwrap())
    })
}

fn unpack(vn: u32, vl: u32, num: u32, addr: u32, usn: bool, flg: bool, masked: bool) -> u32 {
    0x6000_0000
        | (masked as u32) << 28
        | vn << 26
        | vl << 24
        | num << 16
        | (flg as u32) << 15
        | (usn as u32) << 14
        | addr
}

fn setup() -> (Vif, Vu, Gs) {
    let mut vu = Vu::new(1);
    vu.programs.clear();
    vu.fallback = Some(reference::run_program);
    (Vif::new(1), vu, Gs::new())
}

const STCYCL_44: u32 = 0x0100_0404;

#[test]
fn unpack_v4_32() {
    let (mut vif, mut vu, mut gs) = setup();
    let s = [
        STCYCL_44,
        unpack(3, 0, 2, 5, false, false, false),
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
    ];
    vif.write(&pack(&s), &mut vu, &mut gs);
    assert_eq!(qw(&vu, 5), [1, 2, 3, 4]);
    assert_eq!(qw(&vu, 6), [5, 6, 7, 8]);
    assert_eq!(vif.pending_words(), 0); // trailing NOP padding consumed
}

#[test]
fn unpack_v3_s32_v2_16_usn() {
    let (mut vif, mut vu, mut gs) = setup();
    let s = [
        STCYCL_44,
        unpack(2, 0, 2, 0, false, false, false), // V3-32
        1,
        2,
        3,
        4,
        5,
        6,
        unpack(0, 0, 2, 2, false, false, false), // S-32
        9,
        10,
        unpack(1, 1, 2, 4, false, false, false), // V2-16 signed
        0xffff_8000,
        0x0001_7fff,
        unpack(1, 1, 2, 6, true, false, false), // V2-16 unsigned
        0xffff_8000,
        0x0001_7fff,
    ];
    vif.write(&pack(&s), &mut vu, &mut gs);
    assert_eq!(&qw(&vu, 0)[..3], &[1, 2, 3]);
    assert_eq!(&qw(&vu, 1)[..3], &[4, 5, 6]);
    assert_eq!(qw(&vu, 2), [9; 4]);
    assert_eq!(qw(&vu, 3), [10; 4]);
    assert_eq!(
        qw(&vu, 4),
        [0xffff_8000, 0xffff_ffff, 0xffff_8000, 0xffff_ffff]
    );
    assert_eq!(qw(&vu, 5), [0x7fff, 1, 0x7fff, 1]);
    assert_eq!(qw(&vu, 6), [0x8000, 0xffff, 0x8000, 0xffff]);
}

#[test]
fn unpack_v4_8_and_v4_5() {
    let (mut vif, mut vu, mut gs) = setup();
    let s = [
        STCYCL_44,
        unpack(3, 2, 1, 0, false, false, false), // V4-8 signed
        0x80_7f_01_ff,
        unpack(3, 3, 2, 1, false, false, false), // V4-5
        0x8000_7fff,
    ];
    vif.write(&pack(&s), &mut vu, &mut gs);
    assert_eq!(qw(&vu, 0), [0xffff_ffff, 1, 0x7f, 0xffff_ff80]);
    assert_eq!(qw(&vu, 1), [0xf8, 0xf8, 0xf8, 0]);
    assert_eq!(qw(&vu, 2), [0, 0, 0, 0x80]);
}

#[test]
fn unpack_mask_row_col_and_mode() {
    let (mut vif, mut vu, mut gs) = setup();
    // mask row 0: x=data, y=row, z=col, w=protect
    let mask = 0b11_10_01_00;
    let s = [
        STCYCL_44,
        0x2000_0000, // STMASK
        mask,
        0x3000_0000, // STROW
        100,
        200,
        300,
        400,
        0x3100_0000, // STCOL
        7,
        8,
        9,
        10,
        unpack(3, 0, 1, 0, false, false, true),
        1,
        2,
        3,
        4,
        0x0500_0001, // STMOD offset
        unpack(3, 0, 1, 1, false, false, false),
        1,
        2,
        3,
        4,
        0x0500_0002, // STMOD difference
        unpack(3, 0, 1, 2, false, false, false),
        1,
        1,
        1,
        1,
    ];
    vu.data[12..16].copy_from_slice(&0xdeadu32.to_le_bytes());
    vif.write(&pack(&s), &mut vu, &mut gs);
    assert_eq!(qw(&vu, 0), [1, 200, 7, 0xdead]);
    assert_eq!(qw(&vu, 1), [101, 202, 303, 404]);
    assert_eq!(qw(&vu, 2), [101, 201, 301, 401]);
    assert_eq!(vif.row, [101, 201, 301, 401]);
}

#[test]
fn unpack_cycle_skip_and_fill() {
    let (mut vif, mut vu, mut gs) = setup();
    // CL=4, WL=2: write 2, skip 2.
    let s = [
        0x0100_0204,
        unpack(0, 0, 4, 0, false, false, false),
        1,
        2,
        3,
        4,
    ];
    vif.write(&pack(&s), &mut vu, &mut gs);
    assert_eq!(qw(&vu, 0)[0], 1);
    assert_eq!(qw(&vu, 1)[0], 2);
    assert_eq!(qw(&vu, 2)[0], 0);
    assert_eq!(qw(&vu, 4)[0], 3);
    assert_eq!(qw(&vu, 5)[0], 4);
    // CL=1, WL=3: one data write then two filling writes (ROW), 6 writes use 2 data.
    let s = [
        0x3000_0000,
        50,
        51,
        52,
        53,
        0x0100_0301,
        unpack(0, 0, 6, 0x10, false, false, false),
        7,
        8,
    ];
    vif.write(&pack(&s), &mut vu, &mut gs);
    assert_eq!(qw(&vu, 0x10), [7; 4]);
    assert_eq!(qw(&vu, 0x11), [50, 51, 52, 53]);
    assert_eq!(qw(&vu, 0x12), [50, 51, 52, 53]);
    assert_eq!(qw(&vu, 0x13), [8; 4]);
    assert_eq!(qw(&vu, 0x15), [50, 51, 52, 53]);
}

#[test]
fn command_straddles_writes_and_flg_tops() {
    let (mut vif, mut vu, mut gs) = setup();
    let s = [
        STCYCL_44,
        0x0300_0100, // BASE 0x100
        0x0200_0080, // OFFSET 0x80 -> TOPS = BASE
        unpack(3, 0, 2, 1, false, true, false),
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
    ];
    let q = pack(&s);
    vif.write(&q[..1], &mut vu, &mut gs);
    assert!(vif.pending_words() > 0);
    assert_eq!(qw(&vu, 0x101), [0; 4]);
    vif.write(&q[1..], &mut vu, &mut gs);
    assert_eq!(qw(&vu, 0x101), [1, 2, 3, 4]);
    assert_eq!(qw(&vu, 0x102), [5, 6, 7, 8]);
    assert_eq!(vif.read_reg(0xc0), 0x100);
}

fn upload_and_call(vif: &mut Vif, vu: &mut Vu, gs: &mut Gs, ibit: bool) {
    // micro program: vi1 = TOP (xtop); sq vf1 -> (vi1); [E] ; nop
    let prog: [(u32, u32); 3] = [
        (0x0000_02ff, 0x8001_06bc),                                // xtop vi1
        (0x4000_02ff, 0x01 << 25 | 0xf << 21 | 1 << 16 | 1 << 11), // sq.xyzw vf1, 0(vi1) [E]
        (0x0000_02ff, 0x8000_033c),
    ];
    let mut s = vec![0, 0x4a00_0000 | 3 << 16]; // NOP so MPG data is 64-bit aligned
    for (hi, lo) in prog {
        s.push(lo);
        s.push(hi);
    }
    s.push(0x0300_0040); // BASE 0x40
    s.push(0x0200_0020); // OFFSET 0x20
    s.push(0x1400_0000 | (ibit as u32) << 31); // MSCAL 0
    s.push(0x1400_0000); // MSCAL 0 again (double buffer)
    vif.write(&pack(&s), vu, gs);
}

#[test]
fn mpg_mscal_end_to_end_with_double_buffer() {
    let (mut vif, mut vu, mut gs) = setup();
    vu.vf[1] = [11, 22, 33, 44];
    upload_and_call(&mut vif, &mut vu, &mut gs, false);
    // First MSCAL: TOP = 0x40 (TOPS=BASE after OFFSET); second: TOP = 0x60.
    assert_eq!(qw(&vu, 0x40), [11, 22, 33, 44]);
    assert_eq!(qw(&vu, 0x60), [11, 22, 33, 44]);
    assert_eq!(vif.read_reg(0xe0), 0x60);
    assert_eq!(vif.read_reg(0xc0), 0x40);
    assert!(!vu.running);
    assert!(!vif.stalled());
}

#[test]
fn interrupt_bit_stalls_until_stc() {
    let (mut vif, mut vu, mut gs) = setup();
    vu.vf[1] = [1, 2, 3, 4];
    upload_and_call(&mut vif, &mut vu, &mut gs, true);
    assert!(vif.stalled());
    assert!(vif.take_irq());
    assert!(!vif.take_irq());
    assert_ne!(vif.read_reg(0) & STAT_INT, 0);
    // The second MSCAL has not run yet.
    assert_eq!(qw(&vu, 0x60), [0; 4]);
    vif.write_reg(0x10, 8); // FBRST.STC
    assert!(!vif.stalled());
    vif.resume(&mut vu, &mut gs);
    assert_eq!(qw(&vu, 0x60), [1, 2, 3, 4]);
    assert_eq!(vif.read_reg(0) & STAT_INT, 0);
}

#[test]
fn mscal_without_translation_logs_and_stops() {
    let mut vif = Vif::new(1);
    let mut vu = Vu::new(1);
    let mut gs = Gs::new();
    upload_and_call(&mut vif, &mut vu, &mut gs, false);
    assert!(vu.errors >= 1);
    assert!(!vu.running);
}

#[test]
fn registers_and_direct() {
    let (mut vif, mut vu, mut gs) = setup();
    // DIRECT 2 qwords; the code sits in word 3 so its data is qword aligned.
    let mut stream = vec![0, 0, 0, 0x5000_0002];
    stream.extend([1, 2, 3, 4, 5, 6, 7, 8]);
    stream.push(0x0700_1234); // MARK
    stream.push(0x0100_0102); // STCYCL
    vif.write(&pack(&stream), &mut vu, &mut gs);
    assert_eq!(vif.commands[0x50], 1);
    assert_eq!(vif.read_reg(0x30), 0x1234);
    assert_ne!(vif.read_reg(0) & STAT_MRK, 0);
    assert_eq!(vif.read_reg(0x40), 0x0102);
    vif.write_reg(0x30, 5);
    assert_eq!(vif.read_reg(0) & STAT_MRK, 0);
    vif.write_reg(0x100, 77);
    assert_eq!(vif.read_reg(0x100), 77);
    vif.write_reg(0x10, 1); // reset
    assert_eq!(vif.read_reg(0x40), 0);
}

/// The plain UNPACK path (no mask, plain mode, CL == WL) against the general
/// loop (forced with MASK = 0, i.e. every element selects data): all
/// formats, signed and unsigned, random payloads, wrapping addresses.
#[test]
fn unpack_plain_path_matches_general() {
    let mut x: u32 = 0x1234_5678;
    let mut rnd = move || {
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        x
    };
    for vn in 0..4u32 {
        for vl in 0..4u32 {
            if vl == 3 && vn != 3 {
                continue;
            }
            for usn in [false, true] {
                let num = 1 + rnd() % 40;
                let addr = 1000 + rnd() % 24; // wraps past 1024
                let bits = (32 >> vl) * (vn + 1) * num;
                let words = bits.div_ceil(32) as usize;
                let payload: Vec<u32> = (0..words).map(|_| rnd()).collect();
                let run = |masked: bool| {
                    let (mut vif, mut vu, mut gs) = setup();
                    let mut s = vec![STCYCL_44];
                    if masked {
                        s.push(0x2000_0000); // STMASK 0
                        s.push(0);
                    }
                    s.push(unpack(vn, vl, num, addr, usn, false, masked));
                    s.extend_from_slice(&payload);
                    vif.write(&pack(&s), &mut vu, &mut gs);
                    vu.data.clone()
                };
                assert!(run(false) == run(true), "V{}-{} usn {usn}", vn + 1, 32 >> vl);
            }
        }
    }
}
