import SwiftData
import SwiftUI
import WebKit

enum CodeEditorTheme: String, Codable, CaseIterable {
    case materialLight, solarizedLight, githubLight, aura, tokyoNightDay,
        dracula, tokyoNight, materialDark, tokyoNightStorm, githubDark,
        solarizedDark, xcodeDark, xcodeLight
}

func getConfig() -> WKWebViewConfiguration {
    // configuring the `WKWebView` is very important
    // without doing this the local index.html will not be able to read
    // the css or js files properly
    let config = WKWebViewConfiguration()
    config.preferences.setValue(true, forKey: "allowFileAccessFromFileURLs")
    config.setValue(true, forKey: "allowUniversalAccessFromFileURLs")
    return config
}

struct ResponsiveEditorWebView: UIViewRepresentable {

    let url: URL
    @State private var webView: WKWebView = WKWebView(
        frame: .zero,
        configuration: getConfig()
    )
    @State private var haveSetInitialContent: Bool = false
    @Environment(\.openURL) var openURL
    @Environment(\.modelContext) var modelContext

    @Binding var theme: WebViewTheme
    @Binding var editorTheme: CodeEditorTheme
    @Binding var editingNote: NoteModel?
    @Environment(\.colorScheme) var colorScheme {
        didSet {
            applyWebViewColorScheme()
        }
    }

    func makeUIView(context: Context) -> WKWebView {
        // set the configuration on the `WKWebView`
        //        let webView = WKWebView(frame: .zero, configuration: config)
        self.webView.navigationDelegate = context.coordinator
        self.webView.scrollView.isScrollEnabled = false
        self.webView.scrollView.zoomScale = 1
        self.webView.scrollView.minimumZoomScale = 1
        self.webView.scrollView.maximumZoomScale = 1
        self.webView.allowsBackForwardNavigationGestures = false
        self.webView.configuration.userContentController.add(
            context.coordinator,
            name: "editor-update"
        )
        if #available(iOS 16.4, macOS 13.3, *) {
            self.webView.isInspectable = true  // Enable inspection
        }

        // now load the local url
        self.webView.loadFileURL(url, allowingReadAccessTo: url)
        emitEditorThemeEvent(theme: editorTheme)
        applyWebViewColorScheme()
        return self.webView
    }

    func updateUIView(_ uiView: WKWebView, context: Context) {
        //        let colorScheme = context.environment.colorScheme
        applyWebViewColorScheme()
        emitEditorThemeEvent(theme: editorTheme)
        if !haveSetInitialContent {
            print("Setting initial content")
            let body = editingNote?.markdown.body.replacingOccurrences(
                of: "`",
                with: "\\`"
            )
            self.webView.evaluateJavaScript(
                """
                window.localStorage.setItem("editor-initial-value", `\(body ?? "")`)
                """
            ) { (result, error) in
                if error != nil {
                    print("set initial value error: ", error)
                } else {
                    print("Set initial value result: ", result)
                }
            }
            haveSetInitialContent = true
        }
        uiView.loadFileURL(url, allowingReadAccessTo: url)
    }
    func makeCoordinator() -> Coordinator {
        Coordinator(self)
    }
    func emitEditorThemeEvent(theme: CodeEditorTheme) {
        print("Emitting editor theme event")
        self.webView.evaluateJavaScript(
            """
            window.dispatchEvent(new CustomEvent("set-editor-theme", {
                detail: "\(theme.rawValue)"
            }))
            """
        ) { (result, error) in
            if error != nil {
                print("Error: ", error)
            } else {
                print("Result: ", result)
            }
        }
    }
    func applyWebViewColorScheme() {
        print("Applying webview color scheme")
        self.webView.evaluateJavaScript(
            """
            window.localStorage.setItem("darkMode", "\(colorScheme == .dark ? "true" : "false")")
            """
        ) { (result, error) in
            if error != nil {
                print("Error: ", error)
            } else {
                print("Result: ", result)
            }
        }
    }
    class Coordinator: NSObject, WKNavigationDelegate, WKScriptMessageHandler {
        var parent: ResponsiveEditorWebView
        init(_ parent: ResponsiveEditorWebView) {
            self.parent = parent
        }

        // Delegate method to decide policy for navigation
        func webView(
            _ webView: WKWebView,
            decidePolicyFor navigationAction: WKNavigationAction,
            decisionHandler: @escaping (WKNavigationActionPolicy) -> Void
        ) {
            if let url = navigationAction.request.url {
                // Check if the link should open in the default browser (e.g., an external link)
                // You can add logic here to only open specific URLs externally

                // Example logic: if it's not a link to your internal website, open externally
                // if url.host != "www.myinternalwebsite.com" {

                if navigationAction.navigationType == .linkActivated
                    && url.host != webView.url?.host
                {
                    // Open the URL using the environment's openURL action
                    parent.openURL(url)

                    // Cancel the navigation within the web view
                    decisionHandler(.cancel)
                    return
                }
            }

            // Allow the navigation within the web view for other links
            decisionHandler(.allow)
        }
        func userContentController(
            _ userContentController: WKUserContentController,
            didReceive message: WKScriptMessage
        ) {
            if message.name == "editor-update" {
                if parent.editingNote != nil {
                    print("Message: \(message.body)")
                    parent.editingNote!.markdown.body = message.body as! String
                    //                       parent.modelContext.insert(parent.editingNote!)
                }
            }
        }
    }

}
