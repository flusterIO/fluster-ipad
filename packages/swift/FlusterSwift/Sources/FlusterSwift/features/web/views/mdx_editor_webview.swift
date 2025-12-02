import SwiftData
import SwiftUI
import WebKit

public enum CodeSyntaxTheme: String, Codable, CaseIterable {
    case materialLight, solarizedLight, githubLight, aura, tokyoNightDay,
        dracula, tokyoNight, materialDark, tokyoNightStorm, githubDark,
        solarizedDark, xcodeDark, xcodeLight
}

public struct MdxEditorWebview: UIViewRepresentable {

    @State private var webView: WKWebView = WKWebView(
        frame: .zero,
        configuration: getConfig()
    )
    @State private var didSetInitialContent = false
    @State var haveSetInitialContent: Bool = false
    @Environment(\.openURL) var openURL
    @Environment(\.modelContext) var modelContext
    @Environment(\.colorScheme) var colorScheme
    @AppStorage(AppStorageKeys.webviewFontSize.rawValue) private
        var webviewFontSize: WebviewFontSize = .base
    let url: URL
    @Binding var theme: WebViewTheme
    @Binding var editorThemeDark: CodeSyntaxTheme
    @Binding var editorThemeLight: CodeSyntaxTheme
    @Binding var editingNote: NoteModel?
    @Binding var editorKeymap: EditorKeymap
    

    let container: MdxEditorWebviewContainer

    public init(
        url: URL,
        theme: Binding<WebViewTheme>,
        editorThemeDark: Binding<CodeSyntaxTheme>,
        editorThemeLight: Binding<CodeSyntaxTheme>,
        editingNote: Binding<NoteModel?>,
        editorKeymap: Binding<EditorKeymap>,
        container: MdxEditorWebviewContainer,
    ) {
        self.url = url
        self._theme = theme
        self._editorThemeDark = editorThemeDark
        self._editorThemeLight = editorThemeLight
        self._editingNote = editingNote
        self._editorKeymap = editorKeymap
        self.container = container
    }

    public func makeUIView(context: Context) -> WKWebView {
        let webView = container.webView

        webView.navigationDelegate = context.coordinator
        let editorContentControllers = [
            "editor-update",
            "request-initial-editor-data",
//            "set-editor-viewport-height",
        ]
        for controllerName in editorContentControllers {
            addUserContentController(
                controller: webView.configuration.userContentController,
                coordinator: context.coordinator,
                name: controllerName
            )
        }

        // Loading the page only once
        webView.loadFileURL(url, allowingReadAccessTo: url)

        return webView
    }

    public func updateUIView(_ uiView: WKWebView, context: Context) {
        //        uiView.scrollView.contentInset = .zero
        //        uiView.scrollView.scrollIndicatorInsets = .zero
    }
    public func makeCoordinator() -> Coordinator {
        Coordinator(self)
    }
    public func setInitialProperties() {
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
    public func setInitialContent() {
        let s = editingNote?.markdown.body.toQuotedJavascriptString() ?? "''"
        container.runJavascript(
            """
            window.localStorage.setItem("editor-initial-value", \(s))
            window.setEditorContent(\(s))
            """
        )
    }
}

extension MdxEditorWebview {
    public final class Coordinator: NSObject, WKNavigationDelegate,
        WKScriptMessageHandler
    {
        var parent: MdxEditorWebview

        init(_ parent: MdxEditorWebview) {
            self.parent = parent
        }

        public func webView(
            _ webView: WKWebView,
            didFinish navigation: WKNavigation!
        ) {
            // On Load
            guard !parent.didSetInitialContent else { return }

            webView.evaluateJavaScript(
                """
                window.localStorage.setItem("editor-initial-value", \(parent.editingNote?.markdown.body.toQuotedJavascriptString() ?? "''"));
                """
            )
            parent.setInitialProperties()
            parent.container.webView.isHidden = false
//            parent.container.requestDocumentSize()
            parent.didSetInitialContent = true
        }
//        public func webView(
//            _ webView: WKWebView,
//            didStartProvisionalNavigation navigation: WKNavigation!
//        ) {
//            parent.isLoading = true  // Set loading to true when navigation starts
//        }

//        public func webView(_ webView: WKWebView, didFinish navigation: WKNavigation!)
//        {
//        }

        public func webView(
            _ webView: WKWebView,
            didFail navigation: WKNavigation!,
            withError error: Error
        ) {
            print(
                "WebView navigation failed with error: \(error.localizedDescription)"
            )
        }
        public func userContentController(
            _ userContentController: WKUserContentController,
            didReceive message: WKScriptMessage
        ) {
//            if message.name == "set-editor-viewport-height" {
//                print("Message body: \(message.body)")
//                if let n = NumberFormatter().number(
//                    from: message.body as! String
//                ) {
//                    print("N: \(n)")
//                    parent.viewportHeight = CGFloat(truncating: n)
//                }
//            } else
            if message.name == "editor-update",
                let str = message.body as? String
            {
                parent.editingNote?.markdown.body = str
            } else if message.name == "request-initial-editor-data" {
                print("Request for initial editor data received...")
                parent.setInitialProperties()
                parent.setInitialContent()
            }
        }
    }
}
