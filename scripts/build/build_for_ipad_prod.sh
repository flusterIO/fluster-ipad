#!/usr/bin/env bash
# Production iPad build. Same order as build_for_ipad.sh, with FLUSTER_PROD_BUILD set
# and the release (--release) cargo-swift build:
#   1. Generate cross-language types from Rust (turbo docgen)
#   2. Conundrum embedded TS/CSS
#   3. Rust -> Swift (release, iOS)
#   4. Shared TS packages + wasm
#   5. iPad webviews
set -euo pipefail

export FLUSTER_IOS_ROOT="${FLUSTER_IOS_ROOT:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)}"
export FLUSTER_PROD_BUILD=true

# 1. Types first.
cd "$FLUSTER_IOS_ROOT"
turbo docgen --force

# 2. Conundrum web assets that the Rust crate embeds.
cd "$FLUSTER_IOS_ROOT/packages/rust/conundrum/"
pnpm build:tsProd
pnpm build:scss
pnpm copy:katexcss
pnpm copy:scss_input

# 3. Rust -> Swift (release).
cd "$FLUSTER_IOS_ROOT/packages/rust/conundrum_swift"
pnpm build:ipadSwiftProd

# 4. TS packages + wasm that depend on the generated types.
cd "$FLUSTER_IOS_ROOT"
WIREIT_PARALLEL=3 pnpm build:ipadStep2Prod

cd "$FLUSTER_IOS_ROOT/packages/typescript/codemirror-lang-bib"
pnpm build:scss
pnpm build:ts
pnpm build:scss

cd "$FLUSTER_IOS_ROOT/packages/webview_utils"
pnpm build:tsProd
pnpm build:tailwind

# 5. Webviews.
cd "$FLUSTER_IOS_ROOT"
WIREIT_PARALLEL=5 pnpm build:webviewsIpadProd
