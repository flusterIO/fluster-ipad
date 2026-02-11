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
  public var editingNoteId: String?
  @Query private var notes: [NoteModel]
  var editingNote: NoteModel? {
      notes.isEmpty ? nil : notes.first!
    }

  public init(editingNoteId: String?) {
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
    if let _editingNote = editingNote, editingNoteIsValid(note: _editingNote, appState: appState) {
      ViewMdxNoteView(item: _editingNote)
    } else {
      NoNoteSelectedView()
    }
  }
}

#Preview {
  ViewEditingNoteMdxPage(editingNoteId: nil)
}
