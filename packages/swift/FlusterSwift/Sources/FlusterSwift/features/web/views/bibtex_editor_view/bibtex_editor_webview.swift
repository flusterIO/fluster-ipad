//
//  bibtex_editor_webview.swift
//  Fluster
//
//  Created by Andrew on 11/22/25.
//

import SwiftUI
import WebKit
import FlusterData

public struct BibtexEditorWebview: UIViewRepresentable {
  let url: URL = Bundle.main.url(
    forResource: "index",
    withExtension: "html",
    subdirectory: "bibtex_editor_webview"
  )!
  @State private var webView: WKWebView = WKWebView(
    frame: .zero,
    configuration: getConfig()
  )
  @State private var show: Bool = false
  @Environment(\.openURL) var openURL
  @Environment(\.modelContext) var modelContext
  @Environment(\.colorScheme) var colorScheme
  @AppStorage(AppStorageKeys.webviewFontSize.rawValue) private
    var webviewFontSize: WebviewFontSize = .base
  @AppStorage(AppStorageKeys.theme.rawValue) private var theme: WebViewTheme =
    .fluster
  @State private var didSetInitialContent = false
  @State var haveSetInitialContent: Bool = false
  @AppStorage(AppStorageKeys.editorThemeDark.rawValue) private
    var editorThemeDark: CodeSyntaxTheme = .dracula
  @AppStorage(AppStorageKeys.editorThemeLight.rawValue) private
    var editorThemeLight: CodeSyntaxTheme = .githubLight
  @AppStorage(AppStorageKeys.editorKeymap.rawValue) private var editorKeymap: EditorKeymap = .base
  @Binding var value: String
  let container: BibtexEditorWebviewContainer

  public init(value: Binding<String>, container: BibtexEditorWebviewContainer) {
    self._value = value
    self.container = container
  }

  public func makeUIView(context: Context) -> WKWebView {
    let webView = container.webView

    webView.navigationDelegate = context.coordinator
    webView.isHidden = true
    let controllers = [
      BibtexEditorWebviewActions.onEditorChange.rawValue,
      BibtexEditorWebviewActions.requestBibtexEditorData.rawValue,
      BibtexEditorWebviewActions.setWebviewLoaded.rawValue
    ]

    for controllerName in controllers {
      addUserContentController(
        controller: webView.configuration.userContentController,
        coordinator: context.coordinator,
        name: controllerName
      )
    }

    webView.loadFileURL(url, allowingReadAccessTo: url)

    return webView
  }

  public func updateUIView(_ uiView: WKWebView, context: Context) {
    uiView.isHidden = !self.show
  }
  public func makeCoordinator() -> Coordinator {
    Coordinator(self)
  }
  public func setInitialProperties() {
    container.setInitialProperties(
      initialValue: value,
      codeEditorTheme: colorScheme == .dark
        ? editorThemeDark : editorThemeLight,
      editorKeymap: editorKeymap,
      theme: theme,
      fontSize: webviewFontSize,
      darkMode: colorScheme == .dark
    )
  }
}

extension BibtexEditorWebview {
  public final class Coordinator: NSObject, WKNavigationDelegate,
    WKScriptMessageHandler
  {
    var parent: BibtexEditorWebview

    init(_ parent: BibtexEditorWebview) {
      self.parent = parent
    }

    public func webView(_ webView: WKWebView, didFinish navigation: WKNavigation!) {
      webView.isHidden = !self.parent.show
      print("Here 1")
      guard !parent.didSetInitialContent else { return }
      print("Here 2")

      let body =
        parent.value
        .toQuotedJavascriptString()

      webView.evaluateJavaScript(
        """
        window.localStorage.setItem("\(BibtexEditorWebviewLocalStorageKeys.initialValue.rawValue)", `\(body)`);
        """
      )
      parent.setInitialProperties()
      parent.didSetInitialContent = true
      parent.container.webView.isHidden = !self.parent.show
    }

    public func userContentController(
      _ userContentController: WKUserContentController,
      didReceive message: WKScriptMessage
    ) {
      switch message.name {
        case BibtexEditorWebviewActions.setWebviewLoaded.rawValue:
          print("Showing editor...")
          self.parent.show = true
        case BibtexEditorWebviewActions.requestBibtexEditorData.rawValue:
          parent.setInitialProperties()
          parent.container.setInitialContent(
            entryBody: parent.value
          )
        case BibtexEditorWebviewActions.onEditorChange.rawValue:
          if let body = message.body as? String {
            parent.value = body
          }
        default:
          return
      }
    }
  }
}
