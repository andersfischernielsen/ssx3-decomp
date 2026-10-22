//! `src/camera/trigger/`.

use ssx3_ee_rt::funcs::GuestFn;

pub mod cameratriggerfactory;
pub mod cameratriggerlist;
pub mod cameratriggerman;
pub mod cameratriggervolumes;
pub mod triggeralgorithms;

/// Every replacement in this directory.
pub fn overrides() -> Vec<(u32, GuestFn, &'static str)> {
    let mut v = Vec::new();
    v.extend_from_slice(cameratriggerfactory::OVERRIDES);
    v.extend_from_slice(cameratriggerlist::OVERRIDES);
    v.extend_from_slice(cameratriggerman::OVERRIDES);
    v.extend_from_slice(cameratriggervolumes::OVERRIDES);
    v.extend_from_slice(triggeralgorithms::OVERRIDES);
    v
}
