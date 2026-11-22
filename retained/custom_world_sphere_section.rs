//! Native World_GetObjectSectionData_2 (0x003a6cd8), retained uninstalled.
//! No tests, cargo, game runs or replays: disabled by the user.

use crate::abi;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::{A1, V0, V1};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x003a_6cd8,
    world_get_object_section_data_2,
    "World_GetObjectSectionData_2",
)];

const INSTANCE_PROPERTY: u32 = 0x88;
const PROPERTY_COLLIDER_RESOURCE: u32 = 0x0c;
const RESOURCE_DATA: u32 = 8;

/// `void* World_GetObjectSectionData_2(World* unused, Instance* instance)`.
/// Return `instance->property->colliderResource->data`, following offsets
/// +0x88, +0x0c and +8. Retain the resource pointer in V1 and the resulting
/// data pointer in V0, with the native sign-extended word loads. The sphere
/// consumer treats that data as the runtime header whose first word is the
/// base of the 0x140-byte sphere-body array; this getter does not index it.
pub fn world_get_object_section_data_2(c: &mut Cpu) {
    c.cycles += 4;
    let instance = c.g[A1][0] as u32;
    c.g[V0][0] = Addr(instance).off(INSTANCE_PROPERTY).lw(c);
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(PROPERTY_COLLIDER_RESOURCE).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(RESOURCE_DATA).lw(c);
    abi::ret(c);
}
