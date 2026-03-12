//
//  File.swift
//  Fluster
//
//  Created by Andrew on 2/21/26.
//

import FlusterData
import SwiftData
import SwiftUI

struct AssociateNoteWithBibEntryView: View {
  let editingNoteId: String?
  @State private var searchQuery: String = ""
  @Query private var bibEntries: [BibEntryModel]
  var filteredEntries: [BibEntryModel] {
    do {
      if searchQuery.isEmpty {
        return bibEntries
      }
      let items = try bibEntries.filter(
        #Predicate<BibEntryModel> { entry in
          return entry.data.localizedStandardContains(searchQuery)
        })
      return items
    } catch {
      print("Error: \(error.localizedDescription)")
      return []
    }
  }

  init(editingNoteId: String?) {
    self.editingNoteId = editingNoteId
  }

  var body: some View {
    Group {
      if var en = editingNoteId {
        List(filteredEntries, id: \.id) { item in
          AssociateNoteWithBibEntryItemView(item: item, editingNoteId: en)
            .listStyle(.plain)
            .listRowSeparator(.hidden)
        }
        .searchable(
          text: $searchQuery, placement: .toolbarPrincipal, prompt: "Search bibliography entries"
        )
        .scrollContentBackground(.hidden)
        .scrollIndicators(.never)
        .frame(maxWidth: 768)
      } else {
        NoNoteSelectedView()
      }
    }
    .navigationTitle("Associate note with citation")
  }
}
