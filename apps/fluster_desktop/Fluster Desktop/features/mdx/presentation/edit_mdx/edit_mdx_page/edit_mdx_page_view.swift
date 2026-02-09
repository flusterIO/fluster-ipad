//
//  edit_mdx_page_view.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/15/26.
//

import FlusterData
import SwiftUI
import WebKit
import SwiftData

struct EditMdxPageView: View {
  @EnvironmentObject private var appState: AppState
  @Environment(\.modelContext) private var modelContext
  var editingNote: NoteModel? {
    guard let id = appState.editingNoteId else { return nil }
    return modelContext.model(for: id) as? NoteModel
  }
  @Binding var webview: WKWebView

  var body: some View {
    if let _editingNote = editingNote {
      MdxEditorWebview(editingNote: _editingNote, webView: $webview)
    } else {
      NoNoteSelectedView()
    }
  }
}

#Preview {
  EditMdxPageView(
    webview: .constant(
      WKWebView(
        frame: .infinite, configuration: getWebViewConfig()
      )))
}
