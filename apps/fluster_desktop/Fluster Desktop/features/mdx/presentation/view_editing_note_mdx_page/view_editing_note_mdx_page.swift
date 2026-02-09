//
//  view_editing_note_mdx_page.swift
//  Fluster
//
//  Created by Andrew on 1/19/26.
//

import FlusterData
import SwiftData
import SwiftUI

struct ViewEditingNoteMdxPage: View {
    @EnvironmentObject private var appState: AppState
    @Environment(\.modelContext) private var modelContext
    var editingNote: NoteModel? {
      guard let id = appState.editingNoteId else { return nil }
      return modelContext.model(for: id) as? NoteModel
    }

  var body: some View {
    if let _editingNote = editingNote {
      ViewMdxNoteView(item: _editingNote)
    } else {
      NoNoteSelectedView()
    }
  }
}

#Preview {
  ViewEditingNoteMdxPage()
}
