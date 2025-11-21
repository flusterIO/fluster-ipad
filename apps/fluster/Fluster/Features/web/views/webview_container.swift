import WebKit
import Combine

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
        return view
    }()
}
