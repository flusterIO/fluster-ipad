//
//  default_note_view.swift
//  Fluster
//
//  Created by Andrew on 2/12/26.
//

import Foundation

public enum DefaultNoteView: String, CaseIterable {
  case paper, markdown, editor, details
  public func toString() -> String {
    switch self {
      case .editor:
        return "Editor"
      case .markdown:
        return "Markdown"
      case .paper:
        return "Paper"
      case .details:
        return "Details"
    }
  }
  public func toMainKey() -> MainViewKey {
    switch self {
      case .editor:
        return .noteEditingPage
      case .markdown:
        return .noteViewMdx
      case .paper:
        return .paper
      case .details:
        return .editingNoteDetails
    }
  }
}
