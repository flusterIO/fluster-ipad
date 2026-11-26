//
//  fs_sync_progress.swift
//  Fluster Desktop
//

import Foundation

nonisolated enum FsSyncProgress: Sendable, Equatable {
  case checkingServer
  case scanning
  case planning
  case applying(completed: Int, total: Int)
  case finishing

  var description: String {
    switch self {
      case .checkingServer: return "Connecting to the Conundrum server…"
      case .scanning: return "Scanning your notes directory…"
      case .planning: return "Comparing files with your notes…"
      case .applying(let completed, let total): return "Syncing notes (\(completed) of \(total))…"
      case .finishing: return "Saving…"
    }
  }

  /// 0...1 when determinate.
  var fractionCompleted: Double? {
    guard case .applying(let completed, let total) = self, total > 0 else { return nil }
    return Double(completed) / Double(total)
  }
}
