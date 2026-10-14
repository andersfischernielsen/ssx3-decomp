use super::*;

fn f(x: f32) -> u32 {
    x.to_bits()
}
fn fv(v: [f32; 4]) -> [u32; 4] {
    v.map(f32::to_bits)
}

const UNOP: u32 = 0x0000_02ff;
const LNOP: u32 = 0x8000_033c;

// --- tiny assembler for tests ---------------------------------------------
fn up_add(dest: u32, fd: u32, fs: u32, ft: u32) -> u32 {
    dest << 21 | ft << 16 | fs << 11 | fd << 6 | 0x28
}
fn iaddiu(it: u32, is: u32, imm: u32) -> u32 {
    0x08 << 25 | it << 16 | is << 11 | (imm & 0x7ff)
}
fn iaddi(it: u32, is: u32, imm: i32) -> u32 {
    0x40 << 25 | it << 16 | is << 11 | ((imm as u32 & 31) << 6) | 0x32
}
fn ibne(it: u32, is: u32, off: i32) -> u32 {
    0x29 << 25 | it << 16 | is << 11 | (off as u32 & 0x7ff)
}
fn sq(dest: u32, fs: u32, it: u32, imm: i32) -> u32 {
    0x01 << 25 | dest << 21 | it << 16 | fs << 11 | (imm as u32 & 0x7ff)
}
fn xgkick(is: u32) -> u32 {
    0x40 << 25 | is << 11 | (0x1b << 6) | 0x3c
}
fn div(fs: u32, fsf: u32, ft: u32, ftf: u32) -> u32 {
    0x40 << 25 | ftf << 23 | fsf << 21 | ft << 16 | fs << 11 | (0x0e << 6) | 0x3c
}

fn load_prog(vu: &mut Vu, pairs: &[(u32, u32)]) {
    let bytes: Vec<u8> = pairs
        .iter()
        .flat_map(|(hi, lo)| lo.to_le_bytes().into_iter().chain(hi.to_le_bytes()))
        .collect();
    vu.write_code(0, &bytes);
}

fn interp_vu(index: usize) -> Vu {
    let mut vu = Vu::new(index);
    vu.programs.clear();
    vu.fallback = Some(reference::run_program);
    vu
}

fn one_upper(vu: &mut Vu, op: impl FnOnce(&mut Vu)) {
    vu.begin_pair(&[]);
    op(vu);
    vu.end_pair();
    vu.flush();
}

#[test]
fn add_dest_mask_and_broadcast() {
    let mut vu = Vu::new(1);
    vu.vf[1] = fv([1.0, 2.0, 3.0, 4.0]);
    vu.vf[2] = fv([10.0, 20.0, 30.0, 40.0]);
    vu.vf[3] = fv([-1.0, -1.0, -1.0, -1.0]);
    // ADDy.xz vf3, vf1, vf2y
    one_upper(&mut vu, |v| v.add(0b1010, 3, 1, Src::Bc(2, Y)));
    assert_eq!(vu.vf[3], fv([21.0, -1.0, 23.0, -1.0]));
    // MAC: only x,z lanes considered, none zero/negative.
    assert_eq!(vu.mac, 0);
    // SUB.xyzw vf4 = vf1 - vf1 -> zero flags on all four lanes.
    one_upper(&mut vu, |v| v.sub(0b1111, 4, 1, Src::Vec(1)));
    assert_eq!(vu.mac & 0xf, 0xf);
    assert_eq!(vu.status & 0x41, 0x41); // Z and sticky ZS
                                        // MUL.w vf5 = vf1 * -2 (via I): sign flag for w only (bit 4).
    vu.i = f(-2.0);
    one_upper(&mut vu, |v| v.mul(0b0001, 5, 1, Src::I));
    assert_eq!(vu.vf[5][3], f(-8.0));
    assert_eq!(vu.mac, 1 << 4);
    assert_eq!(vu.status & 0x3, 0x2);
    assert_eq!(vu.status & 0xc0, 0xc0, "sticky Z from before plus sticky S");
    // vf0 is never written.
    one_upper(&mut vu, |v| v.add(0b1111, 0, 1, Src::Vec(1)));
    assert_eq!(vu.vf[0], [0, 0, 0, ONE]);
}

