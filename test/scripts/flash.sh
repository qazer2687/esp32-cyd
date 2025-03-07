#!/usr/bin/env bash

set -e

BUILD_MODE=""
case "$1" in
"" | "release")
    bash scripts/build.sh
    BUILD_MODE="release"
    ;;
"debug")
    bash scripts/build.sh debug
    BUILD_MODE="debug"
    ;;
*)
    echo "Wrong argument. Only \"debug\"/\"release\" arguments are supported"
    exit 1
    ;;
esac

echo "Converting ELF to BIN..."

espflash save-image --chip esp32 target/xtensa-esp32-espidf/${BUILD_MODE}/test target/xtensa-esp32-espidf/${BUILD_MODE}/firmware.bin

#web-flash --chip esp32 target/xtensa-esp32-espidf/${BUILD_MODE}/test
