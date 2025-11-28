//
//  bib_entry_selection_item.swift
//  Fluster
//
//  Created by Andrew on 11/24/25.
//

import SwiftData
import SwiftUI
import FlusterSwift

struct BibEntrySelectionItem: View {
    @State private var isSelected: Bool = false
    @Environment(ThemeManager.self) private var themeManager: ThemeManager
    @Bindable var editingNote: NoteModel
    let entry: BibEntryModel
    
    var body: some View {
        HStack {
            Toggle(
                isOn: $isSelected,
                label: {
                    Text(entry.getTitle())
                }
            )
            .tint(themeManager.theme.primary)
        }
        .onAppear {
            isSelected = self.getIsSelected()
        }
        .onChange(
            of: isSelected,
            {
                if isSelected {
                    if !editingNote.containsCitation(citation: entry) {
                        editingNote.citations.append(
                            self.entry
                        )
                    }
                } else {
                    editingNote.citations = editingNote.citations.filter({
                        $0.id != self.entry.id
                    })
                }
            }
        )
    }
    func getIsSelected() -> Bool {
        return editingNote.citations.contains(where: { $0.id == self.entry.id })
    }
}