#[test]
fn madd_acc_and_overflow_flags() {
    let mut vu = Vu::new(0);
    vu.vf[1] = fv([1.0, 2.0, 3.0, 4.0]);
    vu.vf[2] = fv([2.0, 2.0, 2.0, 2.0]);
    one_upper(&mut vu, |v| v.mula(0b1111, 1, Src::Vec(2)));
    assert_eq!(vu.acc, fv([2.0, 4.0, 6.0, 8.0]));
    one_upper(&mut vu, |v| v.madd(0b1111, 3, 1, Src::Bc(2, X)));
    assert_eq!(vu.vf[3], fv([4.0, 8.0, 12.0, 16.0]));
    one_upper(&mut vu, |v| v.msuba(0b1000, 1, Src::Q));
    // overflow: max * 2
    vu.vf[4] = [PS2_MAX; 4];
    one_upper(&mut vu, |v| v.mul(0b0010, 5, 4, Src::Bc(2, Y)));
    assert_eq!(vu.vf[5][2], PS2_MAX);
    assert_eq!(vu.mac, 1 << (12 + 1)); // O flag, lane z
    assert_eq!(vu.status & 0x208, 0x208); // O and sticky OS
                                          // exponent-255 input reads as max
    vu.vf[6] = [0x7f80_0000; 4];
    one_upper(&mut vu, |v| v.add(0b1000, 7, 6, Src::Bc(0, X)));
    assert_eq!(vu.vf[7][0], PS2_MAX);
    // underflow
    vu.vf[8] = [f(1e-30); 4];
    one_upper(&mut vu, |v| v.mul(0b1000, 9, 8, Src::Vec(8)));
    assert_eq!(vu.vf[9][0], 0);
    assert_eq!(vu.mac, (1 << 3) | (1 << 11)); // Z and U on x
}

#[test]
fn opmula_opmsub_cross_product() {
    let mut vu = Vu::new(0);
    vu.vf[1] = fv([1.0, 0.0, 0.0, 9.0]);
    vu.vf[2] = fv([0.0, 1.0, 0.0, 9.0]);
    vu.vf[3] = fv([5.0, 5.0, 5.0, 5.0]);
    one_upper(&mut vu, |v| v.opmula(1, 2));
    one_upper(&mut vu, |v| v.opmsub(3, 2, 1));
    assert_eq!(vu.vf[3], fv([0.0, 0.0, 1.0, 5.0]));
}

#[test]
fn clip_flags_shift() {
    let mut vu = Vu::new(1);
    vu.vf[1] = fv([2.0, -2.0, 0.5, 0.0]);
    vu.vf[2] = fv([0.0, 0.0, 0.0, -1.0]); // |w| = 1
    one_upper(&mut vu, |v| v.clipw(1, 2));
    assert_eq!(vu.clip, 0b00_10_01); // +x, -y
    vu.vf[1] = fv([0.0, 0.0, -3.0, 0.0]);
    one_upper(&mut vu, |v| v.clipw(1, 2));
    assert_eq!(vu.clip, (0b00_10_01 << 6) | 0b10_00_00);
    // FCAND sees it (after flush), vi1 = 1
    vu.begin_pair(&[]);
    vu.fcand(0x20);
    vu.end_pair();
    assert_eq!(vu.vi[1], 1);
    // 24-bit wrap
    for _ in 0..4 {
        one_upper(&mut vu, |v| v.clipw(1, 2));
    }
    assert_eq!(vu.clip, 0x82_0820);
}

#[test]
fn ftoi_itof() {
    let mut vu = Vu::new(0);
    vu.vf[1] = fv([1.5, -2.75, 1e12, 0.0625]);
    one_upper(&mut vu, |v| v.ftoi(0, 0b1111, 2, 1));
    assert_eq!(vu.vf[2], [1, (-2i32) as u32, 0x7fff_ffff, 0]);
    one_upper(&mut vu, |v| v.ftoi(4, 0b1111, 3, 1));
    assert_eq!(vu.vf[3], [24, (-44i32) as u32, 0x7fff_ffff, 1]);
    one_upper(&mut vu, |v| v.itof(4, 0b1110, 4, 3));
    assert_eq!(&vu.vf[4][..2], &fv([1.5, -2.75, 0.0, 0.0])[..2]);
    one_upper(&mut vu, |v| v.itof(12, 0b0001, 5, 3));
    assert_eq!(vu.vf[5][3], f(1.0 / 4096.0));
    one_upper(&mut vu, |v| v.itof(15, 0b0001, 5, 3));
    assert_eq!(vu.vf[5][3], f(1.0 / 32768.0));
    one_upper(&mut vu, |v| v.abs(0b1111, 6, 1));
    assert_eq!(vu.vf[6][1], f(2.75));
}

