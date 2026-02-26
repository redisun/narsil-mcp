#!/bin/bash
# Wrapper script to ensure CUDA libraries are found and logs are captured.
# Adjust LD_LIBRARY_PATH if your CUDA installation is in a non-standard location.

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
LOG_FILE="${SCRIPT_DIR}/narsil.log"

# Add common CUDA library paths if they exist
if [ -d "/usr/local/cuda/lib64" ]; then
    export LD_LIBRARY_PATH="/usr/local/cuda/lib64:${LD_LIBRARY_PATH}"
fi
if [ -d "/opt/cuda/lib64" ]; then
    export LD_LIBRARY_PATH="/opt/cuda/lib64:${LD_LIBRARY_PATH}"
fi

echo "--- Server starting $(date) ---" >> "$LOG_FILE"

# Run the binary and redirect stderr (where logs go) to the log file.
# We use '2>>' to redirect stderr (logs) while keeping stdout for MCP.
exec "${SCRIPT_DIR}/target/release/narsil-mcp" "$@" 2>> "$LOG_FILE"
