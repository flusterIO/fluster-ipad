//
//  app_storage_keys.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/15/26.
//

import Foundation

public enum DesktopAppStorageKeys: String {
  case colorScheme
  case theme
  case hasLaunchedPreviously
  /// The notesDirectory is a string that is initially empty. This value must always be checked for it's empty status instead of a null value.
  case notesDirectory
  /// Defaults to true.
  case respectGitIgnore
  // -- UI State --
  case noteSidebarSectionOpen
  case flusterSidebarSectionOpen
}
