#!/bin/bash
# Flash a binary to FRDM-MCXA156 via J-Link
BIN="$1"
SCRIPT=$(mktemp)
cat > "$SCRIPT" <<EOF
device MCXA156
si SWD
speed 1000
connect
r
h
loadfile $BIN
SetPC 0x800
g
qc
EOF
JLinkExe -NoGui 1 -CommanderScript "$SCRIPT"
rm -f "$SCRIPT"
