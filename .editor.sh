#!/bin/sh
pynvim "+colo soda" "+AutoSaveToggle"  "+set mouse=a" src/main.rs 2>&1 1>/tmp/pynvim.$$.out &
