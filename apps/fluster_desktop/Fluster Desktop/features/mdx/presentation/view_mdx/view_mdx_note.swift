//
//  view_mdx_note.swift
//  Fluster
//
//  Created by Andrew on 1/19/26.
//

import FlusterData
import FlusterWebviewClients
import SwiftData
import SwiftUI
import WebKit

struct ViewMdxNoteView: View {
  var editingNoteId: String?
  @State private var mdxWebview: WKWebView = WKWebView(
    frame: .infinite, configuration: getWebViewConfig()
  )

  @Environment(\.modelContext) private var modelContext
  @Query private var notes: [NoteModel]

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
            webview: $mdxWebview,
            url: Bundle.main.url(
                forResource: "index",
                withExtension: "html",
                subdirectory: "standalone_mdx_preview"
            )!,
            messageHandlerKeys: [
                SplitviewEditorWebviewActions.onTagClick.rawValue,
                SplitviewEditorWebviewActions.setWebviewLoaded.rawValue,
                SplitviewEditorWebviewActions.requestSplitviewEditorData.rawValue,
                SplitviewEditorWebviewActions.requestParsedMdxContent.rawValue
            ],
            messageHandler: messageHandler,
            onLoad: onWebviewLoad
          )
      }
  }

  func onWebviewLoad() async {
    Task {
      do {
        if let en = editingNote {
          try await setParsedEditorContent(note: en)
        }
        print("Loaded initial editor data")
      } catch {
        print("Error initalizing Mdx Editor Webview: \(error.localizedDescription)")
      }
    }
  }
  public func messageHandler(_ handlerKey: String, _ messageBody: Any) {
    switch handlerKey {
      case SplitviewEditorWebviewActions.requestParsedMdxContent.rawValue:
        Task(priority: .high) {
          await self.onWebviewLoad()
        }

      case SplitviewEditorWebviewActions.requestParsedMdxContent.rawValue:
        Task(priority: .high) {
          if let en = editingNote {
            do {
              try await en.preParse(modelContext: modelContext)
              try await setParsedEditorContent(
                note: en
              )
            } catch {
              print("Error: \(error.localizedDescription)")
            }
          }
        }
      default:
        return
    }
  }

  func setParsedEditorContent(note: NoteModel) async throws {
    try await MdxEditorClient.setParsedEditorContent(
      note: note,
      evaluateJavaScript: mdxWebview.evaluateJavaScript
    )
  }
}
