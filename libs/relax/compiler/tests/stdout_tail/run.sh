#!/bin/bash
set -eu
H=$(cd "$(dirname "$0")" && pwd)
cd "$H"
export RELAX_CACHE=off
RELAX_STD="$H/../../std" "$1" test stdout-tail
