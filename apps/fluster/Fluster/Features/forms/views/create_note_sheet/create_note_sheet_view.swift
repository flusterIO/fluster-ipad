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
import FlusterData

struct CreateNoteSheetView: View {
  @State private var titleValue: String = ""
  @State private var selectedSubject: SubjectModel?
  @State private var selectedTopic: TopicModel?
  @State private var paths: [CreateNotePath] = []
  @FocusState private var textFieldFocused: Bool
  @Environment(\.modelContext) var modelContext
  @Environment(\.mainTab) var mainTab
  @Environment(\.dismiss) var dismiss
  @Binding var editingNote: NoteModel?
  let dismissOnSubmit: Bool

  var body: some View {
    NavigationStack(path: $paths) {
      Form {
        TextField("Title", text: $titleValue)
          .focused($textFieldFocused)
          .onAppear {
            textFieldFocused = true
          }
        NavigationLink(
          value: CreateNotePath.subjectSelect,
          label: {
            Text("Subject")
          }
        )
        NavigationLink(
          value: CreateNotePath.topicSelect,
          label: {
            Text("Topic")
          }
        )
        HStack(alignment: .lastTextBaseline) {
          Spacer()
          Button(
            action: {
              if titleValue.trimmingCharacters(
                in: .whitespacesAndNewlines
              ).isEmpty {
                return
              }
              let model = NoteModel(
                id: nil,
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
              } else {
                mainTab.wrappedValue = .markdown
              }
            },
            label: {
              Text("Create")
            }
          )
          .buttonStyle(.glassProminent)
        }
      }
      .onDisappear {
        if let note = editingNote, let _subject = selectedSubject {
          note.subject = _subject
        }
      }
      .navigationTitle("Create new note")
      .navigationDestination(for: CreateNotePath.self) { p in
        switch p {
          case .subjectSelect:
            LinkSubjectToNoteView(
              selection: $selectedSubject,
              paths: $paths
            )
            .navigationTitle("Subjects")
          case .topicSelect:
            LinkTopicToNoteView(
              selection: $selectedTopic,
              paths: $paths
            )
            .navigationTitle("Topics")
          case .createTopic:
            CreateTopicView(
              selectedTopic: $selectedTopic,
              paths: $paths
            )
          case .createSubject:
            CreateSubjectView(
              selectedSubject: $selectedSubject,
              paths: $paths
            )
        }
      }
    }
    .navigationTitle("Create note")
  }
}

#Preview {
  CreateNoteSheetView(
    editingNote: .constant(nil),
    dismissOnSubmit: false
  )
}
