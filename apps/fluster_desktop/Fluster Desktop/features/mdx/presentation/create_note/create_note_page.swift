//
//  create_note_page.swift
//  Fluster
//
//  Created by Andrew on 1/17/26.
//

import FlusterData
import FlusterMdx
import SwiftData
import SwiftUI

struct CreateNotePage: View {
  @EnvironmentObject private var appState: AppState
  @State private var titleText: String = ""
  @State private var subjectText: String = ""
  @State private var topicText: String = ""
  @FocusState private var focusedInitialField: Bool
  @Query private var subjects: [SubjectModel]
  @Query private var topics: [TopicModel]
  @Environment(\.modelContext) private var modelContext: ModelContext
  var body: some View {
    GeometryReader { geo in
      VStack {
        VStack(alignment: .leading) {
          Text("Create Note")
            .font(.title2)
            .padding(.top, 8)
          Text(
            "You can also create a note directly on your file system, and synchronize your notes."
          )
          .font(.caption)
          .foregroundStyle(.secondary)
          Text("Title")
            .font(.headline)
            .padding(.top, 8)
          TextField(
            text: $titleText,
            prompt: Text("Title"),
            label: {
              Text("Title")
            }
          )
          .focused($focusedInitialField)
          if geo.size.width > 400 {
            HStack {
              VStack(alignment: .leading) {
                Text("Subject")
                  .font(.headline)
                  .padding(.top, 8)
                TextField(
                  text: $subjectText, prompt: Text("Subject"),
                  label: {
                    Text("Subject")
                  }
                )
                .textInputSuggestions {
                  ForEach(subjects) { subject in
                    Text(subject.value).textInputCompletion(subject.value)
                  }
                }
              }
              VStack(alignment: .leading) {
                Text("Topic")
                  .font(.headline)
                  .padding(.top, 8)
                TextField(
                  text: $topicText, prompt: Text("Topic"),
                  label: {
                    Text("Topic")
                  }
                )
                .textInputSuggestions {
                  ForEach(topics) { topic in
                    Text(topic.value).textInputCompletion(topic.value)
                  }
                }
              }
            }
            .onAppear {
                focusedInitialField = true
            }
          } else {
            VStack {
              Text("Subject")
                .font(.headline)
                .padding(.top, 8)
              TextField(
                text: $subjectText, prompt: Text("Subject"),
                label: {
                  Text("Subject")
                }
              )
              .textInputSuggestions {
                ForEach(subjects) { subject in
                  Text(subject.value).textInputCompletion(subject.value)
                }
              }
            }
            VStack {
              Text("Topic")
                .font(.headline)
                .padding(.top, 8)
              TextField(
                text: $topicText, prompt: Text("Topic"),
                label: {
                  Text("Topic")
                }
              )
              .textInputSuggestions {
                ForEach(topics) { topic in
                  Text(topic.value).textInputCompletion(topic.value)
                }
              }
            }
          }
          HStack {
            Spacer()
            Button(
              action: {
                // Handle create note here
                if !titleText.isEmpty {
                  let item = NoteModel.fromNoteBody(noteBody: "# \"\(titleText)\"")
                  if !subjectText.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty {
                    let sl = subjectText.lowercased()
                    let existingSubject = subjects.first(where: { $0.value.lowercased() == sl })
                    let subject = existingSubject ?? SubjectModel(value: subjectText)
                    item.subject = subject
                  }
                  if !topicText.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty {
                    let tl = topicText.lowercased()
                    let existingTopic = topics.first(where: { $0.value.lowercased() == tl })
                    let topic = existingTopic ?? TopicModel(value: topicText)
                    item.topic = topic
                  }
                  modelContext.insert(item)
                  do {
                    try modelContext.save()
                    titleText = ""
                    subjectText = ""
                    topicText = ""
                    appState.mainView = .dashboard
                  } catch {
                    print("Error creating note: \(error.localizedDescription)")
                  }
                }
              },
              label: {
                Label(
                  title: {
                    Text("Create")
                  },
                  icon: {
                    Image(systemName: "plus")
                  })
              }
            )
            .padding(.top)
            .buttonStyle(.borderedProminent)
          }
        }
        .padding()
        .frame(maxWidth: 450)
        .glassEffect(in: .rect(cornerRadius: 12))
      }
      .frame(width: geo.size.width, height: geo.size.height, alignment: .center)
    }
  }
}

#Preview {
  CreateNotePage()
}
