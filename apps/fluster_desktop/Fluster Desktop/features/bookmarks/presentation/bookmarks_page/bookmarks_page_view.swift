//
//  bookmarks_page_view.swift
//  Fluster
//
//  Created by Andrew on 1/18/26.
//

import FlusterData
import SwiftData
import SwiftUI

struct BookmarksPageView: View {
  @Query(
    filter: #Predicate<NoteModel> { note in
      note.bookmarked
    }, sort: \.lastRead, order: .reverse, animation: .default) var notes: [NoteModel]
  var body: some View {
    NoteSearchResultsListView(notes: notes)
  }
}

#Preview {
  BookmarksPageView()
}
