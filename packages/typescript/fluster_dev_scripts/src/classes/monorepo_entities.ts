import { ProjectRootDir } from "../classes/package_json";

class MonorepoProjectWebsite extends ProjectRootDir {
    constructor() {
        super("/Users/bigsexy/Desktop/swift/Fluster/apps/website");
    }
}
class MonorepoProjectStandaloneMdxPreview extends ProjectRootDir {
    constructor() {
        super(
            "/Users/bigsexy/Desktop/swift/Fluster/packages/webviews/standalone_mdx_preview",
        );
    }
}
class MonorepoProjectStandaloneMdxEditor extends ProjectRootDir {
    constructor() {
        super(
            "/Users/bigsexy/Desktop/swift/Fluster/packages/webviews/standalone_mdx_editor",
        );
    }
}
class MonorepoProjectSplitviewMdxEditor extends ProjectRootDir {
    constructor() {
        super(
            "/Users/bigsexy/Desktop/swift/Fluster/packages/webviews/splitview_mdx_editor",
        );
    }
}
class MonorepoProjectNoteDetailWebview extends ProjectRootDir {
    constructor() {
        super(
            "/Users/bigsexy/Desktop/swift/Fluster/packages/webviews/note_detail_webview",
        );
    }
}
class MonorepoProjectDictionaryWebview extends ProjectRootDir {
    constructor() {
        super(
            "/Users/bigsexy/Desktop/swift/Fluster/packages/webviews/dictionary_webview",
        );
    }
}
class MonorepoProjectBibtexEditorWebview extends ProjectRootDir {
    constructor() {
        super(
            "/Users/bigsexy/Desktop/swift/Fluster/packages/webviews/bibtex_editor_webview",
        );
    }
}
class MonorepoProjectBibEntryDetailsWebview extends ProjectRootDir {
    constructor() {
        super(
            "/Users/bigsexy/Desktop/swift/Fluster/packages/webviews/bib_entry_details_webview",
        );
    }
}
class MonorepoProjectWebviewUtils extends ProjectRootDir {
    constructor() {
        super("/Users/bigsexy/Desktop/swift/Fluster/packages/webview_utils");
    }
}

class MonorepoProjectLezer extends ProjectRootDir {
    constructor() {
        super("/Users/bigsexy/Desktop/swift/Fluster/packages/typescript/lezer");
    }
}

/**
 * Not auto updated, so p
 */
export const allMonorepoPackages = [
    MonorepoProjectWebsite,

    MonorepoProjectStandaloneMdxPreview,

    MonorepoProjectStandaloneMdxEditor,

    MonorepoProjectSplitviewMdxEditor,

    MonorepoProjectNoteDetailWebview,

    MonorepoProjectDictionaryWebview,

    MonorepoProjectBibtexEditorWebview,

    MonorepoProjectBibEntryDetailsWebview,

    MonorepoProjectWebviewUtils,

    MonorepoProjectLezer,
];
