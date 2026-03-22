
trash $FLUSTER_IOS_ROOT/packages/webviews/standalone_mdx_preview/public/mathjax
trash $FLUSTER_IOS_ROOT/packages/webviews/splitview_mdx_editor/public/mathjax
trash $FLUSTER_IOS_ROOT/packages/webviews/note_detail_webview/public/mathjax
trash $FLUSTER_IOS_ROOT/packages/webviews/dictionary_webview/public/mathjax

mkdir -p $FLUSTER_IOS_ROOT/packages/webviews/standalone_mdx_preview/public/mathjax/output/chtml/fonts/woff-v2/
mkdir -p $FLUSTER_IOS_ROOT/packages/webviews/splitview_mdx_editor/public/mathjax/output/chtml/fonts/woff-v2/
mkdir -p $FLUSTER_IOS_ROOT/packages/webviews/note_detail_webview/public/mathjax/output/chtml/fonts/woff-v2/
mkdir -p $FLUSTER_IOS_ROOT/packages/webviews/dictionary_webview/public/mathjax/output/chtml/fonts/woff-v2/

cp -r $FLUSTER_IOS_ROOT/node_modules/mathjax-full/es5/* $FLUSTER_IOS_ROOT/packages/webviews/standalone_mdx_preview/public/mathjax/output/chtml/fonts/woff-v2/
cp -r $FLUSTER_IOS_ROOT/node_modules/mathjax-full/es5/* $FLUSTER_IOS_ROOT/packages/webviews/splitview_mdx_editor/public/mathjax/output/chtml/fonts/woff-v2/
cp -r $FLUSTER_IOS_ROOT/node_modules/mathjax-full/es5/* $FLUSTER_IOS_ROOT/packages/webviews/note_detail_webview/public/mathjax/output/chtml/fonts/woff-v2/
cp -r $FLUSTER_IOS_ROOT/node_modules/mathjax-full/es5/* $FLUSTER_IOS_ROOT/packages/webviews/dictionary_webview/public/mathjax/output/chtml/fonts/woff-v2/
