//
//  command_palette_id.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/14/26.
//

import FlusterData
import SwiftData
import SwiftUI

public enum CommandPaletteSecondaryView: Codable, Hashable {
  case searchByTag(TagModel)
  case searchByTopic(TopicModel)
  case searchBySubject(SubjectModel)
  case searchByCitation(BibEntryModel)
  case associateByEntriesWithEditingNote
  /// Accepts a boolean indicating whether or not a new bib entry is associated with the current note being edited, if one exists. This has no effect if `editingNote == nil`
  case createBibEntry(Bool)
  case editBibEntry(BibEntryModel)
}

public enum CommandPaletteId: Codable, Hashable {
  case parentWithNoFunctionality
  case root, showPanelRight, createNewNote, toggleDarkMode
  case setEditorThemeDark(CodeEditorTheme)
  case setEditorThemeLight(CodeEditorTheme)
  case navigate(MainViewKey)
  case viewNoteById(String)
  case pushCommandPaletteView(CommandPaletteSecondaryView)

  /// If true, will push a view rather than calling an action.
  var isNavigationId: Bool {
    if case .pushCommandPaletteView = self { return true }
    return false
  }
}
