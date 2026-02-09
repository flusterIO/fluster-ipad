//
//  global_state.swift
//  Fluster
//
//  Created by Andrew on 1/18/26.
//

import Combine
import FlusterData
import Foundation
import Observation
import SwiftData
import SwiftUI

public class AppState: ObservableObject {
  static let shared = AppState()
  @Published var mainView: MainViewKey = .dashboard
  @Published var editingNoteId: PersistentIdentifier?
  @Published var commandPaletteNavigation = NavigationPath()

  func setEditingNote(editingNote: NoteModel?) {
    self.editingNoteId = editingNote?.persistentModelID
  }
  func commandPaletteNavigate(to route: CommandPaletteSecondaryView) {
    // Pushing to the path overlays the sidebar selection
    commandPaletteNavigation.append(route)
  }
}
