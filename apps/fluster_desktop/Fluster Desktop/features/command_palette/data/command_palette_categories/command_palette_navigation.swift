//
//  command_palette_navigation.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/15/26.
//

import Foundation
import SwiftData

class CommandPaletteNavigationRoot: CommandPaletteItem {
  init() {
    super.init(
      id: .parentWithNoFunctionality, title: "Navigation", icon: "house.circle", subtitle: nil,
      itemType: .children)
  }

  override func children(modelContext: ModelContext, appState: AppState) -> [CommandPaletteItem] {
    return [
      CommandPaletteItem(
        id: .navigate(.dashboard), title: "Dashboard", icon: "chart.bar.horizontal.page",
        subtitle: nil, itemType: .commandPaletteAction),
      CommandPaletteItem(
        id: .navigate(.search), title: "Search Page", icon: "text.magnifyingglass", subtitle: nil,
        itemType: .commandPaletteAction),
      CommandPaletteItem(
        id: .navigate(.createNote), title: "Create Note", icon: "plus", subtitle: nil,
        itemType: .commandPaletteAction),
      CommandPaletteItem(
        id: .navigate(.noteViewMdx), title: "View Active Note Mdx", icon: "document.text",
        subtitle: nil, itemType: .commandPaletteAction),
      CommandPaletteItem(
        id: .navigate(.paper), title: "View Active Note Paper", icon: "scribble.variable", subtitle: nil,
        itemType: .commandPaletteAction),
      CommandPaletteItem(
        id: .navigate(.noteEditingPage), title: "Edit Active Note", icon: "keyboard", subtitle: nil,
        itemType: .commandPaletteAction),
      CommandPaletteItem(
        id: .navigate(.settings), title: "Settings", icon: "gear", subtitle: nil,
        itemType: .commandPaletteAction)
    ]
  }
}
