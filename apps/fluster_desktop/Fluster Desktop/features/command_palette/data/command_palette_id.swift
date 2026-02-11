//
//  command_palette_id.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/14/26.
//

import FlusterData
import Foundation
import SwiftData

public enum CommandPaletteSecondaryView: Codable, Hashable {
  case searchByTag(TagModel)
  case searchByTopic(TopicModel)
  case searchBySubject(SubjectModel)
}

public enum CommandPaletteId: Codable, Hashable {
  case parentWithNoFunctionality
  case root, showPanelRight, createNewNote, toggleDarkMode
  case navigate(MainViewKey)
  case viewNoteById(String)
  case pushCommandPaletteView(CommandPaletteSecondaryView)

  /// If true, will push a view rather than calling an action.
  var isNavigationId: Bool {
    if case .pushCommandPaletteView = self { return true }
    return false
  }
}
