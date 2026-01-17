//
//  Fluster_DesktopApp.swift
//  Fluster Desktop
//
//  Created by Andrew on 1/14/26.
//

import FlusterData
import SwiftData
import SwiftUI
import UniformTypeIdentifiers

@main
struct Fluster_DesktopApp: App {
  @AppStorage(DesktopAppStorageKeys.colorScheme.rawValue) private var selectedTheme: AppTheme =
    .dark
  let appData = AppDataContainer.shared
  var body: some Scene {
    WindowGroup("Fluster", id: "flusterDesktop") {
      ContentView()
        .toolbarBackground(.hidden, for: .automatic)
        .environment(\.createDataHandler, appData.dataHandlerCreator())
        .preferredColorScheme(selectedTheme.colorScheme)
    }
    .windowStyle(.hiddenTitleBar)
    .modelContainer(appData.sharedModelContainer)
  }
}

extension UTType {
  static var itemDocument: UTType {
    UTType(importedAs: "com.example.item-document")
  }
}
