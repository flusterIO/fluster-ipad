//
//  fs_sync_plan.swift
//  Fluster Desktop
//

import Foundation

/// Everything a sync will do, decided before any note is touched.
nonisolated struct FsSyncPlan: Sendable, Equatable {
  nonisolated struct Update: Sendable, Equatable {
    let entry: FsManifestEntry
    let noteId: String
    /// True when the note was matched by front matter `id` and is being moved to a new path.
    let relinked: Bool
  }

  nonisolated struct Relink: Sendable, Equatable {
    let entry: FsManifestEntry
    let noteId: String
  }

  nonisolated struct Conflict: Sendable, Equatable {
    let entry: FsManifestEntry
    let noteId: String
  }

  /// Files with no matching note.
  var creations: [FsManifestEntry] = []
  /// Notes whose content will be replaced by their file.
  var updates: [Update] = []
  /// Notes with identical content that only need their `fsPath` updated.
  var relinks: [Relink] = []
  /// Notes kept as-is because they changed and the policy didn't allow overwriting them.
  var conflicts: [Conflict] = []
  /// Synced notes whose file wasn't in the manifest.
  var missing: [FsSyncNoteSnapshot] = []
  var unchangedCount: Int = 0

  /// Files whose contents must be downloaded.
  var pathsToFetch: [String] {
    creations.map(\.relativePath) + updates.map(\.entry.relativePath)
  }
}
