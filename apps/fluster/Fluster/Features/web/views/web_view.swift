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
    let webView: WKWebView = WKWebView(frame: .zero, configuration: getConfig())
    @Environment(\.openURL) var openURL
    @Binding var theme: WebViewTheme {
        willSet {
            newValue
            self.webView.evaluateJavaScript(
                """
                document.body.setAttribute("data-fluster-theme", "\(newValue)")
                """
            )
        }
    }
    @Environment(\.colorScheme) var colorScheme

    func makeUIView(context: Context) -> WKWebView {
        // set the configuration on the `WKWebView`
        //        let webView = WKWebView(frame: .zero, configuration: config)
        self.webView.navigationDelegate = context.coordinator
        self.webView.scrollView.isScrollEnabled = false
        self.webView.scrollView.zoomScale = 1
        self.webView.scrollView.minimumZoomScale = 1
        self.webView.scrollView.maximumZoomScale = 1
        // now load the local url
        self.webView.loadFileURL(url, allowingReadAccessTo: url)
        return self.webView
    }

    func updateUIView(_ uiView: WKWebView, context: Context) {
        uiView.loadFileURL(url, allowingReadAccessTo: url)
    }
    func makeCoordinator() -> Coordinator {
        Coordinator(self)
    }
    class Coordinator: NSObject, WKNavigationDelegate {
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
    }

}
