
cd $FLUSTER_IOS_ROOT/packages/rust/conundrum/
# pnpm build:rust
pnpm build:ts
pnpm build:scss
pnpm copy:katexcss
pnpm copy:scss_input

cd $FLUSTER_IOS_ROOT/packages/rust/conundrum_swift
pnpm build:macSwift
pnpm copy:extraSwift
# pnpm build:postCodegenCleanup

cd $FLUSTER_IOS_ROOT
WIREIT_PARALLEL=3 pnpm build:macStep2

cd $FLUSTER_IOS_ROOT/packages/rust/conundrum_ts/
pnpm copy:globalCSS


cd $FLUSTER_IOS_ROOT/packages/typescript/codemirror-lang-bib
pnpm build:scss
pnpm build:ts
pnpm build:scss

cd $FLUSTER_IOS_ROOT/packages/webview_utils
pnpm build:ts
pnpm build:tailwind


cd $FLUSTER_IOS_ROOT

WIREIT_PARALLEL=5 pnpm build:webviewsMac

