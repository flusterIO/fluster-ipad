//
//  by_bib_entry_search_result_view.swift
//  Fluster
//
//  Created by Andrew on 11/10/25.
//

import SwiftData
import SwiftUI
import FlusterSwift

struct ByBibEntrySearchResults: View {
    @Query private var notes: [NoteModel]
    var bibEntryId: String
    @Binding var editingNote: NoteModel?

    init(bibEntryId: String, editingNote: Binding<NoteModel?>) {
        self.bibEntryId = bibEntryId
        self._editingNote = editingNote
        _notes = Query(
            filter: #Predicate {
                $0.citations.contains(where: { citation in
                    citation.id == bibEntryId
                })
            },
            sort: [SortDescriptor(\NoteModel.last_read, order: .reverse)]
        )
    }

    var body: some View {
        if notes.isEmpty {
            NoNotesFoundView(subtitle: "No notes are currently linked to this bibliography entry.")
        } else {
            List(notes, id: \.id) { note in
                NoteSearchResultItemInnerView(editingNote: $editingNote, item: note)
            }
            .toolbarRole(.navigationStack)
            .navigationTitle("Matching Notes")
        }
    }
}
