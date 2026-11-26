//
//  fs_sync_path.swift
//  Fluster Desktop
//

import Foundation

nonisolated enum FsSyncPath {
  /// Canonical form for a root-relative path: `/`-separated with no empty or `.` segments.
  /// Both stored `fsPath`s and manifest paths go through this before comparison.
  static func normalize(_ path: String) -> String {
    path.split(separator: "/", omittingEmptySubsequences: true)
      .filter { $0 != "." }
      .joined(separator: "/")
  }
}

/// Turns the stored notes directory setting into an absolute path for the server.
nonisolated enum NotesDirectoryResolver {
  static func resolve(_ stored: String) throws -> String {
    let trimmed = stored.trimmingCharacters(in: .whitespacesAndNewlines)
    guard !trimmed.isEmpty else { throw FsSyncError.notesDirectoryNotSet }
    if trimmed.hasPrefix("file://"), let url = URL(string: trimmed) {
      return url.path(percentEncoded: false)
    }
    // The settings folder picker stores `URL.absoluteString` without its scheme,
    // which is percent-encoded (e.g. `My%20Notes`).
    if trimmed.contains("%"), let decoded = trimmed.removingPercentEncoding {
      return decoded
    }
    return trimmed
  }
}
