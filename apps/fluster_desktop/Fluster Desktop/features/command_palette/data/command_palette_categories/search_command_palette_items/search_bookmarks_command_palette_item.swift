//
//  File.swift
//  Fluster
//
//  Created by Andrew on 3/4/26.
//

import FlusterData
import Foundation
import SwiftData

class SearchBookmarksCommandPaletteItem: CommandPaletteItem {
  init() {
    super.init(
      id: .parentWithNoFunctionality, uniqueId: "searchBookmarks", title: "Search bookmarks",
      icon: "bookmark.fill", subtitle: nil,
      itemType: .children, noneFoundText: "No notes found.")
  }

  public override func children(modelContext: ModelContext, appState: AppState)
    -> [CommandPaletteItem]
  {
    let fetchDescriptor = FetchDescriptor<NoteModel>(
        predicate: #Predicate<NoteModel> { note in
            note.bookmarked
        }
    )
    do {
      let res = try modelContext.fetch(fetchDescriptor)
      return res.map { note in
        let item = CommandPaletteItem(
          id: .viewNoteById(note.id),
          uniqueId: "note-\(note.frontMatter.title ?? note.markdown.title ?? UUID().uuidString)",
          title: note.getPreferedTitle(),
          icon: "text.document", subtitle: nil, itemType: .commandPaletteAction)
        return item
      }
    } catch {
      print("Error retrieving notes: \(error.localizedDescription)")
    }
    return []
  }
}
