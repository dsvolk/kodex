#!/bin/bash

# Set "chatgpt.cliExecutable": "/Users/<USERNAME>/code/kodex/scripts/debug-kodex.sh" in VSCode settings to always get the 
# latest kodex-rs binary when debugging Kodex Extension.


set -euo pipefail

KODEX_RS_DIR=$(realpath "$(dirname "$0")/../kodex-rs")
(cd "$KODEX_RS_DIR" && cargo run --quiet --bin kodex -- "$@")