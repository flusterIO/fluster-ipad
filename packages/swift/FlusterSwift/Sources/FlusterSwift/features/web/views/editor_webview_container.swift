import Combine
import SwiftUI
import WebKit

@MainActor
public final class EditorWebViewContainer: WebviewContainer {
    public override init() {}
    public func emitEditorThemeEvent(theme: CodeEditorTheme) {
        print("Changing editor theme event")
        self.runJavascript(
            """
            window.localStorage.setItem("editor-theme", "\(theme.rawValue)")
            window.setEditorTheme("\(theme.rawValue)")
            """
        )
    }
    public func setEditorLightTheme(theme: CodeEditorTheme) {
        self.runJavascript(
            """
            window.localStorage.setItem("editor-theme-light", "\(theme.rawValue)")
            window.setEditorThemeLight("\(theme.rawValue)")
            """
        )
    }
    public func setEditorDarkTheme(theme: CodeEditorTheme) {
        self.runJavascript(
            """
            window.localStorage.setItem("editor-theme-dark", "\(theme.rawValue)")
            window.setEditorThemeDark("\(theme.rawValue)")
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
        self.runJavascript("""
            window.resetMdxPreviewScrollPosition()
            """)
    }
    public func setInitialProperties(
        editingNote: NoteModel?,
        codeEditorTheme: CodeEditorTheme,
        editorKeymap: EditorKeymap,
        theme: WebViewTheme,
        fontSize: WebviewFontSize,
        editorThemeDark: CodeEditorTheme,
        editorThemeLight: CodeEditorTheme,
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
