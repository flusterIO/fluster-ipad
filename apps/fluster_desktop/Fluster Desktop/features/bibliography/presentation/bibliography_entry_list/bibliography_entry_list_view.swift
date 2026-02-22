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
  var filteredEntries: [BibEntryModel] {
    do {
      let entries = try entries.filter(
        #Predicate<BibEntryModel> { entry in
          entry.data.contains(searchQuery)
        })
      return entries
    } catch {
      print("Error: \(error.localizedDescription)")
      return []
    }
  }
  var body: some View {
    ZStack(alignment: .center) {
      if entries.isEmpty {
        Text("No results found.")
          .font(.headline)
          .padding()
      }

      List(filteredEntries, id: \.id) { entry in
        BibliographyEntrySearchResultListItemView(item: entry)
      }
      .scrollIndicators(.hidden)
      .listStyle(.plain)
      .listRowSeparator(.hidden)
      .scrollContentBackground(.hidden)
      .frame(maxWidth: 768)
      .searchable(text: $searchQuery, placement: .toolbarPrincipal, prompt: "Search bibliography")
    }
  }
}

#Preview {
  BibliographyEntryListView(entries: [])
}
