//
//  command_palette_view_window.swift
//  Fluster
//
//  Created by Andrew on 2/7/26.
//

import AppKit
import Combine
import SwiftData
import SwiftUI

class CommandPaletteController: NSObject, ObservableObject {
  // We inherit from NSObject to handle window delegates if needed
  private var panel: CommandPalettePanel?

  func toggle(modelContainer: ModelContainer) {
    if let panel = panel, panel.isVisible {
      hide()
    } else {
        show(modelContainer)
    }
  }

  func show(_ modelContainer: ModelContainer) {
    if panel == nil {
      // Your custom SwiftUI View for the palette
      let rootView = CommandPaletteContainerView(
        close: { [weak self] in self?.hide() },
      )
      .ignoresSafeArea()
//      .frame(
//        width: CGFloat(COMMAND_PALETTE_WIDTH), height: CGFloat(COMMAND_PALETTE_HEIGHT),
//        alignment: .top
//      )
//      .frame(maxWidth: .infinity, maxHeight: .infinity)
      .environmentObject(AppState.shared)
      .modelContainer(modelContainer)

      panel = CommandPalettePanel(rootView: rootView)
      panel?.center()

      // Optional: Hide palette if user clicks elsewhere
      NotificationCenter.default.addObserver(
        forName: NSWindow.didResignKeyNotification,
        object: panel,
        queue: .main
      ) { [weak self] _ in
        self?.hide()
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
