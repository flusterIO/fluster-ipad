import SwiftData
import SwiftUI
import WebKit
import FlusterSwift

enum CodeEditorTheme: String, Codable, CaseIterable {
    case materialLight, solarizedLight, githubLight, aura, tokyoNightDay,
        dracula, tokyoNight, materialDark, tokyoNightStorm, githubDark,
        solarizedDark, xcodeDark, xcodeLight
}

struct WebViewWrapper: UIViewRepresentable {

    let url: URL
    @State private var webView: WKWebView = WKWebView(
        frame: .zero,
        configuration: getConfig()
    )
    @Environment(\.openURL) var openURL
    @Environment(\.modelContext) var modelContext
    @AppStorage(AppStorageKeys.webviewFontSize.rawValue) private
        var webviewFontSize: WebviewFontSize = .base
    @Binding var theme: WebViewTheme
    @State private var didSetInitialContent = false
    @State var haveSetInitialContent: Bool = false
    @Binding var editorThemeDark: CodeEditorTheme
    @Binding var editorThemeLight: CodeEditorTheme
    @Binding var editingNote: NoteModel?
    @Binding var editorKeymap: EditorKeymap

    let container: EditorWebViewContainer
    @Environment(\.colorScheme) var colorScheme

    func makeUIView(context: Context) -> WKWebView {
        let webView = container.webView

        webView.navigationDelegate = context.coordinator
        webView.configuration.userContentController.add(
            context.coordinator,
            name: "editor-update"
        )
        webView.configuration.userContentController.add(
            context.coordinator,
            name: "request-initial-data"
        )

//        webView.scrollView.contentInsetAdjustmentBehavior = .never

        // Loading the page only once
        webView.loadFileURL(url, allowingReadAccessTo: url)

        return webView
    }

    func updateUIView(_ uiView: WKWebView, context: Context) {
//        uiView.scrollView.contentInset = .zero
//        uiView.scrollView.scrollIndicatorInsets = .zero
    }
    func makeCoordinator() -> Coordinator {
        Coordinator(self)
    }
    func setInitialProperties() {
        container.setInitialProperties(
            editingNote: editingNote,
            codeEditorTheme: colorScheme == .dark
                ? editorThemeDark : editorThemeLight,
            editorKeymap: editorKeymap,
            theme: theme,
            fontSize: webviewFontSize,
            editorThemeDark: editorThemeDark,
            editorThemeLight: editorThemeLight,
            darkMode: colorScheme == .dark
        )
    }
    func setInitialContent() {
        print("Setting initial content")
        let s = editingNote?.markdown.body.toQuotedJavascriptString() ?? "''"
        container.runJavascript(
            """
            window.localStorage.setItem("editor-initial-value", \(s))
            window.setEditorContent(\(s))
            """
        )
    }
}

extension WebViewWrapper {
    final class Coordinator: NSObject, WKNavigationDelegate,
        WKScriptMessageHandler
    {
        var parent: WebViewWrapper

        init(_ parent: WebViewWrapper) {
            self.parent = parent
        }

        func webView(_ webView: WKWebView, didFinish navigation: WKNavigation!)
        {
            guard !parent.didSetInitialContent else { return }
            parent.didSetInitialContent = true

            webView.evaluateJavaScript(
                """
                window.localStorage.setItem("editor-initial-value", \(parent.editingNote?.markdown.body.toQuotedJavascriptString() ?? "''"));
                """
            )
            parent.setInitialProperties()
            parent.container.webView.isHidden = false
        }

        func userContentController(
            _ userContentController: WKUserContentController,
            didReceive message: WKScriptMessage
        ) {
            if message.name == "editor-update",
                let str = message.body as? String
            {
                parent.editingNote?.markdown.body = str
            }
            if message.name == "request-initial-data" {
                print("Request for initial editor data received...")
                parent.setInitialProperties()
                parent.setInitialContent()
            }
        }
    }
}
