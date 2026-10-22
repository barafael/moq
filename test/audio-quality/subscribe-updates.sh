#!/usr/bin/env bash
# Manual subscription-send measurement using the production watch/publish pages.
set -euo pipefail

AQ_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
CLIENT="$AQ_DIR/clients/js"
# shellcheck source-path=SCRIPTDIR source=../lib/harness.sh
source "$AQ_DIR/../lib/harness.sh"
harness_begin subscribe-updates "just test subscribe-updates$(harness_argv "$@")"
(
    cd "$CLIENT"
    bun install --frozen-lockfile
    bunx vite build --outDir "$HARNESS_RUN/page" --emptyOutDir
)
harness_port page
harness_spawn viewer "$HARNESS_RUN/viewer.log" bun "$CLIENT/updates.ts" \
    --page "$HARNESS_RUN/page" --port "$HARNESS_PORT" --out "$HARNESS_RUN/updates.json" "$@"
status=0
harness_wait "$HARNESS_PID" || status=$?
cat "$HARNESS_RUN/viewer.log"
exit "$status"
