$FLAT_BUFFER_PATH -o $FLUSTER_IOS_ROOT/packages/swift/FlusterData/Sources/FlusterData/code_gen/flat_buffer/ $FLUSTER_IOS_ROOT/flatbuffers_schemas/v1_flat_buffer_schema.fbs --swift
$FLAT_BUFFER_PATH -o $FLUSTER_IOS_ROOT/packages/rust/fluster_core_utilities/src/code_gen/flat_buffer/ $FLUSTER_IOS_ROOT/flatbuffers_schemas/v1_flat_buffer_schema.fbs --rust
$FLAT_BUFFER_PATH -o $FLUSTER_IOS_ROOT/packages/webview_utils/src/core/code_gen/flat_buffer/ $FLUSTER_IOS_ROOT/flatbuffers_schemas/v1_flat_buffer_schema.fbs --ts
$FLAT_BUFFER_PATH -o $FLUSTER_IOS_ROOT/packages/generated/desktop_bindings/src/flat_buffer/ $FLUSTER_IOS_ROOT/flatbuffers_schemas/v1_flat_buffer_schema.fbs --ts
typeshare $FLUSTER_IOS_ROOT/packages/rust/fluster_core_utilities --lang=typescript --output-folder=$FLUSTER_IOS_ROOT/packages/webview_utils/src/core/code_gen/typeshare
typeshare $FLUSTER_IOS_ROOT/packages/rust/fluster_core_utilities --lang=swift --output-folder=$FLUSTER_IOS_ROOT/packages/swift/FlusterData/Sources/FlusterData/code_gen/typeshare
typeshare $FLUSTER_IOS_ROOT/packages/rust/conundrum --lang=typescript --output-folder=$FLUSTER_IOS_ROOT/packages/webview_utils/src/core/code_gen/typeshare
typeshare $FLUSTER_IOS_ROOT/packages/rust/conundrum --lang=typescript --output-folder=$FLUSTER_IOS_ROOT/packages/rust/wasm/fluster_wasm/src_typescript/core/code_gen/typeshare
typeshare $FLUSTER_IOS_ROOT/packages/rust/conundrum --lang=typescript --output-folder=$FLUSTER_IOS_ROOT/packages/rust/conundrum_ts/src/code_gen/typeshare
