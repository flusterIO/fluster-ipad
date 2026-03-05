export PROJECT_NAME := `basename $(PWD)`
export COMMIT := `git rev-parse HEAD`
export BRANCH := `git rev-parse --abbrev-rev HEAD`
export BUILD_DATE := `date -u +"%Y-%m-%d%H:%M:%SZ"`
# Set env variables
export RUST_BACKTRACE := "1"

set dotenv-required
set dotenv-path := ".env.local"
set dotenv-load := true

format_package_jsons:
	pnpm syncpack fix-mismatches
	pnpm syncpack format

format_swift:
	swift-format format --configuration={{justfile_directory()}}/.swift-format -ipr {{justfile_directory()}}/apps/fluster
	swift-format format --configuration={{justfile_directory()}}/.swift-format -ipr {{justfile_directory()}}/packages/swift

resolve_swift_packages:
	cd {{justfile_directory()}}/packages/swift/FlusterData; swift package resolve
	cd {{justfile_directory()}}/packages/swift/FlusterMdx; swift package resolve
	cd {{justfile_directory()}}/packages/swift/FlusterSwift; swift package resolve
	cd {{justfile_directory()}}/packages/swift/FlusterAI; swift package resolve

format: format_package_jsons format_swift

benchmark_pre_parser:
	cd {{justfile_directory()}}/packages/rust/fluster_pre_parser && cargo criterion --message-format="json" | tsx {{justfile_directory()}}/scripts/documentation/pipe_criterion_message_data_to_persistent_data.ts

build_website:
	cd apps/website; pnpm build

website_dev:
	cd apps/website; pnpm dev

generate_build_output:
	cd {{justfile_directory()}}/apps/fluster; xcodebuild | tee xcodebuild.log | xcpretty

lint:
	cd {{justfile_directory()}}/apps/fluster; swiftlint lint

build_internal_cli:
	cd {{justfile_directory()}}/packages/rust/fluster_internal_cli; cargo build

build_fluster_wasm:
	# cd {{justfile_directory()}}/packages/typescript/wasm/fluster_wasm; pnpm build
	cd {{justfile_directory()}}/packages/typescript/wasm/fluster_wasm; pnpm build

gather_component_docs: write_zod_schema_docs write_component_list_note
	tsx {{justfile_directory()}}/scripts/gather_component_docs.ts

generate_documentation_diagrams:
	./node_modules/@mermaid-js/mermaid-cli/src/cli.js -i ./docs/development/dependency_diagram/package_diagram.mmd -o ./docs/development/dependency_diagram/package_diagram_dark.png -t dark -b transparent
	./node_modules/@mermaid-js/mermaid-cli/src/cli.js -i ./docs/development/dependency_diagram/package_diagram.mmd -o ./docs/development/dependency_diagram/package_diagram_light.png -t default 
	./node_modules/@mermaid-js/mermaid-cli/src/cli.js -i ./docs/development/ai_parser_sequence/ai_parser_diagram.mmd -o ./docs/development/ai_parser_sequence/package_diagram_dark.png -t dark -b transparent
	./node_modules/@mermaid-js/mermaid-cli/src/cli.js -i ./docs/development/ai_parser_sequence/ai_parser_diagram.mmd -o ./docs/development/ai_parser_sequence/package_diagram_light.png -t default 
	./node_modules/@mermaid-js/mermaid-cli/src/cli.js -i ./docs/development/rust_parsing_sequence/rust_parsing_sequence.mmd -o ./docs/development/rust_parsing_sequence/rust_parsing_diagram_dark.png -t dark -b transparent
	./node_modules/@mermaid-js/mermaid-cli/src/cli.js -i ./docs/development/rust_parsing_sequence/rust_parsing_sequence.mmd -o ./docs/development/rust_parsing_sequence/rust_parsing_diagram_light.png -t default 

write_component_list_note:
	tsx {{justfile_directory()}}/scripts/documentation/write_embeddable_component_list_note.ts

