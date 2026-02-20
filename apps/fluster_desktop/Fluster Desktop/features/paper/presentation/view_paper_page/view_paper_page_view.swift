//
//  view_paper_page_view.swift
//  Fluster
//
//  Created by Andrew on 1/20/26.
//

import FlusterData
import PaperKit
import SwiftData
import SwiftUI

struct ViewPaperPageView: View {
  @EnvironmentObject private var appState: AppState
  @Environment(\.modelContext) private var modelContext
  public var editingNoteId: String?
  @Query var notes: [NoteModel]
  //    @State private var markup: PaperMarkup = PaperMarkup.
  public var editingNote: NoteModel? {
    notes.isEmpty ? nil : notes.first
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
    if var _editingNote = editingNote, editingNoteIsValid(note: _editingNote, appState: appState) {
      let noteBinding = Binding<NoteModel>(
        get: {
          _editingNote
        },
        set: { newValue in
          _editingNote = newValue
        }
      )
      PaperView(editingNote: noteBinding)
    } else {
      NoNoteSelectedView()
        .navigationTitle("Paper")
    }
  }
}

#Preview {
  ViewPaperPageView(editingNoteId: nil)
}
