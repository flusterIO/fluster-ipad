//
//  auto_setting_setting_section.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/16/26.
//

import FlusterData
import SwiftData
import SwiftUI

struct AutoSettingSettingSection: View {
  @State private var searchAutoTagText: String = ""
  @State private var showAutoTagCreate: Bool = false
  @Query private var autoTaggables: [AutoTaggable]
  @State private var sortOrder: [KeyPathComparator] = [KeyPathComparator(\AutoTaggable.value)]
  @State private var selectedAutoTag: AutoTaggable? = nil
  @State private var editingTag: AutoTaggable? = nil
  @State private var pageIdx: Int = 0
  @Environment(\.modelContext) private var modelContext: ModelContext

  var filteredAutoTags: [AutoTaggable] {
    if searchAutoTagText.isEmpty {
      return autoTaggables.sorted(using: sortOrder)
    }
    do {
      let res = try autoTaggables.filter(
        #Predicate<AutoTaggable> { t in
          t.value.localizedStandardContains(searchAutoTagText)
        })
      return res.sorted(using: sortOrder)
    } catch {
      print("Error: \(error)")
      return []
    }
  }

  var body: some View {
      let countBinding = Binding<Double>(
        get: {
            Double(filteredAutoTags.count)
        },
        set: { newValue in
            print("Attempting to set count to \(newValue)")
        }
      )
    SettingsSection(
      title: "Auto Settings",
      subtitle:
        "Use auto-settings to apply tags, topics or subjects to notes based on their file path. The glob you provide will be joined with your notes directory. All notes that satisfy the glob you provide will have the relevant tag, topic or subject applied automatically."
    ) {
      HStack(alignment: .bottom) {
        VStack(alignment: .leading) {
          Text("Search")
          TextField(
            text: $searchAutoTagText, prompt: Text("Search"),
            label: {
              Text("Search Autotags")
            }
          )
          .textFieldStyle(.roundedBorder)
          .frame(maxWidth: 350)
        }
        Spacer()
        Button(
          action: {
            editingTag = nil
            showAutoTagCreate = true
          },
          label: {
            Label(
              title: {
                Text("Create")
              },
              icon: {
                Image(systemName: "plus")
              })
          })
      }
      Group {
        if autoTaggables.isEmpty {
          Text("No auto-settings have been added yet.")
            .frame(maxWidth: .infinity)
            .padding()
        } else {
          Table(of: AutoTaggable.self) {
            TableColumn("Value", value: \.value)
            TableColumn("Glob", value: \.glob)
            TableColumn("Type") { autoTaggable in
              Text(autoTaggable.settingType.rawValue)
                .padding(.horizontal, 6)
                .padding(.vertical, 4)
                .background(
                  autoTaggable.settingType.taggableColor(), in: RoundedRectangle(cornerRadius: 10)
                )
                .foregroundColor(.white)
            }
            .width(min: 80, ideal: 120, max: 150)
            TableColumn("Actions") { autoTaggable in
              Button(
                action: {
                  selectedAutoTag = autoTaggable
                },
                label: {
                  Image(systemName: "ellipsis.circle")
                }
              )
              .presentationDetents([.medium])
            }
            .width(min: 80, ideal: 80, max: 80)
          } rows: {
            ForEach(filteredAutoTags) { autoTag in
              TableRow(autoTag)
            }
          }
          .frame(maxWidth: .infinity, minHeight: 375)
          .tableStyle(.inset)
          .popover(
            item: $selectedAutoTag,
            content: { autoTag in
              VStack(spacing: 20) {
                Text("Actions for \"\(autoTag.value)\"")
                  .lineLimit(1)
                  .font(.headline)
                Button(
                  action: {
                    editingTag = autoTag
                    showAutoTagCreate = true
                  },
                  label: {
                    Label(
                      title: {
                        Text("Edit")
                          .frame(maxWidth: .infinity, alignment: .leading)
                      },
                      icon: {
                        Image(systemName: "pencil")
                      })
                  }
                )
                Button(
                  action: {
                    modelContext.delete(autoTag)
                    do {
                      try modelContext.save()
                    } catch {
                      print("Error: \(error.localizedDescription)")
                    }
                  },
                  label: {
                    Label(
                      title: {
                        Text("Delete")
                          .frame(maxWidth: .infinity, alignment: .leading)
                      },
                      icon: {
                        Image(systemName: "trash")
                      })
                  }
                )
              }
              .padding()
            }
          )
            AutoSettingTableFooterView(sortOrder: $sortOrder, pageIdx: $pageIdx, count: countBinding)
        }
      }
    }
    .sheet(
      isPresented: $showAutoTagCreate,
      content: {
        AutoSettingCreateModal(open: $showAutoTagCreate, editingTag: $editingTag)
          .frame(width: 768)
          .presentationDetents([.large])
      })
  }
}

#Preview {
  UISettingSection()
}
