//
//  preview_data_provider.swift
//  Fluster
//
//  Created by Andrew on 2/10/26.
//

import Foundation
import FlusterData
import SwiftData

@MainActor
class PreviewDataProvider {
  static let previewContainer: ModelContainer = {
    do {
      let config = ModelConfiguration(isStoredInMemoryOnly: true)
      let schema = getDevSchema()
      let container = try ModelContainer(for: schema, configurations: config)

      return container
    } catch {
      fatalError("Failed to create model container for previewing: \(error.localizedDescription)")
    }
  }()
}
