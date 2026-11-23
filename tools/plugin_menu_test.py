#!/usr/bin/env python3
"""Scripted headless test of the plugin peaks and events on the Select Peak
screen (docs/plans/plugin-peaks.md, docs/plugins.md "Peaks and events").

    tools/plugin_menu_test.py [scenario ...]     # default: all
    tools/plugin_menu_test.py list

Needs `cargo build --profile game -p ssx3 --bins` and the SSX 3 disc image
(`SSX 3 (USA).iso`) and `saves/` in the repository, like the other headless
runs. It copies `examples/plugins/menu-test` and `world-copy` to a scratch
folder (nothing goes into `plugins/`), has the importer make its content from
your disc into the cache folder, and drives the menus with scripted pad input
(`SSX3_INPUT`, libpad bits). Each scenario checks the log and the file reads;
the screenshots stay in the scratch folder for a look.

The career scenarios (`career_*`) check that a plugin event leaves the
player's save alone (docs/plugins.md, "Career and saves"): they run on a copy of
`saves/ssx3-mc0.ps2` (the tree's card is never written) and compare the RAM
the game's save blocks are built from (`SAVE_REGIONS`) before and after a race.

Frames are compared only where the path is the same with and without the
plugin (`SSX3_EVENT`): through the menus the plugin's longer screen shifts
the load timing, so the race starts a few frames apart.
"""
import hashlib
import os
import shutil
import subprocess
import sys
import tempfile

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
HEADLESS = os.path.join(ROOT, "target/game/ssx3-headless")
SSX3 = os.path.join(ROOT, "target/game/ssx3")

START, X, TRI = 0x8, 0x4000, 0x1000
UP, DOWN = 0x10, 0x40


def press(ev, frame, buttons, n=8):
    ev.append(f"{frame}:{buttons:04x}:{n}")


def to_select_peak():
    """Title, Single Event, character, Continue: the Select Peak screen at ~2850."""
    ev = []
    press(ev, 1600, START)
    press(ev, 2000, X)
    press(ev, 2500, X)
    press(ev, 2700, X)
    return ev


def plugin_event():
    # Down to the first plugin peak, peak, Race, first event, rules, load, card.
    ev = to_select_peak()
    press(ev, 2900, DOWN, 6)
    for f in (3050, 3200, 3350, 3550, 3750, 3950):
        press(ev, f, X)
    return ",".join(ev), 4300


def retail_snow_jam():
    ev = to_select_peak()
    for f in (2850, 3000, 3150, 3350, 3550, 3750, 3950):
        press(ev, f, X)
    return ",".join(ev), 4300


def back_and_forth():
    # A plugin peak, its goals, back; Up to Peak 1, its goals and events.
    ev = to_select_peak()
    press(ev, 2900, DOWN, 6)
    press(ev, 3050, X)
    press(ev, 3200, TRI)
    press(ev, 3350, UP, 6)
    press(ev, 3450, X)
    press(ev, 3600, X)
    return ",".join(ev), 3800


def limits():
    # The fourth plugin peak (10 of its 13 events fit), down its list.
    ev = to_select_peak()
    for i in range(4):
        press(ev, 2900 + 50 * i, DOWN, 6)
    for f in (3200, 3350):
        press(ev, f, X)
    for f in (3500, 3550, 3600):
        press(ev, f, DOWN, 6)
    return ",".join(ev), 3800


def style_only():
    # A peak with freestyle events only: its goal list has the one row.
    ev = to_select_peak()
    for i in range(3):
        press(ev, 2900 + 50 * i, DOWN, 6)
    for f in (3200, 3350):
        press(ev, f, X)
    return ",".join(ev), 3600


def next_event(plugin):
    """Run the first event to the end (about frame 19500), results: Up to
    "Next event", X: the in-game Select Peak (OV.LUI). With `plugin`: Down to
    the first plugin peak, peak, Race, first event; else Peak 1 and its first
    event (the game's own)."""
    ev = plugin_event()[0] if plugin else retail_snow_jam()[0]
    press(ev := ev.split(","), 20400, UP, 4)
    press(ev, 20700, X)
    if plugin:
        press(ev, 21000, DOWN, 6)
    for f in (21200, 21400, 21600) if plugin else (21200, 21400, 21600, 21800):
        press(ev, f, X)
    return ",".join(ev), 23800


# The RAM the game's save blocks are made of (BESaveInt_BuildSystemSave 0x152758 and
# BuildProfileSave 0x152950, beintsavefile.cpp): the system header, the score
# tables, the player flags and the two profiles. What a save would write to the
# memory card is a function of these bytes (plus a CRC).
SAVE_REGIONS = [("system header", 0x535898, 0x288), ("score tables", 0x535C18, 0xA28),
                ("player flags", 0x535B20, 0xA8), ("profiles", 0x4A6CA8, 2 * 0x9B50)]


