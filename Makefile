PROJECT_NAME := $(shell basename $(PWD))
COMMIT=$(shell git rev-parse HEAD)
BRANCH=$(shell git rev-parse --abbrev-rev HEAD)
BUILD_DATE := $(shell date -u +"%Y-%m-%d%H:%M:%SZ")
VERSION := $(shell git describe --tags --abbrev=0)

format_package_jsons:
	pnpm syncpack fix-mismatches
	pnpm syncpack format

lint:
	cd ${FLUSTER_IOS_ROOT}/apps/fluster; swiftlint lint

build_webview_utils:
	pnpm run -C packages/webview_utils build

build_editor_splitview_webview: build_webview_utils
	pnpm run -C packages/webviews/editor_splitview_webview build

build_bibtex_editor_webview: build_webview_utils
	pnpm run -C packages/webviews/bibtex_editor_webview build