#[test]
fn div_sqrt_rsqrt_and_q_latency() {
    let mut vu = Vu::new(1);
    vu.vf[1] = fv([1.0, 4.0, -9.0, 0.0]);
    vu.vf[2] = fv([10.0, 0.0, 0.0, 0.0]);
    vu.q = f(100.0);
    // DIV q = vf2x / vf1y = 2.5; an upper in the next pairs still sees old Q.
    vu.begin_pair(&[]);
    vu.div(2, 0, 1, 1);
    vu.end_pair();
    vu.begin_pair(&[]);
    vu.mul(0b1000, 3, 1, Src::Q);
    vu.end_pair();
    vu.flush_flags_only_for_test();
    assert_eq!(vu.vf[3][0], f(100.0), "old Q before latency");
    vu.begin_pair(&[]);
    vu.waitq();
    vu.end_pair();
    vu.begin_pair(&[]);
    vu.mul(0b1000, 3, 1, Src::Q);
    vu.end_pair();
    assert_eq!(vu.vf[3][0], f(2.5));
    // after 7 cycles Q is visible without WAITQ
    vu.begin_pair(&[]);
    vu.sqrt(1, 1);
    vu.end_pair();
    for _ in 0..6 {
        vu.begin_pair(&[]);
        vu.end_pair();
    }
    vu.begin_pair(&[]);
    assert_eq!(vu.q, f(2.0));
    vu.end_pair();
    // SQRT of negative: |x| and I flag
    vu.sqrt(1, 2);
    vu.flush();
    assert_eq!(vu.q, f(3.0));
    assert_eq!(vu.status & 0x410, 0x410);
    // DIV by zero: D flag, +max
    vu.div(2, 0, 1, 3);
    vu.flush();
    assert_eq!(vu.q, PS2_MAX);
    assert_eq!(vu.status & 0x30, 0x20);
    // RSQRT 10 / sqrt(4)
    vu.rsqrt(2, 0, 1, 1);
    vu.flush();
    assert_eq!(vu.q, f(5.0));
}

impl Vu {
    /// Test helper: drain only the flag pipeline (keeps Q pending).
    fn flush_flags_only_for_test(&mut self) {
        self.flag_pipe.clear();
    }
}

#[test]
fn pair_reads_old_state_upper_wins() {
    let mut vu = Vu::new(1);
    vu.vf[1] = fv([1.0, 1.0, 1.0, 1.0]);
    // upper: vf1 = vf1 + vf1 ; lower: MOVE vf2, vf1 -> vf2 gets the OLD vf1
    vu.begin_pair(&[]);
    vu.add(0b1111, 1, 1, Src::Vec(1));
    vu.vmove(0b1111, 2, 1);
    vu.end_pair();
    assert_eq!(vu.vf[1], fv([2.0; 4]));
    assert_eq!(vu.vf[2], fv([1.0; 4]));
    // both write vf3: upper wins, lower discarded
    vu.begin_pair(&[]);
    vu.add(0b1000, 3, 1, Src::Vec(1));
    vu.vmove(0b1111, 3, 2);
    vu.end_pair();
    assert_eq!(vu.vf[3], [f(4.0), 0, 0, 0]);
}

#[test]
fn mac_flags_have_latency() {
    let mut vu = Vu::new(1);
    vu.vi[2] = 0xffff;
    vu.begin_pair(&[]);
    vu.sub(0b1111, 1, 0, Src::Vec(0)); // all-zero x,y,z (w = 1-1 = 0 too)
    vu.end_pair();
    vu.begin_pair(&[]);
    vu.fmand(3, 2);
    vu.end_pair();
    assert_eq!(vu.vi[3], 0, "MAC not visible one pair later");
    for _ in 0..2 {
        vu.begin_pair(&[]);
        vu.end_pair();
    }
    vu.begin_pair(&[]);
    vu.fmand(3, 2);
    vu.end_pair();
    assert_eq!(vu.vi[3], 0xf, "visible 4 cycles after issue");
}

#[test]
fn stall_on_fmac_result() {
    let mut vu = Vu::new(1);
    vu.vf[1] = fv([1.0; 4]);
    vu.begin_pair(&[(1, 0b1111)]);
    vu.add(0b1111, 2, 1, Src::Vec(1));
    vu.end_pair();
    let c = vu.cycle;
    vu.begin_pair(&[(2, 0b1000)]);
    assert_eq!(vu.cycle, c + 3, "3 stall cycles for a 4-cycle FMAC result");
    vu.end_pair();
}

#[test]
fn micro_program_branch_delay_slot_ebit() {
    let mut vu = interp_vu(1);
    let prog = [
        (UNOP, iaddiu(1, 0, 3)),               // 0x00
        (UNOP, iaddiu(2, 0, 0)),               // 0x08
        (UNOP, iaddi(1, 1, -1)),               // 0x10 loop:
        (UNOP, iaddiu(2, 2, 1)),               // 0x18
        (UNOP, ibne(0, 1, -3)),                // 0x20 ibne vi1, vi0, loop
        (UNOP, iaddiu(3, 3, 1)),               // 0x28 delay slot
        (0x4000_0000 | UNOP, iaddiu(4, 0, 7)), // 0x30 [E]
        (UNOP, iaddiu(5, 0, 9)),               // 0x38 E delay slot
        (UNOP, iaddiu(6, 0, 1)),               // 0x40 must not run
    ];
    load_prog(&mut vu, &prog);
    vu.start(0);
    let mut kick = |_: &[Qword]| {};
    assert!(vu.run(1000, &mut kick));
    assert!(!vu.running);
    assert_eq!(&vu.vi[1..7], &[0, 3, 3, 7, 9, 0]);
    assert_eq!(vu.tpc, 0x40);
    // Budget expiry leaves it running and resumable.
    vu.start(0);
    assert!(!vu.run(3, &mut kick));
    assert!(vu.running);
    assert!(vu.run(1000, &mut kick));
    assert_eq!(vu.vi[2], 3);
}

