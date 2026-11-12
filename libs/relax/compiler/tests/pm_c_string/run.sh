#!/bin/bash
# usage: run.sh <makepad binary>: `proc_macro::Literal::c_string` through the JIT proc-macro
# host; prints "pm_c_string: ok" or "pm_c_string: FAILED" (exit 1).
H=$(cd "$(dirname "$0")" && pwd)
export MAKEPAD_RUN_CACHE=off MAKEPAD_RUN_STD="$H/../../std"
ERR=$(mktemp)
OUT=$(cd "$H" && "$1" run --quiet pm-c-string-app 2>"$ERR")
if [ "$OUT" = "hello" ]; then rm -f "$ERR"; echo "pm_c_string: ok"; else tail -5 "$ERR"; rm -f "$ERR"; echo "pm_c_string: FAILED"; exit 1; fi
