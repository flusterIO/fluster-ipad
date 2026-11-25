#!/usr/bin/env bash
# Builds everything the iPad app needs, in dependency order:
#   1. Generate cross-language types from the Rust source (typeshare + cleanup via `turbo docgen`)
#   2. Conundrum embedded TS/CSS (consumed by the Rust crate)
#   3. Rust -> Swift (cargo swift, iOS targets)
#   4. Shared TS packages + wasm, which import the generated types
#   5. iPad webviews
set -euo pipefail

export FLUSTER_IOS_ROOT="${FLUSTER_IOS_ROOT:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)}"

# 1. Generate types first so every later step sees the latest Rust types.
cd "$FLUSTER_IOS_ROOT"
turbo docgen --force

# 2. Conundrum web assets that the Rust crate embeds.
cd "$FLUSTER_IOS_ROOT/packages/rust/conundrum/"
pnpm build:ts
pnpm build:scss
pnpm copy:katexcss
pnpm copy:scss_input

# 3. Rust -> Swift (this compiles the conundrum crate for iOS).
cd "$FLUSTER_IOS_ROOT/packages/rust/conundrum_swift"
pnpm build:ipadSwift

# 4. TS packages + wasm that depend on the generated types.
cd "$FLUSTER_IOS_ROOT"
WIREIT_PARALLEL=3 pnpm build:ipadStep2

cd "$FLUSTER_IOS_ROOT/packages/typescript/codemirror-lang-bib"
pnpm build:scss
pnpm build:ts
pnpm build:scss

cd "$FLUSTER_IOS_ROOT/packages/webview_utils"
pnpm build:ts
pnpm build:tailwind

# 5. Webviews.
cd "$FLUSTER_IOS_ROOT"
WIREIT_PARALLEL=5 pnpm build:webviewsIpad
