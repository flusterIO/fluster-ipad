//
//  command_palette_root.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/15/26.
//

import Foundation
import SwiftData

public class CommandPaletteRoot: CommandPaletteItem {
    public init() {
        super.init(id: .root, uniqueId: "home", title: "Root", icon: "house.circle", subtitle: nil, itemType: .children)
    }
    
    override public func children(modelContext: ModelContext, appState: AppState) -> [CommandPaletteItem] {
        return [
           CommandPaletteNavigationRoot(),
           SearchByNoteTitleCommandPaletteItem(),
           SearchByTagCommandPaletteItem(),
           SearchByTopicCommandPaletteItem(),
           SearchBySubjectCommandPaletteItem(),
           CommandPaletteItem(id: .toggleDarkMode, uniqueId: "toggle-dark-mode", title: "Toggle Dark Mode", icon: "moon.fill", subtitle: nil, itemType: .commandPaletteAction)
        ]
    }
}
