//
//  associate_note_with_bib_entries_view.swift
//  Fluster
//
//  Created by Andrew on 11/24/25.
//

import SwiftData
import SwiftUI

struct AssociateNoteWithBibEntryView: View {
    @Query(sort: \BibEntryModel.title, animation: .default) private var bibEntries: [BibEntryModel]
    @Bindable var editingNote: NoteModel
    @Binding var open: Bool

    init(editingNote: NoteModel, open: Binding<Bool>) {
        self.editingNote = editingNote
        _open = open
    }
    var body: some View {
        List(bibEntries, id: \.id) { entry in
            BibEntrySelectionItem(editingNote: editingNote, entry: entry)
        }
        .toolbar(content: {
            Button(
                action: {
                    open = false
                },
                label: {
                    Label("Close", systemImage: "x.circle.fill")
                }
            )
        })
        .navigationTitle("Bibliography Entries")
    }
}
