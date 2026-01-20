//
//  search_by_tag_command_palette_item.swift
//  Fluster
//
//  Created by Andrew on 1/20/26.
//

import FlusterData
import SwiftData
import SwiftUI

class SearchByTagCommandPaletteItem: CommandPaletteItem {
  init() {
    super.init(
      id: .parentWithNoFunctionality, title: "Search by Tag", icon: "tag.fill", subtitle: nil,
      itemType: .children, noneFoundText: "No tags found.")
  }
  public override func children(modelContext: ModelContext) -> [CommandPaletteItem] {
    let fetchDescriptor = FetchDescriptor<TagModel>(
      sortBy: [
        SortDescriptor(\TagModel.lastAccess, order: .reverse)
      ],
    )
    do {
      let tags = try modelContext.fetch(fetchDescriptor)
      return tags.map { tag in
        CommandPaletteItem(
          id: .pushCommandPaletteView(.searchByTag(tag)), title: tag.value, icon: "tag.fill",
          subtitle: nil,
          itemType: .commandPaletteAction)
      }
    } catch {
      print("Error retrieving tags: \(error.localizedDescription)")
    }
    return []
  }
}
