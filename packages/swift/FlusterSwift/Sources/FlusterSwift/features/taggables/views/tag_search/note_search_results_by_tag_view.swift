//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/9/25.
//

import SwiftData
import SwiftUI
import FlusterData

public struct NoteSearchResultsByTagView: View {
  @Query(sort: \NoteModel.lastRead, order: .reverse) private var notes: [NoteModel]
  @State private var noteQuery: String = ""
  @Binding var editingNote: NoteModel?
  public let tag: TagModel

  var filteredNotes: [NoteModel] {
    return noteQuery.isEmpty
      ? notes
      : notes.sortNotesByMarkdownBodyMatch(
        query: noteQuery,
        filterNoMatch: true
      )
  }

  public init(tag: TagModel, editingNote: Binding<NoteModel?>) {
    self._editingNote = editingNote
    self.tag = tag
    let queryValue = tag.value
    _notes = Query(
      filter: #Predicate<NoteModel> { note in
        note.tags.contains(where: { $0.value == queryValue })
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
      }
    }
    .searchable(text: $noteQuery, placement: .toolbar, prompt: "Search notes")
    .navigationTitle("Notes")
  }
}

#Preview {
  NoteSearchResultsByTagView(
    tag: TagModel(value: "myTag"),
    editingNote: .constant(nil)
  )
}
