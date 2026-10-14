//! Decoded-op dispatch onto the `Vu` semantic library, in two forms kept
//! side by side so they cannot drift: `exec_*` calls the methods (macro mode
//! and the test oracle), `*_call` renders the same call as Rust source (the
//! static recompiler).

use super::decode::{Fmac, Lower, Upper};
use super::{Src, Vu};
use crate::Qword;

pub fn exec_upper(vu: &mut Vu, u: &Upper) {
    match *u {
        Upper::Nop => {}
        Upper::Unknown(w) => vu.unknown_op(u32::MAX, w),
        Upper::Fmac {
            op,
            acc,
            dest,
            fd,
            fs,
            src,
        } => match (op, acc) {
            (Fmac::Add, false) => vu.add(dest, fd, fs, src),
            (Fmac::Add, true) => vu.adda(dest, fs, src),
            (Fmac::Sub, false) => vu.sub(dest, fd, fs, src),
            (Fmac::Sub, true) => vu.suba(dest, fs, src),
            (Fmac::Madd, false) => vu.madd(dest, fd, fs, src),
            (Fmac::Madd, true) => vu.madda(dest, fs, src),
            (Fmac::Msub, false) => vu.msub(dest, fd, fs, src),
            (Fmac::Msub, true) => vu.msuba(dest, fs, src),
            (Fmac::Mul, false) => vu.mul(dest, fd, fs, src),
            (Fmac::Mul, true) => vu.mula(dest, fs, src),
            (Fmac::Max, _) => vu.max(dest, fd, fs, src),
            (Fmac::Mini, _) => vu.mini(dest, fd, fs, src),
        },
        Upper::Opmula { fs, ft } => vu.opmula(fs, ft),
        Upper::Opmsub { fd, fs, ft } => vu.opmsub(fd, fs, ft),
        Upper::Itof { frac, dest, ft, fs } => vu.itof(frac, dest, ft, fs),
        Upper::Ftoi { frac, dest, ft, fs } => vu.ftoi(frac, dest, ft, fs),
        Upper::Abs { dest, ft, fs } => vu.abs(dest, ft, fs),
        Upper::Clip { fs, ft } => vu.clipw(fs, ft),
    }
}

/// Execute a lower op. Returns `Some(target)` when a branch is taken.
/// `pc` is the byte address of the pair (for relative branches and links).
pub fn exec_lower(vu: &mut Vu, l: &Lower, pc: u32, kick: &mut dyn FnMut(&[Qword])) -> Option<u32> {
    let mask = vu.code.len() as u32 - 1;
    let rel = |off: i16| (pc as i32 + 8 + off as i32 * 8) as u32 & mask;
    match *l {
        Lower::Lq { dest, ft, is, imm } => vu.lq(dest, ft, is, imm),
        Lower::Sq { dest, fs, it, imm } => vu.sq(dest, fs, it, imm),
        Lower::Ilw { dest, it, is, imm } => vu.ilw(dest, it, is, imm),
        Lower::Isw { dest, it, is, imm } => vu.isw(dest, it, is, imm),
        Lower::Iaddiu { it, is, imm } => vu.iaddiu(it, is, imm),
        Lower::Isubiu { it, is, imm } => vu.isubiu(it, is, imm),
        Lower::Fceq { imm } => vu.fceq(imm),
        Lower::Fcset { imm } => vu.fcset(imm),
        Lower::Fcand { imm } => vu.fcand(imm),
        Lower::Fcor { imm } => vu.fcor(imm),
        Lower::Fseq { it, imm } => vu.fseq(it, imm),
        Lower::Fsset { imm } => vu.fsset(imm),
        Lower::Fsand { it, imm } => vu.fsand(it, imm),
        Lower::Fsor { it, imm } => vu.fsor(it, imm),
        Lower::Fmeq { it, is } => vu.fmeq(it, is),
        Lower::Fmand { it, is } => vu.fmand(it, is),
        Lower::Fmor { it, is } => vu.fmor(it, is),
        Lower::Fcget { it } => vu.fcget(it),
        Lower::B { off } => return Some(rel(off)),
        Lower::Bal { it, off } => {
            vu.link(it, pc + 16);
            return Some(rel(off));
        }
        Lower::Jr { is } => return Some(vu.jr_target(is)),
        Lower::Jalr { it, is } => {
            let t = vu.jr_target(is);
            vu.link(it, pc + 16);
            return Some(t);
        }
        Lower::Ibeq { it, is, off } => return vu.ibeq(it, is).then(|| rel(off)),
        Lower::Ibne { it, is, off } => return vu.ibne(it, is).then(|| rel(off)),
        Lower::Ibltz { is, off } => return vu.ibltz(is).then(|| rel(off)),
        Lower::Ibgtz { is, off } => return vu.ibgtz(is).then(|| rel(off)),
        Lower::Iblez { is, off } => return vu.iblez(is).then(|| rel(off)),
        Lower::Ibgez { is, off } => return vu.ibgez(is).then(|| rel(off)),
        Lower::Iadd { id, is, it } => vu.iadd(id, is, it),
        Lower::Isub { id, is, it } => vu.isub(id, is, it),
        Lower::Iaddi { it, is, imm } => vu.iaddi(it, is, imm),
        Lower::Iand { id, is, it } => vu.iand(id, is, it),
        Lower::Ior { id, is, it } => vu.ior(id, is, it),
        Lower::Move { dest, ft, fs } => vu.vmove(dest, ft, fs),
        Lower::Mr32 { dest, ft, fs } => vu.mr32(dest, ft, fs),
        Lower::Lqi { dest, ft, is } => vu.lqi(dest, ft, is),
        Lower::Sqi { dest, fs, it } => vu.sqi(dest, fs, it),
        Lower::Lqd { dest, ft, is } => vu.lqd(dest, ft, is),
        Lower::Sqd { dest, fs, it } => vu.sqd(dest, fs, it),
        Lower::Div { fs, fsf, ft, ftf } => vu.div(fs, fsf, ft, ftf),
        Lower::Sqrt { ft, ftf } => vu.sqrt(ft, ftf),
        Lower::Rsqrt { fs, fsf, ft, ftf } => vu.rsqrt(fs, fsf, ft, ftf),
        Lower::Waitq => vu.waitq(),
        Lower::Mtir { it, fs, fsf } => vu.mtir(it, fs, fsf),
        Lower::Mfir { dest, ft, is } => vu.mfir(dest, ft, is),
        Lower::Ilwr { dest, it, is } => vu.ilwr(dest, it, is),
        Lower::Iswr { dest, it, is } => vu.iswr(dest, it, is),
        Lower::Rnext { dest, ft } => vu.rnext(dest, ft),
        Lower::Rget { dest, ft } => vu.rget(dest, ft),
        Lower::Rinit { fs, fsf } => vu.rinit(fs, fsf),
        Lower::Rxor { fs, fsf } => vu.rxor(fs, fsf),
        Lower::Mfp { dest, ft } => vu.mfp(dest, ft),
        Lower::Xtop { it } => vu.xtop(it),
        Lower::Xitop { it } => vu.xitop(it),
        Lower::Xgkick { is } => vu.xgkick(is, kick),
        Lower::EfuVec { op, fs } => vu.efu_vec(op, fs),
        Lower::EfuScalar { op, fs, fsf } => vu.efu_scalar(op, fs, fsf),
        Lower::Waitp => vu.waitp(),
        Lower::Unknown(w) => vu.unknown_op(pc, w),
    }
    None
}

