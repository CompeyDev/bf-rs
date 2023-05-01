#!/bin/bash

prog_name=bfrs
win_target_codename=x86_64-pc-windows-gnu
linux_target_codename=x86_64-unknown-linux-gnu
TYPE=$1

function build_win() {
    echo "win :: building package... "

    cargo build --$TYPE >/dev/null && \
    mv target/release/$prog_name.exe . >/dev/null && \
    zip bfrs-$win_target_codename.zip $prog_name.exe
}

function build_linux() {
    echo "linux :: building package..."

    cargo build --$TYPE --target $linux_target_codename >/dev/null && \
    mv target/$linux_target_codename/release/$prog_name . >/dev/null && \
    zip bfrs-$linux_target_codename.zip $prog_name >/dev/null
}

if [ $TYPE = release ]; then
    build_win # Build windows release artifacts
    build_linux # Build linux release artifacts
else
    cargo build --$TYPE && mv target/$TYPE/bfrs.exe .
fi
