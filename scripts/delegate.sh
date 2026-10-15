#!/usr/bin/env bash
# Delegate a mechanical task to a cheap model via OpenCode (one worker per call).
#
#   scripts/delegate.sh [--write | --here] [--model M] [--json] <task-name> <prompt | @prompt-file>
#   scripts/delegate.sh --cleanup <task-name>      # remove worktree + branch
#
# Every run uses --auto (permissions not explicitly denied are approved;
#   a non-interactive run can't answer prompts).
# Read-only (default): runs in the main tree; the prompt must say "do not
#   modify any files" (check `git status` afterwards).
# --write: creates .claude/worktrees/delegate-<task> on branch delegate/<task>
#   (from the current HEAD: commit what the worker needs first) and runs there.
# --here: a write task in the main tree, for work that needs the main tree's
#   gitignored build inputs (generated/ee): the prompt must confine the worker
#   to its own file(s); review with `git diff -- <file>`.
# Output goes to .delegate/logs/<task>.log (gitignored); the last line of this
# script's stdout is the log path. Review the worktree's diff yourself.
set -euo pipefail

root="$(git rev-parse --show-toplevel)"
model="${DELEGATE_MODEL:-ollama/deepseek-v4.1-flash:cloud}"
fallback_model="ollama-cloud/deepseek-v4.1-flash"
timeout="${DELEGATE_TIMEOUT:-1200}"   # seconds per worker
write=0 json=0

if [[ "${1:-}" == "--cleanup" ]]; then
  name="${2:?task name}"
  wt="$root/.claude/worktrees/delegate-$name"
  [[ -d "$wt" ]] && git -C "$root" worktree remove --force "$wt"
  git -C "$root" branch -D "delegate/$name" 2>/dev/null || true
  echo "cleaned up $name"
  exit 0
fi

while [[ $# -gt 0 ]]; do
  case "$1" in
    --write) write=1; shift ;;
    --here) write=2; shift ;;
    --json) json=1; shift ;;
    --model) model="$2"; shift 2 ;;
    *) break ;;
  esac
done
name="${1:?task name}"; prompt="${2:?prompt or @file}"
[[ "$name" =~ ^[A-Za-z0-9._-]+$ ]] || { echo "task name must be [A-Za-z0-9._-]+" >&2; exit 2; }
if [[ "$prompt" == @* ]]; then prompt="$(cat "${prompt#@}")"; fi

# The provider list changes (e.g. the local `ollama/` provider disappearing
# while `ollama-cloud/` stays): use the requested model only if OpenCode lists
# it, else the fallback.
if ! opencode models 2>/dev/null | grep -qxF "$model"; then
  echo "delegate: model $model not listed by OpenCode, using $fallback_model" >&2
  model="$fallback_model"
fi

logdir="$root/.delegate/logs"; mkdir -p "$logdir"
log="$logdir/$name.log"

dir="$root"; flags=(--auto)
if [[ $write == 1 ]]; then
  dir="$root/.claude/worktrees/delegate-$name"
  if [[ ! -d "$dir" ]]; then
    git -C "$root" worktree add -q -b "delegate/$name" "$dir" HEAD
  fi
fi
[[ $json == 1 ]] && flags+=(--format json)

{
  echo "# task: $name"
  echo "# model: $model"
  echo "# dir: $dir"
  echo "# mode: $(case $write in 1) echo write-worktree;; 2) echo write-here;; *) echo read-only;; esac)"
  echo "# started: $(date '+%Y-%m-%d %H:%M:%S')"
  echo "# --- prompt ---"
  echo "$prompt"
  echo "# --- output ---"
} > "$log"

rc=0
# Hard timeout: a worker waiting on an unavailable model would otherwise hang.
(cd "$dir" && perl -e 'alarm shift; exec @ARGV' "$timeout" \
  opencode run --model "$model" --title "delegate:$name" ${flags[@]+"${flags[@]}"} "$prompt") >> "$log" 2>&1 || rc=$?
[[ $rc == 142 ]] && echo "# TIMEOUT after ${timeout}s" >> "$log"
echo "# exit: $rc  finished: $(date '+%Y-%m-%d %H:%M:%S')" >> "$log"
echo "$log"
exit $rc
