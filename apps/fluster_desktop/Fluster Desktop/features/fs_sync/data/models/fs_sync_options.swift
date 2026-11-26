//
//  fs_sync_options.swift
//  Fluster Desktop
//

import Foundation

/// What to do when a file and its note have both changed since they last matched.
nonisolated enum FsSyncConflictPolicy: String, CaseIterable, Codable, Sendable, Identifiable {
  /// Take the file if it was modified after the note, otherwise keep the note and report a conflict.
  case preferNewest
  /// Always overwrite the note with the file.
  case preferFileSystem
  /// Never overwrite a note that differs from its file; report a conflict instead.
  case preferDatabase

  var id: String { rawValue }

  var label: String {
    switch self {
      case .preferNewest: return "Newest wins"
      case .preferFileSystem: return "File system wins"
      case .preferDatabase: return "Database wins"
    }
  }
}

/// What to do with a synced note whose file no longer exists.
nonisolated enum FsSyncMissingFilePolicy: String, CaseIterable, Codable, Sendable, Identifiable {
  /// Leave the note untouched and report it.
  case keepNote
  /// Keep the note but forget its file path, so it becomes a database-only note.
  case detachNote
  /// Delete the note.
  case deleteNote

  var id: String { rawValue }

  var label: String {
    switch self {
      case .keepNote: return "Keep note"
      case .detachNote: return "Keep note, unlink file"
      case .deleteNote: return "Delete note"
    }
  }
}

nonisolated struct FsSyncOptions: Sendable, Equatable {
  var respectGitignore: Bool = true
  var includeHidden: Bool = false
  var fileKinds: Set<FsNoteFileKind> = Set(FsNoteFileKind.allCases)
  var conflictPolicy: FsSyncConflictPolicy = .preferNewest
  var missingFilePolicy: FsSyncMissingFilePolicy = .keepNote
  /// Files fetched per `contents` request. Clamped to the server's maximum.
  var contentBatchSize: Int = 50

  var effectiveBatchSize: Int {
    min(max(contentBatchSize, 1), FsSyncEndpoints.maxContentPathsPerRequest)
  }
}

/// `UserDefaults` keys owned by the sync feature.
nonisolated enum FsSyncStorageKeys: String {
  case conflictPolicy = "fsSyncConflictPolicy"
  case missingFilePolicy = "fsSyncMissingFilePolicy"
  case lastSyncedAt = "fsSyncLastSyncedAt"
  case watchForChanges = "fsSyncWatchForChanges"
}
