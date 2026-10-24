//! Debug switches (`SSX3_GFX_*`), read once.

use std::sync::OnceLock;

pub struct Dbg {
    /// `SSX3_GFX_TRACE=<frame>`: trace that frame's primitives.
    pub trace: Option<u64>,
    /// `SSX3_GFX_TRACE_OUT`: where the trace goes.
    pub trace_out: String,
    /// `SSX3_GFX_FLIP=0`: no flip packet.
    pub no_flip: bool,
    /// `SSX3_GFX_PATH3=0..3`: which PATH3 chains run (bit 0 phase 1, bit 1 phase 2).
    pub path3: u32,
    /// `SSX3_GFX_SKIP=n,a-b`: draws whose payloads are dropped.
    pub skip: Vec<usize>,
    /// `SSX3_GFX_TEXDUMP=dir`: decoded textures as raw RGBA.
    pub texdump: Option<String>,
    /// `SSX3_GFX_SKIPTEST=<test>[a|c|z]`: batches dropped (or kept without
    /// depth writes) by TEST.
    pub skip_test: Vec<(u64, u8)>,
    /// `SSX3_GFX_PROBE=x,y`: a pixel after every item that changes it.
    pub probe: Option<(u32, u32)>,
    /// `SSX3_GFX_TEXLOG=<frame>`: texture keys used in that frame.
    pub texlog: Option<u64>,
    /// `SSX3_GFX_PLANLOG=<test>`: blend plans of batches with that TEST.
    pub planlog: Option<u64>,
    /// `SSX3_GFX_VUCHECK=1`: compare the parallel VU1 calls with a
    /// sequential run.
    pub vucheck: bool,
}

pub fn get() -> &'static Dbg {
    static D: OnceLock<Dbg> = OnceLock::new();
    D.get_or_init(|| {
        let var = |k: &str| std::env::var(k).ok();
        Dbg {
            trace: var("SSX3_GFX_TRACE").and_then(|v| v.parse().ok()),
            trace_out: var("SSX3_GFX_TRACE_OUT").unwrap_or_else(|| "/tmp/gfx-trace.txt".into()),
            no_flip: var("SSX3_GFX_FLIP").is_some_and(|v| v == "0"),
            path3: var("SSX3_GFX_PATH3")
                .and_then(|v| v.parse().ok())
                .unwrap_or(3),
            skip: var("SSX3_GFX_SKIP")
                .map(|v| {
                    v.split(',')
                        .flat_map(|x| match x.split_once('-') {
                            Some((a, b)) => {
                                a.parse().unwrap_or(1)..b.parse::<usize>().map_or(0, |b| b + 1)
                            }
                            None => x.parse().map_or(0..0, |a: usize| a..a + 1),
                        })
                        .collect()
                })
                .unwrap_or_default(),
            texdump: var("SSX3_GFX_TEXDUMP"),
            skip_test: var("SSX3_GFX_SKIPTEST")
                .map(|v| {
                    v.split(',')
                        .filter_map(|x| {
                            let m = x
                                .as_bytes()
                                .last()
                                .copied()
                                .filter(|c| matches!(c, b'c' | b'a' | b'z'))
                                .unwrap_or(0);
                            u64::from_str_radix(
                                x.trim_start_matches("0x").trim_end_matches(['a', 'c', 'z']),
                                16,
                            )
                            .ok()
                            .map(|t| (t, m))
                        })
                        .collect()
                })
                .unwrap_or_default(),
            probe: var("SSX3_GFX_PROBE").and_then(|v| {
                v.split_once(',')
                    .and_then(|(x, y)| Some((x.parse().ok()?, y.parse().ok()?)))
            }),
            texlog: var("SSX3_GFX_TEXLOG").and_then(|v| v.parse().ok()),
            planlog: var("SSX3_GFX_PLANLOG").and_then(|v| u64::from_str_radix(&v, 16).ok()),
            vucheck: var("SSX3_GFX_VUCHECK").is_some(),
        }
    })
}
