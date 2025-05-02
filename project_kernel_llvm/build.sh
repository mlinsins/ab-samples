#!/bin/bash
set -e

cd linux/

/bin/bash "$LOG_HOOK" TIMESTAMP BUILD_START "$(date -Ins)"

LLVM=1 make defconfig

/bin/bash "$LOG_HOOK" TIMESTAMP POST_CONFIGURE "$(date -Ins)"
LLVM=1 make -j16
/bin/bash "$LOG_HOOK" TIMESTAMP POST_MAKE "$(date -Ins)"
# not much testing we can do here
/bin/bash "$LOG_HOOK" TIMESTAMP POST_MAKE_CHECK "$(date -Ins)"
/bin/bash "$LOG_HOOK" TIMESTAMP BUILD_END "$(date -Ins)"

/bin/bash "$ATTESTATION_HOOK" "arch/x86/boot/bzImage"
