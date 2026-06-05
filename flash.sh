#!/bin/bash
# Flash a binary to FRDM-MCXA156 via J-Link
ELF="$1"
BIN=$(mktemp /tmp/flash_XXXXXX.bin)
arm-none-eabi-objcopy -O binary "$ELF" "$BIN"
SCRIPT=$(mktemp)
cat > "$SCRIPT" <<EOF
device MCXA156
si SWD
speed 1000
connect
r
h
loadbin $BIN 0x0
SetPC 0x800
g
qc
EOF
JLinkExe -NoGui 1 -CommanderScript "$SCRIPT"
rm -f "$BIN" "$SCRIPT"
