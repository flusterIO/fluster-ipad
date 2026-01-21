//
//  webview_container.swift
//  Fluster
//
//  Created by Andrew on 1/20/26.
//

import FlusterData
import SwiftUI
import WebKit

func getWebViewConfig() -> WKWebViewConfiguration {
  let config = WKWebViewConfiguration()
  config.preferences.setValue(true, forKey: "allowFileAccessFromFileURLs")
  config.setValue(true, forKey: "allowUniversalAccessFromFileURLs")
  return config
}

struct WebViewContainer: NSViewRepresentable {  // Use UIViewRepresentable for iOS/iPadOS
  @Binding var webView: WKWebView
  let url: URL
  var messageHandlerKeys: [String]
  var messageHandler: ((String, Any) -> Void)?
  var onLoad: (@Sendable () async -> Void)?

  typealias ViewType = WKWebView
  func makeNSView(context: Context) -> WKWebView { makeWebView(context: context) }
  func updateNSView(_ nsView: WKWebView, context: Context) {}

  private func makeWebView(context: Context) -> WKWebView {
    webView.isHidden = true
    let config = WKWebViewConfiguration()
    // 1. Register the handler name here
    for handler in messageHandlerKeys {
      config.userContentController.add(context.coordinator, name: handler)
    }

    DispatchQueue.main.async {
      if let scrollView = webView.enclosingScrollView {
        scrollView.horizontalScrollElasticity = .none
        scrollView.verticalScrollElasticity = .none
      }
    }
    //        webView.scroll
    webView.navigationDelegate = context.coordinator
    webView.load(URLRequest(url: url))
    return webView
  }

  func makeCoordinator() -> Coordinator {
    Coordinator(self)
  }

  class Coordinator: NSObject, WKScriptMessageHandler, WKNavigationDelegate {
    let parent: WebViewContainer
    init(_ parent: WebViewContainer) {
      self.parent = parent
    }

    // 2. This function catches the JS "postMessage"
    func userContentController(
      _ userContentController: WKUserContentController, didReceive message: WKScriptMessage
    ) {
      if let messageHandler = parent.messageHandler {
        for handler in parent.messageHandlerKeys {
          if message.name == handler {
            messageHandler(handler, message.body)
          }
        }
      } else {
        print("No message handler found.")
        return
      }
    }
    func webView(
      _ webView: WKWebView, didFailProvisionalNavigation navigation: WKNavigation!,
      withError error: Error
    ) {
      print("Navigation failed early: \(error.localizedDescription)")
    }
    func webView(_ webView: WKWebView, didFinish navigation: WKNavigation!) {
      print("Web content loaded...")
      if let onLoad = parent.onLoad {
        Task {
          await onLoad()
        }
      }
      parent.webView.isHidden = false
    }
  }
}

struct WebViewContainerView: View {
  @Binding var webview: WKWebView
  public let url: URL
  public let messageHandlerKeys: [String]
  public let messageHandler: ((String, Any) -> Void)?
  public let onLoad: (@Sendable () async -> Void)?

  @State private var show: Bool = false
  @AppStorage(AppStorageKeys.theme.rawValue) private var theme: WebViewTheme = .fluster
  @Environment(\.colorScheme) private var colorScheme: ColorScheme
  var body: some View {
    ZStack {
      WebViewContainer(
        webView: $webview, url: url, messageHandlerKeys: messageHandlerKeys,
        messageHandler: messageHandler,
        onLoad: onLoadHandler
      )
      if !show {
        ProgressView("Loading...")
          .tint(Color.accent)
          .foregroundStyle(.foreground)
          .progressViewStyle(.circular)
      }
    }
    .onChange(
      of: colorScheme,
      {
        Task {
          await setColorScheme(colorScheme: colorScheme)
        }
      }
    )
    .scrollBounceBehavior(.basedOnSize, axes: [])
  }
  func onLoadHandler() async {
    await setColorScheme(colorScheme: colorScheme)
    if let additionalOnLoad = onLoad {
      await additionalOnLoad()
    }
    show = true
  }
  func runJavascript(_ js: String) async throws {
    do {
      try await webview.evaluateJavaScript(js)
    } catch {
      print("Error: \(error.localizedDescription)")
    }
  }
  func setColorScheme(colorScheme: ColorScheme) async {
    do {
      if colorScheme == .dark {
        try await runJavascript(
          """
          document.body.classList.add("dark")
          """)
      } else {
        try await runJavascript(
          """
          document.body.classList.remove("dark")
          """)
      }
    } catch {
      print("Error setting color scheme: \(error.localizedDescription)")
    }
  }
}
