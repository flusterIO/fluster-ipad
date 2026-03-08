import eslint from "@eslint/js";
import { defineConfig } from "eslint/config";
import tseslint, { Config, ConfigWithExtends } from "typescript-eslint";

const lenient = [tseslint.configs.recommendedTypeChecked];
const strict = [
  tseslint.configs.strictTypeChecked,
  tseslint.configs.stylisticTypeChecked,
];

export default tseslint.config(
  // 1. GLOBAL IGNORES
  // Must be its own isolated object to apply globally across the whole monorepo
  {
    ignores: [
      "target/**",
      "**/.wireit/**",
      "**/node_modules/**",
      "apps/fluster/**",
      "apps/fluster_desktop/**",
      "**/*.xcframework/**",
      ".nvim/",
      "shared_assets/",
      "packages/rust/conundrum/",
      "packages/rust/fluster_bibliography/",
      "packages/rust/fluster_core_utilities/",
      "packages/rust/fluster_database/",
      "packages/rust/fluster_desktop_fs/",
      "packages/rust/fluster_internal_cli/",
      "packages/rust/fluster_pre_parser/",
      "packages/rust/fluster_swift_mdx_parser/",
      "packages/rust/uniffi_bindgen/",
      "packages/swift/",
      "packages/rust/fluster_rust/FlusterRust/**",
      "packages/webviews/*/node_modules/**",
      "packages/webviews/*/build/**",
      "packages/webviews/bibtex_editor_webview/bibtex_editor_webview_ipad/**",
      "packages/webviews/bibtex_editor_webview/bibtex_editor_webview_mac/**",
      "packages/webviews/reference_note_webview/reference_note_webview/**",
      "packages/webviews/standalone_mdx_editor/standalone_mdx_editor/**",
      "packages/webviews/note_detail_webview/note_detail_webview_ipad/**",
      "packages/webviews/note_detail_webview/note_detail_webview_mac/**",
      "packages/webviews/standalone_mdx_preview/standalone_mdx_preview_mac/**",
      "packages/webviews/standalone_mdx_preview/standalone_mdx_preview_ipad/**",
      "packages/webviews/splitview_mdx_editor/splitview_mdx_editor_mac/**",
      "packages/webviews/splitview_mdx_editor/splitview_mdx_editor_ipad/**",
      "packages/webviews/dictionary_webview/dictionary_webview_mac/**",
      "packages/webviews/dictionary_webview/dictionary_webview_ipad/**",
      "packages/webview_utils/dist_dev/**",
      "packages/webview_utils/dist_dev/dist/**",
      "apps/website/node_modules/**",
      "packages/rust/fluster_swift_mdx_parser/FlusterSwiftMdxParser/**",
      "packages/rust/fluster_bibliography/FlusterBibliography/**",
      ".nvim/xcodebuild/**",
      "docs/component_docs/**",
      "packages/rust/fluster_pre_parser/src/embedded/component_docs/**",
      "packages/typescript/wasm/fluster_wasm/dist/**",
      "packages/typescript/wasm/fluster_wasm/pkg/**",
      "packages/rust/wasm/fluster_wasm/dist/**",
      "./node_modules/**",
    ],
  },
  eslint.configs.recommended,
  ...tseslint.configs.strictTypeChecked,
  ...tseslint.configs.stylisticTypeChecked,
  {
    languageOptions: {
      parserOptions: {
        projectService: true,
        tsconfigRootDir: import.meta.dirname,
      },
    },

    linterOptions: {
      reportUnusedInlineConfigs: "warn",
    },

    // NOTE: Don't delete these comments!!!! Leave them for future referece. It' s a horrible practice, but it's a config file, so fuck it...
    rules: {
      "@typescript-eslint/no-restricted-imports": [
        "warn",
        {
          paths: [
            // {
            //     name: "react-dom",
            //     message: "Don't import this. This is really just here to remember how to do this.",
            //     allowTypeImports: true,
            // },
          ],
        },
      ],
      "@typescript-eslint/restrict-template-expressions": [
        "warn",
        {
          allowNumber: true,
          allowBoolean: false,
          allowNullish: false,
        },
      ],
      "@typescript-eslint/switch-exhaustiveness-check": "error",
      "@typescript-eslint/unbound-method": "warn",
      "@typescript-eslint/consistent-type-imports": [
        "warn",
        {
          prefer: "type-imports",
          fixStyle: "inline-type-imports",
        },
      ],
      "@typescript-eslint/no-floating-promises": [
        "warn",
        {
          ignoreVoid: true, // Allows 'void someAsyncFunc()' to explicitly be ignore
        },
      ],
      // '@typescript-eslint/no-restricted-syntax': ['error', {
      //    selector: 'VariableDeclarator[id.typeAnnotation.typeAnnotation.typeName.name="Mass"] > NumericLiteral',
      //    message: 'Always use the "toMass()" utility to create Mass types; do not assign raw numbers.'
      // }]
      // "@typescript-eslint/naming-convention": [
      //     "error",
      //     {
      //         selector: "interface",
      //         format: ["PascalCase"],
      //         custom: {
      //             regex: "^I[A-Z]", // Enforce (or ban) the "I" prefix based on your team's preference
      //             match: false,
      //         },
      //     },
      // ],
    },
  },
);
