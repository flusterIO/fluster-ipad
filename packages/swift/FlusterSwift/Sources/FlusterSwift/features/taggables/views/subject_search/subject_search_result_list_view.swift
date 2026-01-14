//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/9/25.
//

import SwiftData
import SwiftUI
import FlusterData

public struct SubjectSearchResultListView: View {
  @Query(sort: \SubjectModel.lastAccess, order: .reverse) private
    var subjects: [SubjectModel]
  @Environment(ThemeManager.self) private var themeManager: ThemeManager
  @Environment(\.modelContext) private var modelContext: ModelContext
  @State private var showDeleteConfirmation: Bool = false
  @State private var subjectToDelete: SubjectModel?
  @Binding var subjectQuery: String
  @Binding var editingNote: NoteModel?
  public init(subjectQuery: Binding<String>, editingNote: Binding<NoteModel?>) {
    self._subjectQuery = subjectQuery
    self._editingNote = editingNote
    if !subjectQuery.wrappedValue.isEmpty {
      let t = subjectQuery.wrappedValue
      _subjects = Query(
        filter: #Predicate<SubjectModel> { subject in
          subject.value.localizedStandardContains(t)
        },
        sort: [
          SortDescriptor(\SubjectModel.lastAccess, order: .reverse)
        ],
        animation: .default
      )
    }
  }
  public var body: some View {
    if subjects.isEmpty && subjectQuery.isEmpty {
      NoNotesFoundView(
        title: "No subjects found",
        subtitle:
          "Add a subject to your note to filter your notes by subject here."
      )
      .navigationTitle("Subjects")
    } else {
      List(subjects, id: \.id) { subject in
        NavigationLink(
          destination: {
            NoteSearchResultsBySubjectView(
              subject: subject,
              editingNote: $editingNote
            )
          },
          label: {
            Text((subject).value)
          }
        )
        .swipeActions(
          edge: .leading,
          content: {
            Button(
              action: {
                subjectToDelete = subject
                showDeleteConfirmation = true
              },
              label: {
                Label("Delete", systemImage: "trash")
              }
            )
            .tint(themeManager.theme.destructive)
          },
        )
      }
      .searchable(text: $subjectQuery, prompt: "Search Subjects")
      .navigationTitle("Subjects")
      .confirmationDialog(
        "deleteSubjectConfirmation",
        isPresented: $showDeleteConfirmation,
        actions: {
          Button(
            action: {
              Task {
                if let _subjectToDelete = subjectToDelete {
                  await deleteSubject(_subjectToDelete)
                }
              }
            },
            label: {
              Label("Delete", systemImage: "trash")
            }
          )
          .tint(themeManager.theme.destructive)
          .foregroundStyle(themeManager.theme.destructive_foreground)
        },
        message: {
          Text(
            "Are you sure you want to remove this subject? This will automatically remove this subject from all associated notes."
          )
        }
      )
    }
  }

  func deleteSubject(_ subject: SubjectModel) async {
    let subjectValue = subject.value
    let noteFetchDescriptor = FetchDescriptor<NoteModel>(
      predicate: #Predicate<NoteModel> { note in
        note.subject?.value == subjectValue
      }
    )

    do {
      let noteResults = try modelContext.fetch(noteFetchDescriptor)
      for n in noteResults {
        n.subject = nil
      }
    } catch {
      print("An error occurred while deleting tags \(error)")
    }

    modelContext.delete(subject)
  }
}

#Preview {
  SubjectSearchResultListView(
    subjectQuery: .constant(""),
    editingNote: .constant(nil)
  )
}
