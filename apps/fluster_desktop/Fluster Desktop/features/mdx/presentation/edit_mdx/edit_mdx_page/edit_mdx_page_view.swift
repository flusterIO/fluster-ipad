//
//  edit_mdx_page_view.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/15/26.
//

import FlusterData
import SwiftData
import SwiftUI
import WebKit

struct EditMdxPageView: View {
  public var editingNoteId: String?
  @EnvironmentObject private var appState: AppState
  @Environment(\.modelContext) private var modelContext
  @Query private var notes: [NoteModel]
  var editingNote: NoteModel? {
    notes.isEmpty ? nil : notes.first!
  }
  @Binding var webview: WKWebView
  public init(editingNoteId: String?, webview: Binding<WKWebView>) {
    self.editingNoteId = editingNoteId
    self._webview = webview
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
    if let _editingNote = editingNote, editingNoteIsValid(note: _editingNote, appState: appState) {
      MdxEditorWebview(editingNote: _editingNote, webView: $webview)
    } else {
      NoNoteSelectedView()
    }
  }
}

#Preview {
  EditMdxPageView(
    editingNoteId: nil,
    webview: .constant(
      WKWebView(
        frame: .infinite, configuration: getWebViewConfig()
      )))
}
