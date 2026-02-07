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
      id: .parentWithNoFunctionality, uniqueId: "navigation", title: "Navigation", icon: "house.circle", subtitle: nil,
      itemType: .children)
  }

  override func children(modelContext: ModelContext, appState: AppState) -> [CommandPaletteItem] {
    return [
      CommandPaletteItem(
        id: .navigate(.dashboard), uniqueId: "nav-dashboard", title: "Dashboard", icon: "chart.bar.horizontal.page",
        subtitle: nil, itemType: .commandPaletteAction),
      CommandPaletteItem(
        id: .navigate(.search), uniqueId: "nav-search", title: "Search Page", icon: "text.magnifyingglass", subtitle: nil,
        itemType: .commandPaletteAction),
      CommandPaletteItem(
        id: .navigate(.createNote), uniqueId: "nav-create-note", title: "Create Note", icon: "plus", subtitle: nil,
        itemType: .commandPaletteAction),
      CommandPaletteItem(
        id: .navigate(.noteViewMdx), uniqueId: "nav-view-mdx", title: "View Active Note Mdx", icon: "text.document",
        subtitle: nil, itemType: .commandPaletteAction),
      CommandPaletteItem(
        id: .navigate(.paper), uniqueId: "nav-paper", title: "View Active Note Paper", icon: "scribble.variable", subtitle: nil,
        itemType: .commandPaletteAction),
      CommandPaletteItem(
        id: .navigate(.noteEditingPage), uniqueId: "nav-editing-page", title: "Edit Active Note", icon: "keyboard", subtitle: nil,
        itemType: .commandPaletteAction),
      CommandPaletteItem(
        id: .navigate(.settings), uniqueId: "nav-settings", title: "Settings", icon: "gear", subtitle: nil,
        itemType: .commandPaletteAction)
    ]
  }
}
