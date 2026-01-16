//
//  search_by_note_title_command_palette_category.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/15/26.
//

import Foundation
import SwiftData

public class SearchByNoteTitleCommandPaletteCategory: CommandPaletteItem {
  public override init(
    id: CommandPaletteId, title: String, icon: String, subtitle: String?, hasChildren: Bool = false
  ) {
    super.init(
      id: .parentWithNoFunctionality, title: "Search Notes by Title", icon: "", subtitle: nil,
      hasChildren: true)
  }
    public override func children(modelContext: ModelContext) -> [CommandPaletteItem] {
    return [
        
    ]
  }
}
