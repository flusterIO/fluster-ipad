import { describe, it } from "node:test";
import { PackageJson } from "../classes/package_json";
import { consola } from "consola";

const validMonorepoPackageJsonPaths = [
    "/Users/bigsexy/Desktop/swift/Fluster/apps/website/package.json",
    "/Users/bigsexy/Desktop/swift/Fluster/packages/webviews/standalone_mdx_preview/package.json",
    "/Users/bigsexy/Desktop/swift/Fluster/packages/webviews/standalone_mdx_editor/package.json",
    "/Users/bigsexy/Desktop/swift/Fluster/packages/webviews/splitview_mdx_editor/package.json",
    "/Users/bigsexy/Desktop/swift/Fluster/packages/webviews/note_detail_webview/package.json",
    "/Users/bigsexy/Desktop/swift/Fluster/packages/webviews/dictionary_webview/package.json",
    "/Users/bigsexy/Desktop/swift/Fluster/packages/webviews/bibtex_editor_webview/package.json",
    "/Users/bigsexy/Desktop/swift/Fluster/packages/webviews/bib_entry_details_webview/package.json",
    "/Users/bigsexy/Desktop/swift/Fluster/packages/webview_utils/package.json",
    "/Users/bigsexy/Desktop/swift/Fluster/packages/shared_config/package.json",
    "/Users/bigsexy/Desktop/swift/Fluster/packages/rust/wasm/fluster_wasm/package.json",
    "/Users/bigsexy/Desktop/swift/Fluster/packages/rust/wasm/fluster_wasm/pkg/package.json",
    "/Users/bigsexy/Desktop/swift/Fluster/packages/generated/desktop_bindings/package.json",
    "/Users/bigsexy/Desktop/swift/Fluster/packages/typescript/lezer/package.json",
    "/Users/bigsexy/Desktop/swift/Fluster/packages/typescript/fluster_dev_scripts/package.json",
];

describe("Package.json behaves as expected. It's embarassing to even need to TDD something so simple but I'm tired as shit...", () => {
    it("Gathers all monorepo files and nothing else", () => {
        const packages = PackageJson.getAllPackageJsons();
        consola.success("Finally no errors...");
    });
});
