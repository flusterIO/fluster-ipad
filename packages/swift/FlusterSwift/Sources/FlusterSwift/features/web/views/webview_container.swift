import Combine
import FlusterData
import FlusterWebviewClients
import SwiftUI
import WebKit

#if os(iOS)
  @MainActor func getConfig() -> WKWebViewConfiguration {
    let config = WKWebViewConfiguration()
    config.preferences.setValue(true, forKey: "allowFileAccessFromFileURLs")
    config.setValue(true, forKey: "allowUniversalAccessFromFileURLs")
    //    config.preferences.setValue(true, forKey: "")
    return config
  }

  //typealias AnyWebviewAction = NoteDetailWebviewActions | SplitviewEditorWebviewActions
  @MainActor
  public class WebviewContainer<WebviewEventsType: RawRepresentable>:
    NSObject, ObservableObject, WKNavigationDelegate  // 1. Inherit NSObject & Delegate
  where WebviewEventsType.RawValue == String {
    public let scrollViewBounce: Bool = true
    public let scrollEnabled: Bool = false
    public var onLoad: (@Sendable (WKWebView) async -> Void)?
    lazy var webView: WKWebView = {
      let config = getConfig()
      let view = WKWebView(frame: .zero, configuration: config)
      view.isHidden = true
      view.scrollView.bouncesZoom = false  // Disable zoom bounce
      view.scrollView.minimumZoomScale = 1.0  // Prevent zooming out
      view.scrollView.maximumZoomScale = 1.0
      let source = """
        document.body?.classList.add("\(WebviewEnvironment.iPad.rawValue)")
        """
      view.allowsBackForwardNavigationGestures = false
      let webviewEnvironmentScript = WKUserScript(
        source: source,
        injectionTime: .atDocumentEnd,
        forMainFrameOnly: true
      )

      view.configuration.userContentController.addUserScript(webviewEnvironmentScript)
      view.navigationDelegate = self
      if #available(iOS 16.4, macOS 13.3, *) {
        view.isInspectable = true
      }

      return view
    }()  // Required to be iffe instead of computed property to maintain state.

    public init(
      bounce: Bool = false,
      scrollEnabled: Bool = false,
      onLoad: (@Sendable (WKWebView) async -> Void)?
    ) {
      self.onLoad = onLoad
      super.init()
      self.webView.scrollView.bounces = bounce
      self.webView.scrollView.isScrollEnabled = scrollEnabled
    }

    public func runJavascript(_ script: String) {
      self.webView.evaluateJavaScript(script) { result, error in
        if let error = error {
          print("--- Error ---")
          print("Error executing JS: \(error.localizedDescription)")
            print("Command: \(script.trunc(length: 200))")
          print("-------------")
        } else if let result = result {
          print("JS Result: \(result)")
        }
      }
    }

    public func sendEvent(_ event: WebviewEventsType, eventDetail: String = "") {
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
        document.body.classList.add('\(colorScheme == .dark ? "dark" : "light")'); null;
        """
      )
    }
    public func setWebviewTheme(theme: WebViewTheme) {
      self.runJavascript(
        """
        window.setWebviewTheme("\(theme.rawValue)"); null;
        """
      )
    }
    public func setWebviewFontSize(fontSize: WebviewFontSize) {
      self.runJavascript(
        """
        window.setWebViewFontSize('\(fontSize.cssClass())'); null;
        """
      )
    }
    public func applyWebViewColorScheme(darkMode: Bool) {
      self.runJavascript(
        """
        window.setDarkMode(\(darkMode ? "true" : "false")); null;
        """
      )
    }
    public func resetScrollPosition(
      containerId: String = "mdx-preview",
      scrollStorageKeys: [String]
    ) {
      var s = ""
      for (i, scrollKey) in scrollStorageKeys.enumerated() {
        s = s + "\"\(scrollKey)\"\(i == scrollStorageKeys.count - 1 ? "" : ",")"
      }
      self.runJavascript(
        """
        window.resetMdxPreviewScrollPosition("\(containerId)", [\(s)])
        """)
    }

    public func requestDocumentSize() {
      self.runJavascript(
        """
        window.requestDocumentSize(); null;
        """
      )
    }
    public func addClassToWebviewContainer(className: String) {
        Task(priority: .high) {
          do {
            try await WebviewContainerClient.setClassToWebviewContainer(
              className: className, method: .append, evaluateJavaScript: webView.evaluateJavaScript)
          } catch {
            print("Error: \(error.localizedDescription)")
          }
        }
    }
    public func removeClassFromWebviewContainer(className: String) {
      Task(priority: .high) {
        do {
          try await WebviewContainerClient.setClassToWebviewContainer(
            className: className, method: .remove, evaluateJavaScript: webView.evaluateJavaScript)
        } catch {
          print("Error: \(error.localizedDescription)")
        }
      }
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
          window.setScreenSize(\(width), \(height)); null;
          """
        )
      }
    }

    // Delegate Methods

    // onLoad
    public func webView(_ webView: WKWebView, didFinish navigation: WKNavigation!) {
      setLoading(isLoading: false)
      if let ol = self.onLoad {
        Task(priority: .high) {
          await ol(webView)
        }
      }
    }
  }
#endif