def save_bytes(ram):
    return {name: ram[a:a + n] for name, a, n in SAVE_REGIONS}


def changed(a, b):
    """The names of the save regions that differ between two RAM images."""
    x, y = save_bytes(a), save_bytes(b)
    return [name for name in x if x[name] != y[name]]


def words(path):
    return open(path, errors="replace").read() if os.path.exists(path) else ""


class Run:
    def __init__(self, work, name, env, input_, frames, shots_from, every=100, dump=None):
        self.dir = os.path.join(work, name)
        shutil.rmtree(self.dir, ignore_errors=True)
        os.makedirs(self.dir)
        e = dict(os.environ, SSX3_SHOTS=self.dir, SSX3_SHOT_FROM=str(shots_from),
                 SSX3_FILE_LOG=os.path.join(self.dir, "files.log"), **env)
        if input_:
            e["SSX3_INPUT"] = input_
        self.ram = None
        if dump:
            self.ram_path = os.path.join(self.dir, "ram.bin")
            e["SSX3_RAM_DUMP"] = self.ram_path
        p = subprocess.run([HEADLESS, str(frames), "--shot-every", str(every)], cwd=ROOT, env=e,
                           capture_output=True, text=True)
        self.out = p.stdout + p.stderr
        open(os.path.join(self.dir, "out.log"), "w").write(self.out)
        self.files = words(os.path.join(self.dir, "files.log"))
        if dump:
            self.ram = open(self.ram_path, "rb").read()
            os.remove(self.ram_path)

    def shot(self, frame):
        return os.path.join(self.dir, f"frame{frame:05d}.png")


FAILED = []


def check(ok, what):
    print(("  ok   " if ok else "  FAIL ") + what)
    if not ok:
        FAILED.append(what)


def halted(run):
    return "halted: None" not in run.out


