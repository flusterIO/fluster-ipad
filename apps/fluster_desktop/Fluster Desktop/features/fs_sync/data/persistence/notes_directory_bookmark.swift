//
//  notes_directory_bookmark.swift
//  Fluster Desktop
//

import Foundation

/// Persists sandbox access to the notes directory across launches.
///
/// Syncing itself doesn't need this (the server reads the files), but watching
/// the folder for changes does: a sandboxed app can only observe a directory
/// it holds a security-scoped bookmark for.
nonisolated enum NotesDirectoryBookmark {
  private static let key = "fsSyncNotesDirectoryBookmark"

  /// Call with the URL returned by the folder picker, while it is still accessible.
  static func save(_ url: URL, defaults: UserDefaults = .standard) throws {
    let data = try url.bookmarkData(
      options: .withSecurityScope, includingResourceValuesForKeys: nil, relativeTo: nil)
    defaults.set(data, forKey: key)
  }

  /// The bookmarked directory, and whether the bookmark should be re-saved.
  static func resolve(defaults: UserDefaults = .standard) -> (url: URL, isStale: Bool)? {
    guard let data = defaults.data(forKey: key) else { return nil }
    var isStale = false
    guard
      let url = try? URL(
        resolvingBookmarkData: data, options: .withSecurityScope, relativeTo: nil,
        bookmarkDataIsStale: &isStale)
    else { return nil }
    return (url, isStale)
  }
}
