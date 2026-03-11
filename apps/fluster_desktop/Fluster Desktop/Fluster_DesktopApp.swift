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
import WebKit

struct WindowAccessor: NSViewRepresentable {
  var color: NSColor

  func makeNSView(context: Context) -> NSView {
    let view = NSView()
    DispatchQueue.main.async {
      // Reaches up to the parent window
      view.window?.backgroundColor = color
    }
    return view
  }

  func updateNSView(_ nsView: NSView, context: Context) {}
}

@main
struct Fluster_DesktopApp: App {
  private var appData: AppDataContainer { AppDataContainer.shared }
  private var paletteController = CommandPaletteController()
  var x: Int {
    print("Stored: \(appData.sharedModelContainer.mainContext.sqliteCommand)")
    return 1
  }
  init() {
    SchemeRegistration.registerCustomScheme()
  }
  var body: some Scene {
    WindowGroup("Fluster", id: DesktopWindowId.mainDesktopWindowGroup.rawValue) {
      ContentView()
        .background(WindowAccessor(color: NSColor.windowBackground))
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

struct SchemeRegistration {
  static func registerCustomScheme() {
    let register = NSSelectorFromString("registerSchemeForCustomProtocol:")

    // We cast to NSObjectProtocol to check for the selector safely
    if let webViewClass = WKWebView.self as Any as? NSObjectProtocol,
      webViewClass.responds(to: register)
    {
      // Registering "app" as a custom protocol makes WebKit treat it
      // like "file://" or "https://", enabling persistent IndexedDB.
      webViewClass.perform(register, with: "app")

      print("🚀 Registered 'app://' as a secure custom scheme.")
    }
  }
}
