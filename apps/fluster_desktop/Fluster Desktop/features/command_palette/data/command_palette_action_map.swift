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

public func executeCommandPaletteAction(
  action: CommandPaletteId
) {
  switch action {
    case .root:
      break
    case .pushCommandPaletteView:
      print("This should never be reached here.")
    case .navigate(let mainKey):
      MainNavigationEventEmitter.shared.emitChange(to: mainKey)
    case .toggleDarkMode:
      let currentDarkMode =
        UserDefaults.standard.string(forKey: DesktopAppStorageKeys.colorScheme.rawValue)
        == AppTheme.dark.rawValue
      if currentDarkMode {
        UserDefaults.standard.set(
          AppTheme.light.rawValue, forKey: DesktopAppStorageKeys.colorScheme.rawValue)
      } else {
        UserDefaults.standard.set(
          AppTheme.dark.rawValue, forKey: DesktopAppStorageKeys.colorScheme.rawValue)
      }
    case .createNewNote:
      MainNavigationEventEmitter.shared.emitChange(to: MainViewKey.noteEditingPage)
    case .showPanelRight:
      print("Show Panel Right")
    case .parentWithNoFunctionality:
      return
  }
}
