//
//  FlusterApp.swift
//  Fluster
//
//  Created by Andrew on 10/28/25.
//

import SwiftUI
import SwiftData

@main
struct FlusterApp: App {
    var sharedModelContainer: ModelContainer = {
        let schema = Schema([
            MarkdownNote.self,
            NoteModel.self,
            NoteTag.self,
            SubjectModel.self,
            TopicModel.self,
            TagModel.self,
            BibEntryModel.self
        ])
        let modelConfiguration = ModelConfiguration(schema: schema, isStoredInMemoryOnly: false)

        do {
            return try ModelContainer(for: schema, configurations: [modelConfiguration])
        } catch {
            fatalError("Could not create ModelContainer: \(error)")
        }
    }()

    var body: some Scene {
        WindowGroup {
            MainView()
                .ignoresSafeArea()
        }
        .modelContainer(sharedModelContainer)
    }
}
