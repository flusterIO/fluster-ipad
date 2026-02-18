//
//  editing_note_bibliography_page_view.swift
//  Fluster
//
//  Created by Andrew on 2/17/26.
//

import FlusterData
import SwiftData
import SwiftUI

struct EditingNoteBibliographyPageView: View {
  let editingNoteId: String?
  @Query var entries: [BibEntryModel]

  init(editingNoteId: String?) {
    self.editingNoteId = editingNoteId
    if let en = editingNoteId {
      self._entries = Query(
        filter: #Predicate<BibEntryModel> { entry in
          entry.notes.contains(where: { $0.id == en })
        },
        animation: .default
      )
    } else {
      self._entries = Query(
        filter: #Predicate<BibEntryModel> { entry in
          false
        },
        animation: .default
      )
    }
  }
  var body: some View {
    Group {
      if editingNoteId == nil {
        NoNoteSelectedView()
      } else {
        BibliographyEntryListView(entries: entries)
      }
    }.toolbar(content: {
      ToolbarItem(
        id: "add-bib-entry", placement: .primaryAction,
        content: {
          Button(
            action: {
              AppState.shared.commandPaletteNavigate(to: .createBibEntry)
            },
            label: {
              Label(
                title: {
                  Text("Create")
                },
                icon: {
                  Image(systemName: "plus")
                })
            })
        })
    })
  }
}

#Preview {
  EditingNoteBibliographyPageView(editingNoteId: nil)
}
