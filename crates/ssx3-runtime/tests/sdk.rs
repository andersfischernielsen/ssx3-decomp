//! Tests for the runtime state and the native SDK boundary.

use ssx3_runtime::{sdk, Cpu};

#[test]
fn init_heap_sets_and_returns_heap_end() {
    let mut c = Cpu::new();
    let end = sdk::init_heap(&mut c, 0x0010_0000, 0x0008_0000);
    assert_eq!(c.heap_base, 0x0010_0000);
    assert_eq!(c.heap_end, 0x0018_0000);
    assert_eq!(end, 0x0018_0000);
    assert_eq!(sdk::end_of_heap(&mut c), 0x0010_0000);
    assert_eq!(sdk::get_heap_end(&mut c), 0x0010_0000);
}

#[test]
fn init_main_thread_returns_the_stack_pointer() {
    let mut c = Cpu::new();
    let sp = sdk::init_main_thread(&mut c, 0x004a_30f0, 0x01e0_0000, 0x1000, 0);
    assert_eq!(sp, 0x01e0_0000);
}

#[test]
fn gs_imr_roundtrips() {
    let mut c = Cpu::new();
    sdk::gs_put_imr(&mut c, 0xdead_beef);
    assert_eq!(c.gs_imr, 0xdead_beef);
    assert_eq!(sdk::gs_get_imr(&mut c), 0xdead_beef);
}

#[test]
fn set_memory_mode_returns_previous() {
    let mut c = Cpu::new();
    assert_eq!(sdk::set_memory_mode(&mut c, 1), 0);
    assert_eq!(sdk::set_memory_mode(&mut c, 2), 1);
    assert_eq!(c.memory_mode, 2);
}

#[test]
fn dynamic_syscall_records_and_returns_zero() {
    let mut c = Cpu::new();
    c.set_reg(3, 0x1234);
    assert_eq!(sdk::dynamic_syscall(&mut c), 0);
    assert_eq!(c.syscalls, vec![0x1234]);
    assert_eq!(c.reg(2), 0);
}

#[test]
fn note_syscall_is_bounded() {
    let mut c = Cpu::new();
    c.syscall_limit = 3;
    for i in 0..10 {
        c.note_syscall(i);
    }
    assert_eq!(c.syscalls.len(), 3);
}

#[test]
fn exit_stops_and_unwinds() {
    let mut c = Cpu::new();
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| sdk::exit(&mut c, 7)));
    assert!(r.is_err(), "exit should unwind via ExitReached");
    assert!(c.trap.is_some());
}

#[test]
fn register_zero_is_hardwired() {
    let mut c = Cpu::new();
    c.set_reg(0, 0xffff_ffff);
    assert_eq!(c.reg(0), 0);
}

#[test]
fn memory_helpers_roundtrip() {
    let mut c = Cpu::new();
    c.store_u32(0x0010_1000, 0x1122_3344);
    assert_eq!(c.load_u32(0x0010_1000), 0x1122_3344);
    c.store_u8(0x0010_1004, 0xff);
    assert_eq!(c.load_i8(0x0010_1004), 0xffff_ffff_ffff_ffff);
    c.store_u64(0x0010_2000, 0x0102_0304_0506_0708);
    assert_eq!(c.load_u64(0x0010_2000), 0x0102_0304_0506_0708);
}

#[test]
fn tick_enforces_the_step_limit() {
    let mut c = Cpu::new();
    c.step_limit = 3;
    assert!(c.tick(0x1000, 0));
    assert!(c.tick(0x1004, 0));
    assert!(c.tick(0x1008, 0));
    assert!(!c.tick(0x100c, 0), "fourth tick exceeds the limit");
    assert!(c.trap.is_some());
}
