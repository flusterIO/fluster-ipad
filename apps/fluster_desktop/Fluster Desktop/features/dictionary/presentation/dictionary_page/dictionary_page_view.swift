//
//  dictionary_page_view.swift
//  Fluster
//
//  Created by Andrew on 2/23/26.
//

import FlatBuffers
import FlusterData
import FlusterSwift
import FlusterWebviewClients
import SwiftData
import SwiftUI
import WebKit

struct DictionaryPageView: View {
  @State private var webview: WKWebView = WKWebView(
    frame: .infinite, configuration: getWebViewConfig()
  )
  @Query(sort: \DictionaryEntryModel.label, order: .forward) private var entries:
    [DictionaryEntryModel]
  @AppStorage(AppStorageKeys.defaultNoteView.rawValue) private var defaultNoteView:
    DefaultNoteView = .markdown
  @EnvironmentObject private var appState: AppState
  var body: some View {
    WebViewContainerView(
      implementation: WebviewImplementation.dictionary,
      editingNoteId: appState.editingNoteId,
      webview: $webview,
      url: URL.embeddedFlusterUrl(folder: "dictionary_webview_mac", fileName: "index_mac.html"),
      messageHandlerKeys: [
        MdxPreviewWebviewActions.onTagClick.rawValue,
        MdxPreviewWebviewActions.viewNoteById.rawValue,
        SplitviewEditorWebviewActions.setWebviewLoaded.rawValue,
        SplitviewEditorWebviewActions.requestSplitviewEditorData.rawValue,
        DictionaryWebviewActions.requestDictionaryData.rawValue,
        MdxPreviewWebviewActions.requestNoteData.rawValue
      ],
      messageHandler: messageHandler,
      onLoad: onWebviewLoad,
      mathjaxFontUrl: "/dictionary_webview_mac/mathjax/output/chtml/fonts/woff-v2"
    )
  }

  func setDictionaryContent(entries: [DictionaryEntryModel]) async throws {
    try await DictionaryState.setDictionaryState(
      payload: DictionaryState(
        entries: entries.map { entry in
          entry.toWebviewDictionaryEntry()
        }), eval: self.webview.evaluateJavaScript)
  }

  func onWebviewLoad() async {
    Task {
      do {
        try await setDictionaryContent(entries: entries)
        print("Loaded initial editor data")
      } catch {
        print("Error initializing Mdx Editor Webview: \(error.localizedDescription)")
      }
    }
  }

  public func messageHandler(_ handlerKey: String, _ messageBody: Any) {
    switch handlerKey {
      case MdxPreviewWebviewActions.requestNoteData.rawValue:
        Task(priority: .high) {
          await self.onWebviewLoad()
        }
      case DictionaryWebviewActions.requestDictionaryData.rawValue:
        Task(priority: .high) {
          await self.onWebviewLoad()
        }
      case MdxPreviewWebviewActions.viewNoteById.rawValue:
        let noteId = messageBody as? String
        AppState.shared.setEditingNoteId(editingNoteId: noteId)
        AppState.shared.mainView = defaultNoteView.toMainKey()
      default:
        print("No action implemented for \(handlerKey)")
        return
    }
  }
}
