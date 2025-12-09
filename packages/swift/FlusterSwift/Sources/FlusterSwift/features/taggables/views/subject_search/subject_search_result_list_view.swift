//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/9/25.
//

import SwiftData
import SwiftUI

public struct SubjectSearchResultListView: View {
    @Query(sort: \SubjectModel.lastAccess, order: .reverse) private
        var subjects: [SubjectModel]
    @Binding var subjectQuery: String
    @Binding var editingNote: NoteModel?
    public init(subjectQuery: Binding<String>, editingNote: Binding<NoteModel?>)
    {
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
            List(subjects) { subject in
                NavigationLink(
                    destination: {
                        NoteSearchResultsBySubjectView(
                            subject: subject,
                            editingNote: $editingNote
                        )
                    },
                    label: {
                        Text(subject.value)
                    }
                )
            }
            .searchable(text: $subjectQuery, prompt: "Search Subjects")
            .navigationTitle("Subjects")
        }
    }
}

#Preview {
    SubjectSearchResultListView(
        subjectQuery: .constant(""),
        editingNote: .constant(nil)
    )
}
