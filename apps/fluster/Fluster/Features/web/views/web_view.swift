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
    @Binding var editorTheme: CodeEditorTheme
    @Binding var editingNote: NoteModel?
    @Binding var editorKeymap: EditorKeymap

    let container: EditorWebViewContainer
    @Environment(\.colorScheme) var colorScheme {
        didSet {
            setInitialContent()
        }
    }

    func makeUIView(context: Context) -> WKWebView {
        let webView = container.webView

        webView.navigationDelegate = context.coordinator
        webView.configuration.userContentController.add(
            context.coordinator,
            name: "editor-update"
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
            parent.container.setInitialProperties(
                editingNote: parent.editingNote,
                codeEditorTheme: parent.editorTheme,
                editorKeymap: parent.editorKeymap,
                theme: parent.theme,
                darkMode: parent.colorScheme == .dark
            )
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
        }
    }
}
