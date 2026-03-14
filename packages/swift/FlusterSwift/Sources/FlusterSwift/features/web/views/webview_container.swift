import Combine
import FlusterData
import FlusterWebviewClients
import SwiftData
import SwiftUI
import WebKit

#if os(iOS)
  @MainActor public func getConfig() -> WKWebViewConfiguration {
    let config = WebContextManager.createSharedConfiguration()
    config.setURLSchemeHandler(WasmSchemeHandler(), forURLScheme: "app")
    config.setValue(true, forKey: "allowUniversalAccessFromFileURLs")
    config.preferences.setValue(true, forKey: "allowFileAccessFromFileURLs")
    //    config.preferences.setValue(true, forKey: "")
    return config
  }

  //typealias AnyWebviewAction = NoteDetailWebviewActions | SplitviewEditorWebviewActions
  public struct WebviewContainer:
    UIViewRepresentable
  {
    public typealias UIViewType = WKWebView

    let parent: IosWebviewContainer
    @Environment(\.openURL) public var openUrl: OpenURLAction
    @Binding var webView: WKWebView
    @Binding var show: Bool
    @Environment(\.colorScheme) private var colorScheme: ColorScheme
    let url: URL
    var messageHandlerKeys: [String]
    var messageHandler: ((String, Any) -> Void)?
    var onLoad: (@Sendable () async -> Void)?

    init(
      parent: IosWebviewContainer,
      webView: Binding<WKWebView>,
      show: Binding<Bool>,
      url: URL,
      messageHandlerKeys: [String],
      messageHandler: ((String, Any) -> Void)? = nil,
      onLoad: (@Sendable () async -> Void)? = nil
    ) {
      self.parent = parent
      self._webView = webView
      self.url = url
      self._show = show
      self.messageHandlerKeys = messageHandlerKeys
      self.messageHandler = messageHandler
      self.onLoad = onLoad
    }

    public func makeUIView(context: Context) -> WKWebView {
      webView.isHidden = true
      webView.backgroundColor = .clear
      webView.underPageBackgroundColor = .clear
      webView.layer.backgroundColor =
        colorScheme == .dark ? UIColor.black.cgColor : UIColor.white.cgColor
      if colorScheme == .dark {
        addDarkModeScript(controller: webView.configuration.userContentController)
      }
      addContentControllerList(
        items: messageHandlerKeys,
        controller: webView.configuration.userContentController,
        coordinator: context.coordinator
      )
      addUserContentController(
        controller: webView.configuration.userContentController, coordinator: context.coordinator,
        name: WebviewContainerEvents.reduxStateLoaded.rawValue)

      webView.scrollView.bounces = false
      webView.scrollView.isScrollEnabled = false
      webView.scrollView.showsVerticalScrollIndicator = false
      webView.scrollView.showsHorizontalScrollIndicator = false
      webView.navigationDelegate = context.coordinator
      webView.uiDelegate = context.coordinator
      webView.isInspectable = true
      let source = """
        document.body?.classList.add("\(WebviewEnvironment.macOS.rawValue)")
        """
      let webviewEnvironmentScript = WKUserScript(
        source: source,
        injectionTime: .atDocumentEnd,
        forMainFrameOnly: true
      )
      webView.configuration.userContentController.addUserScript(webviewEnvironmentScript)
      let request = URLRequest(url: url)
      webView.load(request)
      return webView
    }

    public func updateUIView(_ uiView: WKWebView, context: Context) {
      context.coordinator.parent = self
      if uiView.navigationDelegate == nil {
        uiView.navigationDelegate = context.coordinator
      }
      if uiView.uiDelegate == nil {
        uiView.uiDelegate = context.coordinator
      }
    }
    @MainActor
    func addUserContentController(
      controller: WKUserContentController, coordinator: WKScriptMessageHandler, name: String
    ) {
      controller.removeScriptMessageHandler(forName: name)
      controller.add(LeakFreeScriptHandler(coordinator), name: name)
    }

    @MainActor
    func addContentControllerList(
      items: [String], controller: WKUserContentController, coordinator: WKScriptMessageHandler
    ) {
      for item in items {
        addUserContentController(controller: controller, coordinator: coordinator, name: item)
      }
    }

    func addDarkModeScript(controller: WKUserContentController) {
      let source = """
        document.body?.classList.add("dark")
        """

      // 2. Create the script to run at the very start (atDocumentStart)
      let userScript = WKUserScript(
        source: source,
        injectionTime: .atDocumentEnd,
        forMainFrameOnly: true
      )
      controller.addUserScript(userScript)
    }

    public func makeCoordinator() -> Coordinator {
      Coordinator(parent: self)
    }

    public class Coordinator: NSObject, WKScriptMessageHandler, WKNavigationDelegate, WKUIDelegate {
      var parent: WebviewContainer

      public init(parent: WebviewContainer) {
        self.parent = parent
      }

      // This is where JavaScript messages arrive
      public func userContentController(
        _ userContentController: WKUserContentController, didReceive message: WKScriptMessage
      ) {
        print("Received Message: ", message.name)
        if message.name == WebviewContainerEvents.reduxStateLoaded.rawValue {
          self.parent.show = true
          self.parent.webView.isHidden = false
          if let onLoad = self.parent.onLoad {
            Task(priority: .high) {
              await onLoad()
            }
          }
        }

        // Route the message to your container's messageHandler
        parent.messageHandler?(message.name, message.body)
      }

      public func webView(
        _ webView: WKWebView,
        decidePolicyFor navigationAction: WKNavigationAction,
        decisionHandler: @escaping @MainActor (WKNavigationActionPolicy) -> Void
      ) {
        print("Navigation Action: \(navigationAction)")
        // Check if the navigation was triggered by a user clicking a link
        if navigationAction.navigationType == .linkActivated {
          if let url = navigationAction.request.url {
            // Open the URL in the system default browser
            self.parent.openUrl(url, prefersInApp: false)

            // Cancel the navigation within the webview
            decisionHandler(.cancel)
            return
          }
        }

        // Allow other types of navigation (like the initial load)
        decisionHandler(.allow)
      }

      public func webView(
        _ webView: WKWebView,
        createWebViewWith configuration: WKWebViewConfiguration,
        for navigationAction: WKNavigationAction,
        windowFeatures: WKWindowFeatures
      ) -> WKWebView? {
        // Intercept target="_blank"
        if let url = navigationAction.request.url {
          self.parent.openUrl(url)
        }
        return nil
      }
    }
  }
  class LeakFreeScriptHandler: NSObject, WKScriptMessageHandler {
    weak var delegate: WKScriptMessageHandler?

    init(_ delegate: WKScriptMessageHandler) {
      self.delegate = delegate
    }

    func userContentController(
      _ userContentController: WKUserContentController, didReceive message: WKScriptMessage
    ) {
      delegate?.userContentController(userContentController, didReceive: message)
    }
  }
#endif
