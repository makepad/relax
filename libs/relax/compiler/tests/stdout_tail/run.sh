#!/bin/bash
set -eu
H=$(cd "$(dirname "$0")" && pwd)
cd "$H"
RELAX_STD="$H/../../std" "$1" test stdout-tail
