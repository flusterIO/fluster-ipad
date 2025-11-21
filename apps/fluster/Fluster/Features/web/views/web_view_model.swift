import SwiftUI
import WebKit
import Combine


class WebViewModel: ObservableObject {
    
    let webView: WKWebView
    
    @Published var urlString: String = ""
    
    init() {
        let webView = WKWebView()
        webView.scrollView.isScrollEnabled = false
        webView.scrollView.zoomScale = 1
        webView.scrollView.minimumZoomScale = 1
        webView.scrollView.maximumZoomScale = 1
        webView.allowsBackForwardNavigationGestures = false
        if #available(iOS 16.4, macOS 13.3, *) {
            webView.isInspectable = true  // Enable inspection
        }
        self.webView = webView
    }

    func loadUrl(urlString: String) {
        guard let url = URL(string: urlString) else { return }
        webView.load(URLRequest(url: url))
    }

    func runJavascript(_ script: String) {
        webView.evaluateJavaScript(script) { result, error in
            if let error = error {
                print("Error executing JS: \(error.localizedDescription)")
            } else if let result = result {
                print("JS Result: \(result)")
            }
        }
    }
}
