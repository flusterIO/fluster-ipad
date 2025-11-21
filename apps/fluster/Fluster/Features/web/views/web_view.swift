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
    @Binding var editorTheme: CodeEditorTheme
    @Binding var editingNote: NoteModel? {
        willSet {
            didSetInitialContent = false
        }
        didSet {
            setInitialContent()
        }
    }
    @State var haveSetInitialContent: Bool = false
    @StateObject var viewModel: WebViewModel = WebViewModel()

    let container: EditorWebViewContainer
    @Environment(\.colorScheme) var colorScheme {
        didSet {
            applyWebViewColorScheme()
        }
    }

    func makeUIView(context: Context) -> WKWebView {
        let webView = container.webView

        webView.navigationDelegate = context.coordinator
        webView.configuration.userContentController.add(context.coordinator, name: "editor-update")

        // Loading the page only once
        webView.loadFileURL(url, allowingReadAccessTo: url)

        return webView
    }

    func updateUIView(_ uiView: WKWebView, context: Context) {
        // if !haveSetInitialContent {
        //            applyWebViewColorScheme()
        //            emitEditorThemeEvent(theme: editorTheme)
        //            setInitialContent()
        // }
        // uiView.loadFileURL(url, allowingReadAccessTo: url)
    }
    func makeCoordinator() -> Coordinator {
        Coordinator(self)
    }
    func setInitialContent() {
        print("Attempting to set initial content")
        if !haveSetInitialContent {
            print("Setting initial content")
            let body = editingNote?.markdown.body.replacingOccurrences(
                of: "`",
                with: "\\`"
            )
            self.viewModel.runJavascript(
                """
                window.localStorage.setItem("editor-initial-value", `\(body ?? "")`)
                """)
        }
    }
    func emitEditorThemeEvent(theme: CodeEditorTheme) {
        print("Changing editor theme event")
        self.viewModel.runJavascript(
            """
            window.localStorage.setItem("editor-theme", "\(theme.rawValue)")
            """
        )
    }
    func applyWebViewColorScheme() {
        print("Applying webview color scheme")
        self.viewModel.runJavascript(
            """
            window.localStorage.setItem("darkMode", "\(colorScheme == .dark ? "true" : "false")")
            """
        )
    }
    // class Coordinator: NSObject, WKNavigationDelegate, WKScriptMessageHandler {
    //     var parent: ResponsiveEditorWebView
    //     init(_ parent: ResponsiveEditorWebView) {
    //         self.parent = parent
    //     }

    //     // Delegate method to decide policy for navigation
    //     func webView(
    //         _ webView: WKWebView,
    //         decidePolicyFor navigationAction: WKNavigationAction,
    //         decisionHandler: @escaping (WKNavigationActionPolicy) -> Void
    //     ) {
    //         if let url = navigationAction.request.url {
    //             // Check if the link should open in the default browser (e.g., an external link)
    //             // You can add logic here to only open specific URLs externally

    //             // Example logic: if it's not a link to your internal website, open externally
    //             // if url.host != "www.myinternalwebsite.com" {

    //             if navigationAction.navigationType == .linkActivated
    //                 && url.host != webView.url?.host
    //             {
    //                 // Open the URL using the environment's openURL action
    //                 parent.openURL(url)

    //                 // Cancel the navigation within the web view
    //                 decisionHandler(.cancel)
    //                 return
    //             }
    //         }

    //         // Allow the navigation within the web view for other links
    //         decisionHandler(.allow)
    //     }
    //     func userContentController(
    //         _ userContentController: WKUserContentController,
    //         didReceive message: WKScriptMessage
    //     ) {
    //         if message.name == "editor-update" {
    //             if parent.editingNote != nil {
    //                 print("Message: \(message.body)")
    //                 parent.editingNote!.markdown.body = message.body as! String
    //             }
    //         }
    //     }
    // }

}

extension ResponsiveEditorWebView {
    final class Coordinator: NSObject, WKNavigationDelegate, WKScriptMessageHandler {
        var parent: ResponsiveEditorWebView

        init(_ parent: ResponsiveEditorWebView) {
            self.parent = parent
        }

        func webView(_ webView: WKWebView, didFinish navigation: WKNavigation!) {
            guard !parent.didSetInitialContent else { return }
            parent.didSetInitialContent = true

            let body =
                parent.editingNote?.markdown.body
                .replacingOccurrences(of: "`", with: "\\`") ?? ""

            webView.evaluateJavaScript(
                """
                window.localStorage.setItem("editor-initial-value", `\(body)`);
                """)
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
