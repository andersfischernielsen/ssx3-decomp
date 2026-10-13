//! Controller (padman) RPC server, provided natively.
//!
//! Placeholder: accepts every request. The EE's libpad keeps its pad state in
//! an EE-side DMA buffer that the IOP refreshes every frame; that refresh is
//! implemented once the boot path reaches pad polling.

use crate::cpu::Cpu;

pub fn rpc(c: &mut Cpu, method: u32, _args: u32, _ssize: u32, ret: u32, rsize: u32) -> bool {
    if c.sif.trace {
        eprintln!("[pad] method {method:#x} recv {rsize}");
    }
    if rsize > 0 {
        let z = vec![0u8; rsize as usize];
        c.bus.write_bytes(ret, &z);
    }
    true
}
