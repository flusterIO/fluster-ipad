//
//  command_palette_action_map.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/14/26.
//

import SwiftData
import SwiftUI

public typealias CommandPaletteAction = (ModelContext, CommandPaletteId) -> Void

public let commandPaletteActionMap: [CommandPaletteId: CommandPaletteAction] = [
  CommandPaletteId.createNewNote: { _, _ in print("create new note") }
]

