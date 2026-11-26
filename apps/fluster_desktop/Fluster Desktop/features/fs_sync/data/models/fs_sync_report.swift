//
//  fs_sync_report.swift
//  Fluster Desktop
//

import Foundation

nonisolated struct FsSyncIssue: Sendable, Equatable, Identifiable {
  let relativePath: String
  let message: String
  var id: String { relativePath + "|" + message }
}

/// The outcome of one sync, shown to the user.
nonisolated struct FsSyncReport: Sendable, Equatable {
  var rootPath: String
  var startedAt: Date
  var finishedAt: Date = .now
  var scannedFileCount: Int = 0
  var createdCount: Int = 0
  var updatedCount: Int = 0
  var relinkedCount: Int = 0
  var unchangedCount: Int = 0
  var detachedCount: Int = 0
  var deletedCount: Int = 0
  /// Missing-file notes left in place by `.keepNote`, or protected by the empty-manifest guard.
  var missingKeptCount: Int = 0
  var conflicts: [FsSyncIssue] = []
  /// Files that could not be synced.
  var failures: [FsSyncIssue] = []
  /// Non-fatal problems, such as a note that synced but failed to parse.
  var warnings: [FsSyncIssue] = []
  var wasCancelled: Bool = false

  var changedCount: Int { createdCount + updatedCount + relinkedCount + detachedCount + deletedCount }

  var summary: String {
    var parts: [String] = []
    if createdCount > 0 { parts.append("\(createdCount) created") }
    if updatedCount > 0 { parts.append("\(updatedCount) updated") }
    if relinkedCount > 0 { parts.append("\(relinkedCount) moved") }
    if detachedCount > 0 { parts.append("\(detachedCount) unlinked") }
    if deletedCount > 0 { parts.append("\(deletedCount) deleted") }
    parts.append("\(unchangedCount) unchanged")
    if !conflicts.isEmpty { parts.append("\(conflicts.count) conflicts") }
    if !failures.isEmpty { parts.append("\(failures.count) failed") }
    let prefix = wasCancelled ? "Cancelled after syncing" : "Synced \(scannedFileCount) files"
    return "\(prefix): " + parts.joined(separator: ", ") + "."
  }
}
