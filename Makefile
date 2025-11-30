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

build_webview_utils:
	pnpm run -C packages/webview_utils build

build_standalone_mdx_preview_webview: build_webview_utils
	pnpm run -C packages/webviews/standalone_mdx_preview build

build_standalone_mdx_editor_webview: build_webview_utils
	pnpm run -C packages/webviews/standalone_mdx_editor build

build_editor_splitview_webview: build_webview_utils build_standalone_mdx_preview_webview build_standalone_mdx_editor_webview

build_bibtex_editor_webview: build_webview_utils
	pnpm run -C packages/webviews/bibtex_editor_webview build


build_all_webviews: build_webview_utils build_editor_splitview_webview build_standalone_mdx_editor_webview build_standalone_mdx_preview_webview build_bibtex_editor_webview

pre_swift_build: generate_initial_note_paths build_all_webviews
