//
//  view_mdx_note.swift
//  Fluster
//
//  Created by Andrew on 1/19/26.
//

import ConundrumSwift
import FlusterData
import FlusterSwift
import FlusterWebviewClients
import SwiftData
import SwiftUI
import WebKit

struct MdxContentWebview: View {
  var editingNoteId: String?
  @State private var mdxWebview: WKWebView = WKWebView(
    frame: .zero, configuration: getWebViewConfig()
  )

  @Environment(\.modelContext) private var modelContext
  @Query private var notes: [NoteModel]
  @AppStorage(AppStorageKeys.embeddedCslFile.rawValue) private var cslFormat: EmbeddedCslFileSwift =
    .apa

  @Environment(\.colorScheme) private var colorScheme: ColorScheme
  @AppStorage(AppStorageKeys.codeBlockThemeDark.rawValue) var codeBlockThemeDark:
    SupportedCodeBlockTheme = .solarizedDark
  @AppStorage(AppStorageKeys.codeBlockThemeLight.rawValue) var codeBlockThemeLight:
    SupportedCodeBlockTheme = .solarizedLight
  @AppStorage(AppStorageKeys.webviewFontScale.rawValue) var webviewFontScale: Double = 1
  @AppStorage(AppStorageKeys.webviewMathFontScale.rawValue) var webviewMathFontScale: Double = 1.2

  var editingNote: NoteModel? {
    notes.isEmpty ? nil : notes.first!
  }

  init(editingNoteId: String?) {
    self.editingNoteId = editingNoteId
    if let _id = editingNoteId {
      let predicate = #Predicate<NoteModel> { $0.id == _id }
      _notes = Query(filter: predicate)
    } else {
      _notes = Query(
        filter: #Predicate<NoteModel> { note in
          false
        })
    }
  }

  var body: some View {
    if editingNoteId == nil {
      NoNoteSelectedView()
    } else {
      WebViewContainerView(
        implementation: WebviewImplementation.mdxViewer,
        editingNoteId: editingNoteId,
        webview: $mdxWebview,
        url: URL.embeddedFlusterUrl(
          folder: "standalone_mdx_preview_mac", fileName: "index_mac.html"),
        messageHandlerKeys: [
          MdxPreviewWebviewActions.onTagClick.rawValue,
          SplitviewEditorWebviewActions.setWebviewLoaded.rawValue,
          SplitviewEditorWebviewActions.requestSplitviewEditorData.rawValue,
          MdxPreviewWebviewActions.requestNoteData.rawValue
        ],
        messageHandler: messageHandler,
        onLoad: onWebviewLoad,
        mathjaxFontUrl: "/standalone_mdx_preview_mac"
      )
      .toolbar {
        ToolbarItem(
          placement: .primaryAction,
          content: {
            Button(
              action: {
                if let en = editingNote {
                  en.bookmarked = !en.bookmarked
                }
              },
              label: {
                Label(
                  title: {
                    Text("Bookmark")
                  },
                  icon: {
                    if let en = editingNote, en.bookmarked {
                      Image(systemName: "bookmark.fill")
                        .foregroundStyle(.primary)
                    } else {
                      Image(systemName: "bookmark")
                    }
                  })
              }
            )
          })
      }
      .task {
        if let en = editingNote {
          do {
            try await en.preParseIfEdited(
              modelContext: modelContext,
              uiParams: UiParams(
                darkMode: colorScheme == .dark, fontScalar: Float(webviewFontScale),
                mathFontScalar: Float(webviewMathFontScale),
                syntaxTheme: colorScheme == .dark ? codeBlockThemeDark : codeBlockThemeLight)
            )
          } catch {
            print("Error: \(error.localizedDescription)")
          }
        }
      }
    }
  }

  func onWebviewLoad() async {
  }
  public func messageHandler(_ handlerKey: String, _ messageBody: Any) {
    switch handlerKey {
      case MdxPreviewWebviewActions.requestNoteData.rawValue:
        Task(priority: .high) {
          await self.onWebviewLoad()
        }
      default:
        return
    }
  }
}
