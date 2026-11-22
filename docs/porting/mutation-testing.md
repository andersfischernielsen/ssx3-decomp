# Mutation testing the differential tests

A differential test that passes shows the port and the generated body agree on
the states the fixture builds. It does not show the test would fail if they
did not: a fixture that never reaches a branch passes either way.
[cargo-mutants](https://mutants.rs/) checks that. It makes small changes to the
hand-written body (`<` to `<=`, `|` to `^`, a deleted `!`, a deleted match
arm), rebuilds, and runs the unit's tests. A change no test fails on (a
*survivor*) marks a line the tests do not check.

Over 323 ported functions it made 3,999 changes and 421 survived. The 60
survivors in the #42/#47 tagsunk ports held 33 fixture gaps (#53) and no port
mistakes.

## When

On a port PR or a new unit, for that PR's functions only, after the module is
green (porting loop step 6). Not on a whole wave: a function costs a few
minutes (below), so 80 functions take most of a day.

## Running it

Install once: `cargo install --locked cargo-mutants` (a cargo subcommand, not a
dependency; checked with 27.1.0). Then, from a green tree:

```sh
python3 tools/mutants.py crates/ssx3-game/src/game/dirtysock/tags.rs tag_field_set_date
git diff origin/main... > /tmp/pr.diff && python3 tools/mutants.py --in-diff /tmp/pr.diff
```

The first form takes a unit and function names; the second, every mutant on
the lines a diff adds (run it on the PR's branch, so the diff matches the
tree). `--list` counts the mutants and runs nothing. The script lists the
mutants once, leaves out the `c.cycles +=` lines (the tests compare cycles, so
their `-=` and `*=` are caught, or equivalent when the count happens to match;
they are more than half of all mutants), and runs the rest in place with only
the touched units' tests. `.cargo/mutants.toml` adds the `game` profile,
incremental builds and `SSX3_DIFF_STRICT=args`. It prints per function the
caught, missed, unviable and timed-out mutants, then each survivor with its
source line. Everything is under `target/mutants.out/` (`missed.txt`, a log
per mutant).

One mutant is a rebuild of `ssx3-game` and a run of the unit's tests: 30-45 s
on 8 cores (105 s without incremental builds). The first run rebuilds once for
incremental (about 4 minutes) and leaves about 3.5 GB of incremental data in
`target/`. Over the 323 functions it came to 29 s per mutant, about 12 mutants
and 6 minutes per function. Keep to one heavy process at a time, as for any
build.

## Reading a survivor

Each survivor is one of three kinds. Examples from the tagsunk run (lines at
`main`):

- **Gap: the fixture never reaches the difference.** `tagsunk.rs:11304`,
  `lobby_man_send_race_report`: `(s0 < 0x3)` to `(s0 <= 0x3)` survived; the
  bound is `track - 14 < 3`, and the fixture's tracks were {0, 14, 15, 16}, so
  track 17 was never tried. A real porting mistake there would have passed too.
- **Equivalent: the change cannot alter the result.** `tagsunk.rs:11720`,
  `lobby_man_static_init`: `t6 | 0xff00` to `t6 ^ 0xff00`; t6 is
  `0xffff_ffff_ffff_0000` two lines up, so no bits are shared and `|` = `^`.
- **Unreachable: it needs a state the game cannot be in.**
  `tagsunk.rs:13719`, `c_setup_man_update`: `sp | 0xc` to `sp ^ 0xc` differ
  only when bit 2 or 3 of `sp` is set, and the EE stack is 16-byte aligned (the
  function's own `sq` saves would fault otherwise).

Of those 60: 33 gaps, 21 equivalent, 6 unreachable.

## Fixing a gap

Fix the test, not the port: extend the fixture so it reaches the line (here,
add track 17). Then run the exact mutant again; it must now be caught:

```sh
cargo mutants --in-place --package ssx3-game --file crates/ssx3-game/src/game/dirtysock/tagsunk.rs \
  -F 'tagsunk.rs:11304:42: replace < with <=' -- game::dirtysock::tagsunk::
```

Equivalent and unreachable survivors stay; say why in the PR (one line each).
