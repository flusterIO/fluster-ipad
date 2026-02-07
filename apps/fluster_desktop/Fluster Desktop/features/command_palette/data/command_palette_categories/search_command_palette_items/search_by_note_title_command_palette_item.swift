//
//  search_by_note_title.swift
//  Fluster
//
//  Created by Andrew on 1/20/26.
//

import Foundation
import FlusterData
import SwiftData

class SearchByNoteTitleCommandPaletteItem: CommandPaletteItem {
  init() {
    super.init(
      id: .parentWithNoFunctionality, uniqueId: "searchByNote", title: "Search by title", icon: "character.text.justify", subtitle: nil,
      itemType: .children, noneFoundText: "No notes found.")
  }
    
    public override func children(modelContext: ModelContext, appState: AppState) -> [CommandPaletteItem] {
        let fetchDescriptor = FetchDescriptor<NoteModel>()
        do {
            let res = try modelContext.fetch(fetchDescriptor)
            return res.map { note in
                let item = CommandPaletteItem(id: .navigate(.noteViewMdx),
                                              uniqueId: "note-\(note.frontMatter.title ?? note.markdown.title ?? UUID().uuidString)",
                                              title: note.frontMatter.title ?? note.markdown.title ?? "No title found", icon: "text.document", subtitle: nil, itemType: .commandPaletteAction)
                item.onAccept = {
                    appState.editingNote = note
                }
                return item
            }
        } catch {
            print("Error retrieving notes: \(error.localizedDescription)")
        }
        return []
    }
}
