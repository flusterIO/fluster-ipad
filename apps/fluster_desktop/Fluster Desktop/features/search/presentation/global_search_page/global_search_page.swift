//
//  global_search_page.swift
//  Fluster
//
//  Created by Andrew on 1/19/26.
//

import FlusterData
import SwiftData
import SwiftUI

enum GlobalSearchFunction: String, CaseIterable {
  case fullText = "Full Text"
  case byTitle = "By Title"
  func toQuery(notes: [NoteModel], query: String) -> [NoteModel] {
    switch self {
      case .byTitle:
        notes.filter { note in
          let title = note.getPreferedTitle()
          return title != DEFAULT_NOTE_TITLE && title.contains(query)
        }
      case .fullText:
        notes.filter { note in
          note.markdown.body.contains(query)
        }
    }
  }
}

struct GlobalSearchPage: View {
  @Query public var notes: [NoteModel]
  @State private var searchType: GlobalSearchFunction = .fullText
  @State private var searchByTopic: TopicModel? = nil
  @State private var searchBySubject: SubjectModel? = nil
  @State private var query: String = ""
  var filteredNotes: [NoteModel] {
    if query.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty {
      return notes
    } else {
      return searchType.toQuery(notes: notes, query: query)
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
        LazyVStack(alignment: .center, spacing: 20) {
          ForEach(filteredNotes, id: \.id) { note in
            NoteSearchResultItemView(
              item: note, searchByTopic: $searchByTopic, searchBySubject: $searchBySubject,
              dismissOnNavigate: true
            )
            .listRowSeparator(.hidden)
            .listRowBackground(Color.clear)
            .frame(maxWidth: 768)
          }
        }
      }
    }
    .onChange(
      of: searchType,
      {
        query = ""
      }
    )
    .searchable(text: $query, placement: .toolbarPrincipal, prompt: "Search Notes")
    .toolbar {
      ToolbarItem {
        Picker(
          selection: $searchType,
          content: {
            ForEach(GlobalSearchFunction.allCases, id: \.rawValue) { searchFunc in
              Text(searchFunc.rawValue).tag(searchFunc)
            }
          },
          label: {
            Label(
              title: {
                Text("Search Type")
              },
              icon: {
                Image(systemName: "magnifyingglass")
              })
          })
      }
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
  }
}
