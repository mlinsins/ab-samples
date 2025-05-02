#!/bin/bash
set -e

tar -xf xz-5.6.3.tar.gz 
cd xz-5.6.3/

/bin/bash "$LOG_HOOK" TIMESTAMP BUILD_START "$(date -Ins)"
./configure 
/bin/bash "$LOG_HOOK" TIMESTAMP POST_CONFIGURE "$(date -Ins)"
make -j2
/bin/bash "$LOG_HOOK" TIMESTAMP POST_MAKE "$(date -Ins)"
# make check -j2
/bin/bash "$LOG_HOOK" TIMESTAMP POST_MAKE_CHECK "$(date -Ins)"
/bin/bash "$LOG_HOOK" TIMESTAMP BUILD_END "$(date -Ins)"

/bin/bash "$ATTESTATION_HOOK" "src/xz/.libs/xz"
