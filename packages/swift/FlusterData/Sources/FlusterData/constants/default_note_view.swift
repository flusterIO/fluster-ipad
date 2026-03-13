import Foundation

/// The desktop main view
public enum MainViewKey: String, Codable, Hashable {
  case dashboard, paper, settings, search, createNote, bookmarks, globalBibliography,
    noteEditingPage, noteViewMdx, editingNoteBibliography, editingNoteDetails, globalDictionary
}

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
