//
//  create_note_sheet_view.swift
//  Fluster
//
//  Created by Andrew on 11/5/25.
//

import PencilKit
import SwiftUI
import SwiftData

struct CreateNoteSheetView: View {
    @State private var titleValue: String = ""
    @Binding var editingNote: NoteModel?
    @FocusState private var textFieldFocused: Bool
    @Environment(\.modelContext) var modelContext

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
                    if titleValue.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty {
                        return
                    }
                    let model = NoteModel(
                        id: nil,
                        drawing: PKDrawing.init().dataRepresentation(),
                        markdown: MarkdownNote(body: "# \(titleValue)")
                    )
                    modelContext.insert(model)
                    editingNote = model
                    titleValue = ""
                }
                .buttonStyle(.glassProminent)
            }
        }
        .navigationTitle("Create new note")
    }
}

#Preview {
    CreateNoteSheetView(editingNote: .constant(nil))
}