/// Regression (differential test, race capture): MAX/MINI compare and return
/// raw bits like PCSX2's integer MIN_MAX_PS. SSX 3 moves integer vertex
/// colours with `MAX.xyzw vf9, vf4, vf4`; flushing denormals zeroed them.
#[test]
fn max_mini_pass_raw_bits() {
    let mut vu = Vu::new(1);
    vu.vf[4] = [0x63, 0x51, 0x62, 0x80];
    one_upper(&mut vu, |v| v.max(0b1111, 9, 4, Src::Vec(4)));
    assert_eq!(vu.vf[9], [0x63, 0x51, 0x62, 0x80]);
    one_upper(&mut vu, |v| v.mini(0b1111, 10, 4, Src::Vec(4)));
    assert_eq!(vu.vf[10], [0x63, 0x51, 0x62, 0x80]);
    // Ordering is sign-magnitude on the raw pattern: -1 < -0 < +0 < denormal < 1.
    vu.vf[1] = [f(-1.0), 0x8000_0000, 0, 5];
    vu.vf[2] = [0x8000_0000, 0, 7, f(1.0)];
    one_upper(&mut vu, |v| v.max(0b1111, 3, 1, Src::Vec(2)));
    assert_eq!(vu.vf[3], [0x8000_0000, 0, 7, f(1.0)]);
    one_upper(&mut vu, |v| v.mini(0b1111, 3, 1, Src::Vec(2)));
    assert_eq!(vu.vf[3], [f(-1.0), 0x8000_0000, 0, 5]);
    // Exponent-255 patterns are not clamped by MAX/MINI.
    vu.vf[1] = [0x7f90_0000, 0xff90_0000, 0, 0];
    one_upper(&mut vu, |v| v.max(0b1100, 3, 1, Src::Bc(0, X)));
    assert_eq!(&vu.vf[3][..2], &[0x7f90_0000, 0]);
}

/// ADDi.<dest> fd, fs, I (upper funct 0x22).
fn up_addi(dest: u32, fd: u32, fs: u32) -> u32 {
    dest << 21 | fs << 11 | fd << 6 | 0x22
}

const IBIT: u32 = 0x8000_0000;
const EBIT: u32 = 0x4000_0000;

fn ibit_prog() -> [(u32, u32); 4] {
    [
        (IBIT | up_addi(8, 1, 0), f(1.0)),        // ADDi.x vf1, vf0, I  | LOI 1.0
        (IBIT | up_addi(4, 1, 0), f(2.0)),        // ADDi.y vf1, vf0, I  | LOI 2.0
        (IBIT | EBIT | up_addi(2, 1, 0), f(3.0)), // ADDi.z vf1, vf0, I  | LOI 3.0  [E]
        (UNOP, LNOP),
    ]
}

/// Regression (found by the differential test against ps2-web-recomp and
/// PCSX2): the upper op of an I-bit pair reads the *old* I; LOI's value is
/// visible from the next pair on (PCSX2 `_vu1Exec`, microVU `doIbit`).
#[test]
fn ibit_upper_reads_old_i() {
    let mut vu = interp_vu(1);
    load_prog(&mut vu, &ibit_prog());
    vu.i = f(5.0);
    vu.start(0);
    assert!(vu.run(100, &mut |_| {}));
    assert_eq!(&vu.vf[1][..3], &fv([5.0, 1.0, 2.0, 0.0])[..3]);
    assert_eq!(vu.i, f(3.0));
}

/// The translator emits LOI after the upper call, so translated code agrees.
#[test]
fn ibit_translation_emits_loi_after_upper() {
    let bytes: Vec<u8> = ibit_prog()
        .iter()
        .flat_map(|(hi, lo)| lo.to_le_bytes().into_iter().chain(hi.to_le_bytes()))
        .collect();
    let src = recomp::translate_program(&bytes, 0);
    let add = src.find("vu.add(0b1000, 1, 0, Src::I)").expect("upper");
    let loi = src.find("vu.loi(0x3f800000)").expect("loi");
    assert!(add < loi, "LOI must follow the upper op:\n{src}");
}

fn up_mulq(dest: u32, fd: u32, fs: u32) -> u32 {
    dest << 21 | fs << 11 | fd << 6 | 0x1c
}
const WAITQ: u32 = 0x8000_03bf;

/// DIV, then `MULq.x vf3, vf1, Q` in the same pair as `lower`, then `MULq.y`
/// one pair later. vf1 = (1, 4, -9, 0), vf2.x = 10.
fn q_wait_prog(lower: u32) -> [(u32, u32); 4] {
    [
        (UNOP, div(2, 0, 1, 1)),        // DIV Q, vf2.x, vf1.y  (10 / 4)
        (up_mulq(8, 3, 1), lower),      // MULq.x vf3, vf1, Q | lower
        (EBIT | up_mulq(4, 3, 1), LNOP), // MULq.y vf3, vf1, Q [E]
        (UNOP, LNOP),
    ]
}

