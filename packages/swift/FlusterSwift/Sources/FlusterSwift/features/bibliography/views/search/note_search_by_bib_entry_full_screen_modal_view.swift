//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 1/8/26.
//

import SwiftData
import SwiftUI

public struct NoteSearchResultsByCitationView: View {
  @Query(sort: \NoteModel.lastRead, order: .reverse) private var notes: [NoteModel]
  @State private var noteQuery: String = ""
  @Binding var editingNote: NoteModel?
  let bibEntry: BibEntryModel
  var filteredNotes: [NoteModel] {
    return noteQuery.isEmpty
      ? notes
      : MdxTextUtils.sortNotesByMarkdownBodyMatch(
        notes: notes,
        query: noteQuery,
        filterNoMatch: true
      )
  }

  public init(bibEntry: BibEntryModel, editingNote: Binding<NoteModel?>) {
    self._editingNote = editingNote
    self.bibEntry = bibEntry
    let queryValue = bibEntry.id
    _notes = Query(
      filter: #Predicate<NoteModel> { note in
          note.citations.contains(where: { cit in
              cit.id == queryValue
          })
      },
      sort: [SortDescriptor(\NoteModel.lastRead, order: .reverse)],
      animation: .default
    )
  }

  public var body: some View {
    ZStack {
      List(filteredNotes) { note in
        NoteSearchResultItemView(item: note, editingNote: $editingNote)
      }
      if filteredNotes.isEmpty {
        NoNotesFoundView()
          .searchable(text: $noteQuery, prompt: "Search notes")
          .navigationTitle("Notes")
      }
    }
    .searchable(text: $noteQuery, prompt: "Search notes")
    .navigationTitle("Notes")
  }
}
