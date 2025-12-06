//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 11/29/25.
//

import Combine
import Foundation
import SwiftUI

@MainActor
public final class MdxPreviewWebviewContainer: WebviewContainer<MdxPreviewWebviewEvents> {
//    public override init() {}
    public func setEditorLightTheme(theme: CodeSyntaxTheme) {
        self.runJavascript(
            """
            window.localStorage.setItem("code-theme-light", "\(theme.rawValue)")
            window.setCodeSyntaxThemeLight("\(theme.rawValue)")
            """
        )
    }
    public func setEditorDarkTheme(theme: CodeSyntaxTheme) {
        self.runJavascript(
            """
            window.localStorage.setItem("code-theme-dark", "\(theme.rawValue)")
            window.setCodeSyntaxThemeDark("\(theme.rawValue)")
            """
        )
    }
    public func setInitialContent(note: NoteModel) {
        // TODO: Move this to method on markdown class to keep everything in one place.
        let content = MdxText(body: note.markdown.body)
        content.parse()
        self.runJavascript(
            """
            window.setMdxPreviewContent(\(content.body.toQuotedJavascriptString()))
            """
        )
    }
    public func resetScrollPosition() {
        self.runJavascript("""
            if ('resetMdxPreviewScrollPosition' in window) {
                window.resetMdxPreviewScrollPosition()
            }
            """)
    }
    public func setInitialProperties(
        editingNote: NoteModel?,
        codeEditorTheme: CodeSyntaxTheme,
        editorKeymap: EditorKeymap,
        theme: WebViewTheme,
        fontSize: WebviewFontSize,
        editorThemeDark: CodeSyntaxTheme,
        editorThemeLight: CodeSyntaxTheme,
        darkMode: Bool
    ) {
        // TODO: Implement this initial data in a minimal way.
        self.applyWebViewColorScheme(darkMode: darkMode)
        self.setWebviewTheme(theme: theme)
        self.setWebviewFontSize(fontSize: fontSize)
        self.setEditorDarkTheme(theme: editorThemeDark)
        self.setEditorLightTheme(theme: editorThemeLight)
        if editingNote != nil {
            self.setInitialContent(note: editingNote!)
        }
    }
}
