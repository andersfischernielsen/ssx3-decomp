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


def words(path):
    return open(path, errors="replace").read() if os.path.exists(path) else ""


class Run:
    def __init__(self, work, name, env, input_, frames, shots_from, every=100):
        self.dir = os.path.join(work, name)
        shutil.rmtree(self.dir, ignore_errors=True)
        os.makedirs(self.dir)
        e = dict(os.environ, SSX3_SHOTS=self.dir, SSX3_SHOT_FROM=str(shots_from),
                 SSX3_FILE_LOG=os.path.join(self.dir, "files.log"), **env)
        if input_:
            e["SSX3_INPUT"] = input_
        p = subprocess.run([HEADLESS, str(frames), "--shot-every", str(every)], cwd=ROOT, env=e,
                           capture_output=True, text=True)
        self.out = p.stdout + p.stderr
        open(os.path.join(self.dir, "out.log"), "w").write(self.out)
        self.files = words(os.path.join(self.dir, "files.log"))

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
    all_scenarios = ["plugin_event", "retail_with_plugin", "no_plugin", "back_and_forth",
                     "limits", "style_only", "event_boot_frames"]
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
