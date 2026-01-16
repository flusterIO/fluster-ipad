//
//  command_palette_id.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/14/26.
//

import Foundation

public enum CommandPaletteId: Codable, Hashable {
    case parentWithNoFunctionality
    case root, showPanelRight, createNewNote, toggleDarkMode
    case navigate(MainViewKey)
}
