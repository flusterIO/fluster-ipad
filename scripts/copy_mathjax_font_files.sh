
trash $FLUSTER_IOS_ROOT/packages/webviews/standalone_mdx_preview/public/Mathjax*.woff
trash $FLUSTER_IOS_ROOT/packages/webviews/splitview_mdx_editor/public/Mathjax*.woff
trash $FLUSTER_IOS_ROOT/packages/webviews/note_detail_webview/public/Mathjax*.woff
trash $FLUSTER_IOS_ROOT/packages/webviews/dictionary_webview/public/Mathjax*.woff

cp -r $FLUSTER_IOS_ROOT/node_modules/mathjax-full/es5/output/chtml/fonts/woff-v2/* $FLUSTER_IOS_ROOT/packages/webviews/standalone_mdx_preview/public/
cp -r $FLUSTER_IOS_ROOT/node_modules/mathjax-full/es5/output/chtml/fonts/woff-v2/* $FLUSTER_IOS_ROOT/packages/webviews/splitview_mdx_editor/public/
cp -r $FLUSTER_IOS_ROOT/node_modules/mathjax-full/es5/output/chtml/fonts/woff-v2/* $FLUSTER_IOS_ROOT/packages/webviews/note_detail_webview/public/
cp -r $FLUSTER_IOS_ROOT/node_modules/mathjax-full/es5/output/chtml/fonts/woff-v2/* $FLUSTER_IOS_ROOT/packages/webviews/dictionary_webview/public/
