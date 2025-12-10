//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/10/25.
//

import SwiftUI
import SwiftData

public struct EditNoteTaggablesView: View {
    @Environment(\.modelContext) private var modelContext
    @Environment(ThemeManager.self) private var themeManager: ThemeManager
    @State private var subjectValue: String = ""
    @State private var topicValue: String = ""
    @FocusState private var focus: FormFieldFocus?
    @Binding var editingNote: NoteModel
    @Binding var open: Bool
    public init(editingNote: Binding<NoteModel>, open: Binding<Bool>) {
        self._editingNote = editingNote
        self._open = open
    }
    public var body: some View {
        Form {
            VStack(alignment: .center) {
                TextField(
                    text: $subjectValue,
                    prompt: Text("Subject"),
                    label: {
                        Text("Subject input")
                    }
                )
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
//                    .buttonStyle(.glassProminent)
                    .labelStyle(.titleAndIcon)
                    .padding()
                    Button(
                        role: .confirm,
                        action: {
                            do {
                                if (!subjectValue.isEmpty) && (editingNote.subject?.value != subjectValue) {
                                    let fetchDescriptor = FetchDescriptor<SubjectModel>()
                                    let existingSubjects: [SubjectModel] = try modelContext.fetch(fetchDescriptor)
                                    let existingSubject = existingSubjects.first(
                                        where: {
                                            $0.value == subjectValue
                                        })
                                    editingNote.subject =
                                    existingSubject
                                    ?? SubjectModel(value: subjectValue)
                                }
                                if (!topicValue.isEmpty) && (editingNote.topic?.value != topicValue) {
                                    let fetchDescriptor = FetchDescriptor<TopicModel>()
                                    let existingTopics: [TopicModel] = try modelContext.fetch(fetchDescriptor)
                                    let existingTopic = existingTopics.first(
                                        where: {
                                            $0.value == topicValue
                                        })
                                    editingNote.topic =
                                    existingTopic
                                    ?? TopicModel(value: topicValue)
                                }
                            } catch {
                                print("An error occurred while updating note taggables. \(error)")
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
            .interactiveDismissDisabled(true)
        }
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
