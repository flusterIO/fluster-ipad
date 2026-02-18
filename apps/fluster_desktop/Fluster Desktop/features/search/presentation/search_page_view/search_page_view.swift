//
//  search_page_view.swift
//  fluster_desktop
//
//  Created by Andrew on 1/14/26.
//

import FlusterData
import SwiftUI

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
      if note.frontMatter.title != nil
        && note.frontMatter.title!.localizedCaseInsensitiveContains(searchText)
      {
        return true
      }
      if note.markdown.title != nil
        && note.markdown.title!.localizedCaseInsensitiveContains(searchText)
      {
        return true
      }
      return false
    }
  }
  var body: some View {
    ScrollView {
      ZStack {
        if filteredNotes.isEmpty {
          Text("No results found.")
            .font(.headline)
            .padding()
        }
        VStack(alignment: .center, spacing: 20) {
          ForEach(filteredNotes, id: \.id) { note in
            NoteSearchResultItemView(
              item: note, searchByTopic: $searchByTopic, searchBySubject: $searchBySubject,
              dismissOnNavigate: true
            )
            .listRowSeparator(.hidden)
            .listRowBackground(Color.clear)
            .frame(maxWidth: 768)
          }
          .navigationDestination(
            item: $searchByTopic,
            destination: { topic in
              SearchByTopicView(item: topic)
            }
          )
          .navigationDestination(
            item: $searchBySubject,
            destination: { subject in
              SearchBySubjectView(item: subject)
            }
          )
          .searchable(
            text: $searchText, placement: .toolbarPrincipal, prompt: Text("Search results"))
        }
        .padding()
      }
    }
    .scrollContentBackground(.hidden)
    .scrollIndicators(.hidden)
    .scrollClipDisabled(true)
  }
}

#Preview {
  SearchPageView(
    notes: [NoteModel.fromNoteBody(noteBody: "# My Note")]
  )
}
