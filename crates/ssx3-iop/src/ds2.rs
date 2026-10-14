//! DualShock 2 controller on the SIO2 bus.
//!
//! A packet starts with `0x01` (controller), then the command byte; the pad
//! answers `0xFF`, its mode id (`0x41` digital, `0x73` analog, `0x79`
//! analog with pressure, `0xF3` in config mode) and `0x5A`, then command
//! data. Commands (behaviour as documented by PCSX2's `PadDualshock2.cpp`
//! and the community "DualShock 2 protocol" notes):
//!
//! | cmd  | meaning |
//! | ---- | ------- |
//! | 0x40 | ? (constant reply) |
//! | 0x41 | which buttons are reported (analog modes) |
//! | 0x42 | poll: buttons, sticks, pressures; bytes 3/4 drive the motors |
//! | 0x43 | enter (byte 3 = 1) / leave (0) config mode |
//! | 0x44 | set mode: byte 3 analog on/off, byte 4 = 3 locks the mode button |
//! | 0x45 | status: controller type, analog light |
//! | 0x46, 0x47, 0x4C | constant tables (actuator / mode descriptions) |
//! | 0x4D | map motors to poll bytes |
//! | 0x4F | which response bytes to send (0x3FFFF: pressures, mode 0x79) |
//!
//! Buttons come from the host as the libpad bit layout, active high
//! (bit 0 Select … bit 15 Square); on the wire they are active low, low byte
//! first. Sticks come from the host too (`sticks`, 0x80 = centre).

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Digital,
    Analog,
    Pressure,
}

pub struct DualShock2 {
    pub connected: bool,
    /// Host input, libpad layout, active high.
    pub buttons: u16,
    /// rx, ry, lx, ly.
    pub sticks: [u8; 4],
    pub mode: Mode,
    pub config: bool,
    pub analog_light: bool,
    pub locked: bool,
    response_bytes: u32,
    /// Motor mapping from 0x4D: small motor, large motor (0xFF = off).
    small_map: u8,
    large_map: u8,
    pub motors: [u8; 2],
    stage: bool,
    pub polls: u64,
}

impl Default for DualShock2 {
    fn default() -> Self {
        Self::new()
    }
}

/// Pressure byte order after the sticks (0x79 mode): libpad button bits.
const PRESSURE_BITS: [u32; 12] = [5, 7, 4, 6, 12, 13, 14, 15, 10, 11, 8, 9];

impl DualShock2 {
    pub fn new() -> Self {
        Self {
            connected: true,
            buttons: 0,
            sticks: [0x80; 4],
            mode: Mode::Digital,
            config: false,
            analog_light: false,
            locked: false,
            response_bytes: 0,
            small_map: 0xFF,
            large_map: 0xFF,
            motors: [0; 2],
            stage: false,
            polls: 0,
        }
    }

    pub fn mode_id(&self) -> u8 {
        if self.config {
            return 0xF3;
        }
        match self.mode {
            Mode::Digital => 0x41,
            Mode::Analog => 0x73,
            Mode::Pressure => 0x79,
        }
    }

    /// Poll reply byte `i` (3 and up).
    fn poll_byte(&self, i: usize) -> u8 {
        let wire = !self.buttons;
        match i {
            3 => wire as u8,
            4 => (wire >> 8) as u8,
            5..=8 => self.sticks[i - 5],
            9..=20 => {
                if self.buttons >> PRESSURE_BITS[i - 9] & 1 != 0 {
                    0xFF
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    /// Answer one packet (`p[0]` = 0x01). One reply byte per packet byte.
    pub fn transfer(&mut self, p: &[u8]) -> Vec<u8> {
        if !self.connected {
            return vec![0xFF; p.len()];
        }
        let mut out = vec![0u8; p.len()];
        out[0] = 0xFF;
        if p.len() < 2 {
            return out;
        }
        let cmd = p[1];
        out[1] = self.mode_id();
        if p.len() > 2 {
            out[2] = 0x5A;
        }
        let was_config = self.config;
        for i in 3..p.len() {
            let b = p[i];
            out[i] = match cmd {
                0x40 => match i {
                    5 => 0x02,
                    8 => 0x5A,
                    _ => 0,
                },
                0x41 => match (self.mode, i) {
                    (Mode::Digital, _) => 0,
                    (_, 3 | 4) => 0xFF,
                    (_, 5) => 0x03,
                    (_, 8) => 0x5A,
                    _ => 0,
                },
                0x42 => {
                    if i == 3 || i == 4 {
                        self.motors[i - 3] = b;
                    }
                    self.poll_byte(i)
                }
                0x43 => {
                    if i == 3 {
                        self.config = b != 0;
                    }
                    // Outside config mode the pad also reports its buttons.
                    if was_config {
                        0
                    } else {
                        self.poll_byte(i)
                    }
                }
                0x44 => {
                    match i {
                        3 => {
                            self.analog_light = b != 0;
                            self.mode = if self.analog_light { Mode::Analog } else { Mode::Digital };
                        }
                        4 => self.locked = b == 3,
                        _ => {}
                    }
                    0
                }
                0x45 => match i {
                    3 => 0x03, // DualShock 2
                    4 => 0x02,
                    5 => self.analog_light as u8,
                    6 => 0x02,
                    7 => 0x01,
                    _ => 0,
                },
                0x46 => {
                    if i == 3 {
                        self.stage = b != 0;
                    }
                    match i {
                        5 => 0x01,
                        6 => 1 + !self.stage as u8,
                        7 => self.stage as u8,
                        8 => {
                            if self.stage {
                                0x14
                            } else {
                                0x0A
                            }
                        }
                        _ => 0,
                    }
                }
                0x47 => match i {
                    5 => 0x02,
                    7 => 0x01,
                    _ => 0,
                },
                0x4C => {
                    if i == 3 {
                        self.stage = b != 0;
                    }
                    match i {
                        6 => {
                            if self.stage {
                                0x07
                            } else {
                                0x04
                            }
                        }
                        _ => 0,
                    }
                }
                0x4D => match i {
                    3 => std::mem::replace(&mut self.small_map, b),
                    4 => std::mem::replace(&mut self.large_map, b),
                    _ => 0xFF,
                },
                0x4F => {
                    match i {
                        3 => self.response_bytes = b as u32,
                        4 => self.response_bytes |= (b as u32) << 8,
                        5 => {
                            self.response_bytes |= (b as u32) << 16;
                            (self.mode, self.analog_light) = match self.response_bytes {
                                0x3_FFFF => (Mode::Pressure, true),
                                0x3F => (Mode::Analog, true),
                                _ => (Mode::Digital, false),
                            };
                        }
                        _ => {}
                    }
                    if i == 8 {
                        0x5A
                    } else {
                        0
                    }
                }
                _ => 0,
            };
        }
        if cmd == 0x42 {
            self.polls += 1;
        }
        out
    }
}
