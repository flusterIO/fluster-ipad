//
//  bibtex_editor_webview_container.swift
//  Fluster
//
//  Created by Andrew on 11/22/25.
//

import Foundation

final class BibtexEditorWebviewContainer: WebviewContainer {
    func emitEditorThemeEvent(theme: CodeEditorTheme) {
        print("Changing editor theme event")
        self.runJavascript(
            """
            window.localStorage.setItem("editor-theme", "\(theme.rawValue)")
            window.setEditorTheme("\(theme.rawValue)")
            """
        )
    }
    func setEditorKeymap(editorKeymap: EditorKeymap) {
        print("Applying editor keymap")
        self.runJavascript(
            """
            window.localStorage.setItem("editor-keymap", "\(editorKeymap.rawValue)")
            window.setEditorKeymap("\(editorKeymap.rawValue)")
            """
        )
    }
    func clearEditorData() {
        self.runJavascript("""
            window.clearBibtexEditorData()
            """)
    }
    func setInitialContent(entryBody: String) {
        let body = entryBody.toQuotedJavascriptString()
        self.runJavascript(
            """
            window.localStorage.setItem("bibtex-editor-initial-value", \(body))
            window.setBibtexEditorContent(\(body))
            """
        )
    }
    func setInitialProperties(
        initialValue: String?,
        codeEditorTheme: CodeEditorTheme,
        editorKeymap: EditorKeymap,
        theme: WebViewTheme,
        fontSize: WebviewFontSize,
        darkMode: Bool
    ) {
        self.applyWebViewColorScheme(darkMode: darkMode)
        self.emitEditorThemeEvent(theme: codeEditorTheme)
        self.setEditorKeymap(editorKeymap: editorKeymap)
        self.setWebviewTheme(theme: theme)
        self.setWebviewFontSize(fontSize: fontSize)
        self.setInitialContent(entryBody: initialValue ?? "")
    }
}
