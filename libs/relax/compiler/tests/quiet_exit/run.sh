#!/bin/bash
set -eu
H=$(cd "$(dirname "$0")" && pwd)
cd "$H"
export RELAX_CACHE=off
RELAX_STD="$H/../../std" "$1" test quiet-exit
if OUT=$(RELAX_STD="$H/../../std" "$1" run --quiet quiet-exit 2>&1); then
    echo "quiet_exit: expected exit 2"
    exit 1
else
    rc=$?
fi
test "$rc" = 2
printf '%s\n' "$OUT" | grep -q '^relax: session log kept at '
