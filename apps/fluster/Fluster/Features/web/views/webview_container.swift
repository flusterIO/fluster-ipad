import Combine
import SwiftUI
import WebKit

func getConfig() -> WKWebViewConfiguration {
    // configuring the `WKWebView` is very important
    // without doing this the local index.html will not be able to read
    // the css or js files properly
    let config = WKWebViewConfiguration()
    config.preferences.setValue(true, forKey: "allowFileAccessFromFileURLs")
    config.setValue(true, forKey: "allowUniversalAccessFromFileURLs")
    return config
}

final class EditorWebViewContainer: ObservableObject {
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
    func emitEditorThemeEvent(theme: CodeEditorTheme) {
        print("Changing editor theme event")
        self.runJavascript(
            """
            window.localStorage.setItem("editor-theme", "\(theme.rawValue)")
            window.setEditorTheme("\(theme.rawValue)")
            """
        )
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
    func setEditorKeymap(editorKeymap: EditorKeymap) {
        print("Applying webview color scheme")
        self.runJavascript(
            """
            window.localStorage.setItem("editor-keymap", "\(editorKeymap.rawValue)")
            window.setEditorKeymap("\(editorKeymap.rawValue)")
            """
        )
    }
    func setInitialContent(note: NoteModel) {
        let body = note.markdown.body.replacingOccurrences(
            of: "`",
            with: "\\`"
        )
        self.runJavascript(
            """
            window.setEditorContent(`\(body)`)
            """
        )
    }
    func setWebviewTheme(theme: WebViewTheme) {
        self.runJavascript("""
            window.setWebviewTheme("\(theme.rawValue)")
            """)
    }
    func setInitialProperties(
        editingNote: NoteModel?,
        codeEditorTheme: CodeEditorTheme,
        editorKeymap: EditorKeymap,
        theme: WebViewTheme,
        darkMode: Bool
    ) {
        self.applyWebViewColorScheme(darkMode: darkMode)
        self.emitEditorThemeEvent(theme: codeEditorTheme)
        self.setEditorKeymap(editorKeymap: editorKeymap)
        self.setWebviewTheme(theme: theme)
        if editingNote != nil {
            self.setInitialContent(note: editingNote!)
        }
    }
}
