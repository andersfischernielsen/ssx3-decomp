# Tick-rate pass: classification tables

Worker outputs of the tick-rate pass (`../configurable-tick-rate.md`), one row per
site with the class and the reason. They are the evidence behind the conversions;
consult them when a newly ported function loads `1/60` or counts frames.

| files | pass | columns |
| --- | --- | --- |
| `t1-a..d.tsv` | T1: the `.lit4` `1/60` entries of the inventory (`../tick-rate-functions.tsv`) | `entry site function class reason` (`STEP`, `NOT_A_STEP`, `UNSURE`) |
| `t2-a..c.tsv` | T2: functions ported after the inventory and their `1/60` load sites | `addr name unit labels sites pool` |
| `t4-a..e.tsv` | T4: per-tick updates with no `1/60` marker (`tools/scan_tick_patterns.py`) | `site function class kind constant reason conversion` |
| `t5-a..h.tsv` | T5: the rest of the game (menus, replays, online, unvisited screens) | as T4, plus `func_addr ported` |
| `chasecam-callers.tsv` | `ChaseCamController_Update` blend weights, per caller | `site function address rate_source unit action why` |

The writer inputs and briefs of T1/T4/T5 were removed (git history has them); the
writer rules are in `AGENTS.md` ("Tick-rate pass").
