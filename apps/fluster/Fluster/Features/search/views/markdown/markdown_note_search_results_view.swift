//
//  markdown_note_search_result.swift
//  Fluster
//
//  Created by Andrew on 11/2/25.
//

import SwiftData
import SwiftUI

struct MarkdownNotesSearchResultsWrappedQuery: View {
    @Environment(\.modelContext) var modelContext
    @Query var notes: [NoteModel]
    @State private var confirmationDeleteModalOpen: Bool = false
    @Environment(ThemeManager.self) private var themeManager: ThemeManager
    @Binding var editingNote: NoteModel?
    @Binding var searchQuery: String
    
    var sortedNotes: [NoteModel] {
        var titleResults: [NoteModel] = []
        var bodyResults: [NoteModel] = []
        for note in notes {
            if NoteModel.isTitleMatch(noteBody: note.markdown._body, query: searchQuery) {
                titleResults.append(note)
            } else {
                bodyResults.append(note)
            }
        }
        return titleResults + bodyResults
    }

    init(editingNote: Binding<NoteModel?>, searchQuery: Binding<String>) {
        _editingNote = editingNote
        _searchQuery = searchQuery
        let _query = searchQuery.wrappedValue // Can't use getter in predicate
        if (!searchQuery.wrappedValue.isEmpty) {
            _notes = Query(
                filter: #Predicate<NoteModel> { note in
                    note.markdown._body.localizedStandardContains(
                        _query
                    )
                },
                sort: [SortDescriptor(\NoteModel.last_read, order: .reverse)],
                animation: .default
            )
        }
    }

    var body: some View {
        if notes.isEmpty {
            EmptyMarkdownSearchResultsView(editingNote: $editingNote)
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
                .onAppear {
                    print("View Database: ", modelContext.sqliteCommand)
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
    @Binding var editingNote: NoteModel?
    @State private var searchQuery: String = ""
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
