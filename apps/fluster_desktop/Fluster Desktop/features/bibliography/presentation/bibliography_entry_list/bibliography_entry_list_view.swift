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
  let abstractLineLimit: Int
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
      ScrollView {
        LazyVStack(alignment: .center, spacing: 16, pinnedViews: .sectionHeaders) {
          ForEach(filteredEntries, id: \.id) { entry in
            BibliographyEntrySearchResultListItemView(
              item: entry, abstractLineLimit: abstractLineLimit,
              toCitation: self.toCitation
            )
            .contextMenu(menuItems: {
              Button(
                action: {
                  self.toEditCitation(entry)
                },
                label: {
                  Label(
                    title: {
                      Text("Edit")
                    },
                    icon: {
                      Image(systemName: "pencil")
                    })
                })
            })
            .listStyle(.plain)
            .listRowSeparator(.hidden)
          }
        }
      }
      .scrollIndicators(.never)
      .scrollContentBackground(.hidden)
      .frame(maxWidth: 768)
      .searchable(text: $searchQuery, placement: .toolbarPrincipal, prompt: "Search bibliography")
    }
  }

  public func toCitation(_ item: BibEntryModel) {
    AppState.shared.commandPaletteNavigate(to: .searchByCitation(item))
  }

  public func toEditCitation(_ item: BibEntryModel) {
    AppState.shared.commandPaletteNavigate(to: .editBibEntry(item))
  }
}

#Preview {
  BibliographyEntryListView(entries: [], abstractLineLimit: 10)
}
