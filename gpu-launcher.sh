#!/bin/bash
# Wrapper script to ensure CUDA libraries are found and logs are captured

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
LOG_FILE="${SCRIPT_DIR}/narsil.log"

# Set the required library paths for CUDA 13 on EndeavourOS/Arch
export LD_LIBRARY_PATH="/usr/lib:/opt/cuda/lib64:${LD_LIBRARY_PATH}"

echo "--- Server starting $(date) ---" >> "$LOG_FILE"

# Run the binary and redirect stderr (where logs go) to the log file
# We use 'tee' so it still goes to stderr if the client needs it,
# but many MCP clients ignore stderr or hide it.
exec "${SCRIPT_DIR}/target/release/narsil-mcp" "$@" 2>> "$LOG_FILE"