write_zod_schema_docs: 
	tsx {{justfile_directory()}}/scripts/documentation/write_zod_schema_to_markdown.ts


write_in_content_docs_by_id: write_zod_schema_docs gather_component_docs write_zod_schema_docs
	tsx {{justfile_directory()}}/scripts/documentation/write_content_by_in_content_doc_id.ts

generate_docs: write_in_content_docs_by_id write_zod_schema_docs generate_documentation_diagrams gather_component_docs write_component_list_note


## Deprecated, now that component docs are being gathered by typescript, I think?
generate_component_docs_paths: build_internal_cli generate_documentation_diagrams 

generate_initial_note_paths: build_internal_cli gather_component_docs
	tsx {{justfile_directory()}}/scripts/generate_initial_note_paths.ts

generate_initial_note_data: generate_initial_note_paths write_in_content_docs_by_id write_zod_schema_docs
	cp /Users/bigsexy/Desktop/notes/content/physics/ipad_app_notes/on_the_gravitational_nature_of_time.mdx /Users/bigsexy/Desktop/swift/Fluster/docs/initial_note_docs/on_the_gravitational_nature_of_time.mdx
	./target/debug/fluster_internal_cli parse-initial-notes

build_cross_language_schemas: generate_initial_note_data
	$FLAT_BUFFER_PATH -o ./packages/swift/FlusterData/Sources/FlusterData/code_gen/flat_buffer/ ./flatbuffers_schemas/v1_flat_buffer_schema.fbs --swift
	$FLAT_BUFFER_PATH -o ./packages/rust/fluster_core_utilities/src/code_gen/flat_buffer/ ./flatbuffers_schemas/v1_flat_buffer_schema.fbs --rust
	$FLAT_BUFFER_PATH -o ./packages/webview_utils/src/core/code_gen/flat_buffer/ ./flatbuffers_schemas/v1_flat_buffer_schema.fbs --ts
	$FLAT_BUFFER_PATH -o ./packages/generated/desktop_bindings/src/flat_buffer/ ./flatbuffers_schemas/v1_flat_buffer_schema.fbs --ts
	typeshare {{justfile_directory()}}/packages/rust/fluster_core_utilities --lang=typescript --output-folder={{justfile_directory()}}/packages/webview_utils/src/core/code_gen/typeshare
	typeshare {{justfile_directory()}}/packages/rust/fluster_core_utilities --lang=swift --output-folder={{justfile_directory()}}/packages/swift/FlusterData/Sources/FlusterData/code_gen/typeshare


build_desktop_fs:
	cd {{justfile_directory()}}/packages/rust/fluster_desktop_fs; cargo swift package -y --xcframework-name FlusterDT

build_fluster_bibliography:
	cd {{justfile_directory()}}/packages/rust/fluster_bibliography; cargo swift package -y --xcframework-name FlusterBib

clear_macos_database:
	trash "/Users/bigsexy/Library/Containers/iglooDevelopment.Fluster-Desktop/Data/Library/Application Support/default.store"

build_fluster_lezer:
	cd {{justfile_directory()}}/packages/typescript/lezer; pnpm build

build_cross_language_all: build_cross_language_schemas 
	tsx {{justfile_directory()}}/scripts/post_cross_language_modifications.ts

generate_initial_launch_data: generate_initial_note_paths generate_component_docs_paths generate_initial_note_data write_in_content_docs_by_id

build_ipad_simulator:
	cd {{justfile_directory()}}/apps/fluster; xcodebuild -destination "platform=iOS Simulator,name=iPad Pro 13-inch M5 26.1" -scheme Fluster build

launch_ipad_simulator: build_ipad_simulator
	cd {{justfile_directory()}}/apps/fluster; xcrun simctl launch "iPad Pro 13-inch M5 26.1" iglooDevelopment.Fluster 

build_all_rust: build_cross_language_all
	cargo build

build_fluster_pre_parser:
	cd {{justfile_directory()}}/packages/rust/fluster_pre_parser; cargo build

