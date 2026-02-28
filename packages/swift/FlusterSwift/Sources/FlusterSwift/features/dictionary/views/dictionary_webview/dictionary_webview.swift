//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/25/25.
//

import SwiftData
import SwiftUI
import WebKit
import FlusterData

#if os(iOS)
public struct DictionaryWebview: UIViewRepresentable {
  let url: URL = Bundle.main.url(
    forResource: "index_ipad",
    withExtension: "html",
    subdirectory: "dictionary_webview_ipad"
  )!
  @Query(sort: \DictionaryEntryModel.label, order: .forward) private var dictionaryEntries:
    [DictionaryEntryModel]
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
  let container: DictionaryWebviewContainer

  public init(container: DictionaryWebviewContainer) {
    self.container = container
  }

  public func makeUIView(context: Context) -> WKWebView {
    let webView = container.webView

    webView.navigationDelegate = context.coordinator
    webView.isHidden = true
    let controllers = [
      DictionaryWebviewActions.requestDictionaryData.rawValue,
      DictionaryWebviewActions.setWebviewLoaded.rawValue
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
      entries: dictionaryEntries,
      codeEditorTheme: colorScheme == .dark
        ? editorThemeDark : editorThemeLight,
      theme: theme,
      fontSize: webviewFontSize,
      darkMode: colorScheme == .dark
    )
  }
}

extension DictionaryWebview {
  public final class Coordinator: NSObject, WKNavigationDelegate,
    WKScriptMessageHandler
  {
    var parent: DictionaryWebview

    init(_ parent: DictionaryWebview) {
      self.parent = parent
    }

    public func webView(_ webView: WKWebView, didFinish navigation: WKNavigation!) {
      webView.isHidden = !self.parent.show
      guard !parent.didSetInitialContent else { return }
//
//      let body =
//        parent.value
//        .toQuotedJavascriptString()

      parent.setInitialProperties()
      parent.didSetInitialContent = true
      parent.container.webView.isHidden = !self.parent.show
    }

    public func userContentController(
      _ userContentController: WKUserContentController,
      didReceive message: WKScriptMessage
    ) {
      switch message.name {
        case DictionaryWebviewActions.setWebviewLoaded.rawValue:
          self.parent.show = true
        case DictionaryWebviewActions.requestDictionaryData.rawValue:
          parent.setInitialProperties()
//          parent.container.setDictionaryContent(
//            entries: self.parent.dictionaryEntries
//          )
        default:
          return
      }
    }
  }
}
#endif
