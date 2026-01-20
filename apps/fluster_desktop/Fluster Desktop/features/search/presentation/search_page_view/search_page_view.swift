//
//  search_page_view.swift
//  fluster_desktop
//
//  Created by Andrew on 1/14/26.
//

import SwiftUI
import FlusterData

struct SearchPageView: View {
    let notes: [NoteModel]
    @State private var searchText: String = ""
    @State private var searchByTopic: TopicModel? = nil
    @State private var searchBySubject: SubjectModel? = nil
    var filteredNotes: [NoteModel] {
        if searchText.isEmpty {
            return notes
        }
        return notes.filter { note in
            if note.frontMatter.title != nil && note.frontMatter.title!.localizedCaseInsensitiveContains(searchText) {
                return true
           }
            if note.markdown.title != nil && note.markdown.title!.localizedCaseInsensitiveContains(searchText) {
                return true
           }
            return false
        }
    }
    var body: some View {
        VStack(alignment: .center) {
            List(filteredNotes){ note in
                NoteSearchResultItemView(item: note, searchByTopic: $searchByTopic, searchBySubject: $searchBySubject, dismissOnNavigate: true)
                    .listRowSeparator(.hidden)
                    .listRowBackground(Color.clear)
            }
            .navigationDestination(item: $searchByTopic, destination: { topic in
                SearchByTopicView(item: topic)
            })
            .navigationDestination(item: $searchBySubject, destination: { subject in
                SearchBySubjectView(item: subject)
           })
            .frame(maxWidth: 768)
            .listStyle(.plain)
            .searchable(text: $searchText, placement: .toolbarPrincipal, prompt: Text("Search results"))
        }
    }
}

#Preview {
    SearchPageView(
        notes: [NoteModel.fromNoteBody(noteBody: "# My Note")]
    )
}
