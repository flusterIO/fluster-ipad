//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/4/25.
//

import SwiftData
import SwiftUI

public struct CreateTopicView: View {
  @State private var textValue: String = ""
  @Environment(\.dismiss) private var dismiss
  @Environment(\.modelContext) private var modelContext
  @Binding var selectedTopic: TopicModel?
  @Binding var paths: [CreateNotePath]
  public init(selectedTopic: Binding<TopicModel?>, paths: Binding<[CreateNotePath]>) {
    self._selectedTopic = selectedTopic
    self._paths = paths
  }
  public var body: some View {
    Form {
      TextField(
        text: $textValue,
        label: {
          Label("Create", systemImage: "plus")
        }
      )
      HStack {
        Spacer()
        Button(
          action: {
            if !textValue.isTrimmedEmpty() {
              let fetchDescriptor = FetchDescriptor<
                TopicModel
              >()
              do {
                let topics = try modelContext.fetch(
                  fetchDescriptor
                )
                let existingTopic = topics.first(where: { sub in
                  sub.value == textValue
                })
                selectedTopic =
                  existingTopic
                  ?? TopicModel(value: textValue)
                if existingTopic == nil {
                  print("Saving new topic")
                  modelContext.insert(selectedTopic!)
                }
                textValue = ""
                paths = []
              } catch {
                print(
                  "A error occurre while saving this topic: \(error)"
                )
              }
            }
          },
          label: {
            Text("Create")
          }
        )
        .disabled(textValue.isEmpty)
      }
    }
    .navigationTitle("Create topic")
  }
}

#Preview {
  CreateTopicView(selectedTopic: .constant(nil), paths: .constant([]))
}
