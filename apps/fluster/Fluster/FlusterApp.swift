//
//  FlusterApp.swift
//  Fluster
//
//  Created by Andrew on 10/28/25.
//

import FlusterSwift
import SwiftData
import SwiftUI

typealias NoteModel = AppSchemaV1.NoteModel
typealias BibEntryModel = AppSchemaV1.BibEntryModel
typealias SubjectModel = AppSchemaV1.SubjectModel
typealias TagModel = AppSchemaV1.TagModel
typealias TopicModel = AppSchemaV1.TopicModel
typealias MarkdownNote = AppSchemaV1.MarkdownNote


@main
struct FlusterApp: App {
    let appData = AppDataContainer.shared
    @AppStorage(AppStorageKeys.hasLaunchedPreviously.rawValue) private var hasPreviouslyLaunched: Bool = false

    var body: some Scene {
        WindowGroup {
            MainView()
                .ignoresSafeArea()
                .environment(\.createDataHandler, appData.dataHandlerCreator())
        }
        .modelContainer(appData.sharedModelContainer)
    }
}
