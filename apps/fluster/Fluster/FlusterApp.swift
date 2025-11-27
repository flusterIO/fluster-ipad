//
//  FlusterApp.swift
//  Fluster
//
//  Created by Andrew on 10/28/25.
//

import SwiftData
import SwiftUI

typealias NoteModel = AppSchemaV1.NoteModel
typealias BibEntryModel = AppSchemaV1.BibEntryModel
typealias SubjectModel = AppSchemaV1.SubjectModel
typealias TagModel = AppSchemaV1.TagModel
typealias TopicModel = AppSchemaV1.TopicModel

@main
struct FlusterApp: App {
    @AppStorage(AppStorageKeys.isFirstLaunch.rawValue) private
        var isFirstLaunch = true

    var body: some Scene {
        WindowGroup {
            MainView()
                .ignoresSafeArea()
        }
        .modelContainer(
            AppDataContainer.create(isInitialLaunch: &isFirstLaunch)
        )
    }
}
