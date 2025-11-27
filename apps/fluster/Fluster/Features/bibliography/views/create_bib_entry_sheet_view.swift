//
//  create_bib_entry_sheet_view.swift
//  Fluster
//
//  Created by Andrew on 11/3/25.
//

import SwiftData
import SwiftUI

struct CreateBibEntrySheetView: View {
    @Binding var editingBibEntry: BibEntryModel?
    @State private var newEntryValue: String = ""
    @Environment(\.modelContext) var modelContext
    @Environment(\.dismiss) var dismiss
    @Environment(ThemeManager.self) private var themeManager: ThemeManager
    @Environment(NoteModel.self) private var editingNote: NoteModel?
    let container: BibtexEditorWebviewContainer

    var body: some View {
        VStack {
            Spacer(minLength: 8)
            HStack(alignment: .center) {
                Text("Enter valid bibtex entry.")
                    .font(.caption)
            }
            if let editingEntryBinding = Binding($editingBibEntry) {
                BibtexEditorWebview(
                    value: editingEntryBinding.data,
                    container: container
                )
            } else {
                BibtexEditorWebview(
                    value: $newEntryValue,
                    container: container
                )
            }
            Spacer(minLength: 8)
            HStack(alignment: .center) {
                Spacer()
                Button("Cancel") {
                    newEntryValue = ""
                    dismiss()
                }
                Spacer()
                Button(editingBibEntry == nil ? "Create" : "Update") {
                    let inputValue =
                        editingBibEntry == nil ? self.newEntryValue : editingBibEntry!.data
                    if inputValue.trimmingCharacters(
                        in: .whitespacesAndNewlines
                    ).isEmpty {
                        return
                    }
                    if editingBibEntry == nil {
                        // -- If the model should be created new. --
                        let newEntry = BibEntryModel(data: inputValue)
                        if let _editingNote = editingNote {
                            _editingNote.citations.append(newEntry)
                        } else {
                            modelContext.insert(newEntry)
                        }
                        newEntryValue = ""
                        container.clearEditorData()
                        dismiss()
                    } else {
                        // -- If the model needs to be updated. --
                        editingBibEntry!.data = inputValue
                        container.clearEditorData()
                        dismiss()
                    }
                }
                .buttonStyle(.glassProminent)
                Spacer()
            }
            .padding()
            Spacer(minLength: 8)
        }
    }
}

#Preview {
    CreateBibEntrySheetView(
        editingBibEntry: .constant(nil),
        container: BibtexEditorWebviewContainer()
    )
    .environment(ThemeManager(initialTheme: FlusterDark()))

}
