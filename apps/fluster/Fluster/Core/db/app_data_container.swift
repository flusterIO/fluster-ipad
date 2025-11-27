//
//  app_data_container.swift
//  Fluster
//
//  Created by Andrew on 11/27/25.
//

import SwiftData

actor AppDataContainer {
    @MainActor
    static func create(isInitialLaunch: inout Bool) -> ModelContainer {
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
                configurations: [modelConfiguration]
            )

            print("isInitialLaunch: \(isInitialLaunch)")
            if isInitialLaunch {
                let notes = InitialNoteModelPathJsonDecoder.decode(
                    from: "initial_note_docs/initial_note_paths"
                )
                print("Notes: \(notes)")
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
