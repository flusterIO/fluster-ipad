//
//  bibliography_entry_list_view.swift
//  Fluster
//
//  Created by Andrew on 2/18/26.
//

import FlusterData
import SwiftUI

struct BibliographyEntryListView: View {
  let entries: [BibEntryModel]
  @State private var searchQuery: String = ""
  var body: some View {
    ZStack(alignment: .center) {
      if entries.isEmpty {
        Text("No results found.")
          .font(.headline)
          .padding()
      }
      List(entries, id: \.id) { entry in
        BibliographyEntrySearchResultListItemView(item: entry)
      }
      .searchable(text: $searchQuery, placement: .toolbarPrincipal, prompt: "Search bibliography")
    }
  }
}

#Preview {
  BibliographyEntryListView(entries: [])
}
