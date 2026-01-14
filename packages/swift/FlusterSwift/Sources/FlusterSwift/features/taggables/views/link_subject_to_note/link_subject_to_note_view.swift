//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/4/25.
//

import SwiftData
import FlusterData
import SwiftUI

public struct LinkSubjectToNoteView: View {
  @Environment(\.dismiss) private var dismiss
  @Environment(ThemeManager.self) private var themeManager: ThemeManager
  @Query private var subjects: [SubjectModel]
  @State private var searchQuery: String = ""
  @State private var showSubjectSearch: Bool = false
  @Binding var paths: [CreateNotePath]
  @Binding var selection: SubjectModel?
  var filteredSubjects: [SubjectModel] {
    return searchQuery.isEmpty
      ? subjects : subjects.filter { $0.value.contains(searchQuery) }
  }
  public init(
    selection: Binding<SubjectModel?>,
    paths: Binding<[CreateNotePath]>
  ) {
    self._selection = selection
    self._paths = paths
  }
  public var body: some View {
    if subjects.isEmpty {
      NoNotesFoundView(
        title: "No subjects found",
        subtitle: searchQuery.isEmpty
          ? "Tap the + button to create a new subject"
          : "No subjects found that match your query"
      )
      .toolbar {
        ToolbarItem(content: {
          NavigationLink(
            value: CreateNotePath.createSubject,
            label: {
              Label("Create", systemImage: "plus")
            }
          )
        })
      }
    } else {
      HStack(
        alignment: .top,
        content: {
          if filteredSubjects.isEmpty {
            Text("No subjects found")
              .font(.title)
              .foregroundStyle(themeManager.theme.muted_foreground)
          } else {
            List(filteredSubjects) { subject in
              Text(subject.value)
                .onTapGesture {
                  selection = subject
                  dismiss()
                }
            }
          }
        }
      )
      .onAppear {
        showSubjectSearch = true
      }
      .onDisappear {
        showSubjectSearch = false
      }
      .toolbar {
        ToolbarItem(content: {
          NavigationLink(
            value: CreateNotePath.createSubject,
            label: {
              Label("Create", systemImage: "plus")
            }
          )
        })
      }
      .pickerStyle(.navigationLink)
      .searchable(
        text: $searchQuery,
        isPresented: $showSubjectSearch,
        placement: .automatic,
        prompt: LocalizedStringResource("Search subjects")
      )
      .searchable(text: $searchQuery)
    }
  }
}

#Preview {
  LinkSubjectToNoteView(selection: .constant(nil), paths: .constant([]))
}
