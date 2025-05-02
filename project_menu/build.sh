#!/bin/bash
set -e

cd menu/

/bin/bash "$LOG_HOOK" TIMESTAMP BUILD_START "$(date -Ins)"
autoreconf --install && ./configure
/bin/bash "$LOG_HOOK" TIMESTAMP POST_CONFIGURE "$(date -Ins)"
make -j4
/bin/bash "$LOG_HOOK" TIMESTAMP POST_MAKE "$(date -Ins)"
/bin/bash "$LOG_HOOK" TIMESTAMP BUILD_END "$(date -Ins)"

/bin/bash "$ATTESTATION_HOOK" "install-menu/install-menu"
