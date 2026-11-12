//! Clock selection shared by CDVD and memory-card timestamps.

pub(crate) fn seconds(fixed: u64) -> u64 {
    let rtc = ssx3_core::config::var("SSX3_RTC");
    let real = ssx3_core::config::var("SSX3_REAL_CLOCK");
    select(rtc.as_deref(), real.as_deref(), fixed, || {
        web_time::SystemTime::now()
            .duration_since(web_time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    })
}

fn select(rtc: Option<&str>, real: Option<&str>, fixed: u64, host: impl FnOnce() -> u64) -> u64 {
    if let Some(value) = rtc.and_then(|v| v.parse().ok()) {
        value
    } else if real == Some("1") {
        host()
    } else {
        fixed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_one_enables_the_host_clock() {
        for value in [
            None,
            Some("0"),
            Some(""),
            Some("false"),
            Some("true"),
            Some("2"),
        ] {
            assert_eq!(select(None, value, 42, || panic!("host clock read")), 42);
        }
        assert_eq!(select(None, Some("1"), 42, || 99), 99);
    }

    #[test]
    fn valid_rtc_takes_precedence_including_zero() {
        for value in [None, Some("0"), Some("1")] {
            for rtc in ["0", "3600"] {
                assert_eq!(
                    select(Some(rtc), value, 42, || panic!("host clock read")),
                    rtc.parse::<u64>().unwrap()
                );
            }
        }
    }

    #[test]
    fn invalid_rtc_falls_back_to_the_selected_clock() {
        for rtc in ["", "invalid", "-1", "18446744073709551616"] {
            assert_eq!(
                select(Some(rtc), Some("0"), 42, || panic!("host clock read")),
                42
            );
            assert_eq!(select(Some(rtc), Some("1"), 42, || 99), 99);
        }
    }
}
