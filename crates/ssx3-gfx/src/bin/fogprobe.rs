//! Debug: what the depth-fog setup pass (PATH3 phase 1, before the PSMT8H
//! CLUT pass) leaves in the frame's alpha, as a function of Z. Runs the
//! frame's own packets on the software GS over a known Z pattern.
//!
//! ```sh
//! fogprobe frame-0.bin
//! ```

use ssx3_ps2::frame_sink;
use ssx3_ps2::gs::{swizzle, Gs};

fn main() {
    let path = std::env::args().nth(1).expect("dump");
    let bytes = std::fs::read(&path).unwrap();
    let (mem, carry) = frame_sink::load_dump(&bytes).unwrap();
    let built = frame_sink::build_dump(&mem, &carry).unwrap();
    let mut q = Vec::new();
    let mut asr = [0u32; 2];
    ssx3_render::chain::walk(&mem, built.frame.gif_heads[0], false, false, 0, &mut asr, &mut |d: &[u128]| q.extend_from_slice(d));
    // Cut before the first tag that sets PRIM 0x156 (the CLUT pass).
    let mut i = 0;
    let mut cut = q.len();
    while i < q.len() {
        let lo = q[i] as u64;
        let nloop = (lo & 0x7fff) as usize;
        let flg = (lo >> 58) & 3;
        let nreg = match (lo >> 60) as usize { 0 => 16, n => n };
        if (lo >> 46) & 1 != 0 && (lo >> 47) & 0x7ff == 0x156 {
            cut = i;
            break;
        }
        // A+D PRIM writes too.
        if flg == 0 && nreg == 1 && ((q[i] >> 64) & 0xf) == 0xe {
            let mut hit = false;
            for k in 0..nloop {
                let d = q[i + 1 + k];
                if (d >> 64) as u8 == 0 && (d as u64 & 0x7ff) == 0x156 {
                    hit = true;
                }
            }
            if hit {
                cut = i;
                break;
            }
        }
        i += 1 + match flg { 0 => nloop * nreg, 1 => (nloop * nreg).div_ceil(2), _ => nloop };
    }
    eprintln!("p1: {} qwords, cut at {cut}", q.len());
    let mut gs = Gs::new_cpu();
    let (zbp, w, h) = (0x1c00u32, 512u32, 448u32);
    let zof = |x: u32, y: u32| -> u32 { (x.wrapping_mul(2654435761) ^ y.wrapping_mul(40503) ^ (x * 7 + y * 131) << 9) & 0xff_ffff };
    gs.draw_log = Some(Vec::new());
    let fp = ssx3_render::frame::flip_packet(&mem);
    eprintln!("flip packet {} qwords, first tag {:#x}", fp.len(), fp[0]);
    gs.gif_write(1, &fp);
    gs.sync();
    for y in 0..h {
        for x in 0..w {
            swizzle::write_pixel(&mut gs.vram, 0x31, zbp, 8, x, y, zof(x, y));
            swizzle::write_pixel(&mut gs.vram, 0, 0, 8, x, y, 0);
        }
    }
    gs.vram_written();
    gs.gif_write(1, &q[..cut]);
    gs.sync();
    gs.sync_vram();
    eprintln!("vram nonzero words: {}, backend {:?}", gs.vram.iter().filter(|&&v| v != 0).count(), gs.backend());
    let nz = (0..h).flat_map(|y| (0..w).map(move |x| (x, y))).filter(|&(x, y)| swizzle::read_pixel(&gs.vram, 0, 0, 8, x, y) != 0).count();
    for e in gs.draw_log.as_ref().unwrap() {
        eprintln!("draw prim {:#x} prims {} frame {:#x} tex0 {:#x} test {:#x}", e.prim, e.prims, e.frame, e.tex0, e.test);
    }
    eprintln!("frame pixels nonzero: {nz}; regs {:x?}", gs.persistent_regs());
    // For each alpha bit: which Z bit of which neighbour matches everywhere.
    for b in 0..8 {
        let mut found = Vec::new();
        for (dx, dy) in [(0i32, 0i32), (1, 0), (-1, 0), (0, 1), (0, -1), (8, 0), (-8, 0)] {
            for k in 0..24 {
                let mut ok = true;
                let mut n = 0;
                'p: for y in 2..h - 2 {
                    for x in 16..w - 16 {
                        let a = (swizzle::read_pixel(&gs.vram, 0, 0, 8, x, y) >> (24 + b)) & 1;
                        let z = (zof((x as i32 + dx) as u32, (y as i32 + dy) as u32) >> k) & 1;
                        n += 1;
                        if a != z {
                            ok = false;
                            break 'p;
                        }
                    }
                }
                if ok && n > 0 {
                    found.push(format!("z[{k}]@({dx},{dy})"));
                }
            }
        }
        let ones = (0..h).flat_map(|y| (0..w).map(move |x| (x, y))).filter(|&(x, y)| (swizzle::read_pixel(&gs.vram, 0, 0, 8, x, y) >> (24 + b)) & 1 != 0).count();
        println!("alpha bit {b}: {found:?} (set in {ones} pixels)");
    }
}
