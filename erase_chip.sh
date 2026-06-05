#!/usr/bin/env bash

printf 'device MCXA156\nsi SWD\nspeed 1000\nconnect\nr\nh\nerase\nqc\n' | JLinkExe -NoGui 1
