import Combine
import SwiftUI
import WebKit

@MainActor
public final class MdxEditorWebviewContainer: WebviewContainer {
    public func emitEditorThemeEvent(theme: CodeSyntaxTheme) {
        self.runJavascript(
            """
            window.localStorage.setItem("code-theme", "\(theme.rawValue)")
            window.setCodeSyntaxTheme("\(theme.rawValue)")
            """
        )
    }
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
    public func setEditorKeymap(editorKeymap: EditorKeymap) {
        print("Applying editor keymap")
        self.runJavascript(
            """
            window.localStorage.setItem("editor-keymap", "\(editorKeymap.rawValue)")
            window.setEditorKeymap("\(editorKeymap.rawValue)")
            """
        )
    }
    public func setInitialContent(note: NoteModel) {
        self.runJavascript(
            """
            window.setEditorContent(\(note.markdown.body.toQuotedJavascriptString()))
            """
        )
    }
    public func resetScrollPosition() {
        self.runJavascript(
            """
            if ('resetMdxPreviewScrollPosition' in window) {
                window.resetMdxPreviewScrollPosition()
            }
            """)
    }
    public func setParsedEditorContent(content: String) {
        self.runJavascript("""
            window.setParsedEditorContent(\(content.toQuotedJavascriptString()))
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
        self.applyWebViewColorScheme(darkMode: darkMode)
        self.emitEditorThemeEvent(theme: codeEditorTheme)
        self.setEditorKeymap(editorKeymap: editorKeymap)
        self.setWebviewTheme(theme: theme)
        self.setWebviewFontSize(fontSize: fontSize)
        self.setEditorDarkTheme(theme: editorThemeDark)
        self.setEditorLightTheme(theme: editorThemeLight)
        if editingNote != nil {
            self.setInitialContent(note: editingNote!)
        }
    }
}
