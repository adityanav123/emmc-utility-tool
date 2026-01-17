#!/usr/bin/env bash


# CROSS COMPILE TO ARM64
cross build --release --target aarch64-unknown-linux-musl || {
    echo "arm64 build failed"
}