build_fluster_swift_mdx_parser: build_cross_language_all build_fluster_pre_parser
	cd {{justfile_directory()}}/packages/rust/fluster_swift_mdx_parser; cargo-swift swift package -y --xcframework-name FlusterSwiftMdxParse

build_fluster_core_rust_utilities: build_cross_language_all
	cd {{justfile_directory()}}/packages/rust/fluster_core_utilities; cargo build

build_webview_utils: build_cross_language_all build_fluster_lezer gather_component_docs build_fluster_wasm
	pnpm run -C packages/webview_utils build

build_dictionary_webview: build_cross_language_all build_webview_utils
	pnpm run -C packages/webviews/dictionary_webview build

build_dictionary_webview_mac: build_cross_language_all build_webview_utils
	pnpm run -C packages/webviews/dictionary_webview build:mac

build_dictionary_webview_ipad: build_cross_language_all build_webview_utils
	pnpm run -C packages/webviews/dictionary_webview build:ipad

build_note_detail_webview: build_cross_language_all build_webview_utils
	pnpm run -C packages/webviews/note_detail_webview build

build_note_detail_webview_ipad: build_cross_language_all build_webview_utils
	pnpm run -C packages/webviews/note_detail_webview build:ipad

build_note_detail_webview_mac: build_cross_language_all build_webview_utils
	pnpm run -C packages/webviews/note_detail_webview build:mac

build_standalone_mdx_preview_webview_ipad: build_webview_utils
	pnpm run -C packages/webviews/standalone_mdx_preview build:ipad

build_standalone_mdx_preview_webview_mac: build_webview_utils
	pnpm run -C packages/webviews/standalone_mdx_preview build:mac

build_standalone_mdx_editor_webview: build_webview_utils
	pnpm run -C packages/webviews/standalone_mdx_editor build

build_splitview_mdx_editor: build_webview_utils 
	cd {{justfile_directory()}}/packages/webviews/splitview_mdx_editor; pnpm build

build_splitview_mdx_editor_mac: build_webview_utils 
	cd {{justfile_directory()}}/packages/webviews/splitview_mdx_editor; pnpm build:mac
build_splitview_mdx_editor_ipad: build_webview_utils 
	cd {{justfile_directory()}}/packages/webviews/splitview_mdx_editor; pnpm build:ipad

build_bibtex_editor_webview: build_webview_utils
	pnpm run -C packages/webviews/bibtex_editor_webview build

build_bibtex_editor_webview_ipad: build_webview_utils
	pnpm run -C packages/webviews/bibtex_editor_webview build:ipad
build_bibtex_editor_webview_mac: build_webview_utils
	pnpm run -C packages/webviews/bibtex_editor_webview build:mac

build_fluster_language_lezer:
	./node_modules/@lezer/generator/src/lezer-generator.cjs ./packages/typescript/lezer/src/fluster/fluster_math.grammar -o ./packages/typescript/lezer/src/fluster/fluster_math_parser.js

build_all_webviews: build_cross_language_all build_webview_utils build_splitview_mdx_editor build_bibtex_editor_webview build_note_detail_webview build_dictionary_webview

pre_ipad_build: generate_initial_launch_data write_zod_schema_docs build_cross_language_all build_fluster_bibliography generate_initial_note_paths build_fluster_swift_mdx_parser build_splitview_mdx_editor_ipad build_bibtex_editor_webview_ipad build_note_detail_webview_ipad build_dictionary_webview_ipad

generate_shiki_themes:
	tsx scripts/write_bundled_themes.ts

pre_desktop_build: generate_shiki_themes write_zod_schema_docs generate_initial_launch_data build_fluster_swift_mdx_parser build_cross_language_all build_fluster_bibliography generate_initial_note_paths build_fluster_core_rust_utilities build_desktop_fs build_webview_utils build_splitview_mdx_editor_mac build_standalone_mdx_preview_webview_mac build_bibtex_editor_webview_mac build_dictionary_webview_mac build_note_detail_webview_mac

test_rust: build_cross_language_schemas
	cargo nextest run --no-capture
