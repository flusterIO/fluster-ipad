import Combine
import SwiftUI
import WebKit

@MainActor func getConfig() -> WKWebViewConfiguration {
    let config = WKWebViewConfiguration()
    config.preferences.setValue(true, forKey: "allowFileAccessFromFileURLs")
    config.setValue(true, forKey: "allowUniversalAccessFromFileURLs")
    return config
}

//typealias AnyWebviewAction = NoteDetailWebviewActions | SplitviewEditorWebviewActions
@MainActor
public class WebviewContainer<WebviewEventsType: RawRepresentable>:
    ObservableObject
where WebviewEventsType.RawValue == String {
    public let scrollViewBounce: Bool = true
    public let scrollEnabled: Bool = false
    let webView: WKWebView = {
        let view = WKWebView(frame: .zero, configuration: getConfig())
        view.isHidden = true
        view.scrollView.bouncesZoom = false  // Disable zoom bounce
        view.scrollView.minimumZoomScale = 1.0  // Prevent zooming out
        view.scrollView.maximumZoomScale = 1.0
        view.allowsBackForwardNavigationGestures = false
        // At best, this will only work for the editor.
        if let screen = UIScreen.current {
            view.frame = CGRect(
                x: 0,
                y: 0,
                width: screen.bounds.width,
                height: screen.bounds.height
            )
        }
        if #available(iOS 16.4, macOS 13.3, *) {
            view.isInspectable = true
        }

        return view
    }()

    public init(bounce: Bool = false, scrollEnabled: Bool = false) {
        self.webView.scrollView.bounces = bounce
        self.webView.scrollView.isScrollEnabled = scrollEnabled
        //        self._colorScheme = colorScheme
    }

    public func runJavascript(_ script: String) {
        self.webView.evaluateJavaScript(script) { result, error in
            if let error = error {
                print("Error executing JS: \(error.localizedDescription)")
                print("Command: \(script)")
            } else if let result = result {
                print("JS Result: \(result)")
            }
        }
    }

    public func sendEvent(_ event: WebviewEventsType, eventDetail: String = "")
    {
        self.runJavascript(
            """
            window?.dispatchEvent(new CustomEvent("\(event.rawValue)", {
                    detail: \(eventDetail.toQuotedJavascriptString())
                }))
            """
        )
    }
    /// A utility function used to append some initial styles to the window before loading the webview. Not sure if this will even work...
    public func preShow(colorScheme: ColorScheme) {
        self.runJavascript(
            """
            document.body.classList.add('\(colorScheme == .dark ? "dark" : "light")')
            """
        )
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
        self.runJavascript(
            """
            window.localStorage.setItem("dark-mode", "\(darkMode ? "true" : "false")")
            window.setDarkMode(\(darkMode ? "true" : "false"))
            """
        )
    }

    public func requestDocumentSize() {
        self.runJavascript(
            """
            window.requestDocumentSize()
            """
        )
    }
    public func addClassToWebviewContainer(className: String) {
        self.runJavascript(
            """
            document.getElementById("webview-container")?.classList.add("\(className)")
            """
        )
    }
    public func removeClassFromWebviewContainer(className: String) {
        self.runJavascript(
            """
            document.getElementById("webview-container")?.classList.remove("\(className)")
            """
        )
    }

    public func setLoading(isLoading: Bool) {
        if isLoading {
            self.addClassToWebviewContainer(className: "loading")
        } else {
            self.removeClassFromWebviewContainer(className: "loading")
        }
    }

    public func sendScreenSize() {
        let width = UIScreen.current?.bounds.width
        let height = UIScreen.current?.bounds.height
        if width != nil && height != nil {
            self.runJavascript(
                """
                window.setScreenSize(\(width), \(height))
                """
            )
        }
    }
}
