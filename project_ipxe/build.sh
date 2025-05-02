#!/bin/bash
set -e

tar -xf ipxe-1.21.1.tar.gz
cd ipxe-1.21.1/src/

/bin/bash "$LOG_HOOK" TIMESTAMP BUILD_START "$(date -Ins)"
/bin/bash "$LOG_HOOK" TIMESTAMP POST_CONFIGURE "$(date -Ins)"
NO_WERROR=1 make -j4 
/bin/bash "$LOG_HOOK" TIMESTAMP POST_MAKE "$(date -Ins)"
/bin/bash "$LOG_HOOK" TIMESTAMP BUILD_END "$(date -Ins)"

/bin/bash "$ATTESTATION_HOOK" "bin/ipxe.dsk"