/// Regression (race "girders", `docs/investigation/race-geometry.md`): a
/// WAITQ stalls its whole pair *before* the upper op, so an upper op reading
/// Q in the WAITQ pair sees the new Q (PCSX2 microVU `mVU_WAITQ` pass 1
/// folds the wait into the pair stall; the interpreter tests lower stalls
/// before running the upper). SSX 3's near-plane clipper interpolates with
/// `MADD.xyzw vf21, vf22, Q` + `WAITQ`; with the old Q the clipped vertex
/// landed behind the eye and drew a long sliver across the screen.
#[test]
fn waitq_pair_upper_reads_new_q() {
    let mut vu = interp_vu(1);
    load_prog(&mut vu, &q_wait_prog(WAITQ));
    vu.vf[1] = fv([1.0, 4.0, -9.0, 0.0]);
    vu.vf[2] = fv([10.0, 0.0, 0.0, 0.0]);
    vu.q = f(100.0);
    vu.start(0);
    assert!(vu.run(100, &mut |_| {}));
    assert_eq!(vu.vf[3][0], f(2.5), "upper in the WAITQ pair sees the new Q");
    assert_eq!(vu.vf[3][1], f(10.0));
}

/// A second DIV issued while the first is in flight also stalls the pair
/// first: its upper sees the first DIV's result (SSX 3 software-pipelines
/// DIV in `d159716b`).
#[test]
fn div_pair_upper_reads_previous_div_result() {
    let mut vu = interp_vu(1);
    // second DIV: Q = vf2.x / vf1.x = 10
    load_prog(&mut vu, &q_wait_prog(div(2, 0, 1, 0)));
    vu.vf[1] = fv([1.0, 4.0, -9.0, 0.0]);
    vu.vf[2] = fv([10.0, 0.0, 0.0, 0.0]);
    vu.q = f(100.0);
    vu.start(0);
    assert!(vu.run(100, &mut |_| {}));
    assert_eq!(vu.vf[3][0], f(2.5), "first DIV landed before the upper");
    // The second DIV is still in flight one pair later: old (first) Q.
    assert_eq!(vu.vf[3][1], f(4.0 * 2.5));
    assert_eq!(vu.q, f(10.0));
}

/// The translator emits the Q stall between `begin_pair` and the upper op.
#[test]
fn waitq_translation_stalls_before_upper() {
    let bytes: Vec<u8> = q_wait_prog(WAITQ)
        .iter()
        .flat_map(|(hi, lo)| lo.to_le_bytes().into_iter().chain(hi.to_le_bytes()))
        .collect();
    let src = recomp::translate_program(&bytes, 0);
    let mul = src.find("vu.mul(0b1000, 3, 1, Src::Q)").expect("upper");
    let stall = src[..mul].rfind("vu.stall_q()").expect("stall_q before the upper");
    let begin = src[..mul].rfind("vu.begin_pair").expect("begin_pair");
    assert!(begin < stall, "stall_q follows begin_pair:\n{src}");
}

/// Branch paths for the VI-backup tests: 0x28 [E] vi7 = 1 (fall-through),
/// 0x38 [E] vi7 = 2 (taken, `off` 3 from a branch at 0x18).
fn vi_backup_tail(p: &mut Vec<(u32, u32)>) {
    p.push((UNOP, LNOP)); // 0x20 delay slot
    p.push((EBIT | UNOP, iaddiu(7, 0, 1))); // 0x28 not taken
    p.push((UNOP, LNOP));
    p.push((EBIT | UNOP, iaddiu(7, 0, 2))); // 0x38 taken
    p.push((UNOP, LNOP));
}

/// PCSX2 `_vuBackupVI`: a conditional branch reads the old value of a VI
/// written by an integer ALU op in the immediately preceding pair.
/// (This test used to pass either way; the quirk was in fact disabled.)
#[test]
fn branch_reads_old_vi_written_by_previous_pair() {
    let mut vu = interp_vu(1);
    let mut p = vec![
        (UNOP, iaddiu(1, 0, 1)), // 0x00 vi1 = 1
        (UNOP, LNOP),            // 0x08
        (UNOP, iaddi(1, 1, -1)), // 0x10 vi1 = 0
        (UNOP, ibne(0, 1, 3)),   // 0x18 reads OLD vi1 (1) -> taken to 0x38
    ];
    vi_backup_tail(&mut p);
    load_prog(&mut vu, &p);
    vu.start(0);
    assert!(vu.run(100, &mut |_| {}));
    assert_eq!(vu.vi[7], 2);
}

