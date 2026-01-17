//
//  app_container.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/15/26.
//

import Foundation
import SwiftData
import FlusterData
//import FlusterMdx
//import FlusterSwiftMdxParser
//import FlusterData
//import FlusterSwiftMdxParser

@MainActor  // This was required by the mainContext key, but there's almost surely a way to do this multi-threaded. Look into this later.
@available(iOS 26, macOS 14, *)
public final class AppDataContainer {
  public static let shared = AppDataContainer()
  public var sharedModelContainer: ModelContainer {
    let schema = Schema([
      NoteModel.self,
      BibEntryModel.self,
      AutoTaggable.self
    ])
    let modelConfiguration = ModelConfiguration(
      schema: schema,
      isStoredInMemoryOnly: false
    )
    do {
      let container = try ModelContainer(
        for: schema,
        migrationPlan: AppDataMigrationPlan.self,
        configurations: [modelConfiguration]
      )
      return container
    } catch {
      fatalError("Could not create ModelContainer: \(error)")
    }
  }

  private init() {}

  public func dataHandlerCreator() -> @Sendable () async -> DataHandler {
    let container = sharedModelContainer
    return {
      DataHandler(modelContainer: container)
    }
  }
}
