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
  private var appData: AppDataContainer { AppDataContainer.shared }
  private var paletteController = CommandPaletteController()
  var body: some Scene {
    WindowGroup("Fluster", id: DesktopWindowId.mainDesktopWindowGroup.rawValue) {
      ContentView()
        .toolbarBackground(.hidden, for: .automatic)
    }
    .modelContainer(appData.sharedModelContainer)
    .environment(\.createDataHandler, appData.dataHandlerCreator())
    .windowStyle(.automatic)
    .windowToolbarStyle(.unified)
    .commands {
      SidebarCommands()
      TextEditingCommands()
      ToolbarCommands()

      // Or define your own custom one:
      CommandMenu("Layout") {
        Button("Toggle Sidebar") {
          NSApp.sendAction(#selector(NSSplitViewController.toggleSidebar(_:)), to: nil, from: nil)
        }
        .keyboardShortcut("l", modifiers: [.command, .shift])
      }
      CommandMenu("Tools") {
        Button("Command Palette") {
          paletteController.toggle()
        }
        .keyboardShortcut("p", modifiers: [.command, .shift])
      }
    }
  }
}