/// microVU `analyzeBranchVI`: no backup when the branch pair stalls.
#[test]
fn stalled_branch_reads_current_vi() {
    let mut vu = interp_vu(1);
    let mut p = vec![
        (UNOP, iaddiu(1, 0, 1)),               // 0x00 vi1 = 1
        (up_add(15, 1, 0, 0), LNOP),           // 0x08 vf1 = vf0 + vf0
        (UNOP, iaddi(1, 1, -1)),               // 0x10 vi1 = 0
        (up_add(15, 2, 1, 1), ibne(0, 1, 3)),  // 0x18 stalls on vf1 -> current vi1 (0)
    ];
    vi_backup_tail(&mut p);
    load_prog(&mut vu, &p);
    vu.start(0);
    assert!(vu.run(100, &mut |_| {}));
    assert_eq!(vu.vi[7], 1);
}

/// JR is not subject to the VI backup (PCSX2 `_vuJR`, `mVUanalyzeJump`).
#[test]
fn jr_reads_current_vi() {
    let jr = |is: u32| 0x24 << 25 | is << 11;
    let mut vu = interp_vu(1);
    let p = [
        (UNOP, iaddiu(2, 0, 5)),               // 0x00 vi2 = 5 (0x28)
        (UNOP, LNOP),                          // 0x08
        (UNOP, iaddiu(2, 0, 6)),               // 0x10 vi2 = 6 (0x30)
        (UNOP, jr(2)),                         // 0x18 -> 0x30
        (UNOP, LNOP),                          // 0x20 delay
        (EBIT | UNOP, iaddiu(7, 0, 1)),        // 0x28 old target
        (EBIT | UNOP, iaddiu(8, 0, 2)),        // 0x30 new target
        (UNOP, LNOP),
        (UNOP, LNOP),
    ];
    load_prog(&mut vu, &p);
    vu.start(0);
    assert!(vu.run(100, &mut |_| {}));
    assert_eq!((vu.vi[7], vu.vi[8]), (0, 2));
}

#[test]
fn xgkick_packed_and_image() {
    let mut vu = Vu::new(1);
    // PACKED: nloop 2, nreg 2 -> 4 qwords; then IMAGE nloop 3, EOP.
    let packed: u128 = 2 | (2u128 << 60);
    let image: u128 = 3 | (1 << 15) | (2u128 << 58);
    let base = 0x3fd; // wraps at 1024 qwords
    let mut q = base;
    let mut put = |vu: &mut Vu, v: u128| {
        let o = (q & 1023) * 16;
        vu.data[o..o + 16].copy_from_slice(&v.to_le_bytes());
        q += 1;
    };
    put(&mut vu, packed);
    for i in 0..4 {
        put(&mut vu, 100 + i);
    }
    put(&mut vu, image);
    for i in 0..3 {
        put(&mut vu, 200 + i);
    }
    put(&mut vu, 0xdead); // not part of the packet
    vu.vi[3] = base as u16;
    let mut got = Vec::new();
    vu.xgkick(3, &mut |p| got.extend_from_slice(p));
    assert_eq!(got, vec![packed, 100, 101, 102, 103, image, 200, 201, 202]);
    // REGLIST: nloop 3 * nreg 1 = 3 regs -> 2 qwords
    let reglist: u128 = 3 | (1 << 15) | (1u128 << 58) | (1u128 << 60);
    vu.data[0..16].copy_from_slice(&reglist.to_le_bytes());
    assert_eq!(vu.gif_packet(0).len(), 3);
}

#[test]
fn program_with_sq_and_xgkick() {
    let mut vu = interp_vu(1);
    vu.vf[1] = [1 | (1 << 15), 1 << 28, 0, 0]; // GIF tag: PACKED, nloop 1, EOP, nreg 1
    vu.vf[2] = fv([1.0, 2.0, 3.0, 4.0]);
    let prog = [
        (UNOP, iaddiu(1, 0, 0x10)),
        (UNOP, sq(0b1111, 1, 1, 0)),
        (UNOP, sq(0b1111, 2, 1, 1)),
        (0x4000_0000 | UNOP, xgkick(1)),
        (UNOP, LNOP),
    ];
    load_prog(&mut vu, &prog);
    vu.start(0);
    let mut pkts = Vec::new();
    assert!(vu.run(100, &mut |p| pkts.push(p.to_vec())));
    assert_eq!(pkts.len(), 1);
    assert_eq!(pkts[0].len(), 2);
    assert_eq!(pkts[0][1] as u32, f(1.0));
}

