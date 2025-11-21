import SwiftData
import SwiftUI
import WebKit

enum CodeEditorTheme: String, Codable, CaseIterable {
    case materialLight, solarizedLight, githubLight, aura, tokyoNightDay,
        dracula, tokyoNight, materialDark, tokyoNightStorm, githubDark,
        solarizedDark, xcodeDark, xcodeLight
}

struct ResponsiveEditorWebView: UIViewRepresentable {

    let url: URL
    @State private var webView: WKWebView = WKWebView(
        frame: .zero,
        configuration: getConfig()
    )
    @Environment(\.openURL) var openURL
    @Environment(\.modelContext) var modelContext

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

        // Loading the page only once
        webView.loadFileURL(url, allowingReadAccessTo: url)

        return webView
    }

    func updateUIView(_ uiView: WKWebView, context: Context) {
    }
    func makeCoordinator() -> Coordinator {
        Coordinator(self)
    }
    func setInitialProperties() {
        container.setInitialProperties(
            editingNote: editingNote,
            codeEditorTheme: colorScheme == .dark ? editorThemeDark : editorThemeLight,
            editorKeymap: editorKeymap,
            theme: theme,
            darkMode: colorScheme == .dark
        )
    }
    func setInitialContent() {
        print("Setting initial content")
        let body = editingNote?.markdown.body.replacingOccurrences(
            of: "`",
            with: "\\`"
        )
        container.runJavascript(
            """
            window.localStorage.setItem("editor-initial-value", `\(body ?? "")`)
            window.setEditorContent(`\(body ?? "")`)
            """
        )
    }
}

extension ResponsiveEditorWebView {
    final class Coordinator: NSObject, WKNavigationDelegate,
        WKScriptMessageHandler
    {
        var parent: ResponsiveEditorWebView

        init(_ parent: ResponsiveEditorWebView) {
            self.parent = parent
        }

        func webView(_ webView: WKWebView, didFinish navigation: WKNavigation!)
        {
            guard !parent.didSetInitialContent else { return }
            parent.didSetInitialContent = true

            let body =
                parent.editingNote?.markdown.body
                .replacingOccurrences(of: "`", with: "\\`") ?? ""

            webView.evaluateJavaScript(
                """
                window.localStorage.setItem("editor-initial-value", `\(body)`);
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
