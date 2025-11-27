//
//  associate_note_with_bib_entries_view.swift
//  Fluster
//
//  Created by Andrew on 11/24/25.
//

import SwiftUI
import SwiftData

struct AssociateNoteWithBibEntryView: View {
    @Query private var bibEntries: [BibEntryModel]
    @Bindable var editingNote: NoteModel
    var body: some View {
        List(bibEntries, id: \.id) {entry in
            BibEntrySelectionItem(editingNote: editingNote, entry: entry)
        }
        .navigationTitle("Bibliography Entries")
    }
}
