//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/10/25.
//

import SwiftData
import SwiftUI
import FlusterData

public struct EditNoteTaggablesView: View {
  @Environment(\.modelContext) private var modelContext
  @Environment(ThemeManager.self) private var themeManager: ThemeManager
  @Environment(\.colorScheme) private var colorScheme: ColorScheme
  @State private var subjectValue: String = ""
  @State private var topicValue: String = ""
  @State private var verticalDragAmount: CGFloat = 0
  @State private var opacityAmount: CGFloat = 1
  @Query private var subjects: [SubjectModel]
  @Query private var topics: [TopicModel]
  @FocusState private var focus: FormFieldFocus?
  @Binding var editingNote: NoteModel
  @Binding var open: Bool
  var filteredSubjects: [SubjectModel] {
    if subjectValue.isEmpty {
      return subjects
    }
    do {
      return try subjects.filter(
        #Predicate<SubjectModel> { subject in
          subject.value.localizedStandardContains(subjectValue)
        }
      )
    } catch {
      return []
    }
  }
  var filteredTopics: [TopicModel] {
    if topicValue.isEmpty {
      return topics
    }
    do {
      return try topics.filter(
        #Predicate<TopicModel> { topic in
          topic.value.localizedStandardContains(topicValue)
        }
      )
    } catch {
      return []
    }
  }
  public init(editingNote: Binding<NoteModel>, open: Binding<Bool>) {
    self._editingNote = editingNote
    self._open = open
  }
  public var body: some View {
    VStack {
      VStack(alignment: .leading) {
        Text("Subject")
          .font(.headline)
          .padding(.leading)
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
        .padding(.horizontal)
        .padding(.bottom)
        .focused($focus, equals: .subject)
        .onSubmit {
          focus = .topic
        }
        ScrollView(
          .horizontal,
          content: {
            HStack {
              ForEach(filteredSubjects) { sub in
                Text(sub.value)
                  .padding(4)
                  .font(.caption)
                  .background(themeManager.theme.primary)
                  .foregroundStyle(
                    themeManager.theme
                      .primary_foreground
                  )
                  .clipShape(
                    RoundedRectangle(cornerRadius: 16)
                  )
                  .onTapGesture {
                    subjectValue = sub.value
                  }
              }
            }
            .padding(.horizontal)
            .padding(.bottom)
          }
        )
        Text("Topic")
          .font(.headline)
          .padding(.leading)
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
        .padding(.horizontal)
        .focused($focus, equals: .topic)
        .onTapGesture {
          focus = .topic
        }
        ScrollView(
          .horizontal,
          content: {
            HStack {
              ForEach(filteredTopics) { top in
                Text(top.value)
                  .padding(4)
                  .font(.caption)
                  .background(themeManager.theme.primary)
                  .foregroundStyle(
                    themeManager.theme
                      .primary_foreground
                  )
                  .clipShape(
                    RoundedRectangle(cornerRadius: 16)
                  )
                  .onTapGesture {
                    topicValue = top.value
                  }
              }
            }
            .padding()
          }
        )
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
                  && (editingNote.subject?.value
                    != subjectValue)
                {
                  let existingSubject =
                    subjects.first(
                      where: {
                        $0.value == subjectValue
                      })
                  editingNote.subject =
                    existingSubject
                    ?? SubjectModel(value: subjectValue)
                }
                if (!topicValue.isEmpty)
                  && (editingNote.topic?.value
                    != topicValue)
                {
                  let existingTopic = topics.first(
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

              // Clear the topic or subject if the user has deleted the input.
              if subjectValue.isEmpty {
                editingNote.subject = nil
              }

              if topicValue.isEmpty {
                editingNote.topic = nil
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
      .roundedCornerWithBorder(
        lineWidth: 2,
        borderColor: themeManager.theme.border,
        radius: 16
      )
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
      .opacity(opacityAmount)
      .offset(y: verticalDragAmount)
    }
    .frame(
      width: UIScreen.current?.bounds.width,
      height: UIScreen.current?.bounds.height
    )
    .background(colorScheme == .dark ? .black : .white)
    .ignoresSafeArea()
    .gesture(
      DragGesture()
        .onChanged { drag in
          withAnimation {
            verticalDragAmount = drag.translation.height
            if drag.translation.height < 100 {
              opacityAmount = (100 - verticalDragAmount) / 100
            } else {
              opacityAmount = 0
            }
          }
        }
        .onEnded { drag in
          withAnimation {
            if drag.translation.height > 100 {
              open = false
              opacityAmount = 0
            } else {
              verticalDragAmount = 0
              opacityAmount = 1
            }
          }
        }
    )
  }

  enum FormFieldFocus: Hashable {
    case topic, subject
  }
}
