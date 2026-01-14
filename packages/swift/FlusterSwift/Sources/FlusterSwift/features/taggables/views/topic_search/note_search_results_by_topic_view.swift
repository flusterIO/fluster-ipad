//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/9/25.
//

import SwiftData
import SwiftUI
import FlusterData

public struct NoteSearchResultsByTopicView: View {
  @Query(sort: \NoteModel.lastRead, order: .reverse) private var notes: [NoteModel]
  @State private var noteQuery: String = ""
  @Binding var editingNote: NoteModel?
  let topic: TopicModel
  var filteredNotes: [NoteModel] {
    return noteQuery.isEmpty
      ? notes
      : notes.sortNotesByMarkdownBodyMatch(
        query: noteQuery,
        filterNoMatch: true
      )
  }

  public init(topic: TopicModel, editingNote: Binding<NoteModel?>) {
    self._editingNote = editingNote
    self.topic = topic
    let queryValue = topic.value
    _notes = Query(
      filter: #Predicate<NoteModel> { note in
        note.topic?.value == queryValue
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
    .searchable(text: $noteQuery, prompt: "Search notes")
    .navigationTitle("Notes")
  }
}

#Preview {
  NoteSearchResultsByTopicView(
    topic: TopicModel(value: "math"),
    editingNote: .constant(nil)
  )
}
