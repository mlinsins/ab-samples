#!/bin/bash
set -e

source ~/.cargo/env
/bin/bash "$LOG_HOOK" TIMESTAMP BUILD_START "$(date -Ins)"
cargo fetch
/bin/bash "$LOG_HOOK" TIMESTAMP POST_CONFIGURE "$(date -Ins)"
cargo build --release --offline -j4
/bin/bash "$LOG_HOOK" TIMESTAMP POST_MAKE "$(date -Ins)"
# cargo test -j4
/bin/bash "$LOG_HOOK" TIMESTAMP POST_MAKE_CHECK "$(date -Ins)"
/bin/bash "$LOG_HOOK" TIMESTAMP BUILD_END "$(date -Ins)"

/bin/bash "$ATTESTATION_HOOK" "target/release/project_rust_small"
