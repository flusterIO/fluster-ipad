//
//  File.swift
//  FlusterSwift
//
//  Created by Andrew on 2/17/26.
//

import Foundation

public enum FlusterCategoryIcon: Codable {
  case dashboard, search, settings, create, editor, paper, markdown, bibliography, noteDetails

  public func toSfIcon() -> String {
    switch self {
      case .bibliography:
        return "book.pages"
      case .dashboard:
        return "house"
      case .search:
        return "magnifyingglass"
      case .settings:
        return "gearshape"
      case .create:
        return "plus"
      case .editor:
        return "keyboard"
      case .paper:
        return "scribble.variable"
      case .markdown:
        return "text.document"
      case .noteDetails:
        return "text.page.badge.magnifyingglass"
    }
  }
}
