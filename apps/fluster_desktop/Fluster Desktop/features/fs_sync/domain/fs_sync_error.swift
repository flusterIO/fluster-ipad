//
//  fs_sync_error.swift
//  Fluster Desktop
//

import Foundation

/// Sync-level failures. Transport failures surface as `ConundrumServerError`.
nonisolated enum FsSyncError: LocalizedError, Equatable, Sendable {
  case notesDirectoryNotSet
  case incompatibleServer(serverVersion: Int, appVersion: Int)

  var errorDescription: String? {
    switch self {
      case .notesDirectoryNotSet:
        return "Choose a notes directory before syncing."
      case .incompatibleServer(let serverVersion, let appVersion):
        return
          "The Conundrum server speaks sync protocol v\(serverVersion), but this version of Fluster expects v\(appVersion). Update whichever is older."
    }
  }
}
