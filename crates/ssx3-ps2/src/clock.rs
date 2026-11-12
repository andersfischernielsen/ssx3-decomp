//! Clock selection shared by CDVD and memory-card timestamps.

use std::ffi::OsStr;

pub(crate) fn seconds(fixed: u64) -> u64 {
    let rtc = ssx3_core::config::var("SSX3_RTC");
    let real = ssx3_core::config::var_os("SSX3_REAL_CLOCK");
    select(rtc.as_deref(), real.as_deref(), fixed, || {
        web_time::SystemTime::now()
            .duration_since(web_time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    })
}

fn select(rtc: Option<&str>, real: Option<&OsStr>, fixed: u64, host: impl FnOnce() -> u64) -> u64 {
    if let Some(value) = rtc.and_then(|v| v.parse().ok()) {
        value
    } else if real.is_some_and(|value| value != OsStr::new("0")) {
        host()
    } else {
        fixed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unset_or_zero_uses_the_fixed_clock() {
        for value in [None, Some(OsStr::new("0"))] {
            assert_eq!(select(None, value, 42, || panic!("host clock read")), 42);
        }
    }

    #[test]
    fn other_present_values_keep_enabling_the_host_clock() {
        for value in ["1", "", "false", "true", "2", "00", " 0"] {
            assert_eq!(select(None, Some(OsStr::new(value)), 42, || 99), 99);
        }
    }

    #[test]
    fn non_unicode_values_keep_enabling_the_host_clock() {
        #[cfg(windows)]
        let value = {
            use std::os::windows::ffi::OsStringExt;
            std::ffi::OsString::from_wide(&[0xd800])
        };
        #[cfg(unix)]
        let value = {
            use std::os::unix::ffi::OsStringExt;
            std::ffi::OsString::from_vec(vec![0xff])
        };
        #[cfg(any(windows, unix))]
        assert_eq!(select(None, Some(&value), 42, || 99), 99);
    }

    #[test]
    fn valid_rtc_takes_precedence_including_zero() {
        for value in [None, Some("0"), Some("1")] {
            for rtc in ["0", "3600"] {
                assert_eq!(
                    select(Some(rtc), value.map(OsStr::new), 42, || panic!(
                        "host clock read"
                    )),
                    rtc.parse::<u64>().unwrap()
                );
            }
        }
    }

    #[test]
    fn invalid_rtc_falls_back_to_the_selected_clock() {
        for rtc in ["", "invalid", "-1", "18446744073709551616"] {
            assert_eq!(
                select(Some(rtc), Some(OsStr::new("0")), 42, || panic!(
                    "host clock read"
                )),
                42
            );
            assert_eq!(select(Some(rtc), Some(OsStr::new("1")), 42, || 99), 99);
        }
    }
}
