//
//  command_palette_view_window.swift
//  Fluster
//
//  Created by Andrew on 2/7/26.
///

import AppKit
import Combine
import FlusterData
import SwiftData
import SwiftUI

@MainActor
class CommandPaletteController: NSObject, ObservableObject {
  private var panel: CommandPalettePanel?

  private var appData: AppDataContainer { AppDataContainer.shared }

  func toggle(
    appState: AppState,
    onCommandSelected: @escaping (CommandPaletteItem) -> CommandPaletteSelectResponse
  ) {
    if let panel = panel, panel.isVisible {
      hide()
    } else {
      show(appState, onCommandSelected)
    }
  }

  func show(
    _ appState: AppState,
    _ onCommandSelected: @escaping (CommandPaletteItem) -> CommandPaletteSelectResponse
  ) {
    let rootView = CommandPaletteContainerView(
      close: { [weak self] in self?.hide() },
      onCommandSelected: onCommandSelected,
    )
    .ignoresSafeArea()
    .environmentObject(appState)
    .modelContainer(appData.sharedModelContainer)
    if let _panel = panel {
      _panel.contentView = NSHostingView(rootView: rootView)
    } else {
      panel = CommandPalettePanel(rootView: rootView)
      panel?.center()
      // Close on click-away
      NotificationCenter.default.addObserver(
        forName: NSWindow.didResignKeyNotification,
        object: panel,
        queue: .main
      ) { [weak panel] _ in
        panel?.orderOut(nil)
      }
    }

    panel?.makeKeyAndOrderFront(nil)
    // Bring the app to the foreground so the text field gets focus
    NSApp.activate(ignoringOtherApps: true)
  }

  func hide() {
    panel?.orderOut(nil)
  }
}
