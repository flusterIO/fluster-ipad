//
//  associate_note_with_bib_entries_view.swift
//  Fluster
//
//  Created by Andrew on 11/24/25.
//

import FlusterData
import SwiftData
import SwiftUI

struct AssociateNoteWithBibEntryView: View {
  @Query(sort: \BibEntryModel.title, animation: .default) private var bibEntries: [BibEntryModel]
  @Binding var editingNote: NoteModel
  @Binding var open: Bool
  @State private var searchQuery: String = ""
  var filteredEntries: [BibEntryModel] {
    if searchQuery.isEmpty {
      return bibEntries
    } else {
      do {
        let res = try bibEntries.filter(
          #Predicate<BibEntryModel> { entry in
            entry._data.localizedStandardContains(searchQuery)
          })
        return res
      } catch {
        print("Error: \(error.localizedDescription)")
        return bibEntries
      }
    }
    return bibEntries
  }

  init(editingNote: Binding<NoteModel>, open: Binding<Bool>) {
    self._editingNote = editingNote
    self._open = open
  }
  var body: some View {
    List(filteredEntries, id: \.id) { entry in
      BibEntrySelectionItem(editingNote: $editingNote, entry: entry)
    }
    .searchable(
      text: $searchQuery, placement: .navigationBarDrawer(displayMode: .always), prompt: "Search"
    )
    .toolbar(content: {
      Button(
        action: {
          open = false
        },
        label: {
          Label("Close", systemImage: "x.circle.fill")
        }
      )
    })
    .navigationTitle("Bibliography Entries")
  }
}
