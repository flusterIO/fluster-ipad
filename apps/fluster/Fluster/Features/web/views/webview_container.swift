import Combine
import SwiftUI
import WebKit
import FlusterSwift

func getConfig() -> WKWebViewConfiguration {
    // configuring the `WKWebView` is very important
    // without doing this the local index.html will not be able to read
    // the css or js files properly
    let config = WKWebViewConfiguration()
    config.preferences.setValue(true, forKey: "allowFileAccessFromFileURLs")
    config.setValue(true, forKey: "allowUniversalAccessFromFileURLs")
    return config
}

class WebviewContainer: ObservableObject {
    @Published var justToConform: Bool = false
    let webView: WKWebView = {
        let view = WKWebView(frame: .zero, configuration: getConfig())
        view.scrollView.isScrollEnabled = false
        view.scrollView.minimumZoomScale = 1
        view.scrollView.maximumZoomScale = 1
        view.allowsBackForwardNavigationGestures = false
        view.isHidden = true
        if #available(iOS 16.4, macOS 13.3, *) {
            view.isInspectable = true
        }

        return view
    }()
    func runJavascript(_ script: String) {
        self.webView.evaluateJavaScript(script) { result, error in
            if let error = error {
                print("Error executing JS: \(error.localizedDescription)")
            } else if let result = result {
                print("JS Result: \(result)")
            }
        }
    }
    func setWebviewTheme(theme: WebViewTheme) {
        self.runJavascript(
            """
            window.setWebviewTheme("\(theme.rawValue)")
            """)
    }
    func setWebviewFontSize(fontSize: WebviewFontSize) {
        self.runJavascript(
            """
            window.localStorage.setItem("webview-font-class", "\(fontSize.cssClass())");
            window.setWebViewFontSize('\(fontSize.cssClass())');
            """)
    }
    func applyWebViewColorScheme(darkMode: Bool) {
        print("Applying webview color scheme")
        self.runJavascript(
            """
            window.localStorage.setItem("dark-mode", "\(darkMode ? "true" : "false")")
            window.setDarkMode(\(darkMode ? "true" : "false"))
            """
        )
    }
}

final class EditorWebViewContainer: WebviewContainer {
    func emitEditorThemeEvent(theme: CodeEditorTheme) {
        print("Changing editor theme event")
        self.runJavascript(
            """
            window.localStorage.setItem("editor-theme", "\(theme.rawValue)")
            window.setEditorTheme("\(theme.rawValue)")
            """
        )
    }
    func setEditorLightTheme(theme: CodeEditorTheme) {
        self.runJavascript(
            """
            window.localStorage.setItem("editor-theme-light", "\(theme.rawValue)")
            window.setEditorThemeLight("\(theme.rawValue)")
            """
        )
    }
    func setEditorDarkTheme(theme: CodeEditorTheme) {
        self.runJavascript(
            """
            window.localStorage.setItem("editor-theme-dark", "\(theme.rawValue)")
            window.setEditorThemeDark("\(theme.rawValue)")
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
    func setInitialContent(note: NoteModel) {
        self.runJavascript(
            """
            window.setEditorContent(\(note.markdown.body.toQuotedJavascriptString()))
            """
        )
    }
    func resetScrollPosition() {
        self.runJavascript("""
            window.resetMdxPreviewScrollPosition()
            """)
    }
    func setInitialProperties(
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
