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
    @Query var notes: [NoteModel]
    @Binding var searchQuery: String
    @Binding var activeCategory: SearchCategoryId

    var body: some View {
        if notes.isEmpty {
            EmptyMarkdownSearchResultsView(activeCategory: $activeCategory)
        } else {
            List {
            ForEach(notes, id: \.id) {note in
                NoteSearchResultItemView(item: note)
            }
            .onDelete(perform: removeRows)
            }
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
    MarkdownNotesSearchResultsView(searchQuery: .constant(""), activeCategory: .constant(.citation))
}
