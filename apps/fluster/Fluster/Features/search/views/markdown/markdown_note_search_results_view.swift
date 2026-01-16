//
//  markdown_note_search_result.swift
//  Fluster
//
//  Created by Andrew on 11/2/25.
//

import FlusterSwift
import SwiftData
import SwiftUI
import FlusterData
import FlusterMdx

struct MarkdownNotesSearchResultsWrappedQuery: View {
  @Environment(\.modelContext) var modelContext
  @Query(sort: \NoteModel.lastRead, order: .reverse) var notes: [NoteModel]
  @State private var confirmationDeleteModalOpen: Bool = false
  @Environment(ThemeManager.self) private var themeManager: ThemeManager
  @Binding var editingNote: NoteModel?
  @Binding var searchQuery: String

  var sortedNotes: [NoteModel] {
    return searchQuery.isEmpty
      ? notes
      : notes.sortNotesByMarkdownBodyMatch(
        query: searchQuery, filterNoMatch: false)
  }

  init(editingNote: Binding<NoteModel?>, searchQuery: Binding<String>) {
    _editingNote = editingNote
    _searchQuery = searchQuery
    let _query = searchQuery.wrappedValue  // Can't use getter in predicate
    if !searchQuery.wrappedValue.isEmpty {
      _notes = Query(
        filter: #Predicate<NoteModel> { note in
          note.markdown._body.localizedStandardContains(
            _query
          )
        },
        sort: [SortDescriptor(\NoteModel.lastRead, order: .reverse)],
        animation: .default
      )
    }
  }

  var body: some View {
    if NoteModel.count(modelContext: modelContext) == 0 {
      EmptyMarkdownSearchResultsView(editingNote: $editingNote)
        .onAppear {
          print("Length: \(notes.count)")
        }
        .navigationTitle("Recently accessed notes")
    } else {
      List {
        ForEach(sortedNotes, id: \.id) { note in
          NoteSearchResultItemView(
            item: note,
            editingNote: $editingNote
          )
          .onTapGesture {
            editingNote = note
          }
        }
      }
      .searchable(
        text: $searchQuery,
        placement: .toolbarPrincipal,
        prompt: "Search"
      )
      .navigationTitle("Recently accessed notes")
    }
  }
}

struct MarkdownNotesSearchResultsView: View {
  @State private var searchQuery: String = ""
  @Binding var editingNote: NoteModel?
  var body: some View {
    MarkdownNotesSearchResultsWrappedQuery(
      editingNote: $editingNote,
      searchQuery: $searchQuery
    )
  }
}

#Preview {
  MarkdownNotesSearchResultsView(
    editingNote: .constant(nil),
  )
}
