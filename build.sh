#!/bin/bash

prog_name=bfrs
win_target_codename=x86_64-pc-windows-gnu
linux_target_codename=x86_64-unknown-linux-gnu
TYPE=$1
ALL=$2

function build_win() {
    echo "[*] win :: building package... "

    cargo build --$TYPE --target $win_target_codename && \
    mv target/release/$prog_name.exe . && \
    zip bfrs-$win_target_codename.zip $prog_name.exe >/dev/null
}

function build_linux() {
    echo "[*] linux :: building package..."
	
    mv build.rs build.rs.bak >/dev/null 2>&1
    cargo build --$TYPE && \
    mv target/$linux_target_codename/release/$prog_name . && \
    zip bfrs-$linux_target_codename.zip $prog_name >/dev/null
    mv build.rs.bak build.rs >/dev/null 2>&1
}

if [ $TYPE = release ]; then
    if [ $ALL = true ]; then
        build_win # Build windows release artifacts
        build_linux # Build linux release artifacts
    elif [ $ALL = "win" ]; then
        build_win
    elif [ $ALL = "linux" ]; then
        build_linux
    fi
else
    cargo build --$TYPE && mv target/$TYPE/bfrs.exe .
fi
