//
//  app_data_container.swift
//  Fluster
//
//  Created by Andrew on 11/27/25.
//

import FlusterRust
import Foundation
import SwiftData

public typealias NoteModel = AppSchemaV1.NoteModel
public typealias BibEntryModel = AppSchemaV1.BibEntryModel
public typealias SubjectModel = AppSchemaV1.SubjectModel
public typealias TagModel = AppSchemaV1.TagModel
public typealias TopicModel = AppSchemaV1.TopicModel
public typealias MarkdownNote = AppSchemaV1.MarkdownNote

public let INITIAL_NOTES_DATA_PATH = "initial_note_docs/initial_notes_parsed_data"

@MainActor  // This was required by the mainContext key, but there's almost surely a way to do this multi-threaded. Look into this later.
@available(iOS 26, *)
public final class AppDataContainer {
    public static let shared = AppDataContainer()
    public var sharedModelContainer: ModelContainer {
        let hasLaunchedPreviously = UserDefaults.standard.bool(
            forKey: AppStorageKeys.hasLaunchedPreviously.rawValue
        )
        print("isInitialLaunch: \(hasLaunchedPreviously)")
        let schema = Schema([
            NoteModel.self,
            BibEntryModel.self,
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

            if !hasLaunchedPreviously {
                let noteData = InitialNotesDataJsonDecoder.decode(
                    from: INITIAL_NOTES_DATA_PATH
                )
                

                if let nd = noteData {
                    nd.forEach { noteItemData in
                        let noteModel = NoteModel.fromInitialDataParsingResult(
                            data: noteItemData,
                            existingTags: []
                        )
                        container.mainContext.insert(noteModel)
                    }
                } else {
                    fatalError("Failed to load initial notes.")
                }
            }
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
