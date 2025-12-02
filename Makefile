PROJECT_NAME := $(shell basename $(PWD))
COMMIT=$(shell git rev-parse HEAD)
BRANCH=$(shell git rev-parse --abbrev-rev HEAD)
BUILD_DATE := $(shell date -u +"%Y-%m-%d%H:%M:%SZ")
VERSION := $(shell git describe --tags --abbrev=0)

format_package_jsons:
	pnpm syncpack fix-mismatches
	pnpm syncpack format

generate_build_output:
	cd ${FLUSTER_IOS_ROOT}/apps/fluster; xcodebuild | tee xcodebuild.log | xcpretty

lint:
	cd ${FLUSTER_IOS_ROOT}/apps/fluster; swiftlint lint

generate_initial_note_paths:
	tsx ${FLUSTER_IOS_ROOT}/scripts/generate_initial_note_paths.ts

build_ipad_simulator:
	cd ${FLUSTER_IOS_ROOT}/apps/fluster; xcodebuild -destination "platform=iOS Simulator,name=iPad Pro 13-inch M5 26.1" -scheme Fluster build

launch_ipad_simulator: build_ipad_simulator
	cd ${FLUSTER_IOS_ROOT}/apps/fluster; xcrun simctl launch "iPad Pro 13-inch M5 26.1" iglooDevelopment.Fluster 

build_all_rust:
	cargo build

build_fluster_rust: build_all_rust
	cd ${FLUSTER_IOS_ROOT}/packages/rust/fluster_rust; cargo-swift swift package -y

build_webview_utils:
	pnpm run -C packages/webview_utils build

build_standalone_mdx_preview_webview: build_webview_utils
	pnpm run -C packages/webviews/standalone_mdx_preview build

build_standalone_mdx_editor_webview: build_webview_utils
	pnpm run -C packages/webviews/standalone_mdx_editor build

build_splitview_mdx_editor: build_webview_utils 
	cd ${FLUSTER_IOS_ROOT}/packages/webviews/splitview_mdx_editor; pnpm build

build_bibtex_editor_webview: build_webview_utils
	pnpm run -C packages/webviews/bibtex_editor_webview build

build_all_webviews: build_webview_utils build_fluster_rust build_splitview_mdx_editor build_standalone_mdx_editor_webview build_standalone_mdx_preview_webview build_bibtex_editor_webview

pre_swift_build: generate_initial_note_paths build_fluster_rust build_all_webviews
