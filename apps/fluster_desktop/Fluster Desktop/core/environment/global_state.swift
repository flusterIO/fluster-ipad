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
import SwiftUI

public class AppState: ObservableObject {
  static let shared = AppState()
  @Published var mainView: MainViewKey = .dashboard
  @Published var editingNote: NoteModel?
  @Published var commandPaletteNavigation = NavigationPath()

  func commandPaletteNavigate(to route: CommandPaletteSecondaryView) {
    // Pushing to the path overlays the sidebar selection
    commandPaletteNavigation.append(route)
  }
}
