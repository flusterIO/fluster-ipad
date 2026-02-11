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
  @StateObject private var appState: AppState = AppState.shared
  @AppStorage(DesktopAppStorageKeys.colorScheme.rawValue) private var selectedTheme: AppTheme =
    .dark
  private var appData: AppDataContainer { AppDataContainer.shared }
  @StateObject private var paletteController = CommandPaletteController()
  var body: some Scene {
    WindowGroup("Fluster", id: DesktopWindowId.mainDesktopWindowGroup.rawValue) {
        ContentView(editingNoteId: appState.editingNoteId)
        .toolbarBackground(.hidden, for: .automatic)
        .preferredColorScheme(selectedTheme.colorScheme)
    }
    .environmentObject(appState)
    .environment(\.createDataHandler, appData.dataHandlerCreator())
    .windowStyle(.automatic)
    .windowToolbarStyle(.unified)
    .modelContainer(appData.sharedModelContainer)
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
          paletteController.toggle(
            appState: appState,
            onCommandSelected: handleCommandPaletteSelect
          )
        }
        .keyboardShortcut("p", modifiers: [.command, .shift])
      }
    }
  }

  func toggleDarkMode() {
    let currentDarkMode =
      UserDefaults.standard.string(forKey: DesktopAppStorageKeys.colorScheme.rawValue)
      == AppTheme.dark.rawValue
    if currentDarkMode {
      UserDefaults.standard.set(
        AppTheme.light.rawValue, forKey: DesktopAppStorageKeys.colorScheme.rawValue)
    } else {
      UserDefaults.standard.set(
        AppTheme.dark.rawValue, forKey: DesktopAppStorageKeys.colorScheme.rawValue)
    }
  }

  func handleCommandPaletteSelect(_ command: CommandPaletteItem) -> CommandPaletteSelectResponse {
    if command.onAccept != nil {
      command.onAccept!()
    }
    if command.itemType == .children {
      return .appendToTree(command)
    } else {
      switch command.id {
        case .pushCommandPaletteView(let data):
          appState.commandPaletteNavigate(to: data)
        case .viewNoteById(let noteId):
          let note = NoteModel.fetchFirstById(id: noteId, modelContext: appData.sharedModelContainer.mainContext)
          if let _note = note {
            appState.setEditingNote(editingNote: _note)
            appState.mainView = .noteViewMdx
          }
        case .navigate(let mainKey):
          print("Would've run...")
        //          appState.mainView = mainKey
        //          MainNavigationEventEmitter.shared.emitChange(to: mainKey)
        case .toggleDarkMode:
          toggleDarkMode()
        case .createNewNote:
          MainNavigationEventEmitter.shared.emitChange(to: MainViewKey.noteEditingPage)
        case .showPanelRight:
          print("Show Panel Right")
        case .parentWithNoFunctionality:
          return .clearAndClose
        case .root:
          return .clearAndClose
      }
      return .clearAndClose
    }
  }
}
