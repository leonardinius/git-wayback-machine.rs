#!/bin/sh
pynvim "+colo soda" "+AutoSaveToggle" "+set spell" "+set mouse=a" src/main.rs 2>&1 1>/tmp/pynvim.$$.out &