#[test]
fn macro_mode_ops_and_ctrl() {
    let mut vu = Vu::new(0);
    vu.vf[1] = fv([1.0, 2.0, 3.0, 4.0]);
    vu.vf[2] = fv([1.0, 1.0, 1.0, 1.0]);
    // VADD.xyzw vf3, vf1, vf2
    vu.macro_op(0x4a00_0000 | up_add(0xf, 3, 1, 2));
    assert_eq!(vu.vf[3], fv([2.0, 3.0, 4.0, 5.0]));
    // VDIV Q, vf1w, vf2x -> immediately visible
    vu.macro_op(0x4a00_0000 | (div(1, 3, 2, 0) & 0x01ff_ffff));
    assert_eq!(vu.read_ctrl(22), f(4.0));
    // VIADDI vi1, vi0, 5 ; VIADD vi2, vi1, vi1
    vu.macro_op(0x4a00_0000 | 1 << 16 | (5 << 6) | 0x32);
    vu.macro_op(0x4a00_0000 | 1 << 16 | 1 << 11 | 2 << 6 | 0x30);
    assert_eq!(vu.read_ctrl(2), 10);
    // VMTIR vi3, vf3y (index 0x3c: bits10..6 = 0x0f, funct 0x3c)
    vu.vf[3][1] = 0x1234_5678;
    vu.macro_op(0x4a00_0000 | 1 << 21 | 3 << 16 | 3 << 11 | (0x0f << 6) | 0x3c);
    assert_eq!(vu.vi[3], 0x5678);
    // VCALLMS ignored
    vu.macro_op(0x4a00_0038);
    // ctrl: status sticky write, clip, R, TPC
    vu.write_ctrl(16, 0xfff);
    assert_eq!(vu.read_ctrl(16) & 0xfc0, 0xfc0);
    vu.write_ctrl(18, 0x123456);
    assert_eq!(vu.read_ctrl(18), 0x123456);
    vu.write_ctrl(20, 0xffff_ffff);
    assert_eq!(vu.r, 0x3fff_ffff);
    vu.write_ctrl(31, 0x10);
    assert_eq!(vu.cmsar1_request, Some(0x80));
    vu.running = true;
    vu.peer_running = true;
    assert_eq!(vu.read_ctrl(29), 0x101);
}

#[test]
fn missing_translation_stops_vu() {
    let mut vu = Vu::new(1);
    load_prog(&mut vu, &[(0x4000_0000 | UNOP, LNOP), (UNOP, LNOP)]);
    vu.start(0);
    assert!(vu.run(100, &mut |_| {}));
    assert!(!vu.running);
    assert_eq!(vu.errors, 1);
}

#[test]
fn segments_track_mpg_runs() {
    let mut vu = Vu::new(1);
    vu.write_code(0, &[1; 0x800]);
    vu.write_code(0x800, &[2; 0x800]);
    assert_eq!(vu.segments.len(), 1);
    assert_eq!((vu.segments[0].start, vu.segments[0].end), (0, 0x1000));
    let mut expect = vec![1u8; 0x800];
    expect.extend(vec![2u8; 0x800]);
    assert_eq!(vu.segment_hash(0x10), Some(program_hash(0, &expect)));
    // a new upload at 0 trims the old segment
    vu.write_code(0, &[3; 0x400]);
    assert_eq!(vu.segments.len(), 2);
    assert!(vu
        .segments
        .iter()
        .any(|s| s.start == 0x400 && s.end == 0x1000));
}

// --- oracle: translated code vs the reference interpreter -------------------

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 16) as u32
    }
    fn word(&mut self) -> u32 {
        const F: [f32; 8] = [1.0, 0.5, -2.0, 3.25, 100.0, -0.125, 7.0, 0.0];
        match self.next() % 4 {
            0 => self.next() % 8,
            1 => F[(self.next() % 8) as usize].to_bits(),
            2 => (self.next() % 64) | 0x8000,
            _ => self.next() % 0x400,
        }
    }
}

fn seed_state(vu: &mut Vu, seed: u64, entries: &[u32]) {
    let mut r = Rng(seed | 1);
    for i in 0..vu.data.len() / 4 {
        let w = r.word();
        vu.data[i * 4..i * 4 + 4].copy_from_slice(&w.to_le_bytes());
    }
    for i in 1..32 {
        for l in 0..4 {
            vu.vf[i][l] = r.word();
        }
    }
    for i in 1..16 {
        vu.vi[i] = (r.next() % 0x100) as u16;
    }
    // Register-jump sources (JR vi14/vi15) point at real entry points.
    vu.vi[14] = (entries[(r.next() as usize) % entries.len()] / 8) as u16;
    vu.vi[15] = (entries[(r.next() as usize) % entries.len()] / 8) as u16;
    vu.vif_top = 0x100;
    vu.vif_itop = 0x20;
}

fn snapshot(vu: &Vu) -> String {
    format!(
        "vf={:?} vi={:?} acc={:?} q={:x} p={:x} i={:x} r={:x} mac={:x} st={:x} clip={:x} tpc={:x} cyc={} errs={}",
        vu.vf, vu.vi, vu.acc, vu.q, vu.p, vu.i, vu.r, vu.mac, vu.status, vu.clip, vu.tpc, vu.cycle, vu.errors
    )
}

