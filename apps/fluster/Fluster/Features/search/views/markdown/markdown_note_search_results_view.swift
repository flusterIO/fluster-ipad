//
//  markdown_note_search_result.swift
//  Fluster
//
//  Created by Andrew on 11/2/25.
//

import SwiftData
import SwiftUI

struct MarkdownNotesSearchResultsView: View {
    @Environment(\.modelContext) var modelContext
    @Query(sort: \NoteModel.last_read) var notes: [NoteModel]
    @Binding var searchQuery: String
    @Binding var activeCategory: SearchCategoryId
    @Binding var editingNote: NoteModel?
    @Environment(ThemeManager.self) private var themeManager: ThemeManager

    var body: some View {
        if notes.isEmpty {
            EmptyMarkdownSearchResultsView(activeCategory: $activeCategory)
        } else {
            List {
            ForEach(notes, id: \.id) {note in
                NoteSearchResultItemView(item: note, editingNote: $editingNote)
                    .onTapGesture {
                        editingNote = note
                    }
            }
            .onDelete(perform: removeRows)
            }
            .navigationTitle("Recently accessed notes")
        }
    }
    func removeRows(at offset: IndexSet) {
        let items = notes[offset.first!...offset.last!]
        for item in items {
            modelContext.delete(item)
        }
    }
}

#Preview {
    MarkdownNotesSearchResultsView(searchQuery: .constant(""), activeCategory: .constant(.citation), editingNote: .constant(nil))
}
