//
//  model_context_extensions.swift
//  Fluster
//
//  Created by Andrew on 11/18/25.
//

import Foundation
import SwiftData

extension ModelContext {
  public var sqliteCommand: String {
    if let url = self.container.configurations.first?.url.path(percentEncoded: false) {
      "sqlite3 \"\(url)\""
    } else {
      "No SQLite database found."
    }
  }
  public func saveChanges() {
    do {
      try self.save()
    } catch {
      let nsError = error as NSError
      print("Detailed Save Error: \(nsError.localizedDescription)")
      print("Reason: \(nsError.localizedFailureReason ?? "Unknown")")
      print("User Info: \(nsError.userInfo)")

      // This is the key for "Silent" errors:
      if let conflicts = nsError.userInfo["conflictList"] as? NSArray {
        print("Merge Conflicts: \(conflicts)")
      }
    }
  }
}
