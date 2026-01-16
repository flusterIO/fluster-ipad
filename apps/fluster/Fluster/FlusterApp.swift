//
//  FlusterApp.swift
//  Fluster
//
//  Created by Andrew on 10/28/25.
//

import FlusterSwift
import SwiftData
import SwiftUI
import FlusterData

// typealias NoteModel = AppSchemaV1.NoteModel
// typealias BibEntryModel = AppSchemaV1.BibEntryModel
// typealias SubjectModel = AppSchemaV1.SubjectModel
// typealias TagModel = AppSchemaV1.TagModel
// typealias TopicModel = AppSchemaV1.TopicModel
// typealias MarkdownNote = AppSchemaV1.MarkdownNote

@main
struct FlusterApp: App {
    let appData: AppDataContainer
  @AppStorage(AppStorageKeys.hasLaunchedPreviously.rawValue) private var hasPreviouslyLaunched:
    Bool = false
    
    init() {
        AppDataContainer.setup(
            initialNotesUrl: Bundle.main.url(
                forResource: "initial_note_docs/initial_notes_parsed_data",
                withExtension: "json"
              )!
        )
        self.appData = AppDataContainer.shared
    }

  var body: some Scene {
    WindowGroup {
      MainView()
        .ignoresSafeArea()
        .environment(\.createDataHandler, appData.dataHandlerCreator())
    }
    .modelContainer(appData.sharedModelContainer)
  }
}