def main():
    names = sys.argv[1:]
    all_scenarios = ["career_plugin", "career_optout", "career_retail", "career_next_event",
                     "plugin_event", "retail_with_plugin", "no_plugin", "back_and_forth",
                     "limits", "style_only", "event_boot_frames", "next_event_plugin", "next_event_retail",
                     "next_event_no_plugin"]
    if names == ["list"]:
        print("\n".join(all_scenarios))
        return 0
    names = names or all_scenarios
    for need in (HEADLESS, SSX3):
        if not os.path.exists(need):
            sys.exit(f"{need}: build first (cargo build --profile game -p ssx3 --bins)")
    work = tempfile.mkdtemp(prefix="ssx3-plugin-menu-")
    plugins = os.path.join(work, "plugins")
    os.makedirs(plugins)
    for p in ("world-copy", "menu-test"):
        shutil.copytree(os.path.join(ROOT, "examples/plugins", p), os.path.join(plugins, p))
    env = {"SSX3_PLUGIN_DIR": plugins, "SSX3_PLUGINS": "menu-test"}
    prep = subprocess.run([SSX3, "plugins", "prepare", "menu-test"], cwd=ROOT,
                          env=dict(os.environ, **env), capture_output=True, text=True)
    if prep.returncode != 0:
        sys.exit(f"prepare failed:\n{prep.stdout}{prep.stderr}")
    print(f"scratch folder: {work}")

    career_names = [n for n in names if n.startswith("career_")]
    if career_names:
        real = os.path.join(ROOT, "saves/ssx3-mc0.ps2")
        if not os.path.exists(real):
            print("career scenarios skipped: no saves/ssx3-mc0.ps2")
        else:
            real_md5 = md5(real)
            # The card the runs use is a copy: the tree's card is never touched.
            card = os.path.join(work, "card.ps2")
            shutil.copy(real, card)
            card_md5 = md5(card)
            cenv = dict(env, SSX3_MEMCARD=card)
            optout = os.path.join(work, "plugins-optout")
            shutil.copytree(plugins, optout)
            mf = os.path.join(optout, "menu-test/plugin.toml")
            text = open(mf).read()
            marker = 'template = "ARA1"\nworld = "BAX"\n'
            assert marker in text
            open(mf, "w").write(text.replace(marker, marker + "career = true\n", 1))
            prep = subprocess.run([SSX3, "plugins", "prepare", "menu-test"], cwd=ROOT,
                                  env=dict(os.environ, **dict(env, SSX3_PLUGIN_DIR=optout)),
                                  capture_output=True, text=True)
            if prep.returncode != 0:
                sys.exit(f"prepare (opt-out copy) failed:\n{prep.stdout}{prep.stderr}")

            def career_pair(name, run_env, event):
                """RAM at the start of the race and after it (frame 17000, the player
                has finished): which save regions changed, and the career-guard log."""
                run_env = dict(run_env, SSX3_EVENT=event)
                a = Run(work, name + "-start", run_env, "", 1500, 99999, dump=True)
                b = Run(work, name + "-end", run_env, "", 17000, 99999, dump=True)
                return a, b, changed(a.ram, b.ram)

            def lines(run, what):
                return [l for l in run.out.splitlines() if "[career-guard]" in l and what in l]

            if "career_plugin" in career_names:
                print("career_plugin: a plugin event's race (SSX3_EVENT=menu-test/copy): the save regions stay as they were")
                a, b, diff = career_pair("career_plugin", cenv, "menu-test/copy")
                check(not halted(b), "the game does not halt")
                check(diff == [], f"no save region changed (changed: {diff})")
                check(len(lines(b, "race result of rider 0 kept out")) == 1 and len(lines(b, "placement of rider 0 kept out")) == 1,
                      "the player's result and placement were kept out of the career")
                check(not lines(b, "recorded"), "nothing was recorded")
                check(md5(card) == card_md5, "the memory card file is unchanged")
            if "career_optout" in career_names:
                print("career_optout: the same event with `career = true`: the game records it as it would for its template")
                oenv = dict(cenv, SSX3_PLUGIN_DIR=optout)
                a, b, diff = career_pair("career_optout", oenv, "menu-test/copy")
                check(not halted(b), "the game does not halt")
                check(len(lines(b, "race result of rider 0 recorded")) == 1 and len(lines(b, "placement of rider 0 recorded")) == 1,
                      "the player's result and placement were recorded (not kept out)")
                check(not lines(b, "kept out"), "nothing was kept out")
            if "career_retail" in career_names:
                print("career_retail: the game's own ARA1 with the plugin installed: recorded as ever")
                a, b, diff = career_pair("career_retail", cenv, "ARA1")
                check(not halted(b), "the game does not halt")
                check(len(lines(b, "race result of rider 0 recorded")) == 1 and len(lines(b, "placement of rider 0 recorded")) == 1,
                      "the player's result and placement were recorded")
                check(not lines(b, "kept out"), "nothing was kept out")
                print(f"  (save regions changed by this race: {diff or 'none: a last place records no best time or medal'})")
            if "career_next_event" in career_names:
                print("career_next_event: a plugin race, results, Next event, the game's own event: the second race is recorded")
                ev = plugin_event()[0].split(",")
                press(ev, 20400, UP, 4)
                press(ev, 20700, X)
                for f in (21200, 21400, 21600, 21800, 24600, 24900, 25200):  # last three: the race card
                    press(ev, f, X)
                i = ",".join(ev)
                r = Run(work, "career_next_event", cenv, i, 46000, 99999, every=1000)
                check(not halted(r), "the game does not halt")
                kept = lines(r, "placement of rider 0 kept out")
                recorded = lines(r, "placement of rider 0 recorded")
                check(len(kept) == 1, "the plugin race's placement was kept out")
                check(len(recorded) == 1, "the game's own race after it was recorded")
                check("data/worlds/BAM.big" in r.files, "the second race loaded the game's own world")
            check(md5(real) == real_md5, "saves/ssx3-mc0.ps2 in the tree is unchanged")
            print(f"  card copy: {card}")

    if "plugin_event" in names:
        print("plugin_event: Select Peak -> Peak Spike -> Race -> Copy of Snow Jam -> race")
        i, n = plugin_event()
        r = Run(work, "plugin_event", env, i, n, 3400)
        check(not halted(r), "the game does not halt")
        check("[plugin-menu] peak menu-test/spike (Peak Spike): 2 race, 1 freestyle events" in r.out,
              "the plan lists the peak and its events")
        check("[plugin-menu] start menu-test/copy (template ARA1, world BAX)" in r.out, "the event starts")
        check("data/worlds/BAX.big" in r.files and "data/worlds/BAM.big" not in r.files,
              "the race loads the plugin's world (BAX), not BAM")
        check("DATA/UI/FE.LUI generated" in r.out and "DATA/UI/MAPGFX.BIG generated" in r.out,
              "FE.LUI and MAPGFX.BIG are generated in memory")
    if "retail_with_plugin" in names:
        print("retail_with_plugin: the game's own Snow Jam, plugin installed")
        i, n = retail_snow_jam()
        r = Run(work, "retail_with_plugin", env, i, n, 4300)
        check(not halted(r), "the game does not halt")
        check("[plugin-menu] start" not in r.out, "no plugin event starts")
        check("data/worlds/BAM.big" in r.files and "data/worlds/BAX.big" not in r.files,
              "the race loads BAM")
    if "no_plugin" in names:
        print("no_plugin: the game's own Snow Jam, plugins off")
        i, n = retail_snow_jam()
        r = Run(work, "no_plugin", {"SSX3_PLUGINS": "0", "SSX3_PLUGIN_DIR": plugins}, i, n, 4300)
        check(not halted(r), "the game does not halt")
        check("[plugin-menu]" not in r.out and "generated" not in r.out, "nothing of the plugin menu runs")
        check("data/worlds/BAM.big" in r.files, "the race loads BAM")
    if "back_and_forth" in names:
        print("back_and_forth: plugin peak, back, Peak 1 goals and events")
        i, n = back_and_forth()
        r = Run(work, "back_and_forth", env, i, n, 2900)
        check(not halted(r), "the game does not halt")
        check("[plugin-menu] start" not in r.out, "nothing starts")
        a, b = r.shot(3300), r.shot(3400)
        check(os.path.exists(a) and os.path.exists(b) and md5(a) != md5(b),
              "the game's own peak picture is back after a plugin peak")
    if "limits" in names:
        print("limits: the fourth plugin peak, a list of 10")
        i, n = limits()
        r = Run(work, "limits", env, i, n, 3400)
        check(not halted(r), "the game does not halt")
        check("menu-test/tall-11 left out" in r.out and "menu-test/tall-13 left out" in r.out,
              "events past the list size are left out, with a log line")
    if "style_only" in names:
        print("style_only: a peak with freestyle events only")
        i, n = style_only()
        r = Run(work, "style_only", env, i, n, 3400)
        check(not halted(r), "the game does not halt")
    if "event_boot_frames" in names:
        print("event_boot_frames: SSX3_EVENT=ARA1 with and without the plugin: the same frame")
        a = Run(work, "event_boot_plugin", dict(env, SSX3_EVENT="ARA1"), "", 1300, 1300, 1300)
        b = Run(work, "event_boot_none", {"SSX3_PLUGINS": "0", "SSX3_EVENT": "ARA1"}, "", 1300, 1300, 1300)
        check(os.path.exists(a.shot(1300)) and os.path.exists(b.shot(1300)) and md5(a.shot(1300)) == md5(b.shot(1300)),
              "race frame 1300 is identical")
    if "next_event_plugin" in names:
        print("next_event_plugin: race, results, Next event: the in-game Select Peak lists the plugin peak")
        i, n = next_event(True)
        r = Run(work, "next_event_plugin", env, i, n, 20800, every=200)
        check(not halted(r), "the game does not halt")
        check("DATA/UI/OV.LUI generated" in r.out, "OV.LUI is generated in memory")
        check(r.out.count("[plugin-menu] start menu-test/copy") == 2,
              "the plugin event starts a second time from the in-game screen")
        check(r.files.count("data/worlds/BAX.big") >= 2 and "data/worlds/BAM.big" not in r.files,
              "the second race loads the plugin's world (BAX)")
        print(f"  look at {r.shot(21000)} (rows) and {r.shot(21400)} (events)")
    if "next_event_retail" in names:
        print("next_event_retail: race, results, Next event, the game's own Peak 1 event (plugin installed)")
        i, n = next_event(False)
        r = Run(work, "next_event_retail", env, i, n, 20800, every=200)
        check(not halted(r), "the game does not halt")
        check("DATA/UI/OV.LUI generated" in r.out, "OV.LUI is generated in memory")
        check("[plugin-menu] start" not in r.out, "no plugin event starts")
        check("data/worlds/BAM.big" in r.files and "data/worlds/BAX.big" not in r.files,
              "the game's own world is loaded again")
    if "next_event_no_plugin" in names:
        print("next_event_no_plugin: the same screen with plugins off")
        i, n = next_event(False)
        r = Run(work, "next_event_no_plugin", {"SSX3_PLUGINS": "0", "SSX3_PLUGIN_DIR": plugins}, i, n, 20800, every=200)
        check(not halted(r), "the game does not halt")
        check("generated" not in r.out, "nothing is generated")
        check("data/worlds/BAM.big" in r.files, "the game's own world is loaded")
    print(f"screenshots and logs: {work}")
    if FAILED:
        print(f"{len(FAILED)} check(s) failed")
        return 1
    print("all checks passed")
    return 0


def md5(path):
    return hashlib.md5(open(path, "rb").read()).hexdigest()


if __name__ == "__main__":
    sys.exit(main())
