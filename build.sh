#!/bin/bash

prog_name=bfrs
win_target_codename=x86_64-pc-windows-gnu
linux_target_codename=x86_64-unknown-linux-gnu
TYPE=$1
ALL=$2

function build_win() {
    echo "[*] win :: building package... "

    cargo build --$TYPE && \
    mv target/release/$prog_name.exe . >/dev/null && \
    zip bfrs-$win_target_codename.zip $prog_name.exe >/dev/null
}

function build_linux() {
    echo "[*] linux :: building package..."

    cargo build --$TYPE --target $linux_target_codename && \
    mv target/$linux_target_codename/release/$prog_name . && \
    zip bfrs-$linux_target_codename.zip $prog_name >/dev/null
}

if [ $TYPE = release ]; then
    build_win # Build windows release artifacts
    
    if [ $ALL = true ]; then
        build_linux # Build linux release artifacts
    fi
else
    cargo build --$TYPE && mv target/$TYPE/bfrs.exe .
fi
