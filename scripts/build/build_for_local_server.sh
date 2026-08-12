set -e

cd $FLUSTER_IOS_ROOT/packages/rust/conundrum/
# pnpm build:rust
pnpm build:ts
pnpm build:scss
pnpm copy:katexcss
pnpm copy:scss_input


# cd $FLUSTER_IOS_ROOT
# WIREIT_PARALLEL=3 pnpm build:macStep2

cd $FLUSTER_IOS_ROOT/packages/rust/conundrum_ts/
pnpm copy:globalCSS

cd $FLUSTER_IOS_ROOT/packages/webview_utils
pnpm build:ts
pnpm build:tailwind
