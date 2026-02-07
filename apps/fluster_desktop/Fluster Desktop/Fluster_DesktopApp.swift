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
  @StateObject private var paletteController = CommandPaletteController()
  @AppStorage(DesktopAppStorageKeys.colorScheme.rawValue) private var selectedTheme: AppTheme =
    .dark
    @State private var appState: AppState = AppState.shared
  @State private var appData = AppDataContainer.shared
  var body: some Scene {
    WindowGroup("Fluster", id: "flusterDesktop") {
      ContentView()
        .toolbarBackground(.hidden, for: .automatic)
        .preferredColorScheme(selectedTheme.colorScheme)
    }
    .commands {
      CommandMenu("Tools") {
        Button("Command Palette") {
            paletteController.toggle(modelContainer: appData.sharedModelContainer)
        }
        .keyboardShortcut("p", modifiers: [.command, .shift])
      }
    }
    .environmentObject(appState)
    .environment(\.createDataHandler, appData.dataHandlerCreator())
    .windowStyle(.automatic)
    .windowToolbarStyle(.unified)
    .modelContainer(appData.sharedModelContainer)
  }
}
