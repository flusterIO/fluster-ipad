//
//  create_note_sheet_view.swift
//  Fluster
//
//  Created by Andrew on 11/5/25.
//

import PencilKit
import SwiftData
import SwiftUI
import FlusterSwift

struct CreateNoteSheetView: View {
    @State private var titleValue: String = ""
    @FocusState private var textFieldFocused: Bool
    @Environment(\.modelContext) var modelContext
    @Environment(\.dismiss) var dismiss
    @Binding var editingNote: NoteModel?
    let dismissOnSubmit: Bool

    var body: some View {
        Form {
            TextField("Title", text: $titleValue)
                .focused($textFieldFocused)
                .onAppear {
                    textFieldFocused = true
                }
            HStack(alignment: .lastTextBaseline) {
                Spacer()
                Button("Create") {
                    if titleValue.trimmingCharacters(
                        in: .whitespacesAndNewlines
                    ).isEmpty {
                        return
                    }
                    let model = NoteModel(
                        id: nil,
                        drawing: PKDrawing.init().dataRepresentation(),
                        markdown: MarkdownNote(
                            body: "# \(titleValue)",
                            summary: nil
                        )
                    )
                    modelContext.insert(model)
                    editingNote = model
                    titleValue = ""
                    if dismissOnSubmit {
                        dismiss()
                    }
                }
                .buttonStyle(.glassProminent)
            }
        }
        .navigationTitle("Create new note")
    }
}

#Preview {
    CreateNoteSheetView(editingNote: .constant(nil), dismissOnSubmit: false)
}