#[test]
fn oracle_translated_matches_reference_on_vutext() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../orig/SLUS_207.72");
    let Ok(elf) = std::fs::read(path) else {
        eprintln!("skipping: {path} not present");
        return;
    };
    let (_, vutext) = vutext_section(&elf);
    let words: Vec<u32> = vutext
        .chunks_exact(4)
        .map(|c| u32::from_le_bytes(c.try_into().unwrap()))
        .collect();
    let segs = recomp::extract_mpg_segments(&words);
    assert_eq!(
        segs.len(),
        generated::PROGRAMS.len(),
        "generated/vu is stale: rerun vu_recomp"
    );
    let (mut runs, mut finished) = (0, 0);
    for s in &segs {
        let entry = generated::PROGRAMS
            .iter()
            .find(|e| e.hash == program_hash(s.addr, &s.bytes))
            .expect("translation for segment (rerun vu_recomp)");
        // Entry points: load address and every pair after an E-bit delay slot.
        let mut entries = vec![s.addr];
        for (i, c) in s.bytes.chunks_exact(8).enumerate() {
            let hi = u32::from_le_bytes(c[4..8].try_into().unwrap());
            if hi & 0x4000_0000 != 0 && (i + 2) * 8 < s.bytes.len() {
                entries.push(s.addr + (i as u32 + 2) * 8);
            }
        }
        for (k, &pc) in entries.iter().enumerate() {
            for seed in 0..2u64 {
                let mut a = Vu::new(1);
                let mut b = interp_vu(1);
                for vu in [&mut a, &mut b] {
                    vu.write_code(s.addr, &s.bytes);
                    seed_state(vu, seed * 7919 + k as u64 * 31 + entry.hash, &entries);
                    vu.start(pc);
                }
                assert!(a.lookup(pc).is_some(), "entry {pc:#x} not a leader");
                let (mut ka, mut kb) = (Vec::new(), Vec::new());
                // Budgets are charged per block (translated) vs per pair
                // (reference), so resume both until done or a hard cap.
                let mut da = false;
                let mut db = false;
                for _ in 0..3 {
                    da = da || a.run(100_000, &mut |p| ka.push(p.to_vec()));
                    db = db || b.run(100_000, &mut |p| kb.push(p.to_vec()));
                    if da && db {
                        break;
                    }
                }
                runs += 1;
                if !(da && db) || a.errors > 0 {
                    continue;
                }
                finished += 1;
                assert_eq!(
                    snapshot(&a),
                    snapshot(&b),
                    "segment {:#x} entry {pc:#x} seed {seed}",
                    entry.hash
                );
                assert!(a.data == b.data, "data memory differs at entry {pc:#x}");
                assert_eq!(ka, kb, "XGKICK packets differ at entry {pc:#x}");
            }
        }
    }
    eprintln!("oracle: {finished}/{runs} runs finished and matched");
    assert!(finished > runs / 2);
}

pub(crate) fn vutext_section(elf: &[u8]) -> (u32, &[u8]) {
    let rd16 = |o: usize| u16::from_le_bytes([elf[o], elf[o + 1]]) as usize;
    let rd32 = |o: usize| u32::from_le_bytes(elf[o..o + 4].try_into().unwrap()) as usize;
    let shoff = rd32(0x20);
    let strtab = rd32(shoff + rd16(0x32) * 40 + 16);
    for i in 0..rd16(0x30) {
        let sh = shoff + i * 40;
        let n = strtab + rd32(sh);
        if elf[n..].starts_with(b".vutext\0") {
            let (off, size) = (rd32(sh + 16), rd32(sh + 20));
            return (rd32(sh + 12) as u32, &elf[off..off + size]);
        }
    }
    panic!("no .vutext");
}

#[test]
fn vutext_decodes_without_unknown_opcodes() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../orig/SLUS_207.72");
    let Ok(elf) = std::fs::read(path) else {
        eprintln!("skipping: {path} not present");
        return;
    };
    let (_, vutext) = vutext_section(&elf);
    let words: Vec<u32> = vutext
        .chunks_exact(4)
        .map(|c| u32::from_le_bytes(c.try_into().unwrap()))
        .collect();
    let mut pairs = 0;
    let mut unknown = Vec::new();
    for s in recomp::extract_mpg_segments(&words) {
        for (i, c) in s.bytes.chunks_exact(8).enumerate() {
            let lo = u32::from_le_bytes(c[0..4].try_into().unwrap());
            let hi = u32::from_le_bytes(c[4..8].try_into().unwrap());
            pairs += 1;
            if let decode::Upper::Unknown(w) = decode::decode_upper(hi) {
                unknown.push((s.addr + i as u32 * 8, w));
            }
            if hi & 0x8000_0000 == 0 {
                if let decode::Lower::Unknown(w) = decode::decode_lower(lo) {
                    unknown.push((s.addr + i as u32 * 8, w));
                }
            }
        }
    }
    eprintln!("decoded {pairs} pairs, {} unknown", unknown.len());
    assert!(pairs > 7000);
    assert!(unknown.is_empty(), "unknown opcodes: {unknown:x?}");
}
