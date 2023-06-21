#!/bin/bash

if [ $# -eq 1 ] && [ $1 = "--release" ]
then
    cargo build --release
    cp ./target/release/libprojec_p2.so ./python/projec_p2.so
else
    cargo build
    cp ./target/debug/libprojec_p2.so ./python/projec_p2.so
fi
