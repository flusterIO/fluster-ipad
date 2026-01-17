//
//  auto_setting_create_modal.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/16/26.
//

import FlusterData
import SwiftData
import SwiftUI

struct AutoSettingCreateModal: View {
  @Binding var open: Bool
  @Binding var editingTag: AutoTaggable?
  @AppStorage(DesktopAppStorageKeys.notesDirectory.rawValue) private var notesDirectory: String = ""
  @State private var valueText: String = ""
  @State private var globText: String = ""
  @State private var taggableType: AutoTaggableType = .tag
  @Environment(\.modelContext) private var modelContext: ModelContext
  var body: some View {
    VStack(alignment: .trailing) {
      HStack {
        Text(notesDirectory)
          .font(.headline)
          .lineLimit(1)
        TextField(
          text: $globText, prompt: Text("**/*.{mdx,md}"),
          label: {
            Text("Glob")
          })
      }
      .frame(maxWidth: .infinity)
      .padding()
      HStack(alignment: .bottom, spacing: 32) {
        VStack(alignment: .leading) {
          Text("Type")
            .font(.headline)
          Picker(
            selection: $taggableType,
            content: {
              Text(AutoTaggableType.subject.rawValue).tag(AutoTaggableType.subject)
              Text(AutoTaggableType.topic.rawValue).tag(AutoTaggableType.topic)
              Text(AutoTaggableType.tag.rawValue).tag(AutoTaggableType.tag)
            },
            label: {
              Text("Type")
            }
          )
          .labelsHidden()
          .pickerStyle(.segmented)
        }
        VStack(alignment: .leading) {
          Text("Value")
            .font(.headline)
          TextField(
            text: $valueText, prompt: Text("Value"),
            label: {
              Text("Value")
            })
        }
      }
      .padding()
      HStack(alignment: .center) {
        Button(
          action: {
            if !valueText.isEmpty && !globText.isEmpty {
              do {
                if let editing = editingTag {
                  editing.settingType = taggableType
                  editing.value = valueText
                  editing.glob = globText
                  try modelContext.save()
                  editingTag = nil
                } else {
                  let item = AutoTaggable(
                    glob: globText, value: valueText, settingType: taggableType)
                  modelContext.insert(item)
                }
                try modelContext.save()
                open = false
              } catch {
                print("Error saving model context: \(error.localizedDescription)")
              }
            }
          },
          label: {
            Label(
              title: {
                Text(editingTag == nil ? "Create" : "Update")
              },
              icon: {
                Image(systemName: "plus")
              })
          }
        )
        .buttonStyle(.borderedProminent)
      }
      .padding(.horizontal)
    }
    .padding(.vertical)
    .frame(maxWidth: .infinity)
    .onChange(
      of: editingTag,
      {
        if editingTag == nil {
          valueText = ""
          globText = ""
        } else {
          valueText = editingTag!.value
          globText = editingTag!.glob
        }
      }
    )
    .onAppear {
      if editingTag == nil {
        valueText = ""
        globText = ""
      } else {
        valueText = editingTag!.value
        globText = editingTag!.glob
        taggableType = editingTag!.settingType
      }
    }
  }
}

#Preview {
  AutoSettingCreateModal(open: .constant(true), editingTag: .constant(nil))
}
