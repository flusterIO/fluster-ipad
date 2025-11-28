//
//  app_data_migration.swift
//  Fluster
//
//  Created by Andrew on 11/27/25.
//

import SwiftData

public enum AppDataMigrationPlan: SchemaMigrationPlan {
    public static var schemas: [any VersionedSchema.Type] {
        [
            AppSchemaV1.self
        ]
    }
    public static var stages: [MigrationStage] {
        [
//           migrateV1ToV2
        ]
    }
    
//    static let migrateV1ToV2 = MigrationStage.lightweight(fromVersion: AppSchemaV1.self, toVersion: AppSchemaV2.self)
        
}
