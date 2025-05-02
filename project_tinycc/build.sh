#!/bin/bash
set -e

/bin/bash "$LOG_HOOK" TIMESTAMP BUILD_START "$(date -Ins)"
./configure
/bin/bash "$LOG_HOOK" TIMESTAMP POST_CONFIGURE "$(date -Ins)"
make -j4
/bin/bash "$LOG_HOOK" TIMESTAMP POST_MAKE "$(date -Ins)"
# make test -j4
/bin/bash "$LOG_HOOK" TIMESTAMP POST_MAKE_CHECK "$(date -Ins)"
/bin/bash "$LOG_HOOK" TIMESTAMP BUILD_END "$(date -Ins)"

/bin/bash "$ATTESTATION_HOOK" "tcc"
