//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/25/25.
//

import FlusterData
import SwiftData
import SwiftUI
import WebKit

#if os(iOS)
  public struct DictionaryWebview: View {
    let url: URL = URL.embeddedFlusterUrl(
      folder: "dictionary_webview_ipad", fileName: "index_ipad.html")
    @State private var webView: WKWebView = WKWebView(
      frame: .infinite, configuration: getConfig()
    )

    @Binding private var editingNote: NoteModel?
    @Query(sort: \DictionaryEntryModel.label, order: .forward) private var entries:
      [DictionaryEntryModel]
    @AppStorage(AppStorageKeys.defaultNoteView.rawValue) private var defaultNoteView:
      DefaultNoteView = .markdown
      @State private var showWebView: Bool = false
      
      
      public init(editingNote: Binding<NoteModel?>) {
          self._editingNote = editingNote
      }

    public var body: some View {
      IosWebviewContainer(
        implementation: .dictionary,
        editingNote: $editingNote,
        webView: $webView,
        show: $showWebView,
        url: url,
        messageHandlerKeys: [
          MdxPreviewWebviewActions.onTagClick.rawValue,
          MdxPreviewWebviewActions.viewNoteById.rawValue,
          SplitviewEditorWebviewActions.setWebviewLoaded.rawValue,
          SplitviewEditorWebviewActions.requestSplitviewEditorData.rawValue,
          DictionaryWebviewActions.requestDictionaryData.rawValue,
          MdxPreviewWebviewActions.requestNoteData.rawValue
        ],
        messageHandler: self.messageHandler,
        onLoad: {
        }
      )
    }

    func messageHandler(_ messageKey: String, _ messageBody: Any) {
    }
  }
#endif
