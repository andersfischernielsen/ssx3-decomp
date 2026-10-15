---
name: opencode-delegate
description: Delegate well-scoped mechanical tasks (renames, boilerplate, docs, fixtures, read-only exploration) to cheap open models through OpenCode workers via scripts/delegate.sh, each write task in its own git worktree; review and verify every result before merging. Use when there are several independent low-judgment tasks that can run in parallel.
---

# Delegating to OpenCode workers

OpenCode is a full coding agent (reads and edits files, runs shell). Each
`opencode run` is one worker. Workers on cheap open models are **fast and
cheap but weaker than you**: worse at reasoning, design and debugging, more
likely to misread intent or make subtle mistakes. Treat them as a quick
junior pair of hands: precise instructions in, verified diffs out.

Known-good invocation (OpenCode v2.0.28 here; check `opencode run --help`
before using any other flag, the docs online describe older versions):

```sh
opencode run --model "ollama/deepseek-v4.1-flash:cloud" "<prompt>"
# flags that exist: --model/-m, --format json, --auto, --title, --file/-f,
# --standalone, --session/-s, --continue/-c, --agent, --thinking
# there is no working-directory flag: run it from the target directory
```

Use `scripts/delegate.sh`, which does the worktree/log bookkeeping. It checks
that the model is listed by `opencode models` and otherwise falls back to
`ollama-cloud/deepseek-v4.1-flash` (on 2026-10-15 the `ollama/` provider
vanished from OpenCode while Ollama itself still served the model, and
workers hung with "Model unavailable"), and it kills a worker after
`DELEGATE_TIMEOUT` seconds (default 1200). If a log ends without output, run
`opencode run --model <id> "Reply with: pong"` to check the provider.

## 1. What to delegate

Delegate only well-scoped, low-judgment work:
- renames and mechanical refactors with an exact before/after;
- boilerplate following an existing example in the repo;
- docs: formatting, tables, filling a template from given facts;
- test fixtures and data files;
- read-only exploration: "list every X", "find where Y is used", summaries.

Keep on Claude: design decisions, debugging, anything needing judgment about
PS2/emulation semantics or byte-identical behaviour, reviewing diffs, and
merging. In this repo specifically, never delegate changes to generated code
semantics, float/cycle-exact replacements in `crates/ssx3-game`, or anything
whose correctness is only checkable by a replay, unless you verify it with
the replay yourself.

## 2. Isolation

Every run passes `--auto` (a non-interactive worker can't answer permission
prompts; permissions explicitly denied in the OpenCode config stay denied).

- **Write tasks**: `scripts/delegate.sh --write <task> <prompt>` creates
  `.claude/worktrees/delegate-<task>` on branch `delegate/<task>` from HEAD
  and runs OpenCode there. One task, one worktree. **Commit everything the
  worker needs first**: the worktree is cut from HEAD, so uncommitted or
  untracked files are missing there (the first test run hit exactly this).
- **Write tasks that need gitignored build inputs** (in this repo: anything
  compiling against `generated/ee`, e.g. `crates/ssx3-game` work):
  `scripts/delegate.sh --here <task> <prompt>` runs in the main tree. Give each
  worker its own file (e.g. a feature-gated module) so parallel workers never
  touch the same file, and review with `git diff -- <file>`. Example:
  `docs/investigation/phase2-worker-guide.md` (Phase 2 rewrites).
- **Read-only tasks**: `scripts/delegate.sh <task> <prompt>` runs in the main
  tree. Tell the worker "do not modify any files", and check `git status`
  afterwards.
- Note: worktrees lack gitignored files (`generated/ee`, `generated/iop`,
  `orig/`, `saves/`, the ISO). Crates that don't depend on generated code
  (`ssx3-ee-rt`, `ssx3-vu`, `ssx3-ps2`, `ssx3-iop`, `ssx3-core`,
  `ssx3-recomp`) can be checked in a fresh worktree; anything depending on
  `generated/ee` (ssx3, ssx3-game) needs those copied in and a long build, so
  prefer delegating tasks that stay out of those crates.

## 3. Prompts

Every prompt must be self-contained (the worker knows nothing about this
conversation):
- exact files to read and to change (paths);
- the exact change, with an example of the desired result when possible;
- done-criteria: what must be true when finished (e.g. "`cargo check -p
  ssx3-vu` passes", "the table has one row per function in X");
- "Do not touch any other files. Do not refactor, reformat or 'improve'
  anything else. Do not commit." (You commit after review.)
- for read-only tasks: "Do not modify any files; print the answer."

Long prompts: write them to a file and pass `@path/to/prompt.txt`.

## 4. Fan out, then wait

Run independent tasks in parallel as background processes, then wait for all:

```sh
scripts/delegate.sh --write rename-a @/tmp/p-a.txt > /tmp/a.out 2>&1 &
scripts/delegate.sh --write rename-b @/tmp/p-b.txt > /tmp/b.out 2>&1 &
wait
```

(With the Bash tool: one call with `run_in_background`, or several.) Mind the
machine: workers that run cargo builds compete for CPU; keep builds in
parallel workers to `cargo check -p <one crate>` and cap the number of
concurrent build-running workers at ~2. Logs: `.delegate/logs/<task>.log`.

## 5. Accepting a result

Never trust the worker's own summary. For each write task:
1. `git -C .claude/worktrees/delegate-<task> status` and `git diff` — read the
   whole diff; check it touches only the intended files and does exactly the
   asked change.
2. Run the verification yourself in that worktree (`cargo check -p <crate>`,
   `cargo test -p <crate>`, or the relevant script).
3. Merge (commit in the worktree, then `git merge` / cherry-pick into the
   working branch) or discard. Fixing small mistakes yourself is fine; if the
   result is wrong in substance, discard and either re-prompt more precisely
   or do it yourself.

For read-only tasks: spot-check the claims against the code before relying
on them.

## 6. Clean up

`scripts/delegate.sh --cleanup <task>` removes the worktree and its branch.
Do it for every task once merged or discarded. Delete logs you no longer need
(`.delegate/logs/`).
