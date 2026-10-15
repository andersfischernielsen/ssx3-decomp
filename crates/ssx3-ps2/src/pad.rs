//! Controller support: the IOP side of libpad (PADMAN.IRX), provided natively.
//!
//! The EE's libpad keeps no button state of its own: `scePadPortOpen` hands the
//! IOP a 256-byte buffer holding two 0x80-byte `pad_data` frames, and the IOP
//! DMAs fresh controller data into it every frame. `scePadRead` (0x3ffa58) and
//! friends copy whichever frame has the larger `frame` counter. Layout (from the
//! game's `scePadPortOpen` at 0x3ff708 and the public libpad structure):
//!
//! | off  | field |
//! | ---- | ----- |
//! | 0x00 | data[32]: [0]=0, [1]=mode id (0x73 DualShock), [2..4]=buttons (active-low), [4..8]=sticks |
//! | 0x58 | frame |
//! | 0x60 | length (bytes valid in data) |
//! | 0x64 | modeConfig, modeCurId, model, buttonDataReady |
//! | 0x68 | nrOfModes, modeCurOffs, nrOfActuators, numActComb |
//! | 0x70 | state (6 = stable), reqState (0 = complete), currentTask |

use crate::cpu::Cpu;

#[derive(Clone, Copy, Default, Debug)]
pub struct OpenPort {
    pub port: u32,
    pub slot: u32,
    pub buf: u32,
}

/// RPC server for padman (0x80000100 / 0x80000101).
pub fn rpc(c: &mut Cpu, method: u32, args: u32, _ssize: u32, ret: u32, rsize: u32) -> bool {
    let w = |c: &mut Cpu, i: u32| c.bus.ram_u32(args + 4 * i);
    let cmd = w(c, 0);
    if m!(c).sif.trace {
        eprintln!("[pad] fn {method:#x} cmd {cmd:#x}");
    }
    // Default reply: echo the request and report success in word 3.
    if rsize >= 0x10 && ret != args {
        let req = c.bus.read_bytes(args, rsize.min(0x80) as usize);
        c.bus.write_bytes(ret, &req);
    }
    let result = match cmd & 0xff {
        // Open: [1]=port [2]=slot [4]=buffer.
        0x01 => {
            let p = OpenPort { port: w(c, 1), slot: w(c, 2), buf: w(c, 4) & 0x01FF_FFFF };
            m!(c).sif.pads.retain(|o| !(o.port == p.port && o.slot == p.slot));
            m!(c).sif.pads.push(p);
            eprintln!("[pad] open port {} slot {} buffer {:#x}", p.port, p.slot, p.buf);
            refresh(c);
            1
        }
        // GET_PORTMAX / GET_SLOTMAX.
        0x0b => 2,
        0x0c => 1,
        _ => 1,
    };
    if rsize >= 0x18 {
        c.bus.set_ram_u32(ret + 0x0c, result);
        c.bus.set_ram_u32(ret + 0x14, 1);
    } else if rsize >= 4 {
        c.bus.set_ram_u32(ret, result);
    }
    true
}

/// Write the current host input into every open pad buffer (called per VBlank).
pub fn refresh(c: &mut Cpu) {
    m!(c).sif.pad_frame = m!(c).sif.pad_frame.wrapping_add(1);
    let frame = m!(c).sif.pad_frame;
    let pads = m!(c).sif.pads.clone();
    for p in pads {
        let buttons = !m!(c).sif.pad_buttons[(p.port & 1) as usize];
        let mut d = [0u8; 0x80];
        d[0] = 0;
        d[1] = 0x73;
        d[2..4].copy_from_slice(&buttons.to_le_bytes());
        d[4..8].copy_from_slice(&m!(c).sif.pad_sticks[(p.port & 1) as usize]);
        d[0x58..0x5c].copy_from_slice(&frame.to_le_bytes());
        d[0x60..0x64].copy_from_slice(&32u32.to_le_bytes());
        d[0x64] = 2; // modeConfig: mode table available
        d[0x65] = 0x73; // modeCurId: DualShock
        d[0x66] = 3; // model
        d[0x67] = 1; // buttonDataReady
        d[0x68] = 2; // nrOfModes
        d[0x69] = 1; // modeCurOffs
        d[0x6a] = 2; // nrOfActuators
        d[0x6b] = 1;
        d[0x50..0x54].copy_from_slice(&[0x04, 0x00, 0x07, 0x00]); // modeTable
        d[0x70] = 6; // state: stable
        d[0x71] = 0; // reqState: complete
        d[0x72] = 1; // currentTask
        let at = p.buf + if frame & 1 == 0 { 0 } else { 0x80 };
        c.bus.write_bytes(at, &d);
    }
}

/// Host input: set or clear a button (libpad bit layout, active-high here).
pub fn set_button(c: &mut Cpu, port: usize, bit: u16, pressed: bool) {
    let b = &mut m!(c).sif.pad_buttons[port & 1];
    if pressed {
        *b |= bit;
    } else {
        *b &= !bit;
    }
}

/// Host input: an analog stick axis (`axis` 0..4 = rx, ry, lx, ly; 0x80 = centre).
pub fn set_stick(c: &mut Cpu, port: usize, axis: usize, value: u8) {
    m!(c).sif.pad_sticks[port & 1][axis & 3] = value;
}
