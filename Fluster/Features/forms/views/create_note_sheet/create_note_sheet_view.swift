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
    @Binding var editingNoteId: String?
    @Environment(\.modelContext) var modelContext

    var body: some View {
        Form {
            TextField("Title", text: $titleValue)
            HStack(alignment: .lastTextBaseline) {
                Spacer()
                Button("Create") {
                    let model = NoteModel(
                        id: nil,
                        drawing: PKDrawing.init().dataRepresentation(),
                        markdown: MarkdownNote(body: "# \(titleValue)")
                    )
                    modelContext.insert(model)
                    editingNoteId = model.id
                    titleValue = ""
                }
                .buttonStyle(.glassProminent)
            }
        }
    }
}

#Preview {
    CreateNoteSheetView(editingNoteId: .constant(nil))
}
