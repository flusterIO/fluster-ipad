//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 12/4/25.
//

import SwiftData
import SwiftUI

public struct LinkSubjectToNoteView: View {
    @Query private var subjects: [SubjectModel]
    @State private var searchQuery: String = ""
    @State private var showSubjectSearch: Bool = false
    @Binding var selection: SubjectModel?
    var filteredSubjects: [SubjectModel] {
        return subjects.filter { $0.value.contains(searchQuery) }
    }
    public init(selection: Binding<SubjectModel?>) {
        self._selection = selection
    }
    public var body: some View {
        if filteredSubjects.isEmpty {
            NoNotesFoundView(
                title: "No subjects found",
                subtitle: searchQuery.isEmpty
                    ? "Tap the + button to create a new subject"
                    : "No subjects found that match your query"
            )
            .toolbar {
                ToolbarItem(content: {
                    NavigationLink(
                        destination: {
                            CreateTagView(selectedSubject: $selection)
                        },
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
                    List(filteredSubjects) { subject in
                        Text(subject.value)
                    }
                    .onAppear {
                        showSubjectSearch = true
                    }
                    .onDisappear {
                        showSubjectSearch = false
                    }
                }
            )
            .toolbar {
                ToolbarItem(content: {
                    NavigationLink(
                        destination: {
                            CreateTagView(selectedSubject: $selection)
                        },
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
    LinkSubjectToNoteView(selection: .constant(nil))
}
