//
//  model_context_extensions.swift
//  Fluster
//
//  Created by Andrew on 11/18/25.
//

import SwiftData
import Foundation

public extension ModelContext {
    var sqliteCommand: String {
        if let url = container.configurations.first?.url.path(percentEncoded: false) {
            "sqlite3 \"\(url)\""
        } else {
            "No SQLite database found."
        }
    }
}
