//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/10/25.
//

import SwiftData
import SwiftUI

public struct EditNoteTaggablesView: View {
    @Environment(\.modelContext) private var modelContext
    @Environment(ThemeManager.self) private var themeManager: ThemeManager
    @State private var subjectValue: String = ""
    @State private var topicValue: String = ""
    @Query private var subjects: [SubjectModel]
    @FocusState private var focus: FormFieldFocus?
    @Binding var editingNote: NoteModel
    @Binding var open: Bool
    public init(editingNote: Binding<NoteModel>, open: Binding<Bool>) {
        self._editingNote = editingNote
        self._open = open
    }
    public var body: some View {
        VStack(alignment: .leading) {
            TextField(
                text: $subjectValue,
                prompt: Text("Subject"),
                label: {
                    Text("Subject input")
                }
            )
            .padding()
            .background(themeManager.theme.input)
            .cornerRadius(16)
            .padding()
            .focused($focus, equals: .subject)
            .onSubmit {
                focus = .topic
            }
            TextField(
                text: $topicValue,
                prompt: Text("Topic"),
                label: {
                    Text("Topic input")
                }
            )
            .padding()
            .background(themeManager.theme.input)
            .cornerRadius(16)
            .padding()
            .focused($focus, equals: .topic)
            .onTapGesture {
                focus = .topic
            }
            HStack {
                Spacer()
                Button(
                    role: .cancel,
                    action: {
                        subjectValue = ""
                        topicValue = ""
                        open = false
                    },
                    label: {
                        Label("Cancel", systemImage: "xmark")
                    }
                )
                .labelStyle(.titleAndIcon)
                .padding()
                Button(
                    role: .confirm,
                    action: {
                        do {
                            if (!subjectValue.isEmpty)
                                && (editingNote.subject?.value != subjectValue)
                            {
                                let fetchDescriptor = FetchDescriptor<
                                    SubjectModel
                                >()
                                let existingSubjects: [SubjectModel] =
                                    try modelContext.fetch(fetchDescriptor)
                                let existingSubject = existingSubjects.first(
                                    where: {
                                        $0.value == subjectValue
                                    })
                                editingNote.subject =
                                    existingSubject
                                    ?? SubjectModel(value: subjectValue)
                            }
                            if (!topicValue.isEmpty)
                                && (editingNote.topic?.value != topicValue)
                            {
                                let fetchDescriptor = FetchDescriptor<
                                    TopicModel
                                >()
                                let existingTopics: [TopicModel] =
                                    try modelContext.fetch(fetchDescriptor)
                                let existingTopic = existingTopics.first(
                                    where: {
                                        $0.value == topicValue
                                    })
                                editingNote.topic =
                                    existingTopic
                                    ?? TopicModel(value: topicValue)
                            }
                        } catch {
                            print(
                                "An error occurred while updating note taggables. \(error)"
                            )
                        }
                        subjectValue = ""
                        topicValue = ""
                        open = false
                    },
                    label: {
                        Label("Save", systemImage: "checkmark")
                    }
                )
                .padding()
                .labelStyle(.titleAndIcon)
                .buttonStyle(.glassProminent)
            }
        }
        .frame(
            maxWidth: 768
        )
        .padding()
        .background(themeManager.theme.background.opacity(0.4))
        .clipShape(RoundedRectangle(cornerRadius: 16))
        .roundedCornerWithBorder(lineWidth: 2, borderColor: themeManager.theme.border, radius: 16)
        .foregroundStyle(themeManager.theme.foreground)
        .interactiveDismissDisabled(true)
        .onAppear {
            if let subject = editingNote.subject {
                subjectValue = subject.value
            }
            if let topic = editingNote.topic {
                topicValue = topic.value
            }
            focus = .subject
        }
        .navigationTitle("Note Details")
        .interactiveDismissDisabled(true)
        .padding()
    }

    enum FormFieldFocus: Hashable {
        case topic, subject
    }
}
