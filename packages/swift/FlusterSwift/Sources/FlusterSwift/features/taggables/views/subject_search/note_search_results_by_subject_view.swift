//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/9/25.
//

import SwiftData
import SwiftUI
import FlusterData

public struct NoteSearchResultsBySubjectView: View {
  @Query(sort: \NoteModel.lastRead, order: .reverse) private var notes: [NoteModel]
  @State private var noteQuery: String = ""
  @Binding var editingNote: NoteModel?
  let subject: SubjectModel
  var filteredNotes: [NoteModel] {
    return noteQuery.isEmpty
      ? notes
      : notes.sortNotesByMarkdownBodyMatch(
        query: noteQuery,
        filterNoMatch: true
      )
  }

  public init(subject: SubjectModel, editingNote: Binding<NoteModel?>) {
    self._editingNote = editingNote
    self.subject = subject
    let queryValue = subject.value
    _notes = Query(
      filter: #Predicate<NoteModel> { note in
        note.subject?.value == queryValue
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

#Preview {
  NoteSearchResultsBySubjectView(
    subject: SubjectModel(value: "math"),
    editingNote: .constant(nil)
  )
}