fn src_text(s: Src) -> String {
    const F: [&str; 4] = ["X", "Y", "Z", "W"];
    match s {
        Src::Vec(t) => format!("Src::Vec({t})"),
        Src::Bc(t, f) => format!("Src::Bc({t}, {})", F[f as usize & 3]),
        Src::Q => "Src::Q".into(),
        Src::I => "Src::I".into(),
    }
}

fn d(m: u8) -> String {
    format!("0b{m:04b}")
}

/// Rust source for an upper op (None for NOP).
pub fn upper_call(u: &Upper) -> Option<String> {
    Some(match *u {
        Upper::Nop => return None,
        Upper::Unknown(w) => format!("vu.unknown_op(PC, {w:#010x});"),
        Upper::Fmac {
            op,
            acc,
            dest,
            fd,
            fs,
            src,
        } => {
            let s = src_text(src);
            match (op, acc) {
                (Fmac::Max, _) | (Fmac::Mini, _) | (_, false) => {
                    format!("vu.{}({}, {fd}, {fs}, {s});", op.name(), d(dest))
                }
                (_, true) => format!("vu.{}a({}, {fs}, {s});", op.name(), d(dest)),
            }
        }
        Upper::Opmula { fs, ft } => format!("vu.opmula({fs}, {ft});"),
        Upper::Opmsub { fd, fs, ft } => format!("vu.opmsub({fd}, {fs}, {ft});"),
        Upper::Itof { frac, dest, ft, fs } => format!("vu.itof({frac}, {}, {ft}, {fs});", d(dest)),
        Upper::Ftoi { frac, dest, ft, fs } => format!("vu.ftoi({frac}, {}, {ft}, {fs});", d(dest)),
        Upper::Abs { dest, ft, fs } => format!("vu.abs({}, {ft}, {fs});", d(dest)),
        Upper::Clip { fs, ft } => format!("vu.clipw({fs}, {ft});"),
    })
}

