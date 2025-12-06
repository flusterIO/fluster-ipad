//
//  create_note_sheet_view.swift
//  Fluster
//
//  Created by Andrew on 11/5/25.
//

import FlusterSwift
import PencilKit
import SwiftData
import SwiftUI

struct CreateNoteSheetView: View {
    @State private var titleValue: String = ""
    @State private var selectedSubject: SubjectModel?
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
            NavigationLink(
                destination: {
                    LinkSubjectToNoteView(selection: $selectedSubject)
                },
                label: {
                    Text("Subject")
                }
            )

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
                        ),
                        subject: selectedSubject,
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
        .onDisappear {
            if let note = editingNote, let _subject = selectedSubject {
                note.subject = _subject
            }
        }
        .navigationTitle("Create new note")
    }
}

#Preview {
    CreateNoteSheetView(editingNote: .constant(nil), dismissOnSubmit: false)
}
