//
//  app_data_container.swift
//  Fluster
//
//  Created by Andrew on 11/27/25.
//

import SwiftData

public typealias NoteModel = AppSchemaV1.NoteModel
public typealias BibEntryModel = AppSchemaV1.BibEntryModel
public typealias SubjectModel = AppSchemaV1.SubjectModel
public typealias TagModel = AppSchemaV1.TagModel
public typealias TopicModel = AppSchemaV1.TopicModel
public typealias MarkdownNote = AppSchemaV1.MarkdownNote

@available(iOS 17, *)
public actor AppDataContainer {
    @MainActor
    public static func create(isInitialLaunch: inout Bool) -> ModelContainer {
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

            if isInitialLaunch {
                let notes = InitialNoteModelPathJsonDecoder.decode(
                    from: "initial_note_docs/initial_note_paths"
                )
                if notes.isEmpty {
                    fatalError("Failed to load initial notes.")
                }
                notes.forEach { notePath in
                    let noteBody =
                        InitialNoteModelPathJsonDecoder
                        .loadInitialNoteFromPath(notePath: notePath)
                    let noteModel = NoteModel.fromNoteBody(
                        noteBody: noteBody
                    )
                    container.mainContext.insert(noteModel)
                }
                isInitialLaunch = false
            }

            return container
        } catch {
            fatalError("Could not create ModelContainer: \(error)")
        }
    }
}
