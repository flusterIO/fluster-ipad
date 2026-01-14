//
//  Fluster_DesktopApp.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/14/26.
//

import SwiftUI
import SwiftData
import UniformTypeIdentifiers

@main
struct Fluster_DesktopApp: App {
    var body: some Scene {
        DocumentGroup(editing: .itemDocument, migrationPlan: Fluster_DesktopMigrationPlan.self) {
            ContentView()
        }
    }
}

extension UTType {
    static var itemDocument: UTType {
        UTType(importedAs: "com.example.item-document")
    }
}

struct Fluster_DesktopMigrationPlan: SchemaMigrationPlan {
    static var schemas: [VersionedSchema.Type] = [
        Fluster_DesktopVersionedSchema.self,
    ]

    static var stages: [MigrationStage] = [
        // Stages of migration between VersionedSchema, if required.
    ]
}

struct Fluster_DesktopVersionedSchema: VersionedSchema {
    static var versionIdentifier = Schema.Version(1, 0, 0)

    static var models: [any PersistentModel.Type] = [
        Item.self,
    ]
}
