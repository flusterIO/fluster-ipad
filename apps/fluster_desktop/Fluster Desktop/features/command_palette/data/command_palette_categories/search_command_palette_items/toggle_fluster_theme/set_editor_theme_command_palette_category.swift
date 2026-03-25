//
//  set_fluster_theme_command_palette_category.swift
//  Fluster
//
//  Created by Andrew on 3/25/26.
//

import FlusterData
import Foundation
import SwiftData

class SetEditorThemeLightCommandPaletteItem: CommandPaletteItem {
    init() {
      super.init(
        id: .parentWithNoFunctionality, uniqueId: "set-fluster-theme-light", title: "Light Mode",
        icon: "sun.max", subtitle: nil,
        itemType: .children, noneFoundText: "No themes found.")
    }
    public override func children(modelContext: ModelContext, appState: AppState) -> [CommandPaletteItem] {
        let themes = CodeEditorTheme.allCases.filter { theme in
            !theme.isDark()
        }
        
        return themes.map{ theme in
            CommandPaletteItem(id: .setEditorThemeLight(theme), uniqueId: "theme-light-\(theme.rawValue)", title: theme.toThemeLabel(), icon: "sun.max", subtitle: nil, itemType: .commandPaletteAction, noneFoundText: "No themes found.")
        }
    }
}


class SetEditorThemeDarkCommandPaletteItem: CommandPaletteItem {
    init() {
      super.init(
        id: .parentWithNoFunctionality, uniqueId: "set-fluster-theme-dark", title: "Dark Mode",
        icon: "moon", subtitle: nil,
        itemType: .children, noneFoundText: "No themes found.")
    }
    public override func children(modelContext: ModelContext, appState: AppState) -> [CommandPaletteItem] {
        let themes = CodeEditorTheme.allCases.filter { theme in
            theme.isDark()
        }
        
        return themes.map{ theme in
            CommandPaletteItem(id: .setEditorThemeDark(theme), uniqueId: "theme-light-\(theme.rawValue)", title: theme.toThemeLabel(), icon: "sun.max", subtitle: nil, itemType: .commandPaletteAction, noneFoundText: "No themes found.")
        }
    }
}


class SetEditorThemeCommandPaletteItem: CommandPaletteItem {
  init() {
    super.init(
      id: .parentWithNoFunctionality, uniqueId: "set-fluster-theme-dark", title: "Set Editor Theme",
      icon: "paintpalette", subtitle: nil,
      itemType: .children, noneFoundText: "No themes found.")
  }

  public override func children(modelContext: ModelContext, appState: AppState)
    -> [CommandPaletteItem]
  {
    return [
        SetEditorThemeLightCommandPaletteItem(),
        SetEditorThemeDarkCommandPaletteItem()
    ]
  }
}
