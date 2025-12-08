PROJECT_NAME := $(shell basename $(PWD))
COMMIT=$(shell git rev-parse HEAD)
BRANCH=$(shell git rev-parse --abbrev-rev HEAD)
BUILD_DATE := $(shell date -u +"%Y-%m-%d%H:%M:%SZ")
VERSION := $(shell git describe --tags --abbrev=0)
FLAT_BUFFER_PATH=/Users/bigsexy/dev-utils/bin/flatc

format_package_jsons:
	pnpm syncpack fix-mismatches
	pnpm syncpack format

generate_build_output:
	cd ${FLUSTER_IOS_ROOT}/apps/fluster; xcodebuild | tee xcodebuild.log | xcpretty

lint:
	cd ${FLUSTER_IOS_ROOT}/apps/fluster; swiftlint lint

build_cross_language_schemas:
	${FLAT_BUFFER_PATH} -o ./packages/swift/FlusterSwift/Sources/FlusterSwift/core/code_gen/flat_buffer/ ./flatbuffers_schemas/v1_flat_buffer_schema.fbs --swift
	${FLAT_BUFFER_PATH} -o ./packages/rust/fluster_core_utilities/src/code_gen/flat_buffer/ ./flatbuffers_schemas/v1_flat_buffer_schema.fbs --rust
	${FLAT_BUFFER_PATH} -o ./packages/webview_utils/src/core/code_gen//flat_buffer/ ./flatbuffers_schemas/v1_flat_buffer_schema.fbs --ts
	typeshare ${FLUSTER_IOS_ROOT}/packages/rust/fluster_core_utilities --lang=typescript --output-folder=${FLUSTER_IOS_ROOT}/packages/webview_utils/src/core/code_gen/typeshare
	typeshare ${FLUSTER_IOS_ROOT}/packages/rust/fluster_core_utilities --lang=swift --output-folder=${FLUSTER_IOS_ROOT}/packages/swift/FlusterSwift/Sources/FlusterSwift/core/code_gen/typeshare

generate_initial_note_paths:
	tsx ${FLUSTER_IOS_ROOT}/scripts/generate_initial_note_paths.ts

build_ipad_simulator:
	cd ${FLUSTER_IOS_ROOT}/apps/fluster; xcodebuild -destination "platform=iOS Simulator,name=iPad Pro 13-inch M5 26.1" -scheme Fluster build

launch_ipad_simulator: build_ipad_simulator
	cd ${FLUSTER_IOS_ROOT}/apps/fluster; xcrun simctl launch "iPad Pro 13-inch M5 26.1" iglooDevelopment.Fluster 

build_all_rust: build_cross_language_schemas
	cargo build

build_fluster_rust: build_all_rust
	cd ${FLUSTER_IOS_ROOT}/packages/rust/fluster_rust; cargo-swift swift package -y

build_webview_utils: build_cross_language_schemas
	pnpm run -C packages/webview_utils build

build_note_detail_webview: build_cross_language_schemas build_webview_utils
	pnpm run -C packages/webviews/note_detail_webview build

build_standalone_mdx_preview_webview: build_webview_utils
	pnpm run -C packages/webviews/standalone_mdx_preview build

build_standalone_mdx_editor_webview: build_webview_utils
	pnpm run -C packages/webviews/standalone_mdx_editor build

build_splitview_mdx_editor: build_webview_utils 
	cd ${FLUSTER_IOS_ROOT}/packages/webviews/splitview_mdx_editor; pnpm build

build_bibtex_editor_webview: build_webview_utils
	pnpm run -C packages/webviews/bibtex_editor_webview build

build_all_webviews: build_cross_language_schemas build_webview_utils build_splitview_mdx_editor build_bibtex_editor_webview build_note_detail_webview

pre_ipad_build: build_cross_language_schemas generate_initial_note_paths build_fluster_rust build_all_webviews
