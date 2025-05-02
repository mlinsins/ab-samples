#!/bin/bash
set -e

tar -xf neovim-0.11.0.tar.gz
cd neovim-0.11.0

/bin/bash "$LOG_HOOK" TIMESTAMP BUILD_START "$(date -Ins)"
make deps
/bin/bash "$LOG_HOOK" TIMESTAMP POST_CONFIGURE "$(date -Ins)"
make CMAKE_BUILD_TYPE=RelWithDebInfo -j4
/bin/bash "$LOG_HOOK" TIMESTAMP POST_MAKE "$(date -Ins)"
/bin/bash "$LOG_HOOK" TIMESTAMP BUILD_END "$(date -Ins)"

/bin/bash "$ATTESTATION_HOOK" "build/bin/nvim"
