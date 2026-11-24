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
green (porting loop step 6). A wave's functions fit too now: the script builds
once for all of them (below), so their cost is mostly the tests.

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
mutants once with cargo-mutants, leaves out the `c.cycles +=` lines (the tests
compare cycles, so their `-=` and `*=` are caught, or equivalent when the count
happens to match; they are more than half of all mutants), and runs the rest
with only the touched units' tests. It prints per function the caught, missed,
unviable and timed-out mutants, then each survivor with its source line.
Everything is under `target/mutants.out/` (`missed.txt`, a log per mutant).
The first run rebuilds once for incremental (about 4 minutes) and leaves about
3.5 GB of incremental data in `target/`. Keep to one heavy process at a time,
as for any build.

### Build once

cargo-mutants rebuilds `ssx3-game` for every mutant (30-45 s on 8 cores; over
323 functions, 29 s per mutant and 6 minutes per function). The script
instead writes all the mutants into the source behind switches, builds once,
and runs the tests once per mutant with only its switch on
(`SSX3_MUTANT=<id>`):

```rust
// `a < b` with mutants 7 (`<=`) and 8 (`>`):
(if crate::game::bx::bxstring::__mutant(7) { a <= b }
 else if crate::game::bx::bxstring::__mutant(8) { a > b }
 else { a < b })
```

The switch goes around the smallest expression that holds the mutant (the
operator's binary expression, the `match` of a deleted arm), found by
`tools/mutant-spans` (a small `syn` program). Each branch is the original
text with one of cargo-mutants' replacements, from cargo-mutants' own list, so
with its switch on a mutant runs the code cargo-mutants would have built. The
build, tests and timeout are cargo-mutants' too (`.cargo/mutants.toml`; 5
times the unmutated run, at least 20 s).

A mutant that does not compile (`>>` to `<<` where `<<` parses as generic
arguments, `*` to `/` on a type without `Div`) fails the switched build;
rustc's errors name its lines, and the script builds again without it. Those
mutants, and the few a switch cannot hold (in a `const`, a pattern or a macro
call), are then built alone the cargo-mutants way: unviable if that fails,
tested otherwise. The files are restored and checked at the end, also after
an error or Ctrl-C.

On `bxstring.rs` (23 mutants) and four `libc.rs` functions (105, 4 of them
unviable) it agreed with `--cargo-mutants` on every mutant, in 89 s against
1,206 s and 1,448 s against 6,648 s (13 and 4.6 times faster, with other
builds on the machine). The build costs about as much as one or two
cargo-mutants mutants (plus one more, and one per mutant, when a mutant does
not compile); after that a mutant costs one run of the touched units' tests:
1.4 s for `bxstring`, 11 s for the 132 `libc` tests. At those rates a wave's
1,000 or so mutants would take from half an hour to three hours.

The one difference: build-once builds the lib test target only, where
cargo-mutants also builds the plain lib, so a mutant that compiles only under
`cfg(test)` would be tested here and unviable there. `--cargo-mutants` runs
cargo-mutants in place instead, one rebuild per mutant; use it to check a
surprising result.

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
