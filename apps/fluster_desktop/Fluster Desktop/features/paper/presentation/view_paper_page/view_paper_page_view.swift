//
//  view_paper_page_view.swift
//  Fluster
//
//  Created by Andrew on 1/20/26.
//

import FlusterData
import SwiftData
import SwiftUI
import PaperKit

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
    if let _editingNote = editingNote, editingNoteIsValid(note: _editingNote, appState: appState) {
      Text("View Paper")
        GeometryReader { geo in
            Text("Paper....")
//            PaperView(paperMarkup: markup)
        }
    } else {
      NoNoteSelectedView()
    }
  }
}

#Preview {
  ViewPaperPageView(editingNoteId: nil)
}
