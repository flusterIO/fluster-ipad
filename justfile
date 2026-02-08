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

format: format_package_jsons format_swift

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

generate_component_docs_paths: build_internal_cli
	./target/debug/fluster_internal_cli gather-component-doc-paths

generate_initial_note_paths: build_internal_cli
	tsx {{justfile_directory()}}/scripts/generate_initial_note_paths.ts

generate_initial_note_data: generate_initial_note_paths
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
	cd {{justfile_directory()}}/packages/rust/fluster_desktop_fs; cargo swift package -y


clear_macos_database:
	trash "/Users/bigsexy/Library/Containers/iglooDevelopment.Fluster-Desktop/Data/Library/Application Support/default.store"


build_cross_language: build_cross_language_schemas build_fluster_swift_mdx_parser 

generate_initial_launch_data: generate_initial_note_paths generate_component_docs_paths generate_initial_note_data

build_ipad_simulator:
	cd {{justfile_directory()}}/apps/fluster; xcodebuild -destination "platform=iOS Simulator,name=iPad Pro 13-inch M5 26.1" -scheme Fluster build

launch_ipad_simulator: build_ipad_simulator
	cd {{justfile_directory()}}/apps/fluster; xcrun simctl launch "iPad Pro 13-inch M5 26.1" iglooDevelopment.Fluster 

build_all_rust: build_cross_language_schemas
	cargo build

build_fluster_swift_mdx_parser: build_all_rust
	cd {{justfile_directory()}}/packages/rust/fluster_swift_mdx_parser; cargo-swift swift package -y

build_fluster_core_rust_utilities: build_all_rust
	cd {{justfile_directory()}}/packages/rust/fluster_core_utilities; cargo build

build_webview_utils: build_cross_language_schemas
	pnpm run -C packages/webview_utils build

build_dictionary_webview: build_cross_language_schemas build_webview_utils
	pnpm run -C packages/webviews/dictionary_webview build

build_note_detail_webview: build_cross_language_schemas build_webview_utils
	pnpm run -C packages/webviews/note_detail_webview build

build_standalone_mdx_preview_webview: build_webview_utils
	pnpm run -C packages/webviews/standalone_mdx_preview build

build_standalone_mdx_editor_webview: build_webview_utils
	pnpm run -C packages/webviews/standalone_mdx_editor build

build_splitview_mdx_editor: build_webview_utils 
	cd {{justfile_directory()}}/packages/webviews/splitview_mdx_editor; pnpm build

build_bibtex_editor_webview: build_webview_utils
	pnpm run -C packages/webviews/bibtex_editor_webview build

build_all_webviews: build_cross_language_schemas build_webview_utils build_splitview_mdx_editor build_bibtex_editor_webview build_note_detail_webview build_dictionary_webview

pre_ipad_build: generate_initial_launch_data build_cross_language_schemas generate_initial_note_paths build_fluster_swift_mdx_parser build_all_webviews

generate_shiki_themes:
	tsx scripts/write_bundled_themes.ts

pre_desktop_build: generate_shiki_themes generate_initial_launch_data build_cross_language_schemas generate_initial_note_paths build_fluster_core_rust_utilities build_desktop_fs build_webview_utils build_splitview_mdx_editor

create_desktop_feature feature_name:
	python scripts/create_feature_util.py {{feature_name}}

test_rust: build_cross_language_schemas
	cargo nextest run --no-capture
