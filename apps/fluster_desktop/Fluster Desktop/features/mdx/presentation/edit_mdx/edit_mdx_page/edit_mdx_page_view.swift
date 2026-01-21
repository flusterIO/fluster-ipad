//
//  edit_mdx_page_view.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/15/26.
//

import SwiftUI
import WebKit

struct EditMdxPageView: View {
  @Environment(AppState.self) private var appState: AppState

  var body: some View {
    if let editingNote = appState.editingNote {
      MdxEditorWebview(editingNote: editingNote)
    } else {
      NoNoteSelectedView()
    }
  }
}

#Preview {
  EditMdxPageView()
}