/// Rust source for a non-branch lower op (None for a no-op MOVE). Branches
/// are emitted by the recompiler itself. `PC` in the text is replaced by the
/// caller.
pub fn lower_call(l: &Lower) -> Option<String> {
    Some(match *l {
        Lower::Lq { dest, ft, is, imm } => format!("vu.lq({}, {ft}, {is}, {imm});", d(dest)),
        Lower::Sq { dest, fs, it, imm } => format!("vu.sq({}, {fs}, {it}, {imm});", d(dest)),
        Lower::Ilw { dest, it, is, imm } => format!("vu.ilw({}, {it}, {is}, {imm});", d(dest)),
        Lower::Isw { dest, it, is, imm } => format!("vu.isw({}, {it}, {is}, {imm});", d(dest)),
        Lower::Iaddiu { it, is, imm } => format!("vu.iaddiu({it}, {is}, {imm:#x});"),
        Lower::Isubiu { it, is, imm } => format!("vu.isubiu({it}, {is}, {imm:#x});"),
        Lower::Fceq { imm } => format!("vu.fceq({imm:#08x});"),
        Lower::Fcset { imm } => format!("vu.fcset({imm:#08x});"),
        Lower::Fcand { imm } => format!("vu.fcand({imm:#08x});"),
        Lower::Fcor { imm } => format!("vu.fcor({imm:#08x});"),
        Lower::Fseq { it, imm } => format!("vu.fseq({it}, {imm:#05x});"),
        Lower::Fsset { imm } => format!("vu.fsset({imm:#05x});"),
        Lower::Fsand { it, imm } => format!("vu.fsand({it}, {imm:#05x});"),
        Lower::Fsor { it, imm } => format!("vu.fsor({it}, {imm:#05x});"),
        Lower::Fmeq { it, is } => format!("vu.fmeq({it}, {is});"),
        Lower::Fmand { it, is } => format!("vu.fmand({it}, {is});"),
        Lower::Fmor { it, is } => format!("vu.fmor({it}, {is});"),
        Lower::Fcget { it } => format!("vu.fcget({it});"),
        Lower::Iadd { id, is, it } => format!("vu.iadd({id}, {is}, {it});"),
        Lower::Isub { id, is, it } => format!("vu.isub({id}, {is}, {it});"),
        Lower::Iaddi { it, is, imm } => format!("vu.iaddi({it}, {is}, {imm});"),
        Lower::Iand { id, is, it } => format!("vu.iand({id}, {is}, {it});"),
        Lower::Ior { id, is, it } => format!("vu.ior({id}, {is}, {it});"),
        Lower::Move { dest, ft, fs } => {
            if dest == 0 || ft == 0 {
                return None;
            }
            format!("vu.vmove({}, {ft}, {fs});", d(dest))
        }
        Lower::Mr32 { dest, ft, fs } => format!("vu.mr32({}, {ft}, {fs});", d(dest)),
        Lower::Lqi { dest, ft, is } => format!("vu.lqi({}, {ft}, {is});", d(dest)),
        Lower::Sqi { dest, fs, it } => format!("vu.sqi({}, {fs}, {it});", d(dest)),
        Lower::Lqd { dest, ft, is } => format!("vu.lqd({}, {ft}, {is});", d(dest)),
        Lower::Sqd { dest, fs, it } => format!("vu.sqd({}, {fs}, {it});", d(dest)),
        Lower::Div { fs, fsf, ft, ftf } => format!("vu.div({fs}, {fsf}, {ft}, {ftf});"),
        Lower::Sqrt { ft, ftf } => format!("vu.sqrt({ft}, {ftf});"),
        Lower::Rsqrt { fs, fsf, ft, ftf } => format!("vu.rsqrt({fs}, {fsf}, {ft}, {ftf});"),
        Lower::Waitq => "vu.waitq();".into(),
        Lower::Mtir { it, fs, fsf } => format!("vu.mtir({it}, {fs}, {fsf});"),
        Lower::Mfir { dest, ft, is } => format!("vu.mfir({}, {ft}, {is});", d(dest)),
        Lower::Ilwr { dest, it, is } => format!("vu.ilwr({}, {it}, {is});", d(dest)),
        Lower::Iswr { dest, it, is } => format!("vu.iswr({}, {it}, {is});", d(dest)),
        Lower::Rnext { dest, ft } => format!("vu.rnext({}, {ft});", d(dest)),
        Lower::Rget { dest, ft } => format!("vu.rget({}, {ft});", d(dest)),
        Lower::Rinit { fs, fsf } => format!("vu.rinit({fs}, {fsf});"),
        Lower::Rxor { fs, fsf } => format!("vu.rxor({fs}, {fsf});"),
        Lower::Mfp { dest, ft } => format!("vu.mfp({}, {ft});", d(dest)),
        Lower::Xtop { it } => format!("vu.xtop({it});"),
        Lower::Xitop { it } => format!("vu.xitop({it});"),
        Lower::Xgkick { is } => format!("vu.xgkick({is}, kick);"),
        Lower::EfuVec { op, fs } => format!("vu.efu_vec(Efu::{:?}, {fs});", op),
        Lower::EfuScalar { op, fs, fsf } => format!("vu.efu_scalar(Efu::{:?}, {fs}, {fsf});", op),
        Lower::Waitp => "vu.waitp();".into(),
        Lower::Unknown(w) => format!("vu.unknown_op(PC, {w:#010x});"),
        Lower::B { .. }
        | Lower::Bal { .. }
        | Lower::Jr { .. }
        | Lower::Jalr { .. }
        | Lower::Ibeq { .. }
        | Lower::Ibne { .. }
        | Lower::Ibltz { .. }
        | Lower::Ibgtz { .. }
        | Lower::Iblez { .. }
        | Lower::Ibgez { .. } => return None,
    })
}
