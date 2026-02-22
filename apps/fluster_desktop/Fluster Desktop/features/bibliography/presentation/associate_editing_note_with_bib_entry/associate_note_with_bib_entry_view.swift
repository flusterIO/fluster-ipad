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
  @Query private var notes: [NoteModel]
  var editingNote: NoteModel? {
    notes.isEmpty ? nil : notes.first!
  }
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
    if let eid = editingNoteId {
      var descriptor = FetchDescriptor(
        predicate: #Predicate<NoteModel> { note in
          note.id == eid
        }
      )
      descriptor.fetchLimit = 1
      self._notes = Query(descriptor)
    } else {
      self._notes = Query(
        filter: #Predicate<NoteModel> { _ in
          false
        }
      )
    }
  }

  var body: some View {
    Group {
      if let en = editingNote {
        List(filteredEntries, id: \.id) { item in
          AssociateNoteWithBibEntryItemView(item: item, editingNote: en)
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
