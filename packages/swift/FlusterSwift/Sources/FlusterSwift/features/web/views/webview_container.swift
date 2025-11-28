import Combine
import SwiftUI
import WebKit

@MainActor func getConfig() -> WKWebViewConfiguration {
    // configuring the `WKWebView` is very important
    // without doing this the local index.html will not be able to read
    // the css or js files properly
    let config = WKWebViewConfiguration()
    config.preferences.setValue(true, forKey: "allowFileAccessFromFileURLs")
    config.setValue(true, forKey: "allowUniversalAccessFromFileURLs")
    return config
}

@MainActor
public class WebviewContainer: ObservableObject {
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

    public init() {}
    
    public func runJavascript(_ script: String) {
        self.webView.evaluateJavaScript(script) { result, error in
            if let error = error {
                print("Error executing JS: \(error.localizedDescription)")
            } else if let result = result {
                print("JS Result: \(result)")
            }
        }
    }
    public func setWebviewTheme(theme: WebViewTheme) {
        self.runJavascript(
            """
            window.setWebviewTheme("\(theme.rawValue)")
            """
        )
    }
    public func setWebviewFontSize(fontSize: WebviewFontSize) {
        self.runJavascript(
            """
            window.localStorage.setItem("webview-font-class", "\(fontSize.cssClass())");
            window.setWebViewFontSize('\(fontSize.cssClass())');
            """
        )
    }
    public func applyWebViewColorScheme(darkMode: Bool) {
        print("Applying webview color scheme")
        self.runJavascript(
            """
            window.localStorage.setItem("dark-mode", "\(darkMode ? "true" : "false")")
            window.setDarkMode(\(darkMode ? "true" : "false"))
            """
        )
    }
}
