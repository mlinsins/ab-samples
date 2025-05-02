#!/bin/bash
set -e

cd gprolog/src/

/bin/bash "$LOG_HOOK" TIMESTAMP BUILD_START "$(date -Ins)"
./configure --without-doc-dir --without-links-dir --with-install-dir=in-place
/bin/bash "$LOG_HOOK" TIMESTAMP POST_CONFIGURE "$(date -Ins)"
make -j4
/bin/bash "$LOG_HOOK" TIMESTAMP POST_MAKE "$(date -Ins)"
# make check -j4
/bin/bash "$LOG_HOOK" TIMESTAMP POST_MAKE_CHECK "$(date -Ins)"
make install
/bin/bash "$LOG_HOOK" TIMESTAMP BUILD_END "$(date -Ins)"

/bin/bash "$ATTESTATION_HOOK" "../bin/gprolog"
