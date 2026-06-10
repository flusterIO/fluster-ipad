cd $FLUSTER_IOS_ROOT
turbo docgen --force

cd $FLUSTER_IOS_ROOT/packages/rust/conundrum/
FLUSTER_PROD_BUILD=true pnpm build:tsProd
pnpm build:scss
pnpm copy:katexcss
pnpm copy:scss_input

cd $FLUSTER_IOS_ROOT/packages/rust/conundrum_swift
pnpm build:macSwiftProd

cd $FLUSTER_IOS_ROOT
WIREIT_PARALLEL=3 pnpm build:macStep2

cd $FLUSTER_IOS_ROOT/packages/rust/conundrum_ts/
pnpm copy:globalCSS


cd $FLUSTER_IOS_ROOT/packages/typescript/codemirror-lang-bib
pnpm build:scss

FLUSTER_PROD_BUILD=true pnpm build:ts
pnpm build:scss

cd $FLUSTER_IOS_ROOT/packages/webview_utils
pnpm build:tsProd
pnpm build:tailwind


cd $FLUSTER_IOS_ROOT

WIREIT_PARALLEL=5 pnpm build:webviewsMacProd
