#!/bin/bash
set -e

tar -xf hello_2.10.orig.tar.gz
cd hello-2.10/

/bin/bash "$LOG_HOOK" TIMESTAMP BUILD_START "$(date -Ins)"
./configure
/bin/bash "$LOG_HOOK" TIMESTAMP POST_CONFIGURE "$(date -Ins)"
make -j4
/bin/bash "$LOG_HOOK" TIMESTAMP POST_MAKE "$(date -Ins)"
/bin/bash "$LOG_HOOK" TIMESTAMP BUILD_END "$(date -Ins)"

/bin/bash "$ATTESTATION_HOOK" "hello"
